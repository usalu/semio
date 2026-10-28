//! 🧊 Borrowed complete declaration metadata has no request-time parser, allocator or lazy initializer.
#[derive(Debug)]
pub struct StaticTypeExpression {
    pub kind: &'static str,
    pub name: Option<&'static str>,
    pub width: Option<i64>,
    pub fixed: bool,
    pub aggregate: Option<&'static str>,
    pub lower: Option<i64>,
    pub upper: Option<i64>,
    pub lower_expression: Option<&'static str>,
    pub upper_expression: Option<&'static str>,
    pub unique: bool,
    pub optional: bool,
    pub item: Option<&'static StaticTypeExpression>,
}
#[derive(Debug)]
pub struct StaticAttribute {
    pub name: &'static str,
    pub type_expr: StaticTypeExpression,
    pub optional: bool,
    pub derived: bool,
    pub declared_by: &'static str,
}
#[derive(Debug)]
pub struct StaticDerived {
    pub name: &'static str,
    pub type_expr: StaticTypeExpression,
    pub redeclares: Option<&'static str>,
    pub expression: &'static str,
}
#[derive(Debug)]
pub struct StaticEntity {
    pub key: &'static str,
    pub name: &'static str,
    pub abstract_entity: bool,
    pub supertype: Option<&'static str>,
    pub supertype_expression: &'static str,
    pub attributes: &'static [StaticAttribute],
    pub all_attributes: &'static [StaticAttribute],
    pub derived: &'static [StaticDerived],
    pub inverse: &'static [&'static str],
    pub where_rules: &'static [&'static str],
    pub unique_rules: &'static [&'static str],
}
#[derive(Debug)]
pub struct StaticType {
    pub key: &'static str,
    pub name: &'static str,
    pub kind: &'static str,
    pub members: &'static [&'static str],
    pub type_expr: Option<&'static StaticTypeExpression>,
    pub where_rules: &'static [&'static str],
}
include!("metadata-data.rs");
include!("select-parents.rs");

fn normalized(name: &str, key: &mut [u8; 128]) -> Option<usize> {
    if name.len() > key.len() || !name.is_ascii() {
        return None;
    }
    for (slot, byte) in key.iter_mut().zip(name.bytes()) {
        *slot = byte.to_ascii_uppercase();
    }
    Some(name.len())
}

/// 📖 Resolve a released entity with bounded stack normalization and binary slice lookup.
pub fn entity(name: &str) -> Option<&'static StaticEntity> {
    let mut key = [0u8; 128];
    let length = normalized(name, &mut key)?;
    ENTITIES.binary_search_by(|entity| entity.key.as_bytes().cmp(&key[..length])).ok().map(|index| &ENTITIES[index])
}

/// 📖 Resolve a released type without heap initialization.
pub fn defined_type(name: &str) -> Option<&'static StaticType> {
    let mut key = [0u8; 128];
    let length = normalized(name, &mut key)?;
    TYPES.binary_search_by(|declaration| declaration.key.as_bytes().cmp(&key[..length])).ok().map(|index| &TYPES[index])
}

/// 🧵 Borrow direct SELECT parents through bounded normalization and immutable binary lookup.
pub fn select_parents(name: &str) -> &'static [&'static str] {
    let mut key = [0u8; 128];
    let Some(length) = normalized(name, &mut key) else { return &[] };
    SELECT_PARENTS.binary_search_by(|(name, _)| name.as_bytes().cmp(&key[..length])).ok().map(|index| SELECT_PARENTS[index].1).unwrap_or(&[])
}

/// 🌳 Walk the released single-supertype chain without allocating inherited declarations.
pub fn is_subtype(actual: &str, expected: &str) -> bool {
    let Some(mut actual) = entity(actual) else { return false };
    for _ in 0..32 {
        if actual.key.eq_ignore_ascii_case(expected) {
            return true;
        }
        let Some(parent) = actual.supertype.and_then(entity) else { return false };
        actual = parent;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::select_parents;

    #[test]
    fn select_parent_lookup_normalizes_only_bounded_ascii_release_names() {
        for name in ["IFCCOLOURSPECIFICATION", "IfcColourSpecification", "ifccolourspecification"] {
            assert_eq!(select_parents(name), &["IFCCOLOUR"]);
        }
        for name in ["", "IFCUNKNOWN", "IFC4X3_ADD2.IFCLABEL", "FOREIGN.IFCLABEL", "IFCLABÉL", " IFCLABEL", "IFCLABEL "] {
            assert!(select_parents(name).is_empty(), "{name}");
        }
        assert!(select_parents(&"A".repeat(128)).is_empty());
        assert!(select_parents(&"A".repeat(129)).is_empty());
        assert_eq!(select_parents("ifclabel"), &["IFCSIMPLEVALUE"]);
    }
}
