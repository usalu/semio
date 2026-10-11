//! 🔺️ Handcrafted sparse diff over the canonical XLSX authority: non-XML OPC state plus one
//! logical XML document per XML-bearing content part. Semantic workbook data is never diffed as
//! an independent persisted tree.
//!
//! **OPC diff**: the shared [`OpcDiff`] of `zip::opc::diff` (content-type entries, parts and relationship owners as positional list deltas); the XML parts are a
//! positional list delta of the kernel (`protocol::list_delta`) whose patch carries the part's content type and its document diff.

#[cfg(test)]
use crate::schema::snapshot::XlsxWorkbook;
use crate::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxXmlPart};
use crate::schema::vocabulary::{element_matches, expanded_element_name, namespace_scope, SML_NS, SML_NS_STRICT};
use crate::XlsxSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use semio_s_artifact_stdio_xml::schema::diff::XmlDiff;
use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;
use semio_s_artifact_stdio_xml::{XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA};
#[cfg(test)]
use semio_s_artifact_stdio_zip::opc::{OpcContentTypes, OpcPackage, OpcPart, OpcRelationship, OpcTargetMode};
use semio_s_artifact_stdio_contract::kernel::list_delta::RowPatch;
use semio_s_artifact_stdio_contract::list_delta::compose_optional;
pub use semio_s_artifact_stdio_zip::opc::diff::OpcDiff;
use std::collections::BTreeMap;

//#region 🔖️XmlPartDiffTypes
/// 🩹 The sparse patch of one XML part: its content type and its document diff.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct XlsxXmlPartDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<XmlDiff>,
}

semio_s_artifact_stdio_contract::stdio_list_delta! {
    /// 🪡️ The positional delta of the XML parts, keyed by part name.
    pub XlsxXmlPartsDelta { removal: XlsxXmlPartRemoval, insertion: XlsxXmlPartInsertion, relocation: XlsxXmlPartRelocation, modification: XlsxXmlPartModification, row: XlsxXmlPart, patch: XlsxXmlPartDiff, key: path }
}
//#endregion 🔖️XmlPartDiffTypes

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.xlsx`.
/// Every keyed collection (content-type entries, OPC parts, relationships by owner, XML parts) is a positional list delta of the kernel (`protocol::list_delta`):
/// `removed`/`inserted`/`moved` rows with base/after coordinates and `modified` rows with sparse typed patches -- never a whole `order` key list.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.xlsx.diff")]
pub struct XlsxDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub opc: Option<OpcDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub xml_parts: Option<XlsxXmlPartsDelta>,
}
//#endregion 🔖️Diff

//#region 🔖️XmlPartDiffLogic
fn xml_snapshot(document: &semio_s_artifact_stdio_xml::schema::snapshot::XmlDocument) -> XmlSnapshot {
    XmlSnapshot { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc: document.clone() }
}

fn apply_xml_part(part: &mut XlsxXmlPart, diff: &XlsxXmlPartDiff, capability: protocol::ApplyCapability) -> MutationApplyResult<()> {
    if let Some(content_type) = &diff.content_type {
        part.content_type.clone_from(content_type);
    }
    if let Some(document) = &diff.document {
        part.document = document.apply(&xml_snapshot(&part.document), capability)?.doc;
    }
    Ok(())
}

impl RowPatch<XlsxXmlPart> for XlsxXmlPartDiff {
    fn commit_into(&self, row: &mut XlsxXmlPart, capability: protocol::ApplyCapability) -> Result<(), MutationApplyError> {
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

    fn inverse(&self, row: &XlsxXmlPart) -> Self {
        Self { content_type: self.content_type.as_ref().map(|_| row.content_type.clone()), document: self.document.as_ref().map(|document| document.inverse(&xml_snapshot(&row.document))) }
    }

    fn is_empty(&self) -> bool {
        self.content_type.is_none() && self.document.is_none()
    }
}

fn apply_xml_parts(items: &mut Vec<XlsxXmlPart>, diff: &XlsxXmlPartsDelta, capability: protocol::ApplyCapability) -> MutationApplyResult<()> {
    *items = diff.commit_onto(items, capability)?;
    Ok(())
}

fn rewind_xml_parts(base: &Vec<XlsxXmlPart>, diff: &XlsxXmlPartsDelta) -> XlsxXmlPartsDelta {
    diff.inverse(base)
}
/// 🔢️ `(advertised, actual)`: the `uniqueCount` a shared strings table (`sst` root) advertises and the number of `si` entries it holds; `None` for any other part and for a
/// table that advertises no readable count.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn shared_string_counts(part: &XlsxXmlPart) -> Option<(usize, usize)> {
    let root = part.document.root.as_ref()?;
    let XmlNode::Element { name, attrs, children } = root else { return None };
    let scope = namespace_scope(&[], root);
    let (namespace, local) = expanded_element_name(name, &scope).ok()?;
    if local != "sst" || ![SML_NS, SML_NS_STRICT].contains(&namespace.as_str()) {
        return None;
    }
    let advertised = attrs.iter().find(|attr| attr.name == "uniqueCount")?.value.parse::<usize>().ok()?;
    let actual = children.iter().filter(|child| element_matches(child, &namespace_scope(&scope, child), &[namespace.as_str()], "si").unwrap_or(false)).count();
    Some((advertised, actual))
}

/// 🧮️ The central applier's derived-data rule: a shared strings table whose advertised `uniqueCount` matched its entries before the diff keeps matching after it. The count is
/// never written by a leaf diff -- every kind that adds or removes an entry leaves it to this rule -- and a table that already disagreed with itself is left untouched.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn refresh_unique_counts(next: &mut Vec<XlsxXmlPart>, base: &[XlsxXmlPart], diff: &XlsxXmlPartsDelta) {
    for modification in &diff.modified {
        if !base.iter().find(|part| part.path == modification.id).and_then(shared_string_counts).is_some_and(|(advertised, actual)| advertised == actual) {
            continue;
        }
        let Some(part) = next.iter_mut().find(|part| part.path == modification.id) else { continue };
        let Some((_, actual)) = shared_string_counts(part) else { continue };
        if let Some(XmlNode::Element { attrs, .. }) = part.document.root.as_mut() {
            if let Some(attr) = attrs.iter_mut().find(|attr| attr.name == "uniqueCount") {
                attr.value = actual.to_string();
            }
        }
    }
}
//#endregion 🔖️XmlPartDiffLogic

//#region 🔖️Apply
impl MutationDiff<XlsxSnapshot> for XlsxDiff {
    fn apply(&self, base: &XlsxSnapshot, capability: protocol::ApplyCapability) -> MutationApplyResult<XlsxSnapshot> {
        let mut next = base.clone();
        if let Some(d) = &self.opc {
            d.commit_into(&mut next.opc, capability).map_err(|error| error.under(["opc"]))?;
        }
        if let Some(diff) = &self.xml_parts {
            apply_xml_parts(&mut next.xml_parts, diff, capability).map_err(|error| error.under(["xmlParts"]))?;
            refresh_unique_counts(&mut next.xml_parts, &base.xml_parts, diff);
        }
        next.validate_authority().map_err(|error| MutationApplyError::new("mutation.apply.invalid-snapshot", error.to_string()))?;
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
impl DiffAlgebra<XlsxSnapshot> for XlsxDiff {
    fn inverse(&self, base: &XlsxSnapshot) -> Self {
        XlsxDiff { opc: self.opc.as_ref().map(|d| d.rewind(&base.opc)), xml_parts: self.xml_parts.as_ref().map(|diff| rewind_xml_parts(&base.xml_parts, diff)) }
    }

    fn is_empty(&self) -> bool {
        self.opc.is_none() && self.xml_parts.is_none()
    }
}
//#endregion 🔖️DiffAlgebra

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: **hand-rolled** `protocol::DiffCodec` for `XlsxDiff` — required per the doc comment on
/// `XlsxDiff` itself (real `cargo check` failure: `XlsxCellValue: DslField` not satisfied, plus the
/// generic positional list delta type has no `DslField` impl either). Same grammar
/// style `GifDiff`/`SvgDiff`'s hand-rolled codecs use (bracket-depth-aware split, hex for
/// strings/bytes, `[0]`/`[1,x]` for `Option<T>`, `[removed];[modified];[added]` for collection
/// triples) — see `f6-recon-report.md` §5 for the primitive rationale; this file re-derives its own
/// copies of the small helper functions since each hand-rolled codec is self-contained (no shared
/// "hand-roll helpers" module exists yet). One addition beyond the gif/svg precedent: a GENERIC
/// `enc_triple`/`dec_triple` pair, since the positional list delta is reused across the canonical
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
    crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook {
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
    let mut snap = crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook {
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

/// 🧪️ The demo cases proper, built declaratively: `default()` (empty diff) and an archive comment edit.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<XlsxDiff> {
    vec![XlsxDiff::default(), XlsxDiff { opc: Some(OpcDiff { comment: Some("archive comment".into()), ..Default::default() }), xml_parts: None }]
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
