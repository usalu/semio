//! 🌱️ Table and publisher births run as paid Source actions fenced by exact accepted proofs.
use super::*;
use semio_framework_value_control_contract::{OriginalEffectProducer,OriginalEffectReply,OriginalEffectReplyComplete,OriginalEffectAuthority,RetainedAllocationEffect};
use semio_framework_value_native_outcome::OriginalConstructionAcceptedProof as Accepted;
use semio_framework_value_native_receiving_frame::NativeReceivingConstructionBinding as Binding;
use semio_framework_value_retained_clone::{retained_clone_grant_admits,RetirementDemand};

/// ⛽️ One birth action fills a whole backing: the lock, 32 cells, header and output placement plus slack.
pub const ORIGINAL_RETURN_BIRTH_ACTION_WORK:u64=40;
/// 🧭️ Orders the Source actions that fill and publish the table and the publisher.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OriginalReturnBirthStage{Table,TableCommit,Publisher,PublisherCommit,Release}
impl OriginalReturnBirthStage{/// 🔢️ Counts the stages of one birth chain, so holders of per-stage state size themselves from the chain.
 pub const COUNT:usize=Self::Release as usize+1;}
impl OriginalReturnBirthStage{
 /// ⛽️ Quotes the Work units one action of this stage performs.
 pub const fn work_demand(self)->u64{match self{Self::Table|Self::Publisher=>ORIGINAL_RETURN_BIRTH_ACTION_WORK,_=>1}}
 /// 🧮️ Quotes the items one action of this stage posts.
 pub const fn item_demand(self)->usize{self.minimum_work() as usize}
 /// 🪫️ Quotes the Work units below which a birth action cannot progress at all.
 pub const fn minimum_work(self)->u64{match self{Self::Table|Self::Publisher=>2,_=>1}}
}
/// 🌗️ Reports how far one hidden Source owner progressed through its two-phase birth.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OriginalReturnBirthPhase{Absent,Building,Staged,Fenced,Live}
fn sum(a:RetirementDemand,b:RetirementDemand)->Result<RetirementDemand,ValueError>{let over=||refused("original return birth quote overflow");Ok(RetirementDemand{copy_bytes:a.copy_bytes.checked_add(b.copy_bytes).ok_or_else(over)?,capacity_bytes:a.capacity_bytes.checked_add(b.capacity_bytes).ok_or_else(over)?,release_bytes:a.release_bytes.checked_add(b.release_bytes).ok_or_else(over)?,depth:a.depth.max(b.depth)})}
fn step(outcome:RetainedWorkOutcome<Option<P>,ValueError>)->Result<RetainedCloneStep,ValueError>{match outcome.result?{Some(progress)=>Ok(RetainedCloneStep::Complete(progress)),None=>Ok(RetainedCloneStep::Progress(Default::default()))}}

impl<I:OriginalReturnOrigin> OriginalReturnBudget<I>{
 /// 🌗️ Observes the hidden origin.
 pub fn origin_phase(&self)->OriginalReturnBirthPhase{if self.origin_pending.is_some()||self.origin_action.is_some(){OriginalReturnBirthPhase::Staged}else if self.origin_commit.is_some(){OriginalReturnBirthPhase::Fenced}else if self.origin.is_some(){OriginalReturnBirthPhase::Live}else{OriginalReturnBirthPhase::Absent}}
 /// 🌗️ Observes the hidden table.
 pub fn table_phase(&self)->OriginalReturnBirthPhase{if self.table_pending.is_some(){OriginalReturnBirthPhase::Staged}else if self.table_commit.is_some(){OriginalReturnBirthPhase::Fenced}else if self.table.is_some(){OriginalReturnBirthPhase::Live}else if !self.table_birth.is_empty()||self.table_action.is_some(){OriginalReturnBirthPhase::Building}else{OriginalReturnBirthPhase::Absent}}
 /// 🌗️ Observes the hidden publisher.
 pub fn publisher_phase(&self)->OriginalReturnBirthPhase{if self.publisher_pending.is_some(){OriginalReturnBirthPhase::Staged}else if self.publisher_commit.is_some(){OriginalReturnBirthPhase::Fenced}else if self.publisher.is_some(){OriginalReturnBirthPhase::Live}else if !self.publisher_birth.is_empty()||self.publisher_action.is_some(){OriginalReturnBirthPhase::Building}else{OriginalReturnBirthPhase::Absent}}
 /// 🔀️ Lends the committed table and publisher together for one exclusive extent step.
 pub fn escrow_mut(&mut self)->Option<(&mut OriginalReturnEscrowTable,&mut OriginalEscrowPublisher)>{if self.table_commit.is_some()||self.publisher_commit.is_some(){return None}Some((self.table.as_deref_mut()?,self.publisher.as_deref_mut()?))}
 /// 📏️ Prices dropping the three accepted commit fences.
 pub fn release_demand()->RetirementDemand{RetirementDemand{copy_bytes:3*size_of::<Option<OriginalReturnBirthLock>>()+size_of::<Option<I>>(),depth:1,..Default::default()}}
 /// 🔓️ Publishes the committed owners once their commit actions are accepted.
 pub fn release_births(&mut self,origin:&Accepted<'_>,table:Option<&Accepted<'_>>,publisher:Option<&Accepted<'_>>,binding:&Binding,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<P>,ValueError>{
  let fail=|error|RetainedWorkOutcome{result:Err(error),work:0};
  if !binding.matches_effect(effect)||self.origin_with_post(origin).is_none()||self.table_commit.is_some()!=table.is_some()||self.publisher_commit.is_some()!=publisher.is_some()||table.is_some_and(|proof|self.table_with_post(proof).is_none())||publisher.is_some_and(|proof|self.publisher_with_post(proof).is_none()){return fail(refused("original return release changed its accepted commits"))}
  let d=Self::release_demand();if !retained_clone_grant_admits(effect.grant(),d){return RetainedWorkOutcome{result:Ok(None),work:0}}match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Ok(None),work:0},Err(error)=>return fail(error)}
  self.publisher_placed|=self.publisher_commit.is_some();self.origin_commit=None;self.origin_prior=None;self.table_commit=None;self.publisher_commit=None;RetainedWorkOutcome{result:Ok(Some(P{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}
 }
 /// ♻️ Retains the same partially initialized publisher backing until its actual Source pays cancellation.
 pub fn cancel_publisher_birth<'a>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'a,RetainedCloneStep,ValueError>)->OriginalEffectReplyComplete<'a,RetainedCloneStep,ValueError>{self.publisher_birth.cancel_into(effect,reply)}
 /// ⛽️ Quotes the exact Work one whole stage action performs, including its action-lock bind.
 pub fn birth_work(&self,stage:OriginalReturnBirthStage)->u64{match stage{OriginalReturnBirthStage::Table=>1+self.table_birth.remaining_work(),OriginalReturnBirthStage::Publisher=>1+self.publisher_birth.remaining_work(),_=>1}}
 /// 📏️ Quotes the next action of the selected birth stage.
 pub fn birth_demand(&self,stage:OriginalReturnBirthStage)->Result<RetirementDemand,ValueError>{match stage{
  OriginalReturnBirthStage::Table=>sum(self.table_birth_demand(),self.table_birth_action_demand()),
  OriginalReturnBirthStage::TableCommit=>Ok(Self::table_commit_demand()),
  OriginalReturnBirthStage::Publisher=>sum(self.publisher_birth_demand(),self.publisher_birth_action_demand()),
  OriginalReturnBirthStage::PublisherCommit=>Ok(Self::publisher_commit_demand()),
  OriginalReturnBirthStage::Release=>Ok(Self::release_demand())}}
 /// 🧮️ Quotes the items and extents one whole stage charges: every remaining paid fill step of a table or publisher plus its action-lock bind.
 pub fn birth_total(&self,stage:OriginalReturnBirthStage)->Result<(usize,RetirementDemand),ValueError>{
  let lock=std::mem::size_of::<Option<OriginalReturnBirthLock>>();
  match stage{
   OriginalReturnBirthStage::Table=>{if self.table.is_some()||self.table_pending.is_some(){return Ok((0,Default::default()))}let(mut items,mut demand)=self.table_birth.remaining_demand();if self.table_action.is_none(){items+=1;demand.copy_bytes+=lock}Ok((items,demand))}
   OriginalReturnBirthStage::Publisher=>{if self.publisher.is_some()||self.publisher_pending.is_some(){return Ok((0,Default::default()))}let(mut items,mut demand)=self.publisher_birth.remaining_demand();if self.publisher_action.is_none(){items+=1;demand.copy_bytes+=lock}Ok((items,demand))}
   _=>Ok((1,self.birth_demand(stage)?))
  }
 }
 /// 🌱️ Runs one birth stage under the exact accepted proofs and its paid construction action.
 pub fn birth_action(&mut self,stage:OriginalReturnBirthStage,origin:&Accepted<'_>,table:Option<&Accepted<'_>>,birth:Option<&Accepted<'_>>,publisher:Option<&Accepted<'_>>,binding:&Binding,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
  let budget=self;let missing=||refused("original return birth lacks its accepted proof");
  match stage{
   OriginalReturnBirthStage::Table=>{
    let mut lock=P::default();
    let mut total=lock;loop{if budget.table_birth.ready_to_publish()&&budget.table_action.as_ref().is_some_and(|held|held.action!=binding.action()){budget.table_action=None}if budget.table_birth.ready_to_publish()&&budget.table_action.is_none(){match budget.bind_table_birth_action(origin,binding,effect).result?{Some(progress)=>total=total.checked_add(progress)?,None=>break}}let d=budget.birth_demand(stage)?;let next=total.checked_add(P{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})?;if !next.fits(effect.grant())||effect.remaining_work_units()?<1{break}
     let mut slot=None;drop(budget.advance_table_birth(origin,binding,effect,OriginalEffectReply::borrow(&mut slot)?));match slot.take().ok_or_else(||refused("original return table birth recorded no reply"))?? {RetainedCloneStep::Complete(p)=>return Ok(RetainedCloneStep::Complete(total.checked_add(p)?)),other=>{let p=other.progress();if p==P::default(){break}total=total.checked_add(p)?}}}
    Ok(RetainedCloneStep::Progress(total))
   },
   OriginalReturnBirthStage::TableCommit=>step(budget.commit_table(birth.ok_or_else(missing)?,binding,effect)),
   OriginalReturnBirthStage::Publisher=>{
    let table=table.ok_or_else(missing)?;let mut lock=P::default();
    let mut total=lock;loop{if budget.publisher_birth.ready_to_publish()&&budget.publisher_action.as_ref().is_some_and(|held|held.action!=binding.action()){budget.publisher_action=None}if budget.publisher_birth.ready_to_publish()&&budget.publisher_action.is_none(){match budget.bind_publisher_birth_action(origin,binding,effect).result?{Some(progress)=>total=total.checked_add(progress)?,None=>break}}let d=budget.birth_demand(stage)?;let next=total.checked_add(P{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})?;if !next.fits(effect.grant())||effect.remaining_work_units()?<1{break}
     let mut slot=None;drop(budget.advance_publisher_birth(origin,table,binding,effect,OriginalEffectReply::borrow(&mut slot)?));match slot.take().ok_or_else(||refused("original return publisher birth recorded no reply"))?? {RetainedCloneStep::Complete(p)=>return Ok(RetainedCloneStep::Complete(total.checked_add(p)?)),other=>{let p=other.progress();if p==P::default(){break}total=total.checked_add(p)?}}}
    Ok(RetainedCloneStep::Progress(total))
   },
   OriginalReturnBirthStage::PublisherCommit=>step(budget.commit_publisher(birth.ok_or_else(missing)?,binding,effect)),
   OriginalReturnBirthStage::Release=>step(budget.release_births(origin,table,publisher,binding,effect))
  }
 }
 /// 🌱️ Stages or refreshes the hidden origin under one paid Source action.
 pub fn origin_action(&mut self,input:I,binding:&Binding,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<P>,ValueError>{if self.origin.is_some(){self.refresh_turn(input,binding,effect)}else{self.stage_initial_origin(input,binding,effect)}}
 /// 🔒️ Commits the accepted hidden origin under one paid Source action.
 pub fn origin_commit_action(&mut self,accepted:&Accepted<'_>,binding:&Binding,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<P>,ValueError>{self.commit_origin(accepted,binding,effect)}
}

/// 🧵️ Lends the exact accepted proofs and the original budget guard to one paid birth action.
pub struct OriginalReturnBirthContext<'turn,'source,I:OriginalReturnOrigin>{pub stage:OriginalReturnBirthStage,pub binding:&'turn Binding,pub origin:&'turn Accepted<'turn>,pub table:Option<&'turn Accepted<'turn>>,pub birth:Option<&'turn Accepted<'turn>>,pub publisher:Option<&'turn Accepted<'turn>>,pub budget:&'turn mut OriginalReturnBudgetReference<'source,I>}
/// 🪑️ Retains the Source authority that runs every birth stage in its own paid action.
pub struct OriginalReturnBirthProducer<'a,I:OriginalReturnOrigin+'static>{source:&'a OriginalReturnSource<I>}
impl<'a,I:OriginalReturnOrigin+'static> OriginalReturnBirthProducer<'a,I>{
 /// 🔐️ Checks the existing Source guard without acquiring another lock.
 pub fn authenticates(&self,budget:&OriginalReturnBudgetReference<'_,I>)->bool{self.source.owns_reference(budget)}
 /// 📏️ Quotes the resident authority slot before moving the producer.
 pub fn admission_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<OriginalEffectAuthority<Self>>>(),depth:1,..Default::default()}}
 /// 🏠️ Retains the same actual Source before its first birth action.
 pub fn admit(source:&'a OriginalReturnSource<I>,budget:&OriginalReturnBudgetReference<'_,I>,output:&mut Option<OriginalEffectAuthority<Self>>,binding:&Binding,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<P>,ValueError>{
  let fail=|error|RetainedWorkOutcome{result:Err(error),work:0};if output.is_some()||!source.owns_reference(budget)||!binding.matches_effect(effect){return fail(refused("original return birth producer changed its destination or action"))}
  let d=Self::admission_demand();if !retained_clone_grant_admits(effect.grant(),d){return RetainedWorkOutcome{result:Ok(None),work:0}}match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Ok(None),work:0},Err(error)=>return fail(error)}
  *output=Some(OriginalEffectAuthority::new(Self{source}));RetainedWorkOutcome{result:Ok(Some(P{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}
 }
}
impl<I:OriginalReturnOrigin+'static> OriginalEffectProducer for OriginalReturnBirthProducer<'_,I>{
 type Output=RetainedCloneStep;type Failure=ValueError;
 type Context<'turn,'source>=OriginalReturnBirthContext<'turn,'source,I> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Result<(),ValueError>where 'source:'turn{if !self.source.owns_reference(&*c.budget)||c.binding.source().is_none()||c.origin.work_binding().source_turn()!=c.binding.source(){return Err(refused("original return birth changed its held Source"))}Ok(())}
 fn source_turn<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Option<semio_framework_value_control_contract::OriginalSourceTurnIdentity>where 'source:'turn{c.binding.source()}
 fn receive<'reply,'turn,'source>(&mut self,c:&mut Self::Context<'turn,'source>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>where 'source:'turn{
  self.validate_context(c)?;if !c.binding.matches_effect(effect){return Err(refused("original return birth changed its action"))}
  let stage=c.stage;let demand=c.budget.get().birth_demand(stage)?;
  if effect.grant().maximum_items<stage.item_demand()||!retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<stage.minimum_work(){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  let result=c.budget.get_mut().birth_action(stage,c.origin,c.table,c.birth,c.publisher,c.binding,effect);
  Ok(reply.complete(result))
 }
 fn recover<'reply,'turn,'source>(&mut self,c:&mut Self::Context<'turn,'source>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>where 'source:'turn{self.receive(c,effect,reply)}
}
