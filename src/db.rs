use std::cell::RefCell;

use crate::models::Item;

pub type DBType = DB<Item>;

pub trait Row<T> {
    fn set_id(&mut self, id: usize) {}
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct DB<T> {
    items: RefCell<Vec<T>>,
}

impl<T> DB<T>
where
    T: Clone + Row<T>,
{
    fn next_pk(&self) -> usize {
        self.items.borrow().len()
    }
    pub fn all(&self) -> Vec<T> {
        self.items.borrow().iter().cloned().collect()
    }
    pub fn get(&self, pk: usize) -> Option<T> {
        self.items.borrow().get(pk).cloned()
    }
    pub fn insert(&self, mut item: T) -> T {
        item.set_id(self.next_pk());
        let mut items = self.items.borrow_mut();
        items.push(item.clone());
        item
    }
}
