use crate::{
    chunk::{Chunk, OpCode},
    value::Value,
};

#[derive(Debug, Clone, Default)]
pub struct Vm {
    stack: Vec<Value>,
}

impl Vm {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> InterpretResult {
        self.run(chunk)
    }

    pub fn push_value(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop_value(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    fn run(&mut self, chunk: &Chunk) -> InterpretResult {
        let ip = &mut 0;

        loop {
            let opcode = chunk.read_opcode(ip);

            match opcode {
                OpCode::Constant => {
                    let constant = chunk.read_constant(ip);
                    self.push_value(constant);
                }
                OpCode::Negate => {
                    let value = self.pop_value();
                    self.push_value(-value);
                }
                OpCode::Return => {
                    let value = self.pop_value();
                    println!("ret: {}", value);
                    return InterpretResult::Ok;
                }
            }
        }
    }
}

impl Chunk {
    fn read_byte(&self, idx: &mut usize) -> u8 {
        let byte = self.read(*idx);
        *idx += 1;
        byte
    }

    fn read_opcode(&self, idx: &mut usize) -> OpCode {
        unsafe { std::mem::transmute(self.read_byte(idx)) }
    }

    fn read_constant(&self, idx: &mut usize) -> Value {
        let idx = self.read_byte(idx);
        self.constants[idx as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}
