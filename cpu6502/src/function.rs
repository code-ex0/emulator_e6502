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

///
/// # ADC
/// this is the add with carry instruction,
/// it adds the value of the memory location to the accumulator and the carry bit,
/// if overflow occurs the carry bit is set, this enables multiple byte addition
/// and sets the carry, zero, negative and overflow flags as appropriate
///
/// # Arguments
/// * `cpu` - the cpu
/// * `mode` - the addressing mode
///
/// # Example
/// ```
/// use cpu6502::Cpu6502;
/// use cpu6502::instruction::AddressingMode;
/// use cpu6502::function::adc;
/// use cpu6502::Ram;
/// use cpu6502::Flag;
/// let mut cpu = Cpu6502::new(Ram::new());
/// cpu.a = 0x01;
/// cpu.set_flag(Flag::Carry, true);
/// adc(&mut cpu, AddressingMode::Immediate);
/// assert_eq!(cpu.a, 0x02);
/// ```
/// # Flags
/// * `Carry` - set if overflow occurs
/// * `Zero` - set if result is zero
/// * `Negative` - set if bit 7 of the result is set
/// * `Overflow` - set if sign bit is different between accumulator and memory
///
/// # Addressing Mode
/// all addressing modes are supported with this instruction
/// * `Immediate` - add value to accumulator
/// * `ZeroPage` - add value at address to accumulator
/// * `ZeroPageX` - add value at address + x to accumulator
/// * `Absolute` - add value at address to accumulator
/// * `AbsoluteX` - add value at address + x to accumulator
/// * `AbsoluteY` - add value at address + y to accumulator
/// * `IndirectX` - add value at address + x to accumulator
/// * `IndirectY` - add value at address + y to accumulator
///
///
/// # See
/// * [https://www.masswerk.at/6502/6502_instruction_set.html#ADC](https://www.masswerk.at/6502/6502_instruction_set.html#ADC)
///
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

///
/// # BRK
/// this is the break instruction,
/// it forces the generation of an interrupt request and sets the break flag to one,
/// the program counter and processor status are pushed on the stack
/// and sets the break and interrupt flags as appropriate
///
/// # Arguments
/// * `cpu` - the cpu
/// * `mode` - the addressing mode
///
/// # Example
/// ```
/// use cpu6502::Cpu6502;
/// use cpu6502::instruction::AddressingMode;
/// use cpu6502::function::brk;
/// use cpu6502::Ram;
/// use cpu6502::Flag;
/// let mut cpu = Cpu6502::new(Ram::new());
/// cpu.pc = 0x1234;
/// cpu.set_flag(Flag::Break, false);
/// brk(&mut cpu, AddressingMode::Implied);
/// assert_eq!(cpu.pc, 0xFFFE);
/// assert_eq!(cpu.get_flag(Flag::Break), true);
/// ```
///
/// # Flags
/// * `Break` - set to one
/// * `Interrupt` - set to one
///
/// # Addressing Mode
/// all addressing modes are supported with this instruction
/// * `Implied` - break
///
/// # See
/// * [https://www.masswerk.at/6502/6502_instruction_set.html#BRK](https://www.masswerk.at/6502/6502_instruction_set.html#BRK)
///
/// # Note
/// * needs to be tested
pub fn brk(cpu: &mut Cpu6502, _mode: AddressingMode) {
    cpu.push_word_stack(cpu.pc);
    cpu.push_stack(cpu.status | Flag::Break as Byte);
    cpu.set_flag(Flag::Interrupt, true);
    cpu.pc = cpu.read_word(0xFFFE);
}

///
/// # CMP
/// this is the compare instruction,
/// it compares the contents of the accumulator with another memory held value
/// and sets the zero, negative and carry flags as appropriate
/// # Arguments
/// * `cpu` - the cpu
/// * `mode` - the addressing mode
///
/// # Example
/// ```
/// use cpu6502::Cpu6502;
/// use cpu6502::instruction::AddressingMode;
/// use cpu6502::function::cmp;
/// use cpu6502::Ram;
/// use cpu6502::Flag;
/// let mut cpu = Cpu6502::new(Ram::new());
/// cpu.a = 0x01;
/// cmp(&mut cpu, AddressingMode::Immediate);
/// assert_eq!(cpu.get_flag(Flag::Carry), true);
/// assert_eq!(cpu.get_flag(Flag::Zero), false);
/// assert_eq!(cpu.get_flag(Flag::Negative), false);
/// ```
///
/// # Flags
/// * `Carry` - set if accumulator is greater than or equal to the value
/// * `Zero` - set if accumulator is equal to the value
/// * `Negative` - set if bit 7 of the result is set
///
/// # Addressing Mode
/// all addressing modes are supported with this instruction
/// * `Immediate` - compare accumulator with value
/// * `ZeroPage` - compare accumulator with value at address
/// * `ZeroPageX` - compare accumulator with value at address + x
/// * `Absolute` - compare accumulator with value at address
/// * `AbsoluteX` - compare accumulator with value at address + x
/// * `AbsoluteY` - compare accumulator with value at address + y
/// * `IndirectX` - compare accumulator with value at address + x
/// * `IndirectY` - compare accumulator with value at address + y
///
/// # See
///
/// * [https://www.masswerk.at/6502/6502_instruction_set.html#CMP](https://www.masswerk.at/6502/6502_instruction_set.html#CMP)
///
pub fn cmp(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let value = cpu.read_byte(address);
    let result = cpu.a.wrapping_sub(value);
    cpu.set_flag(Flag::Carry, cpu.a >= value);
    cpu.set_flag(Flag::Zero, result == 0);
    cpu.set_flag(Flag::Negative, result & 0x80 != 0);
}

///
/// # DEC
/// this is the decrement memory instruction
/// it subtracts one from the value held at a specified memory location
/// and sets the zero and carry flags as appropriate
/// # Arguments
/// * `cpu` - the cpu
/// * `mode` - the addressing mode
/// # Example
/// ```
/// use cpu6502::Cpu6502;
/// use cpu6502::instruction::AddressingMode;
/// use cpu6502::function::dec;
/// use cpu6502::Ram;
/// let mut cpu = Cpu6502::new(Ram::new());
/// cpu.write_byte(0x1234, 0x04);
/// dec(&mut cpu, AddressingMode::Absolute);
/// assert_eq!(cpu.read_byte(0x1234), 0x03);
/// ```
/// # Flags
/// * `Zero` - set if result is zero
/// * `Negative` - set if bit 7 of the result is set
/// # Addressing Mode
/// all addressing modes are supported with this instruction
/// * `ZeroPage` - decrement value at address
/// * `ZeroPageX` - decrement value at address + x
/// * `Absolute` - decrement value at address
/// * `AbsoluteX` - decrement value at address + x
/// * `IndirectX` - decrement value at address + x
/// * `IndirectY` - decrement value at address + y
/// # See
/// * [https://www.masswerk.at/6502/6502_instruction_set.html#DEC](https://www.masswerk.at/6502/6502_instruction_set.html#DEC)
/// # Note
/// * needs to be tested
pub fn dec(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let value = cpu.read_byte(address).wrapping_sub(1);
    cpu.write_byte(address, value);
    cpu.set_flag(Flag::Zero, value == 0);
    cpu.set_flag(Flag::Negative, value & 0x80 != 0);
}

///
/// # EOR
/// this is the exclusive or instruction,
/// it operates by exclusive or'ing the accumulator contents with another memory held value
/// and then storing the result in the accumulator
/// and sets the zero and negative flags as appropriate
/// # Arguments
/// * `cpu` - the cpu
/// * `mode` - the addressing mode
/// # Example
/// ```
/// use cpu6502::Cpu6502;
/// use cpu6502::instruction::AddressingMode;
/// use cpu6502::function::eor;
/// use cpu6502::Ram;
/// let mut cpu = Cpu6502::new(Ram::new());
/// cpu.a = 0x01;
/// eor(&mut cpu, AddressingMode::Immediate);
/// assert_eq!(cpu.a, 0x02);
/// ```
/// # Flags
/// * `Zero` - set if result is zero
/// * `Negative` - set if bit 7 of the result is set
/// # See
/// * [https://www.masswerk.at/6502/6502_instruction_set.html#EOR](https://www.masswerk.at/6502/6502_instruction_set.html#EOR)
/// # Note
/// * needs to be tested
pub fn eor(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let value = cpu.read_byte(address);
    cpu.a ^= value;
    cpu.set_flag(Flag::Zero, cpu.a == 0);
    cpu.set_flag(Flag::Negative, cpu.a & 0x80 != 0);
}

///
/// # INC
/// this is the increment memory instruction,
/// it adds one to the value held at a specified memory location
/// and sets the zero and negative flags as appropriate
/// # Arguments
/// * `cpu` - the cpu
/// * `mode` - the addressing mode
/// # Example
/// ```
/// use cpu6502::Cpu6502;
/// use cpu6502::instruction::AddressingMode;
/// use cpu6502::function::inc;
/// use cpu6502::Ram;
/// let mut cpu = Cpu6502::new(Ram::new());
/// cpu.write_byte(0x1234, 0x04);
/// inc(&mut cpu, AddressingMode::Absolute);
/// assert_eq!(cpu.read_byte(0x1234), 0x05);
/// ```
/// # Flags
/// * `Zero` - set if result is zero
/// * `Negative` - set if bit 7 of the result is set
/// # Addressing Mode
/// all addressing modes are supported with this instruction
/// * `ZeroPage` - increment value at address
/// * `ZeroPageX` - increment value at address + x
/// * `Absolute` - increment value at address
/// * `AbsoluteX` - increment value at address + x
/// # See
/// * [https://www.masswerk.at/6502/6502_instruction_set.html#INC](https://www.masswerk.at/6502/6502_instruction_set.html#INC)
/// # Note
/// * needs to be tested
pub fn inc(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let value = cpu.read_byte(address).wrapping_add(1);
    cpu.write_byte(address, value);
    cpu.set_flag(Flag::Zero, value == 0);
    cpu.set_flag(Flag::Negative, value & 0x80 != 0);
}

///
/// # JMP
/// this is the jump instruction,
/// it sets the program counter to the address specified by the operand
/// # Arguments
/// * `cpu` - the cpu
/// * `mode` - the addressing mode
/// # Example
/// ```
/// use cpu6502::Cpu6502;
/// use cpu6502::instruction::AddressingMode;
/// use cpu6502::Ram;
/// let mut cpu = Cpu6502::new(Ram::new());
/// cpu.pc = 0x1234;
/// jmp(&mut cpu, AddressingMode::Absolute);
/// assert_eq!(cpu.pc, 0x5678);
/// ```
/// # Addressing Mode
/// all addressing modes are supported with this instruction
/// * `Absolute` - jump to address
/// * `Indirect` - jump to address
/// # See
/// * [https://www.masswerk.at/6502/6502_instruction_set.html#JMP](https://www.masswerk.at/6502/6502_instruction_set.html#JMP)
/// # Note
/// * needs to be tested
pub fn jmp(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    cpu.pc = address;
}

///
/// # LDA
/// this is the load accumulator instruction,
/// it loads a byte of memory into the accumulator setting the zero and negative flags as appropriate
///
///
pub fn lda(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    cpu.a = cpu.read_byte(address);
    cpu.set_flag(Flag::Zero, cpu.a == 0);
    cpu.set_flag(Flag::Negative, cpu.a & 0x80 != 0);
}

///
/// ldx
/// this is the load x register instruction
/// it loads a byte of memory into the x register setting the zero and negative flags as appropriate
///
pub fn ldx(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    cpu.x = cpu.read_byte(address);
    cpu.set_flag(Flag::Zero, cpu.x == 0);
    cpu.set_flag(Flag::Negative, cpu.x & 0x80 != 0);
}

///
/// ldy
/// this is the load y register instruction
/// it loads a byte of memory into the y register setting the zero and negative flags as appropriate
///
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
// todo
}

pub fn ror(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn rti(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn rts(cpu: &mut Cpu6502, mode: AddressingMode) {
    cpu.pc = cpu.pop_word_stack();
    cpu.pc += 1;
}

pub fn sbc(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn sta(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    cpu.write_byte(address, cpu.a);
}

pub fn stx(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn sty(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn tax(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn tay(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn tsx(cpu: &mut Cpu6502, mode: AddressingMode) {
    let value = cpu.sp;
    cpu.x = value;
    cpu.pc += 1;
    cpu.set_flag(Flag::Zero, value == 0);
    cpu.set_flag(Flag::Negative, value & 0x80 != 0);
}

pub fn txa(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn txs(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn tya(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn bcc(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn bcs(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn beq(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn bmi(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn bne(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn bpl(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn bvc(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn bvs(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn clc(cpu: &mut Cpu6502, mode: AddressingMode) {
    cpu.set_flag(Flag::Carry, false);
    cpu.pc += 1;
}

pub fn cld(cpu: &mut Cpu6502, mode: AddressingMode) {
    cpu.set_flag(Flag::Decimal, false);
    cpu.pc += 1;
}

pub fn cli(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn clv(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn sec(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn sed(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn sei(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn nop(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn bit(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn kil(cpu: &mut Cpu6502, mode: AddressingMode) {
    exit(1)
}

pub fn lax(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn sax(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn dcp(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn isc(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn rla(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn rra(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn slo(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn sre(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn anc(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn alr(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn arr(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn xaa(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn axs(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn ahx(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn asl(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn jsr(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    cpu.push_word_stack(cpu.pc);
    cpu.pc = address;
}

pub fn and(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn tas(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn shx(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn shy(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn las(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn cpy(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn cpx(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn iny(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn inx(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn dey(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn dex(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

pub fn asr(cpu: &mut Cpu6502, mode: AddressingMode) {
// todo
}

