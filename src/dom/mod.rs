use phf::{phf_ordered_map, OrderedMap};
#[cfg(test)]
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum DOMElements {
    A,
    Abbr,
    Acronym,
    Address,
    Applet,
    Area,
    Article,
    Aside,
    Audio,
    B,
    Base,
    Bdi,
    Bdo,
    Big,
    Blink,
    Blockquote,
    Body,
    Br,
    Button,
    Canvas,
    Caption,
    Center,
    Cite,
    Code,
    Col,
    Colgroup,
    Content,
    Data,
    Datalist,
    Dd,
    Del,
    Details,
    Dfn,
    Dialog,
    Dir,
    Div,
    Dl,
    Dt,
    Em,
    Embed,
    Fieldset,
    Figcaption,
    Figure,
    Font,
    Footer,
    Form,
    Frame,
    Frameset,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Head,
    Header,
    Hgroup,
    Hr,
    Html,
    I,
    Iframe,
    Img,
    Input,
    Ins,
    Kbd,
    Keygen,
    Label,
    Legend,
    Li,
    Link,
    Main,
    Map,
    Mark,
    Marquee,
    Menu,
    MenuItem,
    Meta,
    Meter,
    Nav,
    Noembed,
    Noscript,
    Object,
    Ol,
    Optgroup,
    Option,
    Output,
    P,
    Param,
    Picture,
    Pre,
    Progress,
    Q,
    Rp,
    Rt,
    Rtc,
    Ruby,
    S,
    Samp,
    Script,
    Section,
    Select,
    Small,
    Source,
    Spacer,
    Span,
    Strike,
    Strong,
    Style,
    Sub,
    Summary,
    Sup,
    Table,
    Tbody,
    Td,
    Textarea,
    Tfoot,
    Th,
    Thead,
    Time,
    Title,
    Tr,
    Track,
    Tt,
    U,
    Ul,
    Var,
    Video,
    Wbr,
    Xmp,
}

#[cfg(test)]
impl Serialize for DOMElements {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let raw_dom_element = map_enum_dom_element_to_raw(self);
        raw_dom_element.serialize(serializer)
    }
}

static DOM_ELEMENTS: OrderedMap<&'static str, bool> = phf_ordered_map! {
    "a" => false,
    "abbr" => false,
    "acronym" => false,
    "address" => false,
    "applet" => false,
    "area" => false,
    "article" => false,
    "aside" => false,
    "audio" => false,
    "b" => false,
    "base" => true,
    "bdi" => false,
    "bdo" => false,
    "big" => false,
    "blink" => false,
    "blockquote" => false,
    "body" => false,
    "br" => false,
    "button" => false,
    "canvas" => false,
    "caption" => false,
    "center" => false,
    "cite" => false,
    "code" => false,
    "col" => true,
    "colgroup" => true,
    "content" => false,
    "data" => false,
    "datalist" => false,
    "dd" => false,
    "del" => false,
    "details" => false,
    "dfn" => false,
    "dialog" => false,
    "dir" => false,
    "div" => false,
    "dl" => false,
    "dt" => false,
    "em" => false,
    "embed" => false,
    "fieldset" => false,
    "figcaption" => false,
    "figure" => false,
    "font" => false,
    "footer" => false,
    "form" => false,
    "frame" => false,
    "frameset" => false,
    "h1" => false,
    "h2" => false,
    "h3" => false,
    "h4" => false,
    "h5" => false,
    "h6" => false,
    "head" => true,
    "header" => false,
    "hgroup" => false,
    "hr" => false,
    "html" => true,
    "i" => false,
    "iframe" => false,
    "img" => false,
    "input" => false,
    "ins" => false,
    "kbd" => false,
    "keygen" => false,
    "label" => false,
    "legend" => false,
    "li" => false,
    "link" => true,
    "main" => false,
    "map" => false,
    "mark" => false,
    "marquee" => false,
    "menu" => false,
    "menuitem" => false,
    "meta" => true,
    "meter" => false,
    "nav" => false,
    "noembed" => true,
    "noscript" => true,
    "object" => false,
    "ol" => false,
    "optgroup" => false,
    "option" => false,
    "output" => false,
    "p" => false,
    "param" => true,
    "picture" => true,
    "pre" => false,
    "progress" => false,
    "q" => false,
    "rp" => false,
    "rt" => false,
    "rtc" => false,
    "ruby" => false,
    "s" => false,
    "samp" => false,
    "script" => true,
    "section" => false,
    "select" => false,
    "small" => false,
    "source" => true,
    "spacer" => false,
    "span" => false,
    "strike" => false,
    "strong" => false,
    "style" => true,
    "sub" => false,
    "summary" => false,
    "sup" => false,
    "table" => false,
    "tbody" => false,
    "td" => false,
    "textarea" => false,
    "tfoot" => false,
    "th" => false,
    "thead" => false,
    "time" => false,
    "title" => true,
    "tr" => false,
    "track" => true,
    "tt" => false,
    "u" => false,
    "ul" => false,
    "var" => false,
    "video" => false,
    "wbr" => false,
    "xmp" => false,
};

fn map_raw_dom_element_to_enum(raw_dom_element: &'static str) -> DOMElements {
    match raw_dom_element {
        "a" => DOMElements::A,
        "abbr" => DOMElements::Abbr,
        "acronym" => DOMElements::Acronym,
        "address" => DOMElements::Address,
        "applet" => DOMElements::Applet,
        "area" => DOMElements::Area,
        "article" => DOMElements::Article,
        "aside" => DOMElements::Aside,
        "audio" => DOMElements::Audio,
        "b" => DOMElements::B,
        "base" => DOMElements::Base,
        "bdi" => DOMElements::Bdi,
        "bdo" => DOMElements::Bdo,
        "big" => DOMElements::Big,
        "blink" => DOMElements::Blink,
        "blockquote" => DOMElements::Blockquote,
        "body" => DOMElements::Body,
        "br" => DOMElements::Br,
        "button" => DOMElements::Button,
        "canvas" => DOMElements::Canvas,
        "caption" => DOMElements::Caption,
        "center" => DOMElements::Center,
        "cite" => DOMElements::Cite,
        "code" => DOMElements::Code,
        "col" => DOMElements::Col,
        "colgroup" => DOMElements::Colgroup,
        "content" => DOMElements::Content,
        "data" => DOMElements::Data,
        "datalist" => DOMElements::Datalist,
        "dd" => DOMElements::Dd,
        "del" => DOMElements::Del,
        "details" => DOMElements::Details,
        "dfn" => DOMElements::Dfn,
        "dialog" => DOMElements::Dialog,
        "dir" => DOMElements::Dir,
        "div" => DOMElements::Div,
        "dl" => DOMElements::Dl,
        "dt" => DOMElements::Dt,
        "em" => DOMElements::Em,
        "embed" => DOMElements::Embed,
        "fieldset" => DOMElements::Fieldset,
        "figcaption" => DOMElements::Figcaption,
        "figure" => DOMElements::Figure,
        "font" => DOMElements::Font,
        "footer" => DOMElements::Footer,
        "form" => DOMElements::Form,
        "frame" => DOMElements::Frame,
        "frameset" => DOMElements::Frameset,
        "h1" => DOMElements::H1,
        "h2" => DOMElements::H2,
        "h3" => DOMElements::H3,
        "h4" => DOMElements::H4,
        "h5" => DOMElements::H5,
        "h6" => DOMElements::H6,
        "head" => DOMElements::Head,
        "header" => DOMElements::Header,
        "hgroup" => DOMElements::Hgroup,
        "hr" => DOMElements::Hr,
        "html" => DOMElements::Html,
        "i" => DOMElements::I,
        "iframe" => DOMElements::Iframe,
        "img" => DOMElements::Img,
        "input" => DOMElements::Input,
        "ins" => DOMElements::Ins,
        "kbd" => DOMElements::Kbd,
        "keygen" => DOMElements::Keygen,
        "label" => DOMElements::Label,
        "legend" => DOMElements::Legend,
        "li" => DOMElements::Li,
        "link" => DOMElements::Link,
        "main" => DOMElements::Main,
        "map" => DOMElements::Map,
        "mark" => DOMElements::Mark,
        "marquee" => DOMElements::Marquee,
        "menu" => DOMElements::Menu,
        "menuitem" => DOMElements::MenuItem,
        "meta" => DOMElements::Meta,
        "meter" => DOMElements::Meter,
        "nav" => DOMElements::Nav,
        "noembed" => DOMElements::Noembed,
        "noscript" => DOMElements::Noscript,
        "object" => DOMElements::Object,
        "ol" => DOMElements::Ol,
        "optgroup" => DOMElements::Optgroup,
        "option" => DOMElements::Option,
        "output" => DOMElements::Output,
        "p" => DOMElements::P,
        "param" => DOMElements::Param,
        "picture" => DOMElements::Picture,
        "pre" => DOMElements::Pre,
        "progress" => DOMElements::Progress,
        "q" => DOMElements::Q,
        "rp" => DOMElements::Rp,
        "rt" => DOMElements::Rt,
        "rtc" => DOMElements::Rtc,
        "ruby" => DOMElements::Ruby,
        "s" => DOMElements::S,
        "samp" => DOMElements::Samp,
        "script" => DOMElements::Script,
        "section" => DOMElements::Section,
        "select" => DOMElements::Select,
        "small" => DOMElements::Small,
        "source" => DOMElements::Source,
        "spacer" => DOMElements::Spacer,
        "span" => DOMElements::Span,
        "strike" => DOMElements::Strike,
        "strong" => DOMElements::Strong,
        "style" => DOMElements::Style,
        "sub" => DOMElements::Sub,
        "summary" => DOMElements::Summary,
        "sup" => DOMElements::Sup,
        "table" => DOMElements::Table,
        "tbody" => DOMElements::Tbody,
        "td" => DOMElements::Td,
        "textarea" => DOMElements::Textarea,
        "tfoot" => DOMElements::Tfoot,
        "th" => DOMElements::Th,
        "thead" => DOMElements::Thead,
        "time" => DOMElements::Time,
        "title" => DOMElements::Title,
        "tr" => DOMElements::Tr,
        "track" => DOMElements::Track,
        "tt" => DOMElements::Tt,
        "u" => DOMElements::U,
        "ul" => DOMElements::Ul,
        "var" => DOMElements::Var,
        "video" => DOMElements::Video,
        "wbr" => DOMElements::Wbr,
        "xmp" => DOMElements::Xmp,
        _ => unreachable!("Unknown DOM element: {}", raw_dom_element),
    }
}

fn map_enum_dom_element_to_raw(dom_element: &DOMElements) -> &'static str {
    match dom_element {
        DOMElements::A => "a",
        DOMElements::Abbr => "abbr",
        DOMElements::Acronym => "acronym",
        DOMElements::Address => "address",
        DOMElements::Applet => "applet",
        DOMElements::Area => "area",
        DOMElements::Article => "article",
        DOMElements::Aside => "aside",
        DOMElements::Audio => "audio",
        DOMElements::B => "b",
        DOMElements::Base => "base",
        DOMElements::Bdi => "bdi",
        DOMElements::Bdo => "bdo",
        DOMElements::Big => "big",
        DOMElements::Blink => "blink",
        DOMElements::Blockquote => "blockquote",
        DOMElements::Body => "body",
        DOMElements::Br => "br",
        DOMElements::Button => "button",
        DOMElements::Canvas => "canvas",
        DOMElements::Caption => "caption",
        DOMElements::Center => "center",
        DOMElements::Cite => "cite",
        DOMElements::Code => "code",
        DOMElements::Col => "col",
        DOMElements::Colgroup => "colgroup",
        DOMElements::Content => "content",
        DOMElements::Data => "data",
        DOMElements::Datalist => "datalist",
        DOMElements::Dd => "dd",
        DOMElements::Del => "del",
        DOMElements::Details => "details",
        DOMElements::Dfn => "dfn",
        DOMElements::Dialog => "dialog",
        DOMElements::Dir => "dir",
        DOMElements::Div => "div",
        DOMElements::Dl => "dl",
        DOMElements::Dt => "dt",
        DOMElements::Em => "em",
        DOMElements::Embed => "embed",
        DOMElements::Fieldset => "fieldset",
        DOMElements::Figcaption => "figcaption",
        DOMElements::Figure => "figure",
        DOMElements::Font => "font",
        DOMElements::Footer => "footer",
        DOMElements::Form => "form",
        DOMElements::Frame => "frame",
        DOMElements::Frameset => "frameset",
        DOMElements::H1 => "h1",
        DOMElements::H2 => "h2",
        DOMElements::H3 => "h3",
        DOMElements::H4 => "h4",
        DOMElements::H5 => "h5",
        DOMElements::H6 => "h6",
        DOMElements::Head => "head",
        DOMElements::Header => "header",
        DOMElements::Hgroup => "hgroup",
        DOMElements::Hr => "hr",
        DOMElements::Html => "html",
        DOMElements::I => "i",
        DOMElements::Iframe => "iframe",
        DOMElements::Img => "img",
        DOMElements::Input => "input",
        DOMElements::Ins => "ins",
        DOMElements::Kbd => "kbd",
        DOMElements::Keygen => "keygen",
        DOMElements::Label => "label",
        DOMElements::Legend => "legend",
        DOMElements::Li => "li",
        DOMElements::Link => "link",
        DOMElements::Main => "main",
        DOMElements::Map => "map",
        DOMElements::Mark => "mark",
        DOMElements::Marquee => "marquee",
        DOMElements::Menu => "menu",
        DOMElements::MenuItem => "menuitem",
        DOMElements::Meta => "meta",
        DOMElements::Meter => "meter",
        DOMElements::Nav => "nav",
        DOMElements::Noembed => "noembed",
        DOMElements::Noscript => "noscript",
        DOMElements::Object => "object",
        DOMElements::Ol => "ol",
        DOMElements::Optgroup => "optgroup",
        DOMElements::Option => "option",
        DOMElements::Output => "output",
        DOMElements::P => "p",
        DOMElements::Param => "param",
        DOMElements::Picture => "picture",
        DOMElements::Pre => "pre",
        DOMElements::Progress => "progress",
        DOMElements::Q => "q",
        DOMElements::Rp => "rp",
        DOMElements::Rt => "rt",
        DOMElements::Rtc => "rtc",
        DOMElements::Ruby => "ruby",
        DOMElements::S => "s",
        DOMElements::Samp => "samp",
        DOMElements::Script => "script",
        DOMElements::Section => "section",
        DOMElements::Select => "select",
        DOMElements::Small => "small",
        DOMElements::Source => "source",
        DOMElements::Spacer => "spacer",
        DOMElements::Span => "span",
        DOMElements::Strike => "strike",
        DOMElements::Strong => "strong",
        DOMElements::Style => "style",
        DOMElements::Sub => "sub",
        DOMElements::Summary => "summary",
        DOMElements::Sup => "sup",
        DOMElements::Table => "table",
        DOMElements::Tbody => "tbody",
        DOMElements::Td => "td",
        DOMElements::Textarea => "textarea",
        DOMElements::Tfoot => "tfoot",
        DOMElements::Th => "th",
        DOMElements::Thead => "thead",
        DOMElements::Time => "time",
        DOMElements::Title => "title",
        DOMElements::Tr => "tr",
        DOMElements::Track => "track",
        DOMElements::Tt => "tt",
        DOMElements::U => "u",
        DOMElements::Ul => "ul",
        DOMElements::Var => "var",
        DOMElements::Video => "video",
        DOMElements::Wbr => "wbr",
        DOMElements::Xmp => "xmp",
    }
}

pub fn entries() -> HashMap<&'static str, bool> {
    DOM_ELEMENTS.entries().map(|(k, v)| (*k, *v)).collect()
}

pub fn entries_with_enum_dom_elements() -> HashMap<DOMElements, bool> {
    DOM_ELEMENTS
        .entries()
        .map(|(k, v)| (map_raw_dom_element_to_enum(k), *v))
        .collect()
}

pub fn for_each(mut callback: impl FnMut(&'static str, bool)) {
    DOM_ELEMENTS.into_iter().for_each(|(k, v)| callback(k, *v));
}

pub fn for_each_with_enum_dom_elements(mut callback: impl FnMut(DOMElements, bool)) {
    DOM_ELEMENTS
        .into_iter()
        .for_each(|(k, v)| callback(map_raw_dom_element_to_enum(k), *v));
}

pub fn get(name: &str) -> Option<bool> {
    DOM_ELEMENTS.get(name).copied()
}

pub fn get_with_enum_dom_elements(dom_element: DOMElements) -> Option<bool> {
    let raw_dom_element = map_enum_dom_element_to_raw(&dom_element);
    DOM_ELEMENTS.get(raw_dom_element).copied()
}

pub fn has(name: &str) -> bool {
    DOM_ELEMENTS.contains_key(name)
}

pub fn keys() -> impl Iterator<Item = &'static str> {
    DOM_ELEMENTS.keys().copied()
}

pub fn values() -> impl Iterator<Item = bool> {
    DOM_ELEMENTS.values().copied()
}

#[cfg(test)]
mod test {
    use crate::dom;
    use insta::{assert_json_snapshot, Settings};

    #[test]
    fn snapshot_for_entries() {
        let dom_entries = dom::entries();

        let mut settings = Settings::clone_current();
        settings.set_sort_maps(true);
        settings.bind(|| {
            assert_json_snapshot!(dom_entries);
        });
    }

    #[test]
    fn snapshot_for_entries_with_enum_dom_elements() {
        let dom_entries = dom::entries_with_enum_dom_elements();

        let mut settings = Settings::clone_current();
        settings.set_sort_maps(true);
        settings.bind(|| {
            assert_json_snapshot!(dom_entries);
        });
    }

    #[test]
    fn test_for_each() {
        let mut el_count = 0;
        dom::for_each(|_, _| el_count += 1);
        assert_eq!(el_count, 129);
        let mut reserved_count = 0;
        dom::for_each(|_, v| {
            if v {
                reserved_count += 1
            }
        });
        assert_eq!(reserved_count, 16);
        let mut elements_list = Vec::new();
        dom::for_each(|k, _| {
            elements_list.push(k);
        });
        assert_eq!(elements_list, dom::keys().collect::<Vec<_>>());
    }

    #[test]
    fn test_for_each_with_enum_dom_elements() {
        let mut el_count = 0;
        dom::for_each_with_enum_dom_elements(|_, _| el_count += 1);
        assert_eq!(el_count, 129);
        let mut reserved_count = 0;
        dom::for_each_with_enum_dom_elements(|_, v| {
            if v {
                reserved_count += 1
            }
        });
        assert_eq!(reserved_count, 16);
        let mut elements_list = Vec::new();
        dom::for_each_with_enum_dom_elements(|k, _| {
            elements_list.push(k);
        });
        assert_eq!(
            elements_list,
            dom::keys()
                .map(dom::map_raw_dom_element_to_enum)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_get() {
        assert_eq!(dom::get("a"), Some(false));
        assert_eq!(dom::get("html"), Some(true));
        assert_eq!(dom::get("unknown"), None);
    }

    #[test]
    fn test_get_with_enum_dom_elements() {
        assert_eq!(
            dom::get_with_enum_dom_elements(dom::DOMElements::A),
            Some(false)
        );
        assert_eq!(
            dom::get_with_enum_dom_elements(dom::DOMElements::Html),
            Some(true)
        );
    }

    #[test]
    fn test_has() {
        assert!(dom::has("a"));
        assert!(dom::has("html"));
        assert!(!dom::has("unknown"));
    }

    #[test]
    fn test_keys() {
        let keys = dom::keys().collect::<Vec<_>>();
        for key in keys {
            assert!(dom::entries().contains_key(key));
        }
    }
}
