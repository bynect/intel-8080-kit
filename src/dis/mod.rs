use super::op::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct OpError(usize);

impl fmt::Display for OpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "expected {} bytes", self.0)
    }
}

pub fn disassemble_raw(bin: &Vec<u8>) -> Result<Vec<RawOpcode>, OpError> {
    let mut ops = Vec::new();

    let mut i = 0;
    while i < bin.len() {
        ops.push(match bin[i] {
            0x00 => {
                i += 1;
                RawOpcode::NOP
            }
            0x01 => {
                i += 3;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::LXI_B
                }
            }
            0x02 => {
                i += 1;
                RawOpcode::STAX_B
            }
            0x03 => {
                i += 1;
                RawOpcode::INX_B
            }
            0x04 => {
                i += 1;
                RawOpcode::INR_B
            }
            0x05 => {
                i += 1;
                RawOpcode::DCR_B
            }
            0x06 => {
                i += 2;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::MVI_B
                }
            }
            0x07 => {
                i += 1;
                RawOpcode::RLC
            }
            0x09 => {
                i += 1;
                RawOpcode::DAD_B
            }
            0x0a => {
                i += 1;
                RawOpcode::LDAX_B
            }
            0x0b => {
                i += 1;
                RawOpcode::DCX_B
            }
            0x0c => {
                i += 1;
                RawOpcode::INR_C
            }
            0x0d => {
                i += 1;
                RawOpcode::DCR_C
            }
            0x0e => {
                i += 2;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::MVI_C
                }
            }
            0x0f => {
                i += 1;
                RawOpcode::RRC
            }
            0x11 => {
                i += 3;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::LXI_D
                }
            }
            0x12 => {
                i += 1;
                RawOpcode::STAX_D
            }
            0x13 => {
                i += 1;
                RawOpcode::INX_D
            }
            0x14 => {
                i += 1;
                RawOpcode::INR_D
            }
            0x15 => {
                i += 1;
                RawOpcode::DCR_D
            }
            0x16 => {
                i += 2;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::MVI_D
                }
            }
            0x17 => {
                i += 1;
                RawOpcode::RAL
            }
            0x19 => {
                i += 1;
                RawOpcode::DAD_D
            }
            0x1a => {
                i += 1;
                RawOpcode::LDAX_D
            }
            0x1b => {
                i += 1;
                RawOpcode::DCX_D
            }
            0x1c => {
                i += 1;
                RawOpcode::INR_E
            }
            0x1d => {
                i += 1;
                RawOpcode::DCR_E
            }
            0x1e => {
                i += 2;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::MVI_E
                }
            }
            0x1f => {
                i += 1;
                RawOpcode::RAR
            }
            0x21 => {
                i += 3;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::LXI_H
                }
            }
            0x22 => {
                i += 3;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::SHLD
                }
            }
            0x23 => {
                i += 1;
                RawOpcode::INX_H
            }
            0x24 => {
                i += 1;
                RawOpcode::INR_H
            }
            0x25 => {
                i += 1;
                RawOpcode::DCR_H
            }
            0x26 => {
                i += 2;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::MVI_H
                }
            }
            0x27 => {
                i += 1;
                RawOpcode::DAA
            }
            0x29 => {
                i += 1;
                RawOpcode::DAD_H
            }
            0x2a => {
                i += 3;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::LHLD
                }
            }
            0x2b => {
                i += 1;
                RawOpcode::DCX_H
            }
            0x2c => {
                i += 1;
                RawOpcode::INR_L
            }
            0x2d => {
                i += 1;
                RawOpcode::DCR_L
            }
            0x2e => {
                i += 2;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::MVI_L
                }
            }
            0x2f => {
                i += 1;
                RawOpcode::CMA
            }
            0x31 => {
                i += 3;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::LXI_SP
                }
            }
            0x32 => {
                i += 3;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::STA
                }
            }
            0x33 => {
                i += 1;
                RawOpcode::INX_SP
            }
            0x34 => {
                i += 1;
                RawOpcode::INR_M
            }
            0x35 => {
                i += 1;
                RawOpcode::DCR_M
            }
            0x36 => {
                i += 2;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::MVI_M
                }
            }
            0x37 => {
                i += 1;
                RawOpcode::STC
            }
            0x39 => {
                i += 1;
                RawOpcode::DAD_SP
            }
            0x3a => {
                i += 3;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::LDA
                }
            }
            0x3b => {
                i += 1;
                RawOpcode::DCX_SP
            }
            0x3c => {
                i += 1;
                RawOpcode::INR_A
            }
            0x3d => {
                i += 1;
                RawOpcode::DCR_A
            }
            0x3e => {
                i += 2;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::MVI_A
                }
            }
            0x3f => {
                i += 1;
                RawOpcode::CMC
            }
            0x40 => {
                i += 1;
                RawOpcode::MOV_B_B
            }
            0x41 => {
                i += 1;
                RawOpcode::MOV_B_C
            }
            0x42 => {
                i += 1;
                RawOpcode::MOV_B_D
            }
            0x43 => {
                i += 1;
                RawOpcode::MOV_B_E
            }
            0x44 => {
                i += 1;
                RawOpcode::MOV_B_H
            }
            0x45 => {
                i += 1;
                RawOpcode::MOV_B_L
            }
            0x46 => {
                i += 1;
                RawOpcode::MOV_B_M
            }
            0x47 => {
                i += 1;
                RawOpcode::MOV_B_A
            }
            0x48 => {
                i += 1;
                RawOpcode::MOV_C_B
            }
            0x49 => {
                i += 1;
                RawOpcode::MOV_C_C
            }
            0x4a => {
                i += 1;
                RawOpcode::MOV_C_D
            }
            0x4b => {
                i += 1;
                RawOpcode::MOV_C_E
            }
            0x4c => {
                i += 1;
                RawOpcode::MOV_C_H
            }
            0x4d => {
                i += 1;
                RawOpcode::MOV_C_L
            }
            0x4e => {
                i += 1;
                RawOpcode::MOV_C_M
            }
            0x4f => {
                i += 1;
                RawOpcode::MOV_C_A
            }
            0x50 => {
                i += 1;
                RawOpcode::MOV_D_B
            }
            0x51 => {
                i += 1;
                RawOpcode::MOV_D_C
            }
            0x52 => {
                i += 1;
                RawOpcode::MOV_D_D
            }
            0x53 => {
                i += 1;
                RawOpcode::MOV_D_E
            }
            0x54 => {
                i += 1;
                RawOpcode::MOV_D_H
            }
            0x55 => {
                i += 1;
                RawOpcode::MOV_D_L
            }
            0x56 => {
                i += 1;
                RawOpcode::MOV_D_M
            }
            0x57 => {
                i += 1;
                RawOpcode::MOV_D_A
            }
            0x58 => {
                i += 1;
                RawOpcode::MOV_E_B
            }
            0x59 => {
                i += 1;
                RawOpcode::MOV_E_C
            }
            0x5a => {
                i += 1;
                RawOpcode::MOV_E_D
            }
            0x5b => {
                i += 1;
                RawOpcode::MOV_E_E
            }
            0x5c => {
                i += 1;
                RawOpcode::MOV_E_H
            }
            0x5d => {
                i += 1;
                RawOpcode::MOV_E_L
            }
            0x5e => {
                i += 1;
                RawOpcode::MOV_E_M
            }
            0x5f => {
                i += 1;
                RawOpcode::MOV_E_A
            }
            0x60 => {
                i += 1;
                RawOpcode::MOV_H_B
            }
            0x61 => {
                i += 1;
                RawOpcode::MOV_H_C
            }
            0x62 => {
                i += 1;
                RawOpcode::MOV_H_D
            }
            0x63 => {
                i += 1;
                RawOpcode::MOV_H_E
            }
            0x64 => {
                i += 1;
                RawOpcode::MOV_H_H
            }
            0x65 => {
                i += 1;
                RawOpcode::MOV_H_L
            }
            0x66 => {
                i += 1;
                RawOpcode::MOV_H_M
            }
            0x67 => {
                i += 1;
                RawOpcode::MOV_H_A
            }
            0x68 => {
                i += 1;
                RawOpcode::MOV_L_B
            }
            0x69 => {
                i += 1;
                RawOpcode::MOV_L_C
            }
            0x6a => {
                i += 1;
                RawOpcode::MOV_L_D
            }
            0x6b => {
                i += 1;
                RawOpcode::MOV_L_E
            }
            0x6c => {
                i += 1;
                RawOpcode::MOV_L_H
            }
            0x6d => {
                i += 1;
                RawOpcode::MOV_L_L
            }
            0x6e => {
                i += 1;
                RawOpcode::MOV_L_M
            }
            0x6f => {
                i += 1;
                RawOpcode::MOV_L_A
            }
            0x70 => {
                i += 1;
                RawOpcode::MOV_M_B
            }
            0x71 => {
                i += 1;
                RawOpcode::MOV_M_C
            }
            0x72 => {
                i += 1;
                RawOpcode::MOV_M_D
            }
            0x73 => {
                i += 1;
                RawOpcode::MOV_M_E
            }
            0x74 => {
                i += 1;
                RawOpcode::MOV_M_H
            }
            0x75 => {
                i += 1;
                RawOpcode::MOV_M_L
            }
            0x76 => {
                i += 1;
                RawOpcode::HLT
            }
            0x77 => {
                i += 1;
                RawOpcode::MOV_M_A
            }
            0x78 => {
                i += 1;
                RawOpcode::MOV_A_B
            }
            0x79 => {
                i += 1;
                RawOpcode::MOV_A_C
            }
            0x7a => {
                i += 1;
                RawOpcode::MOV_A_D
            }
            0x7b => {
                i += 1;
                RawOpcode::MOV_A_E
            }
            0x7c => {
                i += 1;
                RawOpcode::MOV_A_H
            }
            0x7d => {
                i += 1;
                RawOpcode::MOV_A_L
            }
            0x7e => {
                i += 1;
                RawOpcode::MOV_A_M
            }
            0x7f => {
                i += 1;
                RawOpcode::MOV_A_A
            }
            0x80 => {
                i += 1;
                RawOpcode::ADD_B
            }
            0x81 => {
                i += 1;
                RawOpcode::ADD_C
            }
            0x82 => {
                i += 1;
                RawOpcode::ADD_D
            }
            0x83 => {
                i += 1;
                RawOpcode::ADD_E
            }
            0x84 => {
                i += 1;
                RawOpcode::ADD_H
            }
            0x85 => {
                i += 1;
                RawOpcode::ADD_L
            }
            0x86 => {
                i += 1;
                RawOpcode::ADD_M
            }
            0x87 => {
                i += 1;
                RawOpcode::ADD_A
            }
            0x88 => {
                i += 1;
                RawOpcode::ADC_B
            }
            0x89 => {
                i += 1;
                RawOpcode::ADC_C
            }
            0x8a => {
                i += 1;
                RawOpcode::ADC_D
            }
            0x8b => {
                i += 1;
                RawOpcode::ADC_E
            }
            0x8c => {
                i += 1;
                RawOpcode::ADC_H
            }
            0x8d => {
                i += 1;
                RawOpcode::ADC_L
            }
            0x8e => {
                i += 1;
                RawOpcode::ADC_M
            }
            0x8f => {
                i += 1;
                RawOpcode::ADC_A
            }
            0x90 => {
                i += 1;
                RawOpcode::SUB_B
            }
            0x91 => {
                i += 1;
                RawOpcode::SUB_C
            }
            0x92 => {
                i += 1;
                RawOpcode::SUB_D
            }
            0x93 => {
                i += 1;
                RawOpcode::SUB_E
            }
            0x94 => {
                i += 1;
                RawOpcode::SUB_H
            }
            0x95 => {
                i += 1;
                RawOpcode::SUB_L
            }
            0x96 => {
                i += 1;
                RawOpcode::SUB_M
            }
            0x97 => {
                i += 1;
                RawOpcode::SUB_A
            }
            0x98 => {
                i += 1;
                RawOpcode::SBB_B
            }
            0x99 => {
                i += 1;
                RawOpcode::SBB_C
            }
            0x9a => {
                i += 1;
                RawOpcode::SBB_D
            }
            0x9b => {
                i += 1;
                RawOpcode::SBB_E
            }
            0x9c => {
                i += 1;
                RawOpcode::SBB_H
            }
            0x9d => {
                i += 1;
                RawOpcode::SBB_L
            }
            0x9e => {
                i += 1;
                RawOpcode::SBB_M
            }
            0x9f => {
                i += 1;
                RawOpcode::SBB_A
            }
            0xa0 => {
                i += 1;
                RawOpcode::ANA_B
            }
            0xa1 => {
                i += 1;
                RawOpcode::ANA_C
            }
            0xa2 => {
                i += 1;
                RawOpcode::ANA_D
            }
            0xa3 => {
                i += 1;
                RawOpcode::ANA_E
            }
            0xa4 => {
                i += 1;
                RawOpcode::ANA_H
            }
            0xa5 => {
                i += 1;
                RawOpcode::ANA_L
            }
            0xa6 => {
                i += 1;
                RawOpcode::ANA_M
            }
            0xa7 => {
                i += 1;
                RawOpcode::ANA_A
            }
            0xa8 => {
                i += 1;
                RawOpcode::XRA_B
            }
            0xa9 => {
                i += 1;
                RawOpcode::XRA_C
            }
            0xaa => {
                i += 1;
                RawOpcode::XRA_D
            }
            0xab => {
                i += 1;
                RawOpcode::XRA_E
            }
            0xac => {
                i += 1;
                RawOpcode::XRA_H
            }
            0xad => {
                i += 1;
                RawOpcode::XRA_L
            }
            0xae => {
                i += 1;
                RawOpcode::XRA_M
            }
            0xaf => {
                i += 1;
                RawOpcode::XRA_A
            }
            0xb0 => {
                i += 1;
                RawOpcode::ORA_B
            }
            0xb1 => {
                i += 1;
                RawOpcode::ORA_C
            }
            0xb2 => {
                i += 1;
                RawOpcode::ORA_D
            }
            0xb3 => {
                i += 1;
                RawOpcode::ORA_E
            }
            0xb4 => {
                i += 1;
                RawOpcode::ORA_H
            }
            0xb5 => {
                i += 1;
                RawOpcode::ORA_L
            }
            0xb6 => {
                i += 1;
                RawOpcode::ORA_M
            }
            0xb7 => {
                i += 1;
                RawOpcode::ORA_A
            }
            0xb8 => {
                i += 1;
                RawOpcode::CMP_B
            }
            0xb9 => {
                i += 1;
                RawOpcode::CMP_C
            }
            0xba => {
                i += 1;
                RawOpcode::CMP_D
            }
            0xbb => {
                i += 1;
                RawOpcode::CMP_E
            }
            0xbc => {
                i += 1;
                RawOpcode::CMP_H
            }
            0xbd => {
                i += 1;
                RawOpcode::CMP_L
            }
            0xbe => {
                i += 1;
                RawOpcode::CMP_M
            }
            0xbf => {
                i += 1;
                RawOpcode::CMP_A
            }
            0xc0 => {
                i += 1;
                RawOpcode::RNZ
            }
            0xc1 => {
                i += 1;
                RawOpcode::POP_B
            }
            0xc2 => {
                i += 3;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::JNZ
                }
            }
            0xc3 => {
                i += 3;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::JMP
                }
            }
            0xc4 => {
                i += 3;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::CNZ
                }
            }
            0xc5 => {
                i += 1;
                RawOpcode::PUSH_B
            }
            0xc6 => {
                i += 2;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::ADI
                }
            }
            0xc7 => {
                i += 1;
                RawOpcode::RST_0
            }
            0xc8 => {
                i += 1;
                RawOpcode::RZ
            }
            0xc9 => {
                i += 1;
                RawOpcode::RET
            }
            0xca => {
                i += 3;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::JZ
                }
            }
            0xcc => {
                i += 3;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::CZ
                }
            }
            0xcd => {
                i += 3;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::CALL
                }
            }
            0xce => {
                i += 2;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::ACI
                }
            }
            0xcf => {
                i += 1;
                RawOpcode::RST_1
            }
            0xd0 => {
                i += 1;
                RawOpcode::RNC
            }
            0xd1 => {
                i += 1;
                RawOpcode::POP_D
            }
            0xd2 => {
                i += 3;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::JNC
                }
            }
            0xd3 => {
                i += 2;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::OUT
                }
            }
            0xd4 => {
                i += 3;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::CNC
                }
            }
            0xd5 => {
                i += 1;
                RawOpcode::PUSH_D
            }
            0xd6 => {
                i += 2;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::SUI
                }
            }
            0xd7 => {
                i += 1;
                RawOpcode::RST_2
            }
            0xd8 => {
                i += 1;
                RawOpcode::RC
            }
            0xda => {
                i += 3;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::JC
                }
            }
            0xdb => {
                i += 2;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::IN
                }
            }
            0xdc => {
                i += 3;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::CC
                }
            }
            0xde => {
                i += 2;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::SBI
                }
            }
            0xdf => {
                i += 1;
                RawOpcode::RST_3
            }
            0xe0 => {
                i += 1;
                RawOpcode::RPO
            }
            0xe1 => {
                i += 1;
                RawOpcode::POP_H
            }
            0xe2 => {
                i += 3;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::JPO
                }
            }
            0xe3 => {
                i += 1;
                RawOpcode::XTHL
            }
            0xe4 => {
                i += 3;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::CPO
                }
            }
            0xe5 => {
                i += 1;
                RawOpcode::PUSH_H
            }
            0xe6 => {
                i += 2;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::ANI
                }
            }
            0xe7 => {
                i += 1;
                RawOpcode::RST_4
            }
            0xe8 => {
                i += 1;
                RawOpcode::RPE
            }
            0xe9 => {
                i += 1;
                RawOpcode::PCHL
            }
            0xea => {
                i += 3;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::JPE
                }
            }
            0xeb => {
                i += 1;
                RawOpcode::XCHG
            }
            0xec => {
                i += 3;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::CPE
                }
            }
            0xee => {
                i += 2;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::XRI
                }
            }
            0xef => {
                i += 1;
                RawOpcode::RST_5
            }
            0xf0 => {
                i += 1;
                RawOpcode::RP
            }
            0xf1 => {
                i += 1;
                RawOpcode::POP_PSW
            }
            0xf2 => {
                i += 3;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::JP
                }
            }
            0xf3 => {
                i += 1;
                RawOpcode::DI
            }
            0xf4 => {
                i += 3;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::CP
                }
            }
            0xf5 => {
                i += 1;
                RawOpcode::PUSH_PSW
            }
            0xf6 => {
                i += 2;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::ORI
                }
            }
            0xf7 => {
                i += 1;
                RawOpcode::RST_6
            }
            0xf8 => {
                i += 1;
                RawOpcode::RM
            }
            0xf9 => {
                i += 1;
                RawOpcode::SPHL
            }
            0xfa => {
                i += 3;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::JM
                }
            }
            0xfb => {
                i += 1;
                RawOpcode::EI
            }
            0xfc => {
                i += 3;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::CM
                }
            }
            0xfe => {
                i += 2;
                if i >= bin.len() {
                    return Err(OpError(i - bin.len()));
                } else {
                    RawOpcode::CPI
                }
            }
            0xff => {
                i += 1;
                RawOpcode::RST_7
            }
            _ => {
                i += 1;
                RawOpcode::NOP
            }
        });
    }

    Ok(ops)
}

pub fn disassemble(bin: &Vec<u8>) -> Result<Vec<Opcode>, OpError> {
    let mut ops = Vec::new();

    let mut i = 0;
    while i < bin.len() {
        ops.push(match bin[i] {
            0x00 => {
                i += 1;
                Opcode::Nop
            }
            0x01 => {
                i += 3;
                let b1 = bin.get(i - 2).ok_or(OpError(2))?;
                let b2 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::LxiB(*b1, *b2)
            }
            0x02 => {
                i += 1;
                Opcode::StaxB
            }
            0x03 => {
                i += 1;
                Opcode::InxB
            }
            0x04 => {
                i += 1;
                Opcode::InrB
            }
            0x05 => {
                i += 1;
                Opcode::DcrB
            }
            0x06 => {
                i += 2;
                let b1 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::MviB(*b1)
            }
            0x07 => {
                i += 1;
                Opcode::Rlc
            }
            0x09 => {
                i += 1;
                Opcode::DadB
            }
            0x0a => {
                i += 1;
                Opcode::LdaxB
            }
            0x0b => {
                i += 1;
                Opcode::DcxB
            }
            0x0c => {
                i += 1;
                Opcode::InrC
            }
            0x0d => {
                i += 1;
                Opcode::DcrC
            }
            0x0e => {
                i += 2;
                let b1 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::MviC(*b1)
            }
            0x0f => {
                i += 1;
                Opcode::Rrc
            }
            0x11 => {
                i += 3;
                let b1 = bin.get(i - 2).ok_or(OpError(2))?;
                let b2 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::LxiD(*b1, *b2)
            }
            0x12 => {
                i += 1;
                Opcode::StaxD
            }
            0x13 => {
                i += 1;
                Opcode::InxD
            }
            0x14 => {
                i += 1;
                Opcode::InrD
            }
            0x15 => {
                i += 1;
                Opcode::DcrD
            }
            0x16 => {
                i += 2;
                let b1 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::MviD(*b1)
            }
            0x17 => {
                i += 1;
                Opcode::Ral
            }
            0x19 => {
                i += 1;
                Opcode::DadD
            }
            0x1a => {
                i += 1;
                Opcode::LdaxD
            }
            0x1b => {
                i += 1;
                Opcode::DcxD
            }
            0x1c => {
                i += 1;
                Opcode::InrE
            }
            0x1d => {
                i += 1;
                Opcode::DcrE
            }
            0x1e => {
                i += 2;
                let b1 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::MviE(*b1)
            }
            0x1f => {
                i += 1;
                Opcode::Rar
            }
            0x21 => {
                i += 3;
                let b1 = bin.get(i - 2).ok_or(OpError(2))?;
                let b2 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::LxiH(*b1, *b2)
            }
            0x22 => {
                i += 3;
                let b1 = bin.get(i - 2).ok_or(OpError(2))?;
                let b2 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::Shld(*b1, *b2)
            }
            0x23 => {
                i += 1;
                Opcode::InxH
            }
            0x24 => {
                i += 1;
                Opcode::InrH
            }
            0x25 => {
                i += 1;
                Opcode::DcrH
            }
            0x26 => {
                i += 2;
                let b1 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::MviH(*b1)
            }
            0x27 => {
                i += 1;
                Opcode::Daa
            }
            0x29 => {
                i += 1;
                Opcode::DadH
            }
            0x2a => {
                i += 3;
                let b1 = bin.get(i - 2).ok_or(OpError(2))?;
                let b2 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::Lhld(*b1, *b2)
            }
            0x2b => {
                i += 1;
                Opcode::DcxH
            }
            0x2c => {
                i += 1;
                Opcode::InrL
            }
            0x2d => {
                i += 1;
                Opcode::DcrL
            }
            0x2e => {
                i += 2;
                let b1 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::MviL(*b1)
            }
            0x2f => {
                i += 1;
                Opcode::Cma
            }
            0x31 => {
                i += 1;
                Opcode::LxiSpD16
            }
            0x32 => {
                i += 3;
                let b1 = bin.get(i - 2).ok_or(OpError(2))?;
                let b2 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::Sta(*b1, *b2)
            }
            0x33 => {
                i += 1;
                Opcode::InxSp
            }
            0x34 => {
                i += 1;
                Opcode::InrM
            }
            0x35 => {
                i += 1;
                Opcode::DcrM
            }
            0x36 => {
                i += 2;
                let b1 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::MviM(*b1)
            }
            0x37 => {
                i += 1;
                Opcode::Stc
            }
            0x39 => {
                i += 1;
                Opcode::DadSp
            }
            0x3a => {
                i += 3;
                let b1 = bin.get(i - 2).ok_or(OpError(2))?;
                let b2 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::Lda(*b1, *b2)
            }
            0x3b => {
                i += 1;
                Opcode::DcxSp
            }
            0x3c => {
                i += 1;
                Opcode::InrA
            }
            0x3d => {
                i += 1;
                Opcode::DcrA
            }
            0x3e => {
                i += 2;
                let b1 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::MviA(*b1)
            }
            0x3f => {
                i += 1;
                Opcode::Cmc
            }
            0x40 => {
                i += 1;
                Opcode::MovBB
            }
            0x41 => {
                i += 1;
                Opcode::MovBC
            }
            0x42 => {
                i += 1;
                Opcode::MovBD
            }
            0x43 => {
                i += 1;
                Opcode::MovBE
            }
            0x44 => {
                i += 1;
                Opcode::MovBH
            }
            0x45 => {
                i += 1;
                Opcode::MovBL
            }
            0x46 => {
                i += 1;
                Opcode::MovBM
            }
            0x47 => {
                i += 1;
                Opcode::MovBA
            }
            0x48 => {
                i += 1;
                Opcode::MovCB
            }
            0x49 => {
                i += 1;
                Opcode::MovCC
            }
            0x4a => {
                i += 1;
                Opcode::MovCD
            }
            0x4b => {
                i += 1;
                Opcode::MovCE
            }
            0x4c => {
                i += 1;
                Opcode::MovCH
            }
            0x4d => {
                i += 1;
                Opcode::MovCL
            }
            0x4e => {
                i += 1;
                Opcode::MovCM
            }
            0x4f => {
                i += 1;
                Opcode::MovCA
            }
            0x50 => {
                i += 1;
                Opcode::MovDB
            }
            0x51 => {
                i += 1;
                Opcode::MovDC
            }
            0x52 => {
                i += 1;
                Opcode::MovDD
            }
            0x53 => {
                i += 1;
                Opcode::MovDE
            }
            0x54 => {
                i += 1;
                Opcode::MovDH
            }
            0x55 => {
                i += 1;
                Opcode::MovDL
            }
            0x56 => {
                i += 1;
                Opcode::MovDM
            }
            0x57 => {
                i += 1;
                Opcode::MovDA
            }
            0x58 => {
                i += 1;
                Opcode::MovEB
            }
            0x59 => {
                i += 1;
                Opcode::MovEC
            }
            0x5a => {
                i += 1;
                Opcode::MovED
            }
            0x5b => {
                i += 1;
                Opcode::MovEE
            }
            0x5c => {
                i += 1;
                Opcode::MovEH
            }
            0x5d => {
                i += 1;
                Opcode::MovEL
            }
            0x5e => {
                i += 1;
                Opcode::MovEM
            }
            0x5f => {
                i += 1;
                Opcode::MovEA
            }
            0x60 => {
                i += 1;
                Opcode::MovHB
            }
            0x61 => {
                i += 1;
                Opcode::MovHC
            }
            0x62 => {
                i += 1;
                Opcode::MovHD
            }
            0x63 => {
                i += 1;
                Opcode::MovHE
            }
            0x64 => {
                i += 1;
                Opcode::MovHH
            }
            0x65 => {
                i += 1;
                Opcode::MovHL
            }
            0x66 => {
                i += 1;
                Opcode::MovHM
            }
            0x67 => {
                i += 1;
                Opcode::MovHA
            }
            0x68 => {
                i += 1;
                Opcode::MovLB
            }
            0x69 => {
                i += 1;
                Opcode::MovLC
            }
            0x6a => {
                i += 1;
                Opcode::MovLD
            }
            0x6b => {
                i += 1;
                Opcode::MovLE
            }
            0x6c => {
                i += 1;
                Opcode::MovLH
            }
            0x6d => {
                i += 1;
                Opcode::MovLL
            }
            0x6e => {
                i += 1;
                Opcode::MovLM
            }
            0x6f => {
                i += 1;
                Opcode::MovLA
            }
            0x70 => {
                i += 1;
                Opcode::MovMB
            }
            0x71 => {
                i += 1;
                Opcode::MovMC
            }
            0x72 => {
                i += 1;
                Opcode::MovMD
            }
            0x73 => {
                i += 1;
                Opcode::MovME
            }
            0x74 => {
                i += 1;
                Opcode::MovMH
            }
            0x75 => {
                i += 1;
                Opcode::MovML
            }
            0x76 => {
                i += 1;
                Opcode::Hlt
            }
            0x77 => {
                i += 1;
                Opcode::MovMA
            }
            0x78 => {
                i += 1;
                Opcode::MovAB
            }
            0x79 => {
                i += 1;
                Opcode::MovAC
            }
            0x7a => {
                i += 1;
                Opcode::MovAD
            }
            0x7b => {
                i += 1;
                Opcode::MovAE
            }
            0x7c => {
                i += 1;
                Opcode::MovAH
            }
            0x7d => {
                i += 1;
                Opcode::MovAL
            }
            0x7e => {
                i += 1;
                Opcode::MovAM
            }
            0x7f => {
                i += 1;
                Opcode::MovAA
            }
            0x80 => {
                i += 1;
                Opcode::AddB
            }
            0x81 => {
                i += 1;
                Opcode::AddC
            }
            0x82 => {
                i += 1;
                Opcode::AddD
            }
            0x83 => {
                i += 1;
                Opcode::AddE
            }
            0x84 => {
                i += 1;
                Opcode::AddH
            }
            0x85 => {
                i += 1;
                Opcode::AddL
            }
            0x86 => {
                i += 1;
                Opcode::AddM
            }
            0x87 => {
                i += 1;
                Opcode::AddA
            }
            0x88 => {
                i += 1;
                Opcode::AdcB
            }
            0x89 => {
                i += 1;
                Opcode::AdcC
            }
            0x8a => {
                i += 1;
                Opcode::AdcD
            }
            0x8b => {
                i += 1;
                Opcode::AdcE
            }
            0x8c => {
                i += 1;
                Opcode::AdcH
            }
            0x8d => {
                i += 1;
                Opcode::AdcL
            }
            0x8e => {
                i += 1;
                Opcode::AdcM
            }
            0x8f => {
                i += 1;
                Opcode::AdcA
            }
            0x90 => {
                i += 1;
                Opcode::SubB
            }
            0x91 => {
                i += 1;
                Opcode::SubC
            }
            0x92 => {
                i += 1;
                Opcode::SubD
            }
            0x93 => {
                i += 1;
                Opcode::SubE
            }
            0x94 => {
                i += 1;
                Opcode::SubH
            }
            0x95 => {
                i += 1;
                Opcode::SubL
            }
            0x96 => {
                i += 1;
                Opcode::SubM
            }
            0x97 => {
                i += 1;
                Opcode::SubA
            }
            0x98 => {
                i += 1;
                Opcode::SbbB
            }
            0x99 => {
                i += 1;
                Opcode::SbbC
            }
            0x9a => {
                i += 1;
                Opcode::SbbD
            }
            0x9b => {
                i += 1;
                Opcode::SbbE
            }
            0x9c => {
                i += 1;
                Opcode::SbbH
            }
            0x9d => {
                i += 1;
                Opcode::SbbL
            }
            0x9e => {
                i += 1;
                Opcode::SbbM
            }
            0x9f => {
                i += 1;
                Opcode::SbbA
            }
            0xa0 => {
                i += 1;
                Opcode::AnaB
            }
            0xa1 => {
                i += 1;
                Opcode::AnaC
            }
            0xa2 => {
                i += 1;
                Opcode::AnaD
            }
            0xa3 => {
                i += 1;
                Opcode::AnaE
            }
            0xa4 => {
                i += 1;
                Opcode::AnaH
            }
            0xa5 => {
                i += 1;
                Opcode::AnaL
            }
            0xa6 => {
                i += 1;
                Opcode::AnaM
            }
            0xa7 => {
                i += 1;
                Opcode::AnaA
            }
            0xa8 => {
                i += 1;
                Opcode::XraB
            }
            0xa9 => {
                i += 1;
                Opcode::XraC
            }
            0xaa => {
                i += 1;
                Opcode::XraD
            }
            0xab => {
                i += 1;
                Opcode::XraE
            }
            0xac => {
                i += 1;
                Opcode::XraH
            }
            0xad => {
                i += 1;
                Opcode::XraL
            }
            0xae => {
                i += 1;
                Opcode::XraM
            }
            0xaf => {
                i += 1;
                Opcode::XraA
            }
            0xb0 => {
                i += 1;
                Opcode::OraB
            }
            0xb1 => {
                i += 1;
                Opcode::OraC
            }
            0xb2 => {
                i += 1;
                Opcode::OraD
            }
            0xb3 => {
                i += 1;
                Opcode::OraE
            }
            0xb4 => {
                i += 1;
                Opcode::OraH
            }
            0xb5 => {
                i += 1;
                Opcode::OraL
            }
            0xb6 => {
                i += 1;
                Opcode::OraM
            }
            0xb7 => {
                i += 1;
                Opcode::OraA
            }
            0xb8 => {
                i += 1;
                Opcode::CmpB
            }
            0xb9 => {
                i += 1;
                Opcode::CmpC
            }
            0xba => {
                i += 1;
                Opcode::CmpD
            }
            0xbb => {
                i += 1;
                Opcode::CmpE
            }
            0xbc => {
                i += 1;
                Opcode::CmpH
            }
            0xbd => {
                i += 1;
                Opcode::CmpL
            }
            0xbe => {
                i += 1;
                Opcode::CmpM
            }
            0xbf => {
                i += 1;
                Opcode::CmpA
            }
            0xc0 => {
                i += 1;
                Opcode::Rnz
            }
            0xc1 => {
                i += 1;
                Opcode::PopB
            }
            0xc2 => {
                i += 3;
                let b1 = bin.get(i - 2).ok_or(OpError(2))?;
                let b2 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::Jnz(*b1, *b2)
            }
            0xc3 => {
                i += 3;
                let b1 = bin.get(i - 2).ok_or(OpError(2))?;
                let b2 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::Jmp(*b1, *b2)
            }
            0xc4 => {
                i += 3;
                let b1 = bin.get(i - 2).ok_or(OpError(2))?;
                let b2 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::Cnz(*b1, *b2)
            }
            0xc5 => {
                i += 1;
                Opcode::PushB
            }
            0xc6 => {
                i += 1;
                Opcode::AdiD8
            }
            0xc7 => {
                i += 1;
                Opcode::Rst0
            }
            0xc8 => {
                i += 1;
                Opcode::Rz
            }
            0xc9 => {
                i += 1;
                Opcode::Ret
            }
            0xca => {
                i += 3;
                let b1 = bin.get(i - 2).ok_or(OpError(2))?;
                let b2 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::Jz(*b1, *b2)
            }
            0xcc => {
                i += 3;
                let b1 = bin.get(i - 2).ok_or(OpError(2))?;
                let b2 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::Cz(*b1, *b2)
            }
            0xcd => {
                i += 3;
                let b1 = bin.get(i - 2).ok_or(OpError(2))?;
                let b2 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::Call(*b1, *b2)
            }
            0xce => {
                i += 1;
                Opcode::AciD8
            }
            0xcf => {
                i += 1;
                Opcode::Rst1
            }
            0xd0 => {
                i += 1;
                Opcode::Rnc
            }
            0xd1 => {
                i += 1;
                Opcode::PopD
            }
            0xd2 => {
                i += 3;
                let b1 = bin.get(i - 2).ok_or(OpError(2))?;
                let b2 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::Jnc(*b1, *b2)
            }
            0xd3 => {
                i += 1;
                Opcode::OutD8
            }
            0xd4 => {
                i += 3;
                let b1 = bin.get(i - 2).ok_or(OpError(2))?;
                let b2 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::Cnc(*b1, *b2)
            }
            0xd5 => {
                i += 1;
                Opcode::PushD
            }
            0xd6 => {
                i += 1;
                Opcode::SuiD8
            }
            0xd7 => {
                i += 1;
                Opcode::Rst2
            }
            0xd8 => {
                i += 1;
                Opcode::Rc
            }
            0xda => {
                i += 3;
                let b1 = bin.get(i - 2).ok_or(OpError(2))?;
                let b2 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::Jc(*b1, *b2)
            }
            0xdb => {
                i += 1;
                Opcode::InD8
            }
            0xdc => {
                i += 3;
                let b1 = bin.get(i - 2).ok_or(OpError(2))?;
                let b2 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::Cc(*b1, *b2)
            }
            0xde => {
                i += 1;
                Opcode::SbiD8
            }
            0xdf => {
                i += 1;
                Opcode::Rst3
            }
            0xe0 => {
                i += 1;
                Opcode::Rpo
            }
            0xe1 => {
                i += 1;
                Opcode::PopH
            }
            0xe2 => {
                i += 3;
                let b1 = bin.get(i - 2).ok_or(OpError(2))?;
                let b2 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::Jpo(*b1, *b2)
            }
            0xe3 => {
                i += 1;
                Opcode::Xthl
            }
            0xe4 => {
                i += 3;
                let b1 = bin.get(i - 2).ok_or(OpError(2))?;
                let b2 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::Cpo(*b1, *b2)
            }
            0xe5 => {
                i += 1;
                Opcode::PushH
            }
            0xe6 => {
                i += 1;
                Opcode::AniD8
            }
            0xe7 => {
                i += 1;
                Opcode::Rst4
            }
            0xe8 => {
                i += 1;
                Opcode::Rpe
            }
            0xe9 => {
                i += 1;
                Opcode::Pchl
            }
            0xea => {
                i += 3;
                let b1 = bin.get(i - 2).ok_or(OpError(2))?;
                let b2 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::Jpe(*b1, *b2)
            }
            0xeb => {
                i += 1;
                Opcode::Xchg
            }
            0xec => {
                i += 3;
                let b1 = bin.get(i - 2).ok_or(OpError(2))?;
                let b2 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::Cpe(*b1, *b2)
            }
            0xee => {
                i += 1;
                Opcode::XriD8
            }
            0xef => {
                i += 1;
                Opcode::Rst5
            }
            0xf0 => {
                i += 1;
                Opcode::Rp
            }
            0xf1 => {
                i += 1;
                Opcode::PopPsw
            }
            0xf2 => {
                i += 3;
                let b1 = bin.get(i - 2).ok_or(OpError(2))?;
                let b2 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::Jp(*b1, *b2)
            }
            0xf3 => {
                i += 1;
                Opcode::Di
            }
            0xf4 => {
                i += 3;
                let b1 = bin.get(i - 2).ok_or(OpError(2))?;
                let b2 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::Cp(*b1, *b2)
            }
            0xf5 => {
                i += 1;
                Opcode::PushPsw
            }
            0xf6 => {
                i += 1;
                Opcode::OriD8
            }
            0xf7 => {
                i += 1;
                Opcode::Rst6
            }
            0xf8 => {
                i += 1;
                Opcode::Rm
            }
            0xf9 => {
                i += 1;
                Opcode::Sphl
            }
            0xfa => {
                i += 3;
                let b1 = bin.get(i - 2).ok_or(OpError(2))?;
                let b2 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::Jm(*b1, *b2)
            }
            0xfb => {
                i += 1;
                Opcode::Ei
            }
            0xfc => {
                i += 3;
                let b1 = bin.get(i - 2).ok_or(OpError(2))?;
                let b2 = bin.get(i - 1).ok_or(OpError(1))?;
                Opcode::Cm(*b1, *b2)
            }
            0xfe => {
                i += 1;
                Opcode::CpiD8
            }
            0xff => {
                i += 1;
                Opcode::Rst7
            }
            _ => {
                i += 1;
                Opcode::Nop
            }
        });
    }

    Ok(ops)
}
