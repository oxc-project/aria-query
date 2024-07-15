mod aria_abstract_roles;
use crate::definition::ARIARoleDefinition;
use std::collections::HashMap;

pub fn entries() -> HashMap<&'static str, &'static ARIARoleDefinition> {
    aria_abstract_roles::ARIA_ABSTRACT_ROLES
        .entries()
        .map(|(k, v)| (*k, *v))
        .collect()
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
}
