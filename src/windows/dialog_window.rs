use druid::{Data, Env, EventCtx, Widget, WidgetExt, WindowDesc};
use druid::widget::{Button, CrossAxisAlignment, Flex, Label, Padding, SizedBox};
use crate::windows::primary_screen_center;

pub fn new<T: Data>(
    message: impl AsRef<str>,
    on_ok: impl Fn(&mut EventCtx, &mut T, &Env) + 'static) -> WindowDesc<T> {

    let msg = message.as_ref().to_string();
    let size = (400.0, 150.0);
    WindowDesc::new(ui(msg, on_ok))
        .title("Information")
        .window_size(size)
        .resizable(false)
        .set_position(primary_screen_center(size))
}

fn ui<T: Data>(
    message: String,
    on_ok: impl Fn(&mut EventCtx, &mut T, &Env) + 'static) -> impl Widget<T> {

    Padding::new(10.0, Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::End)
        .with_flex_child(Label::new(message)
            .expand(), 1.0)
        .with_spacer(5.0)
        .with_child(Flex::row()
            .with_child(SizedBox::new(Button::new("Ok")
                .on_click(move |ctx, data: &mut T, env| {
                    on_ok(ctx, data, env);
                    ctx
                        .window()
                        .close()
                }))
                .width(80.0)
                .height(30.0))
            .with_spacer(5.0)
            .with_child(SizedBox::new(Button::new("Cancel")
                .on_click(move |ctx, _data: &mut T, _env| {
                    ctx
                        .window()
                        .close()
                }))
                .width(80.0)
                .height(30.0))
        )
    )
}