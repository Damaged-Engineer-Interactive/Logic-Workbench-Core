use std::any::Any;
use std::collections::HashMap;
use super::value::ValueType;
use super::value::Value;

pub struct SmallValue {
    size: usize,
    pub state: u64, // Lower 32 bits: value, upper 32 bits: tri
}

impl SmallValue {
    const STATE_LOW: u64 = 0x0000000000000000;
    const STATE_HIGH: u64 = 0x00000000FFFFFFFF;
    const STATE_TRI: u64 = 0xFFFFFFFF00000000;

    fn range_masks() -> HashMap<usize, u64> {
        [
            (1, 0x00_00_00_01),
            (2, 0x00_00_00_03),
            (3, 0x00_00_00_07),
            (4, 0x00_00_00_0F),
            (5, 0x00_00_00_1F),
            (6, 0x00_00_00_3F),
            (7, 0x00_00_00_7F),
            (8, 0x00_00_00_FF),
            (9, 0x00_00_01_FF),
            (10, 0x00_00_03_FF),
            (11, 0x00_00_07_FF),
            (12, 0x00_00_0F_FF),
            (13, 0x00_00_1F_FF),
            (14, 0x00_00_3F_FF),
            (15, 0x00_00_7F_FF),
            (16, 0x00_00_FF_FF),
            (17, 0x00_01_FF_FF),
            (18, 0x00_03_FF_FF),
            (19, 0x00_07_FF_FF),
            (20, 0x00_0F_FF_FF),
            (21, 0x00_1F_FF_FF),
            (22, 0x00_3F_FF_FF),
            (23, 0x00_7F_FF_FF),
            (24, 0x00_FF_FF_FF),
            (25, 0x01_FF_FF_FF),
            (26, 0x03_FF_FF_FF),
            (27, 0x07_FF_FF_FF),
            (28, 0x0F_FF_FF_FF),
            (29, 0x1F_FF_FF_FF),
            (30, 0x3F_FF_FF_FF),
            (31, 0x7F_FF_FF_FF),
            (32, 0xFF_FF_FF_FF),
        ]
        .iter()
        .copied()
        .collect()
    }

    pub fn new(size: usize) -> Self {
        if!((1..=64).contains(&size)){
            panic!("Invalid size for SmallValue: {}",size);
        }
        Self { size, state: SmallValue::STATE_LOW }
    }

}

impl Value for SmallValue {
    fn size(&self) -> usize {
        self.size
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_type(&self) -> ValueType {
        ValueType::Small
    }

    fn get_bit(&self, offset: usize) -> u8 {
        let value = self.get_bit_value(offset);
        let tri = self.get_bit_tri(offset);
        (tri << 1) | value
    }

    fn get_bit_value(&self, offset: usize) -> u8 {
        ((self.state >> offset) & 0x1) as u8
    }

    fn get_bit_tri(&self, offset: usize) -> u8 {
        ((self.state >> (offset + 16)) & 0x1) as u8
    }

    fn set_bit(&mut self, offset: usize, value: u8) {
        self.set_bit_value(offset, value & 1);
        self.set_bit_tri(offset, (value >> 1) & 1);
    }

    fn set_bit_value(&mut self, offset: usize, value: u8) {
        if value == 1 {
            self.state |= 1 << offset;
        } else {
            self.state &= !(1 << offset);
        }
    }

    fn set_bit_tri(&mut self, offset: usize, value: u8) {
        if value == 1 {
            self.state |= 1 << (offset + 16);
        } else {
            self.state &= !(1 << (offset + 16));
        }
    }

    fn get_range_value(&self, offset: usize, size: usize) -> u64 {
        let mask = Self::range_masks().get(&size).copied().unwrap_or(0) << offset;
        ((self.state & mask) >> offset) as u64
    }

    fn get_range_tri(&self, offset: usize, size: usize) -> u64 {
        let mask = Self::range_masks().get(&size).copied().unwrap_or(0) << (offset + 16);
        ((self.state & mask) >> (offset + 16)) as u64
    }

    fn get_small(&self) -> u64 {
        self.state
    }

    fn get_small_value(&self) -> u64 {
        (self.state & 0xFFFF) as u64
    }

    fn get_small_tri(&self) -> u64 {
        ((self.state >> 16) & 0xFFFF) as u64
    }

    fn set_small(&mut self, _value: u64) {
        self.state = _value;
    }

    fn set_small_value(&mut self, value: u64) {
        self.state = (self.state & 0xFFFF0000) | ((value as u64) & 0xFFFF);
    }

    fn set_small_tri(&mut self, value: u64) {
        self.state = (self.state & 0x0000FFFF) | (((value as u64) & 0xFFFF) << 16);
    }

    fn from_value(&mut self, from: &dyn Value) {
        match from.get_type() {
            ValueType::Small => self.set_small(from.get_small()),
            ValueType::Unknown => panic!("Unknown ValueType"),
        }
    }

    fn low(&mut self) {
        self.state = Self::STATE_LOW;
    }

    fn high(&mut self) {
        self.state = Self::STATE_HIGH;
    }

    fn tri(&mut self) {
        self.state = Self::STATE_TRI;
    }

    fn arithmetic_and(&mut self, a: &dyn Value, b: &dyn Value) {
        let value = a.get_small_value() & b.get_small_value();
        let tri = a.get_small_tri() | b.get_small_tri();
        self.set_small_value(value);
        self.set_small_tri(tri);
    }

    fn arithmetic_nand(&mut self, a: &dyn Value, b: &dyn Value) {
        let value = !(a.get_small_value() & b.get_small_value());
        let tri = a.get_small_tri() | b.get_small_tri();
        self.set_small_value(value);
        self.set_small_tri(tri);
    }

    fn arithmetic_or(&mut self, a: &dyn Value, b: &dyn Value) {
        let value = a.get_small_value() | b.get_small_value();
        let tri = a.get_small_tri() | b.get_small_tri();
        self.set_small_value(value);
        self.set_small_tri(tri);
    }

    fn arithmetic_nor(&mut self, a: &dyn Value, b: &dyn Value) {
        let value = !(a.get_small_value() | b.get_small_value());
        let tri = a.get_small_tri() | b.get_small_tri();
        self.set_small_value(value);
        self.set_small_tri(tri);
    }

    fn arithmetic_not(&mut self, with: &dyn Value) {
        let value = !(with.get_small_value());
        let tri = with.get_small_tri();
        self.set_small_value(value);
        self.set_small_tri(tri);
    }

    fn arithmetic_xor(&mut self, a: &dyn Value, b: &dyn Value) {
        let value = a.get_small_value() ^ b.get_small_value();
        let tri = a.get_small_tri() | b.get_small_tri();
        self.set_small_value(value);
        self.set_small_tri(tri);
    }

    fn arithmetic_xnor(&mut self, a: &dyn Value, b: &dyn Value) {
        let value = !(a.get_small_value() ^ b.get_small_value());
        let tri = a.get_small_tri() | b.get_small_tri();
        self.set_small_value(value);
        self.set_small_tri(tri);
    }
}
