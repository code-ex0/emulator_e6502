use super::*;

#[test]
fn test_lda_immediate() {
    let mut cpu = get_cpu();

    cpu.memory.write(0x0001, 0x01);
    lda(&mut cpu, AddressingMode::Immediate);
    assert_eq!(cpu.registers.a, 0x01);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), false);
}

#[test]
fn test_lda_zero_page() {
    let mut cpu = get_cpu();

    cpu.memory.write(0x0001, 0x01);
    cpu.memory.write(0x0002, 0x01);
    lda(&mut cpu, AddressingMode::ZeroPage);
    assert_eq!(cpu.registers.a, 0x01);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), false);
}

#[test]
fn test_lda_zero_page_x() {
    let mut cpu = get_cpu();

    cpu.memory.write(0x0001, 0x01);
    cpu.memory.write(0x0002, 0x01);
    cpu.registers.x = 0x01;
    lda(&mut cpu, AddressingMode::ZeroPageX);
    assert_eq!(cpu.registers.a, 0x01);
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
    assert_eq!(cpu.registers.a, 0x01);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), false);
}

#[test]
fn test_lda_absolute_x() {
    let mut cpu = get_cpu();

    cpu.memory.write(0x0001, 0x01);
    cpu.memory.write(0x0002, 0x01);
    cpu.memory.write(0x0102, 0x01);
    cpu.registers.x = 0x01;
    lda(&mut cpu, AddressingMode::AbsoluteX);
    assert_eq!(cpu.registers.a, 0x01);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), false);
}

#[test]
fn test_lda_absolute_y() {
    let mut cpu = get_cpu();

    cpu.memory.write(0x0001, 0x01);
    cpu.memory.write(0x0002, 0x01);
    cpu.memory.write(0x0102, 0x01);
    cpu.registers.y = 0x01;
    lda(&mut cpu, AddressingMode::AbsoluteY);
    assert_eq!(cpu.registers.a, 0x01);
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
    cpu.registers.x = 0x01;
    lda(&mut cpu, AddressingMode::IndirectX);
    assert_eq!(cpu.registers.a, 0x01);
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
    cpu.registers.y = 0x01;
    lda(&mut cpu, AddressingMode::IndirectY);
    assert_eq!(cpu.registers.a, 0x01);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), false);
}

#[test]
fn test_lda_negative() {
    let mut cpu = get_cpu();

    cpu.memory.write(0x0001, 0x80);
    lda(&mut cpu, AddressingMode::Immediate);
    assert_eq!(cpu.registers.a, 0x80);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), true);
}

#[test]
fn test_lda_zero() {
    let mut cpu = get_cpu();

    cpu.memory.write(0x0001, 0x00);
    lda(&mut cpu, AddressingMode::Immediate);
    assert_eq!(cpu.registers.a, 0x00);
    assert_eq!(cpu.get_flag(Flag::Zero), true);
    assert_eq!(cpu.get_flag(Flag::Negative), false);
}