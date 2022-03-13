use crate::value::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    Constant,
    Negate,
    Return,
}

#[derive(Debug, Default, Clone)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<usize>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn read(&self, idx: usize) -> u8 {
        self.code[idx]
    }

    pub fn add_constant(&mut self, value: impl Into<Value>) -> usize {
        self.constants.push(value.into());
        self.constants.len() - 1
    }

    pub fn disassemble_chunk(&self, name: impl AsRef<str>) {
        println!("== {} ==", name.as_ref());
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset]);
        }

        let instruction: u8 = self.code[offset];
        match unsafe { std::mem::transmute(instruction) } {
            ins @ (OpCode::Negate | OpCode::Return) => {
                self.simple_instruction(format!("{:?}", ins), offset)
            }
            ins @ OpCode::Constant => self.constant_instruction(format!("{:?}", ins), offset),
            _ => {
                println!("Unknown opcode {}", instruction);
                offset + 1
            }
        }
    }

    pub fn simple_instruction(&self, name: impl AsRef<str>, offset: usize) -> usize {
        println!("{}", name.as_ref());
        offset + 1
    }

    pub fn constant_instruction(&self, name: impl AsRef<str>, offset: usize) -> usize {
        let constant = self.code[offset + 1];
        println!(
            "{:16} {:4} `{}`",
            name.as_ref(),
            constant,
            self.constants[constant as usize]
        );
        offset + 2
    }
}
