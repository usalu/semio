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

use crate::standards::v1::subsets::animation::schema::{mutations::SemioAnimationMutation, snapshot::SemioAnimationSnapshot};
#[cfg(test)]
use crate::standards::v1::subsets::audio::schema::mutations::set_sample_rate;
use crate::standards::v1::subsets::audio::schema::{mutations::SemioAudioMutation, snapshot::SemioAudioSnapshot};
use crate::standards::v1::subsets::base::schema::diff::SemioDiff;
use crate::standards::v1::subsets::base::schema::snapshot::{SemioSnapshot, SemioSubsetSnapshot};
use crate::standards::v1::subsets::brep::schema::{mutations::SemioBrepMutation, snapshot::SemioBrepSnapshot};
use crate::standards::v1::subsets::cad::schema::{mutations::SemioCadMutation, snapshot::SemioCadSnapshot};
use crate::standards::v1::subsets::document::schema::{mutations::SemioDocumentMutation, snapshot::SemioDocumentSnapshot};
use crate::standards::v1::subsets::drawing::schema::{mutations::SemioDrawingMutation, snapshot::SemioDrawingSnapshot};
use crate::standards::v1::subsets::flow::schema::{mutations::SemioFlowMutation, snapshot::SemioFlowSnapshot};
use crate::standards::v1::subsets::graph::schema::{mutations::SemioGraphMutation, snapshot::SemioGraphSnapshot};
use crate::standards::v1::subsets::image::schema::{mutations::SemioImageMutation, snapshot::SemioImageSnapshot};
use crate::standards::v1::subsets::kit::schema::{mutations::SemioKitMutation, snapshot::SemioKitSnapshot};
use crate::standards::v1::subsets::mesh::schema::{mutations::SemioMeshMutation, snapshot::SemioMeshSnapshot};
use crate::standards::v1::subsets::model::schema::{mutations::SemioModelMutation, snapshot::SemioModelSnapshot};
use crate::standards::v1::subsets::object::schema::{mutations::SemioObjectMutation, snapshot::SemioObjectSnapshot};
use crate::standards::v1::subsets::presentation::schema::{mutations::SemioPresentationMutation, snapshot::SemioPresentationSnapshot};
use crate::standards::v1::subsets::table::schema::{mutations::SemioTableMutation, snapshot::SemioTableSnapshot};
use crate::standards::v1::subsets::text::schema::{mutations::SemioTextMutation, snapshot::SemioTextSnapshot};
use crate::standards::v1::subsets::value::schema::{mutations::SemioValueMutation, snapshot::SemioValueSnapshot};
use crate::standards::v1::subsets::video::schema::{mutations::SemioVideoMutation, snapshot::SemioVideoSnapshot};
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::RetireOwned)]
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
    <SemioMutation as protocol::Mutation<SemioSnapshot>>::diff(mutation, base)
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

/// 🚦️ The messages in `outcome` that genuinely REFUSE the mutation — `Error` and `Fatal` only,
/// rendered for a failure report. The frozen mutation-outcome contract
/// (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/📋️contract-freeze.md` §C2)
/// makes `Info`/`Warning` ADVISORY: they ride along with a diff that WAS applied in full, and the
/// contract's own worked example is `.info("mutation.cascade", …)` — which `🧊️brep`'s
/// `delete-vertex` and `🕸️graph`'s `delete-node` both raise on every well-formed body, naming the
/// edges the deletion also had to remove. A caller that reads "any message" as "rejected" therefore
/// reports a refusal that never happened and fails a scenario the codec answered correctly.
///
/// Generic over the diff type and declared ONCE here rather than eighteen times, because the
/// question is the envelope's, not any one arm's: every `mutate-semio-*` adapter needs it, and none
/// of them can name this crate's private `protocol` extern-crate alias to ask it directly.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_mutation_refusals<D>(outcome: &protocol::MutationOutcome<D>) -> Vec<String> {
    outcome.messages().iter().filter(|message| message.level >= semio_framework_diagnostic::Severity::Error).map(|message| format!("{:?} {:?}: {}", message.level, message.code, message.message)).collect()
}

/// 🚦️ The FAULT CODES of the refusing messages in `outcome`, in order — the same `Error`/`Fatal`
/// filter [`semio_mutation_refusals`] applies, reduced to the frozen `mutation.*` code alone. The
/// envelope's own routing law is stated in terms of codes (a mismatched arm must raise exactly
/// `mutation.target-missing`), and `✉️mutate-semio-base` cannot read `MutationMessage::code` itself
/// because `protocol` is a private extern-crate alias of this crate.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_mutation_refusal_codes<D>(outcome: &protocol::MutationOutcome<D>) -> Vec<String> {
    outcome.messages().iter().filter(|message| message.level >= semio_framework_diagnostic::Severity::Error).map(|message| message.code.0.clone()).collect()
}

/// 🏷️ Free-function face of the envelope's own subset discriminator — the tag `KINDS` spells for
/// each wrapper variant and the only observable an owner-root test can read back out of a routed
/// envelope without naming any of the eighteen arms' snapshot types.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_subset_tag(snapshot: &SemioSnapshot) -> &'static str {
    crate::standards::v1::subsets::base::schema::snapshot::subset_tag(&snapshot.subset)
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
        SemioMutation::ApplyBrep(apply_brep::ApplyBrep { mutation: SemioBrepMutation::DeleteVertex(crate::standards::v1::subsets::brep::schema::mutations::delete_vertex::DeleteVertex { id: "v-absent".into() }) }),
        SemioMutation::ApplyMesh(apply_mesh::ApplyMesh { mutation: SemioMeshMutation::DeleteMesh(crate::standards::v1::subsets::mesh::schema::mutations::delete_mesh::DeleteMesh { id: "mesh-absent".into() }) }),
        SemioMutation::ApplyDrawing(apply_drawing::ApplyDrawing {
            mutation: SemioDrawingMutation::DragNodes(crate::standards::v1::subsets::drawing::schema::mutations::drag_nodes::DragNodes { ats: Vec::new(), offset: crate::standards::v1::subsets::base::schema::geometry::SemioPoint2::default() }),
        }),
        SemioMutation::ApplyAnimation(apply_animation::ApplyAnimation {
        }),
        SemioMutation::ApplyPresentation(apply_presentation::ApplyPresentation {
        }),
        SemioMutation::ApplyText(apply_text::ApplyText { mutation: SemioTextMutation::RemoveRun(crate::standards::v1::subsets::text::schema::mutations::remove_run::RemoveRun { index: 99 }) }),
        SemioMutation::ApplyTable(apply_table::ApplyTable { mutation: SemioTableMutation::RemoveRow(crate::standards::v1::subsets::table::schema::mutations::remove_row::RemoveRow { index: 99 }) }),
        SemioMutation::ApplyGraph(apply_graph::ApplyGraph {
            mutation: SemioGraphMutation::DeleteNode(crate::standards::v1::subsets::graph::schema::mutations::delete_node::DeleteNode { id: crate::standards::v1::subsets::graph::schema::snapshot::GraphNodeId::new("absent") }),
        }),
        SemioMutation::ApplyObject(apply_object::ApplyObject { mutation: SemioObjectMutation::DeleteBrep(crate::standards::v1::subsets::object::schema::mutations::delete_brep::DeleteBrep {}) }),
        SemioMutation::ApplyKit(apply_kit::ApplyKit { mutation: SemioKitMutation::RemoveType(crate::standards::v1::subsets::kit::schema::mutations::remove_type::RemoveType { id: "absent".into() }) }),
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
