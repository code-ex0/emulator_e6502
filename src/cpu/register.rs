///
/// File: cpu/register.rs
/// The register module contains the implementation of the register struct and the implementation of the register functions.
///

#[derive(Clone, Copy)]
pub struct Registers {
    pub a: u8, // Registre accumulateur
    pub x: u8, // Registre X
    pub y: u8, // Registre Y
    pub sp: u8, // Registre du pointeur de pile (Stack Pointer)
    pub pc: u16, // Registre du compteur ordinal (Program Counter)
    pub status: u8, // Registre d'état
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            pc: 0,
            status: 0,
        }
    }
}