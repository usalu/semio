//! ✂️ Clipboard work borrows the complete original request until its funded terminal close.

use super::*;
use semio_s_artifact_procedural_3d_delete_selection as delete_selection;
#[cfg(test)]
#[path="🧪️custody/🦀️.rs"]
mod original_custody_tests;

/// ♻️ Only a rejected completion remains private; the complete original request stays in the shared wrapper.
pub(super) struct Puzzle3dClipboardJob{
 rejection:std::mem::ManuallyDrop<Option<semio_plugin::app::ArtifactToolCompletionRejection<EditorApp<Puzzle3dPlayApp>>>>,
 retirement:semio_framework_value::retained_clone::RetainedCloneClose,
 outbox:semio_s_plugin_puzzle_jobs::JobOutbox,
 closing:bool,
 completed:bool,
}
impl Default for Puzzle3dClipboardJob{fn default()->Self{Self{rejection:std::mem::ManuallyDrop::new(None),retirement:Default::default(),outbox:Default::default(),closing:false,completed:false}}}
impl Puzzle3dClipboardJob{
 fn emit(request:&ArtifactReservedToolJobRequest<EditorApp<Puzzle3dPlayApp>>)->Emit<Puzzle3dMutation,Puzzle3dConfigMutation,NoDraftMutation>{
  let Some(ArtifactReservedToolInput::Action{args,interaction,..})=request.input.as_ref()else{return Emit::default()};
  let snapshot=request.snapshot.as_ref();let scene_snapshot=puzzle3d_scene_snapshot_from_play_snapshot(snapshot);let marks=Puzzle3dInteractionSnapshot::from_selection_state(interaction.get());
  match request.tool_id.as_str(){
   "copy"=>{let selected=puzzle3d_selected_objects_from(&marks,&scene_snapshot);match puzzle3d_copy_fragment_from(&scene_snapshot,selected){Ok(fragment)=>Emit{effects:vec![Effect::ClipboardWrite{fragment}],..Default::default()},Err(_)=>Emit::default()}},
   "cut"=>{let objects=puzzle3d_selected_objects_from(&marks,&scene_snapshot);let Ok(fragment)=puzzle3d_copy_fragment_from(&scene_snapshot,objects)else{return Emit::default()};let mutations=puzzle3d_cut_operations_from(snapshot.typed(),&marks);Emit{artifact_mutations:mutations,effects:vec![Effect::ClipboardWrite{fragment}],..Default::default()}},
   "paste"=>{let Some(args)=args.as_ref()else{return Emit::default()};let Some(fragment)=args.get("fragment").and_then(|value|FromValue::from_value(value.clone()).ok())else{return Emit::default()};match puzzle3d_paste_operations(&fragment,&PastePlacement::default()){Ok(mutations)=>Emit{artifact_mutations:mutations,..Default::default()},Err(_)=>Emit::default()}},
   _=>Emit::default(),
  }
 }
 fn step_value<'a>(&'a mut self,request:&mut ArtifactReservedToolJobRequest<EditorApp<Puzzle3dPlayApp>>,cx:&mut StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{
  use semio_s_plugin_puzzle_jobs::{OutboxPhase,JobTurn};
  match self.outbox.phase(cx)?{OutboxPhase::Building=>return self.outbox.advance(cx).map_err(Into::into),OutboxPhase::Delivered|OutboxPhase::Retiring=>{self.outbox.retire_step(cx)?;return Ok(None)},OutboxPhase::Idle=>{}}
  if self.closing||cx.is_cancelled(){return self.outbox.settle(JobTurn::Cancelled,cx).map_err(Into::into)}if cx.should_yield(){return Ok(None)}
  if !request.raw_wire.is_empty(){let bytes=2*std::mem::size_of::<Vec<u8>>();let demand=semio_framework_value::RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()};if !cx.retained_work_can_enter(demand){return Ok(None)}let wire=std::mem::take(&mut request.raw_wire);cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return self.outbox.settle(JobTurn::Prepare(wire),cx).map_err(Into::into)}
  if self.completed{return self.outbox.settle(JobTurn::Complete,cx).map_err(Into::into)}
  let emit=Self::emit(request);cx.consume_fuel(1);if let Err(rejected)=request.completion.complete(Ok(emit),EphemeralEmit::default()){*self.rejection=Some(rejected);return self.outbox.settle(JobTurn::Fault(b"puzzle3d-clipboard-completion-rejected".to_vec()),cx).map_err(Into::into)}self.completed=true;self.outbox.settle(JobTurn::Complete,cx).map_err(Into::into)
 }
 fn demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
  if !self.outbox.terminal_is_empty(){return self.outbox.retirement_demands()}self.retirement.next_owner_demands::<semio_plugin::app::ArtifactToolCompletionRejection<EditorApp<Puzzle3dPlayApp>>>(self.rejection.is_some(),&None,body)
 }
}
impl semio_plugin::plugin_app_close_prelude::ArtifactReservedWork<EditorApp<Puzzle3dPlayApp>> for Puzzle3dClipboardJob{
 fn step<'a>(&'a mut self,request:&'a mut ArtifactReservedToolJobRequest<EditorApp<Puzzle3dPlayApp>>,cx:&mut StepContext<'_>,_:&mut semio_framework_job::OriginalJobReturnClaim<'_>)->semio_framework_job::InteractiveJobStep<'a>{semio_framework_job::InteractiveJobStep::from_callback(self.step_value(request,cx))}
 fn borrow_outcome<'a>(&'a self,descriptor:&'a semio_framework_job::JobOutcomeDescriptor)->Result<semio_framework_job::JobOutcomeView<'a>,semio_framework_value::ValueError>{self.outbox.borrow_outcome(descriptor)}
 fn begin_close(&mut self){self.closing=true}
 fn close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.demands(body)}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
  self.closing=true;let demand=self.demands(cx.retained_grant().maximum_copy_bytes)?;let step=if !self.outbox.terminal_is_empty(){semio_framework_job::close_original_retained_step(cx,demand,|grant|self.outbox.close_step(grant))?}else{semio_framework_job::close_original_effect_step(cx,demand,|effect|{if self.retirement.is_empty(){let turn=self.retirement.begin_granted(&mut self.rejection,effect);semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(semio_framework_value::RetainedCloneStep::Complete(Default::default()))).map_err(Into::into),work:turn.work})}else{self.retirement.step_granted(effect)}})?};Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{step})
 }
 fn terminal_is_empty(&self)->bool{self.closing&&self.rejection.is_none()&&self.retirement.is_empty()&&self.outbox.terminal_is_empty()}
}

/// ✂️ Copies then deletes the selected objects as one mutation list.
pub(super) fn puzzle3d_cut_operations(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, interaction: &InteractionView<'_>) -> Vec<Puzzle3dMutation> {
    puzzle3d_cut_operations_from(doc.snapshot.typed(), &Puzzle3dInteractionSnapshot::from_interaction(interaction))
}

pub fn puzzle3d_cut_operations_from(before: &Puzzle3dSnapshot, marks: &Puzzle3dInteractionSnapshot) -> Vec<Puzzle3dMutation> {
    let host = puzzle3d_scene_snapshot_from_document(before);
    let ids: Vec<String> = puzzle3d_selected_objects_from(marks, &host).into_iter().map(|object| object.id).collect();
    semio_s_artifact_puzzle_3d_commands_document::delete_selection::delete_selection_mutations(before, &ids, &HashSet::new(), &[], &[], &[])
}
