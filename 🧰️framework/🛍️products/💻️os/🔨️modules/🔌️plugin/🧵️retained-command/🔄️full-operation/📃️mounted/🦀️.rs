//! 📃️ Mounted result custody preserves an accepted plan, framing owner and every refusal until paid handoff or close.
use super::*;
#[path="🖼️ui/🦀️.rs"]
mod ui_copy;
pub use ui_copy::MountedUiCopy;
use super::pending_result_page::{PendingPublicationPage,PublicationPagePlan,PublicationPageIntent,PublicationPageSource,PublicationRawReason,PublicationPageWriteFailure};
use semio_framework_value::RetainedWorkOutcome;
/// 🧷️ Page framing may pause without replaying its accepted Store or outbox action.
pub struct MountedPagePublication{
 pub ui_copy:MountedUiCopy,pub interaction_close:Option<ControlledRetirement<Vec<semio_plugin_decl_emission::InteractionWrite>>>,
 pub plan:Option<PublicationPagePlan>,pub owner:Option<PendingPublicationPage>,
 failure:Option<PublicationPageWriteFailure>,failure_close:Option<ControlledRetirement<PublicationPageWriteFailure>>,
}
impl MountedPagePublication{
 pub const fn new()->Self{Self{interaction_close:None,ui_copy:MountedUiCopy::new(),plan:None,owner:None,failure:None,failure_close:None}}
 pub fn terminal_is_empty(&self)->bool{self.interaction_close.is_none()&&self.ui_copy.terminal_is_empty()&&self.plan.is_none()&&self.owner.is_none()&&self.failure.is_none()&&self.failure_close.is_none()}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(owner)=self.interaction_close.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of_val(&self.interaction_close),depth:1,..Default::default()})}else{owner.next_demand(body)}}
  if !self.ui_copy.terminal_is_empty(){return self.ui_copy.close_demands(body)}
  if let Some(owner)=self.failure_close.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<PublicationPageWriteFailure>>>(),depth:1,..Default::default()})}else{owner.next_demand(body)}}
  if self.failure.is_some(){return Ok(RetirementDemand{copy_bytes:2*size_of::<Option<PublicationPageWriteFailure>>()+size_of::<Option<ControlledRetirement<PublicationPageWriteFailure>>>(),depth:1,..Default::default()})}
  if let Some(owner)=self.owner.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<PendingPublicationPage>>(),depth:1,..Default::default()})}else{owner.close_demands(body)}}
  Ok(RetirementDemand{copy_bytes:usize::from(self.plan.is_some())*size_of::<Option<PublicationPagePlan>>(),depth:1,..Default::default()})
 }
 pub fn close_step(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{
  use semio_framework_value::RetirementPollExit as Exit;
  let result=(||->Result<RetainedCloneStep,Exit>{
   let grant=effect.grant();let body=grant.maximum_copy_bytes;
   if let Some(owner)=self.interaction_close.as_mut(){if !owner.terminal_is_empty(){return Exit::resume(owner.step(body,effect))}let copy=size_of_val(&self.interaction_close);if !plugin_grant_funds(RetirementDemand{copy_bytes:copy,depth:1,..Default::default()},grant)||grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}self.interaction_close=None;Exit::pay(effect,1)?;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))}
   if !self.ui_copy.terminal_is_empty(){return Ok(self.ui_copy.close_step(effect).result?)}
   let d=self.close_demands(body)?;
   if !plugin_grant_funds(d,grant)||grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
   if let Some(owner)=self.failure_close.as_mut(){if !owner.terminal_is_empty(){return Exit::resume(owner.step(body,effect))}self.failure_close=None;}
   else if self.failure.is_some(){self.failure_close=Some(ControlledRetirement::retain(self.failure.take().unwrap()));}
   else if let Some(owner)=self.owner.as_mut(){if !owner.terminal_is_empty(){return Ok(owner.close_step(effect).result?)}self.owner=None;}
   else if self.plan.is_some(){self.plan=None;}
   else{return Ok(RetainedCloneStep::Complete(Default::default()))}
   Exit::pay(effect,1)?;
   Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))
  })();
  Exit::settle(effect,result)
 }
}
impl<A:ArtifactApp> MountedTypedCommandFullOperation<A>{
 pub fn page_publication_pending(&self)->bool{self.page_publication.interaction_close.is_some()||self.page_publication.plan.is_some()||self.page_publication.owner.is_some()||self.page_publication.failure.is_some()||self.page_publication.failure_close.is_some()}
 /// 🪪️ A denied scalar acceptance leaves its immutable producer untouched.
 pub fn reserve_result_intent(&mut self,intent:PublicationPageIntent,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
  if self.page_publication_pending()||self.result_page.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"mounted result intent requires empty custody").into())}
  let d=RetirementDemand{copy_bytes:size_of::<Option<PublicationPagePlan>>(),depth:1,..Default::default()};let mut captured=false;
  semio_framework_job::close_original_retained_step(cx,d,|_|{self.page_publication.plan=Some(PublicationPagePlan{token:self.next_token(),intent});captured=true;RetainedWorkOutcome{work:1,result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}})?;Ok(captured)
 }
 /// 🏪️ The retained Store receipt, rather than a replayed commit, defines the delivered generation.
 pub fn reserve_store_result(&mut self,lane:TypedOperationResultLane,receipt:store::os_store::presence::LaneItemReceipt,revision:Option<[u8;32]>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
  let d=RetirementDemand{copy_bytes:size_of::<Option<PublicationPagePlan>>()+8*size_of::<u64>()+2*size_of::<[u8;32]>()+3*size_of::<bool>()+size_of::<Option<store::os_store::presence::LaneItemReceipt>>(),depth:1,..Default::default()};
  let mut captured=false;cx.with_original_retirement(|cx|{semio_framework_job::close_original_retained_step(cx,d,|_|{
   match lane{TypedOperationResultLane::Artifact=>{self.published_artifact=true;self.artifact_generation=receipt.generation_after;self.operation.generation=semio_framework_job::Generation(receipt.generation_after);if let Some(revision)=revision{self.canonical_revision=revision;self.operation.base_revision=semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().unwrap()));}},TypedOperationResultLane::Config=>{self.published_config=true;self.config_generation=receipt.generation_after;},TypedOperationResultLane::Draft=>self.draft_generation=receipt.generation_after,TypedOperationResultLane::Presence=>self.presence_generation=receipt.generation_after,TypedOperationResultLane::Transient=>self.transient_generation=receipt.generation_after,TypedOperationResultLane::WindowConfig=>{self.published_window_config=true;self.pending_window_config_receipt=None;},TypedOperationResultLane::WindowTransient=>{if let Some(authority)=self.window_transient_authority.as_mut(){authority.generation=receipt.generation_after;}},_=>unreachable!("accepted Store lane")};
   assert!(!self.page_publication_pending());self.page_publication.plan=Some(PublicationPagePlan{token:self.next_token(),intent:PublicationPageIntent::LaneReceipt{lane,generation_before:receipt.generation_before,generation_after:receipt.generation_after}});captured=true;RetainedWorkOutcome{work:1,result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
  })?;Ok(())})?;Ok(captured)
 }
 /// 🛑️ Cancellation changes the result selector only after custody from accepted actions has drained.
 pub fn reserve_cancelled_result(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
  let d=RetirementDemand{copy_bytes:size_of::<Option<PublicationPagePlan>>()+size_of::<bool>(),depth:1,..Default::default()};
  cx.with_original_retirement(|cx|{semio_framework_job::close_original_retained_step(cx,d,|_|{
   let lane=if self.terminal_fault.is_some(){TypedOperationResultLane::Fault}else{semio_plugin_decl_app::cancellation_result_lane(self.user_cancel_requested,false)};
   let reason=if self.terminal_fault.is_some(){PublicationRawReason::Fault}else if lane==TypedOperationResultLane::Terminal{PublicationRawReason::Empty}else{PublicationRawReason::Cancelled};
   self.page_publication.plan=Some(PublicationPagePlan{token:self.next_token(),intent:PublicationPageIntent::Raw{lane,reason}});self.ui_pending=false;RetainedWorkOutcome{work:1,result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
  })?;Ok(())})
 }
 /// 🪙️ Accepted page custody runs before freshness, cancellation or new semantic actions.
 pub fn advance_pending_result_page(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
  if !self.page_publication_pending(){return Ok(false)}
  if self.page_publication.interaction_close.is_some(){let d=self.page_publication.close_demands(cx.retained_grant().maximum_copy_bytes)?;cx.with_original_retirement(|cx|{semio_framework_job::close_original_effect_step(cx,d,|effect|self.page_publication.close_step(effect))?;Ok(())})?;return Ok(true)}
  if self.page_publication.failure.is_some()||self.page_publication.failure_close.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"mounted accepted page retains its original framing refusal for paid closure").into())}
  if self.page_publication.owner.is_none(){
   let plan=self.page_publication.plan.ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"mounted accepted page lost its frozen plan"))?;
   let mut d=PendingPublicationPage::capture_demands();d.copy_bytes+=size_of::<Option<PublicationPagePlan>>();
   let page=&mut self.page_publication;
   cx.with_original_retirement(|cx|{semio_framework_job::close_original_retained_step(cx,d,|g|{let child=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-size_of::<Option<PublicationPagePlan>>(),..g};let turn=PendingPublicationPage::capture(plan,child);let result=turn.result.map(|captured|match captured{Some((owner,mut p))=>{page.owner=Some(owner);page.plan=None;p.copied_bytes+=size_of::<Option<PublicationPagePlan>>();RetainedCloneStep::Progress(p)},None=>RetainedCloneStep::Progress(Default::default())});RetainedWorkOutcome{work:turn.work,result}})?;Ok(())})?;return Ok(true)
  }
  let owner=self.page_publication.owner.as_ref().unwrap();
  if !owner.backing_ready(){
   let d=owner.backing_demands(cx.retained_grant().maximum_copy_bytes)?;
   cx.with_original_retirement(|cx|{semio_framework_job::close_original_retained_step(cx,d,|g|self.page_publication.owner.as_mut().unwrap().advance_backing(g))?;Ok(())})?;return Ok(true)
  }
  if owner.is_framed(){
   if !owner.ready_to_handoff(){let d=owner.traversal_close_demands(cx.retained_grant().maximum_copy_bytes)?;cx.with_original_retirement(|cx|{semio_framework_job::close_original_effect_step(cx,d,|effect|self.page_publication.owner.as_mut().unwrap().close_traversal(effect))?;Ok(())})?;return Ok(true)}
   let coordination=size_of::<Option<PendingPublicationPage>>()+size_of::<Option<TypedOperationResultPage>>()+size_of::<bool>()+size_of::<MountedTypedCommandFullOperationStage>();
   let mut d=owner.handoff_demands()?;d.copy_bytes+=coordination;
   cx.with_original_retirement(|cx|{semio_framework_job::close_original_retained_step(cx,d,|g|{
    let turn=self.page_publication.owner.as_mut().unwrap().take_page(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-coordination,..g});
    let result=turn.result.map(|page|match page{Some((page,mut p))=>{self.page_publication.owner=None;assert!(self.result_page.is_none());self.result_page=Some(page);self.result_page_presented=false;self.stage=MountedTypedCommandFullOperationStage::AwaitingAck;p.copied_bytes+=coordination;RetainedCloneStep::Progress(p)},None=>RetainedCloneStep::Progress(Default::default())});
    RetainedWorkOutcome{work:turn.work,result}
   })?;Ok(())})?;return Ok(true)
  }
  let plan=owner.plan().unwrap();let error_copy=2*size_of::<Option<PublicationPageWriteFailure>>()+size_of::<MountedTypedCommandFullOperationStage>();
  let mut d=owner.write_demands(cx.retained_grant().maximum_copy_bytes.saturating_sub(error_copy))?;d.copy_bytes+=error_copy;

  if !plugin_grant_funds(d,cx.retained_grant())||cx.retained_grant().maximum_items==0||cx.fuel_remaining()==0{return Ok(true)}

  let source=match plan.intent{
   PublicationPageIntent::Ui=>match self.publication.as_ref(){Some(ArtifactToolCompletionValue::Emit(Ok(emit),_))=>PublicationPageSource::Ui(&emit.ui_scope),_=>return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"mounted UI page lost its immutable source").into())},
   PublicationPageIntent::Download=>match self.publication.as_ref(){Some(ArtifactToolCompletionValue::Download(Ok(download),_))=>PublicationPageSource::Download{filename:&download.filename,mime:&download.mime_type,encoding:download.encoding.as_deref(),bytes:u64::try_from(download.chunks.bytes()).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"download byte count exceeds result wire u64"))?},_=>return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"mounted download page lost its immutable source").into())},
   PublicationPageIntent::Raw{reason,..}=>{let bytes:&[u8]=match reason{PublicationRawReason::Empty=>b"",PublicationRawReason::AdmissionCancelled=>b"latest-wins command admission cancelled or rejected before worker publication",PublicationRawReason::Complete=>b"typed-operation-complete",PublicationRawReason::Cancelled=>b"typed-operation cancelled before its next publication unit",PublicationRawReason::Fault=>{if let Some(fault)=self.terminal_fault.as_ref(){fault.as_bytes()}else{match self.publication.as_ref(){Some(ArtifactToolCompletionValue::Emit(Err(fault),_)|ArtifactToolCompletionValue::Download(Err(fault),_))=>fault.bounded.as_bytes(),_=>return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"mounted fault page lost its bounded reason").into())}}}};PublicationPageSource::Raw{reason,bytes}},
   _=>PublicationPageSource::None,
  };
  let raw=matches!(plan.intent,PublicationPageIntent::Raw{..});let page=&mut self.page_publication;let stage=&mut self.stage;
  if raw{
   cx.with_original_retirement(|cx|{semio_framework_job::close_original_retained_step(cx,d,|g|{
    let overhead=error_copy;
    let turn=page.owner.as_mut().unwrap().write_raw_step(&source,RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-overhead,..g});
    let mut p=turn.progress;
    if let Err(failure)=turn.result{page.failure=Some(failure);*stage=MountedTypedCommandFullOperationStage::Retiring;p.copied_bytes+=error_copy;}
    RetainedWorkOutcome{work:u128::from(turn.work),result:Ok(RetainedCloneStep::Progress(p))}
   })?;Ok(())})?;
  }else{
   let grant=cx.retained_grant();let maximum_work=cx.fuel_remaining();let mut actual=None;
   let mut receiving=None;cx.with_original_retirement(|cx|{receiving=Some(cx.with_native_retirement(|decode|{actual=Some(page.owner.as_mut().unwrap().write_retirement_step(&source,RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-error_copy,..grant},maximum_work,decode));Ok::<(),ValueError>(())}));Ok(())})?;let receiving=receiving.expect("original retirement receiver returns its complete refusal");
   let mut p=RetainedCloneProgress::default();let mut work=0;
   if let Some(turn)=actual{p=turn.progress;work=turn.work;if let Err(failure)=turn.result{page.failure=Some(failure);}}
   if let Err(error)=receiving{match page.failure.as_mut(){Some(failure)=>failure.context=Some(error),None=>page.failure=Some(PublicationPageWriteFailure{primary:None,forwarding:None,context:Some(error)})}}
   if page.failure.is_some(){*stage=MountedTypedCommandFullOperationStage::Retiring;p.copied_items=1;p.copied_bytes+=error_copy;if work==0{work=1;}}
   cx.consume_fuel(work);if let Some(issued)=cx.receiving_issued_grant(){cx.consume_receiving_retained(issued,p)?;}else{cx.consume_retained(p)?;}

  }
  if self.page_publication.failure.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"mounted accepted page framing refused; original diagnostics retained for paid closure").into())}
  Ok(true)
 }
}
