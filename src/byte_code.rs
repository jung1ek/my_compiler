// operation code => mul, add ...

type value = f32;

#[derive(Debug)]
// #[repr(u8)]
pub enum OpCode {
    OpReturn,
    OpConstant
}

#[derive(Debug)]
pub struct ChunkSt {
    code: Vec<u8>,
    const_idx: u8,
    constants: Vec<f32>,
    lines: Vec<u8>
}

impl ChunkSt {
    pub fn init()-> Self {
        Self {
            code: Vec::<u8>::new(),
            constants: Vec::<f32>::new(),
            lines: Vec::<u8>::new(),
            const_idx: 0
        }
    }
}

pub trait Chunk {
    fn write_chunk(&mut self, byte: u8,line: u8);
    fn add_constant(&mut self, value: f32)-> u8;
    //debug
    fn constant_instruct(&self,name: &str, offset: u8);
    fn disassemble_chunk(&self, name: &str);
}

impl Chunk for ChunkSt {
    fn write_chunk(&mut self, byte: u8,line: u8) {
        self.code.push(byte);
        self.lines.push(line);
    }

    fn add_constant(&mut self,value: f32)-> u8 {
        self.constants.push(value);
        self.const_idx = self.const_idx+1;
        return self.const_idx-1;
    }

    fn constant_instruct(&self,name: &str, offset: u8){
        let constant = self.code[(offset+1) as usize]; // const idx
        print!("{:<16} {} '{}'\n",name,constant,self.constants[constant as usize]);

    }

    fn disassemble_chunk(&self, name: &str) {
        println!("== {} ==",name);
        let mut i = 0;
        while i<self.code.len(){
            print!("{:04} ",i);
            if i>0 && self.lines[i] == self.lines[i-1]{
                print!("  | ");
            } else {
                print!("{:} ",self.lines[i]);
            }
            let instruction = self.code[i];
            let op_code = unsafe {std::mem::transmute::<u8, OpCode>(instruction)};
            match op_code {
                OpCode::OpReturn=> {
                    print!("OP_RETURN\n");
                    i+=1;
                },
                OpCode::OpConstant=> {
                    self.constant_instruct("OP_CONSTANT",i as u8);
                    i+=2;
                }
                _=> {
                    println!("Unknown opcode {:?}",op_code);
                    i+=1;}
            }
        }
    }
}