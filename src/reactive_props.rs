use std::{fmt::Display, rc::Rc};

use sycamore::reactive::{RcSignal, ReadSignal, Signal};

#[derive(Clone)]
pub struct ReactiveBool<'cx>(Rc<dyn Fn() -> bool + 'cx>);

impl<'cx> ReactiveBool<'cx> {
    pub fn get(&self) -> bool {
        (self.0)()
    }
}

impl<'cx> From<&'cx Signal<bool>> for ReactiveBool<'cx> {
    fn from(value: &'cx Signal<bool>) -> Self {
        ReactiveBool(Rc::new(move || *value.get()))
    }
}
impl<'cx> From<&'cx ReadSignal<bool>> for ReactiveBool<'cx> {
    fn from(value: &'cx ReadSignal<bool>) -> Self {
        ReactiveBool(Rc::new(move || *value.get()))
    }
}
impl From<RcSignal<bool>> for ReactiveBool<'static> {
    fn from(value: RcSignal<bool>) -> Self {
        ReactiveBool(Rc::new(move || *value.get()))
    }
}
impl<'cx, F> From<F> for ReactiveBool<'cx>
where
    F: Fn() -> bool + 'cx,
{
    fn from(value: F) -> Self {
        ReactiveBool(Rc::new(move || value()))
    }
}
impl From<bool> for ReactiveBool<'static> {
    fn from(value: bool) -> Self {
        ReactiveBool(Rc::new(move || value))
    }
}

impl Default for ReactiveBool<'static> {
    fn default() -> Self {
        ReactiveBool(Rc::new(|| false))
    }
}

#[derive(Clone)]
pub struct ReactiveStr<'cx>(Rc<dyn Fn() -> String + 'cx>);

impl<'cx> ReactiveStr<'cx> {
    pub fn get(&self) -> String {
        (self.0)()
    }
}

impl<'cx, D: Display + 'cx> From<D> for ReactiveStr<'cx> {
    fn from(value: D) -> Self {
        ReactiveStr(Rc::new(move || value.to_string()))
    }
}

impl Default for ReactiveStr<'static> {
    fn default() -> Self {
        ReactiveStr(Rc::new(|| String::new()))
    }
}
