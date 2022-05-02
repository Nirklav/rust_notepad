use druid::widget::{Flex, Label, MainAxisAlignment, Padding, SizedBox};
use druid::{Color, FontDescriptor, FontStyle, Widget, WidgetExt, WindowDesc};
use druid_shell::piet::FontFamily;
use crate::AppState;
use crate::windows::primary_screen_center;

pub fn new() -> WindowDesc<AppState> {
    let size = (400.0, 95.0);
    WindowDesc::new(ui())
        .title("About")
        .window_size(size)
        .resizable(false)
        .set_position(primary_screen_center(size))
}

const LABEL_WIDTH : f64 = 80.0;
const REPOSITORY : &str = "https://github.com/Nirklav/rust_notepad";
const LICENSE : &str = "https://github.com/Nirklav/rust_notepad/blob/master/License.md";

pub fn ui() -> impl Widget<AppState> {
    Padding::new(5.0, Flex::column()
        .with_flex_child(Flex::row()
            .with_child(SizedBox::new(Label::new("Repository:"))
                .width(LABEL_WIDTH))
            .with_spacer(5.0)
            .with_child(Label::new(REPOSITORY)
                .with_text_color(Color::rgb(0.4, 0.4, 0.8))
                .with_font(FontDescriptor::new(FontFamily::default())
                    .with_size(16.0)
                    .with_style(FontStyle::Italic))
                .on_click(|_ctx, _state: &mut AppState, _env| {
                    let _ = open::that(REPOSITORY);
                })
                .expand_width()), 1.0)
        .with_spacer(5.0)
        .with_flex_child(Flex::row()
            .main_axis_alignment(MainAxisAlignment::Start)
            .with_child(SizedBox::new(Label::new("License:"))
                .width(LABEL_WIDTH))
            .with_spacer(5.0)
            .with_child(Label::new("MIT License")
                .with_text_color(Color::rgb(0.4, 0.4, 0.8))
                .with_font(FontDescriptor::new(FontFamily::default())
                    .with_size(16.0)
                    .with_style(FontStyle::Italic))
                .on_click(|_ctx, _state: &mut AppState, _env| {
                    let _ = open::that(LICENSE);
                })
                .expand_width()), 1.0))
}