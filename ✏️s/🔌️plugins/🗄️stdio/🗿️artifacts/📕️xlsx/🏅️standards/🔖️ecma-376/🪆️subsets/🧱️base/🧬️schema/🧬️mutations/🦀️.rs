//! 🧬️ XlsxMutation — document mutation dispatch. Every variant's `diff()` is handcrafted (never
//! apply-and-capture) and every variant's `inverse()` is handcrafted, key/index-aware.

use crate::schema::diff::XlsxDiff;


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
}

/// 🧾️ Kebab-case spelling of every `XlsxMutation` variant, in declaration order — the exhaustive
/// mutation catalog `xlsx-ecma-376-base` (`../../🔣️oracle.json`) is measured against
/// this exact list. `kinds_match_enum_and_catalog` proves it never drifts from either side.
pub const KINDS: &[&str] = &["insert-sheet", "remove-sheet", "rename-sheet", "set-cell", "insert-cell", "remove-cell", "insert-shared-string", "remove-shared-string", "set-shared-string"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot` through its own diff — the diff is the single semantics source, never a separate imperative apply path.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
/// 🧮️ The cell and shared-string mutations that carry `base` to `next`, replayed against `base` step by step so every revision-bound address is fresh.
/// `None` when `next` changes anything those kinds do not address (the sheet list, the OPC layer, which parts exist), detected by the final comparison.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn net_mutations(base: &XlsxSnapshot, next: &XlsxSnapshot) -> Option<Vec<XlsxMutation>> {
    if base == next {
        return Some(Vec::new());
    }
    let before = base.project_workbook().ok()?;
    let after = next.project_workbook().ok()?;
    if before.sheets.iter().map(|sheet| &sheet.name).ne(after.sheets.iter().map(|sheet| &sheet.name)) || before.shared_strings.len() != after.shared_strings.len() {
        return None;
    }
    let mut state = base.clone();
    let mut leaves = Vec::new();
    let mut step = |state: &mut XlsxSnapshot, leaf: Option<XlsxMutation>| -> Option<()> {
        let leaf = leaf?;
        if !apply_xlsx_mutation(state, &leaf).messages().is_empty() {
            return None;
        }
        leaves.push(leaf);
        Some(())
    };
    for (index, (old, new)) in before.shared_strings.iter().zip(&after.shared_strings).enumerate() {
        if old != new {
            step(&mut state, Some(XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index, value: new.clone() })))?;
        }
    }
    for (old_sheet, new_sheet) in before.sheets.iter().zip(&after.sheets) {
        for cell in &new_sheet.cells {
            match old_sheet.cells.iter().find(|candidate| candidate.row == cell.row && candidate.col == cell.col) {
                Some(old) if old.value == cell.value => {}
                Some(_) => {
                    let leaf = cell_address::xlsx_cell_address(&state, &old_sheet.name, cell.row, cell.col).ok().map(|address| XlsxMutation::SetCell(set_cell::SetCell { address, value: cell.value.clone() }));
                    step(&mut state, leaf)?;
                }
                None => {
                    let leaf = cell_address::xlsx_cell_vacancy_address(&state, &old_sheet.name, cell.row, cell.col).ok().map(|address| XlsxMutation::InsertCell(insert_cell::InsertCell { address, value: cell.value.clone() }));
                    step(&mut state, leaf)?;
                }
            }
        }
        for cell in &old_sheet.cells {
            if !new_sheet.cells.iter().any(|candidate| candidate.row == cell.row && candidate.col == cell.col) {
                let leaf = cell_address::xlsx_cell_address(&state, &old_sheet.name, cell.row, cell.col).ok().map(|address| XlsxMutation::RemoveCell(remove_cell::RemoveCell { address }));
                step(&mut state, leaf)?;
            }
        }
    }
    (state == *next).then_some(leaves)
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

/// 🧩️ The diff that adds XML part `path` of `content_type` with `document` at `index` (appended when `None`), together with its content-type override.
/// `None` when a part of that name already exists.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn insert_xml_part_diff(base: &XlsxSnapshot, path: &str, content_type: &str, document: &semio_s_artifact_stdio_xml::schema::snapshot::XmlDocument, index: Option<usize>) -> Option<XlsxDiff> {
    use crate::schema::diff::{NamedModified, NamedTripleDiff, XlsxOpcContentTypesDiff, XlsxOpcDiff};
    let key = path.trim_start_matches('/');
    if base.xml_part(key).is_some() || base.opc.part(key).is_some() {
        return None;
    }
    let position = index.map_or(base.xml_parts.len(), |index| index.min(base.xml_parts.len()));
    let mut order: Vec<String> = base.xml_parts.iter().map(|part| part.path.clone()).collect();
    order.insert(position, key.to_string());
    let appended = position == base.xml_parts.len();
    let name = format!("/{key}");
    let current = base.opc.content_types.overrides.iter().find(|(existing, _)| *existing == name).map(|(_, value)| value.clone());
    let overrides = match current {
        Some(current) if current == content_type => None,
        Some(_) => Some(NamedTripleDiff { modified: vec![NamedModified { key: name, diff: content_type.to_string() }], ..Default::default() }),
        None => Some(NamedTripleDiff { added: vec![(name, content_type.to_string())], ..Default::default() }),
    };
    Some(XlsxDiff {
        opc: overrides.map(|overrides| XlsxOpcDiff { content_types: Some(XlsxOpcContentTypesDiff { defaults: None, overrides: Some(overrides) }), ..Default::default() }),
        xml_parts: Some(NamedTripleDiff { added: vec![crate::schema::snapshot::XlsxXmlPart { path: key.to_string(), content_type: content_type.to_string(), document: document.clone() }], order: if appended { Vec::new() } else { order }, ..Default::default() }),
    })
}

/// 🧩️ The diff that removes XML part `path` and its content-type override; `None` when no such part exists.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn remove_xml_part_diff(base: &XlsxSnapshot, path: &str) -> Option<XlsxDiff> {
    use crate::schema::diff::{NamedTripleDiff, XlsxOpcContentTypesDiff, XlsxOpcDiff};
    let key = path.trim_start_matches('/');
    base.xml_part(key)?;
    let name = format!("/{key}");
    let has_override = base.opc.content_types.overrides.iter().any(|(existing, _)| *existing == name);
    Some(XlsxDiff {
        opc: has_override.then(|| XlsxOpcDiff { content_types: Some(XlsxOpcContentTypesDiff { defaults: None, overrides: Some(NamedTripleDiff { removed: vec![name], ..Default::default() }) }), ..Default::default() }),
        xml_parts: Some(NamedTripleDiff { removed: vec![key.to_string()], ..Default::default() }),
    })
}

/// 🧩️ The diff that retypes XML part `path`: the part's own `content_type` and its content-type override move together. Empty when the part is absent
/// or already carries `content_type` in both places.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn retype_xml_part_diff(base: &XlsxSnapshot, path: &str, content_type: &str) -> XlsxDiff {
    use crate::schema::diff::{NamedModified, NamedTripleDiff, XlsxOpcContentTypesDiff, XlsxOpcDiff, XlsxXmlPartDiff};
    let Some(part) = base.xml_part(path) else { return XlsxDiff::default() };
    let name = format!("/{}", path.trim_start_matches('/'));
    let overrides = match base.opc.content_types.overrides.iter().find(|(existing, _)| *existing == name) {
        Some((_, current)) if current == content_type => None,
        Some(_) => Some(NamedTripleDiff { modified: vec![NamedModified { key: name, diff: content_type.to_string() }], ..Default::default() }),
        None => Some(NamedTripleDiff { added: vec![(name, content_type.to_string())], ..Default::default() }),
    };
    let retyped = (part.content_type != content_type).then(|| NamedTripleDiff { modified: vec![NamedModified { key: part.path.clone(), diff: XlsxXmlPartDiff { content_type: Some(content_type.to_string()), document: None } }], ..Default::default() });
    XlsxDiff { opc: overrides.map(|overrides| XlsxOpcDiff { content_types: Some(XlsxOpcContentTypesDiff { defaults: None, overrides: Some(overrides) }), ..Default::default() }), xml_parts: retyped }
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
    crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![XlsxSheet { name: "Sheet1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(1.0) }] }, XlsxSheet { name: "Sheet2".into(), cells: vec![] }],
        shared_strings: vec!["hello".into()],
    })
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
        XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet: XlsxSheet { name: "x".into(), cells: vec![] }, index: None }),
        XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name: "Sheet2".into() }),
        XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name: "Sheet2".into(), new_name: "Renamed".into() }),
        XlsxMutation::SetCell(set_cell::SetCell { address, value: XlsxCellValue::Boolean(true) }),
        XlsxMutation::InsertCell(insert_cell::InsertCell { address: vacancy, value: XlsxCellValue::InlineString("created".into()) }),
        XlsxMutation::RemoveCell(remove_cell::RemoveCell { address: cell_address::xlsx_cell_address(&base, "Sheet1", 1, 0).expect("fixture cell address") }),
        XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value: "z".into(), index: None }),
        XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index: 0 }),
        XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index: 0, value: "y".into() }),
    ]
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
