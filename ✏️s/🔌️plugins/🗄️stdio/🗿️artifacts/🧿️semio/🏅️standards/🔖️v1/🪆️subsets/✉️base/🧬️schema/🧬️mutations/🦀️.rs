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
use protocol::OpBinary;
use protocol::OpText;

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
/// `ReplaceByteRange` just got. Each renamed variant carries `#[value(rename = "<noun>")]` so the
/// wire tag (`"brep"`, `"mesh"`, …) is unchanged — the catalog, the feature files and every
/// committed fixture still speak the bare noun.
//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = SemioSnapshot, diff = SemioDiff, schema = "SemioMutation")]
#[value(tag = "mutation", content = "payload", rename_all = "camelCase")]
pub enum SemioMutation {
    /// 🧨 Full-snapshot replace — the only way to change SUBSET KIND (there is no sparse
    /// representation for "this artifact used to be a video, now it's a flow").
    SetSnapshot(set_snapshot::SetSnapshot),
    #[value(rename = "brep")]
    ApplyBrep(apply_brep::ApplyBrep),
    #[value(rename = "mesh")]
    ApplyMesh(apply_mesh::ApplyMesh),
    #[value(rename = "model")]
    ApplyModel(apply_model::ApplyModel),
    #[value(rename = "value")]
    ApplyValue(apply_value::ApplyValue),
    #[value(rename = "document")]
    ApplyDocument(apply_document::ApplyDocument),
    #[value(rename = "cad")]
    ApplyCad(apply_cad::ApplyCad),
    #[value(rename = "drawing")]
    ApplyDrawing(apply_drawing::ApplyDrawing),
    #[value(rename = "image")]
    ApplyImage(apply_image::ApplyImage),
    #[value(rename = "video")]
    ApplyVideo(apply_video::ApplyVideo),
    #[value(rename = "audio")]
    ApplyAudio(apply_audio::ApplyAudio),
    #[value(rename = "animation")]
    ApplyAnimation(apply_animation::ApplyAnimation),
    #[value(rename = "presentation")]
    ApplyPresentation(apply_presentation::ApplyPresentation),
    #[value(rename = "flow")]
    ApplyFlow(apply_flow::ApplyFlow),
    #[value(rename = "text")]
    ApplyText(apply_text::ApplyText),
    #[value(rename = "table")]
    ApplyTable(apply_table::ApplyTable),
    #[value(rename = "graph")]
    ApplyGraph(apply_graph::ApplyGraph),
    #[value(rename = "object")]
    ApplyObject(apply_object::ApplyObject),
    #[value(rename = "kit")]
    ApplyKit(apply_kit::ApplyKit),
}

/// 🏷️ Kebab-case spelling of every `SemioMutation` variant, in declaration order — the vocabulary
/// the `semio-v1-base` mutation catalog (`../../🔣️oracle.json`) declares and
/// `mutate-semio-any`'s exhaustive test case measures itself against. The eighteen wrapper variants
/// are spelled by their SUBSET name (`brep`, `mesh`, …), which is what the envelope actually routes
/// on; `kinds_match_the_enum_and_the_catalog` below pins the list with a WILDCARD-FREE match, so a
/// nineteenth subset cannot be added without extending both it and `KINDS`.
pub const KINDS: &[&str] = &["set-snapshot", "brep", "mesh", "model", "value", "document", "cad", "drawing", "image", "video", "audio", "animation", "presentation", "flow", "text", "table", "graph", "object", "kit"];

/// ▶️ Applies a mutation to `snapshot` in place, returning the diff (mirrors gif's
/// `apply_gif_mutation` convention — used by the builder's `mutate()` and the set-snapshot leaf).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_semio_mutation(snapshot: &mut SemioSnapshot, mutation: &SemioMutation) -> protocol::MutationOutcome<SemioDiff> {
    let outcome = <SemioMutation as Mutation<SemioSnapshot>>::diff(mutation, snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ Free-function face of [`Mutation::inverse`], named only in this subset's own reachable types.
/// `protocol` is a private `extern crate semio_framework_os_kernel as protocol;` alias that nothing
/// re-exports, so an owner-root test adapter compiled as an external crate cannot bring the
/// `Mutation` trait into scope to call the method form — the structural gap wave 7 recorded for
/// `kit`/`object`/`text`/`table`, and the same thin-wrapper remedy `kit` adopted. Used by
/// `mutate-semio-any`'s `inverse-*` scenarios.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_semio_mutation(mutation: &SemioMutation, base: &SemioSnapshot) -> Vec<SemioMutation> {
    <SemioMutation as Mutation<SemioSnapshot>>::inverse(mutation, base)
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
    outcome.messages().iter().filter(|message| message.level >= protocol::Severity::Error).map(|message| format!("{:?} {:?}: {}", message.level, message.code, message.message)).collect()
}

/// 🚦️ The FAULT CODES of the refusing messages in `outcome`, in order — the same `Error`/`Fatal`
/// filter [`semio_mutation_refusals`] applies, reduced to the frozen `mutation.*` code alone. The
/// envelope's own routing law is stated in terms of codes (a mismatched arm must raise exactly
/// `mutation.target-missing`), and `mutate-semio-any` cannot read `MutationMessage::code` itself
/// because `protocol` is a private extern-crate alias of this crate.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_mutation_refusal_codes<D>(outcome: &protocol::MutationOutcome<D>) -> Vec<String> {
    outcome.messages().iter().filter(|message| message.level >= protocol::Severity::Error).map(|message| message.code.0.clone()).collect()
}

/// 🏷️ Free-function face of the envelope's own subset discriminator — the tag `KINDS` spells for
/// each wrapper variant and the only observable an owner-root test can read back out of a routed
/// envelope without naming any of the eighteen arms' snapshot types.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_subset_tag(snapshot: &SemioSnapshot) -> &'static str {
    crate::standards::v1::subsets::base::schema::snapshot::subset_tag(&snapshot.subset)
}
//#endregion 🔖️Mutation

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &SemioMutation, base: &SemioSnapshot) -> protocol::MutationOutcome<SemioDiff> {
    use SemioSubsetSnapshot as S;
    match (this, &base.subset) {
        (SemioMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }), _) => protocol::MutationOutcome::new(SemioDiff::Replace(Box::new(snapshot.clone()))),
        (SemioMutation::ApplyBrep(apply_brep::ApplyBrep { mutation }), S::Brep(b)) => <SemioBrepMutation as Mutation<SemioBrepSnapshot>>::diff(mutation, b).map(SemioDiff::Brep),
        (SemioMutation::ApplyMesh(apply_mesh::ApplyMesh { mutation }), S::Mesh(b)) => <SemioMeshMutation as Mutation<SemioMeshSnapshot>>::diff(mutation, b).map(SemioDiff::Mesh),
        (SemioMutation::ApplyModel(apply_model::ApplyModel { mutation }), S::Model(b)) => <SemioModelMutation as Mutation<SemioModelSnapshot>>::diff(mutation, b).map(SemioDiff::Model),
        (SemioMutation::ApplyValue(apply_value::ApplyValue { mutation }), S::Value(b)) => <SemioValueMutation as Mutation<SemioValueSnapshot>>::diff(mutation, b).map(SemioDiff::Value),
        (SemioMutation::ApplyDocument(apply_document::ApplyDocument { mutation }), S::Document(b)) => <SemioDocumentMutation as Mutation<SemioDocumentSnapshot>>::diff(mutation, b).map(SemioDiff::Document),
        (SemioMutation::ApplyCad(apply_cad::ApplyCad { mutation }), S::Cad(b)) => <SemioCadMutation as Mutation<SemioCadSnapshot>>::diff(mutation, b).map(SemioDiff::Cad),
        (SemioMutation::ApplyDrawing(apply_drawing::ApplyDrawing { mutation }), S::Drawing(b)) => <SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::diff(mutation, b).map(SemioDiff::Drawing),
        (SemioMutation::ApplyImage(apply_image::ApplyImage { mutation }), S::Image(b)) => <SemioImageMutation as Mutation<SemioImageSnapshot>>::diff(mutation, b).map(SemioDiff::Image),
        (SemioMutation::ApplyVideo(apply_video::ApplyVideo { mutation }), S::Video(b)) => <SemioVideoMutation as Mutation<SemioVideoSnapshot>>::diff(mutation, b).map(SemioDiff::Video),
        (SemioMutation::ApplyAudio(apply_audio::ApplyAudio { mutation }), S::Audio(b)) => <SemioAudioMutation as Mutation<SemioAudioSnapshot>>::diff(mutation, b).map(SemioDiff::Audio),
        (SemioMutation::ApplyAnimation(apply_animation::ApplyAnimation { mutation }), S::Animation(b)) => <SemioAnimationMutation as Mutation<SemioAnimationSnapshot>>::diff(mutation, b).map(SemioDiff::Animation),
        (SemioMutation::ApplyPresentation(apply_presentation::ApplyPresentation { mutation }), S::Presentation(b)) => <SemioPresentationMutation as Mutation<SemioPresentationSnapshot>>::diff(mutation, b).map(SemioDiff::Presentation),
        (SemioMutation::ApplyFlow(apply_flow::ApplyFlow { mutation }), S::Flow(b)) => <SemioFlowMutation as Mutation<SemioFlowSnapshot>>::diff(mutation, b).map(SemioDiff::Flow),
        (SemioMutation::ApplyText(apply_text::ApplyText { mutation }), S::Text(b)) => <SemioTextMutation as Mutation<SemioTextSnapshot>>::diff(mutation, b).map(SemioDiff::Text),
        (SemioMutation::ApplyTable(apply_table::ApplyTable { mutation }), S::Table(b)) => <SemioTableMutation as Mutation<SemioTableSnapshot>>::diff(mutation, b).map(SemioDiff::Table),
        (SemioMutation::ApplyGraph(apply_graph::ApplyGraph { mutation }), S::Graph(b)) => <SemioGraphMutation as Mutation<SemioGraphSnapshot>>::diff(mutation, b).map(SemioDiff::Graph),
        (SemioMutation::ApplyObject(apply_object::ApplyObject { mutation }), S::Object(b)) => <SemioObjectMutation as Mutation<SemioObjectSnapshot>>::diff(mutation, b).map(SemioDiff::Object),
        (SemioMutation::ApplyKit(apply_kit::ApplyKit { mutation }), S::Kit(b)) => <SemioKitMutation as Mutation<SemioKitSnapshot>>::diff(mutation, b).map(SemioDiff::Kit),
        _ => protocol::MutationOutcome::error("mutation.target-missing", "Mutation subset does not match the snapshot subset.", ["subset"]),
    }
}

/// ↩️ `Vec::new()` on a kind mismatch — same convention every migrated subset's `agg_inverse` uses
/// where there is nothing to restore.
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &SemioMutation, base: &SemioSnapshot) -> Vec<SemioMutation> {
    use SemioSubsetSnapshot as S;
    match (this, &base.subset) {
        (SemioMutation::SetSnapshot(_), _) => vec![SemioMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        (SemioMutation::ApplyBrep(apply_brep::ApplyBrep { mutation }), S::Brep(b)) => {
            <SemioBrepMutation as Mutation<SemioBrepSnapshot>>::inverse(mutation, b).into_iter().map(|inner| SemioMutation::ApplyBrep(apply_brep::ApplyBrep { mutation: inner })).collect()
        }
        (SemioMutation::ApplyMesh(apply_mesh::ApplyMesh { mutation }), S::Mesh(b)) => {
            <SemioMeshMutation as Mutation<SemioMeshSnapshot>>::inverse(mutation, b).into_iter().map(|inner| SemioMutation::ApplyMesh(apply_mesh::ApplyMesh { mutation: inner })).collect()
        }
        (SemioMutation::ApplyModel(apply_model::ApplyModel { mutation }), S::Model(b)) => {
            <SemioModelMutation as Mutation<SemioModelSnapshot>>::inverse(mutation, b).into_iter().map(|inner| SemioMutation::ApplyModel(apply_model::ApplyModel { mutation: inner })).collect()
        }
        (SemioMutation::ApplyValue(apply_value::ApplyValue { mutation }), S::Value(b)) => {
            <SemioValueMutation as Mutation<SemioValueSnapshot>>::inverse(mutation, b).into_iter().map(|inner| SemioMutation::ApplyValue(apply_value::ApplyValue { mutation: inner })).collect()
        }
        (SemioMutation::ApplyDocument(apply_document::ApplyDocument { mutation }), S::Document(b)) => {
            <SemioDocumentMutation as Mutation<SemioDocumentSnapshot>>::inverse(mutation, b).into_iter().map(|inner| SemioMutation::ApplyDocument(apply_document::ApplyDocument { mutation: inner })).collect()
        }
        (SemioMutation::ApplyCad(apply_cad::ApplyCad { mutation }), S::Cad(b)) => {
            <SemioCadMutation as Mutation<SemioCadSnapshot>>::inverse(mutation, b).into_iter().map(|inner| SemioMutation::ApplyCad(apply_cad::ApplyCad { mutation: inner })).collect()
        }
        (SemioMutation::ApplyDrawing(apply_drawing::ApplyDrawing { mutation }), S::Drawing(b)) => {
            <SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::inverse(mutation, b).into_iter().map(|inner| SemioMutation::ApplyDrawing(apply_drawing::ApplyDrawing { mutation: inner })).collect()
        }
        (SemioMutation::ApplyImage(apply_image::ApplyImage { mutation }), S::Image(b)) => {
            <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(mutation, b).into_iter().map(|inner| SemioMutation::ApplyImage(apply_image::ApplyImage { mutation: inner })).collect()
        }
        (SemioMutation::ApplyVideo(apply_video::ApplyVideo { mutation }), S::Video(b)) => {
            <SemioVideoMutation as Mutation<SemioVideoSnapshot>>::inverse(mutation, b).into_iter().map(|inner| SemioMutation::ApplyVideo(apply_video::ApplyVideo { mutation: inner })).collect()
        }
        (SemioMutation::ApplyAudio(apply_audio::ApplyAudio { mutation }), S::Audio(b)) => {
            <SemioAudioMutation as Mutation<SemioAudioSnapshot>>::inverse(mutation, b).into_iter().map(|inner| SemioMutation::ApplyAudio(apply_audio::ApplyAudio { mutation: inner })).collect()
        }
        (SemioMutation::ApplyAnimation(apply_animation::ApplyAnimation { mutation }), S::Animation(b)) => {
            <SemioAnimationMutation as Mutation<SemioAnimationSnapshot>>::inverse(mutation, b).into_iter().map(|inner| SemioMutation::ApplyAnimation(apply_animation::ApplyAnimation { mutation: inner })).collect()
        }
        (SemioMutation::ApplyPresentation(apply_presentation::ApplyPresentation { mutation }), S::Presentation(b)) => {
            <SemioPresentationMutation as Mutation<SemioPresentationSnapshot>>::inverse(mutation, b).into_iter().map(|inner| SemioMutation::ApplyPresentation(apply_presentation::ApplyPresentation { mutation: inner })).collect()
        }
        (SemioMutation::ApplyFlow(apply_flow::ApplyFlow { mutation }), S::Flow(b)) => {
            <SemioFlowMutation as Mutation<SemioFlowSnapshot>>::inverse(mutation, b).into_iter().map(|inner| SemioMutation::ApplyFlow(apply_flow::ApplyFlow { mutation: inner })).collect()
        }
        (SemioMutation::ApplyText(apply_text::ApplyText { mutation }), S::Text(b)) => {
            <SemioTextMutation as Mutation<SemioTextSnapshot>>::inverse(mutation, b).into_iter().map(|inner| SemioMutation::ApplyText(apply_text::ApplyText { mutation: inner })).collect()
        }
        (SemioMutation::ApplyTable(apply_table::ApplyTable { mutation }), S::Table(b)) => {
            <SemioTableMutation as Mutation<SemioTableSnapshot>>::inverse(mutation, b).into_iter().map(|inner| SemioMutation::ApplyTable(apply_table::ApplyTable { mutation: inner })).collect()
        }
        (SemioMutation::ApplyGraph(apply_graph::ApplyGraph { mutation }), S::Graph(b)) => {
            <SemioGraphMutation as Mutation<SemioGraphSnapshot>>::inverse(mutation, b).into_iter().map(|inner| SemioMutation::ApplyGraph(apply_graph::ApplyGraph { mutation: inner })).collect()
        }
        (SemioMutation::ApplyObject(apply_object::ApplyObject { mutation }), S::Object(b)) => {
            <SemioObjectMutation as Mutation<SemioObjectSnapshot>>::inverse(mutation, b).into_iter().map(|inner| SemioMutation::ApplyObject(apply_object::ApplyObject { mutation: inner })).collect()
        }
        (SemioMutation::ApplyKit(apply_kit::ApplyKit { mutation }), S::Kit(b)) => {
            <SemioKitMutation as Mutation<SemioKitSnapshot>>::inverse(mutation, b).into_iter().map(|inner| SemioMutation::ApplyKit(apply_kit::ApplyKit { mutation: inner })).collect()
        }
        // 🛡️ Same kind-mismatch fallback as `agg_diff` above; nothing to restore.
        _ => Vec::new(),
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🎙️ Real delegating text/binary op codec — replaces the old whole-enum `serde_json` passthrough.
/// Text is one `tag:payload` line: `payload` for the 18 wrapped variants is exactly that subset's
/// OWN already-real `OpText::print_op()`/`parse_op()` output (genuine reuse, never re-derived
/// here); `setSnapshot`'s payload is hex(`SemioSnapshot::print_dsl`) — real delegation to this
/// envelope's own now-real `ArtifactDsl` (📸️snapshot/🦀️.rs), hex-flattened to keep
/// `print_op`'s one-physical-line contract.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn subset_mutation_tag(m: &SemioMutation) -> &'static str {
    match m {
        SemioMutation::SetSnapshot(_) => "setSnapshot",
        SemioMutation::ApplyBrep(_) => "brep",
        SemioMutation::ApplyMesh(_) => "mesh",
        SemioMutation::ApplyModel(_) => "model",
        SemioMutation::ApplyValue(_) => "value",
        SemioMutation::ApplyDocument(_) => "document",
        SemioMutation::ApplyCad(_) => "cad",
        SemioMutation::ApplyDrawing(_) => "drawing",
        SemioMutation::ApplyImage(_) => "image",
        SemioMutation::ApplyVideo(_) => "video",
        SemioMutation::ApplyAudio(_) => "audio",
        SemioMutation::ApplyAnimation(_) => "animation",
        SemioMutation::ApplyPresentation(_) => "presentation",
        SemioMutation::ApplyFlow(_) => "flow",
        SemioMutation::ApplyText(_) => "text",
        SemioMutation::ApplyTable(_) => "table",
        SemioMutation::ApplyGraph(_) => "graph",
        SemioMutation::ApplyObject(_) => "object",
        SemioMutation::ApplyKit(_) => "kit",
    }
}

/// 🏷️ Binary tag ordinal for [`SemioMutation`] — `0` = `SetSnapshot`, `1..=18` = the 18 wrapped
/// subset kinds (enum declaration order).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn mutation_tag(m: &SemioMutation) -> u8 {
    match m {
        SemioMutation::SetSnapshot(_) => 0,
        SemioMutation::ApplyBrep(_) => 1,
        SemioMutation::ApplyMesh(_) => 2,
        SemioMutation::ApplyModel(_) => 3,
        SemioMutation::ApplyValue(_) => 4,
        SemioMutation::ApplyDocument(_) => 5,
        SemioMutation::ApplyCad(_) => 6,
        SemioMutation::ApplyDrawing(_) => 7,
        SemioMutation::ApplyImage(_) => 8,
        SemioMutation::ApplyVideo(_) => 9,
        SemioMutation::ApplyAudio(_) => 10,
        SemioMutation::ApplyAnimation(_) => 11,
        SemioMutation::ApplyPresentation(_) => 12,
        SemioMutation::ApplyFlow(_) => 13,
        SemioMutation::ApplyText(_) => 14,
        SemioMutation::ApplyTable(_) => 15,
        SemioMutation::ApplyGraph(_) => 16,
        SemioMutation::ApplyObject(_) => 17,
        SemioMutation::ApplyKit(_) => 18,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_hex_snapshot(snapshot: &SemioSnapshot) -> String {
    let text = <SemioSnapshot as store::ArtifactDsl>::print_dsl(snapshot);
    text.as_bytes().iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_hex_snapshot(hex: &str) -> Result<SemioSnapshot, String> {
    if !hex.len().is_multiple_of(2) {
        return Err("setSnapshot: odd hex length".to_string());
    }
    let mut bytes = Vec::with_capacity(hex.len() / 2);
    let mut i = 0usize;
    while i < hex.len() {
        let byte = u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| format!("setSnapshot: invalid hex: {e}"))?;
        bytes.push(byte);
        i += 2;
    }
    let text = String::from_utf8(bytes).map_err(|e| format!("setSnapshot: utf8 decode: {e}"))?;
    <SemioSnapshot as store::ArtifactDsl>::parse_dsl(&text).map_err(|e| format!("setSnapshot: dsl decode: {e}"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_semio_mutation(m: &SemioMutation) -> String {
    let tag = subset_mutation_tag(m);
    match m {
        SemioMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("{tag}:{}", enc_hex_snapshot(snapshot)),
        SemioMutation::ApplyBrep(apply_brep::ApplyBrep { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyMesh(apply_mesh::ApplyMesh { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyModel(apply_model::ApplyModel { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyValue(apply_value::ApplyValue { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyDocument(apply_document::ApplyDocument { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyCad(apply_cad::ApplyCad { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyDrawing(apply_drawing::ApplyDrawing { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyImage(apply_image::ApplyImage { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyVideo(apply_video::ApplyVideo { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyAudio(apply_audio::ApplyAudio { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyAnimation(apply_animation::ApplyAnimation { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyPresentation(apply_presentation::ApplyPresentation { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyFlow(apply_flow::ApplyFlow { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyText(apply_text::ApplyText { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyTable(apply_table::ApplyTable { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyGraph(apply_graph::ApplyGraph { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyObject(apply_object::ApplyObject { mutation }) => format!("{tag}:{}", mutation.print_op()),
        SemioMutation::ApplyKit(apply_kit::ApplyKit { mutation }) => format!("{tag}:{}", mutation.print_op()),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_semio_mutation(line: &str) -> Result<SemioMutation, String> {
    let (tag, rest) = line.split_once(':').ok_or_else(|| format!("semio mutation: missing ':' in {line:?}"))?;
    match tag {
        "setSnapshot" => Ok(SemioMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_hex_snapshot(rest)? })),
        "brep" => Ok(SemioMutation::ApplyBrep(apply_brep::ApplyBrep { mutation: SemioBrepMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "mesh" => Ok(SemioMutation::ApplyMesh(apply_mesh::ApplyMesh { mutation: SemioMeshMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "model" => Ok(SemioMutation::ApplyModel(apply_model::ApplyModel { mutation: SemioModelMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "value" => Ok(SemioMutation::ApplyValue(apply_value::ApplyValue { mutation: SemioValueMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "document" => Ok(SemioMutation::ApplyDocument(apply_document::ApplyDocument { mutation: SemioDocumentMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "cad" => Ok(SemioMutation::ApplyCad(apply_cad::ApplyCad { mutation: SemioCadMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "drawing" => Ok(SemioMutation::ApplyDrawing(apply_drawing::ApplyDrawing { mutation: SemioDrawingMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "image" => Ok(SemioMutation::ApplyImage(apply_image::ApplyImage { mutation: SemioImageMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "video" => Ok(SemioMutation::ApplyVideo(apply_video::ApplyVideo { mutation: SemioVideoMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "audio" => Ok(SemioMutation::ApplyAudio(apply_audio::ApplyAudio { mutation: SemioAudioMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "animation" => Ok(SemioMutation::ApplyAnimation(apply_animation::ApplyAnimation { mutation: SemioAnimationMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "presentation" => Ok(SemioMutation::ApplyPresentation(apply_presentation::ApplyPresentation { mutation: SemioPresentationMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "flow" => Ok(SemioMutation::ApplyFlow(apply_flow::ApplyFlow { mutation: SemioFlowMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "text" => Ok(SemioMutation::ApplyText(apply_text::ApplyText { mutation: SemioTextMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "table" => Ok(SemioMutation::ApplyTable(apply_table::ApplyTable { mutation: SemioTableMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "graph" => Ok(SemioMutation::ApplyGraph(apply_graph::ApplyGraph { mutation: SemioGraphMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "object" => Ok(SemioMutation::ApplyObject(apply_object::ApplyObject { mutation: SemioObjectMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        "kit" => Ok(SemioMutation::ApplyKit(apply_kit::ApplyKit { mutation: SemioKitMutation::parse_op(rest).map_err(|e| e.to_string())? })),
        other => Err(format!("semio mutation: unknown tag {other:?}")),
    }
}

impl OpText for SemioMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_semio_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        print_semio_mutation(self)
    }
}

impl OpBinary for SemioMutation {
    /// ⚡️ Real delegating binary: `format u8` + `tag u8` ([`mutation_tag`]) as two genuine,
    /// individually protocol-walkable fixed header fields, then ONE opaque trailing payload — for
    /// the 18 wrapped variants, the wrapped subset's OWN real `OpBinary::encode_op()` bytes
    /// (genuine reuse); for `SetSnapshot`, the wrapped snapshot's own real
    /// `ArtifactPack::encode_pack()` bytes.
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut out = vec![OP_BINARY_FORMAT, mutation_tag(self)];
        let payload: Vec<u8> = match self {
            SemioMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => <SemioSnapshot as store::ArtifactPack>::encode_pack(snapshot),
            SemioMutation::ApplyBrep(apply_brep::ApplyBrep { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyMesh(apply_mesh::ApplyMesh { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyModel(apply_model::ApplyModel { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyValue(apply_value::ApplyValue { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyDocument(apply_document::ApplyDocument { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyCad(apply_cad::ApplyCad { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyDrawing(apply_drawing::ApplyDrawing { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyImage(apply_image::ApplyImage { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyVideo(apply_video::ApplyVideo { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyAudio(apply_audio::ApplyAudio { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyAnimation(apply_animation::ApplyAnimation { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyPresentation(apply_presentation::ApplyPresentation { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyFlow(apply_flow::ApplyFlow { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyText(apply_text::ApplyText { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyTable(apply_table::ApplyTable { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyGraph(apply_graph::ApplyGraph { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyObject(apply_object::ApplyObject { mutation }) => mutation.encode_op()?,
            SemioMutation::ApplyKit(apply_kit::ApplyKit { mutation }) => mutation.encode_op()?,
        };
        out.extend_from_slice(&payload);
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "op header", offset: 0, detail: "truncated".to_string() });
        }
        let format = bytes[0];
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported format {format}") });
        }
        let tag = bytes[1];
        let payload = &bytes[2..];
        Ok(match tag {
            0 => SemioMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: <SemioSnapshot as store::ArtifactPack>::decode_pack(payload)? }),
            1 => SemioMutation::ApplyBrep(apply_brep::ApplyBrep { mutation: SemioBrepMutation::decode_op(payload)? }),
            2 => SemioMutation::ApplyMesh(apply_mesh::ApplyMesh { mutation: SemioMeshMutation::decode_op(payload)? }),
            3 => SemioMutation::ApplyModel(apply_model::ApplyModel { mutation: SemioModelMutation::decode_op(payload)? }),
            4 => SemioMutation::ApplyValue(apply_value::ApplyValue { mutation: SemioValueMutation::decode_op(payload)? }),
            5 => SemioMutation::ApplyDocument(apply_document::ApplyDocument { mutation: SemioDocumentMutation::decode_op(payload)? }),
            6 => SemioMutation::ApplyCad(apply_cad::ApplyCad { mutation: SemioCadMutation::decode_op(payload)? }),
            7 => SemioMutation::ApplyDrawing(apply_drawing::ApplyDrawing { mutation: SemioDrawingMutation::decode_op(payload)? }),
            8 => SemioMutation::ApplyImage(apply_image::ApplyImage { mutation: SemioImageMutation::decode_op(payload)? }),
            9 => SemioMutation::ApplyVideo(apply_video::ApplyVideo { mutation: SemioVideoMutation::decode_op(payload)? }),
            10 => SemioMutation::ApplyAudio(apply_audio::ApplyAudio { mutation: SemioAudioMutation::decode_op(payload)? }),
            11 => SemioMutation::ApplyAnimation(apply_animation::ApplyAnimation { mutation: SemioAnimationMutation::decode_op(payload)? }),
            12 => SemioMutation::ApplyPresentation(apply_presentation::ApplyPresentation { mutation: SemioPresentationMutation::decode_op(payload)? }),
            13 => SemioMutation::ApplyFlow(apply_flow::ApplyFlow { mutation: SemioFlowMutation::decode_op(payload)? }),
            14 => SemioMutation::ApplyText(apply_text::ApplyText { mutation: SemioTextMutation::decode_op(payload)? }),
            15 => SemioMutation::ApplyTable(apply_table::ApplyTable { mutation: SemioTableMutation::decode_op(payload)? }),
            16 => SemioMutation::ApplyGraph(apply_graph::ApplyGraph { mutation: SemioGraphMutation::decode_op(payload)? }),
            17 => SemioMutation::ApplyObject(apply_object::ApplyObject { mutation: SemioObjectMutation::decode_op(payload)? }),
            18 => SemioMutation::ApplyKit(apply_kit::ApplyKit { mutation: SemioKitMutation::decode_op(payload)? }),
            other => return Err(protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("unknown tag {other}") }),
        })
    }
}
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
        SemioMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: SemioSnapshot::default() }),
        SemioMutation::ApplyBrep(apply_brep::ApplyBrep { mutation: SemioBrepMutation::DeleteVertex(crate::standards::v1::subsets::brep::schema::mutations::delete_vertex::DeleteVertex { id: "v-absent".into() }) }),
        SemioMutation::ApplyMesh(apply_mesh::ApplyMesh { mutation: SemioMeshMutation::DeleteMesh(crate::standards::v1::subsets::mesh::schema::mutations::delete_mesh::DeleteMesh { id: "mesh-absent".into() }) }),
        SemioMutation::ApplyModel(apply_model::ApplyModel { mutation: SemioModelMutation::SetSnapshot(crate::standards::v1::subsets::model::schema::mutations::set_snapshot::SetSnapshot { snapshot: Default::default() }) }),
        SemioMutation::ApplyValue(apply_value::ApplyValue { mutation: SemioValueMutation::SetSnapshot(crate::standards::v1::subsets::value::schema::mutations::set_snapshot::SetSnapshot { snapshot: Default::default() }) }),
        SemioMutation::ApplyDocument(apply_document::ApplyDocument { mutation: SemioDocumentMutation::SetSnapshot(crate::standards::v1::subsets::document::schema::mutations::set_snapshot::SetSnapshot { snapshot: Default::default() }) }),
        SemioMutation::ApplyCad(apply_cad::ApplyCad { mutation: SemioCadMutation::SetSnapshot(crate::standards::v1::subsets::cad::schema::mutations::set_snapshot::SetSnapshot { snapshot: Default::default() }) }),
        SemioMutation::ApplyDrawing(apply_drawing::ApplyDrawing {
            mutation: SemioDrawingMutation::DragNodes(crate::standards::v1::subsets::drawing::schema::mutations::drag_nodes::DragNodes { ats: Vec::new(), offset: crate::standards::v1::subsets::base::schema::geometry::SemioPoint2::default() }),
        }),
        SemioMutation::ApplyImage(apply_image::ApplyImage { mutation: SemioImageMutation::SetSnapshot(crate::standards::v1::subsets::image::schema::mutations::set_snapshot::SetSnapshot { snapshot: Default::default() }) }),
        SemioMutation::ApplyVideo(apply_video::ApplyVideo { mutation: SemioVideoMutation::SetSnapshot(crate::standards::v1::subsets::video::schema::mutations::set_snapshot::SetSnapshot { snapshot: Default::default() }) }),
        SemioMutation::ApplyAudio(apply_audio::ApplyAudio { mutation: SemioAudioMutation::SetSnapshot(crate::standards::v1::subsets::audio::schema::mutations::set_snapshot::SetSnapshot { snapshot: Default::default() }) }),
        SemioMutation::ApplyAnimation(apply_animation::ApplyAnimation {
            mutation: SemioAnimationMutation::SetSnapshot(crate::standards::v1::subsets::animation::schema::mutations::set_snapshot::SetSnapshot { snapshot: SemioAnimationSnapshot::default() }),
        }),
        SemioMutation::ApplyPresentation(apply_presentation::ApplyPresentation {
            mutation: SemioPresentationMutation::SetSnapshot(crate::standards::v1::subsets::presentation::schema::mutations::set_snapshot::SetSnapshot { snapshot: Default::default() }),
        }),
        SemioMutation::ApplyFlow(apply_flow::ApplyFlow { mutation: SemioFlowMutation::SetSnapshot(crate::standards::v1::subsets::flow::schema::mutations::set_snapshot::SetSnapshot { snapshot: Default::default() }) }),
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
/// 🧪️ Handcrafted `📸️set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `🦀️.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📸️set-snapshot/🧪️tests/✉️replaces-the-a60559/🦀️.rs"]
mod set_snapshot_replaces_the_envelope_wrapping_a_value_subset;
//#endregion 🧪️FixtureCases
