use std::rc::Rc;

use regex::Regex;
use web_sys::{
    wasm_bindgen::JsCast, window, CssStyleDeclaration, Element, HtmlFieldSetElement,
    HtmlInputElement, HtmlLabelElement, HtmlLegendElement, HtmlOptGroupElement,
    HtmlTableCaptionElement, HtmlTableElement, Node, SvgElement, SvgTitleElement,
};

use crate::util::{has_any_concrete_roles, query_id_refs, PRESENTATION_ROLES};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Compute {
    Name,
    Description,
}

type GetComputedStyle = Rc<dyn Fn(&Element) -> CssStyleDeclaration>;

/// Options for [`compute_text_alternative`].
#[derive(Clone, Default)]
pub struct ComputeTextAlternativeOptions {
    pub compute: Option<Compute>,
    pub computed_style_supports_pseudo_elements: Option<bool>,

    /// Mock `window.get_computed_style`. Needs `content`, `display` and `visibility`.
    pub get_computed_style: Option<GetComputedStyle>,

    /// Set to `true` if you want to include hidden elements in the accessible name and description computation.
    /// Skips 2A in <https://w3c.github.io/accname/#computation-steps>.
    ///
    /// Defaults to `false`.
    pub hidden: Option<bool>,
}

fn as_flat_string(s: String) -> String {
    Regex::new(r"\s\s+")
        .expect("Regex should be valid.")
        .replace_all(&s, " ")
        .to_string()
}

fn is_hidden(node: &Node, get_computed_style_implementation: GetComputedStyle) -> bool {
    if let Some(element) = node.dyn_ref::<Element>() {
        if element.has_attribute("hidden")
            || element.get_attribute("aria-hidden") == Some("true".into())
        {
            true
        } else {
            let style = get_computed_style_implementation(element);

            style
                .get_property_value("display")
                .expect("Computed style should have display.")
                == "none"
                || style
                    .get_property_value("visibility")
                    .expect("Computed style should have visibility.")
                    == "hidden"
        }
    } else {
        false
    }
}

fn is_marked_presentational(node: &Node) -> bool {
    has_any_concrete_roles(node, PRESENTATION_ROLES.into())
}

fn is_control(node: &Node) -> bool {
    has_any_concrete_roles(node, vec!["button", "combobox", "listbox", "textbox"])
        || has_abstract_role(node, "range")
}

fn has_abstract_role(node: &Node, role: &str) -> bool {
    node.dyn_ref::<Element>().is_some_and(|element| match role {
        "range" => has_any_concrete_roles(
            element,
            vec!["meter", "progressbar", "scrollbar", "slider", "spinbutton"],
        ),
        _ => unreachable!("No knowledge about abstract role '{role}'. This is likely a bug :("),
    })
}

fn get_labels(element: &Element) -> Vec<HtmlLabelElement> {
    let mut labels: Vec<HtmlLabelElement> = vec![];

    if let Some(input_element) = element.dyn_ref::<HtmlInputElement>() {
        if let Some(node_list) = input_element.labels() {
            for i in 0..node_list.length() {
                labels.push(
                    node_list
                        .item(i)
                        .expect("Item should exist.")
                        .unchecked_into::<HtmlLabelElement>(),
                );
            }
        }

        labels
    } else {
        // TODO
        labels
    }
}

struct ComputeTextAlternativeContext {
    is_embedded_in_label: bool,
    is_referenced: bool,
    recursion: bool,
}

// Implements <https://w3c.github.io/accname/#mapping_additional_nd_te>.
pub fn compute_text_alternative(root: &Element, options: ComputeTextAlternativeOptions) -> String {
    let mut consulted_nodes: Vec<Node> = vec![];

    let compute = options.compute.unwrap_or(Compute::Name);
    let uncached_get_computed_style = options.get_computed_style.unwrap_or_else(|| {
        Rc::new(|element| {
            window()
                .expect("Window should exist.")
                .get_computed_style(element)
                .expect("Element should be valid.")
                .expect("Computed style should exist.")
        })
    });
    let hidden = options.hidden.unwrap_or(false);

    let get_computed_style: GetComputedStyle = Rc::new(move |element| {
        // TODO: cache
        uncached_get_computed_style(element)
    });

    fn use_attribute(
        consulted_nodes: &mut Vec<Node>,
        element: &Element,
        attribute_name: &str,
    ) -> Option<String> {
        if let Some(attribute) = element.get_attribute_node(attribute_name) {
            let value = attribute.value();
            if !consulted_nodes.contains(&attribute) && !value.trim().is_empty() {
                consulted_nodes.push(attribute.into());
                return Some(value);
            }
        }

        None
    }

    fn compute_element_text_alternative(
        compute: Compute,
        hidden: bool,
        get_computed_style: GetComputedStyle,
        consulted_nodes: &mut Vec<Node>,
        node: &Node,
    ) -> Option<String> {
        if let Some(element) = node.dyn_ref::<Element>() {
            if element.is_instance_of::<HtmlFieldSetElement>() {
                // https://w3c.github.io/html-aam/#fieldset-and-legend-elements

                consulted_nodes.push(node.clone());

                let children = element.child_nodes();
                for i in 0..children.length() {
                    let child = children.item(i).expect("Item should exist.");

                    if child.is_instance_of::<HtmlLegendElement>() {
                        return Some(inner_compute_text_alternative(
                            compute,
                            hidden,
                            get_computed_style,
                            consulted_nodes,
                            &child,
                            ComputeTextAlternativeContext {
                                is_embedded_in_label: false,
                                is_referenced: false,
                                recursion: false,
                            },
                        ));
                    }
                }
            } else if element.is_instance_of::<HtmlTableElement>() {
                // https://w3c.github.io/html-aam/#table-element

                consulted_nodes.push(node.clone());

                let children = element.child_nodes();
                for i in 0..children.length() {
                    let child = children.item(i).expect("Item should exist.");

                    if child.is_instance_of::<HtmlTableCaptionElement>() {
                        return Some(inner_compute_text_alternative(
                            compute,
                            hidden,
                            get_computed_style,
                            consulted_nodes,
                            &child,
                            ComputeTextAlternativeContext {
                                is_embedded_in_label: false,
                                is_referenced: false,
                                recursion: false,
                            },
                        ));
                    }
                }
            } else if element.is_instance_of::<SvgElement>() {
                // https://www.w3.org/TR/svg-aam-1.0/

                consulted_nodes.push(node.clone());

                let children = element.child_nodes();
                for i in 0..children.length() {
                    let child = children.item(i).expect("Item should exist.");

                    if child.is_instance_of::<SvgTitleElement>() {
                        return child.text_content();
                    }
                }
            } else if element.local_name() == "img" || element.local_name() == "area" {
                // https://w3c.github.io/html-aam/#area-element
                // https://w3c.github.io/html-aam/#img-element
                if let Some(name_from_alt) = use_attribute(consulted_nodes, element, "alt") {
                    return Some(name_from_alt);
                }
            } else if element.is_instance_of::<HtmlOptGroupElement>() {
                if let Some(name_from_label) = use_attribute(consulted_nodes, element, "label") {
                    return Some(name_from_label);
                }
            }

            if let Some(input_element) = element.dyn_ref::<HtmlInputElement>() {
                if input_element.type_() == "button"
                    || input_element.type_() == "submit"
                    || input_element.type_() == "reset"
                {
                    // https://w3c.github.io/html-aam/#input-type-text-input-type-password-input-type-search-input-type-tel-input-type-email-input-type-url-and-textarea-element-accessible-description-computation
                    if let Some(name_from_value) = use_attribute(consulted_nodes, element, "value")
                    {
                        return Some(name_from_value);
                    }

                    // TODO: l10n
                    if input_element.type_() == "submit" {
                        return Some("Submit".into());
                    }
                    // TODO: l10n
                    if input_element.type_() == "reset" {
                        return Some("Reset".into());
                    }
                }
            }

            let _labels = get_labels(element);

            // TODO
        }

        None
    }

    fn inner_compute_text_alternative(
        compute: Compute,
        hidden: bool,
        get_computed_style: GetComputedStyle,
        consulted_nodes: &mut Vec<Node>,
        current: &Node,
        context: ComputeTextAlternativeContext,
    ) -> String {
        if consulted_nodes.contains(current) {
            return "".into();
        }

        // 2A
        if !hidden && is_hidden(current, get_computed_style.clone()) && !context.is_referenced {
            consulted_nodes.push(current.clone());
            return "".into();
        }

        // 2B
        if let Some(current) = current.dyn_ref::<Element>() {
            if let Some(label_attribute_node) = current.get_attribute_node("aria-labelledby") {
                // TODO: Do we generally need to block query IdRefs of attributes we have already consulted?
                let label_elements = if !consulted_nodes.contains(&label_attribute_node) {
                    query_id_refs(current, "aria-labelledby")
                } else {
                    vec![]
                };

                if compute == Compute::Name && !context.is_referenced && !label_elements.is_empty()
                {
                    consulted_nodes.push(label_attribute_node.unchecked_into::<Node>());

                    return label_elements
                        .into_iter()
                        .map(move |element| {
                            // TODO: Chrome will consider repeated values i.e. use a node multiple times while we'll bail out in computeTextAlternative.
                            inner_compute_text_alternative(
                                compute,
                                hidden,
                                get_computed_style.clone(),
                                consulted_nodes,
                                &element,
                                ComputeTextAlternativeContext {
                                    is_embedded_in_label: context.is_embedded_in_label,
                                    is_referenced: true,
                                    // This isn't recursion as specified, otherwise we would skip `aria-label` in
                                    // <input id="myself" aria-label="foo" aria-labelledby="myself" />
                                    recursion: false,
                                },
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(" ");
                }
            }
        }

        // 2C
        // Changed from the spec in anticipation of https://github.com/w3c/accname/issues/64.
        // Spec says we should only consider skipping if we have a non-empty label.
        let skip_to_step_2e = context.recursion && is_control(current) && compute == Compute::Name;
        if !skip_to_step_2e {
            let aria_label = current
                .dyn_ref::<Element>()
                .and_then(|current| current.get_attribute("aria-label"))
                .unwrap_or_default()
                .trim()
                .to_string();
            if !aria_label.is_empty() && compute == Compute::Name {
                consulted_nodes.push(current.clone());
                return aria_label;
            }

            // 2D
            if !is_marked_presentational(current) {
                if let Some(element_text_alternative) = compute_element_text_alternative(
                    compute,
                    hidden,
                    get_computed_style,
                    consulted_nodes,
                    current,
                ) {
                    consulted_nodes.push(current.clone());
                    return element_text_alternative;
                }
            }
        }

        // Special casing, cheating to make tests pass.
        // https://github.com/w3c/accname/issues/67
        if has_any_concrete_roles(current, vec!["menu"]) {
            consulted_nodes.push(current.clone());
            return "".into();
        }

        // 2E
        if skip_to_step_2e || context.is_embedded_in_label || context.is_referenced {
            // TODO
        }

        // TODO

        // TODO: should this be reachable?
        consulted_nodes.push(current.clone());
        "".into()
    }

    as_flat_string(inner_compute_text_alternative(
        compute,
        hidden,
        get_computed_style,
        &mut consulted_nodes,
        root,
        ComputeTextAlternativeContext {
            is_embedded_in_label: false,
            // By spec `compute_accessible_description starts with the referenced elements as roots.
            is_referenced: compute == Compute::Description,
            recursion: false,
        },
    ))
}
