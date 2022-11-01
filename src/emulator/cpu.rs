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

    pub fn read_register(&self, register_num: usize) -> u32 {
        if register_num == 0 {
            0
        } else {
            self.register[register_num]
        }
    }

    pub fn write_register(&mut self, register_num: usize, data: u32) {
        if register_num != 0 {
            self.register[register_num] = data;
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

    pub fn slt(&mut self, rd: u8, rs: u8) {
        self.register[rd as usize] = if self.register[rd as usize] < self.register[rs as usize] {
            1
        } else {
            0
        };
    }

    pub fn addi(&mut self, rd: u8, rs: u8, imm: i32) {
        self.register[rd as usize] = self.register[rs as usize].wrapping_add(imm as u32);
    }
}
