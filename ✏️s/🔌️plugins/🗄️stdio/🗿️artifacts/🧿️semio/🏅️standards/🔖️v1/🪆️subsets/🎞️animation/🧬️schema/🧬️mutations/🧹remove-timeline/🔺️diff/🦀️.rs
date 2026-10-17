//! 🔺️ Diff for `RemoveTimeline`.

#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::schema::triples::IndexedTripleDiff;
#[allow(unused_imports)]
use crate::schema::diff::SemioAnimationDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioAnimationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::timeline_at;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveTimeline, base: &SemioAnimationSnapshot) -> protocol::MutationOutcome<SemioAnimationDiff> {
    if timeline_at(base, payload.index).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No timeline exists at index #{}.", payload.index), [payload.index.to_string()]);
    }
    
    let super::RemoveTimeline { index } = payload;
    protocol::MutationOutcome::new(SemioAnimationDiff { timelines: Some(IndexedTripleDiff { removed: vec![*index], ..Default::default() }) })
}
//#endregion 🔖️Diff
