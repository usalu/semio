//! 🔺️ `scale-object` — sparse diff construction: sets `transform.scale`, keeping
//! translation/rotation. A non-finite or non-positive `scale` component is `mutation.invariant`
//! (Fatal, empty diff — a zero/negative scale factor is degenerate); a `scale` identical to the
//! object's current scale is `mutation.no-op` (Warning, empty diff).

use super::mutation::ScaleObject;
use crate::artifacts::semio::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &ScaleObject, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<SemioObjectDiff> {
    let s = payload.scale;
    if !s.x.is_finite() || !s.y.is_finite() || !s.z.is_finite() || s.x <= 0.0 || s.y <= 0.0 || s.z <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Object scale must be finite and positive.".to_string(), ["transform".to_string()]);
    }
    if base.transform.scale == s {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Object is already at this scale.".to_string());
    }
    let mut transform = base.transform.clone();
    transform.scale = s;
    protocol::MutationOutcome::new(SemioObjectDiff { transform: Some(transform), ..Default::default() })
}
//#endregion 🔖️Diff
