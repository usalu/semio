//! 🧳️ Prepaid original failure storage preserves nested refusal custody.
use super::*;
fn fixture()->serde_json::Value{let value:serde_json::Value=serde_json::from_str(include_str!("failure-storage.json")).unwrap();let schema:serde_json::Value=serde_json::from_str(include_str!("failure-storage.schema.json")).unwrap();for(key,expected)in value.as_object().unwrap(){assert_eq!(&schema["properties"][key]["const"],expected)}assert_eq!(schema["required"].as_array().unwrap().len(),value.as_object().unwrap().len());value}
fn close_failure(failure:OriginalRetirementFailure){
 let mut owner=crate::retirement::controlled::ControlledRetirement::retain(failure);
 for _ in 0..4096{if owner.terminal_is_empty(){return}let d=owner.next_demand(4096).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let(step,heap)=crate::observe_retirement_allocations(||owner.step(grant).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}panic!("original failure did not retire")
}
#[test]
fn original_failure_storage_admits_before_child_and_retains_all_refusals(){
 let f=fixture();
 let mut calls=0;let denied=OriginalFailureStorage::admit(f["deniedCapacity"].as_u64().unwrap() as usize);
 assert_eq!(denied.work,0);if denied.result.is_ok(){calls+=1}assert_eq!(calls,f["childCallsOnDenial"].as_u64().unwrap());
 let admission=OriginalFailureStorage::admit(OriginalFailureStorage::birth_bytes());assert_eq!(admission.work,1);let (reserve,progress)=admission.result.unwrap();
 assert_eq!(progress.retained_capacity_bytes,OriginalFailureStorage::birth_bytes());assert_eq!(progress.released_bytes,0);
 let child=OriginalRetirementFailure::from_primary_and_work(ValueError::literal(crate::ValueRefusalKind::Canceled,"child owner"),ValueError::literal(crate::ValueRefusalKind::WorkLimit,"child settlement"));
 let child_pointer=child.primary().unwrap().message.as_ptr();let (failure,heap)=crate::observe_retirement_allocations(||reserve.attach(child,ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"outer settlement")));assert_eq!(heap,(0,0));
 assert_eq!(failure.work_refusal().unwrap().message,f["outerSettlement"].as_str().unwrap());
 let child=failure.child().unwrap();assert_eq!(child.primary().unwrap().message.as_ptr(),child_pointer);assert_eq!(child.primary().unwrap().message,f["childPrimary"].as_str().unwrap());assert_eq!(child.work_refusal().unwrap().message,f["childSettlement"].as_str().unwrap());close_failure(failure);
}

#[test]
fn unused_nodes_remain_owned_until_paid_release(){
 let mut pool=crate::OriginalFailureStoragePool::default();let bytes=OriginalFailureStorage::birth_bytes();
 for _ in 0..2{let admitted=OriginalFailureStorage::admit(bytes);assert_eq!(admitted.work,1);let (node,receipt)=admitted.result.unwrap();assert_eq!(receipt.retained_capacity_bytes,bytes);pool.put(node)}
 assert!(pool.release_one(bytes-1).is_err());let node=pool.take().unwrap();pool.put(node);
 for _ in 0..2{assert_eq!(pool.release_one(bytes).unwrap().released_bytes,bytes)}assert!(pool.is_empty());
}

#[test]
fn denied_prepaid_node_never_runs_child_or_settlement(){
 let f=fixture();let calls=std::cell::Cell::new(0u64);let mut slot=None;
 let turn=crate::run_with_failure_storage(&mut slot,||{calls.set(calls.get()+1);RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:1}},|_|{calls.set(calls.get()+1);Ok(())},|step|step.progress());
 assert!(turn.is_none());assert_eq!(calls.get(),f["childCallsOnDenial"].as_u64().unwrap());
}
#[test]
fn prepaid_outer_settlement_retains_nested_failures_and_actual_work(){
 let f=fixture();let bytes=OriginalFailureStorage::birth_bytes();let mut slot=Some(OriginalFailureStorage::admit(bytes).result.unwrap().0);let actual=f["actualWork"].as_u64().unwrap();
 let (turn,heap)=crate::observe_retirement_allocations(||crate::run_with_failure_storage(&mut slot,||RetainedWorkOutcome::<RetainedCloneStep,OriginalRetirementFailure>{result:Err(OriginalRetirementFailure::from_primary_and_work(ValueError::literal(crate::ValueRefusalKind::Canceled,"child owner"),ValueError::literal(crate::ValueRefusalKind::WorkLimit,"child settlement"))),work:actual},|work|{assert_eq!(work,actual);Err(ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"outer settlement"))},|step|step.progress()).unwrap());
 assert_eq!(heap,(0,0));assert_eq!(turn.work,actual);assert!(slot.is_none());let failure=turn.result.unwrap_err();assert_eq!(failure.work_refusal().unwrap().message,f["outerSettlement"].as_str().unwrap());let child=failure.child().unwrap();assert_eq!(child.primary().unwrap().message,f["childPrimary"].as_str().unwrap());assert_eq!(child.work_refusal().unwrap().message,f["childSettlement"].as_str().unwrap());close_failure(failure);
}

#[test]
fn failed_node_allocation_and_failed_zero_receipt_remain_distinct(){
 let f=fixture();
 let failure=OriginalRetirementFailure::from_primary_and_receipt(ValueError::literal(crate::ValueRefusalKind::AllocationFailed,"allocation refused"),ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"receipt refused"));
 assert_eq!(failure.primary().unwrap().message,f["allocationFailure"].as_str().unwrap());assert_eq!(failure.receipt_refusal().unwrap().message,f["receiptFailure"].as_str().unwrap());assert_eq!(failure.retained_progress(),Default::default());
}
#[test]
fn prepaid_generic_scalar_result_preserves_actual_work_and_receipt_on_settlement_refusal(){
 let f=fixture();let bytes=OriginalFailureStorage::birth_bytes();let mut slot=Some(OriginalFailureStorage::admit(bytes).result.unwrap().0);let actual=f["actualWork"].as_u64().unwrap();let effects=RetainedCloneProgress{copied_items:f["genericItems"].as_u64().unwrap()as usize,..Default::default()};
 let(turn,heap)=crate::observe_retirement_allocations(||crate::run_with_failure_storage(&mut slot,||RetainedWorkOutcome{result:Ok(f["genericResult"].as_bool().unwrap()),work:actual},|work|{assert_eq!(work,actual);Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"outer settlement"))},|value|{assert!(*value);effects}).unwrap());
 assert_eq!(heap,(0,0));assert_eq!(turn.work,actual);let error=turn.result.unwrap_err();assert_eq!(error.retained_progress(),effects);assert_eq!(error.work_refusal().unwrap().message,f["outerSettlement"].as_str().unwrap());assert!(slot.is_some());
 let node=slot.take().unwrap();let(receipt,heap)=crate::observe_retirement_allocations(||node.release(bytes).unwrap());assert_eq!(receipt.released_bytes,bytes);assert_eq!(heap,(0,bytes));
}

struct Funding{pool:crate::OriginalFailureStoragePool,quotes:u64,posts:u64,posted:RetainedCloneProgress,committed:u64,refuse_post:bool,refuse_commit:bool,allow_prepare:bool,refuse_cancel:bool,cancelled:u64}
impl OriginalFailureFunding for Funding{
 fn quote(&mut self,_:RetirementDemand,grant:RetainedCloneGrant)->Result<RetainedCloneGrant,ValueError>{self.quotes+=1;Ok(grant)}
 fn post(&mut self,_:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>{self.posts+=1;self.posted=progress;if self.refuse_post{Err(ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"receipt refused"))}else{Ok(())}}
 fn cancel(&mut self)->Result<(),ValueError>{self.cancelled+=1;if self.refuse_cancel{Err(ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"receipt refused"))}else{Ok(())}}
 fn pool(&mut self)->&mut crate::OriginalFailureStoragePool{&mut self.pool}
 fn prepare(&mut self,_:u64)->Result<bool,ValueError>{Ok(self.allow_prepare)}
 fn commit(&mut self,work:u64)->Result<(),ValueError>{self.committed+=work;if self.refuse_commit{Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"outer settlement"))}else{Ok(())}}
}
#[test]
fn refused_actual_node_receipt_retains_the_original_paid_node(){
 let bytes=OriginalFailureStorage::birth_bytes();let mut funding=Funding{pool:Default::default(),quotes:0,posts:0,posted:Default::default(),committed:0,refuse_post:true,refuse_commit:false,allow_prepare:true,refuse_cancel:false,cancelled:0};let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:bytes,maximum_release_bytes:bytes,maximum_depth:1,..Default::default()};
 let(turn,heap)=crate::observe_retirement_allocations(||refill_original_failure_storage(&mut funding,grant));let error=turn.result.unwrap_err();assert_eq!(turn.work,1);assert_eq!(funding.committed,1);assert_eq!((funding.quotes,funding.posts),(1,1));assert_eq!(heap,(bytes,0));assert_eq!(funding.posted.retained_capacity_bytes,bytes);assert_eq!(error.retained_progress(),funding.posted);assert_eq!(error.receipt_refusal().unwrap().message,"receipt refused");assert!(!funding.pool.is_empty());
 let(released,heap)=crate::observe_retirement_allocations(||funding.pool.release_one(bytes).unwrap());assert_eq!(released.released_bytes,bytes);assert_eq!(heap,(0,bytes));assert!(funding.pool.is_empty());
}
#[test]
fn allocation_null_posts_zero_and_preserves_post_refusal(){
 let bytes=OriginalFailureStorage::birth_bytes();let mut funding=Funding{pool:Default::default(),quotes:0,posts:0,posted:Default::default(),committed:0,refuse_post:true,refuse_commit:false,allow_prepare:true,refuse_cancel:false,cancelled:0};let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:bytes,maximum_depth:1,..Default::default()};
 let((turn,refused),heap)=crate::observe_retirement_allocations(||crate::allocation_testing::refuse_one_allocation_on_this_thread(||refill_original_failure_storage(&mut funding,grant)));let error=turn.result.unwrap_err();assert_eq!(refused,1);assert_eq!(heap,(0,0));assert_eq!(turn.work,1);assert_eq!(funding.posted,Default::default());assert_eq!(error.primary().unwrap().kind,crate::ValueRefusalKind::AllocationFailed);assert_eq!(error.receipt_refusal().unwrap().message,"receipt refused");assert!(funding.pool.is_empty());
}

impl OriginalRetirementControl for Funding{
 fn prepare_work(&mut self,work:u64)->Result<bool,ValueError>{OriginalFailureFunding::prepare(self,work)}
 fn commit_work(&mut self,work:u64)->Result<(),ValueError>{OriginalFailureFunding::commit(self,work)}
 fn cancel_prepared_work(&mut self){}
 fn ledger_identity(&self)->u64{19}
 fn retirement_recipient_identity(&self)->Option<u64>{Some(23)}
 fn charge(&mut self,bytes:usize)->Result<(),ValueError>{assert_eq!(bytes,0);Ok(())}
 fn quote_failure_storage(&mut self,d:RetirementDemand,g:RetainedCloneGrant)->Result<RetainedCloneGrant,ValueError>{OriginalFailureFunding::quote(self,d,g)}
 fn post_failure_storage(&mut self,g:RetainedCloneGrant,p:RetainedCloneProgress)->Result<(),ValueError>{OriginalFailureFunding::post(self,g,p)}
 fn cancel_failure_storage(&mut self)->Result<(),ValueError>{Ok(())}
 fn failure_storage_pool(&mut self)->&mut crate::OriginalFailureStoragePool{&mut self.pool}
}
struct RefusingChild{calls:std::sync::Arc<std::sync::atomic::AtomicUsize>,failed:bool,done:bool}
impl ErasedSnapshotRetirement for RefusingChild{
 fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{Ok(if self.failed{1}else{3})}
 fn close_step(&mut self,_:RetainedCloneGrant)->OriginalRetirementOutcome{self.calls.fetch_add(1,std::sync::atomic::Ordering::SeqCst);if !self.failed{self.failed=true;return RetainedWorkOutcome{result:Err(OriginalRetirementFailure::from_primary_and_work(ValueError::literal(crate::ValueRefusalKind::Canceled,"child owner"),ValueError::literal(crate::ValueRefusalKind::WorkLimit,"child settlement"))),work:3}}self.done=true;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}
 fn terminal_is_empty(&self)->bool{self.done}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
 fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(1)}
}
#[test]
fn original_ticket_retains_child_and_both_settlement_refusals(){
 let f=fixture();let bytes=OriginalFailureStorage::birth_bytes();let calls=std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));let mut slot=Some(Box::new(RefusingChild{calls:calls.clone(),failed:false,done:false}));let child_pointer=slot.as_ref().unwrap().as_ref() as *const RefusingChild;let mut funding=Funding{pool:Default::default(),quotes:0,posts:0,posted:Default::default(),committed:0,refuse_post:false,refuse_commit:false,allow_prepare:true,refuse_cancel:false,cancelled:0};
 let first=crate::factory_ticket_controlled_demands(slot.as_ref().unwrap(),0,&mut funding).unwrap();assert_eq!(first.capacity_bytes,bytes);let denied=RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()};let turn=crate::close_factory_ticket_controlled(&mut slot,denied,&mut funding);assert_eq!(turn.work,0);assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst),0);assert!(slot.is_some());
 let refill=RetainedCloneGrant{maximum_capacity_bytes:bytes,..denied};let(turn,heap)=crate::observe_retirement_allocations(||crate::close_factory_ticket_controlled(&mut slot,refill,&mut funding));assert!(turn.result.is_ok());assert_eq!(turn.work,1);assert_eq!(heap,(bytes,0));assert!(slot.is_some());let next=crate::factory_ticket_controlled_demands(slot.as_ref().unwrap(),0,&mut funding).unwrap();assert_eq!(next.capacity_bytes,0);funding.refuse_commit=true;
 let(turn,heap)=crate::observe_retirement_allocations(||crate::close_factory_ticket_controlled(&mut slot,denied,&mut funding));assert_eq!(heap,(0,0));assert_eq!(turn.work,f["actualWork"].as_u64().unwrap());assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst),1);assert!(slot.is_some());let failure=turn.result.unwrap_err();assert_eq!(failure.work_refusal().unwrap().message,f["outerSettlement"].as_str().unwrap());let child=failure.child().unwrap();assert_eq!(child.primary().unwrap().message,f["childPrimary"].as_str().unwrap());assert_eq!(child.work_refusal().unwrap().message,f["childSettlement"].as_str().unwrap());assert!(funding.pool.is_empty());assert_eq!(slot.as_ref().unwrap().as_ref() as *const RefusingChild,child_pointer);close_failure(failure);
 funding.refuse_commit=false;let(turn,heap)=crate::observe_retirement_allocations(||crate::close_factory_ticket_controlled(&mut slot,refill,&mut funding));assert!(turn.result.is_ok());assert_eq!(heap,(bytes,0));let(turn,heap)=crate::observe_retirement_allocations(||crate::close_factory_ticket_controlled(&mut slot,denied,&mut funding));assert_eq!(turn.work,1);assert_eq!(heap,(0,0));assert!(slot.as_ref().unwrap().terminal_is_empty());
 let shell=std::mem::size_of::<RefusingChild>();let close=RetainedCloneGrant{maximum_release_bytes:shell,..denied};let(turn,heap)=crate::observe_retirement_allocations(||crate::close_factory_ticket_controlled(&mut slot,close,&mut funding));assert_eq!(turn.work,1);assert_eq!(turn.result.unwrap().progress().released_bytes,shell);assert_eq!(heap,(0,shell));assert!(slot.is_none());let(released,heap)=crate::observe_retirement_allocations(||funding.pool.release_one(bytes).unwrap());assert_eq!(released.released_bytes,bytes);assert_eq!(heap,(0,bytes));
}

struct ChangingPool{pools:[crate::OriginalFailureStoragePool;2],selected:usize,posted:u64,pending:bool}
impl OriginalRetirementControl for ChangingPool{
 fn prepare_work(&mut self,_:u64)->Result<bool,ValueError>{assert!(!self.pending);self.pending=true;Ok(true)}
 fn commit_work(&mut self,actual:u64)->Result<(),ValueError>{assert!(self.pending);self.pending=false;self.posted+=actual;Ok(())}
 fn cancel_prepared_work(&mut self){self.pending=false}
 fn ledger_identity(&self)->u64{19}
 fn retirement_recipient_identity(&self)->Option<u64>{Some(23)}
 fn charge(&mut self,_:usize)->Result<(),ValueError>{self.selected=1;Ok(())}
 fn quote_failure_storage(&mut self,_:RetirementDemand,g:RetainedCloneGrant)->Result<RetainedCloneGrant,ValueError>{Ok(g)}
 fn post_failure_storage(&mut self,g:RetainedCloneGrant,p:RetainedCloneProgress)->Result<(),ValueError>{assert!(p.fits(g));Ok(())}
 fn cancel_failure_storage(&mut self)->Result<(),ValueError>{Ok(())}
 fn failure_storage_pool(&mut self)->&mut crate::OriginalFailureStoragePool{&mut self.pools[self.selected]}
}
#[test]
fn control_pool_replacement_retains_original_node_and_actual_work(){
 let bytes=OriginalFailureStorage::birth_bytes();let mut control=ChangingPool{pools:[Default::default(),Default::default()],selected:0,posted:0,pending:false};let(node,_)=OriginalFailureStorage::admit(bytes).result.unwrap();control.pools[0].put(node);let error=ValueError::new(crate::ValueRefusalKind::Canceled,"owned child cause".to_owned());let pointer=error.message.as_ptr();
 let(turn,heap)=crate::observe_retirement_allocations(||crate::run_original_retirement_work(&mut control,3,|control|{control.charge(0).unwrap();RetainedWorkOutcome{result:Err(error.into()),work:3}}));assert_eq!(heap,(0,0));assert_eq!(turn.work,3);assert_eq!(control.posted,3);assert!(control.pools[0].is_empty());assert!(!control.pools[1].is_empty());let failure=turn.result.unwrap_err();assert_eq!(failure.primary().unwrap().message.as_ptr(),pointer);close_failure(failure);assert_eq!(control.pools[1].release_one(bytes).unwrap().released_bytes,bytes);
}
#[test]
fn panicking_effect_returns_original_prepaid_node_to_current_pool(){
 let bytes=OriginalFailureStorage::birth_bytes();let mut control=ChangingPool{pools:[Default::default(),Default::default()],selected:0,posted:0,pending:false};let(node,_)=OriginalFailureStorage::admit(bytes).result.unwrap();control.pools[0].put(node);
 let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||crate::run_original_retirement_work(&mut control,1,|control|{control.charge(0).unwrap();panic!("fixture effect panic")})));assert!(result.is_err());assert_eq!(control.pending,fixture()["unwindReservationHeld"].as_bool().unwrap());assert!(control.pools[0].is_empty());assert!(!control.pools[1].is_empty());assert_eq!(control.pools[1].release_one(bytes).unwrap().released_bytes,bytes);control.cancel_prepared_work();assert!(!control.pending);
}

#[test]
fn refused_work_cancels_the_exact_original_node_reservation(){
 let bytes=OriginalFailureStorage::birth_bytes();let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:bytes,maximum_depth:1,..Default::default()};
 for refuse_cancel in [false,true]{let mut funding=Funding{pool:Default::default(),quotes:0,posts:0,posted:Default::default(),committed:0,refuse_post:false,refuse_commit:false,allow_prepare:false,refuse_cancel,cancelled:0};let(turn,heap)=crate::observe_retirement_allocations(||refill_original_failure_storage(&mut funding,grant));assert_eq!(heap,(0,0));assert_eq!(turn.work,0);assert_eq!((funding.quotes,funding.posts,funding.cancelled,funding.committed),(1,0,1,0));assert!(funding.pool.is_empty());if refuse_cancel{assert_eq!(turn.result.unwrap_err().receipt_refusal().unwrap().message,"receipt refused")}else{assert_eq!(turn.result.unwrap().progress(),Default::default())}}
}
