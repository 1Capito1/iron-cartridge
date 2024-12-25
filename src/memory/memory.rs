use std::ops::{Index, IndexMut};

pub struct Memory(Box<[u8; 65536]>);

impl Memory {
    pub fn new() -> Self {
        Self(Box::new([0; 65536]))
    }
}

impl Index<u16> for Memory {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        &self.0[index as usize]
    }
}
impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}
