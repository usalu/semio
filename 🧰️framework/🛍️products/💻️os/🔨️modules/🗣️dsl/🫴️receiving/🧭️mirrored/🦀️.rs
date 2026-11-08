//! 🧭️ A vocabulary whose persisted binary form is its mirror vocabulary's receives through the mirror's own original cursor and converts the finished value.
use super::*;
/// 🪞️ The persisted binary wire of `Self` is the binary wire of its mirror vocabulary.
pub trait OriginalMirroredOperation:RetireOwned+Sized{
 type Mirror:OpBinaryReceiving;
 fn from_mirror(mirror:Self::Mirror)->Self;
}
/// 🎯️ Delegates every turn to the mirror cursor and converts only the finished mirror value.
pub struct MirroredOperationReceiving<M:OriginalMirroredOperation>{inner:<M::Mirror as OpBinaryReceiving>::Receiving}
/// 🎟️ Admits the mirror's cursor under the same original source and range.
pub fn admit_mirrored<M:OriginalMirroredOperation>(source:&[u8],range:(usize,usize),grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(MirroredOperationReceiving<M>,RetainedCloneProgress)>>{
 let turn=<M::Mirror as OpBinaryReceiving>::admit_receiving(source,range,grant);
 RetainedWorkOutcome{result:turn.result.map(|admitted|admitted.map(|(inner,progress)|(MirroredOperationReceiving{inner},progress))),work:turn.work}
}
impl<M:OriginalMirroredOperation> RetireOwned for MirroredOperationReceiving<M>{
 type RetirementCursor=<<M::Mirror as OpBinaryReceiving>::Receiving as RetireOwned>::RetirementCursor;
 fn retirement_ready(&self)->bool{self.inner.retirement_ready()}
 fn retirement_seed(self)->Self::RetirementCursor{self.inner.retirement_seed()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.inner.retirement_birth_bytes()}
 fn controlled_retirement_supported()->bool{<<M::Mirror as OpBinaryReceiving>::Receiving as RetireOwned>::controlled_retirement_supported()}
}
impl<M:OriginalMirroredOperation+OpBinaryReceiving<DomainFailure=<M::Mirror as OpBinaryReceiving>::DomainFailure>> OpBinaryReceivingCursor<M> for MirroredOperationReceiving<M>{
 fn demands(&self,source:&[u8],body:usize)->Result<RetirementDemand,ValueError>{self.inner.demands(source,body)}
 fn step_in(&mut self,source:&[u8],native:&mut NativeDecodeControl<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{self.inner.step_in(source,native,body,effect)}
 fn domain_failure(&self)->Option<&<M::Mirror as OpBinaryReceiving>::DomainFailure>{self.inner.domain_failure()}
 fn domain_failure_demand(&self)->Result<RetirementDemand,ValueError>{self.inner.domain_failure_demand()}
 fn domain_failure_work_demand(&self)->Result<u64,ValueError>{self.inner.domain_failure_work_demand()}
 fn take_domain_failure_into(&mut self,recipient:&mut Option<<M::Mirror as OpBinaryReceiving>::DomainFailure>,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{self.inner.take_domain_failure_into(recipient,grant)}
 fn is_ready(&self)->bool{self.inner.is_ready()}
 fn take_into(&mut self,recipient:&mut Option<M>,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{
  if recipient.is_some()||!self.inner.is_ready(){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
  let mut mirror=None;let turn=self.inner.take_into(&mut mirror,grant);
  if let Some(mirror)=mirror{*recipient=Some(M::from_mirror(mirror));}
  turn
 }
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.inner.close_demands(body)}
 fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{self.inner.close_step(effect)}
 fn terminal_is_empty(&self)->bool{self.inner.terminal_is_empty()}
}
/// 🪞️ Declares a vocabulary whose persisted binary wire is `$mirror`'s, converted by `$from` once received.
#[macro_export]
macro_rules! original_mirrored_operation_receiving{
 ($operation:ty,$mirror:ty,$from:expr)=>{
  impl $crate::original_receiving::mirrored::OriginalMirroredOperation for $operation{type Mirror=$mirror;fn from_mirror(mirror:$mirror)->Self{($from)(mirror)}}
  impl $crate::original_receiving::OpBinaryReceiving for $operation{
   type DomainFailure=<$mirror as $crate::original_receiving::OpBinaryReceiving>::DomainFailure;
   type Receiving=$crate::original_receiving::mirrored::MirroredOperationReceiving<Self>;
   fn admit_receiving(source:&[u8],range:(usize,usize),grant:$crate::original_receiving::RetainedCloneGrant)->$crate::original_receiving::RetainedWorkOutcome<Option<(Self::Receiving,$crate::original_receiving::RetainedCloneProgress)>>{$crate::original_receiving::mirrored::admit_mirrored::<Self>(source,range,grant)}
  }
 };
}
