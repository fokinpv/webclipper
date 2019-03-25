use std::cell::RefCell;

use crate::models::Item;

pub type DBType = DB<Item>;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct DB<T> {
    pub items: RefCell<Vec<T>>,
}

impl<T> DB<T>
where
    T: Clone,
{
    pub fn next_pk(&self) -> usize {
        self.items.borrow().len()
    }
    pub fn all(&self) -> Vec<T> {
        self.items.borrow().iter().cloned().collect()
    }
    pub fn get(&self, pk: usize) -> Option<T> {
        self.items.borrow().get(pk).cloned()
    }
    pub fn insert(&self, item: T) {
        let mut items = self.items.borrow_mut();
        items.push(item);
    }
}
