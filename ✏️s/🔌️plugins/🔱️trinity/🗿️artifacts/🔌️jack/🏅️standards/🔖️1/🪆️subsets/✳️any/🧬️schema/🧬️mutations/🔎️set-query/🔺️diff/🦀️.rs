//! 🔺️ Sparse diff builder for `SetQuery` — only the `query` slot moves.
use crate::standards::v1::subsets::any::schema::diff::JackDiff;
use crate::JackSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::SetQuery, base: &JackSnapshot) -> protocol::MutationOutcome<JackDiff> {
    if base.query == payload.value {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "The Jack query already reads this text.");
    }
    protocol::MutationOutcome::new(JackDiff { query: Some(payload.value.clone()), ..JackDiff::default() })
}
//#endregion 🔖️Diff
