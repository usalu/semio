//! 🔺️ DocxDiff — sparse diff over `DocxSnapshot` (`opc: OpcPackage` + the authoritative XML parts). No `snapshot: Option<DocxSnapshot>` full-replace slot.
//!
//! The OPC layer is the shared [`OpcDiff`] of `zip::opc::diff` (content-type entries, parts and relationship owners as positional list deltas). The XML parts are a
//! positional list delta of the kernel (`protocol::list_delta`): `removed`/`inserted`/`moved` rows with base/after coordinates and `modified` rows whose sparse typed patch
//! carries the part's content type and its document diff — never a whole `order` key list.

#[cfg(test)]
use crate::schema::snapshot::DocxDocument;
use crate::schema::snapshot::{DocxBlock, DocxXmlPart, DocxXmlParts, DOCX_MAX_XML_PARTS};
#[cfg(test)]
use crate::schema::snapshot::{DocxParagraph, DocxRun, DocxStyle};
use crate::DocxSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use semio_s_artifact_stdio_contract::kernel::list_delta::{BuildList, ItemList, RowPatch};
use semio_s_artifact_stdio_contract::list_delta::compose_optional;
use semio_s_artifact_stdio_xml::schema::diff::XmlDiff;
#[cfg(test)]
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlNode};
use semio_s_artifact_stdio_xml::{XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA};
pub use semio_s_artifact_stdio_zip::opc::diff::OpcDiff;
use semio_s_artifact_stdio_zip::opc::retained::RetainedOpcPackage;
#[cfg(test)]
use semio_s_artifact_stdio_zip::opc::{OpcPart, OpcRelationship, OpcTargetMode};

//#region 🔖️XmlPartDiffTypes
/// 🗂️ The XML parts of a DOCX as the list the positional delta rebuilds.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DocxXmlPartRows(pub DocxXmlParts);

impl ItemList<DocxXmlPart> for DocxXmlPartRows {
    fn count(&self) -> usize {
        self.0.len()
    }
    fn at(&self, index: usize) -> Option<&DocxXmlPart> {
        self.0.get(index)
    }
    fn to_rows(&self) -> Vec<DocxXmlPart> {
        self.0.iter().cloned().collect()
    }
}

impl BuildList<DocxXmlPart> for DocxXmlPartRows {
    fn from_rows(rows: Vec<DocxXmlPart>) -> Self {
        Self(rows.into_iter().collect())
    }
}

/// 👁️ The XML parts of a DOCX read in place, so the inverse of a delta never clones a retained document it does not reinsert.
struct DocxXmlPartsView<'a>(&'a DocxXmlParts);

impl ItemList<DocxXmlPart> for DocxXmlPartsView<'_> {
    fn count(&self) -> usize {
        self.0.len()
    }
    fn at(&self, index: usize) -> Option<&DocxXmlPart> {
        self.0.get(index)
    }
    fn to_rows(&self) -> Vec<DocxXmlPart> {
        self.0.iter().cloned().collect()
    }
}

/// 🩹 The sparse patch of one XML part: its content type and its document diff.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxXmlPartDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<XmlDiff>,
}

semio_s_artifact_stdio_contract::stdio_list_delta! {
    /// 🪡️ The positional delta of the XML parts, keyed by part name.
    pub DocxXmlPartsDelta { removal: DocxXmlPartRemoval, insertion: DocxXmlPartInsertion, relocation: DocxXmlPartRelocation, modification: DocxXmlPartModification, row: DocxXmlPart, patch: DocxXmlPartDiff, list: DocxXmlPartRows, key: String = |part| part.path.clone() }
}
//#endregion 🔖️XmlPartDiffTypes

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.docx`.
/// 🧪️ F6 VERIFIED: `#[derive(dsl::)]` on this struct fails to compile with TWO independent,
/// simultaneous reasons (both captured verbatim via a real `cargo check -p semio-s-plugin-stdio
/// --lib`, per `f6-recon-report.md` §3, then reverted): (1) enum-in-tree —
/// the XML part patch holds a recursive `XmlDiff` tree (a genuine data-carrying enum) that `DslField` has no impl for; (2) the retained XML documents
/// of its rows are no `DslField` either. `DiffBinary,DiffCodec,DiffText` is hand-rolled below, following the svg/gif template exactly (§5 of the
/// recon report).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.docx.diff")]
pub struct DocxDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub opc: Option<OpcDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub xml_parts: Option<DocxXmlPartsDelta>,
}
//#endregion 🔖️Diff

//#region 🔖️PathAddressing
/// 🧭️ One step down into a nested table cell's block list: `body[block_index]` must be a `Table`;
/// descend to `rows[row].cells[cell].blocks`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxPathSegment {
    pub block_index: usize,
    pub row: usize,
    pub cell: usize,
}

/// 🧭️ Addresses one block-list slot: `segments` navigate through nested `Table`s (mirrors svg's
/// `NodePath` chain-of-indices precedent, adapted for docx's Paragraph/Table mixed tree),
/// `index` is the slot within the innermost list.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxBlockPath {
    #[value(default)]
    pub segments: Vec<DocxPathSegment>,
    pub index: usize,
}

/// 🧭️ Resolves the block list a path's segments navigate to (the parent list `path.index` slots
/// into), immutable form. `pub` so the mutations module can look up prior state for its own
/// handcrafted `diff()`/`inverse()` bodies without duplicating this traversal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn resolve_blocks<'a>(body: &'a [DocxBlock], segments: &[DocxPathSegment]) -> Option<&'a [DocxBlock]> {
    match segments.split_first() {
        None => Some(body),
        Some((seg, rest)) => {
            let DocxBlock::Table(table) = body.get(seg.block_index)? else { return None };
            let row = table.rows.get(seg.row)?;
            let cell = row.cells.get(seg.cell)?;
            resolve_blocks(&cell.blocks, rest)
        }
    }
}
//#endregion 🔖️PathAddressing

//#region 🔖️XmlPartDiffLogic
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_snapshot(document: &semio_s_artifact_stdio_xml::schema::snapshot::retained::RetainedXmlDocument) -> XmlSnapshot {
    XmlSnapshot {
        schema: STDIO_XML_DOCUMENT_SCHEMA.into(),
        doc: document.materialize_exact().expect("valid retained DOCX XML authority materializes for diff"),
    }
}

fn apply_xml_part(part: &mut DocxXmlPart, diff: &DocxXmlPartDiff, capability: protocol::ApplyCapability) -> MutationApplyResult<()> {
    if let Some(content_type) = &diff.content_type {
        part.content_type.clone_from(content_type);
    }
    if let Some(document) = &diff.document {
        let next = document.apply(&xml_snapshot(&part.document), capability)?.doc;
        part.replace_document(next).map_err(|error| MutationApplyError::new("mutation.apply.ownership", error.into_message()).at(["document"]))?;
    }
    Ok(())
}

impl RowPatch<DocxXmlPart> for DocxXmlPartDiff {
    fn commit_into(&self, row: &mut DocxXmlPart, capability: protocol::ApplyCapability) -> Result<(), MutationApplyError> {
        apply_xml_part(row, self, capability)
    }

    fn absorb(&mut self, later: Self) {
        if later.content_type.is_some() {
            self.content_type = later.content_type;
        }
        self.document = match (self.document.take(), later.document) {
            (None, value) | (value, None) => value,
            (Some(mut left), Some(right)) => {
                left.absorb(right);
                Some(left)
            }
        };
    }

    fn inverse(&self, row: &DocxXmlPart) -> Self {
        Self { content_type: self.content_type.as_ref().map(|_| row.content_type.clone()), document: self.document.as_ref().map(|document| document.inverse(&xml_snapshot(&row.document))) }
    }

    fn is_empty(&self) -> bool {
        self.content_type.is_none() && self.document.is_none()
    }
}

fn apply_xml_parts(items: &mut DocxXmlParts, diff: &DocxXmlPartsDelta, capability: protocol::ApplyCapability) -> MutationApplyResult<()> {
    if items.len() + diff.inserted.len() > DOCX_MAX_XML_PARTS + diff.removed.len() {
        return Err(MutationApplyError::new("mutation.apply.ownership", "the XML parts exceed the DOCX part capacity").at(["inserted"]));
    }
    let rows = DocxXmlPartRows(std::mem::take(items));
    *items = diff.commit_onto(&rows, capability)?.0;
    Ok(())
}

fn rewind_xml_parts(base: &DocxXmlParts, diff: &DocxXmlPartsDelta) -> DocxXmlPartsDelta {
    DocxXmlPartsDelta::from_parts(diff.clone().into_parts().inverse(&DocxXmlPartsView(base)))
}
//#endregion 🔖️XmlPartDiffLogic

//#region 🔖️Apply
impl MutationDiff<DocxSnapshot> for DocxDiff {
    fn apply(&self, base: &DocxSnapshot, capability: protocol::ApplyCapability) -> MutationApplyResult<DocxSnapshot> {
        let mut next = base.clone();
        if let Some(diff) = &self.opc {
            let mut opc = next.opc.materialize_package_exact().map_err(|error| MutationApplyError::new("mutation.apply.ownership", error.to_string()).under(["opc"]))?;
            diff.commit_into(&mut opc, capability).map_err(|error| error.under(["opc"]))?;
            next.opc = RetainedOpcPackage::try_from_package(opc).map_err(|error| MutationApplyError::new("mutation.apply.ownership", error.to_string()).under(["opc"]))?;
        }
        if let Some(diff) = &self.xml_parts {
            apply_xml_parts(&mut next.xml_parts, diff, capability).map_err(|error| error.under(["xmlParts"]))?;
        }
        next.validate_authority().and_then(|()| next.project_document().map(|_| ())).map_err(|error| MutationApplyError::new("mutation.apply.invalid-snapshot", error.to_string()))?;
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        self.opc = match (self.opc.take(), other.opc) {
            (None, value) | (value, None) => value,
            (Some(mut left), Some(right)) => {
                left.absorb(right);
                Some(left)
            }
        };
        self.xml_parts = compose_optional(self.xml_parts.take(), other.xml_parts);
    }
}
//#endregion 🔖️Apply

//#region 🔖️DiffAlgebra
impl DiffAlgebra<DocxSnapshot> for DocxDiff {
    fn inverse(&self, base: &DocxSnapshot) -> Self {
        DocxDiff {
            opc: self.opc.as_ref().map(|diff| {
                let opc = base.opc.materialize_package_exact().expect("a valid retained DOCX OPC authority materializes for inverse diff");
                diff.rewind(&opc)
            }),
            xml_parts: self.xml_parts.as_ref().map(|diff| rewind_xml_parts(&base.xml_parts, diff)),
        }
    }

    fn is_empty(&self) -> bool {
        self.opc.is_none() && self.xml_parts.is_none()
    }
}
//#endregion 🔖️DiffAlgebra


//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: **hand-rolled** `protocol::DiffCodec` for `DocxDiff` (real compile errors captured above
/// on `DocxDiff`'s own doc comment) — same grammar style `GifDiff`/`SvgDiff`'s hand-rolled codecs
/// use (bracket-depth-aware split, hex for strings/bytes, `[0]`/`[1,x]` for `Option<T>`, a single
/// uppercase tag letter for data-carrying enums). This file re-derives its own copies of the small
/// helper functions since each hand-rolled codec is self-contained (no shared "hand-roll helpers"
/// module exists yet — flagged in `f6-recon-report.md` §5 as a good future extraction once ≥3
/// artifacts hand-roll, not worth adding here for one more). 
//#region 🔖️Primitives














//#endregion 🔖️Primitives

//#region 🔖️XmlValueCodecs




//#endregion 🔖️XmlValueCodecs

//#region 🔖️ValueCodecs




















//#endregion 🔖️ValueCodecs

//#region 🔖️BinaryCodecs
/// 🧪️ FG-wave: real recursive BINARY twins of every text-form codec above, backing the upgraded
/// `DiffBinary,DiffCodec,DiffText::encode_diff`/`decode_diff` below (and, via re-export, `../🧬️mutations/🦀️.rs`'s
/// own upgraded `OpBinary`) — replaces F6's `print_diff().into_bytes()` text-as-binary shortcut.
/// Real LEB128-varint-framed length-prefixed strings/bytes (`store::pack_rt::write_varint_u64` +
/// `store::ByteReader`), 1-byte tri-state presence tags, and 1-byte enum-variant tags — genuinely
/// structured binary, never hex-ASCII text reused as "binary". Same shape
/// `📰️xml/…/🔺️diff/🦀️.rs`'s own `BinaryPrimitives`/`XmlValueBinaryCodecs`/
/// `DiffValueBinaryCodecs` regions establish; duplicated here (not imported) per this repo's
/// per-artifact hand-roll convention (no shared "hand-roll helpers" module exists yet, see this
/// file's own `HandcraftedDiffCodec` doc comment).
//#region 🔖️BinaryPrimitives




//#endregion 🔖️BinaryPrimitives

//#region 🔖️XmlValueBinaryCodecs






//#endregion 🔖️XmlValueBinaryCodecs

//#region 🔖️ValueBinaryCodecs




















//#endregion 🔖️ValueBinaryCodecs
//#endregion 🔖️BinaryCodecs

//#region 🔖️TopLevel
//#endregion 🔖️TopLevel

//#region 🔖️DemoCases
/// 🧪️ FG-wave: representative `DocxDiff` values (both top-level fields, the recursive
/// `Paragraph`/`Table` `DocxBlockDiff` tree incl. a nested table-cell block list, both
/// `style`/`based_on` tri-states, and the OPC layer's content-types/parts/relationships-by-owner
/// triples) — the single source of truth reused by `diff_codec_text_binary_roundtrip_law` below
/// AND by `⚙️engine/🦀️.rs`'s `diff_grammar_conformance_law`/`protocol_walk_law`
/// conformance tests, same shape `📷️png/…/🔺️diff/🦀️.rs`'s own `demo_diff_cases()`
/// establishes.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn xml_node(name: &str) -> XmlNode {
    XmlNode::Element { name: name.to_string(), attrs: vec![XmlAttr { name: "a".into(), value: "1".into() }], children: vec![] }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn snapshot_a() -> DocxSnapshot {
    let mut snapshot = crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_docx(DocxDocument {
        body: vec![DocxBlock::Paragraph(DocxParagraph { runs: vec![DocxRun { text: "old".into(), bold: false, extra_run_properties: vec![xml_node("rPr")], ..Default::default() }], style: None, extra_paragraph_properties: Vec::new() })],
        styles: vec![DocxStyle { id: "keep".into(), name: "Keep".into(), based_on: Some("toRemove".into()) }],
    });
    snapshot.opc.set_part("word/media/to-remove.bin", "application/octet-stream", vec![1, 2]);
    snapshot
}

/// 🧪️ The declaration the demo's second snapshot gives its main document part.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_declaration() -> semio_s_artifact_stdio_xml::schema::snapshot::XmlDeclaration {
    semio_s_artifact_stdio_xml::schema::snapshot::XmlDeclaration { version: "1.0".into(), encoding: Some("UTF-8".into()), standalone: Some(true), ..Default::default() }
}

/// 🧪️ The external root relationship, the binary part and the extension default the demo's second snapshot carries instead of / besides the first's.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_added_rows() -> (OpcRelationship, OpcPart, (String, String)) {
    (
        OpcRelationship { id: "rIdDemoExternal".into(), rel_type: "http://example.invalid/relationships/demo".into(), target: "https://example.invalid/demo".into(), target_mode: OpcTargetMode::External },
        OpcPart { path: "word/media/added.bin".into(), content_type: "application/octet-stream".into(), bytes: vec![3, 4] },
        ("zzdemo".into(), "application/x-semio-demo".into()),
    )
}

/// 🧪️ The second demo snapshot: [`snapshot_a`] with its first and last XML part swapped, a declaration on the main document, `to-remove.bin` replaced by
/// `added.bin` at the same position, an external root relationship and an extension default appended.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn snapshot_b() -> DocxSnapshot {
    let mut snapshot = snapshot_a();
    let last = snapshot.xml_parts.len() - 1;
    snapshot.xml_parts.swap(0, last);
    let main = snapshot.xml_parts.iter_mut().find(|part| part.path == "word/document.xml").expect("the minimal DOCX has a main document part");
    let mut document = main.materialize_document_exact().expect("the main document materializes");
    document.declaration = Some(demo_declaration());
    main.replace_document(document).expect("the main document is retained again");
    let (relationship, part, entry) = demo_added_rows();
    snapshot
        .opc
        .edit_package(|package| {
            let at = package.parts.iter().position(|existing| existing.path == "word/media/to-remove.bin").expect("snapshot a carries to-remove.bin");
            package.parts[at] = part;
            let owned: Vec<OpcRelationship> = package.relationships.relationships("").into_iter().flatten().cloned().chain(std::iter::once(relationship)).collect();
            package.relationships.replace_owner(String::new(), owned);
            package.content_types.defaults.push(entry);
        })
        .expect("the demo edits are valid");
    snapshot
}

/// 🧪️ The declared diff from [`snapshot_a`] to [`snapshot_b`]: positional rows only, written out by hand.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_forward_diff() -> DocxDiff {
    use semio_s_artifact_stdio_zip::opc::diff::{OpcContentTypeEntriesDelta, OpcContentTypeInsertion, OpcContentTypeRow, OpcContentTypesDiff, OpcOwnerModification, OpcOwnerPatch, OpcOwnersDelta, OpcPartInsertion, OpcPartRemoval, OpcPartsDelta, OpcRelationshipInsertion, OpcRelationshipsDelta};
    let base = snapshot_a();
    let opc = base.opc.materialize_package_exact().expect("snapshot a materializes");
    let at = opc.parts.iter().position(|existing| existing.path == "word/media/to-remove.bin").expect("snapshot a carries to-remove.bin");
    let root = opc.relationships.relationships("").map_or(0, Vec::len);
    let last = base.xml_parts.len() - 1;
    let first_path = base.xml_parts.get(0).expect("a first XML part").path.clone();
    let last_path = base.xml_parts.get(last).expect("a last XML part").path.clone();
    let (relationship, part, entry) = demo_added_rows();
    DocxDiff {
        opc: Some(OpcDiff {
            comment: None,
            content_types: Some(OpcContentTypesDiff {
                defaults: Some(OpcContentTypeEntriesDelta { inserted: vec![OpcContentTypeInsertion { index: opc.content_types.defaults.len(), row: OpcContentTypeRow { name: entry.0, content_type: entry.1 } }], ..Default::default() }),
                overrides: None,
            }),
            parts: Some(OpcPartsDelta { removed: vec![OpcPartRemoval { id: "word/media/to-remove.bin".into(), index: at }], inserted: vec![OpcPartInsertion { index: at, row: part }], ..Default::default() }),
            relationships: Some(OpcOwnersDelta {
                modified: vec![OpcOwnerModification { id: String::new(), patch: OpcOwnerPatch { relationships: OpcRelationshipsDelta { inserted: vec![OpcRelationshipInsertion { index: root, row: relationship }], ..Default::default() } } }],
                ..Default::default()
            }),
        }),
        xml_parts: Some(DocxXmlPartsDelta {
            moved: vec![DocxXmlPartRelocation { id: first_path, from: 0, to: last }, DocxXmlPartRelocation { id: last_path, from: last, to: 0 }],
            modified: vec![DocxXmlPartModification { id: "word/document.xml".into(), patch: DocxXmlPartDiff { content_type: None, document: Some(XmlDiff { declaration: Some(Some(demo_declaration())), ..Default::default() }) } }],
            ..Default::default()
        }),
    }
}

/// 🧪️ The demo cases proper: the empty diff, the declared forward diff and its inverse.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<DocxDiff> {
    let forward = demo_forward_diff();
    let backward = forward.inverse(&snapshot_a());
    vec![DocxDiff::default(), forward, backward]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
mod handcrafted_diff_codec_tests;
//#endregion 🧪️Tests
//#endregion 🔖️HandcraftedDiffCodec

#[cfg(test)]
#[path = "🧪️tests/🔬️result-apply/🦀️.rs"]
mod result_apply_tests;
