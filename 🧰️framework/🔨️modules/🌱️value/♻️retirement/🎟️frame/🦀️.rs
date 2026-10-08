//! 🎟️ One admitted typed frame receives the same original when its constructor grant is refused.
use crate::{ErasedSnapshotRetirement,ValueError};
/// 🎟️ Admits an allocation-free domain cursor constructor before transferring its original payload.
pub fn admit_retirement_frame<T, R: ErasedSnapshotRetirement + 'static>(value: T, grant: crate::retained_clone::RetainedCloneGrant, construct: impl FnOnce(T) -> R) -> Result<(Box<dyn ErasedSnapshotRetirement>, crate::retained_clone::RetainedCloneProgress), (ValueError, T)> {
    let bytes = std::mem::size_of::<R>();
    if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_capacity_bytes < bytes { return Err((ValueError::literal(crate::ValueRefusalKind::OwnershipLimit, "domain retirement requires its full cursor birth"), value)); }
    let owner = Box::new(construct(value));
    Ok((owner, crate::retained_clone::RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: bytes, ..Default::default() }))
}

