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
    dec_element, dec_format, dec_row, dec_str, dec_value, diff_add_element, diff_insert_row,
    diff_remove_element, diff_remove_row, diff_set_comments, diff_set_format,
    diff_set_row_property, diff_set_snapshot, enc_element, enc_format, enc_row, enc_str,
    enc_value, split_top_level, strip_brackets, PlyDiff,
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
    SetSnapshot { snapshot: PlySnapshot },
    SetFormat { format: PlyFormat },
    InsertComment { index: usize, comment: String },
    RemoveComment { index: usize },
    AddElement { index: usize, element: PlyElement },
    RemoveElement { name: String },
    InsertRow { element_name: String, index: usize, row: PlyRow },
    RemoveRow { element_name: String, index: usize },
    SetRowProperty { element_name: String, row_index: usize, property_name: String, value: PlyValue },
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
                if *index < comments.len() { comments.remove(*index); }
                diff_set_comments(comments)
            }
            PlyMutation::AddElement { index, element } => diff_add_element(*index, element.clone()),
            PlyMutation::RemoveElement { name } => diff_remove_element(name),
            PlyMutation::InsertRow { element_name, index, row } => diff_insert_row(element_name, *index, row.clone()),
            PlyMutation::RemoveRow { element_name, index } => diff_remove_row(element_name, *index),
            PlyMutation::SetRowProperty { element_name, row_index, property_name, value } => {
                diff_set_row_property(element_name, *row_index, property_name, value.clone())
            }
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
                let at = base.elements.iter().find(|e| &e.name == element_name)
                    .map(|e| (*index).min(e.rows.len()))
                    .unwrap_or(*index);
                vec![PlyMutation::RemoveRow { element_name: element_name.clone(), index: at }]
            }
            PlyMutation::RemoveRow { element_name, index } => {
                match base.elements.iter().find(|e| &e.name == element_name).and_then(|e| e.rows.get(*index)) {
                    Some(row) => vec![PlyMutation::InsertRow { element_name: element_name.clone(), index: *index, row: row.clone() }],
                    None => vec![PlyMutation::NoMutation],
                }
            }
            PlyMutation::SetRowProperty { element_name, row_index, property_name, .. } => {
                let prior = base.elements.iter().find(|e| &e.name == element_name).and_then(|el| {
                    let prop_idx = el.properties.iter().position(|p| p.name() == property_name)?;
                    el.rows.get(*row_index)?.values.get(prop_idx).cloned()
                });
                match prior {
                    Some(value) => vec![PlyMutation::SetRowProperty {
                        element_name: element_name.clone(),
                        row_index: *row_index,
                        property_name: property_name.clone(),
                        value,
                    }],
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
    format!(
        "[{},{},[{}],[{}]]",
        enc_str(&s.schema),
        enc_format(s.format),
        s.comments.iter().map(|c| enc_str(c)).collect::<Vec<_>>().join(","),
        s.elements.iter().map(enc_element).collect::<Vec<_>>().join(","),
    )
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
        PlyMutation::SetRowProperty { element_name, row_index, property_name, value } => format!(
            "set-row-property element-name={} row-index={row_index} property-name={} value={}",
            enc_str(element_name), enc_str(property_name), enc_value(value),
        ),
    }
}
fn parse_ply_mutation(line: &str) -> Result<PlyMutation, String> {
    if line == "no-mutation" {
        return Ok(PlyMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|tok| tok.split_once('=').ok_or_else(|| format!("ply mutation: bad arg token {tok:?}")))
        .collect::<Result<Vec<_>, String>>()?
        .into_iter()
        .collect();
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
        "set-row-property" => Ok(PlyMutation::SetRowProperty {
            element_name: dec_str(arg("element-name")?)?,
            row_index: usize_arg("row-index")?,
            property_name: dec_str(arg("property-name")?)?,
            value: dec_value(arg("value")?)?,
        }),
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

/// ⚡️ Binary = the text bytes verbatim, same simplification as `PlyDiff`'s hand-rolled codec.
impl protocol::OpBinary for PlyMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_op().into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_op(line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 0, detail: e.to_string() })
    }
}
//#endregion OpCodecs

//#region 🧪️Tests
#[cfg(test)]
mod codec_tests {
    use super::*;
    use crate::artifacts::ply::schema::snapshot::{PlyProperty, PlyScalarType};

    fn base() -> PlySnapshot {
        PlySnapshot {
            schema: crate::artifacts::ply::STDIO_PLY_DOCUMENT_SCHEMA.into(),
            format: PlyFormat::Ascii,
            comments: vec!["hi".into()],
            elements: vec![PlyElement {
                name: "vertex".into(),
                count: 1,
                properties: vec![PlyProperty::Scalar { name: "x".into(), kind: PlyScalarType::Float }],
                rows: vec![PlyRow { values: vec![PlyValue::Float(1.5)] }],
            }],
        }
    }

    /// 🧪️ F6: `OpText`/`OpBinary` round-trip laws for the hand-rolled `PlyMutation` grammar —
    /// exercises every variant incl. `SetSnapshot`'s whole nested snapshot, `AddElement`'s bare
    /// `PlyElement` payload (itself containing `PlyProperty`), `InsertRow`'s `PlyRow` payload, and
    /// `SetRowProperty`'s bare `PlyValue` payload (incl. the recursive `List` variant).
    #[test]
    fn op_text_binary_roundtrip_law() {
        let snapshot = base();
        let mutations = vec![
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
            PlyMutation::SetRowProperty {
                element_name: "face".into(),
                row_index: 0,
                property_name: "vertex_indices".into(),
                value: PlyValue::List(vec![PlyValue::Int(3), PlyValue::Int(4), PlyValue::Int(5)]),
            },
        ];
        for mutation in mutations {
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
