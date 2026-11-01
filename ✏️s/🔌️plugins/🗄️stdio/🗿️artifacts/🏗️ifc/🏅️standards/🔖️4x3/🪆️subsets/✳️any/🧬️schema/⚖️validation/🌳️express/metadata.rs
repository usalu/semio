//! 🧊 Borrowed complete declaration metadata: the retained released catalog projected once into immutable statics, so no request-time parser exists and lookups never allocate.
use semio_s_artifact_stdio_ifc4x3_add2_catalog as ifc4x3_add2_catalog;
use semio_s_artifact_stdio_ifc_declarations::{IfcAttributeDeclaration, IfcTypeExpression};
use semio_s_artifact_stdio_ifc_json_pack::Leaker;
use std::sync::OnceLock;
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
include!("select-parents.rs");

struct Tables {
    entities: &'static [StaticEntity],
    types: &'static [StaticType],
}
static TABLES: OnceLock<Tables> = OnceLock::new();

fn expression(leak: &mut Leaker, source: &IfcTypeExpression) -> StaticTypeExpression {
    StaticTypeExpression {
        kind: leak.text(&source.kind),
        name: source.name.as_deref().map(|text| leak.text(text)),
        width: source.width,
        fixed: source.fixed,
        aggregate: source.aggregate.as_deref().map(|text| leak.text(text)),
        lower: source.lower,
        upper: source.upper,
        lower_expression: source.lower_expression.as_deref().map(|text| leak.text(text)),
        upper_expression: source.upper_expression.as_deref().map(|text| leak.text(text)),
        unique: source.unique,
        optional: source.optional,
        item: source.item.as_deref().map(|item| &*Box::leak(Box::new(expression(leak, item)))),
    }
}

fn attributes(leak: &mut Leaker, sources: &[IfcAttributeDeclaration]) -> &'static [StaticAttribute] {
    Leaker::slice(sources.iter().map(|source| StaticAttribute { name: leak.text(&source.name), type_expr: expression(leak, &source.type_expr), optional: source.optional, derived: source.derived, declared_by: leak.text(&source.declared_by) }).collect())
}

fn texts(leak: &mut Leaker, sources: &[String]) -> &'static [&'static str] {
    Leaker::slice(sources.iter().map(|source| leak.text(source)).collect())
}

fn build() -> Tables {
    let catalog = ifc4x3_add2_catalog::catalog();
    let mut leak = Leaker::default();
    let entities = catalog
        .entities
        .iter()
        .map(|(key, source)| StaticEntity {
            key: leak.text(key),
            name: leak.text(&source.name),
            abstract_entity: source.abstract_entity,
            supertype: source.supertype.as_deref().map(|text| leak.text(text)),
            supertype_expression: leak.text(&source.supertype_expression),
            attributes: attributes(&mut leak, &source.attributes),
            all_attributes: attributes(&mut leak, &source.all_attributes),
            derived: Leaker::slice(source.derived.iter().map(|derived| StaticDerived { name: leak.text(&derived.name), type_expr: expression(&mut leak, &derived.type_expr), redeclares: derived.redeclares.as_deref().map(|text| leak.text(text)), expression: leak.text(&derived.expression) }).collect()),
            inverse: texts(&mut leak, &source.inverse),
            where_rules: texts(&mut leak, &source.where_rules),
            unique_rules: texts(&mut leak, &source.unique_rules),
        })
        .collect();
    let types = catalog
        .types
        .iter()
        .map(|(key, source)| StaticType {
            key: leak.text(key),
            name: leak.text(&source.name),
            kind: leak.text(&source.kind),
            members: texts(&mut leak, &source.members),
            type_expr: source.type_expr.as_ref().map(|item| &*Box::leak(Box::new(expression(&mut leak, item)))),
            where_rules: texts(&mut leak, &source.where_rules),
        })
        .collect();
    Tables { entities: Leaker::slice(entities), types: Leaker::slice(types) }
}

fn tables() -> &'static Tables {
    TABLES.get_or_init(build)
}

/// 📚 Borrow every released entity in ascending key order; the first call projects the retained catalog once.
pub fn entities() -> &'static [StaticEntity] {
    tables().entities
}

/// 📚 Borrow every released defined type in ascending key order.
pub fn types() -> &'static [StaticType] {
    tables().types
}

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
    entities().binary_search_by(|entity| entity.key.as_bytes().cmp(&key[..length])).ok().map(|index| &entities()[index])
}

/// 📖 Resolve a released type without heap initialization.
pub fn defined_type(name: &str) -> Option<&'static StaticType> {
    let mut key = [0u8; 128];
    let length = normalized(name, &mut key)?;
    types().binary_search_by(|declaration| declaration.key.as_bytes().cmp(&key[..length])).ok().map(|index| &types()[index])
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
