# intel-8080-kit

[![cratesio-badge](https://img.shields.io/crates/v/intel-8080-kit)](https://crates.io/crates/intel-8080-kit)

An Intel 8080 library crate written in Rust that provides an assembler, a disassembler and an emulator with respective executables.

The assembler currently supports only a subset of the Intel 8080 assembly language.

## Assembler example

```sh
$ cargo run --bin asm8080 tests/basic.asm
Emitted 8 bytes to out.bin from tests/basic.asm.
```

## Disassembler example

```sh
$ cargo run --bin dis8080 out.bin
PC        OPCODE
0000      In(0)
0002      MviB(100)
0004      AddB
0005      Out(10)
0007      Hlt
```

## Emulator example

```sh
$ cargo run --bin emu8080 out.bin
Input byte from port 0.
Output byte 100 to port 10.
Execution of out.bin took 23.8µs.
```
