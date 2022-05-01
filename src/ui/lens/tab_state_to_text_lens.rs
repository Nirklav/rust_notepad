use druid::Lens;
use crate::state::tab_content::TabContent;
use crate::state::tab_state::TabState;

pub struct TabStateToTextLens;

impl TabStateToTextLens {
    pub fn new() -> Self {
        TabStateToTextLens
    }
}

impl Lens<TabState, String> for TabStateToTextLens {
    fn with<V, F: FnOnce(&String) -> V>(&self, tab: &TabState, f: F) -> V {
        match &tab.content {
            TabContent::Clear { text } => f(text),
            TabContent::Opened { text, .. } => f(text),
            TabContent::Closed { .. } => panic!("Tab don't decrypted")
        }
    }

    fn with_mut<V, F: FnOnce(&mut String) -> V>(&self, tab: &mut TabState, f: F) -> V {
        match &mut tab.content {
            TabContent::Clear { text } => f(text),
            TabContent::Opened { text, .. } => f(text),
            TabContent::Closed { .. } => panic!("Tab don't decrypted")
        }
    }
}