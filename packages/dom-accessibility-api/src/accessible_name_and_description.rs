use std::rc::Rc;

use web_sys::Element;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Compute {
    Name,
    Description,
}

/// Options for [`compute_text_alternative`].
#[derive(Clone, Default)]
pub struct ComputeTextAlternativeOptions {
    pub compute: Option<Compute>,
    pub computed_style_supports_pseudo_elements: Option<bool>,

    /// Mock `window.get_computed_style`. Needs `content`, `display` and `visibility`.
    pub get_computed_style: Option<Rc<dyn Fn()>>,

    /// Set to `true` if you want to include hidden elements in the accessible name and description computation.
    /// Skips 2A in <https://w3c.github.io/accname/#computation-steps>.
    ///
    /// Defaults to `false`.
    pub hidden: Option<bool>,
}

// Implements <https://w3c.github.io/accname/#mapping_additional_nd_te>.
pub fn compute_text_alternative(
    _root: &Element,
    _options: ComputeTextAlternativeOptions,
) -> String {
    todo!("compute_text_alternative")
}
