//! 🧬️ XlsxMutation — document mutation dispatch. Every variant's `diff()` is handcrafted (never
//! apply-and-capture) and every variant's `inverse()` is handcrafted, key/index-aware.

use crate::schema::diff::XlsxDiff;
use semio_s_artifact_stdio_zip::opc::diff::{OpcContentTypeEntriesDelta, OpcContentTypePatch, OpcContentTypeRow, OpcContentTypesDiff, OpcDiff};


#[cfg(test)]
use crate::schema::snapshot::XlsxCell;
#[cfg(test)]
use crate::schema::snapshot::XlsxWorkbook;
use crate::schema::snapshot::{XlsxCellValue, XlsxSheet};
use crate::XlsxSnapshot;
use protocol::Mutation;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlNode};
#[cfg(test)]
use semio_s_artifact_stdio_zip::opc::OpcRelationship;
#[cfg(test)]
use semio_s_artifact_stdio_zip::opc::OpcTargetMode;

//#region 🔖️Mutations
#[path = "🧭️canonical-edit/🦀️.rs"]
mod canonical_edit;
#[path = "🧭️edit-rules/🦀️.rs"]
pub mod edit_rules;
#[path = "🔗opc-layer/🦀️.rs"]
pub(crate) mod opc_layer;
#[path = "📎set-relationship/🦀️.rs"]
pub mod set_relationship;
#[path = "🧷remove-relationship/🦀️.rs"]
pub mod remove_relationship;
#[path = "📇set-content-type/🦀️.rs"]
pub mod set_content_type;
#[path = "🧺remove-content-type/🦀️.rs"]
pub mod remove_content_type;
use canonical_edit::XlsxPlan;
#[path = "🧭️cell-address/🦀️.rs"]
pub mod cell_address;
#[path = "➕️insert-cell/🦀️.rs"]
pub mod insert_cell;
#[path = "📥️insert-shared-string/🦀️.rs"]
pub mod insert_shared_string;
#[path = "➕insert-sheet/🦀️.rs"]
pub mod insert_sheet;
/// 📐️ Typed content mutation for `stdio.xlsx`. It addresses sheets by NAME (identity), cells by revision-bound
/// canonical addresses, and shared strings by index. Every leaf builds its own sparse diff and concrete inverse.
//#region 🔖️Leaves
#[path = "🧽️remove-cell/🦀️.rs"]
pub mod remove_cell;
#[path = "📤️remove-shared-string/🦀️.rs"]
pub mod remove_shared_string;
#[path = "➖remove-sheet/🦀️.rs"]
pub mod remove_sheet;
#[path = "🏷️rename-sheet/🦀️.rs"]
pub mod rename_sheet;
#[path = "✍️set-cell/🦀️.rs"]
pub mod set_cell;
#[path = "🔤️set-shared-string/🦀️.rs"]
pub mod set_shared_string;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = XlsxSnapshot, diff = XlsxDiff, schema = "XlsxMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum XlsxMutation {
    /// ➕️ Inserts a brand-new sheet (possibly pre-populated with cells).
    InsertSheet(insert_sheet::InsertSheet),
    /// ➖️ Removes the sheet named `name`.
    RemoveSheet(remove_sheet::RemoveSheet),
    /// 🏷️ Renames the sheet named `name` to `new_name` (a remove-old+add-new at the diff level —
    /// `name` is the sheet's identity, see the snapshot module's doc comment).
    RenameSheet(rename_sheet::RenameSheet),
    /// ✍️ Replaces one revision-bound canonical SpreadsheetML cell value.
    SetCell(set_cell::SetCell),
    /// ➕️ Inserts one cell into a revision-bound canonical SpreadsheetML vacancy.
    InsertCell(insert_cell::InsertCell),
    /// ➖️ Removes the cell at `(row, col)` in sheet `sheet_name`.
    RemoveCell(remove_cell::RemoveCell),
    /// ➕️ Appends a new shared string.
    InsertSharedString(insert_shared_string::InsertSharedString),
    /// ➖️ Removes the shared string at `index`.
    RemoveSharedString(remove_shared_string::RemoveSharedString),
    /// ✍️ Replaces the shared string at `index`.
    SetSharedString(set_shared_string::SetSharedString),
    /// 📎 Writes one relationship of an owner part (inserted at `index`, or changed in place).
    SetRelationship(set_relationship::SetRelationship),
    /// 🧷 Removes one relationship of an owner part.
    RemoveRelationship(remove_relationship::RemoveRelationship),
    /// 📇 Writes one `[Content_Types].xml` entry (inserted at `index`, or changed in place).
    SetContentType(set_content_type::SetContentType),
    /// 🧺 Removes one `[Content_Types].xml` entry.
    RemoveContentType(remove_content_type::RemoveContentType),
}

/// 🧾️ Kebab-case spelling of every `XlsxMutation` variant, in declaration order — the exhaustive
/// mutation catalog `xlsx-ecma-376-base` (`../../🔣️oracle.json`) is measured against
/// this exact list. `kinds_match_enum_and_catalog` proves it never drifts from either side.
pub const KINDS: &[&str] = &["insert-sheet", "remove-sheet", "rename-sheet", "set-cell", "insert-cell", "remove-cell", "insert-shared-string", "remove-shared-string", "set-shared-string", "set-relationship", "remove-relationship", "set-content-type", "remove-content-type"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot` through its own diff — the diff is the single semantics source, never a separate imperative apply path.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
#[cfg(test)]
pub fn apply_xlsx_mutation(snapshot: &mut XlsxSnapshot, mutation: &XlsxMutation) -> protocol::MutationOutcome<XlsxDiff> {
    let outcome = Mutation::diff(mutation, snapshot);
    match protocol::apply_diff(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
/// 🧾️ The outcome of a prepared plan: its compact diff, or the refusal naming why it cannot be built.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn plan_outcome(plan: Result<XlsxPlan, String>) -> protocol::MutationOutcome<XlsxDiff> {
    match plan {
        Ok(plan) => protocol::MutationOutcome::new(plan.diff),
        Err(message) => protocol::MutationOutcome::error("mutation.target-mismatch", message, ["xmlParts"]),
    }
}

/// ↩️ The concrete inverse of a prepared plan; a refused plan has nothing to undo.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn plan_inverse(plan: Result<XlsxPlan, String>) -> Vec<XlsxMutation> {
    plan.map(|plan| plan.inverse).unwrap_or_default()
}
/// 🌳 The XML diff that rewrites every attribute value equal to a member of `from` into `to` anywhere under `node` -- a namespace declaration is an
/// ordinary attribute, so one walk covers `xmlns`, `xmlns:r` and whatever prefixed alias a package uses. `None` when nothing under `node` declares one of `from`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn retarget_attribute_values_diff(node: &XmlNode, from: &[&str], to: &str) -> Option<semio_s_artifact_stdio_xml::schema::diff::XmlNodeDiff> {
    use semio_s_artifact_stdio_xml::schema::diff::{XmlAttrModified, XmlAttributesDiff, XmlChildModified, XmlChildrenDiff, XmlElementDiff, XmlNodeDiff};
    let XmlNode::Element { attrs, children, .. } = node else { return None };
    let modified: Vec<XmlAttrModified> = attrs.iter().filter(|attr| from.contains(&attr.value.as_str()) && attr.value != to).map(|attr| XmlAttrModified { name: attr.name.clone(), value: to.to_string() }).collect();
    let attributes = (!modified.is_empty()).then(|| XmlAttributesDiff { order: attrs.iter().map(|attr| attr.name.clone()).collect(), removed: Vec::new(), modified, added: Vec::new() });
    let nested: Vec<XmlChildModified> = children.iter().enumerate().filter_map(|(index, child)| retarget_attribute_values_diff(child, from, to).map(|diff| XmlChildModified { index, diff })).collect();
    let children = (!nested.is_empty()).then(|| XmlChildrenDiff { removed: Vec::new(), modified: nested, added: Vec::new() });
    (attributes.is_some() || children.is_some()).then(|| XmlNodeDiff::Element(XmlElementDiff { name: None, attributes, children }))
}

/// 🧾️ The diff that applies one XML `root` edit to each named part: `edits` pairs a part path with the root diff it takes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn root_edits_diff(edits: Vec<(String, semio_s_artifact_stdio_xml::schema::diff::XmlNodeDiff)>) -> XlsxDiff {
    canonical_edit::modified_parts(edits.into_iter().map(|(key, root)| (key, semio_s_artifact_stdio_xml::schema::diff::XmlDiff { root: Some(root), ..Default::default() })).collect())
}

/// 🌿️ The attribute edit that sets (`Some`) or removes (`None`) attribute `name` on the ROOT element of `document`; `None` when nothing changes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn root_attribute_diff(document: &semio_s_artifact_stdio_xml::schema::snapshot::XmlDocument, name: &str, value: Option<&str>) -> Option<semio_s_artifact_stdio_xml::schema::diff::XmlNodeDiff> {
    use semio_s_artifact_stdio_xml::schema::diff::{XmlAttrAdded, XmlAttrModified, XmlAttributesDiff, XmlElementDiff, XmlNodeDiff};
    let Some(XmlNode::Element { attrs, .. }) = document.root.as_ref() else { return None };
    let existing = attrs.iter().find(|attr| attr.name == name);
    let mut order: Vec<String> = attrs.iter().map(|attr| attr.name.clone()).collect();
    let attributes = match (existing, value) {
        (Some(attr), Some(value)) if attr.value != value => XmlAttributesDiff { order, modified: vec![XmlAttrModified { name: name.into(), value: value.into() }], ..Default::default() },
        (None, Some(value)) => {
            order.push(name.to_string());
            XmlAttributesDiff { order, added: vec![XmlAttrAdded { name: name.into(), value: value.into() }], ..Default::default() }
        }
        (Some(_), None) => {
            order.retain(|attr| attr != name);
            XmlAttributesDiff { order, removed: vec![name.into()], ..Default::default() }
        }
        _ => return None,
    };
    Some(XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: Some(attributes), children: None }))
}

/// 🧩️ The diff that adds XML part `path` of `content_type` with `document` at `index` (appended when `None`), together with its content-type override at
/// `override_index` (appended when `None`). `None` when a part of that name already exists.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn insert_xml_part_diff(base: &XlsxSnapshot, path: &str, content_type: &str, document: &semio_s_artifact_stdio_xml::schema::snapshot::XmlDocument, index: Option<usize>, override_index: Option<usize>) -> Option<XlsxDiff> {
    use crate::schema::diff::XlsxXmlPartsDelta;
    use semio_s_artifact_stdio_contract::list_delta::insertion_index;
    let key = path.trim_start_matches('/');
    if base.xml_part(key).is_some() || base.opc.part(key).is_some() {
        return None;
    }
    let name = format!("/{key}");
    let list = &base.opc.content_types.overrides;
    let overrides = match list.iter().position(|(existing, _)| *existing == name) {
        Some(at) => (list[at].1 != content_type).then(|| OpcContentTypeEntriesDelta::modification(&name, OpcContentTypePatch { content_type: Some(content_type.to_string()) })),
        None => Some(OpcContentTypeEntriesDelta::insertion(insertion_index(list.len(), override_index), OpcContentTypeRow { name, content_type: content_type.to_string() })),
    };
    Some(XlsxDiff {
        opc: overrides.map(|overrides| OpcDiff { content_types: Some(OpcContentTypesDiff { defaults: None, overrides: Some(overrides) }), ..Default::default() }),
        xml_parts: Some(XlsxXmlPartsDelta::insertion(insertion_index(base.xml_parts.len(), index), crate::schema::snapshot::XlsxXmlPart { path: key.to_string(), content_type: content_type.to_string(), document: document.clone() })),
    })
}

/// 🧩️ The diff that removes XML part `path` and its content-type override; `None` when no such part exists.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn remove_xml_part_diff(base: &XlsxSnapshot, path: &str) -> Option<XlsxDiff> {
    use crate::schema::diff::XlsxXmlPartsDelta;
    let key = path.trim_start_matches('/');
    let at = base.xml_parts.iter().position(|part| part.path == key)?;
    let name = format!("/{key}");
    let overrides = &base.opc.content_types.overrides;
    let content_types = overrides.iter().position(|(existing, _)| *existing == name).map(|position| OpcContentTypesDiff { defaults: None, overrides: Some(OpcContentTypeEntriesDelta::removal_by_id(name.clone(), position)) });
    Some(XlsxDiff { opc: content_types.map(|content_types| OpcDiff { content_types: Some(content_types), ..Default::default() }), xml_parts: Some(XlsxXmlPartsDelta::removal_by_id(key, at)) })
}

/// 🧩️ The diff that retypes XML part `path`: the part's own `content_type` and its content-type override move together. Without `override_index` a type the extension
/// default already yields needs no explicit override and drops a differing one; with it an explicit override is always written at that position. Empty when the part is
/// absent or already carries `content_type` in both places.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn retype_xml_part_diff(base: &XlsxSnapshot, path: &str, content_type: &str, override_index: Option<usize>) -> XlsxDiff {
    use crate::schema::diff::{XlsxXmlPartDiff, XlsxXmlPartsDelta};
    use semio_s_artifact_stdio_contract::list_delta::insertion_index;
    let Some(part) = base.xml_part(path) else { return XlsxDiff::default() };
    let name = format!("/{}", path.trim_start_matches('/'));
    let default = path.rsplit('.').next().and_then(|extension| base.opc.content_types.defaults.iter().find(|(existing, _)| existing.eq_ignore_ascii_case(extension))).map(|(_, value)| value.as_str());
    let list = &base.opc.content_types.overrides;
    let overrides = match list.iter().position(|(existing, _)| *existing == name) {
        Some(at) if list[at].1 == content_type => None,
        Some(at) if override_index.is_none() && default == Some(content_type) => Some(OpcContentTypeEntriesDelta::removal_by_id(name, at)),
        Some(_) => Some(OpcContentTypeEntriesDelta::modification(&name, OpcContentTypePatch { content_type: Some(content_type.to_string()) })),
        None if override_index.is_none() && default == Some(content_type) => None,
        None => Some(OpcContentTypeEntriesDelta::insertion(insertion_index(list.len(), override_index), OpcContentTypeRow { name, content_type: content_type.to_string() })),
    };
    let retyped = (part.content_type != content_type).then(|| XlsxXmlPartsDelta::modification(&part.path, XlsxXmlPartDiff { content_type: Some(content_type.to_string()), document: None }));
    XlsxDiff { opc: overrides.map(|overrides| OpcDiff { content_types: Some(OpcContentTypesDiff { defaults: None, overrides: Some(overrides) }), ..Default::default() }), xml_parts: retyped }
}

/// 🧭️ Where XML part `path` and its explicit content-type override sit in their lists: `(part index, override index)`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn xml_part_positions(base: &XlsxSnapshot, path: &str) -> Option<(usize, Option<usize>)> {
    let key = path.trim_start_matches('/');
    let part = base.xml_parts.iter().position(|part| part.path == key)?;
    let name = format!("/{key}");
    Some((part, base.opc.content_types.overrides.iter().position(|(existing, _)| *existing == name)))
}
//#endregion 🔖️MutationTrait

//#region 🔖️DemoCases
/// 🧪️ FG-wave: representative `XlsxSnapshot`/`XlsxMutation` fixtures -- the single source of
/// truth reused by this file's own `mutation_diff_law`/`inverse_law`/`op_text_binary_roundtrip_law`
/// tests below AND by `⚙️engine/🦀️.rs`'s `ops_grammar_conformance_law`/`protocol_walk_law`
/// conformance tests, same shape docx's own `demo_mutation_cases()` establishes (this wave's OPC
/// pattern-setter). Promoted from the former test-only `fixture`/`sweep_a`/`sweep_b`/
/// `sample_mutations` (the last renamed for the same convention).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn fixture() -> XlsxSnapshot {
    opc_layer::with_demo_entries(crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![XlsxSheet { name: "Sheet1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(1.0) }] }, XlsxSheet { name: "Sheet2".into(), cells: vec![] }],
        shared_strings: vec!["hello".into()],
    }))
}

//#region 🔖️Fixtures
/// 📦️ Content type of the sweep fixtures' binary OPC parts.
#[cfg(test)]
const SWEEP_BINARY_CONTENT_TYPE: &str = "application/octet-stream";

/// 🌱 `sweep_a`/`sweep_b`: differ in EVERY lane the snapshot carries. The authoritative XML parts
/// (built by `build_minimal_xlsx` from each workbook): the workbook part (sheet `toDrop` replaced by
/// `added`), the `toModify` worksheet (cells removed + modified + added), the shared-string table
/// (index-keyed, a different length on each side, so `a -> b` removes + modifies and `b -> a` adds +
/// modifies). The lossless OPC lane: binary parts (one removed, one modified in bytes AND content type,
/// one added), a content-type default and overrides, and part-owned relationships (one owner removed,
/// one modified, one added — the added one External).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn sweep_a() -> XlsxSnapshot {
    let mut snapshot = crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![
            XlsxSheet { name: "toModify".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(1.0) }, XlsxCell { row: 2, col: 0, value: XlsxCellValue::Boolean(false) }] },
            XlsxSheet { name: "stay".into(), cells: vec![] },
            XlsxSheet { name: "toDrop".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::SharedString(0) }] },
        ],
        // 🎯️ Length 3 vs `sweep_b`'s 2: per this ticket's "known structural trap" note, a
        // single same-direction `between()` over an index-keyed (pairwise-position-matched)
        // collection can never show BOTH `removed` AND `added` from one call -- so `a -> b`
        // exercises `shared_strings.removed` (index 2, since `b` is shorter) +
        // `shared_strings.modified` (index 1); `b -> a` (asserted separately in
        // `field_sweep`) exercises `shared_strings.added` (the same index 2, recurring).
        shared_strings: vec!["keep".into(), "toModify".into(), "toRemove".into()],
    });
    snapshot.opc.content_types.set_default("bin", SWEEP_BINARY_CONTENT_TYPE);
    snapshot.opc.set_part("xl/media/toModify.bin", SWEEP_BINARY_CONTENT_TYPE, b"old".to_vec());
    snapshot.opc.set_part("xl/media/toRemove.bin", SWEEP_BINARY_CONTENT_TYPE, b"gone".to_vec());
    snapshot.opc.relationships.replace_owner("xl/media/toModify.bin".into(), vec![OpcRelationship { id: "rId1".into(), rel_type: "http://example/sweep".into(), target: "old.bin".into(), target_mode: OpcTargetMode::Internal }]);
    snapshot.opc.relationships.replace_owner("xl/media/toRemove.bin".into(), vec![OpcRelationship { id: "rId1".into(), rel_type: "http://example/sweep".into(), target: "gone.bin".into(), target_mode: OpcTargetMode::Internal }]);
    snapshot.opc.parts.sort_by(|left, right| left.path.cmp(&right.path));
    snapshot
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn sweep_b() -> XlsxSnapshot {
    let mut snapshot = crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![
            XlsxSheet {
                name: "toModify".into(),
                cells: vec![
                    // row (1,0) survives with a changed value; row (2,0) is dropped; row
                    // (3,0) is a NET-NEW cell -- exercises `cells.removed` +
                    // `cells.modified` + `cells.added` all in the SAME sheet-modify.
                    XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(2.0) },
                    XlsxCell { row: 3, col: 0, value: XlsxCellValue::Formula { expr: "SUM(A1:A2)".into(), cached: Some(Box::new(XlsxCellValue::Number(3.0))) } },
                ],
            },
            XlsxSheet { name: "stay".into(), cells: vec![] },
            XlsxSheet { name: "added".into(), cells: vec![XlsxCell { row: 1, col: 1, value: XlsxCellValue::InlineString("brand new".into()) }] },
        ],
        // 🎯️ Length 2: index 2 ("toRemove") no longer exists — exercises
        // `shared_strings.removed` on `a -> b` (see `sweep_a`'s doc comment); the same
        // index recurs as `shared_strings.added` on `b -> a`.
        shared_strings: vec!["keep".into(), "toModify-changed".into()],
    });
    snapshot.opc.content_types.set_default("bin", SWEEP_BINARY_CONTENT_TYPE);
    snapshot.opc.content_types.set_default("dat", SWEEP_BINARY_CONTENT_TYPE);
    snapshot.opc.set_part("xl/media/toModify.bin", "application/x-semio-sweep", b"new".to_vec());
    snapshot.opc.set_part("xl/media/added.dat", SWEEP_BINARY_CONTENT_TYPE, b"fresh".to_vec());
    snapshot.opc.relationships.replace_owner("xl/media/toModify.bin".into(), vec![OpcRelationship { id: "rId1".into(), rel_type: "http://example/sweep".into(), target: "new.bin".into(), target_mode: OpcTargetMode::Internal }]);
    snapshot.opc.relationships.replace_owner("xl/media/added.dat".into(), vec![OpcRelationship { id: "rId1".into(), rel_type: "http://example/sweep".into(), target: "added.bin".into(), target_mode: OpcTargetMode::External }]);
    snapshot.opc.parts.sort_by(|left, right| left.path.cmp(&right.path));
    snapshot
}
//#endregion 🔖️Fixtures

/// 🧪️ The demo cases proper -- one representative `XlsxMutation` per variant.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<XlsxMutation> {
    let base = fixture();
    let address = cell_address::xlsx_cell_address(&base, "Sheet1", 1, 0).expect("fixture cell address");
    let vacancy = cell_address::xlsx_cell_vacancy_address(&base, "Sheet1", 2, 1).expect("fixture cell vacancy address");
    vec![
        XlsxMutation::InsertSheet(insert_sheet::InsertSheet::minted(&base, XlsxSheet { name: "x".into(), cells: vec![] }, None).expect("the gesture mints the sheet slot")),
        XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name: "Sheet2".into() }),
        XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name: "Sheet2".into(), new_name: "Renamed".into() }),
        XlsxMutation::SetCell(set_cell::SetCell { address, value: XlsxCellValue::Boolean(true), node: None }),
        XlsxMutation::InsertCell(insert_cell::InsertCell { address: vacancy, value: XlsxCellValue::InlineString("created".into()), node: None }),
        XlsxMutation::RemoveCell(remove_cell::RemoveCell { address: cell_address::xlsx_cell_address(&base, "Sheet1", 1, 0).expect("fixture cell address") }),
        XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value: "z".into(), index: None, node: None }),
        XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index: 0 }),
        XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index: 0, value: "y".into(), node: None }),
    ]
    .into_iter()
    .chain(opc_layer::demo_cases())
    .collect()
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `🦀️.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧪️FixtureTests
