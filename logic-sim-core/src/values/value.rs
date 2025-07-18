use std::any::Any;

pub enum ValueType {
    Unknown,
    Small, // 1-64
    //Medium, // 65-128
    //Large // 128-1024
}

pub trait Value {
    fn size(&self) -> usize;

    fn as_any(&self) -> &dyn Any {
        panic!("Must be implemented by sub-class!");
    }

    fn get_type(&self) -> ValueType {
        ValueType::Unknown
    }

    fn get_bit(&self, _offset: usize) -> u8 {
        panic!("Must be implemented by sub-class!");
    }

    fn get_bit_value(&self, _offset: usize) -> u8 {
        panic!("Must be implemented by sub-class!");
    }

    fn get_bit_tri(&self, _offset: usize) -> u8 {
        panic!("Must be implemented by sub-class!");
    }

    fn set_bit(&mut self, _offset: usize, _value: u8) {
        panic!("Must be implemented by sub-class!");
    }

    fn set_bit_value(&mut self, _offset: usize, _value: u8) {
        panic!("Must be implemented by sub-class!");
    }

    fn set_bit_tri(&mut self, _offset: usize, _value: u8) {
        panic!("Must be implemented by sub-class!");
    }

    fn get_range_value(&self, _offset: usize, _size: usize) -> u64 {
        panic!("Must be implemented by sub-class!");
    }

    fn get_range_tri(&self, _offset: usize, _size: usize) -> u64 {
        panic!("Must be implemented by sub-class!");
    }

    fn get_small(&self) -> u64 {
        panic!("Must be implemented by sub-class!");
    }

    fn get_small_value(&self) -> u64 {
        panic!("Must be implemented by sub-class!");
    }

    fn get_small_tri(&self) -> u64 {
        panic!("Must be implemented by sub-class!");
    }

    fn set_small(&mut self, _value: u64) {
        panic!("Must be implemented by sub-class!");
    }

    fn set_small_value(&mut self, _value: u64) {
        panic!("Must be implemented by sub-class!");
    }

    fn set_small_tri(&mut self, _value: u64) {
        panic!("Must be implemented by sub-class!");
    }

    fn from_value(&mut self, _from: &dyn Value) {
        panic!("Must be implemented by sub-class!");
    }

    fn low(&mut self) {
        panic!("Must be implemented by sub-class!");
    }

    fn high(&mut self) {
        panic!("Must be implemented by sub-class!");
    }

    fn tri(&mut self) {
        panic!("Must be implemented by sub-class!");
    }

    // Arithmetic operations
    fn arithmetic_and(&mut self, _a: &dyn Value, _b: &dyn Value) {
        panic!("Must be implemented by sub-class!");
    }

    fn arithmetic_nand(&mut self, _a: &dyn Value, _b: &dyn Value) {
        panic!("Must be implemented by sub-class!");
    }

    fn arithmetic_or(&mut self, _a: &dyn Value, _b: &dyn Value) {
        panic!("Must be implemented by sub-class!");
    }

    fn arithmetic_nor(&mut self, _a: &dyn Value, _b: &dyn Value) {
        panic!("Must be implemented by sub-class!");
    }

    fn arithmetic_not(&mut self, _with: &dyn Value) {
        panic!("Must be implemented by sub-class!");
    }

    fn arithmetic_xor(&mut self, _a: &dyn Value, _b: &dyn Value) {
        panic!("Must be implemented by sub-class!");
    }

    fn arithmetic_xnor(&mut self, _a: &dyn Value, _b: &dyn Value) {
        panic!("Must be implemented by sub-class!");
    }
}
