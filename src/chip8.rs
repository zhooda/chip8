use crate::memory::Ram;

const MEM_OFFSET: usize = 0x200;

pub struct Chip8 {
    ram: Ram,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 { ram: Ram::new() }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        for i in 0..data.len() {
            self.ram.write_byte((MEM_OFFSET + i) as u16, data[i]);
            // println!("{:x}: {:x}", MEM_OFFSET + i, data[i]);
        }
    }
}
