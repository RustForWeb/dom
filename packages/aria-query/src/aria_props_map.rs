use std::{collections::HashMap, sync::LazyLock};

use crate::{
    AriaPropertyDefinitionType,
    types::{AriaProperty, AriaPropertyDefinition},
};

pub static ARIA: LazyLock<HashMap<AriaProperty, AriaPropertyDefinition>> = LazyLock::new(|| {
    HashMap::from([
        (
            AriaProperty::AriaActivedescendant,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Id,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaAtomic,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Boolean,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaAutocomplete,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Token,
                values: Some(vec![
                    "inline".into(),
                    "list".into(),
                    "both".into(),
                    "none".into(),
                ]),
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaBraillelabel,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::String,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaBrailleroledescription,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::String,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaBusy,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Boolean,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaChecked,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Tristate,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaColcount,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Integer,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaColindex,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Integer,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaColspan,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Integer,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaControls,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Idlist,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaCurrent,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Token,
                values: Some(vec![
                    "page".into(),
                    "step".into(),
                    "location".into(),
                    "date".into(),
                    "time".into(),
                    "true".into(),
                    "false".into(),
                ]),
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaDescribedby,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Idlist,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaDescription,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::String,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaDetails,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Id,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaDisabled,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Boolean,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaDropeffect,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Tokenlist,
                values: Some(vec![
                    "copy".into(),
                    "execute".into(),
                    "link".into(),
                    "move".into(),
                    "none".into(),
                    "popup".into(),
                ]),
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaErrormessage,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Id,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaExpanded,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Boolean,
                values: None,
                allow_undefined: Some(true),
            },
        ),
        (
            AriaProperty::AriaFlowto,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Idlist,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaGrabbed,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Boolean,
                values: None,
                allow_undefined: Some(true),
            },
        ),
        (
            AriaProperty::AriaHaspopup,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Token,
                values: Some(vec![
                    "false".into(),
                    "true".into(),
                    "menu".into(),
                    "listbox".into(),
                    "tree".into(),
                    "grid".into(),
                    "dialog".into(),
                ]),
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaHidden,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Boolean,
                values: None,
                allow_undefined: Some(true),
            },
        ),
        (
            AriaProperty::AriaInvalid,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Token,
                values: Some(vec![
                    "grammar".into(),
                    "false".into(),
                    "spelling".into(),
                    "true".into(),
                ]),
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaKeyshortcuts,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::String,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaLabel,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::String,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaLabelledby,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Idlist,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaLevel,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Integer,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaLive,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Token,
                values: Some(vec!["assertive".into(), "off".into(), "polite".into()]),
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaModal,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Boolean,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaMultiline,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Boolean,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaMultiselectable,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Boolean,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaOrientation,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Token,
                values: Some(vec![
                    "vertical".into(),
                    "undefined".into(),
                    "horizontal".into(),
                ]),
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaOwns,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Idlist,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaPlaceholder,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::String,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaPosinset,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Integer,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaPressed,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Tristate,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaReadonly,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Boolean,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaRelevant,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Tokenlist,
                values: Some(vec![
                    "additions".into(),
                    "all".into(),
                    "removals".into(),
                    "text".into(),
                ]),
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaRequired,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Boolean,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaRoledescription,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::String,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaRowcount,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Integer,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaRowindex,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Integer,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaRowspan,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Integer,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaSelected,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Boolean,
                values: None,
                allow_undefined: Some(true),
            },
        ),
        (
            AriaProperty::AriaSetsize,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Integer,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaRelevant,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Token,
                values: Some(vec![
                    "ascending".into(),
                    "descending".into(),
                    "none".into(),
                    "other".into(),
                ]),
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaValuemax,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Number,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaValuemin,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Number,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaValuenow,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::Number,
                values: None,
                allow_undefined: None,
            },
        ),
        (
            AriaProperty::AriaValuetext,
            AriaPropertyDefinition {
                r#type: AriaPropertyDefinitionType::String,
                values: None,
                allow_undefined: None,
            },
        ),
    ])
});
