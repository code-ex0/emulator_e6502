use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::flag::Flag;
use crate::cpu::function::dec;
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