use std::{collections::HashMap, hash::RandomState, sync::LazyLock};

use ordered_hash_map::OrderedHashMap;

use crate::types::{
    AriaAbstractRole, AriaNameFromSource, AriaProperty, AriaRole, AriaRoleDefinition,
    AriaRoleDefinitionKey, AriaRoleDefinitionSuperClass, AriaRoleRelation, AriaRoleRelationConcept,
};

pub static ARIA_GRAPHICS_ROLES: LazyLock<
    OrderedHashMap<AriaRoleDefinitionKey, AriaRoleDefinition, RandomState>,
> = LazyLock::new(|| {
    OrderedHashMap::from_iter([
        (
            AriaRoleDefinitionKey::GraphicsDocument,
            AriaRoleDefinition {
                r#abstract: false,
                accessible_name_required: true,
                base_concepts: vec![],
                children_presentational: false,
                name_from: vec![AriaNameFromSource::Author],
                prohibited_props: vec![],
                props: HashMap::from([
                    (AriaProperty::AriaDisabled, None),
                    (AriaProperty::AriaErrormessage, None),
                    (AriaProperty::AriaExpanded, None),
                    (AriaProperty::AriaHaspopup, None),
                    (AriaProperty::AriaInvalid, None),
                ]),
                related_concepts: vec![
                    AriaRoleRelation {
                        module: Some("GRAPHICS".into()),
                        concept: Some(AriaRoleRelationConcept {
                            name: "graphics-object".into(),
                            attributes: None,
                            constraints: None,
                        }),
                    },
                    AriaRoleRelation {
                        module: Some("ARIA".into()),
                        concept: Some(AriaRoleRelationConcept {
                            name: "img".into(),
                            attributes: None,
                            constraints: None,
                        }),
                    },
                    AriaRoleRelation {
                        module: Some("ARIA".into()),
                        concept: Some(AriaRoleRelationConcept {
                            name: "article".into(),
                            attributes: None,
                            constraints: None,
                        }),
                    },
                ],
                require_context_role: vec![],
                required_context_role: vec![],
                required_owned_elements: vec![],
                required_props: HashMap::new(),
                super_class: vec![vec![
                    AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                    AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                    AriaRoleDefinitionSuperClass::Role(AriaRole::Document),
                ]],
            },
        ),
        (
            AriaRoleDefinitionKey::GraphicsObject,
            AriaRoleDefinition {
                r#abstract: false,
                accessible_name_required: false,
                base_concepts: vec![],
                children_presentational: false,
                name_from: vec![AriaNameFromSource::Author, AriaNameFromSource::Contents],
                prohibited_props: vec![],
                props: HashMap::from([
                    (AriaProperty::AriaErrormessage, None),
                    (AriaProperty::AriaExpanded, None),
                    (AriaProperty::AriaHaspopup, None),
                    (AriaProperty::AriaInvalid, None),
                ]),
                related_concepts: vec![
                    AriaRoleRelation {
                        module: Some("GRAPHICS".into()),
                        concept: Some(AriaRoleRelationConcept {
                            name: "graphics-document".into(),
                            attributes: None,
                            constraints: None,
                        }),
                    },
                    AriaRoleRelation {
                        module: Some("ARIA".into()),
                        concept: Some(AriaRoleRelationConcept {
                            name: "group".into(),
                            attributes: None,
                            constraints: None,
                        }),
                    },
                    AriaRoleRelation {
                        module: Some("ARIA".into()),
                        concept: Some(AriaRoleRelationConcept {
                            name: "img".into(),
                            attributes: None,
                            constraints: None,
                        }),
                    },
                    AriaRoleRelation {
                        module: Some("GRAPHICS".into()),
                        concept: Some(AriaRoleRelationConcept {
                            name: "graphics-symbol".into(),
                            attributes: None,
                            constraints: None,
                        }),
                    },
                ],
                require_context_role: vec![],
                required_context_role: vec![],
                required_owned_elements: vec![],
                required_props: HashMap::new(),
                super_class: vec![vec![
                    AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                    AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                    AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    AriaRoleDefinitionSuperClass::Role(AriaRole::Group),
                ]],
            },
        ),
        (
            AriaRoleDefinitionKey::GraphicsSymbol,
            AriaRoleDefinition {
                r#abstract: false,
                accessible_name_required: true,
                base_concepts: vec![],
                children_presentational: true,
                name_from: vec![AriaNameFromSource::Author],
                prohibited_props: vec![],
                props: HashMap::from([
                    (AriaProperty::AriaDisabled, None),
                    (AriaProperty::AriaErrormessage, None),
                    (AriaProperty::AriaExpanded, None),
                    (AriaProperty::AriaHaspopup, None),
                    (AriaProperty::AriaInvalid, None),
                ]),
                related_concepts: vec![],
                require_context_role: vec![],
                required_context_role: vec![],
                required_owned_elements: vec![],
                required_props: HashMap::new(),
                super_class: vec![vec![
                    AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                    AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                    AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    AriaRoleDefinitionSuperClass::Role(AriaRole::Img),
                ]],
            },
        ),
    ])
});
