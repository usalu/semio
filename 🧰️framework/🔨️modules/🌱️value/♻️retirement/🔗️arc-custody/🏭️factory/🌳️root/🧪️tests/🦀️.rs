//! 🧪️ An Arc root elects its payload only when it is the unique owner: a foreign alias keeps the payload alive and the root still reaches terminal closure.
use semio_framework_value::{factory::ArcRootRetirement,ArtifactOwnedValueRetirementFactory,ErasedSnapshotRetirement,OwnedValueRetirementFactory,OriginalRetirementFailure,OriginalRetirementPoll,ReplyStash,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll};
use semio_framework_value_clone_source_testing::drive::{drive as drive_turn,warm};
use semio_framework_value::{close_factory_ticket,factory_ticket_demands,retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete},RetirementDemand,ValueError};
use std::sync::{Arc,Weak,atomic::{AtomicUsize,Ordering}};

const WORK:u64=1<<10;
fn drive(grant:RetainedCloneGrant,call:impl FnMut(&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll)->RetainedCloneStep{drive_turn(grant,WORK,call).result.unwrap()}

fn admitted(alias:Arc<String>)->Box<dyn ErasedSnapshotRetirement>{
    let demand=ArcRootRetirement::<String>::admission_demand();
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:0,maximum_depth:demand.depth};
    let mut original=Some(alias);let mut factory:Option<Arc<dyn ArtifactOwnedValueRetirementFactory<String>>>=Some(Arc::new(OwnedValueRetirementFactory::<String>::default()));let mut output=None;
    drive(grant,|effect|ArcRootRetirement::admit_original(&mut original,&mut factory,&mut output,effect));
    assert!(original.is_none());output.expect("root admitted into its resident frame")
}

fn close(mut owner:Box<dyn ErasedSnapshotRetirement>){
    let mut stash=ReplyStash::<OriginalRetirementFailure>::default();let body=1<<12;
    for turn in 0..4096{
        if owner.terminal_is_empty(){return}
        let demand=owner.next_demand(body).unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes.max(body),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
        drive(grant,|effect|{
            let answer=stash.run(|reply,recover|(if recover{owner.recover_close_into(body,effect,reply)}else{owner.close_into(body,effect,reply)}).map(drop)).unwrap();
            let work=effect.performed_work();
            match answer{Some(result)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work}),None=>RetainedWorkPoll::Pending{work}}
        });
        assert!(turn<4095,"arc root closure exceeded its bounded turn law");
    }
}

#[test]
fn root_with_a_foreign_alias_leaves_the_payload_to_that_alias(){
    let foreign=Arc::new(String::from("shared media snapshot"));let weak:Weak<String>=Arc::downgrade(&foreign);
    warm();let owner=admitted(Arc::clone(&foreign));assert_eq!(Arc::strong_count(&foreign),2);
    close(owner);
    assert_eq!(Arc::strong_count(&foreign),1);assert_eq!(weak.upgrade().as_deref().map(String::as_str),Some("shared media snapshot"));
}

#[test]
fn root_that_is_the_last_owner_elects_and_retires_the_payload(){
    let alias=Arc::new(String::from("unique media snapshot"));let weak=Arc::downgrade(&alias);
    warm();let owner=admitted(alias);assert_eq!(weak.strong_count(),1);
    close(owner);
    assert!(weak.upgrade().is_none());
}

#[test]
fn root_closes_to_the_same_terminal_whichever_alias_is_released_last(){
    let foreign=Arc::new(String::from("ordered media snapshot"));let weak=Arc::downgrade(&foreign);
    warm();let owner=admitted(Arc::clone(&foreign));drop(foreign);assert_eq!(weak.strong_count(),1);
    close(owner);
    assert!(weak.upgrade().is_none());
}

type Reply<'r>=OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>;
type Done<'r>=OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>;
#[derive(Default)]
struct Tally{entered:AtomicUsize,recovered:AtomicUsize,executed:AtomicUsize}
struct ReplyOnly{stash:ReplyStash<OriginalRetirementFailure>,tally:Arc<Tally>,done:bool}
impl ErasedSnapshotRetirement for ReplyOnly{
    fn resident_reply(&mut self)->Option<&mut ReplyStash<OriginalRetirementFailure>>{Some(&mut self.stash)}
    fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{Ok(1)}
    fn close_into<'r>(&mut self,_:usize,_:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.tally.entered.fetch_add(1,Ordering::SeqCst);Ok(reply.pending())}
    fn recover_close_into<'r>(&mut self,_:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{
        self.tally.recovered.fetch_add(1,Ordering::SeqCst);
        if !effect.performed(1)?{return Err(ValueError::invariant("reply-only law turn lacks its work"))}
        self.tally.executed.fetch_add(1,Ordering::SeqCst);self.done=true;
        Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))
    }
    fn close_step(&mut self,_:usize,_:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::invariant("reply-only owner must not be driven by close_step").into()),work:0})}
    fn terminal_is_empty(&self)->bool{self.done}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(1)}
}

#[test]
fn boxed_reply_only_cursor_closes_through_the_factory_ticket_across_a_pending_turn_exactly_once(){
    let tally=Arc::new(Tally::default());warm();
    let mut slot:Option<Box<dyn ErasedSnapshotRetirement>>=Some(Box::new(ReplyOnly{stash:ReplyStash::default(),tally:Arc::clone(&tally),done:false}));
    let body=1<<12;let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:body,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};
    let step=drive(grant,|effect|close_factory_ticket(&mut slot,body,effect));
    assert_eq!(step.progress(),Default::default());
    assert_eq!((tally.entered.load(Ordering::SeqCst),tally.recovered.load(Ordering::SeqCst),tally.executed.load(Ordering::SeqCst)),(1,1,1),"the pending turn kept its state and executed exactly once");
    assert!(slot.as_ref().is_some_and(|owner|owner.terminal_is_empty()));
    drop(slot.take());
}

#[test]
fn arc_root_closes_through_the_factory_ticket(){
    let foreign=Arc::new(String::from("ticket media snapshot"));let weak=Arc::downgrade(&foreign);
    warm();let mut slot=Some(admitted(foreign));let body=1<<12;
    for turn in 0..4096{
        let Some(owner)=slot.as_ref()else{break};
        let demand:RetirementDemand=factory_ticket_demands(owner,body).unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes.max(body),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
        drive(grant,|effect|close_factory_ticket(&mut slot,body,effect));
        assert!(turn<4095,"ticket closure exceeded its bounded turn law");
    }
    assert!(slot.is_none(),"the final shell release went through the ticket");assert!(weak.upgrade().is_none());
}
