use asm_8080::asm::{codegen, lexer::tokenize};
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    for arg in &args[1..] {
        let path = Path::new(&arg);

        if path.exists() {
            let src = fs::read_to_string(arg).unwrap();
            let ops = tokenize(&src).unwrap();
            let bin = codegen(&ops);

            let mut file = File::create("out.bin").unwrap();
            file.write(&bin).unwrap();
            println!("Generated {} bytes from {}.", bin.len(), arg);
        } else {
            eprintln!("{} doesn't exist.", arg);
        }
    }
}
