use dict::{Dict, DictIface};

pub struct HashTable<T> {
    pub value: Dict<T>
}

impl HashMap<T> {
    pub fn new()-> Self{
        Self {
            value: Dict::<T>::new()
        }
    }
}

pub trait HashMapTrait {
    fn add(&mut self);
    fn get(&self);
}

impl HashMapTrait for HashMap{
    fn add(&mut self){}
    fn get(&self){}
}
