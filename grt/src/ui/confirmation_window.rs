//  This file is part of Sulis, a turn based RPG written in Rust.
//  Copyright 2018 Jared Stephen
//
//  Sulis is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  Sulis is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License//
//  along with Sulis.  If not, see <http://www.gnu.org/licenses/>

use std::rc::Rc;
use std::cell::RefCell;

use ui::{Button, Callback, Label, Widget, WidgetKind};

pub struct ConfirmationWindow {
    accept_callback: Callback,
}

impl ConfirmationWindow {
    pub fn new(accept_callback: Callback) -> Rc<ConfirmationWindow> {
        Rc::new(ConfirmationWindow {
            accept_callback
        })
    }
}

impl WidgetKind for ConfirmationWindow {
    fn get_name(&self) -> &str {
        "confirmation_window"
    }

    fn layout(&self, widget: &mut Widget) {
        widget.do_base_layout();
    }

    fn on_add(&self, _widget: &Rc<RefCell<Widget>>) -> Vec<Rc<RefCell<Widget>>> {
        let label = Widget::with_theme(Label::empty(), "title");

        let cancel = Widget::with_theme(Button::empty(), "cancel");
        cancel.borrow_mut().state.add_callback(Callback::remove_parent());

        let accept = Widget::with_theme(Button::empty(), "accept");
        accept.borrow_mut().state.add_callback(self.accept_callback.clone());

        vec![cancel, accept, label]
    }
}
