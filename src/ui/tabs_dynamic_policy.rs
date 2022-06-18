use druid::{FontDescriptor, WidgetExt, Data, FontFamily, Widget};
use druid::widget::{Button, Flex, Label, MainAxisAlignment, SizedBox, TabInfo, TabsPolicy, TextBox, ViewSwitcher};
use crate::error::AppError;
use crate::state::tab::Tab;
use crate::state::tabs::Tabs;
use crate::ui::lens::tab_state_to_password_lens::TabStateToPasswordLens;
use crate::ui::lens::tab_state_to_text_lens::TabStateToTextLens;
use crate::ui::lens::tabs_tate_to_tab_state_lens::TabsStateToTabStateLens;
use crate::ui::tab_close_button::close_button;
use crate::{AppState, windows};
use crate::ui::context_menu_controller::ContextMenuController;
use crate::ui::copy_cut_paste_controller::CopyCutPasteController;
use crate::ui::text_box_menu_factory::TextBoxMenuFactory;

#[derive(Clone, Data)]
pub struct TabsDynamicPolicy;

impl TabsDynamicPolicy {
    fn text_box() -> Box<dyn Widget<Tab>> {
        let font = FontDescriptor::new(FontFamily::MONOSPACE)
            .with_size(14.0);

        let text_box = TextBox::multiline()
            .with_line_wrapping(false)
            .with_font(font)
            .expand()
            .controller(ContextMenuController::new(TextBoxMenuFactory))
            .controller(CopyCutPasteController)
            .lens(TabStateToTextLens::new());

        Box::new(text_box)
    }

    fn password_box() -> Box<dyn Widget<Tab>> {
        let password_box = Flex::row()
            .main_axis_alignment(MainAxisAlignment::Center)
            .with_child(TextBox::new()
                .lens(TabStateToPasswordLens::new()))
            .with_spacer(5.0)
            .with_child(SizedBox::new(Button::new("Ok")
                .on_click(|ctx, tab: &mut Tab, _env| {
                    if let Err(AppError::InvalidPassword) = tab.open() {
                        ctx.new_window(windows::information_window::new("Invalid password"));
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
    type Input = Tabs;
    type BodyWidget = Flex<Tabs>;
    type LabelWidget = Flex<Tabs>;
    type Build = ();

    fn tabs_changed(&self, old_tabs: &Tabs, tabs: &Tabs) -> bool {
        old_tabs.rev() != tabs.rev()
    }

    fn tabs(&self, tabs: &Tabs) -> Vec<u64> {
        tabs.keys()
    }

    fn tab_info(&self, key: u64, data: &Tabs) -> TabInfo<Tabs> {
        let tab = data.get(key);
        TabInfo::new(tab.name.clone(), false)
    }

    fn tab_body(&self, key: u64, _tabs: &Tabs) -> Self::BodyWidget {
        let switcher = ViewSwitcher::<Tab, bool>::new(
            |tab, _env| -> bool { tab.opened() },
            move |val, _tab, _env| -> Box<dyn Widget<Tab>> {
                match val {
                    true => TabsDynamicPolicy::text_box(),
                    false => TabsDynamicPolicy::password_box()
                }
            })
            .lens(TabsStateToTabStateLens::new(key));

        Flex::column()
            .with_flex_child(switcher, 1.0)
    }

    fn tab_label(&self, key: u64, info: TabInfo<Tabs>, _tabs: &Tabs) -> Self::LabelWidget {
        Flex::row()
            .with_child(Label::new(info.name))
            .with_child(close_button()
                .on_click(move |ctx, _tabs: &mut Tabs, _env| {
                    ctx.new_window(windows::dialog_window::new(
                        "Are you sure?",
                        move |_ctx, state: &mut AppState, _env| {
                            state.tabs.remove(key).expect("Unexpected error")
                        }));
                }))
    }
}