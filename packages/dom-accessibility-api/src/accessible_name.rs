use web_sys::Element;

use crate::{
    accessible_name_and_description::{compute_text_alternative, ComputeTextAlternativeOptions},
    util::has_any_concrete_roles,
};

/// Implements <https://w3c.github.io/aria/#namefromprohibited>.
fn prohibits_naming(element: &Element) -> bool {
    has_any_concrete_roles(
        element,
        vec![
            "caption",
            "code",
            "deletion",
            "emphasis",
            "generic",
            "insertion",
            "none",
            "paragraph",
            "presentation",
            "strong",
            "subscript",
            "superscript",
        ],
    )
}

/// Implements <https://w3c.github.io/accname/#mapping_additional_nd_name>.
pub fn compute_accessible_name(root: &Element, options: ComputeTextAlternativeOptions) -> String {
    if prohibits_naming(root) {
        "".into()
    } else {
        compute_text_alternative(root, options)
    }
}
