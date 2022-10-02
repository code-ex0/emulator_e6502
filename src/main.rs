// emulate e6502 cpu

pub const MEMORY_SIZE: usize = 0x10000;

pub struct Cpu {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub sp: u16,
    pub status: u8,
    pub memory: Memory,
}

pub struct Memory {
    pub mem: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        Memory { mem: [0; MEMORY_SIZE] }
    }

    pub fn randomize(&mut self) {
        for i in 0..MEMORY_SIZE {
            self.mem[i] = rand::random::<u8>();
        }
    }
}


impl Cpu {

    pub fn new() -> Cpu {
        Cpu {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0,
            status: 0,
            memory: Memory::new(),
        }
    }

    pub fn load(&mut self, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            self.memory.mem[i] = byte;
        }
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.memory.mem[self.pc as usize];
            self.pc += 1;
            match opcode {
                0x00 => {
                    println!("BRK");
                    break;
                },
                _ => panic!("Unknown opcode: {:02x}", opcode),
            }
        }
    }

    pub fn dump(&self) {
        println!("A: {:02x} X: {:02x} Y: {:02x} PC: {:04x} SP: {:02x} Status: {:08b}", self.a, self.x, self.y, self.pc, self.sp, self.status);
    }

    pub fn hexdump(&self) {
        let mut line: [u8; 16] = [0; 16];
        let mut line_empty: bool = true;
        let mut line_ascii: String = String::new();
        let mut line_address: u16 = 0;
        for (i, &byte) in self.memory.mem.iter().enumerate() {
            if i % 16 == 0 {
                // print the line
                if !line_empty {
                    print!("{:04x}  ", line_address);
                    for byte in line.iter() {
                        print!("{:02x} ", byte);
                    }
                    print!("  ");
                    for byte in line_ascii.chars() {
                        print!("{}", byte);
                    }
                    println!("");
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
            print!("{:04x}  ", line_address);
            for byte in line.iter() {
                print!("{:02x} ", byte);
            }
            print!("  ");
            for byte in line_ascii.chars() {
                print!("{}", byte);
            }
            println!("");
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.memory.mem[addr as usize]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.memory.mem[addr as usize] = data;
    }
}

impl Cpu {

    // Immediate
    pub const INS_LDA_IM: u8 = 0xA9;
    // Zero Page
    pub const INS_LDA_ZP: u8 = 0xA5;
    // Zero Page, X
    pub const INS_LDA_ZPX: u8 = 0xB5;
    // Absolute
    pub const INS_JSR: u8 = 0x20;

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.pc = 0xFFFC;
        self.sp = 0x0100;
        self.status = 0;
        self.memory = Memory::new();
    }

    pub fn execute(&mut self, ticks: u32) {
        let mut ticks = ticks;
        while ticks > 0 {
            let instruction = self.fetch_byte(&mut ticks);
            match instruction {
                Cpu::INS_LDA_IM => {
                    self.a = self.fetch_byte(&mut ticks);
                    self.ldaset_status();
                },
                Cpu::INS_LDA_ZP => {
                    let address = self.fetch_byte(&mut ticks);
                    self.a = self.read_byte(address, &mut ticks);
                    self.ldaset_status();
                },
                Cpu::INS_LDA_ZPX => {
                    let address = self.fetch_byte(&mut ticks);
                    let address = address.wrapping_add(self.x);
                    ticks -= 1;
                    self.a = self.read_byte(address, &mut ticks);
                    self.ldaset_status();
                },
                Cpu::INS_JSR => {
                    let address = self.fetch_word(&mut ticks);
                    self.push_word(self.pc - 1, &mut ticks);
                    self.pc = address;
                },
                _ => {
                    panic!("Unknown instruction: {:02x}", instruction);
                }
            }
        }
    }

    pub fn fetch_byte(&mut self, ticks: &mut u32) -> u8 {
        let data = self.memory.mem[self.pc as usize];
        self.pc += 1;
        *ticks-=1;
        data
    }

    pub fn push_byte(&mut self, data: u8, ticks: &mut u32) {
        self.memory.mem[self.sp as usize] = data;
        self.sp -= 1;
        *ticks -= 1;
    }

    pub fn read_byte(&self, address: u8, ticks: &mut u32) -> u8 {
        let data = self.memory.mem[address as usize];
        *ticks-=1;
        data
    }

    pub fn fetch_word(&mut self, ticks: &mut u32) -> u16 {
        let low = self.fetch_byte(ticks);
        let high = self.fetch_byte(ticks);
        (high as u16) << 8 | low as u16
    }

    pub fn push_word(&mut self, word: u16, ticks: &mut u32) {
        let high = (word >> 8) as u8;
        let low = word as u8;
        *ticks -= 1;
        self.push_byte(high, ticks);
        self.push_byte(low, ticks);
    }



    pub fn ldaset_status(&mut self) {
        if self.a & 0b10000000 != 0 {
            self.status |= 0b10000000;
        }
        if self.a == 0 {
            self.status |= 0b00000010;
        }
    }

    pub fn asm(&mut self, _asm: &str) {
        todo!("asm");
    }
}

fn main() {
    let mut cpu = Cpu::new();
    cpu.reset();
    cpu.write(0xFFFC, Cpu::INS_JSR);
    cpu.write(0xFFFD, 0x00);
    cpu.write(0xFFFE, 0x00);
    cpu.write(0x0000, Cpu::INS_JSR);
    cpu.write(0x0001, 0x00);
    cpu.write(0x0002, 0xff);
    cpu.write(0xff00, Cpu::INS_LDA_IM);
    cpu.write(0xff01, 0x42);
    cpu.write(0xff02, Cpu::INS_LDA_ZP);
    cpu.write(0xff03, 0x02);


    cpu.hexdump();
    cpu.execute(17);
    cpu.dump();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lda_zp() {
        let mut cpu = Cpu::new();
        cpu.reset();
        cpu.write(0xFFFC, Cpu::INS_LDA_ZP);
        cpu.write(0xFFFD, 0x42);
        cpu.write(0x0042, 0x84);
        cpu.execute(4);
        assert_eq!(cpu.a, 0x84);
    }

    #[test]
    fn test_lda_zpx() {
        let mut cpu = Cpu::new();
        cpu.reset();
        cpu.x = 0x01;
        cpu.write(0xFFFC, Cpu::INS_LDA_ZPX);
        cpu.write(0xFFFD, 0x42);
        cpu.write(0x0043, 0x84);
        cpu.execute(5);
        assert_eq!(cpu.a, 0x84);
    }

    #[test]
    fn test_lda_im() {
        let mut cpu = Cpu::new();
        cpu.reset();
        cpu.write(0xFFFC, Cpu::INS_LDA_IM);
        cpu.write(0xFFFD, 0x84);
        cpu.execute(2);
        assert_eq!(cpu.a, 0x84);
    }

    #[test]
    fn test_jsr() {
        let mut cpu = Cpu::new();
        cpu.reset();
        cpu.write(0xFFFC, Cpu::INS_JSR);
        cpu.write(0xFFFD, 0x42);
        cpu.write(0xFFFE, 0x42);
        cpu.write(0x4242, Cpu::INS_LDA_IM);
        cpu.write(0x4243, 0x84);
        cpu.execute(7);
        assert_eq!(cpu.pc, 0x4244);
        assert_eq!(cpu.a, 0x84);
    }
}