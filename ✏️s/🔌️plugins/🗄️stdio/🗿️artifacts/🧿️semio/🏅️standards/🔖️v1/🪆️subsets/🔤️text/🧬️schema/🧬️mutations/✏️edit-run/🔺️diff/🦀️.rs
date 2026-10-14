//! 🔺️ Diff for `EditRun`.

use semio_s_artifact_stdio_semio_base::schema::triples::IndexModified;
use semio_s_artifact_stdio_semio_base::schema::triples::IndexedTripleDiff;
use semio_s_artifact_stdio_semio_text_model::schema::diff::SemioTextDiff;
use semio_s_artifact_stdio_semio_text_model::schema::diff::SemioTextRunDiff;
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Diff
/// 🧮️ Sets one run's `content`: a sparse `modified` row naming only the new content.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::EditRun, base: &SemioTextSnapshot) -> protocol::MutationOutcome<SemioTextDiff> {
    let Some(existing) = base.runs.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Run #{} does not exist.", payload.index), [payload.index.to_string()]);
    };
    if existing.content == payload.new_text {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Run #{} content is already \"{}\".", payload.index, payload.new_text));
    }
    protocol::MutationOutcome::new(SemioTextDiff { runs: Some(IndexedTripleDiff { modified: vec![IndexModified { index: payload.index, diff: SemioTextRunDiff { content: Some(payload.new_text.clone()), ..Default::default() } }], ..Default::default() }) })
}
//#endregion 🔖️Diff
