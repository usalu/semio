//! 🔺️ `edit-run` — sparse diff construction; an out-of-range BASE index is `mutation.target-missing`,
//! and setting content equal to the current value is `mutation.no-op`.

use super::mutation::EditRun;
use crate::artifacts::semio::standards::v1::subsets::text::schema::diff::{SemioTextDiff, SemioTextRunList};
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &EditRun, base: &SemioTextSnapshot) -> protocol::MutationOutcome<SemioTextDiff> {
    let Some(existing) = base.runs.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Run #{} does not exist.", payload.index), [payload.index.to_string()]);
    };
    if existing.content == payload.new_content {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Run #{} content is already \"{}\".", payload.index, payload.new_content));
    }
    let mut runs = base.runs.clone();
    runs[payload.index].content = payload.new_content.clone();
    protocol::MutationOutcome::new(SemioTextDiff { runs: Some(SemioTextRunList { values: runs }) })
}
//#endregion 🔖️Diff
