use web_sys::{wasm_bindgen::JsCast, Document, DocumentFragment, Element, Node};

use crate::get_role::get_role;

pub const PRESENTATION_ROLES: [&str; 2] = ["presentation", "none"];

pub fn query_id_refs(element: &Element, attribute_name: &str) -> Vec<Element> {
    if let Some(attribute_value) = element.get_attribute(attribute_name) {
        let ids = attribute_value.split(' ');

        let root = element.get_root_node();

        type GetElementById = dyn Fn(&str) -> Option<Element>;
        let get_element_by_id: Box<GetElementById> = if root.is_instance_of::<Document>() {
            let document = root.unchecked_into::<Document>();
            Box::new(move |element_id: &str| document.get_element_by_id(element_id))
        } else if root.is_instance_of::<DocumentFragment>() {
            let document_fragment = root.unchecked_into::<DocumentFragment>();
            Box::new(move |element_id: &str| document_fragment.get_element_by_id(element_id))
        } else {
            unreachable!("Node should be Document or DocumentFragment.")
        };

        ids.filter_map(get_element_by_id).collect()
    } else {
        vec![]
    }
}

pub fn has_any_concrete_roles(node: &Node, roles: Vec<&str>) -> bool {
    node.dyn_ref::<Element>()
        .is_some_and(|element| get_role(element).is_some_and(|role| roles.contains(&role.as_str())))
}
