#![allow(dead_code)]

use std::fs::File;
use std::io::Read;
use std::{thread, time};

use emul::util::types::Address;
use emul::cpu::cpu_6502::{Cpu6502, ExecutionState};
use emul::memory::ram::Ram;


#[derive(Clone, Copy)]
pub struct InputOutput {
    keyboard: [bool; 16],
    display: [bool; 64 * 32],

}

#[derive(Clone, Copy)]
pub struct Emulator {
    memory: Ram,
    pub cpu: Cpu6502,
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

    emulator.flash_ram();
    emulator.reset();

    loop {
        let kill = emulator.cpu.execute_instruction();
        emulator.dump_cpu();
        match kill {
            None => {}
            Some(x) => {
                match x {
                    ExecutionState::Running => {
                        thread::sleep(time::Duration::from_millis(100));
                    }
                    ExecutionState::Error | ExecutionState::Stopped => {
                        emulator.cpu.memory.hexdump();
                        break
                    }
                }
            }
        }
    }
}
