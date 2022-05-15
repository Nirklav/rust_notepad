use druid::Data;
use crate::ui::password_text::PasswordText;

#[derive(Clone, Data)]
pub struct NewTab {
    pub name: String,
    pub password: PasswordText,
}

impl NewTab {
    pub fn new() -> Self {
        NewTab {
            name: String::new(),
            password: PasswordText::new()
        }
    }

    pub fn take(&mut self) -> (String, String) {
        let name = self.name.clone();
        let password = self.password.value().to_string();

        self.name.clear();
        self.password.clear();

        (name, password)
    }
}