use semio_framework_value::retirement::controlled::ControlledRetirement;
#[cfg(test)] thread_local!{static WITNESS_LOOKUPS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};}
fn witness_lookup_observed(){#[cfg(test)] WITNESS_LOOKUPS.with(|count|count.set(count.get()+1));}
#[derive(Clone,Copy,PartialEq,Eq)] enum TokenWitness { Select, Local, Keyed, Close, FindKeyed }
/// 📦️ Detaches exact cancellation scopes without dropping any original graph owner.
struct CancellationHandleAliases { state:std::sync::Arc<semio_framework_value::retirement::controlled::RetainedOwnerGate<ToolCancellationState>>, generation:std::sync::Arc<std::sync::atomic::AtomicU64>, active:std::sync::Arc<std::sync::atomic::AtomicUsize> }
pub(crate) struct CancellationLeaseParts { aliases:Option<CancellationHandleAliases>, tokens:[Option<semio_framework_job::CancelToken>;4], claims:[Option<PublicationClaimHandle>;5], token_retirements:[Option<semio_framework_async::CancelTokenRetirement>;4], document_id:Option<u64>, claim_retirements:[Option<ControlledRetirement<PublicationClaimHandle>>;5], token_witness:[TokenWitness;4] }
impl CancellationLeaseParts {
 pub(crate) fn unpublished(handle:Option<ToolCancellationHandle>,tokens:[Option<semio_framework_job::CancelToken>;3],claims:[Option<PublicationClaimHandle>;4],document_id:Option<u64>)->Self{
  let [a,b,c]=tokens;let [x,y,z,w]=claims;let mut tokens=[a,b,c,None];let mut claims=[x,y,z,w,None];
  let aliases=handle.map(|ToolCancellationHandle{state,app_scope,publication_scope,app_generation,active}|{tokens[3]=Some(app_scope);claims[4]=Some(publication_scope);CancellationHandleAliases{state,generation:app_generation,active}});
  Self{aliases,tokens,claims,token_retirements:std::array::from_fn(|_|None),document_id,claim_retirements:std::array::from_fn(|_|None),token_witness:[TokenWitness::Select;4]}
 }
 #[doc(hidden)]
 pub fn terminal_is_empty(&self)->bool{self.aliases.is_none()&&self.tokens.iter().all(Option::is_none)&&self.claims.iter().all(Option::is_none)&&self.token_retirements.iter().all(Option::is_none)&&self.claim_retirements.iter().all(Option::is_none)}
 #[doc(hidden)]
 pub fn retirement_demands(&self,canonical:&ToolCancellationHandle,original_parent:&semio_framework_job::CancelToken,body:usize)->Result<RetirementDemand,ValueError>{
  let item=RetirementDemand{depth:1,..Default::default()};
  for index in 0..4{if self.tokens[index].is_some()||self.token_retirements[index].is_some(){return self.token_demands(index,canonical,Some(original_parent))}}
  for index in 0..5{if self.claims[index].is_some()||self.claim_retirements[index].is_some(){return plugin_typed_owner_demand(&self.claims[index],&self.claim_retirements[index],body)}}
  if let Some(aliases)=self.aliases.as_ref(){if !std::sync::Arc::ptr_eq(&aliases.state,&canonical.state)||!std::sync::Arc::ptr_eq(&aliases.generation,&canonical.app_generation)||!std::sync::Arc::ptr_eq(&aliases.active,&canonical.active){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original cancellation handle aliases lost their issuing canonical owner"))}return Ok(RetirementDemand{copy_bytes:size_of::<CancellationHandleAliases>(),..item})}Ok(item)
 }
 #[doc(hidden)]
 pub fn retirement_work_demand(&self,body:usize)->Result<u64,ValueError>{
  for index in 0..4{if self.tokens[index].is_some()||self.token_retirements[index].is_some(){return Ok(self.token_work_demand(index))}}
  for index in 0..5{if self.claims[index].is_some()||self.claim_retirements[index].is_some(){return semio_plugin_ownership::bounded_ownership::plugin_typed_owner_work_demand(&self.claims[index],&self.claim_retirements[index],body)}}
  Ok(u64::from(self.aliases.is_some()))
 }
 #[doc(hidden)]
 pub fn close_step(&mut self,canonical:&ToolCancellationHandle,original_parent:&semio_framework_job::CancelToken,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{
  use semio_framework_value::RetirementPollExit as Exit;let grant=effect.grant();let result=(||->Result<RetainedCloneStep,Exit>{
   let demand=self.retirement_demands(canonical,original_parent,grant.maximum_copy_bytes)?;if !plugin_grant_funds(demand,grant)||grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
   for index in 0..4{if self.tokens[index].is_some()||self.token_retirements[index].is_some(){return self.token_step(index,canonical,Some(original_parent),effect)}}
   for index in 0..5{if self.claims[index].is_some()||self.claim_retirements[index].is_some(){return Exit::resume(plugin_typed_owner_original(&mut self.claims[index],&mut self.claim_retirements[index],effect))}}
   if self.aliases.is_some(){self.aliases.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}Ok(RetainedCloneStep::Complete(Default::default()))
  })();Exit::settle(effect,result)
 }
 fn token_demands(&self,index:usize,_canonical:&ToolCancellationHandle,_original_parent:Option<&semio_framework_job::CancelToken>)->Result<RetirementDemand,ValueError>{
   let one=RetirementDemand{depth:1,..Default::default()};
   let Some(owner)=self.token_retirements[index].as_ref()else{return Ok(one)};
   if owner.terminal_is_empty()||self.token_witness[index]!=TokenWitness::Close{return Ok(one)}
   owner.retirement_demands()
  }
  fn token_work_demand(&self,index:usize)->u64{
   if self.tokens[index].is_some()||self.token_retirements[index].as_ref().is_some_and(|owner|owner.terminal_is_empty()){1}else{match self.token_witness[index]{TokenWitness::Local|TokenWitness::Keyed|TokenWitness::Close=>2,_=>1}}
  }
  fn token_step(&mut self,index:usize,canonical:&ToolCancellationHandle,original_parent:Option<&semio_framework_job::CancelToken>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,semio_framework_value::RetirementPollExit>{
   let empty=RetainedCloneProgress::default();
   {
    if let Some(token)=self.tokens[index].take(){self.token_retirements[index]=Some(semio_framework_async::CancelTokenRetirement::from_token(token));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}
    let owner=self.token_retirements[index].as_mut().expect("original cancellation token remains retained");
    if owner.terminal_is_empty(){self.token_retirements[index]=None;self.token_witness[index]=TokenWitness::Select;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}
    if self.token_witness[index]==TokenWitness::Select{
     witness_lookup_observed();
     if self.tokens.iter().flatten().chain(std::iter::once(&canonical.app_scope)).chain(original_parent).any(|token|owner.is_original_alias_witness(token)){self.token_witness[index]=TokenWitness::Local}
     else{self.token_witness[index]=if self.document_id.is_some(){TokenWitness::FindKeyed}else{TokenWitness::Close}}
     return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))
    }
    if self.token_witness[index]==TokenWitness::FindKeyed{witness_lookup_observed();let state=match canonical.state.try_lock(){Ok(state)=>state,Err(_)=>return Ok(RetainedCloneStep::Progress(empty))};let id=self.document_id.expect("keyed selection retains its original id");self.token_witness[index]=if state.keyed_documents.get(id).is_some_and(|scope|owner.is_original_alias_witness(&scope.token)){TokenWitness::Keyed}else{TokenWitness::Close};return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}
    let turn=match self.token_witness[index]{
     TokenWitness::Local=>{let witness=self.tokens.iter().flatten().chain(std::iter::once(&canonical.app_scope)).chain(original_parent).find(|token|owner.is_original_alias_witness(token));match witness{Some(witness)=>owner.return_alias_step(witness,effect),None=>{self.token_witness[index]=TokenWitness::Select;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}}}
     TokenWitness::Keyed=>{witness_lookup_observed();let state=match canonical.state.try_lock(){Ok(state)=>state,Err(_)=>return Ok(RetainedCloneStep::Progress(empty))};let witness=state.keyed_documents.get(self.document_id.expect("keyed witness retains its original id")).filter(|scope|owner.is_original_alias_witness(&scope.token));match witness{Some(scope)=>owner.return_alias_step(&scope.token,effect),None=>{self.token_witness[index]=TokenWitness::Select;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}}}
     _=>owner.close_step(effect)
    };
    match turn.result{Ok(step)=>Ok(RetainedCloneStep::Progress(step.progress())),Err(semio_framework_async::CancelTokenRetirementError::Blocked(_))=>{self.token_witness[index]=TokenWitness::Select;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))},Err(semio_framework_async::CancelTokenRetirementError::Refused(error))=>Err(error.into())}
   }
  }

}
#[doc(hidden)]
pub struct ToolCancellationLeaseRetirement { original:Option<ManuallyDrop<ToolCancellationLease>>, operation:Option<ManuallyDrop<ToolDocumentCancellationScope>>, document:Option<ManuallyDrop<ToolKeyedDocumentCancellationScope>>, parts:ManuallyDrop<Option<CancellationLeaseParts>>, detached:bool }
impl ToolCancellationLeaseRetirement {
 #[doc(hidden)]
 pub fn new(original:ToolCancellationLease)->Self{Self{original:Some(ManuallyDrop::new(original)),operation:None,document:None,parts:ManuallyDrop::new(None),detached:false}}
 #[doc(hidden)]
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.operation.is_none()&&self.document.is_none()&&self.parts.is_none()}
 #[doc(hidden)]
 pub fn retirement_demands(&self,canonical:&ToolCancellationHandle,body:usize)->Result<RetirementDemand,ValueError>{
  let item=RetirementDemand{depth:1,..Default::default()};
  if !self.detached{return Ok(RetirementDemand{copy_bytes:size_of::<ToolDocumentCancellationScope>()+size_of::<ToolKeyedDocumentCancellationScope>(),..item});}
  if self.original.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<ToolCancellationLease>()+size_of::<ToolDocumentCancellationScope>()+size_of::<ToolKeyedDocumentCancellationScope>(),..item});}
  let Some(parts)=self.parts.as_ref()else{return Ok(Default::default());};
  for index in 0..4{if parts.tokens[index].is_some()||parts.token_retirements[index].is_some(){return parts.token_demands(index,canonical,None);}}
  for index in 0..5{if parts.claims[index].is_some()||parts.claim_retirements[index].is_some(){return plugin_typed_owner_demand(&parts.claims[index],&parts.claim_retirements[index],body);}}
  if let Some(aliases)=parts.aliases.as_ref(){if !std::sync::Arc::ptr_eq(&aliases.state,&canonical.state)||!std::sync::Arc::ptr_eq(&aliases.generation,&canonical.app_generation)||!std::sync::Arc::ptr_eq(&aliases.active,&canonical.active){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original cancellation handle aliases lost their issuing canonical owner"));}return Ok(RetirementDemand{copy_bytes:size_of::<CancellationHandleAliases>(),..item});}
  Ok(item)
 }
 #[doc(hidden)]
 pub fn retirement_work_demand(&self,body:usize)->Result<u64,ValueError>{
  if !self.detached||self.original.is_some(){return Ok(1)}
  let Some(parts)=self.parts.as_ref()else{return Ok(0)};
  for index in 0..4{if parts.tokens[index].is_some()||parts.token_retirements[index].is_some(){return Ok(parts.token_work_demand(index))}}
  for index in 0..5{if parts.claims[index].is_some()||parts.claim_retirements[index].is_some(){return semio_plugin_ownership::bounded_ownership::plugin_typed_owner_work_demand(&parts.claims[index],&parts.claim_retirements[index],body)}}
  Ok(1)
 }
 #[doc(hidden)]
 pub fn close_step(&mut self,canonical:&ToolCancellationHandle,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{
  use semio_framework_value::RetirementPollExit as Exit;let grant=effect.grant();let result=(||->Result<RetainedCloneStep,Exit>{
  let demand=self.retirement_demands(canonical,grant.maximum_copy_bytes)?;
  if !plugin_grant_funds(demand,grant)||grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
  if !self.detached{
   let original=self.original.as_mut().unwrap();let mut state=match original.handle.state.try_lock(){Ok(state)=>state,Err(_)=>return Ok(RetainedCloneStep::Progress(Default::default()))};
   original.publication_claim.finish();let mut copied=0;
   if original.keyed{
    if state.keyed_operations.get(original.key.operation_id.0).is_some_and(|scope|scope.operation==original.key&&PublicationClaimHandle::same_original(&scope.publication_claim,&original.publication_claim)){
     let detached=state.keyed_operations.remove(original.key.operation_id.0).unwrap();self.operation=Some(ManuallyDrop::new(detached));copied+=size_of::<ToolDocumentCancellationScope>();
     let id=u64::from(original.document.0);let document=state.keyed_documents.get_mut(id).expect("exact operation retains its original document scope");document.active-=1;
     if document.active==0{self.document=state.keyed_documents.remove(id).map(ManuallyDrop::new);copied+=size_of::<ToolKeyedDocumentCancellationScope>();}
     original.handle.active.fetch_sub(1,std::sync::atomic::Ordering::AcqRel);
    }
   }else{
    let index=ToolCancellationHandle::slot(original.document);if state.get(index).is_some_and(|scope|scope.document==original.document&&scope.generation==original.document_generation&&scope.operation==original.key){self.operation=state.take(index).map(ManuallyDrop::new);copied+=size_of::<ToolDocumentCancellationScope>();original.handle.active.fetch_sub(1,std::sync::atomic::Ordering::AcqRel);}
   }
   drop(state);original.finished=true;self.detached=true;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:copied,..Default::default()}));
  }
  if let Some(original)=self.original.take(){
   let crate::ToolCancellationLeaseOriginal{handle,token,publication_claim:claim,document_claim,keyed,document,..}=ManuallyDrop::into_inner(original).into_retirement_source();
   let ToolCancellationHandle{state,app_scope,publication_scope,app_generation,active}=handle;
   let mut parts=CancellationLeaseParts{aliases:Some(CancellationHandleAliases{state,generation:app_generation,active}),tokens:[Some(token),None,None,Some(app_scope)],claims:[Some(claim),document_claim,None,None,Some(publication_scope)],token_retirements:std::array::from_fn(|_|None),document_id:keyed.then(||u64::from(document.0)),claim_retirements:std::array::from_fn(|_|None),token_witness:[TokenWitness::Select;4]};
   if let Some(operation)=self.operation.take(){let ToolDocumentCancellationScope{token,publication_claim,..}=ManuallyDrop::into_inner(operation);parts.tokens[1]=Some(token);parts.claims[2]=Some(publication_claim);}
   if let Some(document)=self.document.take(){let ToolKeyedDocumentCancellationScope{token,publication_claim,..}=ManuallyDrop::into_inner(document);parts.tokens[2]=Some(token);parts.claims[3]=Some(publication_claim);}
   *self.parts=Some(parts);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));
  }
  let Some(parts)=self.parts.as_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()));};
  for index in 0..4{if parts.tokens[index].is_some()||parts.token_retirements[index].is_some(){return parts.token_step(index,canonical,None,effect);}}
  for index in 0..5{if parts.claims[index].is_some()||parts.claim_retirements[index].is_some(){return Exit::resume(plugin_typed_owner_original(&mut parts.claims[index],&mut parts.claim_retirements[index],effect));}}
  if let Some(original)=parts.aliases.take(){drop(original);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
  self.parts.take();Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))
  })();Exit::settle(effect,result)
 }
}
impl Drop for ToolCancellationLeaseRetirement{fn drop(&mut self){assert!(self.terminal_is_empty(),"original cancellation lease retirement retains all refused owners");}}
#[doc(hidden)]
pub fn original_lease_cursor_admit(original:&mut Option<ToolCancellationLease>,grant:RetainedCloneGrant)->Result<(Option<Box<ToolCancellationLeaseRetirement>>,RetainedCloneProgress),ValueError>{
 let empty=RetainedCloneProgress::default();if original.is_none(){return Ok((None,empty));}let layout=std::alloc::Layout::new::<ToolCancellationLeaseRetirement>();let demand=RetirementDemand{copy_bytes:size_of::<ToolCancellationLease>(),capacity_bytes:layout.size(),depth:1,..Default::default()};if grant.maximum_items==0||!plugin_grant_funds(demand,grant){return Ok((None,empty));}
 let Some(pointer)=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<ToolCancellationLeaseRetirement>())else{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"original cancellation lease cursor allocation was refused"));};let value=original.take().unwrap();let owner=unsafe{pointer.as_ptr().write(ToolCancellationLeaseRetirement::new(value));Box::from_raw(pointer.as_ptr())};Ok((Some(owner),RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:layout.size(),..empty}))
}
#[doc(hidden)]
pub fn original_lease_cursor_release(original:&mut Option<Box<ToolCancellationLeaseRetirement>>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
 let Some(owner)=original.as_ref()else{return Ok(Default::default());};if !owner.terminal_is_empty(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original cancellation cursor frame still retains its children"));}let bytes=size_of::<ToolCancellationLeaseRetirement>();if grant.maximum_items==0||grant.maximum_release_bytes<bytes||grant.maximum_depth==0{return Ok(Default::default());}original.take();Ok(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()})
}
#[cfg(test)]
mod cancellation_lease_tests{include!("🧪️tests/🦀️.rs");}
