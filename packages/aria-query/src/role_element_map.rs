use std::{hash::RandomState, sync::LazyLock};

use ordered_hash_map::OrderedHashMap;

use crate::{AriaRoleRelationConcept, roles_map::ROLES, types::AriaRoleDefinitionKey};

pub static ROLE_ELEMENTS: LazyLock<
    OrderedHashMap<AriaRoleDefinitionKey, Vec<AriaRoleRelationConcept>, RandomState>,
> = LazyLock::new(|| {
    let mut role_elements = OrderedHashMap::with_hasher(RandomState::new());

    for (key, role) in ROLES.iter() {
        let mut relation_concepts: Vec<AriaRoleRelationConcept> = vec![];

        let concepts = role
            .base_concepts
            .iter()
            .chain(role.related_concepts.iter());

        for relation in concepts {
            if relation.module == Some("HTML".into())
                && let Some(concept) = &relation.concept
            {
                relation_concepts.push(concept.clone());
            }
        }

        if !relation_concepts.is_empty() {
            role_elements.insert(*key, relation_concepts);
        }
    }

    role_elements
});
