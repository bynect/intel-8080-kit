use super::Opcode;
use std::collections::HashMap;

pub fn tokenize(src: &str) -> Result<Vec<Opcode>, ()> {
    let mut out = Vec::new();

    let mut err = false;
    let mut err2 = false;
    let mut err3 = false;

    let mut s = String::new();

    for line in src.lines() {
        if let Some(com) = line.find('#') {
            s.push_str(&line[..com]);
        } else if let Some(com) = line.find(';') {
            s.push_str(&line[..com]);
        } else {
            s.push_str(&line[..]);
        }
        s.push('\n');
    }

    let mut words = s.split_ascii_whitespace();

    let mut pc = 0;
    let mut labels = HashMap::new();
    let mut defined = HashMap::new();

    let mut next_byte = |s: &str| {
        if let Ok(v) = s.parse::<u8>() {
            v
        } else {
            eprintln!("Expected byte instead of {}.", s);
            err2 = true;
            0u8
        }
    };

    let mut next_short = |s: &str| {
        if let Ok(v) = s.parse::<u16>() {
            v
        } else {
            eprintln!("Expected short instead of {}.", s);
            err3 = true;
            0u16
        }
    };

    let next_address = |idx: usize,
                        pc: u16,
                        s: &str,
                        h: &mut HashMap<String, Vec<usize>>,
                        h2: &mut HashMap<String, u16>| {
        if let Ok(v) = s.parse::<u16>() {
            v
        } else if s == "$" {
            pc - 3
        } else if let Some(v) = h2.get(s) {
            *v as u16
        } else if let Some(vec) = h.get_mut(s) {
            vec.push(idx);
            0u16
        } else {
            h.insert(s.into(), vec![idx]);
            0u16
        }
    };

    while let Some(w) = words.next() {
        let w = w.to_ascii_lowercase();

        if w.ends_with(':') {
            if defined.insert(w[..w.len() - 1].to_string(), pc).is_some() {
                eprintln!("Labels redefined ({}).", &w[..w.len() - 1]);
                err = true;
            }
        } else {
            match &w[..] {
                "nop" => {
                    pc += 1;
                    out.push(Opcode::Nop);
                }
                "rlc" => {
                    pc += 1;
                    out.push(Opcode::Rlc);
                }
                "rrc" => {
                    pc += 1;
                    out.push(Opcode::Rrc);
                }
                "ral" => {
                    pc += 1;
                    out.push(Opcode::Ral);
                }
                "rar" => {
                    pc += 1;
                    out.push(Opcode::Rar);
                }
                "shld" => {
                    pc += 3;
                    out.push(Opcode::Shld(next_address(
                        out.len(),
                        pc,
                        words.next().unwrap(),
                        &mut labels,
                        &mut defined,
                    )));
                }
                "daa" => {
                    pc += 1;
                    out.push(Opcode::Daa);
                }
                "lhld" => {
                    pc += 3;
                    out.push(Opcode::Lhld(next_address(
                        out.len(),
                        pc,
                        words.next().unwrap(),
                        &mut labels,
                        &mut defined,
                    )));
                }
                "cma" => {
                    pc += 1;
                    out.push(Opcode::Cma);
                }
                "sta" => {
                    pc += 3;
                    out.push(Opcode::Sta(next_address(
                        out.len(),
                        pc,
                        words.next().unwrap(),
                        &mut labels,
                        &mut defined,
                    )));
                }
                "stc" => {
                    pc += 1;
                    out.push(Opcode::Stc);
                }
                "lda" => {
                    pc += 3;
                    out.push(Opcode::Lda(next_address(
                        out.len(),
                        pc,
                        words.next().unwrap(),
                        &mut labels,
                        &mut defined,
                    )));
                }
                "cmc" => {
                    pc += 1;
                    out.push(Opcode::Cmc);
                }
                "hlt" => {
                    pc += 1;
                    out.push(Opcode::Hlt);
                }
                "rnz" => {
                    pc += 1;
                    out.push(Opcode::Rnz);
                }
                "jnz" => {
                    pc += 3;
                    out.push(Opcode::Jnz(next_address(
                        out.len(),
                        pc,
                        words.next().unwrap(),
                        &mut labels,
                        &mut defined,
                    )));
                }
                "jmp" => {
                    pc += 3;
                    out.push(Opcode::Jmp(next_address(
                        out.len(),
                        pc,
                        words.next().unwrap(),
                        &mut labels,
                        &mut defined,
                    )));
                }
                "cnz" => {
                    pc += 3;
                    out.push(Opcode::Cnz(next_address(
                        out.len(),
                        pc,
                        words.next().unwrap(),
                        &mut labels,
                        &mut defined,
                    )));
                }
                "rz" => {
                    pc += 1;
                    out.push(Opcode::Rz);
                }
                "ret" => {
                    pc += 1;
                    out.push(Opcode::Ret);
                }
                "jz" => {
                    pc += 3;
                    out.push(Opcode::Jz(next_address(
                        out.len(),
                        pc,
                        words.next().unwrap(),
                        &mut labels,
                        &mut defined,
                    )));
                }
                "cz" => {
                    pc += 3;
                    out.push(Opcode::Cz(next_address(
                        out.len(),
                        pc,
                        words.next().unwrap(),
                        &mut labels,
                        &mut defined,
                    )));
                }
                "call" => {
                    pc += 3;
                    out.push(Opcode::Call(next_address(
                        out.len(),
                        pc,
                        words.next().unwrap(),
                        &mut labels,
                        &mut defined,
                    )));
                }
                "rnc" => {
                    pc += 1;
                    out.push(Opcode::Rnc);
                }
                "jnc" => {
                    pc += 3;
                    out.push(Opcode::Jnc(next_address(
                        out.len(),
                        pc,
                        words.next().unwrap(),
                        &mut labels,
                        &mut defined,
                    )));
                }
                "cnc" => {
                    pc += 3;
                    out.push(Opcode::Cnc(next_address(
                        out.len(),
                        pc,
                        words.next().unwrap(),
                        &mut labels,
                        &mut defined,
                    )));
                }
                "rc" => {
                    pc += 1;
                    out.push(Opcode::Rc);
                }
                "jc" => {
                    pc += 3;
                    out.push(Opcode::Jc(next_address(
                        out.len(),
                        pc,
                        words.next().unwrap(),
                        &mut labels,
                        &mut defined,
                    )));
                }
                "cc" => {
                    pc += 3;
                    out.push(Opcode::Cc(next_address(
                        out.len(),
                        pc,
                        words.next().unwrap(),
                        &mut labels,
                        &mut defined,
                    )));
                }
                "rpo" => {
                    pc += 1;
                    out.push(Opcode::Rpo);
                }
                "jpo" => {
                    pc += 3;
                    out.push(Opcode::Jpo(next_address(
                        out.len(),
                        pc,
                        words.next().unwrap(),
                        &mut labels,
                        &mut defined,
                    )));
                }
                "xthl" => {
                    pc += 1;
                    out.push(Opcode::Xthl);
                }
                "cpo" => {
                    pc += 3;
                    out.push(Opcode::Cpo(next_address(
                        out.len(),
                        pc,
                        words.next().unwrap(),
                        &mut labels,
                        &mut defined,
                    )));
                }
                "rpe" => {
                    pc += 1;
                    out.push(Opcode::Rpe);
                }
                "pchl" => {
                    pc += 1;
                    out.push(Opcode::Pchl);
                }
                "jpe" => {
                    pc += 3;
                    out.push(Opcode::Jpe(next_address(
                        out.len(),
                        pc,
                        words.next().unwrap(),
                        &mut labels,
                        &mut defined,
                    )));
                }
                "xchg" => {
                    pc += 1;
                    out.push(Opcode::Xchg);
                }
                "cpe" => {
                    pc += 3;
                    out.push(Opcode::Cpe(next_address(
                        out.len(),
                        pc,
                        words.next().unwrap(),
                        &mut labels,
                        &mut defined,
                    )));
                }
                "rp" => {
                    pc += 1;
                    out.push(Opcode::Rp);
                }
                "jp" => {
                    pc += 3;
                    out.push(Opcode::Jp(next_address(
                        out.len(),
                        pc,
                        words.next().unwrap(),
                        &mut labels,
                        &mut defined,
                    )));
                }
                "di" => {
                    pc += 1;
                    out.push(Opcode::Di);
                }
                "cp" => {
                    pc += 3;
                    out.push(Opcode::Cp(next_address(
                        out.len(),
                        pc,
                        words.next().unwrap(),
                        &mut labels,
                        &mut defined,
                    )));
                }
                "rm" => {
                    pc += 1;
                    out.push(Opcode::Rm);
                }
                "sphl" => {
                    pc += 1;
                    out.push(Opcode::Sphl);
                }
                "jm" => {
                    pc += 3;
                    out.push(Opcode::Jm(next_address(
                        out.len(),
                        pc,
                        words.next().unwrap(),
                        &mut labels,
                        &mut defined,
                    )));
                }
                "ei" => {
                    pc += 1;
                    out.push(Opcode::Ei);
                }
                "cm" => {
                    pc += 3;
                    out.push(Opcode::Cm(next_address(
                        out.len(),
                        pc,
                        words.next().unwrap(),
                        &mut labels,
                        &mut defined,
                    )));
                }

                "ani" => {
                    pc += 2;
                    out.push(Opcode::Ani(next_byte(words.next().unwrap())));
                }
                "adi" => {
                    pc += 2;
                    out.push(Opcode::Adi(next_byte(words.next().unwrap())));
                }
                "aci" => {
                    pc += 2;
                    out.push(Opcode::Aci(next_byte(words.next().unwrap())));
                }
                "out" => {
                    pc += 2;
                    out.push(Opcode::Out(next_byte(words.next().unwrap())));
                }
                "sui" => {
                    pc += 2;
                    out.push(Opcode::Sui(next_byte(words.next().unwrap())));
                }
                "in" => {
                    pc += 2;
                    out.push(Opcode::In(next_byte(words.next().unwrap())));
                }
                "sbi" => {
                    pc += 2;
                    out.push(Opcode::Sbi(next_byte(words.next().unwrap())));
                }
                "xri" => {
                    pc += 2;
                    out.push(Opcode::Xri(next_byte(words.next().unwrap())));
                }
                "ori" => {
                    pc += 2;
                    out.push(Opcode::Ori(next_byte(words.next().unwrap())));
                }
                "cpi" => {
                    pc += 2;
                    out.push(Opcode::Cpi(next_byte(words.next().unwrap())));
                }

                "ldax" => {
                    pc += 1;
                    let w2 = words.next().unwrap().trim();

                    match w2 {
                        "b" => out.push(Opcode::LdaxB),
                        "d" => out.push(Opcode::LdaxD),
                        x => {
                            eprintln!("Unknown operand for ldax ({}).", x);
                            err = true;
                        }
                    }
                }
                "lxi" => {
                    pc += 3;
                    let mut w2 = words.next().unwrap().trim();
                    if let Some(w3) = w2.strip_suffix(',') {
                        w2 = w3.trim();
                    }

                    match w2 {
                        "b" => out.push(Opcode::LxiB(
                            next_byte(words.next().unwrap()),
                            next_byte(words.next().unwrap()),
                        )),
                        "d" => out.push(Opcode::LxiD(
                            next_byte(words.next().unwrap()),
                            next_byte(words.next().unwrap()),
                        )),
                        "h" => out.push(Opcode::LxiH(
                            next_byte(words.next().unwrap()),
                            next_byte(words.next().unwrap()),
                        )),
                        x => {
                            eprintln!("Unknown operand for lxi ({}).", x);
                            err = true;
                        }
                    }
                }
                "stax" => {
                    pc += 1;
                    let w2 = words.next().unwrap().trim();

                    match w2 {
                        "b" => out.push(Opcode::StaxB),
                        "d" => out.push(Opcode::StaxD),
                        x => {
                            eprintln!("Unknown operand for stax ({}).", x);
                            err = true;
                        }
                    }
                }
                "pop" => {
                    pc += 1;
                    let w2 = words.next().unwrap().trim();

                    match w2 {
                        "b" => out.push(Opcode::PopB),
                        "d" => out.push(Opcode::PopD),
                        "h" => out.push(Opcode::PopH),
                        "psw" => out.push(Opcode::PopPsw),
                        x => {
                            eprintln!("Unknown operand for pop ({}).", x);
                            err = true;
                        }
                    }
                }
                "push" => {
                    pc += 1;
                    let w2 = words.next().unwrap().trim();

                    match w2 {
                        "b" => out.push(Opcode::PushB),
                        "d" => out.push(Opcode::PushD),
                        "h" => out.push(Opcode::PushH),
                        "psw" => out.push(Opcode::PushPsw),
                        x => {
                            eprintln!("Unknown operand for push ({}).", x);
                            err = true;
                        }
                    }
                }
                "dad" => {
                    pc += 1;
                    let w2 = words.next().unwrap().trim();

                    match w2 {
                        "b" => out.push(Opcode::DadB),
                        "d" => out.push(Opcode::DadD),
                        "h" => out.push(Opcode::DadH),
                        "sp" => out.push(Opcode::DadSp),
                        x => {
                            eprintln!("Unknown operand for dad ({}).", x);
                            err = true;
                        }
                    }
                }
                "inx" => {
                    pc += 1;
                    let w2 = words.next().unwrap().trim();

                    match w2 {
                        "b" => out.push(Opcode::InxB),
                        "d" => out.push(Opcode::InxD),
                        "h" => out.push(Opcode::InxH),
                        "sp" => out.push(Opcode::InxSp),
                        x => {
                            eprintln!("Unknown operand for inx ({}).", x);
                            err = true;
                        }
                    }
                }
                "dcx" => {
                    pc += 1;
                    let w2 = words.next().unwrap().trim();

                    match w2 {
                        "b" => out.push(Opcode::DcxB),
                        "d" => out.push(Opcode::DcxD),
                        "h" => out.push(Opcode::DcxH),
                        "sp" => out.push(Opcode::DcxSp),
                        x => {
                            eprintln!("Unknown operand for dcx ({}).", x);
                            err = true;
                        }
                    }
                }
                "inr" => {
                    pc += 1;
                    let w2 = words.next().unwrap().trim();

                    match w2 {
                        "b" => out.push(Opcode::InrB),
                        "c" => out.push(Opcode::InrC),
                        "d" => out.push(Opcode::InrD),
                        "e" => out.push(Opcode::InrE),
                        "h" => out.push(Opcode::InrH),
                        "l" => out.push(Opcode::InrL),
                        "m" => out.push(Opcode::InrM),
                        "a" => out.push(Opcode::InrA),
                        x => {
                            eprintln!("Unknown operand for inr ({}).", x);
                            err = true;
                        }
                    }
                }
                "dcr" => {
                    pc += 1;
                    let w2 = words.next().unwrap().trim();

                    match w2 {
                        "b" => out.push(Opcode::DcrB),
                        "c" => out.push(Opcode::DcrC),
                        "d" => out.push(Opcode::DcrD),
                        "e" => out.push(Opcode::DcrE),
                        "h" => out.push(Opcode::DcrH),
                        "l" => out.push(Opcode::DcrL),
                        "m" => out.push(Opcode::DcrM),
                        "a" => out.push(Opcode::DcrA),
                        x => {
                            eprintln!("Unknown operand for dcr ({}).", x);
                            err = true;
                        }
                    }
                }
                "add" => {
                    pc += 1;
                    let w2 = words.next().unwrap().trim();

                    match w2 {
                        "b" => out.push(Opcode::AddB),
                        "c" => out.push(Opcode::AddC),
                        "d" => out.push(Opcode::AddD),
                        "e" => out.push(Opcode::AddE),
                        "h" => out.push(Opcode::AddH),
                        "l" => out.push(Opcode::AddL),
                        "m" => out.push(Opcode::AddM),
                        "a" => out.push(Opcode::AddA),
                        x => {
                            eprintln!("Unknown operand for add ({}).", x);
                            err = true;
                        }
                    }
                }
                "adc" => {
                    pc += 1;
                    let w2 = words.next().unwrap().trim();

                    match w2 {
                        "b" => out.push(Opcode::AdcB),
                        "c" => out.push(Opcode::AdcC),
                        "d" => out.push(Opcode::AdcD),
                        "e" => out.push(Opcode::AdcE),
                        "h" => out.push(Opcode::AdcH),
                        "l" => out.push(Opcode::AdcL),
                        "m" => out.push(Opcode::AdcM),
                        "a" => out.push(Opcode::AdcA),
                        x => {
                            eprintln!("Unknown operand for adc ({}).", x);
                            err = true;
                        }
                    }
                }
                "sub" => {
                    pc += 1;
                    let w2 = words.next().unwrap().trim();

                    match w2 {
                        "b" => out.push(Opcode::SubB),
                        "c" => out.push(Opcode::SubC),
                        "d" => out.push(Opcode::SubD),
                        "e" => out.push(Opcode::SubE),
                        "h" => out.push(Opcode::SubH),
                        "l" => out.push(Opcode::SubL),
                        "m" => out.push(Opcode::SubM),
                        "a" => out.push(Opcode::SubA),
                        x => {
                            eprintln!("Unknown operand for sub ({}).", x);
                            err = true;
                        }
                    }
                }
                "sbb" => {
                    pc += 1;
                    let w2 = words.next().unwrap().trim();

                    match w2 {
                        "b" => out.push(Opcode::SbbB),
                        "c" => out.push(Opcode::SbbC),
                        "d" => out.push(Opcode::SbbD),
                        "e" => out.push(Opcode::SbbE),
                        "h" => out.push(Opcode::SbbH),
                        "l" => out.push(Opcode::SbbL),
                        "m" => out.push(Opcode::SbbM),
                        "a" => out.push(Opcode::SbbA),
                        x => {
                            eprintln!("Unknown operand for sbb ({}).", x);
                            err = true;
                        }
                    }
                }
                "ana" => {
                    pc += 1;
                    let w2 = words.next().unwrap().trim();

                    match w2 {
                        "b" => out.push(Opcode::AnaB),
                        "c" => out.push(Opcode::AnaC),
                        "d" => out.push(Opcode::AnaD),
                        "e" => out.push(Opcode::AnaE),
                        "h" => out.push(Opcode::AnaH),
                        "l" => out.push(Opcode::AnaL),
                        "m" => out.push(Opcode::AnaM),
                        "a" => out.push(Opcode::AnaA),
                        x => {
                            eprintln!("Unknown operand for ana ({}).", x);
                            err = true;
                        }
                    }
                }
                "xra" => {
                    pc += 1;
                    let w2 = words.next().unwrap().trim();

                    match w2 {
                        "b" => out.push(Opcode::XraB),
                        "c" => out.push(Opcode::XraC),
                        "d" => out.push(Opcode::XraD),
                        "e" => out.push(Opcode::XraE),
                        "h" => out.push(Opcode::XraH),
                        "l" => out.push(Opcode::XraL),
                        "m" => out.push(Opcode::XraM),
                        "a" => out.push(Opcode::XraA),
                        x => {
                            eprintln!("Unknown operand for xra ({}).", x);
                            err = true;
                        }
                    }
                }
                "ora" => {
                    pc += 1;
                    let w2 = words.next().unwrap().trim();

                    match w2 {
                        "b" => out.push(Opcode::OraB),
                        "c" => out.push(Opcode::OraC),
                        "d" => out.push(Opcode::OraD),
                        "e" => out.push(Opcode::OraE),
                        "h" => out.push(Opcode::OraH),
                        "l" => out.push(Opcode::OraL),
                        "m" => out.push(Opcode::OraM),
                        "a" => out.push(Opcode::OraA),
                        x => {
                            eprintln!("Unknown operand for ora ({}).", x);
                            err = true;
                        }
                    }
                }
                "cmp" => {
                    pc += 1;
                    let w2 = words.next().unwrap().trim();

                    match w2 {
                        "b" => out.push(Opcode::CmpB),
                        "c" => out.push(Opcode::CmpC),
                        "d" => out.push(Opcode::CmpD),
                        "e" => out.push(Opcode::CmpE),
                        "h" => out.push(Opcode::CmpH),
                        "l" => out.push(Opcode::CmpL),
                        "m" => out.push(Opcode::CmpM),
                        "a" => out.push(Opcode::CmpA),
                        x => {
                            eprintln!("Unknown operand for cmp ({}).", x);
                            err = true;
                        }
                    }
                }
                "mov" => {
                    pc += 1;
                    let mut w2 = words.next().unwrap().trim();
                    if let Some(w3) = w2.strip_suffix(',') {
                        w2 = w3.trim();
                    }

                    match w2 {
                        "b" => {
                            let w3 = words.next().unwrap().trim();

                            match w3 {
                                "b" => out.push(Opcode::MovBB),
                                "c" => out.push(Opcode::MovBC),
                                "d" => out.push(Opcode::MovBD),
                                "e" => out.push(Opcode::MovBE),
                                "h" => out.push(Opcode::MovBH),
                                "l" => out.push(Opcode::MovBL),
                                "m" => out.push(Opcode::MovBM),
                                "a" => out.push(Opcode::MovBA),

                                y => {
                                    eprintln!("Unknown operand for mov ({}).", y);
                                    err = true;
                                }
                            }
                        }
                        "c" => {
                            let w3 = words.next().unwrap().trim();

                            match w3 {
                                "b" => out.push(Opcode::MovCB),
                                "c" => out.push(Opcode::MovCC),
                                "d" => out.push(Opcode::MovCD),
                                "e" => out.push(Opcode::MovCE),
                                "h" => out.push(Opcode::MovCH),
                                "l" => out.push(Opcode::MovCL),
                                "m" => out.push(Opcode::MovCM),
                                "a" => out.push(Opcode::MovCA),

                                y => {
                                    eprintln!("Unknown operand for mov ({}).", y);
                                    err = true;
                                }
                            }
                        }
                        "d" => {
                            let w3 = words.next().unwrap().trim();

                            match w3 {
                                "b" => out.push(Opcode::MovDB),
                                "c" => out.push(Opcode::MovDC),
                                "d" => out.push(Opcode::MovDD),
                                "e" => out.push(Opcode::MovDE),
                                "h" => out.push(Opcode::MovDH),
                                "l" => out.push(Opcode::MovDL),
                                "m" => out.push(Opcode::MovDM),
                                "a" => out.push(Opcode::MovDA),

                                y => {
                                    eprintln!("Unknown operand for mov ({}).", y);
                                    err = true;
                                }
                            }
                        }
                        "e" => {
                            let w3 = words.next().unwrap().trim();

                            match w3 {
                                "b" => out.push(Opcode::MovEB),
                                "c" => out.push(Opcode::MovEC),
                                "d" => out.push(Opcode::MovED),
                                "e" => out.push(Opcode::MovEE),
                                "h" => out.push(Opcode::MovEH),
                                "l" => out.push(Opcode::MovEL),
                                "m" => out.push(Opcode::MovEM),
                                "a" => out.push(Opcode::MovEA),

                                y => {
                                    eprintln!("Unknown operand for mov ({}).", y);
                                    err = true;
                                }
                            }
                        }
                        "h" => {
                            let w3 = words.next().unwrap().trim();

                            match w3 {
                                "b" => out.push(Opcode::MovHB),
                                "c" => out.push(Opcode::MovHC),
                                "d" => out.push(Opcode::MovHD),
                                "e" => out.push(Opcode::MovHE),
                                "h" => out.push(Opcode::MovHH),
                                "l" => out.push(Opcode::MovHL),
                                "m" => out.push(Opcode::MovHM),
                                "a" => out.push(Opcode::MovHA),

                                y => {
                                    eprintln!("Unknown operand for mov ({}).", y);
                                    err = true;
                                }
                            }
                        }
                        "l" => {
                            let w3 = words.next().unwrap().trim();

                            match w3 {
                                "b" => out.push(Opcode::MovLB),
                                "c" => out.push(Opcode::MovLC),
                                "d" => out.push(Opcode::MovLD),
                                "e" => out.push(Opcode::MovLE),
                                "h" => out.push(Opcode::MovLH),
                                "l" => out.push(Opcode::MovLL),
                                "m" => out.push(Opcode::MovLM),
                                "a" => out.push(Opcode::MovLA),

                                y => {
                                    eprintln!("Unknown operand for mov ({}).", y);
                                    err = true;
                                }
                            }
                        }
                        "m" => {
                            let w3 = words.next().unwrap().trim();

                            match w3 {
                                "b" => out.push(Opcode::MovMB),
                                "c" => out.push(Opcode::MovMC),
                                "d" => out.push(Opcode::MovMD),
                                "e" => out.push(Opcode::MovME),
                                "h" => out.push(Opcode::MovMH),
                                "l" => out.push(Opcode::MovML),
                                "a" => out.push(Opcode::MovMA),

                                y => {
                                    eprintln!("Unknown operand for mov ({}).", y);
                                    err = true;
                                }
                            }
                        }
                        "a" => {
                            let w3 = words.next().unwrap().trim();

                            match w3 {
                                "b" => out.push(Opcode::MovAB),
                                "c" => out.push(Opcode::MovAC),
                                "d" => out.push(Opcode::MovAD),
                                "e" => out.push(Opcode::MovAE),
                                "h" => out.push(Opcode::MovAH),
                                "l" => out.push(Opcode::MovAL),
                                "m" => out.push(Opcode::MovAM),
                                "a" => out.push(Opcode::MovAA),

                                y => {
                                    eprintln!("Unknown operand for mov ({}).", y);
                                    err = true;
                                }
                            }
                        }
                        x => {
                            eprintln!("Unknown operand for mov ({}).", x);
                            err = true;
                        }
                    }
                }
                "mvi" => {
                    pc += 2;
                    let mut w2 = words.next().unwrap().trim();
                    if let Some(w3) = w2.strip_suffix(',') {
                        w2 = w3.trim();
                    }

                    match w2 {
                        "b" => out.push(Opcode::MviB(next_byte(words.next().unwrap()))),
                        "c" => out.push(Opcode::MviC(next_byte(words.next().unwrap()))),
                        "d" => out.push(Opcode::MviD(next_byte(words.next().unwrap()))),
                        "e" => out.push(Opcode::MviE(next_byte(words.next().unwrap()))),
                        "h" => out.push(Opcode::MviH(next_byte(words.next().unwrap()))),
                        "l" => out.push(Opcode::MviL(next_byte(words.next().unwrap()))),
                        "m" => out.push(Opcode::MviM(next_byte(words.next().unwrap()))),
                        "a" => out.push(Opcode::MviA(next_byte(words.next().unwrap()))),
                        x => {
                            eprintln!("Unknown operand for mvi ({}).", x);
                            err = true;
                        }
                    }
                }
                "rst" => {
                    pc += 1;
                    let w2 = words.next().unwrap().trim();

                    match w2 {
                        "0" => out.push(Opcode::Rst0),
                        "1" => out.push(Opcode::Rst1),
                        "2" => out.push(Opcode::Rst2),
                        "3" => out.push(Opcode::Rst3),
                        "4" => out.push(Opcode::Rst4),
                        "5" => out.push(Opcode::Rst5),
                        "6" => out.push(Opcode::Rst6),
                        "7" => out.push(Opcode::Rst7),
                        x => {
                            eprintln!("Unknown operand for mvi ({}).", x);
                            err = true;
                        }
                    }
                }
                "org" => {
                    let new_pc = next_short(words.next().unwrap());
                    if let Some(diff) = new_pc.checked_sub(pc) {
                        for _ in 0..diff {
                            out.push(Opcode::Nop);
                        }
                        pc = new_pc;
                    } else if !err {
                        eprintln!("org operand overflows pc.");
                        err = true;
                    }
                }
                z => {
                    eprintln!("Unknown opcode ({}).", z);
                    err = true;
                }
            }
        }
    }

    for (k, v) in labels.iter() {
        if let Some(label) = defined.get(k) {
            for pc in v {
                match out[*pc] {
                    Opcode::Jnz(ref mut addr)
                    | Opcode::Jmp(ref mut addr)
                    | Opcode::Cnz(ref mut addr)
                    | Opcode::Jz(ref mut addr)
                    | Opcode::Cz(ref mut addr)
                    | Opcode::Call(ref mut addr)
                    | Opcode::Jnc(ref mut addr)
                    | Opcode::Cnc(ref mut addr)
                    | Opcode::Jc(ref mut addr)
                    | Opcode::Cc(ref mut addr)
                    | Opcode::Jpo(ref mut addr)
                    | Opcode::Cpo(ref mut addr)
                    | Opcode::Jpe(ref mut addr)
                    | Opcode::Cpe(ref mut addr)
                    | Opcode::Jp(ref mut addr)
                    | Opcode::Cp(ref mut addr)
                    | Opcode::Jm(ref mut addr)
                    | Opcode::Cm(ref mut addr) => {
                        *addr = *label as u16;
                    }

                    _ => continue,
                }
            }
        } else {
            eprintln!("Use of undefined label {}.", k);
            err = true;
        }
    }

    if err || err2 || err3 {
        Err(())
    } else {
        Ok(out)
    }
}
