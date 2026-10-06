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
//! `../🔺️diff/🦀️.rs`'s module doc comment). `OpText`/`OpBinary` hand-rolled below,
//! reusing the diff file's `pub(crate)` grammar primitives (`hex_encode`/`enc_element`/
//! `split_top_level`/`encode_option`/...) rather than duplicating them a second time in this file.

use crate::schema::diff::{diff_add_element, diff_insert_row, diff_remove_element, diff_remove_row, diff_set_comments, diff_set_format, diff_set_row_property, diff_set_snapshot, PlyDiff};






















use crate::schema::snapshot::{PlyElement, PlyFormat, PlyRow, PlyValue};
use crate::PlySnapshot;
use protocol::Mutation;
use protocol::OpBinary;
use protocol::OpText;

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
#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = PlySnapshot, diff = PlyDiff, schema = "PlyMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum PlyMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    PatchSnapshot(patch_snapshot::PatchSnapshot),
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
pub const KINDS: &[&str] = &["set-snapshot", "patch-snapshot", "set-format", "insert-comment", "remove-comment", "add-element", "remove-element", "insert-row", "remove-row", "set-row-property"];
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
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
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
        PlyMutation::PatchSnapshot(patch) => return <patch_snapshot::PatchSnapshot as protocol::MutationKind<PlySnapshot, PlyMutation>>::diff(patch, base),
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
pub(crate) fn agg_inverse(this: &PlyMutation, base: &PlySnapshot) -> Result<Vec<PlyMutation>, semio_framework_value::ValueError> {
    Ok({
    match this {
        PlyMutation::SetSnapshot(_) => vec![PlyMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        PlyMutation::PatchSnapshot(patch) => return Ok(<patch_snapshot::PatchSnapshot as protocol::MutationKind<PlySnapshot, PlyMutation>>::inverse(patch, base)?),
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
            let at = base.elements.iter().find(|e| &e.name == element_name).map_or(*index, |e| (*index).min(e.rows.len()));
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

    })
}
//#endregion 🔖️MutationTrait

//#region OpCodecs








//#region 🔖️RealBinaryOpFrame






//#endregion 🔖️RealBinaryOpFrame
//#endregion OpCodecs

//#region 🔖️DemoMutationCases
/// ✅️ Every `PlyMutation` variant built off a small `base()` snapshot — the single case list
/// `op_text_binary_roundtrip_law` (this file) AND `ops_grammar_conformance_law`/
/// `protocol_walk_law` (`⚙️engine/🦀️.rs`) all exercise. Covers `SetSnapshot`'s whole
/// nested snapshot, `AddElement`'s bare `PlyElement` payload (itself containing `PlyProperty`),
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
        PlyMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch: semio_s_artifact_stdio_contract::editing::SnapshotPatch::Set { path: "/schema".into(), value: semio_framework_value::DslValue::String("stdio.patch-snapshot.witness".into()) } }),
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
#[path = "🧪️tests/🔬️codec/🦀️.rs"]
mod codec_tests;
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
