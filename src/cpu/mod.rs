use crate::bus::Bus;


use std::{
    mem,
    env,
    io::Write
};

const mnemnoics: [&str; 256] = ["nop", "lxi b,#", "stax b", "inx b",
    "inr b", "dcr b", "mvi b,#", "rlc", "ill", "dad b", "ldax b", "dcx b",
    "inr c", "dcr c", "mvi c,#", "rrc", "ill", "lxi d,#", "stax d", "inx d",
    "inr d", "dcr d", "mvi d,#", "ral", "ill", "dad d", "ldax d", "dcx d",
    "inr e", "dcr e", "mvi e,#", "rar", "ill", "lxi h,#", "shld", "inx h",
    "inr h", "dcr h", "mvi h,#", "daa", "ill", "dad h", "lhld", "dcx h",
    "inr l", "dcr l", "mvi l,#", "cma", "ill", "lxi sp,#", "sta $", "inx sp",
    "inr M", "dcr M", "mvi M,#", "stc", "ill", "dad sp", "lda $", "dcx sp",
    "inr a", "dcr a", "mvi a,#", "cmc", "mov b,b", "mov b,c", "mov b,d",
    "mov b,e", "mov b,h", "mov b,l", "mov b,M", "mov b,a", "mov c,b", "mov c,c",
    "mov c,d", "mov c,e", "mov c,h", "mov c,l", "mov c,M", "mov c,a", "mov d,b",
    "mov d,c", "mov d,d", "mov d,e", "mov d,h", "mov d,l", "mov d,M", "mov d,a",
    "mov e,b", "mov e,c", "mov e,d", "mov e,e", "mov e,h", "mov e,l", "mov e,M",
    "mov e,a", "mov h,b", "mov h,c", "mov h,d", "mov h,e", "mov h,h", "mov h,l",
    "mov h,M", "mov h,a", "mov l,b", "mov l,c", "mov l,d", "mov l,e", "mov l,h",
    "mov l,l", "mov l,M", "mov l,a", "mov M,b", "mov M,c", "mov M,d", "mov M,e",
    "mov M,h", "mov M,l", "hlt", "mov M,a", "mov a,b", "mov a,c", "mov a,d",
    "mov a,e", "mov a,h", "mov a,l", "mov a,M", "mov a,a", "add b", "add c",
    "add d", "add e", "add h", "add l", "add M", "add a", "adc b", "adc c",
    "adc d", "adc e", "adc h", "adc l", "adc M", "adc a", "sub b", "sub c",
    "sub d", "sub e", "sub h", "sub l", "sub M", "sub a", "sbb b", "sbb c",
    "sbb d", "sbb e", "sbb h", "sbb l", "sbb M", "sbb a", "ana b", "ana c",
    "ana d", "ana e", "ana h", "ana l", "ana M", "ana a", "xra b", "xra c",
    "xra d", "xra e", "xra h", "xra l", "xra M", "xra a", "ora b", "ora c",
    "ora d", "ora e", "ora h", "ora l", "ora M", "ora a", "cmp b", "cmp c",
    "cmp d", "cmp e", "cmp h", "cmp l", "cmp M", "cmp a", "rnz", "pop b",
    "jnz $", "jmp $", "cnz $", "push b", "adi #", "rst 0", "rz", "ret", "jz $",
    "ill", "cz $", "call $", "aci #", "rst 1", "rnc", "pop d", "jnc $", "out p",
    "cnc $", "push d", "sui #", "rst 2", "rc", "ill", "jc $", "in p", "cc $",
    "ill", "sbi #", "rst 3", "rpo", "pop h", "jpo $", "xthl", "cpo $", "push h",
    "ani #", "rst 4", "rpe", "pchl", "jpe $", "xchg", "cpe $", "ill", "xri #",
    "rst 5", "rp", "pop psw", "jp $", "di", "cp $", "push psw", "ori #",
    "rst 6", "rm", "sphl", "jm $", "ei", "cm $", "ill", "cpi #", "rst 7"
];

const CYCLES: [u8; 256] = [
    //  0  1   2   3   4   5   6   7   8  9   A   B   C   D   E  F
        4, 10, 7,  5,  5,  5,  7,  4,  4, 10, 7,  5,  5,  5,  7, 4,  // 0
        4, 10, 7,  5,  5,  5,  7,  4,  4, 10, 7,  5,  5,  5,  7, 4,  // 1
        4, 10, 16, 5,  5,  5,  7,  4,  4, 10, 16, 5,  5,  5,  7, 4,  // 2
        4, 10, 13, 5,  10, 10, 10, 4,  4, 10, 13, 5,  5,  5,  7, 4,  // 3
        5, 5,  5,  5,  5,  5,  7,  5,  5, 5,  5,  5,  5,  5,  7, 5,  // 4
        5, 5,  5,  5,  5,  5,  7,  5,  5, 5,  5,  5,  5,  5,  7, 5,  // 5
        5, 5,  5,  5,  5,  5,  7,  5,  5, 5,  5,  5,  5,  5,  7, 5,  // 6
        7, 7,  7,  7,  7,  7,  7,  7,  5, 5,  5,  5,  5,  5,  7, 5,  // 7
        4, 4,  4,  4,  4,  4,  7,  4,  4, 4,  4,  4,  4,  4,  7, 4,  // 8
        4, 4,  4,  4,  4,  4,  7,  4,  4, 4,  4,  4,  4,  4,  7, 4,  // 9
        4, 4,  4,  4,  4,  4,  7,  4,  4, 4,  4,  4,  4,  4,  7, 4,  // A
        4, 4,  4,  4,  4,  4,  7,  4,  4, 4,  4,  4,  4,  4,  7, 4,  // B
        5, 10, 10, 10, 11, 11, 7,  11, 5, 10, 10, 10, 11, 17, 7, 11, // C
        5, 10, 10, 10, 11, 11, 7,  11, 5, 10, 10, 10, 11, 17, 7, 11, // D
        5, 10, 10, 18, 11, 11, 7,  11, 5, 5,  10, 4,  11, 17, 7, 11, // E
        5, 10, 10, 4,  11, 11, 7,  11, 5, 5,  10, 4,  11, 17, 7, 11  // F
];

mod flags;
use flags::Flags;

use log::{info, debug};

const MEMORY: usize = 0x10000;



pub struct Registers {
    pub pc: u16,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    pub sp: u16,
}

impl Registers {
    fn set_bc(&mut self, value: u16) {
        self.c = (value & 0xff) as u8;
        self.b = ((value >> 8) & 0xff) as u8;
        
    }

    fn set_de(&mut self, value: u16) {
        self.e = (value & 0xff) as u8;
        self.d = ((value >> 8) & 0xff) as u8;
    }

    fn set_hl(&mut self, value: u16) {
        self.l = (value & 0xff) as u8;
        self.h = ((value >> 8) & 0xff) as u8;
    }
}

pub struct Cpu {
    pub regs: Registers,

    pub flags: Flags,

    pub memory: [u8; MEMORY],

    pub hlted: bool,

    pub instructions: usize,

    pub interupts_enabled: bool,

    pub cycles_count: usize,

    immediate: [u8; 2],

    cycle: u8,

    // io: IO,

    // Only returns NONE if function pointer has no function

    pub io_in: Box<dyn Fn(u8) -> Result<u8, ()>>,
    pub io_out: Box<dyn Fn(u8, u8) -> Result<(), ()>>,

    pub bus: Bus,

    pub output: String,
}

impl Cpu {
    pub fn init(pc: u16, program: &[u8], bus: Bus) -> Cpu {
        let mut c = Cpu {
            interupts_enabled: false,
        
            regs: Registers {
                a: 0,
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                h: 0,
                l: 0,

                pc: 0,
                sp: 0,
            },

            cycle: 0,

            flags: Flags {
                sign: false,
                zero: false,
                parity: false,
                carry: false,
                aux_carry: false,
            },

            memory: [0; MEMORY],
            hlted: false,
            instructions: 0,
            immediate: [0, 0],

            output: String::new(),

            cycles_count: 0,

            bus,

            io_in: Box::new(|port: u8| {
                Err(())
            }),

            io_out: Box::new(|port: u8, value: u8| {
                Err(())
            }),

            // io: IO::default(),

        };
        #[cfg(feature = "cpm")]
        {
            info!("CP/M Is enabled");
        }

        #[cfg(not(feature = "cpm"))]
        {
            info!("CP/M Disabled")
        }
        

        c.load_into_memory(program, pc as usize).unwrap();
        c
    }

    // Load array of bytes into memory

    pub fn load_into_memory(&mut self, bytes: &[u8], address: usize) -> Result<(), ()> {
        if bytes.len() == 0 {
            return Err(());
        }

        

        // log::info!("Cpu loaded file to {}", address);
        

        for i in 0..bytes.len() {
            self.bus.write(bytes[i], i + address);
        }

        log::info!("CPU Loaded program to address: 0x{:X}", address);

        Ok(())
    }

    // Cycle with cycle durations
    pub fn cycle_d(&mut self) {
        if self.cycle > CYCLES[self.memory[self.regs.pc as usize] as usize] {
            self.cycle();
           self.cycle = 0;
        } 
        self.cycle += 1;

    }


    pub fn set_io(&mut self, input: fn(u8) -> Result<u8, ()>, output: fn(u8, u8) -> Result<(), ()>) {
        self.io_in = Box::new(input);
        self.io_out = Box::new(output);
    }
    // Cycle the cpu once

    pub fn cycle(&mut self) {
        
        let mut advance = 1;
        self.instructions += 1;
        let opcode = self.bus.read(self.regs.pc as usize);
        // println!("{:X}", opcode);
        self.immediate = [self.bus.read((self.regs.pc + 1) as usize), self.bus.read((self.regs.pc + 2) as usize)];
        // println!("{}", advance);
        // self.cycles_count += (1 + CYCLES[opcode as usize]) as usize;
        debug!("Opcode: {}, ({}, {}), PC (0x{:X}),   SP: {}", mnemnoics[opcode as usize], self.immediate[0], self.immediate[1], self.regs.pc, self.regs.sp);
        self.cycles_count += 1;
        match opcode {

            0x00 | 0x20 | 0x28 | 0x38 | 0x30 | 0x08 | 0xD9 => {}

            /*
            DATA TRANSFER INSTRUCTIONS
            */
            
            0xE3 => {
                self.regs.l = self.bus.read(self.regs.sp as usize);
                self.regs.h = self.bus.read((self.regs.sp + 1) as usize);
            }

            // MOV INSTRUCTIONS
            0x40 => {},
            0x41 => self.regs.b = self.regs.c,
            0x42 => self.regs.b = self.regs.d,
            0x43 => self.regs.b = self.regs.e,
            0x44 => self.regs.b = self.regs.h,
            0x45 => self.regs.b = self.regs.l,
            0x46 => self.regs.b = self.get_m(),
            0x47 => self.regs.b = self.regs.a,
            0x48 => self.regs.c = self.regs.b,
            0x49 => {},
            0x4A => self.regs.c = self.regs.d,
            0x4B => self.regs.c = self.regs.e,
            0x4C => self.regs.c = self.regs.h,
            0x4D => self.regs.c = self.regs.l,
            0x4E => self.regs.c = self.get_m(),
            0x4F => self.regs.c = self.regs.a,
            0x50 => self.regs.d = self.regs.b, 
            0x51 => self.regs.d = self.regs.c,
            0x52 => {},
            0x53 => self.regs.d = self.regs.e,
            0x54 => self.regs.d = self.regs.h,
            0x55 => self.regs.d = self.regs.l,
            0x56 => self.regs.d = self.get_m(),
            0x57 => self.regs.d = self.regs.a,
            0x58 => self.regs.e = self.regs.b,
            0x59 => self.regs.e = self.regs.c,
            0x5A => self.regs.e = self.regs.d,
            0x5B => {},
            0x5C => self.regs.e = self.regs.h,
            0x5D => self.regs.e = self.regs.l,
            0x5E => self.regs.e = self.get_m(),
            0x5F => self.regs.e = self.regs.a,
            0x60 => self.regs.h = self.regs.b,
            0x61 => self.regs.h = self.regs.c,
            0x62 => self.regs.h = self.regs.d,
            0x63 => self.regs.h = self.regs.e,
            0x64 => {},
            0x65 => self.regs.h = self.regs.l,
            0x66 => self.regs.h = self.get_m(),
            0x67 => self.regs.h = self.regs.a,
            0x68 => self.regs.l = self.regs.b,
            0x69 => self.regs.l = self.regs.c,
            0x6A => self.regs.l = self.regs.d,
            0x6B => self.regs.l = self.regs.e,
            0x6C => self.regs.l = self.regs.h,
            0x6D => {},
            0x6E => self.regs.l = self.get_m(),
            0x6F => self.regs.l = self.regs.a,
            0x70 => self.set_m(self.regs.b),
            0x71 => self.set_m(self.regs.c),
            0x72 => self.set_m(self.regs.d),
            0x73 => self.set_m(self.regs.e),
            0x74 => self.set_m(self.regs.h),
            0x75 => self.set_m(self.regs.l),
            0x77 => self.set_m(self.regs.a),
            0x78 => self.regs.a = self.regs.b,
            0x79 => self.regs.a = self.regs.c,
            0x7A => self.regs.a = self.regs.d,
            0x7B => self.regs.a = self.regs.e,
            0x7C => self.regs.a = self.regs.h,
            0x7D => self.regs.a = self.regs.l,
            0x7E => self.regs.a = self.get_m(),
            0x7F => {},

            /*
            MVI Instructions, move immediate value into register
            */
            0x06 => {self.regs.b = self.immediate[0]; advance = 2},
            0x16 => {self.regs.d = self.immediate[0]; advance = 2},
            0x26 => {self.regs.h = self.immediate[0]; advance = 2},
            0x36 => {self.bus.write(self.immediate[0], self.cmb_be(self.regs.h, self.regs.l) as usize); advance = 2},
            0x0E => {self.regs.c = self.immediate[0]; advance = 2},
            0x1E => {self.regs.e = self.immediate[0]; advance = 2},
            0x2E => {self.regs.l = self.immediate[0]; advance = 2},
            0x3E => {self.regs.a = self.immediate[0]; advance = 2},

            /*
            LXI Instructions
            */

            0x01 => {self.set_bc_imm(); advance = 3},
            0x11 => {self.set_de_imm(); advance = 3},
            0x21 => {self.set_hl_imm(); advance = 3},
            0x31 => {self.regs.sp = self.cmb_le(self.immediate[0], self.immediate[1]); advance = 3},
            
            // 50 % sure this works

            // STA and LDA
            0x32 => {self.mem_write(self.cmb_le(self.immediate[0], self.immediate[1]).into(), self.regs.a); advance = 3},
            0x3A => {self.regs.a = self.bus.read(self.cmb_le(self.immediate[0], self.immediate[1]) as usize); advance = 3},

            // LHLD and SHLD
            0x2A => {
                self.regs.l = self.bus.read(self.cmb_le(self.immediate[0], self.immediate[1]) as usize) as u8;
                self.regs.h = self.bus.read(((self.cmb_le(self.immediate[0], self.immediate[1])) + 1) as usize) as u8;
                advance = 3;
            }

            0x22 => {
                self.mem_write(self.cmb_le(self.immediate[0], self.immediate[1]) as usize, self.regs.l);
                self.mem_write(((self.cmb_le(self.immediate[0], self.immediate[1])) + 1).into(), self.regs.h);
                advance = 3;
            }

            // LDAX
            0x0A => self.ldax(self.regs.b, self.regs.c),
            0x1A => self.ldax(self.regs.d, self.regs.e),

            // STAX

            0x02 => {
                
                self.mem_write(self.cmb_be(self.regs.b, self.regs.c) as usize, self.regs.a);
            }

            0x12 => {
                self.mem_write(self.cmb_be(self.regs.d, self.regs.e) as usize, self.regs.a);
            }

            // DAD
            0x09 => {
                let hl32: u32= self.cmb_be(self.regs.h, self.regs.l).into();
                let bc32: u32 = self.cmb_be(self.regs.b, self.regs.c).into();
                let res: u32 = hl32 + bc32;
                self.regs.h = ((res & 0xff00) >> 8) as u8;
                self.regs.l = (res & 0xff) as u8;
                self.flags.carry = (res & 0xffff0000) > 0;
            }

            0x19 => {
                let hl32: u32= self.cmb_be(self.regs.h, self.regs.l).into();
                let de32: u32 = self.cmb_be(self.regs.d, self.regs.e).into();
                let res: u32 = hl32 + de32;
                self.regs.h = ((res & 0xff00) >> 8) as u8;
                self.regs.l = (res & 0xff) as u8;
                self.flags.carry = (res & 0xffff0000) > 0;
            }

            0x29 => {
                let hl32: u32= self.cmb_be(self.regs.h, self.regs.l).into();
                let de32: u32 = self.cmb_be(self.regs.b, self.regs.c).into();
                let res: u32 = hl32 + hl32;
                self.regs.h = ((res & 0xff00) >> 8) as u8;
                self.regs.l = (res & 0xff) as u8;
                self.flags.carry = (res & 0xffff0000) > 0;
            }

            // 0x32 => {
            //     println!("{}", self.cmb_le(self.immediate[0], self.immediate[1]));
            // }



            0x27 => {
                // let mut answer = self.a.to_u16();

                // let least = answer & 0xf;
        
                // if self.conditions.ac || least > 9 {
                //     answer += 6;
        
                //     if answer & 0xf < least {
                //         self.conditions.ac = true;
                //     }
                // }
        
                // let least = answer & 0xf;
                // let mut most = (answer >> 4) & 0xf;
        
                // if self.conditions.cy || most > 9 {
                //     most += 6;
                // }
        
                // let answer = ((most << 4) as u16) | least as u16;
                // self.conditions.set_all_except_ac(answer);
        
                // self.a = answer.into();

                let mut answer = self.regs.a as u16;

                let lsb = answer & 0xf;
                if self.flags.aux_carry || lsb > 9 {
                    answer += 6;
                    if answer & 0xf < lsb {
                        self.flags.aux_carry = true;
                    }


                }

                let least = answer & 0xf;
                let mut msb = (answer >> 4) & 0xf;
                if self.flags.carry || msb > 9 {
                    msb += 6;
                }

                let answer = (msb << 4) as u16 | lsb as u16;
                // self.flags.
                self.regs.a = answer as u8;
            }









            /*
            Branch instructions
            */

            0xc3 => {self.jmp_immediate(); advance = 0},

            0xCA => {self.jmp_if(self.flags.zero    == true,  &mut advance)}
            0xC2 => {self.jmp_if(self.flags.zero    == false, &mut advance)}
            0xD2 => {self.jmp_if(self.flags.carry   == false, &mut advance)}
            0xDA => {self.jmp_if(self.flags.carry   == true,  &mut advance)}
            0xE2 => {self.jmp_if(self.flags.parity  == false, &mut advance)}
            0xEA => {self.jmp_if(self.flags.parity  == true,  &mut advance)}
            0xF2 => {self.jmp_if(self.flags.sign    == false, &mut advance)}
            0xFA => {self.jmp_if(self.flags.sign    == true,  &mut advance)}

            0xCD => self.call_imm(&mut advance),
            0xC4 => self.call_if(self.flags.zero    == false, &mut advance),
            0xD4 => self.call_if(self.flags.carry   == false, &mut advance),
            0xE4 => self.call_if(self.flags.parity  == false, &mut advance),
            0xF4 => self.call_if(self.flags.sign    == false, &mut advance),
            0xCC => self.call_if(self.flags.zero    == true,  &mut advance),
            0xDC => self.call_if(self.flags.carry   == true,  &mut advance),
            0xEC => self.call_if(self.flags.parity  == true,  &mut advance),
            0xFC => self.call_if(self.flags.sign    == true,  &mut advance),


            0xC9 => {self.ret(); advance = 3},
            0xC8 => {self.ret_if(self.flags.zero == true, &mut advance)}
            0xD8 => {self.ret_if(self.flags.carry == true, &mut advance)}
            0xE8 => {self.ret_if(self.flags.parity == true, &mut advance)}
            0xF8 => {self.ret_if(self.flags.sign == true, &mut advance)}
            0xC0 => {self.ret_if(self.flags.zero == false, &mut advance)}
            0xD0 => {self.ret_if(self.flags.carry == false, &mut advance)}
            0xE0 => {self.ret_if(self.flags.parity == false, &mut advance)}
            0xF0 => {self.ret_if(self.flags.sign == false, &mut advance)}

            /*
            Math
            */

            0xC6 => {self.add(self.immediate[0]); advance = 2}
            0xCE => {self.add(self.immediate[0] + self.flags.carry as u8); advance = 2}

            0xD6 => {self.sub(self.immediate[0]); advance = 2}
            0xDE => {self.sub(self.immediate[0] + self.flags.carry as u8); advance = 2}

            0x80 => self.add(self.regs.b),
            0x81 => self.add(self.regs.c),
            0x82 => self.add(self.regs.d),
            0x83 => self.add(self.regs.e),
            0x84 => self.add(self.regs.h),
            0x85 => self.add(self.regs.l),
            0x86 => self.add(self.get_m()),
            0x87 => self.add(self.regs.a),
            0x88 => self.add(self.regs.b + (self.flags.carry as u8)),
            0x89 => self.add(self.regs.c + (self.flags.carry as u8)),
            0x8A => self.add(self.regs.d + (self.flags.carry as u8)),
            0x8B => self.add(self.regs.e + (self.flags.carry as u8)),
            0x8C => self.add(self.regs.h + (self.flags.carry as u8)),
            0x8D => self.add(self.regs.l + (self.flags.carry as u8)),
            0x8E => self.add(self.get_m() + (self.flags.carry as u8)),
            0x8F => self.add(self.regs.a + (self.flags.carry as u8)),


            0x90 => self.sub(self.regs.b),
            0x91 => self.sub(self.regs.c),
            0x92 => self.sub(self.regs.d),
            0x93 => self.sub(self.regs.e),
            0x94 => self.sub(self.regs.h),
            0x95 => self.sub(self.regs.l),
            0x96 => self.sub(self.get_m()),
            0x97 => self.sub(self.regs.a),
            0x98 => self.sub(self.regs.b + (self.flags.carry as u8)),
            0x99 => self.sub(self.regs.c + (self.flags.carry as u8)),
            0x9A => self.sub(self.regs.d + (self.flags.carry as u8)),
            0x9B => self.sub(self.regs.e + (self.flags.carry as u8)),
            0x9C => self.sub(self.regs.h + (self.flags.carry as u8)),
            0x9D => self.sub(self.regs.l + (self.flags.carry as u8)),
            0x9E => self.sub(self.get_m() + (self.flags.carry as u8)),
            0x9F => self.sub(self.regs.a + (self.flags.carry as u8)),
            

            0xE6 => {self.ani(); advance = 2}
            0xA0 => self.ana(self.regs.b),
            0xA1 => self.ana(self.regs.c),
            0xA2 => self.ana(self.regs.d),
            0xA3 => self.ana(self.regs.e),
            0xA4 => self.ana(self.regs.h),
            0xA5 => self.ana(self.regs.l),
            0xA6 => self.ana(self.get_m()),
            0xA7 => self.ana(self.regs.a),


            0xF6 => {self.ori(); advance = 2}
            0xB0 => self.ora(self.regs.b),
            0xB1 => self.ora(self.regs.c),
            0xB2 => self.ora(self.regs.d),
            0xB3 => self.ora(self.regs.e),
            0xB4 => self.ora(self.regs.h),
            0xB5 => self.ora(self.regs.l),
            0xB6 => self.ora(self.get_m()),
            0xB7 => self.ora(self.regs.a),

            0xEE => {self.xri(); advance = 2}

            0x04 => self.regs.b = self.inr(self.regs.b),
            0x0C => self.regs.c = self.inr(self.regs.c),
            0x14 => self.regs.d = self.inr(self.regs.d),
            0x1C => self.regs.e = self.inr(self.regs.e),
            0x24 => self.regs.h = self.inr(self.regs.h),
            0x2C => self.regs.l = self.inr(self.regs.l),
            0x34 => { let m = self.get_m(); let res = self.inr(m); self.set_m(res); }
            0x3C => self.regs.a = self.inr(self.regs.a),
            0x05 => self.regs.b = self.dcr(self.regs.b),
            0x0D => self.regs.c = self.dcr(self.regs.c),
            0x15 => self.regs.d = self.dcr(self.regs.d),
            0x1D => self.regs.e = self.dcr(self.regs.e),
            0x25 => self.regs.h = self.dcr(self.regs.h),
            0x2D => self.regs.l = self.dcr(self.regs.l),
            0x35 => { let m = self.get_m(); let res = self.dcr(m); self.set_m(res); }
            0x3D => self.regs.a = self.dcr(self.regs.a),

            // INX and DCX

            0x03 => self.regs.set_bc(self.cmb_be(self.regs.b, self.regs.c) + 1),
            0x13 => self.regs.set_de(self.cmb_be(self.regs.d, self.regs.e) + 1),
            0x23 => self.regs.set_hl(self.cmb_be(self.regs.h, self.regs.l) + 1),
            0x33 => self.regs.sp += 1,
            0x0B => self.regs.set_bc(self.cmb_be(self.regs.b, self.regs.c) - 1),
            0x1B => self.regs.set_de(self.cmb_be(self.regs.d, self.regs.e) - 1),
            0x2B => self.regs.set_hl(self.cmb_be(self.regs.h, self.regs.l) - 1),
            0x3B => self.regs.sp -= 2,

            /*
            Stack Functions
            */

            0xC5 => self.push_regs(self.regs.b, self.regs.c),
            0xD5 => self.push_regs(self.regs.d, self.regs.e),
            0xE5 => self.push_regs(self.regs.h, self.regs.l),
            
            0xC1 => self.pop_into_bc(),
            0xD1 => self.pop_into_de(),
            0xE1 => self.pop_into_hl(),

            0xF1 => {
                self.regs.a = self.bus.read((self.regs.sp + 1) as usize);
                let psw: u8 = self.bus.read(self.regs.sp as usize);
                self.flags.zero = (0x01 == (psw & 0x01));
                self.flags.sign = (0x02 == (psw & 0x02));
                self.flags.parity = (0x04 == (psw & 0x04));
                self.flags.carry =  (0x05 == (psw & 0x08));
                self.flags.aux_carry = (0x10 == (psw & 0x10));
                self.regs.sp += 2;

            }

            0xf5 => {
                self.mem_write((self.regs.sp - 1) as usize, self.regs.a);
                let psw: u8 = (self.flags.zero as u8) | ((self.flags.sign as u8) << 1) | 
                ((self.flags.parity as u8) << 2) | ((self.flags.carry as u8) << 3) | ((self.flags.aux_carry as u8) << 4);
                self.mem_write((self.regs.sp - 2) as usize, psw);
                self.regs.sp = self.regs.sp - 2;
            }

            // Logical

            0xFE => {self.cmp(self.immediate[0].into()); advance = 2}
            0xB8 => self.cmp(self.regs.b.into()),
            0xB9 => self.cmp(self.regs.c.into()),
            0xBA => self.cmp(self.regs.d.into()),
            0xBB => self.cmp(self.regs.e.into()),
            0xBC => self.cmp(self.regs.h.into()),
            0xBD => self.cmp(self.regs.l.into()),
            0xBF => self.cmp(self.regs.a.into()),
            0xBE => self.cmp(self.get_m().into()),

            0xEB => {mem::swap(&mut self.regs.h, &mut self.regs.d); mem::swap(&mut self.regs.l, &mut self.regs.e);}
            

            0xA8 => self.xra(self.regs.b),
            0xA9 => self.xra(self.regs.c),
            0xAA => self.xra(self.regs.d),
            0xAB => self.xra(self.regs.e),
            0xAC => self.xra(self.regs.h),
            0xAD => self.xra(self.regs.l),
            0xAE => self.xra(self.get_m()),
            0xAF => self.xra(self.regs.a),

            0x0F => {
                let x: u8 = self.regs.a;
                self.regs.a = ((x & 1) << 7) | (x >> 1);
                self.flags.carry = (1 == (x&1));
            }

            0x07 => {
                let x: u8 = self.regs.a;
                self.regs.a = ((x & 1) << 7) | (x >> 1);
                self.flags.carry = (1 == (x&1));
            }

            /*
            IO Instructions 
            */
            0xDB => {
                // self.io.cpu_read(self.immediate[0]);
                self.regs.a = (self.io_in)(self.immediate[0]).unwrap();
                advance = 2;
            }
            0xD3 => {
                (self.io_out)(self.regs.a, self.immediate[0]);
                advance = 2},
            0xFB => {
                // log::info!("Interupts Enabled");
                self.interupts_enabled = true;
            },
            
            /*
            MISC Instructions
            */
            
            // ENABLE CFLAG
            0x37 => self.flags.carry = true,
            // DISABLE CFLAG
            0x3F => self.flags.carry = false,

            // CMA, Compliment A
            0x2F => self.regs.a = !self.regs.a,

            0x76 => {
                log::info!("CPU Halted by force");
                self.hlted = true;
            },

            0xF9 => {
                self.regs.sp = self.cmb_le(self.regs.h, self.regs.l);
            }
            // RLC0
            0xF3 => {
                self.interupts_enabled = false;
            }
            
            

            // _ => unimplemented!("Opcode 0x{:X}", self.memory[self.regs.pc as usize]),
            _ => {
                log::error!("Unkown Opcode: {}, 0x{:X}", mnemnoics[opcode as usize], opcode);
                self.hlted = true;
            }
            // _ => unimplemented!(),
        }

        self.regs.pc += advance;
    }

    pub fn debug(&self) {
        println!("OP 0x{:X}", self.memory[self.regs.pc as usize]);
        // println!("{:X}", self.memory[0x1000]);
        println!("A {:X}", self.regs.pc);
    }

    fn set_bc_imm(&mut self) {
        self.regs.b = self.immediate[1];
        self.regs.c = self.immediate[0];
    }

    fn set_de_imm(&mut self) {
        self.regs.d = self.immediate[1];
        self.regs.e = self.immediate[0];
    }

    fn set_hl_imm(&mut self) {
        self.regs.h = self.immediate[1];
        self.regs.l = self.immediate[0];
    }



    fn jmp_if(&mut self, condition: bool, advance: *mut u16) {
        if (condition == true) {
            self.jmp_immediate();
            unsafe {
                *advance = 0;
            }
        } else {
            unsafe {
                *advance = 3;
            }
        }
    }

    fn call_if(&mut self, condition: bool, advance: *mut u16) {
        if (condition == true) {
            self.call_imm(advance);
        } else {
            unsafe {
                *advance = 3;
            }
        }
    }

    fn jmp_cpm(&mut self) {

    }

    fn jmp_immediate(&mut self) {
        self.regs.pc = self.cmb_le(self.immediate[0], self.immediate[1]);

        #[cfg(feature = "cpm")]
        {
            if self.cmb_le(self.immediate[0], self.immediate[1]) == 0 {
                self.hlted = true;
                log::info!("CP/M Bios Requested a power off");
            }

            // self.jmp_cpm();
        }
    }

    fn ret(&mut self) {
        self.regs.pc = self.cmb_le(self.bus.read(self.regs.sp as usize), self.bus.read((self.regs.sp + 1) as usize));

        self.regs.sp += 2;
    }

    fn ret_if(&mut self, condition: bool, advance: *mut u16) {
        if condition == true {
            self.ret();
            unsafe {
                *advance = 3;
            }
        } else {
            unsafe {
                *advance = 1;
            }
        }
    }

    fn push_regs(&mut self, reg1: u8, reg2: u8) {
        self.mem_write((self.regs.sp - 2) as usize, reg2);
        self.mem_write((self.regs.sp - 1) as usize, reg1);
        self.regs.sp -= 2;
    }

    fn pop_into_bc(&mut self) {
        self.regs.c = self.bus.read(self.regs.sp as usize);
        self.regs.b = self.bus.read((self.regs.sp + 1) as usize);
        self.regs.sp += 2;
    }

    fn pop_into_de(&mut self) {
        self.regs.e = self.bus.read(self.regs.sp as usize);
        self.regs.d = self.bus.read((self.regs.sp + 1) as usize);
        self.regs.sp += 2;
    }

    fn pop_into_hl(&mut self) {
        self.regs.l = self.bus.read(self.regs.sp as usize);
        self.regs.h = self.bus.read((self.regs.sp + 1) as usize);
        self.regs.sp += 2;
    }

    // Normal CPU Call
    
    fn nm_call(&mut self) {
        // println!("{}", self.regs.sp.wrapping_sub(1) );

        self.mem_write((self.regs.sp - 1) as usize, ((self.regs.pc >> 8) & 0xff) as u8);
        self.mem_write((self.regs.sp - 2) as usize, (self.regs.pc & 0xff) as u8);

        self.regs.sp -= 2;
        
        self.jmp_immediate();
    }

    // Call with CP\M Support
    // #[cfg(feature = "cpm")]
    fn cpm_call(&mut self, advance: *mut u16) {

        const BDOS: u16 = 5;

        const WRITESTR: u8 = 9;
        const WRITE: u8 = 2;
        let addr = self.cmb_le(self.immediate[0], self.immediate[1]);

        // CPM Bios routine handling

        if addr == BDOS {
            match self.regs.c {
                WRITESTR => {
                    let string_addr = self.cmb_be(self.regs.d, self.regs.e);
                    let mut c = 0;
                    while self.bus.read((string_addr + c) as usize) != '$' as u8 {
                        print!("{}", self.bus.read((string_addr + c) as usize) as char);
                        self.output.push(self.bus.read((string_addr + c) as usize) as char);
                        c += 1;
                    }
                    std::io::stdout().flush().unwrap();
                },
                WRITE => {
                    print!("{}", self.regs.e as char);
                    std::io::stdout().flush().unwrap();
                    
                }
                _ => unimplemented!("Invalid CPM Bios routine {}", self.regs.c),
            }

            unsafe {
                *advance = 3;
            }
            
        } else {
            self.nm_call();
            unsafe {
                *advance = 0;
            }
        }
    }

    fn call_imm(&mut self, advance: *mut u16) {
        #[cfg(not(feature = "cpm"))]
        {
            self.nm_call();
            unsafe {
                *advance = 0;
            }
        }

        #[cfg(feature = "cpm")]
        {
            // Add --features cpm to your cargo build arguments to include CP\M functionality
            self.cpm_call(advance);
        }
    }

    pub fn cmb_le(&self, val1: u8, val2: u8) -> u16 {
        (val2 as u16) << 8 | val1 as u16
    }

    pub fn cmb_be(&self, val1: u8, val2: u8) -> u16 {
        (val1 as u16) << 8 | val2 as u16
    }

    fn mem_write(&mut self, pos: usize, val: u8) {
        self.bus.write(val, pos);
    }

    /* 
    Arithmitic instructions
    */

    fn ani(&mut self) {
        let res = (self.regs.a & self.immediate[0]) as u16;
        self.flags.carry = self.flags.calc_add_cy(res);
        self.flags.zero = self.flags.calc_zero(res as u8);
        self.flags.sign = self.flags.calc_sign(res as u8);
        self.flags.parity = self.flags.calc_parity(res as u8);
        self.regs.a = (self.regs.a & self.immediate[0]) as u8
    }

    fn ana(&mut self, val: u8) {
        let res = (self.regs.a & val) as u16;
        self.flags.carry = self.flags.calc_add_cy(res);
        self.flags.zero = self.flags.calc_zero(res as u8);
        self.flags.sign = self.flags.calc_sign(res as u8);
        self.flags.parity = self.flags.calc_parity(res as u8);
        self.regs.a = (self.regs.a & val) as u8
    }

    fn ori(&mut self) {
        let res = (self.regs.a | self.immediate[0]) as u16;
        self.flags.carry = self.flags.calc_add_cy(res);
        self.flags.zero = self.flags.calc_zero(res as u8);
        self.flags.sign = self.flags.calc_sign(res as u8);
        self.flags.parity = self.flags.calc_parity(res as u8);
        self.regs.a = res as u8;
    }


    fn ora(&mut self, val: u8) {
        let res = (self.regs.a | val) as u16;
        self.flags.carry = self.flags.calc_add_cy(res);
        self.flags.zero = self.flags.calc_zero(res as u8);
        self.flags.sign = self.flags.calc_sign(res as u8);
        self.flags.parity = self.flags.calc_parity(res as u8);
        self.flags.aux_carry = self.flags.calc_aux(res, self.regs.a);
        self.regs.a = res as u8;
    }

    fn xri(&mut self) {
        let res = (self.regs.a ^ self.immediate[0]) as u16;
        self.flags.carry = self.flags.calc_add_cy(res);
        self.flags.zero = self.flags.calc_zero(res as u8);
        self.flags.sign = self.flags.calc_sign(res as u8);
        self.flags.parity = self.flags.calc_parity(res as u8);
        self.regs.a = res as u8;
    }

    fn xra(&mut self, val: u8) {
        let res = (self.regs.a ^ val) as u16;
        self.flags.carry = self.flags.calc_add_cy(res);
        self.flags.zero = self.flags.calc_zero(res as u8);
        self.flags.sign = self.flags.calc_sign(res as u8);
        self.flags.parity = self.flags.calc_parity(res as u8);
        self.regs.a = res as u8;
    }

    fn add(&mut self, val: u8) {
        let res: u16 = ((self.regs.a as u16) + val as u16) as u16;
        self.flags.carry = self.flags.calc_add_cy(res);
        self.flags.zero = self.flags.calc_zero(res as u8);
        self.flags.sign = self.flags.calc_sign(res as u8);
        self.flags.parity = self.flags.calc_parity(res as u8);
        self.flags.aux_carry = self.flags.calc_aux(res, self.regs.a);
        self.regs.a = res as u8;
    }

    fn inr(&mut self, reg: u8) -> u8 {
        let res: u16 = ((reg as u16) + 1 as u16) as u16;
        self.flags.carry = self.flags.calc_add_cy(res);
        self.flags.zero = self.flags.calc_zero(res as u8);
        self.flags.sign = self.flags.calc_sign(res as u8);
        self.flags.parity = self.flags.calc_parity(res as u8);
        self.flags.aux_carry = self.flags.calc_aux(res, self.regs.a);
        res as u8
        
    }



    // Returns add but with setting flags

    fn get_add(&mut self, val: u16) -> u8 {
        let res: u16 = ((self.regs.a as u16) + val as u16) as u16;
        self.flags.carry = self.flags.calc_add_cy(res);
        self.flags.zero = self.flags.calc_zero(res as u8);
        self.flags.sign = self.flags.calc_sign(res as u8);
        self.flags.parity = self.flags.calc_parity(res as u8);
        self.flags.aux_carry = self.flags.calc_aux(res, self.regs.a);

        return res as u8;
    }

    fn sub(&mut self, val: u8) {
        let r1 = (self.regs.a as u16).wrapping_sub(val as u16);

        // self.flags.carry = (self.regs.a as u16) > 0xff;
        // self.flags.carry = !(r1 & 0x100) == 1;
        self.flags.carry = (r1 > 0xff);
        self.flags.zero = (r1 & 0xff) == 0;
        self.flags.sign = self.flags.calc_sign(r1 as u8);
        self.flags.parity = self.flags.calc_parity(r1 as u8);
        self.regs.a = r1 as u8;
        // println!("{}", self.flags.carry);
        // i8080_set_flag(cpu, FLAG_C, !(res16 & 0x100));
    }

    fn get_sub(&mut self, val: u8) -> u8 {
        let r1 = (self.regs.a as u16).wrapping_sub(val as u16);

        // self.flags.carry = (self.regs.a as u16) > 0xff;
        // self.flags.carry = !(r1 & 0x100) == 1;
        self.flags.carry = (r1 > 0xff);
        self.flags.zero = (r1 & 0xff) == 0;
        self.flags.sign = self.flags.calc_sign(r1 as u8);
        self.flags.parity = self.flags.calc_parity(r1 as u8);
        r1 as u8
    }

    fn dcr(&mut self, reg: u8) -> u8 {
        let r1 = (reg as u16).wrapping_sub(1 as u16);

        // self.flags.carry = (self.regs.a as u16) > 0xff;
        // self.flags.carry = !(r1 & 0x100) == 1;
        self.flags.carry = (r1 > 0xff);
        self.flags.zero = (r1 & 0xff) == 0;
        self.flags.sign = self.flags.calc_sign(r1 as u8);
        self.flags.parity = self.flags.calc_parity(r1 as u8);
        r1 as u8
    }
    
    fn set_m(&mut self, val: u8) {
        self.mem_write((self.cmb_be(self.regs.h, self.regs.l) as usize) as usize, val);
    }

    fn get_m(&self) -> u8 {
        self.bus.read(self.cmb_be(self.regs.h, self.regs.l) as usize)
    }

    // DATA TRANSFER

    fn ldax(&mut self, reg1: u8, reg2: u8) {
        self.regs.a = self.bus.read(self.cmb_be(reg1, reg2) as usize);
    }

    // Logical

    fn cmp(&mut self, val: u16) {
        // let res: u16 = (self.regs.a as u16) - val as u16;
        let res: u16 = (self.regs.a as u16).wrapping_sub(val as u16);
        // let
        self.flags.zero = res == 0;
        self.flags.sign = self.flags.calc_sign(res as u8);
        self.flags.carry = (self.regs.a < val as u8);
    }

    // Communication with the outside machine/emulator

    // Return list of registers with corrosponding names

    pub fn get_regs(&self) -> ([&str; 9], [u8; 7], [u16; 2]) {
        let regs = [
            self.regs.a,
            self.regs.b,
            self.regs.c,
            self.regs.d,
            self.regs.e,
            self.regs.h,
            self.regs.l,
        ];

        let regs16 = [self.regs.pc, self.regs.sp];

        let names = [
            "A",
            "B",
            "C",
            "D",
            "E",
            "H",
            "L",
            "PC",
            "SP",
        ];

        (names, regs, regs16)
    }

    pub fn send_interupt(&mut self, interupt: u16) {
        self.mem_write((self.regs.sp - 1) as usize, ((self.regs.pc >> 8) & 0xff) as u8);
        self.mem_write((self.regs.sp - 2) as usize, (self.regs.pc & 0xff) as u8);

        self.regs.sp -= 2;

        self.regs.pc = 8 * interupt;
    }


}

#[cfg(test)]
mod cpu_test {
    use super::*;


    #[test]
    fn io() {
        let mut porta = 0;
        let memory = [0; 0x1000];
        let bus = Bus::new(memory.to_vec());
        let mut cpu = Cpu::init(0x0, &[0x3E, 0x10, 0xD3,0x45, 0xdb, 0xff], bus);
        // cpu.io_out = Box::new(|port: u8, value: u8| {

        //     assert_eq!(value, 0x45);
        //     assert_eq!(port, 0x10);

        //     Ok(())
        // });

        cpu.set_io(|port: u8| {
            Ok(0xff)
        }, |port: u8, value: u8| {
            assert_eq!(value, 0x45);
            assert_eq!(port, 0x10);
            Ok(())
        });

        cpu.cycle();
        cpu.cycle();

        // cpu.io_in = Box::new(|port: u8| {
        //     Ok(0xff)
        // });

        cpu.cycle();
        assert_eq!(cpu.regs.a, 0xff);
    }
    // #[allow(warnings)]
    // #[test]
    // fn lxi_bc() {
    //     let prog = [0x01, 0x34, 0x12];
    //     // LXI B, 0x1234
    //     let mut cpu = Cpu::init(0x0, &prog);
    //     cpu.cycle();

    //     assert_eq!(cpu.regs.b, 0x12);
    //     assert_eq!(cpu.regs.c, 0x34);

    // }

    // #[test]
    // fn lxi_de() {
    //     let prog = [0x11, 0x34, 0x12];
    //     // LXI B, 0x1234
    //     let mut cpu = Cpu::init(0x0, &prog);
    //     cpu.cycle();

    //     assert_eq!(cpu.regs.d, 0x12);
    //     assert_eq!(cpu.regs.e, 0x34);

    // }

    // #[test]
    // fn lxi_hl() {
    //     let prog = [0x21, 0x34, 0x12];
    //     // LXI B, 0x1234
    //     let mut cpu = Cpu::init(0x0, &prog);
    //     cpu.cycle();

    //     assert_eq!(cpu.regs.h, 0x12);
    //     assert_eq!(cpu.regs.l, 0x34);

    // }

    // #[test]
    // fn set_and_read_m() {
    //     let prog = [0x21, 0x00, 0x80, 0x36, 0xff];
    //     // Prog in assembly is:

    //     /*

    //     lxi h, 0x8000

    //     mvi m, 0xff

    //     */
        
        
    //     let mut cpu = Cpu::init(0x0, &prog);
    //     for i in 0..2 {
    //         cpu.cycle();
    //     }

        
    //     assert_eq!(cpu.memory[0x8000], 0xff);
    // }
}