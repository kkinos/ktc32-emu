use anyhow::Result;
pub mod cpu;
pub mod memory;

use cpu::Cpu;
use memory::Memory;

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Type {
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
    pub break_point: u32,
}

impl Emulator {
    pub fn new(program: Vec<u8>) -> Self {
        let bread_point = program.len() as u32;
        Self {
            memory: Memory::new(program),
            cpu: Cpu::new(),
            break_point: bread_point,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            self.step()?;
            if self.cpu.pc == self.break_point || self.cpu.pc >= memory::MEMORY_SIZE {
                break;
            }
        }
        Ok(())
    }

    pub fn step(&mut self) -> Result<()> {
        let word_32 = self.memory.read_data(self.cpu.pc)?;
        let word_16 = (word_32 & 0x0000FFFF) as u16;
        let current_pc = self.cpu.pc;

        if (CHECK_32BIT_INST & word_32) == 32 {
            self.cpu.pc += 4;

            let inst = Self::decode_32(word_32);
            match &inst {
                Type::I32Format {
                    mnemonic,
                    rd,
                    rs,
                    imm,
                } => match mnemonic.as_str() {
                    "LUI" => {
                        println!(
                            " pc : 0x{:08x} inst : 0b{:032b} {} r{}  {}",
                            current_pc, word_32, mnemonic, rd, imm
                        )
                    }
                    _ => {
                        println!(
                            " pc : 0x{:08x} inst : 0b{:032b} {} r{} r{} {}",
                            current_pc, word_32, mnemonic, rd, rs, imm
                        )
                    }
                },
                Type::JFormat { mnemonic, rd, imm } => {
                    println!(
                        " pc : 0x{:08x} inst : 0b{:032b} {} r{}  {}",
                        current_pc, word_32, mnemonic, rd, imm
                    )
                }
                _ => {}
            }
            match self.execute(&inst) {
                Ok(_) => {}
                Err(error) => {
                    println!("{}", error)
                }
            }
        } else {
            self.cpu.pc += 2;

            let inst = Self::decode_16(word_16);

            match &inst {
                Type::RFormat { mnemonic, rd, rs } => {
                    println!(
                        " pc : 0x{:08x} inst : 0b{:016b} {} r{} r{}",
                        current_pc, word_16, mnemonic, rd, rs
                    );
                }
                Type::I16Format { mnemonic, rd, imm } => {
                    println!(
                        " pc : 0x{:08x} inst : 0b{:016b} {} r{} {}",
                        current_pc, word_16, mnemonic, rd, imm
                    );
                }
                _ => {}
            }
            match self.execute(&inst) {
                Ok(_) => {}
                Err(error) => {
                    println!("{}", error)
                }
            }
        }
        Ok(())
    }

    pub fn decode_32(word: u32) -> Type {
        let opcode = (word & 0x0000003F) as u8;
        let rd = ((word & 0x000007C0) >> 6) as u8;
        let rs = ((word & 0x0000F800) >> 11) as u8;
        let imm_i = ((word & 0xFFFF0000) as i32) >> 16;
        let imm_j = ((word & 0xFFFFF800) as i32) >> 11;

        match opcode {
            0b100000 => Type::I32Format {
                mnemonic: String::from("ADDI"),
                rd,
                rs,
                imm: imm_i,
            },
            0b100001 => Type::I32Format {
                mnemonic: String::from("ANDI"),
                rd,
                rs,
                imm: imm_i,
            },
            0b100010 => Type::I32Format {
                mnemonic: String::from("ORI"),
                rd,
                rs,
                imm: imm_i,
            },
            0b100011 => Type::I32Format {
                mnemonic: String::from("XORI"),
                rd,
                rs,
                imm: imm_i,
            },
            0b100100 => Type::I32Format {
                mnemonic: String::from("SLTI"),
                rd,
                rs,
                imm: imm_i,
            },
            0b100101 => Type::I32Format {
                mnemonic: String::from("SLTIU"),
                rd,
                rs,
                imm: imm_i,
            },
            0b100110 => Type::I32Format {
                mnemonic: String::from("BEQ"),
                rd,
                rs,
                imm: imm_i,
            },
            0b100111 => Type::I32Format {
                mnemonic: String::from("BNQ"),
                rd,
                rs,
                imm: imm_i,
            },
            0b101000 => Type::I32Format {
                mnemonic: String::from("BLT"),
                rd,
                rs,
                imm: imm_i,
            },
            0b101001 => Type::I32Format {
                mnemonic: String::from("BGE"),
                rd,
                rs,
                imm: imm_i,
            },
            0b101010 => Type::I32Format {
                mnemonic: String::from("BLTU"),
                rd,
                rs,
                imm: imm_i,
            },
            0b101011 => Type::I32Format {
                mnemonic: String::from("BGEU"),
                rd,
                rs,
                imm: imm_i,
            },
            0b101100 => Type::I32Format {
                mnemonic: String::from("JALR"),
                rd,
                rs,
                imm: imm_i,
            },
            0b101101 => Type::I32Format {
                mnemonic: String::from("LB"),
                rd,
                rs,
                imm: imm_i,
            },
            0b101110 => Type::I32Format {
                mnemonic: String::from("LH"),
                rd,
                rs,
                imm: imm_i,
            },
            0b101111 => Type::I32Format {
                mnemonic: String::from("LBU"),
                rd,
                rs,
                imm: imm_i,
            },
            0b110000 => Type::I32Format {
                mnemonic: String::from("LHU"),
                rd,
                rs: 0,
                imm: imm_i,
            },
            0b110001 => Type::I32Format {
                mnemonic: String::from("LW"),
                rd,
                rs,
                imm: imm_i,
            },
            0b110010 => Type::I32Format {
                mnemonic: String::from("LUI"),
                rd,
                rs,
                imm: imm_i,
            },
            0b110011 => Type::I32Format {
                mnemonic: String::from("SB"),
                rd,
                rs,
                imm: imm_i,
            },
            0b110100 => Type::I32Format {
                mnemonic: String::from("SH"),
                rd,
                rs,
                imm: imm_i,
            },
            0b110101 => Type::I32Format {
                mnemonic: String::from("SW"),
                rd,
                rs,
                imm: imm_i,
            },
            0b111111 => Type::JFormat {
                mnemonic: String::from("JAL"),
                rd,
                imm: imm_j,
            },
            _ => Type::I32Format {
                mnemonic: String::from("UNKNOWN"),
                rd: 0,
                rs: 0,
                imm: 0,
            },
        }
    }

    pub fn decode_16(word: u16) -> Type {
        let opcode = (word & 0x003F) as u8;
        let rd = ((word & 0x07C0) >> 6) as u8;
        let rs = ((word & 0xF800) >> 11) as u8;
        let imm = ((word & 0xF800) >> 11) as u8;

        match opcode {
            0b000000 => Type::RFormat {
                mnemonic: String::from("MOV"),
                rd,
                rs,
            },
            0b000001 => Type::RFormat {
                mnemonic: String::from("ADD"),
                rd,
                rs,
            },
            0b000010 => Type::RFormat {
                mnemonic: String::from("SUB"),
                rd,
                rs,
            },
            0b000011 => Type::RFormat {
                mnemonic: String::from("AND"),
                rd,
                rs,
            },
            0b000100 => Type::RFormat {
                mnemonic: String::from("OR"),
                rd,
                rs,
            },
            0b000101 => Type::RFormat {
                mnemonic: String::from("XOR"),
                rd,
                rs,
            },
            0b000110 => Type::RFormat {
                mnemonic: String::from("SLL"),
                rd,
                rs,
            },
            0b000111 => Type::RFormat {
                mnemonic: String::from("SRL"),
                rd,
                rs,
            },
            0b001000 => Type::RFormat {
                mnemonic: String::from("SRA"),
                rd,
                rs,
            },
            0b001001 => Type::RFormat {
                mnemonic: String::from("SLT"),
                rd,
                rs,
            },
            0b001010 => Type::RFormat {
                mnemonic: String::from("SLTU"),
                rd,
                rs,
            },
            0b010000 => Type::I16Format {
                mnemonic: String::from("SLLI"),
                rd,
                imm,
            },
            0b010001 => Type::I16Format {
                mnemonic: String::from("SRLI"),
                rd,
                imm,
            },
            0b010010 => Type::I16Format {
                mnemonic: String::from("SRAI"),
                rd,
                imm,
            },
            _ => Type::RFormat {
                mnemonic: String::from("UNKNOWN"),
                rd: 0,
                rs: 0,
            },
        }
    }

    #[allow(clippy::single_match)]
    pub fn execute(&mut self, format: &Type) -> Result<()> {
        match format {
            Type::RFormat { mnemonic, rd, rs } => match mnemonic.as_str() {
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

            Type::I16Format { mnemonic, rd, imm } => match mnemonic.as_str() {
                "SLLI" => self.cpu.slli(*rd, *imm),
                "SRLI" => self.cpu.srli(*rd, *imm),
                "SRAI" => self.cpu.srai(*rd, *imm),
                _ => {}
            },

            Type::I32Format {
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
                    if self.cpu.get_reg(*rd) == self.cpu.get_reg(*rs) {
                        self.cpu.pc = self.cpu.pc.wrapping_add(*imm as u32);
                    }
                }
                "BNQ" => {
                    if self.cpu.get_reg(*rd) != self.cpu.get_reg(*rs) {
                        self.cpu.pc = self.cpu.pc.wrapping_add(*imm as u32);
                    }
                }
                "BLT" => {
                    if (self.cpu.get_reg(*rd) as i32) < (self.cpu.get_reg(*rs) as i32) {
                        self.cpu.pc = self.cpu.pc.wrapping_add(*imm as u32);
                    }
                }
                "BGE" => {
                    if (self.cpu.get_reg(*rd) as i32) >= (self.cpu.get_reg(*rs) as i32) {
                        self.cpu.pc = self.cpu.pc.wrapping_add(*imm as u32);
                    }
                }
                "BLTU" => {
                    if self.cpu.get_reg(*rd) < self.cpu.get_reg(*rs) {
                        self.cpu.pc = self.cpu.pc.wrapping_add(*imm as u32);
                    }
                }
                "BGEU" => {
                    if self.cpu.get_reg(*rd) >= self.cpu.get_reg(*rs) {
                        self.cpu.pc = self.cpu.pc.wrapping_add(*imm as u32);
                    }
                }
                "JALR" => {
                    self.cpu.set_reg(*rd, self.cpu.pc);
                    self.cpu.pc = self.cpu.get_reg(*rs).wrapping_add(*imm as u32);
                }
                "LB" => self.cpu.set_reg(
                    *rd,
                    ((self
                        .memory
                        .read_data_8bit(self.cpu.register[*rs as usize].wrapping_add(*imm as u32))?
                        as i8) as i32) as u32,
                ),
                "LH" => self.cpu.set_reg(
                    *rd,
                    ((self.memory.read_data_16bit(
                        self.cpu.register[*rs as usize].wrapping_add(*imm as u32),
                    )? as i16) as i32) as u32,
                ),
                "LBU" => self.cpu.set_reg(
                    *rd,
                    self.memory
                        .read_data_8bit(self.cpu.register[*rs as usize].wrapping_add(*imm as u32))?
                        as u32,
                ),
                "LHU" => self.cpu.set_reg(
                    *rd,
                    self.memory.read_data_16bit(
                        self.cpu.register[*rs as usize].wrapping_add(*imm as u32),
                    )? as u32,
                ),
                "LW" => self.cpu.set_reg(
                    *rd,
                    self.memory
                        .read_data(self.cpu.register[*rs as usize].wrapping_add(*imm as u32))?,
                ),
                "LUI" => self.cpu.set_reg(*rd, (*imm << 16) as u32),
                "SB" => self.memory.write_data_8bit(
                    self.cpu.get_reg(*rs).wrapping_add(*imm as u32),
                    self.cpu.get_reg(*rd) as u8,
                )?,
                "SH" => self.memory.write_data_16bit(
                    self.cpu.get_reg(*rs).wrapping_add(*imm as u32),
                    self.cpu.get_reg(*rd) as u16,
                )?,
                "SW" => self.memory.write_data(
                    self.cpu.get_reg(*rs).wrapping_add(*imm as u32),
                    self.cpu.get_reg(*rd),
                )?,
                _ => {}
            },

            Type::JFormat { mnemonic, rd, imm } => match mnemonic.as_str() {
                "JAL" => {
                    self.cpu.set_reg(*rd, self.cpu.pc);
                    self.cpu.pc = self.cpu.pc.wrapping_add(*imm as u32);
                }
                _ => {}
            },
        }
        Ok(())
    }
}
