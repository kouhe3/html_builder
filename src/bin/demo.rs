#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(default_field_values)]

use html_builder::*;

fn main() {
    let document = Html { .. }([
        Head {}([Title {}(["Website title".into()])]),
        Body {}([
            Div {
                id: "header".into(),
                ..
            }([H1 { .. }(["Some Title".into()])]),
            Div {
                id: "content".into(),
                ..
            }([
                P { .. }(["Some text".into()]),
                Ul { .. }([
                    Li { .. }(["list 1".into()]),
                    Li { .. }(["list 2".into()]),
                ]),
            ]),
            Div {
                id: "footer".into(),
                ..
            }([P { .. }(["© 2025".into()])]),
        ]),
    ]);
    println!("{:?}", document);
}
