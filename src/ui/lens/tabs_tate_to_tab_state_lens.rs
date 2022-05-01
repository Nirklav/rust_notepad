use druid::Lens;
use crate::state::tab_state::TabState;
use crate::state::tabs_state::TabsState;

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

impl Lens<TabsState, TabState> for TabsStateToTabStateLens {
    fn with<V, F: FnOnce(&TabState) -> V>(&self, data: &TabsState, f: F) -> V {
        let tab = data.get(self.key);
        f(&tab)
    }

    fn with_mut<V, F: FnOnce(&mut TabState) -> V>(&self, data: &mut TabsState, f: F) -> V {
        let mut tab = data.get_mut(self.key);
        f(&mut tab)
    }
}