//! 🔗️ The OPC layer's relationship and content-type rows as concrete diffs: each builder reads `base` and states the sparse diff of its one row — the
//! written or removed relationship of an owner, the written or removed `[Content_Types].xml` entry — with the after-list position an inserted row takes.

use super::*;
use crate::schema::diff::XlsxDiff;
use semio_s_artifact_stdio_contract::list_delta::insertion_index;
use semio_s_artifact_stdio_zip::opc::diff::{OpcContentTypeEntriesDelta, OpcContentTypePatch, OpcContentTypeRow, OpcContentTypesDiff, OpcDiff, OpcOwnerPatch, OpcOwnerRow, OpcOwnerRemoval, OpcOwnersDelta, OpcRelationshipPatch, OpcRelationshipsDelta};
use semio_s_artifact_stdio_zip::opc::{OpcPackage, OpcRelationship, OpcTargetMode};

/// 📦️ Reads the OPC package of `base` through `read`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn with_package<R>(base: &XlsxSnapshot, read: impl FnOnce(&OpcPackage) -> R) -> Result<R, String> {
    Ok(read(&base.opc))
}

/// 🧾️ The outcome of one OPC-row diff: its diff, or the refusal naming why the row cannot be written.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn outcome(diff: Result<Result<XlsxDiff, String>, String>) -> protocol::MutationOutcome<XlsxDiff> {
    match diff.and_then(|diff| diff) {
        Ok(diff) => protocol::MutationOutcome::new(diff),
        Err(message) => protocol::MutationOutcome::error("mutation.target-mismatch", message, Vec::<String>::new()),
    }
}

/// 🔎️ Relationship `id` of `owner` with the position it holds among the owner's relationships.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn relationship_at(opc: &OpcPackage, owner: &str, id: &str) -> Option<(usize, OpcRelationship)> {
    let list = opc.relationships.relationships(owner)?;
    list.iter().position(|existing| existing.id == id).map(|at| (at, list[at].clone()))
}

/// ✍️ The diff that writes `relationship` into `owner`: a new relationship is inserted at `index` (last by default), an existing id is changed in place, and an
/// owner without relationships gains its first at its sorted position among the owners.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn relationship_write_diff(opc: &OpcPackage, owner: &str, relationship: &OpcRelationship, index: Option<usize>) -> Result<XlsxDiff, String> {
    let relationships = match opc.relationships.relationships(owner) {
        None => OpcOwnersDelta::insertion(opc.relationships.groups().take_while(|(existing, _)| existing.as_str() < owner).count(), OpcOwnerRow { owner: owner.to_string(), relationships: vec![relationship.clone()] }),
        Some(list) => {
            let rows = match list.iter().position(|existing| existing.id == relationship.id) {
                Some(at) if list[at] == *relationship => return Ok(XlsxDiff::default()),
                Some(at) => OpcRelationshipsDelta::modification(
                    &relationship.id,
                    OpcRelationshipPatch {
                        rel_type: (list[at].rel_type != relationship.rel_type).then(|| relationship.rel_type.clone()),
                        target: (list[at].target != relationship.target).then(|| relationship.target.clone()),
                        target_mode: (list[at].target_mode != relationship.target_mode).then_some(relationship.target_mode),
                    },
                ),
                None => OpcRelationshipsDelta::insertion(insertion_index(list.len(), index), relationship.clone()),
            };
            OpcOwnersDelta::modification(owner, OpcOwnerPatch { relationships: rows })
        }
    };
    Ok(XlsxDiff { opc: Some(OpcDiff { relationships: Some(relationships), ..Default::default() }), ..Default::default() })
}

/// ➖️ The diff that removes relationship `id` of `owner`; an owner left without relationships disappears with its last one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn relationship_removal_diff(opc: &OpcPackage, owner: &str, id: &str) -> Result<XlsxDiff, String> {
    let list = opc.relationships.relationships(owner).ok_or_else(|| format!("no relationships are owned by {owner:?}"))?;
    let at = list.iter().position(|existing| existing.id == id).ok_or_else(|| format!("{owner:?} owns no relationship {id:?}"))?;
    let relationships = if list.len() == 1 {
        let position = opc.relationships.groups().position(|(existing, _)| existing == owner).ok_or_else(|| format!("no relationships are owned by {owner:?}"))?;
        OpcOwnersDelta { removed: vec![OpcOwnerRemoval { id: owner.to_string(), index: position }], ..Default::default() }
    } else {
        OpcOwnersDelta::modification(owner, OpcOwnerPatch { relationships: OpcRelationshipsDelta::removal_by_id(id, at) })
    };
    Ok(XlsxDiff { opc: Some(OpcDiff { relationships: Some(relationships), ..Default::default() }), ..Default::default() })
}

/// 📇️ The `[Content_Types].xml` entries of one kind: the extension defaults, or the part-name overrides.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn entries(opc: &OpcPackage, is_override: bool) -> &[(String, String)] {
    if is_override {
        &opc.content_types.overrides
    } else {
        &opc.content_types.defaults
    }
}

/// 🔎️ Entry `name` of the defaults (`is_override` false) or the overrides with its position and content type.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn content_type_at(opc: &OpcPackage, is_override: bool, name: &str) -> Option<(usize, String)> {
    entries(opc, is_override).iter().position(|(key, _)| key == name).map(|at| (at, entries(opc, is_override)[at].1.clone()))
}

/// 🧾️ The diff of one content-type entry list wrapped for its kind.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn content_type_rows(rows: OpcContentTypeEntriesDelta, is_override: bool) -> XlsxDiff {
    let content_types = if is_override { OpcContentTypesDiff { overrides: Some(rows), defaults: None } } else { OpcContentTypesDiff { defaults: Some(rows), overrides: None } };
    XlsxDiff { opc: Some(OpcDiff { content_types: Some(content_types), ..Default::default() }), ..Default::default() }
}

/// ✍️ The diff that gives entry `name` the content type `content_type`: a new entry is added at `index` (last by default), an existing one changed in place.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn content_type_write_diff(opc: &OpcPackage, is_override: bool, name: &str, content_type: &str, index: Option<usize>) -> Result<XlsxDiff, String> {
    let list = entries(opc, is_override);
    if !is_override && list.iter().any(|(key, _)| key != name && key.eq_ignore_ascii_case(name)) {
        return Err(format!("default extension {name:?} already exists in another letter case"));
    }
    let rows = match list.iter().position(|(key, _)| key == name) {
        Some(at) if list[at].1 == content_type => return Ok(XlsxDiff::default()),
        Some(_) => OpcContentTypeEntriesDelta::modification(name, OpcContentTypePatch { content_type: Some(content_type.to_string()) }),
        None => OpcContentTypeEntriesDelta::insertion(insertion_index(list.len(), index), OpcContentTypeRow { name: name.to_string(), content_type: content_type.to_string() }),
    };
    Ok(content_type_rows(rows, is_override))
}

/// ➖️ The diff that removes entry `name`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn content_type_removal_diff(opc: &OpcPackage, is_override: bool, name: &str) -> Result<XlsxDiff, String> {
    let list = entries(opc, is_override);
    let at = list.iter().position(|(key, _)| key == name).ok_or_else(|| format!("the content types hold no {} {name:?}", if is_override { "override" } else { "default" }))?;
    Ok(content_type_rows(OpcContentTypeEntriesDelta::removal_by_id(name, at), is_override))
}

/// 🧪️ The package of `snapshot` with an external root relationship `rIdDemoExternal` and an unused extension default `zzdemo` added: the entries the demo cases remove.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn with_demo_entries(mut snapshot: XlsxSnapshot) -> XlsxSnapshot {
    let external = OpcRelationship { id: "rIdDemoExternal".into(), rel_type: "http://example.invalid/relationships/demo".into(), target: "https://example.invalid/demo".into(), target_mode: OpcTargetMode::External };
    let owned: Vec<OpcRelationship> = snapshot.opc.relationships.relationships("").into_iter().flatten().cloned().chain(std::iter::once(external)).collect();
    snapshot.opc.relationships.replace_owner(String::new(), owned);
    snapshot.opc.content_types.defaults.push(("zzdemo".into(), "application/x-semio-demo".into()));
    snapshot
}

/// 🧪️ One representative case per OPC-layer kind, in declaration order, valid on a fixture carrying [`with_demo_entries`].
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_cases() -> Vec<XlsxMutation> {
    vec![
        XlsxMutation::SetRelationship(set_relationship::SetRelationship { owner: String::new(), id: "rIdDemoAdded".into(), rel_type: "http://example.invalid/relationships/added".into(), target: "https://example.invalid/added".into(), external: true, index: Some(0) }),
        XlsxMutation::RemoveRelationship(remove_relationship::RemoveRelationship { owner: String::new(), id: "rIdDemoExternal".into() }),
        XlsxMutation::SetContentType(set_content_type::SetContentType { is_override: false, name: "zzadded".into(), content_type: "application/x-semio-demo".into(), index: Some(0) }),
        XlsxMutation::RemoveContentType(remove_content_type::RemoveContentType { is_override: false, name: "zzdemo".into() }),
    ]
}
