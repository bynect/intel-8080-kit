use std::fmt;

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawOpcode {
    NOP = 0x00,
    LXI_B = 0x01,
    STAX_B = 0x02,
    INX_B = 0x03,
    INR_B = 0x04,
    DCR_B = 0x05,
    MVI_B = 0x06,
    RLC = 0x07,
    DAD_B = 0x09,
    LDAX_B = 0x0a,
    DCX_B = 0x0b,
    INR_C = 0x0c,
    DCR_C = 0x0d,
    MVI_C = 0x0e,
    RRC = 0x0f,
    LXI_D = 0x11,
    STAX_D = 0x12,
    INX_D = 0x13,
    INR_D = 0x14,
    DCR_D = 0x15,
    MVI_D = 0x16,
    RAL = 0x17,
    DAD_D = 0x19,
    LDAX_D = 0x1a,
    DCX_D = 0x1b,
    INR_E = 0x1c,
    DCR_E = 0x1d,
    MVI_E = 0x1e,
    RAR = 0x1f,
    LXI_H = 0x21,
    SHLD = 0x22,
    INX_H = 0x23,
    INR_H = 0x24,
    DCR_H = 0x25,
    MVI_H = 0x26,
    DAA = 0x27,
    DAD_H = 0x29,
    LHLD = 0x2a,
    DCX_H = 0x2b,
    INR_L = 0x2c,
    DCR_L = 0x2d,
    MVI_L = 0x2e,
    CMA = 0x2f,
    LXI_SP = 0x31,
    STA = 0x32,
    INX_SP = 0x33,
    INR_M = 0x34,
    DCR_M = 0x35,
    MVI_M = 0x36,
    STC = 0x37,
    DAD_SP = 0x39,
    LDA = 0x3a,
    DCX_SP = 0x3b,
    INR_A = 0x3c,
    DCR_A = 0x3d,
    MVI_A = 0x3e,
    CMC = 0x3f,
    MOV_B_B = 0x40,
    MOV_B_C = 0x41,
    MOV_B_D = 0x42,
    MOV_B_E = 0x43,
    MOV_B_H = 0x44,
    MOV_B_L = 0x45,
    MOV_B_M = 0x46,
    MOV_B_A = 0x47,
    MOV_C_B = 0x48,
    MOV_C_C = 0x49,
    MOV_C_D = 0x4a,
    MOV_C_E = 0x4b,
    MOV_C_H = 0x4c,
    MOV_C_L = 0x4d,
    MOV_C_M = 0x4e,
    MOV_C_A = 0x4f,
    MOV_D_B = 0x50,
    MOV_D_C = 0x51,
    MOV_D_D = 0x52,
    MOV_D_E = 0x53,
    MOV_D_H = 0x54,
    MOV_D_L = 0x55,
    MOV_D_M = 0x56,
    MOV_D_A = 0x57,
    MOV_E_B = 0x58,
    MOV_E_C = 0x59,
    MOV_E_D = 0x5a,
    MOV_E_E = 0x5b,
    MOV_E_H = 0x5c,
    MOV_E_L = 0x5d,
    MOV_E_M = 0x5e,
    MOV_E_A = 0x5f,
    MOV_H_B = 0x60,
    MOV_H_C = 0x61,
    MOV_H_D = 0x62,
    MOV_H_E = 0x63,
    MOV_H_H = 0x64,
    MOV_H_L = 0x65,
    MOV_H_M = 0x66,
    MOV_H_A = 0x67,
    MOV_L_B = 0x68,
    MOV_L_C = 0x69,
    MOV_L_D = 0x6a,
    MOV_L_E = 0x6b,
    MOV_L_H = 0x6c,
    MOV_L_L = 0x6d,
    MOV_L_M = 0x6e,
    MOV_L_A = 0x6f,
    MOV_M_B = 0x70,
    MOV_M_C = 0x71,
    MOV_M_D = 0x72,
    MOV_M_E = 0x73,
    MOV_M_H = 0x74,
    MOV_M_L = 0x75,
    HLT = 0x76,
    MOV_M_A = 0x77,
    MOV_A_B = 0x78,
    MOV_A_C = 0x79,
    MOV_A_D = 0x7a,
    MOV_A_E = 0x7b,
    MOV_A_H = 0x7c,
    MOV_A_L = 0x7d,
    MOV_A_M = 0x7e,
    MOV_A_A = 0x7f,
    ADD_B = 0x80,
    ADD_C = 0x81,
    ADD_D = 0x82,
    ADD_E = 0x83,
    ADD_H = 0x84,
    ADD_L = 0x85,
    ADD_M = 0x86,
    ADD_A = 0x87,
    ADC_B = 0x88,
    ADC_C = 0x89,
    ADC_D = 0x8a,
    ADC_E = 0x8b,
    ADC_H = 0x8c,
    ADC_L = 0x8d,
    ADC_M = 0x8e,
    ADC_A = 0x8f,
    SUB_B = 0x90,
    SUB_C = 0x91,
    SUB_D = 0x92,
    SUB_E = 0x93,
    SUB_H = 0x94,
    SUB_L = 0x95,
    SUB_M = 0x96,
    SUB_A = 0x97,
    SBB_B = 0x98,
    SBB_C = 0x99,
    SBB_D = 0x9a,
    SBB_E = 0x9b,
    SBB_H = 0x9c,
    SBB_L = 0x9d,
    SBB_M = 0x9e,
    SBB_A = 0x9f,
    ANA_B = 0xa0,
    ANA_C = 0xa1,
    ANA_D = 0xa2,
    ANA_E = 0xa3,
    ANA_H = 0xa4,
    ANA_L = 0xa5,
    ANA_M = 0xa6,
    ANA_A = 0xa7,
    XRA_B = 0xa8,
    XRA_C = 0xa9,
    XRA_D = 0xaa,
    XRA_E = 0xab,
    XRA_H = 0xac,
    XRA_L = 0xad,
    XRA_M = 0xae,
    XRA_A = 0xaf,
    ORA_B = 0xb0,
    ORA_C = 0xb1,
    ORA_D = 0xb2,
    ORA_E = 0xb3,
    ORA_H = 0xb4,
    ORA_L = 0xb5,
    ORA_M = 0xb6,
    ORA_A = 0xb7,
    CMP_B = 0xb8,
    CMP_C = 0xb9,
    CMP_D = 0xba,
    CMP_E = 0xbb,
    CMP_H = 0xbc,
    CMP_L = 0xbd,
    CMP_M = 0xbe,
    CMP_A = 0xbf,
    RNZ = 0xc0,
    POP_B = 0xc1,
    JNZ = 0xc2,
    JMP = 0xc3,
    CNZ = 0xc4,
    PUSH_B = 0xc5,
    ADI = 0xc6,
    RST_0 = 0xc7,
    RZ = 0xc8,
    RET = 0xc9,
    JZ = 0xca,
    CZ = 0xcc,
    CALL = 0xcd,
    ACI = 0xce,
    RST_1 = 0xcf,
    RNC = 0xd0,
    POP_D = 0xd1,
    JNC = 0xd2,
    OUT = 0xd3,
    CNC = 0xd4,
    PUSH_D = 0xd5,
    SUI = 0xd6,
    RST_2 = 0xd7,
    RC = 0xd8,
    JC = 0xda,
    IN = 0xdb,
    CC = 0xdc,
    SBI = 0xde,
    RST_3 = 0xdf,
    RPO = 0xe0,
    POP_H = 0xe1,
    JPO = 0xe2,
    XTHL = 0xe3,
    CPO = 0xe4,
    PUSH_H = 0xe5,
    ANI = 0xe6,
    RST_4 = 0xe7,
    RPE = 0xe8,
    PCHL = 0xe9,
    JPE = 0xea,
    XCHG = 0xeb,
    CPE = 0xec,
    XRI = 0xee,
    RST_5 = 0xef,
    RP = 0xf0,
    POP_PSW = 0xf1,
    JP = 0xf2,
    DI = 0xf3,
    CP = 0xf4,
    PUSH_PSW = 0xf5,
    ORI = 0xf6,
    RST_6 = 0xf7,
    RM = 0xf8,
    SPHL = 0xf9,
    JM = 0xfa,
    EI = 0xfb,
    CM = 0xfc,
    CPI = 0xfe,
    RST_7 = 0xff,
}

impl From<u8> for RawOpcode {
    fn from(t: u8) -> RawOpcode {
        match t {
            0x08 | 0x10 | 0x18 | 0x20 | 0x28 | 0x30 | 0x38 | 0xcb | 0xd9 | 0xdd | 0xed | 0xfd => {
                RawOpcode::NOP
            }
            _ => unsafe { std::mem::transmute(t) },
        }
    }
}

impl From<&u8> for RawOpcode {
    fn from(t: &u8) -> RawOpcode {
        From::from(*t)
    }
}

impl Into<u8> for RawOpcode {
    fn into(self) -> u8 {
        unsafe { std::mem::transmute(self) }
    }
}

impl fmt::Display for RawOpcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}(0x{:02x?})", self, *self as u8)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Nop,
    LxiB(u8, u8),
    StaxB,
    InxB,
    InrB,
    DcrB,
    MviB(u8),
    Rlc,
    DadB,
    LdaxB,
    DcxB,
    InrC,
    DcrC,
    MviC(u8),
    Rrc,
    LxiD(u8, u8),
    StaxD,
    InxD,
    InrD,
    DcrD,
    MviD(u8),
    Ral,
    DadD,
    LdaxD,
    DcxD,
    InrE,
    DcrE,
    MviE(u8),
    Rar,
    LxiH(u8, u8),
    Shld(u8, u8),
    InxH,
    InrH,
    DcrH,
    MviH(u8),
    Daa,
    DadH,
    Lhld(u8, u8),
    DcxH,
    InrL,
    DcrL,
    MviL(u8),
    Cma,
    LxiSpD16,
    Sta(u8, u8),
    InxSp,
    InrM,
    DcrM,
    MviM(u8),
    Stc,
    DadSp,
    Lda(u8, u8),
    DcxSp,
    InrA,
    DcrA,
    MviA(u8),
    Cmc,
    MovBB,
    MovBC,
    MovBD,
    MovBE,
    MovBH,
    MovBL,
    MovBM,
    MovBA,
    MovCB,
    MovCC,
    MovCD,
    MovCE,
    MovCH,
    MovCL,
    MovCM,
    MovCA,
    MovDB,
    MovDC,
    MovDD,
    MovDE,
    MovDH,
    MovDL,
    MovDM,
    MovDA,
    MovEB,
    MovEC,
    MovED,
    MovEE,
    MovEH,
    MovEL,
    MovEM,
    MovEA,
    MovHB,
    MovHC,
    MovHD,
    MovHE,
    MovHH,
    MovHL,
    MovHM,
    MovHA,
    MovLB,
    MovLC,
    MovLD,
    MovLE,
    MovLH,
    MovLL,
    MovLM,
    MovLA,
    MovMB,
    MovMC,
    MovMD,
    MovME,
    MovMH,
    MovML,
    Hlt,
    MovMA,
    MovAB,
    MovAC,
    MovAD,
    MovAE,
    MovAH,
    MovAL,
    MovAM,
    MovAA,
    AddB,
    AddC,
    AddD,
    AddE,
    AddH,
    AddL,
    AddM,
    AddA,
    AdcB,
    AdcC,
    AdcD,
    AdcE,
    AdcH,
    AdcL,
    AdcM,
    AdcA,
    SubB,
    SubC,
    SubD,
    SubE,
    SubH,
    SubL,
    SubM,
    SubA,
    SbbB,
    SbbC,
    SbbD,
    SbbE,
    SbbH,
    SbbL,
    SbbM,
    SbbA,
    AnaB,
    AnaC,
    AnaD,
    AnaE,
    AnaH,
    AnaL,
    AnaM,
    AnaA,
    XraB,
    XraC,
    XraD,
    XraE,
    XraH,
    XraL,
    XraM,
    XraA,
    OraB,
    OraC,
    OraD,
    OraE,
    OraH,
    OraL,
    OraM,
    OraA,
    CmpB,
    CmpC,
    CmpD,
    CmpE,
    CmpH,
    CmpL,
    CmpM,
    CmpA,
    Rnz,
    PopB,
    Jnz(u8, u8),
    Jmp(u8, u8),
    Cnz(u8, u8),
    PushB,
    AdiD8,
    Rst0,
    Rz,
    Ret,
    Jz(u8, u8),
    Cz(u8, u8),
    Call(u8, u8),
    AciD8,
    Rst1,
    Rnc,
    PopD,
    Jnc(u8, u8),
    OutD8,
    Cnc(u8, u8),
    PushD,
    SuiD8,
    Rst2,
    Rc,
    Jc(u8, u8),
    InD8,
    Cc(u8, u8),
    SbiD8,
    Rst3,
    Rpo,
    PopH,
    Jpo(u8, u8),
    Xthl,
    Cpo(u8, u8),
    PushH,
    AniD8,
    Rst4,
    Rpe,
    Pchl,
    Jpe(u8, u8),
    Xchg,
    Cpe(u8, u8),
    XriD8,
    Rst5,
    Rp,
    PopPsw,
    Jp(u8, u8),
    Di,
    Cp(u8, u8),
    PushPsw,
    OriD8,
    Rst6,
    Rm,
    Sphl,
    Jm(u8, u8),
    Ei,
    Cm(u8, u8),
    CpiD8,
    Rst7,
}
