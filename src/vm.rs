// Virtual Machine, 
//TODO dereference instead of index to track the ip and stacktop
use crate::byte_code::*;
use crate::scanner::*;
use crate::op::*;
use crate::compiler::*;
use crate::value::*;

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
            stack: [Value::new(ValueType::Float(0.0));MAX_STACK],
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
        let constant = self.chunk.constants[idx].clone();
        return constant;
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
                    let value = match self.pop().value {
                        ValueType::Float(f)=>f,
                        _=> { println!("RuntimeError");0.0}
                    };
                    self.push(Value::new(ValueType::Float(-value)));
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
        self.stack[self.stack_top] = Value::new(ValueType::Float(0.0));
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
	
	// compilation
	if !compile(source,&mut chunk){
		chunk = ChunkSt::init();
		return InterpretResult::InterpretCompileError;
	}
    chunk.disassemble_chunk("test");
	let mut vm = VM::new(chunk);
	//running; runtime.
	let result: InterpretResult = vm.run();

	return result;
}

// in place modification.
pub fn binary_op(vm: &mut VM, op:fn(f32,f32)-> f32){
    loop {
        let b = match vm.pop().value {
            ValueType::Float(f)=>f,
            _=>{println!("Runtime error");0.0}
        };
        let a = match vm.pop().value {
            ValueType::Float(f)=>f,
            _=>{println!("Runtime error");0.0}
        };
        vm.push(Value::new(ValueType::Float(op(a,b))));
        if true {
            break;
        };
    };
}
