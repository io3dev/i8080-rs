
use std::io::Read;

use i8080_rs::{
    cpu::Cpu,
    bus::Bus,
};



fn main() {
    simple_log::file("logs/cpu.log", "debug", 100, 10).unwrap();
    // simple_log::quick!("debug");
    let memory = [0; 0x10000];
    let bus = Bus::new(memory.to_vec());
    let args: Vec<String> = std::env::args().collect();
    
    let mut file = std::fs::File::open(&args[1]).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let mut cpu = Cpu::init(0x100, &buf, bus);
    cpu.io_in = Box::new(|port: u8| {
        Ok(2)
    });

    // cpu.set_io(|port: u8| {
    //     Ok(2)
    // });
    cpu.regs.pc = 0x100;
    // cpu.
    loop {
        if !cpu.hlted {
            cpu.cycle();
        }
    }
    
}

pub struct Io {

}

// impl Cpu::IoIn for Io {
    
// }