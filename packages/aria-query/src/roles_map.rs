use std::{hash::RandomState, sync::LazyLock};

use ordered_hash_map::OrderedHashMap;

use crate::{
    etc::roles::{
        r#abstract::ARIA_ABSTRACT_ROLES, dpub::ARIA_DPUB_ROLES, graphics::ARIA_GRAPHICS_ROLES,
        literal::ARIA_LITERAL_ROLES,
    },
    types::{AriaRoleDefinition, AriaRoleDefinitionKey, AriaRoleDefinitionSuperClass},
};

pub static ROLES: LazyLock<OrderedHashMap<AriaRoleDefinitionKey, AriaRoleDefinition, RandomState>> =
    LazyLock::new(|| {
        let role_definitions = (*ARIA_ABSTRACT_ROLES)
            .clone()
            .into_iter()
            .chain(ARIA_LITERAL_ROLES.clone())
            .chain(ARIA_DPUB_ROLES.clone())
            .chain(ARIA_GRAPHICS_ROLES.clone());

        let immutable_roles = OrderedHashMap::from_iter(role_definitions);
        let mut roles = immutable_roles.clone();

        for role_definition in roles.values_mut() {
            for super_classes in &role_definition.super_class {
                for super_class in super_classes {
                    let super_class_role =
                        immutable_roles.iter().find(|(name, _)| match super_class {
                            AriaRoleDefinitionSuperClass::AbstractRole(role) => {
                                **name == AriaRoleDefinitionKey::from(*role)
                            }
                            AriaRoleDefinitionSuperClass::Role(role) => {
                                **name == AriaRoleDefinitionKey::from(*role)
                            }
                        });

                    if let Some((_, super_class_definition)) = super_class_role {
                        for (prop, prop_value) in &super_class_definition.props {
                            if !role_definition.props.contains_key(prop) {
                                role_definition.props.insert(*prop, prop_value.clone());
                            }
                        }
                    }
                }
            }
        }

        roles
    });
