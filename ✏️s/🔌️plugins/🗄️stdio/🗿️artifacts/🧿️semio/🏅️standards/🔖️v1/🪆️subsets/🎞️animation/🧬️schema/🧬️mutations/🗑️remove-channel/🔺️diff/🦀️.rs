//! 🔺️ Diff for `RemoveChannel`.

#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::schema::triples::IndexedTripleDiff;
#[allow(unused_imports)]
use crate::schema::diff::SemioAnimationDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioAnimationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::channel_at;
#[allow(unused_imports)]
use crate::schema::mutations::support::diff_channel_collection;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveChannel, base: &SemioAnimationSnapshot) -> protocol::MutationOutcome<SemioAnimationDiff> {
    if channel_at(base, payload.timeline_index, payload.index).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No channel exists at index #{}.", payload.index), [payload.index.to_string()]);
    }
    
    let super::RemoveChannel { timeline_index, index } = payload;
    protocol::MutationOutcome::new(diff_channel_collection(*timeline_index, IndexedTripleDiff { removed: vec![*index], ..Default::default() }))
}
//#endregion 🔖️Diff
