use std::cell::RefCell;

pub struct DB<T> {
    pub items: RefCell<Vec<T>>
}

impl<T> DB<T> {
    pub fn new() -> Self {
        DB {
            items: RefCell::new(Vec::new())
        }
    }
    pub fn add(&self, item: T) {
        let mut items = self.items.borrow_mut();
        items.push(item);
    }
}
