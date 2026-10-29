//! 🎟️ The original emitted input remains retained through lease, Window authority, mounted registry and receipt handoffs.
use crate::dispatch_bundle::DispatchCtx;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_instance_extension::InstanceExtension;
use semio_framework_plugin_job::original_emission::{OriginalEmissionInput,OriginalEmissionCapture,OriginalEmissionReceipt,admit_original_emission_receipt};
use semio_framework_plugin_retained_command::typed_operation::{MountedTypedCommandFullOperation,original_emission::{OriginalEmissionAdmission,OriginalEmissionPublicationFrontier}};
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalConstructionDemand,admit_original_construction,close_original_retained_step};
use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneProgress,RetainedCloneStep,RetirementDemand};
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use semio_framework::kernel::InvocationResult;
use store::{MemberFactory,SpaceMember};
impl<'ctx,A:ArtifactApp,M:store::ReferenceSpaceMember+MemberFactory+'static,X:InstanceExtension> DispatchCtx<'ctx,A,M,X>{
 /// 📥️ A producer keeps its original input slot until the admitted native owner takes it; pending work never repeats production.
 pub async fn dispatch_original_emission(&mut self,input:&mut Option<OriginalEmissionInput<A>>,output:&mut Option<InvocationResult>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.tools.original_emission_admission.is_none(){
   if let Some(source)=self.tools.original_emission_receipt{let operation=source.operation(cx)?;if admit_original_emission_receipt(operation,cx,output)?{self.tools.original_emission_receipt=None;return Ok(true)}return Ok(false)}
   if input.is_none(){return Err(invalid("original emission ingress requires its retained producer input").into())}
   OriginalEmissionAdmission::<A>::admit(cx,&mut self.tools.original_emission_admission)?;return Ok(false)
  }
  let phase=self.tools.original_emission_admission.as_ref().unwrap().phase;
  if phase<8{self.tools.original_emission_admission.as_ref().unwrap().validate(cx)?;}
  match phase{
   0=>{let source=self.tools.original_emission_admission.as_mut().unwrap();if source.capture.is_none(){OriginalEmissionCapture::<A>::admit(cx,&mut source.capture)?;}else{source.advance_phase(0,1,cx)?;}},
   1=>{if self.tools.original_emission_admission.as_ref().unwrap().operation.is_some(){self.tools.original_emission_admission.as_mut().unwrap().advance_phase(1,2,cx)?;return Ok(false)}let input=input.as_ref().ok_or_else(||invalid("original emission lease requires its retained original metadata"))?;if let Some((operation,lease))=self.admit_original_tool_lease(&input.verb,&input.meta,false,cx)?{let source=self.tools.original_emission_admission.as_mut().unwrap();source.operation=Some(operation);*source.lease=Some(lease);}},
   2=>{let source=self.tools.original_emission_admission.as_mut().unwrap();if source.capture.as_ref().unwrap().borrow().is_some(){source.advance_phase(2,3,cx)?;}else{source.capture.as_mut().unwrap().receive(input,cx)?;}},
   3=>{
    if self.tools.original_emission_admission.as_ref().unwrap().mounted.is_some(){self.tools.original_emission_admission.as_mut().unwrap().advance_phase(3,4,cx)?;return Ok(false)}
    let source=self.tools.original_emission_admission.as_ref().unwrap();let input=source.capture.as_ref().and_then(OriginalEmissionCapture::borrow).ok_or_else(||invalid("original mounted emission lost its captured producer input"))?;
    if self.tools.live_runtime_instance_id!=Some(input.meta.instance_id){return Err(invalid("original emission does not belong to the mounted live app instance").into())}
    if self.stores.unsupported_publication_contracts.contains_key(&input.verb){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original emission declares an unsupported publication lane").into())}
    let registration=self.tools.app_tool_registrations.get(&input.verb).or_else(||self.tools.framework_tool_registrations.get(&input.verb)).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original emission requires its exact registered native publication owner"))?;
    let frontier=OriginalEmissionPublicationFrontier{canonical_revision:self.stores.store.content_revision_now(),artifact_generation:self.stores.store.generation_now(),config_generation:self.stores.config_store.generation_now(),draft_generation:self.stores.draft_store.generation_now(),presence_generation:self.stores.presence_store.generation().await,transient_generation:self.stores.transient_store.generation().await,publication_lanes:registration.publication_lanes};
    let operation=source.operation.unwrap();if operation.generation.0!=frontier.artifact_generation||operation.base_revision.0!=u64::from_be_bytes(frontier.canonical_revision[..8].try_into().unwrap()){return Err(invalid("original emission Store frontier changed before its mounted birth").into())}
    let source=self.tools.original_emission_admission.as_mut().unwrap();MountedTypedCommandFullOperation::admit_original_emission(source.capture.as_mut().unwrap(),operation,frontier,&mut source.lease,&self.tools.tool_cancellations,cx,&mut source.mounted)?;
   },
   4=>{let source=self.tools.original_emission_admission.as_mut().unwrap();if source.window_config.as_ref().is_some_and(|owner|owner.handoff_complete()){source.advance_phase(4,5,cx)?;}else{source.capture_window_config(&mut self.stores.window_config_store,cx)?;}},
   5=>{let source=self.tools.original_emission_admission.as_mut().unwrap();if source.window_transient.as_ref().is_some_and(|owner|owner.handoff_complete()){source.advance_phase(5,6,cx)?;}else{source.capture_window_transient(&mut self.stores.window_transient_store,cx)?;}},
   6=>{
    let source=self.tools.original_emission_admission.as_mut().unwrap();if source.mounted.as_ref().unwrap().captured_child_content.is_some(){source.advance_phase(6,7,cx)?;return Ok(false)}let mounted=source.mounted.as_mut().unwrap();let bytes=semio_framework_value::retirement::arc_custody::arc_bytes::<ChildContentView>();let copy=size_of::<ChildContentView>()+2*size_of::<std::sync::Arc<ChildContentView>>();
    if admit_original_construction(cx,&mut mounted.captured_child_content,OriginalConstructionDemand{items:1,copy_bytes:copy,capacity_bytes:bytes,release_bytes:0,depth:1},|_|Ok((std::sync::Arc::new(ChildContentView::clone(&*self.child_composition.child_content_root)),RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:bytes,..Default::default()})))?{mounted.captured_child_content_generation=self.child_composition.child_content_generation;}
   },
   7=>{
    if self.tools.original_emission_admission.as_ref().unwrap().mounted.is_none(){self.tools.original_emission_admission.as_mut().unwrap().advance_phase(7,8,cx)?;return Ok(false)}
    let source=self.tools.original_emission_admission.as_ref().unwrap();let mounted=source.mounted.as_ref().unwrap();let operation=source.operation.unwrap();let id=operation.operation.0;
    if mounted.canonical_revision!=self.stores.store.content_revision_now()||mounted.artifact_generation!=self.stores.store.generation_now()||mounted.config_generation!=self.stores.config_store.generation_now()||mounted.draft_generation!=self.stores.draft_store.generation_now()||mounted.captured_child_content_generation!=self.child_composition.child_content_generation||!self.tools.tool_operations.can_insert(id)||self.tools.typed_operation_reservations[id as usize%semio_framework_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS]!=Some(id){return Err(invalid("original emission publication destination or captured Store frontier changed").into())}
    let receipt=OriginalEmissionReceipt::capture(operation,cx)?;let mut mounted=None;if self.tools.original_emission_admission.as_mut().unwrap().take_mounted_into(&mut mounted,cx)?{self.tools.tool_operations.insert_admitted(id,mounted.unwrap());self.tools.release_reserved_emit_slot(id);self.tools.original_emission_receipt=Some(receipt);}
   },
   8=>{
    let source=self.tools.original_emission_admission.as_mut().unwrap();if !source.terminal_is_empty(){cx.with_original_retirement(|cx|source.close_step(&self.tools.tool_cancellations,cx))?;return Ok(false)}
    let d=RetirementDemand{copy_bytes:size_of::<Option<Box<OriginalEmissionAdmission<A>>>>(),release_bytes:size_of::<OriginalEmissionAdmission<A>>(),depth:1,..Default::default()};close_original_retained_step(cx,d,|_|{self.tools.original_emission_admission.take();Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))})?;
   },
   _=>return Err(invalid("original emission admission has an invalid retained phase").into())
  }
  Ok(false)
 }
}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
