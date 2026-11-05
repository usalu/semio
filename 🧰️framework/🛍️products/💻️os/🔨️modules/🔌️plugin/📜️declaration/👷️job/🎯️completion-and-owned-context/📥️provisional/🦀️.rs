//! 📥️ Original provisional projections retain the genuine immutable captured context issuer.
use super::{ArtifactOwnedContextHandle,ArtifactOwnedToolJobContext,ArtifactApp};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome};
use semio_framework_value::retained_clone::{RetainedCloneSource,RetainedCloneSourceAdmission};
impl<A:ArtifactApp> ArtifactOwnedContextHandle<A>{
 /// 🎟️ Quotes one original immutable context alias without copying provisional leaves.
 pub fn original_alias_demands(&self)->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
 /// 🪢️ Preserves the concrete issuer and exact work of one paid original alias.
 pub fn duplicate_original(&self,grant:RetainedCloneGrant)->RetainedWorkOutcome<(Self,RetainedCloneProgress)>{
  match self.source.as_ref(){
   Some(super::Source::Admitted(source))=>{let turn=source.try_duplicate(grant);RetainedWorkOutcome{result:turn.result.map(|(source,p)|(Self{source:std::mem::ManuallyDrop::new(Some(super::Source::Admitted(source)))},p)),work:turn.work}},
   _=>RetainedWorkOutcome{result:Err(super::unadmitted()),work:0}
  }
 }
 /// 🔐️ Consumes the genuine paid context alias into staged original projection authority.
 pub fn provisional_source_admission(self)->RetainedCloneSourceAdmission<ArtifactOwnedToolJobContext<A>,Self>{RetainedCloneSource::admission_borrowed(self,Self::get)}
}
