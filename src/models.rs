#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: usize,
    pub content: String,
}
