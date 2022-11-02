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

    pub fn mov(&mut self, rd: u8, rs: u8) {
        self.register[rd as usize] = self.register[rs as usize];
    }
    pub fn add(&mut self, rd: u8, rs: u8) {
        self.register[rd as usize] =
            self.register[rd as usize].wrapping_add(self.register[rs as usize]);
    }
    pub fn sub(&mut self, rd: u8, rs: u8) {
        self.register[rd as usize] =
            self.register[rd as usize].wrapping_sub(self.register[rs as usize]);
    }

    pub fn and(&mut self, rd: u8, rs: u8) {
        self.register[rd as usize] = self.register[rd as usize] & self.register[rs as usize];
    }

    pub fn or(&mut self, rd: u8, rs: u8) {
        self.register[rd as usize] = self.register[rd as usize] | self.register[rs as usize];
    }

    pub fn xor(&mut self, rd: u8, rs: u8) {
        self.register[rd as usize] = self.register[rd as usize] ^ self.register[rs as usize];
    }

    pub fn sll(&mut self, rd: u8, rs: u8) {
        self.register[rd as usize] =
            self.register[rd as usize] << (self.register[rs as usize] & 0x0000_001F);
    }
    pub fn srl(&mut self, rd: u8, rs: u8) {
        self.register[rd as usize] =
            self.register[rd as usize] >> (self.register[rs as usize] & 0x0000_001F);
    }
    pub fn sra(&mut self, rd: u8, rs: u8) {
        self.register[rd as usize] = ((self.register[rd as usize] as i32)
            >> (self.register[rs as usize] & 0x0000_001F))
            as u32;
    }

    pub fn slt(&mut self, rd: u8, rs: u8) {
        self.register[31] =
            if (self.register[rd as usize] as i32) < (self.register[rs as usize] as i32) {
                1
            } else {
                0
            };
    }

    pub fn sltu(&mut self, rd: u8, rs: u8) {
        self.register[31] = if self.register[rd as usize] < self.register[rs as usize] {
            1
        } else {
            0
        };
    }

    pub fn slli(&mut self, rd: u8, imm: u8) {
        self.register[rd as usize] = self.register[rd as usize] << imm;
    }
    pub fn srli(&mut self, rd: u8, imm: u8) {
        self.register[rd as usize] = self.register[rd as usize] >> imm;
    }
    pub fn srai(&mut self, rd: u8, imm: u8) {
        self.register[rd as usize] = ((self.register[rd as usize] as i32) >> imm) as u32;
    }

    pub fn addi(&mut self, rd: u8, rs: u8, imm: i32) {
        self.register[rd as usize] = self.register[rs as usize].wrapping_add(imm as u32);
    }
    pub fn andi(&mut self, rd: u8, rs: u8, imm: i32) {
        self.register[rd as usize] = self.register[rs as usize] & (imm as u32);
    }
    pub fn ori(&mut self, rd: u8, rs: u8, imm: i32) {
        self.register[rd as usize] = self.register[rs as usize] | (imm as u32);
    }
    pub fn xori(&mut self, rd: u8, rs: u8, imm: i32) {
        self.register[rd as usize] = self.register[rs as usize] ^ (imm as u32);
    }
    pub fn slti(&mut self, rd: u8, rs: u8, imm: i32) {
        self.register[rd as usize] = if (self.register[rs as usize] as i32) < imm {
            1
        } else {
            0
        };
    }
    pub fn sltiu(&mut self, rd: u8, rs: u8, imm: i32) {
        self.register[rd as usize] = if self.register[rs as usize] < (imm as u32) {
            1
        } else {
            0
        };
    }
}
