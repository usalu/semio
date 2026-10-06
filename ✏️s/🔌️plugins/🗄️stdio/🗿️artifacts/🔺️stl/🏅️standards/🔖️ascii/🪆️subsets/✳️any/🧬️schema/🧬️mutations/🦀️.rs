//! 🧬️ StlMutation — document mutation dispatch. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: real vocabulary beyond
//! the universal `{NoMutation, SetSnapshot}` stub — `SetSolidName` plus
//! `InsertTriangle`/`RemoveTriangle`/`SetTriangleNormal`/`SetTriangleVertices` for the index-keyed
//! `triangles` collection. Every variant's `diff()` is handcrafted (constructs `StlDiff` directly
//! via the `schema::diff` builders) — apply-and-capture is never used.
//!
//! 🧪️ F6 (OpText/OpBinary + DiffCodec wave): **HAND-ROLL path** — every variant's payload closure
//! (incl. `SetSnapshot`'s whole `StlSnapshot`) has zero data-carrying enums (§3a of
//! `f6-recon-report.md`'s decision rule), and `#[derive(dsl::DslOps)]` DID compile cleanly on a
//! first attempt, exactly like `GifMutation`'s pilot. It was reverted for the SAME reason
//! `StlDiff`'s derive was (see `🔺️diff::component`'s top doc comment and `StlTriangle`'s doc
//! comment in `📸️snapshot::component`): a real, reproduced `dsl`-grammar bug where nested
//! `Shape::Tuple` levels (`vertices: [[f64; 3]; 3]`, reachable via `SetSnapshot`, `InsertTriangle`,
//! `SetTriangleVertices`) print flat and cannot be re-parsed. `OpText`/`OpBinary` below are
//! hand-rolled instead, reusing `🔺️diff::component`'s `pub(crate)` grammar primitives
//! (`enc_vec3`/`enc_vertices`/`enc_triangle`/`hex_encode_str`/`split_top_level`/`strip_brackets`) —
//! same intra-artifact reuse pattern `svg`'s `SvgMutation` uses over `SvgDiff`'s primitives.
//!
//! 🧬️ Ticket 26/08/29/S-END-TO-END (mutation-leaf migration): `protocol::Mutation<P>` now requires
//! `DESCRIPTORS`/`descriptor()`, which only `#[derive(dsl::Mutations)]` synthesizes. Every variant
//! moved to its own mutation-leaf folder beside this file (`../🖼️tiff/…/🧱️baseline/🧬️schema/🧬️mutations/`
//! is the reference shape). `NoMutation` was dropped: the derive requires every variant to wrap
//! exactly one leaf payload (a unit variant wraps none), and `"no"` is not an `APPROVED_VERB`
//! (`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs`) a derived `SEMANTICS.verb`
//! could hold anyway.

use crate::schema::diff::{self, StlDiff};
use crate::schema::snapshot::StlTriangle;
use crate::StlSnapshot;
use protocol::Mutation;
use protocol::{OpBinary, OpText};

//#region 🔖️Mutations
#[path = "➕insert-triangle/🦀️.rs"]
pub mod insert_triangle;
#[path = "➖remove-triangle/🦀️.rs"]
pub mod remove_triangle;
/// 📐️ Typed content mutation for `stdio.stl`.
//#region 🔖️Leaves
#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🏷️set-solid-name/🦀️.rs"]
pub mod set_solid_name;
#[path = "🧭set-triangle-normal/🦀️.rs"]
pub mod set_triangle_normal;
#[path = "📐set-triangle-vertices/🦀️.rs"]
pub mod set_triangle_vertices;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = StlSnapshot, diff = StlDiff, schema = "StlMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum StlMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    PatchSnapshot(patch_snapshot::PatchSnapshot),
    SetSolidName(set_solid_name::SetSolidName),
    InsertTriangle(insert_triangle::InsertTriangle),
    RemoveTriangle(remove_triangle::RemoveTriangle),
    SetTriangleNormal(set_triangle_normal::SetTriangleNormal),
    SetTriangleVertices(set_triangle_vertices::SetTriangleVertices),
}
//#endregion 🔖️Mutations

//#region 🔖️Kinds
/// 🧾️ Kebab-case spelling of every `StlMutation` variant, in declaration order — the vocabulary
/// `../../🔮️oracles/🔣️.json`'s `stl-ascii-any` catalog is measured against. Kept honest by
/// `kinds_match_enum_and_catalog` below (the framework never parses Rust to learn this list).
pub const KINDS: &[&str] = &["set-snapshot", "patch-snapshot", "set-solid-name", "insert-triangle", "remove-triangle", "set-triangle-normal", "set-triangle-vertices"];
//#endregion 🔖️Kinds

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`, returning a typed error outcome without changing the
/// snapshot when an index target is missing or out of range.
pub fn apply_stl_mutation(snapshot: &mut StlSnapshot, mutation: &StlMutation) -> protocol::MutationOutcome<StlDiff> {
    let outcome = <StlMutation as Mutation<StlSnapshot>>::diff(mutation, snapshot);
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
/// ▶️ Lifted verbatim from the former hand-rolled `impl Mutation<StlSnapshot> for StlMutation`;
/// every leaf's `MutationKind::diff` reconstructs its `StlMutation` and delegates here, so this
/// stays the single place the forward semantics are written.
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &StlMutation, base: &StlSnapshot) -> protocol::MutationOutcome<StlDiff> {
    protocol::MutationOutcome::new(match this {
        StlMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff::diff_set_snapshot(base, snapshot),
        StlMutation::PatchSnapshot(patch) => return <patch_snapshot::PatchSnapshot as protocol::MutationKind<StlSnapshot, StlMutation>>::diff(patch, base),
        StlMutation::SetSolidName(set_solid_name::SetSolidName { name }) => diff::diff_set_solid_name(name),
        StlMutation::InsertTriangle(insert_triangle::InsertTriangle { index, triangle }) => diff::diff_insert_triangle(*index, *triangle),
        StlMutation::RemoveTriangle(remove_triangle::RemoveTriangle { index }) => diff::diff_remove_triangle(*index),
        StlMutation::SetTriangleNormal(set_triangle_normal::SetTriangleNormal { index, normal }) => diff::diff_set_triangle_normal(*index, *normal),
        StlMutation::SetTriangleVertices(set_triangle_vertices::SetTriangleVertices { index, vertices }) => diff::diff_set_triangle_vertices(*index, *vertices),
    })
}

/// ↩️ Lifted verbatim from the former hand-rolled `impl Mutation<StlSnapshot> for StlMutation`.
/// Index-targeted variants look the prior value up in `base`; a stale/out-of-range index inverts
/// to the EMPTY inverse (`Vec::new()`) rather than a `NoMutation` stand-in, now that the derive
/// forbids a unit variant.
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &StlMutation, base: &StlSnapshot) -> Result<Vec<StlMutation>, semio_framework_value::ValueError> {
    Ok({
    match this {
        StlMutation::SetSnapshot(_) => vec![StlMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        StlMutation::PatchSnapshot(patch) => <patch_snapshot::PatchSnapshot as protocol::MutationKind<StlSnapshot, StlMutation>>::inverse(patch, base)?,
        StlMutation::SetSolidName(_) => vec![StlMutation::SetSolidName(set_solid_name::SetSolidName { name: base.solid_name.clone() })],
        StlMutation::InsertTriangle(insert_triangle::InsertTriangle { index, .. }) => {
            vec![StlMutation::RemoveTriangle(remove_triangle::RemoveTriangle { index: (*index).min(base.triangles.len()) })]
        }
        StlMutation::RemoveTriangle(remove_triangle::RemoveTriangle { index }) => match base.triangles.get(*index) {
            Some(t) => vec![StlMutation::InsertTriangle(insert_triangle::InsertTriangle { index: *index, triangle: *t })],
            None => Vec::new(),
        },
        StlMutation::SetTriangleNormal(set_triangle_normal::SetTriangleNormal { index, .. }) => match base.triangles.get(*index) {
            Some(t) => vec![StlMutation::SetTriangleNormal(set_triangle_normal::SetTriangleNormal { index: *index, normal: t.normal })],
            None => Vec::new(),
        },
        StlMutation::SetTriangleVertices(set_triangle_vertices::SetTriangleVertices { index, .. }) => match base.triangles.get(*index) {
            Some(t) => vec![StlMutation::SetTriangleVertices(set_triangle_vertices::SetTriangleVertices { index: *index, vertices: t.vertices })],
            None => Vec::new(),
        },
    }

    })
}
//#endregion 🔖️MutationTrait

//#region OpCodecs








//#region 🔖️OpBinaryCodec






//#endregion 🔖️OpBinaryCodec
//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🎯 FG1: one representative case per `StlMutation` variant, incl. the two struct-valued payloads
/// (`SetSnapshot`, `InsertTriangle`) and the fixed-size-array payloads (`SetTriangleNormal`,
/// `SetTriangleVertices`) that carry the doubly-nested `[[f64; 3]; 3]` — shared by
/// `op_text_binary_roundtrip_law` below AND `⚙️engine::conformance_laws`'s `ops_grammar_
/// conformance_law`/`protocol_walk_law` (same reuse pattern `binary`'s own `demo_mutation_cases`
/// establishes).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<StlMutation> {
    let base = StlSnapshot { schema: crate::STDIO_STL_DOCUMENT_SCHEMA.into(), solid_name: "mesh".into(), triangles: vec![StlTriangle { normal: [0.0, 0.0, 1.0], vertices: [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]] }] };
    vec![
        StlMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: StlSnapshot { solid_name: "renamed".into(), ..base } }),
        StlMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch: semio_s_artifact_stdio_contract::editing::SnapshotPatch::Set { path: "/solidName".into(), value: semio_framework_value::DslValue::String("patched".into()) } }),
        StlMutation::SetSolidName(set_solid_name::SetSolidName { name: "renamed".into() }),
        StlMutation::InsertTriangle(insert_triangle::InsertTriangle { index: 1, triangle: StlTriangle { normal: [1.0, 0.0, 0.0], vertices: [[99.0, 0.0, 0.0], [100.0, 0.0, 0.0], [99.0, 1.0, 0.0]] } }),
        StlMutation::RemoveTriangle(remove_triangle::RemoveTriangle { index: 1 }),
        StlMutation::SetTriangleNormal(set_triangle_normal::SetTriangleNormal { index: 0, normal: [1.0, 0.0, 0.0] }),
        StlMutation::SetTriangleVertices(set_triangle_vertices::SetTriangleVertices { index: 0, vertices: [[9.0, 9.0, 9.0], [8.0, 8.0, 8.0], [7.0, 7.0, 7.0]] }),
    ]
}
//#endregion 🔖️DemoCases

//#region Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion Tests

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📸️set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `🦀️.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📸️set-snapshot/🧪️tests/✏️renames/🦀️.rs"]
mod set_snapshot_renames_the_solid_and_closes_the_wedge_with_a_third_facet;
//#endregion 🧪️FixtureCases
