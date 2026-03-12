// Operation for virtual machine.
use crate::byte_code::*;

pub fn print_value(value: Value) {
    print!("{}\n",value);
}

pub fn add(a: Value, b: Value)-> Value {
    a + b
}

pub fn mul(a: Value, b: Value)-> Value {
    a * b
}

pub fn sub(a: Value, b: Value)-> Value {
    a - b
}

pub fn div(a: Value, b: Value)-> Value {
    a / b
}

