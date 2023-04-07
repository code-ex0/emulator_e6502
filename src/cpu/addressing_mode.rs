///
/// File: cpu/addressing_mode.rs
/// The addressing mode module contains the implementation of the addressing mode
/// enum and the implementation of the addressing mode functions.
///

use crate::cpu::cpu_6502::Cpu6502;
use std::fmt;


#[derive(Clone, Copy)]
pub enum AddressingMode {
    Implied,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    Relative,
    Accumulator,
}

impl AddressingMode {
    pub fn get_address(&self, cpu: &mut Cpu6502) -> u16 {
        match self {
            AddressingMode::Implied => 0,
            AddressingMode::Immediate => {
                cpu.registers.pc += 1;
                cpu.registers.pc
            },
            AddressingMode::ZeroPage => cpu.read_byte(cpu.registers.pc + 1) as u16,
            AddressingMode::ZeroPageX => ((cpu.read_byte(cpu.registers.pc + 1) + cpu.registers.x) & 0xFF) as u16,
            AddressingMode::ZeroPageY => ((cpu.read_byte(cpu.registers.pc + 1) + cpu.registers.y) & 0xFF) as u16,
            AddressingMode::Absolute => cpu.read_word(cpu.registers.pc + 1),
            AddressingMode::AbsoluteX => cpu.read_word(cpu.registers.pc + 1) + cpu.registers.x as u16,
            AddressingMode::AbsoluteY => cpu.read_word(cpu.registers.pc + 1) + cpu.registers.y as u16,
            AddressingMode::Indirect => {
                let address = cpu.read_word(cpu.registers.pc + 1);
                let low_byte = cpu.read_byte(address) as u16;
                let high_byte = if address & 0xFF == 0xFF {
                    cpu.read_byte(address & !0xFF) as u16
                } else {
                    cpu.read_byte(address + 1) as u16
                };
                low_byte | (high_byte << 8)
            },
            AddressingMode::IndirectX => {
                let address = cpu.read_byte(cpu.registers.pc + 1) + cpu.registers.x;
                cpu.read_word(address as u16)
            }
            AddressingMode::IndirectY => {
                let address = cpu.read_byte(cpu.registers.pc + 1);
                cpu.read_word(address as u16) + cpu.registers.y as u16
            }
            AddressingMode::Relative => cpu.read_byte(cpu.registers.pc + 1) as u16,
            AddressingMode::Accumulator => cpu.registers.a as u16,
        }
    }
}


impl fmt::Display for AddressingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mode = match self {
            AddressingMode::Immediate => "immediate",
            AddressingMode::ZeroPage => "zero_page",
            AddressingMode::ZeroPageX => "zero_page_x",
            AddressingMode::ZeroPageY => "zero_page_y",
            AddressingMode::Absolute => "absolute",
            AddressingMode::AbsoluteX => "absolute_x",
            AddressingMode::AbsoluteY => "absolute_y",
            AddressingMode::Indirect => "indirect",
            AddressingMode::IndirectX => "indirect_x",
            AddressingMode::IndirectY => "indirect_y",
            AddressingMode::Relative => "relative",
            AddressingMode::Implied => "implied",
            AddressingMode::Accumulator => "accumulator",
        };
        write!(f, "{}", mode)
    }
}