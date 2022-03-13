mod chunk;
mod value;
mod vm;

use chunk::*;
use vm::*;

fn main() {
    let mut vm = Vm::new();
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write(unsafe { std::mem::transmute(OpCode::Constant) }, 123);
    chunk.write(constant as u8, 123);
    chunk.write(unsafe { std::mem::transmute(OpCode::Negate) }, 123);
    chunk.write(unsafe { std::mem::transmute(OpCode::Return) }, 123);
    chunk.disassemble_chunk("test chunk");

    println!("== execution ==");

    vm.interpret(&chunk);
}
