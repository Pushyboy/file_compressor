use std::array::from_fn;
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use serde_big_array::BigArray;

#[derive(Serialize, Deserialize, Debug)]
pub struct ByteMap<T> 
    where T: Clone + Default + Serialize + DeserializeOwned
{
    #[serde(with = "BigArray")]
    map: [Option<T>; 256],
}

impl<T> ByteMap<T> 
    where T: Clone + Default + Serialize + DeserializeOwned
{
    /// Creates a new `ByteMap`
    /// A `ByteMap` maps a byte to a value
    pub fn new() -> Self {
        ByteMap {
            map: from_fn(|_| None),
        }
    }

    pub fn get(&self, idx: u8) -> &Option<T> {
        &self.map[idx as usize]
    }

    pub fn get_copy(&self) -> [Option<T>; 256] {
        self.map.clone()
    }

    pub fn insert(&mut self, item: T, idx: u8) {
        self.map[idx as usize] = Some(item);
    }

    pub fn remove(&mut self, idx: u8) -> Option<T> {
        self.map[idx as usize].take()
    }

    pub fn modify_or_default<F>(&mut self, idx: u8, closure: F, default: T) 
        where F: FnOnce(&mut T)
    {
        if self.map[idx as usize].is_none() {
            self.map[idx as usize] = Some(default);
        } else {
            closure(&mut self.map[idx as usize].unwrap());
        }
    }
}

pub struct CountMap {
    map: [usize; 256],
}

impl CountMap {
    /// Creates a new `CountMap`
    /// A `CountMap` maps a byte to its count
    pub fn new() -> CountMap {
        CountMap {
            map: [0; 256]
        }
    }

    #[inline]
    pub fn get(&self, idx: u8) -> usize {
        self.map[idx as usize]
    }

    #[inline]
    pub fn increment_count(&mut self, idx: u8) {
        self.map[idx as usize] += 1;
    }

    #[inline]
    pub fn increment_count_by(&mut self, idx: u8, num: usize) {
        self.map[idx as usize] += num;
    }

    #[inline]
    pub fn get_copy(&self) -> [usize; 256] {
        self.map.clone()
    }
}