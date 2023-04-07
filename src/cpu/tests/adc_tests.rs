use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::flag::Flag;
use crate::cpu::function::adc;
use super::*;
extern crate rand;

#[test]
fn test_adc_simple() {
    let mut cpu = get_cpu();

    cpu.registers.a = 0x01;
    cpu.memory.write(0x0001, 0x01);
    cpu.set_flag(Flag::Carry, true);
    adc(&mut cpu, AddressingMode::Immediate);
    assert_eq!(cpu.registers.a, 0x03);
    assert_eq!(cpu.get_flag(Flag::Carry), false);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), false);
    assert_eq!(cpu.get_flag(Flag::Overflow), false);
}

#[test]
fn test_adc_overflow() {
    let mut cpu = get_cpu();

    cpu.registers.a = 0x7F;
    cpu.memory.write(0x0001, 0x01);
    cpu.set_flag(Flag::Carry, true);
    adc(&mut cpu, AddressingMode::Immediate);
    assert_eq!(cpu.registers.a, 0x81);
    assert_eq!(cpu.get_flag(Flag::Carry), false);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), true);
    assert_eq!(cpu.get_flag(Flag::Overflow), true);
}

#[test]
fn test_adc_carry() {
    let mut cpu = get_cpu();

    cpu.registers.a = 0xFF;
    cpu.memory.write(0x0001, 0x01);
    cpu.set_flag(Flag::Carry, true);
    adc(&mut cpu, AddressingMode::Immediate);
    assert_eq!(cpu.registers.a, 0x01);
    assert_eq!(cpu.get_flag(Flag::Carry), true);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), false);
    assert_eq!(cpu.get_flag(Flag::Overflow), false);
}

#[test]
fn test_adc_zero() {
    let mut cpu = get_cpu();

    cpu.registers.a = 0x00;
    cpu.memory.write(0x0001, 0x00);
    cpu.set_flag(Flag::Carry, true);
    adc(&mut cpu, AddressingMode::Immediate);
    assert_eq!(cpu.registers.a, 0x01);
    assert_eq!(cpu.get_flag(Flag::Carry), false);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), false);
    assert_eq!(cpu.get_flag(Flag::Overflow), false);
}

#[test]
fn test_adc_negative() {
    let mut cpu = get_cpu();

    cpu.registers.a = 0x80;
    cpu.memory.write(0x0001, 0x01);
    cpu.set_flag(Flag::Carry, true);
    adc(&mut cpu, AddressingMode::Immediate);
    assert_eq!(cpu.registers.a, 0x82);
    assert_eq!(cpu.get_flag(Flag::Carry), false);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), true);
    assert_eq!(cpu.get_flag(Flag::Overflow), false);
}

// loop random tests
#[test]
fn test_adc_random() {
    for _ in 0..1000 {
        let mut cpu = get_cpu();
        let a = rand::random::<u8>();
        let b = rand::random::<u8>();
        let c = rand::random::<bool>();

        cpu.registers.a = a;
        cpu.memory.write(0x0001, b);
        cpu.set_flag(Flag::Carry, c);
        adc(&mut cpu, AddressingMode::Immediate);

        let expected = a.wrapping_add(b).wrapping_add(c as u8);
        assert_eq!(cpu.registers.a, expected);
        assert_eq!(cpu.get_flag(Flag::Carry), expected < a || expected < b);
        assert_eq!(cpu.get_flag(Flag::Zero), expected == 0);
        assert_eq!(cpu.get_flag(Flag::Negative), expected & 0x80 != 0);
        assert_eq!(cpu.get_flag(Flag::Overflow), (a ^ b) & 0x80 == 0 && (a ^ expected) & 0x80 != 0);
    }
}