extern crate harsh;

use harsh::{Harsh, HarshBuilder};
use std::cell::RefCell;

#[derive(Default, Clone)]
pub struct HashID {
    harsh: Harsh,
}

impl HashID {
    pub fn new() -> Self {
        HashID {
            harsh: HarshBuilder::new().length(10).init().unwrap(),
        }
    }
    pub fn encode(&self, id: usize) -> String {
        self.harsh.encode(&[id as u64]).unwrap()
    }
    pub fn decode(&self, hash: &str) -> Option<usize> {
        match self.harsh.decode(hash) {
            Some(hash) => Some(hash[0] as usize),
            None => None,
        }
    }
}
