//! 🧬️ DocxMutation — document mutation dispatch. Every variant's `diff()` is handcrafted (never
//! apply-and-capture) and every variant's `inverse()` is handcrafted, key/index-aware.

use crate::artifacts::docx::schema::diff::{
    diff_insert_block, diff_insert_style, diff_remove_block, diff_remove_part, diff_remove_style, diff_set_block_content,
    diff_set_part, diff_set_run_formatting, diff_set_run_text, diff_set_snapshot, diff_set_style_based_on, diff_set_style_name,
    resolve_blocks, DocxBlockPath, DocxDiff,
};
use crate::artifacts::docx::schema::snapshot::{DocxBlock, DocxStyle};
use crate::artifacts::docx::DocxSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.docx`. Beyond the baseline `{NoMutation, SetSnapshot}`,
/// this addresses the `document.body` block tree via `DocxBlockPath` (segments navigate through
/// nested `Table`s, mirrors svg's `NodePath` precedent), named styles by `DocxStyle::id`, and the
/// raw OPC layer by part path (for content this typed layer doesn't cover).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum DocxMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: DocxSnapshot,
    },
    /// ➕️ Inserts `block` at `path` (`path.index` = insertion index, FINAL state).
    InsertBlock {
        path: DocxBlockPath,
        block: DocxBlock,
    },
    /// ➖️ Removes the block at `path` (`path.index` = BASE-state index).
    RemoveBlock {
        path: DocxBlockPath,
    },
    /// ✍️ Replaces the full content of the block at `path` with `block`.
    SetBlockContent {
        path: DocxBlockPath,
        block: DocxBlock,
    },
    /// ✍️ Replaces the literal text of run `run_index` in the paragraph at `path`.
    SetRunText {
        path: DocxBlockPath,
        run_index: usize,
        text: String,
    },
    /// 🎨️ Sets run `run_index`'s bold/italic/underline flags in the paragraph at `path`.
    SetRunFormatting {
        path: DocxBlockPath,
        run_index: usize,
        bold: bool,
        italic: bool,
        underline: bool,
    },
    /// ➕️ Inserts a named style.
    InsertStyle {
        style: DocxStyle,
    },
    /// ➖️ Removes the style with id `id`.
    RemoveStyle {
        id: String,
    },
    /// 🏷️ Renames the style with id `id`.
    SetStyleName {
        id: String,
        name: String,
    },
    /// 🔗 Sets (or, if `None`, clears) the style with id `id`'s `based_on`.
    SetStyleBasedOn {
        id: String,
        based_on: Option<String>,
    },
    /// ✍️ Sets a raw OPC part (content this typed layer doesn't model), inserting or replacing.
    SetPart {
        path: String,
        content_type: String,
        bytes: Vec<u8>,
    },
    /// ➖️ Removes a raw OPC part by path.
    RemovePart {
        path: String,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` -- the diff is the single semantics source, never a separate imperative
/// apply path (apply-and-capture is banned).
pub fn apply_docx_mutation(snapshot: &mut DocxSnapshot, mutation: &DocxMutation) -> DocxDiff {
    let diff = Mutation::diff(mutation, snapshot);
    *snapshot = protocol::MutationDiff::apply(&diff, snapshot);
    diff
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
fn block_at<'a>(base: &'a DocxSnapshot, path: &DocxBlockPath) -> Option<&'a DocxBlock> {
    resolve_blocks(&base.document.body, &path.segments)?.get(path.index)
}

fn style_at<'a>(base: &'a DocxSnapshot, id: &str) -> Option<&'a DocxStyle> {
    base.document.styles.iter().find(|s| s.id == id)
}

fn part_at<'a>(base: &'a DocxSnapshot, path: &str) -> Option<&'a crate::artifacts::zip::opc::OpcPart> {
    let p = path.trim_start_matches('/');
    base.opc.parts.iter().find(|part| part.path == p)
}
//#endregion 🔖️Helpers

//#region 🔖️MutationTrait
impl Mutation<DocxSnapshot> for DocxMutation {
    type Diff = DocxDiff;

    fn diff(&self, base: &DocxSnapshot) -> Self::Diff {
        match self {
            DocxMutation::NoMutation => DocxDiff::default(),
            DocxMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            DocxMutation::InsertBlock { path, block } => diff_insert_block(path, block.clone()),
            DocxMutation::RemoveBlock { path } => diff_remove_block(path),
            DocxMutation::SetBlockContent { path, block } => match block_at(base, path) {
                Some(old) => diff_set_block_content(path, old, block),
                None => DocxDiff::default(),
            },
            DocxMutation::SetRunText { path, run_index, text } => diff_set_run_text(&base.document, path, *run_index, text),
            DocxMutation::SetRunFormatting { path, run_index, bold, italic, underline } => {
                diff_set_run_formatting(&base.document, path, *run_index, *bold, *italic, *underline)
            }
            DocxMutation::InsertStyle { style } => diff_insert_style(style.clone()),
            DocxMutation::RemoveStyle { id } => diff_remove_style(id),
            DocxMutation::SetStyleName { id, name } => diff_set_style_name(id, name),
            DocxMutation::SetStyleBasedOn { id, based_on } => diff_set_style_based_on(id, based_on.clone()),
            DocxMutation::SetPart { path, content_type, bytes } => diff_set_part(&base.opc, path, content_type, bytes.clone()),
            DocxMutation::RemovePart { path } => diff_remove_part(path),
        }
    }

    fn inverse(&self, base: &DocxSnapshot) -> Vec<Self> {
        match self {
            DocxMutation::NoMutation => vec![DocxMutation::NoMutation],
            DocxMutation::SetSnapshot { .. } => vec![DocxMutation::SetSnapshot { snapshot: base.clone() }],
            DocxMutation::InsertBlock { path, .. } => vec![DocxMutation::RemoveBlock { path: path.clone() }],
            DocxMutation::RemoveBlock { path } => match block_at(base, path) {
                Some(block) => vec![DocxMutation::InsertBlock { path: path.clone(), block: block.clone() }],
                None => vec![DocxMutation::NoMutation],
            },
            DocxMutation::SetBlockContent { path, .. } => match block_at(base, path) {
                Some(block) => vec![DocxMutation::SetBlockContent { path: path.clone(), block: block.clone() }],
                None => vec![DocxMutation::NoMutation],
            },
            DocxMutation::SetRunText { path, run_index, .. } => {
                let old = resolve_blocks(&base.document.body, &path.segments)
                    .and_then(|blocks| blocks.get(path.index))
                    .and_then(|b| match b { DocxBlock::Paragraph(p) => p.runs.get(*run_index), _ => None })
                    .map(|r| r.text.clone());
                match old {
                    Some(text) => vec![DocxMutation::SetRunText { path: path.clone(), run_index: *run_index, text }],
                    None => vec![DocxMutation::NoMutation],
                }
            }
            DocxMutation::SetRunFormatting { path, run_index, .. } => {
                let old = resolve_blocks(&base.document.body, &path.segments)
                    .and_then(|blocks| blocks.get(path.index))
                    .and_then(|b| match b { DocxBlock::Paragraph(p) => p.runs.get(*run_index), _ => None });
                match old {
                    Some(run) => vec![DocxMutation::SetRunFormatting {
                        path: path.clone(),
                        run_index: *run_index,
                        bold: run.bold,
                        italic: run.italic,
                        underline: run.underline,
                    }],
                    None => vec![DocxMutation::NoMutation],
                }
            }
            DocxMutation::InsertStyle { style } => vec![DocxMutation::RemoveStyle { id: style.id.clone() }],
            DocxMutation::RemoveStyle { id } => match style_at(base, id) {
                Some(style) => vec![DocxMutation::InsertStyle { style: style.clone() }],
                None => vec![DocxMutation::NoMutation],
            },
            DocxMutation::SetStyleName { id, .. } => match style_at(base, id) {
                Some(style) => vec![DocxMutation::SetStyleName { id: id.clone(), name: style.name.clone() }],
                None => vec![DocxMutation::NoMutation],
            },
            DocxMutation::SetStyleBasedOn { id, .. } => match style_at(base, id) {
                Some(style) => vec![DocxMutation::SetStyleBasedOn { id: id.clone(), based_on: style.based_on.clone() }],
                None => vec![DocxMutation::NoMutation],
            },
            DocxMutation::SetPart { path, .. } => match part_at(base, path) {
                Some(part) => vec![DocxMutation::SetPart { path: path.clone(), content_type: part.content_type.clone(), bytes: part.bytes.clone() }],
                None => vec![DocxMutation::RemovePart { path: path.clone() }],
            },
            DocxMutation::RemovePart { path } => match part_at(base, path) {
                Some(part) => vec![DocxMutation::SetPart { path: path.clone(), content_type: part.content_type.clone(), bytes: part.bytes.clone() }],
                None => vec![DocxMutation::NoMutation],
            },
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for DocxMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for DocxMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|e| protocol::ProtocolError::Malformed {
            what: "op encode",
            offset: 0,
            detail: e.to_string(),
        })
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|e| protocol::ProtocolError::Malformed {
            what: "op decode",
            offset: 0,
            detail: e.to_string(),
        })
    }
}
//#endregion OpCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::docx::schema::diff::{DocxBlockDiff, DocxOpcPartDiff, DocxPathSegment};
    use crate::artifacts::docx::schema::snapshot::{DocxDocument, DocxParagraph, DocxRun, DocxTable, DocxTableCell, DocxTableRow};
    use crate::artifacts::zip::opc::{OpcPackage, OpcRelationship, OpcTargetMode, REL_TYPE_OFFICE_DOCUMENT, RELS_CONTENT_TYPE};
    use protocol::command::DiffAlgebra;
    use protocol::MutationDiff;

    fn fixture() -> DocxSnapshot {
        crate::artifacts::docx::engine::build_minimal_docx(DocxDocument {
            body: vec![
                DocxBlock::paragraph("first"),
                DocxBlock::paragraph("second"),
            ],
            styles: vec![DocxStyle { id: "Normal".into(), name: "Normal".into(), based_on: None }],
        })
    }

    fn table_path(block_index: usize, row: usize, cell: usize, index: usize) -> DocxBlockPath {
        DocxBlockPath { segments: vec![DocxPathSegment { block_index, row, cell }], index }
    }

    #[test]
    fn insert_then_remove_block_apply_and_inverse() {
        let base = fixture();
        let insert = DocxMutation::InsertBlock { path: DocxBlockPath { segments: vec![], index: 1 }, block: DocxBlock::paragraph("inserted") };
        let mut after = base.clone();
        apply_docx_mutation(&mut after, &insert);
        assert_eq!(after.document.body.len(), 3);
        assert_eq!(after.document.body[1], DocxBlock::paragraph("inserted"));

        let inverses = Mutation::inverse(&insert, &base);
        let mut restored = after.clone();
        for inv in &inverses {
            apply_docx_mutation(&mut restored, inv);
        }
        assert_eq!(restored, base);
    }

    #[test]
    fn remove_block_inverse_restores_removed_block() {
        let base = fixture();
        let remove = DocxMutation::RemoveBlock { path: DocxBlockPath { segments: vec![], index: 0 } };
        let mut after = base.clone();
        apply_docx_mutation(&mut after, &remove);
        assert_eq!(after.document.body.len(), 1);
        for inv in Mutation::inverse(&remove, &base) {
            apply_docx_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);
    }

    #[test]
    fn set_run_text_and_formatting_apply_and_inverse() {
        let base = fixture();
        let mutation = DocxMutation::SetRunText { path: DocxBlockPath { segments: vec![], index: 0 }, run_index: 0, text: "changed".into() };
        let mut after = base.clone();
        apply_docx_mutation(&mut after, &mutation);
        let DocxBlock::Paragraph(p) = &after.document.body[0] else { panic!("paragraph") };
        assert_eq!(p.runs[0].text, "changed");
        for inv in Mutation::inverse(&mutation, &base) {
            apply_docx_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);

        let fmt = DocxMutation::SetRunFormatting { path: DocxBlockPath { segments: vec![], index: 0 }, run_index: 0, bold: true, italic: true, underline: true };
        let mut after2 = base.clone();
        apply_docx_mutation(&mut after2, &fmt);
        let DocxBlock::Paragraph(p2) = &after2.document.body[0] else { panic!("paragraph") };
        assert!(p2.runs[0].bold && p2.runs[0].italic && p2.runs[0].underline);
        for inv in Mutation::inverse(&fmt, &base) {
            apply_docx_mutation(&mut after2, &inv);
        }
        assert_eq!(after2, base);
    }

    #[test]
    fn table_path_addressing_sets_nested_cell_content() {
        let mut base = fixture();
        base.document.body.push(DocxBlock::Table(DocxTable {
            rows: vec![DocxTableRow { cells: vec![DocxTableCell { blocks: vec![DocxBlock::paragraph("cell")], ..Default::default() }], ..Default::default() }],
            ..Default::default()
        }));
        let path = table_path(2, 0, 0, 0);
        let mutation = DocxMutation::SetBlockContent { path: path.clone(), block: DocxBlock::paragraph("changed cell") };
        let mut after = base.clone();
        apply_docx_mutation(&mut after, &mutation);
        let DocxBlock::Table(t) = &after.document.body[2] else { panic!("table") };
        assert_eq!(t.rows[0].cells[0].blocks[0], DocxBlock::paragraph("changed cell"));
        for inv in Mutation::inverse(&mutation, &base) {
            apply_docx_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);
        let _ = &mut base; // silence unused-mut if the push above is later removed
    }

    #[test]
    fn style_mutations_apply_and_inverse() {
        let base = fixture();
        let insert = DocxMutation::InsertStyle { style: DocxStyle { id: "Heading1".into(), name: "heading 1".into(), based_on: Some("Normal".into()) } };
        let mut after = base.clone();
        apply_docx_mutation(&mut after, &insert);
        assert_eq!(after.document.styles.len(), 2);
        for inv in Mutation::inverse(&insert, &base) {
            apply_docx_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);

        let rename = DocxMutation::SetStyleName { id: "Normal".into(), name: "Body Text".into() };
        let mut after2 = base.clone();
        apply_docx_mutation(&mut after2, &rename);
        assert_eq!(after2.document.styles[0].name, "Body Text");
        for inv in Mutation::inverse(&rename, &base) {
            apply_docx_mutation(&mut after2, &inv);
        }
        assert_eq!(after2, base);

        let based_on = DocxMutation::SetStyleBasedOn { id: "Normal".into(), based_on: Some("Other".into()) };
        let mut after3 = base.clone();
        apply_docx_mutation(&mut after3, &based_on);
        assert_eq!(after3.document.styles[0].based_on, Some("Other".into()));
        for inv in Mutation::inverse(&based_on, &base) {
            apply_docx_mutation(&mut after3, &inv);
        }
        assert_eq!(after3, base);
    }

    #[test]
    fn opc_part_mutations_apply_and_inverse() {
        let base = fixture();
        let set = DocxMutation::SetPart { path: "word/numbering.xml".into(), content_type: "application/xml".into(), bytes: b"<w:numbering/>".to_vec() };
        let mut after = base.clone();
        apply_docx_mutation(&mut after, &set);
        assert_eq!(after.opc.part_bytes("word/numbering.xml"), Some(b"<w:numbering/>".as_slice()));
        for inv in Mutation::inverse(&set, &base) {
            apply_docx_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);

        let mut with_part = base.clone();
        apply_docx_mutation(&mut with_part, &set);
        let remove = DocxMutation::RemovePart { path: "word/numbering.xml".into() };
        let mut after2 = with_part.clone();
        apply_docx_mutation(&mut after2, &remove);
        assert_eq!(after2.opc.part_bytes("word/numbering.xml"), None);
        for inv in Mutation::inverse(&remove, &with_part) {
            apply_docx_mutation(&mut after2, &inv);
        }
        assert_eq!(after2, with_part);
    }

    //#region 🔖️Fixtures
    /// 🌱 `sweep_a`/`sweep_b`: differ in EVERY mutable field, both `document` and `opc`. Body uses
    /// different-length lists so the recipe's naive positional `between_indexed` shows
    /// removed+modified+added simultaneously (per this ticket's "known structural trap" note): a
    /// removed tail on `sweep_a`, a modified-in-every-field first paragraph, and an added tail on
    /// `sweep_b` (a table, exercising the recursive nested triple down to `blocks`). Styles (a
    /// name-keyed collection, order-independent) get one removed, one modified-in-every-field, one
    /// added. OPC content_types/parts/relationships each get one removed, one modified, one added.
    fn sweep_a() -> DocxSnapshot {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        opc.content_types.set_default("toRemove", "application/octet-stream");
        opc.set_part("word/document.xml", "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml", b"<w:document/>".to_vec());
        opc.set_part("word/toModify.xml", "application/xml", b"old".to_vec());
        opc.set_part("word/toRemove.xml", "application/xml", b"gone".to_vec());
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "word/document.xml");
        opc.add_relationship("", "rId9", "http://example/toRemove", "word/toRemove.xml");
        opc.relationships.insert(
            "word/document.xml".into(),
            vec![OpcRelationship { id: "rId2".into(), rel_type: "http://example/toModify".into(), target: "media/old.png".into(), target_mode: OpcTargetMode::Internal }],
        );
        // 🎯️ A relationships OWNER present only in `a` (owned by the part that itself gets
        // removed) -- exercises `relationships.removed` at the owner-key level, distinct from
        // `""`'s own list merely losing one entry (which exercises `relationships.modified`).
        opc.relationships.insert(
            "word/toRemove.xml".into(),
            vec![OpcRelationship { id: "rId8".into(), rel_type: "http://example/ownerToRemove".into(), target: "media/gone.png".into(), target_mode: OpcTargetMode::Internal }],
        );

        DocxSnapshot::from_parts(
            opc,
            DocxDocument {
                body: vec![
                    DocxBlock::Paragraph(DocxParagraph { runs: vec![DocxRun { text: "old".into(), bold: false, ..Default::default() }], style: None, extra_paragraph_properties: Vec::new() }),
                    DocxBlock::paragraph("stay"),
                    DocxBlock::Table(DocxTable {
                        rows: vec![DocxTableRow { cells: vec![DocxTableCell { blocks: vec![DocxBlock::paragraph("toDrop cell")], ..Default::default() }], ..Default::default() }],
                        ..Default::default()
                    }),
                ],
                styles: vec![
                    DocxStyle { id: "keep".into(), name: "Keep".into(), based_on: None },
                    DocxStyle { id: "toModify".into(), name: "old".into(), based_on: None },
                    DocxStyle { id: "toRemove".into(), name: "Gone".into(), based_on: None },
                ],
            },
        )
    }

    fn sweep_b() -> DocxSnapshot {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        opc.content_types.set_default("added", "application/octet-stream");
        opc.set_part("word/document.xml", "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml", b"<w:document/>changed".to_vec());
        opc.set_part("word/toModify.xml", "application/xml", b"new".to_vec());
        opc.set_part("word/added.xml", "application/xml", b"fresh".to_vec());
        // 🩹 AFTER the `set_part` calls above (which already appended `toModify`'s override entry
        // at position 1): a bare `set_override` on an EXISTING key updates its VALUE in place,
        // never its position -- so this exercises `content_types.overrides.modified` (the value
        // really differs from `sweep_a`'s "application/xml") without perturbing list order, which
        // matters because `overrides` is order-sensitive `Vec<(String,String)>` equality (the OPC
        // module's own type, not ours to change) and `between(a,b).apply(a)` only reconstructs
        // survivors in `a`'s original relative order + appends -- this fixture is built so that
        // convention already matches `sweep_b`'s own construction order.
        opc.content_types.set_override("word/toModify.xml", "application/xml-modified");
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "word/document.xml");
        opc.relationships.insert(
            "word/document.xml".into(),
            vec![OpcRelationship { id: "rId2".into(), rel_type: "http://example/toModify".into(), target: "media/new.png".into(), target_mode: OpcTargetMode::Internal }],
        );
        opc.relationships.insert("word/added.xml".into(), vec![OpcRelationship { id: "rId3".into(), rel_type: "http://example/added".into(), target: "media/added.png".into(), target_mode: OpcTargetMode::Internal }]);

        DocxSnapshot::from_parts(
            opc,
            DocxDocument {
                // 🎯️ Length 2 vs `sweep_a`'s 3: per this ticket's "known structural trap" note, a
                // single same-direction `between()` call can never show BOTH a top-level `removed`
                // AND a top-level `added` (only one tail flavor per direction) -- so `a -> b`
                // exercises `body.removed` (the dropped `Table`, index 2) + `body.modified[0]`
                // (whose OWN nested `runs` diff exercises modified+added together, since ONE run
                // survives-and-changes while a SECOND run is net-new); `b -> a` (the reverse
                // direction, asserted separately in `field_sweep` below) exercises `body.added`
                // (the very same `Table`, recursed structurally as the added item's payload).
                body: vec![
                    DocxBlock::Paragraph(DocxParagraph {
                        runs: vec![
                            DocxRun { text: "new".into(), bold: true, ..Default::default() },
                            DocxRun { text: "second run".into(), italic: true, ..Default::default() },
                        ],
                        style: Some("keep".into()),
                        extra_paragraph_properties: Vec::new(),
                    }),
                    DocxBlock::paragraph("stay"),
                ],
                styles: vec![
                    DocxStyle { id: "keep".into(), name: "Keep".into(), based_on: None },
                    DocxStyle { id: "toModify".into(), name: "new".into(), based_on: Some("keep".into()) },
                    DocxStyle { id: "added".into(), name: "Added".into(), based_on: None },
                ],
            },
        )
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️MutationDiffLaw
    fn sample_mutations() -> Vec<DocxMutation> {
        vec![
            DocxMutation::NoMutation,
            DocxMutation::SetSnapshot { snapshot: sweep_b() },
            DocxMutation::InsertBlock { path: DocxBlockPath { segments: vec![], index: 1 }, block: DocxBlock::paragraph("x") },
            DocxMutation::RemoveBlock { path: DocxBlockPath { segments: vec![], index: 0 } },
            DocxMutation::SetBlockContent { path: DocxBlockPath { segments: vec![], index: 0 }, block: DocxBlock::paragraph("y") },
            DocxMutation::SetRunText { path: DocxBlockPath { segments: vec![], index: 0 }, run_index: 0, text: "z".into() },
            DocxMutation::SetRunFormatting { path: DocxBlockPath { segments: vec![], index: 0 }, run_index: 0, bold: true, italic: false, underline: true },
            DocxMutation::InsertStyle { style: DocxStyle { id: "Heading1".into(), name: "heading 1".into(), based_on: None } },
            DocxMutation::RemoveStyle { id: "Normal".into() },
            DocxMutation::SetStyleName { id: "Normal".into(), name: "Body".into() },
            DocxMutation::SetStyleBasedOn { id: "Normal".into(), based_on: Some("Heading1".into()) },
            DocxMutation::SetPart { path: "word/numbering.xml".into(), content_type: "application/xml".into(), bytes: b"<w:numbering/>".to_vec() },
            // 🎯️ `RemovePart` targets `word/styles.xml`, the LAST part in `fixture()`'s
            // `opc.parts` (built after `word/document.xml` since `fixture()`'s document has a
            // style) -- like svg's own `SetAttribute{value:None}` precedent (see that artifact's
            // `sample_mutations` doc comment), OPC parts are a NAME-keyed collection (position
            // carries no OPC-spec meaning), so `RemovePart`'s mutation-level inverse (`SetPart`,
            // which treats a not-currently-present path as an APPEND) only restores the exact
            // original Vec position when the removed item was already last -- exact positional
            // restoration in the general case is only guaranteed at the DIFF level, same caveat
            // as svg's.
            DocxMutation::RemovePart { path: "word/styles.xml".into() },
        ]
    }

    #[test]
    fn mutation_diff_law() {
        for mutation in sample_mutations() {
            let base = fixture();
            let diff_direct = Mutation::diff(&mutation, &base);
            let applied_via_diff = MutationDiff::apply(&diff_direct, &base);

            let mut via_apply = base.clone();
            let diff_from_apply = apply_docx_mutation(&mut via_apply, &mutation);

            assert_eq!(applied_via_diff, via_apply, "mutation_diff_law: apply mismatch for {mutation:?}");
            assert_eq!(diff_direct, diff_from_apply, "mutation_diff_law: diff mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️MutationDiffLaw

    //#region 🔖️InverseLaw
    #[test]
    fn inverse_law() {
        for mutation in sample_mutations() {
            let base = fixture();

            let mut round_tripped = base.clone();
            apply_docx_mutation(&mut round_tripped, &mutation);
            for inverse_mutation in <DocxMutation as Mutation<DocxSnapshot>>::inverse(&mutation, &base) {
                apply_docx_mutation(&mut round_tripped, &inverse_mutation);
            }
            assert_eq!(round_tripped, base, "inverse_law (mutation-level) failed for {mutation:?}");

            let diff = Mutation::diff(&mutation, &base);
            let next = MutationDiff::apply(&diff, &base);
            let inverse_diff = DiffAlgebra::inverse(&diff, &base);
            let restored = MutationDiff::apply(&inverse_diff, &next);
            assert_eq!(restored, base, "inverse_law (diff-level) failed for {mutation:?}");
        }
    }
    //#endregion 🔖️InverseLaw

    //#region 🔖️AbsorbLaw
    fn assert_absorb_matches_sequential(base: &DocxSnapshot, d1: &DocxDiff, d2: &DocxDiff) -> DocxDiff {
        let sequential = MutationDiff::apply(d2, &MutationDiff::apply(d1, base));
        let mut absorbed = d1.clone();
        MutationDiff::absorb(&mut absorbed, d2.clone());
        assert_eq!(MutationDiff::apply(&absorbed, base), sequential, "absorb_law: apply(absorb(d1,d2), base) != sequential");
        absorbed
    }

    fn body_diff(diff: &DocxDiff) -> &crate::artifacts::docx::schema::diff::DocxBlocksDiff {
        diff.document.as_ref().expect("document diff present").body.as_ref().expect("body diff present")
    }

    #[test]
    fn absorb_law() {
        // Canonical: Insert(2)+Remove(0) -> {removed:[0], added:[(1,f)]}.
        {
            let base = fixture();
            let d1 = Mutation::diff(&DocxMutation::InsertBlock { path: DocxBlockPath { segments: vec![], index: 2 }, block: DocxBlock::paragraph("f") }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&DocxMutation::RemoveBlock { path: DocxBlockPath { segments: vec![], index: 0 } }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = body_diff(&absorbed);
            assert_eq!(triple.removed, vec![0]);
            assert_eq!(triple.added.len(), 1);
            assert_eq!(triple.added[0].index, 1);
            assert_eq!(triple.added[0].item, DocxBlock::paragraph("f"));
        }

        // Canonical: Insert(2,f)+Insert(2,g) -> both survive.
        {
            let base = fixture();
            let d1 = Mutation::diff(&DocxMutation::InsertBlock { path: DocxBlockPath { segments: vec![], index: 2 }, block: DocxBlock::paragraph("f") }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&DocxMutation::InsertBlock { path: DocxBlockPath { segments: vec![], index: 2 }, block: DocxBlock::paragraph("g") }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = body_diff(&absorbed);
            assert_eq!(triple.added.len(), 2, "both inserts must survive absorb, not LWW-clobber");
            assert!(triple.added.iter().any(|a| a.item == DocxBlock::paragraph("f")));
            assert!(triple.added.iter().any(|a| a.item == DocxBlock::paragraph("g")));
        }

        // Canonical: Insert(1,f)+SetField(1,v) -> patch into the added payload.
        {
            let base = fixture();
            let d1 = Mutation::diff(&DocxMutation::InsertBlock { path: DocxBlockPath { segments: vec![], index: 1 }, block: DocxBlock::paragraph("f") }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&DocxMutation::SetRunText { path: DocxBlockPath { segments: vec![], index: 1 }, run_index: 0, text: "patched".into() }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = body_diff(&absorbed);
            assert!(triple.modified.is_empty(), "patch-into-added must not surface as a separate modified entry");
            assert_eq!(triple.added.len(), 1);
            assert_eq!(triple.added[0].item, DocxBlock::paragraph("patched"));
        }

        // Canonical: Modify+Remove -> the modify is annihilated by the later remove.
        {
            let base = fixture();
            let d1 = Mutation::diff(&DocxMutation::SetRunText { path: DocxBlockPath { segments: vec![], index: 1 }, run_index: 0, text: "patched".into() }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&DocxMutation::RemoveBlock { path: DocxBlockPath { segments: vec![], index: 1 } }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = body_diff(&absorbed);
            assert!(triple.modified.is_empty(), "modify of a since-removed item must not survive absorb");
            assert_eq!(triple.removed, vec![1]);
        }

        // Associativity over a triple.
        {
            let base = fixture();
            let d1 = Mutation::diff(&DocxMutation::InsertBlock { path: DocxBlockPath { segments: vec![], index: 2 }, block: DocxBlock::paragraph("f") }, &base);
            let mid1 = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&DocxMutation::InsertBlock { path: DocxBlockPath { segments: vec![], index: 2 }, block: DocxBlock::paragraph("g") }, &mid1);
            let mid2 = MutationDiff::apply(&d2, &mid1);
            let d3 = Mutation::diff(&DocxMutation::RemoveBlock { path: DocxBlockPath { segments: vec![], index: 0 } }, &mid2);
            let sequential = MutationDiff::apply(&d3, &mid2);

            let mut left = d1.clone();
            MutationDiff::absorb(&mut left, d2.clone());
            MutationDiff::absorb(&mut left, d3.clone());

            let mut d2_then_d3 = d2.clone();
            MutationDiff::absorb(&mut d2_then_d3, d3.clone());
            let mut right = d1.clone();
            MutationDiff::absorb(&mut right, d2_then_d3);

            assert_eq!(MutationDiff::apply(&left, &base), sequential, "absorb associativity (left) failed");
            assert_eq!(MutationDiff::apply(&right, &base), sequential, "absorb associativity (right) failed");
        }
    }
    //#endregion 🔖️AbsorbLaw

    //#region 🔖️BetweenRoundtripLaw
    #[test]
    fn between_roundtrip_law() {
        let a = sweep_a();
        let b = sweep_b();
        assert_eq!(MutationDiff::apply(&<DocxDiff as DiffAlgebra<DocxSnapshot>>::between(&a, &b), &a), b);
        assert_eq!(MutationDiff::apply(&<DocxDiff as DiffAlgebra<DocxSnapshot>>::between(&b, &a), &b), a);

        let sample = fixture();
        assert_eq!(MutationDiff::apply(&<DocxDiff as DiffAlgebra<DocxSnapshot>>::between(&sample, &sample), &sample), sample);

        // "Real" fixture leg: a realistic multi-paragraph document diffed against a mutated variant.
        let real = crate::artifacts::docx::engine::build_minimal_docx(DocxDocument {
            body: vec![DocxBlock::paragraph("Chapter One"), DocxBlock::paragraph("Body text goes here.")],
            styles: vec![DocxStyle { id: "Normal".into(), name: "Normal".into(), based_on: None }],
        });
        let mut mutated = real.clone();
        apply_docx_mutation(&mut mutated, &DocxMutation::SetRunText { path: DocxBlockPath { segments: vec![], index: 0 }, run_index: 0, text: "Chapter Two".into() });
        assert_ne!(real, mutated);
        assert_eq!(MutationDiff::apply(&<DocxDiff as DiffAlgebra<DocxSnapshot>>::between(&real, &mutated), &real), mutated);
        assert_eq!(MutationDiff::apply(&<DocxDiff as DiffAlgebra<DocxSnapshot>>::between(&mutated, &real), &mutated), real);
    }
    //#endregion 🔖️BetweenRoundtripLaw

    //#region 🔖️CodecRetentionLaw
    #[test]
    fn codec_retention_law() {
        let snap = crate::artifacts::docx::engine::build_minimal_docx(DocxDocument {
            body: vec![DocxBlock::Paragraph(DocxParagraph {
                runs: vec![DocxRun { text: "Hello".into(), bold: true, italic: true, underline: true, extra_run_properties: Vec::new() }],
                style: Some("Normal".into()),
                extra_paragraph_properties: Vec::new(),
            })],
            styles: vec![DocxStyle { id: "Normal".into(), name: "Normal".into(), based_on: None }],
        });
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <DocxSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }
    //#endregion 🔖️CodecRetentionLaw

    //#region 🔖️FieldSweep
    /// 🎯️ THE acceptance criterion: `sweep_a`/`sweep_b` differ in every mutable field across BOTH
    /// `opc` and `document` (see the fixtures' doc comment for exactly how each collection flavor
    /// -- removed/modified/added -- is exercised, and this ticket's "known structural trap" note
    /// for why the two snapshots use different-length body lists rather than a single same-length
    /// pairwise collection).
    #[test]
    fn field_sweep() {
        let a = sweep_a();
        let b = sweep_b();

        let diff_ab = <DocxDiff as DiffAlgebra<DocxSnapshot>>::between(&a, &b);
        assert_eq!(MutationDiff::apply(&diff_ab, &a), b);
        let diff_ba = <DocxDiff as DiffAlgebra<DocxSnapshot>>::between(&b, &a);
        assert_eq!(MutationDiff::apply(&diff_ba, &b), a);
        assert!(<DocxDiff as DiffAlgebra<DocxSnapshot>>::between(&a, &a).is_empty());

        // opc: content_types (both defaults+overrides), parts, relationships all populated.
        let opc_diff = diff_ab.opc.as_ref().expect("opc diff present");
        let ct = opc_diff.content_types.as_ref().expect("content_types diff present");
        let defaults = ct.defaults.as_ref().expect("defaults diff present");
        assert!(!defaults.added.is_empty(), "content_types.defaults: added not exercised");
        let overrides = ct.overrides.as_ref().expect("overrides diff present");
        assert!(!overrides.modified.is_empty(), "content_types.overrides: modified not exercised");
        let parts = opc_diff.parts.as_ref().expect("parts diff present");
        assert!(!parts.removed.is_empty(), "opc.parts: removed not exercised");
        assert!(!parts.modified.is_empty(), "opc.parts: modified not exercised");
        assert!(!parts.added.is_empty(), "opc.parts: added not exercised");
        let part_mod = &parts.modified[0];
        assert!(matches!(&part_mod.diff, DocxOpcPartDiff { bytes: Some(_), .. }));
        let rels = opc_diff.relationships.as_ref().expect("relationships diff present");
        assert!(!rels.removed.is_empty(), "opc.relationships: removed (owner) not exercised");
        assert!(!rels.modified.is_empty(), "opc.relationships: modified (owner) not exercised");
        assert!(!rels.added.is_empty(), "opc.relationships: added (owner) not exercised");

        // document.body: `a -> b` exercises removed (top) + modified-with-nested-runs-added (per
        // the "known structural trap" note, one same-direction `between()` can't show BOTH a
        // top-level removed AND a top-level added -- see `sweep_b`'s doc comment).
        let doc_diff = diff_ab.document.as_ref().expect("document diff present");
        let body_diff = doc_diff.body.as_ref().expect("body diff present");
        assert!(!body_diff.removed.is_empty(), "body: removed not exercised");
        assert_eq!(body_diff.modified.len(), 1);
        let DocxBlockDiff::Paragraph(p_diff) = &body_diff.modified[0].diff else { panic!("expected paragraph diff") };
        let runs_diff = p_diff.runs.as_ref().expect("modified paragraph: runs not exercised");
        assert_eq!(p_diff.style, Some(Some("keep".to_string())), "modified paragraph: style tri-state Some(Some(_)) not exercised");
        assert!(!runs_diff.modified.is_empty(), "modified paragraph: runs.modified not exercised");
        let run_diff = &runs_diff.modified[0].diff;
        assert!(run_diff.text.is_some() && run_diff.bold.is_some(), "modified run: text/bold not exercised");
        assert!(!runs_diff.added.is_empty(), "modified paragraph: runs.added (nested) not exercised");

        // `b -> a` exercises the OTHER direction's top-level `added` (the very same dropped
        // `Table`, carried whole as the added item's payload, recursively structured).
        let body_diff_ba = diff_ba.document.as_ref().unwrap().body.as_ref().expect("body diff (b->a) present");
        assert!(!body_diff_ba.added.is_empty(), "body (b->a): added not exercised");
        let DocxBlock::Table(added_table) = &body_diff_ba.added[0].item else { panic!("expected added table") };
        assert!(!added_table.rows.is_empty());

        // document.styles: removed+modified(name+based_on tri-state)+added.
        let styles_diff = doc_diff.styles.as_ref().expect("styles diff present");
        assert!(!styles_diff.removed.is_empty(), "styles: removed not exercised");
        assert!(!styles_diff.added.is_empty(), "styles: added not exercised");
        let style_mod = styles_diff.modified.iter().find(|m| m.key == "toModify").expect("toModify style modified");
        assert!(style_mod.diff.name.is_some());
        assert_eq!(style_mod.diff.based_on, Some(Some("keep".to_string())), "style based_on tri-state Some(Some(_)) not exercised");

        // Some(None) tri-state coverage: style based_on cleared, going the OTHER direction (b -> a
        // clears "toModify"'s based_on since a's copy has based_on: None).
        let style_mod_ba = diff_ba.document.as_ref().unwrap().styles.as_ref().unwrap().modified.iter().find(|m| m.key == "toModify").expect("toModify present in b->a");
        assert_eq!(style_mod_ba.diff.based_on, Some(None), "style based_on tri-state Some(None) not exercised");
    }
    //#endregion 🔖️FieldSweep
}
//#endregion 🧪️Tests
