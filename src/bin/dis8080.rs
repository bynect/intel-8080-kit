use intel_8080_kit::dis::{disassemble, disassemble_raw};
use std::{env, fs, path::Path};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let mut start = 1;

    let raw = if args.len() > 1 && args[1] == "--raw" {
        start += 1;
        true
    } else {
        false
    };

    for arg in &args[start..] {
        let path = Path::new(&arg);

        if path.exists() {
            let bin = fs::read(arg).unwrap();

            if raw {
                if let Ok(out) = disassemble_raw(&bin) {
                    println!("PC{:<8}OPCODE", "");

                    let mut i = 0;
                    while i < out.len() {
                        let op = out[i];
                        println!("{:04}{:<6}{:?}", i, "", op);
                        i += op.size();
                    }
                } else {
                    eprintln!("Erroneous binary file.");
                }
            } else {
                if let Ok(out) = disassemble(&bin) {
                    println!("PC{:<8}OPCODE", "");

                    let mut i = 0;
                    let mut pc = 0;
                    while i < out.len() {
                        let op = out[i];
                        println!("{:04}{:<6}{:?}", pc, "", op);
                        pc += op.size();
                        i += 1;
                    }
                } else {
                    eprintln!("Erroneous binary file.");
                }
            }
        } else {
            eprintln!("{} doesn't exist.", arg);
        }
    }
}
