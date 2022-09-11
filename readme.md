# I8080-RS

Intel 8080 CPU Emulator in rust, supports CP/M bios write support

uses the log crate for CPU info about the opcode execution and cpu errors


<img src="https://upload.wikimedia.org/wikipedia/commons/3/3a/KL_Intel_i8080_Black_Background.jpg" width="200" height="100">

## Usage

```rust
use std::io::Read;
use i8080_rs::{
    cpu::Cpu,
    bus::Bus,
};

fn main() {
    let memory = [0; 0x10000];
    let bus = Bus::new(memory.to_vec());
    let args: Vec<String> = std::env::args().collect();
  
    let mut file = std::fs::File::open(&args[1]).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let mut cpu = Cpu::init(0x100, &buf, bus);

    cpu.regs.pc = 0x100;

    loop {
        if !cpu.hlted {
            cpu.cycle();
        }
    }
    
}

```

Configure IO

```rust
cpu.set_io(|port: u8| {
    Ok(0xff)
}, |port: u8, value: u8| {
    println!("Out Values, Port: 0x{:X}, Value: 0x{:X}", port, value)
    Ok(())
});
```
