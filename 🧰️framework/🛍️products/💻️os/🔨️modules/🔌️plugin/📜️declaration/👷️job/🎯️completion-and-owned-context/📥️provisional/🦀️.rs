//! 📥️ Original provisional projections retain the genuine immutable captured context issuer.
use super::{ArtifactOwnedContextHandle,ArtifactOwnedToolJobContext,ArtifactApp};
use semio_framework_value::{RetirementDemand,RetainedAllocationEffect,RetainedCloneProgress,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{OriginalEffectReplyComplete,OriginalProjectionReply,OriginalSourceProjection,RetainedCloneSource,RetainedCloneSourceAdmission};
impl<A:ArtifactApp> ArtifactOwnedContextHandle<A>{
 /// 🎟️ Quotes one original immutable context alias without copying provisional leaves.
 pub fn original_alias_demands(&self)->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
 /// 🪢️ Preserves the concrete issuer and exact work of one paid original alias.
 pub fn duplicate_original(&self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<(Self,RetainedCloneProgress)>{
  match self.source.as_ref(){
   Some(super::Source::Admitted(source))=>{let turn=source.try_duplicate(effect);RetainedWorkOutcome{result:turn.result.map(|(source,p)|(Self{source:std::mem::ManuallyDrop::new(Some(super::Source::Admitted(source)))},p)),work:turn.work}},
   _=>RetainedWorkOutcome{result:Err(super::unadmitted()),work:0}
  }
 }
 /// 🔐️ Consumes the genuine paid context alias into staged original projection authority.
 pub fn provisional_source_admission(self)->RetainedCloneSourceAdmission<ArtifactOwnedToolJobContext<A>,Self>{RetainedCloneSource::admission_borrowed(self)}
}
impl<A:ArtifactApp> OriginalSourceProjection<ArtifactOwnedToolJobContext<A>> for ArtifactOwnedContextHandle<A>{
 fn projection_work_demand(&self)->Result<u64,ValueError>{Ok(1)}
 fn project_source<'reply,'source>(&'source self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalProjectionReply<'reply,'source,ArtifactOwnedToolJobContext<A>>)->Result<OriginalEffectReplyComplete<'reply,std::ptr::NonNull<ArtifactOwnedToolJobContext<A>>,ValueError>,ValueError>{if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"context source projection exceeded its original work"))))}Ok(reply.complete(Ok(self.get())))}
 fn recover_source_projection<'reply,'source>(&'source self,_:&mut RetainedAllocationEffect<'_>,reply:OriginalProjectionReply<'reply,'source,ArtifactOwnedToolJobContext<A>>)->Result<OriginalEffectReplyComplete<'reply,std::ptr::NonNull<ArtifactOwnedToolJobContext<A>>,ValueError>,ValueError>{Ok(reply.pending())}
}
