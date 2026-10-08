//! 🔺️ Diff for `ChangeRunLanguage`.

use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
use crate::standards::v1::subsets::text::schema::diff::{SemioTextDiff, SemioTextRunDiff};
use crate::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Diff
/// 🧮️ Sets one run's `language`: a sparse `modified` row naming only the new language.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::ChangeRunLanguage, base: &SemioTextSnapshot) -> protocol::MutationOutcome<SemioTextDiff> {
    let Some(existing) = base.runs.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Run #{} does not exist.", payload.index), [payload.index.to_string()]);
    };
    if existing.language == payload.new_language {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Run #{} language is already \"{}\".", payload.index, payload.new_language));
    }
    protocol::MutationOutcome::new(SemioTextDiff { runs: Some(IndexedTripleDiff { modified: vec![IndexModified { index: payload.index, diff: SemioTextRunDiff { language: Some(payload.new_language.clone()), ..Default::default() } }], ..Default::default() }) })
}
//#endregion 🔖️Diff
