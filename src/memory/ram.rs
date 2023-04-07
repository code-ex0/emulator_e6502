///
/// File: memory/ram.rs
/// The RAM module contains the implementation of the RAM chip.
/// The RAM chip is a 64KB memory chip that is used to store the
/// program and data that is being executed by the CPU.
///

use crate::util::types::{Byte, Address};
use crate::util::constants::{MEMORY_SIZE};

#[derive(Clone, Copy)]
pub struct Ram {
    memory: [Byte; MEMORY_SIZE],
}



impl Ram {
    pub fn new() -> Ram {
        Ram {
            memory: [0; MEMORY_SIZE],
        }
    }

    pub fn read(&self, address: Address) -> Byte {
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: Address, data: Byte) {
        self.memory[address as usize] = data;
    }

    pub fn load(&mut self, data: &[u8], offset: Address) {
        for (i, &byte) in data.iter().enumerate() {
            self.write(offset + i as Address, byte);
        }
    }

    pub fn reset(&mut self) {
        self.memory = [0; MEMORY_SIZE];
    }

    pub fn dump(&self, offset: Address, len: usize) -> Vec<u8> {
        self.memory[offset as usize..offset as usize + len].to_vec()
    }

    pub fn hexdump(&self) {
        let mut line: [u8; 16] = [0; 16];
        let mut line_empty: bool = true;
        let mut line_ascii: String = String::new();
        let mut line_address: u16 = 0;
        for (i, &byte) in self.memory.iter().enumerate() {
            if i % 16 == 0 {
                if !line_empty {
                    print!("{:04X}  ", line_address);
                    for byte in line.iter() {
                        print!("{:02X} ", byte);
                    }
                    print!("  ");
                    for byte in line_ascii.chars() {
                        print!("{}", byte);
                    }
                    println!();
                }
                line = [0; 16];
                line_empty = true;
                line_ascii = String::new();
                line_address = i as u16;
            }
            if byte != 0 {
                line_empty = false;
            }
            line[i % 16] = byte;
            if byte >= 32 && byte <= 126 {
                line_ascii.push(byte as char);
            } else {
                line_ascii.push('.');
            }
        }
        if !line_empty {
            print!("{:04X}  ", line_address);
            for byte in line.iter() {
                print!("{:02X} ", byte);
            }
            print!("  ");
            for byte in line_ascii.chars() {
                print!("{}", byte);
            }
            println!();
        }
    }
}