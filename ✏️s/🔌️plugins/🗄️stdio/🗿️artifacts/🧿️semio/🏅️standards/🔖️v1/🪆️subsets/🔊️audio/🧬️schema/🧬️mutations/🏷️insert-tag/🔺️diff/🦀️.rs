//! 🔺️ Diff for `InsertTag`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertTag, base: &SemioAudioSnapshot) -> protocol::MutationOutcome<SemioAudioDiff> {
    let super::InsertTag { index, tag } = payload;
    protocol::MutationOutcome::new({
        SemioAudioDiff { tags: Some(IndexedTripleDiff { added: vec![IndexAdded { index: (*index).min(base.tags.len()), item: tag.clone() }], ..Default::default() }), ..Default::default() }
    })
}
//#endregion 🔖️Diff
