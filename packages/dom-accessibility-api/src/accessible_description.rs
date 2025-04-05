use web_sys::Element;

use crate::{
    accessible_name_and_description::{
        Compute, ComputeTextAlternativeOptions, compute_text_alternative,
    },
    util::query_id_refs,
};

/// Implements <https://w3c.github.io/accname/#mapping_additional_nd_description>.
pub fn compute_accessible_description(
    root: &Element,
    options: ComputeTextAlternativeOptions,
) -> String {
    let mut description = query_id_refs(root, "aria-describedby")
        .iter()
        .map(|element| {
            compute_text_alternative(
                element,
                ComputeTextAlternativeOptions {
                    compute: Some(Compute::Description),
                    ..options.clone()
                },
            )
        })
        .collect::<Vec<_>>()
        .join(" ");

    // TODO: Technically we need to make sure that node wasn't used for the accessible name.
    //       This causes `description_1.0_combobox-focusable-manual` to fail.

    // https://w3c.github.io/aria/#aria-description
    // Mentions that aria-description should only be calculated if aria-describedby didn't provide a description.
    if description.is_empty() {
        if let Some(aria_description) = root.get_attribute("aria-description") {
            description = aria_description;
        }
    }

    // https://www.w3.org/TR/html-aam-1.0/#accessible-name-and-description-computation
    // Says for so many elements to use the `title` that we assume all elements are considered.
    if description.is_empty() {
        if let Some(title) = root.get_attribute("title") {
            description = title;
        }
    }

    description
}
