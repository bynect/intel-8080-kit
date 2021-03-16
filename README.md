# asm-8080

An Intel 8080 library crate written in Rust that provides an assembler, a disassembler and an emulator with example executables.

The assembler currently supports only a subset of the Intel 8080 assembly language.

## Assembler example

```sh
$ cargo run --bin asm8080 tests/mult.asm
Emitted 22 bytes to out.bin from tests/mult.asm.
```

## Disassembler example

```sh
$ cargo run --bin dis8080 out.bin
PC        OPCODE
0000      MviB(0)
0002      MviE(9)
0004      MovAC
0005      Rar
0006      MovCA
0007      DcrE
0008      Jz(21)
0011      MovAB
0012      Jnc(16)
0015      AddD
0016      Rar
0017      MovBA
0018      Jmp(4)
0021      Ret
```
