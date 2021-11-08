pub struct ChecklistItem {
    name: String,
    value: bool,
}
impl Clone for ChecklistItem {
    fn clone(&self) -> Self {
        ChecklistItem {
            name: self.name.clone(),
            value: self.value,
        }
    }
}
