use druid::{commands, Data, Env, Event, EventCtx, Target, Widget};
use druid::widget::Controller;
use druid_shell::{HotKey, SysMods};

pub struct CopyCutPasteController;

impl<T: Data, W: Widget<T>> Controller<T, W> for CopyCutPasteController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::KeyDown(k_e) if HotKey::new(SysMods::Cmd, "c").matches(k_e) => {
                ctx.submit_command(commands::COPY.to(Target::Auto));
                ctx.set_handled();
            }
            Event::KeyDown(k_e) if HotKey::new(SysMods::Cmd, "x").matches(k_e) => {
                ctx.submit_command(commands::CUT.to(Target::Auto));
                ctx.set_handled();
            }
            Event::KeyDown(k_e) if HotKey::new(SysMods::Cmd, "v").matches(k_e) => {
                ctx.submit_command(commands::PASTE.to(Target::Auto));
                ctx.set_handled();
            }
            e => child.event(ctx, e, data, env),
        };
    }
}