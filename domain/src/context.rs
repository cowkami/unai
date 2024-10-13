use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Context {
    id: ContextId,
    name: String,
}

#[derive(Debug, Clone)]
pub struct ContextId(Uuid);

impl ContextId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl TryFrom<String> for ContextId {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, &'static str> {
        Ok(Self(Uuid::parse_str(&value).expect("Invalid UUID")))
    }
}

impl ToString for ContextId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
