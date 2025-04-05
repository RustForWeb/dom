// TODO: remove
#![allow(dead_code, unused_variables)]

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    sync::atomic::AtomicU64,
};

use web_sys::{Element, HtmlElement, Node, NodeList, ShadowRoot, wasm_bindgen::JsCast};

// Element is not Send and WebAssembly is single threaded, so this should be fine for now.
thread_local! {
    static COUNTER_MAP: RefCell<HashMap<Element, u64>> = RefCell::new(HashMap::new());
    static UNCONTROLLED_NODES: RefCell<HashMap<Element, bool>> = RefCell::new(HashMap::new());
    static MARKER_MAP: RefCell<HashMap<String, HashMap<Element, u64>>> = RefCell::new(HashMap::new());
    static LOCK_COUNT: AtomicU64 = const { AtomicU64::new(0) };
}

type Undo = Box<dyn Fn()>;

fn get_default_parent(original_target: &[Element]) -> Option<HtmlElement> {
    original_target
        .first()
        .expect("At least one element is required.")
        .owner_document()
        .and_then(|document| document.body())
}

#[derive(Clone, Debug)]
enum NodeOrShadowRoot {
    Node(Node),
    ShadowRoot(ShadowRoot),
}

impl From<Node> for NodeOrShadowRoot {
    fn from(value: Node) -> Self {
        NodeOrShadowRoot::Node(value)
    }
}

impl From<ShadowRoot> for NodeOrShadowRoot {
    fn from(value: ShadowRoot) -> Self {
        NodeOrShadowRoot::ShadowRoot(value)
    }
}

fn unwrap_host(node: NodeOrShadowRoot) -> Option<Element> {
    match node {
        NodeOrShadowRoot::Node(node) => match node.parent_node() {
            None => None,
            Some(parent_node) => unwrap_host(NodeOrShadowRoot::Node(parent_node)),
        },
        NodeOrShadowRoot::ShadowRoot(shadow_root) => Some(shadow_root.host()),
    }
}

fn correct_targets(parent: HtmlElement, targets: Vec<Element>) -> Vec<Element> {
    targets
        .into_iter()
        .filter_map(|target| {
            if parent.contains(Some(&target)) {
                return Some(target);
            }

            if let Some(corrected_target) = unwrap_host(NodeOrShadowRoot::Node((*target).clone())) {
                if parent.contains(Some(&corrected_target)) {
                    return Some(corrected_target);
                }
            }

            None
        })
        .collect()
}

/// Marks everything except given nodes with an attribute.
///
/// Parameters:
/// * `original_target`: Elements to keep on the page.
/// * `parent_node`: Top element, defaults to document.body.
/// * `marker_name`:  A special attribute to mark every node.
/// * `control_attribute`: Attribute to control.
///
/// Return:
/// Undo command.
fn apply_attribute_to_others(
    original_target: Vec<Element>,
    parent_node: HtmlElement,
    marker_name: &str,
    control_attribute: &str,
) -> Undo {
    let targets = correct_targets(parent_node, original_target);

    MARKER_MAP.with_borrow_mut(|marker_map| {
        if !marker_map.contains_key(marker_name) {
            marker_map.insert(marker_name.into(), HashMap::new());
        }
    });

    let hidden_nodes: Vec<Element> = vec![];
    // TODO: insert doesn't exist, because Node doesn't implement Hash
    let elements_to_keep: HashSet<Node> = HashSet::new();
    let elements_to_stop: HashSet<Node> = HashSet::new();

    // TODO

    Box::new(|| {})
}

fn node_list_to_elements(node_list: NodeList) -> Vec<Element> {
    let mut elements: Vec<Element> = vec![];
    for n in 0..node_list.length() {
        if let Some(element) = node_list.item(n).and_then(|node| node.dyn_into().ok()) {
            elements.push(element);
        }
    }
    elements
}

/// Marks everything except given nodes as aria-hidden.
///
/// Parameters:
/// * `original_target`: Elements to keep on the page.
/// * `parent_node`: Top element, defaults to document.body.
/// * `marker_name`:  A special attribute to mark every node.
///
/// Return:
/// Undo command.
pub fn hide_others(
    original_target: Vec<Element>,
    parent_node: Option<HtmlElement>,
    marker_name: Option<&str>,
) -> Undo {
    let marker_name = marker_name.unwrap_or("data-aria-hidden");

    let mut targets = original_target.clone();
    let active_parent_node = parent_node.or(get_default_parent(&original_target));

    if let Some(active_parent_node) = active_parent_node {
        // We should not hide aria-live elements (https://github.com/theKashey/aria-hidden/issues/10).
        targets.extend(node_list_to_elements(
            active_parent_node
                .query_selector_all("[aria-live]")
                .expect("Element should be queried."),
        ));

        apply_attribute_to_others(targets, active_parent_node, marker_name, "aria-hidden")
    } else {
        Box::new(|| {})
    }
}

/// Marks everything except given nodes as inert.
///
/// Parameters:
/// * `original_target`: Elements to keep on the page.
/// * `parent_node`: Top element, defaults to document.body.
/// * `marker_name`:  A special attribute to mark every node.
///
/// Return:
/// Undo command.
pub fn inert_others(
    original_target: Vec<Element>,
    parent_node: Option<HtmlElement>,
    marker_name: Option<&str>,
) -> Undo {
    let marker_name = marker_name.unwrap_or("data-inert-ed");

    let active_parent_node = parent_node.or(get_default_parent(&original_target));

    if let Some(active_parent_node) = active_parent_node {
        apply_attribute_to_others(original_target, active_parent_node, marker_name, "inert")
    } else {
        Box::new(|| {})
    }
}

/// Whether the current browser supports inert.
pub fn supports_inert() -> bool {
    todo!()
}

/// Automatic function to "suppress" DOM elements - _hide_ or _inert_ in the best possible way.
///
/// Parameters:
/// * `original_target`: Elements to keep on the page.
/// * `parent_node`: Top element, defaults to document.body.
/// * `marker_name`:  A special attribute to mark every node.
///
/// Return:
/// Undo command.
pub fn supress_others(
    original_target: Vec<Element>,
    parent_node: Option<HtmlElement>,
    marker_name: Option<&str>,
) -> Undo {
    if supports_inert() {
        inert_others(original_target, parent_node, marker_name)
    } else {
        hide_others(original_target, parent_node, marker_name)
    }
}
