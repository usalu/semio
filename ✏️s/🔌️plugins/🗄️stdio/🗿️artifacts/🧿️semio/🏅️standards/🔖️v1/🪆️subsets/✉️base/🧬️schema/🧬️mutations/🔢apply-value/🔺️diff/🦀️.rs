//! 🔺️ Diff for `ApplyValue`.

#[allow(unused_imports)]
use protocol::{ColdMutation, Mutation};
#[allow(unused_imports)]
use crate::schema::diff::SemioDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSubsetSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_value_mutations::schema::mutations::SemioValueMutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::ApplyValue, base: &SemioSnapshot) -> protocol::MutationOutcome<SemioDiff> {
    match &base.subset {
        SemioSubsetSnapshot::Value(b) => <SemioValueMutation as ColdMutation<SemioValueSnapshot>>::diff(&payload.mutation, b).map(SemioDiff::Value),
        _ => protocol::MutationOutcome::error("mutation.target-missing", "Mutation subset does not match the snapshot subset.", ["subset"]),
    }
}
//#endregion 🔖️Diff
