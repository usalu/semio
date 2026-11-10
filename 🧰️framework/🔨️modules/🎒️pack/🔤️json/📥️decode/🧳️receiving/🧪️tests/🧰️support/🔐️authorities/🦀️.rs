//! 🔐️ Finite original Source, Work and construction authorities that fund one test turn.
use semio_framework_value::{RetainedAllocationEffect,RetainedWorkOutcome,ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,retained_clone_grant_admits};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeReceivingFailure};
use semio_framework_value_testing_admission as fx;
use std::sync::Arc;
pub const WORK:u64=1<<20;
pub struct ConstructionTurn{original:Arc<std::sync::Mutex<RetainedCloneGrant>>,submitted:RetainedCloneProgress,turn:fx::OriginalSourceTurnIdentity}
impl fx::OriginalConstructionRecipient for ConstructionTurn{
type Context<'turn,'source>=() where 'source:'turn;
fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError> where 'source:'turn{Ok(())}
fn source_turn<'turn,'source>(&self,_:&())->Option<fx::OriginalSourceTurnIdentity> where 'source:'turn{Some(self.turn)}
fn grant<'a,'turn,'source>(&mut self,_:&mut (),quote:fx::OriginalConstructionGrant<'a>)->fx::OriginalConstructionGrantComplete<'a> where 'source:'turn{
let allowed=*self.original.lock().unwrap();
if !retained_clone_grant_admits(allowed,quote.demand()){return quote.complete(Err(ValueError::invariant("original JSON test turn does not admit construction demand")));}
quote.complete(Ok(allowed))
}
fn accept<'a,'turn,'source>(&mut self,_:&mut (),receipt:fx::OriginalConstructionAccept<'a>)->fx::OriginalConstructionAcceptComplete<'a> where 'source:'turn{let p=receipt.progress();let result=self.submitted.checked_add(p).map(|sum|self.submitted=sum);receipt.complete(result)}
fn submitted<'turn,'source>(&self,_:&())->RetainedCloneProgress where 'source:'turn{self.submitted}
}
pub struct ConstructionWork{turn:fx::OriginalSourceTurnIdentity}
impl fx::native_decoding::OriginalWorkRecipient for ConstructionWork{type Context<'turn,'source>=() where 'source:'turn;fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}fn source_turn<'turn,'source>(&self,_:&())->Option<fx::OriginalSourceTurnIdentity>where 'source:'turn{Some(self.turn)}fn settle_partial<'a,'turn,'source>(&mut self,_:&mut (),check:fx::native_decoding::OriginalWorkPartialCheck<'a>)->fx::native_decoding::OriginalWorkPartialCheckComplete<'a>where 'source:'turn{check.complete(Ok(()))}fn recover_partial<'a,'turn,'source>(&mut self,_:&mut (),check:fx::native_decoding::OriginalWorkPartialCheck<'a>)->fx::native_decoding::OriginalWorkPartialCheckComplete<'a>where 'source:'turn{check.complete(Ok(()))}fn admits<'a,'turn,'source>(&mut self,_:&mut (),quote:fx::native_decoding::OriginalWorkQuote<'a>,_:u64,_:u64)->fx::native_decoding::OriginalWorkQuoteComplete<'a>where 'source:'turn{quote.complete(Ok(true))}}
pub struct Authorities{pub turn:fx::OriginalSourceTurn,construction:fx::native_decoding::OriginalConstructionAuthority<ConstructionTurn>,work:fx::native_decoding::OriginalWorkAuthority<ConstructionWork>,grant:Arc<std::sync::Mutex<RetainedCloneGrant>>}
impl Authorities{
pub fn new()->Self{let issuer=fx::OriginalSourceTurnIssuer::new();let turn=issuer.issue().expect("fixture Source dispatch");let identity=turn.identity();let grant=Arc::new(std::sync::Mutex::new(RetainedCloneGrant::default()));let mut fresh=Self{turn,construction:fx::native_decoding::OriginalConstructionAuthority::new(ConstructionTurn{original:grant.clone(),submitted:Default::default(),turn:identity}),work:fx::native_decoding::OriginalWorkAuthority::new(ConstructionWork{turn:identity}),grant};let mut accept=|_|true;let mut control=NativeDecodeControl::new(1<<10,&mut accept);fresh.effect(&mut control,RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()},WORK,|_,_|());fresh}
pub fn effect<R>(&mut self,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant,work:u64,f:impl FnOnce(&mut NativeDecodeControl<'_>,&mut RetainedAllocationEffect<'_>)->R)->R{
*self.grant.lock().unwrap()=grant;
self.construction.with_loan(&mut (),|loan|self.work.with_loan(&mut (),|work_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(loan,|control|control.with_effect(grant,work,|control,effect|f(control,effect)))))).unwrap().unwrap().unwrap().unwrap()
}
pub fn receive<R>(&mut self,control:&mut NativeDecodeControl<'_>,g:RetainedCloneGrant,f:impl FnOnce(&mut NativeDecodeControl<'_>,fx::OriginalSourceTurnWitness<'_>)->RetainedWorkOutcome<R,NativeReceivingFailure>)->RetainedWorkOutcome<R,NativeReceivingFailure>{
*self.grant.lock().unwrap()=g;let witness=self.turn.witness();let mut work=0;
let result=self.construction.with_loan(&mut (),|loan|self.work.with_loan(&mut (),|work_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(loan,|control|{let turn=f(control,witness);work=turn.work;turn.result}))).unwrap()).unwrap();RetainedWorkOutcome{result,work}
}
}
