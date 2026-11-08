//! 🌳️ Persistent roots reuse the neutral native immutable projection and clone owner.
use crate::{RetirementDemand,ValueError,snapshot_projection::ImmutableRootProjection,retirement::RetireOwned,retained_clone::{RetainedClone,RetainedCloneRef,RetainedCloneStep,OriginalEffectReply,OriginalEffectReplyComplete}};
use crate::list::persistent::PersistentList;
pub use semio_framework_value_snapshot_projection::ImmutableRootCloneCursor;
unsafe impl<T:RetireOwned+Sync> ImmutableRootProjection for PersistentList<T>{
 fn root_projection_demand(_:RetainedCloneRef<'_,Self>)->Result<RetirementDemand,ValueError>{Ok(Self::projection_demand())}
 fn root_projection_work_demand(original:RetainedCloneRef<'_,Self>)->Result<u64,ValueError>{Self::projection_work_demand(original)}
 fn project_root_into<'r>(original:RetainedCloneRef<'_,Self>,output:&mut Option<Self>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{Self::project_into(original,output,effect,reply)}
 fn recover_project_root_into<'r>(_:RetainedCloneRef<'_,Self>,_:&mut Option<Self>,_:&mut crate::RetainedAllocationEffect<'_>,_:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{Err(ValueError::invariant("native persistent projection has no unrecorded callback"))}
}
impl<T:RetireOwned+Sync> RetainedClone for PersistentList<T>{type Cursor=ImmutableRootCloneCursor<Self>;fn retained_clone_cursor()->Self::Cursor{Default::default()}}
#[cfg(test)]use crate::retained_clone::{RetainedCloneCursor,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneClose};
#[cfg(test)]#[path="../../../🧬️retained-clone/🔗️sealed-shared-list/📋️list/🧪️tests/🦀️.rs"]mod list_tests;
