use druid::Lens;
use crate::state::tab_content::TabContent;
use crate::state::tab_state::TabState;
use crate::ui::password_text::PasswordText;

pub struct TabStateToPasswordLens;

impl TabStateToPasswordLens {
    pub fn new() -> Self {
        TabStateToPasswordLens
    }
}

impl Lens<TabState, PasswordText> for TabStateToPasswordLens {
    fn with<V, F: FnOnce(&PasswordText) -> V>(&self, tab: &TabState, f: F) -> V {
        match &tab.content {
            TabContent::Clear { .. } => panic!("Clear text don't have password"),
            TabContent::Opened { .. } => panic!("Tab already opened"),
            TabContent::Closed { password } => f(password)
        }
    }

    fn with_mut<V, F: FnOnce(&mut PasswordText) -> V>(&self, tab: &mut TabState, f: F) -> V {
        match &mut tab.content {
            TabContent::Clear { .. } => panic!("Clear text don't have password"),
            TabContent::Opened { .. } => panic!("Tab already opened"),
            TabContent::Closed { password } => f(password)
        }
    }
}