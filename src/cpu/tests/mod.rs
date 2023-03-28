#![allow(unused_imports)]
use crate::cpu::{
    instruction,
    flag::Flag,
    cpu_6502::Cpu6502,
    addressing_mode::AddressingMode,
    function::*,
};
use crate::cpu::register::Registers;
use crate::memory::ram::Ram;
use crate::util::constants::{MEMORY_SIZE, STACK_SIZE, RESET_ADDRESS_LOW, RESET_ADDRESS_HIGH, OPCODE_KIL};
use crate::util::types::{Byte, Word, Address};


///
/// prepare a cpu for testing
///
pub fn get_cpu() -> Cpu6502 {
    let ram = Ram::new();
    let mut cpu = Cpu6502::new(ram);
    cpu.reset();
    cpu.memory.reset();
    cpu
}

///
/// # adc
/// Test the ADC instruction
///
#[cfg(test)]
mod adc_tests;



///
/// # lda
/// Test the LDA instruction
///
#[cfg(test)]
mod lda_tests;

///
/// # ldx
/// Test the LDX instruction
///
#[cfg(test)]
mod ldx_tests;

///
/// # ldy
/// Test the LDY instruction
///
#[cfg(test)]
mod ldy_tests;



///
/// # brk
/// Test the BRK instruction
///
#[cfg(test)]
mod brk_tests;

///
/// # cmp
/// Test the CMP instruction
///
#[cfg(test)]
mod cmp_tests;


///
/// # dec
/// Test the DEC instruction
///
#[cfg(test)]
mod dec_tests;