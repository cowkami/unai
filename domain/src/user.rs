#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
}

#[derive(Debug, Clone)]
pub enum UserDemand {
    Chat,
    CreateImage,
}

impl TryFrom<String> for UserDemand {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Chat" => Ok(Self::Chat),
            "CreateImage" => Ok(Self::CreateImage),
            _ => Err("Failed to convert to UserDemand"),
        }
    }
}
