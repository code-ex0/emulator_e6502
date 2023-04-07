use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::flag::Flag;
use crate::cpu::function::cmp;
use super::*;

#[test]
fn test_cmp_equal() {
    let mut cpu = get_cpu();

    cpu.registers.a = 0x01;
    cpu.memory.write(0x0001, 0x01);
    cmp(&mut cpu, AddressingMode::Immediate);
    assert_eq!(cpu.get_flag(Flag::Carry), true);
    assert_eq!(cpu.get_flag(Flag::Zero), true);
    assert_eq!(cpu.get_flag(Flag::Negative), false);
}

#[test]
fn test_cmp_upper() {
    let mut cpu = get_cpu();

    cpu.registers.a = 0x02;
    cpu.memory.write(0x0001, 0x01);
    cmp(&mut cpu, AddressingMode::Immediate);
    assert_eq!(cpu.get_flag(Flag::Carry), true);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), false);
}

#[test]
fn test_cmp_lower() {
    let mut cpu = get_cpu();

    cpu.registers.a = 0x01;
    cpu.memory.write(0x0001, 0x02);
    cmp(&mut cpu, AddressingMode::Immediate);
    assert_eq!(cpu.get_flag(Flag::Carry), false);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), true);
}

#[test]
fn test_cmp_zero() {
    let mut cpu = get_cpu();

    cpu.registers.a = 0x00;
    cpu.memory.write(0x0001, 0x00);
    cmp(&mut cpu, AddressingMode::Immediate);
    assert_eq!(cpu.get_flag(Flag::Carry), true);
    assert_eq!(cpu.get_flag(Flag::Zero), true);
    assert_eq!(cpu.get_flag(Flag::Negative), false);
}

#[test]
fn test_cmp_negative() {
    let mut cpu = get_cpu();

    cpu.registers.a = 0x80;
    cpu.memory.write(0x0001, 0x00);
    cmp(&mut cpu, AddressingMode::Immediate);
    assert_eq!(cpu.get_flag(Flag::Carry), true);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), true);
}