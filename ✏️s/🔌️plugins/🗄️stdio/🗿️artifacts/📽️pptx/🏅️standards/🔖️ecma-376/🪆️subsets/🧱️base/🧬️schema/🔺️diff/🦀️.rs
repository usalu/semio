//! 🔺️ PptxDiff -- sparse diff over `PptxSnapshot` (`schema`, `opc: OpcPackage`, `xml_parts`). The OPC layer is diffed part by part, content-type
//! entry by content-type entry and relationship by relationship; each XML part is diffed as a part (its content type, and its document through the
//! XML artifact's own recursive `XmlDiff`). A diff names only what changed -- never a whole `opc` or `xml_parts` owner.

use crate::schema::snapshot::{PptxParagraph, PptxPresentation, PptxRun, PptxShape, PptxSlide, PptxTransform, PptxXmlPart};
use crate::PptxSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use semio_s_artifact_stdio_xml::schema::diff::XmlDiff;
#[cfg(test)]
use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;
use semio_s_artifact_stdio_xml::schema::snapshot::XmlDocument;
use semio_s_artifact_stdio_xml::{XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_zip::opc::{OpcContentTypes, OpcPackage, OpcPart, OpcRelationship, OpcTargetMode};

//#region 🔖️GenericCollectionTriples
/// 🌳 Index-keyed collection triple, generic over the item type `T` and its per-field diff type
/// `D`. `removed`/`modified` indices refer to BASE state (descending removal order on apply);
/// `added` indices refer to FINAL state (ascending insert, `min(index, len)`).
// 🩹 `#[derive(ToValue, FromValue)]` synthesizes `D: ToValue + FromValue`/`T: ...` automatically
// per own type parameter (see `🌱️value/✨️derive`'s module docs) — no explicit
// `#[value(bound = "...")]` override needed here.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IndexedTripleDiff<D, T> {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<IndexModified<D>>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<IndexAdded<T>>,
}

impl<D, T> Default for IndexedTripleDiff<D, T> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IndexModified<D> {
    pub index: usize,
    pub diff: D,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IndexAdded<T> {
    pub index: usize,
    pub item: T,
}

/// 🏷️ Name/key-keyed collection triple, generic over key `K`, item `T`, and per-field diff `D`.
/// `added` carries the full item (which already contains its own key). No explicit
/// `#[value(bound = "...")]` needed — auto-synthesized per type parameter, same as
/// `IndexedTripleDiff` above.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct NamedTripleDiff<K, D, T> {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<K>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<NamedModified<K, D>>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<T>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub order: Vec<K>,
}

impl<K, D, T> Default for NamedTripleDiff<K, D, T> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new(), order: Vec::new() }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct NamedModified<K, D> {
    pub key: K,
    pub diff: D,
}
//#endregion 🔖️GenericCollectionTriples

//#region 🔖️OpcDiffTypes
pub type PptxOpcCtEntriesDiff = NamedTripleDiff<String, String, (String, String)>;
pub type PptxOpcPartsDiff = NamedTripleDiff<String, PptxOpcPartDiff, OpcPart>;
pub type PptxOpcRelListDiff = NamedTripleDiff<String, PptxOpcRelDiff, OpcRelationship>;
pub type PptxOpcRelationshipsDiff = NamedTripleDiff<String, PptxOpcRelListDiff, (String, Vec<OpcRelationship>)>;

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PptxOpcContentTypesDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub defaults: Option<PptxOpcCtEntriesDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<PptxOpcCtEntriesDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PptxOpcPartDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PptxOpcRelDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rel_type: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub target_mode: Option<OpcTargetMode>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PptxOpcDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub content_types: Option<PptxOpcContentTypesDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<PptxOpcPartsDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub relationships: Option<PptxOpcRelationshipsDiff>,
}
//#endregion 🔖️OpcDiffTypes

//#region 🔖️XmlPartDiffTypes
pub type PptxXmlPartsDiff = NamedTripleDiff<String, PptxXmlPartDiff, PptxXmlPart>;

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PptxXmlPartDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<XmlDiff>,
}
//#endregion 🔖️XmlPartDiffTypes

//#region 🔖️Diff
/// 🔺️ Sparse diff over the canonical PPTX snapshot fields.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pptx.diff")]
pub struct PptxDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub opc: Option<PptxOpcDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub xml_parts: Option<PptxXmlPartsDiff>,
}
//#endregion 🔖️Diff

//#region 🔖️GenericNamedEngine
/// 🧮️ The key sequence `removed`/`added` alone imply — survivors in base order, then the additions
/// in carried order. `NamedTripleDiff::order` is populated exactly when the real target sequence is
/// NOT this one, so an order-insignificant collection never pays for the field.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn default_named_order<K: PartialEq + Clone>(base_keys: &[K], removed: &[K], added_keys: &[K]) -> Vec<K> {
    base_keys.iter().filter(|k| !removed.contains(k)).chain(added_keys.iter()).cloned().collect()
}

/// 🔀️ Rebuilds `items` into `order` when one is carried, and fails loudly when it is not a
/// permutation of what the collection actually holds — a silently dropped or duplicated item is
/// precisely the failure this field exists to prevent.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn reorder_named<K: PartialEq, T>(items: &mut Vec<T>, order: &[K], key_of: impl Fn(&T) -> K) -> MutationApplyResult<()> {
    if order.is_empty() {
        return Ok(());
    }
    if order.len() != items.len() {
        return Err(MutationApplyError::new("mutation.apply.invalid-order", "named ordering does not cover the resulting collection").at(["order"]));
    }
    let mut pool: Vec<Option<T>> = std::mem::take(items).into_iter().map(Some).collect();
    for key in order {
        let slot = pool.iter().position(|held| matches!(held, Some(item) if key_of(item) == *key)).ok_or_else(|| MutationApplyError::new("mutation.apply.invalid-order", "named ordering names an item the collection does not carry").at(["order"]))?;
        items.push(pool[slot].take().expect("the slot was located as occupied one line above"));
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_named<K, T, D>(base: &[T], other: &[T], key_of: impl Fn(&T) -> K, diff_item: impl Fn(&T, &T) -> Option<D>) -> Option<NamedTripleDiff<K, D, T>>
where
    K: PartialEq + Clone,
    T: Clone + PartialEq,
{
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for b in base {
        let bk = key_of(b);
        match other.iter().find(|o| key_of(o) == bk) {
            None => removed.push(bk),
            Some(o) if o != b => {
                if let Some(d) = diff_item(b, o) {
                    modified.push(NamedModified { key: bk, diff: d });
                }
            }
            Some(_) => {}
        }
    }
    let mut added = Vec::new();
    for o in other {
        let ok = key_of(o);
        if !base.iter().any(|b| key_of(b) == ok) {
            added.push(o.clone());
        }
    }
    let base_keys: Vec<K> = base.iter().map(&key_of).collect();
    let other_keys: Vec<K> = other.iter().map(&key_of).collect();
    let added_keys: Vec<K> = added.iter().map(&key_of).collect();
    let order = if default_named_order(&base_keys, &removed, &added_keys) == other_keys { Vec::new() } else { other_keys };
    if removed.is_empty() && modified.is_empty() && added.is_empty() && order.is_empty() {
        None
    } else {
        Some(NamedTripleDiff { removed, modified, added, order })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_named<K, T, D>(items: &mut Vec<T>, diff: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, apply_item: impl Fn(&mut T, &D) -> MutationApplyResult<()>) -> MutationApplyResult<()>
where
    K: PartialEq + Clone,
    T: Clone,
{
    let keys: Vec<K> = items.iter().map(&key_of).collect();
    for key in &diff.removed {
        if !keys.contains(key) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "named removal target does not exist").at(["removed"]));
        }
    }
    for (index, key) in diff.removed.iter().enumerate() {
        if diff.removed[..index].contains(key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named removal target is repeated").at(["removed"]));
        }
    }
    let mut modified_keys = Vec::new();
    for modified in &diff.modified {
        if !keys.contains(&modified.key) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "named modification target does not exist").at(["modified"]));
        }
        if diff.removed.contains(&modified.key) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "named modification targets a removed item").at(["modified"]));
        }
        if modified_keys.contains(&modified.key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named modification target is repeated").at(["modified"]));
        }
        modified_keys.push(modified.key.clone());
    }
    let mut added_keys = Vec::new();
    for item in &diff.added {
        let key = key_of(item);
        if keys.contains(&key) || added_keys.contains(&key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named addition target already exists").at(["added"]));
        }
        added_keys.push(key);
    }
    items.retain(|i| !diff.removed.contains(&key_of(i)));
    for m in &diff.modified {
        let item = items.iter_mut().find(|i| key_of(i) == m.key).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "named modification target does not exist").at(["modified"]))?;
        apply_item(item, &m.diff).map_err(|error| error.under(["modified"]))?;
    }
    for item in &diff.added {
        items.push(item.clone());
    }
    reorder_named(items, &diff.order, &key_of)?;
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_named<K, T, D>(base_items: &[T], diff: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, inverse_item: impl Fn(&T, &D) -> D) -> NamedTripleDiff<K, D, T>
where
    K: PartialEq + Clone,
    T: Clone,
{
    let removed: Vec<K> = diff.added.iter().map(&key_of).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_items.iter().find(|i| key_of(i) == m.key) {
            modified.push(NamedModified { key: m.key.clone(), diff: inverse_item(original, &m.diff) });
        }
    }
    let mut added = Vec::new();
    for k in &diff.removed {
        if let Some(original) = base_items.iter().find(|i| &key_of(i) == k) {
            added.push(original.clone());
        }
    }
    let base_keys: Vec<K> = base_items.iter().map(&key_of).collect();
    let other_keys = if diff.order.is_empty() { default_named_order(&base_keys, &diff.removed, &removed) } else { diff.order.clone() };
    let added_keys: Vec<K> = added.iter().map(&key_of).collect();
    let order = if default_named_order(&other_keys, &removed, &added_keys) == base_keys { Vec::new() } else { base_keys };
    NamedTripleDiff { removed, modified, added, order }
}

/// 🧮️ Name-keyed absorb — identity is the KEY (not position), so no index transport is needed:
/// a `d2`-removal of a `d1`-added key annihilates the add; a `d2`-modify of a `d1`-added key
/// patches into the carried payload; everything else composes directly on the shared key space.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_named<K, T, D>(d1: NamedTripleDiff<K, D, T>, d2: NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&mut T, &D)) -> NamedTripleDiff<K, D, T>
where
    K: PartialEq + Clone,
    T: Clone,
    D: Clone,
{
    let d1_added_keys: Vec<K> = d1.added.iter().map(&key_of).collect();
    let d1_order = d1.order.clone();
    let mut removed = d1.removed.clone();
    let mut annihilated: Vec<K> = Vec::new();
    for k in &d2.removed {
        if d1_added_keys.contains(k) {
            annihilated.push(k.clone());
        } else if !removed.contains(k) {
            removed.push(k.clone());
        }
    }
    let mut working_added: Vec<T> = d1.added.into_iter().filter(|a| !annihilated.contains(&key_of(a))).collect();
    let mut modified: Vec<NamedModified<K, D>> = d1.modified.into_iter().filter(|m| !removed.contains(&m.key)).collect();
    for m2 in &d2.modified {
        if let Some(added) = working_added.iter_mut().find(|a| key_of(a) == m2.key) {
            apply_item(added, &m2.diff);
            continue;
        }
        if removed.contains(&m2.key) {
            continue;
        }
        match modified.iter_mut().find(|m| m.key == m2.key) {
            Some(existing) => existing.diff = absorb_item(existing.diff.clone(), m2.diff.clone()),
            None => modified.push(NamedModified { key: m2.key.clone(), diff: m2.diff.clone() }),
        }
    }
    for a2 in &d2.added {
        let k2 = key_of(a2);
        match working_added.iter_mut().find(|a| key_of(a) == k2) {
            Some(existing) => *existing = a2.clone(),
            None => working_added.push(a2.clone()),
        }
    }
    let order = if !d2.order.is_empty() {
        d2.order
    } else if d1_order.is_empty() {
        Vec::new()
    } else {
        let mut composed: Vec<K> = d1_order.into_iter().filter(|k| !d2.removed.contains(k)).collect();
        for a2 in &d2.added {
            let k2 = key_of(a2);
            if !composed.contains(&k2) {
                composed.push(k2);
            }
        }
        composed
    };
    NamedTripleDiff { removed, modified, added: working_added, order }
}
//#endregion 🔖️GenericNamedEngine

//#region 🔖️OpcDiffLogic
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_ct_entries(old: &[(String, String)], new: &[(String, String)]) -> Option<PptxOpcCtEntriesDiff> {
    between_named(old, new, |(k, _)| k.clone(), |(_, ov), (_, nv)| (ov != nv).then(|| nv.clone()))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_ct_entries(entries: &mut Vec<(String, String)>, diff: &PptxOpcCtEntriesDiff) -> MutationApplyResult<()> {
    apply_named(
        entries,
        diff,
        |(k, _)| k.clone(),
        |(_, v), nv| {
            *v = nv.clone();
            Ok(())
        },
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_ct_entries(base: &[(String, String)], diff: &PptxOpcCtEntriesDiff) -> PptxOpcCtEntriesDiff {
    inverse_named(base, diff, |(k, _)| k.clone(), |(_, v), _| v.clone())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_ct_entries(a: PptxOpcCtEntriesDiff, b: PptxOpcCtEntriesDiff) -> PptxOpcCtEntriesDiff {
    // 🏷️ `D = String` here is already a whole-value replace (LWW) -- absorbing two such diffs on
    // the SAME key is just "the later one wins", i.e. `b`.
    absorb_named(a, b, |(k, _)| k.clone(), |_av, bv| bv, |(_, v), nv| *v = nv.clone())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_content_types(old: &OpcContentTypes, new: &OpcContentTypes) -> Option<PptxOpcContentTypesDiff> {
    let defaults = diff_ct_entries(&old.defaults, &new.defaults);
    let overrides = diff_ct_entries(&old.overrides, &new.overrides);
    if defaults.is_none() && overrides.is_none() {
        None
    } else {
        Some(PptxOpcContentTypesDiff { defaults, overrides })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_part(old: &OpcPart, new: &OpcPart) -> Option<PptxOpcPartDiff> {
    if old == new {
        return None;
    }
    Some(PptxOpcPartDiff { content_type: (old.content_type != new.content_type).then(|| new.content_type.clone()), bytes: (old.bytes != new.bytes).then(|| new.bytes.clone()) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_part(part: &mut OpcPart, diff: &PptxOpcPartDiff) {
    if let Some(v) = &diff.content_type {
        part.content_type = v.clone();
    }
    if let Some(v) = &diff.bytes {
        part.bytes = v.clone();
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn part_with_diff_applied(part: &OpcPart, diff: &PptxOpcPartDiff) -> OpcPart {
    let mut out = part.clone();
    apply_part(&mut out, diff);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_part(base: &OpcPart, diff: &PptxOpcPartDiff) -> PptxOpcPartDiff {
    PptxOpcPartDiff { content_type: diff.content_type.as_ref().map(|_| base.content_type.clone()), bytes: diff.bytes.as_ref().map(|_| base.bytes.clone()) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_part_diff(mut a: PptxOpcPartDiff, b: PptxOpcPartDiff) -> PptxOpcPartDiff {
    if b.content_type.is_some() {
        a.content_type = b.content_type;
    }
    if b.bytes.is_some() {
        a.bytes = b.bytes;
    }
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_parts(old: &[OpcPart], new: &[OpcPart]) -> Option<PptxOpcPartsDiff> {
    between_named(old, new, |p| p.path.clone(), diff_part)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_rel(old: &OpcRelationship, new: &OpcRelationship) -> Option<PptxOpcRelDiff> {
    if old == new {
        return None;
    }
    Some(PptxOpcRelDiff { rel_type: (old.rel_type != new.rel_type).then(|| new.rel_type.clone()), target: (old.target != new.target).then(|| new.target.clone()), target_mode: (old.target_mode != new.target_mode).then_some(new.target_mode) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_rel(rel: &mut OpcRelationship, diff: &PptxOpcRelDiff) {
    if let Some(v) = &diff.rel_type {
        rel.rel_type = v.clone();
    }
    if let Some(v) = &diff.target {
        rel.target = v.clone();
    }
    if let Some(v) = diff.target_mode {
        rel.target_mode = v;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_rel(base: &OpcRelationship, diff: &PptxOpcRelDiff) -> PptxOpcRelDiff {
    PptxOpcRelDiff { rel_type: diff.rel_type.as_ref().map(|_| base.rel_type.clone()), target: diff.target.as_ref().map(|_| base.target.clone()), target_mode: diff.target_mode.map(|_| base.target_mode) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_rel_diff(mut a: PptxOpcRelDiff, b: PptxOpcRelDiff) -> PptxOpcRelDiff {
    if b.rel_type.is_some() {
        a.rel_type = b.rel_type;
    }
    if b.target.is_some() {
        a.target = b.target;
    }
    if b.target_mode.is_some() {
        a.target_mode = b.target_mode;
    }
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_rel_list(old: &[OpcRelationship], new: &[OpcRelationship]) -> Option<PptxOpcRelListDiff> {
    between_named(old, new, |r| r.id.clone(), diff_rel)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_rel_list(list: &mut Vec<OpcRelationship>, diff: &PptxOpcRelListDiff) -> MutationApplyResult<()> {
    apply_named(
        list,
        diff,
        |r| r.id.clone(),
        |relationship, change| {
            apply_rel(relationship, change);
            Ok(())
        },
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn rel_list_with_diff_applied(list: &[OpcRelationship], diff: &PptxOpcRelListDiff) -> Vec<OpcRelationship> {
    let mut out = list.to_vec();
    out.retain(|relationship| !diff.removed.contains(&relationship.id));
    for modified in &diff.modified {
        if let Some(relationship) = out.iter_mut().find(|relationship| relationship.id == modified.key) {
            apply_rel(relationship, &modified.diff);
        }
    }
    out.extend(diff.added.iter().cloned());
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_rel_list(base: &[OpcRelationship], diff: &PptxOpcRelListDiff) -> PptxOpcRelListDiff {
    inverse_named(base, diff, |r| r.id.clone(), inverse_rel)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_rel_list_diff(a: PptxOpcRelListDiff, b: PptxOpcRelListDiff) -> PptxOpcRelListDiff {
    absorb_named(a, b, |r| r.id.clone(), absorb_rel_diff, apply_rel)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_relationships(old: &semio_s_artifact_stdio_zip::opc::OpcRelationshipOwners, new: &semio_s_artifact_stdio_zip::opc::OpcRelationshipOwners) -> Option<PptxOpcRelationshipsDiff> {
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for (owner, list) in old.groups() {
        match new.relationships(owner) {
            None => removed.push(owner.clone()),
            Some(nlist) => {
                if let Some(d) = diff_rel_list(list, nlist) {
                    modified.push(NamedModified { key: owner.clone(), diff: d });
                }
            }
        }
    }
    let mut added = Vec::new();
    for (owner, list) in new.groups() {
        if old.relationships(owner).is_none() {
            added.push((owner.clone(), list.clone()));
        }
    }
    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        None
    } else {
        Some(PptxOpcRelationshipsDiff { removed, modified, added, order: Vec::new() })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_relationships(rels: &mut semio_s_artifact_stdio_zip::opc::OpcRelationshipOwners, diff: &PptxOpcRelationshipsDiff) -> MutationApplyResult<()> {
    let mut added = std::collections::HashSet::new();
    for owner in &diff.removed {
        if rels.relationships(owner).is_none() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "relationship owner does not exist").at(vec!["removed".to_string(), owner.clone()]));
        }
        if !added.insert(owner) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "relationship owner is repeated").at(vec!["removed".to_string(), owner.clone()]));
        }
    }
    for modified in &diff.modified {
        if rels.relationships(&modified.key).is_none() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "relationship owner does not exist").at(vec!["modified".to_string(), modified.key.clone()]));
        }
        if diff.removed.contains(&modified.key) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "relationship owner is removed and modified").at(vec!["modified".to_string(), modified.key.clone()]));
        }
    }
    for (owner, _) in &diff.added {
        if rels.relationships(owner).is_some() || !added.insert(owner) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "relationship owner already exists").at(vec!["added".to_string(), owner.clone()]));
        }
    }
    for owner in &diff.removed {
        rels.remove_owner(owner);
    }
    for m in &diff.modified {
        let list = rels.relationships_mut(&m.key).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "relationship owner does not exist").at(vec!["modified".to_string(), m.key.clone()]))?;
        apply_rel_list(list, &m.diff).map_err(|error| error.under(vec!["modified".to_string(), m.key.clone()]))?;
    }
    for (owner, list) in &diff.added {
        rels.replace_owner(owner.clone(), list.clone());
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_relationships(base: &semio_s_artifact_stdio_zip::opc::OpcRelationshipOwners, diff: &PptxOpcRelationshipsDiff) -> PptxOpcRelationshipsDiff {
    let removed: Vec<String> = diff.added.iter().map(|(owner, _)| owner.clone()).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(list) = base.relationships(&m.key) {
            modified.push(NamedModified { key: m.key.clone(), diff: inverse_rel_list(list, &m.diff) });
        }
    }
    let mut added = Vec::new();
    for owner in &diff.removed {
        if let Some(list) = base.relationships(owner) {
            added.push((owner.clone(), list.clone()));
        }
    }
    PptxOpcRelationshipsDiff { removed, modified, added, order: Vec::new() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_relationships(d1: PptxOpcRelationshipsDiff, d2: PptxOpcRelationshipsDiff) -> PptxOpcRelationshipsDiff {
    absorb_named(d1, d2, |(owner, _)| owner.clone(), absorb_rel_list_diff, |(_, list), diff| *list = rel_list_with_diff_applied(list, diff))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_opc(base: &OpcPackage, other: &OpcPackage) -> Option<PptxOpcDiff> {
    let content_types = diff_content_types(&base.content_types, &other.content_types);
    let parts = diff_parts(&base.parts, &other.parts);
    let relationships = diff_relationships(&base.relationships, &other.relationships);
    let comment = (base.comment != other.comment).then(|| other.comment.clone());
    if comment.is_none() && content_types.is_none() && parts.is_none() && relationships.is_none() {
        None
    } else {
        Some(PptxOpcDiff { content_types, parts, relationships, comment })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_opc_diff(opc: &mut OpcPackage, diff: &PptxOpcDiff) -> MutationApplyResult<()> {
    if let Some(d) = &diff.content_types {
        if let Some(dd) = &d.defaults {
            apply_ct_entries(&mut opc.content_types.defaults, dd).map_err(|error| error.under(["contentTypes", "defaults"]))?;
        }
        if let Some(dd) = &d.overrides {
            apply_ct_entries(&mut opc.content_types.overrides, dd).map_err(|error| error.under(["contentTypes", "overrides"]))?;
        }
    }
    if let Some(d) = &diff.parts {
        apply_named(
            &mut opc.parts,
            d,
            |p| p.path.clone(),
            |part, change| {
                apply_part(part, change);
                Ok(())
            },
        )
        .map_err(|error| error.under(["parts"]))?;
    }
    if let Some(d) = &diff.relationships {
        apply_relationships(&mut opc.relationships, d).map_err(|error| error.under(["relationships"]))?;
    }
    if let Some(comment) = &diff.comment {
        opc.comment.clone_from(comment);
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_opc_diff(base: &OpcPackage, diff: &PptxOpcDiff) -> PptxOpcDiff {
    PptxOpcDiff {
        comment: diff.comment.as_ref().map(|_| base.comment.clone()),
        content_types: diff
            .content_types
            .as_ref()
            .map(|d| PptxOpcContentTypesDiff { defaults: d.defaults.as_ref().map(|dd| inverse_ct_entries(&base.content_types.defaults, dd)), overrides: d.overrides.as_ref().map(|dd| inverse_ct_entries(&base.content_types.overrides, dd)) }),
        parts: diff.parts.as_ref().map(|d| inverse_named(&base.parts, d, |p| p.path.clone(), inverse_part)),
        relationships: diff.relationships.as_ref().map(|d| inverse_relationships(&base.relationships, d)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_opc_diff(a: PptxOpcDiff, b: PptxOpcDiff) -> PptxOpcDiff {
    PptxOpcDiff {
        comment: b.comment.or(a.comment),
        content_types: match (a.content_types, b.content_types) {
            (None, x) => x,
            (x, None) => x,
            (Some(ca), Some(cb)) => Some(PptxOpcContentTypesDiff {
                defaults: match (ca.defaults, cb.defaults) {
                    (None, x) => x,
                    (x, None) => x,
                    (Some(da), Some(db)) => Some(absorb_ct_entries(da, db)),
                },
                overrides: match (ca.overrides, cb.overrides) {
                    (None, x) => x,
                    (x, None) => x,
                    (Some(da), Some(db)) => Some(absorb_ct_entries(da, db)),
                },
            }),
        },
        parts: match (a.parts, b.parts) {
            (None, x) => x,
            (x, None) => x,
            (Some(pa), Some(pb)) => Some(absorb_named(pa, pb, |p| p.path.clone(), absorb_part_diff, |part, diff| *part = part_with_diff_applied(part, diff))),
        },
        relationships: match (a.relationships, b.relationships) {
            (None, x) => x,
            (x, None) => x,
            (Some(ra), Some(rb)) => Some(absorb_relationships(ra, rb)),
        },
    }
}
//#endregion 🔖️OpcDiffLogic

//#region 🔖️XmlPartDiffLogic
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_snapshot(document: &XmlDocument) -> XmlSnapshot {
    XmlSnapshot { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc: document.clone() }
}

fn diff_xml_part(base: &PptxXmlPart, other: &PptxXmlPart) -> Option<PptxXmlPartDiff> {
    let document = XmlDiff::between(&xml_snapshot(&base.document), &xml_snapshot(&other.document));
    let diff = PptxXmlPartDiff { content_type: (base.content_type != other.content_type).then(|| other.content_type.clone()), document: (!document.is_empty()).then_some(document) };
    (diff.content_type.is_some() || diff.document.is_some()).then_some(diff)
}

fn apply_xml_part(part: &mut PptxXmlPart, diff: &PptxXmlPartDiff) -> MutationApplyResult<()> {
    if let Some(content_type) = &diff.content_type {
        part.content_type.clone_from(content_type);
    }
    if let Some(document) = &diff.document {
        part.document = protocol::apply_diff(document, &xml_snapshot(&part.document)).map_err(|error| error.under(["document"]))?.doc;
    }
    Ok(())
}

fn inverse_xml_part(base: &PptxXmlPart, diff: &PptxXmlPartDiff) -> PptxXmlPartDiff {
    PptxXmlPartDiff { content_type: diff.content_type.as_ref().map(|_| base.content_type.clone()), document: diff.document.as_ref().map(|document| document.inverse(&xml_snapshot(&base.document))) }
}

fn absorb_xml_part(mut first: PptxXmlPartDiff, second: PptxXmlPartDiff) -> PptxXmlPartDiff {
    if second.content_type.is_some() {
        first.content_type = second.content_type;
    }
    first.document = match (first.document.take(), second.document) {
        (None, value) => value,
        (value, None) => value,
        (Some(mut left), Some(right)) => {
            left.absorb(right);
            Some(left)
        }
    };
    first
}

fn between_xml_parts(base: &[PptxXmlPart], other: &[PptxXmlPart]) -> Option<PptxXmlPartsDiff> {
    between_named(base, other, |part| part.path.clone(), diff_xml_part)
}

fn apply_xml_parts(items: &mut Vec<PptxXmlPart>, diff: &PptxXmlPartsDiff) -> MutationApplyResult<()> {
    apply_named(items, diff, |part| part.path.clone(), |part, change| apply_xml_part(part, change))
}

fn inverse_xml_parts(base: &[PptxXmlPart], diff: &PptxXmlPartsDiff) -> PptxXmlPartsDiff {
    inverse_named(base, diff, |part| part.path.clone(), inverse_xml_part)
}
//#endregion 🔖️XmlPartDiffLogic

//#region 🔖️Apply
impl MutationDiff<PptxSnapshot> for PptxDiff {
    fn apply(&self, base: &PptxSnapshot, _capability: protocol::ApplyCapability) -> MutationApplyResult<PptxSnapshot> {
        let mut next = base.clone();
        if let Some(schema) = &self.schema {
            next.schema.clone_from(schema);
        }
        if let Some(diff) = &self.opc {
            apply_opc_diff(&mut next.opc, diff).map_err(|error| error.under(["opc"]))?;
        }
        if let Some(diff) = &self.xml_parts {
            apply_xml_parts(&mut next.xml_parts, diff).map_err(|error| error.under(["xmlParts"]))?;
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.schema.is_some() {
            self.schema = other.schema;
        }
        self.opc = match (self.opc.take(), other.opc) {
            (None, value) => value,
            (value, None) => value,
            (Some(left), Some(right)) => Some(absorb_opc_diff(left, right)),
        };
        self.xml_parts = match (self.xml_parts.take(), other.xml_parts) {
            (None, value) => value,
            (value, None) => value,
            (Some(left), Some(right)) => Some(absorb_named(left, right, |part| part.path.clone(), absorb_xml_part, |part, diff| {
                let _ = apply_xml_part(part, diff);
            })),
        };
    }
}
//#endregion 🔖️Apply

//#region 🔖️DiffAlgebra
impl DiffAlgebra<PptxSnapshot> for PptxDiff {
    fn inverse(&self, base: &PptxSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            opc: self.opc.as_ref().map(|diff| inverse_opc_diff(&base.opc, diff)),
            xml_parts: self.xml_parts.as_ref().map(|diff| inverse_xml_parts(&base.xml_parts, diff)),
        }
    }

    fn between(base: &PptxSnapshot, other: &PptxSnapshot) -> Self {
        Self { schema: (base.schema != other.schema).then(|| other.schema.clone()), opc: diff_opc(&base.opc, &other.opc), xml_parts: between_xml_parts(&base.xml_parts, &other.xml_parts) }
    }

    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.opc.is_none() && self.xml_parts.is_none()
    }
}
//#endregion 🔖️DiffAlgebra


#[cfg(test)]
pub(crate) fn demo_snapshot_a() -> PptxSnapshot {
    crate::standards::v_ecma_376::subsets::base::schema::construction::minimal::build_minimal_pptx(PptxPresentation {
        slides: vec![PptxSlide {
            shapes: vec![
                PptxShape::TextBox { text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "old".into(), bold: false, italic: false, font_size: Some(10) }] }], position: PptxTransform { x: 1, y: 1, cx: 1, cy: 1 } },
                PptxShape::Other { node: XmlNode::Element { name: "p:graphicFrame".into(), attrs: Vec::new(), children: Vec::new() } },
            ],
        }],
    })
}

#[cfg(test)]
pub(crate) fn demo_snapshot_b() -> PptxSnapshot {
    crate::standards::v_ecma_376::subsets::base::schema::construction::minimal::build_minimal_pptx(PptxPresentation {
        slides: vec![PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("new")], position: PptxTransform { x: 9, y: 9, cx: 9, cy: 9 } }] }],
    })
}

#[cfg(test)]
pub(crate) fn demo_diff_cases() -> Vec<PptxDiff> {
    let a = demo_snapshot_a();
    let b = demo_snapshot_b();
    vec![PptxDiff::default(), PptxDiff::between(&a, &b), PptxDiff::between(&b, &a), PptxDiff::between(&a, &a)]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
mod handcrafted_diff_codec_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️result-apply/🦀️.rs"]
mod result_apply_tests;
