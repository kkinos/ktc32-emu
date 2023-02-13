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
    pub fn get_reg(&mut self, num: u8) -> u32 {
        if num == 0 {
            0
        } else {
            self.register[num as usize]
        }
    }
    pub fn set_reg(&mut self, num: u8, data: u32) {
        if num != 0 {
            self.register[num as usize] = data;
        }
    }

    pub fn mov(&mut self, rd: u8, rs: u8) {
        let data = self.get_reg(rs);
        self.set_reg(rd, data);
    }
    pub fn add(&mut self, rd: u8, rs: u8) {
        let data = self.get_reg(rd).wrapping_add(self.get_reg(rs));
        self.set_reg(rd, data);
    }
    pub fn sub(&mut self, rd: u8, rs: u8) {
        let data = self.get_reg(rd).wrapping_sub(self.get_reg(rs));
        self.set_reg(rd, data);
    }

    pub fn and(&mut self, rd: u8, rs: u8) {
        let data = self.get_reg(rd) & self.get_reg(rs);
        self.set_reg(rd, data);
    }

    pub fn or(&mut self, rd: u8, rs: u8) {
        let data = self.get_reg(rd) | self.get_reg(rs);
        self.set_reg(rd, data);
    }

    pub fn xor(&mut self, rd: u8, rs: u8) {
        let data = self.get_reg(rd) ^ self.get_reg(rs);
        self.set_reg(rd, data);
    }

    pub fn sll(&mut self, rd: u8, rs: u8) {
        let data = self.get_reg(rd) << self.get_reg(rs) & 0x0000_001F;
        self.set_reg(rd, data)
    }
    pub fn srl(&mut self, rd: u8, rs: u8) {
        let data = self.get_reg(rd) >> self.get_reg(rs) & 0x0000_001F;
        self.set_reg(rd, data);
    }
    pub fn sra(&mut self, rd: u8, rs: u8) {
        let data = (self.get_reg(rd) as i32) >> (self.get_reg(rs) & 0x0000_001F);
        self.set_reg(rd, data as u32);
    }

    pub fn slt(&mut self, rd: u8, rs: u8) {
        let data = if (self.get_reg(rd) as i32) < (self.get_reg(rs) as i32) {
            1
        } else {
            0
        };
        self.set_reg(31, data);
    }

    pub fn sltu(&mut self, rd: u8, rs: u8) {
        let data = if self.get_reg(rd) < self.get_reg(rs) {
            1
        } else {
            0
        };
        self.set_reg(31, data)
    }

    pub fn slli(&mut self, rd: u8, imm: u8) {
        let data = self.get_reg(rd) << imm;
        self.set_reg(rd, data);
    }
    pub fn srli(&mut self, rd: u8, imm: u8) {
        let data = self.get_reg(rd) >> imm;
        self.set_reg(rd, data);
    }
    pub fn srai(&mut self, rd: u8, imm: u8) {
        let data = (self.get_reg(rd) as i32) >> imm;
        self.set_reg(rd, data as u32);
    }

    pub fn addi(&mut self, rd: u8, rs: u8, imm: i32) {
        let data = self.get_reg(rs).wrapping_add(imm as u32);
        self.set_reg(rd, data);
    }
    pub fn andi(&mut self, rd: u8, rs: u8, imm: i32) {
        let data = self.get_reg(rs) & (imm as u32);
        self.set_reg(rd, data);
    }
    pub fn ori(&mut self, rd: u8, rs: u8, imm: i32) {
        let data = self.get_reg(rs) | (imm as u32);
        self.set_reg(rd, data);
    }
    pub fn xori(&mut self, rd: u8, rs: u8, imm: i32) {
        let data = self.get_reg(rs) ^ (imm as u32);
        self.set_reg(rd, data);
    }
    pub fn slti(&mut self, rd: u8, rs: u8, imm: i32) {
        let data = if (self.get_reg(rs) as i32) < imm {
            1
        } else {
            0
        };
        self.set_reg(rd, data)
    }
    pub fn sltiu(&mut self, rd: u8, rs: u8, imm: i32) {
        let data = if self.get_reg(rs) < (imm as u32) {
            1
        } else {
            0
        };
        self.set_reg(rd, data)
    }
}
