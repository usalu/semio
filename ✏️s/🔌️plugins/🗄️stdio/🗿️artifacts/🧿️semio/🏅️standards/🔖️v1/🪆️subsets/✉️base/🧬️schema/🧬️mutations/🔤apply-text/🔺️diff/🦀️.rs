//! 🔺️ Diff for `ApplyText`.

#[allow(unused_imports)]
use protocol::{ColdMutation, Mutation};
#[allow(unused_imports)]
use crate::schema::diff::SemioDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSubsetSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_text_mutations::schema::mutations::SemioTextMutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::ApplyText, base: &SemioSnapshot) -> protocol::MutationOutcome<SemioDiff> {
    match &base.subset {
        SemioSubsetSnapshot::Text(b) => <SemioTextMutation as ColdMutation<SemioTextSnapshot>>::diff(&payload.mutation, b).map(SemioDiff::Text),
        _ => protocol::MutationOutcome::error("mutation.target-missing", "Mutation subset does not match the snapshot subset.", ["subset"]),
    }
}
//#endregion 🔖️Diff
