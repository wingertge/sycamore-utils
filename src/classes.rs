use std::fmt::Display;

use sycamore::reactive::{RcSignal, ReadSignal, Signal};

use crate::ReactiveStr;
pub trait ToClass {
    fn to_class(self) -> Option<String>;
}

impl ToClass for &str {
    fn to_class(self) -> Option<String> {
        Some(self.to_string())
    }
}
impl ToClass for String {
    fn to_class(self) -> Option<String> {
        Some(self)
    }
}

impl<D: Display> ToClass for RcSignal<D> {
    fn to_class(self) -> Option<String> {
        Some(self.get().to_string())
    }
}
impl<'cx, D: Display> ToClass for &'cx Signal<D> {
    fn to_class(self) -> Option<String> {
        Some(self.get().to_string())
    }
}
impl<'cx, D: Display> ToClass for &'cx ReadSignal<D> {
    fn to_class(self) -> Option<String> {
        Some(self.get().to_string())
    }
}

impl<D: Display> ToClass for Option<D> {
    fn to_class(self) -> Option<String> {
        self.map(|d| d.to_string())
    }
}

impl<'a, T: Into<ReactiveStr<'a>>> ToClass for (bool, T) {
    fn to_class(self) -> Option<String> {
        if self.0 {
            Some(self.1.into().get())
        } else {
            None
        }
    }
}

impl<'a, T1: Into<ReactiveStr<'a>>, T2: Into<ReactiveStr<'a>>> ToClass for (bool, T1, T2) {
    fn to_class(self) -> Option<String> {
        if self.0 {
            Some(self.1.into().get())
        } else {
            Some(self.2.into().get())
        }
    }
}

#[macro_export]
macro_rules! classes {
    [$cx: expr, $($class:expr),*] => {
        ::sycamore::reactive::create_memo($cx, || {
            let classes = ::std::vec![$(::sycamore_utils::ToClass::to_class($class)),*];
            <[::std::string::String]>::join(
                &::std::iter::Iterator::collect::<::std::vec::Vec<::std::string::String>>(
                    ::std::iter::Iterator::filter_map(::std::iter::IntoIterator::into_iter(classes), |c| c)
                ),
                " "
            )
        })
    };
}
