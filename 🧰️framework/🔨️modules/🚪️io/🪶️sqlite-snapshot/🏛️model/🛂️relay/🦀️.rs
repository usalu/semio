//! 🛂️ A stage-local allocation authority charges the SQL ledger once and relays every transaction to the original loan.
use super::{ownership_limit,ValueError,ValueRefusalKind};
use semio_framework_value_control_contract::{NativeAllocationAuthorityPort,NativeAllocationEvent,NativeAllocationReply,NativeAllocationReplyComplete,NativeAllocationRequest,NativeAllocationTransaction,NativeAllocationTransactionPhase,NativeSendAllocationLoan,OriginalSourceTurnIdentity};

/// 🔁️ The resident stage port; each stage lends its ledger and the original loan as the turn context.
pub(super) struct StageRelay;
/// 🧾️ Borrows the SQL ledger counters and the original parent for exactly one stage.
pub(super) struct StageRelayTurn<'turn>{pub(super) owned:&'turn mut usize,pub(super) forwarded:&'turn mut usize,pub(super) maximum:usize,pub(super) required:bool,pub(super) parent:Option<&'turn mut (dyn NativeSendAllocationLoan+'turn)>,slot:Option<NativeAllocationTransaction>,charge:usize}
impl<'turn> StageRelayTurn<'turn>{
 pub(super) fn new(owned:&'turn mut usize,forwarded:&'turn mut usize,maximum:usize,required:bool,parent:Option<&'turn mut (dyn NativeSendAllocationLoan+'turn)>)->Self{Self{owned,forwarded,maximum,required,parent,slot:None,charge:0}}
 pub(super) fn terminal_is_empty(&self)->bool{self.slot.is_none()}
 fn admit(&self,bytes:usize)->Result<NativeAllocationRequest,ValueError>{
  self.owned.checked_add(bytes).filter(|next|*next<=self.maximum).ok_or_else(||ownership_limit("relational SQLite resource limit: allocation bytes"))?;
  if bytes!=0&&self.required&&self.parent.is_none(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"SQLite backing requires the original receiving allocation port"))}
  let next=self.forwarded.checked_add(bytes).ok_or_else(||ownership_limit("relational SQLite resource limit: forwarded allocation bytes"))?;
  let maximum=self.forwarded.checked_add(self.maximum-*self.owned).ok_or_else(||ownership_limit("relational SQLite resource limit: forwarded allocation ceiling"))?;
  Ok(NativeAllocationRequest{bytes,owned_bytes:*self.forwarded,next_owned_bytes:next,maximum_bytes:maximum})
 }
 fn commit(&mut self)->Result<(),ValueError>{
  let bytes=std::mem::take(&mut self.charge);
  *self.owned=self.owned.checked_add(bytes).ok_or_else(||ownership_limit("relational SQLite resource limit: allocation bytes"))?;
  *self.forwarded=self.forwarded.checked_add(bytes).ok_or_else(||ownership_limit("relational SQLite resource limit: forwarded allocation bytes"))?;Ok(())
 }
 fn conclude<'reply>(&mut self,admission:bool,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,ValueError>{
  let invariant=||ValueError::literal(ValueRefusalKind::InvariantViolated,"SQLite stage relay lost its original transaction");
  let state=self.slot.as_mut().ok_or_else(invariant)?;
  let Some(verdict)=state.take_verdict()else{return Ok(reply.pending())};
  let settled=state.phase()==NativeAllocationTransactionPhase::Settled;
  if settled{self.parent.as_deref_mut().ok_or_else(invariant)?.finish(&mut self.slot)?}
  if admission&&verdict.is_ok(){self.commit()?}else if admission{self.charge=0}
  Ok(reply.complete(verdict))
 }
}
impl NativeAllocationAuthorityPort for StageRelay{
 type Context<'turn,'source>=StageRelayTurn<'turn> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&StageRelayTurn<'turn>)->Result<(),ValueError> where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&StageRelayTurn<'turn>)->Option<OriginalSourceTurnIdentity> where 'source:'turn{None}
 fn receive<'reply,'turn,'source>(&mut self,c:&mut StageRelayTurn<'turn>,event:NativeAllocationEvent,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,ValueError> where 'source:'turn{
  match event{
   NativeAllocationEvent::Admit(child)|NativeAllocationEvent::Quote(child)=>{
    let request=match c.admit(child.bytes){Ok(request)=>request,Err(error)=>return Ok(reply.complete(Err(error.into())))};
    c.charge=child.bytes;
    if c.parent.is_none(){let result=c.commit();return Ok(reply.complete(result.map_err(Into::into)))}
    let physical=matches!(event,NativeAllocationEvent::Quote(_));
    let slot=&mut c.slot;c.parent.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"SQLite stage relay lost its original loan"))?.begin(request,physical,slot)?;
    c.conclude(true,reply)
   },
   NativeAllocationEvent::Post{actual,..}=>{
    let Some(parent)=c.parent.as_deref_mut()else{return Ok(reply.complete(Ok(())))};
    let state=c.slot.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"SQLite stage relay post lost its quote"))?;parent.settle(state,Some(actual))?;
    c.conclude(false,reply)
   },
   NativeAllocationEvent::Cancel(_)=>{
    let Some(parent)=c.parent.as_deref_mut()else{return Ok(reply.complete(Ok(())))};
    let state=c.slot.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"SQLite stage relay cancel lost its quote"))?;parent.settle(state,None)?;
    c.conclude(false,reply)
   },
  }
 }
 fn recover<'reply,'turn,'source>(&mut self,c:&mut StageRelayTurn<'turn>,event:NativeAllocationEvent,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,ValueError> where 'source:'turn{
  let Some(parent)=c.parent.as_deref_mut()else{return Ok(reply.complete(Ok(())))};
  let state=c.slot.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"SQLite stage relay recovery lost its transaction"))?;
  if !parent.recover(state)?{return Ok(reply.pending())}
  c.conclude(matches!(event,NativeAllocationEvent::Admit(_)|NativeAllocationEvent::Quote(_)),reply)
 }
}

/// 🎟️ Admits one historical request through the original loan and returns its owned verdict.
pub(super) fn admit(parent:&mut (dyn NativeSendAllocationLoan+'_),request:NativeAllocationRequest)->Result<(),ValueError>{
 let invariant=||ValueError::literal(ValueRefusalKind::InvariantViolated,"SQLite original admission lost its transaction");
 let mut slot=None;parent.begin(request,false,&mut slot)?;
 let state=slot.as_mut().ok_or_else(invariant)?;
 let verdict=match state.take_verdict(){Some(verdict)=>verdict,None=>{if !parent.recover(state)?{return Err(invariant())}state.take_verdict().ok_or_else(invariant)?}};
 parent.finish(&mut slot)?;
 verdict.map_err(|refusal|refusal.into_owned().unwrap_or_else(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"original allocation refusal remains with its owner")))
}
