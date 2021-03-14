use asm_8080::{
    asm::{codegen, lexer::tokenize},
    dis::{disassemble, disassemble_raw},
    op::{Opcode, RawOpcode},
};
use std::{fmt, fs, io::Result};

fn assert_eq_vec<T: PartialEq + fmt::Debug>(v1: &Vec<T>, v2: &Vec<T>) {
    let mut diff = Vec::new();

    if v1.len() == v2.len() {
        for (i, t) in v1.iter().enumerate() {
            if *t != v2[i] {
                diff.push((t, &v2[i]));
            }
        }

        if diff.len() != 0 {
            panic!(
                "Vectors {:?} and {:?} are different, differences: {:?}",
                v1, v2, diff
            );
        }
    } else {
        panic!("Vectors {:?} and {:?} are different", v1, v2);
    }
}

#[test]
fn test_mult() -> Result<()> {
    let bin = fs::read("tests/mult.bin")?;
    let out = disassemble(&bin).unwrap();
    assert_eq_vec(
        &out,
        &vec![
            Opcode::MviB(0),
            Opcode::MviE(9),
            Opcode::MovAC,
            Opcode::Rar,
            Opcode::MovCA,
            Opcode::DcrE,
            Opcode::Jz(21),
            Opcode::MovAB,
            Opcode::Jnc(16),
            Opcode::AddD,
            Opcode::Rar,
            Opcode::MovBA,
            Opcode::Jmp(4),
            Opcode::Ret,
        ],
    );

    let raw = disassemble_raw(&bin);
    assert_eq_vec(
        &raw.unwrap(),
        &vec![
            0x06u8, 0x1eu8, 0x79u8, 0x1fu8, 0x4fu8, 0x1du8, 0xcau8, 0x78u8, 0xd2u8, 0x82u8, 0x1fu8,
            0x47u8, 0xc3u8, 0xc9u8,
        ]
        .iter()
        .map(|t| {
            let x: RawOpcode = t.into();
            x
        })
        .collect(),
    );

    Ok(())
}

#[test]
fn test_codegen() -> Result<()> {
    let bin = fs::read("tests/mult.bin")?;
    let out = disassemble(&bin).unwrap();
    let gen = codegen(&out);
    assert_eq_vec(&bin, &gen);

    Ok(())
}

#[test]
fn test_lexer() -> Result<()> {
    let src = fs::read_to_string("tests/mult.asm")?;
    let bin = fs::read("tests/mult.bin")?;
    let ops = tokenize(&src).unwrap();
    let gen = codegen(&ops);
    assert_eq_vec(&bin, &gen);

    Ok(())
}
