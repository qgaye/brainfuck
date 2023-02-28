use std::{collections::HashMap, error::Error, fmt::Debug};

#[derive(Debug, PartialEq)]
pub enum OpCode {
    SHR,     // >
    SHL,     // <
    ADD,     // +
    SUB,     // -
    PUTCHAR, // .
    GETCHAR, // ,
    LB,      // [
    RB,      // ]
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            b'>' => OpCode::SHR,
            b'<' => OpCode::SHL,
            b'+' => OpCode::ADD,
            b'-' => OpCode::SUB,
            b'.' => OpCode::PUTCHAR,
            b',' => OpCode::GETCHAR,
            b'[' => OpCode::LB,
            b']' => OpCode::RB,
            _ => panic!("unknown u8 opcode"),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(val: OpCode) -> Self {
        match val {
            OpCode::SHR => b'>',
            OpCode::SHL => b'<',
            OpCode::ADD => b'+',
            OpCode::SUB => b'-',
            OpCode::PUTCHAR => b'.',
            OpCode::GETCHAR => b',',
            OpCode::LB => b'[',
            OpCode::RB => b']',
        }
    }
}

pub struct Code {
    pub instrs: Vec<OpCode>,
    pub jtable: HashMap<usize, usize>,
}

impl Code {
    pub fn from(data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let dict: Vec<u8> = vec![
            OpCode::SHR.into(),
            OpCode::SHL.into(),
            OpCode::ADD.into(),
            OpCode::SUB.into(),
            OpCode::PUTCHAR.into(),
            OpCode::GETCHAR.into(),
            OpCode::LB.into(),
            OpCode::RB.into(),
        ];

        let instrs: Vec<OpCode> = data
            .iter()
            .filter(|x| dict.contains(x))
            .map(|x| OpCode::from(*x))
            .collect();
        let mut jstack: Vec<usize> = Vec::new();
        let mut jtable: HashMap<usize, usize> = HashMap::new();
        for (i, op) in instrs.iter().enumerate() {
            if OpCode::LB == *op {
                jstack.push(i);
            }
            if OpCode::RB == *op {
                let j = jstack.pop().ok_or("jstack is empty")?;
                jtable.insert(i, j);
                jtable.insert(j, i);
            }
        }
        Ok(Code { instrs, jtable })
    }
}
