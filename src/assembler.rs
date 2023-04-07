use std::collections::HashMap;
use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::instruction::find_instruction_by_name_and_mode;

pub fn assemble(source_code: &str) -> Vec<u8> {
    let mut labels = HashMap::new();
    let mut machine_code = Vec::new();

    // Première passe : collecter les étiquettes et leurs adresses.
    let mut address: u16 = 0;
    for line in source_code.lines() {
        let parts: Vec<_> = line.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        if parts[0].ends_with(':') {
            let label = parts[0].trim_end_matches(':');
            labels.insert(label.to_owned(), address);
        } else {
            let instruction = parts[0];
            let mode = parts[1];

            let inst = find_instruction_by_name_and_mode(instruction, mode)
                .unwrap_or_else(|| panic!("Instruction non prise en charge : {} {}", instruction, mode));

            address += inst.length as u16;
        }
    }

    // Deuxième passe : assembler les instructions en machine code.
    for line in source_code.lines() {
        let parts: Vec<_> = line.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        if !parts[0].ends_with(':') {
            let instruction = parts[0];
            let mode = parts[1];

            let inst = find_instruction_by_name_and_mode(instruction, mode)
                .unwrap_or_else(|| panic!("Instruction non prise en charge : {} {}", instruction, mode));

            machine_code.push(inst.opcode);

            match inst.addressing_mode {
                AddressingMode::Immediate
                | AddressingMode::ZeroPage
                | AddressingMode::ZeroPageX
                | AddressingMode::ZeroPageY
                | AddressingMode::Relative => {
                    let operand = parts[2];
                    let value: u8 = if operand.starts_with('$') {
                        u8::from_str_radix(&operand[1..], 16).unwrap()
                    } else if labels.contains_key(operand) {
                        (labels[operand] - address) as u8
                    } else {
                        panic!("Opérande non pris en charge: {}", operand);
                    };
                    machine_code.push(value);
                }
                AddressingMode::Absolute
                | AddressingMode::AbsoluteX
                | AddressingMode::AbsoluteY
                | AddressingMode::IndirectX
                | AddressingMode::IndirectY => {
                    let operand = parts[2];
                    let value: u16 = if operand.starts_with('$') {
                        u16::from_str_radix(&operand[1..], 16).unwrap()
                    } else if labels.contains_key(operand) {
                        labels[operand]
                    } else {
                        panic!("Opérande non pris en charge: {}", operand);
                    };
                    machine_code.push((value & 0xFF) as u8); // Low byte
                    machine_code.push((value >> 8) as u8); // High byte
                }
                AddressingMode::Indirect => {
                    // Ajoutez le code pour traiter le cas Indirect ici
                }
                AddressingMode::Accumulator => {
                    // Ajoutez le code pour traiter le cas Accumulator ici
                }
                AddressingMode::Implied => {
                    // Rien à faire pour le mode Implied
                }
            }
        }
    }

    machine_code
}
