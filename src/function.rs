#![allow(arithmetic_overflow, unused)]

use std::process::exit;
use crate::{
    Byte,
    Word,
    Flag,
    Address,
    Cpu6502,
};
use crate::instruction::AddressingMode;

pub fn adc(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let value = cpu.read_byte(address);
    let result = cpu.a + value + cpu.get_flag(Flag::Carry) as Byte;
    // check for overflow
    cpu.set_flag(Flag::Carry, result < cpu.a);
    cpu.set_flag(Flag::Zero, result == 0);
    cpu.set_flag(Flag::Negative, result & 0x80 != 0);
    cpu.set_flag(Flag::Overflow, (cpu.a ^ result) & (value ^ result) & 0x80 != 0);
    cpu.a = result;
}

pub fn brk(cpu: &mut Cpu6502, _mode: AddressingMode) {
    cpu.push_word_stack(cpu.pc);
    cpu.push_stack(cpu.status | Flag::Break as Byte);
    cpu.set_flag(Flag::Interrupt, true);
    cpu.pc = cpu.read_word(0xFFFE);
}

pub fn cmp(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let value = cpu.read_byte(address);
    let result = cpu.a.wrapping_sub(value);
    cpu.set_flag(Flag::Carry, cpu.a >= value);
    cpu.set_flag(Flag::Zero, result == 0);
    cpu.set_flag(Flag::Negative, result & 0x80 != 0);
}

pub fn dec(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let value = cpu.read_byte(address).wrapping_sub(1);
    cpu.write_byte(address, value);
    cpu.set_flag(Flag::Zero, value == 0);
    cpu.set_flag(Flag::Negative, value & 0x80 != 0);
}

pub fn eor(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let value = cpu.read_byte(address);
    cpu.a ^= value;
    cpu.set_flag(Flag::Zero, cpu.a == 0);
    cpu.set_flag(Flag::Negative, cpu.a & 0x80 != 0);
}

pub fn inc(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let value = cpu.read_byte(address).wrapping_add(1);
    cpu.write_byte(address, value);
    cpu.set_flag(Flag::Zero, value == 0);
    cpu.set_flag(Flag::Negative, value & 0x80 != 0);
}

pub fn jmp(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    cpu.pc = address;
}

pub fn lda(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    cpu.a = cpu.read_byte(address);
    cpu.set_flag(Flag::Zero, cpu.a == 0);
    cpu.set_flag(Flag::Negative, cpu.a & 0x80 != 0);
}

pub fn ldx(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    cpu.x = cpu.read_byte(address);
    cpu.set_flag(Flag::Zero, cpu.x == 0);
    cpu.set_flag(Flag::Negative, cpu.x & 0x80 != 0);
}

pub fn ldy(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    cpu.y = cpu.read_byte(address);
    cpu.set_flag(Flag::Zero, cpu.y == 0);
    cpu.set_flag(Flag::Negative, cpu.y & 0x80 != 0);
}

pub fn lsr(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let value = cpu.read_byte(address);
    cpu.set_flag(Flag::Carry, value & 0x01 != 0);
    let result = value >> 1;
    cpu.write_byte(address, result);
    cpu.set_flag(Flag::Zero, result == 0);
    cpu.set_flag(Flag::Negative, result & 0x80 != 0);

}

pub fn ora(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let value = cpu.read_byte(address);
    cpu.a |= value;
    cpu.set_flag(Flag::Zero, cpu.a == 0);
    cpu.set_flag(Flag::Negative, cpu.a & 0x80 != 0);
}

pub fn pha(cpu: &mut Cpu6502, _mode: AddressingMode) {
    cpu.push_stack(cpu.a);
    cpu.pc += 1
}

pub fn php(cpu: &mut Cpu6502, _mode: AddressingMode) {
    cpu.push_stack(cpu.status | Flag::Break as Byte);
    cpu.pc += 1
}

pub fn pla(cpu: &mut Cpu6502, _mode: AddressingMode) {
    cpu.a = cpu.pop_stack();
    cpu.set_flag(Flag::Zero, cpu.a == 0);
    cpu.set_flag(Flag::Negative, cpu.a & 0x80 != 0);
    cpu.pc += 1
}

pub fn plp(cpu: &mut Cpu6502, _mode: AddressingMode) {
    cpu.status = cpu.pop_stack();
    cpu.pc += 1
}

pub fn rol(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn ror(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn rti(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn rts(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn sbc(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn sta(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    cpu.write_byte(address, cpu.a);
}

pub fn stx(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn sty(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn tax(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn tay(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn tsx(cpu: &mut Cpu6502, mode: AddressingMode) {
    let value = cpu.sp;
    cpu.x = value;
    cpu.pc += 1;
    cpu.set_flag(Flag::Zero, value == 0);
    cpu.set_flag(Flag::Negative, value & 0x80 != 0);
}

pub fn txa(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn txs(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn tya(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn bcc(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn bcs(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn beq(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn bmi(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn bne(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn bpl(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn bvc(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn bvs(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn clc(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn cld(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn cli(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn clv(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn sec(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn sed(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn sei(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn nop(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn bit(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn kil(cpu: &mut Cpu6502, mode: AddressingMode) {
    exit(1)
}

pub fn lax(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn sax(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn dcp(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn isc(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn rla(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn rra(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn slo(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn sre(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn anc(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn alr(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn arr(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn xaa(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn axs(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn ahx(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn asl(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn jsr(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn and(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn tas(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn shx(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn shy(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn las(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn cpy(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn cpx(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn iny(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn inx(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn dey(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn dex(cpu: &mut Cpu6502, mode: AddressingMode) {

}

pub fn asr(cpu: &mut Cpu6502, mode: AddressingMode) {

}

