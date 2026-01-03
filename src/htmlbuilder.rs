use std::fmt::Debug;
/* FOR NO NIGHTLY
pub trait Has {
    fn has(self, childs: Vec<Element>) -> Element;
}
impl Has for Html {
    fn has(self, childs: Vec<Element>) -> Element {
        self(childs)
    }
}
impl Has for Div {
    fn has(self, childs: Vec<Element>) -> Element {
        self(childs)
    }
}
impl Has for P {
    fn has(self, childs: Vec<Element>) -> Element {
        self(childs)
    }
}
*/
pub enum Element {
    Tag(Tag),
    Text(&'static str),
    String(String),
}
impl Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Tag(tag) => write!(f, "{:?}", tag),
            Element::Text(text) => write!(f, "{}", text),
            Element::String(string) => write!(f, "{}", string),
        }
    }
}
struct Tag {
    name: &'static str,
    children: Vec<Element>,
    attributes: Vec<(&'static str, &'static str)>,
}
impl From<Tag> for Element {
    fn from(value: Tag) -> Self {
        Element::Tag(value)
    }
}
impl From<&'static str> for Element {
    fn from(value: &'static str) -> Self {
        Element::Text(value)
    }
}
impl From<String> for Element {
    fn from(value: String) -> Self {
        Element::String(value)
    }
}
impl Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}", self.name)?;
        for (key, value) in &self.attributes {
            write!(f, " {}=\"{}\"", key, value)?;
        }
        writeln!(f, ">")?;
        for child in &self.children {
            write!(f, "{:?}", child)?;
        }
        write!(f, "\n</{}>", self.name)?;
        Ok(())
    }
}

macro_rules! define_element {
    ($name:ident, $tag:expr, $($attr:ident),*) => {
        #[derive(Default)]
        pub struct $name {
            $(
                pub $attr: Option<&'static str>,
            )*
        }

        impl<T: Into<Vec<Element>>> FnOnce<(T,)> for $name {
            type Output = Element;
            extern "rust-call" fn call_once(self, args: (T,)) -> Self::Output {
                let mut attributes = Vec::new();
                $(
                    if let Some(val) = self.$attr {
                        attributes.push((stringify!($attr), val));
                    }
                )*
                Element::Tag(Tag {
                    name: $tag,
                    children: args.0.into(),
                    attributes,
                })
            }
        }
    };
}


define_element!(Html, "html", lang);
define_element!(Head, "head",);
define_element!(Body, "body",);
define_element!(Title, "title",);
define_element!(Meta, "meta", charset, name, content);
define_element!(Link, "link", rel, href, r#type);
define_element!(Div, "div", id, class, style);
define_element!(Section, "section", id, class);
define_element!(Article, "article", id, class);
define_element!(Aside, "aside", id, class);
define_element!(Header, "header", id, class);
define_element!(Footer, "footer", id, class);
define_element!(Main, "main", id, class);
define_element!(H1, "h1", id, class);
define_element!(H2, "h2", id, class);
define_element!(H3, "h3", id, class);
define_element!(P, "p", id, class);
define_element!(Span, "span", id, class);
define_element!(Br, "br",);
define_element!(Hr, "hr",);
define_element!(Ul, "ul", id, class);
define_element!(Ol, "ol", id, class, start);
define_element!(Li, "li", id, class, value);
define_element!(A, "a", href, target, rel, id, class);
define_element!(Img, "img", src, alt, width, height, loading);
define_element!(Iframe, "iframe", src, width, height, frameborder);
define_element!(Table, "table", id, class);
define_element!(Thead, "thead",);
define_element!(Tbody, "tbody",);
define_element!(Tr, "tr",);
define_element!(Th, "th", scope, colspan, rowspan);
define_element!(Td, "td", colspan, rowspan);
define_element!(Form, "form", action, method, enctype);
define_element!(Input, "input", r#type, name, value, placeholder, disabled, required, min, max, step);
define_element!(Textarea, "textarea", name, rows, cols, placeholder, disabled, required);
define_element!(Select, "select", name, disabled, multiple);
//define_element!(r#Option, "option", value, selected);
define_element!(Button, "button", r#type, name, value, disabled);
define_element!(Label, "label", r#for);
define_element!(Strong, "strong",);
define_element!(Em, "em",);
define_element!(Code, "code",);
define_element!(Pre, "pre",);
define_element!(Blockquote, "blockquote", cite);
define_element!(Cite, "cite",);
define_element!(Script, "script", src, r#type, r#async, defer);
define_element!(Style, "style", r#type);
define_element!(Nav, "nav", id, class);
define_element!(Figure, "figure",);
define_element!(Figcaption, "figcaption",);
define_element!(Time, "time", datetime);
define_element!(Mark, "mark",);
define_element!(Small, "small",);