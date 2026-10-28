//! 📸️ Media preparation retains exact checkpoint replay custody until its original ledger and decoder return.
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_instance_stores::stores_components::InstanceStores;
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalDomainFailure,OriginalDomainProjection,OriginalDomainView,OriginalConstructionDemand,admit_original_construction};
use semio_framework_value::{ErasedSnapshotRetirement,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_plugin_ownership::bounded_ownership::{plugin_typed_owner_demand,plugin_typed_owner_step,plugin_grant_funds};
use std::{sync::Arc,mem::ManuallyDrop,fmt::Write};
use store::os_store::snapshot_read::ArtifactDerivedHistoryPreview;
fn invalid(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
#[derive(Debug,semio_framework_value::RetireOwned)]
pub struct CheckpointDomainFailure{source:store::VcsError,text:Option<String>}
impl OriginalDomainFailure for CheckpointDomainFailure{fn domain_code(&self)->&str{"checkpoint.replay"}fn domain_message(&self)->&str{self.text.as_deref().unwrap_or("checkpoint replay refused")}fn domain_retained_progress(&self)->RetainedCloneProgress{Default::default()}}
impl OriginalDomainProjection for CheckpointDomainFailure{
 fn projection_view(&self,path:&[usize])->Result<OriginalDomainView<'_>,ValueError>{match path{[]=>Ok(OriginalDomainView::IntrinsicObject(1)),[0]=>Ok(OriginalDomainView::IntrinsicText(self.domain_message())),_=>Err(invalid("checkpoint refusal projection path changed"))}}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{if path.is_empty()&&index==0{Ok("message")}else{Err(invalid("checkpoint refusal field path changed"))}}
}
struct Count(usize);
impl std::fmt::Write for Count{fn write_str(&mut self,text:&str)->std::fmt::Result{self.0=self.0.checked_add(text.len()).ok_or(std::fmt::Error)?;Ok(())}}
pub struct NativeCheckpointExportSource<A:ArtifactApp>{preview:ManuallyDrop<Option<ArtifactDerivedHistoryPreview<A::Snapshot,A::Mutation>>>,retirement:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,snapshot:ManuallyDrop<Option<Arc<A::Snapshot>>>,error:ManuallyDrop<Option<CheckpointDomainFailure>>,error_close:ManuallyDrop<Option<ControlledRetirement<CheckpointDomainFailure>>>,finished:bool,captured:bool}
impl<A:ArtifactApp> NativeCheckpointExportSource<A>{
 pub fn empty()->Self{Self{preview:ManuallyDrop::new(None),retirement:ManuallyDrop::new(None),snapshot:ManuallyDrop::new(None),error:ManuallyDrop::new(None),error_close:ManuallyDrop::new(None),finished:false,captured:false}}
 pub fn captured(&self)->bool{self.captured}
 pub fn snapshot_owner(&self)->Arc<A::Snapshot>{Arc::clone(self.snapshot.as_ref().expect("completed checkpoint retains its original snapshot"))}
 pub fn terminal_is_empty(&self)->bool{self.preview.is_none()&&self.retirement.is_none()&&self.snapshot.is_none()&&self.error.is_none()&&self.error_close.is_none()}
 pub fn close_demands(&self,stores:&InstanceStores<A>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.error.is_some()||self.error_close.is_some(){return plugin_typed_owner_demand(&self.error,&self.error_close,body)}
  if let Some(preview)=self.preview.as_ref(){return stores.store.derived_history_preview_retirement_demands(preview,body)}
  if let Some(owner)=self.retirement.as_ref(){return semio_framework_value::factory_ticket_demands(owner,body)}
  if self.snapshot.is_some(){return Ok(RetirementDemand{capacity_bytes:semio_framework_value::FactorySharedRetirement::<A::Snapshot>::constructor_capacity_bytes(),depth:1,..Default::default()})}Ok(Default::default())
 }
 pub fn close_step(&mut self,stores:&mut InstanceStores<A>,cx:&mut StepContext<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,OriginalTurnFailure>{
  if self.error.is_some()||self.error_close.is_some(){return plugin_typed_owner_step(&mut self.error,&mut self.error_close,grant).map(|step|step.progress()).map_err(Into::into)}
  if self.preview.is_some(){let needs=self.preview.as_ref().and_then(|preview|preview.derived_operation.as_ref()).is_some_and(|owner|owner.requires_original_control());let result=if needs{cx.with_native_retirement(|original|stores.store.retire_derived_history_preview_controlled(&mut self.preview,grant,original))}else{stores.store.retire_derived_history_preview(&mut self.preview,grant)}?;return if let Some((owner,p))=result{*self.retirement=Some(owner);Ok(p)}else{Ok(self.preview.as_mut().map_or(Default::default(),|preview|preview.take_preparation_receipt()))}}
  if self.retirement.is_some(){let needs=self.retirement.as_ref().is_some_and(|owner|owner.requires_original_control());let step=if needs{cx.with_native_retirement(|original|semio_framework_value::close_factory_ticket_controlled(&mut self.retirement,grant,original))?}else{semio_framework_value::close_factory_ticket(&mut self.retirement,grant)?};return Ok(step.progress())}
  if self.snapshot.is_some(){if let Some((owner,p))=stores.store.retire_snapshot_alias(&mut self.snapshot,grant)?{*self.retirement=Some(owner);return Ok(p)}}Ok(Default::default())
 }
 pub fn advance(&mut self,stores:&mut InstanceStores<A>,id:&str,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.captured{return Ok(true)}if cx.should_yield()||cx.is_cancelled(){return Ok(false)}let grant=cx.retained_grant();
  if let Some(error)=self.error.as_mut(){if error.text.is_none(){let mut length=Count(0);write!(&mut length,"{}",error.source).map_err(|_|invalid("checkpoint refusal extent overflowed"))?;if length.0>4096{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"checkpoint refusal exceeds original diagnostic capacity").into())}let source=&error.source;admit_original_construction(cx,&mut error.text,OriginalConstructionDemand{items:1,copy_bytes:length.0+size_of::<String>(),capacity_bytes:length.0,release_bytes:0,depth:1},|_|{let mut text=String::new();text.try_reserve_exact(length.0).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"checkpoint original refusal text allocation refused"))?;write!(&mut text,"{source}").map_err(|_|invalid("checkpoint refusal text exceeded measured extent"))?;let capacity=text.capacity();Ok((text,RetainedCloneProgress{copied_items:1,copied_bytes:length.0+size_of::<String>(),retained_capacity_bytes:capacity,..Default::default()}))})?;return Ok(false)}if let Some(failure)=semio_framework_job::admit_original_domain_failure(&mut self.error,cx)?{return Err(failure)}return Ok(false)}
  if self.finished{
   if self.preview.is_some()||self.retirement.is_some(){let d=self.close_demands(stores,grant.maximum_copy_bytes)?;if grant.maximum_items==0||!plugin_grant_funds(d,grant){return Ok(false)}let p=self.close_step(stores,cx,grant)?;cx.consume_retained(p)?;cx.consume_fuel(p.copied_items as u64);return Ok(false)}
   let copy=size_of::<bool>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy{return Ok(false)}self.captured=true;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;cx.consume_fuel(1);return Ok(true)
  }
  if self.preview.is_none(){if let Some((preview,p))=stores.store.begin_derived_checkpoint_preview(id,grant)?{*self.preview=Some(preview);cx.consume_retained(p)?;cx.consume_fuel(p.copied_items as u64);}return Ok(false)}
  if self.preview.as_ref().unwrap().finished{let copy=size_of::<Arc<A::Snapshot>>()+size_of::<bool>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy{return Ok(false)}*self.snapshot=Some(self.preview.as_mut().unwrap().take_finished_snapshot_owner().map_err(|_|invalid("checkpoint completed source has no original snapshot"))?);self.finished=true;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
  let clock=cx.clock_source();let deadline=cx.deadline_us();let preview=self.preview.as_mut().unwrap();let received=cx.with_native_receiving(|native|Ok::<_,ValueError>(stores.store.step_derived_checkpoint_preview(preview,id,grant,native,&mut||clock().is_none_or(|now|now>=deadline))));let result=match received{Ok(result)=>result,Err(error)=>{let progress=error.retained_progress();let mut failure=OriginalTurnFailure::from(error);if cx.receiving_issued_grant().is_some(){failure.context_refusal=cx.consume_receiving_retained(grant,progress).err();}cx.consume_fuel(progress.copied_items as u64);return Err(failure)}};
  let progress=match &result{Err(store::VcsError::InverseRefused(error)|store::VcsError::NativeEncoding(error))=>error.retained_progress(),_=>preview.take_preparation_receipt()};let reconciled=cx.consume_receiving_retained(grant,progress);cx.consume_fuel(progress.copied_items as u64);
  match result{Ok(_)=>reconciled?,Err(store::VcsError::InverseRefused(error)|store::VcsError::NativeEncoding(error))=>{let mut failure=OriginalTurnFailure::from(error);failure.context_refusal=reconciled.err();return Err(failure)},Err(source)=>{*self.error=Some(CheckpointDomainFailure{source,text:None});reconciled?;}}Ok(false)
 }
}
impl<A:ArtifactApp> Drop for NativeCheckpointExportSource<A>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"checkpoint export requires original Store and decoder closure");}}
