//! 🧬️ PlyMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `PlyDiff` directly — apply-and-capture is banned) and `inverse()` is
//! handcrafted per variant, key/index-aware.
//!
//! 🪆️ Migrated to `#[derive(dsl::Mutations)]` over one mutation-leaf module per variant
//! (mutation-leaf migration recipe, mirroring the stdio.tiff baseline subset's own
//! `🧬️schema/🧬️mutations/🦀️.rs`) to satisfy `protocol::Mutation<P>`'s new
//! `DESCRIPTORS`/`descriptor()` requirement (E0046). `NoMutation` was dropped: the derive requires
//! every variant to wrap exactly one leaf payload, a wrapped variant cannot be `#[default]`, and
//! `"no"` is not an `APPROVED_VERBS` entry. each leaf's own `MutationKind` impl owns its `diff` and concrete `inverse`.
//!
//! 🧪️ F6 CONFIRMED (ticket `f6-recon-report.md` §9 STEP 1b, real `cargo check`, not guessed):
//! `#[derive(dsl::DslOps)]` on `PlyMutation` fails —
//! `error[E0277]: the trait bound `PlyValue: DslField` is not satisfied` at
//! `SetRowProperty { ..., value: PlyValue }` (this file), plus (same run) `PlySnapshot`/
//! `PlyElement`/`PlyRow` are ALL also `DslField`-unsatisfied transitively (`AddElement`,
//! `InsertRow` respectively) — every one of those ultimately bottoms out at
//! `PlyProperty`/`PlyValue`, the same two data-carrying enums that block the Diff side (see
//! `../🔺️diff/🦀️.rs`'s module doc comment). `OpText`/`OpBinary` hand-rolled below,
//! reusing the diff file's `pub(crate)` grammar primitives (`hex_encode`/`enc_element`/
//! `split_top_level`/`encode_option`/...) rather than duplicating them a second time in this file.

use crate::schema::diff::{diff_add_element, diff_insert_comment, diff_insert_row, diff_remove_comment, diff_remove_element, diff_remove_row, diff_set_format, diff_set_row_property, PlyDiff};






















use crate::schema::snapshot::{PlyElement, PlyFormat, PlyRow, PlyValue};
use crate::PlySnapshot;
use protocol::Mutation;



//#region 🔖️Mutations
#[path = "🧱add-element/🦀️.rs"]
pub mod add_element;
#[path = "💬insert-comment/🦀️.rs"]
pub mod insert_comment;
#[path = "📥insert-row/🦀️.rs"]
pub mod insert_row;
#[path = "🗑️remove-comment/🦀️.rs"]
pub mod remove_comment;
#[path = "🚮remove-element/🦀️.rs"]
pub mod remove_element;
#[path = "📤remove-row/🦀️.rs"]
pub mod remove_row;
#[path = "🎚️set-format/🦀️.rs"]
pub mod set_format;
#[path = "🏷️set-row-property/🦀️.rs"]
pub mod set_row_property;
/// 📐️ Typed content mutation for `stdio.ply`.
//#region 🔖️Leaves
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = PlySnapshot, diff = PlyDiff, schema = "PlyMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum PlyMutation {
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
/// `ply-1-0-any` mutation catalog (`../../🔣️oracle.json`) declares and the exhaustive
/// mutate/inverse test case measures itself against. `kinds_cover_every_variant` below is what keeps
/// this list honest against the enum it names, since the framework never parses Rust.
pub const KINDS: &[&str] = &["set-format", "insert-comment", "remove-comment", "add-element", "remove-element", "insert-row", "remove-row", "set-row-property"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: the diff is the single semantics source
/// (`let d = mutation.diff(&*snapshot); *snapshot = d.apply(snapshot); d`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_ply_mutation(snapshot: &mut PlySnapshot, mutation: &PlyMutation) -> protocol::MutationOutcome<PlyDiff> {
    let outcome = <PlyMutation as Mutation<PlySnapshot>>::diff(mutation, snapshot);
    match protocol::apply_diff(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply


//#endregion 🔖️MutationTrait

//#region 🔖️Net
/// 🧮️ The leaves that carry `base` to exactly `next`: the format if it moved, the comments (a changed comment is removed and
/// inserted anew, surplus comments removed last first, missing ones inserted), then every element in place by position (an element
/// of another name or declaration is removed and added anew; the same element re-sets differing cells, removes surplus rows last
/// first and inserts missing ones). An element whose `count` moved without its rows is left unaddressed.
pub fn net_mutations(base: &PlySnapshot, next: &PlySnapshot) -> Vec<PlyMutation> {
    let mut leaves = Vec::new();
    if base.format != next.format {
        leaves.push(PlyMutation::SetFormat(set_format::SetFormat { format: next.format }));
    }
    let comments_paired = base.comments.len().min(next.comments.len());
    for (index, (before, after)) in base.comments.iter().zip(&next.comments).enumerate().filter(|(_, (before, after))| before != after) {
        let _ = before;
        leaves.push(PlyMutation::RemoveComment(remove_comment::RemoveComment { index }));
        leaves.push(PlyMutation::InsertComment(insert_comment::InsertComment { index, comment: after.clone() }));
    }
    leaves.extend((comments_paired..base.comments.len()).rev().map(|index| PlyMutation::RemoveComment(remove_comment::RemoveComment { index })));
    leaves.extend(next.comments.iter().enumerate().skip(comments_paired).map(|(index, comment)| PlyMutation::InsertComment(insert_comment::InsertComment { index, comment: comment.clone() })));
    let elements_paired = base.elements.len().min(next.elements.len());
    for (index, (before, after)) in base.elements.iter().zip(&next.elements).enumerate().filter(|(_, (before, after))| before != after) {
        if before.name != after.name || before.properties != after.properties {
            leaves.push(PlyMutation::RemoveElement(remove_element::RemoveElement { name: before.name.clone() }));
            leaves.push(PlyMutation::AddElement(add_element::AddElement { index, element: after.clone() }));
            continue;
        }
        let rows_paired = before.rows.len().min(after.rows.len());
        for (row_index, (old, new)) in before.rows.iter().zip(&after.rows).enumerate().filter(|(_, (old, new))| old != new) {
            for (cell, property) in before.properties.iter().enumerate() {
                if let (Some(old_value), Some(new_value)) = (old.values.get(cell), new.values.get(cell)) {
                    if old_value != new_value {
                        leaves.push(PlyMutation::SetRowProperty(set_row_property::SetRowProperty { element_name: before.name.clone(), row_index, property_name: property.name().to_string(), value: new_value.clone() }));
                    }
                }
            }
        }
        leaves.extend((rows_paired..before.rows.len()).rev().map(|row_index| PlyMutation::RemoveRow(remove_row::RemoveRow { element_name: before.name.clone(), index: row_index })));
        leaves.extend(after.rows.iter().enumerate().skip(rows_paired).map(|(row_index, row)| PlyMutation::InsertRow(insert_row::InsertRow { element_name: before.name.clone(), index: row_index, row: row.clone() })));
    }
    leaves.extend(base.elements[elements_paired..].iter().rev().map(|element| PlyMutation::RemoveElement(remove_element::RemoveElement { name: element.name.clone() })));
    leaves.extend(next.elements.iter().enumerate().skip(elements_paired).map(|(index, element)| PlyMutation::AddElement(add_element::AddElement { index, element: element.clone() })));
    leaves
}
//#endregion 🔖️Net

//#region OpCodecs








//#region 🔖️RealBinaryOpFrame






//#endregion 🔖️RealBinaryOpFrame
//#endregion OpCodecs

//#region 🔖️DemoMutationCases
/// ✅️ Every `PlyMutation` variant built off a small `base()` snapshot — the single case list
/// `op_text_binary_roundtrip_law` (this file) AND `ops_grammar_conformance_law`/
/// `protocol_walk_law` (`⚙️engine/🦀️.rs`) all exercise. Covers `AddElement`'s bare `PlyElement` payload (itself containing `PlyProperty`),
/// `InsertRow`'s `PlyRow` payload, and `SetRowProperty`'s bare `PlyValue` payload (incl. the
/// recursive `List` variant).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn demo_base_snapshot() -> PlySnapshot {
    use crate::schema::snapshot::{PlyProperty, PlyScalarType};
    PlySnapshot {
        schema: crate::STDIO_PLY_DOCUMENT_SCHEMA.into(),
        format: PlyFormat::Ascii,
        comments: vec!["hi".into()],
        elements: vec![PlyElement { name: "vertex".into(), count: 1, properties: vec![PlyProperty::Scalar { name: "x".into(), kind: PlyScalarType::Float }], rows: vec![PlyRow { values: vec![PlyValue::Float(1.5)] }] }],
    }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<PlyMutation> {
    use crate::schema::snapshot::{PlyProperty, PlyScalarType};
    let snapshot = demo_base_snapshot();
    vec![
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
#[path = "🧪️tests/🔬️codec/🦀️.rs"]
mod codec_tests;
//#endregion 🧪️Tests


#[cfg(test)]
use protocol::{OpBinary,OpText};
