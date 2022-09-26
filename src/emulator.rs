mod cpu;
mod memory;

use cpu::Cpu;
use memory::Memory;

#[derive(Debug)]
pub struct InstData {
    pub inst: String,
    pub r1: u8,
    pub r2: u8,
    pub imm: i32,
}

pub const CHECK_32BIT: u32 = 0x0000_0001;

#[derive(Debug)]
pub struct Emulator {
    pub memory: Memory,
    pub cpu: Cpu,
    pub end_of_address: u32,
}

impl Emulator {
    pub fn new(data: &Vec<u8>, address: u32) -> Self {
        Emulator {
            memory: Memory::new(data),
            cpu: Cpu::new(),
            end_of_address: address,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.step();
            if self.cpu.pc >= self.end_of_address {
                break;
            }
        }
    }

    pub fn step(&mut self) {
        let word_32 = self.memory.read_data(self.cpu.pc);
        let word_16 = (word_32 & 0x0000FFFF) as u16;

        if (CHECK_32BIT & word_32) == 1 {
            self.cpu.pc += 4;

            let inst_data = Self::decode_32(word_32);
            self.execute(&inst_data);

            println!(
                " pc : 0x{:08x} inst : 0b{:032b} {} r{} r{} {}",
                self.cpu.pc - 4,
                word_32,
                inst_data.inst,
                inst_data.r1,
                inst_data.r2,
                inst_data.imm
            );
        } else {
            self.cpu.pc += 2;

            let inst_data = Self::decode_16(word_16);
            self.execute(&inst_data);

            println!(
                " pc : 0x{:08x} inst : 0b{:016b} {} r{} r{}",
                self.cpu.pc - 2,
                word_16,
                inst_data.inst,
                inst_data.r1,
                inst_data.r2
            );
        }
    }

    pub fn decode_32(word: u32) -> InstData {
        let opcode = (word & 0x0000003F) as u8;
        let r1 = ((word & 0x000007C0) >> 6) as u8;
        let r2 = ((word & 0x0000F800) >> 11) as u8;
        let imm = ((word & 0xFFFF0000) as i32) >> 16;

        match opcode {
            0b000011 => InstData {
                inst: "LW".to_string(),
                r1: r1,
                r2: r2,
                imm: imm,
            },
            0b100011 => InstData {
                inst: "ADDI".to_string(),
                r1: r1,
                r2: r2,
                imm: imm,
            },
            0b000111 => InstData {
                inst: "SW".to_string(),
                r1: r1,
                r2: r2,
                imm: imm,
            },
            0b000001 => InstData {
                inst: "JMP".to_string(),
                r1: r1,
                r2: r2,
                imm: imm,
            },
            0b100001 => InstData {
                inst: "JEQ".to_string(),
                r1: r1,
                r2: r2,
                imm: imm,
            },
            _ => InstData {
                inst: "Unknown".to_string(),
                r1: r1,
                r2: r2,
                imm: imm,
            },
        }
    }

    pub fn decode_16(word: u16) -> InstData {
        let opcode = (word & 0x003F) as u8;
        let r1 = ((word & 0x07C0) >> 6) as u8;
        let r2 = ((word & 0xF800) >> 11) as u8;

        match opcode {
            0b000000 => InstData {
                inst: "MOV".to_string(),
                r1: r1,
                r2: r2,
                imm: 0,
            },
            0b100000 => InstData {
                inst: "ADD".to_string(),
                r1: r1,
                r2: r2,
                imm: 0,
            },
            0b110000 => InstData {
                inst: "SUB".to_string(),
                r1: r1,
                r2: r2,
                imm: 0,
            },
            0b010000 => InstData {
                inst: "AND".to_string(),
                r1: r1,
                r2: r2,
                imm: 0,
            },
            0b011000 => InstData {
                inst: "OR".to_string(),
                r1: r1,
                r2: r2,
                imm: 0,
            },
            0b001000 => InstData {
                inst: "SLT".to_string(),
                r1: r1,
                r2: r2,
                imm: 0,
            },
            _ => InstData {
                inst: "Unknown".to_string(),
                r1: r1,
                r2: r2,
                imm: 0,
            },
        }
    }

    pub fn execute(&mut self, inst_data: &InstData) {
        match inst_data.inst.as_str() {
            "MOV" => {
                self.cpu.mov(inst_data.r1, inst_data.r2);
            }
            "ADD" => {
                self.cpu.add(inst_data.r1, inst_data.r2);
            }
            "SUB" => {
                self.cpu.sub(inst_data.r1, inst_data.r2);
            }
            "AND" => {
                self.cpu.and(inst_data.r1, inst_data.r2);
            }
            "OR" => {
                self.cpu.or(inst_data.r1, inst_data.r2);
            }
            "SLT" => {
                self.cpu.slt(inst_data.r1, inst_data.r2);
            }
            "LW" => {
                self.cpu.register[inst_data.r1 as usize] = self
                    .memory
                    .read_data(self.cpu.register[inst_data.r2 as usize] + (inst_data.imm as u32));
            }
            "ADDI" => {
                self.cpu.addi(inst_data.r1, inst_data.r2, inst_data.imm);
            }
            "SW" => self.memory.write_data(
                self.cpu.register[inst_data.r2 as usize] + (inst_data.imm as u32),
                self.cpu.register[inst_data.r1 as usize],
            ),
            "JMP" => self.cpu.pc += inst_data.imm as u32,
            "JEQ" => {
                if self.cpu.register[inst_data.r1 as usize]
                    == self.cpu.register[inst_data.r2 as usize]
                {
                    self.cpu.pc += inst_data.imm as u32
                }
            }

            _ => {}
        }
    }
}
