//! 🔺️ Diff for `ApplyObject`.

#[allow(unused_imports)]
use protocol::{ColdMutation, Mutation};
#[allow(unused_imports)]
use crate::schema::diff::SemioDiff;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_object::schema::mutations::SemioObjectMutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_object::schema::snapshot::SemioObjectSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSubsetSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::ApplyObject, base: &SemioSnapshot) -> protocol::MutationOutcome<SemioDiff> {
    match &base.subset {
        SemioSubsetSnapshot::Object(b) => <SemioObjectMutation as ColdMutation<SemioObjectSnapshot>>::diff(&payload.mutation, b).map(SemioDiff::Object),
        _ => protocol::MutationOutcome::error("mutation.target-missing", "Mutation subset does not match the snapshot subset.", ["subset"]),
    }
}
//#endregion 🔖️Diff
