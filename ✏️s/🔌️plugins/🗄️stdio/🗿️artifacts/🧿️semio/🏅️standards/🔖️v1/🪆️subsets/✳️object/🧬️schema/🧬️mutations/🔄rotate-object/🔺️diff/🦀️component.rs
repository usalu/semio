//! 🔺️ `rotate-object` — sparse diff construction: sets `transform.rotation`, keeping
//! translation/scale. A non-finite `rotation` component is `mutation.invariant` (Fatal, empty
//! diff); a `rotation` identical to the object's current rotation is `mutation.no-op` (Warning,
//! empty diff).

use super::mutation::RotateObject;
use crate::artifacts::semio::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &RotateObject, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<SemioObjectDiff> {
    let r = payload.rotation;
    if !r.x.is_finite() || !r.y.is_finite() || !r.z.is_finite() || !r.w.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Object rotation has a non-finite component.".to_string(), ["transform".to_string()]).await;
    }
    if base.transform.rotation == r {
        return protocol::MutationOutcome::empty().await.warn("mutation.no-op", "Object is already at this rotation.".to_string()).await;
    }
    let mut transform = base.transform.clone();
    transform.rotation = r;
    protocol::MutationOutcome::new(SemioObjectDiff { transform: Some(transform), ..Default::default() }).await
}
//#endregion 🔖️Diff
