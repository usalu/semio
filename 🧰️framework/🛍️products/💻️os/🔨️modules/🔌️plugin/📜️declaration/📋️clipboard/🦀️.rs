//! 📋️ The framework-owned clipboard work every editor declaring a clipboard media type inherits: `copy` and `cut` author the clipboard write, `paste` the placed mutations, each through the request's one completion cell.
use semio_plugin_decl_app::tool_contract::{ArtifactReservedToolInput,ArtifactReservedToolJobRequest,ArtifactToolCompletionRejection};
use semio_plugin_decl_app::app_emission::EphemeralEmit;
use semio_framework_job::{InteractiveJobCloseStep,InteractiveJobStep,JobOutcomeBorrow,JobOutcomeDescriptor,JobOutcomeKind,JobOutcomeView,OriginalJobReturnClaim,OriginalTurnFailure,StepContext};
use semio_framework_value::{FromValue,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::RetainedCloneClose};
use semio_framework::{Fault,kernel::{ClipboardFragment,Effect,PastePlacement}};
use semio_plugin_decl_views::{AppOperationContext,InteractionHoverState,PeerPresenceRoot};
use semio_plugin_interaction::interaction_topology::empty_draft_references;
use crate::{ArtifactEditor,ArtifactView,ConfigView,Emit,EditorApp,InteractionView};

/// 📋️ Authors the clipboard outcome of one reserved `copy`, `cut` or `paste` request from the editor's own clipboard hooks.
pub(crate) struct EditorClipboardWork<E:ArtifactEditor>{rejected:Option<ArtifactToolCompletionRejection<EditorApp<E>>>,retirement:RetainedCloneClose,completed:bool,closing:bool}
impl<E:ArtifactEditor> Default for EditorClipboardWork<E>{fn default()->Self{Self{rejected:None,retirement:Default::default(),completed:false,closing:false}}}
impl<E:ArtifactEditor> EditorClipboardWork<E>{
 fn emit(request:&ArtifactReservedToolJobRequest<EditorApp<E>>)->Result<Emit<E::Mutation,E::ConfigMutation,E::DraftMutation>,Fault>{
  if E::clipboard_media_type().is_none(){return Ok(Emit::default())}
  let Some(ArtifactReservedToolInput::Action{args,interaction}) = request.input.as_ref() else{return Err(Fault::from("the clipboard requires an action input"))};
  let operation=AppOperationContext{app_instance_id:request.app_instance_id,parent_document_id:request.parent_document_id.clone(),operation_id:0,generation:0,canonical_base_revision:request.canonical_base_revision,retained:request.retained,authoring_seed:request.authoring_seed.clone()};
  let document=ArtifactView::with_operation(request.snapshot.as_ref(),request.history.as_ref(),operation);
  match request.tool_id.as_str(){
   "paste"=>{
    let Some(args)=args.as_ref() else{return Ok(Emit::default())};
    let Some(fragment)=args.get("fragment").and_then(|value|ClipboardFragment::from_value(value.clone()).ok()) else{return Ok(Emit::default())};
    let placement=PastePlacement::from_action_args(args).map_err(|error|Fault::from(error.to_string()))?;
    let mutations=E::paste_operations(&document,&fragment,&placement).map_err(|error|Fault::from(error.to_string()))?;
    Ok(Emit::mutations(mutations))
   },
   tool=>{
    let config=ConfigView{snapshot:request.config.as_ref(),window:None};
    let hover=InteractionHoverState::default();
    let view=InteractionView::new(interaction.get(),&hover,&PeerPresenceRoot::EMPTY,empty_draft_references());
    let Ok(fragment)=E::copy_fragment(&document,&config,&view) else{return Ok(Emit::default())};
    let mut emit=Emit::effect(Effect::ClipboardWrite{fragment});
    if tool=="cut"{emit.artifact_mutations=E::cut_operations(&document,&config,&view)}
    Ok(emit)
   },
  }
 }
 fn step_value<'a>(&'a mut self,request:&mut ArtifactReservedToolJobRequest<EditorApp<E>>,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>{
  if cx.is_cancelled(){return JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into)}
  if self.rejected.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"the clipboard completion remains rejected").into())}
  if cx.should_yield(){return Ok(None)}
  if !self.completed{
   let emit=Self::emit(request);cx.consume_fuel(1);
   if let Err(rejected)=request.completion.complete(emit,EphemeralEmit::default()){self.rejected=Some(rejected);return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"the clipboard completion was rejected").into())}
   self.completed=true;
  }
  JobOutcomeBorrow::admit_complete(cx,None,None).map_err(Into::into)
 }
}
impl<E:ArtifactEditor> semio_plugin_decl_app::tool_contract::ArtifactReservedWork<EditorApp<E>> for EditorClipboardWork<E>{
 fn step<'a>(&'a mut self,request:&'a mut ArtifactReservedToolJobRequest<EditorApp<E>>,cx:&mut StepContext<'_>,_:&mut OriginalJobReturnClaim<'_>)->InteractiveJobStep<'a>{InteractiveJobStep::from_callback(self.step_value(request,cx))}
 fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match descriptor.kind(){JobOutcomeKind::Cancelled=>descriptor.cancelled(),JobOutcomeKind::Yield=>descriptor.yielded(),_=>descriptor.complete(None,None)}}
 fn begin_close(&mut self){self.closing=true}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.retirement.next_owner_demands::<ArtifactToolCompletionRejection<EditorApp<E>>>(self.rejected.is_some(),&None,body)}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  self.closing=true;let body=cx.retained_grant().maximum_copy_bytes;let demand=self.close_demands(body)?;
  let step=semio_framework_job::close_original_effect_step(cx,demand,|effect|{
   if self.retirement.is_empty(){let turn=self.retirement.begin_granted(&mut self.rejected,effect);RetainedWorkPoll::Complete(RetainedWorkOutcome{work:turn.work,result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Complete(Default::default()))).map_err(Into::into)})}
   else{self.retirement.step_granted(body,effect)}
  })?;
  Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{step})
 }
 fn terminal_is_empty(&self)->bool{self.closing&&self.rejected.is_none()&&self.retirement.is_empty()}
}
