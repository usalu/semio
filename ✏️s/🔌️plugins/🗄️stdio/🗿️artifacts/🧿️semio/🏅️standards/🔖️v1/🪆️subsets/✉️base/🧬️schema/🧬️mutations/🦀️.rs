//! 🧬️ SemioMutation — the envelope union's own mutation vocabulary. The 18 wrapper variants each
//! carry that subset's OWN, already-real, already-hand-written `SemioXMutation` enum unchanged
//! (`SemioBrepMutation`, `SemioAudioMutation`, …) — every `diff()`/`inverse()` for a wrapped
//! variant delegates straight through to that subset's own `Mutation` impl, so this module never
//! re-derives any of the 18 subsets' own per-field mutation logic; its OWN job is purely the
//! envelope-level routing (does the wrapped mutation's kind match the base snapshot's current
//! kind, and if so thread it through).
//!
//! 🪆️ Mutation-leaf migration: `NoMutation` is dropped (the derive requires every variant to wrap
//! exactly one leaf payload, and `no` is not an approved semantic verb). `#[derive(dsl::Mutations)]`
//! requires each variant's payload to directly implement `protocol::MutationKind<SemioSnapshot,
//! SemioMutation>` + `protocol::MutationLeaf`, and `MutationLeaf`'s provenance is validated against
//! the FILE the `#[derive(dsl::MutationLeaf)]` macro expands in — a subset's own `SemioXMutation` is
//! defined in ITS OWN aggregate file, not under `✉️base/🧬️schema/🧬️mutations/`, so it cannot serve as
//! the direct leaf payload itself. Each of the 18 wrapper variants therefore wraps a THIN leaf struct
//! (`<emoji><kind>/🦀️.rs`, e.g. `🧱brep/🦀️.rs`'s `pub struct Brep { pub(crate) mutation:
//! SemioBrepMutation }`) whose own `MutationKind` impl still does nothing but delegate straight
//! through to the wrapped subset's already-real `Mutation` impl via `agg_diff`/`agg_inverse` — the
//! envelope still routes, it still never redefines a wrapped subset's own vocabulary.
//!
//! Some of the 18 wrapped subsets are migrated onto this same leaf pattern (`model`, `document`,
//! `cad`, `image`, `video`, `audio`, `animation`, `presentation`, `value`, `table`, `graph`,
//! `object`, `kit` — no `NoMutation`/`Default` of their own any more, so their harmless
//! representative case below is `SetSnapshot` with a default snapshot); `flow` is not yet migrated
//! (still carries its own `NoMutation`/`Default`), so its representative case uses `::default()`
//! until its own lane migrates it.

use {semio_s_artifact_stdio_semio_animation::schema::mutations::SemioAnimationMutation, semio_s_artifact_stdio_semio_animation::schema::snapshot::SemioAnimationSnapshot};
#[cfg(test)]
use semio_s_artifact_stdio_semio_audio_mutations::schema::mutations::set_sample_rate;
use {semio_s_artifact_stdio_semio_audio_mutations::schema::mutations::SemioAudioMutation, semio_s_artifact_stdio_semio_audio_model::schema::snapshot::SemioAudioSnapshot};
use crate::schema::diff::SemioDiff;
use crate::schema::snapshot::{SemioSnapshot, SemioSubsetSnapshot};
use {semio_s_artifact_stdio_semio_brep::schema::mutations::SemioBrepMutation, semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot};
use {semio_s_artifact_stdio_semio_cad::schema::mutations::SemioCadMutation, semio_s_artifact_stdio_semio_cad::schema::snapshot::SemioCadSnapshot};
use {semio_s_artifact_stdio_semio_document::schema::mutations::SemioDocumentMutation, semio_s_artifact_stdio_semio_document::schema::snapshot::SemioDocumentSnapshot};
use {semio_s_artifact_stdio_semio_drawing::schema::mutations::SemioDrawingMutation, semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot};
use {semio_s_artifact_stdio_semio_flow::schema::mutations::SemioFlowMutation, semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot};
use {semio_s_artifact_stdio_semio_graph::schema::mutations::SemioGraphMutation, semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot};
use {semio_s_artifact_stdio_semio_image::schema::mutations::SemioImageMutation, semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot};
use {semio_s_artifact_stdio_semio_kit::schema::mutations::SemioKitMutation, semio_s_artifact_stdio_semio_kit::schema::snapshot::SemioKitSnapshot};
use {semio_s_artifact_stdio_semio_mesh::schema::mutations::SemioMeshMutation, semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot};
use {semio_s_artifact_stdio_semio_model::schema::mutations::SemioModelMutation, semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelSnapshot};
use {semio_s_artifact_stdio_semio_object::schema::mutations::SemioObjectMutation, semio_s_artifact_stdio_semio_object::schema::snapshot::SemioObjectSnapshot};
use {semio_s_artifact_stdio_semio_presentation::schema::mutations::SemioPresentationMutation, semio_s_artifact_stdio_semio_presentation::schema::snapshot::SemioPresentationSnapshot};
use {semio_s_artifact_stdio_semio_table_mutations::schema::mutations::SemioTableMutation, semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot};
use {semio_s_artifact_stdio_semio_text_mutations::schema::mutations::SemioTextMutation, semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot};
use {semio_s_artifact_stdio_semio_value_mutations::schema::mutations::SemioValueMutation, semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot};
use {semio_s_artifact_stdio_semio_video_mutations::schema::mutations::SemioVideoMutation, semio_s_artifact_stdio_semio_video_model::schema::snapshot::SemioVideoSnapshot};
use protocol::Mutation;

//#region 🔖️Mutation
#[path = "🎞️apply-animation/🦀️.rs"]
pub mod apply_animation;
#[path = "🔊apply-audio/🦀️.rs"]
pub mod apply_audio;
#[path = "🧱apply-brep/🦀️.rs"]
pub mod apply_brep;
#[path = "📐apply-cad/🦀️.rs"]
pub mod apply_cad;
#[path = "📃apply-document/🦀️.rs"]
pub mod apply_document;
#[path = "🖊️apply-drawing/🦀️.rs"]
pub mod apply_drawing;
#[path = "🔀apply-flow/🦀️.rs"]
pub mod apply_flow;
#[path = "🌐apply-graph/🦀️.rs"]
pub mod apply_graph;
#[path = "🖼️apply-image/🦀️.rs"]
pub mod apply_image;
#[path = "🧰apply-kit/🦀️.rs"]
pub mod apply_kit;
#[path = "🕸️apply-mesh/🦀️.rs"]
pub mod apply_mesh;
#[path = "🏛️apply-model/🦀️.rs"]
pub mod apply_model;
#[path = "📦apply-object/🦀️.rs"]
pub mod apply_object;
#[path = "📽️apply-presentation/🦀️.rs"]
pub mod apply_presentation;
#[path = "🗂️apply-table/🦀️.rs"]
pub mod apply_table;
#[path = "🔤apply-text/🦀️.rs"]
pub mod apply_text;
#[path = "🔢apply-value/🦀️.rs"]
pub mod apply_value;
#[path = "🎬apply-video/🦀️.rs"]
pub mod apply_video;
/// 🔧️ Adjacently tagged (`tag = "mutation"`, `content = "payload"`), NOT internally tagged like
/// every one of the 18 wrapped subset enums' own `#[value(tag = "mutation", ...)]` — an
/// internally-tagged wrapper here would collide key-for-key with a wrapped variant's OWN
/// `"mutation"` discriminator field when serde flattens a newtype variant's fields into the
/// outer value (real bug caught by this file's own `op_text_binary_roundtrip_law` test: printed
/// JSON came out `{"mutation":"audio","mutation":"setSampleRate",...}`, two keys with the same
/// name, which `serde_json` then refuses to parse back). `content = "payload"` nests the wrapped
/// value under its own key instead of flattening it, sidestepping the collision entirely.
///
/// 🏷️ The 18 wrapper variants are named `Apply<Subset>` (`ApplyBrep`, `ApplyMesh`, …), not the bare
/// subset noun: `#[derive(dsl::Mutations)]`'s `MutationLeaf` provenance check asserts
/// `SEMANTICS.kind == to_kebab(VariantIdent)` and `mutation_leaf_descriptor_kebab` REQUIRES at
/// least one hyphen, so a single-word kind like `brep` is rejected outright. `apply` is the
/// approved verb every leaf already carried in its `SEMANTICS.verb`, so the variant becomes
/// `ApplyBrep` / kind `apply-brep` — the same treatment `stdio.binary`'s `Splice` →
/// `ReplaceByteRange` just got. That one spelling is the vocabulary everywhere: the leaf directory,
/// the catalog, the production `DESCRIPTORS`, and the JSON wire tag `applyBrep` the published
/// `🔣️.json` declares.
//#region 🔖️Leaves
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = SemioSnapshot, diff = SemioDiff, schema = "SemioMutation")]
#[value(tag = "mutation", content = "payload", rename_all = "camelCase")]
pub enum SemioMutation {
    ApplyBrep(apply_brep::ApplyBrep),
    ApplyMesh(apply_mesh::ApplyMesh),
    ApplyModel(apply_model::ApplyModel),
    ApplyValue(apply_value::ApplyValue),
    ApplyDocument(apply_document::ApplyDocument),
    ApplyCad(apply_cad::ApplyCad),
    ApplyDrawing(apply_drawing::ApplyDrawing),
    ApplyImage(apply_image::ApplyImage),
    ApplyVideo(apply_video::ApplyVideo),
    ApplyAudio(apply_audio::ApplyAudio),
    ApplyAnimation(apply_animation::ApplyAnimation),
    ApplyPresentation(apply_presentation::ApplyPresentation),
    ApplyFlow(apply_flow::ApplyFlow),
    ApplyText(apply_text::ApplyText),
    ApplyTable(apply_table::ApplyTable),
    ApplyGraph(apply_graph::ApplyGraph),
    ApplyObject(apply_object::ApplyObject),
    ApplyKit(apply_kit::ApplyKit),
}

/// 🏷️ Kebab-case spelling of every `SemioMutation` variant, in declaration order — the leaf kinds the
/// `semio-v1-base` mutation catalog (`../../🔮️oracles/🔣️.json`) declares and `✉️mutate-semio-base`'s
/// exhaustive test case measures itself against. Each wrapper is `apply-<arm>`, the arm being the
/// subset tag the envelope routes on; `kinds_match_the_enum_and_the_catalog` pins the list with a
/// WILDCARD-FREE match, so a nineteenth subset cannot be added without extending both it and `KINDS`.
pub const KINDS: &[&str] = &["apply-brep", "apply-mesh", "apply-model", "apply-value", "apply-document", "apply-cad", "apply-drawing", "apply-image", "apply-video", "apply-audio", "apply-animation", "apply-presentation", "apply-flow", "apply-text", "apply-table", "apply-graph", "apply-object", "apply-kit"];

/// 🧮️ Pure diff face of [`Mutation::diff`], named only in this subset's own reachable types (`protocol` is a private
/// `extern crate` alias, so an owner-root test adapter cannot bring the `Mutation` trait into scope).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_semio_mutation(mutation: &SemioMutation, base: &SemioSnapshot) -> protocol::MutationOutcome<SemioDiff> {
    <SemioMutation as Mutation<SemioSnapshot>>::diff(mutation, base)
}


/// ↩️ Free-function face of [`Mutation::inverse`], named only in this subset's own reachable types.
/// `protocol` is a private `extern crate semio_framework_os_kernel as protocol;` alias that nothing
/// re-exports, so an owner-root test adapter compiled as an external crate cannot bring the
/// `Mutation` trait into scope to call the method form — the structural gap wave 7 recorded for
/// `kit`/`object`/`text`/`table`, and the same thin-wrapper remedy `kit` adopted. Used by
/// `✉️mutate-semio-base`'s `inverse-*` scenarios.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_semio_mutation(mutation: &SemioMutation, base: &SemioSnapshot) -> Result<Vec<SemioMutation>, semio_framework_value::ValueError> {
    Ok({
    <SemioMutation as Mutation<SemioSnapshot>>::inverse(mutation, base)?

    })
}

/// 🏷️ Free-function face of the envelope's own subset discriminator — the tag `KINDS` spells for
/// each wrapper variant and the only observable an owner-root test can read back out of a routed
/// envelope without naming any of the eighteen arms' snapshot types.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_subset_tag(snapshot: &SemioSnapshot) -> &'static str {
    crate::schema::snapshot::subset_tag(&snapshot.subset)
}

//#endregion 🔖️Mutation

//#endregion 🔖️MutationTrait

//#region OpCodecs

//#endregion OpCodecs

//#region 🔖️Demo
/// 🌱 All 19 top-level [`SemioMutation`] tags (`SetSnapshot` and each of the 18 wrapped-kind
/// representative variants) — full dispatch-table coverage for this facet's grammar/protocol
/// conformance-law tests. Single source of truth shared with `🎹️composer/🦀️.rs`'s
/// `ops_grammar_conformance_law`/`protocol_walk_law`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<SemioMutation> {
    vec![
        SemioMutation::ApplyBrep(apply_brep::ApplyBrep { mutation: SemioBrepMutation::DeleteVertex(semio_s_artifact_stdio_semio_brep::schema::mutations::delete_vertex::DeleteVertex { id: "v-absent".into() }) }),
        SemioMutation::ApplyMesh(apply_mesh::ApplyMesh { mutation: SemioMeshMutation::DeleteMesh(semio_s_artifact_stdio_semio_mesh::schema::mutations::delete_mesh::DeleteMesh { id: "mesh-absent".into() }) }),
        SemioMutation::ApplyDrawing(apply_drawing::ApplyDrawing {
            mutation: SemioDrawingMutation::DragNodes(semio_s_artifact_stdio_semio_drawing::schema::mutations::drag_nodes::DragNodes { ats: Vec::new(), offset: semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2::default() }),
        }),
        SemioMutation::ApplyAnimation(apply_animation::ApplyAnimation { mutation: SemioAnimationMutation::RemoveTimeline(semio_s_artifact_stdio_semio_animation::schema::mutations::remove_timeline::RemoveTimeline { index: 99 }) }),
        SemioMutation::ApplyPresentation(apply_presentation::ApplyPresentation { mutation: SemioPresentationMutation::RemoveSlide(semio_s_artifact_stdio_semio_presentation::schema::mutations::remove_slide::RemoveSlide { index: 99 }) }),
        SemioMutation::ApplyText(apply_text::ApplyText { mutation: SemioTextMutation::RemoveRun(semio_s_artifact_stdio_semio_text_mutations::schema::mutations::remove_run::RemoveRun { index: 99 }) }),
        SemioMutation::ApplyTable(apply_table::ApplyTable { mutation: SemioTableMutation::RemoveRow(semio_s_artifact_stdio_semio_table_mutations::schema::mutations::remove_row::RemoveRow { index: 99 }) }),
        SemioMutation::ApplyGraph(apply_graph::ApplyGraph {
            mutation: SemioGraphMutation::DeleteNode(semio_s_artifact_stdio_semio_graph::schema::mutations::delete_node::DeleteNode { id: semio_s_artifact_stdio_semio_graph::schema::snapshot::GraphNodeId::new("absent") }),
        }),
        SemioMutation::ApplyObject(apply_object::ApplyObject { mutation: SemioObjectMutation::DeleteBrep(semio_s_artifact_stdio_semio_object::schema::mutations::delete_brep::DeleteBrep {}) }),
        SemioMutation::ApplyKit(apply_kit::ApplyKit { mutation: SemioKitMutation::RemoveType(semio_s_artifact_stdio_semio_kit::schema::mutations::remove_type::RemoveType { id: "absent".into() }) }),
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests

//#region 🧪️FixtureCases
#[cfg(test)]
#[path = "🖼️apply-image/🧪️tests/🚫️refuses/🦀️.rs"]
mod apply_image_refuses_a_value_envelope;
#[cfg(test)]
#[path = "🌐apply-graph/🧪️tests/↩️inverts/🦀️.rs"]
mod apply_graph_inverts;
#[cfg(test)]
#[path = "🎞️apply-animation/🧪️tests/↩️inverts/🦀️.rs"]
mod apply_animation_inverts;
#[cfg(test)]
#[path = "🎬apply-video/🧪️tests/↩️inverts/🦀️.rs"]
mod apply_video_inverts;
#[cfg(test)]
#[path = "🏛️apply-model/🧪️tests/↩️inverts/🦀️.rs"]
mod apply_model_inverts;
#[cfg(test)]
#[path = "📃apply-document/🧪️tests/↩️inverts/🦀️.rs"]
mod apply_document_inverts;
#[cfg(test)]
#[path = "📐apply-cad/🧪️tests/↩️inverts/🦀️.rs"]
mod apply_cad_inverts;
#[cfg(test)]
#[path = "📦apply-object/🧪️tests/↩️inverts/🦀️.rs"]
mod apply_object_inverts;
#[cfg(test)]
#[path = "📽️apply-presentation/🧪️tests/↩️inverts/🦀️.rs"]
mod apply_presentation_inverts;
#[cfg(test)]
#[path = "🔀apply-flow/🧪️tests/↩️inverts/🦀️.rs"]
mod apply_flow_inverts;
#[cfg(test)]
#[path = "🔊apply-audio/🧪️tests/↩️inverts/🦀️.rs"]
mod apply_audio_inverts;
#[cfg(test)]
#[path = "🔢apply-value/🧪️tests/↩️inverts/🦀️.rs"]
mod apply_value_inverts;
#[cfg(test)]
#[path = "🔤apply-text/🧪️tests/↩️inverts/🦀️.rs"]
mod apply_text_inverts;
#[cfg(test)]
#[path = "🕸️apply-mesh/🧪️tests/↩️inverts/🦀️.rs"]
mod apply_mesh_inverts;
#[cfg(test)]
#[path = "🖊️apply-drawing/🧪️tests/↩️inverts/🦀️.rs"]
mod apply_drawing_inverts;
#[cfg(test)]
#[path = "🗂️apply-table/🧪️tests/↩️inverts/🦀️.rs"]
mod apply_table_inverts;
#[cfg(test)]
#[path = "🧰apply-kit/🧪️tests/↩️inverts/🦀️.rs"]
mod apply_kit_inverts;
#[cfg(test)]
#[path = "🧱apply-brep/🧪️tests/↩️inverts/🦀️.rs"]
mod apply_brep_inverts;
#[cfg(test)]
#[path = "🖼️apply-image/🧪️tests/↩️inverts/🦀️.rs"]
mod apply_image_inverts;
//#endregion 🧪️FixtureCases

#[cfg(test)]
use protocol::{OpBinary,OpText};
