//! 🏁️ Completed native replay keeps its actual result and every private publication root in one original caller custody.
use semio_os_store_engine::lifted_store::{ArtifactStore,HistoryFinalization};
use semio_os_store_engine_branch::transitions::{OriginalHistoryTransitionBatch,OriginalHistoryTransitionBatchOutput};
use original_replay_metadata::OriginalReplayMetadataPreparation;
use semio_os_spr::{Mutation,OpBinary,OpText};
use semio_os_store_replay::EditReplayResult;
use semio_os_store_ledger::{OriginalReplayDiagnostics,PendingCommandReport,original_replay_pending_report::OriginalReplayPendingReport,original_replay_supersessions::OriginalReplaySupersessions};
use semio_framework_value::{ToValue,FromValue,ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneProgress,RetainedCloneStep,retained_clone::RetainedCloneClose};
use semio_framework_job::{StepContext,OriginalSourceBinding,OriginalTurnFailure,OriginalConstructionDemand,admit_original_construction,OriginalConstructionAdmission,OriginalConstructionStatus,OriginalConstructionFailure,close_original_retained_step,close_original_effect_step,InteractiveJobCloseStep};
use semio_framework_value_native_owned::native_decoding::construction::NativeConstructionScope;
use std::mem::{ManuallyDrop,size_of};
/// 🏠️ The caller keeps one resident admission, its refusal and its settle marker across turns; a pending constructor settles before any other work.
#[derive(Default)]
pub struct OriginalReplayResident{admission:OriginalConstructionAdmission,failure:Option<OriginalConstructionFailure>,unit:Option<()>,close:RetainedCloneClose,constructing:bool}
fn drive<O>(cx:&mut StepContext<'_>,output:&mut Option<O>,admission:&mut OriginalConstructionAdmission,failure:&mut Option<OriginalConstructionFailure>,constructing:&mut bool,demand:OriginalConstructionDemand,construct:impl FnOnce(&mut NativeConstructionScope<'_,'_>)->Result<(O,RetainedCloneProgress),ValueError>)->Result<bool,OriginalTurnFailure>{
 if failure.is_some(){return Err(invalid("completed replay construction was refused").into())}
 let status=admit_original_construction(cx,output,admission,demand,1,|scope,_|{*constructing=true;semio_framework_value::RetainedWorkOutcome{result:construct(scope),work:1}})?;
 match status{
  OriginalConstructionStatus::Ready=>{if admission.reset_step(cx)?{*constructing=false;Ok(true)}else{Ok(false)}}
  OriginalConstructionStatus::Pending=>Ok(false),
  OriginalConstructionStatus::Failed=>{if !admission.take_failure_into_step(failure,cx)?{return Ok(false)}*constructing=false;Err(invalid("completed replay construction was refused").into())}
 }
}
impl OriginalReplayResident{
 pub fn constructing(&self)->bool{self.constructing}
 pub fn terminal_is_empty(&self)->bool{self.admission.terminal_is_empty()&&self.failure.is_none()&&self.unit.is_none()&&self.close.is_empty()&&!self.constructing}
 pub fn built<O>(&mut self,cx:&mut StepContext<'_>,output:&mut Option<O>,demand:OriginalConstructionDemand,construct:impl FnOnce()->Result<(O,RetainedCloneProgress),ValueError>)->Result<bool,OriginalTurnFailure>{drive(cx,output,&mut self.admission,&mut self.failure,&mut self.constructing,demand,|_|construct())}
 pub fn built_with<O>(&mut self,cx:&mut StepContext<'_>,output:&mut Option<O>,demand:OriginalConstructionDemand,construct:impl FnOnce(&mut NativeConstructionScope<'_,'_>)->Result<(O,RetainedCloneProgress),ValueError>)->Result<bool,OriginalTurnFailure>{drive(cx,output,&mut self.admission,&mut self.failure,&mut self.constructing,demand,construct)}
 pub fn action(&mut self,cx:&mut StepContext<'_>,d:RetirementDemand,f:impl FnOnce(semio_framework_value::RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>)->Result<bool,OriginalTurnFailure>{let grant=cx.retained_grant();let done=drive(cx,&mut self.unit,&mut self.admission,&mut self.failure,&mut self.constructing,construction(d),|_|f(grant).map(|p|((),p)))?;if done{self.unit=None}Ok(done)}
 /// 🔁️ Finishes a construction whose constructor already ran; the candidate was placed by that constructor.
 pub fn settle(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let done=drive(cx,&mut Some(()),&mut self.admission,&mut self.failure,&mut self.constructing,construction(RetirementDemand::default()),|_|Err(invalid("completed replay construction resumed without its constructor")))?;if done{self.unit=None}Ok(done)}
 /// 📏️ Quotes the next resident close action.
 pub fn close_demands(&self,body:usize)->RetirementDemand{let _=body;fixed(size_of::<OriginalConstructionAdmission>()+size_of::<Option<OriginalConstructionFailure>>())}
 /// ♻️ Returns the resident refusal and admission frames to empty, one funded action per call.
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let body=cx.retained_grant().maximum_copy_bytes;
  if !self.close.is_empty(){let d=close_demand(&self.close,body)?;close_original_effect_step(cx,d,|effect|self.close.step_granted(effect.grant().maximum_copy_bytes,effect))?;return Ok(false)}
  if self.admission.failure_untaken()&&self.admission.take_failure_into_step(&mut self.failure,cx)?{return Ok(false)}
  if self.failure.is_some(){let d=self.close.next_owner_demands::<OriginalConstructionFailure>(true,&None,body)?;close_original_effect_step(cx,d,|effect|begun(&mut self.close,&mut self.failure,effect))?;return Ok(false)}
  if !self.admission.terminal_is_empty(){self.admission.close_step(cx)?;return Ok(false)}
  self.unit=None;self.constructing=false;Ok(true)
 }
}
fn retired(cx:&mut StepContext<'_>,demand:RetirementDemand,advance:impl FnOnce(semio_framework_value::RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{close_original_retained_step(cx,demand,|grant|semio_framework_value::RetainedWorkOutcome{result:advance(grant),work:1})}
/// 🫴️ Runs one receiving step through a local scope; `None` means the turn could not fund it and nothing ran.
pub fn receive<T>(cx:&mut StepContext<'_>,run:impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->T)->Result<Option<T>,OriginalTurnFailure>{
 let mut scope=None;let mut stepped=None;
 if !cx.with_native_receiving(&mut scope,|control,_|{stepped=Some(run(control));Ok(())})?{return Ok(None)}
 let scope=scope.ok_or_else(||invalid("completed replay receiving scope vanished"))?;
 if scope.source_refusal.is_some()||scope.call.is_err(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"completed replay receiving scope refused").with_retained_progress(scope.retained_progress()).into())}
 stepped.map(Some).ok_or_else(||invalid("completed replay receiving step vanished").into())
}
/// 🪙️ Settles one receiving turn's progress and work against the caller's own turn.
pub fn settle_receiving(cx:&mut StepContext<'_>,turn:semio_framework_value::RetainedWorkOutcome<RetainedCloneProgress>)->Result<(),OriginalTurnFailure>{semio_framework_job::receive_original_receiving_work(cx,semio_framework_value::RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}).map(|_|())}
/// ♻️ Runs one effect-shaped close that also needs the original retirement control; `false` means the turn could not fund it.
pub fn retire_controlled(cx:&mut StepContext<'_>,run:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>,&mut dyn semio_framework_value::OriginalRetirementControl)->semio_framework_value::OriginalRetirementPoll)->Result<bool,OriginalTurnFailure>{
 cx.with_original_retirement(|cx|{
  let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
  let Some((poll,performed))=cx.with_original_effect(grant,|cx,effect|cx.with_native_retirement(|control|Ok::<_,ValueError>(run(effect,control))))? else{return Ok(false)};
  let turn=match poll?{semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn,semio_framework_value::RetainedWorkPoll::Pending{work}=>semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work}};
  let progress=match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};
  cx.consume_fuel(u64::try_from(performed.max(turn.work)).unwrap_or(u64::MAX));cx.consume_retained(progress)?;
  turn.result.map_err(OriginalTurnFailure::from)?;Ok(true)
 })
}
/// 🪙️ Runs one effect-shaped owner action inside the turn's own performed-work effect; `false` means the turn could not fund it.
pub fn effected<E:Into<semio_framework_job::OriginalTurnPrimary>>(cx:&mut StepContext<'_>,run:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,E>)->Result<bool,OriginalTurnFailure>{
 let grant=cx.retained_grant();
 let Some((result,performed))=cx.with_original_effect(grant,|_,effect|run(effect))? else{return Ok(false)};
 semio_framework_job::receive_original_retained_work(cx,semio_framework_value::RetainedWorkOutcome{result:result.map(RetainedCloneStep::Progress),work:performed})?;Ok(true)
}
/// 📚️ One metadata turn: the original root read and source return run in their own effects, digests in a receiving scope.
fn advance_metadata<P,M>(cx:&mut StepContext<'_>,store:&mut ArtifactStore<P,M>,owner:&mut OriginalReplayMetadataPreparation<M>,result:&EditReplayResult<P,M>,order:Option<&semio_os_vcs::HistoryPageStack<semio_os_vcs::ArtifactHistoryKey>>)->Result<(),OriginalTurnFailure>where P:semio_os_spr::StoreState,M:semio_os_spr::StoreMutationFor<P>+Send+Sync+'static{
 if owner.root_wanted(){effected(cx,|effect|owner.advance_root(store,effect))?;return Ok(())}
 if owner.return_wanted(){effected(cx,|effect|owner.return_step(store,effect))?;return Ok(())}
 let grant=cx.retained_grant();let fuel=cx.fuel_remaining();
 if let Some(turn)=receive(cx,|native|owner.advance_in_order(store,result,order,native,grant,fuel))?{settle_receiving(cx,turn)?;}Ok(())
}
/// 🪙️ Settles an effect-shaped close that reports through a plain result.
pub fn settled(effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,result:Result<RetainedCloneStep,ValueError>)->semio_framework_value::OriginalRetirementPoll{semio_framework_value::RetirementPollExit::settle(effect,result.map_err(semio_framework_value::RetirementPollExit::from))}
fn begun<T:semio_framework_value::retirement::RetireOwned>(close:&mut RetainedCloneClose,slot:&mut Option<T>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{let turn=close.begin_granted(slot,effect);semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(semio_framework_value::OriginalRetirementFailure::from),work:turn.work})}
#[path="📚️metadata/🦀️.rs"]
pub mod original_replay_metadata;
#[path="⚠️failure/🦀️.rs"]
pub mod failure;
pub use failure::ArtifactOriginalVcsFailure;
#[path="checkpoint-order.rs"]
pub mod checkpoint_order;
pub use checkpoint_order::OriginalCheckpointEditOrder;
#[path="📬️publication/🦀️.rs"]mod publication;
use publication::OriginalReplayStage;

#[path="repin.rs"]
pub mod repin;
#[path="abort.rs"]
pub mod abort;
/// 🧾️ The published frontier is fixed metadata; original event bytes remain in the exact native outbox.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalReplayPublicationReceipt{pub generation_before:u64,pub generation_after:u64,pub revision:[u8;32],pub applies_to_head:bool}
/// 🧳️ A captured result owns every partial preparation until a single publication decision or paid cancellation.
pub struct OriginalReplayPublication<P,M:Mutation<P>+Sync>{binding:Option<OriginalSourceBinding>,caller:OriginalSourceBinding,frontier:(usize,u64,u64,[u8;32]),decoder:(semio_framework_value::retained_clone::NativeAllocationIdentity,u64),result:ManuallyDrop<Option<Box<EditReplayResult<P,M>>>>,request:ManuallyDrop<Option<HistoryFinalization>>,edit_ids:ManuallyDrop<Option<Vec<semio_os_spr::MutationId>>>,transitions:Option<Box<OriginalHistoryTransitionBatch>>,output:ManuallyDrop<Option<OriginalHistoryTransitionBatchOutput>>,metadata:Option<Box<OriginalReplayMetadataPreparation<M>>>,diagnostics:Option<Box<OriginalReplayDiagnostics>>,pending:Option<Box<OriginalReplayPendingReport>>,report:ManuallyDrop<Option<PendingCommandReport>>,overlay:OriginalReplaySupersessions,retirement:RetainedCloneClose,stage:Option<Box<OriginalReplayStage<M>>>,receipt:Option<OriginalReplayPublicationReceipt>,receipt_taken:bool,acknowledged:bool,phase:u8,closing:bool,resident:OriginalReplayResident}
fn invalid(text:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,text)}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn progress(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn construction(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
fn close_demand(owner:&RetainedCloneClose,body:usize)->Result<RetirementDemand,ValueError>{owner.demands(body)}
impl<P,M:Mutation<P>+Sync> OriginalReplayPublication<P,M>{
 /// 📏️ Source and original result frames are independently visible in the capture quote.
 pub fn capture_demands()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Self>()+2*size_of::<EditReplayResult<P,M>>(),capacity_bytes:size_of::<Self>()+size_of::<EditReplayResult<P,M>>(),depth:1,..Default::default()}}
 /// 🫙️ Every actual source, result, private ledger and child frame must return before the publisher shell is released.
 pub fn terminal_is_empty(&self)->bool{self.binding.is_none()&&self.result.is_none()&&self.request.is_none()&&self.edit_ids.is_none()&&self.transitions.is_none()&&self.output.is_none()&&self.metadata.is_none()&&self.diagnostics.is_none()&&self.pending.is_none()&&self.report.is_none()&&self.overlay.terminal_is_empty()&&self.retirement.is_empty()&&self.stage.is_none()&&self.resident.terminal_is_empty()}
 /// 🏁️ Preparation has no effect on visible Store roots.
 pub fn prepared(&self)->bool{self.phase==10&&!self.closing&&self.receipt.is_none()}
}
impl<P,M> OriginalReplayPublication<P,M>where P: semio_os_spr::StoreState,M: semio_os_spr::StoreMutationFor<P>{
 fn validate(&self,store:&ArtifactStore<P,M>,cx:&StepContext<'_>,frontier:bool)->Result<(),ValueError>{let parts=store.parts_ref();if !self.matches_caller(cx)||self.frontier.0!=parts.owners.snapshot_read_leases.identity()||self.frontier.1!=parts.log.envelope.vcs.edits.retained_issuance(){return Err(invalid("completed replay changed its original caller, Store or history issuer"))}if frontier&&(self.frontier.2!=parts.cursor.generation||self.frontier.3!=parts.cursor.content_revision){return Err(invalid("completed replay publication frontier changed"))}Ok(())}
 /// 🪪️ Every original publication step uses the same caller source and consuming decoder recipient that produced the Report.
 pub fn matches_caller(&self,cx:&StepContext<'_>)->bool{self.caller.matches(cx)&&cx.native_receiving_identity()==Some(self.decoder)}
 /// 🎟️ Refusal preserves both caller slots; admitted capture moves the same original result without copying its payloads.
 pub fn admit(store:&ArtifactStore<P,M>,result:&mut Option<EditReplayResult<P,M>>,request:&mut Option<HistoryFinalization>,cx:&mut StepContext<'_>,output:&mut Option<Box<Self>>,resident:&mut OriginalReplayResident)->Result<bool,OriginalTurnFailure>{if resident.constructing(){return resident.settle(cx)}if result.as_ref().is_some_and(|result|result.original_selection.is_some()){return Err(invalid("selected history replay requires its original edit-step intent").into())}Self::capture(store,result,request,&mut None,cx,output,resident)}
 pub fn admit_edit_step(store:&ArtifactStore<P,M>,result:&mut Option<EditReplayResult<P,M>>,edit_ids:&mut Option<Vec<semio_os_spr::MutationId>>,cx:&mut StepContext<'_>,output:&mut Option<Box<Self>>,resident:&mut OriginalReplayResident)->Result<bool,OriginalTurnFailure>{if resident.constructing(){return resident.settle(cx)}if result.as_ref().is_none_or(|result|result.original_selection.is_none())||edit_ids.as_ref().is_none_or(Vec::is_empty){return Err(invalid("native edit-step publication requires its original selected result and IDs").into())}Self::capture(store,result,&mut None,edit_ids,cx,output,resident)}
 fn capture(store:&ArtifactStore<P,M>,result:&mut Option<EditReplayResult<P,M>>,request:&mut Option<HistoryFinalization>,edit_ids:&mut Option<Vec<semio_os_spr::MutationId>>,cx:&mut StepContext<'_>,output:&mut Option<Box<Self>>,resident:&mut OriginalReplayResident)->Result<bool,OriginalTurnFailure>{
  if output.is_some(){return Err(invalid("completed replay publication destination is occupied").into())}let parts=store.parts_ref();let original=result.as_ref().ok_or_else(||invalid("completed replay source is absent"))?;let frontier=(parts.owners.snapshot_read_leases.identity(),parts.log.envelope.vcs.edits.retained_issuance(),parts.cursor.generation,parts.cursor.content_revision);
  if original.original_history!=Some((frontier.0,frontier.1))||original.revision!=frontier.3{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"completed replay belongs to another original history frontier").into())}if original.original_selection.is_none()&&original.drafts.is_empty()||original.state.is_none()||original.report.blocks_finalize(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"completed replay cannot publish empty, missing or blocking semantic output").into())}if request.is_none()&&edit_ids.is_none(){return Err(invalid("completed replay finalization request is absent").into())}store.ensure_durable_group_idle().map_err(|_|ValueError::literal(ValueRefusalKind::WorkLimit,"completed replay waits for its original durable group"))?;if parts.log.envelope.open_transaction.is_some()||store.local_step_pending(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"completed replay waits for the original Store writer").into())}
  let decoder=original.original_decode.filter(|identity|Some(*identity)==cx.native_receiving_identity()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"completed Report requires the same original decoder ledger and retirement recipient"))?;let binding=OriginalSourceBinding::borrow(cx);let d=Self::capture_demands();resident.built(cx,output,construction(d),||Ok((Box::new(Self{binding:Some(binding),caller:binding,frontier,decoder,result:ManuallyDrop::new(Some(Box::new(result.take().unwrap()))),request:ManuallyDrop::new(request.take()),edit_ids:ManuallyDrop::new(edit_ids.take()),transitions:None,output:ManuallyDrop::new(None),metadata:None,diagnostics:None,pending:None,report:ManuallyDrop::new(None),overlay:Default::default(),retirement:Default::default(),stage:None,receipt:None,receipt_taken:false,acknowledged:false,phase:0,closing:false,resident:Default::default()}),progress(d))))
 }
 /// 🪡️ The original transition, effective rows, metadata and diagnostics each advance through one funded owner action.
 pub fn advance(&mut self,store:&mut ArtifactStore<P,M>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(store,cx,true)?;if self.resident.constructing(){self.resident.settle(cx)?;return Ok(self.prepared())}if self.closing||cx.is_cancelled(){return Ok(false)}if self.prepared(){return Ok(true)}let result=self.result.as_mut().ok_or_else(||invalid("completed replay source was already transferred"))?;
  match self.phase{
   0=>{if let Some(ids)=self.edit_ids.as_ref(){let selection=result.original_selection.unwrap();let view=semio_os_spr::HistoryTransitionEditsView{mode:if selection.restoring{semio_os_spr::HistoryTransitionEditMode::Reinstate}else{semio_os_spr::HistoryTransitionEditMode::Revert},mutation_ids:ids};if self.transitions.is_none(){OriginalHistoryTransitionBatch::admit_edits(store,&view,cx,&mut self.transitions)?;}else if !self.transitions.as_ref().unwrap().ready(){self.transitions.as_mut().unwrap().advance_edits(store,&view,cx)?;}else{self.transitions.as_mut().unwrap().take(cx,&mut self.output)?;if self.output.is_some(){self.phase=1;}}}else if self.transitions.is_none(){OriginalHistoryTransitionBatch::admit(store,self.request.as_ref().unwrap(),&result.drafts,cx,&mut self.transitions)?;}else if !self.transitions.as_ref().unwrap().ready(){let source=&mut*result.original;self.transitions.as_mut().unwrap().advance(store,self.request.as_ref().unwrap(),&source.drafts,&mut source.conflict_target,cx)?;}else{self.transitions.as_mut().unwrap().take(cx,&mut self.output)?;if self.output.is_some(){self.phase=1;}}},
   1=>{let owner=self.transitions.as_mut().ok_or_else(||invalid("history replay transitions owner is absent"))?;if !owner.terminal_is_empty(){owner.close_step(cx)?;}else{let d=RetirementDemand{release_bytes:size_of::<OriginalHistoryTransitionBatch>(),..fixed(size_of::<Option<Box<OriginalHistoryTransitionBatch>>>())};retired(cx,d,|_|{self.transitions=None;self.phase=2;Ok(RetainedCloneStep::Progress(progress(d)))})?;}},
   2=>{if self.edit_ids.is_some(){self.resident.action(cx,fixed(1),|_|{self.phase=3;Ok(progress(fixed(1)))})?;}else if !self.output.as_ref().unwrap().applies_to_head{self.resident.action(cx,fixed(1),|_|{self.phase=8;Ok(progress(fixed(1)))})?;}else{let output=self.output.as_ref().unwrap();let event=output.envelopes.last().ok_or_else(||invalid("completed replay has no original Supersede event"))?;let scope=match self.request.as_ref().unwrap(){HistoryFinalization::Overwrite=>None,HistoryFinalization::Scope{alternative_id}=>Some(alternative_id.as_str()),HistoryFinalization::Alternative{..}=>Some(output.branch.as_ref().ok_or_else(||invalid("alternative replay lacks original branch facts"))?.alternative_id.as_str())};let source=&mut*result.original;let d=self.overlay.demands(&source.drafts,&source.supersessions,event,scope,cx.retained_grant().maximum_copy_bytes)?;effected(cx,|effect|self.overlay.advance(&mut source.drafts,&mut source.supersessions,event,scope,effect))?;if self.overlay.prepared(){self.phase=3;}}},
   3=>{if !self.overlay.terminal_is_empty(){let d=self.overlay.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_effect_step(cx,d,|effect|self.overlay.close_step(effect))?;}else{let identity=store.parts_ref().cursor.revision_accumulator.identity_digest;let d=RetirementDemand{capacity_bytes:size_of::<OriginalReplayMetadataPreparation<M>>(),..fixed(2*size_of::<OriginalReplayMetadataPreparation<M>>())};self.resident.built(cx,&mut self.metadata,construction(d),||Ok((Box::new(OriginalReplayMetadataPreparation::new(self.frontier,identity,matches!(self.request.as_ref(),Some(HistoryFinalization::Alternative{..})))),progress(d))))?;if self.metadata.is_some(){self.phase=4;}}},
   4=>{let owner=self.metadata.as_mut().unwrap();if owner.prepared(){self.phase=5;}else{let grant=cx.retained_grant();let fuel=cx.fuel_remaining();advance_metadata(cx,store,owner,result,None)?;}},
   5=>{if self.diagnostics.is_none(){let d=RetirementDemand{capacity_bytes:size_of::<OriginalReplayDiagnostics>(),..fixed(2*size_of::<OriginalReplayDiagnostics>())};self.resident.built(cx,&mut self.diagnostics,construction(d),||Ok((Box::new(OriginalReplayDiagnostics::default()),progress(d))))?;}else{let baseline=&store.parts_ref().log.envelope.edit_messages;let owner=self.diagnostics.as_mut().unwrap();let d=owner.demands(baseline,&result.replayed,cx.retained_grant().maximum_copy_bytes)?;self.resident.action(cx,d,|grant|owner.advance(baseline,&mut result.replayed,grant).map(|step|step.progress()))?;if owner.prepared(){self.phase=6;}}},
   6=>{if self.pending.is_none(){let d=RetirementDemand{capacity_bytes:size_of::<OriginalReplayPendingReport>(),..fixed(2*size_of::<OriginalReplayPendingReport>())};self.resident.built(cx,&mut self.pending,construction(d),||Ok((Box::new(OriginalReplayPendingReport::default()),progress(d))))?;}else{let owner=self.pending.as_mut().unwrap();let d=owner.demands(&result.report)?;self.resident.action(cx,d,|grant|owner.advance(&mut result.report,self.frontier.2,grant,|_|Ok(())))?;if owner.prepared(){self.phase=7;}}},
   7=>{let d=fixed(size_of::<PendingCommandReport>()+128);self.resident.action(cx,d,|grant|{let Some((report,p))=self.pending.as_mut().unwrap().take(grant)?else{return Ok(Default::default())};*self.report=Some(report);self.phase=8;Ok(p)})?;},
   8=>{self.resident.action(cx,fixed(1),|_|{self.phase=9;Ok(progress(fixed(1)))})?;},
   9=>{if self.stage.is_none(){let d=RetirementDemand{capacity_bytes:size_of::<OriginalReplayStage<M>>(),..fixed(2*size_of::<OriginalReplayStage<M>>())};self.resident.built(cx,&mut self.stage,construction(d),||Ok((Box::new(OriginalReplayStage::new()),progress(d))))?;}else if self.stage.as_mut().unwrap().advance(store,result,self.output.as_mut().unwrap(),cx)?{self.phase=10;}},
   _=>return Err(invalid("completed replay preparation phase changed").into()),
  }Ok(self.prepared())
 }
 /// 🪟️ One funded decision moves every prepared original head and appends the same paged message and causal IDs.
 pub fn publish(&mut self,store:&mut ArtifactStore<P,M>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(store,cx,true)?;if self.resident.constructing(){self.resident.settle(cx)?;return Ok(self.receipt.is_some())}if !self.prepared(){return Err(invalid("completed replay publication is not prepared").into())}let stage=self.stage.as_mut().unwrap();let d=stage.publication_demands(store,self.diagnostics.as_deref())?;let before=self.frontier.2;let output=self.output.as_ref().unwrap();let applied=output.applies_to_head;let mut published=false;self.resident.action(cx,d,|grant|{let p=stage.publish(store,self.result.as_mut().unwrap(),output,self.metadata.as_deref_mut(),self.diagnostics.as_deref_mut(),&mut self.report,grant)?;if p.copied_items!=0{self.receipt=Some(OriginalReplayPublicationReceipt{generation_before:before,generation_after:store.parts_ref().cursor.generation,revision:store.parts_ref().cursor.content_revision,applies_to_head:applied});published=true;}Ok(p)})?;Ok(published)}
 /// 🧾️ Receipt custody moves only under the original caller, after the actual common visibility decision.
 pub fn take_receipt(&mut self,cx:&mut StepContext<'_>,output:&mut Option<OriginalReplayPublicationReceipt>)->Result<bool,OriginalTurnFailure>{if self.resident.constructing(){return self.resident.settle(cx)}if !self.matches_caller(cx)||self.receipt_taken{return Err(invalid("completed replay receipt changed its original recipient").into())}let receipt=self.receipt.ok_or_else(||invalid("completed replay is unpublished"))?;let d=fixed(size_of::<OriginalReplayPublicationReceipt>()+size_of::<bool>());self.resident.built(cx,output,construction(d),||{self.receipt_taken=true;Ok((receipt,progress(d)))})}
 /// 🤝️ The original driver acknowledges its fixed receipt before retiring the retained publication owner.
 pub fn acknowledge(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if self.resident.constructing(){return self.resident.settle(cx)}if !self.matches_caller(cx)||!self.receipt_taken||self.acknowledged{return Err(invalid("completed replay acknowledgement changed its original recipient").into())}self.resident.action(cx,fixed(size_of::<bool>()),|_|{self.acknowledged=true;Ok(progress(fixed(size_of::<bool>())))})}
 /// 📏️ The real Store supplies original source ledgers while the mounted parent quotes one exact contextual close action.
 pub fn close_demands(&self,store:&ArtifactStore<P,M>,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(owner)=self.stage.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of::<OriginalReplayStage<M>>(),..fixed(size_of::<Option<Box<OriginalReplayStage<M>>>>())})}else{owner.close_demands(store,body)}}if let Some(owner)=self.transitions.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of::<OriginalHistoryTransitionBatch>(),..fixed(size_of::<Option<Box<OriginalHistoryTransitionBatch>>>())})}else{owner.close_demands(body)}}if !self.overlay.terminal_is_empty(){return self.overlay.close_demands(body)}if let Some(owner)=self.metadata.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of::<OriginalReplayMetadataPreparation<M>>(),..fixed(size_of::<Option<Box<OriginalReplayMetadataPreparation<M>>>>())})}else{owner.close_demands(store,body)}}
  macro_rules! child{($field:ident,$ty:ty)=>{if let Some(owner)=self.$field.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of::<$ty>(),..fixed(size_of::<Option<Box<$ty>>>())})}else{owner.close_demands(body)}}};}child!(diagnostics,OriginalReplayDiagnostics);child!(pending,OriginalReplayPendingReport);if !self.retirement.is_empty(){return close_demand(&self.retirement,body)}macro_rules! payload{($field:ident,$ty:ty)=>{if self.$field.is_some(){return self.retirement.next_owner_demands::<$ty>(true,&None,body)}};}payload!(output,OriginalHistoryTransitionBatchOutput);payload!(report,PendingCommandReport);payload!(request,HistoryFinalization);payload!(edit_ids,Vec<semio_os_spr::MutationId>);if let Some(result)=self.result.as_ref(){return if result.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of::<EditReplayResult<P,M>>(),..fixed(size_of::<Option<Box<EditReplayResult<P,M>>>>())})}else{result.retirement_demands(body)}}if !self.resident.terminal_is_empty(){return Ok(self.resident.close_demands(body))}Ok(fixed(size_of::<Option<OriginalSourceBinding>>()))
 }
 /// ♻️ Cancellation closes codecs before their original source fields and releases every child shell separately.
 pub fn close_step(&mut self,store:&mut ArtifactStore<P,M>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(store,cx,false)?;self.closing=true;
  if let Some(stage)=self.stage.as_mut(){if !stage.terminal_is_empty(){stage.close_step(store,cx)?;}else{let d=RetirementDemand{release_bytes:size_of::<OriginalReplayStage<M>>(),..fixed(size_of::<Option<Box<OriginalReplayStage<M>>>>())};retired(cx,d,|_|{self.stage=None;Ok(RetainedCloneStep::Progress(progress(d)))})?;}return Ok(false)}
  if let Some(owner)=self.transitions.as_mut(){if !owner.terminal_is_empty(){owner.close_step(cx)?;}else{let d=RetirementDemand{release_bytes:size_of::<OriginalHistoryTransitionBatch>(),..fixed(size_of::<Option<Box<OriginalHistoryTransitionBatch>>>())};retired(cx,d,|_|{self.transitions=None;Ok(RetainedCloneStep::Progress(progress(d)))})?;}return Ok(false)}
  if !self.overlay.terminal_is_empty(){let d=self.overlay.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_effect_step(cx,d,|effect|self.overlay.close_step(effect))?;return Ok(false)}
  if let Some(owner)=self.metadata.as_mut(){if !owner.terminal_is_empty(){retire_controlled(cx,|effect,native|owner.close_step(store,native,effect))?;}else{let d=RetirementDemand{release_bytes:size_of::<OriginalReplayMetadataPreparation<M>>(),..fixed(size_of::<Option<Box<OriginalReplayMetadataPreparation<M>>>>())};retired(cx,d,|_|{self.metadata=None;Ok(RetainedCloneStep::Progress(progress(d)))})?;}return Ok(false)}
  if let Some(owner)=self.diagnostics.as_mut(){owner.begin_close();}
  if let Some(owner)=self.diagnostics.as_mut(){if !owner.terminal_is_empty(){let d=owner.close_demands(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_effect_step(cx,d,|effect|{let result=owner.close_step(effect).map_err(semio_framework_value::RetirementPollExit::from);semio_framework_value::RetirementPollExit::settle(effect,result)})?;}else{let d=RetirementDemand{release_bytes:size_of::<OriginalReplayDiagnostics>(),..fixed(size_of::<Option<Box<OriginalReplayDiagnostics>>>())};retired(cx,d,|_|{self.diagnostics=None;Ok(RetainedCloneStep::Progress(progress(d)))})?;}return Ok(false)}
  macro_rules! child{($field:ident,$ty:ty)=>{if let Some(owner)=self.$field.as_mut(){if !owner.terminal_is_empty(){let d=owner.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_effect_step(cx,d,|effect|owner.close_step(effect))?;}else{let d=RetirementDemand{release_bytes:size_of::<$ty>(),..fixed(size_of::<Option<Box<$ty>>>())};retired(cx,d,|_|{self.$field=None;Ok(RetainedCloneStep::Progress(progress(d)))})?;}return Ok(false)}};}child!(pending,OriginalReplayPendingReport);
  if !self.retirement.is_empty(){let d=close_demand(&self.retirement,cx.retained_grant().maximum_copy_bytes)?;close_original_effect_step(cx,d,|effect|self.retirement.step_granted(effect.grant().maximum_copy_bytes,effect))?;return Ok(false)}
  macro_rules! payload{($field:ident,$ty:ty)=>{if self.$field.is_some(){let d=self.retirement.next_owner_demands::<$ty>(true,&None,cx.retained_grant().maximum_copy_bytes)?;close_original_effect_step(cx,d,|effect|begun(&mut self.retirement,&mut self.$field,effect))?;return Ok(false)}};}payload!(output,OriginalHistoryTransitionBatchOutput);payload!(report,PendingCommandReport);payload!(request,HistoryFinalization);payload!(edit_ids,Vec<semio_os_spr::MutationId>);
  if let Some(result)=self.result.as_mut(){if !result.terminal_is_empty(){result.begin_retirement();retire_controlled(cx,|effect,native|result.close_original_step_controlled(effect,native))?;}else{let d=RetirementDemand{release_bytes:size_of::<EditReplayResult<P,M>>(),..fixed(size_of::<Option<Box<EditReplayResult<P,M>>>>())};retired(cx,d,|_|{self.result.take();Ok(RetainedCloneStep::Progress(progress(d)))})?;}return Ok(false)}
  if !self.resident.terminal_is_empty(){self.resident.close_step(cx)?;return Ok(false)}
  retired(cx,fixed(size_of::<Option<OriginalSourceBinding>>()),|_|{self.binding=None;Ok(RetainedCloneStep::Complete(progress(fixed(size_of::<Option<OriginalSourceBinding>>()))))})?;Ok(self.terminal_is_empty())
 }
 /// 📏️ The caller releases this original shell only after all actual children have returned.
 pub fn boxed_release_demands()->RetirementDemand{RetirementDemand{release_bytes:size_of::<Self>(),..fixed(size_of::<Option<Box<Self>>>())}}
}
impl<P,M:Mutation<P>+Sync> Drop for OriginalReplayPublication<P,M>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"completed replay publication retains original owners");}}

#[path="checkpoint.rs"]
pub mod checkpoint;

#[path="checkout.rs"]
pub mod checkout;
