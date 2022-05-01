use druid::{Point, Size};
use druid_shell::Screen;

pub mod main_window;
pub mod dialog_window;
pub mod new_file_window;

pub fn primary_screen_center(size: impl Into<Size>) -> Point {
    let monitors = Screen::get_monitors();
    for monitor in &monitors {
        if monitor.is_primary() {
            let rect = monitor.virtual_work_rect();
            let center = rect.center();
            let size = size.into();
            return Point::new(center.x - size.width / 2.0, center.y - size.height / 2.0);
        }
    }
    panic!("Primary screen not found");
}