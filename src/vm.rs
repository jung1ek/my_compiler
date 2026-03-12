// Virtual Machine, 
//TODO dereference instead of index to track the ip and stacktop
use crate::byte_code::*;
use crate::scanner::*;
use crate::op::*;
use crate::compiler::*;

const MAX_STACK: usize = 256;
// Interpret Result Type
#[derive(Debug)]
pub enum InterpretResult {
    InterpretOk,
    InterpretError,
    InterpretCompileError,
    InterpretRuntimeError
}

#[derive(Debug)]
pub struct VM {
    chunk: ChunkSt,
    ip: usize,

    stack: [Value;MAX_STACK],
    stack_top: usize
}

impl VM {
    pub fn new(chunkst: ChunkSt)-> Self {
        Self {
            chunk: chunkst,
            ip: 0,
            stack: [0.0;MAX_STACK],
            stack_top: 0
        }
    }

    // only for run method. 
    // get operation byte enum.
    fn get_byte(&mut self)-> u8 {
        let byte = self.chunk.code[self.ip];
        self.ip+=1;
        return byte;
    }

    // return constant value of that code idx.
    fn get_constant(&mut self)-> Value {
        let idx = self.get_byte() as usize;
        let const_idx = self.chunk.constants[idx];
        return const_idx;
    }

    // running byte
    pub fn run(&mut self)-> InterpretResult{
        loop {
            let instruction = self.get_byte();
            let op_code = unsafe {std::mem::transmute::<u8,OpCode>(instruction)};
            self.debug_trace_execution();
            match op_code {
                
                OpCode::OpReturn=> {
                    print_value(self.pop());
                    return InterpretResult::InterpretOk
                },
                OpCode::OpConstant=> {
                    // read local var and push to stack.
                    let constant: Value = self.get_constant();
                    self.push(constant);
                },
                OpCode::OpNegate=> {
                    let value = self.pop();
                    self.push(-value);
                },
                OpCode::OpAdd=> {
                    binary_op(self,add);
                },
                OpCode::OpSub=> {
                    binary_op(self,sub);
                },
                OpCode::OpMul=> {
                    binary_op(self,mul);
                },
                OpCode::OpDiv=> {
                    binary_op(self,div);
                },
                _=>{
                    return InterpretResult::InterpretError
                }
            }
            self.debug_trace_execution();
        };
    }

    fn pop(&mut self)-> Value{
        self.stack_top -= 1;
        let a = self.stack[self.stack_top];
        self.stack[self.stack_top] = 0.0;
        return a;
    }

    pub fn push(&mut self, value: Value){
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    fn reset_stack(&mut self){
        self.stack_top = 0;
    }

    pub fn debug_trace_execution(&self) {
        // println!("{:?}",&self.stack[0..self.stack_top]);
    }
}

pub fn interpret(source: &str)-> InterpretResult {
	//defince chunk
	let mut chunk = ChunkSt::init();

	if !compile(source,&mut chunk){
		chunk = ChunkSt::init();
		return InterpretResult::InterpretCompileError;
	}
    chunk.disassemble_chunk("test");
	let mut vm = VM::new(chunk);
	let result: InterpretResult = vm.run();

	return result;
}

// in place modification.
pub fn binary_op(vm: &mut VM, op:fn(Value,Value)-> Value){
    loop {
        let b = vm.pop();
        let a = vm.pop();
        vm.push(op(a,b));
        if true {
            break;
        };
    };
}