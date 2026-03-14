// Operation for virtual machine.
use crate::value::*;

pub fn print_value(value: Value) {
    print!("{:?}\n",value);
}

pub fn add(a: f32, b: f32)-> f32 {
    a + b
}

pub fn mul(a: f32, b: f32)-> f32 {
    a * b
}

pub fn sub(a: f32, b: f32)-> f32 {
    a - b
}

pub fn div(a: f32, b: f32)-> f32 {
    a / b
}

