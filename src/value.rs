
#[derive(Debug)]
#[derive(Clone,Copy)]
pub enum ValueType {
   Boolean(bool),
   Float(f32),
   Nil
}

#[derive(Debug)]
#[derive(Clone,Copy)]
pub struct Value {
    pub value: ValueType
}

impl Value {
    pub fn new(value: ValueType)-> Self{
        Self {
            value: value
        }
    }

    // pub fn as_number(&self)-> f32{
    //     return self.value;
    // }

    // checking 
    pub fn is_bool(&self)-> bool {
        match self.value {
            ValueType::Boolean(b)=> return true,
            _=> return false
        }
    }

    pub fn is_nil(&self)-> bool {
        match self.value {
            ValueType::Nil=> return true,
            _=> return false
        }
    }

    pub fn is_number(&self)-> bool {
        match self.value {
            ValueType::Float(n)=> return true,
            _=> return false
        }
    }

}