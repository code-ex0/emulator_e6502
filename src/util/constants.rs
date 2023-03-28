use crate::util::types::Address;

// syze of the memory
pub const MEMORY_SIZE: usize = 0x10000;

// size of the stack
pub const STACK_SIZE: usize = 0x100;

pub const RESET_ADDRESS_LOW: Address = 0xFFFC;
pub const RESET_ADDRESS_HIGH: Address = 0xFFFD;

pub const OPCODE_KIL: &str = "KIL";