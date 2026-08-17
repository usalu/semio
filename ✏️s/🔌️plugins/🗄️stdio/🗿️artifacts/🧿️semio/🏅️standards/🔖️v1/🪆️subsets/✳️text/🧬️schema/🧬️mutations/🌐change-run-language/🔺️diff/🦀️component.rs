//! 🔺️ `change-run-language` — sparse diff construction; an out-of-range BASE index is
//! `mutation.target-missing`, and a language already matching the run is `mutation.no-op`.

use super::mutation::ChangeRunLanguage;
use crate::artifacts::semio::standards::v1::subsets::text::schema::diff::{SemioTextDiff, SemioTextRunList};
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeRunLanguage, base: &SemioTextSnapshot) -> protocol::MutationOutcome<SemioTextDiff> {
    let Some(existing) = base.runs.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Run #{} does not exist.", payload.index), [payload.index.to_string()]);
    };
    if existing.language == payload.new_language {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Run #{} language is already \"{}\".", payload.index, payload.new_language));
    }
    let mut runs = base.runs.clone();
    runs[payload.index].language = payload.new_language.clone();
    protocol::MutationOutcome::new(SemioTextDiff { runs: Some(SemioTextRunList { values: runs }) })
}
//#endregion 🔖️Diff
