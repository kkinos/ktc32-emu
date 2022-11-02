mod cpu;
mod memory;

use cpu::Cpu;
use memory::Memory;

#[derive(Debug)]
pub enum Format {
    RFormat {
        mnemonic: String,
        rd: u8,
        rs: u8,
    },
    I16Format {
        mnemonic: String,
        rd: u8,
        imm: u8,
    },
    I32Format {
        mnemonic: String,
        rd: u8,
        rs: u8,
        imm: i32,
    },
    JFormat {
        mnemonic: String,
        rd: u8,
        imm: i32,
    },
}

pub const CHECK_32BIT_INST: u32 = 0x0000_0020;

#[derive(Debug)]
pub struct Emulator {
    pub memory: Memory,
    pub cpu: Cpu,
    pub end_of_address: u32,
    pub break_point: u32,
}

impl Emulator {
    pub fn new(data: &Vec<u8>, address: u32) -> Self {
        Emulator {
            memory: Memory::new(data),
            cpu: Cpu::new(),
            end_of_address: address,
            break_point: address,
        }
    }

    pub fn run(&mut self) {
        if self.cpu.pc > self.end_of_address {
            return;
        }
        loop {
            self.step();
            if self.cpu.pc == self.break_point || self.cpu.pc == self.end_of_address {
                self.step();
                break;
            } else if self.cpu.pc > self.end_of_address {
                break;
            }
        }
    }

    pub fn step(&mut self) {
        let word_32 = self.memory.read_memory(self.cpu.pc);
        let word_16 = (word_32 & 0x0000FFFF) as u16;
        let current_pc = self.cpu.pc;

        if (CHECK_32BIT_INST & word_32) == 32 {
            self.cpu.pc += 4;

            let format = Self::decode_32(word_32);
            self.execute(&format);

            match format {
                Format::I32Format {
                    mnemonic,
                    rd,
                    rs,
                    imm,
                } => {
                    println!(
                        " pc : 0x{:08x} inst : 0b{:032b} {} r{} r{} {}",
                        current_pc, word_32, mnemonic, rd, rs, imm
                    )
                }
                Format::JFormat { mnemonic, rd, imm } => {
                    println!(
                        " pc : 0x{:08x} inst : 0b{:032b} {} r{}  {}",
                        current_pc, word_32, mnemonic, rd, imm
                    )
                }
                _ => {}
            }
        } else {
            self.cpu.pc += 2;

            let format = Self::decode_16(word_16);
            self.execute(&format);

            match format {
                Format::RFormat { mnemonic, rd, rs } => {
                    println!(
                        " pc : 0x{:08x} inst : 0b{:016b} {} r{} r{}",
                        current_pc, word_16, mnemonic, rd, rs
                    );
                }
                Format::I16Format { mnemonic, rd, imm } => {
                    println!(
                        " pc : 0x{:08x} inst : 0b{:016b} {} r{} {}",
                        current_pc, word_16, mnemonic, rd, imm
                    );
                }
                _ => {}
            }
        }
        self.cpu.register[0] = 0;
    }

    pub fn decode_32(word: u32) -> Format {
        let opcode = (word & 0x0000003F) as u8;
        let rd = ((word & 0x000007C0) >> 6) as u8;
        let rs = ((word & 0x0000F800) >> 11) as u8;
        let imm_i = ((word & 0xFFFF0000) as i32) >> 16;
        let imm_j = ((word & 0xFFFFF800) as i32) >> 11;

        match opcode {
            0b100000 => Format::I32Format {
                mnemonic: String::from("ADDI"),
                rd: rd,
                rs: rs,
                imm: imm_i,
            },
            0b100001 => Format::I32Format {
                mnemonic: String::from("ANDI"),
                rd: rd,
                rs: rs,
                imm: imm_i,
            },
            0b100010 => Format::I32Format {
                mnemonic: String::from("ORI"),
                rd: rd,
                rs: rs,
                imm: imm_i,
            },
            0b100011 => Format::I32Format {
                mnemonic: String::from("XORI"),
                rd: rd,
                rs: rs,
                imm: imm_i,
            },
            0b100100 => Format::I32Format {
                mnemonic: String::from("SLTI"),
                rd: rd,
                rs: rs,
                imm: imm_i,
            },
            0b100101 => Format::I32Format {
                mnemonic: String::from("SLTIU"),
                rd: rd,
                rs: rs,
                imm: imm_i,
            },
            0b100110 => Format::I32Format {
                mnemonic: String::from("BEQ"),
                rd: rd,
                rs: rs,
                imm: imm_i,
            },
            0b100111 => Format::I32Format {
                mnemonic: String::from("BNQ"),
                rd: rd,
                rs: rs,
                imm: imm_i,
            },
            0b101000 => Format::I32Format {
                mnemonic: String::from("BLT"),
                rd: rd,
                rs: rs,
                imm: imm_i,
            },
            0b101001 => Format::I32Format {
                mnemonic: String::from("BGE"),
                rd: rd,
                rs: rs,
                imm: imm_i,
            },
            0b101010 => Format::I32Format {
                mnemonic: String::from("BLTU"),
                rd: rd,
                rs: rs,
                imm: imm_i,
            },
            0b101011 => Format::I32Format {
                mnemonic: String::from("BGEU"),
                rd: rd,
                rs: rs,
                imm: imm_i,
            },
            0b101100 => Format::I32Format {
                mnemonic: String::from("JALR"),
                rd: rd,
                rs: rs,
                imm: imm_i,
            },
            0b101101 => Format::I32Format {
                mnemonic: String::from("LB"),
                rd: rd,
                rs: rs,
                imm: imm_i,
            },
            0b101110 => Format::I32Format {
                mnemonic: String::from("LH"),
                rd: rd,
                rs: rs,
                imm: imm_i,
            },
            0b101111 => Format::I32Format {
                mnemonic: String::from("LBU"),
                rd: rd,
                rs: rs,
                imm: imm_i,
            },
            0b110000 => Format::I32Format {
                mnemonic: String::from("LHU"),
                rd: rd,
                rs: 0,
                imm: imm_i,
            },
            0b110001 => Format::I32Format {
                mnemonic: String::from("LW"),
                rd: rd,
                rs: rs,
                imm: imm_i,
            },
            0b110010 => Format::I32Format {
                mnemonic: String::from("LUI"),
                rd: rd,
                rs: rs,
                imm: imm_i,
            },
            0b110011 => Format::I32Format {
                mnemonic: String::from("SB"),
                rd: rd,
                rs: rs,
                imm: imm_i,
            },
            0b110100 => Format::I32Format {
                mnemonic: String::from("SH"),
                rd: rd,
                rs: rs,
                imm: imm_i,
            },
            0b110101 => Format::I32Format {
                mnemonic: String::from("SW"),
                rd: rd,
                rs: rs,
                imm: imm_i,
            },
            0b111111 => Format::JFormat {
                mnemonic: String::from("JAL"),
                rd: rd,
                imm: imm_j,
            },
            _ => Format::I32Format {
                mnemonic: String::from("UNKNOWN"),
                rd: 0,
                rs: 0,
                imm: 0,
            },
        }
    }

    pub fn decode_16(word: u16) -> Format {
        let opcode = (word & 0x003F) as u8;
        let rd = ((word & 0x07C0) >> 6) as u8;
        let rs = ((word & 0xF800) >> 11) as u8;
        let imm = ((word & 0xF800) >> 11) as u8;

        match opcode {
            0b000000 => Format::RFormat {
                mnemonic: String::from("MOV"),
                rd: rd,
                rs: rs,
            },
            0b000001 => Format::RFormat {
                mnemonic: String::from("ADD"),
                rd: rd,
                rs: rs,
            },
            0b000010 => Format::RFormat {
                mnemonic: String::from("SUB"),
                rd: rd,
                rs: rs,
            },
            0b000011 => Format::RFormat {
                mnemonic: String::from("AND"),
                rd: rd,
                rs: rs,
            },
            0b000100 => Format::RFormat {
                mnemonic: String::from("OR"),
                rd: rd,
                rs: rs,
            },
            0b000101 => Format::RFormat {
                mnemonic: String::from("XOR"),
                rd: rd,
                rs: rs,
            },
            0b000110 => Format::RFormat {
                mnemonic: String::from("SLL"),
                rd: rd,
                rs: rs,
            },
            0b000111 => Format::RFormat {
                mnemonic: String::from("SRL"),
                rd: rd,
                rs: rs,
            },
            0b001000 => Format::RFormat {
                mnemonic: String::from("SRA"),
                rd: rd,
                rs: rs,
            },
            0b001001 => Format::RFormat {
                mnemonic: String::from("SLT"),
                rd: rd,
                rs: rs,
            },
            0b001010 => Format::RFormat {
                mnemonic: String::from("SLTU"),
                rd: rd,
                rs: rs,
            },
            0b010000 => Format::I16Format {
                mnemonic: String::from("SLLI"),
                rd: rd,
                imm: imm,
            },
            0b010001 => Format::I16Format {
                mnemonic: String::from("SRLI"),
                rd: rd,
                imm: imm,
            },
            0b010010 => Format::I16Format {
                mnemonic: String::from("SRAI"),
                rd: rd,
                imm: imm,
            },
            _ => Format::RFormat {
                mnemonic: String::from("UNKNOWN"),
                rd: 0,
                rs: 0,
            },
        }
    }

    pub fn execute(&mut self, format: &Format) {
        match format {
            Format::RFormat { mnemonic, rd, rs } => match mnemonic.as_str() {
                "MOV" => self.cpu.mov(*rd, *rs),
                "ADD" => self.cpu.add(*rd, *rs),
                "SUB" => self.cpu.sub(*rd, *rs),
                "AND" => self.cpu.and(*rd, *rs),
                "OR" => self.cpu.or(*rd, *rs),
                "XOR" => self.cpu.xor(*rd, *rs),
                "SLL" => self.cpu.sll(*rd, *rs),
                "SRL" => self.cpu.srl(*rd, *rs),
                "SRA" => self.cpu.sra(*rd, *rs),
                "SLT" => self.cpu.slt(*rd, *rs),
                "SLTU" => self.cpu.sltu(*rd, *rs),
                _ => {}
            },

            Format::I16Format { mnemonic, rd, imm } => match mnemonic.as_str() {
                "SLLI" => self.cpu.slli(*rd, *imm),
                "SRLI" => self.cpu.srli(*rd, *imm),
                "SRAI" => self.cpu.srai(*rd, *imm),
                _ => {}
            },

            Format::I32Format {
                mnemonic,
                rd,
                rs,
                imm,
            } => match mnemonic.as_str() {
                "ADDI" => self.cpu.addi(*rd, *rs, *imm),
                "ANDI" => self.cpu.andi(*rd, *rs, *imm),
                "ORI" => self.cpu.ori(*rd, *rs, *imm),
                "XORI" => self.cpu.xori(*rd, *rs, *imm),
                "SLTI" => self.cpu.slti(*rd, *rs, *imm),
                "SLTIU" => self.cpu.sltiu(*rd, *rs, *imm),
                "BEQ" => {
                    if self.cpu.register[*rd as usize] == self.cpu.register[*rs as usize] {
                        self.cpu.pc = self.cpu.pc.wrapping_add(*imm as u32);
                    }
                }
                "BNQ" => {
                    if self.cpu.register[*rd as usize] != self.cpu.register[*rs as usize] {
                        self.cpu.pc = self.cpu.pc.wrapping_add(*imm as u32);
                    }
                }
                "BLT" => {
                    if (self.cpu.register[*rd as usize] as i32)
                        < (self.cpu.register[*rs as usize] as i32)
                    {
                        self.cpu.pc = self.cpu.pc.wrapping_add(*imm as u32);
                    }
                }
                "BGE" => {
                    if (self.cpu.register[*rd as usize] as i32)
                        >= (self.cpu.register[*rs as usize] as i32)
                    {
                        self.cpu.pc = self.cpu.pc.wrapping_add(*imm as u32);
                    }
                }
                "BLTU" => {
                    if self.cpu.register[*rd as usize] < self.cpu.register[*rs as usize] {
                        self.cpu.pc = self.cpu.pc.wrapping_add(*imm as u32);
                    }
                }
                "BGEU" => {
                    if self.cpu.register[*rd as usize] >= self.cpu.register[*rs as usize] {
                        self.cpu.pc = self.cpu.pc.wrapping_add(*imm as u32);
                    }
                }
                "JALR" => {
                    self.cpu.register[*rd as usize] = self.cpu.pc;
                    self.cpu.pc = self.cpu.register[*rs as usize].wrapping_add(*imm as u32);
                }
                "LB" => {
                    self.cpu.register[*rd as usize] = ((self
                        .memory
                        .read_memory_8bit(self.cpu.register[*rs as usize].wrapping_add(*imm as u32))
                        as i8) as i32) as u32
                }
                "LH" => {
                    self.cpu.register[*rd as usize] = ((self.memory.read_memory_16bit(
                        self.cpu.register[*rs as usize].wrapping_add(*imm as u32),
                    ) as i16) as i32) as u32
                }
                "LBU" => {
                    self.cpu.register[*rd as usize] = self
                        .memory
                        .read_memory_8bit(self.cpu.register[*rs as usize].wrapping_add(*imm as u32))
                        as u32
                }
                "LHU" => {
                    self.cpu.register[*rd as usize] = self.memory.read_memory_16bit(
                        self.cpu.register[*rs as usize].wrapping_add(*imm as u32),
                    ) as u32
                }
                "LW" => {
                    self.cpu.register[*rd as usize] = self
                        .memory
                        .read_memory(self.cpu.register[*rs as usize].wrapping_add(*imm as u32))
                }
                "LUI" => self.cpu.register[*rd as usize] = (*imm << 16) as u32,
                "SB" => self.memory.write_memory_8bit(
                    self.cpu.register[*rs as usize].wrapping_add(*imm as u32),
                    self.cpu.register[*rd as usize] as u8,
                ),
                "SH" => self.memory.write_memory_16bit(
                    self.cpu.register[*rs as usize].wrapping_add(*imm as u32),
                    self.cpu.register[*rd as usize] as u16,
                ),
                "SW" => self.memory.write_memory(
                    self.cpu.register[*rs as usize].wrapping_add(*imm as u32),
                    self.cpu.register[*rd as usize],
                ),
                _ => {}
            },

            Format::JFormat { mnemonic, rd, imm } => match mnemonic.as_str() {
                "JAL" => {
                    self.cpu.register[*rd as usize] = self.cpu.pc;
                    self.cpu.pc = self.cpu.pc.wrapping_add(*imm as u32);
                }
                _ => {}
            },
        }
    }
}
