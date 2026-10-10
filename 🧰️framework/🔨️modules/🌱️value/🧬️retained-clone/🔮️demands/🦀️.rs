//! 🔮️ A normal clone consumer requires exact borrowed effects from its actual selected original child.

use super::{RetainedClone,RetainedCloneCursor,RetainedCloneRef};
use crate::{RetirementDemand,ValueError};

/// 🎟️ Declares normal effects without granting work, binding a lease or transferring original custody.
pub trait RetainedCloneDemandCursor<T:RetainedClone>:RetainedCloneCursor<T> {
    fn normal_demands(&self,source:RetainedCloneRef<'_,T>,maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError>;
}
