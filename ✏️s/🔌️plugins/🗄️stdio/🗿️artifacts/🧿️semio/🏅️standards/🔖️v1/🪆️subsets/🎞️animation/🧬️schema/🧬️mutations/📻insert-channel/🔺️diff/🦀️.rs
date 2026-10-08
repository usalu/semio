//! 🔺️ Diff for `InsertChannel`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertChannel, base: &SemioAnimationSnapshot) -> protocol::MutationOutcome<SemioAnimationDiff> {
    if timeline_at(base, payload.timeline_index).is_none_or(|t| payload.index > t.channels.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No channel slot exists at index #{}.", payload.index), [payload.index.to_string()]);
    }
    use SemioAnimationMutation::*;
    let super::InsertChannel { timeline_index, index, channel } = payload;
    protocol::MutationOutcome::new(diff_channel_collection(*timeline_index, IndexedTripleDiff { added: vec![IndexAdded { index: *index, item: channel.clone() }], ..Default::default() }))
}
//#endregion 🔖️Diff
