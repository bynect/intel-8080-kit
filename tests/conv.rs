use asm_8080::op::RawOpcode;

#[test]
fn raw_conversion() {
    for i in 0..u8::MAX {
        let t: RawOpcode = i.into();

        match i {
            0x08 | 0x10 | 0x18 | 0x20 | 0x28 | 0x30 | 0x38 | 0xcb | 0xd9 | 0xdd | 0xed | 0xfd => {
                if RawOpcode::NOP != t {
                    panic!("{} ({}) != {}", i, RawOpcode::NOP, t);
                }
            }
            _ => {
                if i != t.into() {
                    panic!("{} != {}", i, t);
                }
            }
        }
    }
}
