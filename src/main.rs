
use std::io::Read;

use i8080_rs::{
    cpu::Cpu,
    bus::Bus,
};



fn main() {
    simple_log::file("cpu.log", "debug", 100, 10).unwrap();
    // simple_log::quick!("debug");
    
    let args: Vec<String> = std::env::args().collect();
    
    let mut file = std::fs::File::open(&args[1]).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf);
    let mut cpu = Cpu::init(0x100, &buf);
    // cpu.
    loop {
        if !cpu.hlted {
            cpu.cycle();
        }
    }
    
}