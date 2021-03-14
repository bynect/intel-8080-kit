use asm_8080::op::{Opcode, RawOpcode};

#[test]
fn raw_conversion() {
    for i in 0..u8::MAX {
        let t: RawOpcode = i.into();
        if !i == t.into() {
            panic!()
        }
    }
}
