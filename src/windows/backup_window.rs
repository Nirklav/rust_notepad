use druid::{lens, Widget, WidgetExt, WindowDesc};
use druid::widget::{Align, Button, CrossAxisAlignment, Flex, Label, Padding, SizedBox, Switch, TextBox};
use crate::{AppState, windows};
use crate::windows::primary_screen_center;

pub fn new() -> WindowDesc<AppState> {
    let size = (400.0, 200.0);
    WindowDesc::new(ui())
        .title("Backup")
        .window_size(size)
        .resizable(false)
        .set_position(primary_screen_center(size))
}

pub fn ui() -> impl Widget<AppState> {
    Padding::new(10.0, Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::End)
        .with_child(Align::left(Flex::row()
            .with_child(SizedBox::new(Button::new("Made backup")
                .on_click(|cx, state: &mut AppState, _env| {
                    if let Err(e) = state.made_backup() {
                        let message = format!("Cannot make backup: {}", e);
                        cx.new_window(windows::information_window::new(message));
                    }
                }))
                .width(120.0)
                .height(30.0))
            .with_spacer(10.0)
            .with_child(SizedBox::new(Button::new("Restore backup")
                .on_click(|cx, state: &mut AppState, _env| {
                    if let Err(e) = state.restore_backup() {
                        let message = format!("Cannot restore backup: {}", e);
                        cx.new_window(windows::information_window::new(message));
                    }
                }))
                .width(120.0)
                .height(30.0)))
            .expand_width())
        .with_spacer(10.0)
        .with_child(Flex::row()
            .with_flex_child(Label::new("Backup after each save").expand_width(), 1.0)
            .with_spacer(10.0)
            .with_child(Switch::new()
                .lens(lens!(AppState, config.auto_backup))))
        .with_spacer(10.0)
        .with_child(Flex::row()
            .with_child(SizedBox::new(Label::new("Name")))
            .with_spacer(10.0)
            .with_flex_child(TextBox::new()
                .lens(lens!(AppState, config.backup_folder))
                .expand_width(), 1.0)
            .with_spacer(5.0)
            .with_child(Label::new("/"))
            .with_spacer(5.0)
            .with_flex_child(TextBox::new()
                .lens(lens!(AppState, config.backup_file))
                .expand_width(), 1.0))
        .with_spacer(10.0)
        .with_child(Flex::row()
            .with_child(SizedBox::new(Button::new("Ok")
                .on_click(|cx, state: &mut AppState, _env| {
                    if let Err(e) = state.config.save() {
                        let message = format!("Cannot save configs: {}", e);
                        cx.new_window(windows::information_window::new(message));
                    }
                    cx.window().close()
                }))
                .width(80.0)
                .height(30.0))))
}