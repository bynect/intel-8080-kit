#!/usr/bin/env python3
# -*- coding: utf-8 -*-


import humps
import re


def main():
    comp = re.compile(r"^(0x[\da-f]+)	([\w \,]*)", re.M | re.I)
    match = None

    with open("util/opcodes.txt") as f:
        match = comp.findall(f.read())

    raw_opcode = "RawOpcode"
    wrap_opcode = "Opcode"
    opcode_err = "OpError"

    with open("src/op.rs", "w") as o:
        with open("src/dis/mod.rs", "w") as f2:
            f2.write("use super::op::*;\nuse std::fmt;\n\n")
            f2.write("#[derive(Debug, Clone)]\n")
            f2.write(f"pub struct {opcode_err}(u8);\n\n")
            f2.write(f"impl fmt::Display for {opcode_err} {{\n")
            f2.write(
                f"{' ' * 4}fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{\n"
            )
            f2.write(f"{' ' * 4 * 2}write!(f, \"expected {{}} bytes\", self.0)\n")
            f2.write(f"{' ' * 4}}}\n}}\n\n")

            f2.write(f"pub fn disassemble_raw(bin: &Vec<u8>) -> Vec<{raw_opcode}> {{\n")
            f2.write(f"{' ' * 4}let mut ops = Vec::new();\n\n")
            f2.write(f"{' ' * 4}let mut i = 0;\n")
            f2.write(f"{' ' * 4}while i < bin.len() {{\n")
            f2.write(f"{' ' * 4 * 2}ops.push(match bin[i] {{\n")

            o.write("use std::fmt;\n\n")
            o.write("#[allow(non_camel_case_types)]\n")
            o.write("#[repr(u8)]\n")
            o.write("#[derive(Debug, Clone, Copy)]\n")
            o.write(f"pub enum {raw_opcode} {{\n")

            for m in match:
                if m[1] != "":
                    op = m[1].replace(" ", "_").replace(",", "_")

                    op2 = op.replace("__D16", "")
                    op2 = op2.replace("_D16", "")
                    op2 = op2.replace("__D8", "")
                    op2 = op2.replace("_D8", "")
                    op2 = op2.replace("_adr", "")

                    o.write(f"{' ' * 4}{op2} = {m[0]},\n")

                    f2.write(f"{' ' * 4 * 3}{m[0]} => {{\n{' ' * 4 * 4}i += ")
                    if op.endswith("D16") or op.endswith("_adr"):
                        f2.write(
                            f"3;\n{' ' * 4 * 4}{raw_opcode}::{op2}\n{' ' * 4 * 3}}}\n"
                        )
                    elif op.endswith("D8"):
                        f2.write(
                            f"2;\n{' ' * 4 * 4}{raw_opcode}::{op2}\n{' ' * 4 * 3}}}\n"
                        )
                    else:
                        f2.write(
                            f"1;\n{' ' * 4 * 4}{raw_opcode}::{op2}\n{' ' * 4 * 3}}}\n"
                        )

            o.write("}\n\n")
            o.write(f"impl fmt::Display for {raw_opcode} {{\n")
            o.write(
                f"{' ' * 4}fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{\n"
            )
            o.write(f"{' ' * 4 * 2}write!(f, \"{{}}\", *self as u8)\n")
            o.write(f"{' ' * 4}}}\n}}\n\n")

            o.write(f"impl fmt::LowerHex for {raw_opcode} {{\n")
            o.write(
                f"{' ' * 4}fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{\n"
            )
            o.write(f"{' ' * 4 * 2}let val = *self as u8;\n\n")
            o.write(f"{' ' * 4 * 2}fmt::LowerHex::fmt(&val, f)\n")
            o.write(f"{' ' * 4}}}\n}}\n\n")

            o.write(f"impl fmt::UpperHex for {raw_opcode} {{\n")
            o.write(
                f"{' ' * 4}fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{\n"
            )
            o.write(f"{' ' * 4 * 2}let val = *self as u8;\n\n")
            o.write(f"{' ' * 4 * 2}fmt::UpperHex::fmt(&val, f)\n")
            o.write(f"{' ' * 4}}}\n}}\n\n")

            f2.write(f"{' ' * 4 * 3}_ => {{\n{' ' * 4 * 4}i += ")
            f2.write(f"1;\n{' ' * 4 * 4}{raw_opcode}::NOP\n{' ' * 4 * 3}}}\n")

            f2.write(f"{' ' * 4 * 2}}});\n")
            f2.write(f"{' ' * 4}}}\n\n{' ' * 4}ops\n}}\n\n")

            o.write("#[derive(Debug, Clone, Copy)]\n")
            o.write(f"pub enum {wrap_opcode} {{\n")

            f2.write(
                f"pub fn disassemble(bin: &Vec<u8>) -> Result<Vec<{wrap_opcode}>, {opcode_err}> {{\n"
            )
            f2.write(f"{' ' * 4}let mut ops = Vec::new();\n\n")
            f2.write(f"{' ' * 4}let mut i = 0;\n")
            f2.write(f"{' ' * 4}while i < bin.len() {{\n")
            f2.write(f"{' ' * 4 * 2}ops.push(match bin[i] {{\n")

            for m in match:
                if m[1] != "":
                    op = m[1].replace(" ", "_").replace(",", "_").lower()
                    op = humps.pascalize(op)

                    op = op.replace("__D16", "(u8, u8)")
                    op = op.replace("_D16", "(u8, u8)")
                    op = op.replace("__D8", "(u8)")
                    op = op.replace("_D8", "(u8)")
                    op = op.replace("Adr", "(u8, u8)")

                    op = op.replace("_B", "B")
                    op = op.replace("_C", "C")
                    op = op.replace("_D", "D")
                    op = op.replace("_E", "E")
                    op = op.replace("_H", "H")
                    op = op.replace("_L", "L")
                    op = op.replace("_M", "M")
                    op = op.replace("_A", "A")

                    o.write(f"{' ' * 4}{op},\n")

                    op2 = op.replace("(u8, u8)", "(*b1, *b2)")
                    op2 = op2.replace("(u8)", "(*b1)")

                    f2.write(f"{' ' * 4 * 3}{m[0]} => {{\n{' ' * 4 * 4}i += ")
                    if op2.endswith("(*b1, *b2)"):
                        f2.write(
                            f"3;\n{' ' * 4 * 4}let b1 = bin.get(i - 2).ok_or({opcode_err}(2))?;"
                        )

                        f2.write(
                            f"\n{' ' * 4 * 4}let b2 = bin.get(i - 1).ok_or({opcode_err}(1))?;"
                        )
                        f2.write(
                            f"\n{' ' * 4 * 4}{wrap_opcode}::{op2}\n{' ' * 4 * 3}}}\n"
                        )

                    elif op2.endswith("(*b1)"):
                        f2.write(
                            f"2;\n{' ' * 4 * 4}let b1 = bin.get(i - 1).ok_or({opcode_err}(1))?;"
                        )

                        f2.write(
                            f"\n{' ' * 4 * 4}{wrap_opcode}::{op2}\n{' ' * 4 * 3}}}\n"
                        )

                    else:
                        f2.write(
                            f"1;\n{' ' * 4 * 4}{wrap_opcode}::{op2}\n{' ' * 4 * 3}}}\n"
                        )

            f2.write(f"{' ' * 4 * 3}_ => {{\n{' ' * 4 * 4}i += ")
            f2.write(f"1;\n{' ' * 4 * 4}{wrap_opcode}::Nop\n{' ' * 4 * 3}}}\n")

            f2.write(f"{' ' * 4 * 2}}});\n")
            f2.write(f"{' ' * 4}}}\n\n{' ' * 4}Ok(ops)\n}}\n")

        o.write("}\n")


if __name__ == "__main__":
    main()
