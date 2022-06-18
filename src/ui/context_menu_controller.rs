use druid::{Data, Env, Event, EventCtx, Menu, Widget};
use druid::widget::Controller;
use crate::AppState;

pub trait MenuFactory {
    fn make(&self) -> Menu<AppState>;
}

pub struct ContextMenuController {
    menu_factory: Box<dyn MenuFactory>
}

impl ContextMenuController {
    pub fn new<M: 'static + MenuFactory>(factory: M) -> Self {
        ContextMenuController {
            menu_factory: Box::new(factory)
        }
    }
}

impl<T: Data, W: Widget<T>> Controller<T, W> for ContextMenuController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::MouseDown(ref mouse) if mouse.button.is_right() => {
                let menu = self.menu_factory.make();
                ctx.show_context_menu(menu, mouse.pos);
            }
            _ => child.event(ctx, event, data, env),
        }
    }
}