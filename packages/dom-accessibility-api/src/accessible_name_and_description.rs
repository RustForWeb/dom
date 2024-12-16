use std::rc::Rc;

use regex::Regex;
use web_sys::{
    wasm_bindgen::JsCast, window, CssStyleDeclaration, Element, HtmlFieldSetElement,
    HtmlInputElement, HtmlLabelElement, HtmlLegendElement, HtmlOptGroupElement, HtmlSelectElement,
    HtmlSlotElement, HtmlTableCaptionElement, HtmlTableElement, HtmlTextAreaElement, Node,
    SvgElement, SvgTitleElement,
};

use crate::util::{
    array_to_vec, has_any_concrete_roles, html_collection_to_vec, node_list_to_vec, query_id_refs,
    PRESENTATION_ROLES,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Compute {
    Name,
    Description,
}

pub type GetComputedStyle = Rc<dyn Fn(&Element, Option<&str>) -> CssStyleDeclaration>;

/// Options for [`compute_text_alternative`].
#[derive(Clone, Default)]
pub struct ComputeTextAlternativeOptions {
    pub compute: Option<Compute>,

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
            let style = get_computed_style_implementation(element, None);

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

fn query_selector_all_subtree(element: &Element, selectors: &str) -> Vec<Element> {
    let mut elements = node_list_to_vec(
        element
            .query_selector_all(selectors)
            .expect("Element should be queried."),
    );

    for root in query_id_refs(element, "aria-owns") {
        elements.extend(node_list_to_vec(
            root.query_selector_all(selectors)
                .expect("Element should be queried."),
        ));
    }

    elements
}

fn query_selected_options(listbox: &Element) -> Vec<Element> {
    if let Some(select_element) = listbox.dyn_ref::<HtmlSelectElement>() {
        html_collection_to_vec(select_element.selected_options())
    } else {
        query_selector_all_subtree(listbox, "[aria-selected=\"true\"]")
    }
}

fn is_marked_presentational(node: &Node) -> bool {
    has_any_concrete_roles(node, PRESENTATION_ROLES.into())
}

fn is_native_host_language_text_alternative_element(node: &Node) -> bool {
    // Elements specifically listed in html-aam.
    // We don't need this for `label` or `legend` elements. Their implicit roles already allow "naming from content".
    //
    // https://w3c.github.io/html-aam/#table-element
    node.is_instance_of::<HtmlTableCaptionElement>()
}

fn allows_name_from_content(node: &Node) -> bool {
    has_any_concrete_roles(
        node,
        vec![
            "button",
            "cell",
            "checkbox",
            "columnheader",
            "gridcell",
            "heading",
            "label",
            "legend",
            "link",
            "menuitem",
            "menuitemcheckbox",
            "menuitemradio",
            "option",
            "radio",
            "row",
            "rowheader",
            "switch",
            "tab",
            "tooltip",
            "treeitem",
        ],
    )
}

// TODO: https://github.com/eps1lon/dom-accessibility-api/issues/100
fn is_descendant_of_native_host_language_text_alternative_element(_node: &Node) -> bool {
    false
}

fn get_value_of_textbox(element: &Element) -> String {
    if let Some(input_element) = element.dyn_ref::<HtmlInputElement>() {
        input_element.value()
    } else if let Some(text_area_element) = element.dyn_ref::<HtmlTextAreaElement>() {
        text_area_element.value()
    } else {
        // https://github.com/eps1lon/dom-accessibility-api/issues/4
        element.text_content().unwrap_or("".into())
    }
}

fn get_textual_content(declaration: CssStyleDeclaration) -> String {
    let content = declaration
        .get_property_value("content")
        .expect("CssStyleDeclaration should have content.");
    if Regex::new(r#"^["'].*["']$"#)
        .expect("Regex should be valid.")
        .is_match(&content)
    {
        (&content[1..content.len() - 1]).into()
    } else {
        "".into()
    }
}

// https://html.spec.whatwg.org/multipage/forms.html#category-label
// TODO: form-associated custom elements
fn is_labelable_element(element: &Element) -> bool {
    let local_name = element.local_name();

    local_name == "button"
        || (local_name == "input" && element.get_attribute("type") != Some("hidden".into()))
        || local_name == "meter"
        || local_name == "output"
        || local_name == "progress"
        || local_name == "select"
        || local_name == "textarea"
}

// > [...], then the first such descendant in tree order is the label element's labeled control.
// https://html.spec.whatwg.org/multipage/forms.html#labeled-control
fn find_labelable_element(element: &Element) -> Option<Element> {
    if is_labelable_element(element) {
        return Some(element.clone());
    }

    for child_node in node_list_to_vec::<Node>(element.child_nodes()) {
        if let Some(child_element) = child_node.dyn_ref::<Element>() {
            let descendant_labelable_element = find_labelable_element(child_element);
            if let Some(descendant_labelable_element) = descendant_labelable_element {
                return Some(descendant_labelable_element);
            }
        }
    }

    None
}

// Polyfill of HTMLLabelElement.control
// https://html.spec.whatwg.org/multipage/forms.html#labeled-control
fn get_control_of_label(label: &HtmlLabelElement) -> Option<Element> {
    if let Some(control) = label.control() {
        return Some(control.into());
    }

    let html_for = label.get_attribute("for");
    if let Some(html_for) = html_for {
        return label
            .owner_document()
            .expect("Owner document should exist.")
            .get_element_by_id(&html_for);
    }

    find_labelable_element(label)
}

// Polyfill of HTMLInputElement.labels
// https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/labels
fn get_labels(element: &Element) -> Vec<HtmlLabelElement> {
    if let Some(input_element) = element.dyn_ref::<HtmlInputElement>() {
        input_element
            .labels()
            .map(node_list_to_vec)
            .unwrap_or_default()
    } else if !is_labelable_element(element) {
        vec![]
    } else {
        let document = element
            .owner_document()
            .expect("Owner document should exist.");
        node_list_to_vec(
            document
                .query_selector_all("label")
                .expect("Document should be queried."),
        )
        .into_iter()
        .filter(|label| get_control_of_label(label).is_some_and(|label| label == *element))
        .collect()
    }
}

// Gets the contents of a slot used for computing the accname.
fn get_slot_contents(slot: &HtmlSlotElement) -> Vec<Node> {
    // Computing the accessible name for elements containing slots is not currently defined in the spec.
    // This implementation reflects the behavior of NVDA 2020.2/Firefox 81 and iOS VoiceOver/Safari 13.6.

    let assigned_nodes = slot.assigned_nodes();
    if assigned_nodes.length() == 0 {
        // If no nodes are assigned to the slot, it displays the default content.
        node_list_to_vec(slot.child_nodes())
    } else {
        array_to_vec(assigned_nodes)
    }
}

struct ComputeTextAlternativeContext {
    is_embedded_in_label: bool,
    is_referenced: bool,
    recursion: bool,
}

struct ComputeMiscTextAlternativeContext {
    is_embedded_in_label: bool,
    #[expect(dead_code)]
    is_referenced: bool,
}

// Implements <https://w3c.github.io/accname/#mapping_additional_nd_te>.
pub fn compute_text_alternative(root: &Element, options: ComputeTextAlternativeOptions) -> String {
    let mut consulted_nodes: Vec<Node> = vec![];

    let compute = options.compute.unwrap_or(Compute::Name);
    let uncached_get_computed_style = options.get_computed_style.unwrap_or_else(|| {
        Rc::new(|element, pseudo_elt| {
            let window = window().expect("Window should exist.");

            if let Some(pseudo_elt) = pseudo_elt {
                window.get_computed_style_with_pseudo_elt(element, pseudo_elt)
            } else {
                window.get_computed_style(element)
            }
            .expect("Element should be valid.")
            .expect("Computed style should exist.")
        })
    });
    let hidden = options.hidden.unwrap_or(false);

    let get_computed_style: GetComputedStyle = Rc::new({
        let uncached_get_computed_style = uncached_get_computed_style.clone();

        move |element, pseudo_elt| {
            // TODO: cache
            uncached_get_computed_style(element, pseudo_elt)
        }
    });

    // 2F.i
    fn compute_misc_text_alternative(
        compute: Compute,
        hidden: bool,
        uncached_get_computed_style: GetComputedStyle,
        get_computed_style: GetComputedStyle,
        consulted_nodes: &mut Vec<Node>,
        node: &Node,
        context: ComputeMiscTextAlternativeContext,
    ) -> String {
        let mut accumalated_text = "".to_string();

        if let Some(element) = node.dyn_ref::<Element>() {
            let pseudo_before = uncached_get_computed_style(element, Some("::before"));
            let before_content = get_textual_content(pseudo_before);
            accumalated_text = format!("{before_content} {accumalated_text}");
        }

        // FIXME: Including aria-owns is not defined in the spec, but it is required in the web-platform-test.
        let child_nodes = node
            .dyn_ref::<HtmlSlotElement>()
            .map(get_slot_contents)
            .unwrap_or_else(|| {
                let mut nodes = node_list_to_vec(node.child_nodes());
                nodes.extend(
                    query_id_refs(node, "aria-owns")
                        .into_iter()
                        .map(|element| element.into()),
                );
                nodes
            });
        for child in child_nodes {
            let result = inner_compute_text_alternative(
                compute,
                hidden,
                uncached_get_computed_style.clone(),
                get_computed_style.clone(),
                consulted_nodes,
                &child,
                ComputeTextAlternativeContext {
                    is_embedded_in_label: context.is_embedded_in_label,
                    is_referenced: false,
                    recursion: true,
                },
            );
            // TODO: Unclear why display affects delimiter, see https://github.com/w3c/accname/issues/3.
            let display = if let Some(element) = child.dyn_ref::<Element>() {
                get_computed_style(element, None)
                    .get_property_value("display")
                    .expect("Computed style should have display.")
            } else {
                "inline".into()
            };
            let separator = if display != "inline" { " " } else { "" };
            // Trailing separator for WPT tests.
            accumalated_text = format!("{accumalated_text}{separator}{result}{separator}");
        }

        if let Some(element) = node.dyn_ref::<Element>() {
            let pseudo_after = uncached_get_computed_style(element, Some("::after"));
            let after_content = get_textual_content(pseudo_after);
            accumalated_text = format!("{accumalated_text} {after_content}");
        }

        accumalated_text.trim().into()
    }

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

    fn compute_tooltip_attribute_value(
        consulted_nodes: &mut Vec<Node>,
        node: &Node,
    ) -> Option<String> {
        node.dyn_ref::<Element>()
            .and_then(|element| use_attribute(consulted_nodes, element, "title"))
    }

    fn compute_element_text_alternative(
        compute: Compute,
        hidden: bool,
        uncached_get_computed_style: GetComputedStyle,
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
                            uncached_get_computed_style,
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
                            uncached_get_computed_style,
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

            let labels = get_labels(element);
            if !labels.is_empty() {
                consulted_nodes.push(node.clone());

                return Some(
                    labels
                        .into_iter()
                        .map(|element| {
                            inner_compute_text_alternative(
                                compute,
                                hidden,
                                uncached_get_computed_style.clone(),
                                get_computed_style.clone(),
                                consulted_nodes,
                                &element,
                                ComputeTextAlternativeContext {
                                    is_embedded_in_label: true,
                                    is_referenced: false,
                                    recursion: true,
                                },
                            )
                        })
                        .filter(|label| !label.is_empty())
                        .collect::<Vec<_>>()
                        .join(" "),
                );
            }

            // https://w3c.github.io/html-aam/#input-type-image-accessible-name-computation
            // TODO: WPT test consider label elements but html-aam does not mention them.
            // We follow existing implementations over spec.
            if let Some(input_element) = node.dyn_ref::<HtmlInputElement>() {
                if input_element.type_() == "image" {
                    let name_for_alt = use_attribute(consulted_nodes, input_element, "alt");
                    if let Some(name_for_alt) = name_for_alt {
                        return Some(name_for_alt);
                    }

                    let name_for_title = use_attribute(consulted_nodes, input_element, "title");
                    if let Some(name_for_alt) = name_for_title {
                        return Some(name_for_alt);
                    }

                    // TODO: l10n
                    return Some("Submit Query".into());
                }
            }

            if has_any_concrete_roles(node, vec!["button"]) {
                // https://www.w3.org/TR/html-aam-1.0/#button-element
                let name_from_sub_tree = compute_misc_text_alternative(
                    compute,
                    hidden,
                    uncached_get_computed_style,
                    get_computed_style,
                    consulted_nodes,
                    node,
                    ComputeMiscTextAlternativeContext {
                        is_embedded_in_label: false,
                        is_referenced: false,
                    },
                );
                if !name_from_sub_tree.is_empty() {
                    return Some(name_from_sub_tree);
                }
            }
        }

        None
    }

    fn inner_compute_text_alternative(
        compute: Compute,
        hidden: bool,
        uncached_get_computed_style: GetComputedStyle,
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
                                uncached_get_computed_style.clone(),
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
                    uncached_get_computed_style.clone(),
                    get_computed_style.clone(),
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
            if has_any_concrete_roles(current, vec!["combobox", "listbox"]) {
                consulted_nodes.push(current.clone());

                let selected_options = query_selected_options(
                    current
                        .dyn_ref::<Element>()
                        .expect("Node should be an Element."),
                );
                if selected_options.is_empty() {
                    // Defined per test `name_heading_combobox`.
                    return current
                        .dyn_ref::<HtmlInputElement>()
                        .map(|input_element| input_element.value())
                        .unwrap_or("".into());
                }
                return selected_options
                    .iter()
                    .map(|selected_option| {
                        inner_compute_text_alternative(
                            compute,
                            hidden,
                            uncached_get_computed_style.clone(),
                            get_computed_style.clone(),
                            consulted_nodes,
                            selected_option,
                            ComputeTextAlternativeContext {
                                is_embedded_in_label: context.is_embedded_in_label,
                                is_referenced: false,
                                recursion: true,
                            },
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
            }
            if has_abstract_role(current, "range") {
                consulted_nodes.push(current.clone());
                let element = current
                    .dyn_ref::<Element>()
                    .expect("Node should be an Element.");
                if element.has_attribute("aria-valuetext") {
                    return element
                        .get_attribute("aria-valuetext")
                        .expect("Attribute should exist.");
                }
                if element.has_attribute("aria-valuenow") {
                    return element
                        .get_attribute("aria-valuenow")
                        .expect("Attribute should exist.");
                }
                return element.get_attribute("value").unwrap_or("".into());
            }
            if has_any_concrete_roles(current, vec!["textbox"]) {
                consulted_nodes.push(current.clone());

                return get_value_of_textbox(
                    current
                        .dyn_ref::<Element>()
                        .expect("Node should be an Element."),
                );
            }
        }

        // 2F
        if allows_name_from_content(current)
            || (current.is_instance_of::<Element>() && context.is_referenced)
            || is_native_host_language_text_alternative_element(current)
            || is_descendant_of_native_host_language_text_alternative_element(current)
        {
            let accumulated_text_2f = compute_misc_text_alternative(
                compute,
                hidden,
                uncached_get_computed_style.clone(),
                get_computed_style.clone(),
                consulted_nodes,
                current,
                ComputeMiscTextAlternativeContext {
                    is_embedded_in_label: context.is_embedded_in_label,
                    is_referenced: false,
                },
            );
            if !accumulated_text_2f.is_empty() {
                consulted_nodes.push(current.clone());
                return accumulated_text_2f;
            }
        }

        if current.node_type() == Node::TEXT_NODE {
            consulted_nodes.push(current.clone());
            return current.text_content().unwrap_or("".into());
        }

        if context.recursion {
            consulted_nodes.push(current.clone());
            return compute_misc_text_alternative(
                compute,
                hidden,
                uncached_get_computed_style,
                get_computed_style,
                consulted_nodes,
                current,
                ComputeMiscTextAlternativeContext {
                    is_embedded_in_label: context.is_embedded_in_label,
                    is_referenced: false,
                },
            );
        }

        let tooltip_attribute_value = compute_tooltip_attribute_value(consulted_nodes, current);
        if let Some(tooltip_attribute_value) = tooltip_attribute_value {
            consulted_nodes.push(current.clone());
            return tooltip_attribute_value;
        }

        // TODO: should this be reachable?
        consulted_nodes.push(current.clone());
        "".into()
    }

    as_flat_string(inner_compute_text_alternative(
        compute,
        hidden,
        uncached_get_computed_style,
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
