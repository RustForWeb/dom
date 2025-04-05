use web_sys::{
    Document, DocumentFragment, Element, HtmlCollection, Node, NodeList, js_sys::Array,
    wasm_bindgen::JsCast,
};

use crate::get_role::get_role;

pub const PRESENTATION_ROLES: [&str; 2] = ["presentation", "none"];

pub fn query_id_refs(node: &Node, attribute_name: &str) -> Vec<Element> {
    if let Some(element) = node.dyn_ref::<Element>() {
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
    } else {
        vec![]
    }
}

pub fn has_any_concrete_roles(node: &Node, roles: Vec<&str>) -> bool {
    node.dyn_ref::<Element>()
        .is_some_and(|element| get_role(element).is_some_and(|role| roles.contains(&role.as_str())))
}

pub fn array_to_vec<T: JsCast>(array: Array) -> Vec<T> {
    let mut result = Vec::with_capacity(
        array
            .length()
            .try_into()
            .expect("usize should be at least u32."),
    );
    for i in 0..array.length() {
        result.push(array.get(i).unchecked_into::<T>());
    }
    result
}

pub fn html_collection_to_vec<T: JsCast>(collection: HtmlCollection) -> Vec<T> {
    let mut result = Vec::with_capacity(
        collection
            .length()
            .try_into()
            .expect("usize should be at least u32."),
    );
    for i in 0..collection.length() {
        result.push(
            collection
                .item(i)
                .expect("Item should exist.")
                .unchecked_into::<T>(),
        );
    }
    result
}

pub fn node_list_to_vec<T: JsCast>(node_list: NodeList) -> Vec<T> {
    let mut result = Vec::with_capacity(
        node_list
            .length()
            .try_into()
            .expect("usize should be at least u32."),
    );
    for i in 0..node_list.length() {
        result.push(
            node_list
                .item(i)
                .expect("Item should exist.")
                .unchecked_into::<T>(),
        );
    }
    result
}
