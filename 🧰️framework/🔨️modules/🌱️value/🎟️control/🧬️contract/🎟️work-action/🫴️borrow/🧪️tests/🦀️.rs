//! 🧪️ Borrowing the actual original Work owner preserves its Source, receipt and proof across erasure.
use crate::*;
use std::sync::{Arc,atomic::{AtomicUsize,Ordering}};
struct Source{issuer:OriginalSourceTurnIssuer,turn:OriginalSourceTurnIdentity,counter:Arc<AtomicUsize>,checks:Arc<AtomicUsize>}
impl OriginalWorkRecipient for Source{
 type Context<'turn,'source>=&'turn OriginalSourceTurn where 'source:'turn;
 fn source_turn<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Option<OriginalSourceTurnIdentity>where 'source:'turn{Some(c.identity())}
 fn validate_context<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Result<(),ValueError>where 'source:'turn{self.issuer.validate(c)?;if c.identity()==self.turn{Ok(())}else{Err(ValueError::invariant("original borrowed Work Source changed"))}}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply>where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|n|n<=1)))}
 fn settle_partial<'reply,'turn,'source>(&mut self,c:&mut Self::Context<'turn,'source>,check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{assert_eq!(self.counter.load(Ordering::SeqCst),1);self.checks.fetch_add(1,Ordering::SeqCst);check.complete(self.validate_context(c))}
 fn recover_partial<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.pending()}
}
fn law()->serde_json::Value{serde_json::from_str(include_str!("borrow.json")).unwrap()}
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()}}
#[test]
fn borrowed_original_scope_claims_actual_work_before_source_recovery(){
 let law=law();let issuer=OriginalSourceTurnIssuer::new();let turn=issuer.issue().unwrap();let counter=Arc::new(AtomicUsize::new(0));let checks=Arc::new(AtomicUsize::new(0));let mut authority=OriginalWorkAuthority::new(Source{issuer,turn:turn.identity(),counter:counter.clone(),checks:checks.clone()});let mut context=&turn;
 authority.with_loan(&mut context,|loan|{let mut loan=loan.reborrow();let mut work=OriginalWork::empty();work.recipient=Some(&mut loan);let(mut action,mut receipt,mut claim,mut partial,mut proof,mut marker)=(None,None,None,None,None,None);
 {let mut original=OriginalWorkBorrow::new(&mut work);assert_eq!(original.prepare(1,&mut action).unwrap(),Some(true));let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{let mut scope=original.performed_scope(action,&mut receipt,&mut claim).unwrap();let mut effect=scope.effect(grant(),1,&mut marker).unwrap();assert_eq!(effect.work_source_turn(),Some(turn.identity()));assert!(effect.performed(1).unwrap());counter.fetch_add(1,Ordering::SeqCst);panic!("original borrowed producer interrupted")}));assert!(interrupted.is_err());assert_eq!(original.actual(),u128::from(law["scope"]["claimedBeforeSource"].as_u64().unwrap()));assert_eq!(claim.take().unwrap().unwrap(),original.actual());assert!(receipt.is_none());assert_eq!(checks.load(Ordering::SeqCst),0);assert!(original.partial_pending(action).unwrap());assert!(original.settle_partial_into(action,&mut partial).unwrap());let reply=partial.take().unwrap();assert_eq!(reply.total(),1);assert!(reply.source_refusal().is_none());assert!(original.commit(1,action,&mut proof).unwrap());assert!(original.validates(proof.as_ref().unwrap(),1).unwrap());original.finish(&mut proof,&mut action).unwrap();}
 assert_eq!(work.actual(),1);assert_eq!(marker,None);assert_eq!(checks.load(Ordering::SeqCst),law["scope"]["checks"].as_u64().unwrap()as usize);assert!(action.is_none()&&proof.is_none()&&!work.has_live_actions());
 }).unwrap();assert!(authority.into_recipient().is_ok());
}
#[test]
fn borrowed_original_scope_rejects_a_reconstructed_work_window_before_mutation(){
 let law=law();let issuer=OriginalSourceTurnIssuer::new();let turn=issuer.issue().unwrap();let counter=Arc::new(AtomicUsize::new(0));let checks=Arc::new(AtomicUsize::new(0));let mut authority=OriginalWorkAuthority::new(Source{issuer,turn:turn.identity(),counter,checks});let mut context=&turn;
 authority.with_loan(&mut context,|loan|{let mut loan=loan.reborrow();let mut work=OriginalWork::empty();work.recipient=Some(&mut loan);let(mut action,mut receipt,mut claim,mut marker)=(None,None,None,None);let mut original=OriginalWorkBorrow::new(&mut work);assert_eq!(original.prepare(law["window"]["quoted"].as_u64().unwrap(),&mut action).unwrap(),Some(true));{let mut scope=original.performed_scope(action,&mut receipt,&mut claim).unwrap();assert_eq!(scope.effect(grant(),law["window"]["requested"].as_u64().unwrap(),&mut marker).err().unwrap().kind,ValueRefusalKind::WorkLimit);}assert_eq!(claim.take().unwrap().unwrap(),u128::from(law["window"]["performed"].as_u64().unwrap()));assert!(!original.partial_pending(action).unwrap());assert_eq!(marker,None);original.cancel(&mut action).unwrap();assert!(action.is_none());
 }).unwrap();assert!(authority.into_recipient().is_ok());
}
#[test]
fn borrowed_original_scope_keeps_an_occupied_owned_claim_unchanged(){
 let law=law();let issuer=OriginalSourceTurnIssuer::new();let turn=issuer.issue().unwrap();let counter=Arc::new(AtomicUsize::new(0));let checks=Arc::new(AtomicUsize::new(0));let mut authority=OriginalWorkAuthority::new(Source{issuer,turn:turn.identity(),counter,checks});let mut context=&turn;let error=ValueError::new(ValueRefusalKind::Canceled,"actual original claim refusal".to_owned());let pointer=error.message.as_ptr();let mut claim=Some(Err(error));
 authority.with_loan(&mut context,|loan|{let mut loan=loan.reborrow();let mut work=OriginalWork::empty();work.recipient=Some(&mut loan);let(mut action,mut receipt)=(None,None);let mut original=OriginalWorkBorrow::new(&mut work);assert_eq!(original.prepare(1,&mut action).unwrap(),Some(true));assert!(original.performed_scope(action,&mut receipt,&mut claim).is_err());assert!(receipt.is_none());assert_eq!(claim.as_ref().unwrap().as_ref().unwrap_err().message.as_ptr(),pointer);assert_eq!(original.actual(),u128::from(law["occupied"]["performed"].as_u64().unwrap()));original.cancel(&mut action).unwrap();
 }).unwrap();assert!(authority.into_recipient().is_ok());drop(claim.take());
}

#[test]
fn narrowed_original_work_cannot_widen_or_reuse_its_actual_consumed_window(){
 let issuer=OriginalSourceTurnIssuer::new();let turn=issuer.issue().unwrap();let counter=Arc::new(AtomicUsize::new(0));let checks=Arc::new(AtomicUsize::new(0));let mut authority=OriginalWorkAuthority::new(Source{issuer,turn:turn.identity(),counter:counter.clone(),checks:checks.clone()});let mut context=&turn;
 authority.with_loan(&mut context,|loan|{let mut loan=loan.reborrow();let mut work=OriginalWork::empty();work.recipient=Some(&mut loan);let(mut action,mut receipt,mut claim,mut marker,mut partial,mut proof)=(None,None,None,None,None,None);let mut original=OriginalWorkBorrow::new(&mut work).with_maximum_work(0).unwrap();assert_eq!(original.prepare(1,&mut action).unwrap(),Some(false));assert!(action.is_none());drop(original);
 let mut original=OriginalWorkBorrow::new(&mut work).with_maximum_work(1).unwrap();assert_eq!(original.prepare(1,&mut action).unwrap(),Some(true));{let mut scope=original.performed_scope(action,&mut receipt,&mut claim).unwrap();let mut effect=scope.effect(grant(),1,&mut marker).unwrap();assert!(effect.performed(1).unwrap());counter.fetch_add(1,Ordering::SeqCst);}assert_eq!(claim.take().unwrap().unwrap(),1);
 assert!(original.settle_partial_into(action,&mut partial).unwrap());assert!(partial.take().unwrap().source_refusal().is_none());
 let mut original=original.with_maximum_work(10).unwrap();{let mut scope=original.performed_scope(action,&mut receipt,&mut claim).unwrap();assert!(scope.effect(grant(),1,&mut marker).is_err());}assert_eq!(claim.take().unwrap().unwrap(),1);assert!(original.commit(1,action,&mut proof).unwrap());original.finish(&mut proof,&mut action).unwrap();assert_eq!(original.prepare(1,&mut action).unwrap(),Some(false));assert!(action.is_none());assert_eq!(original.actual(),1);assert_eq!(counter.load(Ordering::SeqCst),1);assert_eq!(checks.load(Ordering::SeqCst),1);
 }).unwrap();assert!(authority.into_recipient().is_ok())
}


#[test]
fn borrowed_work_binding_is_genuine_and_stable_before_an_action_exists(){
 let issuer=OriginalSourceTurnIssuer::new();let turn=issuer.issue().unwrap();let counter=Arc::new(AtomicUsize::new(0));let checks=Arc::new(AtomicUsize::new(0));let mut authority=OriginalWorkAuthority::new(Source{issuer,turn:turn.identity(),counter,checks});let mut context=&turn;let mut binding=None;
 authority.with_loan(&mut context,|loan|{let mut loan=loan.reborrow();let mut work=OriginalWork::empty();work.recipient=Some(&mut loan);let mut original=OriginalWorkBorrow::new(&mut work);assert!(original.action_identity().is_none());let before=original.binding_identity().unwrap();binding=Some(before);let mut action=None;assert_eq!(original.prepare(1,&mut action).unwrap(),Some(true));assert_eq!(original.binding_identity().unwrap(),before);original.cancel(&mut action).unwrap();assert_eq!(original.binding_identity().unwrap(),before);assert_eq!(original.actual(),0)}).unwrap();
 let issuer=OriginalSourceTurnIssuer::new();let turn=issuer.issue().unwrap();let mut foreign=OriginalWorkAuthority::new(Source{issuer,turn:turn.identity(),counter:Arc::new(AtomicUsize::new(0)),checks:Arc::new(AtomicUsize::new(0))});let mut context=&turn;
 foreign.with_loan(&mut context,|loan|{let mut loan=loan.reborrow();let mut work=OriginalWork::empty();work.recipient=Some(&mut loan);assert_ne!(OriginalWorkBorrow::new(&mut work).binding_identity().unwrap(),binding.unwrap())}).unwrap();assert!(authority.into_recipient().is_ok());assert!(foreign.into_recipient().is_ok())
}


struct BindingSource{issuer:OriginalSourceTurnIssuer}
impl OriginalWorkRecipient for BindingSource{
 type Context<'turn,'source>=&'turn OriginalSourceTurn where 'source:'turn;
 fn source_turn<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Option<OriginalSourceTurnIdentity>where 'source:'turn{Some(c.identity())}
 fn validate_context<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Result<(),ValueError>where 'source:'turn{self.issuer.validate(c).map_err(Into::into)}
 fn admits<'a,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,reply:OriginalWorkQuote<'a>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'a>where 'source:'turn{reply.complete(Ok(observed.checked_add(additional).is_some_and(|n|n<=1)))}
 fn settle_partial<'a,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,reply:OriginalWorkPartialCheck<'a>)->OriginalWorkPartialCheckComplete<'a>where 'source:'turn{reply.complete(Ok(()))}
 fn recover_partial<'a,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,reply:OriginalWorkPartialCheck<'a>)->OriginalWorkPartialCheckComplete<'a>where 'source:'turn{reply.pending()}
}
fn observe_effect_binding(authority:&mut OriginalWorkAuthority<BindingSource>,state:&mut OriginalWorkState,turn:&OriginalSourceTurn,expected:Option<OriginalWorkBindingIdentity>,matches:bool)->OriginalWorkBindingIdentity{
 let mut key=None;authority.with_loan(&mut &*turn,|loan|{let mut loan=loan.reborrow();let mut work=OriginalWork{state:state.take(),recipient:Some(&mut loan)};
 {let mut original=OriginalWorkBorrow::new(&mut work);let binding=original.binding_identity().unwrap();key=Some(binding);let(mut action,mut receipt,mut claim,mut marker)=(None,None,None,None);assert_eq!(original.prepare(1,&mut action).unwrap(),Some(true));{let mut scope=original.performed_scope(action,&mut receipt,&mut claim).unwrap();let effect=scope.effect(grant(),1,&mut marker).unwrap();assert_eq!(OriginalWorkBindingIdentity::from_effect(&effect),binding);assert_eq!(OriginalWorkBindingIdentity::from_effect(&effect).matches_effect(&effect),law()["binding"]["fromOriginalEffect"].as_bool().unwrap());assert_eq!(expected.unwrap_or(binding).matches_effect(&effect),matches);}assert_eq!(claim.take().unwrap().unwrap(),0);original.cancel(&mut action).unwrap();assert_eq!(original.actual(),0);}*state=work.state.take();
 }).unwrap();key.unwrap()
}
#[test]
fn genuine_work_binding_matches_later_effects_but_rejects_new_scopes_and_issuers(){
 let issuer=OriginalSourceTurnIssuer::new();let first=issuer.issue().unwrap();let later=issuer.issue().unwrap();let mut authority=OriginalWorkAuthority::new(BindingSource{issuer});let expected=law()["effectBinding"].as_array().unwrap().iter().map(|row|row["matches"].as_bool().unwrap()).collect::<Vec<_>>();let mut state=OriginalWorkState::fresh();let key=observe_effect_binding(&mut authority,&mut state,&first,None,expected[0]);observe_effect_binding(&mut authority,&mut state,&first,Some(key),expected[1]);let mut other_scope=OriginalWorkState::fresh();observe_effect_binding(&mut authority,&mut other_scope,&first,Some(key),expected[2]);
 let source=authority.into_recipient().ok().unwrap();let mut foreign=OriginalWorkAuthority::new(source);observe_effect_binding(&mut foreign,&mut state,&first,Some(key),expected[3]);let current=observe_effect_binding(&mut foreign,&mut state,&first,None,true);observe_effect_binding(&mut foreign,&mut state,&later,Some(current),expected[4]);assert!(foreign.into_recipient().is_ok());
}

#[test]
fn directly_issued_original_receipt_preserves_the_narrowed_work_window(){
 let fixture=law();let law=&fixture["issuedReceipt"];let issuer=OriginalSourceTurnIssuer::new();let turn=issuer.issue().unwrap();let counter=Arc::new(AtomicUsize::new(0));let checks=Arc::new(AtomicUsize::new(0));let mut authority=OriginalWorkAuthority::new(Source{issuer,turn:turn.identity(),counter:counter.clone(),checks:checks.clone()});let mut context=&turn;
 authority.with_loan(&mut context,|loan|{let mut loan=loan.reborrow();let mut work=OriginalWork::empty();work.recipient=Some(&mut loan);let(mut action,mut receipt,mut partial,mut proof)=(None,None,None,None);let mut original=OriginalWorkBorrow::new(&mut work);assert_eq!(original.prepare(1,&mut action).unwrap(),Some(true));let mut original=original.with_maximum_work(law["narrowed"].as_u64().unwrap()).unwrap();original.issue_performed(action,&mut receipt).unwrap();assert_eq!(OriginalWorkSink::remaining_work_units(receipt.as_ref().unwrap()).unwrap(),law["remaining"].as_u64().unwrap());assert_eq!(OriginalWorkSink::performed(receipt.as_mut().unwrap(),law["performed"].as_u64().unwrap()).unwrap(),law["admitted"].as_bool().unwrap());counter.fetch_add(1,Ordering::SeqCst);original.claim_performed(action,&mut receipt).unwrap();assert!(receipt.is_none());assert_eq!(original.performed_total(action).unwrap(),u128::from(law["claimed"].as_u64().unwrap()));assert!(original.settle_partial_into(action,&mut partial).unwrap());assert!(partial.take().unwrap().source_refusal().is_none());assert!(original.commit(1,action,&mut proof).unwrap());original.finish(&mut proof,&mut action).unwrap();assert_eq!(original.prepare(1,&mut action).unwrap(),Some(false));}).unwrap();assert_eq!(checks.load(Ordering::SeqCst),1);assert!(authority.into_recipient().is_ok());
}
