use crate::cpu::cpu_6502::Cpu6502;
use crate::memory::ram::Ram;

///
/// prepare a cpu for testing
///
pub fn get_cpu() -> Cpu6502 {
    let ram = Ram::new();
    let mut cpu = Cpu6502::new(ram);
    cpu.reset();
    cpu.memory.reset();
    cpu
}

///
/// # adc
/// Test the ADC instruction
///
#[cfg(test)]
mod adc_tests;

///
/// # AND
/// Test the AND instruction
///
#[cfg(test)]
mod and_tests;

///
/// # ASL
/// Test the ASL instruction
///
#[cfg(test)]
mod asl_tests;

///
/// # BCC
/// Test the BCC instruction
///
#[cfg(test)]
mod bcc_tests;

///
/// # BCS
/// Test the BCS instruction
///
#[cfg(test)]
mod bcs_tests;

///
/// # BEQ
/// Test the BEQ instruction
///
#[cfg(test)]
mod beq_tests;

///
/// # BIT
/// Test the BIT instruction
///
#[cfg(test)]
mod bit_tests;

///
/// # BMI
/// Test the BMI instruction
///
#[cfg(test)]
mod bmi_tests;

///
/// # BNE
/// Test the BNE instruction
///
#[cfg(test)]
mod bne_tests;

///
/// # BPL
/// Test the BPL instruction
///
#[cfg(test)]
mod bpl_tests;

///
/// # brk
/// Test the BRK instruction
///
#[cfg(test)]
mod brk_tests;

///
/// # BVC
/// Test the BVC instruction
///
#[cfg(test)]
mod bvc_tests;

///
/// # BVS
/// Test the BVS instruction
///
#[cfg(test)]
mod bvs_tests;

///
/// # CLC
/// Test the CLC instruction
///
#[cfg(test)]
mod clc_tests;

///
/// # CLD
/// Test the CLD instruction
///
#[cfg(test)]
mod cld_tests;

///
/// # CLI
/// Test the CLI instruction
///
#[cfg(test)]
mod cli_tests;

///
/// # CLV
/// Test the CLV instruction
///
#[cfg(test)]
mod clv_tests;

///
/// # CMP
/// Test the CMP instruction
///
#[cfg(test)]
mod cmp_tests;

///
/// # CPM
/// Test the CPM instruction
///
#[cfg(test)]
mod cpm_tests;

///
/// # CPX
/// Test the CPX instruction
///
#[cfg(test)]
mod cpx_tests;

///
/// # CPY
/// Test the CPY instruction
///
#[cfg(test)]
mod cpy_tests;

///
/// # DEC
/// Test the DEC instruction
///
#[cfg(test)]
mod dec_tests;

///
/// # DEX
/// Test the DEX instruction
///
#[cfg(test)]
mod dex_tests;

///
/// # DEY
/// Test the DEY instruction
///
#[cfg(test)]
mod dey_tests;

///
/// # EOR
/// Test the EOR instruction
///
#[cfg(test)]
mod eor_tests;

///
/// # INC
/// Test the INC instruction
///
#[cfg(test)]
mod inc_tests;

///
/// # INX
/// Test the INX instruction
///
#[cfg(test)]
mod inx_tests;

///
/// # INY
/// Test the INY instruction
///
#[cfg(test)]
mod iny_tests;

///
/// # JMP
/// Test the JMP instruction
///
#[cfg(test)]
mod jmp_tests;

///
/// # JSR
/// Test the JSR instruction
///
#[cfg(test)]
mod jsr_tests;

///
/// # LDA
/// Test the LDA instruction
///
#[cfg(test)]
mod lda_tests;

///
/// # LDX
/// Test the LDX instruction
///
#[cfg(test)]
mod ldx_tests;

///
/// # LDY
/// Test the LDY instruction
///
#[cfg(test)]
mod ldy_tests;

///
/// # LSR
/// Test the LSR instruction
///
#[cfg(test)]
mod lsr_tests;

///
/// # NOP
/// Test the NOP instruction
///
#[cfg(test)]
mod nop_tests;

///
/// # ORA
/// Test the ORA instruction
///
#[cfg(test)]
mod ora_tests;

///
/// # PHA
/// Test the PHA instruction
///
#[cfg(test)]
mod pha_tests;

///
/// # PHP
/// Test the PHP instruction
///
#[cfg(test)]
mod php_tests;

///
/// # PLA
/// Test the PLA instruction
///
#[cfg(test)]
mod pla_tests;

///
/// # PLP
/// Test the PLP instruction
///
#[cfg(test)]
mod plp_tests;

///
/// # POL
/// Test the POL instruction
///
#[cfg(test)]
mod pol_tests;

///
/// # ROL
/// Test the ROL instruction
///
#[cfg(test)]
mod rol_tests;

///
/// # ROR
/// Test the ROR instruction
///
#[cfg(test)]
mod ror_tests;

///
/// # RTI
/// Test the RTI instruction
///
#[cfg(test)]
mod rti_tests;

///
/// # RTS
/// Test the RTS instruction
///
#[cfg(test)]
mod rts_tests;

///
/// # SBC
/// Test the SBC instruction
///
#[cfg(test)]
mod sbc_tests;

///
/// # SEC
/// Test the SEC instruction
///
#[cfg(test)]
mod sec_tests;

///
/// # SED
/// Test the SED instruction
///
#[cfg(test)]
mod sed_tests;

///
/// # SEI
/// Test the SEI instruction
///
#[cfg(test)]
mod sei_tests;

///
/// # STA
/// Test the STA instruction
///
#[cfg(test)]
mod sta_tests;

///
/// # STX
/// Test the STX instruction
///
#[cfg(test)]
mod stx_tests;

///
/// # STY
/// Test the STY instruction
///
#[cfg(test)]
mod sty_tests;

///
/// # TAX
/// Test the TAX instruction
///
#[cfg(test)]
mod tax_tests;

///
/// # TAY
/// Test the TAY instruction
///
#[cfg(test)]
mod tay_tests;

///
/// # TSX
/// Test the TSX instruction
///
#[cfg(test)]
mod tsx_tests;

///
/// # TXA
/// Test the TXA instruction
///
#[cfg(test)]
mod txa_tests;

///
/// # TXS
/// Test the TXS instruction
///
#[cfg(test)]
mod txs_tests;

///
/// # TYA
/// Test the TYA instruction
///
#[cfg(test)]
mod tya_tests;