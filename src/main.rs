#![feature(unboxed_closures)]
#![feature(fn_traits)]
use std::vec;
mod htmlbuilder;
use htmlbuilder::*;
fn 略<T>() -> T
where
    T: Default,
{
    Default::default()
}
fn main() {
    let document = Html { ..略() }(vec![
        Head { ..略() }(vec![Title { ..略() }(vec![Element::Text("Website title")])]),
        Body { ..略() }(vec![
            Div {
                id: Some("header"),
                ..略()
            }(vec![H1 { ..略() }(vec![Element::Text("Some Title")])]),
            Div {
                id: Some("content"),
                ..略()
            }(vec![
                P { ..略() }(vec![Element::Text("Some text")]),
                Ul { ..略() }(vec![
                    Li { ..略() }(vec![Element::Text("list 1")]),
                    Li { ..略() }(vec![Element::Text("list 2")]),
                ]),
            ]),
            Div {
                id: Some("footer"),
                ..略()
            }(vec![P { ..略() }(vec![Element::Text("© 2025")])]),
        ]),
    ]);
    println!("{:?}", document);
}
