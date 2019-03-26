use crate::db::Row;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: Option<usize>,
    pub content: String,
}

impl Row<Item> for Item {
    fn set_id(&mut self, id: usize) {
        self.id = Some(id)
    }
}
