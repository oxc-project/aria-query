use crate::definition::{
    ARIAAbstractRole, ARIADocumentStructureRole, ARIANameFromSources, ARIARole, ARIARoleDefinition,
    ARIARoleDefinitionKey, ARIARoleRelation, ARIARoleRelationConcept, RoleDefinitions,
};
use phf::phf_ordered_map;

pub static ARIA_ABSTRACT_ROLES: RoleDefinitions = phf_ordered_map! {
    "command" => &ARIARoleDefinition {
        is_abstract: true,
        accessible_name_required: false,
        base_concepts: None,
        children_presentational: false,
        name_from: Some(&[&ARIANameFromSources::Author]),
        prohibited_props: None,
        props: None,
        related_concepts: None,
        require_context_role: None,
        required_context_role: None,
        required_owned_elements: None,
        required_props: None,
        super_class: Some(&[&[&ARIARoleDefinitionKey::ARIAAbstractRole(ARIAAbstractRole::RoleType), &ARIARoleDefinitionKey::ARIAAbstractRole(ARIAAbstractRole::Widget)]]),
    },
    "composite" => &ARIARoleDefinition {
        is_abstract: true,
        accessible_name_required: false,
        base_concepts: None,
        children_presentational: false,
        name_from: Some(&[&ARIANameFromSources::Author]),
        prohibited_props: None,
        props: Some(&phf_ordered_map! {
            "aria-activedescendant" => &None,
            "aria-disabled" => &None,
        }),
        related_concepts: None,
        require_context_role: None,
        required_context_role: None,
        required_owned_elements: None,
        required_props: None,
        super_class: Some(&[&[&ARIARoleDefinitionKey::ARIAAbstractRole(ARIAAbstractRole::RoleType), &ARIARoleDefinitionKey::ARIAAbstractRole(ARIAAbstractRole::Widget)]])
    },
    "input" => &ARIARoleDefinition {
        is_abstract: true,
        accessible_name_required: false,
        base_concepts: None,
        children_presentational: false,
        name_from: Some(&[&ARIANameFromSources::Author]),
        prohibited_props: None,
        props: Some(&phf_ordered_map! {
            "aria-disabled" => &None,
        }),
        related_concepts: Some(&[
            ARIARoleRelation {
                concept: Some(ARIARoleRelationConcept {
                    name: "input",
                    attributes: None,
                    constraints: None
                }),
                module: Some("XForms"),
            }
        ]),
        require_context_role: None,
        required_context_role: None,
        required_owned_elements: None,
        required_props: None,
        super_class: Some(&[&[&ARIARoleDefinitionKey::ARIAAbstractRole(ARIAAbstractRole::RoleType), &ARIARoleDefinitionKey::ARIAAbstractRole(ARIAAbstractRole::Widget)]])
    },
    "landmark" => &ARIARoleDefinition {
        is_abstract: true,
        accessible_name_required: false,
        base_concepts: None,
        children_presentational: false,
        name_from: Some(&[&ARIANameFromSources::Author]),
        prohibited_props: None,
        props: None,
        related_concepts: None,
        require_context_role: None,
        required_context_role: None,
        required_owned_elements: None,
        required_props: None,
        super_class: Some(&[&[&ARIARoleDefinitionKey::ARIAAbstractRole(ARIAAbstractRole::RoleType), &ARIARoleDefinitionKey::ARIAAbstractRole(ARIAAbstractRole::Structure), &ARIARoleDefinitionKey::ARIAAbstractRole(ARIAAbstractRole::Section)]])
    },
    "range" => &ARIARoleDefinition {
        is_abstract: true,
        accessible_name_required: false,
        base_concepts: None,
        children_presentational: false,
        name_from: Some(&[&ARIANameFromSources::Author]),
        prohibited_props: None,
        props: Some(&phf_ordered_map! {
            "aria-valuemax" => &None,
            "aria-valuemin" => &None,
            "aria-valuenow" => &None,
        }),
        related_concepts: None,
        require_context_role: None,
        required_context_role: None,
        required_owned_elements: None,
        required_props: None,
        super_class: Some(&[&[&ARIARoleDefinitionKey::ARIAAbstractRole(ARIAAbstractRole::RoleType), &ARIARoleDefinitionKey::ARIAAbstractRole(ARIAAbstractRole::Structure)]])
    },
    "roletype" => &ARIARoleDefinition {
        is_abstract: true,
        accessible_name_required: false,
        base_concepts: None,
        children_presentational: false,
        name_from: None,
        prohibited_props: None,
        props: Some(&phf_ordered_map! {
            "aria-atomic" => &None,
            "aria-busy" => &None,
            "aria-controls" => &None,
            "aria-describedby" => &None,
            "aria-details" => &None,
            "aria-dropeffect" => &None,
            "aria-flowto" => &None,
            "aria-grabbed" => &None,
            "aria-hidden" => &None,
            "aria-keyshortcuts" => &None,
            "aria-label" => &None,
            "aria-labelledby" => &None,
            "aria-live" => &None,
            "aria-owns" => &None,
            "aria-relevant" => &None,
            "aria-roledescription" => &None,
        }),
        related_concepts: Some(&[
            ARIARoleRelation {
                concept: Some(ARIARoleRelationConcept {
                    name: "role",
                    attributes: None,
                    constraints: None
                }),
                module: Some("XHTML"),
            },
            ARIARoleRelation {
                concept: Some(ARIARoleRelationConcept {
                    name: "type",
                    attributes: None,
                    constraints: None
                }),
                module: Some("Dublin Core"),
            }
        ]),
        require_context_role: None,
        required_context_role: None,
        required_owned_elements: None,
        required_props: None,
        super_class: None
    },
    "section" => &ARIARoleDefinition {
        is_abstract: true,
        accessible_name_required: false,
        base_concepts: None,
        children_presentational: false,
        name_from: None,
        prohibited_props: None,
        props: None,
        related_concepts: Some(&[
            ARIARoleRelation {
                concept: Some(ARIARoleRelationConcept {
                    name: "frontmatter",
                    attributes: None,
                    constraints: None
                }),
                module: Some("DTB"),
            },
            ARIARoleRelation {
                concept: Some(ARIARoleRelationConcept {
                    name: "level",
                    attributes: None,
                    constraints: None
                }),
                module: Some("DTB"),
            },
            ARIARoleRelation {
                concept: Some(ARIARoleRelationConcept {
                    name: "level",
                    attributes: None,
                    constraints: None
                }),
                module: Some("SMIL"),
            },
        ]),
        require_context_role: None,
        required_context_role: None,
        required_owned_elements: None,
        required_props: None,
        super_class: Some(&[&[&ARIARoleDefinitionKey::ARIAAbstractRole(ARIAAbstractRole::RoleType), &ARIARoleDefinitionKey::ARIAAbstractRole(ARIAAbstractRole::Structure)]])
    },
    "sectionhead" => &ARIARoleDefinition {
        is_abstract: true,
        accessible_name_required: false,
        base_concepts: None,
        children_presentational: false,
        name_from: Some(&[&ARIANameFromSources::Author, &ARIANameFromSources::Contents]),
        prohibited_props: None,
        props: None,
        related_concepts: None,
        require_context_role: None,
        required_context_role: None,
        required_owned_elements: None,
        required_props: None,
        super_class: Some(&[&[&ARIARoleDefinitionKey::ARIAAbstractRole(ARIAAbstractRole::RoleType), &ARIARoleDefinitionKey::ARIAAbstractRole(ARIAAbstractRole::Structure)]])
    },
    "select" => &ARIARoleDefinition {
        is_abstract: true,
        accessible_name_required: false,
        base_concepts: None,
        children_presentational: false,
        name_from: Some(&[&ARIANameFromSources::Author]),
        prohibited_props: None,
        props: Some(&phf_ordered_map! {
            "aria-orientation" => &None,
        }),
        related_concepts: None,
        require_context_role: None,
        required_context_role: None,
        required_owned_elements: None,
        required_props: None,
        super_class: Some(&[&[&ARIARoleDefinitionKey::ARIAAbstractRole(ARIAAbstractRole::RoleType), &ARIARoleDefinitionKey::ARIAAbstractRole(ARIAAbstractRole::Widget), &ARIARoleDefinitionKey::ARIAAbstractRole(ARIAAbstractRole::Composite)],
        &[&ARIARoleDefinitionKey::ARIAAbstractRole(ARIAAbstractRole::RoleType), &ARIARoleDefinitionKey::ARIAAbstractRole(ARIAAbstractRole::Structure), &ARIARoleDefinitionKey::ARIAAbstractRole(ARIAAbstractRole::Section), &ARIARoleDefinitionKey::ARIARole(ARIARole::ARIADocumentStructureRole(ARIADocumentStructureRole::Group))]])
    },
    "structure" => &ARIARoleDefinition {
        is_abstract: true,
        accessible_name_required: false,
        base_concepts: None,
        children_presentational: false,
        name_from: None,
        prohibited_props: None,
        props: None,
        related_concepts: None,
        require_context_role: None,
        required_context_role: None,
        required_owned_elements: None,
        required_props: None,
        super_class: Some(&[&[&ARIARoleDefinitionKey::ARIAAbstractRole(ARIAAbstractRole::RoleType)]])
    },
    "widget" => &ARIARoleDefinition {
        is_abstract: true,
        accessible_name_required: false,
        base_concepts: None,
        children_presentational: false,
        name_from: None,
        prohibited_props: None,
        props: None,
        related_concepts: None,
        require_context_role: None,
        required_context_role: None,
        required_owned_elements: None,
        required_props: None,
        super_class: Some(&[&[&ARIARoleDefinitionKey::ARIAAbstractRole(ARIAAbstractRole::RoleType)]])
    },
    "window" => &ARIARoleDefinition {
        is_abstract: true,
        accessible_name_required: false,
        base_concepts: None,
        children_presentational: false,
        name_from: Some(&[&ARIANameFromSources::Author]),
        prohibited_props: None,
        props: Some(&phf_ordered_map! {
            "aria-modal" => &None,
        }),
        related_concepts: None,
        require_context_role: None,
        required_context_role: None,
        required_owned_elements: None,
        required_props: None,
        super_class: Some(&[&[&ARIARoleDefinitionKey::ARIAAbstractRole(ARIAAbstractRole::RoleType)]])
    },
};
