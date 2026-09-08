//! 🪜️ AP214 CC ladder — shared representation-type -> minimum-CC classification plus FILE_SCHEMA
//! / PRODUCT-chain scans, reused by all six `✳️ccN` subset analyzers (ISO 10303-214 §4.3
//! conformance classes: https://www.iso.org/standard/63339.html). Single source of truth: every
//! `✳️ccN`'s `check_ccN_conformance` calls into these primitives rather than re-deriving the
//! ladder or the shared base scans independently — one classification, six consumers.

use super::part21::{Part21Document, Part21Instance, Part21Value};

//#region 🔖️Ladder
/// 🔢️ Minimum ISO 10303-214 conformance class (2..=6) a `*_SHAPE_REPRESENTATION` subtype
/// requires. Returns `None` for any instance type that isn't itself a `*_SHAPE_REPRESENTATION`
/// (case-insensitive suffix match, matching `Part21Instance::is_type`'s own convention) — those
/// aren't ladder-relevant at all. The five explicitly-classified subtypes come straight from the
/// AP214 EXPRESS schema's shape-representation hierarchy; any OTHER `*_SHAPE_REPRESENTATION`
/// instance (including the bare `SHAPE_REPRESENTATION` base type) is treated as rung 2 — the
/// minimal geometry-bearing representation CC1 (config data only) already forbids outright, so it
/// can never honestly be classified as rung 1 (there is no rung 1: CC1 means "none present").
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn file_schema_contains(doc: &Part21Document, schema_name: &str) -> bool {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn walk(value: &Part21Value, schema_name: &str) -> bool {
        match value {
            Part21Value::Str(s) => s.eq_ignore_ascii_case(schema_name),
            Part21Value::List(items) => items.iter().any(|v| walk(v, schema_name)),
            Part21Value::Typed { items, .. } => items.iter().any(|v| walk(v, schema_name)),
            _ => false,
        }
    }
    doc.header.file_schema.iter().any(|v| walk(v, schema_name))
}

/// 🏭️ `product` has no subtypes in ISO 10303-41's `product_definition_schema`, so the chain's first
/// rung is a single exact type name.
pub const PRODUCT_TYPES: &[&str] = &["PRODUCT"];

/// 🏭️ `product_definition_formation` and the one subtype ISO 10303-41 declares for it. Real AP214
/// and AP242 exporters write the SUBTYPE: entity `#822` of this artifact's own committed fixture is
/// `PRODUCT_DEFINITION_FORMATION_WITH_SPECIFIED_SOURCE`, never the bare supertype.
pub const PRODUCT_DEFINITION_FORMATION_TYPES: &[&str] = &["PRODUCT_DEFINITION_FORMATION", "PRODUCT_DEFINITION_FORMATION_WITH_SPECIFIED_SOURCE"];

/// 🏭️ `product_definition` and the one subtype ISO 10303-41 declares for it.
pub const PRODUCT_DEFINITION_TYPES: &[&str] = &["PRODUCT_DEFINITION", "PRODUCT_DEFINITION_WITH_ASSOCIATED_DOCUMENTS"];

/// 🔍️ The first instance whose (case-insensitive) type name is one of `names` — the EXPRESS
/// supertype or any of its enumerated subtypes. Subtyping is enumerated rather than inferred from
/// the name, because a name prefix is not a subtype relation in EXPRESS: `PRODUCT_DEFINITION_
/// FORMATION` begins with `PRODUCT_DEFINITION` and is a different entity entirely.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn instance_of_any<'a>(doc: &'a Part21Document, names: &[&str]) -> Option<&'a Part21Instance> {
    doc.instances.iter().find(|instance| names.iter().any(|name| instance.is_type(name)))
}

/// 🔗️ Real scan: does the document carry at least one instance of each of AP214's core product
/// identity chain rungs (`product` / `product_definition_formation` / `product_definition`), the
/// supertype or one of its ISO 10303-41 subtypes? A presence-only check (not full referential
/// linkage) — honestly scoped to what the generic instance graph alone can verify.
///
/// ⚠️ This used to match the three supertype names EXACTLY, and that was wrong against real data:
/// the committed `📐️hexagonal-cut-concrete-forest-left-ap214.stp` — a real Rhino/ST-Developer
/// export — carries `#822=PRODUCT_DEFINITION_FORMATION_WITH_SPECIFIED_SOURCE`, so every `✳️ccN`
/// analyzer reported the soft `product-definition-chain` diagnostic against a file that genuinely
/// carries the chain. The ladder half of this module already classified `*_SHAPE_REPRESENTATION`
/// subtypes; the product half did not, and only a real export showed it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn has_product_definition_chain(doc: &Part21Document) -> bool {
    instance_of_any(doc, PRODUCT_TYPES).is_some() && instance_of_any(doc, PRODUCT_DEFINITION_FORMATION_TYPES).is_some() && instance_of_any(doc, PRODUCT_DEFINITION_TYPES).is_some()
}

/// ✍️ Real mutation: forces `FILE_SCHEMA` to declare the given schema name (no-op if it already
/// does; otherwise replaces the header record outright) — the composer duty every `✳️ccN`
/// composer performs before hard-gating serialization, so a composer-built document always
/// carries a schema declaration compatible with the class it's being stamped at.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn ensure_file_schema(doc: &mut Part21Document, schema_name: &str) {
    if file_schema_contains(doc, schema_name) {
        return;
    }
    doc.header.file_schema = vec![Part21Value::List(vec![Part21Value::Str(schema_name.to_string())])];
}
//#endregion 🔖️BaseChecks

//#region 🔖️ConformanceEdits
/// 🪜️ The representation type that SITS EXACTLY on a class's ceiling rung — the most capable
/// geometry an ISO 10303-214 conformance class admits, and therefore the type a demotion rewrites an
/// over-rung instance INTO. `None` for a ceiling of 1: CC1 (config data only) admits no
/// `*_SHAPE_REPRESENTATION` at all, so it has no ceiling type and its only conformance repair is
/// deletion — which is why `1️⃣cc1`'s vocabulary carries `remove-shape-representation` where
/// `2️⃣cc2`..`5️⃣cc5` carry `demote-shape-representation`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn ceiling_type_of(max_rung: u8) -> Option<&'static str> {
    match max_rung {
        2 => Some("GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION"),
        3 => Some("GEOMETRICALLY_BOUNDED_SURFACE_SHAPE_REPRESENTATION"),
        4 => Some("MANIFOLD_SURFACE_SHAPE_REPRESENTATION"),
        5 => Some("FACETED_BREP_SHAPE_REPRESENTATION"),
        6 => Some("ADVANCED_BREP_SHAPE_REPRESENTATION"),
        _ => None,
    }
}

/// 🧱️ One `*_SHAPE_REPRESENTATION` instance as a conformance-class edit addresses it: which rung of
/// the ladder it sits on (through its type name) and the three arguments ISO 10303-42's
/// `representation` supertype gives it — `name`, `items` and `context_of_items`. Nothing more is
/// modelled, because nothing more is what a CONFORMANCE CLASS is about: the class restricts which
/// representation types may appear, not what geometry they carry.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ShapeRepresentationRow {
    pub type_name: String,
    pub name: String,
    pub items: Vec<u64>,
    pub context: Option<u64>,
}

/// 🏭️ The three instance ids AP214's product identity chain occupies, as one unit. It is one unit
/// because [`has_product_definition_chain`] is a CONJUNCTION over all three: an edit to a single
/// rung could never deterministically turn the `product-definition-chain` diagnostic on or off, so a
/// vocabulary derived from that rule addresses the triple or it addresses nothing.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ProductIdentity {
    pub product: u64,
    pub product_name: String,
    pub formation: u64,
    pub formation_id: String,
    pub definition: u64,
    pub definition_id: String,
}

/// 🏷️ The schema names `FILE_SCHEMA` declares, flattened out of its nested argument lists.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn file_schema_names(doc: &Part21Document) -> Vec<String> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn walk(value: &Part21Value, out: &mut Vec<String>) {
        match value {
            Part21Value::Str(text) => out.push(text.clone()),
            Part21Value::List(items) | Part21Value::Typed { items, .. } => items.iter().for_each(|item| walk(item, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    doc.header.file_schema.iter().for_each(|value| walk(value, &mut out));
    out
}

/// ✍️ Replaces `FILE_SCHEMA` with exactly `names`, in the `(('A','B'))` nesting Part-21 requires.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn set_file_schema_names(doc: &mut Part21Document, names: &[String]) {
    doc.header.file_schema = vec![Part21Value::List(names.iter().map(|name| Part21Value::Str(name.clone())).collect())];
}

/// 🔎️ The ladder-relevant instance `id` carries, read back as a [`ShapeRepresentationRow`], or
/// `None` when `id` is absent or is not a `*_SHAPE_REPRESENTATION` at all.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn shape_representation_row(doc: &Part21Document, id: u64) -> Option<ShapeRepresentationRow> {
    let instance = doc.instance(id)?;
    let (type_name, args) = instance.entities.iter().find(|(name, _)| ladder_rung_of(name).is_some())?;
    Some(ShapeRepresentationRow {
        type_name: type_name.clone(),
        name: args.first().and_then(Part21Value::as_str).unwrap_or_default().to_string(),
        items: args.get(1).and_then(Part21Value::as_list).map(|items| items.iter().filter_map(Part21Value::as_ref_id).collect()).unwrap_or_default(),
        context: args.get(2).and_then(Part21Value::as_ref_id),
    })
}

/// ✍️ Writes `row` at `id`, replacing whatever was there. The class ceiling is NOT checked here —
/// the guard belongs to each `✳️ccN` vocabulary, which is the thing that knows its own rung and owes
/// the caller a message naming its own class.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn upsert_shape_representation(doc: &mut Part21Document, id: u64, row: &ShapeRepresentationRow) {
    let args = vec![
        Part21Value::Str(row.name.clone()),
        Part21Value::List(row.items.iter().map(|item| Part21Value::Ref(*item)).collect()),
        match row.context {
            Some(context) => Part21Value::Ref(context),
            None => Part21Value::Unset,
        },
    ];
    let instance = Part21Instance { id, entities: vec![(row.type_name.to_ascii_uppercase(), args)] };
    match doc.instances.iter().position(|existing| existing.id == id) {
        Some(at) => doc.instances[at] = instance,
        None => doc.instances.push(instance),
    }
}

/// 🗑️ Deletes the instance at `id`, refusing anything that is not a `*_SHAPE_REPRESENTATION` — a
/// conformance repair must never delete a real geometry or product record because a scenario named
/// the wrong id.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn remove_shape_representation(doc: &mut Part21Document, id: u64) -> Result<(), String> {
    match doc.instances.iter().position(|instance| instance.id == id && instance.entities.iter().any(|(name, _)| ladder_rung_of(name).is_some())) {
        Some(at) => {
            doc.instances.remove(at);
            Ok(())
        }
        None => Err(format!("#{id} is not a *_SHAPE_REPRESENTATION instance in this document -- a ladder edit addresses the ladder, never an arbitrary entity")),
    }
}

/// ⬇️ Rewrites the representation at `id` to `ceiling`, keeping its `name`, `items` and
/// `context_of_items` exactly — the minimal edit that brings an over-rung instance INTO a class
/// without inventing or discarding geometry. Returns the type name it replaced, which is what the
/// inverse needs.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demote_shape_representation(doc: &mut Part21Document, id: u64, ceiling: &str) -> Result<String, String> {
    let row = shape_representation_row(doc, id).ok_or_else(|| format!("#{id} is not a *_SHAPE_REPRESENTATION instance in this document"))?;
    let previous = row.type_name.clone();
    upsert_shape_representation(doc, id, &ShapeRepresentationRow { type_name: ceiling.to_string(), ..row });
    Ok(previous)
}

/// 🔎️ The product identity chain the document carries, or `None` when any of its three rungs is
/// missing — the exact condition [`has_product_definition_chain`] reports on.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn product_identity(doc: &Part21Document) -> Option<ProductIdentity> {
    let text = |instance: &Part21Instance| instance.entities.first().and_then(|(_, args)| args.first()).and_then(Part21Value::as_str).unwrap_or_default().to_string();
    let product = instance_of_any(doc, PRODUCT_TYPES)?;
    let formation = instance_of_any(doc, PRODUCT_DEFINITION_FORMATION_TYPES)?;
    let definition = instance_of_any(doc, PRODUCT_DEFINITION_TYPES)?;
    Some(ProductIdentity { product: product.id, product_name: text(product), formation: formation.id, formation_id: text(formation), definition: definition.id, definition_id: text(definition) })
}

/// ✍️ Writes the whole product identity chain, or — with `None` — removes every instance of all
/// three rungs, which is the only edit that deterministically turns the soft
/// `product-definition-chain` diagnostic ON. Writing keeps each rung's supertype name, because a
/// chain this function AUTHORS has no source to specify and `product_definition_formation` is the
/// form the specification names.
///
/// ⚠️ The authored `PRODUCT` carries three of ISO 10303-41's four attributes: `frame_of_reference`
/// is omitted rather than written as the empty aggregate `()` ISO 10303-21 §6.2 permits, so that
/// this function and the AP214 reference oracle author the SAME shape. The reason is recorded where
/// it was measured — `../../../../🦀️oracle.rs`'s `set_product_identity` — and it is a
/// defect in `ruststep` 0.4, reproduced standalone: that reader cannot parse an empty aggregate as
/// an argument value at all, so emitting the spec-legal `()` would produce a document the registered
/// independent reader refuses to read back.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn set_product_identity(doc: &mut Part21Document, identity: Option<&ProductIdentity>) {
    let chain: Vec<&str> = PRODUCT_TYPES.iter().chain(PRODUCT_DEFINITION_FORMATION_TYPES).chain(PRODUCT_DEFINITION_TYPES).copied().collect();
    doc.instances.retain(|instance| !chain.iter().any(|name| instance.is_type(name)));
    let Some(identity) = identity else { return };
    let rung = |id: u64, type_name: &str, args: Vec<Part21Value>| Part21Instance { id, entities: vec![(type_name.to_string(), args)] };
    doc.instances.push(rung(identity.product, PRODUCT_TYPES[0], vec![Part21Value::Str(identity.product_name.clone()), Part21Value::Str(identity.product_name.clone()), Part21Value::Str(String::new())]));
    doc.instances.push(rung(identity.formation, PRODUCT_DEFINITION_FORMATION_TYPES[0], vec![Part21Value::Str(identity.formation_id.clone()), Part21Value::Unset, Part21Value::Ref(identity.product)]));
    doc.instances.push(rung(identity.definition, PRODUCT_DEFINITION_TYPES[0], vec![Part21Value::Str(identity.definition_id.clone()), Part21Value::Unset, Part21Value::Ref(identity.formation), Part21Value::Unset]));
    doc.instances.sort_by_key(|instance| instance.id);
}
//#endregion 🔖️ConformanceEdits

//#region 🔖️ClassEdits
/// 🎚️ The class-neutral shape of a conformance-class edit, one variant per axis
/// `check_ccN_conformance` actually reads. The six `✳️ccN` vocabularies are NOT copies of each other
/// and are not copies of this: each declares its OWN enum, carrying only the verbs its class admits
/// (`1️⃣cc1` has no way to write a representation at all; `6️⃣cc6` has nothing to demote from), and
/// then routes through here so the ONE implementation of each axis serves all six. This is the
/// family-module rule applied to a vocabulary: what is genuinely shared is shared by construction,
/// what differs per class stays in the class.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum ClassEdit {
    /// 🏷️ `CODE_FILE_SCHEMA` — the axis `file_schema_contains(doc, "AUTOMOTIVE_DESIGN")` reads.
    FileSchema { schemas: Vec<String> },
    /// 🏭️ `CODE_PRODUCT_CHAIN` — the axis [`has_product_definition_chain`] reads.
    ProductIdentity { identity: Option<ProductIdentity> },
    /// 🪜️ The ladder axis: `Some` writes a representation the class admits, `None` deletes one.
    Representation { id: u64, row: Option<ShapeRepresentationRow> },
    /// ⬇️ The ladder axis's repair verb: rewrite an over-rung representation onto the class ceiling.
    Demotion { id: u64 },
}

/// ▶️ Applies one class edit under `class`'s own ceiling. Every rejection names the class and the
/// rung, because a caller who asked CC3 to hold a faceted B-rep needs to be told which class refused
/// and why — not handed a silently unchanged document.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_class_edit(doc: &mut Part21Document, class: &str, max_rung: u8, edit: &ClassEdit) -> Result<(), String> {
    match edit {
        ClassEdit::FileSchema { schemas } => {
            if schemas.iter().all(|name| name.trim().is_empty()) {
                return Err(format!("{class} requires FILE_SCHEMA to declare a schema -- an empty declaration is not an AP214 exchange structure"));
            }
            set_file_schema_names(doc, schemas);
            Ok(())
        }
        ClassEdit::ProductIdentity { identity } => {
            set_product_identity(doc, identity.as_ref());
            Ok(())
        }
        ClassEdit::Representation { id, row: None } => remove_shape_representation(doc, *id),
        ClassEdit::Representation { id, row: Some(row) } => {
            let rung = ladder_rung_of(&row.type_name).ok_or_else(|| format!("{:?} is not a *_SHAPE_REPRESENTATION type -- {class}'s ladder verb addresses the ladder only", row.type_name))?;
            if rung > max_rung {
                return Err(format!("{:?} sits on ladder rung {rung}, above {class}'s ceiling of {max_rung} -- writing it would put the document outside the class it claims", row.type_name));
            }
            upsert_shape_representation(doc, *id, row);
            Ok(())
        }
        ClassEdit::Demotion { id } => {
            let ceiling = ceiling_type_of(max_rung).ok_or_else(|| format!("{class} admits no *_SHAPE_REPRESENTATION at all, so it has no ceiling to demote onto"))?;
            demote_shape_representation(doc, *id, ceiling).map(|_| ())
        }
    }
}

/// ↩️ The in-class inverse of `edit` against the UNMUTATED `base`, or `None` when this class has no
/// verb that can express it.
///
/// ⚠️ `None` is a real answer, not a gap. A conformance class is not closed under inversion: undoing
/// a repair RE-INTRODUCES the violation the repair removed, and a class whose whole point is to
/// forbid rung-6 geometry cannot own a verb that writes rung-6 geometry back. Each `✳️ccN`
/// vocabulary therefore degrades exactly those cases to `SetSnapshot`, and says so at the variant.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn invert_class_edit(base: &Part21Document, max_rung: u8, edit: &ClassEdit) -> Option<ClassEdit> {
    match edit {
        ClassEdit::FileSchema { .. } => Some(ClassEdit::FileSchema { schemas: file_schema_names(base) }),
        ClassEdit::ProductIdentity { .. } => Some(ClassEdit::ProductIdentity { identity: product_identity(base) }),
        ClassEdit::Representation { id, .. } | ClassEdit::Demotion { id } => match shape_representation_row(base, *id) {
            None => Some(ClassEdit::Representation { id: *id, row: None }),
            Some(row) => match ladder_rung_of(&row.type_name) {
                Some(rung) if rung <= max_rung => Some(ClassEdit::Representation { id: *id, row: Some(row) }),
                _ => None,
            },
        },
    }
}
//#endregion 🔖️ClassEdits

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
