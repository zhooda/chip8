# chip8
CHIP-8 vm in Rust

## Build disassembler from src
```sh
$ git clone https://github.com/zhooda/chip8.git
$ cd chip8

# Compile
$ gcc dis/chip8dis.c -o chip8dis

# Disassemble a rom
$ ./chip8dis roms/INVADERS
```

## Build VM from src

```sh
$ git clone https://github.com/zhooda/chip8.git
$ cd chip8

# run
$ cargo run

# or build
$ cargo build
```
