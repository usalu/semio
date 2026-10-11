use semio_framework_value::retirement::controlled::ControlledRetirement;
/// 📦️ Detaches exact cancellation scopes without dropping any original graph owner.
struct CancellationHandleAliases { state:std::sync::Arc<std::sync::Mutex<ToolCancellationState>>, generation:std::sync::Arc<std::sync::atomic::AtomicU64>, active:std::sync::Arc<std::sync::atomic::AtomicUsize> }
struct CancellationLeaseParts { aliases:Option<CancellationHandleAliases>, tokens:[Option<semio_framework_job::CancelToken>;4], claims:[Option<PublicationClaimHandle>;5], token_retirements:[Option<ControlledRetirement<semio_framework_job::CancelToken>>;4], claim_retirements:[Option<ControlledRetirement<PublicationClaimHandle>>;5] }
struct ToolCancellationLeaseRetirement { original:Option<std::mem::ManuallyDrop<ToolCancellationLease>>, operation:Option<std::mem::ManuallyDrop<ToolDocumentCancellationScope>>, document:Option<std::mem::ManuallyDrop<ToolKeyedDocumentCancellationScope>>, parts:std::mem::ManuallyDrop<Option<CancellationLeaseParts>>, detached:bool }
impl ToolCancellationLeaseRetirement {
 fn new(original:ToolCancellationLease)->Self{Self{original:Some(std::mem::ManuallyDrop::new(original)),operation:None,document:None,parts:std::mem::ManuallyDrop::new(None),detached:false}}
 fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.operation.is_none()&&self.document.is_none()&&self.parts.is_none()}
 fn retirement_demands(&self,canonical:&ToolCancellationHandle,body:usize)->Result<RetirementDemand,ValueError>{
  let item=RetirementDemand{depth:1,..Default::default()};
  if !self.detached{return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<ToolDocumentCancellationScope>()+std::mem::size_of::<ToolKeyedDocumentCancellationScope>(),..item});}
  if self.original.is_some(){return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<ToolCancellationLease>()+std::mem::size_of::<ToolDocumentCancellationScope>()+std::mem::size_of::<ToolKeyedDocumentCancellationScope>(),..item});}
  let Some(parts)=self.parts.as_ref()else{return Ok(Default::default());};
  for index in 0..4{if parts.tokens[index].is_some()||parts.token_retirements[index].is_some(){return plugin_typed_owner_demand(&parts.tokens[index],&parts.token_retirements[index],body);}}
  for index in 0..5{if parts.claims[index].is_some()||parts.claim_retirements[index].is_some(){return plugin_typed_owner_demand(&parts.claims[index],&parts.claim_retirements[index],body);}}
  if let Some(aliases)=parts.aliases.as_ref(){if !std::sync::Arc::ptr_eq(&aliases.state,&canonical.state)||!std::sync::Arc::ptr_eq(&aliases.generation,&canonical.app_generation)||!std::sync::Arc::ptr_eq(&aliases.active,&canonical.active){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original cancellation handle aliases lost their issuing canonical owner"));}return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<CancellationHandleAliases>(),..item});}
  Ok(item)
 }
 fn close_step(&mut self,canonical:&ToolCancellationHandle,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let demand=self.retirement_demands(canonical,grant.maximum_copy_bytes)?;
  if !plugin_grant_funds(demand,grant)||grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
  if !self.detached{
   let original=self.original.as_mut().unwrap();let mut state=match original.handle.state.try_lock(){Ok(state)=>state,Err(std::sync::TryLockError::WouldBlock)=>return Ok(RetainedCloneStep::Progress(Default::default())),Err(std::sync::TryLockError::Poisoned(_))=>return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original cancellation registry is poisoned"))};
   original.publication_claim.finish();let mut copied=0;
   if original.keyed{
    if state.keyed_operations.get(original.key.operation_id.0).is_some_and(|scope|scope.operation==original.key&&PublicationClaimHandle::same_original(&scope.publication_claim,&original.publication_claim)){
     let detached=state.keyed_operations.remove(original.key.operation_id.0).unwrap();self.operation=Some(std::mem::ManuallyDrop::new(detached));copied+=std::mem::size_of::<ToolDocumentCancellationScope>();
     let id=u64::from(original.document.0);let document=state.keyed_documents.get_mut(id).expect("exact operation retains its original document scope");document.active-=1;
     if document.active==0{self.document=state.keyed_documents.remove(id).map(std::mem::ManuallyDrop::new);copied+=std::mem::size_of::<ToolKeyedDocumentCancellationScope>();}
     original.handle.active.fetch_sub(1,std::sync::atomic::Ordering::AcqRel);
    }
   }else{
    let index=ToolCancellationHandle::slot(original.document);if state.get(index).is_some_and(|scope|scope.document==original.document&&scope.generation==original.document_generation&&scope.operation==original.key){self.operation=state.take(index).map(std::mem::ManuallyDrop::new);copied+=std::mem::size_of::<ToolDocumentCancellationScope>();original.handle.active.fetch_sub(1,std::sync::atomic::Ordering::AcqRel);}
   }
   drop(state);original.finished=true;self.detached=true;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:copied,..Default::default()}));
  }
  if let Some(original)=self.original.take(){
   let handle=unsafe{std::ptr::read(&original.handle)};let token=unsafe{std::ptr::read(&original.token)};let claim=unsafe{std::ptr::read(&original.publication_claim)};let document_claim=unsafe{std::ptr::read(&original.document_claim)};
   let ToolCancellationHandle{state,app_scope,publication_scope,app_generation,active}=handle;
   let mut parts=CancellationLeaseParts{aliases:Some(CancellationHandleAliases{state,generation:app_generation,active}),tokens:[Some(token),None,None,Some(app_scope)],claims:[Some(claim),document_claim,None,None,Some(publication_scope)],token_retirements:std::array::from_fn(|_|None),claim_retirements:std::array::from_fn(|_|None)};
   if let Some(operation)=self.operation.take(){let ToolDocumentCancellationScope{token,publication_claim,..}=std::mem::ManuallyDrop::into_inner(operation);parts.tokens[1]=Some(token);parts.claims[2]=Some(publication_claim);}
   if let Some(document)=self.document.take(){let ToolKeyedDocumentCancellationScope{token,publication_claim,..}=std::mem::ManuallyDrop::into_inner(document);parts.tokens[2]=Some(token);parts.claims[3]=Some(publication_claim);}
   *self.parts=Some(parts);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));
  }
  let Some(parts)=self.parts.as_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()));};
  for index in 0..4{if parts.tokens[index].is_some()||parts.token_retirements[index].is_some(){return plugin_typed_owner_step(&mut parts.tokens[index],&mut parts.token_retirements[index],grant);}}
  for index in 0..5{if parts.claims[index].is_some()||parts.claim_retirements[index].is_some(){return plugin_typed_owner_step(&mut parts.claims[index],&mut parts.claim_retirements[index],grant);}}
  if let Some(original)=parts.aliases.take(){drop(original);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
  self.parts.take();Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))
 }
}
impl Drop for ToolCancellationLeaseRetirement{fn drop(&mut self){assert!(self.terminal_is_empty(),"original cancellation lease retirement retains all refused owners");}}
fn original_lease_cursor_admit(original:&mut Option<ToolCancellationLease>,grant:RetainedCloneGrant)->Result<(Option<Box<ToolCancellationLeaseRetirement>>,RetainedCloneProgress),ValueError>{
 let empty=RetainedCloneProgress::default();if original.is_none(){return Ok((None,empty));}let layout=std::alloc::Layout::new::<ToolCancellationLeaseRetirement>();let demand=RetirementDemand{copy_bytes:std::mem::size_of::<ToolCancellationLease>(),capacity_bytes:layout.size(),depth:1,..Default::default()};if grant.maximum_items==0||!plugin_grant_funds(demand,grant){return Ok((None,empty));}
 let Some(pointer)=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<ToolCancellationLeaseRetirement>())else{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"original cancellation lease cursor allocation was refused"));};let value=original.take().unwrap();let owner=unsafe{pointer.as_ptr().write(ToolCancellationLeaseRetirement::new(value));Box::from_raw(pointer.as_ptr())};Ok((Some(owner),RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:layout.size(),..empty}))
}
fn original_lease_cursor_release(original:&mut Option<Box<ToolCancellationLeaseRetirement>>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
 let Some(owner)=original.as_ref()else{return Ok(Default::default());};if !owner.terminal_is_empty(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original cancellation cursor frame still retains its children"));}let bytes=std::mem::size_of::<ToolCancellationLeaseRetirement>();if grant.maximum_items==0||grant.maximum_release_bytes<bytes||grant.maximum_depth==0{return Ok(Default::default());}original.take();Ok(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()})
}
#[cfg(test)]
mod cancellation_lease_tests{include!("🧪️tests/🦀️.rs");}
impl<A:ArtifactApp> MountedTypedCommandFullOperation<A>{
 fn lease_retirement_ready(&self)->bool{self.stage==MountedTypedCommandFullOperationStage::Retiring&&self.session.is_none()&&self.session_rejected.is_none()&&!self.worker_semantic_pending&&!self.worker_outcome_pending&&self.worker_fault_capture.is_none()&&self.publication.is_none()&&self.publication_retirement.is_none()&&self.pending_artifact_publication.is_none()&&!self.pending_publication_outcome.pending()&&self.pending_child_publication.is_none()&&self.owned_child_group.is_none()&&self.captured_child_content.is_none()&&self.result_page.is_none()&&self.pending_window_config_receipt.is_none()&&self.raw_input.is_none()&&self.output_chunks.is_none()&&self.output_retirement.is_none()&&self.reserved_producer.is_none()&&self.completion.is_none()&&self.completion_retirement.is_none()&&self.meta.view_state.is_none()&&self.window_config_authority.is_none()&&self.window_transient_authority.is_none()&&self.actor_capture.is_none()&&self.verb.capacity()==0&&!self.meta.actor.has_owner()&&self.retained_close_fault.is_none()&&(self.cancellation_lease.is_some()||self.cancellation_retirement.is_some())}
 fn lease_retirement_demands(&self,canonical:&ToolCancellationHandle,body:usize)->Result<RetirementDemand,ValueError>{if let Some(owner)=self.cancellation_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{release_bytes:std::mem::size_of::<ToolCancellationLeaseRetirement>(),depth:1,..Default::default()})}else{owner.retirement_demands(canonical,body)};}Ok(RetirementDemand{copy_bytes:std::mem::size_of::<ToolCancellationLease>(),capacity_bytes:std::mem::size_of::<ToolCancellationLeaseRetirement>(),depth:1,..Default::default()})}
 fn lease_retirement_step(&mut self,canonical:&ToolCancellationHandle,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let demand=self.lease_retirement_demands(canonical,grant.maximum_copy_bytes)?;if !plugin_grant_funds(demand,grant)||grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(owner)=self.cancellation_retirement.as_mut(){if owner.terminal_is_empty(){return original_lease_cursor_release(&mut self.cancellation_retirement,grant).map(RetainedCloneStep::Progress);}return owner.close_step(canonical,grant);}
  let(owner,progress)=original_lease_cursor_admit(&mut self.cancellation_lease,grant)?;self.cancellation_retirement=owner;Ok(RetainedCloneStep::Progress(progress))
 }
}
