use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::flag::Flag;
use crate::cpu::function::ldy;
use super::*;

#[test]
fn test_ldy_immediate() {
    let mut cpu = get_cpu();

    cpu.memory.write(0x0001, 0x01);
    ldy(&mut cpu, AddressingMode::Immediate);
    assert_eq!(cpu.registers.y, 0x01);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), false);
}

#[test]
fn test_ldy_zero_page() {
    let mut cpu = get_cpu();

    cpu.memory.write(0x0001, 0x01);
    cpu.memory.write(0x0002, 0x01);
    ldy(&mut cpu, AddressingMode::ZeroPage);
    assert_eq!(cpu.registers.y, 0x01);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), false);
}

#[test]
fn test_ldy_zero_page_x() {
    let mut cpu = get_cpu();

    cpu.memory.write(0x0001, 0x01);
    cpu.memory.write(0x0002, 0x01);
    cpu.registers.x = 0x01;
    ldy(&mut cpu, AddressingMode::ZeroPageX);
    assert_eq!(cpu.registers.y, 0x01);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), false);
}

#[test]
fn test_ldy_zero_page_y() {
    let mut cpu = get_cpu();

    cpu.memory.write(0x0001, 0x01);
    cpu.memory.write(0x0002, 0x01);
    cpu.registers.y = 0x01;
    ldy(&mut cpu, AddressingMode::ZeroPageY);
    assert_eq!(cpu.registers.y, 0x01);
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
    assert_eq!(cpu.registers.y, 0x01);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), false);
}

#[test]
fn test_ldy_absolute_x() {
    let mut cpu = get_cpu();

    cpu.memory.write(0x0001, 0x01);
    cpu.memory.write(0x0002, 0x01);
    cpu.memory.write(0x0102, 0x01);
    cpu.registers.x = 0x01;
    ldy(&mut cpu, AddressingMode::AbsoluteX);
    assert_eq!(cpu.registers.y, 0x01);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), false);
}

#[test]
fn test_ldy_absolute_y() {
    let mut cpu = get_cpu();

    cpu.memory.write(0x0001, 0x01);
    cpu.memory.write(0x0002, 0x01);
    cpu.memory.write(0x0102, 0x01);
    cpu.registers.y = 0x01;
    ldy(&mut cpu, AddressingMode::AbsoluteY);
    assert_eq!(cpu.registers.y, 0x01);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), false);
}

#[test]
fn test_ldy_zero() {
    let mut cpu = get_cpu();

    cpu.memory.write(0x0001, 0x00);
    ldy(&mut cpu, AddressingMode::Immediate);
    assert_eq!(cpu.registers.y, 0x00);
    assert_eq!(cpu.get_flag(Flag::Zero), true);
    assert_eq!(cpu.get_flag(Flag::Negative), false);
}

#[test]
fn test_ldy_negative() {
    let mut cpu = get_cpu();

    cpu.memory.write(0x0001, 0x80);
    ldy(&mut cpu, AddressingMode::Immediate);
    assert_eq!(cpu.registers.y, 0x80);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), true);
}