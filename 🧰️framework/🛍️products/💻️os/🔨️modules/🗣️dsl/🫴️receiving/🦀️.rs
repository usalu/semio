//! 🫴️ Ordinal operations retain generic parsing and owned typed binding across original native turns.
pub use semio_framework_dsl_record::BorrowedDslVariants;
use semio_framework_dsl_record::{FieldValue,receiving::{DslOwnedReceivingVariants,OwnedFieldReceiving,OwnedSlot,owned::OwnedAdvanceSelection}};
pub use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,ValueError};
pub use semio_framework_value_native_owned::OriginalReceivingPrimary;
pub use protocol::{OpBinaryReceiving,OpBinaryReceivingCursor};
use semio_framework_value::{AuthoringWallet,OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneStep,RetainedWorkPoll,RetirementDemand,ValueRefusalKind,retirement::{RetireOwned,RetirementCursor,RetirementStep,erased_cursor_turn}};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_os_pack::record::RetainedRecordReceiving;
use std::mem::{ManuallyDrop,size_of};
/// 🎞️ Implements an authored ordinal operation only through its actual owned receiving capability.
#[macro_export]
macro_rules! original_ordinal_operation_receiving{
 ($operation:ty)=>{impl $crate::original_receiving::OpBinaryReceiving for $operation{
  type DomainFailure=$crate::original_receiving::ValueError;
  type Receiving=$crate::original_receiving::OriginalOperationReceiving<Self>;
  fn admit_receiving(source:&[u8],range:(usize,usize),grant:$crate::original_receiving::RetainedCloneGrant)->$crate::original_receiving::RetainedWorkOutcome<Option<(Self::Receiving,$crate::original_receiving::RetainedCloneProgress)>>{$crate::original_receiving::OriginalOperationReceiving::admit(source,range,grant)}
 }};
}
/// 🏷️ Resolves authored protocol tags at compile time and retains one-tag runtime selection.
#[macro_export]
macro_rules! original_tagged_operation_receiving{
 ($operation:ty,$protocol:expr)=>{impl $crate::original_receiving::OpBinaryReceiving for $operation{
  type DomainFailure=$crate::original_receiving::ValueError;
  type Receiving=$crate::original_receiving::OriginalOperationReceiving<Self>;
  fn admit_receiving(source:&[u8],range:(usize,usize),grant:$crate::original_receiving::RetainedCloneGrant)->$crate::original_receiving::RetainedWorkOutcome<Option<(Self::Receiving,$crate::original_receiving::RetainedCloneProgress)>>{
   const TAGS:[u64;<$operation as $crate::original_receiving::BorrowedDslVariants>::VARIANTS.len()]={
    let variants=<$operation as $crate::original_receiving::BorrowedDslVariants>::VARIANTS;let mut tags=[0;<$operation as $crate::original_receiving::BorrowedDslVariants>::VARIANTS.len()];let mut index=0;
    while index<variants.len(){tags[index]=$crate::protocol_record::tag($protocol,variants[index].0);let mut previous=0;while previous<index{assert!(tags[previous]!=tags[index],"authored operation tags must be unique");previous+=1;}index+=1;}tags
   };
   $crate::original_receiving::OriginalOperationReceiving::admit_tagged(source,range,&TAGS,grant)
  }
 }};
}
#[path="📦️pack-value/🦀️.rs"]
mod pack_value;
pub use pack_value::{OriginalPackValueReceiving,PackValueRetirement,decoded_value_retained_bytes,pack_value_conversion_quote};
pub struct OriginalOperationReceiving<M:DslOwnedReceivingVariants+BorrowedDslVariants>{
 source:(usize,usize),range:(usize,usize),position:usize,phase:u8,tag:u64,shift:u8,tags:Option<&'static[u64]>,ordinal:usize,
 record:Option<RetainedRecordReceiving>,generic:OwnedSlot<FieldValue>,binding:Option<M::Cursor>,output:OwnedSlot<M>,closing:u8,closed:bool,failed:bool,
}
fn invalid(text:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,text)}
fn metadata<T>()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
fn grant_child(grant:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_depth:grant.maximum_depth.saturating_sub(1),..grant}}
fn funded(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
fn receipt(demand:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,released_bytes:demand.release_bytes}}
impl<M:DslOwnedReceivingVariants+BorrowedDslVariants> OriginalOperationReceiving<M>{
 /// 🔭️ Observes the receiving phase without advancing it.
 pub fn phase(&self)->u8{self.phase}
 /// 🔭️ Borrows the generic field value once it has been received.
 pub fn generic_value(&self)->Option<&FieldValue>{self.generic.value.as_ref()}
 /// 🎟️ Preserves the full original source and exact operation range without parsing or allocating.
 pub fn admit(source:&[u8],range:(usize,usize),grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(Self,RetainedCloneProgress)>>{let mut work=0;let result=(||{
  if range.0>range.1||range.1>source.len(){return Err(invalid("original operation escaped its full source"))}if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
  work=1;Ok(Some((Self{source:(source.as_ptr()as usize,source.len()),range,position:range.0,phase:0,tag:0,shift:0,tags:None,ordinal:0,record:None,generic:Default::default(),binding:None,output:Default::default(),closing:0,closed:false,failed:false},RetainedCloneProgress{copied_items:1,..Default::default()})))
 })();RetainedWorkOutcome{result,work}}
 /// 🏷️ Borrows immutable authored tags without a runtime metadata owner.
 pub fn admit_tagged(source:&[u8],range:(usize,usize),tags:&'static[u64],grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(Self,RetainedCloneProgress)>>{if tags.len()!=M::VARIANTS.len(){return RetainedWorkOutcome{result:Err(invalid("original operation tag declaration differs from variants")),work:0}}let turn=Self::admit(source,range,grant);RetainedWorkOutcome{result:turn.result.map(|admitted|admitted.map(|(mut owner,p)|{owner.tags=Some(tags);(owner,p)})),work:turn.work}}
 fn validate(&self,source:&[u8])->Result<(),ValueError>{if self.source!=(source.as_ptr()as usize,source.len()){return Err(invalid("original operation source identity changed"))}Ok(())}
 fn spec(&self)->Result<(&'static str,semio_framework_dsl_record::BorrowedRecordSpec),ValueError>{let(keyword,make)=M::VARIANTS.get(self.ordinal).ok_or_else(||invalid("original operation ordinal is not declared"))?;Ok((*keyword,make()))}
 fn demands_inner(&self,source:&[u8],body:usize)->Result<RetirementDemand,ValueError>{
  self.validate(source)?;let mut demand=match self.phase{
   0|1=>metadata::<u64>(),2=>metadata::<usize>(),3=>RetirementDemand{depth:2,..Default::default()},
   4=>{let record=self.record.as_ref().unwrap();if record.is_ready(){metadata::<FieldValue>()}else{return record.demands(source).map(|mut demand|{demand.depth+=1;demand})}},
   5=>{let binding=self.binding.as_ref().unwrap();if binding.is_ready(){metadata::<M>()}else{return binding.demands(&self.generic.value,body).map(|mut demand|{demand.depth+=1;demand})}},
   _=>RetirementDemand{depth:1,..Default::default()},
  };if self.phase==4||self.phase==5{demand.depth+=1}Ok(demand)
 }
}
fn perform(effect:&mut RetainedAllocationEffect<'_>,units:u128)->Result<(),ValueError>{let units=u64::try_from(units).map_err(|_|ValueError::invariant("original operation work exceeds a turn"))?;if effect.performed(units)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original operation work is not funded"))}}
fn unpoll<E>(poll:RetainedWorkPoll<RetainedCloneProgress,E>)->RetainedWorkOutcome<RetainedCloneProgress,E>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{work}=>RetainedWorkOutcome{result:Ok(Default::default()),work}}}
fn stepped(poll:RetainedWorkPoll<RetainedCloneProgress,OriginalRetirementFailure>)->OriginalRetirementPoll{match poll{RetainedWorkPoll::Pending{work}=>RetainedWorkPoll::Pending{work},RetainedWorkPoll::Complete(turn)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work})}}
impl<M:DslOwnedReceivingVariants+BorrowedDslVariants> OriginalOperationReceiving<M>{
 fn step_inner(&mut self,source:&[u8],native:&mut NativeDecodeControl<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
  let result=self.step_action(source,native,body,effect);if result.is_err(){self.failed=true}RetainedWorkOutcome{result,work:effect.performed_work()}
 }
 fn step_action(&mut self,source:&[u8],native:&mut NativeDecodeControl<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,OriginalReceivingPrimary>{
  let grant=effect.grant();let demand=self.demands_inner(source,body)?;
  if self.failed||self.phase==6||self.closing!=0||!funded(grant,demand){return Ok(Default::default())}
  let child=grant_child(grant);
  let record_ready=match self.record.as_ref(){Some(record)=>record.is_ready(),None=>false};
  let binding_ready=match self.binding.as_ref(){Some(binding)=>binding.is_ready(),None=>false};
  match self.phase{
   4 if !record_ready=>{let record=self.record.as_mut().ok_or_else(||ValueError::invariant("record absent"))?;record.step(source,native,&mut effect.nested(child)).result},
   5 if !binding_ready=>{
    let binding=self.binding.as_mut().ok_or_else(||ValueError::invariant("binding absent"))?;
    match binding.select_advance(&self.generic.value,body,effect.maximum_work_units())?{
     OwnedAdvanceSelection::Local{..}=>unpoll(binding.advance_local(&mut self.generic.value,body,&mut effect.nested(child))).result,
     OwnedAdvanceSelection::Nested=>unpoll(binding.advance_nested(&mut self.generic.value,native,body,effect.maximum_work_units(),child)).result,
    }
   },
   3=>{let(_,spec)=self.spec()?;let turn=RetainedRecordReceiving::admit(source,(self.position,self.range.1),spec,semio_os_pack::PackLimits::default(),child);perform(effect,turn.work)?;Ok(match turn.result?{Some((owner,progress))=>{self.record=Some(owner);self.phase=4;progress},None=>Default::default()})},
   4=>{let mut original=None;let record=self.record.as_mut().ok_or_else(||ValueError::invariant("record absent"))?;let turn=record.take_into(&mut original,child);perform(effect,turn.work)?;let progress=turn.result?;if let Some(value)=original{*self.generic.value=Some(FieldValue::Record(value));self.phase=5}Ok(progress)},
   5=>{let binding=self.binding.as_mut().ok_or_else(||ValueError::invariant("binding absent"))?;let turn=unpoll(binding.take_into(&mut self.output.value,&mut effect.nested(child)));if self.output.value.is_some(){self.phase=6}turn.result.map_err(OriginalReceivingPrimary::from)},
   _=>{
    native.checkpoint()?;perform(effect,1)?;
    let mut progress=RetainedCloneProgress::default();
    let result=(||{match self.phase{
      0=>{if source.get(self.position)!=Some(&1)||self.position>=self.range.1{return Err(invalid("original operation binary format is invalid"))}self.position+=1;self.phase=1;progress=receipt(demand)},
      1=>{let byte=source.get(self.position).copied().filter(|_|self.position<self.range.1).ok_or_else(||invalid("original operation ordinal is truncated"))?;if self.shift==63&&byte>1{return Err(invalid("original operation ordinal overflows u64"))}self.tag|=u64::from(byte&127)<<self.shift;self.position+=1;progress=receipt(demand);if byte&128==0{if self.shift>0&&byte==0{return Err(invalid("original operation ordinal is not minimal"))}self.phase=2}else{self.shift=self.shift.checked_add(7).filter(|value|*value<=63).ok_or_else(||invalid("original operation ordinal overflows u64"))?;}},
      2=>{if let Some(tags)=self.tags{let tag=tags.get(self.ordinal).ok_or_else(||invalid("original operation tag is not declared"))?;if *tag!=self.tag{self.ordinal+=1;progress=receipt(demand);return Ok(progress)}}else{self.ordinal=usize::try_from(self.tag).map_err(|_|invalid("original operation ordinal exceeds address space"))?;progress=receipt(demand)}let(keyword,_)=self.spec()?;self.binding=Some(M::receiving_cursor(keyword)?);self.phase=3;progress=receipt(demand)},
      _=>return Err(invalid("original operation phase is invalid")),
     }Ok(progress)
    })();
    result.map_err(|error|OriginalReceivingPrimary::from(error.with_retained_progress(progress)))
   }
  }
 }
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closed{return Ok(Default::default())}if self.closing==0{return Ok(metadata::<()>())}if !self.output.terminal_is_empty(){return self.output.close_demands(body).map(|mut demand|{demand.depth+=1;demand})}
  if let Some(binding)=&self.binding{if !binding.terminal_is_empty(){return binding.close_demands(body).map(|mut demand|{demand.depth+=1;demand})}return Ok(RetirementDemand{depth:1,..Default::default()})}
  if !self.generic.terminal_is_empty(){return self.generic.close_demands(body).map(|mut demand|{demand.depth+=1;demand})}
  if let Some(record)=&self.record{if !record.terminal_is_empty(){return record.close_demands(body).map(|mut demand|{demand.depth+=1;demand})}return Ok(RetirementDemand{depth:1,..Default::default()})}
  Ok(RetirementDemand{depth:1,..Default::default()})
 }
 fn close(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  let grant=effect.grant();let child=grant_child(grant);
  let settle=|effect:&RetainedAllocationEffect<'_>,result:Result<RetainedCloneStep,ValueError>|RetainedWorkPoll::Complete(RetainedWorkOutcome{result:result.map_err(OriginalRetirementFailure::from),work:effect.performed_work()});
  if self.closed{return settle(effect,Ok(RetainedCloneStep::Complete(Default::default())))}
  let demand=match self.close_demand(grant.maximum_copy_bytes){Ok(d)=>d,Err(e)=>return settle(effect,Err(e))};if !funded(grant,demand){return settle(effect,Ok(RetainedCloneStep::Progress(Default::default())))}
  if self.closing==0{if let Err(e)=perform(effect,1){return settle(effect,Err(e))}self.closing=1;return settle(effect,Ok(RetainedCloneStep::Progress(receipt(demand))))}
  if !self.output.terminal_is_empty(){return stepped(self.output.close_step(&mut effect.nested(child)))}
  if let Some(binding)=&mut self.binding{if !binding.terminal_is_empty(){return stepped(binding.close_step(&mut effect.nested(child)))}if let Err(e)=perform(effect,1){return settle(effect,Err(e))}std::mem::forget(self.binding.take());return settle(effect,Ok(RetainedCloneStep::Progress(receipt(demand))))}
  if !self.generic.terminal_is_empty(){return stepped(self.generic.close_step(&mut effect.nested(child)))}
  if let Some(record)=&mut self.record{if !record.terminal_is_empty(){return match record.close_step(&mut effect.nested(child)){RetainedWorkPoll::Pending{work}=>RetainedWorkPoll::Pending{work},RetainedWorkPoll::Complete(turn)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work})}}if let Err(e)=perform(effect,1){return settle(effect,Err(e))}self.record=None;return settle(effect,Ok(RetainedCloneStep::Progress(receipt(demand))))}
  if let Err(e)=perform(effect,1){return settle(effect,Err(e))}self.closed=true;settle(effect,Ok(RetainedCloneStep::Complete(receipt(demand))))
 }
}
impl<M:DslOwnedReceivingVariants+BorrowedDslVariants+OpBinaryReceiving<DomainFailure=ValueError>> protocol::OpBinaryReceivingCursor<M> for OriginalOperationReceiving<M>{
 fn demands(&self,source:&[u8],body:usize)->Result<RetirementDemand,ValueError>{self.demands_inner(source,body)}
 fn step_in(&mut self,source:&[u8],native:&mut NativeDecodeControl<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{self.step_inner(source,native,body,effect)}
 fn domain_failure(&self)->Option<&ValueError>{None}
 fn domain_failure_demand(&self)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
 fn domain_failure_work_demand(&self)->Result<u64,ValueError>{Ok(0)}
 fn take_domain_failure_into(&mut self,recipient:&mut Option<ValueError>,_:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{RetainedWorkOutcome{result:if recipient.is_some(){Err(invalid("operation domain failure requires an empty original recipient"))}else{Ok(Default::default())},work:0}}
 fn is_ready(&self)->bool{self.phase==6&&self.closing==0&&self.output.value.is_some()}
 fn take_into(&mut self,recipient:&mut Option<M>,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{let demand=metadata::<M>();if self.failed||self.phase!=6||self.closing!=0||recipient.is_some()||self.output.value.is_none()||!funded(grant,demand){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}*recipient=self.output.value.take();RetainedWorkOutcome{result:Ok(receipt(demand)),work:1}}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demand(body)}
 fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{self.close(effect)}
 fn terminal_is_empty(&self)->bool{self.closed&&self.output.terminal_is_empty()&&self.binding.is_none()&&self.generic.terminal_is_empty()&&self.record.is_none()}
}
/// ♻️ Retires the operation receiving's remaining owners through its paid close.
#[doc(hidden)]
pub struct OperationRetirement<M:DslOwnedReceivingVariants+BorrowedDslVariants>(ManuallyDrop<OriginalOperationReceiving<M>>);
impl<M:DslOwnedReceivingVariants+BorrowedDslVariants> RetireOwned for OriginalOperationReceiving<M>{
 type RetirementCursor=OperationRetirement<M>;
 fn retirement_seed(self)->Self::RetirementCursor{OperationRetirement(ManuallyDrop::new(self))}
 fn controlled_retirement_supported()->bool{true}
}
impl<M:DslOwnedReceivingVariants+BorrowedDslVariants> RetirementCursor for OperationRetirement<M>{
 fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetirementStep{erased_cursor_turn(self.0.close(effect))}
 fn next_computational_work(&self,_body:usize)->Result<u64,ValueError>{Ok(1)}
 fn terminal_is_empty(&self)->bool{self.0.closed}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.0.close_demand(usize::MAX)?.depth)}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.0.close_demand(usize::MAX)?.copy_bytes)}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.0.close_demand(body)?.copy_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.0.close_demand(usize::MAX).ok().map(|demand|demand.release_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.0.close_demand(body).ok().map(|demand|demand.capacity_bytes)}
 fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
}
impl<M:DslOwnedReceivingVariants+BorrowedDslVariants> Drop for OperationRetirement<M>{fn drop(&mut self){assert!(std::thread::panicking()||self.0.closed,"original operation retirement lost its retained fields");if self.0.closed{unsafe{ManuallyDrop::drop(&mut self.0)}}}}

/// 🧪️ Explicit physical stock, per-turn quantum and total work that [`receive_to_completion`] may spend.
#[derive(Clone,Copy,Debug)]
pub struct ReceivingStock{pub stock:RetainedCloneGrant,pub quantum:RetainedCloneGrant,pub fuel:u64}
use semio_framework_value_native_owned::native_decoding::{OriginalWorkAuthority,OriginalWorkRecipient,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalSourceTurnIdentity,OriginalConstructionAuthority,OriginalConstructionRecipient,OriginalConstructionGrant,OriginalConstructionGrantComplete,OriginalConstructionAccept,OriginalConstructionAcceptComplete};
struct TurnWork(u64);
struct TurnWorkDriver;
impl OriginalWorkRecipient for TurnWorkDriver{
 type Context<'t,'s>=TurnWork where 's:'t;
 fn validate_context<'t,'s>(&self,_:&TurnWork)->Result<(),ValueError> where 's:'t{Ok(())}
 fn source_turn<'t,'s>(&self,_:&TurnWork)->Option<OriginalSourceTurnIdentity> where 's:'t{None}
 fn admits<'r,'t,'s>(&mut self,c:&mut TurnWork,quote:OriginalWorkQuote<'r>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'r> where 's:'t{quote.complete(Ok(observed.checked_add(additional).is_some_and(|work|work<=c.0)))}
 fn settle_partial<'r,'t,'s>(&mut self,_:&mut TurnWork,check:OriginalWorkPartialCheck<'r>)->OriginalWorkPartialCheckComplete<'r> where 's:'t{check.complete(Ok(()))}
 fn recover_partial<'r,'t,'s>(&mut self,_:&mut TurnWork,check:OriginalWorkPartialCheck<'r>)->OriginalWorkPartialCheckComplete<'r> where 's:'t{check.pending()}
}
struct TurnPhysical(RetainedCloneGrant);
struct TurnPhysicalDriver;
impl OriginalConstructionRecipient for TurnPhysicalDriver{
 type Context<'t,'s>=TurnPhysical where 's:'t;
 fn validate_context<'t,'s>(&self,_:&TurnPhysical)->Result<(),ValueError> where 's:'t{Ok(())}
 fn source_turn<'t,'s>(&self,_:&TurnPhysical)->Option<OriginalSourceTurnIdentity> where 's:'t{None}
 fn grant<'r,'t,'s>(&mut self,c:&mut TurnPhysical,quote:OriginalConstructionGrant<'r>)->OriginalConstructionGrantComplete<'r> where 's:'t{quote.complete(Ok(c.0))}
 fn accept<'r,'t,'s>(&mut self,c:&mut TurnPhysical,receipt:OriginalConstructionAccept<'r>)->OriginalConstructionAcceptComplete<'r> where 's:'t{let(grant,progress)=(receipt.grant(),receipt.progress());let result=if !progress.fits(grant)||!progress.fits(c.0){Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"receiving progress exceeds its physical grant"))}else{debit(&mut c.0,progress);Ok(())};receipt.complete(result)}
 fn submitted<'t,'s>(&self,_:&TurnPhysical)->RetainedCloneProgress where 's:'t{RetainedCloneProgress::default()}
}
fn debit(left:&mut RetainedCloneGrant,progress:RetainedCloneProgress){left.maximum_items=left.maximum_items.saturating_sub(progress.copied_items);left.maximum_copy_bytes=left.maximum_copy_bytes.saturating_sub(progress.copied_bytes);left.maximum_capacity_bytes=left.maximum_capacity_bytes.saturating_sub(progress.retained_capacity_bytes);left.maximum_release_bytes=left.maximum_release_bytes.saturating_sub(progress.released_bytes)}
fn quantum_of(stock:&ReceivingStock)->RetainedCloneGrant{let (s,q)=(stock.stock,stock.quantum);RetainedCloneGrant{maximum_items:if stock.fuel==0{0}else{s.maximum_items.min(q.maximum_items)},maximum_copy_bytes:s.maximum_copy_bytes.min(q.maximum_copy_bytes),maximum_capacity_bytes:s.maximum_capacity_bytes.min(q.maximum_capacity_bytes),maximum_release_bytes:s.maximum_release_bytes.min(q.maximum_release_bytes),maximum_depth:s.maximum_depth.min(q.maximum_depth)}}
fn spend(stock:&mut ReceivingStock,progress:RetainedCloneProgress,work:u128){debit(&mut stock.stock,progress);stock.fuel=stock.fuel.saturating_sub(u64::try_from(work).unwrap_or(u64::MAX))}
fn stalled(text:&'static str)->OriginalReceivingPrimary{ValueError::literal(ValueRefusalKind::WorkLimit,text).into()}
/// 🧪️ Drives one stored operation through its own original receiving cursor, paying every turn and the paid close from the explicit stock. A close that the stock cannot pay refuses with `WorkLimit` and abandons the unpaid cursor without running its terminal-empty drop assertions.
pub fn receive_to_completion<M:OpBinaryReceiving>(source:&[u8],range:(usize,usize),mut stock:ReceivingStock)->Result<M,OriginalReceivingPrimary>{
 let turn=M::admit_receiving(source,range,quantum_of(&stock));let admitted=turn.result?;let Some((mut cursor,progress))=admitted else{return Err(stalled("original receiving was denied admission"))};spend(&mut stock,progress,turn.work);
 let mut callback=|_|true;let mut native=NativeDecodeControl::new(stock.stock.maximum_capacity_bytes,&mut callback);let mut failure=None;
 while !cursor.is_ready()&&failure.is_none(){
  if stock.fuel==0{failure=Some(stalled("original receiving exhausted its work"));break}
  let grant=quantum_of(&stock);let mut work=TurnWork(1);let mut physical=TurnPhysical(grant);
  let turn=OriginalWorkAuthority::new(TurnWorkDriver).with_loan(&mut work,|work|native.with_work_recipient(work,|native|OriginalConstructionAuthority::new(TurnPhysicalDriver).with_loan(&mut physical,|physical|native.with_construction_recipient(physical,|native|Ok::<_,ValueError>(cursor.step(source,native,stock.quantum.maximum_copy_bytes,grant))))?)).and_then(|turn|turn);
  let turn=match turn{Ok(turn)=>turn,Err(error)=>{failure=Some(error.into());break}};
  match turn.result{Ok(progress)=>{let idle=turn.work==0&&progress==RetainedCloneProgress::default();spend(&mut stock,progress,turn.work);if idle{failure=Some(stalled("original receiving made no progress"))}},Err(error)=>{spend(&mut stock,error.retained_progress(),turn.work);failure=Some(error)}}
 }
 let mut output=None;
 if failure.is_none(){let turn=cursor.take_into(&mut output,quantum_of(&stock));match turn.result{Ok(progress)=>spend(&mut stock,progress,turn.work),Err(error)=>failure=Some(error.into())}}
 let mut closes=stock.fuel.saturating_add(1);
 while !cursor.terminal_is_empty()&&closes>0{closes-=1;let grant=quantum_of(&stock);let wallet=AuthoringWallet::new(grant,u128::from(stock.fuel)).run_settled(grant,stock.fuel,|effect|cursor.close_step(effect));match wallet{Ok(Some((turn,_)))=>{let work=turn.work;match turn.result{Ok(step)=>spend(&mut stock,step.progress(),work),Err(error)=>{spend(&mut stock,error.retained_progress(),work);failure.get_or_insert(stalled("original receiving close was refused"));}}},Ok(None)=>{failure.get_or_insert(stalled("original receiving close was not admitted"));break},Err(error)=>{failure.get_or_insert(error.into());break}}}
 if !cursor.terminal_is_empty(){failure.get_or_insert(stalled("original receiving close exhausted its work"));std::mem::forget(cursor)}
 match(failure,output){(Some(error),_)=>Err(error),(None,Some(output))=>Ok(output),(None,None)=>Err(stalled("original receiving produced no operation"))}
}
/// ⚖️ Law: each value's own `encode_op` bytes, embedded in a larger source, are received by its original cursor into the value that the `decode_op` oracle returns.
pub fn check_receiving_round_trip<M:OpBinaryReceiving+PartialEq+std::fmt::Debug>(values:&[M])->Result<(),String>{
 let stock=ReceivingStock{stock:RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:64},quantum:RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<16,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64},fuel:1<<20};
 for(index,value)in values.iter().enumerate(){
  let bytes=value.encode_op().map_err(|error|format!("value {index} encode: {error:?}"))?;let oracle=M::decode_op(&bytes).map_err(|error|format!("value {index} oracle decode: {error:?}"))?;
  if&oracle!=value{return Err(format!("value {index} oracle differs: {oracle:?} != {value:?}"))}
  let mut source=vec![0xA5;3];source.extend_from_slice(&bytes);let end=source.len();source.extend_from_slice(&[0x5A;2]);
  let received=receive_to_completion::<M>(&source,(3,end),stock).map_err(|error|format!("value {index} receive: {}",error.message()))?;
  if received!=oracle{return Err(format!("value {index} receiving differs: {received:?} != {oracle:?}"))}
 }
 Ok(())
}
#[path="🧭️mirrored/🦀️.rs"]
pub mod mirrored;
