use druid::kurbo::{Circle, Line};
use druid::{theme, RenderContext, Widget, WidgetExt};
use druid::widget::Painter;
use crate::state::tabs_state::TabsState;

pub fn close_button() -> impl Widget<TabsState> {
    Painter::new(|ctx, _, env| {
        let circle_bounds = ctx.size().to_rect().inset(-2.);
        let cross_bounds = circle_bounds.inset(-5.);
        if ctx.is_hot() {
            ctx.render_ctx.fill(
                Circle::new(
                    circle_bounds.center(),
                    f64::min(circle_bounds.height(), circle_bounds.width()) / 2.,
                ),
                &env.get(theme::BORDER_LIGHT),
            );
        }
        let cross_color = &env.get(if ctx.is_hot() {
            theme::BACKGROUND_DARK
        } else {
            theme::BORDER_LIGHT
        });
        ctx.render_ctx.stroke(
            Line::new(
                (cross_bounds.x0, cross_bounds.y0),
                (cross_bounds.x1, cross_bounds.y1),
            ),
            cross_color,
            2.,
        );
        ctx.render_ctx.stroke(
            Line::new(
                (cross_bounds.x1, cross_bounds.y0),
                (cross_bounds.x0, cross_bounds.y1),
            ),
            cross_color,
            2.,
        );
    }).fix_size(20., 20.)
}