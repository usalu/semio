//! ⏱️ The actual Source clock owns each recorded reading until its original caller takes it.
use semio_framework_value_control_contract::{OriginalSourceTurnIdentity,ValueError,ValueRefusalKind};
use std::sync::atomic::{AtomicU64,Ordering};
static IDS:AtomicU64=AtomicU64::new(1);
fn next()->u64{IDS.try_update(Ordering::Relaxed,Ordering::Relaxed,|id|id.checked_add(1)).expect("original clock identity exhausted")}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original clock Source or action changed")}
/// 📨️ Deposits the actual reading before the original clock producer returns.
pub struct OriginalClockReply<'a>{slot:&'a mut Option<Result<Option<u128>,ValueError>>}
/// ✅️ Represents a recorded reading or a genuinely pending original clock action.
pub struct OriginalClockReplyComplete<'a>{_slot:&'a mut Option<Result<Option<u128>,ValueError>>}
impl<'a> OriginalClockReply<'a>{
 /// 📥️ Records the real reading, including an unavailable clock, before returning.
 pub fn complete(self,reading:Result<Option<u128>,ValueError>)->OriginalClockReplyComplete<'a>{assert!(self.slot.is_none(),"original clock reply occupied");*self.slot=Some(reading);OriginalClockReplyComplete{_slot:self.slot}}
 /// ⏳️ Retains an unresolved original clock reading.
 pub fn pending(self)->OriginalClockReplyComplete<'a>{assert!(self.slot.is_none(),"original clock reply occupied");OriginalClockReplyComplete{_slot:self.slot}}
}
/// 📏️ Quotes the original physical and computational demand of one clock producer turn.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalClockDemand{pub physical:semio_framework_value_retirement::RetirementDemand,pub work:u64}
/// 🏭️ One actual clock producer validates its real borrowed Source on every entry.
pub trait OriginalClockProducer{
 type Context<'turn,'source> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,context:&Self::Context<'turn,'source>)->Result<(),ValueError> where 'source:'turn;
 fn source_turn<'turn,'source>(&self,context:&Self::Context<'turn,'source>)->Option<OriginalSourceTurnIdentity> where 'source:'turn;
 fn demand<'turn,'source>(&self,context:&Self::Context<'turn,'source>)->Result<OriginalClockDemand,ValueError> where 'source:'turn;
 fn read<'reply,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>,reply:OriginalClockReply<'reply>)->Result<OriginalClockReplyComplete<'reply>,ValueError> where 'source:'turn;
 fn recover<'reply,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>,reply:OriginalClockReply<'reply>)->Result<OriginalClockReplyComplete<'reply>,ValueError> where 'source:'turn;
}
/// 📊️ Preserves the actual reading and whether it follows this same clock's prior reading.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalClockObservation{authority:u64,action:u64,source:Option<OriginalSourceTurnIdentity>,reading:Option<u128>,monotonic:bool}
impl OriginalClockObservation{
 /// 📖️ Observes the actual original reading, including an unavailable clock.
 pub fn reading(self)->Option<u128>{self.reading}
 /// 📈️ Reports whether the original reading preserved monotonic order.
 pub fn monotonic(self)->bool{self.monotonic}
}
/// 🎯️ One deadline is bound to the actual Source clock owner that issued it.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalClockDeadline{authority:u64,expires:u128}
impl OriginalClockDeadline{
 /// ⏱️ Observes the exact expiry in this original clock's epoch.
 pub fn expires(self)->u128{self.expires}
}
impl semio_framework_value_retirement::retirement::RetireOwned for OriginalClockObservation{
 type RetirementCursor=semio_framework_value_retirement::retirement::Leaf<Self>;
 fn retirement_seed(self)->Self::RetirementCursor{semio_framework_value_retirement::retirement::leaf(self)}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(semio_framework_value_retirement::retirement::leaf_birth_bytes::<Self>())}
 fn controlled_retirement_supported()->bool{true}
}
impl semio_framework_value_retirement::retirement::RetireOwned for OriginalClockDeadline{
 type RetirementCursor=semio_framework_value_retirement::retirement::Leaf<Self>;
 fn retirement_seed(self)->Self::RetirementCursor{semio_framework_value_retirement::retirement::leaf(self)}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(semio_framework_value_retirement::retirement::leaf_birth_bytes::<Self>())}
 fn controlled_retirement_supported()->bool{true}
}
/// 🧳️ Retains one original reading and its nonforgeable Source provenance.
pub struct OriginalClockAction{authority:u64,identity:u64,source:Option<OriginalSourceTurnIdentity>,work:semio_framework_value_control_contract::OriginalWorkActionIdentity,refusal:Option<ValueError>,reply:Option<Result<Option<u128>,ValueError>>,monotonic:Option<bool>,settled:bool,finished:bool}
impl OriginalClockAction{
 /// 📤️ Transfers the exact producer-context refusal into an empty caller-owned slot.
 pub fn transfer_refusal(&mut self,output:&mut Option<ValueError>)->Result<bool,ValueError>{if output.is_some(){return Err(invalid())}let Some(error)=self.refusal.take()else{return Ok(false)};*output=Some(error);Ok(true)}
 /// 📤️ Transfers the recorded reading directly into an empty caller-owned slot.
 pub fn transfer_reading(&mut self,output:&mut Option<Result<OriginalClockObservation,ValueError>>)->Result<bool,ValueError>{if output.is_some(){return Err(invalid())}if !self.settled{return Ok(false)}let monotonic=self.monotonic.ok_or_else(invalid)?;let Some(reply)=self.reply.take()else{return Err(invalid())};*output=Some(reply.map(|reading|OriginalClockObservation{authority:self.authority,action:self.identity,source:self.source,reading,monotonic}));Ok(true)}
}
impl Drop for OriginalClockAction{fn drop(&mut self){assert!(std::thread::panicking()||self.finished,"original clock action remains live")}}
impl semio_framework_value_retirement::retirement::RetireOwned for OriginalClockAction{
 type RetirementCursor=semio_framework_value_receiving::NativeRetirementGate<Self,<() as semio_framework_value_retirement::retirement::RetireOwned>::RetirementCursor>;
 fn retirement_ready(&self)->bool{false}
 fn retirement_seed(self)->Self::RetirementCursor{semio_framework_value_receiving::NativeRetirementGate::Pending(self)}
 fn retirement_birth_bytes(&self)->Option<usize>{None}
 fn controlled_retirement_supported()->bool{true}
}
/// 🏠️ Owns the real clock producer and its one original in-flight action.
pub struct OriginalClockAuthority<P:OriginalClockProducer>{producer:std::mem::ManuallyDrop<P>,identity:Option<u64>,active:Option<u64>,last:Option<u128>,latest:Option<u64>}
impl<P:OriginalClockProducer> OriginalClockAuthority<P>{
 /// 🌱️ Installs the actual producer without sampling a clock or allocating.
 pub fn new(producer:P)->Self{Self{producer:std::mem::ManuallyDrop::new(producer),identity:None,active:None,last:None,latest:None}}
 /// 👁️ Observes the same resident clock producer without lending mutable authority.
 pub fn inspect<R>(&self,query:impl FnOnce(&P)->R)->R{query(&self.producer)}
 /// ⏳️ Reports whether an original clock action remains in custody.
 pub fn has_pending_action(&self)->bool{self.active.is_some()}
 /// 🪪️ Validates and observes the actual borrowed Source turn.
 pub fn source_turn<'turn,'source>(&self,context:&P::Context<'turn,'source>)->Result<Option<OriginalSourceTurnIdentity>,ValueError> where 'source:'turn{self.producer.validate_context(context)?;Ok(self.producer.source_turn(context))}
 /// 🔐️ Validates the actual Source and same-clock deadline before any new reading.
 pub fn source_turn_for_deadline<'turn,'source>(&self,context:&P::Context<'turn,'source>,deadline:OriginalClockDeadline)->Result<Option<OriginalSourceTurnIdentity>,ValueError> where 'source:'turn{self.producer.validate_context(context)?;if self.identity!=Some(deadline.authority){return Err(invalid())}Ok(self.producer.source_turn(context))}
 /// 📏️ Reads the validated original producer demand before opening an action.
 pub fn demand<'turn,'source>(&self,context:&P::Context<'turn,'source>)->Result<OriginalClockDemand,ValueError> where 'source:'turn{self.producer.validate_context(context)?;self.producer.demand(context)}
 /// 🪢️ Starts or recovers the exact Source clock action without replaying an entered producer.
 pub fn drive<'turn,'source>(&mut self,context:&mut P::Context<'turn,'source>,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>,slot:&mut Option<OriginalClockAction>)->Result<bool,ValueError> where 'source:'turn{
  self.producer.validate_context(context)?;let source=self.producer.source_turn(context);if effect.work_source_turn()!=source{return Err(invalid())}let work=effect.work_identity().action_identity();let demand=self.producer.demand(context)?;if effect.remaining_work_units()?<demand.work||!semio_framework_value_control_contract::retained_clone_grant_admits(effect.grant(),demand.physical){return Ok(false)}let fresh=slot.is_none();if fresh{if self.active.is_some(){return Err(invalid())}let authority=*self.identity.get_or_insert_with(next);let identity=next();self.active=Some(identity);*slot=Some(OriginalClockAction{authority,identity,source,work,refusal:None,reply:None,monotonic:None,settled:false,finished:false})}let action=slot.as_mut().expect("original clock action installed");if Some(action.authority)!=self.identity||self.active!=Some(action.identity)||action.source!=source||action.work!=work||action.finished{return Err(invalid())}if action.refusal.is_some(){return Ok(false)}if action.settled{return Ok(true)}if action.reply.is_some(){return Ok(self.settle_recorded(action))}let reply=OriginalClockReply{slot:&mut action.reply};let complete=if fresh{self.producer.read(context,effect,reply)}else{self.producer.recover(context,effect,reply)};match complete{Ok(complete)=>drop(complete),Err(error)=>{action.refusal=Some(error);return Ok(false)}}if action.reply.is_some(){Ok(self.settle_recorded(action))}else{Ok(false)}
 }
 fn settle_recorded(&mut self,action:&mut OriginalClockAction)->bool{let monotonic=match action.reply.as_ref(){Some(Ok(Some(now)))=>{let valid=self.last.is_none_or(|last|*now>=last);if valid{self.last=Some(*now)}valid},Some(Ok(None))=>false,Some(Err(_))=>false,None=>return false};action.monotonic=Some(monotonic);self.latest=Some(action.identity);action.settled=true;true}
 /// 🎯️ Derives a deadline only from this actual Source clock's latest monotonic reading.
 pub fn deadline_from<'turn,'source>(&self,context:&P::Context<'turn,'source>,observation:OriginalClockObservation,duration:u64)->Result<OriginalClockDeadline,ValueError> where 'source:'turn{self.validate_observation(context,observation)?;if !observation.monotonic{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original clock is unavailable or moved backwards"))}let start=observation.reading.ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original clock is unavailable"))?;let expires=start.checked_add(u128::from(duration)).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original clock deadline overflow"))?;Ok(OriginalClockDeadline{authority:observation.authority,expires})}
 /// 🚦️ Applies an actual Source wallet's fuel to its same-clock deadline and latest reading.
 pub fn should_yield<'turn,'source>(&self,context:&P::Context<'turn,'source>,deadline:OriginalClockDeadline,observation:OriginalClockObservation,fuel:u64)->Result<bool,ValueError> where 'source:'turn{self.validate_observation(context,observation)?;if deadline.authority!=observation.authority{return Err(invalid())}Ok(fuel==0||!observation.monotonic||observation.reading.is_none_or(|now|now>=deadline.expires))}
 fn validate_observation<'turn,'source>(&self,context:&P::Context<'turn,'source>,observation:OriginalClockObservation)->Result<(),ValueError> where 'source:'turn{self.producer.validate_context(context)?;if self.identity!=Some(observation.authority)||self.latest!=Some(observation.action)||self.producer.source_turn(context)!=observation.source{return Err(invalid())}Ok(())}
 /// 🏁️ Releases the exact clock action only after its reading is transferred.
 pub fn finish<'turn,'source>(&mut self,context:&P::Context<'turn,'source>,slot:&mut Option<OriginalClockAction>)->Result<(),ValueError> where 'source:'turn{self.producer.validate_context(context)?;let action=slot.as_mut().ok_or_else(invalid)?;if Some(action.authority)!=self.identity||self.active!=Some(action.identity)||action.source!=self.producer.source_turn(context)||!action.settled||action.reply.is_some()||action.refusal.is_some(){return Err(invalid())}action.finished=true;self.active=None;drop(slot.take());Ok(())}
 /// 🫴️ Transfers the actual clock producer after every original action finishes.
 pub fn into_producer(self)->Result<P,Self>{if self.active.is_some(){return Err(self)}let mut owner=std::mem::ManuallyDrop::new(self);Ok(unsafe{std::mem::ManuallyDrop::take(&mut owner.producer)})}
}
impl<P:OriginalClockProducer> Drop for OriginalClockAuthority<P>{fn drop(&mut self){assert!(std::thread::panicking(),"original clock producer requires transfer or paid retirement")}}
impl<P:OriginalClockProducer+semio_framework_value_retirement::retirement::RetireOwned+Send+'static> semio_framework_value_retirement::retirement::RetireOwned for OriginalClockAuthority<P>{
 type RetirementCursor=semio_framework_value_receiving::NativeRetirementGate<Self,P::RetirementCursor>;
 fn retirement_ready(&self)->bool{self.active.is_none()&&semio_framework_value_retirement::retirement::RetireOwned::retirement_ready(&*self.producer)}
 fn retirement_seed(self)->Self::RetirementCursor{if !semio_framework_value_retirement::retirement::RetireOwned::retirement_ready(&self){return semio_framework_value_receiving::NativeRetirementGate::Pending(self)}let producer=self.into_producer().ok().expect("ready original clock producer");semio_framework_value_receiving::NativeRetirementGate::Ready(semio_framework_value_retirement::retirement::RetireOwned::retirement_seed(producer))}
 fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value_retirement::retirement::RetireOwned::retirement_ready(self).then_some(std::mem::size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{true}
}
/// 🚦️ Reports the retained state of one original Source deadline decision.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OriginalClockDecisionStatus{Pending,Ready,Refused}
/// 🧳️ Keeps the actual clock action, observation and decision across interruption.
#[derive(Default)]
pub struct OriginalClockDecisionFrame{request:Option<(OriginalClockDeadline,u64,Option<OriginalSourceTurnIdentity>)>,action:Option<OriginalClockAction>,reading:Option<Result<OriginalClockObservation,ValueError>>,refusal:Option<ValueError>,decision:Option<bool>,transferred:bool,failed:bool}
impl OriginalClockDecisionFrame{
 /// 🪹️ Creates an empty resident frame without sampling the clock.
 pub fn new()->Self{Self::default()}
 /// 🈳️ Reports whether all original clock custody has returned.
 pub fn terminal_is_empty(&self)->bool{self.request.is_none()&&self.action.is_none()&&self.reading.is_none()&&self.refusal.is_none()&&self.decision.is_none()&&!self.transferred&&!self.failed}
 /// ⏱️ Starts or recovers the same actual clock action and retains its decision.
 pub fn poll<'turn,'source,P:OriginalClockProducer>(&mut self,authority:&mut OriginalClockAuthority<P>,context:&mut P::Context<'turn,'source>,deadline:OriginalClockDeadline,fuel:u64)->Result<OriginalClockDecisionStatus,ValueError> where 'source:'turn{
  if self.transferred{return Err(invalid())}if self.refusal.is_some()||self.failed{return Ok(OriginalClockDecisionStatus::Refused)}let source=match authority.source_turn_for_deadline(context,deadline){Ok(Some(source))=>Some(source),Ok(None)=>return Err(invalid()),Err(error)=>{self.refusal=Some(error);return Ok(OriginalClockDecisionStatus::Refused)}};let request=(deadline,fuel,source);if self.request.is_some_and(|prior|prior!=request){return Err(invalid())}self.request=Some(request);if self.decision.is_some(){return Ok(OriginalClockDecisionStatus::Ready)}if fuel==0&&self.action.is_none()&&self.reading.is_none(){self.decision=Some(true);return Ok(OriginalClockDecisionStatus::Ready)}
  if self.reading.is_none(){match authority.drive(context,&mut self.action){Err(error)=>{self.refusal=Some(error);return Ok(OriginalClockDecisionStatus::Refused)},Ok(false)=>{if let Some(action)=self.action.as_mut(){action.transfer_refusal(&mut self.refusal)?;}return Ok(if self.refusal.is_some(){OriginalClockDecisionStatus::Refused}else{OriginalClockDecisionStatus::Pending})},Ok(true)=>{let action=self.action.as_mut().ok_or_else(invalid)?;if !action.transfer_reading(&mut self.reading)?{return Ok(OriginalClockDecisionStatus::Pending)}}}}
  if self.reading.as_ref().is_some_and(Result::is_err){let Some(Err(error))=self.reading.take()else{unreachable!()};self.refusal=Some(error);self.failed=true;return Ok(OriginalClockDecisionStatus::Refused)}let observation=*self.reading.as_ref().and_then(|reading|reading.as_ref().ok()).ok_or_else(invalid)?;match authority.should_yield(context,deadline,observation,fuel){Ok(decision)=>{self.decision=Some(decision);Ok(OriginalClockDecisionStatus::Ready)},Err(error)=>{self.refusal=Some(error);Ok(OriginalClockDecisionStatus::Refused)}}
 }
 /// ↩️ Clears only a decision whose original clock producer never started.
 pub fn cancel_unstarted(&mut self)->Result<(),ValueError>{if self.action.is_some()||self.reading.is_some()||self.refusal.is_some()||self.decision.is_some()||self.transferred||self.failed{return Err(invalid())}self.request=None;Ok(())}
 /// 📤️ Transfers the exact original refusal into an empty caller-owned slot.
 pub fn transfer_refusal(&mut self,output:&mut Option<ValueError>)->Result<bool,ValueError>{if output.is_some(){return Err(invalid())}let Some(error)=self.refusal.take()else{return Ok(false)};*output=Some(error);Ok(true)}
 /// 📤️ Transfers the actual recorded decision into an empty caller-owned slot.
 pub fn transfer_decision(&mut self,output:&mut Option<bool>)->Result<bool,ValueError>{if output.is_some()||self.transferred{return Err(invalid())}let Some(decision)=self.decision.take()else{return Ok(false)};*output=Some(decision);self.transferred=true;Ok(true)}
 /// 🏁️ Finishes the same original action after its actual decision or refusal transfers.
 pub fn finish<'turn,'source,P:OriginalClockProducer>(&mut self,authority:&mut OriginalClockAuthority<P>,context:&P::Context<'turn,'source>)->Result<(),ValueError> where 'source:'turn{if self.refusal.is_some()||(!self.transferred&&!self.failed){return Err(invalid())}let (deadline,_,source)=self.request.ok_or_else(invalid)?;if authority.source_turn_for_deadline(context,deadline)?!=source{return Err(invalid())}if self.action.is_some(){authority.finish(context,&mut self.action)?}self.request=None;self.reading=None;self.transferred=false;self.failed=false;Ok(())}
}
impl Drop for OriginalClockDecisionFrame{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original clock decision remains live")}}
impl semio_framework_value_retirement::retirement::RetireOwned for OriginalClockDecisionFrame{
 type RetirementCursor=semio_framework_value_receiving::NativeRetirementGate<Self,<() as semio_framework_value_retirement::retirement::RetireOwned>::RetirementCursor>;
 fn retirement_ready(&self)->bool{self.terminal_is_empty()}
 fn retirement_seed(self)->Self::RetirementCursor{if !self.terminal_is_empty(){semio_framework_value_receiving::NativeRetirementGate::Pending(self)}else{semio_framework_value_receiving::NativeRetirementGate::Ready(semio_framework_value_retirement::retirement::RetireOwned::retirement_seed(()))}}
 fn retirement_birth_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{true}
}
/// 🧵️ Borrows the actual Source issuer and its owned dispatch token.
pub struct OriginalSystemClockContext<'a>{issuer:&'a semio_framework_value_control_contract::OriginalSourceTurnIssuer,turn:&'a semio_framework_value_control_contract::OriginalSourceTurn}
impl<'a> OriginalSystemClockContext<'a>{
 /// 🔗️ Binds only a token issued by the actual Source issuer.
 pub fn new(issuer:&'a semio_framework_value_control_contract::OriginalSourceTurnIssuer,turn:&'a semio_framework_value_control_contract::OriginalSourceTurn)->Result<Self,ValueError>{issuer.validate(turn)?;Ok(Self{issuer,turn})}
 /// 🪪️ Observes the identity of the actual retained Source dispatch.
 pub fn source_turn(&self)->OriginalSourceTurnIdentity{self.turn.identity()}
 /// 🔐️ Validates this token against the exact original Source issuer.
 pub fn matches_issuer(&self,identity:semio_framework_value_control_contract::OriginalSourceTurnIssuerIdentity)->bool{self.issuer.identity()==identity&&self.issuer.validate(self.turn).is_ok()}
}
/// 🌐️ An owned host records its real reading or retains the same pending action.
pub trait OriginalHostClockProducer{
 fn read<'reply>(&mut self,context:&OriginalSystemClockContext<'_>,reply:OriginalClockReply<'reply>)->Result<OriginalClockReplyComplete<'reply>,ValueError>;
 fn recover<'reply>(&mut self,context:&OriginalSystemClockContext<'_>,reply:OriginalClockReply<'reply>)->Result<OriginalClockReplyComplete<'reply>,ValueError>;
}
/// 🏠️ Owns the actual host producer and binds it to its issuing Source.
pub struct OriginalHostClock<P:OriginalHostClockProducer>{issuer:semio_framework_value_control_contract::OriginalSourceTurnIssuerIdentity,host:P}
impl<P:OriginalHostClockProducer> OriginalHostClock<P>{
 /// 🪢️ Installs the actual host without reading its clock.
 pub fn new(issuer:&semio_framework_value_control_contract::OriginalSourceTurnIssuer,host:P)->Self{Self{issuer:issuer.identity(),host}}
 /// 📤️ Returns the same host after all original actions have finished.
 pub fn into_host(self)->P{self.host}
}
impl<P:OriginalHostClockProducer> OriginalClockProducer for OriginalHostClock<P>{
 type Context<'turn,'source>=OriginalSystemClockContext<'turn> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Result<(),ValueError> where 'source:'turn{if !c.matches_issuer(self.issuer){return Err(invalid())}Ok(())}
 fn source_turn<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Option<OriginalSourceTurnIdentity> where 'source:'turn{Some(c.source_turn())}
 fn read<'reply,'turn,'source>(&mut self,c:&mut Self::Context<'turn,'source>,reply:OriginalClockReply<'reply>)->Result<OriginalClockReplyComplete<'reply>,ValueError> where 'source:'turn{self.validate_context(c)?;self.host.read(c,reply)}
 fn recover<'reply,'turn,'source>(&mut self,c:&mut Self::Context<'turn,'source>,reply:OriginalClockReply<'reply>)->Result<OriginalClockReplyComplete<'reply>,ValueError> where 'source:'turn{self.validate_context(c)?;self.host.recover(c,reply)}
}
impl<P:OriginalHostClockProducer+semio_framework_value_retirement::retirement::RetireOwned> semio_framework_value_retirement::retirement::RetireOwned for OriginalHostClock<P>{
 type RetirementCursor=P::RetirementCursor;
 fn retirement_ready(&self)->bool{self.host.retirement_ready()}
 fn retirement_seed(self)->Self::RetirementCursor{self.host.retirement_seed()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.host.retirement_birth_bytes()}
 fn controlled_retirement_supported()->bool{P::controlled_retirement_supported()}
}
/// 🏛️ Erases a paid host clock frame while retaining its original reply and retirement ports.
pub trait OriginalErasedHostClock:semio_framework_value_retirement::retirement::controlled::ErasedControlledRetirement{
 fn has_pending_action(&self)->bool;
 fn next_work(&self)->Result<u64,ValueError>;
 fn drive(&mut self,context:&mut OriginalSystemClockContext<'_>,action:&mut Option<OriginalClockAction>)->Result<bool,ValueError>;
 fn finish(&mut self,context:&OriginalSystemClockContext<'_>,action:&mut Option<OriginalClockAction>)->Result<(),ValueError>;
 fn deadline_from(&self,context:&OriginalSystemClockContext<'_>,observation:OriginalClockObservation,duration:u64)->Result<OriginalClockDeadline,ValueError>;
 fn poll_decision(&mut self,context:&mut OriginalSystemClockContext<'_>,frame:&mut OriginalClockDecisionFrame,deadline:OriginalClockDeadline,fuel:u64)->Result<OriginalClockDecisionStatus,ValueError>;
 fn finish_decision(&mut self,context:&OriginalSystemClockContext<'_>,frame:&mut OriginalClockDecisionFrame)->Result<(),ValueError>;
}
impl<P:OriginalHostClockProducer+semio_framework_value_retirement::retirement::RetireOwned+Send+'static> OriginalErasedHostClock for semio_framework_value_retirement::retirement::controlled::ControlledRetirement<OriginalClockAuthority<OriginalHostClock<P>>>{
 fn has_pending_action(&self)->bool{self.original().is_some_and(OriginalClockAuthority::has_pending_action)}
 fn next_work(&self)->Result<u64,ValueError>{self.next_computational_work(0)}
 fn drive(&mut self,c:&mut OriginalSystemClockContext<'_>,a:&mut Option<OriginalClockAction>)->Result<bool,ValueError>{self.original_mut().ok_or_else(invalid)?.drive(c,a)}
 fn finish(&mut self,c:&OriginalSystemClockContext<'_>,a:&mut Option<OriginalClockAction>)->Result<(),ValueError>{self.original_mut().ok_or_else(invalid)?.finish(c,a)}
 fn deadline_from(&self,c:&OriginalSystemClockContext<'_>,o:OriginalClockObservation,d:u64)->Result<OriginalClockDeadline,ValueError>{self.original().ok_or_else(invalid)?.deadline_from(c,o,d)}
 fn poll_decision(&mut self,c:&mut OriginalSystemClockContext<'_>,f:&mut OriginalClockDecisionFrame,d:OriginalClockDeadline,fuel:u64)->Result<OriginalClockDecisionStatus,ValueError>{f.poll(self.original_mut().ok_or_else(invalid)?,c,d,fuel)}
 fn finish_decision(&mut self,c:&OriginalSystemClockContext<'_>,f:&mut OriginalClockDecisionFrame)->Result<(),ValueError>{f.finish(self.original_mut().ok_or_else(invalid)?,c)}
}
/// 🎟️ Admits the actual host owner into a paid retirement frame before erasing its type.
pub fn admit_original_host_clock<P:OriginalHostClockProducer+semio_framework_value_retirement::retirement::RetireOwned+Send+'static>(original:&mut Option<OriginalClockAuthority<OriginalHostClock<P>>>,output:&mut Option<Box<dyn OriginalErasedHostClock>>,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->semio_framework_value_control_contract::RetainedWorkOutcome<Option<semio_framework_value_control_contract::RetainedCloneProgress>,ValueError>{if output.is_some(){return semio_framework_value_control_contract::RetainedWorkOutcome{result:Err(invalid()),work:0}}let turn=semio_framework_value_retirement::retirement::controlled::admit_typed_controlled_retirement(original,effect);semio_framework_value_control_contract::RetainedWorkOutcome{result:turn.result.map(|owner|owner.map(|(owner,progress)|{*output=Some(owner as Box<dyn OriginalErasedHostClock>);progress})),work:turn.work}}
/// 🕰️ One native monotonic clock owns its real origin for the entire Source lifetime.
#[cfg(any(not(target_arch="wasm32"),target_env="p2"))]
pub struct OriginalSystemClock{issuer:semio_framework_value_control_contract::OriginalSourceTurnIssuerIdentity,origin:Option<std::time::Instant>}
#[cfg(any(not(target_arch="wasm32"),target_env="p2"))]
impl OriginalSystemClock{
 /// 🏁️ Binds the actual Source issuer without sampling the clock.
 pub fn new(issuer:&semio_framework_value_control_contract::OriginalSourceTurnIssuer)->Self{Self{issuer:issuer.identity(),origin:None}}
}
#[cfg(any(not(target_arch="wasm32"),target_env="p2"))]
impl OriginalClockProducer for OriginalSystemClock{
 type Context<'turn,'source>=OriginalSystemClockContext<'turn> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Result<(),ValueError> where 'source:'turn{if c.issuer.identity()!=self.issuer{return Err(invalid())}c.issuer.validate(c.turn)?;Ok(())}
 fn source_turn<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Option<OriginalSourceTurnIdentity> where 'source:'turn{Some(c.turn.identity())}
 fn read<'reply,'turn,'source>(&mut self,c:&mut Self::Context<'turn,'source>,reply:OriginalClockReply<'reply>)->Result<OriginalClockReplyComplete<'reply>,ValueError> where 'source:'turn{OriginalHostClockProducer::read(self,c,reply)}
 fn recover<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,reply:OriginalClockReply<'reply>)->Result<OriginalClockReplyComplete<'reply>,ValueError> where 'source:'turn{Ok(reply.pending())}
}
#[cfg(any(not(target_arch="wasm32"),target_env="p2"))]
impl OriginalHostClockProducer for OriginalSystemClock{
 fn read<'reply>(&mut self,c:&OriginalSystemClockContext<'_>,reply:OriginalClockReply<'reply>)->Result<OriginalClockReplyComplete<'reply>,ValueError>{self.validate_context(c)?;let reading=match self.origin{Some(origin)=>std::time::Instant::now().checked_duration_since(origin).map(|elapsed|elapsed.as_micros()),None=>{self.origin=Some(std::time::Instant::now());Some(0)}};Ok(reply.complete(Ok(reading)))}
 fn recover<'reply>(&mut self,_:&OriginalSystemClockContext<'_>,reply:OriginalClockReply<'reply>)->Result<OriginalClockReplyComplete<'reply>,ValueError>{Ok(reply.pending())}
}
#[cfg(any(not(target_arch="wasm32"),target_env="p2"))]
impl semio_framework_value_retirement::retirement::RetireOwned for OriginalSystemClock{
 type RetirementCursor=semio_framework_value_retirement::retirement::Leaf<(semio_framework_value_control_contract::OriginalSourceTurnIssuerIdentity,Option<std::time::Instant>)>;
 fn retirement_seed(self)->Self::RetirementCursor{semio_framework_value_retirement::retirement::leaf((self.issuer,self.origin))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(semio_framework_value_retirement::retirement::leaf_birth_bytes::<(semio_framework_value_control_contract::OriginalSourceTurnIssuerIdentity,Option<std::time::Instant>)>())}
 fn controlled_retirement_supported()->bool{true}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
