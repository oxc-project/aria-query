mod aria_abstract_roles;
use crate::definition::ARIARoleDefinition;
use std::collections::HashMap;

pub fn entries() -> HashMap<&'static str, &'static ARIARoleDefinition> {
    aria_abstract_roles::ARIA_ABSTRACT_ROLES
        .entries()
        .map(|(k, v)| (*k, *v))
        .collect()
}

pub fn for_each(mut callback: impl FnMut(&'static str, &'static ARIARoleDefinition)) {
    aria_abstract_roles::ARIA_ABSTRACT_ROLES
        .into_iter()
        .for_each(|(k, v)| callback(k, v));
}

pub fn keys() -> impl Iterator<Item = &'static str> {
    aria_abstract_roles::ARIA_ABSTRACT_ROLES.keys().copied()
}

pub fn values() -> impl Iterator<Item = &'static ARIARoleDefinition> {
    aria_abstract_roles::ARIA_ABSTRACT_ROLES.values().copied()
}

#[cfg(test)]
mod test {
    use crate::definition::ARIARoleDefinition;
    use crate::role;
    use insta::{assert_json_snapshot, Settings};

    #[test]
    fn snapshot_for_entries() {
        let roles_entries = role::entries();

        let mut settings = Settings::clone_current();
        settings.set_sort_maps(true);
        settings.bind(|| {
            assert_json_snapshot!(roles_entries);
        });
    }

    #[test]
    fn test_for_each() {
        let mut el_count = 0;
        role::for_each(|_, _| el_count += 1);
        assert_eq!(el_count, 12);
        let mut elements_list = Vec::new();
        role::for_each(|k, _| {
            elements_list.push(k);
        });
        assert_eq!(elements_list, role::keys().collect::<Vec<_>>());
    }

    #[test]
    fn test_keys() {
        let keys = role::keys().collect::<Vec<_>>();
        for (_, key) in keys.iter().enumerate() {
            assert!(role::entries().contains_key(key));
        }
    }

    #[test]
    fn snapshot_values() {
        let roles_values = role::values();
        let roles_values = roles_values.collect::<Vec<&'static ARIARoleDefinition>>();
        let mut settings = Settings::clone_current();
        settings.set_sort_maps(true);
        settings.bind(|| {
            assert_json_snapshot!(roles_values);
        });
    }
}
