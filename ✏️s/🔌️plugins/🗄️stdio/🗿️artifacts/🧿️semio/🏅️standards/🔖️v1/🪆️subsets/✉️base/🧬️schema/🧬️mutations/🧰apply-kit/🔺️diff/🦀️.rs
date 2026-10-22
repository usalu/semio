//! 🔺️ Diff for `ApplyKit`.

#[allow(unused_imports)]
use protocol::{ColdMutation, Mutation};
#[allow(unused_imports)]
use crate::schema::diff::SemioDiff;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_kit::schema::mutations::SemioKitMutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_kit::schema::snapshot::SemioKitSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSubsetSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::ApplyKit, base: &SemioSnapshot) -> protocol::MutationOutcome<SemioDiff> {
    match &base.subset {
        SemioSubsetSnapshot::Kit(b) => <SemioKitMutation as ColdMutation<SemioKitSnapshot>>::diff(&payload.mutation, b).map(SemioDiff::Kit),
        _ => protocol::MutationOutcome::error("mutation.target-missing", "Mutation subset does not match the snapshot subset.", ["subset"]),
    }
}
//#endregion 🔖️Diff
