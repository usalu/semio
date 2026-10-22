//! 🔺️ Diff for `ApplyVideo`.

#[allow(unused_imports)]
use protocol::{ColdMutation, Mutation};
#[allow(unused_imports)]
use crate::schema::diff::SemioDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSubsetSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_video_mutations::schema::mutations::SemioVideoMutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_video_model::schema::snapshot::SemioVideoSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::ApplyVideo, base: &SemioSnapshot) -> protocol::MutationOutcome<SemioDiff> {
    match &base.subset {
        SemioSubsetSnapshot::Video(b) => <SemioVideoMutation as ColdMutation<SemioVideoSnapshot>>::diff(&payload.mutation, b).map(SemioDiff::Video),
        _ => protocol::MutationOutcome::error("mutation.target-missing", "Mutation subset does not match the snapshot subset.", ["subset"]),
    }
}
//#endregion 🔖️Diff
