//! 🧬️ GifMutation (89a) — document mutation dispatch. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: the mutation vocabulary
//! beyond the universal `{NoMutation, SetSnapshot}` stub, real `apply` + `inverse` for each.
//! 87a intentionally keeps only `{NoMutation, SetSnapshot}` — it has no frames/loop concept to
//! mutate incrementally.

use crate::artifacts::gif::standards::v89a::subsets::any::schema::diff::{self, GifDiff};
use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::{GifDisposal, GifFrame, GifSnapshot};
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.gif.89a`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum GifMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: GifSnapshot,
    },
    InsertFrame {
        index: usize,
        frame: GifFrame,
    },
    RemoveFrame {
        index: usize,
    },
    SetFrameDelay {
        index: usize,
        delay_cs: u16,
    },
    SetLoopCount {
        loop_count: Option<u16>,
    },
    SetFrameDisposal {
        index: usize,
        disposal: GifDisposal,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. Out-of-range frame indices are no-ops rather than panics
/// -- a stale index (e.g. from a concurrent edit) should degrade gracefully, not crash the engine.
pub fn apply_gif_mutation(snapshot: &mut GifSnapshot, mutation: &GifMutation) -> GifDiff {
    let __diff = <GifMutation as protocol::Mutation<GifSnapshot>>::diff(mutation, snapshot);
    match mutation {
        GifMutation::NoMutation => {}
        GifMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
        GifMutation::InsertFrame { index, frame } => {
            let at = (*index).min(snapshot.frames.len());
            snapshot.frames.insert(at, frame.clone());
        }
        GifMutation::RemoveFrame { index } => {
            if *index < snapshot.frames.len() {
                snapshot.frames.remove(*index);
            }
        }
        GifMutation::SetFrameDelay { index, delay_cs } => {
            if let Some(frame) = snapshot.frames.get_mut(*index) {
                frame.delay_cs = *delay_cs;
            }
        }
        GifMutation::SetLoopCount { loop_count } => {
            snapshot.loop_count = *loop_count;
        }
        GifMutation::SetFrameDisposal { index, disposal } => {
            if let Some(frame) = snapshot.frames.get_mut(*index) {
                frame.disposal = *disposal;
            }
        }
    }

    __diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<GifSnapshot> for GifMutation {
    type Diff = GifDiff;

    fn diff(&self, _base: &GifSnapshot) -> Self::Diff {
        match self {
            GifMutation::NoMutation => GifDiff::default(),
            GifMutation::SetSnapshot { snapshot } => diff::diff_set_snapshot(snapshot),
            GifMutation::InsertFrame { index, frame } => diff::diff_insert_frame(*index, frame.clone()),
            GifMutation::RemoveFrame { index } => diff::diff_remove_frame(*index),
            GifMutation::SetFrameDelay { index, delay_cs } => diff::diff_set_frame_delay(*index, *delay_cs),
            GifMutation::SetLoopCount { loop_count } => diff::diff_set_loop_count(*loop_count),
            GifMutation::SetFrameDisposal { index, disposal } => diff::diff_set_frame_disposal(*index, *disposal),
        }
    }

    /// ↩️ Real, round-trippable inverses: `apply(inverse(m, base), apply(m, base)) == base` for
    /// every variant, including the frame-index ops (proven by
    /// `mutation_apply_inverse_round_trips_every_variant` in the engine's test module).
    fn inverse(&self, base: &GifSnapshot) -> Vec<Self> {
        match self {
            GifMutation::NoMutation => vec![GifMutation::NoMutation],
            GifMutation::SetSnapshot { .. } => vec![GifMutation::SetSnapshot { snapshot: base.clone() }],
            GifMutation::InsertFrame { index, .. } => vec![GifMutation::RemoveFrame { index: *index }],
            GifMutation::RemoveFrame { index } => match base.frames.get(*index) {
                Some(frame) => vec![GifMutation::InsertFrame { index: *index, frame: frame.clone() }],
                None => vec![GifMutation::NoMutation],
            },
            GifMutation::SetFrameDelay { index, .. } => {
                let prior = base.frames.get(*index).map(|f| f.delay_cs).unwrap_or_default();
                vec![GifMutation::SetFrameDelay { index: *index, delay_cs: prior }]
            }
            GifMutation::SetLoopCount { .. } => vec![GifMutation::SetLoopCount { loop_count: base.loop_count }],
            GifMutation::SetFrameDisposal { index, .. } => {
                let prior = base.frames.get(*index).map(|f| f.disposal).unwrap_or_default();
                vec![GifMutation::SetFrameDisposal { index: *index, disposal: prior }]
            }
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
impl protocol::OpText for GifMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for GifMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|e| protocol::ProtocolError::Malformed {
            what: "op encode",
            offset: 0,
            detail: e.to_string(),
        })
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|e| protocol::ProtocolError::Malformed {
            what: "op decode",
            offset: 0,
            detail: e.to_string(),
        })
    }
}
//#endregion OpCodecs

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationDiff;

    fn sample_frame(seed: u8) -> GifFrame {
        GifFrame {
            left: 0,
            top: 0,
            width: 2,
            height: 2,
            rgba: vec![seed; 16],
            delay_cs: 10,
            disposal: GifDisposal::DoNotDispose,
            transparent: false,
            user_input: false,
        }
    }

    fn base_snapshot() -> GifSnapshot {
        GifSnapshot {
            schema: "stdio.gif.89a".into(),
            width: 2,
            height: 2,
            loop_count: Some(0),
            frames: vec![sample_frame(1), sample_frame(2), sample_frame(3)],
        }
    }

    fn round_trips(base: &GifSnapshot, mutation: GifMutation) {
        let diff = mutation.diff(base);
        let mutated = diff.apply(base);
        let inverses = mutation.inverse(base);
        let mut restored = mutated.clone();
        for inv in &inverses {
            let inv_diff = inv.diff(&restored);
            restored = inv_diff.apply(&restored);
        }
        assert_eq!(&restored, base, "apply(inverse(m), apply(m, base)) must recover base for {mutation:?}");
    }

    #[test]
    fn mutation_apply_inverse_round_trips_every_variant() {
        let base = base_snapshot();
        round_trips(&base, GifMutation::NoMutation);
        round_trips(&base, GifMutation::SetSnapshot { snapshot: GifSnapshot { loop_count: Some(5), ..base.clone() } });
        round_trips(&base, GifMutation::InsertFrame { index: 1, frame: sample_frame(9) });
        round_trips(&base, GifMutation::RemoveFrame { index: 1 });
        round_trips(&base, GifMutation::SetFrameDelay { index: 0, delay_cs: 42 });
        round_trips(&base, GifMutation::SetLoopCount { loop_count: Some(3) });
        round_trips(&base, GifMutation::SetLoopCount { loop_count: None });
        round_trips(&base, GifMutation::SetFrameDisposal { index: 2, disposal: GifDisposal::RestoreToBackground });
    }

    #[test]
    fn remove_frame_out_of_range_is_noop_not_panic() {
        let base = base_snapshot();
        let mut snap = base.clone();
        apply_gif_mutation(&mut snap, &GifMutation::RemoveFrame { index: 99 });
        assert_eq!(snap, base);
    }
}
//#endregion Tests
