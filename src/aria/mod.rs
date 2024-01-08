use crate::definition::{
    ARIAProperty, ARIAPropertyDefinition, ARIAPropertyDefinitionType, ARIAState,
};
use phf::{phf_ordered_map, OrderedMap};
use std::collections::HashMap;

static ARIA_PROPS_MAP: OrderedMap<&'static str, &'static ARIAPropertyDefinition> = phf_ordered_map! {
    "aria-activedescendant" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaActivedescendant,
        type_: ARIAPropertyDefinitionType::Id,
        values: None,
        allow_undefined: None,
    },
    "aria-atomic" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaAtomic,
        type_: ARIAPropertyDefinitionType::Boolean,
        values: None,
        allow_undefined: None,
    },
    "aria-autocomplete" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaAutocomplete,
        type_: ARIAPropertyDefinitionType::Token,
        values: Some(&["inline", "list", "both", "none"]),
        allow_undefined: None,
    },
    "aria-braillelabel" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaBraillelabel,
        type_: ARIAPropertyDefinitionType::String,
        values: None,
        allow_undefined: None,
    },
    "aria-brailleroledescription" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaBrailleroledescription,
        type_: ARIAPropertyDefinitionType::String,
        values: None,
        allow_undefined: None,
    },
    "aria-busy" => &ARIAPropertyDefinition {
        key: ARIAProperty::ARIAState(ARIAState::AriaBusy),
        type_: ARIAPropertyDefinitionType::Boolean,
        values: None,
        allow_undefined: None,
    },
    "aria-checked" => &ARIAPropertyDefinition {
        key: ARIAProperty::ARIAState(ARIAState::AriaChecked),
        type_: ARIAPropertyDefinitionType::Tristate,
        values: None,
        allow_undefined: None,
    },
    "aria-colcount" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaColcount,
        type_: ARIAPropertyDefinitionType::Integer,
        values: None,
        allow_undefined: None,
    },
    "aria-colspan" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaColspan,
        type_: ARIAPropertyDefinitionType::Integer,
        values: None,
        allow_undefined: None,
    },
    "aria-controls" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaControls,
        type_: ARIAPropertyDefinitionType::IdList,
        values: None,
        allow_undefined: None,
    },
    "aria-current" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaCurrent,
        type_: ARIAPropertyDefinitionType::Token,
        values: Some(&["page", "step", "location", "date", "time", "true", "false"]),
        allow_undefined: None,
    },
    "aria-describedby" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaDescribedby,
        type_: ARIAPropertyDefinitionType::IdList,
        values: None,
        allow_undefined: None,
    },
    "aria-description" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaDescription,
        type_: ARIAPropertyDefinitionType::String,
        values: None,
        allow_undefined: None,
    },
    "aria-details" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaDetails,
        type_: ARIAPropertyDefinitionType::Id,
        values: None,
        allow_undefined: None,
    },
    "aria-disabled" => &ARIAPropertyDefinition {
        key: ARIAProperty::ARIAState(ARIAState::AriaDisabled),
        type_: ARIAPropertyDefinitionType::Boolean,
        values: None,
        allow_undefined: None,
    },
    "aria-dropeffect" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaDropeffect,
        type_: ARIAPropertyDefinitionType::TokenList,
        values: Some(&["copy", "execute", "link", "move", "none", "popup"]),
        allow_undefined: None,
    },
    "aria-errormessage" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaErrormessage,
        type_: ARIAPropertyDefinitionType::Id,
        values: None,
        allow_undefined: None,
    },
    "aria-expanded" => &ARIAPropertyDefinition {
        key: ARIAProperty::ARIAState(ARIAState::AriaExpanded),
        type_: ARIAPropertyDefinitionType::Boolean,
        values: None,
        allow_undefined: Some(true),
    },
    "aria-flowto" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaFlowto,
        type_: ARIAPropertyDefinitionType::IdList,
        values: None,
        allow_undefined: None,
    },
    "aria-grabbed" => &ARIAPropertyDefinition {
        key: ARIAProperty::ARIAState(ARIAState::AriaGrabbed),
        type_: ARIAPropertyDefinitionType::Boolean,
        values: None,
        allow_undefined: Some(true),
    },
    "aria-haspopup" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaHaspopup,
        type_: ARIAPropertyDefinitionType::Token,
        values: Some(&["false", "true", "menu", "listbox", "tree", "grid", "dialog"]),
        allow_undefined: None,
    },
    "aria-hidden" => &ARIAPropertyDefinition {
        key: ARIAProperty::ARIAState(ARIAState::AriaHidden),
        type_: ARIAPropertyDefinitionType::Boolean,
        values: None,
        allow_undefined: Some(true),
    },
    "aria-invalid" => &ARIAPropertyDefinition {
        key: ARIAProperty::ARIAState(ARIAState::AriaInvalid),
        type_: ARIAPropertyDefinitionType::Token,
        values: Some(&["grammar", "false", "spelling", "true"]),
        allow_undefined: None,
    },
    "aria-keyshortcuts" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaKeyshortcuts,
        type_: ARIAPropertyDefinitionType::String,
        values: None,
        allow_undefined: None,
    },
    "aria-label" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaLabel,
        type_: ARIAPropertyDefinitionType::String,
        values: None,
        allow_undefined: None,
    },
    "aria-labelledby" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaLabelledby,
        type_: ARIAPropertyDefinitionType::IdList,
        values: None,
        allow_undefined: None,
    },
    "aria-level" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaLevel,
        type_: ARIAPropertyDefinitionType::Integer,
        values: None,
        allow_undefined: None,
    },
    "aria-live" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaLive,
        type_: ARIAPropertyDefinitionType::Token,
        values: Some(&["off", "polite", "assertive"]),
        allow_undefined: None,
    },
    "aria-modal" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaModal,
        type_: ARIAPropertyDefinitionType::Boolean,
        values: None,
        allow_undefined: None,
    },
    "aria-multiline" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaMultiline,
        type_: ARIAPropertyDefinitionType::Boolean,
        values: None,
        allow_undefined: None,
    },
    "aria-multiselectable" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaMultiselectable,
        type_: ARIAPropertyDefinitionType::Boolean,
        values: None,
        allow_undefined: None,
    },
    "aria-orientation" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaOrientation,
        type_: ARIAPropertyDefinitionType::Token,
        values: Some(&["horizontal", "vertical", "undefined"]),
        allow_undefined: None,
    },
    "aria-owns" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaOwns,
        type_: ARIAPropertyDefinitionType::IdList,
        values: None,
        allow_undefined: None,
    },
    "aria-placeholder" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaPlaceholder,
        type_: ARIAPropertyDefinitionType::String,
        values: None,
        allow_undefined: None,
    },
    "aria-posinset" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaPosinset,
        type_: ARIAPropertyDefinitionType::Integer,
        values: None,
        allow_undefined: None,
    },
    "aria-pressed" => &ARIAPropertyDefinition {
        key: ARIAProperty::ARIAState(ARIAState::AriaPressed),
        type_: ARIAPropertyDefinitionType::Tristate,
        values: None,
        allow_undefined: None,
    },
    "aria-readonly" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaReadonly,
        type_: ARIAPropertyDefinitionType::Boolean,
        values: None,
        allow_undefined: None,
    },
    "aria-relevant" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaRelevant,
        type_: ARIAPropertyDefinitionType::TokenList,
        values: Some(&["additions", "removals", "text", "all"]),
        allow_undefined: None,
    },
    "aria-required" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaRequired,
        type_: ARIAPropertyDefinitionType::Boolean,
        values: None,
        allow_undefined: None,
    },
    "aria-roledescription" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaRoledescription,
        type_: ARIAPropertyDefinitionType::String,
        values: None,
        allow_undefined: None,
    },
    "aria-rowcount" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaRowcount,
        type_: ARIAPropertyDefinitionType::Integer,
        values: None,
        allow_undefined: None,
    },
    "aria-rowindex" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaRowindex,
        type_: ARIAPropertyDefinitionType::Integer,
        values: None,
        allow_undefined: None,
    },
    "aria-rowspan" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaRowspan,
        type_: ARIAPropertyDefinitionType::Integer,
        values: None,
        allow_undefined: None,
    },
    "aria-selected" => &ARIAPropertyDefinition {
        key: ARIAProperty::ARIAState(ARIAState::AriaSelected),
        type_: ARIAPropertyDefinitionType::Boolean,
        values: None,
        allow_undefined: Some(true),
    },
    "aria-setsize" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaSetsize,
        type_: ARIAPropertyDefinitionType::Integer,
        values: None,
        allow_undefined: None,
    },
    "aria-sort" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaSort,
        type_: ARIAPropertyDefinitionType::Token,
        values: Some(&["ascending", "descending", "none", "other"]),
        allow_undefined: None,
    },
    "aria-valuemax" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaValuemax,
        type_: ARIAPropertyDefinitionType::Number,
        values: None,
        allow_undefined: None,
    },
    "aria-valuemin" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaValuemin,
        type_: ARIAPropertyDefinitionType::Number,
        values: None,
        allow_undefined: None,
    },
    "aria-valuenow" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaValuenow,
        type_: ARIAPropertyDefinitionType::Number,
        values: None,
        allow_undefined: None,
    },
    "aria-valuetext" => &ARIAPropertyDefinition {
        key: ARIAProperty::AriaValuetext,
        type_: ARIAPropertyDefinitionType::String,
        values: None,
        allow_undefined: None,
    },
};

pub fn entries() -> HashMap<&'static str, &'static ARIAPropertyDefinition> {
    ARIA_PROPS_MAP.entries().map(|(k, v)| (*k, *v)).collect()
}

pub fn for_each(mut callback: impl FnMut(&'static str, &'static ARIAPropertyDefinition)) {
    ARIA_PROPS_MAP.into_iter().for_each(|(k, v)| callback(k, v));
}

pub fn get(name: &str) -> Option<&'static ARIAPropertyDefinition> {
    match ARIA_PROPS_MAP.get(name) {
        Some(v) => Some(v),
        None => None,
    }
}

pub fn has(name: &str) -> bool {
    ARIA_PROPS_MAP.contains_key(name)
}

pub fn keys() -> impl Iterator<Item = &'static str> {
    ARIA_PROPS_MAP.keys().copied()
}

pub fn values() -> impl Iterator<Item = &'static ARIAPropertyDefinition> {
    ARIA_PROPS_MAP.values().copied()
}

#[cfg(test)]
mod test {
    use crate::aria;
    use insta::{assert_json_snapshot, Settings};

    #[test]
    fn snapshot_for_entries() {
        let aria_entries = aria::entries();

        let mut settings = Settings::clone_current();
        settings.set_sort_maps(true);
        settings.bind(|| {
            assert_json_snapshot!(aria_entries);
        });
    }

    #[test]
    fn test_for_each() {
        let mut el_count = 0;
        aria::for_each(|_, _| el_count += 1);
        assert_eq!(el_count, 50);
        let mut elements_list = Vec::new();
        aria::for_each(|k, _| {
            elements_list.push(k);
        });
        assert_eq!(elements_list, aria::keys().collect::<Vec<_>>());
    }

    #[test]
    fn test_get() {
        assert!(aria::get("aria-activedescendant").is_some());
        assert!(aria::get("aria-unknown").is_none());
    }

    #[test]
    fn test_has() {
        assert!(aria::has("aria-activedescendant"));
        assert!(!aria::has("aria-unknown"));
    }

    #[test]
    fn test_keys() {
        let keys = aria::keys().collect::<Vec<_>>();
        for (_, key) in keys.iter().enumerate() {
            assert!(aria::entries().contains_key(key));
        }
    }
}
