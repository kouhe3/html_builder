#![feature(default_field_values)]

use html_builder::*;

fn main() {
    let document = Html { .. }.child([
        Head {}.child([Title {}.child(["Website title".into()])]),
        Body {}.child([
            Div {
                id: "header".into(),
                ..
            }
            .child([H1 { .. }.child(["Some Title".into()])]),
            Div {
                id: "content".into(),
                ..
            }
            .child([
                P { .. }.child(["Some text".into()]),
                Ul { .. }.child([
                    Li { .. }.child(["list 1".into()]),
                    Li { .. }.child(["list 2".into()]),
                ]),
            ]),
            Div {
                id: "footer".into(),
                ..
            }
            .child([P { .. }.child(["© 2025".into()])]),
        ]),
    ]);
    println!("{:?}", document);
}
