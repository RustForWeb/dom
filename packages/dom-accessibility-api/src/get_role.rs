use std::{collections::HashMap, sync::LazyLock};

use web_sys::{wasm_bindgen::JsCast, Element, HtmlInputElement, HtmlSelectElement};

use crate::util::PRESENTATION_ROLES;

// https://w3c.github.io/html-aria/#document-conformance-requirements-for-use-of-aria-attributes-in-html

static LOCAL_NAME_TO_ROLE_MAPPINGS: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    HashMap::from([
        ("article".into(), "article".into()),
        ("aside".into(), "complementary".into()),
        ("button".into(), "button".into()),
        ("datalist".into(), "listbox".into()),
        ("dd".into(), "definition".into()),
        ("details".into(), "group".into()),
        ("dialog".into(), "dialog".into()),
        ("dt".into(), "term".into()),
        ("fieldset".into(), "group".into()),
        ("figure".into(), "figure".into()),
        // Warning: Only with an accessible name.
        ("form".into(), "form".into()),
        ("footer".into(), "contentinfo".into()),
        ("h1".into(), "heading".into()),
        ("h2".into(), "heading".into()),
        ("h3".into(), "heading".into()),
        ("h4".into(), "heading".into()),
        ("h5".into(), "heading".into()),
        ("h6".into(), "heading".into()),
        ("header".into(), "banner".into()),
        ("hr".into(), "separator".into()),
        ("html".into(), "document".into()),
        ("legend".into(), "legend".into()),
        ("li".into(), "listitem".into()),
        ("math".into(), "math".into()),
        ("main".into(), "main".into()),
        ("menu".into(), "list".into()),
        ("nav".into(), "navigation".into()),
        ("ol".into(), "list".into()),
        ("optgroup".into(), "group".into()),
        // Warning: Only in certain context.
        ("option".into(), "option".into()),
        ("output".into(), "status".into()),
        ("progress".into(), "progressbar".into()),
        // Warning: Only with an accessible name.
        ("section".into(), "region".into()),
        ("summary".into(), "button".into()),
        ("table".into(), "table".into()),
        ("tbody".into(), "rowgroup".into()),
        ("textarea".into(), "textbox".into()),
        ("tfoot".into(), "rowgroup".into()),
        // Warning: Only in certain context.
        ("td".into(), "cell".into()),
        ("th".into(), "columnheader".into()),
        ("thead".into(), "rowgroup".into()),
        ("tr".into(), "row".into()),
        ("ul".into(), "list".into()),
    ])
});

// https://rawgit.com/w3c/aria/stable/#global_states
// Commented attributes are deprecated.
const GLOBAL_ARIA_ATTRIBUTES: [&str; 18] = [
    "aria-atomic",
    "aria-busy",
    "aria-controls",
    "aria-current",
    "aria-description",
    "aria-describedby",
    "aria-details",
    // "disabled",
    "aria-dropeffect",
    // "errormessage",
    "aria-flowto",
    "aria-grabbed",
    // "haspopup",
    "aria-hidden",
    // "invalid",
    "aria-keyshortcuts",
    "aria-label",
    "aria-labelledby",
    "aria-live",
    "aria-owns",
    "aria-relevant",
    "aria-roledescription",
];

static PROHIBITED_ATTRIBUTES: LazyLock<HashMap<String, Vec<String>>> = LazyLock::new(|| {
    HashMap::from([
        (
            "caption".to_string(),
            vec!["aria-label".to_string(), "aria-labelledby".to_string()],
        ),
        (
            "code".to_string(),
            vec!["aria-label".to_string(), "aria-labelledby".to_string()],
        ),
        (
            "deletion".to_string(),
            vec!["aria-label".to_string(), "aria-labelledby".to_string()],
        ),
        (
            "emphasis".to_string(),
            vec!["aria-label".to_string(), "aria-labelledby".to_string()],
        ),
        (
            "generic".to_string(),
            vec![
                "aria-label".to_string(),
                "aria-labelledby".to_string(),
                "aria-roledescription".to_string(),
            ],
        ),
        (
            "insertion".to_string(),
            vec!["aria-label".to_string(), "aria-labelledby".to_string()],
        ),
        (
            "none".to_string(),
            vec!["aria-label".to_string(), "aria-labelledby".to_string()],
        ),
        (
            "paragraph".to_string(),
            vec!["aria-label".to_string(), "aria-labelledby".to_string()],
        ),
        (
            "presentation".to_string(),
            vec!["aria-label".to_string(), "aria-labelledby".to_string()],
        ),
        (
            "strong".to_string(),
            vec!["aria-label".to_string(), "aria-labelledby".to_string()],
        ),
        (
            "subscript".to_string(),
            vec!["aria-label".to_string(), "aria-labelledby".to_string()],
        ),
        (
            "superscript".to_string(),
            vec!["aria-label".to_string(), "aria-labelledby".to_string()],
        ),
    ])
});

fn has_global_aria_attributes(element: &Element, role: String) -> bool {
    GLOBAL_ARIA_ATTRIBUTES.iter().any(|attribute_name| {
        element.has_attribute(attribute_name)
            && !PROHIBITED_ATTRIBUTES
                .get(&role)
                .is_some_and(|attributes| attributes.contains(&attribute_name.to_string()))
    })
}

fn ignore_presentational_role(element: &Element, implicit_role: String) -> bool {
    // https://rawgit.com/w3c/aria/stable/#conflict_resolution_presentation_none
    has_global_aria_attributes(element, implicit_role)
}

pub fn get_role(element: &Element) -> Option<String> {
    let explicit_role = get_explicit_role(element);
    if explicit_role.is_none()
        || explicit_role
            .as_ref()
            .is_some_and(|explicit_role| PRESENTATION_ROLES.contains(&explicit_role.as_str()))
    {
        let implicit_role = get_implicit_role(element);
        if explicit_role.is_none()
            || ignore_presentational_role(element, implicit_role.clone().unwrap_or("".into()))
        {
            return implicit_role;
        }
    }

    explicit_role
}

fn get_implicit_role(element: &Element) -> Option<String> {
    let local_name = element.local_name();

    if let Some(mapped_by_tag) = LOCAL_NAME_TO_ROLE_MAPPINGS.get(&local_name) {
        return Some(mapped_by_tag.clone());
    }

    match local_name.as_str() {
        "a" | "area" | "link" => element.has_attribute("href").then_some("link".into()),
        "img" => {
            if element.get_attribute("alt") == Some("".into())
                && !ignore_presentational_role(element, "img".into())
            {
                Some("presentation".into())
            } else {
                Some("img".into())
            }
        }
        "input" => {
            let r#type = element.unchecked_ref::<HtmlInputElement>().type_();
            match r#type.as_str() {
                "button" | "image" | "reset" | "submit" => Some("button".into()),
                "checkbox" | "radio" => Some(r#type),
                "range" => Some("slider".into()),
                "email" | "tel" | "text" | "url" => {
                    if element.has_attribute("list") {
                        Some("combobox".into())
                    } else {
                        Some("textbox".into())
                    }
                }
                "search" => {
                    if element.has_attribute("list") {
                        Some("combobox".into())
                    } else {
                        Some("searchbox".into())
                    }
                }
                "number" => Some("spinbutton".into()),
                _ => None,
            }
        }
        "select" => {
            if element.has_attribute("multiple")
                && element.unchecked_ref::<HtmlSelectElement>().size() > 1
            {
                Some("listbox".into())
            } else {
                Some("combobox".into())
            }
        }
        _ => None,
    }
}

fn get_explicit_role(element: &Element) -> Option<String> {
    element.get_attribute("role").and_then(|role| {
        role.trim().split(' ').next().and_then(|explicit_role| {
            (!explicit_role.is_empty()).then_some(explicit_role.to_string())
        })
    })
}
