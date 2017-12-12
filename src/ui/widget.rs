use std::rc::Rc;

use state::GameState;
use io::{Event, TextRenderer};
use ui::{Border, Size, WidgetState, WidgetKind};
use resource::Point;

pub struct Widget<'a> {
    pub state: WidgetState,
    pub kind: Rc<WidgetKind<'a>>,
    pub children: Vec<Widget<'a>>,
}

impl<'a> Widget<'a> {
    fn new(kind: Rc<WidgetKind<'a>>, size: Size, position: Point,
           border: Border) -> Widget<'a> {
        let state = WidgetState::new(size, position, border);

        Widget {
            state,
            kind,
            children: Vec::new(),
        }
    }

    pub fn with_defaults(widget: Rc<WidgetKind<'a>>) -> Widget<'a> {
        Widget::new(widget, Size::as_zero(), Point::as_zero(), Border::as_zero())
    }

    pub fn with_size(widget: Rc<WidgetKind<'a>>,
                     size: Size) -> Widget<'a> {
        Widget::new(widget, size, Point::as_zero(), Border::as_zero())
    }

    pub fn with_position(widget: Rc<WidgetKind<'a>>, size: Size,
                         position: Point) -> Widget<'a> {
        Widget::new(widget, size, position, Border::as_zero())
    }

    pub fn with_border(widget: Rc<WidgetKind<'a>>, size: Size,
                       position: Point, border: Border) -> Widget<'a> {
        Widget::new(widget, size, position, border)
    }

    pub fn add_child(&mut self, widget: Widget<'a>) {
        trace!("Adding {:?} to {:?}", widget.kind.get_name(), self.kind.get_name());
        self.children.push(widget);
    }

    pub fn dispatch_event(&mut self, state: &mut GameState, event: Event) -> bool {
        trace!("Dispatching event {:?} in {:?}", event, self.kind.get_name());
        for child in self.children.iter_mut() {
            if child.state.in_bounds(event.mouse) {
                if !child.state.mouse_is_inside {
                    child.dispatch_event(state, Event::entered_from(&event));
                }

                if child.dispatch_event(state, event) {
                    return true;
                }
            } else if child.state.mouse_is_inside {
                child.dispatch_event(state, Event::exited_from(&event));
            }
        }

        let ref widget_kind = self.kind;
        use io::event::Kind::*;
        match event.kind {
            MouseClick(kind) => widget_kind.on_mouse_click(state, kind, event.mouse),
            MouseMove { change: _change } => widget_kind.on_mouse_move(state,
                                                                  event.mouse),
            MouseEnter => widget_kind.on_mouse_enter(state, event.mouse),
            MouseExit => widget_kind.on_mouse_exit(state, event.mouse),
            MouseScroll { scroll } => widget_kind.on_mouse_scroll(state,
                                                          scroll, event.mouse),
            KeyPress(action) => widget_kind.on_key_press(state, action, event.mouse),
        }
    }

    pub fn draw_text_mode(&self, renderer: &mut TextRenderer) {
        if let Some(ref image) = self.state.background {
            image.fill_text_mode(renderer, self.state.animation_state.get_text(),
                &self.state.position, &self.state.size);
        }

        self.kind.draw_text_mode(renderer);

        for child in self.children.iter() {
            child.draw_text_mode(renderer);
        }
    }
}
