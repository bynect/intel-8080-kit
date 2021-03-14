use asm_8080::dis::{disassemble, disassemble_raw};

#[test]
fn malformed_raw() {
    disassemble_raw(&vec![0x01u8]).unwrap_err();
    disassemble_raw(&vec![0x0eu8]).unwrap_err();
    disassemble_raw(&vec![0x16u8]).unwrap_err();
    disassemble_raw(&vec![0xb2u8]).unwrap();
}

#[test]
fn malformed() {
    disassemble(&vec![0x01u8]).unwrap_err();
    disassemble(&vec![0x0eu8]).unwrap_err();
    disassemble(&vec![0x16u8]).unwrap_err();
    disassemble(&vec![0xb2u8]).unwrap();
}

#[test]
fn malformed_all() {
    // should not panic
    for i in 0..u8::MAX {
        let _ = disassemble(&vec![i]);
        let _ = disassemble_raw(&vec![i]);
    }
}
