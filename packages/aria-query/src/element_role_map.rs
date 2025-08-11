use std::{collections::HashMap, sync::LazyLock};

use crate::{AriaRoleRelationConcept, roles_map::ROLES, types::AriaRoleDefinitionKey};

pub static ELEMENT_ROLES: LazyLock<HashMap<AriaRoleRelationConcept, Vec<AriaRoleDefinitionKey>>> =
    LazyLock::new(|| {
        let mut element_roles = HashMap::new();

        for (key, role) in ROLES.iter() {
            let concepts = role
                .base_concepts
                .iter()
                .chain(role.related_concepts.iter());

            for relation in concepts {
                if relation.module == Some("HTML".into())
                    && let Some(concept) = &relation.concept
                {
                    let element_role_relation = element_roles.get(concept);
                    let mut roles: Vec<AriaRoleDefinitionKey> = vec![];

                    if let Some(element_role_relation) = element_role_relation {
                        roles.extend(element_role_relation);
                    }

                    let mut is_unique = true;
                    for role in &roles {
                        if *role == *key {
                            is_unique = false;
                            break;
                        }
                    }
                    if is_unique {
                        roles.push(*key);
                    }

                    if element_role_relation.is_none() {
                        element_roles.insert(concept.clone(), roles);
                    }
                }
            }
        }

        element_roles
    });
