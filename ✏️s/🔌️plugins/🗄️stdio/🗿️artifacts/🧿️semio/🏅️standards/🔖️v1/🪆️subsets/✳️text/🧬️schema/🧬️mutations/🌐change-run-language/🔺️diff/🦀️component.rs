//! 🔺️ `change-run-language` — sparse diff construction; an out-of-range BASE index is a no-op
//! clone.

use super::mutation::ChangeRunLanguage;
use crate::artifacts::semio::standards::v1::subsets::text::schema::diff::{SemioTextDiff, SemioTextRunList};
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeRunLanguage, base: &SemioTextSnapshot) -> SemioTextDiff {
    let mut runs = base.runs.clone();
    if let Some(run) = runs.get_mut(payload.index) {
        run.language = payload.new_language.clone();
    }
    SemioTextDiff { runs: Some(SemioTextRunList { values: runs }) }
}
//#endregion 🔖️Diff
