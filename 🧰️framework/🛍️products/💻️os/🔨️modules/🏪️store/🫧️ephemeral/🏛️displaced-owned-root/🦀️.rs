//! 🏛️ A displaced shared root keeps its owned-value issuer until a granted turn admits it into the actual retirement frame.
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,ErasedSnapshotRetirement,OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneStep,RetirementDemand,RetirementPollExit,ValueError};
use semio_framework_value::factory::ArcRootRetirement;
use semio_framework_value_snapshot_read::{artifact_retirement_box_close_step,artifact_retirement_box_demands};
use std::{mem::ManuallyDrop,sync::Arc};
type Retired=Box<dyn ErasedSnapshotRetirement>;
/// 🏛️ Holds the displaced root, its issuer and the admitted frame until physical return.
pub struct DisplacedOwnedRootRetirement<P:Send+Sync+'static>{root:ManuallyDrop<Option<Arc<P>>>,factory:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>>,active:ManuallyDrop<Option<Retired>>}
impl<P:Send+Sync+'static> DisplacedOwnedRootRetirement<P>{
 /// 🆕️ Takes custody of the displaced root and its issuer.
 pub fn new(root:Arc<P>,factory:Arc<dyn ArtifactOwnedValueRetirementFactory<P>>)->Self{Self{root:ManuallyDrop::new(Some(root)),factory:ManuallyDrop::new(Some(factory)),active:ManuallyDrop::new(None)}}
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(active)=self.active.as_ref(){return artifact_retirement_box_demands(active,body)}if self.root.is_some(){return Ok(ArcRootRetirement::<P>::admission_demand())}Ok(Default::default())}
}
impl<P:Send+Sync+'static> ErasedSnapshotRetirement for DisplacedOwnedRootRetirement<P>{
 fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  let result=(||->Result<RetainedCloneStep,RetirementPollExit>{
   if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
   let step=if self.active.is_some(){let step=RetirementPollExit::resume(artifact_retirement_box_close_step(&mut self.active,body,effect))?;if self.active.as_ref().is_some_and(|owner|owner.terminal_is_empty()){self.active=ManuallyDrop::new(None);}step}else{RetirementPollExit::resume(ArcRootRetirement::admit_original(&mut self.root,&mut self.factory,&mut self.active,effect))?};
   Ok(RetainedCloneStep::Progress(step.progress()))
  })();
  RetirementPollExit::settle(effect,result)
 }
 fn terminal_is_empty(&self)->bool{self.root.is_none()&&self.factory.is_none()&&self.active.is_none()}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.demands(body)?.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demands(body)}
}
impl<P:Send+Sync+'static> Drop for DisplacedOwnedRootRetirement<P>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"displaced owned root abandoned its payload");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.root);ManuallyDrop::drop(&mut self.factory);ManuallyDrop::drop(&mut self.active);}}}}
