#[derive(Debug)]
pub struct Cpu {
    pub pc: u32,
    pub register: [u32; 32],
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            pc: 0,
            register: [0; 32],
        }
    }

    pub fn mov(&mut self, rs1: u8, rs2: u8) {
        self.register[rs1 as usize] = self.register[rs2 as usize];
    }
    pub fn add(&mut self, rs1: u8, rs2: u8) {
        self.register[rs1 as usize] =
            self.register[rs1 as usize].wrapping_add(self.register[rs2 as usize]);
    }
    pub fn sub(&mut self, rs1: u8, rs2: u8) {
        self.register[rs1 as usize] =
            self.register[rs1 as usize].wrapping_sub(self.register[rs2 as usize]);
    }

    pub fn and(&mut self, rs1: u8, rs2: u8) {
        self.register[rs1 as usize] = self.register[rs1 as usize] & self.register[rs2 as usize];
    }

    pub fn or(&mut self, rs1: u8, rs2: u8) {
        self.register[rs1 as usize] = self.register[rs1 as usize] | self.register[rs2 as usize];
    }

    pub fn slt(&mut self, rs1: u8, rs2: u8) {
        self.register[rs1 as usize] = if self.register[rs1 as usize] < self.register[rs2 as usize] {
            1
        } else {
            0
        };
    }

    pub fn addi(&mut self, rd: u8, rs: u8, imm: i32) {
        self.register[rd as usize] = self.register[rs as usize].wrapping_add(imm as u32);
    }
}
