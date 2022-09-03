# I8080-RS

Intel 8080 CPU Emulator in rust, supports CP/M bios write support

uses the log crate for CPU info about the opcode execution and cpu errors


<img src="https://upload.wikimedia.org/wikipedia/commons/3/3a/KL_Intel_i8080_Black_Background.jpg" width="200" height="100">

## Usage

```rust
use i8080_rs::{
    cpu::Cpu,
    bus::Bus,
}

fn main() {
    let program = [
        0x76 // HLT
    ];

    let mut new_cpu = Cpu::init(0x0, &program);

    loop {
        new_cpu.cycle();
    }
}
```

TST8080 Test

```rust
use i8080_rs::{
    cpu::Cpu,
    bus::Bus,
}

fn main() {
    let tst8080 = std::fs::Open("TST8080.COM");


    let mut new_cpu = Cpu::init(0x100, &tst8080);

    loop {
        new_cpu.cycle();
    }
}

```

TODO

- DAA

