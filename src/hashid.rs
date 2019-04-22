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
    pub fn decode(&self, hash: &str) -> usize {
        0
    }
}
