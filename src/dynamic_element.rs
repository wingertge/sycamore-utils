use sycamore::{builder::ElementBuilder, prelude::*};

pub struct DynamicElement<'cx, G: Html>(Box<dyn FnOnce(Scope<'cx>) -> View<G> + 'cx>);

impl<'cx, G: Html> DynamicElement<'cx, G> {
    pub fn call(self, cx: Scope<'cx>) -> View<G> {
        (self.0)(cx)
    }
}

impl<'cx, G: Html, B, F: FnOnce(Scope<'cx>) -> G + 'cx> From<B> for DynamicElement<'cx, G>
where
    B: Fn() -> ElementBuilder<'cx, G, F> + 'static,
{
    fn from(value: B) -> Self {
        Self(Box::new(move |cx| value().view(cx)))
    }
}
