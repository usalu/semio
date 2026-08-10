//! 🔺️ GifDiff (89a) — sparse diff extended with frame-level ops. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: beyond the universal
//! `{NoMutation, SetSnapshot}` stub, 89a's mutation vocabulary needs its own diff shape per op so
//! each mutation only touches what it actually changed (a full-snapshot replace-diff would still
//! be *correct* but throws away the point of having a typed vocabulary).

use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::{GifDisposal, GifFrame, GifSnapshot};
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️OpPayloads
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameInsert {
    pub index: usize,
    pub frame: GifFrame,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameDelay {
    pub index: usize,
    pub delay_cs: u16,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoopCountChange {
    pub loop_count: Option<u16>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameDisposalChange {
    pub index: usize,
    pub disposal: GifDisposal,
}
//#endregion 🔖️OpPayloads

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.gif.89a`. Exactly one field is populated per mutation (`snapshot` for
/// `SetSnapshot`, one op field for everything else) — `apply` checks `snapshot` first as the
/// full-replace fast path, then applies whichever single op field is present.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gif.89a.diff")]
pub struct GifDiff {
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<GifSnapshot>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insert_frame: Option<FrameInsert>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_frame_at: Option<usize>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_frame_delay: Option<FrameDelay>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_loop_count: Option<LoopCountChange>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_frame_disposal: Option<FrameDisposalChange>,
}

impl MutationDiff<GifSnapshot> for GifDiff {
    fn apply(&self, base: &GifSnapshot) -> GifSnapshot {
        if let Some(snapshot) = &self.snapshot {
            return snapshot.clone();
        }
        let mut next = base.clone();
        if let Some(FrameInsert { index, frame }) = &self.insert_frame {
            let at = (*index).min(next.frames.len());
            next.frames.insert(at, frame.clone());
        }
        if let Some(index) = self.remove_frame_at {
            if index < next.frames.len() {
                next.frames.remove(index);
            }
        }
        if let Some(FrameDelay { index, delay_cs }) = &self.set_frame_delay {
            if let Some(frame) = next.frames.get_mut(*index) {
                frame.delay_cs = *delay_cs;
            }
        }
        if let Some(LoopCountChange { loop_count }) = &self.set_loop_count {
            next.loop_count = *loop_count;
        }
        if let Some(FrameDisposalChange { index, disposal }) = &self.set_frame_disposal {
            if let Some(frame) = next.frames.get_mut(*index) {
                frame.disposal = *disposal;
            }
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            self.snapshot = other.snapshot;
        }
        if other.insert_frame.is_some() {
            self.insert_frame = other.insert_frame;
        }
        if other.remove_frame_at.is_some() {
            self.remove_frame_at = other.remove_frame_at;
        }
        if other.set_frame_delay.is_some() {
            self.set_frame_delay = other.set_frame_delay;
        }
        if other.set_loop_count.is_some() {
            self.set_loop_count = other.set_loop_count;
        }
        if other.set_frame_disposal.is_some() {
            self.set_frame_disposal = other.set_frame_disposal;
        }
    }
}

pub fn diff_set_snapshot(snapshot: &GifSnapshot) -> GifDiff {
    GifDiff { snapshot: Some(snapshot.clone()), ..Default::default() }
}
pub fn diff_insert_frame(index: usize, frame: GifFrame) -> GifDiff {
    GifDiff { insert_frame: Some(FrameInsert { index, frame }), ..Default::default() }
}
pub fn diff_remove_frame(index: usize) -> GifDiff {
    GifDiff { remove_frame_at: Some(index), ..Default::default() }
}
pub fn diff_set_frame_delay(index: usize, delay_cs: u16) -> GifDiff {
    GifDiff { set_frame_delay: Some(FrameDelay { index, delay_cs }), ..Default::default() }
}
pub fn diff_set_loop_count(loop_count: Option<u16>) -> GifDiff {
    GifDiff { set_loop_count: Some(LoopCountChange { loop_count }), ..Default::default() }
}
pub fn diff_set_frame_disposal(index: usize, disposal: GifDisposal) -> GifDiff {
    GifDiff { set_frame_disposal: Some(FrameDisposalChange { index, disposal }), ..Default::default() }
}
//#endregion 🔖️Diff
