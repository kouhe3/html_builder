#![feature(unboxed_closures)]
#![feature(fn_traits)]
use std::vec;
mod htmlbuilder;
use htmlbuilder::*;
fn default<T>() -> T
where
    T: Default,
{
    Default::default()
}

fn main() {
    let document = Html { ..default() }([
        Head { ..default() }([Title { ..default() }(["Website title".into()])]),
        Body { ..default() }([
            Div {
                id: "header".into(),
                ..default()
            }([H1 { ..default() }(["Some Title".into()])]),
            Div {
                id: "content".into(),
                ..default()
            }([
                P { ..default() }(["Some text".into()]),
                Ul { ..default() }([
                    Li { ..default() }(["list 1".into()]),
                    Li { ..default() }(["list 2".into()]),
                ]),
            ]),
            Div {
                id: "footer".into(),
                ..default()
            }([P { ..default() }(["© 2025".into()])]),
        ]),
    ]);
    println!("{:?}", document);
}
