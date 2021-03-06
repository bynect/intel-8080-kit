use intel_8080_kit::emu::{Emulator, Memory};
use std::{
    env, fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

struct MemoryBase([u8; u16::MAX as usize]);

impl MemoryBase {
    pub fn new() -> Self {
        Self([0; u16::MAX as usize])
    }

    pub fn from_slice(base: &[u8]) -> Self {
        let mut mem = Self::new();
        for (i, v) in base.iter().enumerate() {
            mem.0[i] = *v;
        }
        mem
    }
}

impl Memory for MemoryBase {
    fn out_port(&self, port: u8, byte: u8) {
        println!("Output byte {} to port {}.", byte, port);
    }

    fn in_port(&self, port: u8) -> u8 {
        println!("Input byte from port {}.", port);
        0
    }

    fn read_byte(&self, addr: u16) -> u8 {
        self.0[addr as usize]
    }

    fn read_word(&self, addr: u16) -> u16 {
        (self.read_byte(addr + 1) as u16) << 8 | self.read_byte(addr + 1) as u16
    }

    fn write_byte(&mut self, addr: u16, byte: u8) {
        self.0[addr as usize] = byte;
    }

    fn write_word(&mut self, addr: u16, word: u16) {
        self.write_byte(addr, (word & 0xff) as u8);
        self.write_byte(addr + 1, (word >> 8) as u8);
    }
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    for arg in &args[1..] {
        let path = Path::new(&arg);

        if path.exists() {
            let bin = fs::read(arg).unwrap();
            let mem = Box::new(MemoryBase::from_slice(&bin));
            let mut emu = Emulator::new(mem);

            let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            emu.run();
            let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

            println!("Execution of {} took {:?}.", arg, (end - start))
        } else {
            eprintln!("{} doesn't exist.", arg);
        }
    }
}
