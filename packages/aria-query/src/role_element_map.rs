use std::{collections::HashMap, sync::LazyLock};

use crate::{roles_map::ROLES, types::AriaRoleDefinitionKey, AriaRoleRelationConcept};

pub static ROLE_ELEMENTS: LazyLock<HashMap<AriaRoleDefinitionKey, Vec<AriaRoleRelationConcept>>> =
    LazyLock::new(|| {
        let mut role_elements = HashMap::new();

        for (key, role) in ROLES.iter() {
            let mut relation_concepts: Vec<AriaRoleRelationConcept> = vec![];

            let concepts = role
                .base_concepts
                .iter()
                .chain(role.related_concepts.iter());

            for relation in concepts {
                if relation.module == Some("HTML".into()) {
                    if let Some(concept) = &relation.concept {
                        relation_concepts.push(concept.clone());
                    }
                }
            }

            if !relation_concepts.is_empty() {
                role_elements.insert(*key, relation_concepts);
            }
        }

        role_elements
    });
