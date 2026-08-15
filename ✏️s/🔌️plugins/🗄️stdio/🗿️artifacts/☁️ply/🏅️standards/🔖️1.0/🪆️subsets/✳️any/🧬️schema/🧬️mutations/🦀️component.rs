//! 🧬️ PlyMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `PlyDiff` directly — apply-and-capture is banned) and `inverse()` is
//! handcrafted per variant, key/index-aware.
//!
//! 🧪️ F6 CONFIRMED (ticket `f6-recon-report.md` §9 STEP 1b, real `cargo check`, not guessed):
//! `#[derive(dsl::DslOps)]` on `PlyMutation` fails —
//! `error[E0277]: the trait bound `PlyValue: DslField` is not satisfied` at
//! `SetRowProperty { ..., value: PlyValue }` (this file), plus (same run) `PlySnapshot`/
//! `PlyElement`/`PlyRow` are ALL also `DslField`-unsatisfied transitively (`SetSnapshot`,
//! `AddElement`, `InsertRow` respectively) — every one of those ultimately bottoms out at
//! `PlyProperty`/`PlyValue`, the same two data-carrying enums that block the Diff side (see
//! `../🔺️diff/🦀️component.rs`'s module doc comment). `OpText`/`OpBinary` hand-rolled below,
//! reusing the diff file's `pub(crate)` grammar primitives (`hex_encode`/`enc_element`/
//! `split_top_level`/`encode_option`/...) rather than duplicating them a second time in this file.

use crate::artifacts::ply::schema::diff::{
    dec_element, dec_format, dec_row, dec_str, dec_value, diff_add_element, diff_insert_row, diff_remove_element, diff_remove_row, diff_set_comments, diff_set_format, diff_set_row_property, diff_set_snapshot, enc_element, enc_format, enc_row,
    enc_str, enc_value, read_bin_element, read_bin_row, read_bin_snapshot, read_bin_str, read_bin_value, split_top_level, strip_brackets, write_bin_element, write_bin_row, write_bin_snapshot, write_bin_str, write_bin_value, PlyDiff,
};
use crate::artifacts::ply::schema::snapshot::{PlyElement, PlyFormat, PlyRow, PlyValue};
use crate::artifacts::ply::PlySnapshot;
use protocol::Mutation;
#[cfg(test)]
use protocol::OpBinary;
use protocol::OpText;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.ply`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum PlyMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: PlySnapshot,
    },
    SetFormat {
        format: PlyFormat,
    },
    InsertComment {
        index: usize,
        comment: String,
    },
    RemoveComment {
        index: usize,
    },
    AddElement {
        index: usize,
        element: PlyElement,
    },
    RemoveElement {
        name: String,
    },
    InsertRow {
        element_name: String,
        index: usize,
        row: PlyRow,
    },
    RemoveRow {
        element_name: String,
        index: usize,
    },
    SetRowProperty {
        element_name: String,
        row_index: usize,
        property_name: String,
        value: PlyValue,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: the diff is the single semantics source
/// (`let d = mutation.diff(&*snapshot); *snapshot = d.apply(snapshot); d`).
pub fn apply_ply_mutation(snapshot: &mut PlySnapshot, mutation: &PlyMutation) -> PlyDiff {
    let diff = <PlyMutation as protocol::Mutation<PlySnapshot>>::diff(mutation, snapshot);
    *snapshot = <PlyDiff as protocol::MutationDiff<PlySnapshot>>::apply(&diff, snapshot);
    diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<PlySnapshot> for PlyMutation {
    type Diff = PlyDiff;

    /// 🔺️ Every variant handcrafted directly — never apply-and-capture.
    fn diff(&self, base: &PlySnapshot) -> Self::Diff {
        match self {
            PlyMutation::NoMutation => PlyDiff::default(),
            PlyMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            PlyMutation::SetFormat { format } => diff_set_format(*format),
            PlyMutation::InsertComment { index, comment } => {
                let mut comments = base.comments.clone();
                let at = (*index).min(comments.len());
                comments.insert(at, comment.clone());
                diff_set_comments(comments)
            }
            PlyMutation::RemoveComment { index } => {
                let mut comments = base.comments.clone();
                if *index < comments.len() {
                    comments.remove(*index);
                }
                diff_set_comments(comments)
            }
            PlyMutation::AddElement { index, element } => diff_add_element(*index, element.clone()),
            PlyMutation::RemoveElement { name } => diff_remove_element(name),
            PlyMutation::InsertRow { element_name, index, row } => diff_insert_row(element_name, *index, row.clone()),
            PlyMutation::RemoveRow { element_name, index } => diff_remove_row(element_name, *index),
            PlyMutation::SetRowProperty { element_name, row_index, property_name, value } => diff_set_row_property(element_name, *row_index, property_name, value.clone()),
        }
    }

    /// ↩️ Handcrafted per-variant undo, key/index-aware (resolves against `base` so e.g. a
    /// clamped insert position or a to-be-removed payload is recovered exactly).
    fn inverse(&self, base: &PlySnapshot) -> Vec<Self> {
        match self {
            PlyMutation::NoMutation => vec![PlyMutation::NoMutation],
            PlyMutation::SetSnapshot { .. } => vec![PlyMutation::SetSnapshot { snapshot: base.clone() }],
            PlyMutation::SetFormat { .. } => vec![PlyMutation::SetFormat { format: base.format }],
            PlyMutation::InsertComment { index, .. } => {
                let at = (*index).min(base.comments.len());
                vec![PlyMutation::RemoveComment { index: at }]
            }
            PlyMutation::RemoveComment { index } => match base.comments.get(*index) {
                Some(comment) => vec![PlyMutation::InsertComment { index: *index, comment: comment.clone() }],
                None => vec![PlyMutation::NoMutation],
            },
            PlyMutation::AddElement { element, .. } => vec![PlyMutation::RemoveElement { name: element.name.clone() }],
            PlyMutation::RemoveElement { name } => match base.elements.iter().position(|e| &e.name == name) {
                Some(idx) => vec![PlyMutation::AddElement { index: idx, element: base.elements[idx].clone() }],
                None => vec![PlyMutation::NoMutation],
            },
            PlyMutation::InsertRow { element_name, index, .. } => {
                let at = base.elements.iter().find(|e| &e.name == element_name).map(|e| (*index).min(e.rows.len())).unwrap_or(*index);
                vec![PlyMutation::RemoveRow { element_name: element_name.clone(), index: at }]
            }
            PlyMutation::RemoveRow { element_name, index } => match base.elements.iter().find(|e| &e.name == element_name).and_then(|e| e.rows.get(*index)) {
                Some(row) => vec![PlyMutation::InsertRow { element_name: element_name.clone(), index: *index, row: row.clone() }],
                None => vec![PlyMutation::NoMutation],
            },
            PlyMutation::SetRowProperty { element_name, row_index, property_name, .. } => {
                let prior = base.elements.iter().find(|e| &e.name == element_name).and_then(|el| {
                    let prop_idx = el.properties.iter().position(|p| p.name() == property_name)?;
                    el.rows.get(*row_index)?.values.get(prop_idx).cloned()
                });
                match prior {
                    Some(value) => vec![PlyMutation::SetRowProperty { element_name: element_name.clone(), row_index: *row_index, property_name: property_name.clone(), value }],
                    None => vec![PlyMutation::NoMutation],
                }
            }
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ F6: **hand-rolled** `OpText`/`OpBinary` for `PlyMutation` (`#[derive(dsl::DslOps)]`
/// confirmed rejected above) — reuses `PlyDiff`'s `pub(crate)` grammar primitives
/// (`hex_encode`/`enc_element`/`enc_row`/`enc_value`/`split_top_level`/`encode_option`/...)
/// rather than duplicating them a second time in this file. Grammar: `keyword arg=value ...`
/// (space-separated, same shape the derive's own handcrafted-wrapper convention uses, and the
/// same shape svg's hand-rolled `OpText` uses), one match arm per variant (no `DslVariants`
/// scaffolding available since nothing here derives it).
fn enc_snapshot(s: &PlySnapshot) -> String {
    format!("[{},{},[{}],[{}]]", enc_str(&s.schema), enc_format(s.format), s.comments.iter().map(|c| enc_str(c)).collect::<Vec<_>>().join(","), s.elements.iter().map(enc_element).collect::<Vec<_>>().join(","),)
}
fn dec_snapshot(s: &str) -> Result<PlySnapshot, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [schema, format, comments, elements] = parts.as_slice() else { return Err(format!("ply snapshot: expected 4 fields, got {}", parts.len())) };
    Ok(PlySnapshot {
        schema: dec_str(schema)?,
        format: dec_format(format)?,
        comments: split_top_level(strip_brackets(comments)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_str).collect::<Result<Vec<_>, String>>()?,
        elements: split_top_level(strip_brackets(elements)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_element).collect::<Result<Vec<_>, String>>()?,
    })
}

fn print_ply_mutation(m: &PlyMutation) -> String {
    match m {
        PlyMutation::NoMutation => "no-mutation".to_string(),
        PlyMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_snapshot(snapshot)),
        PlyMutation::SetFormat { format } => format!("set-format format={}", enc_format(*format)),
        PlyMutation::InsertComment { index, comment } => format!("insert-comment index={index} comment={}", enc_str(comment)),
        PlyMutation::RemoveComment { index } => format!("remove-comment index={index}"),
        PlyMutation::AddElement { index, element } => format!("add-element index={index} element={}", enc_element(element)),
        PlyMutation::RemoveElement { name } => format!("remove-element name={}", enc_str(name)),
        PlyMutation::InsertRow { element_name, index, row } => format!("insert-row element-name={} index={index} row={}", enc_str(element_name), enc_row(row)),
        PlyMutation::RemoveRow { element_name, index } => format!("remove-row element-name={} index={index}", enc_str(element_name)),
        PlyMutation::SetRowProperty { element_name, row_index, property_name, value } => format!("set-row-property element-name={} row-index={row_index} property-name={} value={}", enc_str(element_name), enc_str(property_name), enc_value(value),),
    }
}
fn parse_ply_mutation(line: &str) -> Result<PlyMutation, String> {
    if line == "no-mutation" {
        return Ok(PlyMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("ply mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("ply mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(PlyMutation::SetSnapshot { snapshot: dec_snapshot(arg("snapshot")?)? }),
        "set-format" => Ok(PlyMutation::SetFormat { format: dec_format(arg("format")?)? }),
        "insert-comment" => Ok(PlyMutation::InsertComment { index: usize_arg("index")?, comment: dec_str(arg("comment")?)? }),
        "remove-comment" => Ok(PlyMutation::RemoveComment { index: usize_arg("index")? }),
        "add-element" => Ok(PlyMutation::AddElement { index: usize_arg("index")?, element: dec_element(arg("element")?)? }),
        "remove-element" => Ok(PlyMutation::RemoveElement { name: dec_str(arg("name")?)? }),
        "insert-row" => Ok(PlyMutation::InsertRow { element_name: dec_str(arg("element-name")?)?, index: usize_arg("index")?, row: dec_row(arg("row")?)? }),
        "remove-row" => Ok(PlyMutation::RemoveRow { element_name: dec_str(arg("element-name")?)?, index: usize_arg("index")? }),
        "set-row-property" => Ok(PlyMutation::SetRowProperty { element_name: dec_str(arg("element-name")?)?, row_index: usize_arg("row-index")?, property_name: dec_str(arg("property-name")?)?, value: dec_value(arg("value")?)? }),
        other => Err(format!("ply mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for PlyMutation {
    fn print_op(&self) -> String {
        print_ply_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_ply_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

//#region 🔖️RealBinaryOpFrame
/// 🧪️ P2-FG3: real binary op frame — upgraded from the F6-era `print_op().into_bytes()`
/// text-as-binary shortcut. Matches `../💾️binary/📡️component.protocol.semio`'s `header fixed 2 |
/// field format u8 | field tag u8 | chain payload bytes` shape exactly — `format` is
/// `store::pack_rt::OP_BINARY_FORMAT`, `tag` is the `PlyMutation` variant ordinal in the SAME
/// 0-9 order `print_ply_mutation`'s own match uses, then each variant's own payload real binary
/// (reusing `PlyDiff`'s `pub(crate)` binary primitives — `write_bin_element`/`write_bin_row`/
/// `write_bin_snapshot`/`write_bin_str`/`write_bin_value` — the same way this file's `OpText`
/// already reuses the text-codec primitives).
fn op_tag(m: &PlyMutation) -> u8 {
    match m {
        PlyMutation::NoMutation => 0,
        PlyMutation::SetSnapshot { .. } => 1,
        PlyMutation::SetFormat { .. } => 2,
        PlyMutation::InsertComment { .. } => 3,
        PlyMutation::RemoveComment { .. } => 4,
        PlyMutation::AddElement { .. } => 5,
        PlyMutation::RemoveElement { .. } => 6,
        PlyMutation::InsertRow { .. } => 7,
        PlyMutation::RemoveRow { .. } => 8,
        PlyMutation::SetRowProperty { .. } => 9,
    }
}
fn op_pack_err(e: dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "ply op binary", offset: 0, detail: e.to_string() }
}

impl protocol::OpBinary for PlyMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut w = dsl::ByteWriter::new();
        w.write_u8(store::pack_rt::OP_BINARY_FORMAT);
        w.write_u8(op_tag(self));
        match self {
            PlyMutation::NoMutation => {}
            PlyMutation::SetSnapshot { snapshot } => write_bin_snapshot(&mut w, snapshot),
            PlyMutation::SetFormat { format } => {
                crate::artifacts::ply::schema::diff::write_bin_format(&mut w, *format);
            }
            PlyMutation::InsertComment { index, comment } => {
                w.write_varint_u64(*index as u64);
                write_bin_str(&mut w, comment);
            }
            PlyMutation::RemoveComment { index } => w.write_varint_u64(*index as u64),
            PlyMutation::AddElement { index, element } => {
                w.write_varint_u64(*index as u64);
                write_bin_element(&mut w, element);
            }
            PlyMutation::RemoveElement { name } => write_bin_str(&mut w, name),
            PlyMutation::InsertRow { element_name, index, row } => {
                write_bin_str(&mut w, element_name);
                w.write_varint_u64(*index as u64);
                write_bin_row(&mut w, row);
            }
            PlyMutation::RemoveRow { element_name, index } => {
                write_bin_str(&mut w, element_name);
                w.write_varint_u64(*index as u64);
            }
            PlyMutation::SetRowProperty { element_name, row_index, property_name, value } => {
                write_bin_str(&mut w, element_name);
                w.write_varint_u64(*row_index as u64);
                write_bin_str(&mut w, property_name);
                write_bin_value(&mut w, value);
            }
        }
        Ok(w.into_bytes())
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut r = dsl::ByteReader::new(bytes);
        let _format = r.read_u8().map_err(op_pack_err)?;
        let tag = r.read_u8().map_err(op_pack_err)?;
        match tag {
            0 => Ok(PlyMutation::NoMutation),
            1 => Ok(PlyMutation::SetSnapshot { snapshot: read_bin_snapshot(&mut r).map_err(op_pack_err)? }),
            2 => Ok(PlyMutation::SetFormat { format: crate::artifacts::ply::schema::diff::read_bin_format(&mut r).map_err(op_pack_err)? }),
            3 => {
                let index = r.read_varint_u64().map_err(op_pack_err)? as usize;
                let comment = read_bin_str(&mut r).map_err(op_pack_err)?;
                Ok(PlyMutation::InsertComment { index, comment })
            }
            4 => Ok(PlyMutation::RemoveComment { index: r.read_varint_u64().map_err(op_pack_err)? as usize }),
            5 => {
                let index = r.read_varint_u64().map_err(op_pack_err)? as usize;
                let element = read_bin_element(&mut r).map_err(op_pack_err)?;
                Ok(PlyMutation::AddElement { index, element })
            }
            6 => Ok(PlyMutation::RemoveElement { name: read_bin_str(&mut r).map_err(op_pack_err)? }),
            7 => {
                let element_name = read_bin_str(&mut r).map_err(op_pack_err)?;
                let index = r.read_varint_u64().map_err(op_pack_err)? as usize;
                let row = read_bin_row(&mut r).map_err(op_pack_err)?;
                Ok(PlyMutation::InsertRow { element_name, index, row })
            }
            8 => {
                let element_name = read_bin_str(&mut r).map_err(op_pack_err)?;
                let index = r.read_varint_u64().map_err(op_pack_err)? as usize;
                Ok(PlyMutation::RemoveRow { element_name, index })
            }
            9 => {
                let element_name = read_bin_str(&mut r).map_err(op_pack_err)?;
                let row_index = r.read_varint_u64().map_err(op_pack_err)? as usize;
                let property_name = read_bin_str(&mut r).map_err(op_pack_err)?;
                let value = read_bin_value(&mut r).map_err(op_pack_err)?;
                Ok(PlyMutation::SetRowProperty { element_name, row_index, property_name, value })
            }
            other => Err(protocol::ProtocolError::Malformed { what: "ply op tag", offset: 1, detail: format!("unknown tag {other}") }),
        }
    }
}
//#endregion 🔖️RealBinaryOpFrame
//#endregion OpCodecs

//#region 🔖️DemoMutationCases
/// ✅️ Every `PlyMutation` variant built off a small `base()` snapshot — the single case list
/// `op_text_binary_roundtrip_law` (this file) AND `ops_grammar_conformance_law`/
/// `protocol_walk_law` (`⚙️engine/🦀️component.rs`) all exercise. Covers `SetSnapshot`'s whole
/// nested snapshot, `AddElement`'s bare `PlyElement` payload (itself containing `PlyProperty`),
/// `InsertRow`'s `PlyRow` payload, and `SetRowProperty`'s bare `PlyValue` payload (incl. the
/// recursive `List` variant).
#[cfg(test)]
fn demo_base_snapshot() -> PlySnapshot {
    use crate::artifacts::ply::schema::snapshot::{PlyProperty, PlyScalarType};
    PlySnapshot {
        schema: crate::artifacts::ply::STDIO_PLY_DOCUMENT_SCHEMA.into(),
        format: PlyFormat::Ascii,
        comments: vec!["hi".into()],
        elements: vec![PlyElement { name: "vertex".into(), count: 1, properties: vec![PlyProperty::Scalar { name: "x".into(), kind: PlyScalarType::Float }], rows: vec![PlyRow { values: vec![PlyValue::Float(1.5)] }] }],
    }
}

#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<PlyMutation> {
    use crate::artifacts::ply::schema::snapshot::{PlyProperty, PlyScalarType};
    let snapshot = demo_base_snapshot();
    vec![
        PlyMutation::NoMutation,
        PlyMutation::SetSnapshot { snapshot: snapshot.clone() },
        PlyMutation::SetFormat { format: PlyFormat::BinaryBigEndian },
        PlyMutation::InsertComment { index: 0, comment: "new comment".into() },
        PlyMutation::RemoveComment { index: 0 },
        PlyMutation::AddElement {
            index: 1,
            element: PlyElement {
                name: "face".into(),
                count: 1,
                properties: vec![PlyProperty::List { name: "vertex_indices".into(), count_kind: PlyScalarType::UChar, value_kind: PlyScalarType::Int }],
                rows: vec![PlyRow { values: vec![PlyValue::List(vec![PlyValue::Int(0), PlyValue::Int(1), PlyValue::Int(2)])] }],
            },
        },
        PlyMutation::RemoveElement { name: "vertex".into() },
        PlyMutation::InsertRow { element_name: "vertex".into(), index: 0, row: PlyRow { values: vec![PlyValue::Float(-2.5)] } },
        PlyMutation::RemoveRow { element_name: "vertex".into(), index: 0 },
        PlyMutation::SetRowProperty { element_name: "vertex".into(), row_index: 0, property_name: "x".into(), value: PlyValue::Float(42.0) },
        PlyMutation::SetRowProperty { element_name: "face".into(), row_index: 0, property_name: "vertex_indices".into(), value: PlyValue::List(vec![PlyValue::Int(3), PlyValue::Int(4), PlyValue::Int(5)]) },
    ]
}
//#endregion 🔖️DemoMutationCases

//#region 🧪️Tests
#[cfg(test)]
mod codec_tests {
    use super::*;

    /// 🧪️ F6/P2-FG3: `OpText`/`OpBinary` round-trip laws for the hand-rolled `PlyMutation`
    /// grammar — `OpBinary` is now a REAL binary frame, no longer text-as-bytes.
    #[test]
    fn op_text_binary_roundtrip_law() {
        for mutation in demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = PlyMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = PlyMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }
}
//#endregion 🧪️Tests
