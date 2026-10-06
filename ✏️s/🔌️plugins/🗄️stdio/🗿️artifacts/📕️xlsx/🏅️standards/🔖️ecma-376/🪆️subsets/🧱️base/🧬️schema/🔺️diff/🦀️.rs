//! 🔺️ Handcrafted sparse diff over the canonical XLSX authority: non-XML OPC state plus one
//! logical XML document per XML-bearing content part. Semantic workbook data is never diffed as
//! an independent persisted tree.
//!
//! **OPC diff placement**: `zip::opc::OpcPackage` (reused directly, not reimplemented — see that
//! module) has no diff type of its own yet, same gap docx's wave found. Defined HERE for the same
//! reason docx defined its own copy (this wave's ownership boundary is xlsx-mounted files only;
//! `zip/📦️opc` is out of bounds) — flagged again in `glue_followup` for hoisting once a third
//! consumer (pptx/bcf) needs the identical shape.

#[cfg(test)]
use crate::schema::snapshot::XlsxWorkbook;
use crate::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxXmlPart};
use crate::XlsxSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use semio_s_artifact_stdio_xml::schema::diff::XmlDiff;
use semio_s_artifact_stdio_xml::{XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_zip::opc::{OpcContentTypes, OpcPackage, OpcPart, OpcRelationship, OpcTargetMode};
use std::collections::BTreeMap;

//#region 🔖️GenericCollectionTriples
/// 🏷️ Name/key-keyed collection triple, generic over key `K`, item `T`, and per-field diff `D`.
/// `removed`/`modified` keys refer to BASE state; `added` carries the full item (already
/// containing its own key). Identity is the KEY, not position — no index transport is needed on
/// absorb.
// 🩹 `#[derive(ToValue, FromValue)]` synthesizes `K: ToValue + FromValue`/`D: .../`T: ...`
// automatically per own type parameter (see `🌱️value/✨️derive`'s module docs) — no explicit
// `#[value(bound = "...")]` override needed here.
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
pub type XlsxOpcCtEntriesDiff = NamedTripleDiff<String, String, (String, String)>;
pub type XlsxOpcPartsDiff = NamedTripleDiff<String, XlsxOpcPartDiff, OpcPart>;
pub type XlsxOpcRelListDiff = NamedTripleDiff<String, XlsxOpcRelDiff, OpcRelationship>;
pub type XlsxOpcRelationshipsDiff = NamedTripleDiff<String, XlsxOpcRelListDiff, (String, Vec<OpcRelationship>)>;

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct XlsxOpcContentTypesDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub defaults: Option<XlsxOpcCtEntriesDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<XlsxOpcCtEntriesDiff>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct XlsxOpcPartDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct XlsxOpcRelDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rel_type: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub target_mode: Option<OpcTargetMode>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct XlsxOpcDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub content_types: Option<XlsxOpcContentTypesDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<XlsxOpcPartsDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub relationships: Option<XlsxOpcRelationshipsDiff>,
}
//#endregion 🔖️OpcDiffTypes

//#region 🔖️XmlPartDiffTypes
pub type XlsxXmlPartsDiff = NamedTripleDiff<String, XlsxXmlPartDiff, XlsxXmlPart>;

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct XlsxXmlPartDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<XmlDiff>,
}
//#endregion 🔖️XmlPartDiffTypes

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.xlsx`.
/// The generic named collection triples and the imported XML diff are encoded by the handcrafted
/// codecs below because they are outside the derivable `DslField` shape.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.xlsx.diff")]
pub struct XlsxDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub opc: Option<XlsxOpcDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub xml_parts: Option<XlsxXmlPartsDiff>,
}
//#endregion 🔖️Diff

//#region 🔖️GenericNamedEngine
fn default_named_order<K: PartialEq + Clone>(base_keys: &[K], removed: &[K], added_keys: &[K]) -> Vec<K> {
    base_keys.iter().filter(|key| !removed.contains(key)).chain(added_keys.iter()).cloned().collect()
}

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
        items.push(pool[slot].take().expect("located occupied slot"));
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
    for (position, key) in diff.removed.iter().enumerate() {
        if !keys.contains(key) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "named removal target does not exist"));
        }
        if diff.removed[..position].contains(key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named removal target is repeated"));
        }
    }
    for (position, modified) in diff.modified.iter().enumerate() {
        if !keys.contains(&modified.key) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "named modification target does not exist"));
        }
        if diff.removed.contains(&modified.key) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "named modification targets a removed item"));
        }
        if diff.modified[..position].iter().any(|candidate| candidate.key == modified.key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named modification target is repeated"));
        }
    }
    for item in &diff.added {
        let key = key_of(item);
        if keys.contains(&key) || diff.added.iter().filter(|candidate| key_of(candidate) == key).count() != 1 {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named addition target already exists"));
        }
    }
    items.retain(|i| !diff.removed.contains(&key_of(i)));
    for m in &diff.modified {
        let item = items.iter_mut().find(|i| key_of(i) == m.key).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "named modification target does not exist"))?;
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

/// 🧮️ Name-keyed absorb — identity is the KEY (not position): a `d2`-removal of a `d1`-added key
/// annihilates the add; a `d2`-modify of a `d1`-added key patches into the carried payload;
/// everything else composes directly on the shared key space.
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
        let k2 = key_of(&a2);
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
        let mut composed: Vec<K> = d1_order.into_iter().filter(|key| !d2.removed.contains(key)).collect();
        for added in &d2.added {
            let key = key_of(added);
            if !composed.contains(&key) {
                composed.push(key);
            }
        }
        composed
    };
    NamedTripleDiff { removed, modified, added: working_added, order }
}
//#endregion 🔖️GenericNamedEngine

//#region 🔖️WorkbookDiffLogic
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_named_for_absorb<K, T, D>(items: &mut Vec<T>, diff: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, apply_item: impl Fn(&mut T, &D))
where
    K: PartialEq + Clone,
    T: Clone,
{
    items.retain(|item| !diff.removed.contains(&key_of(item)));
    for modified in &diff.modified {
        if let Some(item) = items.iter_mut().find(|item| key_of(item) == modified.key) {
            apply_item(item, &modified.diff);
        }
    }
    for added in &diff.added {
        items.push(added.clone());
    }
}
//#endregion 🔖️WorkbookDiffLogic

//#region 🔖️OpcDiffLogic
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_ct_entries(old: &[(String, String)], new: &[(String, String)]) -> Option<XlsxOpcCtEntriesDiff> {
    between_named(old, new, |(k, _)| k.clone(), |(_, ov), (_, nv)| (ov != nv).then(|| nv.clone()))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_ct_entries(entries: &mut Vec<(String, String)>, diff: &XlsxOpcCtEntriesDiff) -> MutationApplyResult<()> {
    apply_named(
        entries,
        diff,
        |(k, _)| k.clone(),
        |(_, value), next| {
            *value = next.clone();
            Ok(())
        },
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_ct_entries(base: &[(String, String)], diff: &XlsxOpcCtEntriesDiff) -> XlsxOpcCtEntriesDiff {
    inverse_named(base, diff, |(k, _)| k.clone(), |(_, v), _| v.clone())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_ct_entries(a: XlsxOpcCtEntriesDiff, b: XlsxOpcCtEntriesDiff) -> XlsxOpcCtEntriesDiff {
    absorb_named(a, b, |(k, _)| k.clone(), |_av, bv| bv, |(_, v), nv| *v = nv.clone())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_content_types(old: &OpcContentTypes, new: &OpcContentTypes) -> Option<XlsxOpcContentTypesDiff> {
    let defaults = diff_ct_entries(&old.defaults, &new.defaults);
    let overrides = diff_ct_entries(&old.overrides, &new.overrides);
    if defaults.is_none() && overrides.is_none() {
        None
    } else {
        Some(XlsxOpcContentTypesDiff { defaults, overrides })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_part(old: &OpcPart, new: &OpcPart) -> Option<XlsxOpcPartDiff> {
    if old == new {
        return None;
    }
    Some(XlsxOpcPartDiff { content_type: (old.content_type != new.content_type).then(|| new.content_type.clone()), bytes: (old.bytes != new.bytes).then(|| new.bytes.clone()) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_part(part: &mut OpcPart, diff: &XlsxOpcPartDiff) {
    if let Some(v) = &diff.content_type {
        part.content_type = v.clone();
    }
    if let Some(v) = &diff.bytes {
        part.bytes = v.clone();
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn part_with_diff_applied(part: &OpcPart, diff: &XlsxOpcPartDiff) -> OpcPart {
    let mut out = part.clone();
    apply_part(&mut out, diff);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_part(base: &OpcPart, diff: &XlsxOpcPartDiff) -> XlsxOpcPartDiff {
    XlsxOpcPartDiff { content_type: diff.content_type.as_ref().map(|_| base.content_type.clone()), bytes: diff.bytes.as_ref().map(|_| base.bytes.clone()) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_part_diff(mut a: XlsxOpcPartDiff, b: XlsxOpcPartDiff) -> XlsxOpcPartDiff {
    if b.content_type.is_some() {
        a.content_type = b.content_type;
    }
    if b.bytes.is_some() {
        a.bytes = b.bytes;
    }
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_parts(old: &[OpcPart], new: &[OpcPart]) -> Option<XlsxOpcPartsDiff> {
    between_named(old, new, |p| p.path.clone(), diff_part)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_rel(old: &OpcRelationship, new: &OpcRelationship) -> Option<XlsxOpcRelDiff> {
    if old == new {
        return None;
    }
    Some(XlsxOpcRelDiff { rel_type: (old.rel_type != new.rel_type).then(|| new.rel_type.clone()), target: (old.target != new.target).then(|| new.target.clone()), target_mode: (old.target_mode != new.target_mode).then_some(new.target_mode) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_rel(rel: &mut OpcRelationship, diff: &XlsxOpcRelDiff) {
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
fn inverse_rel(base: &OpcRelationship, diff: &XlsxOpcRelDiff) -> XlsxOpcRelDiff {
    XlsxOpcRelDiff { rel_type: diff.rel_type.as_ref().map(|_| base.rel_type.clone()), target: diff.target.as_ref().map(|_| base.target.clone()), target_mode: diff.target_mode.map(|_| base.target_mode) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_rel_diff(mut a: XlsxOpcRelDiff, b: XlsxOpcRelDiff) -> XlsxOpcRelDiff {
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
fn diff_rel_list(old: &[OpcRelationship], new: &[OpcRelationship]) -> Option<XlsxOpcRelListDiff> {
    between_named(old, new, |r| r.id.clone(), diff_rel)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_rel_list(list: &mut Vec<OpcRelationship>, diff: &XlsxOpcRelListDiff) -> MutationApplyResult<()> {
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
fn rel_list_with_diff_applied(list: &[OpcRelationship], diff: &XlsxOpcRelListDiff) -> Vec<OpcRelationship> {
    let mut out = list.to_vec();
    apply_named_for_absorb(&mut out, diff, |relationship| relationship.id.clone(), apply_rel);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_rel_list(base: &[OpcRelationship], diff: &XlsxOpcRelListDiff) -> XlsxOpcRelListDiff {
    inverse_named(base, diff, |r| r.id.clone(), inverse_rel)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_rel_list_diff(a: XlsxOpcRelListDiff, b: XlsxOpcRelListDiff) -> XlsxOpcRelListDiff {
    absorb_named(a, b, |r| r.id.clone(), absorb_rel_diff, apply_rel)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_relationships(old: &semio_s_artifact_stdio_zip::opc::OpcRelationshipOwners, new: &semio_s_artifact_stdio_zip::opc::OpcRelationshipOwners) -> Option<XlsxOpcRelationshipsDiff> {
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
        Some(XlsxOpcRelationshipsDiff { removed, modified, added, order: Vec::new() })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_relationships(rels: &mut semio_s_artifact_stdio_zip::opc::OpcRelationshipOwners, diff: &XlsxOpcRelationshipsDiff) -> MutationApplyResult<()> {
    for (position, owner) in diff.removed.iter().enumerate() {
        if rels.relationships(owner).is_none() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "relationship owner does not exist"));
        }
        if diff.removed[..position].contains(owner) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "relationship owner removal is repeated"));
        }
    }
    for (position, m) in diff.modified.iter().enumerate() {
        if rels.relationships(&m.key).is_none() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "relationship owner does not exist"));
        }
        if diff.removed.contains(&m.key) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "relationship owner is both removed and modified"));
        }
        if diff.modified[..position].iter().any(|candidate| candidate.key == m.key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "relationship owner modification is repeated"));
        }
    }
    for (position, (owner, _)) in diff.added.iter().enumerate() {
        if rels.relationships(owner).is_some() || diff.added[..position].iter().any(|(candidate, _)| candidate == owner) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "relationship owner already exists"));
        }
        if diff.removed.contains(owner) || diff.modified.iter().any(|candidate| candidate.key == *owner) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "relationship owner is both changed and added"));
        }
    }
    for owner in &diff.removed {
        rels.remove_owner(owner);
    }
    for m in &diff.modified {
        let list = rels.relationships_mut(&m.key).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "relationship owner does not exist"))?;
        apply_rel_list(list, &m.diff).map_err(|error| error.under(["modified"]))?;
    }
    for (owner, list) in &diff.added {
        rels.replace_owner(owner.clone(), list.clone());
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_relationships(base: &semio_s_artifact_stdio_zip::opc::OpcRelationshipOwners, diff: &XlsxOpcRelationshipsDiff) -> XlsxOpcRelationshipsDiff {
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
    XlsxOpcRelationshipsDiff { removed, modified, added, order: Vec::new() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_relationships(d1: XlsxOpcRelationshipsDiff, d2: XlsxOpcRelationshipsDiff) -> XlsxOpcRelationshipsDiff {
    absorb_named(d1, d2, |(owner, _)| owner.clone(), absorb_rel_list_diff, |(_, list), diff| *list = rel_list_with_diff_applied(list, diff))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_opc(base: &OpcPackage, other: &OpcPackage) -> Option<XlsxOpcDiff> {
    let content_types = diff_content_types(&base.content_types, &other.content_types);
    let parts = diff_parts(&base.parts, &other.parts);
    let relationships = diff_relationships(&base.relationships, &other.relationships);
    let comment = (base.comment != other.comment).then(|| other.comment.clone());
    if comment.is_none() && content_types.is_none() && parts.is_none() && relationships.is_none() {
        None
    } else {
        Some(XlsxOpcDiff { content_types, parts, relationships, comment })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_opc_diff(opc: &mut OpcPackage, diff: &XlsxOpcDiff) -> MutationApplyResult<()> {
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
fn inverse_opc_diff(base: &OpcPackage, diff: &XlsxOpcDiff) -> XlsxOpcDiff {
    XlsxOpcDiff {
        comment: diff.comment.as_ref().map(|_| base.comment.clone()),
        content_types: diff
            .content_types
            .as_ref()
            .map(|d| XlsxOpcContentTypesDiff { defaults: d.defaults.as_ref().map(|dd| inverse_ct_entries(&base.content_types.defaults, dd)), overrides: d.overrides.as_ref().map(|dd| inverse_ct_entries(&base.content_types.overrides, dd)) }),
        parts: diff.parts.as_ref().map(|d| inverse_named(&base.parts, d, |p| p.path.clone(), inverse_part)),
        relationships: diff.relationships.as_ref().map(|d| inverse_relationships(&base.relationships, d)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_opc_diff(a: XlsxOpcDiff, b: XlsxOpcDiff) -> XlsxOpcDiff {
    XlsxOpcDiff {
        comment: b.comment.or(a.comment),
        content_types: match (a.content_types, b.content_types) {
            (None, x) => x,
            (x, None) => x,
            (Some(ca), Some(cb)) => Some(XlsxOpcContentTypesDiff {
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
fn xml_snapshot(document: &semio_s_artifact_stdio_xml::schema::snapshot::XmlDocument) -> XmlSnapshot {
    XmlSnapshot { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc: document.clone() }
}

fn diff_xml_part(base: &XlsxXmlPart, other: &XlsxXmlPart) -> Option<XlsxXmlPartDiff> {
    let document = XmlDiff::between(&xml_snapshot(&base.document), &xml_snapshot(&other.document));
    let diff = XlsxXmlPartDiff { content_type: (base.content_type != other.content_type).then(|| other.content_type.clone()), document: (!document.is_empty()).then_some(document) };
    (diff.content_type.is_some() || diff.document.is_some()).then_some(diff)
}

fn apply_xml_part(part: &mut XlsxXmlPart, diff: &XlsxXmlPartDiff) -> MutationApplyResult<()> {
    if let Some(content_type) = &diff.content_type {
        part.content_type.clone_from(content_type);
    }
    if let Some(document) = &diff.document {
        part.document = document.apply(&xml_snapshot(&part.document))?.doc;
    }
    Ok(())
}

fn inverse_xml_part(base: &XlsxXmlPart, diff: &XlsxXmlPartDiff) -> XlsxXmlPartDiff {
    XlsxXmlPartDiff { content_type: diff.content_type.as_ref().map(|_| base.content_type.clone()), document: diff.document.as_ref().map(|document| document.inverse(&xml_snapshot(&base.document))) }
}

fn absorb_xml_part(mut first: XlsxXmlPartDiff, second: XlsxXmlPartDiff) -> XlsxXmlPartDiff {
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
//#endregion 🔖️XmlPartDiffLogic

//#region 🔖️Apply
impl MutationDiff<XlsxSnapshot> for XlsxDiff {
    fn apply(&self, base: &XlsxSnapshot) -> MutationApplyResult<XlsxSnapshot> {
        let mut next = base.clone();
        if let Some(d) = &self.opc {
            apply_opc_diff(&mut next.opc, d).map_err(|error| error.under(["opc"]))?;
        }
        if let Some(diff) = &self.xml_parts {
            apply_named(&mut next.xml_parts, diff, |part| part.path.clone(), apply_xml_part).map_err(|error| error.under(["xmlParts"]))?;
        }
        next.validate_authority().map_err(|error| MutationApplyError::new("mutation.apply.invalid-snapshot", error.to_string()))?;
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        self.opc = match (self.opc.take(), other.opc) {
            (None, x) => x,
            (x, None) => x,
            (Some(a), Some(b)) => Some(absorb_opc_diff(a, b)),
        };
        self.xml_parts = match (self.xml_parts.take(), other.xml_parts) {
            (None, x) => x,
            (x, None) => x,
            (Some(a), Some(b)) => Some(absorb_named(
                a,
                b,
                |part| part.path.clone(),
                absorb_xml_part,
                |part, diff| {
                    let _ = apply_xml_part(part, diff);
                },
            )),
        };
    }
}
//#endregion 🔖️Apply

//#region 🔖️DiffAlgebra
impl DiffAlgebra<XlsxSnapshot> for XlsxDiff {
    fn inverse(&self, base: &XlsxSnapshot) -> Self {
        XlsxDiff { opc: self.opc.as_ref().map(|d| inverse_opc_diff(&base.opc, d)), xml_parts: self.xml_parts.as_ref().map(|diff| inverse_named(&base.xml_parts, diff, |part| part.path.clone(), inverse_xml_part)) }
    }

    fn between(base: &XlsxSnapshot, other: &XlsxSnapshot) -> Self {
        XlsxDiff { opc: diff_opc(&base.opc, &other.opc), xml_parts: between_named(&base.xml_parts, &other.xml_parts, |part| part.path.clone(), diff_xml_part) }
    }

    fn is_empty(&self) -> bool {
        self.opc.is_none() && self.xml_parts.is_none()
    }
}
//#endregion 🔖️DiffAlgebra

//#region 🔖️MutationConstructors
/// 🧩 Builds the sparse field-by-field diff for a `SetSnapshot` mutation. No `snapshot:
/// Option<XlsxSnapshot>` full-replace slot — this IS `XlsxDiff::between`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &XlsxSnapshot, next: &XlsxSnapshot) -> XlsxDiff {
    XlsxDiff::between(base, next)
}
//#endregion 🔖️MutationConstructors

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: **hand-rolled** `protocol::DiffCodec` for `XlsxDiff` — required per the doc comment on
/// `XlsxDiff` itself (real `cargo check` failure: `XlsxCellValue: DslField` not satisfied, plus the
/// generic `NamedTripleDiff<K,D,T>` collection type has no `DslField` impl either). Same grammar
/// style `GifDiff`/`SvgDiff`'s hand-rolled codecs use (bracket-depth-aware split, hex for
/// strings/bytes, `[0]`/`[1,x]` for `Option<T>`, `[removed];[modified];[added]` for collection
/// triples) — see `f6-recon-report.md` §5 for the primitive rationale; this file re-derives its own
/// copies of the small helper functions since each hand-rolled codec is self-contained (no shared
/// "hand-roll helpers" module exists yet). One addition beyond the gif/svg precedent: a GENERIC
/// `enc_triple`/`dec_triple` pair, since `NamedTripleDiff<K,D,T>` is reused across the canonical
/// content-type, binary-part, relationship-list, relationship-owner, and XML-part collections — writing near-identical
/// bespoke encoders would violate this ticket's "concise code" rule for no benefit.
//#region 🔖️Primitives












//#endregion 🔖️Primitives

//#region 🔖️CellValueCodec




//#endregion 🔖️CellValueCodec

//#region 🔖️WorkbookCodec


//#endregion 🔖️WorkbookCodec

//#region 🔖️BinaryCodecs
/// 🧪️ FG-wave: real recursive BINARY twins of every text-form codec above, backing the upgraded
/// `DiffBinary::encode_diff`/`decode_diff` below (and, via re-export, `../🧬️mutations/🦀️.rs`'s
/// own upgraded `OpBinary`) — replaces F6's `print_diff().into_bytes()` text-as-binary shortcut.
/// Real LEB128-varint-framed length-prefixed strings/bytes (`store::pack_rt::write_varint_u64` +
/// `store::ByteReader`), 1-byte tri-state presence tags, and 1-byte enum-variant tags — genuinely
/// structured binary, never hex-ASCII text reused as "binary". Same shape docx's own
/// `🔺️diff/🦀️.rs` `BinaryPrimitives`/`ValueBinaryCodecs`/`GenericTripleBinaryCodecs`/
/// `DiffValueBinaryCodecs` regions establish (this wave's OPC pattern-setter); duplicated here
/// (not imported) per this repo's per-artifact hand-roll convention (no shared "hand-roll
/// helpers" module exists yet, see this file's own `HandcraftedDiffCodec` doc comment).
//#region 🔖️BinaryPrimitives




//#endregion 🔖️BinaryPrimitives

//#region 🔖️ValueBinaryCodecs








//#endregion 🔖️ValueBinaryCodecs
//#endregion 🔖️BinaryCodecs

//#region 🔖️TopLevel

//#endregion 🔖️TopLevel

//#region 🔖️DemoCases
/// 🧪️ FG-wave: representative `XlsxSnapshot`/`XlsxDiff` values (both top-level fields, every
/// `XlsxCellValue` variant incl. `Formula.cached`, the OPC layer's content-types/parts/
/// relationships-by-owner triples incl. `OpcTargetMode::External`) — the single source of truth
/// reused by `diff_codec_text_binary_roundtrip_law` below AND by `⚙️engine/🦀️.rs`'s
/// `diff_grammar_conformance_law`/`protocol_walk_law` conformance tests, same shape docx's own
/// `snapshot_a()`/`snapshot_b()`/`demo_diff_cases()` establish (this wave's OPC pattern-setter).
/// Promoted from the former test-only `sample_a`/`sample_b` (renamed for the same convention).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn snapshot_a() -> XlsxSnapshot {
    crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![
            XlsxSheet { name: "Sheet1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(1.0) }] },
            XlsxSheet { name: "ToDrop".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::SharedString(0) }] },
        ],
        shared_strings: vec!["hello".into()],
    })
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn snapshot_b() -> XlsxSnapshot {
    let mut snap = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![
            XlsxSheet {
                name: "Sheet1".into(),
                cells: vec![
                    XlsxCell { row: 1, col: 0, value: XlsxCellValue::Boolean(true) },
                    XlsxCell { row: 2, col: 2, value: XlsxCellValue::Formula { expr: "SUM(A1:A2)".into(), cached: Some(Box::new(XlsxCellValue::Number(-3.5))) } },
                    XlsxCell { row: 3, col: 0, value: XlsxCellValue::InlineString("brand new, with: odd [chars]".into()) },
                    XlsxCell { row: 4, col: 0, value: XlsxCellValue::Empty },
                ],
            },
            XlsxSheet { name: "Added".into(), cells: vec![] },
        ],
        shared_strings: vec!["hello".into(), "world".into()],
    });
    snap.opc.content_types.set_default("added", "application/octet-stream");
    snap.opc.set_part("xl/added.xml", "application/xml", b"fresh".to_vec());
    snap.opc.add_relationship("xl/added.xml", "rId9", "http://example/added", "media/added.png");
    snap.opc.relationships.relationships_mut("xl/added.xml").unwrap()[0].target_mode = OpcTargetMode::External;
    snap
}

/// 🧪️ The demo cases proper — `default()` (empty diff) plus every real `between()` shape (both
/// directions, and the trivially-empty self-diff).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<XlsxDiff> {
    let a = snapshot_a();
    let b = snapshot_b();
    vec![XlsxDiff::default(), XlsxDiff::between(&a, &b), XlsxDiff::between(&b, &a), XlsxDiff::between(&a, &a)]
}
//#endregion 🔖️DemoCases
//#endregion 🔖️HandcraftedDiffCodec

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
mod handcrafted_diff_codec_tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️result-apply/🦀️.rs"]
mod result_apply_tests;
