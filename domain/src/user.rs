#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
}

#[derive(Debug, Clone)]
pub enum UserPurpose {
    Chat,
    CreateImage,
}
