#![allow(dead_code)]

use std::fs::File;
use std::io::Read;
use std::{thread, time};
use cpu6502::{Address, Cpu6502, Ram};


#[derive(Clone, Copy)]
pub struct InputOutput {
    keyboard: [bool; 16],
    display: [bool; 64 * 32],

}

#[derive(Clone, Copy)]
pub struct Emulator {
    memory: Ram,
    cpu: Cpu6502,
    io: InputOutput,
}

impl Emulator {
    pub fn new() -> Emulator {
        let ram = Ram::new();
        Emulator {
            memory: ram,
            cpu: Cpu6502::new(ram),
            io: InputOutput {
                keyboard: [false; 16],
                display: [false; 64 * 32],
            },
        }
    }

    pub fn reset(&mut self) {
        self.memory.reset();
        self.cpu.reset();
    }

    pub fn flash_ram(&mut self) {
        self.cpu.memory = self.memory;
    }

    pub fn load(&mut self, data: &[u8], offset: Address) {
        self.memory.load(data, offset);
    }

    pub fn dump(&self, offset: Address, len: usize) -> Vec<u8> {
        self.memory.dump(offset, len)
    }

    pub fn hexdump(&self) {
        self.memory.hexdump();
    }

    pub fn dump_cpu(&self) {
        self.cpu.dump();
    }

    pub fn step(&mut self) -> Option<bool> {
        self.cpu.step()
    }

    pub fn load_binary(&mut self, path: &str, offset: Address) {
        let mut file = File::open(path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        self.load(&buffer, offset);
    }
}

fn main() {
    let mut emulator = Emulator::new();
    emulator.reset();
    emulator.load_binary("o6502-2023-03-01-160025.bin", 0x0000);
    emulator.hexdump();


    // // function start to address 0x0200
    // emulator.load(&[0xA9, 0x09, 0x69, 0x09, 0x4C, 0x00, 0x01, 0x00], 0x200);

    // // function start to address 0x1000
    // emulator.load( &[0xBA, 0x00], 0x1000);

    // reset function
    // emulator.memory.write(0xFFFC, 0x16);
    // emulator.memory.write(0xFFFD, 0x00);

    // // break function
    // emulator.memory.write(0xFFFE, 0x00);
    // emulator.memory.write(0xFFFF, 0x10);

    emulator.flash_ram();
    emulator.reset();

    // emulator.dump_cpu();
    // emulator.hexdump();

    loop {
        let kill = emulator.step();
        emulator.dump_cpu();
        match kill {
            None => {}
            Some(x) => {
                if x == true {
                    emulator.cpu.memory.hexdump();
                    break
                }
                if x == false {
                    emulator.cpu.memory.hexdump();
                    // time sleep 1 second
                    thread::sleep(time::Duration::from_millis(10000));
                }
            }
        }
    }
}
