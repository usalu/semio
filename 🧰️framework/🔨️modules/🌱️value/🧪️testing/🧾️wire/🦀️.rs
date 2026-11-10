//! 🧾️ Canonical language-neutral operation wire witness through the independent JSON oracle.
/// 🎟️ Is the physical close authority of original native conformance laws; it equals `physicalCloseGrant` of the os-store emission fixture, which the os-store laws pin.
pub fn physical_test_close_grant() -> crate::RetainedCloneGrant {
    crate::RetainedCloneGrant { maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:64 }
}

/// 🎟️ Drives one staged original source admission to its output through an original effect bank, paying every turn its exact quote and reporting the summed receipt and actual work.
pub fn admit_owned_source<T:crate::retirement::RetireOwned+Sync,B:crate::retirement::RetireOwned>(owner:T,authority:B)->(crate::RetainedCloneSource<T>,crate::RetainedCloneProgress,u64){
    use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
    let mut admission=crate::RetainedCloneSource::admission_owned(owner,authority);
    let mut bank=EffectBank::new(Wallet::from_profile(&serde_json::json!({"items":1u64<<20,"copyBytes":1u64<<30,"capacityBytes":1u64<<30,"releaseBytes":1u64<<30,"depth":64u64,"fuel":1u64<<40})));
    let mut output=None;let mut total=crate::RetainedCloneProgress::default();let mut work=0u128;
    let(mut frame,mut action)=(None,None);
    while output.is_none()||!admission.terminal_is_empty(){
        let stage=admission.projection_stage(&frame).expect("staged source projection stage");
        let(demand,quote)=match stage{Some(stage)=>admission.projection_stage_quote(&frame,stage).expect("staged source projection quote"),None=>(admission.next_demand().expect("staged source admission quote"),0)};
        let grant=crate::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
        let poll=if let Some(stage)=stage{
            bank.run(grant,quote,|effect|admission.project_staged(&mut frame,&mut action,stage,effect)).0
        }else if admission.phase()=="ready"{
            let maximum=admission.take_output_work_demand();
            bank.run(grant,maximum,|effect|crate::RetainedWorkPoll::Complete(admission.take_output(&mut output,effect))).0
        }else{
            let maximum=admission.advance_work_demand().expect("staged source admission work quote");
            bank.run(grant,maximum,|effect|crate::RetainedWorkPoll::Complete(admission.advance(effect))).0
        };
        assert_ne!(bank.take_admitted(),Some(false),"staged source admission turn was refused by its original Work verdict");
        let turn=match poll{crate::RetainedWorkPoll::Complete(turn)=>turn,crate::RetainedWorkPoll::Pending{..}=>panic!("staged source admission turn stayed pending")};
        work+=turn.work;
        let step=turn.result.expect("staged source admission turn").progress();
        assert!(step.fits(grant));
        total.copied_items+=step.copied_items;total.copied_bytes+=step.copied_bytes;total.retained_capacity_bytes+=step.retained_capacity_bytes;total.released_bytes+=step.released_bytes;
    }
    assert!(admission.terminal_is_empty());
    (output.expect("staged source admission output"),total,u64::try_from(work).expect("staged source admission work fits u64"))
}

use crate::retained_clone::{OriginalSourceTurnIdentity,OriginalWork,OriginalWorkAuthority,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkRecipient,OriginalWorkSink};
use crate::{RetainedAllocationEffect,RetainedCloneGrant,RetainedWorkOutcome,RetainedWorkPoll,ValueError};
struct Producer{maximum:u64}
impl OriginalWorkRecipient for Producer{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply>where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|total|total<=self.maximum)))}
 fn settle_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
 fn recover_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
}
/// 🪢️ Runs one operation inside one actual original effect over the caller's allocation marker and returns the performed work the effect recorded.
pub fn with_effect<R>(grant:RetainedCloneGrant,maximum:u64,actual:&mut Option<usize>,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->(R,u128){
 let mut authority=OriginalWorkAuthority::new(Producer{maximum});
 let result=authority.with_loan(&mut (),|loan|{let mut loan=loan.reborrow();let mut work=OriginalWork::empty();work.recipient=Some(&mut loan);let mut action=None;let mut proof=None;assert_eq!(work.prepare_for_action(maximum,&mut action).unwrap(),Some(true));let mut receipt=None;work.issue_performed_for_action(action,&mut receipt).unwrap();let output={let mut effect=RetainedAllocationEffect::new(grant,maximum,actual,receipt.as_mut().unwrap());operation(&mut effect)};let performed=OriginalWorkSink::observed_work(receipt.as_ref().unwrap());work.claim_performed_for_action(action,&mut receipt).unwrap();if performed==0{work.cancel_for_action(&mut action).unwrap()}else{let mut partial=None;assert!(work.settle_partial_for_action_into(action,&mut partial).unwrap());let partial=partial.take().expect("original fixture Work partial outcome");assert_eq!(partial.total(),performed);assert!(partial.source_refusal().is_none());assert!(work.commit_for_action(performed,action,&mut proof).unwrap());work.finish_for_action(&mut proof,&mut action).unwrap()}(output,performed)}).unwrap();
 assert!(!authority.has_live_actions());result
}
/// 🏁️ A fixture drives every retirement turn to completion, so a suspended poll is a fixture fault.
pub fn settled<T,E>(poll:RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("original fixture turn stayed pending")}}
/// 🪢️ Runs one operation inside one actual original effect and discards the performed-work receipt.
pub fn fx<R>(grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R{with_effect(grant,1<<20,&mut None,operation).0}
