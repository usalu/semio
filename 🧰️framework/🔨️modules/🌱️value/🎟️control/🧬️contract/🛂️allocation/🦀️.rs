//! 🛂️ Original admission and physical allocation failure custody.
use crate::{ValueError,ValueRefusalKind};
/// 🧬️ Canonical allocation progress and refusal wire schema.
pub const NATIVE_ALLOCATION_SCHEMA:&str=include_str!("🧬️schema/🔣️.json");

/// 📬️ The original transaction lends its one reply slot to the actual producer.
pub struct NativeAllocationReply<'reply>{slot:&'reply mut Option<Result<(),NativeAllocationRefusal>>}
/// 🧾️ Only resolving the borrowed reply can complete this invocation.
pub struct NativeAllocationReplyComplete<'reply>{_slot:&'reply mut Option<Result<(),NativeAllocationRefusal>>}
impl<'reply> NativeAllocationReply<'reply>{pub fn complete(self,verdict:Result<(),NativeAllocationRefusal>)->NativeAllocationReplyComplete<'reply>{assert!(self.slot.is_none(),"original allocation reply occupied");*self.slot=Some(verdict);NativeAllocationReplyComplete{_slot:self.slot}}pub fn pending(self)->NativeAllocationReplyComplete<'reply>{assert!(self.slot.is_none(),"original allocation reply occupied");NativeAllocationReplyComplete{_slot:self.slot}}}
/// 🪪️ A stable original port validates each borrowed turn context before issuing events.
pub trait NativeAllocationAuthorityPort{type Context<'turn,'source>:?Sized where 'source:'turn;fn validate_context<'turn,'source>(&self,context:&Self::Context<'turn,'source>)->Result<(),ValueError> where 'source:'turn;fn source_turn<'turn,'source>(&self,context:&Self::Context<'turn,'source>)->Option<crate::OriginalSourceTurnIdentity> where 'source:'turn;fn receive<'reply,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,event:NativeAllocationEvent,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,ValueError> where 'source:'turn;fn recover<'reply,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,event:NativeAllocationEvent,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,ValueError> where 'source:'turn;}
/// 🧭️ Retains one real original port and its active transaction across short turn loans.
pub struct NativeAllocationAuthority<P:NativeAllocationAuthorityPort>{port:std::mem::ManuallyDrop<P>,identity:NativeAllocationIdentity,active:Option<NativeAllocationIdentity>}
impl<P:NativeAllocationAuthorityPort> NativeAllocationAuthority<P>{
 /// 🎟️ Owns the original port instead of accepting replacement callbacks on later turns.
 pub fn new(port:P)->Self{Self{port:std::mem::ManuallyDrop::new(port),identity:NativeAllocationIdentity::issue(),active:None}}
 /// 🪪️ Observes the nonforgeable identity of this original authority.
 pub fn identity(&self)->NativeAllocationIdentity{self.identity}
 /// ⏳️ Reports whether the original port still owns a transaction.
 pub fn has_pending_transaction(&self)->bool{self.active.is_some()}
 /// 🔎️ Observes the actual resident port without transferring or mutating its ownership.
 pub fn inspect_port<R>(&self,query:impl FnOnce(&P)->R)->R{query(&self.port)}
 /// 🫴️ Transfers the actual original port only after every transaction settles.
 pub fn into_port(self)->Result<P,Self>{if self.active.is_some(){return Err(self)}let mut owner=std::mem::ManuallyDrop::new(self);Ok(unsafe{std::mem::ManuallyDrop::take(&mut owner.port)})}
 /// 🫴️ Lends the same port with a context validated against its retained source binding.
 pub fn with_loan<'turn,'source,R>(&mut self,context:&mut P::Context<'turn,'source>,operation:impl FnOnce(&mut dyn NativeAllocationLoan)->R)->Result<R,ValueError> where 'source:'turn{self.port.validate_context(context)?;let source_turn=self.port.source_turn(context);let identity=self.identity;let active=&mut self.active;let port=&mut *self.port;let mut callback=|operation:NativeAllocationInvocation<'_>|{if let Err(error)=port.validate_context(context){return NativeAllocationInvocationResult::Context(error)}if port.source_turn(context)!=source_turn{return NativeAllocationInvocationResult::Context(ValueError::literal(ValueRefusalKind::InvariantViolated,"original allocation Source turn changed"))}match operation{NativeAllocationInvocation::Validate=>NativeAllocationInvocationResult::Validated,NativeAllocationInvocation::Event{event,entered,reply}=>{*entered=true;NativeAllocationInvocationResult::Event(port.receive(context,event,NativeAllocationReply{slot:reply}).map(drop))},NativeAllocationInvocation::Recover{event,reply}=>{NativeAllocationInvocationResult::Recover(port.recover(context,event,NativeAllocationReply{slot:reply}).map(drop))}}};let mut loan:NativeAllocationAuthorityLoan<'_>=NativeAllocationAuthorityLoan{identity,source_turn,active,callback:&mut callback};Ok(operation(&mut loan))}
}
#[cfg(not(target_arch="wasm32"))]
impl<P:NativeAllocationAuthorityPort+Send> NativeAllocationAuthority<P>{
 /// 🚚️ Lends the same validated original port for native Send execution.
 pub fn with_send_loan<'turn,'source,R>(&mut self,context:&mut P::Context<'turn,'source>,operation:impl FnOnce(&mut dyn NativeSendAllocationLoan)->R)->Result<R,ValueError> where 'source:'turn,P::Context<'turn,'source>:Send{self.port.validate_context(context)?;let source_turn=self.port.source_turn(context);let identity=self.identity;let active=&mut self.active;let port=&mut *self.port;let mut callback=|operation:NativeAllocationInvocation<'_>|{if let Err(error)=port.validate_context(context){return NativeAllocationInvocationResult::Context(error)}if port.source_turn(context)!=source_turn{return NativeAllocationInvocationResult::Context(ValueError::literal(ValueRefusalKind::InvariantViolated,"original allocation Source turn changed"))}match operation{NativeAllocationInvocation::Validate=>NativeAllocationInvocationResult::Validated,NativeAllocationInvocation::Event{event,entered,reply}=>{*entered=true;NativeAllocationInvocationResult::Event(port.receive(context,event,NativeAllocationReply{slot:reply}).map(drop))},NativeAllocationInvocation::Recover{event,reply}=>{NativeAllocationInvocationResult::Recover(port.recover(context,event,NativeAllocationReply{slot:reply}).map(drop))}}};let mut loan:NativeAllocationAuthorityLoan<'_,NativeAllocationSendCallback<'_>>=NativeAllocationAuthorityLoan{identity,source_turn,active,callback:&mut callback};Ok(operation(&mut loan))}
}
#[cfg(target_arch="wasm32")]
impl<P:NativeAllocationAuthorityPort> NativeAllocationAuthority<P>{
 /// 🌐️ Lends the same validated original port for the guest executor.
 pub fn with_send_loan<'turn,'source,R>(&mut self,context:&mut P::Context<'turn,'source>,operation:impl FnOnce(&mut dyn NativeSendAllocationLoan)->R)->Result<R,ValueError> where 'source:'turn{self.port.validate_context(context)?;let source_turn=self.port.source_turn(context);let identity=self.identity;let active=&mut self.active;let port=&mut *self.port;let mut callback=|invocation:NativeAllocationInvocation<'_>|{if let Err(error)=port.validate_context(context){return NativeAllocationInvocationResult::Context(error)}if port.source_turn(context)!=source_turn{return NativeAllocationInvocationResult::Context(ValueError::literal(ValueRefusalKind::InvariantViolated,"original allocation Source turn changed"))}match invocation{NativeAllocationInvocation::Validate=>NativeAllocationInvocationResult::Validated,NativeAllocationInvocation::Event{event,entered,reply}=>{*entered=true;NativeAllocationInvocationResult::Event(port.receive(context,event,NativeAllocationReply{slot:reply}).map(drop))},NativeAllocationInvocation::Recover{event,reply}=>{NativeAllocationInvocationResult::Recover(port.recover(context,event,NativeAllocationReply{slot:reply}).map(drop))}}};let mut loan:NativeAllocationAuthorityLoan<'_,NativeAllocationSendCallback<'_>>=NativeAllocationAuthorityLoan{identity,source_turn,active,callback:&mut callback};Ok(operation(&mut loan))}
}
impl<P:NativeAllocationAuthorityPort> Drop for NativeAllocationAuthority<P>{fn drop(&mut self){assert!(std::thread::panicking(),"original allocation authority requires explicit owner transfer")}}
/// 🚦️ Records the callback boundary and whether its exact physical quote remains live.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum NativeAllocationTransactionPhase{AdmitStarted,QuoteStarted,QuoteAccepted,PostStarted,CancelStarted,Settled}
/// 🧳️ Caller-owned transaction stores the exact request and the original callback verdict.
pub struct NativeAllocationTransaction{authority:NativeAllocationIdentity,identity:NativeAllocationIdentity,source_turn:Option<crate::OriginalSourceTurnIdentity>,callback_invoked:bool,request:NativeAllocationRequest,event:NativeAllocationEvent,phase:NativeAllocationTransactionPhase,verdict:Option<Result<(),NativeAllocationRefusal>>,finished:bool}
impl NativeAllocationTransaction{
 /// 🚦️ Observes the original event callback boundary.
 pub fn phase(&self)->NativeAllocationTransactionPhase{self.phase}
 /// 📏️ Borrows the unchanged original requested admission.
 pub fn request(&self)->NativeAllocationRequest{self.request}
 /// 🪪️ Borrows the opaque issuer of this exact transaction.
 pub fn authority_identity(&self)->NativeAllocationIdentity{self.authority}
 /// 🧾️ Observes the exact in-flight original event.
 pub fn event(&self)->NativeAllocationEvent{self.event}
 /// 🫴️ Transfers the recorded original verdict exactly once.
 pub fn take_verdict(&mut self)->Option<Result<(),NativeAllocationRefusal>>{if matches!(self.phase,NativeAllocationTransactionPhase::QuoteAccepted|NativeAllocationTransactionPhase::Settled){self.verdict.take()}else{None}}
 /// 🏁️ Reports that no original callback or verdict remains in this transaction.
 pub fn terminal_is_empty(&self)->bool{self.finished&&self.phase==NativeAllocationTransactionPhase::Settled&&self.verdict.is_none()}
}
impl Drop for NativeAllocationTransaction{fn drop(&mut self){assert!(std::thread::panicking()||self.finished&&self.phase==NativeAllocationTransactionPhase::Settled&&self.verdict.is_none(),"original allocation transaction remains unsettled")}}
/// 🧭️ Internal operation against one validated original port.
pub enum NativeAllocationInvocation<'entry>{Validate,Event{event:NativeAllocationEvent,entered:&'entry mut bool,reply:&'entry mut Option<Result<(),NativeAllocationRefusal>>},Recover{event:NativeAllocationEvent,reply:&'entry mut Option<Result<(),NativeAllocationRefusal>>}}
/// 🧾️ Internal reply retains the real event verdict or its original recovery.
pub enum NativeAllocationInvocationResult{Validated,Event(Result<(),ValueError>),Recover(Result<(),ValueError>),Context(ValueError)}
/// 🎟️ Borrows one validated original port without exposing its callback.
/// 🚚️ Native executors keep the same original callback across Send futures.
#[cfg(not(target_arch="wasm32"))]
pub type NativeAllocationSendCallback<'a>=dyn for<'entry> FnMut(NativeAllocationInvocation<'entry>)->NativeAllocationInvocationResult+Send+'a;
/// 🌐️ Guest executors retain the same original local callback.
#[cfg(target_arch="wasm32")]
pub type NativeAllocationSendCallback<'a>=dyn for<'entry> FnMut(NativeAllocationInvocation<'entry>)->NativeAllocationInvocationResult+'a;
pub struct NativeAllocationAuthorityLoan<'a,F:?Sized=dyn for<'entry> FnMut(NativeAllocationInvocation<'entry>)->NativeAllocationInvocationResult+'a>{identity:NativeAllocationIdentity,source_turn:Option<crate::OriginalSourceTurnIdentity>,active:&'a mut Option<NativeAllocationIdentity>,callback:&'a mut F}
/// 🚚️ A short native Send loan from the actual owned original port.
mod sealed_allocation_loan{pub trait Sealed{}}
/// 🧭️ A sealed loan exposes only validated original allocation transitions.
pub trait NativeAllocationLoan:sealed_allocation_loan::Sealed{
 fn identity(&self)->NativeAllocationIdentity;
 fn source_turn(&self)->Option<crate::OriginalSourceTurnIdentity>;
 fn validate_context(&mut self)->Result<(),ValueError>;
 fn matches_transaction(&self,transaction:Option<&NativeAllocationTransaction>)->bool;
 fn begin(&mut self,request:NativeAllocationRequest,physical:bool,slot:&mut Option<NativeAllocationTransaction>)->Result<(),ValueError>;
 fn settle(&mut self,state:&mut NativeAllocationTransaction,actual:Option<usize>)->Result<(),ValueError>;
 fn recover(&mut self,state:&mut NativeAllocationTransaction)->Result<bool,ValueError>;
 fn finish(&mut self,slot:&mut Option<NativeAllocationTransaction>)->Result<(),ValueError>;
}
/// 🚚️ Native Send execution borrows a sealed loan from the same original authority.
#[cfg(not(target_arch="wasm32"))]
pub trait NativeSendAllocationLoan:NativeAllocationLoan+Send{}
/// 🌐️ Guest execution borrows a sealed local original authority.
#[cfg(target_arch="wasm32")]
pub trait NativeSendAllocationLoan:NativeAllocationLoan{}

impl<F:for<'entry> FnMut(NativeAllocationInvocation<'entry>)->NativeAllocationInvocationResult+?Sized> NativeAllocationAuthorityLoan<'_,F>{
 /// 🔗️ Reborrows the same owned original port without replacing its identity.
 pub fn reborrow(&mut self)->NativeAllocationAuthorityLoan<'_,F>{NativeAllocationAuthorityLoan{identity:self.identity,source_turn:self.source_turn,active:&mut *self.active,callback:&mut *self.callback}}
 /// 🪪️ Identifies the actual owned original port.
 pub fn identity(&self)->NativeAllocationIdentity{self.identity}
 fn validate_current(&mut self)->Result<(),ValueError>{match (self.callback)(NativeAllocationInvocation::Validate){NativeAllocationInvocationResult::Validated=>Ok(()),NativeAllocationInvocationResult::Context(error)=>Err(error),_=>unreachable!("owned authority preserves invocation kind")}}
 /// 🎟️ Starts one historical admission or physical quote in caller-owned custody.
 pub fn begin(&mut self,request:NativeAllocationRequest,physical:bool,slot:&mut Option<NativeAllocationTransaction>)->Result<(),ValueError>{self.validate_current()?;if self.active.is_some()||slot.is_some()||request.owned_bytes.checked_add(request.bytes)!=Some(request.next_owned_bytes)||request.next_owned_bytes>request.maximum_bytes{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original allocation requires a valid unoccupied transaction"))}let identity=NativeAllocationIdentity::issue();*self.active=Some(identity);*slot=Some(NativeAllocationTransaction{authority:self.identity,identity,source_turn:self.source_turn,callback_invoked:false,request,event:if physical{NativeAllocationEvent::Quote(request)}else{NativeAllocationEvent::Admit(request)},phase:if physical{NativeAllocationTransactionPhase::QuoteStarted}else{NativeAllocationTransactionPhase::AdmitStarted},verdict:None,finished:false});let state=slot.as_mut().expect("original transaction present");let event=state.event;match (self.callback)(NativeAllocationInvocation::Event{event,entered:&mut state.callback_invoked,reply:&mut state.verdict}){NativeAllocationInvocationResult::Event(result)=>{result?;if let Some(result)=state.verdict.take(){self.record(state,result)}},NativeAllocationInvocationResult::Context(error)=>return Err(error),NativeAllocationInvocationResult::Recover(_)|NativeAllocationInvocationResult::Validated=>unreachable!("owned authority preserves invocation kind")}Ok(())}
 /// 🧾️ Starts the actual post or a pre-effect cancellation for the quoted request.
 pub fn settle(&mut self,state:&mut NativeAllocationTransaction,actual:Option<usize>)->Result<(),ValueError>{self.validate_current()?;if state.authority!=self.identity||state.source_turn!=self.source_turn||*self.active!=Some(state.identity)||state.phase!=NativeAllocationTransactionPhase::QuoteAccepted||state.verdict.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original quote must be accepted before settlement"))}let event=match actual{Some(actual)=>{state.phase=NativeAllocationTransactionPhase::PostStarted;NativeAllocationEvent::Post{request:state.request,actual}},None=>{state.phase=NativeAllocationTransactionPhase::CancelStarted;NativeAllocationEvent::Cancel(state.request)}};state.event=event;state.callback_invoked=false;match (self.callback)(NativeAllocationInvocation::Event{event,entered:&mut state.callback_invoked,reply:&mut state.verdict}){NativeAllocationInvocationResult::Event(result)=>{result?;if let Some(result)=state.verdict.take(){self.record(state,result)}},NativeAllocationInvocationResult::Context(error)=>return Err(error),NativeAllocationInvocationResult::Recover(_)|NativeAllocationInvocationResult::Validated=>unreachable!("owned authority preserves invocation kind")}Ok(())}
 /// 🪪️ Records the real verdict after an interrupted original callback.
 pub fn recover(&mut self,state:&mut NativeAllocationTransaction)->Result<bool,ValueError>{self.validate_current()?;if state.authority!=self.identity||state.source_turn!=self.source_turn||*self.active!=Some(state.identity)||!matches!(state.phase,NativeAllocationTransactionPhase::AdmitStarted|NativeAllocationTransactionPhase::QuoteStarted|NativeAllocationTransactionPhase::PostStarted|NativeAllocationTransactionPhase::CancelStarted){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original callback requires its unchanged authority"))}if let Some(verdict)=state.verdict.take(){self.record(state,verdict);return Ok(true)}let event=state.event;let result=if !state.callback_invoked{(self.callback)(NativeAllocationInvocation::Event{event,entered:&mut state.callback_invoked,reply:&mut state.verdict})}else{(self.callback)(NativeAllocationInvocation::Recover{event,reply:&mut state.verdict})};match result{NativeAllocationInvocationResult::Event(result)|NativeAllocationInvocationResult::Recover(result)=>{result?;if let Some(verdict)=state.verdict.take(){self.record(state,verdict);Ok(true)}else{Ok(false)}},NativeAllocationInvocationResult::Context(error)=>Err(error),NativeAllocationInvocationResult::Validated=>unreachable!("owned authority preserves invocation kind")}}
 fn record(&mut self,state:&mut NativeAllocationTransaction,result:Result<(),NativeAllocationRefusal>){let quote=state.phase==NativeAllocationTransactionPhase::QuoteStarted&&result.is_ok();state.phase=if quote{NativeAllocationTransactionPhase::QuoteAccepted}else{NativeAllocationTransactionPhase::Settled};state.verdict=Some(result)}
 /// 🏁️ Releases a settled transaction only after its verdict was transferred.
 pub fn finish(&mut self,slot:&mut Option<NativeAllocationTransaction>)->Result<(),ValueError>{self.validate_current()?;let Some(state)=slot.as_ref()else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original transaction is absent"))};if state.authority!=self.identity||state.source_turn!=self.source_turn||state.phase!=NativeAllocationTransactionPhase::Settled||state.verdict.is_some()||*self.active!=Some(state.identity){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original transaction remains unsettled"))}slot.as_mut().expect("original transaction present").finished=true;*self.active=None;drop(slot.take());Ok(())}
}

impl<F:for<'entry> FnMut(NativeAllocationInvocation<'entry>)->NativeAllocationInvocationResult+?Sized> sealed_allocation_loan::Sealed for NativeAllocationAuthorityLoan<'_,F>{}
impl<F:for<'entry> FnMut(NativeAllocationInvocation<'entry>)->NativeAllocationInvocationResult+?Sized> NativeAllocationLoan for NativeAllocationAuthorityLoan<'_,F>{fn identity(&self)->NativeAllocationIdentity{NativeAllocationAuthorityLoan::identity(self)}fn source_turn(&self)->Option<crate::OriginalSourceTurnIdentity>{self.source_turn}fn validate_context(&mut self)->Result<(),ValueError>{self.validate_current()}fn matches_transaction(&self,transaction:Option<&NativeAllocationTransaction>)->bool{match (*self.active,transaction){(None,None)=>true,(Some(active),Some(transaction))=>transaction.authority==self.identity&&transaction.source_turn==self.source_turn&&transaction.identity==active&&!transaction.finished,_=>false}}fn begin(&mut self,request:NativeAllocationRequest,physical:bool,slot:&mut Option<NativeAllocationTransaction>)->Result<(),ValueError>{NativeAllocationAuthorityLoan::begin(self,request,physical,slot)}fn settle(&mut self,state:&mut NativeAllocationTransaction,actual:Option<usize>)->Result<(),ValueError>{NativeAllocationAuthorityLoan::settle(self,state,actual)}fn recover(&mut self,state:&mut NativeAllocationTransaction)->Result<bool,ValueError>{NativeAllocationAuthorityLoan::recover(self,state)}fn finish(&mut self,slot:&mut Option<NativeAllocationTransaction>)->Result<(),ValueError>{NativeAllocationAuthorityLoan::finish(self,slot)}}
#[cfg(not(target_arch="wasm32"))]
impl<F:for<'entry> FnMut(NativeAllocationInvocation<'entry>)->NativeAllocationInvocationResult+Send+?Sized> NativeSendAllocationLoan for NativeAllocationAuthorityLoan<'_,F>{}
#[cfg(target_arch="wasm32")]
impl<F:for<'entry> FnMut(NativeAllocationInvocation<'entry>)->NativeAllocationInvocationResult+?Sized> NativeSendAllocationLoan for NativeAllocationAuthorityLoan<'_,F>{}

/// 📏️ Separates historical admission from a performed physical allocation.
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct NativeAllocationProgress{pub prior_admitted_bytes:Option<usize>,pub requested_bytes:Option<usize>,pub maximum_bytes:Option<usize>,pub admitted_bytes:usize,pub actual_capacity_bytes:Option<usize>}

impl NativeAllocationProgress{
 /// 🧩️ Verifies that an original allocation request is either complete or absent.
 pub fn request_is_complete(&self)->bool{self.prior_admitted_bytes.is_some()==self.requested_bytes.is_some()&&self.requested_bytes.is_some()==self.maximum_bytes.is_some()}
}

/// 🧳️ Retains each independently invoked allocation and receipt refusal inline.
#[derive(Debug,Default,PartialEq)]
pub struct NativeAllocationFailure{pub primary:Option<ValueError>,pub admission:Option<ValueError>,pub forwarded_admission:Option<NativeAllocationRefusal>,pub scoped_admission:Option<NativeAllocationRefusal>,pub forwarded_quote:Option<NativeAllocationRefusal>,pub scoped_quote:Option<NativeAllocationRefusal>,pub forwarded_post:Option<NativeAllocationRefusal>,pub scoped_post:Option<NativeAllocationRefusal>,pub forwarded_cancel:Option<NativeAllocationRefusal>,pub scoped_cancel:Option<NativeAllocationRefusal>,pub receipt:Option<ValueError>,pub extent:Option<ValueError>,pub progress:NativeAllocationProgress}
impl NativeAllocationFailure{
 /// 🔎️ Reports whether any original refusal is present.
 pub fn is_empty(&self)->bool{self.primary.is_none()&&self.admission.is_none()&&self.forwarded_admission.is_none()&&self.scoped_admission.is_none()&&self.forwarded_quote.is_none()&&self.scoped_quote.is_none()&&self.forwarded_post.is_none()&&self.scoped_post.is_none()&&self.forwarded_cancel.is_none()&&self.scoped_cancel.is_none()&&self.receipt.is_none()&&self.extent.is_none()}
}

impl From<ValueError> for NativeAllocationFailure{fn from(primary:ValueError)->Self{Self{primary:Some(primary),..Default::default()}}}
impl std::fmt::Display for NativeAllocationFailure{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{if let Some(error)=self.primary.as_ref().or(self.admission.as_ref()).or(self.receipt.as_ref()).or(self.extent.as_ref()){return std::fmt::Display::fmt(error,f)}for refusal in [&self.forwarded_admission,&self.scoped_admission,&self.forwarded_quote,&self.scoped_quote,&self.forwarded_post,&self.scoped_post,&self.forwarded_cancel,&self.scoped_cancel]{match refusal{Some(NativeAllocationRefusal::Owned(error))=>return std::fmt::Display::fmt(error,f),Some(NativeAllocationRefusal::Retained(_))=>return f.write_str("original allocation refusal remains with its owner"),None=>{}}}f.write_str("native allocation failed without a cause")}}
impl std::error::Error for NativeAllocationFailure{}

/// 🪪️ Identifies one original allocation authority without exposing a forgeable number.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct NativeAllocationIdentity(u64);
static ALLOCATION_IDENTITIES:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1);
impl NativeAllocationIdentity{
 /// 🎟️ Issues a distinct identity for one original allocation issuer.
 pub fn issue()->Self{Self(ALLOCATION_IDENTITIES.try_update(std::sync::atomic::Ordering::Relaxed,std::sync::atomic::Ordering::Relaxed,|id|id.checked_add(1)).expect("allocation issuer identity exhausted"))}
}

/// 🎫️ Identifies a retained original refusal without duplicating its owner.
#[derive(Debug,PartialEq,Eq)]
pub struct NativeAllocationRefusalHandle{issuer:NativeAllocationIdentity,generation:u64}
/// 🧳️ Distinguishes a directly owned refusal from one held by its original mediator.
#[derive(Debug,PartialEq)]
pub enum NativeAllocationRefusal{Owned(ValueError),Retained(NativeAllocationRefusalHandle)}
impl From<ValueError> for NativeAllocationRefusal{fn from(error:ValueError)->Self{Self::Owned(error)}}
impl NativeAllocationRefusal{
 /// 🔎️ Borrows a directly owned refusal without resolving a retained handle.
 pub fn owned(&self)->Option<&ValueError>{match self{Self::Owned(error)=>Some(error),Self::Retained(_)=>None}}
 /// 🫴️ Transfers a directly owned refusal or returns the unresolved original handle.
 pub fn into_owned(self)->Result<ValueError,NativeAllocationRefusalHandle>{match self{Self::Owned(error)=>Ok(error),Self::Retained(handle)=>Err(handle)}}
}
/// 🗃️ Retains one original refusal until the exact issued handle returns.
pub struct NativeAllocationRefusalVault{issuer:NativeAllocationIdentity,generation:u64,refusal:Option<ValueError>}
impl Default for NativeAllocationRefusalVault{fn default()->Self{Self{issuer:NativeAllocationIdentity::issue(),generation:0,refusal:None}}}
impl NativeAllocationRefusalVault{
 /// 🏁️ Reports whether the original refusal has been transferred.
 pub fn terminal_is_empty(&self)->bool{self.refusal.is_none()}
 /// 🎟️ Reports whether another unique refusal handle can be issued.
 pub fn can_retain(&self)->bool{self.refusal.is_none()&&self.generation.checked_add(1).is_some()}
 /// 🎟️ Retains one refusal and issues its unique live handle.
 pub fn retain(&mut self,error:ValueError)->Result<NativeAllocationRefusalHandle,ValueError>{if self.refusal.is_some(){return Err(error)}let Some(generation)=self.generation.checked_add(1)else{return Err(error)};self.generation=generation;self.refusal=Some(error);Ok(NativeAllocationRefusalHandle{issuer:self.issuer,generation})}
 /// 🫴️ Transfers the exact refusal once when its unchanged handle returns.
 pub fn take(&mut self,handle:NativeAllocationRefusalHandle)->Result<ValueError,NativeAllocationRefusalHandle>{if self.refusal.is_none()||handle.issuer!=self.issuer||handle.generation!=self.generation{return Err(handle)}Ok(self.refusal.take().expect("validated refusal owner"))}
}
impl Drop for NativeAllocationRefusalVault{fn drop(&mut self){assert!(std::thread::panicking()||self.refusal.is_none(),"original allocation refusal remains retained")}}

/// 📦️ Describes one cumulative requested admission and its ceiling.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct NativeAllocationRequest{pub bytes:usize,pub owned_bytes:usize,pub next_owned_bytes:usize,pub maximum_bytes:usize}
/// 🧾️ Separates historical admission from physical quote and settlement.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum NativeAllocationEvent{Admit(NativeAllocationRequest),Quote(NativeAllocationRequest),Post{request:NativeAllocationRequest,actual:usize},Cancel(NativeAllocationRequest)}

/// 🎟️ Tracks per-hop physical quotes and actual posts without changing cumulative admission.
/// 📏️ An accepted quote bounds prior spend by a usize grant; one usize receipt fits the wider total on supported targets.
#[derive(Default)]
pub struct NativeAllocationBudget{pending:Option<(NativeAllocationRequest,crate::RetainedCloneGrant)>,spent:u128}
impl NativeAllocationBudget{
 /// 🔁️ Starts a new caller hop only after its earlier quote settled.
 pub fn begin_hop(&mut self){if self.pending.is_none(){self.spent=0}}
 /// 📊️ Borrows actual physical extent observed in this hop.
 pub fn spent(&self)->u128{self.spent}
 /// 🧾️ Borrows the unchanged pending request and original grant.
 pub fn pending(&self)->Option<(NativeAllocationRequest,crate::RetainedCloneGrant)>{self.pending}
 /// 🏁️ Reports that no original allocation callback remains pending.
 pub fn terminal_is_empty(&self)->bool{self.pending.is_none()}
 pub fn receive(&mut self,event:NativeAllocationEvent,grant:crate::RetainedCloneGrant,may_quote:bool)->Result<(),ValueError>{
  match event{
   NativeAllocationEvent::Admit(request)|NativeAllocationEvent::Quote(request)=>{
    if self.pending.is_some(){return Err(budget_invariant("original native allocation already retains a quote"))}
    if request.owned_bytes.checked_add(request.bytes)!=Some(request.next_owned_bytes)||request.next_owned_bytes>request.maximum_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original native allocation quote changes its cumulative ledger"))}
    if !may_quote{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original native decode caller yielded"))}
    if self.spent.checked_add(request.bytes as u128).is_none_or(|bytes|bytes>grant.maximum_capacity_bytes as u128){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original native decode reservation exceeds actual turn"))}
    if matches!(event,NativeAllocationEvent::Quote(_)){self.pending=Some((request,grant))}Ok(())
   },
   NativeAllocationEvent::Post{request,actual}=>{
    let Some((original,issued))=self.pending else{return Err(budget_invariant("original native allocation post has no quote"))};
    if original!=request{return Err(budget_invariant("original native allocation post changed its quoted request"))}
    self.pending=None;
    self.spent=self.spent.checked_add(actual as u128).expect("admitted native physical extents fit the wider accumulator");
    let changed=issued!=grant;let excessive=actual>request.bytes||self.spent>issued.maximum_capacity_bytes as u128;
    if changed&&excessive{return Err(budget_invariant("original grant changed and physical allocation exceeded its quote"))}if changed{return Err(budget_invariant("original native allocation post changed its quoted grant"))}if excessive{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original native physical allocation exceeds its quote"))}Ok(())
   },
   NativeAllocationEvent::Cancel(request)=>{
    let Some((original,issued))=self.pending else{return Err(budget_invariant("original native allocation cancellation has no quote"))};
    if original!=request{return Err(budget_invariant("original native allocation cancellation changed its quoted request"))}
    self.pending=None;if issued!=grant{return Err(budget_invariant("original native allocation cancellation changed its quoted grant"))}Ok(())
   }
  }
 }
}
impl Drop for NativeAllocationBudget{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original native allocation quote remains unsettled")}}
fn budget_invariant(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
