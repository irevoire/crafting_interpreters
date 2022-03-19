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

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        let compiled = compiler::compile(source);
        self.run(compiled)
    }

    fn push_value(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop_value(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    fn binary_op(&mut self, op: impl Fn(Value, Value) -> Value) {
        let b = self.pop_value();
        let a = self.pop_value();
        self.push_value(op(a, b));
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
                OpCode::Add => self.binary_op(|a, b| a + b),
                OpCode::Subtract => self.binary_op(|a, b| a - b),
                OpCode::Multiply => self.binary_op(|a, b| a * b),
                OpCode::Divide => self.binary_op(|a, b| a / b),
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
