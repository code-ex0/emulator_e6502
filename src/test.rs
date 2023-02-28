#![allow(unused_imports)]
use crate::{
    Cpu6502,
    Flag,
    Byte,
    Word,
    Ram,
    function::*,
    instruction::{
        AddressingMode,
        Instruction,
        INSTRUCTIONS
    }
};


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
/// ADC
/// Test the ADC instruction
///
#[cfg(test)]
mod adc_tests {
    use crate::Emulator;
    use super::*;

    #[test]
    fn test_adc_simple() {
        let mut cpu = get_cpu();

        cpu.a = 0x01;
        cpu.memory.write(0x0001, 0x01);
        cpu.set_flag(Flag::Carry, true);
        adc(&mut cpu, AddressingMode::Immediate);
        assert_eq!(cpu.a, 0x03);
        assert_eq!(cpu.get_flag(Flag::Carry), false);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
        assert_eq!(cpu.get_flag(Flag::Overflow), false);
    }

    #[test]
    fn test_adc_overflow() {
        let mut cpu = get_cpu();

        cpu.a = 0x7F;
        cpu.memory.write(0x0001, 0x01);
        cpu.set_flag(Flag::Carry, true);
        adc(&mut cpu, AddressingMode::Immediate);
        assert_eq!(cpu.a, 0x81);
        assert_eq!(cpu.get_flag(Flag::Carry), false);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), true);
        assert_eq!(cpu.get_flag(Flag::Overflow), true);
    }

    #[test]
    fn test_adc_carry() {
        let mut cpu = get_cpu();

        cpu.a = 0xFF;
        cpu.memory.write(0x0001, 0x01);
        cpu.set_flag(Flag::Carry, true);
        adc(&mut cpu, AddressingMode::Immediate);
        assert_eq!(cpu.a, 0x01);
        assert_eq!(cpu.get_flag(Flag::Carry), true);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
        assert_eq!(cpu.get_flag(Flag::Overflow), false);
    }

    #[test]
    fn test_adc_zero() {
        let mut cpu = get_cpu();

        cpu.a = 0x00;
        cpu.memory.write(0x0001, 0x00);
        cpu.set_flag(Flag::Carry, true);
        adc(&mut cpu, AddressingMode::Immediate);
        assert_eq!(cpu.a, 0x01);
        assert_eq!(cpu.get_flag(Flag::Carry), false);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
        assert_eq!(cpu.get_flag(Flag::Overflow), false);
    }

    #[test]
    fn test_adc_negative() {
        let mut cpu = get_cpu();

        cpu.a = 0x80;
        cpu.memory.write(0x0001, 0x01);
        cpu.set_flag(Flag::Carry, true);
        adc(&mut cpu, AddressingMode::Immediate);
        assert_eq!(cpu.a, 0x82);
        assert_eq!(cpu.get_flag(Flag::Carry), false);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), true);
        assert_eq!(cpu.get_flag(Flag::Overflow), false);
    }
}

///
/// lda
/// Test the LDA instruction
///
#[cfg(test)]
mod lda_tests {
    use crate::Emulator;
    use super::*;

    #[test]
    fn test_lda_immediate() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x01);
        lda(&mut cpu, AddressingMode::Immediate);
        assert_eq!(cpu.a, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_lda_zero_page() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x01);
        cpu.memory.write(0x0002, 0x01);
        lda(&mut cpu, AddressingMode::ZeroPage);
        assert_eq!(cpu.a, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_lda_zero_page_x() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x01);
        cpu.memory.write(0x0002, 0x01);
        cpu.x = 0x01;
        lda(&mut cpu, AddressingMode::ZeroPageX);
        assert_eq!(cpu.a, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_lda_absolute() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x01);
        cpu.memory.write(0x0002, 0x01);
        cpu.memory.write(0x0101, 0x01);
        lda(&mut cpu, AddressingMode::Absolute);
        assert_eq!(cpu.a, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_lda_absolute_x() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x01);
        cpu.memory.write(0x0002, 0x01);
        cpu.memory.write(0x0102, 0x01);
        cpu.x = 0x01;
        lda(&mut cpu, AddressingMode::AbsoluteX);
        assert_eq!(cpu.a, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_lda_absolute_y() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x01);
        cpu.memory.write(0x0002, 0x01);
        cpu.memory.write(0x0102, 0x01);
        cpu.y = 0x01;
        lda(&mut cpu, AddressingMode::AbsoluteY);
        assert_eq!(cpu.a, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_lda_indirect_x() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x01);
        cpu.memory.write(0x0002, 0x01);
        cpu.memory.write(0x0003, 0x01);
        cpu.memory.write(0x0004, 0x01);
        cpu.memory.write(0x0101, 0x01);
        cpu.x = 0x01;
        lda(&mut cpu, AddressingMode::IndirectX);
        assert_eq!(cpu.a, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_lda_indirect_y() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x01);
        cpu.memory.write(0x0002, 0x01);
        cpu.memory.write(0x0003, 0x01);
        cpu.memory.write(0x0004, 0x01);
        cpu.memory.write(0x0102, 0x01);
        cpu.y = 0x01;
        lda(&mut cpu, AddressingMode::IndirectY);
        assert_eq!(cpu.a, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_lda_negative() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x80);
        lda(&mut cpu, AddressingMode::Immediate);
        assert_eq!(cpu.a, 0x80);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), true);
    }

    #[test]
    fn test_lda_zero() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x00);
        lda(&mut cpu, AddressingMode::Immediate);
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }
}

///
/// ldx
/// Test the LDX instruction
///
#[cfg(test)]
mod ldx_tests {
    use crate::Emulator;
    use super::*;

    #[test]
    fn test_ldx_immediate() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x01);
        ldx(&mut cpu, AddressingMode::Immediate);
        assert_eq!(cpu.x, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_ldx_zero_page() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x01);
        cpu.memory.write(0x0002, 0x01);
        ldx(&mut cpu, AddressingMode::ZeroPage);
        assert_eq!(cpu.x, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_ldx_zero_page_x() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x01);
        cpu.memory.write(0x0002, 0x01);
        cpu.x = 0x01;
        ldx(&mut cpu, AddressingMode::ZeroPageX);
        assert_eq!(cpu.x, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_ldx_zero_page_y() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x01);
        cpu.memory.write(0x0002, 0x01);
        cpu.y = 0x01;
        ldx(&mut cpu, AddressingMode::ZeroPageY);
        assert_eq!(cpu.x, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_ldx_absolute() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x01);
        cpu.memory.write(0x0002, 0x01);
        cpu.memory.write(0x0101, 0x01);
        ldx(&mut cpu, AddressingMode::Absolute);
        assert_eq!(cpu.x, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_ldx_absolute_x() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x01);
        cpu.memory.write(0x0002, 0x01);
        cpu.memory.write(0x0102, 0x01);
        cpu.x = 0x01;
        ldx(&mut cpu, AddressingMode::AbsoluteX);
        assert_eq!(cpu.x, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_ldx_absolute_y() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x01);
        cpu.memory.write(0x0002, 0x01);
        cpu.memory.write(0x0102, 0x01);
        cpu.y = 0x01;
        ldx(&mut cpu, AddressingMode::AbsoluteY);
        assert_eq!(cpu.x, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_ldx_zero() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x00);
        ldx(&mut cpu, AddressingMode::Immediate);
        assert_eq!(cpu.x, 0x00);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_ldx_negative() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x80);
        ldx(&mut cpu, AddressingMode::Immediate);
        assert_eq!(cpu.x, 0x80);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), true);
    }



}


///
/// ldy
/// Test the LDY instruction
///
#[cfg(test)]
mod ldy_tests {
    use crate::Emulator;
    use super::*;

    #[test]
    fn test_ldy_immediate() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x01);
        ldy(&mut cpu, AddressingMode::Immediate);
        assert_eq!(cpu.y, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_ldy_zero_page() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x01);
        cpu.memory.write(0x0002, 0x01);
        ldy(&mut cpu, AddressingMode::ZeroPage);
        assert_eq!(cpu.y, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_ldy_zero_page_x() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x01);
        cpu.memory.write(0x0002, 0x01);
        cpu.x = 0x01;
        ldy(&mut cpu, AddressingMode::ZeroPageX);
        assert_eq!(cpu.y, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_ldy_zero_page_y() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x01);
        cpu.memory.write(0x0002, 0x01);
        cpu.y = 0x01;
        ldy(&mut cpu, AddressingMode::ZeroPageY);
        assert_eq!(cpu.y, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_ldy_absolute() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x01);
        cpu.memory.write(0x0002, 0x01);
        cpu.memory.write(0x0101, 0x01);
        ldy(&mut cpu, AddressingMode::Absolute);
        assert_eq!(cpu.y, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_ldy_absolute_x() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x01);
        cpu.memory.write(0x0002, 0x01);
        cpu.memory.write(0x0102, 0x01);
        cpu.x = 0x01;
        ldy(&mut cpu, AddressingMode::AbsoluteX);
        assert_eq!(cpu.y, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_ldy_absolute_y() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x01);
        cpu.memory.write(0x0002, 0x01);
        cpu.memory.write(0x0102, 0x01);
        cpu.y = 0x01;
        ldy(&mut cpu, AddressingMode::AbsoluteY);
        assert_eq!(cpu.y, 0x01);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_ldy_zero() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x00);
        ldy(&mut cpu, AddressingMode::Immediate);
        assert_eq!(cpu.y, 0x00);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_ldy_negative() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x80);
        ldy(&mut cpu, AddressingMode::Immediate);
        assert_eq!(cpu.y, 0x80);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), true);
    }
}



///
/// brk
/// Test the BRK instruction
///
#[cfg(test)]
mod brk_tests {
    use crate::Emulator;
    use super::*;

    #[test]
    fn test_brk() {
        let mut cpu = get_cpu();

        cpu.pc = 0x1000;
        cpu.status = 0b11111111;
        cpu.memory.write(0xFFFE, 0x10);
        cpu.memory.write(0xFFFF, 0x10);
        brk(&mut cpu, AddressingMode::Implied);
        assert_eq!(cpu.pc, 0x1010);
        assert_eq!(cpu.get_flag(Flag::Break), true);
        assert_eq!(cpu.get_flag(Flag::Interrupt), true);
        assert_eq!(cpu.memory.read(0x0100), 0x10);
        assert_eq!(cpu.memory.read(0x0101), 0x00);
        assert_eq!(cpu.memory.read(0x0102), 0b11111111);
    }
}

///
/// cmp
/// Test the CMP instruction
///
#[cfg(test)]
mod cmp_tests {
    use crate::Emulator;
    use super::*;

    #[test]
    fn test_cmp_equal() {
        let mut cpu = get_cpu();

        cpu.a = 0x01;
        cpu.memory.write(0x0001, 0x01);
        cmp(&mut cpu, AddressingMode::Immediate);
        assert_eq!(cpu.get_flag(Flag::Carry), true);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_cmp_upper() {
        let mut cpu = get_cpu();

        cpu.a = 0x02;
        cpu.memory.write(0x0001, 0x01);
        cmp(&mut cpu, AddressingMode::Immediate);
        assert_eq!(cpu.get_flag(Flag::Carry), true);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_cmp_lower() {
        let mut cpu = get_cpu();

        cpu.a = 0x01;
        cpu.memory.write(0x0001, 0x02);
        cmp(&mut cpu, AddressingMode::Immediate);
        assert_eq!(cpu.get_flag(Flag::Carry), false);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), true);
    }

    #[test]
    fn test_cmp_zero() {
        let mut cpu = get_cpu();

        cpu.a = 0x00;
        cpu.memory.write(0x0001, 0x00);
        cmp(&mut cpu, AddressingMode::Immediate);
        assert_eq!(cpu.get_flag(Flag::Carry), true);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }

    #[test]
    fn test_cmp_negative() {
        let mut cpu = get_cpu();

        cpu.a = 0x80;
        cpu.memory.write(0x0001, 0x00);
        cmp(&mut cpu, AddressingMode::Immediate);
        assert_eq!(cpu.get_flag(Flag::Carry), true);
        assert_eq!(cpu.get_flag(Flag::Zero), false);
        assert_eq!(cpu.get_flag(Flag::Negative), true);
    }
// todo
}


///
/// dec
/// Test the DEC instruction
///
#[cfg(test)]
mod dec_tests {
    use crate::Emulator;
    use super::*;

    #[test]
    fn test_dec() {
        let mut cpu = get_cpu();

        cpu.memory.write(0x0001, 0x01);
        dec(&mut cpu, AddressingMode::ZeroPage);
        assert_eq!(cpu.memory.read(0x0001), 0x00);
        assert_eq!(cpu.get_flag(Flag::Zero), true);
        assert_eq!(cpu.get_flag(Flag::Negative), false);
    }
}