use anyhow::{anyhow, Result};
#[derive(Debug)]
pub struct Memory {
    pub memory_array: Vec<u8>,
}

pub const MEMORY_SIZE: u32 = 768; // 768Byte

impl Memory {
    pub fn new(memory_array: Vec<u8>) -> Self {
        Self { memory_array }
    }

    pub fn init(&mut self) {
        for _i in 0..((MEMORY_SIZE as usize) - self.memory_array.len()) {
            self.memory_array.push(0);
        }
    }

    pub fn read_data_8bit(&self, address: u32) -> Result<u8> {
        if address > MEMORY_SIZE {
            return Err(anyhow!("out of memory"));
        }
        Ok(self.memory_array[address as usize])
    }

    pub fn read_data_16bit(&self, address: u32) -> Result<u16> {
        if address + 1 > MEMORY_SIZE {
            return Err(anyhow!("out of memory"));
        }
        Ok((self.memory_array[(address + 1) as usize] as u16) << 8
            | self.memory_array[address as usize] as u16)
    }

    pub fn read_data(&self, address: u32) -> Result<u32> {
        if address + 3 > MEMORY_SIZE {
            return Err(anyhow!("out of memory"));
        }
        Ok((self.memory_array[(address + 3) as usize] as u32) << 24
            | (self.memory_array[(address + 2) as usize] as u32) << 16
            | (self.memory_array[(address + 1) as usize] as u32) << 8
            | self.memory_array[address as usize] as u32)
    }

    pub fn write_data_8bit(&mut self, address: u32, data: u8) -> Result<()> {
        if address > MEMORY_SIZE {
            return Err(anyhow!("out of memory"));
        }
        self.memory_array[(address) as usize] = data;
        Ok(())
    }

    pub fn write_data_16bit(&mut self, address: u32, data: u16) -> Result<()> {
        if address + 1 > MEMORY_SIZE {
            return Err(anyhow!("out of memory"));
        }
        self.memory_array[(address) as usize] = (data & 0x00FF) as u8;
        self.memory_array[(address + 1) as usize] = (data & 0xFF00) as u8;
        Ok(())
    }

    pub fn write_data(&mut self, address: u32, data: u32) -> Result<()> {
        if address + 3 > MEMORY_SIZE {
            return Err(anyhow!("out of memory"));
        }
        self.memory_array[(address) as usize] = (data & 0x000000FF) as u8;
        self.memory_array[(address + 1) as usize] = (data & 0x0000FF00) as u8;
        self.memory_array[(address + 2) as usize] = (data & 0x00FF0000) as u8;
        self.memory_array[(address + 3) as usize] = (data & 0xFF000000) as u8;
        Ok(())
    }
}
