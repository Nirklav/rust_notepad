use druid::{lens, Widget, WidgetExt, WindowDesc};
use druid::widget::{Button, Flex, Label, MainAxisAlignment, Padding, SizedBox, Tabs, TextBox};
use crate::{AppState, windows};
use crate::windows::primary_screen_center;

pub fn new() -> WindowDesc<AppState> {
    let size = (400.0, 185.0);
    WindowDesc::new(ui())
        .title("New file")
        .window_size(size)
        .resizable(false)
        .set_position(primary_screen_center(size))
}

fn ui() -> impl Widget<AppState> {
    Tabs::new()
        .with_tab("Clear", clear_ui())
        .with_tab("Protected", protected_ui())
}

const LABEL_WIDTH : f64 = 80.0;

fn clear_ui() -> impl Widget<AppState> {
    Padding::new(5.0, Flex::column()
        .with_child(Flex::row()
            .with_child(SizedBox::new(Label::new("Name:"))
                .width(LABEL_WIDTH))
            .with_spacer(5.0)
            .with_flex_child(TextBox::new()
                .expand_width()
                .lens(lens!(AppState, new_tab.name)), 1.0))
        .with_spacer(5.0)
        .with_child(Flex::row()
            .main_axis_alignment(MainAxisAlignment::End)
            .with_child(SizedBox::new(Button::new("Ok")
                .on_click(|ctx, state: &mut AppState, _env| {
                    if let Err(e) = state.add_new_clear_tab() {
                        ctx.new_window(windows::information_window::new(format!("Error: {}", e)));
                    };
                    ctx
                        .window()
                        .close();
                }))
                .width(70.0)
                .height(30.0))
            .expand_width()))
}

fn protected_ui() -> impl Widget<AppState> {
    Padding::new(5.0, Flex::column()
        .with_child(Flex::row()
            .with_child(SizedBox::new(Label::new("Name:"))
                .width(LABEL_WIDTH))
            .with_spacer(5.0)
            .with_flex_child(TextBox::new()
                .expand_width()
                .lens(lens!(AppState, new_tab.name)), 1.0))
        .with_spacer(5.0)
        .with_child(Flex::row()
            .with_child(SizedBox::new(Label::new("Password:"))
                .width(LABEL_WIDTH))
            .with_spacer(5.0)
            .with_flex_child(TextBox::new()
                .expand_width()
                .lens(lens!(AppState, new_tab.password)), 1.0))
        .with_spacer(5.0)
        .with_child(Flex::row()
            .main_axis_alignment(MainAxisAlignment::End)
            .with_child(SizedBox::new(Button::new("Ok")
                .on_click(|ctx, state: &mut AppState, _env| {
                    if let Err(e) = state.add_new_protected_tab() {
                        ctx.new_window(windows::information_window::new(format!("Error: {}", e)));
                    };
                    ctx
                        .window()
                        .close();
                }))
                .width(70.0)
                .height(30.0))
            .expand_width()))
}