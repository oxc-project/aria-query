mod aria_abstract_roles;
use crate::definition::ARIARoleDefinition;
use std::collections::HashMap;

pub fn entries() -> HashMap<&'static str, &'static ARIARoleDefinition> {
    aria_abstract_roles::ARIA_ABSTRACT_ROLES
        .entries()
        .map(|(k, v)| (*k, *v))
        .collect()
}

pub fn keys() -> impl Iterator<Item = &'static str> {
    aria_abstract_roles::ARIA_ABSTRACT_ROLES.keys().copied()
}

#[cfg(test)]
mod test {
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
    fn test_keys() {
        let keys = role::keys().collect::<Vec<_>>();
        for (_, key) in keys.iter().enumerate() {
            assert!(role::entries().contains_key(key));
        }
    }
}
