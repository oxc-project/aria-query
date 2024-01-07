use phf::{phf_ordered_map, OrderedMap};
use std::collections::HashMap;

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

pub fn entries() -> HashMap<&'static str, bool> {
    DOM_ELEMENTS.entries().map(|(k, v)| (*k, *v)).collect()
}

pub fn for_each(mut callback: impl FnMut(&'static str, bool)) {
    DOM_ELEMENTS.into_iter().for_each(|(k, v)| callback(k, *v));
}

pub fn get(name: &str) -> Option<bool> {
    DOM_ELEMENTS.get(name).copied()
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
    fn test_get() {
        assert_eq!(dom::get("a"), Some(false));
        assert_eq!(dom::get("html"), Some(true));
        assert_eq!(dom::get("unknown"), None);
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
        for (_, key) in keys.iter().enumerate() {
            assert!(dom::entries().contains_key(key));
        }
    }
}
