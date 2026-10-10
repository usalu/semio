//! 🪜️ AP214 CC ladder — shared representation-type -> minimum-CC classification plus FILE_SCHEMA
//! / PRODUCT-chain scans, reused by all six `✳️ccN` subset analyzers (ISO 10303-214 §4.3
//! conformance classes: https://www.iso.org/standard/63339.html). Single source of truth: every
//! `✳️ccN`'s `check_ccN_conformance` calls into these primitives rather than re-deriving the
//! ladder or the shared base scans independently — one classification, six consumers.

use crate::schema::diff::{StepArgAdded, StepArgModified, StepArgsDiff, StepDiff, StepEntitiesDiff, StepEntityAdded, StepEntityDiff, StepEntityModified};
use crate::schema::snapshot::{StepEntity, StepFileSchema, StepValue};
use crate::StepSnapshot;
use semio_framework_value::{FromValue, ToValue};
use semio_s_artifact_stdio_contract::editing::{edited_subtree, SnapshotEditError, SnapshotEditEvent};
use semio_s_artifact_stdio_contract::part21::{Part21Document, Part21Instance, Part21Value};

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
/// 🪜️ The representation type that SITS EXACTLY on a class's ceiling rung -- the most capable
/// geometry an ISO 10303-214 conformance class admits, and therefore the type a demotion rewrites an
/// over-rung instance INTO. `None` for a ceiling of 1: CC1 (config data only) admits no
/// `*_SHAPE_REPRESENTATION` at all, so it has no ceiling type and its only conformance repair is
/// deletion -- which is why `1️⃣cc1`'s vocabulary carries `remove-shape-representation` where
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
/// `representation` supertype gives it -- `name`, `items` and `context_of_items`. Nothing more is
/// modelled, because nothing more is what a CONFORMANCE CLASS is about: the class restricts which
/// representation types may appear, not what geometry they carry.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
//#endregion 🔖️ConformanceEdits

//#region 🔖️EntityDiffs
/// 🧬️ Every entity type name `entity` carries: its leading name and, for a complex instance, each constituent.
fn type_names(entity: &StepEntity) -> impl Iterator<Item = &str> {
    std::iter::once(entity.name.as_str()).chain(entity.complex.iter().map(|part| part.name.as_str()))
}

/// 🔍️ Does `entity` carry any of the (case-insensitive) type names in `names`?
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn is_of_any(entity: &StepEntity, names: &[&str]) -> bool {
    type_names(entity).any(|have| names.iter().any(|want| have.eq_ignore_ascii_case(want)))
}

/// 🪜️ Is `entity` a `*_SHAPE_REPRESENTATION` on the ladder?
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn is_shape_representation(entity: &StepEntity) -> bool {
    type_names(entity).any(|name| ladder_rung_of(name).is_some())
}

/// 🏭️ Does `entity` sit on one of the three product identity chain rungs?
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn is_chain_rung(entity: &StepEntity) -> bool {
    chain_groups().iter().any(|group| is_of_any(entity, group))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn chain_groups() -> [&'static [&'static str]; 3] {
    [PRODUCT_TYPES, PRODUCT_DEFINITION_FORMATION_TYPES, PRODUCT_DEFINITION_TYPES]
}

/// 🔎️ The ladder-relevant instance `id` carries, read back as a [`ShapeRepresentationRow`], or
/// `None` when `id` is absent or is not a `*_SHAPE_REPRESENTATION` at all.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn shape_representation_row(base: &StepSnapshot, id: u64) -> Option<ShapeRepresentationRow> {
    let entity = base.entities.iter().find(|entity| entity.id == id)?;
    let (type_name, args) = if ladder_rung_of(&entity.name).is_some() {
        (entity.name.as_str(), entity.args.as_slice())
    } else {
        entity.complex.iter().find(|part| ladder_rung_of(&part.name).is_some()).map(|part| (part.name.as_str(), part.args.as_slice()))?
    };
    Some(ShapeRepresentationRow {
        type_name: type_name.to_string(),
        name: match args.first() {
            Some(StepValue::String(text)) => text.clone(),
            _ => String::new(),
        },
        items: match args.get(1) {
            Some(StepValue::Aggregate(items)) => items.iter().filter_map(|item| if let StepValue::Reference(id) = item { Some(*id) } else { None }).collect(),
            _ => Vec::new(),
        },
        context: match args.get(2) {
            Some(StepValue::Reference(id)) => Some(*id),
            _ => None,
        },
    })
}

/// 🧱️ The simple entity `row` authors at `id`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn representation_entity(id: u64, row: &ShapeRepresentationRow) -> StepEntity {
    StepEntity {
        id,
        name: row.type_name.to_ascii_uppercase(),
        args: vec![StepValue::String(row.name.clone()), StepValue::Aggregate(row.items.iter().map(|item| StepValue::Reference(*item)).collect()), row.context.map_or(StepValue::Unset, StepValue::Reference)],
        complex: Vec::new(),
    }
}

/// 🏭️ The product identity chain `base` carries (the first instance of each rung), or `None` when any rung is missing -- the exact
/// condition [`has_product_definition_chain`] reports on.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn product_identity(base: &StepSnapshot) -> Option<ProductIdentity> {
    let text = |entity: &StepEntity| match entity.args.first() {
        Some(StepValue::String(text)) => text.clone(),
        _ => String::new(),
    };
    let find = |names: &[&str]| base.entities.iter().find(|entity| is_of_any(entity, names));
    let (product, formation, definition) = (find(PRODUCT_TYPES)?, find(PRODUCT_DEFINITION_FORMATION_TYPES)?, find(PRODUCT_DEFINITION_TYPES)?);
    Some(ProductIdentity { product: product.id, product_name: text(product), formation: formation.id, formation_id: text(formation), definition: definition.id, definition_id: text(definition) })
}

/// 🧬️ The three rungs of `identity`, authored with each rung's supertype name (the form the specification names), ascending by id.
///
/// ⚠️ The authored `PRODUCT` carries three of ISO 10303-41's four attributes: `frame_of_reference` is omitted rather than written as the
/// empty aggregate `()` ISO 10303-21 §6.2 permits, so this and the AP214 reference oracle author the SAME shape. The reason is recorded
/// where it was measured -- `../../../../🦀️oracle.rs`'s `set_product_identity` -- a defect in `ruststep` 0.4: that reader cannot parse
/// an empty aggregate as an argument value at all.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn identity_rungs(identity: &ProductIdentity) -> Vec<StepEntity> {
    let rung = |id: u64, name: &str, args: Vec<StepValue>| StepEntity { id, name: name.to_string(), args, complex: Vec::new() };
    let mut rungs = vec![
        rung(identity.product, PRODUCT_TYPES[0], vec![StepValue::String(identity.product_name.clone()), StepValue::String(identity.product_name.clone()), StepValue::String(String::new())]),
        rung(identity.formation, PRODUCT_DEFINITION_FORMATION_TYPES[0], vec![StepValue::String(identity.formation_id.clone()), StepValue::Unset, StepValue::Reference(identity.product)]),
        rung(identity.definition, PRODUCT_DEFINITION_TYPES[0], vec![StepValue::String(identity.definition_id.clone()), StepValue::Unset, StepValue::Reference(identity.formation), StepValue::Unset]),
    ];
    rungs.sort_by_key(|rung| rung.id);
    rungs
}

/// ✏️ The sparse diff that rewrites `existing` into `entity` in place: its name when it differs, each shared argument position whose value differs, the
/// tail arguments one list has beyond the other, and the complex constituents when they differ.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn rewritten_entity_diff(existing: &StepEntity, entity: &StepEntity) -> StepEntityDiff {
    let shared = existing.args.len().min(entity.args.len());
    let args = StepArgsDiff {
        removed: (shared..existing.args.len()).collect(),
        modified: (0..shared).filter(|index| existing.args[*index] != entity.args[*index]).map(|index| StepArgModified { index, value: entity.args[index].clone() }).collect(),
        added: (shared..entity.args.len()).map(|index| StepArgAdded { index, value: entity.args[index].clone() }).collect(),
    };
    let changed = !(args.removed.is_empty() && args.modified.is_empty() && args.added.is_empty());
    StepEntityDiff { name: (existing.name != entity.name).then(|| entity.name.clone()), args: changed.then_some(args), complex: (existing.complex != entity.complex).then(|| entity.complex.clone()) }
}

/// 🧩️ The diff that sets (`Some`, at `index` when new) or removes (`None`) entity `id`: an absent id with `None` is the empty diff, a present id
/// with `Some` is edited in place name-by-name and argument-by-argument.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn entity_diff(base: &StepSnapshot, id: u64, entity: Option<&StepEntity>, index: Option<usize>) -> StepDiff {
    let entities = match (base.entities.iter().find(|existing| existing.id == id), entity) {
        (None, None) => return StepDiff::default(),
        (Some(existing), Some(entity)) if existing == entity => return StepDiff::default(),
        (Some(_), None) => StepEntitiesDiff { removed: vec![id], ..Default::default() },
        (Some(existing), Some(entity)) => StepEntitiesDiff { modified: vec![StepEntityModified { id, diff: rewritten_entity_diff(existing, entity) }], ..Default::default() },
        (None, Some(entity)) => StepEntitiesDiff { added: vec![StepEntityAdded { index: index.map_or(base.entities.len(), |at| at.min(base.entities.len())), entity: entity.clone() }], ..Default::default() },
    };
    StepDiff { entities: Some(entities), ..Default::default() }
}

/// 🏷️ The diff that declares exactly `schemas` in `FILE_SCHEMA`; an all-blank declaration is refused under `class`'s name.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn file_schema_diff(base: &StepSnapshot, class: &str, schemas: &[String]) -> Result<StepDiff, String> {
    if schemas.iter().all(|name| name.trim().is_empty()) {
        return Err(format!("{class} requires FILE_SCHEMA to declare a schema -- an empty declaration is not an AP214 exchange structure"));
    }
    let file_schema = StepFileSchema { schemas: schemas.to_vec() };
    Ok(StepDiff { file_schema: (base.header.file_schema != file_schema).then_some(file_schema), ..Default::default() })
}

/// 🗑️ The diff that deletes the representation at `id`, refusing anything that is not a `*_SHAPE_REPRESENTATION` -- a conformance repair must
/// never delete a real geometry or product record because a scenario named the wrong id.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn remove_representation_diff(base: &StepSnapshot, id: u64) -> Result<StepDiff, String> {
    match base.entities.iter().find(|entity| entity.id == id) {
        Some(entity) if is_shape_representation(entity) => Ok(entity_diff(base, id, None, None)),
        _ => Err(format!("#{id} is not a *_SHAPE_REPRESENTATION instance in this document -- a ladder edit addresses the ladder, never an arbitrary entity")),
    }
}

/// ✍️ The diff that writes representation `row` at `id` under `class`'s ceiling of `max_rung`. Every rejection names the class and the rung.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn representation_diff(base: &StepSnapshot, class: &str, max_rung: u8, id: u64, row: &ShapeRepresentationRow, index: Option<usize>) -> Result<StepDiff, String> {
    let rung = ladder_rung_of(&row.type_name).ok_or_else(|| format!("{:?} is not a *_SHAPE_REPRESENTATION type -- {class}'s ladder verb addresses the ladder only", row.type_name))?;
    if rung > max_rung {
        return Err(format!("{:?} sits on ladder rung {rung}, above {class}'s ceiling of {max_rung} -- writing it would put the document outside the class it claims", row.type_name));
    }
    if base.entities.iter().any(|entity| entity.id == id && !is_shape_representation(entity)) {
        return Err(format!("#{id} is not a *_SHAPE_REPRESENTATION instance in this document -- a ladder edit addresses the ladder, never an arbitrary entity"));
    }
    Ok(entity_diff(base, id, Some(&representation_entity(id, row)), index))
}

/// ⬇️ The diff that rewrites the representation at `id` onto `class`'s ceiling type, keeping its `name`, `items` and `context_of_items` exactly.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demotion_diff(base: &StepSnapshot, class: &str, max_rung: u8, id: u64) -> Result<StepDiff, String> {
    let ceiling = ceiling_type_of(max_rung).ok_or_else(|| format!("{class} admits no *_SHAPE_REPRESENTATION at all, so it has no ceiling to demote onto"))?;
    let row = shape_representation_row(base, id).ok_or_else(|| format!("#{id} is not a *_SHAPE_REPRESENTATION instance in this document"))?;
    Ok(entity_diff(base, id, Some(&representation_entity(id, &ShapeRepresentationRow { type_name: ceiling.to_string(), ..row })), None))
}

/// 🏭️ The diff that replaces every product identity chain rung with `identity`'s three (`Some`), or removes the chain (`None`) -- the only
/// edit that deterministically turns the soft `product-definition-chain` diagnostic ON. The authored rungs land at their id-ordered slot among
/// the retained entities; an identity that already stands exactly so is the empty diff.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn product_identity_diff(base: &StepSnapshot, identity: Option<&ProductIdentity>) -> StepDiff {
    let removed: Vec<u64> = base.entities.iter().filter(|entity| is_chain_rung(entity)).map(|entity| entity.id).collect();
    let mut working: Vec<&StepEntity> = base.entities.iter().filter(|entity| !is_chain_rung(entity)).collect();
    let rungs = identity.map(identity_rungs).unwrap_or_default();
    let mut added = Vec::new();
    for rung in &rungs {
        let index = working.iter().filter(|entity| entity.id < rung.id).count();
        working.insert(index, rung);
        added.push(StepEntityAdded { index, entity: rung.clone() });
    }
    if working.len() == base.entities.len() && working.iter().zip(&base.entities).all(|(left, right)| *left == right) {
        return StepDiff::default();
    }
    StepDiff { entities: Some(StepEntitiesDiff { removed, added, ..Default::default() }), ..Default::default() }
}

/// 📦️ One row that puts entity `id` back exactly: `entity` is its absolute value (`None` removes it) and `index` the position a new entity takes
/// among the final entities (last when absent).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct EntityRestore {
    pub id: u64,
    pub entity: Option<StepEntity>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

/// 🧩️ The ONE diff that carries `base` to the state `rows` name: removed ids, in-place modifications, and additions ordered by their final
/// index. A row whose entity carries another id is refused.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn restore_diff(base: &StepSnapshot, rows: &[EntityRestore]) -> Result<StepDiff, String> {
    if let Some(row) = rows.iter().find(|row| row.entity.as_ref().is_some_and(|entity| entity.id != row.id)) {
        return Err(format!("entity #{} cannot be restored at #{}", row.entity.as_ref().map_or(0, |entity| entity.id), row.id));
    }
    let mut diff = StepEntitiesDiff::default();
    let mut unplaced = Vec::new();
    for row in rows {
        match (base.entities.iter().find(|existing| existing.id == row.id), &row.entity) {
            (None, None) => {}
            (Some(_), None) => diff.removed.push(row.id),
            (Some(existing), Some(entity)) => {
                if existing != entity {
                    diff.modified.push(StepEntityModified { id: row.id, diff: rewritten_entity_diff(existing, entity) });
                }
            }
            (None, Some(entity)) => match row.index {
                Some(index) => diff.added.push(StepEntityAdded { index, entity: entity.clone() }),
                None => unplaced.push(entity.clone()),
            },
        }
    }
    let total = base.entities.len() - diff.removed.len() + diff.added.len() + unplaced.len();
    diff.added.iter_mut().for_each(|added| added.index = added.index.min(total.saturating_sub(1)));
    diff.added.sort_by_key(|added| added.index);
    let end = total - unplaced.len();
    diff.added.extend(unplaced.into_iter().enumerate().map(|(offset, entity)| StepEntityAdded { index: end + offset, entity }));
    Ok(StepDiff { entities: (!diff.is_empty()).then_some(diff), ..Default::default() })
}

/// ↩️ The single row that puts entity `id` back as `base` holds it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn restore_entity_rows(base: &StepSnapshot, id: u64) -> Vec<EntityRestore> {
    match base.entities.iter().position(|entity| entity.id == id) {
        Some(at) => vec![EntityRestore { id, entity: Some(base.entities[at].clone()), index: Some(at) }],
        None => vec![EntityRestore { id, entity: None, index: None }],
    }
}

/// ↩️ The rows that undo [`product_identity_diff`] with `identity` on `base`: the authored rungs are removed (reverse of the order the diff adds
/// them) and every base chain rung is restored at its exact base position.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn chain_restore_rows(base: &StepSnapshot, identity: Option<&ProductIdentity>) -> Vec<EntityRestore> {
    let mut rows: Vec<EntityRestore> = identity.map(identity_rungs).unwrap_or_default().iter().rev().map(|rung| EntityRestore { id: rung.id, entity: None, index: None }).collect();
    rows.extend(base.entities.iter().enumerate().filter(|(_, entity)| is_chain_rung(entity)).map(|(at, entity)| EntityRestore { id: entity.id, entity: Some(entity.clone()), index: Some(at) }));
    rows
}

/// ✏️ The rows one details-pane edit of the entity list names: inserting an entity writes it at that position, removing one clears it, and any
/// other edit of an entity (its name, an argument, the entity set whole) writes the edited entity in place. `None` for a pointer outside
/// `/entities/<row>`; the order of retained entities, an entity's id and its complex constituents are not addressable and are refused by
/// the row's own diff.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn edit_restore_rows(event: &SnapshotEditEvent, snapshot: &StepSnapshot) -> Result<Option<Vec<EntityRestore>>, SnapshotEditError> {
    let Some(pointer) = (match event {
        SnapshotEditEvent::SetValue { path, .. } | SnapshotEditEvent::InsertValue { path, .. } | SnapshotEditEvent::RemoveValue { path } | SnapshotEditEvent::RenameKey { path, .. } => Some(path.as_str()),
        SnapshotEditEvent::MoveValue { .. } | SnapshotEditEvent::ReplaceSource { .. } => None,
    }) else {
        return Ok(None);
    };
    let segments: Vec<&str> = pointer.split('/').skip(1).collect();
    let ["entities", position, rest @ ..] = segments.as_slice() else { return Ok(None) };
    let refusal = |message: String| SnapshotEditError::new("snapshot-edit.schema-invalid", pointer, message);
    let bounds = |at: usize| SnapshotEditError::new("snapshot-edit.index-out-of-bounds", pointer, format!("position {at} is outside the {} entities", snapshot.entities.len()));
    let row = |insert: bool| -> Result<usize, SnapshotEditError> {
        let at = if insert && *position == "-" { snapshot.entities.len() } else { position.parse().map_err(|_| refusal(format!("'{position}' is no entity position")))? };
        if at > snapshot.entities.len() || (!insert && at == snapshot.entities.len()) {
            return Err(bounds(at));
        }
        Ok(at)
    };
    match (rest, event) {
        ([], SnapshotEditEvent::InsertValue { value, .. }) => {
            let at = row(true)?;
            let entity = StepEntity::from_value(value.clone()).map_err(|error| refusal(error.to_string()))?;
            Ok(Some(vec![EntityRestore { id: entity.id, entity: Some(entity), index: Some(at) }]))
        }
        ([], SnapshotEditEvent::RemoveValue { .. }) => Ok(Some(vec![EntityRestore { id: snapshot.entities[row(false)?].id, entity: None, index: None }])),
        _ => {
            let at = row(false)?;
            let current = &snapshot.entities[at];
            let next = StepEntity::from_value(edited_subtree(&current.to_value(), &format!("/entities/{at}"), event)?).map_err(|error| refusal(error.to_string()))?;
            Ok(Some(if next == *current { Vec::new() } else { vec![EntityRestore { id: current.id, entity: Some(next), index: None }] }))
        }
    }
}
//#endregion 🔖️EntityDiffs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
