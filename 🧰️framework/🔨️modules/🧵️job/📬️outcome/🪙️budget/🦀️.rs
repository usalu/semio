//! 🪙️ Typed return-budget storage, conservation and physical closure without context capability minting.
use semio_framework_value_retained_clone::{RetainedCloneGrant,RetainedCloneProgress as P,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value_retained_clone::retirement::controlled::{RetainedOwnerGate,RetainedOwnerGateGuard};
use semio_framework_job_return_escrow_publication::{OriginalEscrowPublisher,OriginalEscrowPublisherBirth,OriginalEscrowPublication};
use semio_framework_job_payload::{OriginalReturnEscrowTable,OriginalReturnEscrowBirth,OriginalReturnEscrowMessages as E,OriginalReturnEscrowAccounting as M,OriginalReturnEscrowRow,OriginalReturnEscrowIdentity,ORIGINAL_RETURN_ESCROW_RECLAIM_COPY_BYTES};
pub use semio_framework_job_payload::{OriginalReturnBudgetSnapshot,OriginalReturnEscrowAmounts};
const RETURN_SOURCE_CONTROL_WORK:u64=1;
fn refused(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn pay_close(effect:&mut semio_framework_value_retained_clone::RetainedAllocationEffect<'_>,work:u64)->Result<bool,(ValueError,u128)>{let remaining=effect.remaining_work_units().map_err(|error|(error,0))?;if remaining<work{return Ok(false)}match effect.performed(work){Ok(true)=>Ok(true),Ok(false)=>Err((ValueError::literal(ValueRefusalKind::WorkLimit,"return Source close exceeded its original Work grant"),u128::from(work))),Err(error)=>Err((error,0))}}
/// 🪪️ Data-only origin observations; the context owns the actual Source-qualified input constructor.
pub trait OriginalReturnOrigin:Copy{
 fn same_origin(&self,other:&Self)->bool;
 fn recipient(&self)->usize;
 fn source_identity(&self)->usize;
 fn source_turn(&self)->Option<semio_framework_value_control_contract::OriginalSourceTurnIdentity>;
 fn recipient_identity(&self)->usize{self.recipient()}
 fn granted(&self)->RetainedCloneGrant;
 fn fuel(&self)->u64;
 fn spent(&self)->P;
 fn work(&self)->u128;
}
/// 🔒️ See `README.md`.
pub struct OriginalReturnSource<I:OriginalReturnOrigin>{budget:RetainedOwnerGate<OriginalReturnBudget<I>>}
impl<I:OriginalReturnOrigin> OriginalReturnSource<I>{
 pub const fn empty(birth:u64)->Self{Self{budget:RetainedOwnerGate::new(OriginalReturnBudget::<I>::empty(birth))}}
 pub fn try_borrow(&self)->Option<OriginalReturnBudgetReference<'_,I>>{self.budget.try_lock().ok().map(OriginalReturnBudgetReference::Owned)}
 /// 🪪️ Checks the exact held Source guard without acquiring another lock.
 pub fn owns_reference(&self,reference:&OriginalReturnBudgetReference<'_,I>)->bool{match reference{OriginalReturnBudgetReference::Owned(guard)=>self.budget.owns_guard(guard),OriginalReturnBudgetReference::Borrowed(_)=>false}}
 pub fn guards_header_pointer(&self,pointer:usize)->Result<bool,ValueError>{let b=self.try_borrow().ok_or_else(||refused("header Source busy"))?;Ok(b.get().publisher.as_ref().is_some_and(|p|p.owns_header_pointer(pointer)))}
 /// 📏️ Quotes the physical demand of the same selected close primitive that close_step can execute.
 pub fn retirement_demands(&self)->Result<semio_framework_value_retained_clone::RetirementDemand,ValueError>{
  use semio_framework_value_retained_clone::RetirementDemand as D;
  let budget=self.try_borrow().ok_or_else(||refused("return-source close quote is busy"))?;let b=budget.get();
  if b.publication.is_some(){return Ok(D::default())}
  if let Some(publisher)=b.publisher.as_ref(){if !publisher.terminal_is_empty(){return Ok(D::default())}let(copy,release,_)=OriginalEscrowPublisher::retirement_demand();return Ok(D{copy_bytes:copy+2*std::mem::size_of::<bool>(),release_bytes:release,depth:1,..Default::default()})}
  if let Some(table)=b.table.as_ref(){if table.has_reclaimable(){return Ok(D{copy_bytes:ORIGINAL_RETURN_ESCROW_RECLAIM_COPY_BYTES,depth:1,..Default::default()})}if !table.is_empty(){return Ok(D::default())}return Ok(D{copy_bytes:std::mem::size_of::<Option<Box<OriginalReturnEscrowTable>>>(),release_bytes:std::mem::size_of::<OriginalReturnEscrowTable>(),depth:1,..Default::default()})}
  if b.origin.is_none()||b.held_items!=0||b.held_copy_bytes!=0||b.held_release_bytes!=0||b.held_capacity_bytes!=0||b.held_fuel!=0||b.claims!=0{return Ok(D::default())}
  Ok(D{depth:1,..Default::default()})
 }
 pub fn retains_claims(&self)->bool{self.budget.try_lock().map_or(true,|budget|budget.publication.is_some()||budget.origin_pending.is_some()||budget.origin_action.is_some()||budget.origin_commit.is_some()||budget.table_pending.is_some()||budget.table_action.is_some()||budget.table_commit.is_some()||budget.publisher_pending.is_some()||budget.publisher_action.is_some()||budget.publisher_commit.is_some()||!budget.table_birth.is_empty()||!budget.publisher_birth.is_empty()||budget.claims!=0||budget.held_items!=0||budget.held_copy_bytes!=0||budget.held_release_bytes!=0||budget.held_capacity_bytes!=0||budget.held_fuel!=0||budget.table.as_ref().is_some_and(|table|!table.is_empty()&&!table.is_reclaim_only()))}
 pub fn terminal_is_empty(&self)->bool{self.budget.try_lock().is_ok_and(|budget|budget.origin.is_none()&&budget.origin_prior.is_none()&&budget.origin_pending.is_none()&&budget.origin_action.is_none()&&budget.origin_commit.is_none()&&budget.table.is_none()&&budget.table_pending.is_none()&&budget.table_action.is_none()&&budget.table_commit.is_none()&&budget.table_birth.is_empty()&&budget.publisher.is_none()&&budget.publisher_pending.is_none()&&budget.publisher_action.is_none()&&budget.publisher_commit.is_none()&&budget.publisher_birth.is_empty()&&budget.publication.is_none()&&!budget.publisher_placed&&!budget.publisher_fenced&&budget.held_items==0&&budget.held_copy_bytes==0&&budget.held_release_bytes==0&&budget.held_capacity_bytes==0&&budget.held_fuel==0&&budget.claims==0)}
 /// 🧮️ Quotes only the same selected physical close primitive that close_step can execute.
 pub fn retirement_work_demand(&self)->Result<u64,ValueError>{let budget=self.try_borrow().ok_or_else(||refused("return-source close quote is busy"))?;let b=budget.get();if b.publication.is_some()||b.origin_pending.is_some()||b.origin_action.is_some()||b.origin_commit.is_some()||b.table_pending.is_some()||b.table_action.is_some()||b.publisher_pending.is_some()||b.publisher_action.is_some(){return Ok(0)}if !b.table_birth.is_empty()||!b.publisher_birth.is_empty(){return Ok(1)}if let Some(publisher)=b.publisher.as_ref(){return Ok(if publisher.terminal_is_empty(){OriginalEscrowPublisher::retirement_demand().2}else{0})}if let Some(table)=b.table.as_ref(){return Ok(if table.has_reclaimable()||table.is_empty(){RETURN_SOURCE_CONTROL_WORK}else{0})}Ok(if b.origin.is_some()&&b.held_items==0&&b.held_copy_bytes==0&&b.held_release_bytes==0&&b.held_capacity_bytes==0&&b.held_fuel==0&&b.claims==0{RETURN_SOURCE_CONTROL_WORK}else{0})}
 pub fn close_step(&mut self,effect:&mut semio_framework_value_retained_clone::RetainedAllocationEffect<'_>)->RetainedWorkOutcome{let grant=effect.grant();
  let budget=self.budget.get_mut();if budget.publication.is_some()||budget.table_pending.is_some()||budget.publisher_pending.is_some(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P::default())),work:0}}if !budget.table_birth.is_empty(){let before=effect.performed_work();let mut slot=None;let reply=match semio_framework_value_control_contract::OriginalEffectReply::borrow(&mut slot){Ok(reply)=>reply,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};let complete=budget.table_birth.cancel_into(effect,reply);if !complete.is_recorded(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P::default())),work:0}}return RetainedWorkOutcome{result:slot.expect("native cancellation reply"),work:effect.performed_work()-before}}if !budget.publisher_birth.is_empty(){let before=effect.performed_work();let mut slot=None;let reply=match semio_framework_value_control_contract::OriginalEffectReply::borrow(&mut slot){Ok(reply)=>reply,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};let complete=budget.publisher_birth.cancel_into(effect,reply);if !complete.is_recorded(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P::default())),work:0}}return RetainedWorkOutcome{result:slot.expect("native publisher cancellation reply"),work:effect.performed_work()-before}}if let Some(publisher)=budget.publisher.as_ref(){let(copy,release,work)=OriginalEscrowPublisher::retirement_demand();let copy=copy+2*std::mem::size_of::<bool>();if !publisher.terminal_is_empty()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy||grant.maximum_release_bytes<release{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P::default())),work:0}}match pay_close(effect,work){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P::default())),work:0},Err((error,work))=>return RetainedWorkOutcome{result:Err(error),work}}drop(budget.publisher.take());budget.publisher_placed=false;budget.publisher_fenced=false;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P{copied_items:1,copied_bytes:copy,released_bytes:release,..Default::default()})),work:u128::from(work)}}if let Some(table)=budget.table.as_ref(){if table.has_reclaimable(){if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<ORIGINAL_RETURN_ESCROW_RECLAIM_COPY_BYTES{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P::default())),work:0}}match pay_close(effect,RETURN_SOURCE_CONTROL_WORK){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P::default())),work:0},Err((error,work))=>return RetainedWorkOutcome{result:Err(error),work}}let result=budget.table.as_mut().unwrap().reclaim_one();return RetainedWorkOutcome{result:result.map(|_|RetainedCloneStep::Progress(P{copied_items:1,copied_bytes:ORIGINAL_RETURN_ESCROW_RECLAIM_COPY_BYTES,..Default::default()})),work:u128::from(RETURN_SOURCE_CONTROL_WORK)}}let bytes=std::mem::size_of::<OriginalReturnEscrowTable>();let copy=std::mem::size_of::<Option<Box<OriginalReturnEscrowTable>>>();if !table.is_empty()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy||grant.maximum_release_bytes<bytes{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P::default())),work:0}}match pay_close(effect,RETURN_SOURCE_CONTROL_WORK){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P::default())),work:0},Err((error,work))=>return RetainedWorkOutcome{result:Err(error),work}}drop(budget.table.take());return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P{copied_items:1,copied_bytes:copy,released_bytes:bytes,..Default::default()})),work:u128::from(RETURN_SOURCE_CONTROL_WORK)}}if budget.origin.is_none(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(P::default())),work:0}}
  if budget.held_items!=0||budget.held_copy_bytes!=0||budget.held_release_bytes!=0||budget.held_capacity_bytes!=0||budget.held_fuel!=0||budget.claims!=0||grant.maximum_items==0||grant.maximum_depth==0{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P::default())),work:0}}
  match pay_close(effect,RETURN_SOURCE_CONTROL_WORK){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(P::default())),work:0},Err((error,work))=>return RetainedWorkOutcome{result:Err(error),work}}*budget=OriginalReturnBudget::<I>::empty(budget.birth);RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(P{copied_items:1,..Default::default()})),work:u128::from(RETURN_SOURCE_CONTROL_WORK)}
 }
}
impl<I:OriginalReturnOrigin> Drop for OriginalReturnSource<I>{fn drop(&mut self){let budget=self.budget.get_mut();assert!(std::thread::panicking()||budget.origin.is_none()&&budget.table.is_none()&&budget.table_birth.is_empty()&&budget.publisher.is_none()&&budget.publisher_birth.is_empty()&&budget.publication.is_none()&&!budget.publisher_placed&&!budget.publisher_fenced&&budget.held_items==0&&budget.held_copy_bytes==0&&budget.held_release_bytes==0&&budget.held_capacity_bytes==0&&budget.held_fuel==0&&budget.claims==0,"original return source retains its bound budget or live claims");}}
/// 🧵️ Borrows the actual Source and frame-issued construction action for one retained origin producer.
pub struct OriginalReturnOriginContext<'turn,'source,I:OriginalReturnOrigin>{pub binding:&'turn semio_framework_value_native_receiving_frame::NativeReceivingConstructionBinding,pub budget:&'turn mut OriginalReturnBudgetReference<'source,I>}
/// 🪑️ Retains the real origin reply across the same Source action until its paid outer publication.
pub struct OriginalReturnOriginProducer<'a,I:OriginalReturnOrigin+'static>{source:&'a OriginalReturnSource<I>,input:I,error:std::mem::ManuallyDrop<Option<ValueError>>,progress:P,recorded:bool,complete:bool}
impl<'a,I:OriginalReturnOrigin+'static> OriginalReturnOriginProducer<'a,I>{
 /// 🔐️ Checks the existing Source guard without acquiring another lock.
 pub fn authenticates(&self,budget:&OriginalReturnBudgetReference<'_,I>)->bool{self.source.owns_reference(budget)}
 /// 📏️ Quotes the actual input and resident authority slots before moving either owner.
 pub fn admission_demand()->semio_framework_value_retained_clone::RetirementDemand{semio_framework_value_retained_clone::RetirementDemand{copy_bytes:std::mem::size_of::<Option<I>>()+std::mem::size_of::<Option<semio_framework_value_control_contract::OriginalEffectAuthority<Self>>>(),depth:1,..Default::default()}}
 /// 🏠️ Admits only the original Source and captured input under the actual paid action.
 pub fn admit(source:&'a OriginalReturnSource<I>,budget:&OriginalReturnBudgetReference<'_,I>,input:&mut Option<I>,output:&mut Option<semio_framework_value_control_contract::OriginalEffectAuthority<Self>>,binding:&semio_framework_value_native_receiving_frame::NativeReceivingConstructionBinding,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<P>,ValueError>{
  let fail=|error|RetainedWorkOutcome{result:Err(error),work:0};if output.is_some(){return fail(refused("original return producer destination is occupied"))}let Some(original)=input.as_ref()else{return RetainedWorkOutcome{result:Ok(None),work:0}};
  if original.source_turn()!=binding.source()||!binding.matches_effect(effect){return fail(refused("original return producer changed its Source action"))}
  if !source.owns_reference(budget)||original.recipient()!=budget.get()as*const OriginalReturnBudget<I> as usize{return fail(refused("original return producer changed its receiving Source"))}
  let d=Self::admission_demand();if !semio_framework_value_control_contract::retained_clone_grant_admits(effect.grant(),d){return RetainedWorkOutcome{result:Ok(None),work:0}}match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Ok(None),work:0},Err(error)=>return fail(error)}
  let original=input.take().expect("validated original return input");*output=Some(semio_framework_value_control_contract::OriginalEffectAuthority::new(Self{source,input:original,error:std::mem::ManuallyDrop::new(None),progress:Default::default(),recorded:false,complete:false}));RetainedWorkOutcome{result:Ok(Some(P{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}
 }
 /// 🪪️ Observes the actual captured Source turn without issuing a new authority.
 pub fn source_turn(&self)->Option<semio_framework_value_control_contract::OriginalSourceTurnIdentity>{self.input.source_turn()}
 fn receipt_bytes()->usize{std::mem::size_of::<Option<ValueError>>()+std::mem::size_of::<P>()+2*std::mem::size_of::<bool>()}
 /// 📏️ Quotes the actual pending reply, Source placement and paid outer result.
 pub fn demand()->Result<semio_framework_value_retained_clone::RetirementDemand,ValueError>{
  let output=std::mem::size_of::<Option<Result<RetainedCloneStep,semio_framework_value_native_outcome::OriginalReceivingPrimary>>>();
  let copy_bytes=OriginalReturnBudget::<I>::refresh_turn_demand().copy_bytes.checked_add(2*Self::receipt_bytes()).and_then(|n|n.checked_add(output)).ok_or_else(||refused("original return origin reply extent overflow"))?;
  Ok(semio_framework_value_retained_clone::RetirementDemand{copy_bytes,depth:1,..Default::default()})
 }
 pub const fn work_demand()->u64{3}
 pub const fn item_demand()->usize{3}
}
impl<I:OriginalReturnOrigin+'static> semio_framework_value_control_contract::OriginalEffectProducer for OriginalReturnOriginProducer<'_,I>{
 type Output=RetainedCloneStep;type Failure=semio_framework_value_native_outcome::OriginalReceivingPrimary;
 type Context<'turn,'source>=OriginalReturnOriginContext<'turn,'source,I> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Result<(),ValueError>where 'source:'turn{if self.input.source_turn().is_none()||self.input.source_turn()!=c.binding.source()||!self.source.owns_reference(&*c.budget)||self.input.recipient()!=c.budget.get()as*const OriginalReturnBudget<I> as usize{return Err(refused("original return origin producer changed Source"))}Ok(())}
 fn source_turn<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Option<semio_framework_value_control_contract::OriginalSourceTurnIdentity>where 'source:'turn{c.binding.source()}
 fn receive<'reply,'turn,'source>(&mut self,c:&mut Self::Context<'turn,'source>,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,Self::Failure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,Self::Failure>,ValueError>where 'source:'turn{
  self.validate_context(c)?;if self.recorded||self.error.is_some()||!c.binding.matches_effect(effect){return Err(refused("original return origin producer changed its action"))}
  let demand=Self::demand()?;if effect.grant().maximum_items<Self::item_demand()||!semio_framework_value_control_contract::retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<Self::work_demand(){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if !effect.performed(1)?{return Ok(reply.pending())}
  let budget=c.budget.get_mut();let turn=if budget.origin.is_some(){budget.refresh_turn(self.input,c.binding,effect)}else{budget.stage_initial_origin(self.input,c.binding,effect)};let mut progress=P{copied_items:1,copied_bytes:Self::receipt_bytes(),..Default::default()};
  match turn.result{Ok(Some(actual))=>{progress=progress.checked_add(actual)?;self.complete=true},Ok(None)=>self.complete=false,Err(error)=>{*self.error=Some(error);self.complete=false}}
  self.progress=progress;self.recorded=true;Ok(reply.pending())
 }
 fn recover<'reply,'turn,'source>(&mut self,c:&mut Self::Context<'turn,'source>,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,Self::Failure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,Self::Failure>,ValueError>where 'source:'turn{
  self.validate_context(c)?;if !c.binding.matches_effect(effect){return Err(refused("original return origin reply changed its action"))}if !self.recorded{return self.receive(c,effect,reply)}
  let bytes=Self::receipt_bytes()+std::mem::size_of::<Option<Result<RetainedCloneStep,semio_framework_value_native_outcome::OriginalReceivingPrimary>>>();let d=semio_framework_value_retained_clone::RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()};if !semio_framework_value_control_contract::retained_clone_grant_admits(effect.grant(),d)||effect.remaining_work_units()?<1||!effect.performed(1)?{return Ok(reply.pending())}
  let progress=self.progress.checked_add(P{copied_items:1,copied_bytes:bytes,..Default::default()})?;let error=self.error.take();let complete=self.complete;self.progress=Default::default();self.complete=false;self.recorded=false;
  let result=match error{Some(error)=>Err(semio_framework_value_native_outcome::OriginalReceivingPrimary::from_fields(Some(error),None,None,None,None,None,None,None,None,progress)),None=>Ok(if complete{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})};Ok(reply.complete(result))
 }
}
impl<I:OriginalReturnOrigin+'static> Drop for OriginalReturnOriginProducer<'_,I>{fn drop(&mut self){assert!(std::thread::panicking()||!self.recorded&&self.error.is_none(),"original return origin producer retains its original reply")}}

/// 🧵️ Lends the exact accepted origin proof and original budget guard to the paid commit producer.
pub struct OriginalReturnOriginCommitContext<'turn,'source,I:OriginalReturnOrigin>{pub binding:&'turn semio_framework_value_native_receiving_frame::NativeReceivingConstructionBinding,pub accepted:&'turn semio_framework_value_native_outcome::OriginalConstructionAcceptedProof<'turn>,pub budget:&'turn mut OriginalReturnBudgetReference<'source,I>}
/// 🔒️ The committed origin remains fenced until this producer's own actual Post is accepted.
pub struct OriginalReturnOriginCommitProducer<'a,I:OriginalReturnOrigin+'static>{source:&'a OriginalReturnSource<I>,error:std::mem::ManuallyDrop<Option<ValueError>>,progress:P,recorded:bool,complete:bool}
impl<'a,I:OriginalReturnOrigin+'static> OriginalReturnOriginCommitProducer<'a,I>{
 /// 🔐️ Checks the existing Source guard without acquiring another lock.
 pub fn authenticates(&self,budget:&OriginalReturnBudgetReference<'_,I>)->bool{self.source.owns_reference(budget)}
 fn receipt_bytes()->usize{std::mem::size_of::<Option<ValueError>>()+std::mem::size_of::<P>()+2*std::mem::size_of::<bool>()}
 pub fn admission_demand()->semio_framework_value_retained_clone::RetirementDemand{semio_framework_value_retained_clone::RetirementDemand{copy_bytes:std::mem::size_of::<Option<semio_framework_value_control_contract::OriginalEffectAuthority<Self>>>(),depth:1,..Default::default()}}
 /// 🏠️ Retains the same actual Source and accepted original proof before its commit action.
 pub fn admit(source:&'a OriginalReturnSource<I>,budget:&OriginalReturnBudgetReference<'_,I>,accepted:&semio_framework_value_native_outcome::OriginalConstructionAcceptedProof<'_>,output:&mut Option<semio_framework_value_control_contract::OriginalEffectAuthority<Self>>,binding:&semio_framework_value_native_receiving_frame::NativeReceivingConstructionBinding,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<P>,ValueError>{
  let fail=|error|RetainedWorkOutcome{result:Err(error),work:0};if output.is_some()||!source.owns_reference(budget)||!binding.matches_effect(effect){return fail(refused("original Source commit producer changed its destination or action"))}match budget.get().origin_post_is_accepted(accepted){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Ok(None),work:0},Err(error)=>return fail(error)}
  let d=Self::admission_demand();if !semio_framework_value_control_contract::retained_clone_grant_admits(effect.grant(),d){return RetainedWorkOutcome{result:Ok(None),work:0}}match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Ok(None),work:0},Err(error)=>return fail(error)}
  *output=Some(semio_framework_value_control_contract::OriginalEffectAuthority::new(Self{source,error:std::mem::ManuallyDrop::new(None),progress:Default::default(),recorded:false,complete:false}));RetainedWorkOutcome{result:Ok(Some(P{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}
 }
 pub fn demand(budget:&OriginalReturnBudget<I>)->Result<semio_framework_value_retained_clone::RetirementDemand,ValueError>{let output=std::mem::size_of::<Option<Result<RetainedCloneStep,semio_framework_value_native_outcome::OriginalReceivingPrimary>>>();let copy_bytes=budget.origin_commit_demand().copy_bytes.checked_add(2*Self::receipt_bytes()).and_then(|n|n.checked_add(output)).ok_or_else(||refused("original Source commit reply extent overflow"))?;Ok(semio_framework_value_retained_clone::RetirementDemand{copy_bytes,depth:1,..Default::default()})}
 pub const fn work_demand()->u64{3}
 pub const fn item_demand()->usize{3}
}
impl<I:OriginalReturnOrigin+'static> semio_framework_value_control_contract::OriginalEffectProducer for OriginalReturnOriginCommitProducer<'_,I>{
 type Output=RetainedCloneStep;type Failure=semio_framework_value_native_outcome::OriginalReceivingPrimary;
 type Context<'turn,'source>=OriginalReturnOriginCommitContext<'turn,'source,I> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Result<(),ValueError>where 'source:'turn{if !self.source.owns_reference(&*c.budget)||c.binding.source().is_none()||c.accepted.work_binding().source_turn()!=c.binding.source(){return Err(refused("original Source commit changed its held Source"))}Ok(())}
 fn source_turn<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Option<semio_framework_value_control_contract::OriginalSourceTurnIdentity>where 'source:'turn{c.binding.source()}
 fn receive<'reply,'turn,'source>(&mut self,c:&mut Self::Context<'turn,'source>,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,Self::Failure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,Self::Failure>,ValueError>where 'source:'turn{
  self.validate_context(c)?;if self.recorded||self.error.is_some()||!c.binding.matches_effect(effect){return Err(refused("original Source commit changed its action"))}
  let d=Self::demand(c.budget.get())?;if effect.grant().maximum_items<Self::item_demand()||!semio_framework_value_control_contract::retained_clone_grant_admits(effect.grant(),d)||effect.remaining_work_units()?<Self::work_demand(){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if !effect.performed(1)?{return Ok(reply.pending())}let turn=c.budget.get_mut().commit_origin(c.accepted,c.binding,effect);let mut progress=P{copied_items:1,copied_bytes:Self::receipt_bytes(),..Default::default()};match turn.result{Ok(Some(actual))=>{progress=progress.checked_add(actual)?;self.complete=true},Ok(None)=>self.complete=false,Err(error)=>{*self.error=Some(error);self.complete=false}}self.progress=progress;self.recorded=true;Ok(reply.pending())
 }
 fn recover<'reply,'turn,'source>(&mut self,c:&mut Self::Context<'turn,'source>,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,Self::Failure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,Self::Failure>,ValueError>where 'source:'turn{
  self.validate_context(c)?;if !c.binding.matches_effect(effect){return Err(refused("original Source commit reply changed its action"))}if !self.recorded{return self.receive(c,effect,reply)}
  let bytes=Self::receipt_bytes()+std::mem::size_of::<Option<Result<RetainedCloneStep,semio_framework_value_native_outcome::OriginalReceivingPrimary>>>();let d=semio_framework_value_retained_clone::RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()};if !semio_framework_value_control_contract::retained_clone_grant_admits(effect.grant(),d)||effect.remaining_work_units()?<1||!effect.performed(1)?{return Ok(reply.pending())}
  let progress=self.progress.checked_add(P{copied_items:1,copied_bytes:bytes,..Default::default()})?;let error=self.error.take();let complete=self.complete;self.progress=Default::default();self.complete=false;self.recorded=false;let result=match error{Some(error)=>Err(semio_framework_value_native_outcome::OriginalReceivingPrimary::from_fields(Some(error),None,None,None,None,None,None,None,None,progress)),None=>Ok(if complete{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})};Ok(reply.complete(result))
 }
}
impl<I:OriginalReturnOrigin+'static> Drop for OriginalReturnOriginCommitProducer<'_,I>{fn drop(&mut self){assert!(std::thread::panicking()||!self.recorded&&self.error.is_none(),"original Source commit producer retains its original reply")}}

pub enum OriginalReturnBudgetReference<'a,I:OriginalReturnOrigin>{Owned(RetainedOwnerGateGuard<'a,OriginalReturnBudget<I>>),Borrowed(&'a mut OriginalReturnBudget<I>)}
impl<I:OriginalReturnOrigin> OriginalReturnBudgetReference<'_,I>{
 pub fn get(&self)->&OriginalReturnBudget<I>{match self{Self::Owned(budget)=>budget,Self::Borrowed(budget)=>budget}}
 pub fn get_mut(&mut self)->&mut OriginalReturnBudget<I>{match self{Self::Owned(budget)=>budget,Self::Borrowed(budget)=>budget}}
}
#[derive(Clone,Copy,PartialEq,Eq)]
pub enum OriginalReturnDelivery{LocalWitness,CoreWireSnapshot}
#[derive(Clone,Copy)]
struct OriginalReturnBirthLock{action:semio_framework_value_control_contract::OriginalConstructionActionIdentity,work:semio_framework_value_control_contract::OriginalWorkBindingIdentity,work_action:semio_framework_value_control_contract::OriginalWorkActionIdentity}
fn bind_birth_action(slot:&mut Option<OriginalReturnBirthLock>,witness:&semio_framework_value_native_receiving_frame::NativeReceivingConstructionBinding,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<P>,ValueError>{
 let fail=|error|RetainedWorkOutcome{result:Err(error),work:0};let work=witness.work();
 if !witness.matches_effect(effect){return fail(refused("original return birth changed its granted Source action"))}
 if let Some(original)=slot.as_ref(){return if original.action==witness.action()&&original.work==work&&original.work_action==effect.work_identity().action_identity(){RetainedWorkOutcome{result:Ok(None),work:0}}else{fail(refused("original return birth retains another construction action"))}}
 let bytes=std::mem::size_of::<Option<OriginalReturnBirthLock>>();let grant=effect.grant();if grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return RetainedWorkOutcome{result:Ok(None),work:0}}
 match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Ok(None),work:0},Err(error)=>return fail(error)}
 *slot=Some(OriginalReturnBirthLock{action:witness.action(),work,work_action:effect.work_identity().action_identity()});RetainedWorkOutcome{result:Ok(Some(P{copied_items:1,copied_bytes:bytes,..Default::default()})),work:1}
}
fn birth_post_accepted(lock:&OriginalReturnBirthLock,proof:&semio_framework_value_native_outcome::OriginalConstructionAcceptedProof<'_>)->bool{proof.action_identity()==lock.action&&proof.work_binding()==lock.work&&proof.work_action()==lock.work_action}

pub struct OriginalReturnBudget<I:OriginalReturnOrigin>{scope:stock::OriginalReturnScope<I>,pub birth:u64,pub delivery:OriginalReturnDelivery,pub birth_requested:bool,pub publisher_requested:bool,table:Option<Box<OriginalReturnEscrowTable>>,table_pending:Option<Box<OriginalReturnEscrowTable>>,table_action:Option<OriginalReturnBirthLock>,table_commit:Option<OriginalReturnBirthLock>,table_birth:OriginalReturnEscrowBirth,publisher:Option<Box<OriginalEscrowPublisher>>,publisher_pending:Option<Box<OriginalEscrowPublisher>>,publisher_action:Option<OriginalReturnBirthLock>,publisher_commit:Option<OriginalReturnBirthLock>,publisher_birth:OriginalEscrowPublisherBirth,pub publication:Option<OriginalEscrowPublication>,pub publisher_placed:bool,pub publisher_fenced:bool,origin:Option<I>,origin_prior:Option<I>,origin_pending:Option<I>,origin_action:Option<OriginalReturnBirthLock>,origin_commit:Option<OriginalReturnBirthLock>,pub spent:P,pub work:u128,pub held_items:usize,pub held_copy_bytes:usize,pub held_release_bytes:usize,pub held_capacity_bytes:usize,pub held_fuel:u64,pub claims:u8,pub serial:u64,pub failure:Option<(ValueRefusalKind,&'static str)>}
impl<I:OriginalReturnOrigin> Drop for OriginalReturnBudget<I>{fn drop(&mut self){assert!(std::thread::panicking()||self.origin_prior.is_none()&&self.origin_pending.is_none()&&self.origin_action.is_none()&&self.origin_commit.is_none()&&self.table_pending.is_none()&&self.table_action.is_none()&&self.table_commit.is_none()&&self.publisher_pending.is_none()&&self.publisher_action.is_none()&&self.publisher_commit.is_none(),"original return budget retains unposted birth owners")}}
impl<I:OriginalReturnOrigin> OriginalReturnBudget<I>{
 pub const fn empty(birth:u64)->Self{Self{scope:stock::OriginalReturnScope::empty(),birth,delivery:OriginalReturnDelivery::LocalWitness,birth_requested:false,publisher_requested:false,table:None,table_pending:None,table_action:None,table_commit:None,table_birth:OriginalReturnEscrowBirth::new(),publisher:None,publisher_pending:None,publisher_action:None,publisher_commit:None,publisher_birth:OriginalEscrowPublisherBirth::new(),publication:None,publisher_placed:false,publisher_fenced:false,origin:None,origin_prior:None,origin_pending:None,origin_action:None,origin_commit:None,spent:P{copied_items:0,copied_bytes:0,retained_capacity_bytes:0,released_bytes:0},work:0,held_items:0,held_copy_bytes:0,held_release_bytes:0,held_capacity_bytes:0,held_fuel:0,claims:0,serial:0,failure:None}}
 /// 📏️ Selects the genuine next physical table-birth action.
 pub fn table_birth_demand(&self)->semio_framework_value_retained_clone::RetirementDemand{if self.table.is_some()||self.table_pending.is_some(){Default::default()}else{self.table_birth.demand()}}
 /// 📏️ Quotes the private original-action lock before the last table placement.
 pub fn table_birth_action_demand(&self)->semio_framework_value_retained_clone::RetirementDemand{if !self.table_birth.ready_to_publish()||self.table_action.is_some(){return Default::default()}semio_framework_value_retained_clone::RetirementDemand{copy_bytes:std::mem::size_of::<Option<OriginalReturnBirthLock>>(),depth:1,..Default::default()}}
 pub fn bind_table_birth_action(&mut self,origin_proof:&semio_framework_value_native_outcome::OriginalConstructionAcceptedProof<'_>,witness:&semio_framework_value_native_receiving_frame::NativeReceivingConstructionBinding,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<P>,ValueError>{if self.origin_with_post(origin_proof).is_none_or(|origin|origin.source_turn()!=witness.source())||!self.table_birth.ready_to_publish()||self.table_pending.is_some(){return RetainedWorkOutcome{result:Err(refused("original return table is not ready for its Source action lock")),work:0}}bind_birth_action(&mut self.table_action,witness,effect)}
 /// 📨️ Advances the caller-resident birth and deposits its original native result directly.
 pub fn advance_table_birth<'a>(&mut self,origin_proof:&semio_framework_value_native_outcome::OriginalConstructionAcceptedProof<'_>,witness:&semio_framework_value_native_receiving_frame::NativeReceivingConstructionBinding,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'a,RetainedCloneStep,ValueError>)->semio_framework_value_control_contract::OriginalEffectReplyComplete<'a,RetainedCloneStep,ValueError>{
  if self.origin_pending.is_some()||self.origin_action.is_some()||self.origin_with_post(origin_proof).is_none_or(|origin|origin.source_turn()!=witness.source())||!witness.matches_effect(effect){return reply.complete(Err(refused("original return table requires its bound Source action")))}
  if self.table_birth.ready_to_publish()&&self.table_action.as_ref().is_none_or(|lock|lock.action!=witness.action()||lock.work_action!=effect.work_identity().action_identity()||!lock.work.matches_effect(effect)){return reply.complete(Err(refused("original return table requires its bound publication action")))}
  self.table_birth.advance_into(&mut self.table_pending,effect,reply)
 }
 /// ♻️ Retains the same partially initialized backing until its actual Source pays cancellation.
 pub fn cancel_table_birth<'a>(&mut self,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'a,RetainedCloneStep,ValueError>)->semio_framework_value_control_contract::OriginalEffectReplyComplete<'a,RetainedCloneStep,ValueError>{self.table_birth.cancel_into(effect,reply)}
 /// 🛂️ Checks the exact accepted action before a staged Source turn may be published.
 pub fn origin_post_is_accepted(&self,accepted:&semio_framework_value_native_outcome::OriginalConstructionAcceptedProof<'_>)->Result<bool,ValueError>{let Some(lock)=self.origin_action.as_ref()else{return Ok(false)};if self.origin_pending.is_none(){return Ok(false)}Ok(birth_post_accepted(lock,accepted))}
 /// 🛂️ Verifies the actual published acceptance and original finished Work proof for the hidden table.
 pub fn table_post_is_accepted(&self,accepted:&semio_framework_value_native_outcome::OriginalConstructionAcceptedProof<'_>)->Result<bool,ValueError>{let Some(lock)=self.table_action.as_ref()else{return Ok(false)};if self.table_pending.is_none(){return Ok(false)}Ok(birth_post_accepted(lock,accepted))}
 /// 🛂️ Verifies the exact accepted publisher action against its original issuer and Work.
 pub fn publisher_post_is_accepted(&self,accepted:&semio_framework_value_native_outcome::OriginalConstructionAcceptedProof<'_>)->Result<bool,ValueError>{let Some(lock)=self.publisher_action.as_ref()else{return Ok(false)};if self.publisher_pending.is_none(){return Ok(false)}Ok(birth_post_accepted(lock,accepted))}
 /// 📏️ Prices publishing the accepted original Source header and its true initial stock.
 pub fn origin_commit_demand(&self)->semio_framework_value_retained_clone::RetirementDemand{let stock=if self.origin.is_none(){std::mem::size_of::<P>()+std::mem::size_of::<u128>()}else{0};semio_framework_value_retained_clone::RetirementDemand{copy_bytes:3*std::mem::size_of::<Option<I>>()+2*std::mem::size_of::<Option<OriginalReturnBirthLock>>()+stock,depth:1,..Default::default()}}
 fn commit_origin(&mut self,accepted:&semio_framework_value_native_outcome::OriginalConstructionAcceptedProof<'_>,payout:&semio_framework_value_native_receiving_frame::NativeReceivingConstructionBinding,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<P>,ValueError>{
  let fail=|error|RetainedWorkOutcome{result:Err(error),work:0};if !payout.matches_effect(effect)||self.origin_action.as_ref().is_none_or(|lock|lock.work!=payout.work()){return fail(refused("original Source publication changed its Work"))}
  match self.origin_post_is_accepted(accepted){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Ok(None),work:0},Err(error)=>return fail(error)}
  let d=self.origin_commit_demand();if !semio_framework_value_control_contract::retained_clone_grant_admits(effect.grant(),d){return RetainedWorkOutcome{result:Ok(None),work:0}}match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Ok(None),work:0},Err(error)=>return fail(error)}
  let original=self.origin_pending.take().expect("accepted pending original Source");self.origin_prior=self.origin.take();if self.origin_prior.is_none(){self.spent=original.spent();self.work=original.work()}self.origin=Some(original);self.origin_action=None;self.origin_commit=Some(OriginalReturnBirthLock{action:payout.action(),work:payout.work(),work_action:effect.work_identity().action_identity()});RetainedWorkOutcome{result:Ok(Some(P{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}
 }
 /// 👁️ Borrows the new origin only when the exact publication Post is proven accepted.
 pub fn origin_with_post(&self,proof:&semio_framework_value_native_outcome::OriginalConstructionAcceptedProof<'_>)->Option<I>{let lock=self.origin_commit.as_ref()?;birth_post_accepted(lock,proof).then_some(self.origin).flatten()}
 /// 📏️ Prices moving the accepted hidden table into the committed Source slot.
 pub fn table_commit_demand()->semio_framework_value_retained_clone::RetirementDemand{semio_framework_value_retained_clone::RetirementDemand{copy_bytes:2*std::mem::size_of::<Option<Box<OriginalReturnEscrowTable>>>()+2*std::mem::size_of::<Option<OriginalReturnBirthLock>>(),depth:1,..Default::default()}}
 /// 📏️ Prices moving the accepted hidden publisher into the committed Source slot.
 pub fn publisher_commit_demand()->semio_framework_value_retained_clone::RetirementDemand{semio_framework_value_retained_clone::RetirementDemand{copy_bytes:2*std::mem::size_of::<Option<Box<OriginalEscrowPublisher>>>()+2*std::mem::size_of::<Option<OriginalReturnBirthLock>>(),depth:1,..Default::default()}}
 fn commit_table(&mut self,accepted:&semio_framework_value_native_outcome::OriginalConstructionAcceptedProof<'_>,payout:&semio_framework_value_native_receiving_frame::NativeReceivingConstructionBinding,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<P>,ValueError>{
  let fail=|error|RetainedWorkOutcome{result:Err(error),work:0};if self.table.is_some()||!payout.matches_effect(effect)||self.table_action.as_ref().is_none_or(|lock|lock.work!=payout.work()){return fail(refused("original table publication changed its paid destination or Work"))}
  match self.table_post_is_accepted(accepted){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Ok(None),work:0},Err(error)=>return fail(error)}
  let d=Self::table_commit_demand();if !semio_framework_value_control_contract::retained_clone_grant_admits(effect.grant(),d){return RetainedWorkOutcome{result:Ok(None),work:0}}match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Ok(None),work:0},Err(error)=>return fail(error)}
  self.table=self.table_pending.take();self.table_action=None;self.table_commit=Some(OriginalReturnBirthLock{action:payout.action(),work:payout.work(),work_action:effect.work_identity().action_identity()});RetainedWorkOutcome{result:Ok(Some(P{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}
 }
 fn commit_publisher(&mut self,accepted:&semio_framework_value_native_outcome::OriginalConstructionAcceptedProof<'_>,payout:&semio_framework_value_native_receiving_frame::NativeReceivingConstructionBinding,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<P>,ValueError>{
  let fail=|error|RetainedWorkOutcome{result:Err(error),work:0};if self.publisher.is_some()||!payout.matches_effect(effect)||self.publisher_action.as_ref().is_none_or(|lock|lock.work!=payout.work()){return fail(refused("original publisher publication changed its paid destination or Work"))}
  match self.publisher_post_is_accepted(accepted){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Ok(None),work:0},Err(error)=>return fail(error)}
  let d=Self::publisher_commit_demand();if !semio_framework_value_control_contract::retained_clone_grant_admits(effect.grant(),d){return RetainedWorkOutcome{result:Ok(None),work:0}}match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Ok(None),work:0},Err(error)=>return fail(error)}
  self.publisher=self.publisher_pending.take();self.publisher_action=None;self.publisher_commit=Some(OriginalReturnBirthLock{action:payout.action(),work:payout.work(),work_action:effect.work_identity().action_identity()});RetainedWorkOutcome{result:Ok(Some(P{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}
 }
 /// 📏️ Selects the next actual publisher-birth action.
 pub fn publisher_birth_demand(&self)->semio_framework_value_retained_clone::RetirementDemand{if self.publisher.is_some()||self.publisher_pending.is_some(){Default::default()}else{self.publisher_birth.demand()}}
 /// 📏️ Quotes the private original-action lock before the last publisher placement.
 pub fn publisher_birth_action_demand(&self)->semio_framework_value_retained_clone::RetirementDemand{if !self.publisher_birth.ready_to_publish()||self.publisher_action.is_some(){return Default::default()}semio_framework_value_retained_clone::RetirementDemand{copy_bytes:std::mem::size_of::<Option<OriginalReturnBirthLock>>(),depth:1,..Default::default()}}
 pub fn bind_publisher_birth_action(&mut self,origin_proof:&semio_framework_value_native_outcome::OriginalConstructionAcceptedProof<'_>,witness:&semio_framework_value_native_receiving_frame::NativeReceivingConstructionBinding,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<P>,ValueError>{if self.origin_with_post(origin_proof).is_none_or(|origin|origin.source_turn()!=witness.source())||!self.publisher_birth.ready_to_publish()||self.publisher_pending.is_some(){return RetainedWorkOutcome{result:Err(refused("original return publisher is not ready for its Source action lock")),work:0}}bind_birth_action(&mut self.publisher_action,witness,effect)}
 /// 📨️ Deposits the actual paid publisher birth into the original resident slot.
 pub fn advance_publisher_birth<'a>(&mut self,origin_proof:&semio_framework_value_native_outcome::OriginalConstructionAcceptedProof<'_>,table_proof:&semio_framework_value_native_outcome::OriginalConstructionAcceptedProof<'_>,witness:&semio_framework_value_native_receiving_frame::NativeReceivingConstructionBinding,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'a,RetainedCloneStep,ValueError>)->semio_framework_value_control_contract::OriginalEffectReplyComplete<'a,RetainedCloneStep,ValueError>{if self.origin_pending.is_some()||self.origin_action.is_some()||self.origin_with_post(origin_proof).is_none_or(|origin|origin.source_turn()!=witness.source())||self.table_with_post(table_proof).is_none()||!witness.matches_effect(effect){return reply.complete(Err(refused("original publisher requires its bound Source and table action")))}if self.publisher_birth.ready_to_publish()&&self.publisher_action.as_ref().is_none_or(|lock|lock.action!=witness.action()||lock.work_action!=effect.work_identity().action_identity()||!lock.work.matches_effect(effect)){return reply.complete(Err(refused("original return publisher requires its bound publication action")))}self.publisher_birth.advance_into(&mut self.publisher_pending,effect,reply)}
 /// 🪑️ Stages only the original authenticated Source into private pending custody.
 pub fn stage_initial_origin(&mut self,input:I,witness:&semio_framework_value_native_receiving_frame::NativeReceivingConstructionBinding,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<P>,ValueError>{
  let fail=|error|RetainedWorkOutcome{result:Err(error),work:0};
  if self.origin.is_some()||self.origin_pending.is_some()||self.origin_action.is_some()||input.recipient()!=self as*const Self as usize||input.source_turn()!=witness.source()||!witness.matches_effect(effect)||!input.spent().fits(input.granted())||input.work()>u128::from(input.fuel()){return fail(refused("original return origin changed its actual Source or stock"))}
  let demand=Self::refresh_turn_demand();if !semio_framework_value_control_contract::retained_clone_grant_admits(effect.grant(),demand){return RetainedWorkOutcome{result:Ok(None),work:0}}
  match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Ok(None),work:0},Err(error)=>return fail(error)}
  self.origin_pending=Some(input);self.origin_action=Some(OriginalReturnBirthLock{action:witness.action(),work:witness.work(),work_action:effect.work_identity().action_identity()});RetainedWorkOutcome{result:Ok(Some(P{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}
 }
 /// 📏️ Quotes only the actual retained Source-turn header replacement.
 pub fn refresh_turn_demand()->semio_framework_value_retained_clone::RetirementDemand{semio_framework_value_retained_clone::RetirementDemand{copy_bytes:std::mem::size_of::<Option<I>>()+std::mem::size_of::<Option<OriginalReturnBirthLock>>(),depth:1,..Default::default()}}
 /// 🔁️ Stages the same authentic origin and its exact pending paid turn action.
 pub fn refresh_turn(&mut self,input:I,witness:&semio_framework_value_native_receiving_frame::NativeReceivingConstructionBinding,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<P>,ValueError>{
  let fail=|error|RetainedWorkOutcome{result:Err(error),work:0};let Some(origin)=self.origin else{return fail(refused("original return Source is absent"))};
  if !origin.same_origin(&input)||input.source_turn()!=witness.source()||!witness.matches_effect(effect)||self.origin_pending.is_some()||self.origin_action.is_some()||!self.table_birth.is_empty()||self.table_pending.is_some()||self.table_action.is_some()||!self.publisher_birth.is_empty()||self.publisher_pending.is_some()||self.publisher_action.is_some(){return fail(refused("original return Source cannot refresh with different origin or active birth"))}
  if origin.source_turn()==input.source_turn(){return RetainedWorkOutcome{result:Ok(None),work:0}}
  let demand=Self::refresh_turn_demand();if !semio_framework_value_control_contract::retained_clone_grant_admits(effect.grant(),demand){return RetainedWorkOutcome{result:Ok(None),work:0}}
  match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Ok(None),work:0},Err(error)=>return fail(error)}
  self.origin_pending=Some(input);self.origin_action=Some(OriginalReturnBirthLock{action:witness.action(),work:witness.work(),work_action:effect.work_identity().action_identity()});RetainedWorkOutcome{result:Ok(Some(P{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}
 }
 /// 👁️ Observes only the committed original Source.
 pub fn origin(&self)->Option<I>{if self.origin_commit.is_some(){self.origin_prior}else{self.origin}}
 /// 👁️ Borrows the committed table, never its pending birth owner.
 pub fn table(&self)->Option<&OriginalReturnEscrowTable>{if self.table_commit.is_some(){None}else{self.table.as_deref()}}
 pub fn table_mut(&mut self)->Option<&mut OriginalReturnEscrowTable>{if self.table_commit.is_some(){None}else{self.table.as_deref_mut()}}
 /// 👁️ Borrows the table only with its exact accepted publication proof.
 pub fn table_with_post(&self,proof:&semio_framework_value_native_outcome::OriginalConstructionAcceptedProof<'_>)->Option<&OriginalReturnEscrowTable>{self.table_commit.as_ref().filter(|lock|birth_post_accepted(lock,proof)).and_then(|_|self.table.as_deref())}
 /// 👁️ Borrows the committed publisher, never its pending birth owner.
 pub fn publisher(&self)->Option<&OriginalEscrowPublisher>{if self.publisher_commit.is_some(){None}else{self.publisher.as_deref()}}
 pub fn publisher_mut(&mut self)->Option<&mut OriginalEscrowPublisher>{if self.publisher_commit.is_some(){None}else{self.publisher.as_deref_mut()}}
 /// 👁️ Borrows the publisher only with its exact accepted publication proof.
 pub fn publisher_with_post(&self,proof:&semio_framework_value_native_outcome::OriginalConstructionAcceptedProof<'_>)->Option<&OriginalEscrowPublisher>{self.publisher_commit.as_ref().filter(|lock|birth_post_accepted(lock,proof)).and_then(|_|self.publisher.as_deref())}
 pub fn is_bound(&self)->bool{self.origin().is_some()}
 #[cfg(test)]
 fn bind(&mut self,input:I)->Result<bool,ValueError>{
  if input.recipient()!=self as*const Self as usize{return Err(refused("original return lifetime input requires its same preborn receiving budget"))}
  if let Some(origin)=self.origin{if !origin.same_origin(&input){return Err(refused("original return source cannot be rebound"))}if origin.source_turn()!=input.source_turn(){return Err(refused("original return Source turn requires its paid refresh"))}return Ok(false)}
  if !input.spent().fits(input.granted())||input.work()>u128::from(input.fuel()){return Err(refused("original return source exceeds its admitting lifetime stock"))}
  self.origin=Some(input);self.spent=input.spent();self.work=input.work();Ok(true)
 }
 /// 🔁️ See `README.md`.
 pub fn renew_turn(&mut self)->Result<(),ValueError>{
  let origin=self.origin.ok_or_else(||refused(E::STOCK_ABSENT))?;
  if self.claims!=0||self.held_items!=0||self.held_copy_bytes!=0||self.held_release_bytes!=0||self.held_capacity_bytes!=0||self.held_fuel!=0||self.failure.is_some(){return Err(refused("original return source with live claims cannot renew its turn"))}
  self.spent=origin.spent();self.work=origin.work();Ok(())
 }
 pub fn snapshot(&self)->OriginalReturnBudgetSnapshot{let live=if self.origin_commit.is_some()&&self.origin_prior.is_none(){self.origin}else{self.origin()};let(origin,fuel)=live.map_or((RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0},0),|origin|(origin.granted(),origin.fuel()));OriginalReturnBudgetSnapshot{granted:origin,fuel,spent:self.spent,work:self.work,held_items:usize::from(self.held_items),held_copy_bytes:self.held_copy_bytes,held_release_bytes:self.held_release_bytes,held_capacity_bytes:self.held_capacity_bytes,held_fuel:u64::from(self.held_fuel),claims:usize::from(self.claims),serial:self.serial}}
 pub fn refusal(&self)->Option<ValueError>{self.failure.as_ref().map(|(kind,message)|ValueError::literal(kind.clone(),message))}
 pub fn raw_remaining(&self)->Result<(RetainedCloneGrant,u64),ValueError>{if let Some(error)=self.refusal(){return Err(error)}self.origin.ok_or_else(||refused(E::STOCK_ABSENT))?;self.snapshot().raw_remaining()}
 pub fn remaining(&self)->Result<(RetainedCloneGrant,u64),ValueError>{if let Some(error)=self.refusal(){return Err(error)}self.origin.ok_or_else(||refused(E::STOCK_ABSENT))?;self.snapshot().remaining()}
 pub fn record_retained(&mut self,progress:P)->Result<(),ValueError>{if self.origin.is_none(){return Ok(())}let mut state=self.snapshot();let issue=state.advance_retained(progress);self.spent=state.spent;if let Some(issue)=issue{self.failure=Some(issue.failure());return Err(self.refusal().unwrap().with_retained_progress(progress))}Ok(())}
 pub fn record_work(&mut self,work:u64){let _=self.record_work_exact(u128::from(work));}
 pub fn record_work_exact(&mut self,work:u128)->Result<(),ValueError>{if self.origin.is_none(){return Ok(())}let mut state=self.snapshot();let issue=state.advance_work_exact(work);self.work=state.work;if let Some(issue)=issue{let(kind,message)=issue.failure();if self.failure.is_none(){self.failure=Some((kind.clone(),message))}return Err(ValueError::literal(kind,message))}Ok(())}
}

#[path="🌱️birth/🦀️.rs"]
mod birth;
pub use birth::{ORIGINAL_RETURN_BIRTH_ACTION_WORK,OriginalReturnBirthStage,OriginalReturnBirthPhase,OriginalReturnBirthContext,OriginalReturnBirthProducer};

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OriginalReturnPhase{Capture,Receive}
pub struct OriginalReturnClaimState{pub source:usize,pub serial:u64,pub slot:usize,pub local:bool,pub capture:bool,pub receive:bool,pub capture_observed:bool,pub receive_observed:bool,pub live:bool}
pub struct OriginalReturnWitnessState{pub source:usize,pub serial:u64,pub phase:OriginalReturnPhase,pub progress:P,pub work:u64,pub release_capture:bool,pub redeemed:bool,pub received:bool}
impl OriginalReturnClaimState{
 pub fn terminal_is_empty(&self)->bool{!self.live&&!self.capture&&!self.receive&&self.capture_observed&&self.receive_observed}
 pub fn has_unused_capture(&self)->bool{self.live&&self.capture&&self.receive}
 pub fn issue_return(&mut self,phase:OriginalReturnPhase,copy_bytes:usize)->Result<OriginalReturnWitnessState,ValueError>{if !self.live||!match phase{OriginalReturnPhase::Capture=>self.capture,OriginalReturnPhase::Receive=>self.receive&&!self.capture}{return Err(refused("original return claim phase was already issued or requires capture"))}let witness=self.witness(phase,false,copy_bytes)?;match phase{OriginalReturnPhase::Capture=>self.capture=false,OriginalReturnPhase::Receive=>self.receive=false}Ok(witness)}
 pub fn cancel_preissued(&mut self,w:&mut OriginalReturnWitnessState)->Result<(),ValueError>{if !self.live||self.capture||!self.receive||self.capture_observed||self.receive_observed||w.source!=self.source||w.serial!=self.serial||w.phase!=OriginalReturnPhase::Capture||w.redeemed||w.received{return Err(refused("original preissued capture cannot be cancelled"))}let copy=M::witness_copy(false,self.local,0)?;w.phase=OriginalReturnPhase::Receive;w.progress.copied_bytes=copy;w.release_capture=true;self.receive=false;Ok(())}
 pub fn issue_success_return(&mut self,copy_bytes:usize)->Result<OriginalReturnWitnessState,ValueError>{if !self.has_unused_capture(){return Err(refused("normal original return requires its unused capture and receiving holds"))}let witness=self.witness(OriginalReturnPhase::Receive,true,copy_bytes)?;self.capture=false;self.receive=false;Ok(witness)}
 fn witness(&self,phase:OriginalReturnPhase,release_capture:bool,copy_bytes:usize)->Result<OriginalReturnWitnessState,ValueError>{let copied=M::witness_copy(phase==OriginalReturnPhase::Capture,self.local,copy_bytes)?;Ok(OriginalReturnWitnessState{source:self.source,serial:self.serial,phase,progress:P{copied_items:1,copied_bytes:copied,..Default::default()},work:1,release_capture,redeemed:false,received:false})}

}

impl<I:OriginalReturnOrigin> OriginalReturnBudget<I>{
 pub fn claim(&mut self,copy_bytes:usize,destination:usize,epoch:u64)->Result<Option<OriginalReturnClaimCredit<I>>,ValueError>{
  let origin=self.origin.ok_or_else(||refused("original return claim requires its admitted source"))?;if origin.recipient_identity()!=self as*const Self as usize{return Err(refused("original return claim requires its actual bound budget address"))}if let Some(error)=self.refusal(){return Err(error)}let local=self.delivery==OriginalReturnDelivery::LocalWitness;if destination==0||!local&&epoch==0{return Err(refused("original return claim lacks its genuine currencies, epoch or destination"))}let overhead=2*M::UPDATE+M::FINAL+if local{M::LOCAL_ACK}else{0};let plan=self.snapshot().plan_claim(copy_bytes,original_return_claim_copy_bytes::<I>(),overhead)?;
  let identity=OriginalReturnEscrowIdentity{source:origin.source_identity() as u64,source_birth:self.birth,recipient:origin.recipient_identity() as u64,destination:destination as u64,extent:copy_bytes as u64,serial:plan.serial,origin_epoch:epoch};let held=OriginalReturnEscrowAmounts{items:2,copy_bytes:plan.reserved_copy as u64,work:2,release_bytes:0,capacity_bytes:0};let Some(slot)=self.table.as_mut().ok_or_else(||refused("original return table has not been admitted"))?.reserve(identity,held)?else{return Ok(None)};
  self.held_copy_bytes=plan.held_copy_bytes;self.serial=plan.serial;self.claims=plan.claims;self.held_items=plan.held_items;self.held_fuel=plan.held_fuel;Ok(Some(OriginalReturnClaimCredit::<I>{_origin:std::marker::PhantomData,data:OriginalReturnClaimState{source:origin.source_identity(),serial:plan.serial,slot,local,capture:true,receive:true,capture_observed:false,receive_observed:false,live:true}}))
 }
 pub fn redeem(&mut self,credit:&mut OriginalReturnClaimCredit<I>,witness:&mut OriginalReturnWitness<I>)->Result<(),ValueError>{
  let origin=self.origin.ok_or_else(||refused("original return source already closed"))?;if origin.recipient_identity()!=self as*const Self as usize{return Err(refused("original return redemption requires its actual bound budget address"))}if !credit.live&&!witness.redeemed||credit.source!=origin.source_identity()||credit.serial!=witness.serial||witness.source!=origin.source_identity(){return Err(refused("original return witness is foreign to its held claim"))}if witness.redeemed{return Ok(())}
  let observed=match witness.phase{OriginalReturnPhase::Capture=>credit.capture_observed,OriginalReturnPhase::Receive=>credit.receive_observed};if observed||match witness.phase{OriginalReturnPhase::Capture=>credit.capture,OriginalReturnPhase::Receive=>credit.receive}{return Err(refused("original return witness was not issued once by its claim"))}
  let identity=self.table.as_ref().and_then(|table|table.row_at(credit.slot,credit.serial)).ok_or_else(||refused("original return claim row disappeared"))?.identity;if identity.source!=origin.source_identity() as u64||identity.recipient!=origin.recipient_identity() as u64{return Err(refused("original return claim row changed Source"))}let extent=usize::try_from(identity.extent).map_err(|_|refused("original return extent exceeds native size"))?;if witness.release_capture&&(witness.phase!=OriginalReturnPhase::Receive||credit.capture_observed){return Err(refused("original return witness lost its genuine held currencies"))}let plan=self.snapshot().plan_redemption(extent,witness.phase==OriginalReturnPhase::Capture,witness.release_capture,credit.local,witness.progress,witness.work,(M::UPDATE,M::FINAL,M::LOCAL_ACK))?;let held=plan.held;let held_copy=plan.held_copy;let spent=plan.spent;let work=plan.work;let actual=plan.actual;let unused=plan.unused;self.table.as_mut().ok_or_else(||refused("original return table disappeared"))?.redeem(identity,actual,unused)?;self.held_items-=usize::from(held);self.held_copy_bytes-=held_copy;self.held_fuel-=u64::from(held);self.spent=spent;self.work=work;match witness.phase{OriginalReturnPhase::Capture=>credit.data.capture_observed=true,OriginalReturnPhase::Receive=>credit.data.receive_observed=true}if witness.release_capture{credit.data.capture_observed=true;}if credit.capture_observed&&credit.receive_observed{self.claims-=1;credit.data.live=false;}witness.data.redeemed=true;Ok(())
 }

}

/// 🎟️ Nominal origin typing keeps claimed authority distinct from freely constructible phase data.
#[repr(transparent)]
pub struct OriginalReturnClaimCredit<I:OriginalReturnOrigin>{data:OriginalReturnClaimState,_origin:std::marker::PhantomData<fn()->I>}
impl<I:OriginalReturnOrigin> std::ops::Deref for OriginalReturnClaimCredit<I>{type Target=OriginalReturnClaimState;fn deref(&self)->&Self::Target{&self.data}}
impl<I:OriginalReturnOrigin> OriginalReturnClaimCredit<I>{
 pub fn terminal_is_empty(&self)->bool{self.data.terminal_is_empty()}
 pub fn has_unused_capture(&self)->bool{self.data.has_unused_capture()}
 pub fn issue_return(&mut self,phase:OriginalReturnPhase,copy_bytes:usize)->Result<OriginalReturnWitness<I>,ValueError>{self.data.issue_return(phase,copy_bytes).map(|data|OriginalReturnWitness{data,_origin:std::marker::PhantomData})}
 pub fn cancel_preissued(&mut self,w:&mut OriginalReturnWitness<I>)->Result<(),ValueError>{self.data.cancel_preissued(&mut w.data)}
 pub fn issue_success_return(&mut self,copy_bytes:usize)->Result<OriginalReturnWitness<I>,ValueError>{self.data.issue_success_return(copy_bytes).map(|data|OriginalReturnWitness{data,_origin:std::marker::PhantomData})}
}
impl<I:OriginalReturnOrigin> Drop for OriginalReturnClaimCredit<I>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original return claim retains unspent original currencies");}}
/// 🧾️ Only a genuine typed claim can mint this held physical-effect witness.
#[repr(transparent)]
pub struct OriginalReturnWitness<I:OriginalReturnOrigin>{data:OriginalReturnWitnessState,_origin:std::marker::PhantomData<fn()->I>}
impl<I:OriginalReturnOrigin> std::ops::Deref for OriginalReturnWitness<I>{type Target=OriginalReturnWitnessState;fn deref(&self)->&Self::Target{&self.data}}
impl<I:OriginalReturnOrigin> OriginalReturnWitness<I>{
 pub fn is_capture(&self)->bool{self.phase==OriginalReturnPhase::Capture}
 pub fn releases_unused_capture(&self)->bool{self.release_capture}
 pub fn mark_received(&mut self)->Result<(),ValueError>{if !self.redeemed||self.received{return Err(refused("original held return witness has not been redeemed exactly once"))}self.data.received=true;Ok(())}
}
impl<I:OriginalReturnOrigin> Drop for OriginalReturnWitness<I>{fn drop(&mut self){assert!(std::thread::panicking()||self.received,"original held return witness requires its actual receiving caller");}}
pub fn original_return_claim_copy_bytes<I:OriginalReturnOrigin>()->usize{std::mem::size_of::<OriginalReturnEscrowRow>()+std::mem::size_of::<Option<OriginalReturnClaimCredit<I>>>()+3*std::mem::size_of::<u64>()+2*std::mem::size_of::<u8>()+2*std::mem::size_of::<usize>()}

impl<I:OriginalReturnOrigin> OriginalReturnBudget<I>{
 /// 🔎️ Qualifies the original typed preissued capture against its actual receiving budget and destination.
 pub fn qualify_preissued_destination(&self,credit:&OriginalReturnClaimCredit<I>,w:&OriginalReturnWitness<I>,destination:usize,extent:usize)->Result<(),ValueError>{let b=self;let origin=b.origin.ok_or_else(||refused("header exposure changed its actual Source, cell or permit"))?;let row=b.table.as_ref().and_then(|t|t.row_at(credit.slot,credit.serial)).ok_or_else(||refused("header exposure changed its actual Source, cell or permit"))?;if origin.recipient_identity()!=b as*const Self as usize||!credit.live||credit.source!=origin.source_identity()||w.source!=credit.source||w.serial!=credit.serial||w.phase!=OriginalReturnPhase::Capture||w.redeemed||w.received||credit.capture||!credit.receive||credit.capture_observed||credit.receive_observed||row.tombstone||row.identity.source!=origin.source_identity() as u64||row.identity.recipient!=origin.recipient_identity() as u64||row.identity.destination!=destination as u64||row.identity.extent!=extent as u64{return Err(refused("header exposure changed its actual Source, cell or permit"))}Ok(())}
}

/// 📏️ A sized return quote is data; only the Context can qualify its actual Source destination.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct OriginalSizedClaimReserve{pub items:usize,pub immediate_copy_bytes:usize,pub lifetime_copy_bytes:usize,pub work:u64,pub depth:usize}
impl OriginalSizedClaimReserve{
 /// 📏️ The portable CoreWire ceiling for one sized claim's immediate metadata copy: above the native 64-bit actual (565 B) and, by layout, the wasm32 actual.
 pub const CEILING:usize=640;
 /// 🧮️ The exact target-native metadata of one claim placement and both held updates.
 pub fn actual_bytes(claim:usize,identity:usize,destination:usize)->Option<usize>{claim.checked_add(identity)?.checked_add(destination)?.checked_add(M::UPDATE.checked_mul(2)?)?.checked_add(M::FINAL)}
 pub fn for_target(extent:usize,claim:usize,identity:usize,destination:usize)->Result<Self,ValueError>{let actual=Self::actual_bytes(claim,identity,destination).ok_or_else(||refused("sized claim reserve overflow"))?;if actual>Self::CEILING{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"sized claim target exceeds portable metadata ceiling"))}Ok(Self{items:3,immediate_copy_bytes:Self::CEILING,lifetime_copy_bytes:extent.checked_add(Self::CEILING).ok_or_else(||refused("sized claim reserve overflow"))?,work:3,depth:1})}
}
/// 🧮️ Reserved effects never become a second fresh charge.
pub fn source_fresh_progress(total:P,reserved:P)->Option<P>{Some(P{copied_items:total.copied_items.checked_sub(reserved.copied_items)?,copied_bytes:total.copied_bytes.checked_sub(reserved.copied_bytes)?,retained_capacity_bytes:total.retained_capacity_bytes.checked_sub(reserved.retained_capacity_bytes)?,released_bytes:total.released_bytes.checked_sub(reserved.released_bytes)?})}

#[path="🏠️native/🦀️.rs"]pub mod native_storage;

#[path="📦️stock/🦀️.rs"]mod stock;
pub use stock::{CreditedOutput,OriginalReturnStockClaim,OriginalReturnStockWitness,original_return_stock_claim_copy_bytes,original_return_stock_settlement_copy_bytes,original_return_stock_top_up_copy_bytes,OriginalReturnScopeLoan,original_return_scope_admission_copy_bytes,original_return_scope_completion_copy_bytes,original_return_scope_setup_demand,OriginalReturnScopeCounters,OriginalReturnScopePlan,original_return_scope_quantum,original_return_scope_transfer_copy_bytes};
