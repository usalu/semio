//! 🪜️ AP214 CC ladder — shared representation-type -> minimum-CC classification plus FILE_SCHEMA
//! / PRODUCT-chain scans, reused by all six `✳️ccN` subset analyzers (ISO 10303-214 §4.3
//! conformance classes: https://www.iso.org/standard/63339.html). Single source of truth: every
//! `✳️ccN`'s `check_ccN_conformance` calls into these primitives rather than re-deriving the
//! ladder or the shared base scans independently — one classification, six consumers.

use super::part21::{Part21Document, Part21Value};

//#region 🔖️Ladder
/// 🔢️ Minimum ISO 10303-214 conformance class (2..=6) a `*_SHAPE_REPRESENTATION` subtype
/// requires. Returns `None` for any instance type that isn't itself a `*_SHAPE_REPRESENTATION`
/// (case-insensitive suffix match, matching `Part21Instance::is_type`'s own convention) — those
/// aren't ladder-relevant at all. The five explicitly-classified subtypes come straight from the
/// AP214 EXPRESS schema's shape-representation hierarchy; any OTHER `*_SHAPE_REPRESENTATION`
/// instance (including the bare `SHAPE_REPRESENTATION` base type) is treated as rung 2 — the
/// minimal geometry-bearing representation CC1 (config data only) already forbids outright, so it
/// can never honestly be classified as rung 1 (there is no rung 1: CC1 means "none present").
pub fn ladder_rung_of(entity_type: &str) -> Option<u8> {
    let t = entity_type.to_ascii_uppercase();
    if !t.ends_with("SHAPE_REPRESENTATION") {
        return None;
    }
    Some(match t.as_str() {
        "GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION" => 2,
        "GEOMETRICALLY_BOUNDED_SURFACE_SHAPE_REPRESENTATION" => 3,
        "MANIFOLD_SURFACE_SHAPE_REPRESENTATION" => 4,
        "FACETED_BREP_SHAPE_REPRESENTATION" => 5,
        "ADVANCED_BREP_SHAPE_REPRESENTATION" => 6,
        _ => 2,
    })
}

/// 🔍️ Every instance in the document whose (case-insensitive) type name — primary or, for a
/// complex instance, any of its entity names — is a `*_SHAPE_REPRESENTATION` subtype, paired
/// with its ladder rung. Real scan over the full lossless Part21 graph (`Part21Document.instances`
/// via each `Part21Instance.entities`), never fabricated against an unmodeled field.
pub fn shape_representation_instances(doc: &Part21Document) -> Vec<(u64, String, u8)> {
    let mut out = Vec::new();
    for inst in &doc.instances {
        for (name, _) in &inst.entities {
            if let Some(rung) = ladder_rung_of(name) {
                out.push((inst.id, name.clone(), rung));
            }
        }
    }
    out
}

/// 🚧️ The subset of `shape_representation_instances` whose rung exceeds `max_rung` — the exact
/// HARD-flag set every `✳️ccN` analyzer reports (CC1 passes `max_rung = 1`, which every real
/// rung of 2..=6 exceeds, matching "CC1 allows no `*_SHAPE_REPRESENTATION` instance at all").
pub fn ladder_violations(doc: &Part21Document, max_rung: u8) -> Vec<(u64, String, u8)> {
    shape_representation_instances(doc).into_iter().filter(|(_, _, rung)| *rung > max_rung).collect()
}
//#endregion 🔖️Ladder

//#region 🔖️BaseChecks
/// 🏷️ Real, recursive scan: does the `FILE_SCHEMA` header record's argument tree contain the
/// given schema name (e.g. `AUTOMOTIVE_DESIGN`) as a string literal anywhere inside its nested
/// lists? `Part21Header.file_schema` is genuinely `FILE_SCHEMA(('AUTOMOTIVE_DESIGN'))`'s parsed
/// arg list — a list wrapping a list wrapping the schema-name string — so this walks the real
/// retained structure rather than assuming its exact nesting depth.
pub fn file_schema_contains(doc: &Part21Document, schema_name: &str) -> bool {
    fn walk(value: &Part21Value, schema_name: &str) -> bool {
        match value {
            Part21Value::Str(s) => s.eq_ignore_ascii_case(schema_name),
            Part21Value::List(items) => items.iter().any(|v| walk(v, schema_name)),
            Part21Value::Typed(_, items) => items.iter().any(|v| walk(v, schema_name)),
            _ => false,
        }
    }
    doc.header.file_schema.iter().any(|v| walk(v, schema_name))
}

/// 🔗️ Real scan: does the document carry at least one instance of each of AP214's core product
/// identity chain types (`PRODUCT`, `PRODUCT_DEFINITION_FORMATION`, `PRODUCT_DEFINITION`)? A
/// presence-only check (not full referential linkage) — honestly scoped to what `Part21Document`'s
/// own `by_type` alone can verify over the generic instance graph.
pub fn has_product_definition_chain(doc: &Part21Document) -> bool {
    doc.by_type("PRODUCT").next().is_some() && doc.by_type("PRODUCT_DEFINITION_FORMATION").next().is_some() && doc.by_type("PRODUCT_DEFINITION").next().is_some()
}

/// ✍️ Real mutation: forces `FILE_SCHEMA` to declare the given schema name (no-op if it already
/// does; otherwise replaces the header record outright) — the composer duty every `✳️ccN`
/// composer performs before hard-gating serialization, so a composer-built document always
/// carries a schema declaration compatible with the class it's being stamped at.
pub fn ensure_file_schema(doc: &mut Part21Document, schema_name: &str) {
    if file_schema_contains(doc, schema_name) {
        return;
    }
    doc.header.file_schema = vec![Part21Value::List(vec![Part21Value::Str(schema_name.to_string())])];
}
//#endregion 🔖️BaseChecks

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::super::part21::Part21Instance;
    use super::*;

    #[test]
    fn ladder_classifies_named_subtypes_and_defaults_others_to_rung_2() {
        assert_eq!(ladder_rung_of("GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION"), Some(2));
        assert_eq!(ladder_rung_of("GEOMETRICALLY_BOUNDED_SURFACE_SHAPE_REPRESENTATION"), Some(3));
        assert_eq!(ladder_rung_of("MANIFOLD_SURFACE_SHAPE_REPRESENTATION"), Some(4));
        assert_eq!(ladder_rung_of("FACETED_BREP_SHAPE_REPRESENTATION"), Some(5));
        assert_eq!(ladder_rung_of("ADVANCED_BREP_SHAPE_REPRESENTATION"), Some(6));
        assert_eq!(ladder_rung_of("SHAPE_REPRESENTATION"), Some(2));
        assert_eq!(ladder_rung_of("PRODUCT"), None);
    }

    #[test]
    fn ladder_violations_filters_by_max_rung() {
        let doc = Part21Document {
            instances: vec![Part21Instance { id: 1, entities: vec![("MANIFOLD_SURFACE_SHAPE_REPRESENTATION".into(), vec![])] }, Part21Instance { id: 2, entities: vec![("GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION".into(), vec![])] }],
            ..Part21Document::default()
        };
        assert_eq!(ladder_violations(&doc, 1).len(), 2, "CC1 forbids any shape representation at all");
        assert_eq!(ladder_violations(&doc, 3).len(), 1, "only the rung-4 instance exceeds CC3");
        assert!(ladder_violations(&doc, 6).is_empty());
    }

    #[test]
    fn file_schema_contains_walks_nested_list() {
        let mut doc = Part21Document::default();
        doc.header.file_schema = vec![Part21Value::List(vec![Part21Value::Str("AUTOMOTIVE_DESIGN".into())])];
        assert!(file_schema_contains(&doc, "AUTOMOTIVE_DESIGN"));
        assert!(!file_schema_contains(&doc, "IFC4"));
    }

    #[test]
    fn ensure_file_schema_injects_only_when_absent() {
        let mut doc = Part21Document::default();
        ensure_file_schema(&mut doc, "AUTOMOTIVE_DESIGN");
        assert!(file_schema_contains(&doc, "AUTOMOTIVE_DESIGN"));
        doc.header.file_schema = vec![Part21Value::List(vec![Part21Value::Str("OTHER_SCHEMA".into())])];
        ensure_file_schema(&mut doc, "OTHER_SCHEMA");
        assert!(file_schema_contains(&doc, "OTHER_SCHEMA"), "no-op path must not clobber an already-matching schema");
    }

    #[test]
    fn product_chain_requires_all_three_types() {
        let mut doc = Part21Document::default();
        assert!(!has_product_definition_chain(&doc));
        doc.instances.push(Part21Instance { id: 1, entities: vec![("PRODUCT".into(), vec![])] });
        assert!(!has_product_definition_chain(&doc));
        doc.instances.push(Part21Instance { id: 2, entities: vec![("PRODUCT_DEFINITION_FORMATION".into(), vec![])] });
        assert!(!has_product_definition_chain(&doc));
        doc.instances.push(Part21Instance { id: 3, entities: vec![("PRODUCT_DEFINITION".into(), vec![])] });
        assert!(has_product_definition_chain(&doc));
    }
}
//#endregion 🧪️Tests
