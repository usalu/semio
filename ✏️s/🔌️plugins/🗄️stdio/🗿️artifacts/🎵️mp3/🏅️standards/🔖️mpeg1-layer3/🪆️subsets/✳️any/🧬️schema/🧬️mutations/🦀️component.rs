//! 🧬️ Mp3Mutation — the real per-field mutation vocabulary over `Mp3Snapshot`'s three
//! top-level fields (`id3v2`/`frames`/`id3v1`), plus `SetSnapshot` for full replace.

use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::diff::{diff_set_frames, diff_set_id3v1, diff_set_id3v2, diff_set_snapshot, Mp3Diff};
use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Id3v1Tag, Id3v2Tag, Mp3Frame, Mp3Snapshot};
use protocol::Mutation;
use protocol::{OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum Mp3Mutation {
    #[default]
    NoMutation,
    /// 🔁️ Full-snapshot replace.
    SetSnapshot { snapshot: Mp3Snapshot },
    /// 🏷️ Sets (`Some`) or clears (`None`) the ID3v2 tag wholesale.
    SetId3v2 { id3v2: Option<Id3v2Tag> },
    /// 🎼️ Replaces the MPEG frame sequence wholesale.
    SetFrames { frames: Vec<Mp3Frame> },
    /// 🏷️ Sets (`Some`) or clears (`None`) the ID3v1 trailer wholesale.
    SetId3v1 { id3v1: Option<Id3v1Tag> },
}

impl Mutation<Mp3Snapshot> for Mp3Mutation {
    type Diff = Mp3Diff;

    async fn diff(&self, base: &Mp3Snapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            Mp3Mutation::NoMutation => Mp3Diff::default(),
            Mp3Mutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            Mp3Mutation::SetId3v2 { id3v2 } => diff_set_id3v2(id3v2.clone()),
            Mp3Mutation::SetFrames { frames } => diff_set_frames(frames.clone()),
            Mp3Mutation::SetId3v1 { id3v1 } => diff_set_id3v1(id3v1.clone()),
        }).await
    }

    async fn inverse(&self, base: &Mp3Snapshot) -> Vec<Self> {
        match self {
            Mp3Mutation::NoMutation => vec![Mp3Mutation::NoMutation],
            Mp3Mutation::SetSnapshot { .. } => vec![Mp3Mutation::SetSnapshot { snapshot: base.clone() }],
            Mp3Mutation::SetId3v2 { .. } => vec![Mp3Mutation::SetId3v2 { id3v2: base.id3v2.clone() }],
            Mp3Mutation::SetFrames { .. } => vec![Mp3Mutation::SetFrames { frames: base.frames.clone() }],
            Mp3Mutation::SetId3v1 { .. } => vec![Mp3Mutation::SetId3v1 { id3v1: base.id3v1.clone() }],
        }
    }
}

/// ▶️ Applies a mutation to `snapshot` in place, returning the diff (the diff is the single
/// semantics source — never apply-and-capture).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_mp3_mutation(snapshot: &mut Mp3Snapshot, mutation: &Mp3Mutation) -> protocol::MutationOutcome<Mp3Diff> {
    let outcome = <Mp3Mutation as Mutation<Mp3Snapshot>>::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Mutation

//#region OpCodecs
/// 🎙️ Handcrafted `OpText`/`OpBinary` via plain `serde_json` (one line of compact JSON per op) —
/// deliberately NOT `#[derive(dsl::DslOps)]`: `Mp3Frame`/`Id3v2Tag` embed nested collections of
/// named structs, the same generic-collection-diff shape `f6-final-summary.md` §4.4 documents as
/// needing a hand-rolled bridge. This is a SEPARATE wire format from the subset's own
/// `ArtifactDsl`/`ArtifactPack` envelope (which wraps real MP3 bytes, see that file's doc
/// comment) — an op is always plain JSON here.
impl OpText for Mp3Mutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
    }
    async fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl OpBinary for Mp3Mutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|e| protocol::ProtocolError::Io(e.to_string()))
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|e| protocol::ProtocolError::Io(e.to_string()))
    }
}
//#endregion OpCodecs

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Id3Frame, Mp3FrameHeader};
    use protocol::command::DiffAlgebra;
    use protocol::MutationDiff;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn frame() -> Mp3Frame {
        Mp3Frame {
            header: Mp3FrameHeader { mpeg_version_id: 3, layer: 1, protection_bit: true, bitrate_index: 9, sample_rate_index: 0, padding: false, private_bit: false, channel_mode: 3, mode_extension: 0, copyright: false, original: true, emphasis: 0 },
            payload: vec![0u8; 4],
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn base_snapshot() -> Mp3Snapshot {
        Mp3Snapshot { frames: vec![frame()], ..Mp3Snapshot::default() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn variants(base: &Mp3Snapshot) -> Vec<Mp3Mutation> {
        vec![
            Mp3Mutation::NoMutation,
            Mp3Mutation::SetSnapshot { snapshot: Mp3Snapshot { frames: vec![frame(), frame()], ..base.clone() } },
            Mp3Mutation::SetId3v2 { id3v2: Some(Id3v2Tag { major_version: 3, minor_version: 0, flags: 0, frames: vec![Id3Frame { id: "TIT2".into(), flags: 0, data: vec![0] }] }) },
            Mp3Mutation::SetId3v2 { id3v2: None },
            Mp3Mutation::SetFrames { frames: vec![frame(), frame(), frame()] },
            Mp3Mutation::SetId3v1 { id3v1: Some(Id3v1Tag { raw: vec![b'T', b'A', b'G'] }) },
            Mp3Mutation::SetId3v1 { id3v1: None },
        ]
    }

    //#region mutation_diff_law
    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law_every_variant() {
        let base = base_snapshot();
        for m in variants(&base) {
            let mut via_apply = base.clone();
            let returned = apply_mp3_mutation(&mut via_apply, &m);
            let direct = m.diff(&base);
            assert_eq!(direct, returned, "diff mismatch for {m:?}");
            assert_eq!(direct.diff().apply(&base).unwrap(), via_apply, "apply mismatch for {m:?}");
        }
    }
    //#endregion mutation_diff_law

    //#region inverse_law
    #[semio_framework_async_macros::async_test]
    async fn inverse_law_mutation_and_diff_level() {
        let base = base_snapshot();
        for m in variants(&base) {
            let mut round = base.clone();
            apply_mp3_mutation(&mut round, &m);
            for inv in m.inverse(&base) {
                apply_mp3_mutation(&mut round, &inv);
            }
            assert_eq!(round, base, "mutation-level inverse failed for {m:?}");

            let d = m.diff(&base);
            let applied = d.diff().apply(&base).unwrap();
            let undone = d.diff().inverse(&base).apply(&applied).unwrap();
            assert_eq!(undone, base, "diff-level inverse failed for {m:?}");
        }
    }
    //#endregion inverse_law

    //#region op_text_binary_roundtrip_law
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        let base = base_snapshot();
        for m in variants(&base) {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = Mp3Mutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?}");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = Mp3Mutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }
    //#endregion op_text_binary_roundtrip_law
}
//#endregion 🔖️Tests
