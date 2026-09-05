//! 🔺️ Diff for `MoveObject`.

use crate::artifacts::semio::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::MoveObject, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<SemioObjectDiff> {
    let t = payload.translation;
    if !t.x.is_finite() || !t.y.is_finite() || !t.z.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Object translation has a non-finite component.".to_string(), ["transform".to_string()]);
    }
    if base.transform.translation == t {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Object is already at this translation.".to_string());
    }
    let mut transform = base.transform.clone();
    transform.translation = t;
    protocol::MutationOutcome::new(SemioObjectDiff { transform: Some(transform), ..Default::default() })
}
//#endregion 🔖️Diff
