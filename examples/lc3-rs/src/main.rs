use std::{
    io::{self, Read, Write},
    mem,
};

#[derive(Debug)]
enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,   // Program counter
    Cond, // Condition flags
}

#[repr(u16)]
enum Op {
    BR,   // branch
    ADD,  // add
    LD,   // load
    ST,   // store
    JSR,  // jump register
    AND,  // bitwise and
    LDR,  // load register
    STR,  // store register
    RTI,  // unused
    NOT,  // bitwise not
    LDI,  // load indirect
    STI,  // store indirect
    JMP,  // jump
    RES,  // reserved (unused)
    LEA,  // load effective address
    TRAP, // execute trap
}

#[repr(u16)]
enum Flag {
    Pos = 1 << 0,
    Zero = 1 << 1,
    Neg = 1 << 2,
}

#[repr(u16)]
enum Trap {
    Getc = 0x20,  // get character from keyboard, not echoed onto the terminal
    Out = 0x21,   // output a character
    Puts = 0x22,  // output a word string
    In = 0x23,    // get character from keyboard, echoed onto the terminal
    Putsp = 0x24, // output a byte string
    Halt = 0x25,  // halt the program
}

const MEMORY_MAX: usize = 1 << 16;
const REGISTER_COUNT: usize = 10;

fn main() {
    let mut memory: [u16; MEMORY_MAX] = [0; MEMORY_MAX];
    let mut registers: [u16; REGISTER_COUNT] = [0; REGISTER_COUNT];

    registers[Register::Cond as usize] = Flag::Zero as u16;

    registers[Register::PC as usize] = 0x3000;

    let r_pc = Register::PC as usize;
    let r_cond = Register::Cond as usize;

    loop {
        let instr = memory[registers[r_pc] as usize];
        let op: Op = unsafe { mem::transmute(instr >> 12 as u16) };

        registers[r_pc] += 1;

        match op {
            Op::ADD => {
                let r0 = ((instr >> 9) & 0x7) as usize;
                let r1 = ((instr >> 6) & 0x7) as usize;
                let imm_flag = (instr >> 5) & 0x1 == 1;

                let r2_imm5 = if imm_flag {
                    sign_extend(instr & 0x1F, 5) // imm5
                } else {
                    registers[(instr & 0x7) as usize] // r2
                };
                registers[r0] = registers[r1] + r2_imm5;

                update_flags(&mut registers, r0);
            }
            Op::LDI => {
                let r0 = ((instr >> 9) & 0x7) as usize;
                let pc_offset = sign_extend(instr & 0x1FF, 9);
                let addr = memory[(registers[r_pc] + pc_offset) as usize];
                registers[r0] = memory[addr as usize];

                update_flags(&mut registers, r0);
            }
            Op::AND => {
                let r0 = ((instr >> 9) & 0x7) as usize;
                let r1 = ((instr >> 6) & 0x7) as usize;
                let imm_flag = (instr >> 5) & 0x1 == 1;

                let r2_imm5 = if imm_flag {
                    sign_extend(instr & 0x1F, 5) // imm5
                } else {
                    registers[(instr & 0x7) as usize] // r2
                };
                registers[r0] = registers[r1] & r2_imm5;

                update_flags(&mut registers, r0);
            }
            Op::NOT => {
                let r0 = ((instr >> 9) & 0x7) as usize;
                let r1 = ((instr >> 6) & 0x7) as usize;

                registers[r0] = !registers[r1];
                update_flags(&mut registers, r0);
            }
            Op::BR => {
                let cond_flag = (instr >> 9) & 0x7;
                if cond_flag & registers[r_cond] == 1 {
                    let pc_offset = sign_extend(instr & 0x1FF, 9);
                    registers[r_pc] += pc_offset;
                }
            }
            Op::JMP => {
                let r1 = ((instr >> 6) & 0x7) as usize;
                registers[r_pc] = registers[r1];
            }
            Op::JSR => {
                let long_flag = (instr >> 11) & 1;
                registers[Register::R7 as usize] = registers[r_pc];
                if long_flag == 1 {
                    let long_pc_offset = sign_extend(instr & 0x7FF, 11);
                    registers[r_pc] += long_pc_offset; // JSR
                } else {
                    let r1 = ((instr >> 6) & 0x7) as usize;
                    registers[r_pc] = registers[r1]; // JSRR
                }
            }
            Op::LD => {
                let r0 = ((instr >> 9) & 0x7) as usize;
                let pc_offset = sign_extend(instr & 0x1FF, 9);
                registers[r0] = memory[(registers[r_pc] + pc_offset) as usize];
                update_flags(&mut registers, r0);
            }
            Op::LDR => {
                let r0 = ((instr >> 9) & 0x7) as usize;
                let r1 = ((instr >> 6) & 0x7) as usize;
                let offset = sign_extend(instr & 0x3F, 6);
                registers[r0] = memory[(registers[r1] + offset) as usize];
                update_flags(&mut registers, r0);
            }
            Op::LEA => {
                let r0 = ((instr >> 9) & 0x7) as usize;
                let pc_offset = sign_extend(instr & 0x1FF, 9);
                registers[r0] = registers[r_pc] + pc_offset;
                update_flags(&mut registers, r0);
            }
            Op::ST => {
                let r0 = ((instr >> 9) & 0x7) as usize;
                let pc_offset = sign_extend(instr & 0x1FF, 9);
                memory[(registers[r_pc] + pc_offset) as usize] = registers[r0];
            }
            Op::STI => {
                let r0 = ((instr >> 9) & 0x7) as usize;
                let pc_offset = sign_extend(instr & 0x1FF, 9);
                let addr = memory[(registers[r_pc] + pc_offset) as usize];
                memory[addr as usize] = registers[r0];
            }
            Op::STR => {
                let r0 = ((instr >> 9) & 0x7) as usize;
                let r1 = ((instr >> 6) & 0x7) as usize;
                let offset = sign_extend(instr & 0x3F, 6);
                memory[(registers[r1] + offset) as usize] = registers[r0];
            }
            Op::TRAP => {
                let r0 = Register::R0 as usize;
                registers[Register::R7 as usize] = registers[r_pc];
                let trap_code: Trap = unsafe { mem::transmute((instr & 0xFF) as u16) };
                match trap_code {
                    Trap::Getc => {
                        let mut buffer = [0u8; 1];
                        io::stdin().read_exact(&mut buffer).unwrap();
                        registers[r0] = buffer[0] as u16;
                        update_flags(&mut registers, r0);
                    }
                    Trap::Out => {
                        print!("{}", registers[r0] as u8 as char);
                        io::stdout().flush().unwrap();
                    }
                    Trap::Puts => {
                        let addr = registers[r0] as usize;
                        for &x in &memory[addr..] {
                            if x == 0 {
                                break;
                            }
                            print!("{}", x as u8 as char);
                        }
                        io::stdout().flush().unwrap();
                    }
                    Trap::In => {
                        print!("Enter a character: ");
                        io::stdout().flush().unwrap();

                        let mut buffer = [0u8; 1];
                        io::stdin().read_exact(&mut buffer).unwrap();
                        print!("{}", buffer[0] as char);
                        io::stdout().flush().unwrap();

                        registers[r0] = buffer[0] as u16;
                        update_flags(&mut registers, r0);
                    }
                    Trap::Putsp => {
                        let addr = registers[r0] as usize;
                        for &x in &memory[addr..] {
                            if x == 0 {
                                break;
                            }

                            let b1 = (x & 0xFF) as u8 as char;
                            print!("{}", b1);
                            let b2 = (x >> 8) as u8 as char;
                            if b2 != '\0' {
                                print!("{}", b2);
                            }
                        }
                        io::stdout().flush().unwrap();
                    }
                    Trap::Halt => {
                        println!("HALT");
                        break;
                    }
                }
            }

            _ => {}
        }
    }
}

fn sign_extend(x: u16, bit_count: usize) -> u16 {
    x | (((x >> (bit_count - 1)) & 1) as u16 * (0xFFFF << bit_count))
}

fn update_flags(registers: &mut [u16; REGISTER_COUNT], r: usize) {
    registers[r] = match registers[r] {
        0 => Flag::Zero,
        x if x >> 15 == 1 => Flag::Neg,
        _ => Flag::Pos,
    } as u16;
}
