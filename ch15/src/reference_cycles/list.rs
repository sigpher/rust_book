use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}
