use std::{collections::HashMap, hash::RandomState, sync::LazyLock};

use ordered_hash_map::OrderedHashMap;

use crate::types::{
    AriaAbstractRole, AriaNameFromSource, AriaProperty, AriaRole, AriaRoleDefinition,
    AriaRoleDefinitionKey, AriaRoleDefinitionSuperClass, AriaRoleRelation, AriaRoleRelationConcept,
    AriaRoleRelationConceptAttribute, AriaRoleRelationConceptAttributeConstraint,
    AriaRoleRelationConceptConstraint,
};

pub static ARIA_LITERAL_ROLES: LazyLock<
    OrderedHashMap<AriaRoleDefinitionKey, AriaRoleDefinition, RandomState>,
> = LazyLock::new(|| {
    OrderedHashMap::from_iter([
            (
                AriaRoleDefinitionKey::Alertdialog,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "alert".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("XForms".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Alert),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Window),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Dialog),
                        ],
                    ],
                },
            ),
            (
                AriaRoleDefinitionKey::Alert,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaAtomic, Some("true".into())),
                        (AriaProperty::AriaLive, Some("assertive".into())),
                    ]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "alert".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("XForms".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Application,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaActivedescendant, None),
                        (AriaProperty::AriaDisabled, None),
                        (AriaProperty::AriaErrormessage, None),
                        (AriaProperty::AriaExpanded, None),
                        (AriaProperty::AriaHaspopup, None),
                        (AriaProperty::AriaInvalid, None),
                    ]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "Device Independence Delivery Unit".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: None,
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Article,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaPosinset, None),
                        (AriaProperty::AriaSetsize, None),
                    ]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "article".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
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
                AriaRoleDefinitionKey::Banner,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            constraints: Some(vec![
                                AriaRoleRelationConceptConstraint::ScopedToTheBodyElement,
                            ]),
                            name: "header".into(),
                            attributes: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Landmark),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Blockquote,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "blockquote".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Button,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: true,
                    name_from: vec![AriaNameFromSource::Author, AriaNameFromSource::Contents],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaDisabled, None),
                        (AriaProperty::AriaExpanded, None),
                        (AriaProperty::AriaHaspopup, None),
                        (AriaProperty::AriaPressed, None),
                    ]),
                    related_concepts: vec![
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![AriaRoleRelationConceptAttribute {
                                    name: "type".into(),
                                    value: Some("button".into()),
                                    constraints: None,
                                }]),
                                name: "input".into(),
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![AriaRoleRelationConceptAttribute {
                                    name: "type".into(),
                                    value: Some("image".into()),
                                    constraints: None,
                                }]),
                                name: "input".into(),
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![AriaRoleRelationConceptAttribute {
                                    name: "type".into(),
                                    value: Some("reset".into()),
                                    constraints: None,
                                }]),
                                name: "input".into(),
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![AriaRoleRelationConceptAttribute {
                                    name: "type".into(),
                                    value: Some("submit".into()),
                                    constraints: None,
                                }]),
                                name: "input".into(),
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "button".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "trigger".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("XForms".into()),
                        },
                    ],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Command),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Caption,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Prohibited],
                    prohibited_props: vec![AriaProperty::AriaLabel, AriaProperty::AriaLabelledby],
                    props: HashMap::from([]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "caption".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![AriaRole::Figure, AriaRole::Grid, AriaRole::Table],
                    required_context_role: vec![AriaRole::Figure, AriaRole::Grid, AriaRole::Table],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Cell,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author, AriaNameFromSource::Contents],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaColindex, None),
                        (AriaProperty::AriaColspan, None),
                        (AriaProperty::AriaRowindex, None),
                        (AriaProperty::AriaRowspan, None),
                    ]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            constraints: Some(vec![
                                AriaRoleRelationConceptConstraint::AncestorTableElementHasTableRole,
                            ]),
                            name: "td".into(),
                            attributes: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![AriaRole::Row],
                    required_context_role: vec![AriaRole::Row],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Checkbox,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: true,
                    name_from: vec![AriaNameFromSource::Author, AriaNameFromSource::Contents],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaChecked, None),
                        (AriaProperty::AriaErrormessage, None),
                        (AriaProperty::AriaExpanded, None),
                        (AriaProperty::AriaInvalid, None),
                        (AriaProperty::AriaReadonly, None),
                        (AriaProperty::AriaRequired, None),
                    ]),
                    related_concepts: vec![
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![AriaRoleRelationConceptAttribute {
                                    name: "type".into(),
                                    value: Some("checkbox".into()),
                                    constraints: None,
                                }]),
                                name: "input".into(),
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "option".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("ARIA".into()),
                        },
                    ],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::from([(AriaProperty::AriaChecked, None)]),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Input),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Code,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Prohibited],
                    prohibited_props: vec![AriaProperty::AriaLabel, AriaProperty::AriaLabelledby],
                    props: HashMap::from([]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "code".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Columnheader,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author, AriaNameFromSource::Contents],
                    prohibited_props: vec![],
                    props: HashMap::from([(AriaProperty::AriaSort, None)]),
                    related_concepts: vec![
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "th".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![AriaRoleRelationConceptAttribute {
                                    name: "scope".into(),
                                    value: Some("col".into()),
                                    constraints: None,
                                }]),
                                name: "th".into(),
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![AriaRoleRelationConceptAttribute {
                                    name: "scope".into(),
                                    value: Some("colgroup".into()),
                                    constraints: None,
                                }]),
                                name: "th".into(),
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                    ],
                    require_context_role: vec![AriaRole::Row],
                    required_context_role: vec![AriaRole::Row],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Cell),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Cell),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Gridcell),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Gridcell),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                            AriaRoleDefinitionSuperClass::AbstractRole(
                                AriaAbstractRole::Sectionhead,
                            ),
                        ],
                    ],
                },
            ),
            (
                AriaRoleDefinitionKey::Combobox,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaActivedescendant, None),
                        (AriaProperty::AriaAutocomplete, None),
                        (AriaProperty::AriaErrormessage, None),
                        (AriaProperty::AriaInvalid, None),
                        (AriaProperty::AriaReadonly, None),
                        (AriaProperty::AriaRequired, None),
                        (AriaProperty::AriaExpanded, Some("false".into())),
                        (AriaProperty::AriaHaspopup, Some("listbox".into())),
                    ]),
                    related_concepts: vec![
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![
                                    AriaRoleRelationConceptAttribute {
                                        constraints: Some(vec![
                                            AriaRoleRelationConceptAttributeConstraint::Set,
                                        ]),
                                        name: "list".into(),
                                        value: None,
                                    },
                                    AriaRoleRelationConceptAttribute {
                                        name: "type".into(),
                                        value: Some("email".into()),
                                        constraints: None,
                                    },
                                ]),
                                name: "input".into(),
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![
                                    AriaRoleRelationConceptAttribute {
                                        constraints: Some(vec![
                                            AriaRoleRelationConceptAttributeConstraint::Set,
                                        ]),
                                        name: "list".into(),
                                        value: None,
                                    },
                                    AriaRoleRelationConceptAttribute {
                                        name: "type".into(),
                                        value: Some("search".into()),
                                        constraints: None,
                                    },
                                ]),
                                name: "input".into(),
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![
                                    AriaRoleRelationConceptAttribute {
                                        constraints: Some(vec![
                                            AriaRoleRelationConceptAttributeConstraint::Set,
                                        ]),
                                        name: "list".into(),
                                        value: None,
                                    },
                                    AriaRoleRelationConceptAttribute {
                                        name: "type".into(),
                                        value: Some("tel".into()),
                                        constraints: None,
                                    },
                                ]),
                                name: "input".into(),
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![
                                    AriaRoleRelationConceptAttribute {
                                        constraints: Some(vec![
                                            AriaRoleRelationConceptAttributeConstraint::Set,
                                        ]),
                                        name: "list".into(),
                                        value: None,
                                    },
                                    AriaRoleRelationConceptAttribute {
                                        name: "type".into(),
                                        value: Some("text".into()),
                                        constraints: None,
                                    },
                                ]),
                                name: "input".into(),
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![
                                    AriaRoleRelationConceptAttribute {
                                        constraints: Some(vec![
                                            AriaRoleRelationConceptAttributeConstraint::Set,
                                        ]),
                                        name: "list".into(),
                                        value: None,
                                    },
                                    AriaRoleRelationConceptAttribute {
                                        name: "type".into(),
                                        value: Some("url".into()),
                                        constraints: None,
                                    },
                                ]),
                                name: "input".into(),
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![
                                    AriaRoleRelationConceptAttribute {
                                        constraints: Some(vec![
                                            AriaRoleRelationConceptAttributeConstraint::Set,
                                        ]),
                                        name: "list".into(),
                                        value: None,
                                    },
                                    AriaRoleRelationConceptAttribute {
                                        name: "type".into(),
                                        value: Some("url".into()),
                                        constraints: None,
                                    },
                                ]),
                                name: "input".into(),
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![
                                    AriaRoleRelationConceptAttribute {
                                        constraints: Some(vec![
                                            AriaRoleRelationConceptAttributeConstraint::Undefined,
                                        ]),
                                        name: "multiple".into(),
                                        value: None,
                                    },
                                    AriaRoleRelationConceptAttribute {
                                        constraints: Some(vec![
                                            AriaRoleRelationConceptAttributeConstraint::Undefined,
                                        ]),
                                        name: "size".into(),
                                        value: None,
                                    },
                                ]),
                                constraints: Some(vec![
                                    AriaRoleRelationConceptConstraint::TheMultipleAttributeIsNotSetAndTheSizeAttributeDoesNotHaveAValueGreaterThanOne
                                ]),
                                name: "select".into(),
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "select".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("XForms".into()),
                        },
                    ],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::from([
                        (AriaProperty::AriaControls, None),
                        (AriaProperty::AriaExpanded, Some("false".into())),
                    ]),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Input),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Complementary,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                constraints: Some(vec![
                                    AriaRoleRelationConceptConstraint::ScopedToTheBodyElement,
                                    AriaRoleRelationConceptConstraint::ScopedToTheMainElement,
                                ]),
                                name: "aside".into(),
                                attributes: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![AriaRoleRelationConceptAttribute {
                                    constraints: Some(vec![
                                        AriaRoleRelationConceptAttributeConstraint::Set,
                                    ]),
                                    name: "aria-label".into(),
                                    value: None,
                                }]),
                                constraints: Some(vec![
                                    AriaRoleRelationConceptConstraint::ScopedToASectioningContentElement,
                                    AriaRoleRelationConceptConstraint::ScopedToASectioningRootElementOtherThanBody,
                                ]),
                                name: "aside".into(),
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![AriaRoleRelationConceptAttribute {
                                    constraints: Some(vec![
                                        AriaRoleRelationConceptAttributeConstraint::Set,
                                    ]),
                                    name: "aria-labelledby".into(),
                                    value: None,
                                }]),
                                constraints: Some(vec![
                                    AriaRoleRelationConceptConstraint::ScopedToASectioningContentElement,
                                    AriaRoleRelationConceptConstraint::ScopedToASectioningRootElementOtherThanBody,
                                ]),
                                name: "aside".into(),
                            }),
                            module: Some("HTML".into()),
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
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Landmark),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Contentinfo,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            constraints: Some(vec![
                                AriaRoleRelationConceptConstraint::ScopedToTheBodyElement,
                            ]),
                            name: "footer".into(),
                            attributes: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Landmark),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Definition,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "dd".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Deletion,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Prohibited],
                    prohibited_props: vec![AriaProperty::AriaLabel, AriaProperty::AriaLabelledby],
                    props: HashMap::from([]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "del".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Dialog,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "dialog".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Window),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Directory,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![AriaRoleRelation {
                        module: Some("DAISY Guide".into()),
                        concept: None,
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                        AriaRoleDefinitionSuperClass::Role(AriaRole::List),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Document,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "Device Independence Delivery Unit".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: None,
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "html".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                    ],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Emphasis,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Prohibited],
                    prohibited_props: vec![AriaProperty::AriaLabel, AriaProperty::AriaLabelledby],
                    props: HashMap::from([]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "em".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Feed,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![vec!["article".into()]],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                        AriaRoleDefinitionSuperClass::Role(AriaRole::List),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Figure,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "figure".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Form,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![AriaRoleRelationConceptAttribute {
                                    constraints: Some(vec![
                                        AriaRoleRelationConceptAttributeConstraint::Set,
                                    ]),
                                    name: "aria-label".into(),
                                    value: None,
                                }]),
                                name: "form".into(),
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![AriaRoleRelationConceptAttribute {
                                    constraints: Some(vec![
                                        AriaRoleRelationConceptAttributeConstraint::Set,
                                    ]),
                                    name: "aria-labelledby".into(),
                                    value: None,
                                }]),
                                name: "form".into(),
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![AriaRoleRelationConceptAttribute {
                                    constraints: Some(vec![
                                        AriaRoleRelationConceptAttributeConstraint::Set,
                                    ]),
                                    name: "name".into(),
                                    value: None,
                                }]),
                                name: "form".into(),
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
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
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Landmark),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Generic,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Prohibited],
                    prohibited_props: vec![AriaProperty::AriaLabel, AriaProperty::AriaLabelledby],
                    props: HashMap::from([]),
                    related_concepts: vec![
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "a".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "area".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "aside".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "b".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "bdo".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "body".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "data".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "div".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                constraints: Some(vec![
                                    AriaRoleRelationConceptConstraint::ScopedToTheMainElement,
                                    AriaRoleRelationConceptConstraint::ScopedToASectioningContentElement,
                                    AriaRoleRelationConceptConstraint::ScopedToASectioningRootElementOtherThanBody,
                                ]),
                                name: "footer".into(),
                                attributes: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                constraints: Some(vec![
                                    AriaRoleRelationConceptConstraint::ScopedToTheMainElement,
                                    AriaRoleRelationConceptConstraint::ScopedToASectioningContentElement,
                                    AriaRoleRelationConceptConstraint::ScopedToASectioningRootElementOtherThanBody,
                                ]),
                                name: "header".into(),
                                attributes: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "hgroup".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "i".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "pre".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "q".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "samp".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "section".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "small".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "span".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "u".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                    ],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                    ]],
                },
            ),
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
            (
                AriaRoleDefinitionKey::Gridcell,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author, AriaNameFromSource::Contents],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaDisabled, None),
                        (AriaProperty::AriaErrormessage, None),
                        (AriaProperty::AriaExpanded, None),
                        (AriaProperty::AriaHaspopup, None),
                        (AriaProperty::AriaInvalid, None),
                        (AriaProperty::AriaReadonly, None),
                        (AriaProperty::AriaRequired, None),
                        (AriaProperty::AriaSelected, None),
                    ]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            constraints: Some(vec![
            AriaRoleRelationConceptConstraint::AncestorTableElementHasGridRole,
            AriaRoleRelationConceptConstraint::AncestorTableElementHasTreegridRole,
        ]),
                            name: "td".into(),
                            attributes: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![AriaRole::Row],
                    required_context_role: vec![AriaRole::Row],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Cell),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                        ],
                    ],
                },
            ),
            (
                AriaRoleDefinitionKey::Grid,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaMultiselectable, None),
                        (AriaProperty::AriaReadonly, None),
                    ]),
                    related_concepts: vec![],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![
                        vec!["row".into()],
                        vec!["row".into(), "rowgroup".into()],
                    ],
                    required_props: HashMap::new(),
                    super_class: vec![
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Composite),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Table),
                        ],
                    ],
                },
            ),
            (
                AriaRoleDefinitionKey::Group,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaActivedescendant, None),
                        (AriaProperty::AriaDisabled, None),
                    ]),
                    related_concepts: vec![
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "details".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "fieldset".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "optgroup".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "address".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
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
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Heading,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author, AriaNameFromSource::Contents],
                    prohibited_props: vec![],
                    props: HashMap::from([(AriaProperty::AriaLevel, Some("2".into()))]),
                    related_concepts: vec![
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "h1".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "h2".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "h3".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "h4".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "h5".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "h6".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                    ],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::from([(AriaProperty::AriaLevel, Some("2".into()))]),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Sectionhead),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Img,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: true,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![AriaRoleRelationConceptAttribute {
                                    constraints: Some(vec![
                                        AriaRoleRelationConceptAttributeConstraint::Set,
                                    ]),
                                    name: "alt".into(),
                                    value: None,
                                }]),
                                name: "img".into(),
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![AriaRoleRelationConceptAttribute {
                                    constraints: Some(vec![
                                        AriaRoleRelationConceptAttributeConstraint::Undefined,
                                    ]),
                                    name: "alt".into(),
                                    value: None,
                                }]),
                                name: "img".into(),
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "imggroup".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("DTB".into()),
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
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Insertion,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Prohibited],
                    prohibited_props: vec![AriaProperty::AriaLabel, AriaProperty::AriaLabelledby],
                    props: HashMap::from([]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "ins".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Link,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author, AriaNameFromSource::Contents],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaDisabled, None),
                        (AriaProperty::AriaExpanded, None),
                        (AriaProperty::AriaHaspopup, None),
                    ]),
                    related_concepts: vec![
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![AriaRoleRelationConceptAttribute {
                                    constraints: Some(vec![
                                        AriaRoleRelationConceptAttributeConstraint::Set,
                                    ]),
                                    name: "href".into(),
                                    value: None,
                                }]),
                                name: "a".into(),
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![AriaRoleRelationConceptAttribute {
                                    constraints: Some(vec![
                                        AriaRoleRelationConceptAttributeConstraint::Set,
                                    ]),
                                    name: "href".into(),
                                    value: None,
                                }]),
                                name: "area".into(),
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                    ],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Command),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Listbox,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaErrormessage, None),
                        (AriaProperty::AriaExpanded, None),
                        (AriaProperty::AriaInvalid, None),
                        (AriaProperty::AriaMultiselectable, None),
                        (AriaProperty::AriaReadonly, None),
                        (AriaProperty::AriaRequired, None),
                        (AriaProperty::AriaOrientation, Some("vertical".into())),
                    ]),
                    related_concepts: vec![
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![AriaRoleRelationConceptAttribute {
                                    constraints: Some(vec![
                                        AriaRoleRelationConceptAttributeConstraint::GreaterThanOne,
                                    ]),
                                    name: "size".into(),
                                    value: None,
                                }]),
                                constraints: Some(vec![
                    AriaRoleRelationConceptConstraint::TheSizeAttributeValueIsGreaterThanOne
                ]),
                                name: "select".into(),
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![AriaRoleRelationConceptAttribute {
                                    name: "multiple".into(),
                                    value: None,
                                    constraints: None,
                                }]),
                                name: "select".into(),
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "datalist".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "list".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("ARIA".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "select".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("XForms".into()),
                        },
                    ],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![
                        vec!["option".into(), "group".into()],
                        vec!["option".into()],
                    ],
                    required_props: HashMap::new(),
                    super_class: vec![
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Composite),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Select),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Group),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Select),
                        ],
                    ],
                },
            ),
            (
                AriaRoleDefinitionKey::Listitem,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaLevel, None),
                        (AriaProperty::AriaPosinset, None),
                        (AriaProperty::AriaSetsize, None),
                    ]),
                    related_concepts: vec![
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                constraints: Some(vec![
                                    AriaRoleRelationConceptConstraint::DirectDescendantOfOl,
                                    AriaRoleRelationConceptConstraint::DirectDescendantOfUl,
                                    AriaRoleRelationConceptConstraint::DirectDescendantOfMenu,
                                ]),
                                name: "li".into(),
                                attributes: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "item".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("XForms".into()),
                        },
                    ],
                    require_context_role: vec![AriaRole::Directory, AriaRole::List],
                    required_context_role: vec![AriaRole::Directory, AriaRole::List],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::List,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "menu".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "ol".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "ul".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                    ],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![vec!["listitem".into()]],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Log,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([(AriaProperty::AriaLive, Some("polite".into()))]),
                    related_concepts: vec![],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Main,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "main".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Landmark),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Mark,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Prohibited],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaBraillelabel, None),
                        (AriaProperty::AriaBrailleroledescription, None),
                        (AriaProperty::AriaDescription, None),
                    ]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "mark".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Marquee,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Math,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "math".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Menubar,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([(
                        AriaProperty::AriaOrientation,
                        Some("horizontal".into()),
                    )]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "toolbar".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("ARIA".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![
                        vec!["menuitem".into(), "group".into()],
                        vec!["menuitemradio".into(), "group".into()],
                        vec!["menuitemcheckbox".into(), "group".into()],
                        vec!["menuitem".into()],
                        vec!["menuitemcheckbox".into()],
                        vec!["menuitemradio".into()],
                    ],
                    required_props: HashMap::new(),
                    super_class: vec![
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Composite),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Select),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Menu),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Group),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Select),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Menu),
                        ],
                    ],
                },
            ),
            (
                AriaRoleDefinitionKey::Menuitemcheckbox,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: true,
                    name_from: vec![AriaNameFromSource::Author, AriaNameFromSource::Contents],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "menuitem".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("ARIA".into()),
                    }],
                    require_context_role: vec![AriaRole::Group, AriaRole::Menu, AriaRole::Menubar],
                    required_context_role: vec![AriaRole::Group, AriaRole::Menu, AriaRole::Menubar],
                    required_owned_elements: vec![],
                    required_props: HashMap::from([(AriaProperty::AriaChecked, None)]),
                    super_class: vec![
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Input),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Checkbox),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Command),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Menuitem),
                        ],
                    ],
                },
            ),
            (
                AriaRoleDefinitionKey::Menuitemradio,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: true,
                    name_from: vec![AriaNameFromSource::Author, AriaNameFromSource::Contents],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "menuitem".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("ARIA".into()),
                    }],
                    require_context_role: vec![AriaRole::Group, AriaRole::Menu, AriaRole::Menubar],
                    required_context_role: vec![AriaRole::Group, AriaRole::Menu, AriaRole::Menubar],
                    required_owned_elements: vec![],
                    required_props: HashMap::from([(AriaProperty::AriaChecked, None)]),
                    super_class: vec![
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Input),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Menuitem),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Menuitemcheckbox),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Command),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Menuitem),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Menuitemcheckbox),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Input),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Radio),
                        ],
                    ],
                },
            ),
            (
                AriaRoleDefinitionKey::Menuitem,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author, AriaNameFromSource::Contents],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaDisabled, None),
                        (AriaProperty::AriaExpanded, None),
                        (AriaProperty::AriaHaspopup, None),
                        (AriaProperty::AriaPosinset, None),
                        (AriaProperty::AriaSetsize, None),
                    ]),
                    related_concepts: vec![
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "MENU_ITEM".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("JAPI".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "listitem".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("ARIA".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "option".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("ARIA".into()),
                        },
                    ],
                    require_context_role: vec![AriaRole::Group, AriaRole::Menu, AriaRole::Menubar],
                    required_context_role: vec![AriaRole::Group, AriaRole::Menu, AriaRole::Menubar],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Command),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Menu,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([(
                        AriaProperty::AriaOrientation,
                        Some("vertical".into()),
                    )]),
                    related_concepts: vec![
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "MENU".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("JAPI".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "list".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("ARIA".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "select".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("XForms".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "sidebar".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("DTB".into()),
                        },
                    ],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![
                        vec!["menuitem".into(), "group".into()],
                        vec!["menuitemradio".into(), "group".into()],
                        vec!["menuitemcheckbox".into(), "group".into()],
                        vec!["menuitem".into()],
                        vec!["menuitemcheckbox".into()],
                        vec!["menuitemradio".into()],
                    ],
                    required_props: HashMap::new(),
                    super_class: vec![
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Composite),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Select),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Group),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Select),
                        ],
                    ],
                },
            ),
            (
                AriaRoleDefinitionKey::Meter,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: true,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaValuetext, None),
                        (AriaProperty::AriaValuemax, Some("100".into())),
                        (AriaProperty::AriaValuemin, Some("0".into())),
                    ]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "meter".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::from([(AriaProperty::AriaValuenow, None)]),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Range),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Navigation,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "nav".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Landmark),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::None,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![],
                },
            ),
            (
                AriaRoleDefinitionKey::Note,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Option,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: true,
                    name_from: vec![AriaNameFromSource::Author, AriaNameFromSource::Contents],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaChecked, None),
                        (AriaProperty::AriaPosinset, None),
                        (AriaProperty::AriaSetsize, None),
                        (AriaProperty::AriaSelected, Some("false".into())),
                    ]),
                    related_concepts: vec![
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "item".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("XForms".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "listitem".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("ARIA".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "option".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                    ],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::from([(
                        AriaProperty::AriaSelected,
                        Some("false".into()),
                    )]),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Input),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Paragraph,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Prohibited],
                    prohibited_props: vec![AriaProperty::AriaLabel, AriaProperty::AriaLabelledby],
                    props: HashMap::from([]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "p".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Presentation,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Prohibited],
                    prohibited_props: vec![AriaProperty::AriaLabel, AriaProperty::AriaLabelledby],
                    props: HashMap::from([]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            attributes: Some(vec![AriaRoleRelationConceptAttribute {
                                name: "alt".into(),
                                value: Some("".into()),
                                constraints: None,
                            }]),
                            name: "img".into(),
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Progressbar,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: true,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([(AriaProperty::AriaValuetext, None)]),
                    related_concepts: vec![
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "progress".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "status".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("ARIA".into()),
                        },
                    ],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Range),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                        ],
                    ],
                },
            ),
            (
                AriaRoleDefinitionKey::Radiogroup,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaErrormessage, None),
                        (AriaProperty::AriaInvalid, None),
                        (AriaProperty::AriaReadonly, None),
                        (AriaProperty::AriaRequired, None),
                    ]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "list".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("ARIA".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![vec!["radio".into()]],
                    required_props: HashMap::new(),
                    super_class: vec![
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Composite),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Select),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Group),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Select),
                        ],
                    ],
                },
            ),
            (
                AriaRoleDefinitionKey::Radio,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: true,
                    name_from: vec![AriaNameFromSource::Author, AriaNameFromSource::Contents],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaChecked, None),
                        (AriaProperty::AriaPosinset, None),
                        (AriaProperty::AriaSetsize, None),
                    ]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            attributes: Some(vec![AriaRoleRelationConceptAttribute {
                                name: "type".into(),
                                value: Some("radio".into()),
                                constraints: None,
                            }]),
                            name: "input".into(),
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::from([(AriaProperty::AriaChecked, None)]),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Input),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Region,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![AriaRoleRelationConceptAttribute {
                                    constraints: Some(vec![
                                        AriaRoleRelationConceptAttributeConstraint::Set,
                                    ]),
                                    name: "aria-label".into(),
                                    value: None,
                                }]),
                                name: "section".into(),
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![AriaRoleRelationConceptAttribute {
                                    constraints: Some(vec![
                                        AriaRoleRelationConceptAttributeConstraint::Set,
                                    ]),
                                    name: "aria-labelledby".into(),
                                    value: None,
                                }]),
                                name: "section".into(),
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "Device Independence Glossart perceivable unit".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: None,
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
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Landmark),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Rowgroup,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author, AriaNameFromSource::Contents],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "tbody".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "tfoot".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "thead".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                    ],
                    require_context_role: vec![AriaRole::Grid, AriaRole::Table, AriaRole::Treegrid],
                    required_context_role: vec![
                        AriaRole::Grid,
                        AriaRole::Table,
                        AriaRole::Treegrid,
                    ],
                    required_owned_elements: vec![vec!["row".into()]],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Rowheader,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author, AriaNameFromSource::Contents],
                    prohibited_props: vec![],
                    props: HashMap::from([(AriaProperty::AriaSort, None)]),
                    related_concepts: vec![
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![AriaRoleRelationConceptAttribute {
                                    name: "scope".into(),
                                    value: Some("row".into()),
                                    constraints: None,
                                }]),
                                name: "th".into(),
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![AriaRoleRelationConceptAttribute {
                                    name: "scope".into(),
                                    value: Some("rowgroup".into()),
                                    constraints: None,
                                }]),
                                name: "th".into(),
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                    ],
                    require_context_role: vec![AriaRole::Row, AriaRole::Rowgroup],
                    required_context_role: vec![AriaRole::Row, AriaRole::Rowgroup],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Cell),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Cell),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Gridcell),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Gridcell),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                            AriaRoleDefinitionSuperClass::AbstractRole(
                                AriaAbstractRole::Sectionhead,
                            ),
                        ],
                    ],
                },
            ),
            (
                AriaRoleDefinitionKey::Row,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author, AriaNameFromSource::Contents],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaColindex, None),
                        (AriaProperty::AriaExpanded, None),
                        (AriaProperty::AriaLevel, None),
                        (AriaProperty::AriaPosinset, None),
                        (AriaProperty::AriaRowindex, None),
                        (AriaProperty::AriaSelected, None),
                        (AriaProperty::AriaSetsize, None),
                    ]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "tr".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![
                        AriaRole::Grid,
                        AriaRole::Rowgroup,
                        AriaRole::Table,
                        AriaRole::Treegrid,
                    ],
                    required_context_role: vec![
                        AriaRole::Grid,
                        AriaRole::Rowgroup,
                        AriaRole::Table,
                        AriaRole::Treegrid,
                    ],
                    required_owned_elements: vec![
                        vec!["cell".into()],
                        vec!["columnheader".into()],
                        vec!["gridcell".into()],
                        vec!["rowheader".into()],
                    ],
                    required_props: HashMap::new(),
                    super_class: vec![
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Group),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                        ],
                    ],
                },
            ),
            (
                AriaRoleDefinitionKey::Scrollbar,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: true,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaDisabled, None),
                        (AriaProperty::AriaValuetext, None),
                        (AriaProperty::AriaOrientation, Some("vertical".into())),
                        (AriaProperty::AriaValuemax, Some("100".into())),
                        (AriaProperty::AriaValuemin, Some("0".into())),
                    ]),
                    related_concepts: vec![],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::from([
                        (AriaProperty::AriaControls, None),
                        (AriaProperty::AriaValuenow, None),
                    ]),
                    super_class: vec![
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Range),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                        ],
                    ],
                },
            ),
            (
                AriaRoleDefinitionKey::Searchbox,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            attributes: Some(vec![
                                AriaRoleRelationConceptAttribute {
                                    constraints: Some(vec![
                                        AriaRoleRelationConceptAttributeConstraint::Undefined,
                                    ]),
                                    name: "list".into(),
                                    value: None,
                                },
                                AriaRoleRelationConceptAttribute {
                                    name: "type".into(),
                                    value: Some("search".into()),
                                    constraints: None,
                                },
                            ]),
                            constraints: Some(vec![
                                AriaRoleRelationConceptConstraint::TheListAttributeIsNotSet,
                            ]),
                            name: "input".into(),
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Input),
                        AriaRoleDefinitionSuperClass::Role(AriaRole::Textbox),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Search,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Landmark),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Separator,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: true,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaDisabled, None),
                        (AriaProperty::AriaOrientation, Some("horizontal".into())),
                        (AriaProperty::AriaValuemax, Some("100".into())),
                        (AriaProperty::AriaValuemin, Some("0".into())),
                        (AriaProperty::AriaValuenow, None),
                        (AriaProperty::AriaValuetext, None),
                    ]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "hr".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Slider,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: true,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaErrormessage, None),
                        (AriaProperty::AriaHaspopup, None),
                        (AriaProperty::AriaInvalid, None),
                        (AriaProperty::AriaReadonly, None),
                        (AriaProperty::AriaValuetext, None),
                        (AriaProperty::AriaOrientation, Some("horizontal".into())),
                        (AriaProperty::AriaValuemax, Some("100".into())),
                        (AriaProperty::AriaValuemin, Some("0".into())),
                    ]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            attributes: Some(vec![AriaRoleRelationConceptAttribute {
                                name: "type".into(),
                                value: Some("range".into()),
                                constraints: None,
                            }]),
                            name: "input".into(),
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::from([(AriaProperty::AriaValuenow, None)]),
                    super_class: vec![
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Input),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Range),
                        ],
                    ],
                },
            ),
            (
                AriaRoleDefinitionKey::Spinbutton,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaErrormessage, None),
                        (AriaProperty::AriaInvalid, None),
                        (AriaProperty::AriaReadonly, None),
                        (AriaProperty::AriaRequired, None),
                        (AriaProperty::AriaValuetext, None),
                        (AriaProperty::AriaValuenow, Some("0".into())),
                    ]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            attributes: Some(vec![AriaRoleRelationConceptAttribute {
                                name: "type".into(),
                                value: Some("number".into()),
                                constraints: None,
                            }]),
                            name: "input".into(),
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Composite),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Input),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Range),
                        ],
                    ],
                },
            ),
            (
                AriaRoleDefinitionKey::Status,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaAtomic, Some("true".into())),
                        (AriaProperty::AriaLive, Some("polite".into())),
                    ]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "output".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Strong,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Prohibited],
                    prohibited_props: vec![AriaProperty::AriaLabel, AriaProperty::AriaLabelledby],
                    props: HashMap::from([]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "strong".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Subscript,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Prohibited],
                    prohibited_props: vec![AriaProperty::AriaLabel, AriaProperty::AriaLabelledby],
                    props: HashMap::from([]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "sub".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Superscript,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Prohibited],
                    prohibited_props: vec![AriaProperty::AriaLabel, AriaProperty::AriaLabelledby],
                    props: HashMap::from([]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "sup".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Switch,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: true,
                    name_from: vec![AriaNameFromSource::Author, AriaNameFromSource::Contents],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "button".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("ARIA".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::from([(AriaProperty::AriaChecked, None)]),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Input),
                        AriaRoleDefinitionSuperClass::Role(AriaRole::Checkbox),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Table,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaColcount, None),
                        (AriaProperty::AriaRowcount, None),
                    ]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "table".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![
                        vec!["row".into()],
                        vec!["row".into(), "rowgroup".into()],
                    ],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Tablist,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaLevel, None),
                        (AriaProperty::AriaMultiselectable, None),
                        (AriaProperty::AriaOrientation, Some("horizontal".into())),
                    ]),
                    related_concepts: vec![AriaRoleRelation {
                        module: Some("DAISY".into()),
                        concept: Some(AriaRoleRelationConcept {
                            name: "guide".into(),
                            attributes: None,
                            constraints: None,
                        }),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![vec!["tab".into()]],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Composite),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Tabpanel,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Tab,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: true,
                    name_from: vec![AriaNameFromSource::Author, AriaNameFromSource::Contents],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaDisabled, None),
                        (AriaProperty::AriaExpanded, None),
                        (AriaProperty::AriaHaspopup, None),
                        (AriaProperty::AriaPosinset, None),
                        (AriaProperty::AriaSetsize, None),
                        (AriaProperty::AriaSelected, Some("false".into())),
                    ]),
                    related_concepts: vec![],
                    require_context_role: vec![AriaRole::Tablist],
                    required_context_role: vec![AriaRole::Tablist],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                            AriaRoleDefinitionSuperClass::AbstractRole(
                                AriaAbstractRole::Sectionhead,
                            ),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                        ],
                    ],
                },
            ),
            (
                AriaRoleDefinitionKey::Term,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "dfn".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "dt".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
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
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Textbox,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaActivedescendant, None),
                        (AriaProperty::AriaAutocomplete, None),
                        (AriaProperty::AriaErrormessage, None),
                        (AriaProperty::AriaHaspopup, None),
                        (AriaProperty::AriaInvalid, None),
                        (AriaProperty::AriaMultiline, None),
                        (AriaProperty::AriaPlaceholder, None),
                        (AriaProperty::AriaReadonly, None),
                        (AriaProperty::AriaRequired, None),
                    ]),
                    related_concepts: vec![
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![
                                    AriaRoleRelationConceptAttribute {
                                        constraints: Some(vec![
                                            AriaRoleRelationConceptAttributeConstraint::Undefined,
                                        ]),
                                        name: "type".into(),
                                        value: None,
                                    },
                                    AriaRoleRelationConceptAttribute {
                                        constraints: Some(vec![
                                            AriaRoleRelationConceptAttributeConstraint::Undefined,
                                        ]),
                                        name: "list".into(),
                                        value: None,
                                    },
                                ]),
                                constraints: Some(vec![
                                    AriaRoleRelationConceptConstraint::TheListAttributeIsNotSet,
                                ]),
                                name: "input".into(),
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![
                                    AriaRoleRelationConceptAttribute {
                                        constraints: Some(vec![
                                            AriaRoleRelationConceptAttributeConstraint::Undefined,
                                        ]),
                                        name: "list".into(),
                                        value: None,
                                    },
                                    AriaRoleRelationConceptAttribute {
                                        name: "type".into(),
                                        value: Some("email".into()),
                                        constraints: None,
                                    },
                                ]),
                                constraints: Some(vec![
                                    AriaRoleRelationConceptConstraint::TheListAttributeIsNotSet,
                                ]),
                                name: "input".into(),
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![
                                    AriaRoleRelationConceptAttribute {
                                        constraints: Some(vec![
                                            AriaRoleRelationConceptAttributeConstraint::Undefined,
                                        ]),
                                        name: "list".into(),
                                        value: None,
                                    },
                                    AriaRoleRelationConceptAttribute {
                                        name: "type".into(),
                                        value: Some("tel".into()),
                                        constraints: None,
                                    },
                                ]),
                                constraints: Some(vec![
                                    AriaRoleRelationConceptConstraint::TheListAttributeIsNotSet,
                                ]),
                                name: "input".into(),
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![
                                    AriaRoleRelationConceptAttribute {
                                        constraints: Some(vec![
                                            AriaRoleRelationConceptAttributeConstraint::Undefined,
                                        ]),
                                        name: "list".into(),
                                        value: None,
                                    },
                                    AriaRoleRelationConceptAttribute {
                                        name: "type".into(),
                                        value: Some("text".into()),
                                        constraints: None,
                                    },
                                ]),
                                constraints: Some(vec![
                                    AriaRoleRelationConceptConstraint::TheListAttributeIsNotSet,
                                ]),
                                name: "input".into(),
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                attributes: Some(vec![
                                    AriaRoleRelationConceptAttribute {
                                        constraints: Some(vec![
                                            AriaRoleRelationConceptAttributeConstraint::Undefined,
                                        ]),
                                        name: "list".into(),
                                        value: None,
                                    },
                                    AriaRoleRelationConceptAttribute {
                                        name: "type".into(),
                                        value: Some("url".into()),
                                        constraints: None,
                                    },
                                ]),
                                constraints: Some(vec![
                                    AriaRoleRelationConceptConstraint::TheListAttributeIsNotSet,
                                ]),
                                name: "input".into(),
                            }),
                            module: Some("HTML".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "input".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("XForms".into()),
                        },
                        AriaRoleRelation {
                            concept: Some(AriaRoleRelationConcept {
                                name: "textarea".into(),
                                attributes: None,
                                constraints: None,
                            }),
                            module: Some("HTML".into()),
                        },
                    ],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Input),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Time,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "time".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("HTML".into()),
                    }],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Timer,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                        AriaRoleDefinitionSuperClass::Role(AriaRole::Status),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Toolbar,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: false,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([(
                        AriaProperty::AriaOrientation,
                        Some("horizontal".into()),
                    )]),
                    related_concepts: vec![AriaRoleRelation {
                        concept: Some(AriaRoleRelationConcept {
                            name: "menubar".into(),
                            attributes: None,
                            constraints: None,
                        }),
                        module: Some("ARIA".into()),
                    }],
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
                AriaRoleDefinitionKey::Tooltip,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author, AriaNameFromSource::Contents],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![],
                    required_props: HashMap::new(),
                    super_class: vec![vec![
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                        AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                    ]],
                },
            ),
            (
                AriaRoleDefinitionKey::Treegrid,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([]),
                    related_concepts: vec![],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![
                        vec!["row".into()],
                        vec!["row".into(), "rowgroup".into()],
                    ],
                    required_props: HashMap::new(),
                    super_class: vec![
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Composite),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Grid),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Table),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Grid),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Composite),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Select),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Tree),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Group),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Select),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Tree),
                        ],
                    ],
                },
            ),
            (
                AriaRoleDefinitionKey::Treeitem,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author, AriaNameFromSource::Contents],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaExpanded, None),
                        (AriaProperty::AriaHaspopup, None),
                    ]),
                    related_concepts: vec![],
                    require_context_role: vec![AriaRole::Group, AriaRole::Tree],
                    required_context_role: vec![AriaRole::Group, AriaRole::Tree],
                    required_owned_elements: vec![],
                    required_props: HashMap::from([(AriaProperty::AriaSelected, None)]),
                    super_class: vec![
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Listitem),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Input),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Option),
                        ],
                    ],
                },
            ),
            (
                AriaRoleDefinitionKey::Tree,
                AriaRoleDefinition {
                    r#abstract: false,
                    accessible_name_required: true,
                    base_concepts: vec![],
                    children_presentational: false,
                    name_from: vec![AriaNameFromSource::Author],
                    prohibited_props: vec![],
                    props: HashMap::from([
                        (AriaProperty::AriaErrormessage, None),
                        (AriaProperty::AriaInvalid, None),
                        (AriaProperty::AriaMultiselectable, None),
                        (AriaProperty::AriaRequired, None),
                        (AriaProperty::AriaOrientation, Some("vertical".into())),
                    ]),
                    related_concepts: vec![],
                    require_context_role: vec![],
                    required_context_role: vec![],
                    required_owned_elements: vec![
                        vec!["treeitem".into(), "group".into()],
                        vec!["treeitem".into()],
                    ],
                    required_props: HashMap::new(),
                    super_class: vec![
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Widget),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Composite),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Select),
                        ],
                        vec![
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Roletype),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Structure),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Section),
                            AriaRoleDefinitionSuperClass::Role(AriaRole::Status),
                            AriaRoleDefinitionSuperClass::AbstractRole(AriaAbstractRole::Select),
                        ],
                    ],
                },
            ),
        ])
});
