use std::cell::{Ref, RefCell};

#[derive(Default, Clone)]
pub struct DB<T> {
    pub items: RefCell<Vec<T>>
}

impl<T> DB<T>
    where T: Clone
{
    pub fn new() -> Self {
        DB {
            items: RefCell::new(Vec::new())
        }
    }
    pub fn all(&self) -> Ref<Vec<T>> {
        self.items.borrow()
    }
    pub fn add(&self, item: T) {
        let mut items = self.items.borrow_mut();
        items.push(item);
    }
}
