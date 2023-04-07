use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::function::asl;
use crate::cpu::tests::get_cpu;
use crate::cpu::flag::Flag;

#[test]
fn test_asl_accumulator() {
    let mut cpu = get_cpu();
    cpu.registers.a = 0b0101_1010;

    asl(&mut cpu, AddressingMode::Accumulator);

    let expected = 0b1011_0100;
    assert_eq!(cpu.registers.a, expected);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), true);
    assert_eq!(cpu.get_flag(Flag::Carry), false);
}

#[test]
fn test_asl_zero_page() {
    let mut cpu = get_cpu();
    let value = 0b0101_1010;

    cpu.memory.write(0x0020, value);
    cpu.memory.write(0x0001, 0x20);
    asl(&mut cpu, AddressingMode::ZeroPage);

    let expected = 0b1011_0100;
    assert_eq!(cpu.read_byte(0x0020), expected);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), true);
    assert_eq!(cpu.get_flag(Flag::Carry), false);
}

#[test]
fn test_asl_zero_page_x() {
    let mut cpu = get_cpu();
    cpu.registers.x = 0x04;
    let value = 0b0101_1010;

    cpu.memory.write(0x0024, value);
    cpu.memory.write(0x0001, 0x20);
    asl(&mut cpu, AddressingMode::ZeroPageX);

    let expected = 0b1011_0100;
    assert_eq!(cpu.read_byte(0x0024), expected);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), true);
    assert_eq!(cpu.get_flag(Flag::Carry), false);
}

#[test]
fn test_asl_absolute() {
    let mut cpu = get_cpu();
    let value = 0b0101_1010;

    cpu.memory.write(0x1000, value);
    cpu.memory.write(0x0001, 0x00);
    cpu.memory.write(0x0002, 0x10);
    asl(&mut cpu, AddressingMode::Absolute);

    let expected = 0b1011_0100;
    assert_eq!(cpu.read_byte(0x1000), expected);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), true);
    assert_eq!(cpu.get_flag(Flag::Carry), false);
}
