use druid::Lens;
use crate::state::tab::Tab;
use crate::state::tabs::Tabs;

pub struct TabsStateToTabStateLens {
    key: u64
}

impl TabsStateToTabStateLens {
    pub fn new(key: u64) -> Self {
        TabsStateToTabStateLens {
            key
        }
    }
}

impl Lens<Tabs, Tab> for TabsStateToTabStateLens {
    fn with<V, F: FnOnce(&Tab) -> V>(&self, data: &Tabs, f: F) -> V {
        let tab = data.get(self.key);
        f(&tab)
    }

    fn with_mut<V, F: FnOnce(&mut Tab) -> V>(&self, data: &mut Tabs, f: F) -> V {
        let mut tab = data.get_mut(self.key);
        f(&mut tab)
    }
}