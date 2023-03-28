use super::*;

#[test]
fn test_brk() {
    let mut cpu = get_cpu();

    cpu.registers.pc = 0x1000;
    cpu.registers.status = 0b11111111;
    cpu.memory.write(0xFFFE, 0x10);
    cpu.memory.write(0xFFFF, 0x10);
    brk(&mut cpu, AddressingMode::Implied);
    assert_eq!(cpu.registers.pc, 0x1010);
    assert_eq!(cpu.get_flag(Flag::Break), true);
    assert_eq!(cpu.get_flag(Flag::Interrupt), true);
    assert_eq!(cpu.memory.read(0x01FF), 0x10); // Changed this line
    assert_eq!(cpu.memory.read(0x01FE), 0x00); // Changed this line
    assert_eq!(cpu.memory.read(0x01FD), 0b11111111); // Changed this line
}