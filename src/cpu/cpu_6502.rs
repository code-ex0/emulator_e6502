///
/// File: cpu/cpu_6502.rs
/// The cpu module contains the implementation of the cpu struct and the implementation of the cpu functions.
///

use crate::cpu::{
    instruction,
    flag::Flag,
};
use crate::cpu::register::Registers;
use crate::memory::ram::Ram;
use crate::util::constants::{MEMORY_SIZE, STACK_SIZE, RESET_ADDRESS_LOW, RESET_ADDRESS_HIGH, OPCODE_KIL};
use crate::util::types::{Byte, Word, Address};



#[derive(Clone, Copy, PartialEq)]
pub enum ExecutionState {
    Running,
    Stopped,
    Error,
}

#[derive(Clone, Copy)]
pub struct Cpu6502 {
    pub registers: Registers,
    pub memory: Ram,
}

impl Cpu6502 {
    pub fn new(ram: Ram) -> Cpu6502 {
        Cpu6502 {
            registers: Registers::new(),
            memory: ram,
        }
    }

    pub fn reset(&mut self) {
        self.registers.a = 0;
        self.registers.x = 0;
        self.registers.y = 0;
        self.registers.sp = 0xFF;
        self.registers.pc = 0;
        self.registers.status = 0;
        self.registers.pc = self.memory.read(RESET_ADDRESS_LOW) as u16 | (self.memory.read(RESET_ADDRESS_HIGH) as u16) << 8;
    }

    pub fn dump(&self) {
        println!("A: {:02X} X: {:02X} Y: {:02X} SP: {:02X} PC: {:04X} Status: {:02X}", self.registers.a, self.registers.x, self.registers.y, self.registers.sp, self.registers.pc, self.registers.status);
    }

    pub fn execute_instruction(&mut self) -> Option<ExecutionState> {
        if self.registers.pc as usize >= MEMORY_SIZE  {
            println!("{}", MEMORY_SIZE);
            println!("{:?}", self.registers.pc);
            eprintln!("PC out of bounds: {:04X}", self.registers.pc);
            return Some(ExecutionState::Error);
        }
        let opcode = self.memory.read(self.registers.pc);
        let instruction = match instruction::INSTRUCTIONS.get(opcode as usize) {
            Some(instr) => instr,
            None => {
                eprintln!("Unknown opcode: {:02X} at address {:04X}", opcode, self.registers.pc);
                return Some(ExecutionState::Error);
            }
        };
        let addressing_mode = instruction.addressing_mode;
        (instruction.execute)(self, addressing_mode);
        if instruction.name == OPCODE_KIL || instruction.name == "BRK" {
            return Some(ExecutionState::Stopped);
        }
        Some(ExecutionState::Running)
    }

    pub fn read_byte(&mut self, address: Address) -> Byte {
        self.registers.pc += 1;
        self.memory.read(address)
    }

    pub fn read_word(&mut self, address: Address) -> Word {
        let low = self.read_byte(address) as Word;
        let high = self.read_byte(address + 1) as Word;
        low | (high << 8)
    }

    pub fn write_byte(&mut self, address: Address, data: Byte) {
        self.registers.pc += 1;
        self.memory.write(address, data);
    }

    pub fn write_word(&mut self, address: Address, data: Word) {
        self.write_byte(address, data as Byte);
        self.write_byte(address.wrapping_add(1), (data >> 8) as Byte);
    }


    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.registers.status |= flag as Byte;
        } else {
            self.registers.status &= !(flag as Byte);
        }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        (self.registers.status & (flag as Byte)) != 0
    }

    pub fn push_stack(&mut self, data: Byte) {
        self.memory.write((STACK_SIZE as Word + self.registers.sp as Address) as Address, data);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
    }

    pub fn pop_stack(&mut self) -> Byte {
        self.registers.sp = self.registers.sp.wrapping_add(1);
        self.memory.read((STACK_SIZE as Word + self.registers.sp as Address) as Address)
    }

    pub fn push_word_stack(&mut self, data: Word) {
        self.push_stack((data >> 8) as Byte);
        self.push_stack(data as Byte);
    }

    pub fn pop_word_stack(&mut self) -> Word {
        let low = self.pop_stack() as Word;
        let high = self.pop_stack() as Word;
        low | (high << 8)
    }
}