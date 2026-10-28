//! 📦️ The original mounted App box remains owned until its actual close ladder and physical release finish.
use crate::VcsArtifactApp;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_os_kernel::{SpaceMember,MemberFactory};
use semio_framework_plugin_reactor_app::{PluginApp,PluginAppDelegate};
use semio_framework_plugin_ownership::bounded_ownership::{PluginLifecycleStep,PluginLifecycleFailure,original_lifecycle_step};
use semio_framework_plugin_mounted_owner::{MountedOwnerPhaseV1,MountedOwnerTurnV1};
use semio_framework_job::StepContext;
use semio_framework_value::{RetirementDemand,ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
use std::mem::{ManuallyDrop,size_of};
/// 🧳️ A native host recipient keeps the exact constructed allocation without a second App materialization.
pub struct OriginalVcsApp<A:ArtifactApp,M:SpaceMember+MemberFactory+'static>{original:ManuallyDrop<Option<Box<VcsArtifactApp<A,M>>>>}
impl<A:ArtifactApp,M:SpaceMember+MemberFactory+'static> From<Box<VcsArtifactApp<A,M>>> for OriginalVcsApp<A,M>{fn from(original:Box<VcsArtifactApp<A,M>>)->Self{Self{original:ManuallyDrop::new(Some(original))}}}
impl<A:ArtifactApp,M:SpaceMember+MemberFactory+'static> OriginalVcsApp<A,M>{
 /// 👁️ Borrows the unchanged live App; no owner or snapshot is copied.
 pub fn get(&self)->Option<&VcsArtifactApp<A,M>>{self.original.as_deref()}
 /// 🕳️ Terminal custody includes physical release of the original mounted allocation.
 pub fn custody_is_closed(&self)->bool{self.original.is_none()}
 fn frame_demands()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Box<VcsArtifactApp<A,M>>>>(),release_bytes:size_of::<VcsArtifactApp<A,M>>(),depth:1,..Default::default()}}
 fn release_frame(&mut self,cx:&mut StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>{if self.original.is_none(){return Ok(PluginLifecycleStep::Complete(Default::default()))}let demand=Self::frame_demands();original_lifecycle_step(cx,loan,demand,|_,_|{self.original.take();Ok((PluginLifecycleStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}),true))})}
 fn finish_step(&self,first:PluginLifecycleStep,last:PluginLifecycleStep)->Result<PluginLifecycleStep,PluginLifecycleFailure>{let progress=first.progress().unwrap_or_default().checked_add(last.progress().unwrap_or_default())?;Ok(if self.custody_is_closed(){PluginLifecycleStep::Complete(progress)}else{PluginLifecycleStep::Progress(progress)})}
}
impl<A:ArtifactApp,M:SpaceMember+MemberFactory+Send+'static> PluginAppDelegate for OriginalVcsApp<A,M>{
 type Target=VcsArtifactApp<A,M>;
 fn delegate_ref(&self)->&Self::Target{self.original.as_deref().expect("closed original App cannot serve another live request")}
 fn delegate_mut(&mut self)->&mut Self::Target{self.original.as_deref_mut().expect("closed original App cannot serve another live request")}
 fn close_retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{match self.original.as_deref(){None=>Ok(Default::default()),Some(original)if PluginApp::close_terminal_is_empty(original)=>Ok(Self::frame_demands()),Some(original)=>PluginApp::close_retirement_demands(original,body)}}
 fn close_step(&mut self,cx:&mut StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>{if self.custody_is_closed(){return Ok(PluginLifecycleStep::Complete(Default::default()))}let demand=PluginAppDelegate::close_retirement_demands(self,loan.maximum_copy_bytes)?;original_lifecycle_step(cx,loan,demand,|cx,grant|{let first=PluginApp::close_step(self.original.as_deref_mut().unwrap(),cx,grant)?;if !matches!(first,PluginLifecycleStep::Complete(_)){return Ok((first,false))}let last=self.release_frame(cx,grant)?;let step=self.finish_step(first,last)?;Ok((step,self.custody_is_closed()))})}
 fn close_terminal_is_empty(&self)->bool{self.custody_is_closed()}
 fn close_diagnostic(&self)->String{self.original.as_deref().map_or_else(String::new,PluginApp::close_diagnostic)}
 fn maintenance_retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.original.as_deref().map_or(Ok(Default::default()),|original|PluginApp::maintenance_retirement_demands(original,body))}
 fn maintenance_terminal_is_empty(&self)->bool{self.original.as_deref().is_none_or(PluginApp::maintenance_terminal_is_empty)}
 fn maintenance_under_pressure(&self)->bool{self.original.as_deref().is_some_and(PluginApp::maintenance_under_pressure)}
 fn maintenance_step(&mut self,cx:&mut StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,PluginLifecycleFailure>{self.original.as_deref_mut().map_or(Ok(PluginLifecycleStep::Complete(Default::default())),|original|PluginApp::maintenance_step(original,cx,loan))}
 fn mounted_owner_maintenance_phase(&self)->Result<MountedOwnerPhaseV1,semio_framework::Fault>{self.original.as_deref().map_or(Ok(MountedOwnerPhaseV1::Maintenance),PluginApp::mounted_owner_maintenance_phase)}
 fn mounted_owner_maintenance_step(&mut self,turn:&mut MountedOwnerTurnV1<'_,'_>)->Result<PluginLifecycleStep,PluginLifecycleFailure>{self.original.as_deref_mut().map_or(Ok(PluginLifecycleStep::Complete(Default::default())),|original|PluginApp::mounted_owner_maintenance_step(original,turn))}
 fn mounted_owner_close_step(&mut self,turn:&mut MountedOwnerTurnV1<'_,'_>)->Result<PluginLifecycleStep,PluginLifecycleFailure>{if self.custody_is_closed(){return Ok(PluginLifecycleStep::Complete(Default::default()))}let demand=PluginAppDelegate::close_retirement_demands(self,turn.grant().maximum_copy_bytes)?;if !semio_framework_plugin_ownership::bounded_ownership::plugin_grant_funds(demand,turn.grant()){return Ok(PluginLifecycleStep::Progress(Default::default()))}let original=self.original.as_deref_mut().unwrap();let first=PluginApp::mounted_owner_close_step(original,turn)?;if !matches!(first,PluginLifecycleStep::Complete(_)){return Ok(first)}let last=turn.advance_lifecycle(|cx,grant|self.release_frame(cx,grant))?;self.finish_step(first,last)}
}
impl<A:ArtifactApp,M:SpaceMember+MemberFactory+'static> Drop for OriginalVcsApp<A,M>{fn drop(&mut self){assert!(std::thread::panicking()||self.custody_is_closed(),"original App recipient retains its actual allocation until paid physical closure")}}
