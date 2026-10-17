//! 🎟️ Canonical app cancellation is issued only by its retained original creation source.
use super::*;
use semio_framework_job::{OriginalCallerSourceLoan,OriginalCallerSourceParent,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retirement::controlled::RetainedOwnerGate;
use semio_framework_value::retirement::{RetireOwned,controlled::ControlledRetirement};
use std::{mem::{ManuallyDrop,MaybeUninit},sync::{Arc,atomic::{AtomicU64,AtomicUsize}}};

struct Admission{
 source:Option<OriginalCallerSourceLoan>,slots:Option<Box<[MaybeUninit<ToolDocumentCancellationScope>]>>,
 operations:Option<ArtifactFixedRegistry<ToolDocumentCancellationScope>>,documents:Option<ArtifactFixedRegistry<ToolKeyedDocumentCancellationScope>>,
 cancel:Option<semio_framework_job::CancelToken>,claim:Option<PublicationClaimHandle>,generation:Option<Arc<AtomicU64>>,active:Option<Arc<AtomicUsize>>,
 state:Option<Arc<RetainedOwnerGate<ToolCancellationState>>>,
 cancel_close:Option<semio_framework_async::CancelTokenRetirement>,claim_close:Option<ControlledRetirement<PublicationClaimHandle>>,
}
impl Admission{const fn empty()->Self{Self{source:None,slots:None,operations:None,documents:None,cancel:None,claim:None,generation:None,active:None,state:None,cancel_close:None,claim_close:None}}}


struct PendingLease {
 key:ToolOperationKey,source:(semio_framework_job::OperationId,semio_framework_job::Generation,usize),keyed:bool,
 document:Option<semio_framework_job::CancelToken>,token:Option<semio_framework_job::CancelToken>,scope_token:Option<semio_framework_job::CancelToken>,
 claim:Option<PublicationClaimHandle>,document_claim:Option<PublicationClaimHandle>,scope_claim:Option<PublicationClaimHandle>,lease_document_claim:Option<PublicationClaimHandle>,handle:Option<ToolCancellationHandle>,existing_document:bool,
}
impl PendingLease {
 fn into_parts(self)->crate::CancellationLeaseParts{
  crate::CancellationLeaseParts::unpublished(self.handle,[self.token,self.scope_token,self.document],[self.claim,self.lease_document_claim,self.scope_claim,self.document_claim],self.keyed.then_some(u64::from(self.key.document.0)))
 }
}

#[doc(hidden)]
pub struct ToolCancellationIssuer{admission:ManuallyDrop<Admission>,issued:ManuallyDrop<Option<ToolCancellationHandle>>,pending:ManuallyDrop<Option<PendingLease>>,pending_close:ManuallyDrop<Option<crate::CancellationLeaseParts>>,renew_cancel:ManuallyDrop<Option<semio_framework_job::CancelToken>>,renew_claim:ManuallyDrop<Option<PublicationClaimHandle>>,renew_source:Option<(semio_framework_job::OperationId,semio_framework_job::Generation,usize)>,renew_swapped:bool,cleanup_index:Option<usize>,closing:bool}
impl ToolCancellationIssuer{
 #[doc(hidden)]
 pub const fn empty()->Self{Self{admission:ManuallyDrop::new(Admission::empty()),issued:ManuallyDrop::new(None),pending:ManuallyDrop::new(None),pending_close:ManuallyDrop::new(None),renew_cancel:ManuallyDrop::new(None),renew_claim:ManuallyDrop::new(None),renew_source:None,renew_swapped:false,cleanup_index:None,closing:false}}
 #[doc(hidden)]
 pub fn ready(&self)->bool{self.issued.is_some()}
 #[doc(hidden)]
 pub fn handle(&self)->Option<&ToolCancellationHandle>{self.issued.as_ref()}
 #[doc(hidden)]
 pub fn admit_step(&mut self,parent:&OriginalCallerSourceParent,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"closing canonical cancellation issuer cannot admit more births").into())}
  if let Some(source)=self.admission.source.as_ref(){source.validate_context(cx)?}
  if self.ready(){return Ok(true)}
  if cx.should_yield()||cx.is_cancelled(){return Ok(false)}
  if self.admission.source.is_none(){self.admission.source=parent.admit_loan(cx)?;return Ok(false)}
  let grant=cx.retained_grant();let next=self.admission_demands();
  if grant.maximum_items==0||grant.maximum_copy_bytes<next.copy_bytes||grant.maximum_capacity_bytes<next.capacity_bytes||grant.maximum_depth<next.depth{return Ok(false)}
  let original=&mut*self.admission;
  if original.state.is_some(){
   *self.issued=Some(ToolCancellationHandle{state:original.state.take().unwrap(),app_scope:original.cancel.take().unwrap(),publication_scope:original.claim.take().unwrap(),app_generation:original.generation.take().unwrap(),active:original.active.take().unwrap()});
  }else if original.slots.is_none()&&original.operations.is_none(){
   let mut slots=Vec::new();slots.try_reserve_exact(TOOL_CANCELLATION_SLOTS).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original cancellation slot birth was refused"))?;slots.resize_with(TOOL_CANCELLATION_SLOTS,MaybeUninit::uninit);original.slots=Some(slots.into_boxed_slice());
  }else if original.operations.is_none(){
   let registry=ArtifactFixedRegistry::new();if !registry.allocation_admitted{return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"original cancellation operation registry birth was refused").into())}original.operations=Some(registry);
  }else if original.documents.is_none(){
   let registry=ArtifactFixedRegistry::new();if !registry.allocation_admitted{return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"original cancellation document registry birth was refused").into())}original.documents=Some(registry);
  }else if original.cancel.is_none(){
   let Some((cancel,progress))=parent.cancellation()?.admit_child(grant)?else{return Ok(false)};original.cancel=Some(cancel);cx.consume_retained(progress)?;cx.consume_fuel(1);return Ok(false)
  }else if original.claim.is_none(){
   let Some((claim,progress))=PublicationClaim::admit(grant)?else{return Ok(false)};original.claim=Some(claim);cx.consume_retained(progress)?;cx.consume_fuel(1);return Ok(false)
  }else if original.generation.is_none(){original.generation=Some(Arc::new(AtomicU64::new(0)))}
  else if original.active.is_none(){original.active=Some(Arc::new(AtomicUsize::new(0)))}
  else{original.state=Some(Arc::new(RetainedOwnerGate::new(ToolCancellationState{slots:original.slots.take().unwrap(),occupied:[0;TOOL_CANCELLATION_WORDS],allocation_admitted:true,keyed_operations:original.operations.take().unwrap(),keyed_documents:original.documents.take().unwrap()})))}
  cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:next.copy_bytes,retained_capacity_bytes:next.capacity_bytes,..Default::default()})?;cx.consume_fuel(1);Ok(false)
 }
 fn admission_demands(&self)->RetirementDemand{
  let source=&*self.admission;
  let capacity_bytes=if source.state.is_some(){0}else if source.slots.is_none()&&source.operations.is_none(){TOOL_CANCELLATION_SLOTS*size_of::<ToolDocumentCancellationScope>()}else if source.operations.is_none(){semio_framework_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS*size_of::<(u64,ToolDocumentCancellationScope)>()}else if source.documents.is_none(){semio_framework_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS*size_of::<(u64,ToolKeyedDocumentCancellationScope)>()}else if source.cancel.is_none(){semio_framework_job::CancelToken::root_admission_demands().capacity_bytes}else if source.claim.is_none(){PublicationClaim::admission_demands().capacity_bytes}else if source.generation.is_none(){semio_framework_value::shared_retirement_allocation_bytes::<AtomicU64>()}else if source.active.is_none(){semio_framework_value::shared_retirement_allocation_bytes::<AtomicUsize>()}else{semio_framework_value::shared_retirement_allocation_bytes::<RetainedOwnerGate<ToolCancellationState>>()};
  RetirementDemand{copy_bytes:if source.state.is_some(){size_of::<ToolCancellationHandle>()}else{0},capacity_bytes,depth:1,..Default::default()}
 }
 #[doc(hidden)]
 pub fn begin(&mut self,key:&ToolOperationKey,cx:&mut StepContext<'_>)->Result<Option<ToolCancellationLease>,OriginalTurnFailure>{self.begin_original(key,false,cx)}
 #[doc(hidden)]
 pub fn begin_keyed(&mut self,key:&ToolOperationKey,cx:&mut StepContext<'_>)->Result<Option<ToolCancellationLease>,OriginalTurnFailure>{self.begin_original(key,true,cx)}
 fn begin_original(&mut self,key:&ToolOperationKey,keyed:bool,cx:&mut StepContext<'_>)->Result<Option<ToolCancellationLease>,OriginalTurnFailure>{
  if self.closing||!self.ready()||self.pending_close.is_some()||cx.should_yield()||cx.is_cancelled(){return Ok(None)}
  let source=(cx.operation(),cx.generation(),cx.original_source_parent()?.source_identity()?);
  if let Some(pending)=self.pending.as_ref(){if pending.key!=*key||pending.keyed!=keyed{return Ok(None)}if pending.source!=source{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"pending cancellation lease belongs to another original source").into())}}
  let canonical=self.issued.as_ref().unwrap();let grant=cx.retained_grant();
  if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
  if self.pending.is_none(){
   if grant.maximum_copy_bytes<size_of::<PendingLease>(){return Ok(None)}
   let state=match canonical.state.try_lock(){Ok(state)=>state,Err(_)=>return Ok(None)};
   let existing_document=if keyed{
    if !state.keyed_operations.can_insert(key.operation_id.0){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"exact keyed cancellation operation slot is unavailable").into())}
    if let Some(document)=state.keyed_documents.get(u64::from(key.document.0)){if document.token.is_cancelled_now(){return Ok(None)}true}else{if !state.keyed_documents.can_insert(u64::from(key.document.0)){return Ok(None)}false}
   }else{
    if let Some(previous)=state.get(ToolCancellationHandle::slot(key.document)){if previous.document!=key.document{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"fixed cancellation document slot collision").into())}previous.token.cancel_now();previous.publication_claim.cancel();return Ok(None)}false
   };drop(state);
   *self.pending=Some(PendingLease{key:key.clone(),source,keyed,document:None,token:None,scope_token:None,claim:None,document_claim:None,scope_claim:None,lease_document_claim:None,handle:None,existing_document});
   cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<PendingLease>(),..Default::default()})?;cx.consume_fuel(1);return Ok(None)
  }
  let pending=self.pending.as_mut().unwrap();let progress;
  if !pending.existing_document&&pending.document.is_none(){let Some((token,paid))=canonical.app_scope.admit_child(grant)?else{return Ok(None)};pending.document=Some(token);progress=paid;}
  else if keyed&&!pending.existing_document&&pending.document_claim.is_none(){let Some((claim,paid))=PublicationClaim::admit(grant)?else{return Ok(None)};pending.document_claim=Some(claim);progress=paid;}
  else if pending.token.is_none(){
   let admitted=if let Some(document)=pending.document.as_ref(){document.admit_child(grant)?}else{let state=match canonical.state.try_lock(){Ok(state)=>state,Err(_)=>return Ok(None)};let document=state.keyed_documents.get(u64::from(key.document.0)).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"pending keyed document original authority disappeared"))?;if document.token.is_cancelled_now(){return Ok(None)}document.token.admit_child(grant)?};
   let Some((token,paid))=admitted else{return Ok(None)};pending.token=Some(token);progress=paid;
  }else if pending.claim.is_none(){let Some((claim,paid))=PublicationClaim::admit(grant)?else{return Ok(None)};pending.claim=Some(claim);progress=paid;}
  else if pending.handle.is_none(){if grant.maximum_copy_bytes<size_of::<ToolCancellationHandle>(){return Ok(None)}let Some((handle,paid))=canonical.admit_alias(grant)?else{return Ok(None)};pending.handle=Some(handle);progress=paid;}
  else if pending.scope_claim.is_none(){
   let bytes=size_of::<PublicationClaimHandle>()*if keyed{2}else{1}+if keyed{size_of::<semio_framework_job::CancelToken>()}else{0};if grant.maximum_copy_bytes<bytes{return Ok(None)}
   if keyed{let document_claim=if let Some(claim)=pending.document_claim.as_ref(){claim.clone()}else{let state=match canonical.state.try_lock(){Ok(state)=>state,Err(_)=>return Ok(None)};let document=state.keyed_documents.get(u64::from(key.document.0)).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"keyed cancellation claim lost its original document"))?;document.publication_claim.clone()};pending.lease_document_claim=Some(document_claim);pending.scope_token=Some(pending.token.as_ref().unwrap().clone());}
   pending.scope_claim=Some(pending.claim.as_ref().unwrap().clone());progress=RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};
  }else{
   let bytes=size_of::<ToolCancellationLease>()+size_of::<ToolDocumentCancellationScope>()+if keyed&&!pending.existing_document{size_of::<ToolKeyedDocumentCancellationScope>()}else{0};if grant.maximum_copy_bytes<bytes{return Ok(None)}
   let mut state=match canonical.state.try_lock(){Ok(state)=>state,Err(_)=>return Ok(None)};
   if keyed{if !state.keyed_operations.can_insert(key.operation_id.0){return Ok(None)}if pending.existing_document{let Some(document)=state.keyed_documents.get(u64::from(key.document.0))else{return Ok(None)};if document.token.is_cancelled_now(){return Ok(None)}}else if !state.keyed_documents.can_insert(u64::from(key.document.0)){return Ok(None)}}else if state.get(ToolCancellationHandle::slot(key.document)).is_some(){return Ok(None)}
   let mut original=self.pending.take().unwrap();
   if keyed{
    let id=u64::from(key.document.0);if !original.existing_document{state.keyed_documents.insert_admitted(id,ToolKeyedDocumentCancellationScope{token:original.document.take().unwrap(),publication_claim:original.document_claim.take().unwrap(),active:0});}state.keyed_documents.get_mut(id).unwrap().active+=1;
    state.keyed_operations.insert_admitted(key.operation_id.0,ToolDocumentCancellationScope{document:key.document,generation:key.generation.0,token:original.scope_token.take().unwrap(),publication_claim:original.scope_claim.take().unwrap(),operation:original.key.clone()});
   }else{state.insert_admitted(ToolCancellationHandle::slot(key.document),ToolDocumentCancellationScope{document:key.document,generation:key.generation.0,token:original.document.take().unwrap(),publication_claim:original.scope_claim.take().unwrap(),operation:original.key.clone()});}
   canonical.active.fetch_add(1,std::sync::atomic::Ordering::AcqRel);drop(state);
   let lease=ToolCancellationLease::from_original(ToolCancellationLeaseOriginal{handle:original.handle.take().unwrap(),document:key.document,document_generation:key.generation.0,key:original.key,token:original.token.take().unwrap(),publication_claim:original.claim.take().unwrap(),document_claim:original.lease_document_claim.take(),keyed,finished:false});
   cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(Some(lease))
  }
  cx.consume_retained(progress)?;cx.consume_fuel(1);Ok(None)
 }
 #[doc(hidden)]
 pub fn renew_scope_generation(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.closing||!self.ready()||self.pending.is_some()||cx.should_yield()||cx.is_cancelled(){return Ok(false)}
  let source=(cx.operation(),cx.generation(),cx.original_source_parent()?.source_identity()?);let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
  if let Some(original)=self.renew_source{if original!=source{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"cancellation renewal belongs to another original source").into())}}
  else{if self.pending_close.is_some()||grant.maximum_copy_bytes<size_of::<(semio_framework_job::OperationId,semio_framework_job::Generation,usize)>(){return Ok(false)}self.renew_source=Some(source);self.cancel_scope_generation();cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<(semio_framework_job::OperationId,semio_framework_job::Generation,usize)>(),..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
  let progress;
  if !self.renew_swapped{
   let canonical=self.issued.as_ref().unwrap();if self.pending_close.is_some()||Arc::strong_count(&canonical.state)!=1||Arc::weak_count(&canonical.state)!=0||Arc::strong_count(&canonical.app_generation)!=1||Arc::strong_count(&canonical.active)!=1{return Ok(false)}
   let state=match canonical.state.try_lock(){Ok(state)=>state,Err(_)=>return Ok(false)};if state.occupied.iter().any(|word|*word!=0)||state.keyed_operations.occupied!=0||state.keyed_documents.occupied!=0{return Ok(false)}
  }
  if self.renew_swapped{
   if let Some(parts)=self.pending_close.as_mut(){if !parts.terminal_is_empty(){progress=parts.close_step(self.issued.as_ref().unwrap(),self.admission.source.as_ref().unwrap().cancellation()?,grant)?.progress();cx.consume_retained(progress)?;if progress.copied_items>0{cx.consume_fuel(1)}return Ok(false)}self.pending_close.take();}
   if grant.maximum_copy_bytes<size_of::<(semio_framework_job::OperationId,semio_framework_job::Generation,usize)>(){return Ok(false)}self.renew_source=None;self.renew_swapped=false;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<(semio_framework_job::OperationId,semio_framework_job::Generation,usize)>(),..Default::default()})?;cx.consume_fuel(1);return Ok(true)
  }else if self.renew_cancel.is_none(){let original=self.admission.source.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"renewal lost its retained creation authority"))?;let Some((token,paid))=original.cancellation()?.admit_child(grant)?else{return Ok(false)};*self.renew_cancel=Some(token);progress=paid;}
  else if self.renew_claim.is_none(){let Some((claim,paid))=PublicationClaim::admit(grant)?else{return Ok(false)};*self.renew_claim=Some(claim);progress=paid;}
  else{let bytes=size_of::<semio_framework_job::CancelToken>()+size_of::<PublicationClaimHandle>();if grant.maximum_copy_bytes<bytes{return Ok(false)}let canonical=self.issued.as_mut().unwrap();let token=std::mem::replace(&mut canonical.app_scope,self.renew_cancel.take().unwrap());let claim=std::mem::replace(&mut canonical.publication_scope,self.renew_claim.take().unwrap());*self.pending_close=Some(crate::CancellationLeaseParts::unpublished(None,[Some(token),None,None],[Some(claim),None,None,None],None));self.renew_swapped=true;progress=RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};}
  cx.consume_retained(progress)?;cx.consume_fuel(1);Ok(false)
 }
 #[doc(hidden)]
 pub fn cancellation_cleanup_demands(&self,index:usize,body:usize,finished_only:bool)->Result<RetirementDemand,ValueError>{
  if let Some(parts)=self.pending_close.as_ref(){return parts.retirement_demands(self.issued.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"registered cancellation close lost canonical owner"))?,self.admission.source.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"registered cancellation close lost original creation source"))?.cancellation()?,body)}
  let Some(canonical)=self.issued.as_ref()else{return Ok(Default::default())};let state=canonical.state.try_lock().map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"original cancellation cleanup source is busy"))?;
  let scope=if index<TOOL_CANCELLATION_SLOTS{state.get(index)}else{state.keyed_operations.id_at(index-TOOL_CANCELLATION_SLOTS).and_then(|id|state.keyed_operations.get(id))};
  let copy_bytes=if scope.is_some_and(|scope|!finished_only||scope.publication_claim.is_finished()){size_of::<ToolDocumentCancellationScope>()+size_of::<ToolKeyedDocumentCancellationScope>()}else{size_of::<usize>()};Ok(RetirementDemand{copy_bytes,depth:1,..Default::default()})
 }
 #[doc(hidden)]
 pub fn cleanup_slot(&mut self,index:usize,cx:&mut StepContext<'_>,finished_only:bool)->Result<bool,OriginalTurnFailure>{
  if self.pending.is_some()||self.renew_swapped{return Ok(false)}if self.cleanup_index.is_some_and(|selected|selected!=index){return Ok(false)}
  if !self.ready(){return Ok(true)}if cx.should_yield(){return Ok(false)}let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
  if let Some(parts)=self.pending_close.as_mut(){
   if !parts.terminal_is_empty(){let progress=parts.close_step(self.issued.as_ref().unwrap(),self.admission.source.as_ref().unwrap().cancellation()?,grant)?.progress();cx.consume_retained(progress)?;if progress.copied_items>0{cx.consume_fuel(1)}return Ok(false)}
   if grant.maximum_copy_bytes<size_of::<usize>(){return Ok(false)}self.pending_close.take();self.cleanup_index=None;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<usize>(),..Default::default()})?;cx.consume_fuel(1);return Ok(true)
  }
  let canonical=self.issued.as_ref().unwrap();if Arc::strong_count(&canonical.state)!=1||Arc::weak_count(&canonical.state)!=0{return Ok(false)}let mut state=match canonical.state.try_lock(){Ok(state)=>state,Err(_)=>return Ok(false)};
  let scope=if index<TOOL_CANCELLATION_SLOTS{state.get(index)}else{state.keyed_operations.id_at(index-TOOL_CANCELLATION_SLOTS).and_then(|id|state.keyed_operations.get(id))};
  let Some(scope)=scope.filter(|scope|!finished_only||scope.publication_claim.is_finished())else{return Ok(true)};
  let bytes=size_of::<ToolDocumentCancellationScope>()+size_of::<ToolKeyedDocumentCancellationScope>();if grant.maximum_copy_bytes<bytes{return Ok(false)}let document_id=u64::from(scope.document.0);
  let original=if index<TOOL_CANCELLATION_SLOTS{state.take(index).unwrap()}else{let id=state.keyed_operations.id_at(index-TOOL_CANCELLATION_SLOTS).unwrap();state.keyed_operations.remove(id).unwrap()};original.token.cancel_now();original.publication_claim.cancel();
  let document=if index>=TOOL_CANCELLATION_SLOTS{let document=state.keyed_documents.get_mut(document_id).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"exact cancellation cleanup lost original document scope"))?;document.active-=1;if document.active==0{state.keyed_documents.remove(document_id)}else{None}}else{None};
  canonical.active.fetch_sub(1,std::sync::atomic::Ordering::AcqRel);drop(state);let ToolDocumentCancellationScope{token,publication_claim,..}=original;
  let(document_token,document_claim)=document.map_or((None,None),|ToolKeyedDocumentCancellationScope{token,publication_claim,..}|(Some(token),Some(publication_claim)));
  *self.pending_close=Some(crate::CancellationLeaseParts::unpublished(None,[Some(token),document_token,None],[Some(publication_claim),document_claim,None,None],(index>=TOOL_CANCELLATION_SLOTS).then_some(document_id)));self.cleanup_index=Some(index);
  cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(false)
 }
 #[doc(hidden)]
 pub fn scope_generation(&self)->u64{self.issued.as_ref().map_or(0,ToolCancellationHandle::scope_generation)}
 #[doc(hidden)]
 pub fn active_operation_count(&self)->usize{self.issued.as_ref().map_or(0,ToolCancellationHandle::active_operation_count)}
 #[doc(hidden)]
 pub fn cancel_scope_generation(&self){if let Some(issued)=self.issued.as_ref(){issued.cancel_scope_generation()}}
 #[doc(hidden)]
 pub fn terminal_is_empty(&self)->bool{let source=&*self.admission;self.pending.is_none()&&self.pending_close.is_none()&&self.renew_cancel.is_none()&&self.renew_claim.is_none()&&self.issued.is_none()&&source.source.is_none()&&source.slots.is_none()&&source.operations.is_none()&&source.documents.is_none()&&source.cancel.is_none()&&source.claim.is_none()&&source.generation.is_none()&&source.active.is_none()&&source.state.is_none()&&source.cancel_close.is_none()&&source.claim_close.is_none()}
 #[doc(hidden)]
 pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  let one=RetirementDemand{depth:1,..Default::default()};let source=&*self.admission;
  if !self.closing{return Ok(RetirementDemand{copy_bytes:size_of::<bool>(),..one})}
  if self.pending.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<PendingLease>(),..one})}
  if let Some(parts)=self.pending_close.as_ref(){return parts.retirement_demands(self.issued.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"pending lease close lost canonical issuer"))?,self.admission.source.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"pending cancellation close lost original creation source"))?.cancellation()?,body)}
  if self.renew_cancel.is_some()||self.renew_claim.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<semio_framework_job::CancelToken>()+size_of::<PublicationClaimHandle>(),..one})}
  if self.issued.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<ToolCancellationHandle>(),..one})}
  if let Some(state)=source.state.as_ref(){return Ok(RetirementDemand{copy_bytes:size_of::<ToolCancellationState>(),release_bytes:if Arc::strong_count(state)==1&&Arc::weak_count(state)==0{semio_framework_value::shared_retirement_allocation_bytes::<RetainedOwnerGate<ToolCancellationState>>()}else{0},..one})}
  if let Some(slots)=source.slots.as_ref(){return Ok(RetirementDemand{release_bytes:slots.len()*size_of::<ToolDocumentCancellationScope>(),..one})}
  if let Some(registry)=source.operations.as_ref(){return Ok(RetirementDemand{release_bytes:registry.slots.len()*size_of::<(u64,ToolDocumentCancellationScope)>(),..one})}
  if let Some(registry)=source.documents.as_ref(){return Ok(RetirementDemand{release_bytes:registry.slots.len()*size_of::<(u64,ToolKeyedDocumentCancellationScope)>(),..one})}
  if source.cancel.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<semio_framework_job::CancelToken>(),..one})}
  if let Some(cancel)=source.cancel_close.as_ref(){return if cancel.terminal_is_empty()||cancel.is_original_alias_witness(source.source.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original cancellation quote lost its creation source loan"))?.cancellation()?){Ok(one)}else{cancel.retirement_demands()}}
  if source.claim.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<PublicationClaimHandle>(),..one})}
  if let Some(claim)=source.claim_close.as_ref(){return if claim.terminal_is_empty(){Ok(one)}else{claim.selected_demand(body)}}
  if let Some(generation)=source.generation.as_ref(){return Ok(RetirementDemand{release_bytes:if Arc::strong_count(generation)==1&&Arc::weak_count(generation)==0{semio_framework_value::shared_retirement_allocation_bytes::<AtomicU64>()}else{0},..one})}
  if let Some(active)=source.active.as_ref(){return Ok(RetirementDemand{release_bytes:if Arc::strong_count(active)==1&&Arc::weak_count(active)==0{semio_framework_value::shared_retirement_allocation_bytes::<AtomicUsize>()}else{0},..one})}
  Ok(one)
 }
 #[doc(hidden)]
 pub fn close_step(&mut self,parent:&OriginalCallerSourceParent,cx:&mut StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,OriginalTurnFailure>{
  use semio_framework_job::InteractiveJobCloseStep;
  if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
  if let Some(source)=self.admission.source.as_ref(){if !source.terminal_is_empty(){source.validate_retirement_context(cx)?}}
  if cx.should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
  let demand=self.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;let grant=cx.retained_grant();
  if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
  let mut progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()};let source=&mut*self.admission;
  if !self.closing{self.closing=true;if let Some(handle)=self.issued.as_ref(){handle.cancel_scope_generation()}}
  else if let Some(pending)=self.pending.take(){*self.pending_close=Some(pending.into_parts());}
  else if let Some(parts)=self.pending_close.as_mut(){if parts.terminal_is_empty(){self.pending_close.take();}else{progress=parts.close_step(self.issued.as_ref().unwrap(),source.source.as_ref().unwrap().cancellation()?,grant)?.progress()}}
  else if self.renew_cancel.is_some()||self.renew_claim.is_some(){*self.pending_close=Some(crate::CancellationLeaseParts::unpublished(None,[self.renew_cancel.take(),None,None],[self.renew_claim.take(),None,None,None],None));}
  else if let Some(handle)=self.issued.take(){let ToolCancellationHandle{state,app_scope,publication_scope,app_generation,active}=handle;source.state=Some(state);source.cancel=Some(app_scope);source.claim=Some(publication_scope);source.generation=Some(app_generation);source.active=Some(active)}
  else if let Some(state)=source.state.as_ref(){
   if Arc::strong_count(state)!=1||Arc::weak_count(state)!=0{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
   let Ok(borrowed)=state.try_lock()else{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})};if borrowed.occupied.iter().any(|word|*word!=0)||borrowed.keyed_operations.occupied!=0||borrowed.keyed_documents.occupied!=0{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}drop(borrowed);
   match Arc::try_unwrap(source.state.take().unwrap()){Ok(state)=>{let ToolCancellationState{slots,keyed_operations,keyed_documents,..}=state.into_inner();source.slots=Some(slots);source.operations=Some(keyed_operations);source.documents=Some(keyed_documents)},Err(state)=>{source.state=Some(state);return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}}
  }else if source.slots.is_some(){source.slots.take();}
  else if let Some(registry)=source.operations.as_ref(){if registry.occupied!=0{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}source.operations.take();}
  else if let Some(registry)=source.documents.as_ref(){if registry.occupied!=0{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}source.documents.take();}
  else if let Some(cancel)=source.cancel.take(){source.cancel_close=Some(semio_framework_async::CancelTokenRetirement::from_token(cancel));}
  else if let Some(cancel)=source.cancel_close.as_mut(){
   if cancel.terminal_is_empty(){source.cancel_close.take();}else{let step=if cancel.is_original_alias_witness(parent.cancellation()?){cancel.return_alias_step(parent.cancellation()?,grant)}else{cancel.close_step(grant)};progress=match step{Ok(step)=>step.progress(),Err(semio_framework_async::CancelTokenRetirementError::Blocked(_))=>return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()}),Err(semio_framework_async::CancelTokenRetirementError::Refused(failure))=>return Err(failure.into())};}
  }else if let Some(claim)=source.claim.take(){match ControlledRetirement::new(claim){Ok(owner)=>{source.claim_close=Some(owner)},Err((failure,claim))=>{source.claim=Some(claim);return Err(failure.into())}}}
  else if let Some(claim)=source.claim_close.as_mut(){if claim.terminal_is_empty(){source.claim_close.take();}else{progress=claim.step(grant)?.progress()}}
  else if let Some(generation)=source.generation.as_ref(){if Arc::strong_count(generation)!=1||Arc::weak_count(generation)!=0{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}source.generation.take();}
  else if let Some(active)=source.active.as_ref(){if Arc::strong_count(active)!=1||Arc::weak_count(active)!=0{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}source.active.take();}
  else if let Some(original)=source.source.as_mut(){if original.terminal_is_empty(){source.source.take();}else{return original.close_step(parent,cx).map(|step|match step{InteractiveJobCloseStep::Complete{progress}=>InteractiveJobCloseStep::Pending{progress},step=>step})}}
  cx.consume_retained(progress)?;if progress.copied_items>0{cx.consume_fuel(1);}Ok(InteractiveJobCloseStep::Pending{progress})
 }
}
impl std::ops::Deref for ToolCancellationIssuer{type Target=ToolCancellationHandle;fn deref(&self)->&Self::Target{self.issued.as_ref().expect("canonical cancellation requires its admitted creation issuer")}}
impl std::ops::DerefMut for ToolCancellationIssuer{fn deref_mut(&mut self)->&mut Self::Target{self.issued.as_mut().expect("canonical cancellation requires its admitted creation issuer")}}
impl Drop for ToolCancellationIssuer{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"canonical cancellation issuer requires paid original terminal retirement")}}
#[cfg(any(test,feature="media-turn-testing"))]
#[path="🧪️tests/🦀️.rs"]
mod original_laws;
#[cfg(any(test,feature="media-turn-testing"))]
#[doc(hidden)]
pub use original_laws::cancellation_issuer_original_creation_physical_birth_and_close;
#[cfg(test)]
#[path="🧪️tests/📜️contract/🦀️.rs"]
mod contract_laws;
