use druid::{Widget, WidgetExt, WindowDesc};
use druid::widget::{Button, CrossAxisAlignment, Flex, Label, Padding, SizedBox};
use crate::{AppState};
use crate::windows::primary_screen_center;

pub fn new<S: AsRef<str>>(message: S) -> WindowDesc<AppState> {
    let msg = message.as_ref().to_string();
    let size = (400.0, 150.0);
    WindowDesc::new(ui(msg))
        .title("Information")
        .window_size(size)
        .resizable(false)
        .set_position(primary_screen_center(size))
}

pub fn ui(message: String) -> impl Widget<AppState> {
    Padding::new(10.0, Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::End)
        .with_flex_child(Label::new(message)
            .expand(), 1.0)
        .with_spacer(10.0)
        .with_child(SizedBox::new(Button::new("Ok")
            .on_click(|ctx, _data: &mut AppState, _env| {
                ctx
                    .window()
                    .close()
            }))
            .width(80.0)
            .height(30.0)
        )
    )
}