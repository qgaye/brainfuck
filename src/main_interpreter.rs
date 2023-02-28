mod opcode;

use std::{
    env::args,
    error::Error,
    fs::File,
    io::{stdin, stdout, Read, Write},
    vec,
};

use opcode::{Code, OpCode};

struct Interpreter {
    stack: Vec<u8>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self {
            stack: vec![0; 1], 
        }
    }
}

impl Interpreter {
    fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        let code = Code::from(data)?;
        let code_len = code.instrs.len();
        let mut pc = 0; // promgram counter
        let mut sp = 0; // start pointer
        loop {
            if pc >= code_len {
                break;
            }
            match code.instrs[pc] {
                OpCode::SHR => {
                    sp += 1;
                    if sp == self.stack.len() {
                        self.stack.push(0);
                    }
                }
                OpCode::SHL => {
                    sp = if sp == 0 { 0 } else { sp - 1 };
                }
                OpCode::ADD => {
                    // number overflow will panic
                    self.stack[sp] = self.stack[sp].overflowing_add(1).0;
                }
                OpCode::SUB => {
                    self.stack[sp] = self.stack[sp].overflowing_sub(1).0;
                }
                OpCode::PUTCHAR => {
                    stdout().write_all(&[self.stack[sp]][..])?;
                }
                OpCode::GETCHAR => {
                    let mut buf: Vec<u8> = vec![0; 1];
                    stdin().read_exact(&mut buf)?;
                    self.stack[sp] = buf[0];
                }
                OpCode::LB => {
                    if self.stack[sp] == 0x00 {
                        pc = code.jtable[&pc];
                    }
                }
                OpCode::RB => {
                    if self.stack[sp] != 0x00 {
                        pc = code.jtable[&pc];
                    }
                }
            }
            pc += 1;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();
    assert!(args.len() >= 2);               
    let mut file = File::open(&args[1])?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;
    let mut interpreter = Interpreter::default();
    interpreter.run(data)
}
