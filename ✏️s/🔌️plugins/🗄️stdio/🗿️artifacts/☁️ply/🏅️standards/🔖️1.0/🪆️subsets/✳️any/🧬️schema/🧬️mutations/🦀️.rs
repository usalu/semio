//! 🧬️ PlyMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `PlyDiff` directly — apply-and-capture is banned) and `inverse()` is
//! handcrafted per variant, key/index-aware.
//!
//! 🪆️ Migrated to `#[derive(dsl::Mutations)]` over one mutation-leaf module per variant
//! (mutation-leaf migration recipe, mirroring the stdio.tiff baseline subset's own
//! `🧬️schema/🧬️mutations/🦀️.rs`) to satisfy `protocol::Mutation<P>`'s new
//! `DESCRIPTORS`/`descriptor()` requirement (E0046). `NoMutation` was dropped: the derive requires
//! every variant to wrap exactly one leaf payload, a wrapped variant cannot be `#[default]`, and
//! `"no"` is not an `APPROVED_VERBS` entry. `diff`/`inverse` bodies moved verbatim into
//! `agg_diff`/`agg_inverse` free functions below; each leaf's own `MutationKind` impl delegates
//! back into them.
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
use protocol::OpBinary;
use protocol::OpText;

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.ply`.
//#region 🔖️Leaves
#[path = "📄set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🎚set-format/🦀️.rs"]
pub mod set_format;
#[path = "💬insert-comment/🦀️.rs"]
pub mod insert_comment;
#[path = "🗑remove-comment/🦀️.rs"]
pub mod remove_comment;
#[path = "🧱add-element/🦀️.rs"]
pub mod add_element;
#[path = "🚮remove-element/🦀️.rs"]
pub mod remove_element;
#[path = "📥insert-row/🦀️.rs"]
pub mod insert_row;
#[path = "📤remove-row/🦀️.rs"]
pub mod remove_row;
#[path = "🏷set-row-property/🦀️.rs"]
pub mod set_row_property;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = PlySnapshot, diff = PlyDiff, schema = "PlyMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum PlyMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    SetFormat(set_format::SetFormat),
    InsertComment(insert_comment::InsertComment),
    RemoveComment(remove_comment::RemoveComment),
    AddElement(add_element::AddElement),
    RemoveElement(remove_element::RemoveElement),
    InsertRow(insert_row::InsertRow),
    RemoveRow(remove_row::RemoveRow),
    SetRowProperty(set_row_property::SetRowProperty),
}

/// 🏷️ Kebab-case spelling of every `PlyMutation` variant, in declaration order — the vocabulary the
/// `ply-1-0-any` mutation catalog (`../../🧪️oracle/🔣️.json`) declares and the exhaustive
/// mutate/inverse test case measures itself against. `kinds_cover_every_variant` below is what keeps
/// this list honest against the enum it names, since the framework never parses Rust.
pub const KINDS: &[&str] = &["set-snapshot", "set-format", "insert-comment", "remove-comment", "add-element", "remove-element", "insert-row", "remove-row", "set-row-property"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: the diff is the single semantics source
/// (`let d = mutation.diff(&*snapshot); *snapshot = d.apply(snapshot); d`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_ply_mutation(snapshot: &mut PlySnapshot, mutation: &PlyMutation) -> protocol::MutationOutcome<PlyDiff> {
    let outcome = <PlyMutation as Mutation<PlySnapshot>>::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
/// 🔺️ Every variant handcrafted directly — never apply-and-capture. Lifted verbatim from the
/// former `impl Mutation<PlySnapshot> for PlyMutation`'s `diff`; only each match arm's pattern
/// head changed, from `PlyMutation::Variant { .. }` to `PlyMutation::Variant(variant_mod::Variant { .. })`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn agg_diff(this: &PlyMutation, base: &PlySnapshot) -> protocol::MutationOutcome<PlyDiff> {
    protocol::MutationOutcome::new(match this {
        PlyMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        PlyMutation::SetFormat(set_format::SetFormat { format }) => diff_set_format(*format),
        PlyMutation::InsertComment(insert_comment::InsertComment { index, comment }) => {
            let mut comments = base.comments.clone();
            let at = (*index).min(comments.len());
            comments.insert(at, comment.clone());
            diff_set_comments(comments)
        }
        PlyMutation::RemoveComment(remove_comment::RemoveComment { index }) => {
            let mut comments = base.comments.clone();
            if *index < comments.len() {
                comments.remove(*index);
            }
            diff_set_comments(comments)
        }
        PlyMutation::AddElement(add_element::AddElement { index, element }) => diff_add_element(*index, element.clone()),
        PlyMutation::RemoveElement(remove_element::RemoveElement { name }) => diff_remove_element(name),
        PlyMutation::InsertRow(insert_row::InsertRow { element_name, index, row }) => diff_insert_row(element_name, *index, row.clone()),
        PlyMutation::RemoveRow(remove_row::RemoveRow { element_name, index }) => diff_remove_row(element_name, *index),
        PlyMutation::SetRowProperty(set_row_property::SetRowProperty { element_name, row_index, property_name, value }) => diff_set_row_property(element_name, *row_index, property_name, value.clone()),
    })
}

/// ↩️ Handcrafted per-variant undo, key/index-aware (resolves against `base` so e.g. a
/// clamped insert position or a to-be-removed payload is recovered exactly). Lifted verbatim from
/// the former `impl Mutation<PlySnapshot> for PlyMutation`'s `inverse`; the old `NoMutation`
/// fallback arms now return `Vec::new()` — [`protocol::MutationKind::inverse`]'s own documented
/// replacement for the dropped sentinel: "there is no no-op mutation, only an inverse with nothing
/// to undo."
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn agg_inverse(this: &PlyMutation, base: &PlySnapshot) -> Vec<PlyMutation> {
    match this {
        PlyMutation::SetSnapshot(_) => vec![PlyMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        PlyMutation::SetFormat(_) => vec![PlyMutation::SetFormat(set_format::SetFormat { format: base.format })],
        PlyMutation::InsertComment(insert_comment::InsertComment { index, .. }) => {
            let at = (*index).min(base.comments.len());
            vec![PlyMutation::RemoveComment(remove_comment::RemoveComment { index: at })]
        }
        PlyMutation::RemoveComment(remove_comment::RemoveComment { index }) => match base.comments.get(*index) {
            Some(comment) => vec![PlyMutation::InsertComment(insert_comment::InsertComment { index: *index, comment: comment.clone() })],
            None => Vec::new(),
        },
        PlyMutation::AddElement(add_element::AddElement { element, .. }) => vec![PlyMutation::RemoveElement(remove_element::RemoveElement { name: element.name.clone() })],
        PlyMutation::RemoveElement(remove_element::RemoveElement { name }) => match base.elements.iter().position(|e| &e.name == name) {
            Some(idx) => vec![PlyMutation::AddElement(add_element::AddElement { index: idx, element: base.elements[idx].clone() })],
            None => Vec::new(),
        },
        PlyMutation::InsertRow(insert_row::InsertRow { element_name, index, .. }) => {
            let at = base.elements.iter().find(|e| &e.name == element_name).map(|e| (*index).min(e.rows.len())).unwrap_or(*index);
            vec![PlyMutation::RemoveRow(remove_row::RemoveRow { element_name: element_name.clone(), index: at })]
        }
        PlyMutation::RemoveRow(remove_row::RemoveRow { element_name, index }) => match base.elements.iter().find(|e| &e.name == element_name).and_then(|e| e.rows.get(*index)) {
            Some(row) => vec![PlyMutation::InsertRow(insert_row::InsertRow { element_name: element_name.clone(), index: *index, row: row.clone() })],
            None => Vec::new(),
        },
        PlyMutation::SetRowProperty(set_row_property::SetRowProperty { element_name, row_index, property_name, .. }) => {
            let prior = base.elements.iter().find(|e| &e.name == element_name).and_then(|el| {
                let prop_idx = el.properties.iter().position(|p| p.name() == property_name)?;
                el.rows.get(*row_index)?.values.get(prop_idx).cloned()
            });
            match prior {
                Some(value) => vec![PlyMutation::SetRowProperty(set_row_property::SetRowProperty { element_name: element_name.clone(), row_index: *row_index, property_name: property_name.clone(), value })],
                None => Vec::new(),
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_snapshot(s: &PlySnapshot) -> String {
    format!("[{},{},[{}],[{}]]", enc_str(&s.schema), enc_format(s.format), s.comments.iter().map(|c| enc_str(c)).collect::<Vec<_>>().join(","), s.elements.iter().map(enc_element).collect::<Vec<_>>().join(","),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_ply_mutation(m: &PlyMutation) -> String {
    match m {
        PlyMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_snapshot(snapshot)),
        PlyMutation::SetFormat(set_format::SetFormat { format }) => format!("set-format format={}", enc_format(*format)),
        PlyMutation::InsertComment(insert_comment::InsertComment { index, comment }) => format!("insert-comment index={index} comment={}", enc_str(comment)),
        PlyMutation::RemoveComment(remove_comment::RemoveComment { index }) => format!("remove-comment index={index}"),
        PlyMutation::AddElement(add_element::AddElement { index, element }) => format!("add-element index={index} element={}", enc_element(element)),
        PlyMutation::RemoveElement(remove_element::RemoveElement { name }) => format!("remove-element name={}", enc_str(name)),
        PlyMutation::InsertRow(insert_row::InsertRow { element_name, index, row }) => format!("insert-row element-name={} index={index} row={}", enc_str(element_name), enc_row(row)),
        PlyMutation::RemoveRow(remove_row::RemoveRow { element_name, index }) => format!("remove-row element-name={} index={index}", enc_str(element_name)),
        PlyMutation::SetRowProperty(set_row_property::SetRowProperty { element_name, row_index, property_name, value }) => format!("set-row-property element-name={} row-index={row_index} property-name={} value={}", enc_str(element_name), enc_str(property_name), enc_value(value),),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_ply_mutation(line: &str) -> Result<PlyMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("ply mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("ply mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(PlyMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_snapshot(arg("snapshot")?)? })),
        "set-format" => Ok(PlyMutation::SetFormat(set_format::SetFormat { format: dec_format(arg("format")?)? })),
        "insert-comment" => Ok(PlyMutation::InsertComment(insert_comment::InsertComment { index: usize_arg("index")?, comment: dec_str(arg("comment")?)? })),
        "remove-comment" => Ok(PlyMutation::RemoveComment(remove_comment::RemoveComment { index: usize_arg("index")? })),
        "add-element" => Ok(PlyMutation::AddElement(add_element::AddElement { index: usize_arg("index")?, element: dec_element(arg("element")?)? })),
        "remove-element" => Ok(PlyMutation::RemoveElement(remove_element::RemoveElement { name: dec_str(arg("name")?)? })),
        "insert-row" => Ok(PlyMutation::InsertRow(insert_row::InsertRow { element_name: dec_str(arg("element-name")?)?, index: usize_arg("index")?, row: dec_row(arg("row")?)? })),
        "remove-row" => Ok(PlyMutation::RemoveRow(remove_row::RemoveRow { element_name: dec_str(arg("element-name")?)?, index: usize_arg("index")? })),
        "set-row-property" => Ok(PlyMutation::SetRowProperty(set_row_property::SetRowProperty { element_name: dec_str(arg("element-name")?)?, row_index: usize_arg("row-index")?, property_name: dec_str(arg("property-name")?)?, value: dec_value(arg("value")?)? })),
        other => Err(format!("ply mutation: unknown keyword {other:?}")),
    }
}

impl OpText for PlyMutation {
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
/// 0-8 order `print_ply_mutation`'s own match uses, then each variant's own payload real binary
/// (reusing `PlyDiff`'s `pub(crate)` binary primitives — `write_bin_element`/`write_bin_row`/
/// `write_bin_snapshot`/`write_bin_str`/`write_bin_value` — the same way this file's `OpText`
/// already reuses the text-codec primitives).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn op_tag(m: &PlyMutation) -> u8 {
    match m {
        PlyMutation::SetSnapshot(..) => 0,
        PlyMutation::SetFormat(..) => 1,
        PlyMutation::InsertComment(..) => 2,
        PlyMutation::RemoveComment(..) => 3,
        PlyMutation::AddElement(..) => 4,
        PlyMutation::RemoveElement(..) => 5,
        PlyMutation::InsertRow(..) => 6,
        PlyMutation::RemoveRow(..) => 7,
        PlyMutation::SetRowProperty(..) => 8,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn op_pack_err(e: dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "ply op binary", offset: 0, detail: e.to_string() }
}

impl OpBinary for PlyMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut w = dsl::ByteWriter::new();
        w.write_u8(store::pack_rt::OP_BINARY_FORMAT);
        w.write_u8(op_tag(self));
        match self {
            PlyMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => write_bin_snapshot(&mut w, snapshot),
            PlyMutation::SetFormat(set_format::SetFormat { format }) => {
                crate::artifacts::ply::schema::diff::write_bin_format(&mut w, *format);
            }
            PlyMutation::InsertComment(insert_comment::InsertComment { index, comment }) => {
                w.write_varint_u64(*index as u64);
                write_bin_str(&mut w, comment);
            }
            PlyMutation::RemoveComment(remove_comment::RemoveComment { index }) => w.write_varint_u64(*index as u64),
            PlyMutation::AddElement(add_element::AddElement { index, element }) => {
                w.write_varint_u64(*index as u64);
                write_bin_element(&mut w, element);
            }
            PlyMutation::RemoveElement(remove_element::RemoveElement { name }) => write_bin_str(&mut w, name),
            PlyMutation::InsertRow(insert_row::InsertRow { element_name, index, row }) => {
                write_bin_str(&mut w, element_name);
                w.write_varint_u64(*index as u64);
                write_bin_row(&mut w, row);
            }
            PlyMutation::RemoveRow(remove_row::RemoveRow { element_name, index }) => {
                write_bin_str(&mut w, element_name);
                w.write_varint_u64(*index as u64);
            }
            PlyMutation::SetRowProperty(set_row_property::SetRowProperty { element_name, row_index, property_name, value }) => {
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
            0 => Ok(PlyMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: read_bin_snapshot(&mut r).map_err(op_pack_err)? })),
            1 => Ok(PlyMutation::SetFormat(set_format::SetFormat { format: crate::artifacts::ply::schema::diff::read_bin_format(&mut r).map_err(op_pack_err)? })),
            2 => {
                let index = r.read_varint_u64().map_err(op_pack_err)? as usize;
                let comment = read_bin_str(&mut r).map_err(op_pack_err)?;
                Ok(PlyMutation::InsertComment(insert_comment::InsertComment { index, comment }))
            }
            3 => Ok(PlyMutation::RemoveComment(remove_comment::RemoveComment { index: r.read_varint_u64().map_err(op_pack_err)? as usize })),
            4 => {
                let index = r.read_varint_u64().map_err(op_pack_err)? as usize;
                let element = read_bin_element(&mut r).map_err(op_pack_err)?;
                Ok(PlyMutation::AddElement(add_element::AddElement { index, element }))
            }
            5 => Ok(PlyMutation::RemoveElement(remove_element::RemoveElement { name: read_bin_str(&mut r).map_err(op_pack_err)? })),
            6 => {
                let element_name = read_bin_str(&mut r).map_err(op_pack_err)?;
                let index = r.read_varint_u64().map_err(op_pack_err)? as usize;
                let row = read_bin_row(&mut r).map_err(op_pack_err)?;
                Ok(PlyMutation::InsertRow(insert_row::InsertRow { element_name, index, row }))
            }
            7 => {
                let element_name = read_bin_str(&mut r).map_err(op_pack_err)?;
                let index = r.read_varint_u64().map_err(op_pack_err)? as usize;
                Ok(PlyMutation::RemoveRow(remove_row::RemoveRow { element_name, index }))
            }
            8 => {
                let element_name = read_bin_str(&mut r).map_err(op_pack_err)?;
                let row_index = r.read_varint_u64().map_err(op_pack_err)? as usize;
                let property_name = read_bin_str(&mut r).map_err(op_pack_err)?;
                let value = read_bin_value(&mut r).map_err(op_pack_err)?;
                Ok(PlyMutation::SetRowProperty(set_row_property::SetRowProperty { element_name, row_index, property_name, value }))
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<PlyMutation> {
    use crate::artifacts::ply::schema::snapshot::{PlyProperty, PlyScalarType};
    let snapshot = demo_base_snapshot();
    vec![
        PlyMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: snapshot.clone() }),
        PlyMutation::SetFormat(set_format::SetFormat { format: PlyFormat::BinaryBigEndian }),
        PlyMutation::InsertComment(insert_comment::InsertComment { index: 0, comment: "new comment".into() }),
        PlyMutation::RemoveComment(remove_comment::RemoveComment { index: 0 }),
        PlyMutation::AddElement(add_element::AddElement {
            index: 1,
            element: PlyElement {
                name: "face".into(),
                count: 1,
                properties: vec![PlyProperty::List { name: "vertex_indices".into(), count_kind: PlyScalarType::UChar, value_kind: PlyScalarType::Int }],
                rows: vec![PlyRow { values: vec![PlyValue::List(vec![PlyValue::Int(0), PlyValue::Int(1), PlyValue::Int(2)])] }],
            },
        }),
        PlyMutation::RemoveElement(remove_element::RemoveElement { name: "vertex".into() }),
        PlyMutation::InsertRow(insert_row::InsertRow { element_name: "vertex".into(), index: 0, row: PlyRow { values: vec![PlyValue::Float(-2.5)] } }),
        PlyMutation::RemoveRow(remove_row::RemoveRow { element_name: "vertex".into(), index: 0 }),
        PlyMutation::SetRowProperty(set_row_property::SetRowProperty { element_name: "vertex".into(), row_index: 0, property_name: "x".into(), value: PlyValue::Float(42.0) }),
        PlyMutation::SetRowProperty(set_row_property::SetRowProperty { element_name: "face".into(), row_index: 0, property_name: "vertex_indices".into(), value: PlyValue::List(vec![PlyValue::Int(3), PlyValue::Int(4), PlyValue::Int(5)]) }),
    ]
}
//#endregion 🔖️DemoMutationCases

//#region 🧪️Tests
#[cfg(test)]
mod codec_tests {
    use super::*;

    /// 🧪️ F6/P2-FG3: `OpText`/`OpBinary` round-trip laws for the hand-rolled `PlyMutation`
    /// grammar — `OpBinary` is now a REAL binary frame, no longer text-as-bytes.
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
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

    //#region 🔖️KindsCoverageLaw
    /// 🏷️ `KINDS` must name exactly the enum's variants (kebab-case), one entry each — an
    /// exhaustive `match` so the compiler itself fails the moment a variant is added, renamed or
    /// removed without this list being updated alongside it. The manifest side of the same claim
    /// (`../../🧪️oracle/🔣️.json`'s `ply-1-0-any` catalog `kinds`) is checked by the
    /// mutate/inverse test case's own contract gate, which fails if the two lists ever diverge.
    #[semio_framework_async_macros::async_test]
    async fn kinds_cover_every_variant() {
        fn kind_of(mutation: &PlyMutation) -> &'static str {
            match mutation {
                PlyMutation::SetSnapshot(..) => "set-snapshot",
                PlyMutation::SetFormat(..) => "set-format",
                PlyMutation::InsertComment(..) => "insert-comment",
                PlyMutation::RemoveComment(..) => "remove-comment",
                PlyMutation::AddElement(..) => "add-element",
                PlyMutation::RemoveElement(..) => "remove-element",
                PlyMutation::InsertRow(..) => "insert-row",
                PlyMutation::RemoveRow(..) => "remove-row",
                PlyMutation::SetRowProperty(..) => "set-row-property",
            }
        }
        let mut exercised: Vec<&str> = demo_mutation_cases().iter().map(kind_of).collect();
        exercised.sort_unstable();
        exercised.dedup();
        let mut declared: Vec<&str> = KINDS.to_vec();
        declared.sort_unstable();
        assert_eq!(exercised, declared, "KINDS must name exactly the variants demo_mutation_cases() exercises");
        assert_eq!(KINDS.len(), 9, "ply-1-0-any declares 9 PlyMutation variants");
    }
    //#endregion 🔖️KindsCoverageLaw
}
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `📦️glue.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "📄set-snapshot/🧪️tests/lifts-the-second-vertex-and-appends-a-comment/🦀️component.rs"]
    mod tests_set_snapshot_lifts_the_second_vertex_and_appends_a_comment;
}
//#endregion 🧪️FixtureTests
