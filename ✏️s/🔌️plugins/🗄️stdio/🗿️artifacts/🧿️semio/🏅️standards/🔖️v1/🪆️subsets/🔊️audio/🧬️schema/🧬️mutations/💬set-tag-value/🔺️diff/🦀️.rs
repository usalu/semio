//! 🔺️ Diff for `SetTagValue`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetTagValue, base: &SemioAudioSnapshot) -> protocol::MutationOutcome<SemioAudioDiff> {
    if payload.index >= base.tags.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No tag exists at index #{}.", payload.index), [payload.index.to_string()]);
    }
    let super::SetTagValue { index, value } = payload;
    protocol::MutationOutcome::new(match base.tags.get(*index) {
        Some(t) => SemioAudioDiff { tags: Some(IndexedTripleDiff { modified: vec![IndexModified { index: *index, diff: SemioAudioTag { key: t.key.clone(), value: value.clone() } }], ..Default::default() }), ..Default::default() },
        None => SemioAudioDiff::default(),
    })
}
//#endregion 🔖️Diff
