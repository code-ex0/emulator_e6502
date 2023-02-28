#![allow(dead_code)]

mod instruction;
mod function;
mod test;

// type of addressing mode
pub type Address = u16;

// type of one byte
pub type Byte = u8;

// type of two bytes
pub type Word = u16;

// syze of the memory
pub const MEMORY_SIZE: usize = 0x10000;

// size of the stack
pub const STACK_SIZE: usize = 0x100;

#[derive(Clone, Copy)]
pub enum Flag {
    Carry = 0b0000_0001,
    Zero = 0b0000_0010,
    Interrupt = 0b0000_0100,
    Decimal = 0b0000_1000,
    Break = 0b0001_0000,
    Unused = 0b0010_0000,
    Overflow = 0b0100_0000,
    Negative = 0b1000_0000,
}

impl From<Flag> for Byte {
    fn from(flag: Flag) -> Self {
        match flag {
            Flag::Carry => 0b00000001,
            Flag::Zero => 0b00000010,
            Flag::Interrupt => 0b00000100,
            Flag::Decimal => 0b00001000,
            Flag::Break => 0b00010000,
            Flag::Unused => 0b00100000,
            Flag::Overflow => 0b01000000,
            Flag::Negative => 0b10000000,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Ram {
    memory: [Byte; MEMORY_SIZE],
}

#[derive(Clone, Copy)]
pub struct Cpu6502 {
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    pc: u16,
    status: Byte,
    memory: Ram,
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            memory: [0; MEMORY_SIZE],
        }
    }

    pub fn read(&self, address: Address) -> Byte {
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: Address, data: Byte) {
        self.memory[address as usize] = data;
    }

    pub fn load(&mut self, data: &[u8], offset: Address) {
        for (i, &byte) in data.iter().enumerate() {
            self.write(offset + i as Address, byte);
        }
    }

    pub fn reset(&mut self) {
        self.memory = [0; MEMORY_SIZE];
    }

    pub fn dump(&self, offset: Address, len: usize) -> Vec<u8> {
        self.memory[offset as usize..offset as usize + len].to_vec()
    }

    pub fn hexdump(&self) {
        let mut line: [u8; 16] = [0; 16];
        let mut line_empty: bool = true;
        let mut line_ascii: String = String::new();
        let mut line_address: u16 = 0;
        for (i, &byte) in self.memory.iter().enumerate() {
            if i % 16 == 0 {
                if !line_empty {
                    print!("{:04X}  ", line_address);
                    for byte in line.iter() {
                        print!("{:02X} ", byte);
                    }
                    print!("  ");
                    for byte in line_ascii.chars() {
                        print!("{}", byte);
                    }
                    println!();
                }
                line = [0; 16];
                line_empty = true;
                line_ascii = String::new();
                line_address = i as u16;
            }
            if byte != 0 {
                line_empty = false;
            }
            line[i % 16] = byte;
            if byte >= 32 && byte <= 126 {
                line_ascii.push(byte as char);
            } else {
                line_ascii.push('.');
            }
        }
        if !line_empty {
            print!("{:04X}  ", line_address);
            for byte in line.iter() {
                print!("{:02X} ", byte);
            }
            print!("  ");
            for byte in line_ascii.chars() {
                print!("{}", byte);
            }
            println!();
        }
    }
}

impl Cpu6502 {
    pub fn new(ram: Ram) -> Cpu6502 {
        Cpu6502 {
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            pc: 0,
            status: 0,
            memory: ram,
        }
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0;
        self.pc = 0;
        self.status = 0;
        self.pc = self.memory.read(0xFFFC) as u16 | (self.memory.read(0xFFFD) as u16) << 8;
    }

    pub fn dump(&self) {
        println!("A: {:02X} X: {:02X} Y: {:02X} SP: {:02X} PC: {:04X} Status: {:02X}", self.a, self.x, self.y, self.sp, self.pc, self.status);
    }

    pub fn step(&mut self) -> Option<bool> {
        let opcode = self.memory.read(self.pc);
        let instruction = instruction::INSTRUCTIONS[opcode as usize];
        let addressing_mode = instruction.addressing_mode;
        (instruction.execute)(self, addressing_mode);
        if instruction.name == "KIL" {
            return Some(true);
        }
        if instruction.name == "BRK" {
            self.memory.hexdump();
        }
        None
    }

    pub fn read_byte(&mut self, address: Address) -> Byte {
        self.pc += 1;
        self.memory.read(address)
    }

    pub fn read_word(&mut self, address: Address) -> Word {
        let low = self.read_byte(address) as Word;
        let high = self.read_byte(address + 1) as Word;
        low | (high << 8)
    }

    pub fn write_byte(&mut self, address: Address, data: Byte) {
        self.pc += 1;
        self.memory.write(address, data);
    }

    pub fn write_word(&mut self, address: Address, data: Word) {
        self.write_byte(address, data as Byte);
        self.write_byte(address + 1, (data >> 8) as Byte);
    }


    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.status |= flag as Byte;
        } else {
            self.status &= !(flag as Byte);
        }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        (self.status & (flag as Byte)) != 0
    }

    pub fn push_stack(&mut self, data: Byte) {
        self.memory.write(STACK_SIZE as Word + self.sp as Address, data);
        self.sp += 1;
    }

    pub fn pop_stack(&mut self) -> Byte {
        self.sp -= 1;
        self.memory.read((STACK_SIZE as Word + self.sp as Address) as Address)
    }

    pub fn push_word_stack(&mut self, data: Word) {
        self.push_stack((data >> 8) as Byte);
        self.push_stack(data as Byte);
    }

    pub fn pop_word_stack(&mut self) -> Word {
        let low = self.pop_stack() as Word;
        let high = self.pop_stack() as Word;
        low | (high << 8)
    }
}

#[derive(Clone, Copy)]
pub struct InputOutput {
    keyboard: [bool; 16],
    display: [bool; 64 * 32],

}

#[derive(Clone, Copy)]
pub struct Emulator {
    memory: Ram,
    cpu: Cpu6502,
    io: InputOutput,
}

impl Emulator {
    pub fn new() -> Emulator {
        let ram = Ram::new();
        Emulator {
            memory: ram,
            cpu: Cpu6502::new(ram),
            io: InputOutput {
                keyboard: [false; 16],
                display: [false; 64 * 32],
            },
        }
    }

    pub fn reset(&mut self) {
        self.memory.reset();
        self.cpu.reset();
    }

    pub fn flash_ram(&mut self) {
        self.cpu.memory = self.memory;
    }

    pub fn load(&mut self, data: &[u8], offset: Address) {
        self.memory.load(data, offset);
    }

    pub fn dump(&self, offset: Address, len: usize) -> Vec<u8> {
        self.memory.dump(offset, len)
    }

    pub fn hexdump(&self) {
        self.memory.hexdump();
    }

    pub fn dump_cpu(&self) {
        self.cpu.dump();
    }

    pub fn step(&mut self) -> Option<bool> {
        self.cpu.step()
    }
}

fn main() {
    let mut emulator = Emulator::new();

    // function start to address 0x0200
    emulator.load(&[0xA9, 0x09, 0x69, 0x09, 0x4C, 0x00, 0x01, 0x00], 0x200);

    // function start to address 0x1000
    emulator.load( &[0xBA, 0x00], 0x1000);

    // reset function
    emulator.memory.write(0xFFFC, 0x00);
    emulator.memory.write(0xFFFD, 0x02);

    // break function
    emulator.memory.write(0xFFFE, 0x00);
    emulator.memory.write(0xFFFF, 0x10);

    emulator.flash_ram();
    emulator.reset();

    emulator.dump_cpu();
    emulator.hexdump();

    loop {
        let kill = emulator.step();
        emulator.dump_cpu();
        match kill {
            None => {}
            Some(x) => {
                if x == true {
                    emulator.cpu.memory.hexdump();
                    break
                }
            }
        }
    }
}
