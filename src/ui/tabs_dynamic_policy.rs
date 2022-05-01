use druid::{FontDescriptor, WidgetExt, Data, FontFamily, Widget};
use druid::widget::{Button, Flex, Label, MainAxisAlignment, SizedBox, TabInfo, TabsPolicy, TextBox, ViewSwitcher};
use crate::error::AppError;
use crate::state::tab_state::TabState;
use crate::state::tabs_state::TabsState;
use crate::ui::lens::tab_state_to_password_lens::TabStateToPasswordLens;
use crate::ui::lens::tab_state_to_text_lens::TabStateToTextLens;
use crate::ui::lens::tabs_tate_to_tab_state_lens::TabsStateToTabStateLens;
use crate::windows;

#[derive(Clone, Data)]
pub struct TabsDynamicPolicy;

impl TabsDynamicPolicy {
    fn text_box() -> Box<dyn Widget<TabState>> {
        let font = FontDescriptor::new(FontFamily::MONOSPACE)
            .with_size(14.0);

        let text_box = TextBox::multiline()
            .with_line_wrapping(false)
            .with_font(font)
            .expand()
            .lens(TabStateToTextLens::new());

        Box::new(text_box)
    }

    fn password_box() -> Box<dyn Widget<TabState>> {
        let password_box = Flex::row()
            .main_axis_alignment(MainAxisAlignment::Center)
            .with_child(TextBox::new()
                .lens(TabStateToPasswordLens::new()))
            .with_spacer(5.0)
            .with_child(SizedBox::new(Button::new("Ok")
                .on_click(|ctx, tab: &mut TabState, _env| {
                    if let Err(AppError::InvalidPassword) = tab.open() {
                        ctx.new_window(windows::dialog_window::new("Invalid password"));
                    }
                }))
                .width(50.0)
                .height(30.0))
            .expand();

        Box::new(password_box)
    }
}

impl TabsPolicy for TabsDynamicPolicy {
    type Key = u64;
    type Input = TabsState;
    type BodyWidget = Flex<TabsState>;
    type LabelWidget = Label<TabsState>;
    type Build = ();

    fn tabs_changed(&self, old_tabs: &Self::Input, tabs: &Self::Input) -> bool {
        old_tabs.rev() != tabs.rev()
    }

    fn tabs(&self, tabs: &Self::Input) -> Vec<Self::Key> {
        tabs.keys()
    }

    fn tab_info(&self, key: Self::Key, data: &Self::Input) -> TabInfo<Self::Input> {
        let tab = data.get(key);
        TabInfo::new(tab.name.clone(), true)
    }

    fn tab_body(&self, key: Self::Key, _tabs: &Self::Input) -> Self::BodyWidget {
        let switcher = ViewSwitcher::<TabState, bool>::new(
            |tab, _env| -> bool { tab.opened() },
            move |val, _tab, _env| -> Box<dyn Widget<TabState>> {
                match val {
                    true => TabsDynamicPolicy::text_box(),
                    false => TabsDynamicPolicy::password_box()
                }
            })
            .lens(TabsStateToTabStateLens::new(key));

        Flex::column()
            .with_flex_child(switcher, 1.0)
    }

    fn tab_label(&self, _key: Self::Key, info: TabInfo<Self::Input>, _tabs: &Self::Input) -> Self::LabelWidget {
        Label::new(info.name)
    }

    fn close_tab(&self, key: Self::Key, tabs: &mut Self::Input) {
        tabs.remove(key).expect("Unexpected error");
    }
}