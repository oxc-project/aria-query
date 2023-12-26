pub mod dom {
    use std::collections::HashMap;
    use std::sync::OnceLock;

    static DOM_ELEMENTS: OnceLock<HashMap<&'static str, bool>> = OnceLock::new();

    fn init_dom_elements_map() -> &'static HashMap<&'static str, bool> {
        let map = DOM_ELEMENTS.get_or_init(|| {
            let mut m = HashMap::new();
            m.insert("a", false);
            m.insert("abbr", false);
            m.insert("acronym", false);
            m.insert("address", false);
            m.insert("applet", false);
            m.insert("area", false);
            m.insert("article", false);
            m.insert("aside", false);
            m.insert("audio", false);
            m.insert("b", false);
            m.insert("base", true);
            m.insert("bdi", false);
            m.insert("bdo", false);
            m.insert("big", false);
            m.insert("blink", false);
            m.insert("blockquote", false);
            m.insert("body", false);
            m.insert("br", false);
            m.insert("button", false);
            m.insert("canvas", false);
            m.insert("caption", false);
            m.insert("center", false);
            m.insert("cite", false);
            m.insert("code", false);
            m.insert("col", true);
            m.insert("colgroup", true);
            m.insert("content", false);
            m.insert("data", false);
            m.insert("datalist", false);
            m.insert("dd", false);
            m.insert("del", false);
            m.insert("details", false);
            m.insert("dfn", false);
            m.insert("dialog", false);
            m.insert("dir", false);
            m.insert("div", false);
            m.insert("dl", false);
            m.insert("dt", false);
            m.insert("em", false);
            m.insert("embed", false);
            m.insert("fieldset", false);
            m.insert("figcaption", false);
            m.insert("figure", false);
            m.insert("font", false);
            m.insert("footer", false);
            m.insert("form", false);
            m.insert("frame", false);
            m.insert("frameset", false);
            m.insert("h1", false);
            m.insert("h2", false);
            m.insert("h3", false);
            m.insert("h4", false);
            m.insert("h5", false);
            m.insert("h6", false);
            m.insert("head", true);
            m.insert("header", false);
            m.insert("hgroup", false);
            m.insert("hr", false);
            m.insert("html", true);
            m.insert("i", false);
            m.insert("iframe", false);
            m.insert("img", false);
            m.insert("input", false);
            m.insert("ins", false);
            m.insert("kbd", false);
            m.insert("keygen", false);
            m.insert("label", false);
            m.insert("legend", false);
            m.insert("li", false);
            m.insert("link", true);
            m.insert("main", false);
            m.insert("map", false);
            m.insert("mark", false);
            m.insert("marquee", false);
            m.insert("menu", false);
            m.insert("menuitem", false);
            m.insert("meta", true);
            m.insert("meter", false);
            m.insert("nav", false);
            m.insert("noembed", true);
            m.insert("noscript", true);
            m.insert("object", false);
            m.insert("ol", false);
            m.insert("optgroup", false);
            m.insert("option", false);
            m.insert("output", false);
            m.insert("p", false);
            m.insert("param", true);
            m.insert("picture", true);
            m.insert("pre", false);
            m.insert("progress", false);
            m.insert("q", false);
            m.insert("rp", false);
            m.insert("rt", false);
            m.insert("rtc", false);
            m.insert("ruby", false);
            m.insert("s", false);
            m.insert("samp", false);
            m.insert("script", true);
            m.insert("section", false);
            m.insert("select", false);
            m.insert("small", false);
            m.insert("source", true);
            m.insert("spacer", false);
            m.insert("span", false);
            m.insert("strike", false);
            m.insert("strong", false);
            m.insert("style", true);
            m.insert("sub", false);
            m.insert("summary", false);
            m.insert("sup", false);
            m.insert("table", false);
            m.insert("tbody", false);
            m.insert("td", false);
            m.insert("textarea", false);
            m.insert("tfoot", false);
            m.insert("th", false);
            m.insert("thead", false);
            m.insert("time", false);
            m.insert("title", true);
            m.insert("tr", false);
            m.insert("track", true);
            m.insert("tt", false);
            m.insert("u", false);
            m.insert("ul", false);
            m.insert("var", false);
            m.insert("video", false);
            m.insert("wbr", false);
            m.insert("xmp", false);
            m
        });

        map
    }

    pub fn entries() -> &'static HashMap<&'static str, bool> {
        init_dom_elements_map()
    }

    pub fn for_each(mut callback: impl FnMut(&'static str, bool)) {
        init_dom_elements_map()
            .iter()
            .for_each(|(k, v)| callback(k, *v));
    }

    pub fn get(name: &str) -> Option<bool> {
        init_dom_elements_map().get(name).copied()
    }

    pub fn has(name: &str) -> bool {
        init_dom_elements_map().contains_key(name)
    }

    pub fn keys() -> impl Iterator<Item = &'static str> {
        init_dom_elements_map().keys().copied()
    }

    pub fn values() -> impl Iterator<Item = bool> {
        init_dom_elements_map().values().copied()
    }
}

#[cfg(test)]
mod test {
    use crate::dom::entries;
    use insta::{assert_json_snapshot, Settings};

    #[test]
    fn snapshot_for_entries() {
        let dom_entries = entries();

        let mut settings = Settings::clone_current();
        settings.set_sort_maps(true);
        settings.bind(|| {
            assert_json_snapshot!(dom_entries);
        });
    }

    #[test]
    fn test_for_each() {
        let mut el_count = 0;
        crate::dom::for_each(|_, _| el_count += 1);
        assert_eq!(el_count, 129);
        let mut reserved_count = 0;
        crate::dom::for_each(|_, v| {
            if v {
                reserved_count += 1
            }
        });
        assert_eq!(reserved_count, 16);
        let mut elements_list = Vec::new();
        crate::dom::for_each(|k, _| {
            elements_list.push(k);
        });
        assert_eq!(elements_list, crate::dom::keys().collect::<Vec<_>>());
    }

    #[test]
    fn test_get() {
        assert_eq!(crate::dom::get("a"), Some(false));
        assert_eq!(crate::dom::get("html"), Some(true));
        assert_eq!(crate::dom::get("unknown"), None);
    }

    #[test]
    fn test_has() {
        assert!(crate::dom::has("a"));
        assert!(crate::dom::has("html"));
        assert!(!crate::dom::has("unknown"));
    }

    #[test]
    fn test_keys() {
        let keys = crate::dom::keys().collect::<Vec<_>>();
        for (_, key) in keys.iter().enumerate() {
            assert!(crate::dom::entries().contains_key(key));
        }
    }
}
