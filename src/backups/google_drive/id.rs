use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Id(String);

impl Id {
    pub fn new(id: impl ToString) -> Self {
        Id(id.to_string())
    }
}

impl AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}