use intel_8080_kit::asm::{codegen, lexer::tokenize};
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
    process::Command,
    str,
};

const OUT_FILE: &str = "out.bin";

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let mut start = 1;

    let nopp = if args.len() > 1 && args[1] == "--no-pp" {
        start += 1;
        true
    } else {
        false
    };

    for arg in &args[start..] {
        let path = Path::new(&arg);

        if path.exists() {
            if nopp {
                let src = fs::read_to_string(arg).unwrap();
                let ops = tokenize(&src).unwrap();
                let bin = codegen(&ops);

                let mut file = File::create(OUT_FILE).unwrap();
                file.write(&bin).unwrap();
                println!("Emitted {} bytes to {} from {}.", bin.len(), OUT_FILE, arg);
            } else {
                if let Ok(sub) = Command::new("cpp").arg("-nostdinc").arg(path).output() {
                    if sub.status.success() {
                        let src = str::from_utf8(&sub.stdout).unwrap();

                        if let Ok(ops) = tokenize(&src) {
                            let bin = codegen(&ops);

                            let mut file = File::create(OUT_FILE).unwrap();
                            file.write(&bin).unwrap();
                            println!("Emitted {} bytes to {} from {}.", bin.len(), OUT_FILE, arg);
                        }
                    } else {
                        if let Ok(stderr) = str::from_utf8(&sub.stderr) {
                            println!("{}", stderr);
                        } else {
                            println!("Preprocessor error.");
                        }
                    }
                } else {
                    println!("Preprocessor error.");
                }
            }
        } else {
            eprintln!("{} doesn't exist.", arg);
        }
    }
}
