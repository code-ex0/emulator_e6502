///
/// File: cpu/function.rs
/// The function module contains the implementation of the cpu functions.
///

use crate::cpu::{
    flag::Flag,
    addressing_mode::AddressingMode,
    cpu_6502::Cpu6502
};
use crate::util::types::{Byte};

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
/// # Exemple
/// ```
/// use emul::cpu::addressing_mode::AddressingMode;
/// use emul::cpu::cpu_6502::Cpu6502;
/// use emul::cpu::flag::Flag;
/// use emul::cpu::function::adc;
/// use emul::memory::ram::Ram;
///
/// let mut cpu = Cpu6502::new(Ram::new());
/// cpu.registers.a = 0x01;
/// cpu.set_flag(Flag::Carry, true);
/// adc(&mut cpu, AddressingMode::Immediate);
/// assert_eq!(cpu.registers.a, 0x02);
/// assert_eq!(cpu.get_flag(Flag::Carry), false);
/// assert_eq!(cpu.get_flag(Flag::Zero), false);
/// assert_eq!(cpu.get_flag(Flag::Negative), false);
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
    let carry = cpu.get_flag(Flag::Carry) as Byte;
    let result = cpu.registers.a.wrapping_add(value).wrapping_add(carry);

    cpu.set_flag(Flag::Carry, result < cpu.registers.a || result < value);
    cpu.set_flag(Flag::Zero, result == 0);
    cpu.set_flag(Flag::Negative, result & 0x80 != 0);
    cpu.set_flag(Flag::Overflow, (cpu.registers.a ^ result) & (value ^ result) & 0x80 != 0);

    cpu.registers.a = result;
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
/// use emul::cpu::addressing_mode::AddressingMode;
/// use emul::cpu::cpu_6502::Cpu6502;
/// use emul::cpu::flag::Flag;
/// use emul::memory::ram::Ram;
/// use emul::cpu::function::brk;
///
/// let mut cpu = Cpu6502::new(Ram::new());
/// cpu.reset();
/// cpu.registers.pc = 0x1000;
/// cpu.registers.status = 0b11111111;
/// cpu.memory.write(0xFFFE, 0x10);
/// cpu.memory.write(0xFFFF, 0x10);
/// brk(&mut cpu, AddressingMode::Implied);
/// assert_eq!(cpu.registers.pc, 0x1010);
/// assert_eq!(cpu.get_flag(Flag::Break), true);
/// assert_eq!(cpu.get_flag(Flag::Interrupt), true);
/// assert_eq!(cpu.memory.read(0x01FF), 0x10);
/// assert_eq!(cpu.memory.read(0x01FE), 0x00);
/// assert_eq!(cpu.memory.read(0x01FD), 0b11111111);
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
    cpu.push_word_stack(cpu.registers.pc);
    cpu.push_stack(cpu.registers.status | Flag::Break as Byte);
    cpu.set_flag(Flag::Interrupt, true);
    cpu.registers.pc = cpu.read_word(0xFFFE);
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
/// use emul::cpu::addressing_mode::AddressingMode;
/// use emul::cpu::cpu_6502::Cpu6502;
/// use emul::cpu::flag::Flag;
/// use emul::memory::ram::Ram;
/// use emul::cpu::function::cmp;
///
/// let mut cpu = Cpu6502::new(Ram::new());
/// cpu.registers.a = 0x01;
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
    let result = cpu.registers.a.wrapping_sub(value);
    cpu.set_flag(Flag::Carry, cpu.registers.a >= value);
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
/// use emul::cpu::addressing_mode::AddressingMode;
/// use emul::cpu::cpu_6502::Cpu6502;
/// use emul::cpu::flag::Flag;
/// use emul::memory::ram::Ram;
/// use emul::cpu::function::dec;
///
/// let mut cpu = Cpu6502::new(Ram::new());
/// cpu.write_byte(0x0000, 0x04);
/// dec(&mut cpu, AddressingMode::Absolute);
/// assert_eq!(cpu.read_byte(0x0000), 0x03);
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
///
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
/// use emul::cpu::addressing_mode::AddressingMode;
/// use emul::cpu::cpu_6502::Cpu6502;
/// use emul::cpu::flag::Flag;
/// use emul::memory::ram::Ram;
/// use emul::cpu::function::eor;
///
/// let mut cpu = Cpu6502::new(Ram::new());
/// cpu.registers.a = 0b0000_0001;
/// cpu.memory.write(0x0000, 0b0000_0011);
/// eor(&mut cpu, AddressingMode::Absolute);
/// assert_eq!(cpu.registers.a, 0b0000_0010);
/// ```
/// # Flags
/// * `Zero` - set if result is zero
/// * `Negative` - set if bit 7 of the result is set
/// # Addressing Mode
/// all addressing modes are supported with this instruction
/// * `Immediate` - exclusive or accumulator with value
/// * `ZeroPage` - exclusive or accumulator with value at address
/// * `ZeroPageX` - exclusive or accumulator with value at address + x
/// * `Absolute` - exclusive or accumulator with value at address
/// * `AbsoluteX` - exclusive or accumulator with value at address + x
/// * `AbsoluteY` - exclusive or accumulator with value at address + y
/// * `IndirectX` - exclusive or accumulator with value at address + x
/// * `IndirectY` - exclusive or accumulator with value at address + y
/// # See
/// * [https://www.masswerk.at/6502/6502_instruction_set.html#EOR](https://www.masswerk.at/6502/6502_instruction_set.html#EOR)
pub fn eor(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let value = cpu.read_byte(address);
    cpu.registers.a ^= value;
    cpu.set_flag(Flag::Zero, cpu.registers.a == 0);
    cpu.set_flag(Flag::Negative, cpu.registers.a & 0x80 != 0);
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
/// use emul::cpu::addressing_mode::AddressingMode;
/// use emul::cpu::cpu_6502::Cpu6502;
/// use emul::cpu::flag::Flag;
/// use emul::memory::ram::Ram;
/// use emul::cpu::function::inc;
///
/// let mut cpu = Cpu6502::new(Ram::new());
/// cpu.write_byte(0x0000, 0x04);
/// inc(&mut cpu, AddressingMode::Absolute);
/// assert_eq!(cpu.read_byte(0x0000), 0x05);
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
/// use emul::cpu::addressing_mode::AddressingMode;
/// use emul::cpu::cpu_6502::Cpu6502;
/// use emul::cpu::flag::Flag;
/// use emul::memory::ram::Ram;
/// use emul::cpu::function::jmp;
///
/// let mut cpu = Cpu6502::new(Ram::new());
/// cpu.write_byte(0x1235, 0x00);
/// cpu.write_byte(0x1236, 0x10);
/// cpu.registers.pc = 0x1234;
/// jmp(&mut cpu, AddressingMode::Absolute);
/// assert_eq!(cpu.registers.pc, 0x1000);
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
    cpu.registers.pc = address;
}

///
/// # LDA
/// this is the load accumulator instruction,
/// it loads a byte of memory into the accumulator setting the zero and negative flags as appropriate
/// # Arguments
/// * `cpu` - the cpu
/// * `mode` - the addressing mode
/// # Example
/// ```
/// use emul::cpu::addressing_mode::AddressingMode;
/// use emul::cpu::cpu_6502::Cpu6502;
/// use emul::cpu::flag::Flag;
/// use emul::memory::ram::Ram;
/// use emul::cpu::function::lda;
///
/// let mut cpu = Cpu6502::new(Ram::new());
/// cpu.write_byte(0x0000, 0x04);
/// lda(&mut cpu, AddressingMode::Absolute);
/// assert_eq!(cpu.registers.a, 0x04);
/// ```
/// # Flags
/// * `Zero` - set if result is zero
/// * `Negative` - set if bit 7 of the result is set
/// # Addressing Mode
/// all addressing modes are supported with this instruction
/// * `Immediate` - load value
/// * `ZeroPage` - load value at address
/// * `ZeroPageX` - load value at address + x
/// * `Absolute` - load value at address
/// * `AbsoluteX` - load value at address + x
/// * `AbsoluteY` - load value at address + y
/// * `IndirectX` - load value at address + x
/// * `IndirectY` - load value at address + y
/// # See
/// * [https://www.masswerk.at/6502/6502_instruction_set.html#LDA](https://www.masswerk.at/6502/6502_instruction_set.html#LDA)
///
///
pub fn lda(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    cpu.registers.a = cpu.read_byte(address);
    cpu.set_flag(Flag::Zero, cpu.registers.a == 0);
    cpu.set_flag(Flag::Negative, cpu.registers.a & 0x80 != 0);
}

///
/// # LDX
/// this is the load x register instruction
/// it loads a byte of memory into the x register setting the zero and negative flags as appropriate
/// # Arguments
/// * `cpu` - the cpu
/// * `mode` - the addressing mode
/// # Example
/// ```
/// use emul::cpu::addressing_mode::AddressingMode;
/// use emul::cpu::cpu_6502::Cpu6502;
/// use emul::cpu::flag::Flag;
/// use emul::memory::ram::Ram;
/// use emul::cpu::function::ldx;
///
/// let mut cpu = Cpu6502::new(Ram::new());
/// cpu.write_byte(0x0000, 0x04);
/// ldx(&mut cpu, AddressingMode::Absolute);
/// assert_eq!(cpu.registers.x, 0x04);
/// ```
/// # Flags
/// * `Zero` - set if result is zero
/// * `Negative` - set if bit 7 of the result is set
/// # Addressing Mode
/// all addressing modes are supported with this instruction
/// * `Immediate` - load value
/// * `ZeroPage` - load value at address
/// * `ZeroPageY` - load value at address + y
/// * `Absolute` - load value at address
/// * `AbsoluteY` - load value at address + y
/// # See
/// * [https://www.masswerk.at/6502/6502_instruction_set.html#LDX](https://www.masswerk.at/6502/6502_instruction_set.html#LDX)
///
pub fn ldx(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    cpu.registers.x = cpu.read_byte(address);
    cpu.set_flag(Flag::Zero, cpu.registers.x == 0);
    cpu.set_flag(Flag::Negative, cpu.registers.x & 0x80 != 0);
}

///
/// # LDY
/// this is the load y register instruction
/// it loads a byte of memory into the y register setting the zero and negative flags as appropriate
/// # Arguments
/// * `cpu` - the cpu
/// * `mode` - the addressing mode
/// # Example
/// ```
/// use emul::cpu::addressing_mode::AddressingMode;
/// use emul::cpu::cpu_6502::Cpu6502;
/// use emul::cpu::flag::Flag;
/// use emul::memory::ram::Ram;
/// use emul::cpu::function::ldy;
///
/// let mut cpu = Cpu6502::new(Ram::new());
/// cpu.write_byte(0x0000, 0x04);
/// ldy(&mut cpu, AddressingMode::Absolute);
/// assert_eq!(cpu.registers.y, 0x04);
/// ```
/// # Flags
/// * `Zero` - set if result is zero
/// * `Negative` - set if bit 7 of the result is set
/// # Addressing Mode
/// all addressing modes are supported with this instruction
/// * `Immediate` - load value
/// * `ZeroPage` - load value at address
/// * `ZeroPageX` - load value at address + x
/// * `Absolute` - load value at address
/// * `AbsoluteX` - load value at address + x
/// # See
/// * [https://www.masswerk.at/6502/6502_instruction_set.html#LDY](https://www.masswerk.at/6502/6502_instruction_set.html#LDY)
///
pub fn ldy(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    cpu.registers.y = cpu.read_byte(address);
    cpu.set_flag(Flag::Zero, cpu.registers.y == 0);
    cpu.set_flag(Flag::Negative, cpu.registers.y & 0x80 != 0);
}

///
/// # LSR
/// this is the logical shift right instruction
/// it shifts all the bits of the accumulator or memory contents one bit right
/// the bit that was in bit 0 is shifted into the carry flag
/// bit 7 is set to zero
/// # Arguments
/// * `cpu` - the cpu
/// * `mode` - the addressing mode
/// # Example
/// ```
/// use emul::cpu::addressing_mode::AddressingMode;
/// use emul::cpu::cpu_6502::Cpu6502;
/// use emul::cpu::flag::Flag;
/// use emul::memory::ram::Ram;
/// use emul::cpu::function::lsr;
///
/// let mut cpu = Cpu6502::new(Ram::new());
/// cpu.write_byte(0x0000, 0x04);
/// lsr(&mut cpu, AddressingMode::Absolute);
/// assert_eq!(cpu.read_byte(0x0000), 0x02);
/// ```
/// # Flags
/// * `Carry` - set if bit 0 of the value is set
/// * `Zero` - set if result is zero
/// * `Negative` - set if bit 7 of the result is set
/// # Addressing Mode
/// all addressing modes are supported with this instruction
/// * `Accumulator` - shift value in accumulator
/// * `ZeroPage` - shift value at address
/// * `ZeroPageX` - shift value at address + x
/// * `Absolute` - shift value at address
/// * `AbsoluteX` - shift value at address + x
/// # See
/// * [https://www.masswerk.at/6502/6502_instruction_set.html#LSR](https://www.masswerk.at/6502/6502_instruction_set.html#LSR)
///
pub fn lsr(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let value = cpu.read_byte(address);
    cpu.set_flag(Flag::Carry, value & 0x01 != 0);
    let result = value >> 1;
    cpu.write_byte(address, result);
    cpu.set_flag(Flag::Zero, result == 0);
    cpu.set_flag(Flag::Negative, result & 0x80 != 0);
}

///
/// # ORA
/// this is the bitwise or instruction
/// it performs a bitwise or between the accumulator and a byte of memory
/// storing the result in the accumulator
/// # Arguments
/// * `cpu` - the cpu
/// * `mode` - the addressing mode
/// # Example
/// ```
/// use emul::cpu::addressing_mode::AddressingMode;
/// use emul::cpu::cpu_6502::Cpu6502;
/// use emul::cpu::flag::Flag;
/// use emul::memory::ram::Ram;
/// use emul::cpu::function::ora;
///
/// let mut cpu = Cpu6502::new(Ram::new());
/// cpu.write_byte(0x0000, 0x04);
/// ora(&mut cpu, AddressingMode::Absolute);
/// assert_eq!(cpu.registers.a, 0x04);
/// ```
/// # Flags
/// * `Zero` - set if result is zero
/// * `Negative` - set if bit 7 of the result is set
/// # Addressing Mode
/// all addressing modes are supported with this instruction
/// * `Immediate` - bitwise or value
/// * `ZeroPage` - bitwise or value at address
/// * `ZeroPageX` - bitwise or value at address + x
/// * `Absolute` - bitwise or value at address
/// * `AbsoluteX` - bitwise or value at address + x
/// * `AbsoluteY` - bitwise or value at address + y
/// * `IndirectX` - bitwise or value at address + x
/// * `IndirectY` - bitwise or value at address + y
/// # See
/// * [https://www.masswerk.at/6502/6502_instruction_set.html#ORA](https://www.masswerk.at/6502/6502_instruction_set.html#ORA)
///
pub fn ora(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let value = cpu.read_byte(address);
    cpu.registers.a |= value;
    cpu.set_flag(Flag::Zero, cpu.registers.a == 0);
    cpu.set_flag(Flag::Negative, cpu.registers.a & 0x80 != 0);
}

///
/// # PHA
/// this is the push accumulator instruction
/// it pushes the accumulator onto the stack
/// # Arguments
/// * `cpu` - the cpu
/// * `mode` - the addressing mode
/// # Example
/// ```
/// use emul::cpu::addressing_mode::AddressingMode;
/// use emul::cpu::cpu_6502::Cpu6502;
/// use emul::cpu::flag::Flag;
/// use emul::memory::ram::Ram;
/// use emul::cpu::function::pha;
///
/// let mut cpu = Cpu6502::new(Ram::new());
/// cpu.registers.a = 0x04;
/// pha(&mut cpu, AddressingMode::Implied);
/// //assert_eq!(cpu.read_byte(0x01FF), 0x04);
/// ```
/// # Flags
/// * `None`
/// # Addressing Mode
/// * `Implied` - push accumulator
/// # See
/// * [https://www.masswerk.at/6502/6502_instruction_set.html#PHA](https://www.masswerk.at/6502/6502_instruction_set.html#PHA)
/// # Note
/// need to test this
pub fn pha(cpu: &mut Cpu6502, _mode: AddressingMode) {
    cpu.push_stack(cpu.registers.a);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
}

///
/// # PHP
/// this is the push processor status instruction
/// it pushes the processor status onto the stack
/// # Arguments
/// * `cpu` - the cpu
/// * `mode` - the addressing mode
/// # Example
/// ```
/// use emul::cpu::addressing_mode::AddressingMode;
/// use emul::cpu::cpu_6502::Cpu6502;
/// use emul::cpu::flag::Flag;
/// use emul::memory::ram::Ram;
/// use emul::cpu::function::php;
///
/// let mut cpu = Cpu6502::new(Ram::new());
/// cpu.registers.status = 0x04;
/// php(&mut cpu, AddressingMode::Implied);
/// //assert_eq!(cpu.read_byte(0x01FF), 0x04);
/// ```
/// # Flags
/// * `None`
/// # Addressing Mode
/// * `Implied` - push processor status
/// # See
/// * [https://www.masswerk.at/6502/6502_instruction_set.html#PHP](https://www.masswerk.at/6502/6502_instruction_set.html#PHP)
/// # Note
/// need to test this
pub fn php(cpu: &mut Cpu6502, _mode: AddressingMode) {
    cpu.push_stack(cpu.registers.status | Flag::Break as Byte);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
}

///
/// # PLA
/// this is the pull accumulator instruction
/// it pulls the accumulator from the stack
/// # Arguments
/// * `cpu` - the cpu
/// * `mode` - the addressing mode
/// # Example
/// ```
/// use emul::cpu::addressing_mode::AddressingMode;
/// use emul::cpu::cpu_6502::Cpu6502;
/// use emul::cpu::flag::Flag;
/// use emul::memory::ram::Ram;
/// use emul::cpu::function::pla;
///
/// let mut cpu = Cpu6502::new(Ram::new());
/// cpu.write_byte(0x01FF, 0x04);
/// pla(&mut cpu, AddressingMode::Implied);
/// //assert_eq!(cpu.registers.a, 0x04);
/// ```
/// # Flags
/// * `Zero` - set if result is zero
/// * `Negative` - set if bit 7 of the result is set
/// # Addressing Mode
/// * `Implied` - pull accumulator
/// # See
/// * [https://www.masswerk.at/6502/6502_instruction_set.html#PLA](https://www.masswerk.at/6502/6502_instruction_set.html#PLA)
/// # Note
/// need to test this
pub fn pla(cpu: &mut Cpu6502, _mode: AddressingMode) {
    cpu.registers.a = cpu.pop_stack();
    cpu.set_flag(Flag::Zero, cpu.registers.a == 0);
    cpu.set_flag(Flag::Negative, cpu.registers.a & 0x80 != 0);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
}

///
/// # PLP
/// this is the pull processor status instruction
/// it pulls the processor status from the stack
/// # Arguments
/// * `cpu` - the cpu
/// * `mode` - the addressing mode
/// # Example
/// ```
/// use emul::cpu::addressing_mode::AddressingMode;
/// use emul::cpu::cpu_6502::Cpu6502;
/// use emul::cpu::flag::Flag;
/// use emul::memory::ram::Ram;
/// use emul::cpu::function::plp;
///
/// let mut cpu = Cpu6502::new(Ram::new());
/// cpu.write_byte(0x01FF, 0x04);
/// plp(&mut cpu, AddressingMode::Implied);
/// //assert_eq!(cpu.registers.status, 0x04);
/// ```
/// # Flags
/// * `None`
/// # Addressing Mode
/// * `Implied` - pull processor status
/// # See
/// * [https://www.masswerk.at/6502/6502_instruction_set.html#PLP](https://www.masswerk.at/6502/6502_instruction_set.html#PLP)
/// # Note
/// need to test this
pub fn plp(cpu: &mut Cpu6502, _mode: AddressingMode) {
    cpu.registers.status = cpu.pop_stack();
    cpu.registers.pc += 1
}

///
/// # ROL
///
/// # Arguments
/// * `cpu` - the cpu
/// * `mode` - the addressing mode
/// # Example
/// ```
///
pub fn rol(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let mut value = cpu.read_byte(address);
    let carry = cpu.registers.status & Flag::Carry as Byte != 0;
    cpu.set_flag(Flag::Carry, value & 0x80 != 0);
    value <<= 1;
    if carry {
        value |= 1;
    }

    if let AddressingMode::Immediate = mode {
        cpu.registers.a = value;
    } else {
        cpu.write_byte(address, value);
    }

    cpu.set_flag(Flag::Zero, value == 0);
    cpu.set_flag(Flag::Negative, value & 0x80 != 0);
    cpu.registers.pc += 1;
}

pub fn ror(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let mut value = cpu.read_byte(address);
    let carry = cpu.registers.status & Flag::Carry as Byte != 0;
    cpu.set_flag(Flag::Carry, value & 0x01 != 0);
    value >>= 1;
    if carry {
        value |= 0x80;
    }

    if let AddressingMode::Immediate = mode {
        cpu.registers.a = value;
    } else {
        cpu.write_byte(address, value);
    }

    cpu.set_flag(Flag::Zero, value == 0);
    cpu.set_flag(Flag::Negative, value & 0x80 != 0);
    cpu.registers.pc += 1;
}

pub fn rti(cpu: &mut Cpu6502, _mode: AddressingMode) {
    cpu.registers.status = cpu.pop_stack();
    cpu.registers.pc = cpu.pop_word_stack();
}

pub fn rts(cpu: &mut Cpu6502, _mode: AddressingMode) {
    cpu.registers.pc = cpu.pop_word_stack();
    cpu.registers.pc += 1;
}

pub fn sbc(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let value = cpu.read_byte(address);
    let carry = cpu.registers.status & Flag::Carry as Byte != 0;
    let result = cpu.registers.a.wrapping_sub(value).wrapping_sub(!carry as Byte);
    cpu.set_flag(Flag::Carry, result <= cpu.registers.a);
    cpu.set_flag(Flag::Zero, result == 0);
    cpu.set_flag(Flag::Negative, result & 0x80 != 0);
    cpu.set_flag(Flag::Overflow, (cpu.registers.a ^ result) & (value ^ result) & 0x80 != 0);
    cpu.registers.a = result;
    cpu.registers.pc += 1;
}

pub fn sta(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    cpu.write_byte(address, cpu.registers.a);
}

pub fn stx(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    cpu.write_byte(address, cpu.registers.x);
}

pub fn sty(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    cpu.write_byte(address, cpu.registers.y);
}

pub fn tax(cpu: &mut Cpu6502, _mode: AddressingMode) {
    let value = cpu.registers.a;
    cpu.registers.x = value;
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    cpu.set_flag(Flag::Zero, value == 0);
    cpu.set_flag(Flag::Negative, value & 0x80 != 0);
}

pub fn tay(cpu: &mut Cpu6502, _mode: AddressingMode) {
    let value = cpu.registers.a;
    cpu.registers.y = value;
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    cpu.set_flag(Flag::Zero, value == 0);
    cpu.set_flag(Flag::Negative, value & 0x80 != 0);
}

pub fn tsx(cpu: &mut Cpu6502, _mode: AddressingMode) {
    let value = cpu.registers.sp;
    cpu.registers.x = value;
    cpu.registers.pc += 1;
    cpu.set_flag(Flag::Zero, value == 0);
    cpu.set_flag(Flag::Negative, value & 0x80 != 0);
}

pub fn txa(cpu: &mut Cpu6502, _mode: AddressingMode) {
    let value = cpu.registers.x;
    cpu.registers.a = value;
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    cpu.set_flag(Flag::Zero, value == 0);
    cpu.set_flag(Flag::Negative, value & 0x80 != 0);
}

pub fn txs(cpu: &mut Cpu6502, _mode: AddressingMode) {
    let value = cpu.registers.x;
    cpu.registers.sp = value;
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
}

pub fn tya(cpu: &mut Cpu6502, _mode: AddressingMode) {
    let value = cpu.registers.y;
    cpu.registers.a = value;
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    cpu.set_flag(Flag::Zero, value == 0);
    cpu.set_flag(Flag::Negative, value & 0x80 != 0);
}

pub fn bcc(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let carry = cpu.registers.status & Flag::Carry as Byte != 0;
    if !carry {
        cpu.registers.pc = address;
    } else {
        cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    }
}

pub fn bcs(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let carry = cpu.registers.status & Flag::Carry as Byte != 0;
    if carry {
        cpu.registers.pc = address;
    } else {
        cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    }
}

pub fn beq(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let zero = cpu.registers.status & Flag::Zero as Byte != 0;
    if zero {
        cpu.registers.pc = address;
    } else {
        cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    }
}

pub fn bmi(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let negative = cpu.registers.status & Flag::Negative as Byte != 0;
    if negative {
        cpu.registers.pc = address;
    } else {
        cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    }
}

pub fn bne(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let zero = cpu.registers.status & Flag::Zero as Byte != 0;
    if !zero {
        cpu.registers.pc = address;
    } else {
        cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    }

}

pub fn bpl(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let negative = cpu.registers.status & Flag::Negative as Byte != 0;
    if !negative {
        cpu.registers.pc = address;
    } else {
        cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    }
}

pub fn bvc(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let overflow = cpu.registers.status & Flag::Overflow as Byte != 0;
    if !overflow {
        cpu.registers.pc = address;
    } else {
        cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    }
}

pub fn bvs(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let overflow = cpu.registers.status & Flag::Overflow as Byte != 0;
    if overflow {
        cpu.registers.pc = address;
    } else {
        cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    }
}

pub fn clc(cpu: &mut Cpu6502, _mode: AddressingMode) {
    cpu.set_flag(Flag::Carry, false);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
}

pub fn cld(cpu: &mut Cpu6502, _mode: AddressingMode) {
    cpu.set_flag(Flag::Decimal, false);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
}

pub fn cli(cpu: &mut Cpu6502, _mode: AddressingMode) {
    cpu.set_flag(Flag::Interrupt, false);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
}

pub fn clv(cpu: &mut Cpu6502, _mode: AddressingMode) {
    cpu.set_flag(Flag::Overflow, false);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
}

pub fn sec(cpu: &mut Cpu6502, _mode: AddressingMode) {
    cpu.set_flag(Flag::Carry, true);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
}

pub fn sed(cpu: &mut Cpu6502, _mode: AddressingMode) {
    cpu.set_flag(Flag::Decimal, true);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
}

pub fn sei(cpu: &mut Cpu6502, _mode: AddressingMode) {
    cpu.set_flag(Flag::Interrupt, true);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
}

pub fn nop(cpu: &mut Cpu6502, _mode: AddressingMode) {
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
}

pub fn bit(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let value = cpu.read_byte(address);
    let a = cpu.registers.a;
    let result = a & value;
    cpu.set_flag(Flag::Zero, result == 0);
    cpu.set_flag(Flag::Negative, value & 0b10000000 != 0);
    cpu.set_flag(Flag::Overflow, value & 0b01000000 != 0);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
}

pub fn kil(cpu: &mut Cpu6502, _mode: AddressingMode) {
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
}

pub fn lax(_cpu: &mut Cpu6502, _mode: AddressingMode) {

}

pub fn sax(_cpu: &mut Cpu6502, _mode: AddressingMode) {
// todo
}

pub fn dcp(_cpu: &mut Cpu6502, _mode: AddressingMode) {
// todo
}

pub fn isc(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let value = cpu.read_byte(address);
    let result = value.wrapping_sub(1);
    cpu.write_byte(address, result);
    let a = cpu.registers.a;
    let result = a.wrapping_sub(result);
    cpu.set_flag(Flag::Carry, a >= result);
    cpu.set_flag(Flag::Zero, result == 0);
    cpu.set_flag(Flag::Negative, result & 0x80 != 0);
}

pub fn rla(_cpu: &mut Cpu6502, _mode: AddressingMode) {
// todo
}

pub fn rra(_cpu: &mut Cpu6502, _mode: AddressingMode) {
// todo
}

pub fn slo(_cpu: &mut Cpu6502, _mode: AddressingMode) {
// todo
}

pub fn sre(_cpu: &mut Cpu6502, _mode: AddressingMode) {
// todo
}

pub fn anc(_cpu: &mut Cpu6502, _mode: AddressingMode) {
// todo
}

pub fn alr(_cpu: &mut Cpu6502, _mode: AddressingMode) {
// todo
}

pub fn arr(_cpu: &mut Cpu6502, _mode: AddressingMode) {
// todo
}

pub fn xaa(_cpu: &mut Cpu6502, _mode: AddressingMode) {
// todo
}

pub fn axs(_cpu: &mut Cpu6502, _mode: AddressingMode) {
// todo
}

pub fn ahx(_cpu: &mut Cpu6502, _mode: AddressingMode) {
// todo
}

pub fn asl(cpu: &mut Cpu6502, mode: AddressingMode) {
    let value: Byte;
    let result: Byte;

    match mode {
        AddressingMode::Accumulator => {
            value = cpu.registers.a;
            result = value << 1;
            cpu.registers.a = result;
        }
        _ => {
            let address = mode.get_address(cpu);
            value = cpu.read_byte(address);
            result = value << 1;
            cpu.write_byte(address, result);
        }
    }
    cpu.set_flag(Flag::Carry, value & 0x80 != 0);
    cpu.set_flag(Flag::Zero, result == 0);
    cpu.set_flag(Flag::Negative, result & 0x80 != 0);
}

///
/// # JSR
///
/// # Arguments
/// * `cpu` - the cpu
/// * `mode` - the addressing mode
/// # Example

///
///
pub fn jsr(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    cpu.push_word_stack(cpu.registers.pc);
    cpu.registers.pc = address;
}

pub fn and(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let value = cpu.read_byte(address);
    let a = cpu.registers.a;
    let result = a & value;
    cpu.registers.a = result;
    cpu.set_flag(Flag::Zero, result == 0);
    cpu.set_flag(Flag::Negative, result & 0x80 != 0);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
}

pub fn tas(_cpu: &mut Cpu6502, _mode: AddressingMode) {
// todo
}

pub fn shx(_cpu: &mut Cpu6502, _mode: AddressingMode) {
// todo
}

pub fn shy(_cpu: &mut Cpu6502, _mode: AddressingMode) {
// todo
}

pub fn las(_cpu: &mut Cpu6502, _mode: AddressingMode) {
// todo
}

pub fn cpy(_cpu: &mut Cpu6502, _mode: AddressingMode) {
// todo
}

pub fn cpx(_cpu: &mut Cpu6502, _mode: AddressingMode) {
// todo
}

pub fn iny(_cpu: &mut Cpu6502, _mode: AddressingMode) {
// todo
}

pub fn inx(cpu: &mut Cpu6502, mode: AddressingMode) {
    let address = mode.get_address(cpu);
    let value = cpu.read_byte(address);
    cpu.registers.x = value.wrapping_add(1);
    cpu.set_flag(Flag::Zero, cpu.registers.x == 0);
    cpu.set_flag(Flag::Negative, cpu.registers.x & 0x80 != 0);
    cpu.registers.pc += 1;
}

pub fn dey(_cpu: &mut Cpu6502, _mode: AddressingMode) {
// todo
}

pub fn dex(_cpu: &mut Cpu6502, _mode: AddressingMode) {
// todo
}

pub fn asr(_cpu: &mut Cpu6502, _mode: AddressingMode) {
// todo
}

