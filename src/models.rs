use crate::db::Row;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Snippet {
    pub id: Option<usize>,
    pub content: String,
}

impl Row<Snippet> for Snippet {
    fn set_id(&mut self, id: usize) {
        self.id = Some(id)
    }
}
