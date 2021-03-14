use asm_8080::dis::{disassemble, disassemble_raw};
use std::{env, fs, path::Path};

fn main() {
    let mut args = env::args().collect::<Vec<_>>();
    let raw = if args.len() > 1 && args[1] == "--raw" {
        args.remove(1);
        true
    } else {
        false
    };

    for arg in &args[1..] {
        let path = Path::new(&arg);

        if path.exists() {
            let bin = fs::read(arg).unwrap();

            if raw {
                if let Ok(out) = disassemble_raw(&bin) {
                    out.iter().for_each(|&x| print!("{:02x} ", x));
                    println!("\n{:?}", bin);
                } else {
                    eprintln!("Erroneous binary file.");
                }
            } else {
                if let Ok(out) = disassemble(&bin) {
                    println!("{:?}", out);
                } else {
                    eprintln!("Erroneous binary file.");
                }
            }
        } else {
            eprintln!("{} doesn't exist.", arg);
        }
    }
}
