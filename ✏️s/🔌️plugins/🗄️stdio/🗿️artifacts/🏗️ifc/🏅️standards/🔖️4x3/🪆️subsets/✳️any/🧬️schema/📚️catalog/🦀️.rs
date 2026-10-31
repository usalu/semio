//! 📚 IFC4X3_ADD2 EXPRESS declarations and structural validation.
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;
pub mod query;
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcEntity, IfcSnapshot, IfcValue};

use semio_s_artifact_stdio_ifc_declarations as declaration;
pub use declaration::{IfcCatalog,IfcTypeExpression,IfcAttributeDeclaration,IfcDerivedDeclaration,IfcEntityDeclaration,IfcTypeDeclaration};
#[path="../../../../../../📚️catalog/⚖️domain/🦀️.rs"]
mod domain;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IfcDiagnostic {
    pub code: String,
    pub message: String,
    pub entity_id: Option<u64>,
    pub path: Vec<usize>,
}

/// 📏️ Largest inherited serialized tuple in this pinned normative profile.
pub const IFC4X3_MAXIMUM_ATTRIBUTES: usize = 21;
use semio_s_artifact_stdio_ifc4x3_add2_catalog::build_catalog;
static IFC4X3_CATALOG: OnceLock<IfcCatalog> = OnceLock::new();

/// 📖 Resolve the complete retained catalog without a runtime parser or external dependency.
pub fn ifc4x3_catalog() -> &'static IfcCatalog {
    IFC4X3_CATALOG.get_or_init(build_catalog)
}

/// 🧊 Borrow an existing immutable catalog without causing cold heap construction inside a retained request.
pub fn ifc4x3_cached_catalog() -> Option<&'static IfcCatalog> { IFC4X3_CATALOG.get() }

/// 🧬 Resolve inherited serialization positions, including derived redeclarations.
pub fn ifc_attribute_declarations(name:&str)->Option<Vec<IfcAttributeDeclaration>>{declaration::ifc_attributes(ifc4x3_catalog(),name).map(<[IfcAttributeDeclaration]>::to_vec)}

/// 🌳 Test the declared single-supertype inheritance hierarchy.
pub fn ifc_is_subtype(actual:&str,expected:&str)->bool{let catalog=ifc4x3_catalog();let Some(mut entity)=ifc_entity_declaration(actual)else{return false;};for _ in 0..catalog.entities.len(){if entity.name.eq_ignore_ascii_case(expected){return true;}let Some(parent)=entity.supertype.as_deref().and_then(|name|catalog.entities.get(name))else{return false;};entity=parent;}false}

/// 📖 Borrow a released entity without allocating a normalized key or copied attribute vector.
pub fn ifc_entity_declaration(name:&str)->Option<&'static IfcEntityDeclaration>{let mut key=[0u8;128];if name.len()>key.len(){return None;}for(slot,byte)in key.iter_mut().zip(name.bytes()){*slot=byte.to_ascii_uppercase();}ifc4x3_catalog().entities.get(std::str::from_utf8(&key[..name.len()]).ok()?)}

fn member_accepts_type(member: &str, actual: &str, depth: usize) -> bool {
    if depth > 64 { return false; }
    if member == actual || ifc_is_subtype(actual, member) { return true; }
    ifc4x3_catalog().types.get(member).is_some_and(|declaration| declaration.kind == "select" && declaration.members.iter().any(|item| member_accepts_type(item, actual, depth + 1)))
}

fn named_expression(name: &str) -> IfcTypeExpression {
    IfcTypeExpression { kind: "named".into(), name: Some(name.to_ascii_uppercase()), ..Default::default() }
}

fn matches_value(value:&IfcValue,expression:&IfcTypeExpression,entities:&BTreeMap<u64,&IfcEntity>,_depth:usize)->bool{
    let mut tick=||Ok(());
    let mut reference=|id:u64,expected:&str,_tick:&mut dyn FnMut()->Result<(),String>|Ok(entities.get(&id).is_none_or(|entity| std::iter::once(entity.name.as_str()).chain(entity.complex.iter().map(|part|part.name.as_str())).any(|name|ifc_is_subtype(name,expected))));
    domain::check_value(ifc4x3_catalog(),expression,value,false,false,&mut domain::DomainControl{tick:&mut tick,reference:&mut reference}).is_ok()
}

fn report(diagnostics: &mut Vec<IfcDiagnostic>, code: &str, message: impl Into<String>, id: Option<u64>, path: &[usize]) {
    diagnostics.push(IfcDiagnostic { code: code.into(), message: message.into(), entity_id: id, path: path.to_vec() });
}

fn check_bounds(value: &IfcValue, expression: &IfcTypeExpression, id: u64, path: &[usize], diagnostics: &mut Vec<IfcDiagnostic>, depth: usize) {
    if depth > 64 { return; }
    if expression.kind == "named" {
        if let Some(declaration) = expression.name.as_ref().and_then(|name| ifc4x3_catalog().types.get(name)) {
            if let Some(inner) = &declaration.type_expr {
                let value = match value { IfcValue::TypedValue(typed) => typed.items.first().unwrap_or(value), _ => value };
                check_bounds(value, inner, id, path, diagnostics, depth + 1);
            } else if declaration.kind == "select" {
                if let IfcValue::TypedValue(typed) = value {
                    if let Some(value) = typed.items.first() { check_bounds(value, &named_expression(&typed.name), id, path, diagnostics, depth + 1); }
                }
            }
        }
        return;
    }
    let IfcValue::Aggregate(items) = value else { return; };
    let bounded = domain::bounds(expression,items.len()) == (0,0);
    if !bounded { report(diagnostics, "aggregate-bounds", "Aggregate cardinality violates its EXPRESS bounds.", Some(id), path); }
    if (expression.unique || expression.aggregate.as_deref() == Some("SET")) && items.iter().enumerate().any(|(index,item)| items[..index].iter().any(|previous| { let mut tick=||Ok(()); let mut reference=|_:u64,_:&str,_:&mut dyn FnMut()->Result<(),String>|Ok(false); domain::equal(ifc4x3_catalog(),expression.item.as_deref().unwrap_or(expression),item,previous,&mut domain::DomainControl{tick:&mut tick,reference:&mut reference}).unwrap_or(false) })) { report(diagnostics, "aggregate-unique", "Aggregate values must be unique.", Some(id), path); }
    if let Some(inner) = &expression.item {
        for (index, item) in items.iter().enumerate() {
            let mut nested = path.to_vec();
            nested.push(index);
            check_bounds(item, inner, id, &nested, diagnostics, depth + 1);
        }
    }
}

fn check_references(value: &IfcValue, id: u64, path: &[usize], entities: &BTreeMap<u64, &IfcEntity>, diagnostics: &mut Vec<IfcDiagnostic>, depth: usize) {
    if depth > 64 { report(diagnostics, "value-depth", "Value nesting exceeds the structural limit.", Some(id), path); return; }
    let items = match value {
        IfcValue::Reference(target) if !entities.contains_key(target) => { report(diagnostics, "dangling-reference", format!("Reference #{target} has no target."), Some(id), path); return; },
        IfcValue::Aggregate(items) => items,
        IfcValue::TypedValue(typed) => &typed.items,
        _ => return,
    };
    for (index, item) in items.iter().enumerate() {
        let mut nested = path.to_vec();
        nested.push(index);
        check_references(item, id, &nested, entities, diagnostics, depth + 1);
    }
}

/// 🔎 Validate structural EXPRESS domains without evaluating WHERE, UNIQUE, inverse or contextual bound expressions.
pub fn validate_ifc4x3_snapshot(snapshot: &IfcSnapshot) -> Vec<IfcDiagnostic> {
    let mut diagnostics = Vec::new();
    let mut entities = BTreeMap::new();
    let mut seen = BTreeSet::new();
    let expected_schema = vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4X3_ADD2".into())])];
    if snapshot.header.file_schema != expected_schema { report(&mut diagnostics, "file-schema", "FILE_SCHEMA must identify IFC4X3_ADD2.", None, &[]); }
    for entity in &snapshot.entities {
        if entity.id == 0 { report(&mut diagnostics, "entity-id", "Entity identifiers must be positive unsigned 64-bit integers.", Some(entity.id), &[]); }
        if !seen.insert(entity.id) { report(&mut diagnostics, "duplicate-id", "Entity identifiers must be unique.", Some(entity.id), &[]); }
        entities.insert(entity.id, entity);
    }
    for entity in &snapshot.entities {
        for (index, value) in entity.args.iter().enumerate() { check_references(value, entity.id, &[index], &entities, &mut diagnostics, 0); }
        let Some(declaration) = ifc4x3_catalog().entities.get(&entity.name.to_ascii_uppercase()) else { report(&mut diagnostics, "unknown-entity", format!("Unknown IFC4X3_ADD2 entity {}.", entity.name), Some(entity.id), &[]); continue; };
        if !entity.complex.is_empty() { report(&mut diagnostics, "complex-instance", "Complex entity structural validation is unsupported.", Some(entity.id), &[]); continue; }
        if declaration.abstract_entity { report(&mut diagnostics, "abstract-entity", format!("{} is abstract.", declaration.name), Some(entity.id), &[]); }
        if entity.args.len() != declaration.all_attributes.len() { report(&mut diagnostics, "attribute-count", format!("Expected {} positional attributes.", declaration.all_attributes.len()), Some(entity.id), &[]); }
        for (index, attribute) in declaration.all_attributes.iter().enumerate() {
            let Some(value) = entity.args.get(index) else { continue; };
            if attribute.derived {
                if !matches!(value, IfcValue::Derived) { report(&mut diagnostics, "derived-attribute", format!("{} requires a derived marker.", attribute.name), Some(entity.id), &[index]); }
            } else if matches!(value, IfcValue::Unset) {
                if !attribute.optional { report(&mut diagnostics, "required-attribute", format!("{} is required.", attribute.name), Some(entity.id), &[index]); }
            } else if !matches_value(value, &attribute.type_expr, &entities, 0) {
                report(&mut diagnostics, "attribute-type", format!("{} violates its EXPRESS type.", attribute.name), Some(entity.id), &[index]);
            } else { check_bounds(value, &attribute.type_expr, entity.id, &[index], &mut diagnostics, 0); }
        }
    }
    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn official_declarations_and_derived_positions() {
        let catalog = ifc4x3_catalog();
        assert_eq!(catalog.entities.len(), 876);
        assert_eq!(catalog.types.len(), 436);
        assert!(ifc_is_subtype("IFCWALL", "IFCROOT"));
        assert!(ifc_attribute_declarations("IFCSIUNIT").unwrap()[0].derived);
    }

    #[test]
    fn structural_bounds_types_and_references() {
        let mut snapshot = IfcSnapshot::default();
        snapshot.header.file_schema = vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4X3_ADD2".into())])];
        snapshot.entities.push(IfcEntity { id: 1, name: "IFCCARTESIANPOINT".into(), args: vec![IfcValue::Aggregate(vec![IfcValue::Real(0.0)])], complex: vec![] });
        assert!(validate_ifc4x3_snapshot(&snapshot).is_empty());
        snapshot.entities[0].args = vec![IfcValue::Aggregate(vec![])];
        assert_eq!(validate_ifc4x3_snapshot(&snapshot)[0].code, "aggregate-bounds");
        snapshot.entities[0].args = vec![IfcValue::String("x".into())];
        assert_eq!(validate_ifc4x3_snapshot(&snapshot)[0].code, "attribute-type");
    }
}

#[cfg(test)]
mod capacity_tests {
    use super::*;
    #[test]
    fn inherited_tuple_bound_matches_the_neutral_profile_facet() {
        let fixture: semio_framework_pack_json::Value = semio_framework_pack_json::parse(include_str!("../../../../../../🔎️details/🧪️tests/capacity.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let row = &fixture["cases"][1];
        assert_eq!(IFC4X3_MAXIMUM_ATTRIBUTES as u64, row["maximumAttributes"].as_u64().unwrap());
        assert_eq!(ifc4x3_catalog().entities.values().map(|entity| entity.all_attributes.len()).max().unwrap(), IFC4X3_MAXIMUM_ATTRIBUTES);
    }
}
