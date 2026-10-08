//! 🎟️ History cursor frames admit their actual allocation before original ownership moves.
use super::{ArtifactHistoryReadRetirement,ArtifactOwnedValueRetirementFactory,ErasedSnapshotRetirement,ValueError,admit_artifact_retirement};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
use std::{mem::ManuallyDrop,sync::Arc};

pub(super) fn history_read_retirement_birth_bytes<P,Mu:super::Mutation<P>>()->usize {std::mem::size_of::<ArtifactHistoryReadRetirement<P,Mu>>()}

pub(super) fn admit_history_read_retirement<P,Mu,T>(original:&mut Option<T>,snapshots:&Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,mutations:&Arc<dyn ArtifactOwnedValueRetirementFactory<Mu>>,grant:RetainedCloneGrant,install:fn(&mut ArtifactHistoryReadRetirement<P,Mu>,T))->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>
where P:Send+Sync+'static,Mu:super::Mutation<P>+Send+'static {
    if original.is_none(){return Ok(None);}
    admit_artifact_retirement((),grant,|()|{
        let mut owner=ArtifactHistoryReadRetirement {preview:ManuallyDrop::new(None),replay:ManuallyDrop::new(None),finished:ManuallyDrop::new(None),loaded_replay:ManuallyDrop::new(None),active:ManuallyDrop::new(None),plan_retirement:ManuallyDrop::new(None),draft_retirement:ManuallyDrop::new(None),registry_retirement:ManuallyDrop::new(None),snapshots:ManuallyDrop::new(Some(Arc::clone(snapshots))),mutations:ManuallyDrop::new(Some(Arc::clone(mutations))),factory_retirement:std::array::from_fn(|_|None)};
        install(&mut owner,original.take().expect("observed original history cursor"));owner
    }).map(Some).map_err(|(error,())|error)
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
