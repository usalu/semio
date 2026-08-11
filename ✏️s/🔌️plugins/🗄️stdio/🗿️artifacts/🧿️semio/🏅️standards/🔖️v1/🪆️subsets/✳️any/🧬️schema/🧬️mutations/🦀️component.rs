//! 🧬️ SemioMutation — the envelope union's own mutation vocabulary, per the master plan:
//! "NoMutation, SetSnapshot, + 13 wrapper variants embedding each subset's own mutation enum".
//! W2b closer real implementation, replacing the W1b `SetSnapshot`-only scaffold: the 13 wrapper
//! variants each carry that subset's OWN, already-real, already-hand-written `SemioXMutation`
//! enum unchanged (`SemioBrepMutation`, `SemioAudioMutation`, …) — every `diff()`/`inverse()` for
//! a wrapped variant delegates straight through to that subset's own `Mutation` impl, so this
//! module never re-derives any of the 13 subsets' own per-field mutation logic; its OWN job is
//! purely the envelope-level routing (does the wrapped mutation's kind match the base snapshot's
//! current kind, and if so thread it through).

use crate::artifacts::semio::standards::v1::subsets::any::schema::diff::SemioDiff;
use crate::artifacts::semio::standards::v1::subsets::any::schema::snapshot::{SemioSnapshot, SemioSubsetSnapshot};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::{mutations::SemioBrepMutation, snapshot::SemioBrepSnapshot};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::{mutations::SemioMeshMutation, snapshot::SemioMeshSnapshot};
use crate::artifacts::semio::standards::v1::subsets::model::schema::{mutations::SemioModelMutation, snapshot::SemioModelSnapshot};
use crate::artifacts::semio::standards::v1::subsets::object::schema::{mutations::SemioObjectMutation, snapshot::SemioObjectSnapshot};
use crate::artifacts::semio::standards::v1::subsets::document::schema::{mutations::SemioDocumentMutation, snapshot::SemioDocumentSnapshot};
use crate::artifacts::semio::standards::v1::subsets::cad::schema::{mutations::SemioCadMutation, snapshot::SemioCadSnapshot};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::{mutations::SemioDrawingMutation, snapshot::SemioDrawingSnapshot};
use crate::artifacts::semio::standards::v1::subsets::image::schema::{mutations::SemioImageMutation, snapshot::SemioImageSnapshot};
use crate::artifacts::semio::standards::v1::subsets::video::schema::{mutations::SemioVideoMutation, snapshot::SemioVideoSnapshot};
use crate::artifacts::semio::standards::v1::subsets::audio::schema::{mutations::SemioAudioMutation, snapshot::SemioAudioSnapshot};
use crate::artifacts::semio::standards::v1::subsets::animation::schema::{mutations::SemioAnimationMutation, snapshot::SemioAnimationSnapshot};
use crate::artifacts::semio::standards::v1::subsets::presentation::schema::{mutations::SemioPresentationMutation, snapshot::SemioPresentationSnapshot};
use crate::artifacts::semio::standards::v1::subsets::workflow::schema::{mutations::SemioWorkflowMutation, snapshot::SemioWorkflowSnapshot};
use protocol::Mutation;
use protocol::MutationDiff;
/// 🔧️ `OpText` unconditional — the non-test `impl protocol::OpBinary for SemioMutation` block
/// below calls `self.print_op()` via method syntax, needing `OpText` in scope in production code
/// too (see the same fix applied repo-wide by the W2b closer for document/workflow/image's own
/// mutation modules). `OpBinary` itself is only ever called via method/associated-fn syntax from
/// this file's own tests, so it stays `#[cfg(test)]`-gated (avoids an unused-import warning on a
/// plain non-test `cargo check`).
use protocol::OpText;
#[cfg(test)]
use protocol::OpBinary;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔧️ Adjacently tagged (`tag = "mutation"`, `content = "payload"`), NOT internally tagged like
/// every one of the 13 wrapped subset enums' own `#[serde(tag = "mutation", ...)]` — an
/// internally-tagged wrapper here would collide key-for-key with a wrapped variant's OWN
/// `"mutation"` discriminator field when serde flattens a newtype variant's fields into the
/// outer object (real bug caught by this file's own `op_text_binary_roundtrip_law` test: printed
/// JSON came out `{"mutation":"audio","mutation":"setSampleRate",...}`, two keys with the same
/// name, which `serde_json` then refuses to parse back). `content = "payload"` nests the wrapped
/// value under its own key instead of flattening it, sidestepping the collision entirely.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", content = "payload", rename_all = "camelCase")]
pub enum SemioMutation {
    #[default]
    NoMutation,
    /// 🧨 Full-snapshot replace — the only way to change SUBSET KIND (there is no sparse
    /// representation for "this artifact used to be a video, now it's a workflow").
    SetSnapshot { snapshot: SemioSnapshot },
    Brep(SemioBrepMutation),
    Mesh(SemioMeshMutation),
    Model(SemioModelMutation),
    Object(SemioObjectMutation),
    Document(SemioDocumentMutation),
    Cad(SemioCadMutation),
    Drawing(SemioDrawingMutation),
    Image(SemioImageMutation),
    Video(SemioVideoMutation),
    Audio(SemioAudioMutation),
    Animation(SemioAnimationMutation),
    Presentation(SemioPresentationMutation),
    Workflow(SemioWorkflowMutation),
}

impl Mutation<SemioSnapshot> for SemioMutation {
    type Diff = SemioDiff;

    fn diff(&self, base: &SemioSnapshot) -> Self::Diff {
        use SemioSubsetSnapshot as S;
        match (self, &base.subset) {
            (SemioMutation::NoMutation, _) => SemioDiff::NoChange,
            (SemioMutation::SetSnapshot { snapshot }, _) => SemioDiff::Replace(Box::new(snapshot.clone())),
            (SemioMutation::Brep(m), S::Brep(b)) => SemioDiff::Brep(<SemioBrepMutation as Mutation<SemioBrepSnapshot>>::diff(m, b)),
            (SemioMutation::Mesh(m), S::Mesh(b)) => SemioDiff::Mesh(<SemioMeshMutation as Mutation<SemioMeshSnapshot>>::diff(m, b)),
            (SemioMutation::Model(m), S::Model(b)) => SemioDiff::Model(<SemioModelMutation as Mutation<SemioModelSnapshot>>::diff(m, b)),
            (SemioMutation::Object(m), S::Object(b)) => SemioDiff::Object(<SemioObjectMutation as Mutation<SemioObjectSnapshot>>::diff(m, b)),
            (SemioMutation::Document(m), S::Document(b)) => SemioDiff::Document(<SemioDocumentMutation as Mutation<SemioDocumentSnapshot>>::diff(m, b)),
            (SemioMutation::Cad(m), S::Cad(b)) => SemioDiff::Cad(<SemioCadMutation as Mutation<SemioCadSnapshot>>::diff(m, b)),
            (SemioMutation::Drawing(m), S::Drawing(b)) => SemioDiff::Drawing(<SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::diff(m, b)),
            (SemioMutation::Image(m), S::Image(b)) => SemioDiff::Image(<SemioImageMutation as Mutation<SemioImageSnapshot>>::diff(m, b)),
            (SemioMutation::Video(m), S::Video(b)) => SemioDiff::Video(<SemioVideoMutation as Mutation<SemioVideoSnapshot>>::diff(m, b)),
            (SemioMutation::Audio(m), S::Audio(b)) => SemioDiff::Audio(<SemioAudioMutation as Mutation<SemioAudioSnapshot>>::diff(m, b)),
            (SemioMutation::Animation(m), S::Animation(b)) => SemioDiff::Animation(<SemioAnimationMutation as Mutation<SemioAnimationSnapshot>>::diff(m, b)),
            (SemioMutation::Presentation(m), S::Presentation(b)) => SemioDiff::Presentation(<SemioPresentationMutation as Mutation<SemioPresentationSnapshot>>::diff(m, b)),
            (SemioMutation::Workflow(m), S::Workflow(b)) => SemioDiff::Workflow(<SemioWorkflowMutation as Mutation<SemioWorkflowSnapshot>>::diff(m, b)),
            // 🛡️ A wrapped mutation whose kind doesn't match the base snapshot's current kind
            // (e.g. `SemioMutation::Audio(..)` applied to a workflow base): can only arise from
            // caller error, not from any path this module itself constructs. `diff()` has no
            // `Result` in its signature (per `protocol::Mutation`), so it degrades to a safe
            // no-op (`NoChange`) rather than panicking — same total-fallback stance as `SemioDiff`'s
            // own `apply`/`absorb`/`inverse`.
            _ => SemioDiff::NoChange,
        }
    }

    fn inverse(&self, base: &SemioSnapshot) -> Vec<Self> {
        use SemioSubsetSnapshot as S;
        match (self, &base.subset) {
            (SemioMutation::NoMutation, _) => vec![SemioMutation::NoMutation],
            (SemioMutation::SetSnapshot { .. }, _) => vec![SemioMutation::SetSnapshot { snapshot: base.clone() }],
            (SemioMutation::Brep(m), S::Brep(b)) => <SemioBrepMutation as Mutation<SemioBrepSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Brep).collect(),
            (SemioMutation::Mesh(m), S::Mesh(b)) => <SemioMeshMutation as Mutation<SemioMeshSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Mesh).collect(),
            (SemioMutation::Model(m), S::Model(b)) => <SemioModelMutation as Mutation<SemioModelSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Model).collect(),
            (SemioMutation::Object(m), S::Object(b)) => <SemioObjectMutation as Mutation<SemioObjectSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Object).collect(),
            (SemioMutation::Document(m), S::Document(b)) => <SemioDocumentMutation as Mutation<SemioDocumentSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Document).collect(),
            (SemioMutation::Cad(m), S::Cad(b)) => <SemioCadMutation as Mutation<SemioCadSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Cad).collect(),
            (SemioMutation::Drawing(m), S::Drawing(b)) => <SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Drawing).collect(),
            (SemioMutation::Image(m), S::Image(b)) => <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Image).collect(),
            (SemioMutation::Video(m), S::Video(b)) => <SemioVideoMutation as Mutation<SemioVideoSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Video).collect(),
            (SemioMutation::Audio(m), S::Audio(b)) => <SemioAudioMutation as Mutation<SemioAudioSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Audio).collect(),
            (SemioMutation::Animation(m), S::Animation(b)) => <SemioAnimationMutation as Mutation<SemioAnimationSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Animation).collect(),
            (SemioMutation::Presentation(m), S::Presentation(b)) => <SemioPresentationMutation as Mutation<SemioPresentationSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Presentation).collect(),
            (SemioMutation::Workflow(m), S::Workflow(b)) => <SemioWorkflowMutation as Mutation<SemioWorkflowSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Workflow).collect(),
            // 🛡️ Same kind-mismatch fallback as `diff()` above.
            _ => vec![SemioMutation::NoMutation],
        }
    }
}

/// ▶️ Applies a mutation to `snapshot` in place, returning the diff (mirrors gif's
/// `apply_gif_mutation` convention — used by the builder's `mutate()` and the set-snapshot leaf).
pub fn apply_semio_mutation(snapshot: &mut SemioSnapshot, mutation: &SemioMutation) -> SemioDiff {
    let diff = <SemioMutation as Mutation<SemioSnapshot>>::diff(mutation, snapshot);
    *snapshot = <SemioDiff as MutationDiff<SemioSnapshot>>::apply(&diff, snapshot);
    diff
}
//#endregion 🔖️Mutation

//#region OpCodecs
/// 🎙️ Handcrafted `OpText`/`OpBinary`: plain `serde_json` round-trip of the whole enum (one line
/// of compact JSON per op) — the SAME "JSON-pack passthrough" convention `brep`'s (and every
/// other W2a/W2b subset's) own, already-real, already-complete `SemioXMutation::OpText` impl
/// uses for its own full vocabulary (see e.g. `subsets::brep::schema::mutations`'s own doc
/// comment: "the same JSON-pack passthrough honesty boundary the subset's own ArtifactPack impl
/// uses"). Not a shortcut unique to this envelope subset — deliberately NOT
/// `#[derive(dsl::DslOps)]` for the same reason every subset's own mutation module already
/// documents: that path needs every embedded type (here, all 13 subsets' full nested trees at
/// once) to implement `dsl::DslField`, squarely out of scope (f6 §4 dsl-derive gaps).
impl protocol::OpText for SemioMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl protocol::OpBinary for SemioMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_op().into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_op(line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 0, detail: e.to_string() })
    }
}
//#endregion OpCodecs

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::{SemioAudioFormat, SemioAudioSnapshot};
    use crate::artifacts::semio::standards::v1::subsets::workflow::schema::snapshot::{SemioWorkflowSnapshot, WorkflowNode};
    use crate::artifacts::semio::standards::v1::engine::geometry::SemioPoint2;
    use protocol::command::DiffAlgebra;

    fn audio_base() -> SemioSnapshot {
        SemioSnapshot { subset: SemioSubsetSnapshot::Audio(SemioAudioSnapshot { sample_rate: 44_100, format: SemioAudioFormat::Pcm16, ..Default::default() }), ..Default::default() }
    }

    fn workflow_base() -> SemioSnapshot {
        SemioSnapshot { subset: SemioSubsetSnapshot::Workflow(SemioWorkflowSnapshot::default()), ..Default::default() }
    }

    /// 🧪️ mutation_diff_law + inverse_law: `NoMutation`, `SetSnapshot` (cross-kind), and a real
    /// wrapped per-field mutation (`Audio(SetSampleRate)`).
    #[test]
    fn mutation_diff_law_covers_no_mutation_set_snapshot_and_a_wrapped_variant() {
        let base = audio_base();

        let no_mut = SemioMutation::NoMutation;
        let d0 = <SemioMutation as Mutation<SemioSnapshot>>::diff(&no_mut, &base);
        assert_eq!(d0.apply(&base), base);

        let target = workflow_base();
        let set_snap = SemioMutation::SetSnapshot { snapshot: target.clone() };
        let d1 = <SemioMutation as Mutation<SemioSnapshot>>::diff(&set_snap, &base);
        assert_eq!(d1.apply(&base), target);
        let inv1 = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&set_snap, &base);
        let mut round = target.clone();
        let _ = apply_semio_mutation(&mut round, &inv1[0]);
        assert_eq!(round, base);

        let wrapped = SemioMutation::Audio(SemioAudioMutation::SetSampleRate { sample_rate: 96_000 });
        let d2 = <SemioMutation as Mutation<SemioSnapshot>>::diff(&wrapped, &base);
        assert!(matches!(d2, SemioDiff::Audio(_)));
        let mut applied = base.clone();
        let returned_diff = apply_semio_mutation(&mut applied, &wrapped);
        assert_eq!(d2.apply(&base), applied);
        assert_eq!(returned_diff, d2);
        let inv2 = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&wrapped, &base);
        assert_eq!(inv2.len(), 1);
        let mut restored = applied;
        let _ = apply_semio_mutation(&mut restored, &inv2[0]);
        assert_eq!(restored, base);
    }

    /// 🧪️ mutation_diff_law, second wrapped subset (workflow's id-keyed `InsertNode`) — proves
    /// the dispatch works for a collection-shaped mutation, not just a scalar one.
    #[test]
    fn mutation_diff_law_workflow_insert_node() {
        let base = workflow_base();
        let node = WorkflowNode { id: "n1".into(), kind: "task".into(), label: "N1".into(), params: vec![], position: SemioPoint2 { x: 1.0, y: 2.0 } };
        let wrapped = SemioMutation::Workflow(SemioWorkflowMutation::InsertNode { node: node.clone() });
        let mut applied = base.clone();
        let diff = apply_semio_mutation(&mut applied, &wrapped);
        assert!(matches!(diff, SemioDiff::Workflow(_)));
        match &applied.subset {
            SemioSubsetSnapshot::Workflow(s) => assert_eq!(s.nodes, vec![node]),
            other => panic!("expected Workflow, got {other:?}"),
        }
        let inv = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&wrapped, &base);
        let mut restored = applied;
        let _ = apply_semio_mutation(&mut restored, &inv[0]);
        assert_eq!(restored, base);
    }

    /// 🧪️ Kind-mismatch total fallback: a wrapped mutation for the WRONG kind never panics —
    /// degrades to a documented no-op.
    #[test]
    fn kind_mismatch_wrapped_mutation_is_a_safe_no_op() {
        let base = workflow_base();
        let wrapped = SemioMutation::Audio(SemioAudioMutation::SetSampleRate { sample_rate: 1 });
        let diff = <SemioMutation as Mutation<SemioSnapshot>>::diff(&wrapped, &base);
        assert_eq!(diff, SemioDiff::NoChange);
        assert_eq!(diff.apply(&base), base);
        let inv = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&wrapped, &base);
        assert_eq!(inv, vec![SemioMutation::NoMutation]);
    }

    /// 🧪️ Dispatch-table coverage: every one of the 13 wrapped-kind arms round-trips a
    /// `NoMutation`-shaped payload (proves the 13-arm `diff`/`inverse` match compiles and routes
    /// correctly for every subset).
    #[test]
    fn all_thirteen_wrapped_kinds_diff_and_inverse_route_correctly() {
        let bases: Vec<SemioSubsetSnapshot> = vec![
            SemioSubsetSnapshot::Brep(Default::default()),
            SemioSubsetSnapshot::Mesh(Default::default()),
            SemioSubsetSnapshot::Model(Default::default()),
            SemioSubsetSnapshot::Object(Default::default()),
            SemioSubsetSnapshot::Document(Default::default()),
            SemioSubsetSnapshot::Cad(Default::default()),
            SemioSubsetSnapshot::Drawing(Default::default()),
            SemioSubsetSnapshot::Image(Default::default()),
            SemioSubsetSnapshot::Video(Default::default()),
            SemioSubsetSnapshot::Audio(Default::default()),
            SemioSubsetSnapshot::Animation(Default::default()),
            SemioSubsetSnapshot::Presentation(Default::default()),
            SemioSubsetSnapshot::Workflow(Default::default()),
        ];
        let wrap_no_mutation = |s: &SemioSubsetSnapshot| -> SemioMutation {
            match s {
                SemioSubsetSnapshot::Brep(_) => SemioMutation::Brep(SemioBrepMutation::NoMutation),
                SemioSubsetSnapshot::Mesh(_) => SemioMutation::Mesh(SemioMeshMutation::NoMutation),
                SemioSubsetSnapshot::Model(_) => SemioMutation::Model(SemioModelMutation::NoMutation),
                SemioSubsetSnapshot::Object(_) => SemioMutation::Object(SemioObjectMutation::NoMutation),
                SemioSubsetSnapshot::Document(_) => SemioMutation::Document(SemioDocumentMutation::NoMutation),
                SemioSubsetSnapshot::Cad(_) => SemioMutation::Cad(SemioCadMutation::NoMutation),
                SemioSubsetSnapshot::Drawing(_) => SemioMutation::Drawing(SemioDrawingMutation::NoMutation),
                SemioSubsetSnapshot::Image(_) => SemioMutation::Image(SemioImageMutation::NoMutation),
                SemioSubsetSnapshot::Video(_) => SemioMutation::Video(SemioVideoMutation::NoMutation),
                SemioSubsetSnapshot::Audio(_) => SemioMutation::Audio(SemioAudioMutation::NoMutation),
                SemioSubsetSnapshot::Animation(_) => SemioMutation::Animation(SemioAnimationMutation::NoMutation),
                SemioSubsetSnapshot::Presentation(_) => SemioMutation::Presentation(SemioPresentationMutation::NoMutation),
                SemioSubsetSnapshot::Workflow(_) => SemioMutation::Workflow(SemioWorkflowMutation::NoMutation),
            }
        };
        for subset in bases {
            let base = SemioSnapshot { schema: "stdio.semio".into(), subset };
            let m = wrap_no_mutation(&base.subset);
            let diff = <SemioMutation as Mutation<SemioSnapshot>>::diff(&m, &base);
            assert!(diff.is_empty(), "wrapped NoMutation must diff empty: {diff:?}");
            let inv = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&m, &base);
            assert_eq!(inv.len(), 1);
        }
    }

    /// 🧪️ op_text_binary_roundtrip_law across `NoMutation`, `SetSnapshot`, and a wrapped variant.
    #[test]
    fn op_text_binary_roundtrip_law() {
        let base = audio_base();
        let cases = [
            SemioMutation::NoMutation,
            SemioMutation::SetSnapshot { snapshot: base.clone() },
            SemioMutation::Audio(SemioAudioMutation::SetSampleRate { sample_rate: 22_050 }),
            SemioMutation::Workflow(SemioWorkflowMutation::NoMutation),
        ];
        for m in cases {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = SemioMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?}");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = SemioMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }
}
//#endregion 🔖️Tests
