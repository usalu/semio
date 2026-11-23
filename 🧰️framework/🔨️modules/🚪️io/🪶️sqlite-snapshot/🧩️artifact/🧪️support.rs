//! 🧪️ Shared original allocation, Work and construction producers for the SQLite snapshot laws.
use semio_framework_value_testing_admission::*;
use semio_framework_value::RetainedCloneProgress;
pub struct Port<F>(pub F);
impl<F:FnMut(NativeAllocationEvent)->Result<(),ValueError>+Send> NativeAllocationAuthorityPort for Port<F>{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn receive<'reply,'turn,'source>(&mut self,_:&mut (),event:NativeAllocationEvent,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,ValueError>where 'source:'turn{Ok(reply.complete((self.0)(event).map_err(Into::into)))}
 fn recover<'reply,'turn,'source>(&mut self,_:&mut (),_:NativeAllocationEvent,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,ValueError>where 'source:'turn{Ok(reply.pending())}
}
pub struct Work(pub u64);
impl OriginalWorkRecipient for Work{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply>where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|total|total<=self.0)))}
 fn settle_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
 fn recover_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
}
pub struct Recipient<F>{pub grant:RetainedCloneGrant,pub submitted:RetainedCloneProgress,pub refuse:F}
impl<F:FnMut(RetainedCloneProgress)->Result<(),ValueError>+Send> OriginalConstructionRecipient for Recipient<F>{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn submitted<'turn,'source>(&self,_:&())->RetainedCloneProgress where 'source:'turn{self.submitted}
 fn grant<'a,'turn,'source>(&mut self,_:&mut (),quote:OriginalConstructionGrant<'a>)->OriginalConstructionGrantComplete<'a>where 'source:'turn{quote.complete(Ok(self.grant))}
 fn accept<'a,'turn,'source>(&mut self,_:&mut (),receipt:OriginalConstructionAccept<'a>)->OriginalConstructionAcceptComplete<'a>where 'source:'turn{assert_eq!(receipt.grant(),self.grant);let progress=receipt.progress();let result=self.submitted.checked_add(progress).map(|total|self.submitted=total).and_then(|_|(self.refuse)(progress));receipt.complete(result)}
}
pub fn close<T:semio_framework_value::retirement::RetireOwned>(value:T,grant:RetainedCloneGrant){
 let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::new(value).map_err(|(error,_)|error).unwrap();
 let mut bank=semio_framework_value_testing_admission::EffectBank::new(semio_framework_value_testing_admission::BankWallet::new(grant,1<<40));
 for _ in 0..65536{if owner.terminal_is_empty(){break}let turn=bank.run_outcome(grant,1<<20,|effect|{let mut turn=None;with_original_invocation(None,effect,&mut |effect|turn=Some(match owner.step(65536,effect){RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("fixture close stayed pending")})).unwrap();turn.expect("fixture close ran")});turn.result.unwrap();bank.take_admitted();}
 assert!(owner.terminal_is_empty());
}
pub fn settle(allocation:&mut semio_framework_io_control::NativeSnapshotAllocation){if let semio_framework_io_control::NativeSnapshotAllocation::Encode(encode)=allocation{encode.state.settle_claimed_work().unwrap()}}
