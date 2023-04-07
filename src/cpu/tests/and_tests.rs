use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::flag::Flag;
use crate::cpu::function::and;
use crate::cpu::tests::get_cpu;

#[test]
fn test_and_immediate() {
    let mut cpu = get_cpu();
    cpu.registers.a = 0b0100_1010;
    let value = 0b1010_1101;

    cpu.memory.write(0x0001, value);
    and(&mut cpu, AddressingMode::Immediate);

    let expected = 0b0100_1010 & 0b1010_1101;
    assert_eq!(cpu.registers.a, expected);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), false);
}

#[test]
fn test_and_zero_page() {
    let mut cpu = get_cpu();
    cpu.registers.a = 0b1100_1010;
    let value = 0b1010_1101;

    cpu.memory.write(0x0020, value);
    cpu.memory.write(0x0001, 0x20);
    and(&mut cpu, AddressingMode::ZeroPage);

    let expected = 0b1100_1010 & 0b1010_1101;
    assert_eq!(cpu.registers.a, expected);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), true);
}

#[test]
fn test_and_zero_page_x() {
    let mut cpu = get_cpu();
    cpu.registers.a = 0b1100_1010;
    cpu.registers.x = 0x04;
    let value = 0b1010_1101;

    cpu.memory.write(0x0024, value);
    cpu.memory.write(0x0001, 0x20);
    and(&mut cpu, AddressingMode::ZeroPageX);

    let expected = 0b1100_1010 & 0b1010_1101;
    assert_eq!(cpu.registers.a, expected);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), true);
}

#[test]
fn test_and_absolute() {
    let mut cpu = get_cpu();
    cpu.registers.a = 0b1100_1010;
    let value = 0b0010_1101;

    cpu.memory.write(0x1000, value);
    cpu.memory.write(0x0001, 0x00);
    cpu.memory.write(0x0002, 0x10);
    and(&mut cpu, AddressingMode::Absolute);

    let expected = 0b1100_1010 & 0b0010_1101;
    assert_eq!(cpu.registers.a, expected);
    assert_eq!(cpu.get_flag(Flag::Zero), false);
    assert_eq!(cpu.get_flag(Flag::Negative), false);
}
