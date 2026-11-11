//! 🧪️ A Pending shared close recovers on a later turn from whatever grant admits its recorded quote, and a grant that cannot admit it stays Pending without charge.
use semio_framework_value::{close_owner,retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,RetainedCloneProgress},ErasedSnapshotRetirement,OriginalRetirementFailure,RetainedAllocationEffect,RetainedCloneGrant,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,ValueError};
use semio_framework_value_clone_source_testing::drive::{drive,warm};
use semio_framework_value_retirement::{RetireOwned,retirement::{RetirementCursor,RetirementStep,arc_custody::{SharedControlledRetirement,original::SharedOriginalStorage}}};
use std::mem::size_of;

const WORK:u64=1<<10;
const BODY:usize=1<<12;
thread_local!{static RECOVERS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};}
type Reply<'r>=OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>;
type Done<'r>=OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>;

struct Payload;
struct PayloadCursor{phase:u8}
impl RetireOwned for Payload{
    type RetirementCursor=PayloadCursor;
    fn retirement_seed(self)->PayloadCursor{PayloadCursor{phase:0}}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<PayloadCursor>())}
    fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for PayloadCursor{
    fn next_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(if self.phase==2{Default::default()}else{RetirementDemand{copy_bytes:2,depth:1,..Default::default()}})}
    fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{Ok(if self.phase==2{0}else{2})}
    fn terminal_is_empty(&self)->bool{self.phase==2}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
    fn close_into<'r>(&mut self,_:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{assert_eq!(self.phase,0);assert!(effect.performed(1)?);self.phase=1;Ok(reply.pending())}
    fn recover_close_into<'r>(&mut self,_:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{
        assert_eq!(self.phase,1);RECOVERS.with(|count|count.set(count.get()+1));assert!(effect.performed(1)?);self.phase=2;
        Ok(reply.complete(Ok(RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:2,..Default::default()},1))))
    }
}

fn grant_of(demand:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}

fn issued()->SharedControlledRetirement<Payload>{
    warm();let mut source=Some(Payload);let mut storage=None;
    drive(grant_of(SharedOriginalStorage::<Payload>::source_admission_demand().unwrap()),WORK,|effect|{let turn=SharedOriginalStorage::admit_from(&mut source,&mut storage,effect);RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work})});
    SharedControlledRetirement::new(storage.expect("shared original admitted"))
}

#[derive(Default)]
struct Tally{starved:usize,starved_work:u128,recovered:usize}

fn close(starve:bool)->Tally{
    let mut owner=issued();let mut tally=Tally::default();RECOVERS.with(|count|count.set(0));
    for turn in 0..64{
        if owner.terminal_is_empty(){break}
        assert!(turn<63,"shared close exceeded its bounded turn law");
        let demand=owner.next_demand(BODY).unwrap();
        let wide=RetainedCloneGrant{maximum_items:8,maximum_copy_bytes:demand.copy_bytes.max(BODY),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth+8};
        drive(wide,WORK,|effect|{
            if !owner.resident_reply().unwrap().pending(){return close_owner(&mut owner,BODY,effect)}
            let quote=owner.next_demand(BODY).unwrap();let remaining=effect.remaining_work_units().unwrap();
            if starve&&tally.starved==0{
                let before=effect.performed_work();let mut narrow=effect.reborrow(RetainedCloneGrant{maximum_depth:quote.depth-1,..grant_of(quote)},remaining).unwrap();
                let poll=close_owner(&mut owner,BODY/2,&mut narrow);
                assert!(matches!(poll,RetainedWorkPoll::Pending{work:0}));assert_eq!(narrow.performed_work(),before);
                tally.starved+=1;tally.starved_work+=poll.work();return poll
            }
            let mut narrow=effect.reborrow(grant_of(quote),remaining).unwrap();tally.recovered+=1;
            close_owner(&mut owner,BODY/2,&mut narrow)
        });
    }
    assert!(owner.terminal_is_empty());tally
}

#[test]
fn pending_shared_close_recovers_from_a_smaller_sufficient_grant(){
    let tally=close(false);
    assert_eq!(tally.recovered,1);assert_eq!(RECOVERS.with(std::cell::Cell::get),1);
}

#[test]
fn pending_shared_close_stays_pending_without_charge_under_an_insufficient_grant(){
    let tally=close(true);
    assert_eq!((tally.starved,tally.starved_work),(1,0));assert_eq!(tally.recovered,1);assert_eq!(RECOVERS.with(std::cell::Cell::get),1);
}

#[test]
fn recovery_admission_charges_nothing_and_refuses_a_grant_below_the_recorded_quote(){
    warm();let quote=RetirementDemand{copy_bytes:8,capacity_bytes:0,release_bytes:0,depth:2};
    drive(RetainedCloneGrant{maximum_items:8,maximum_copy_bytes:64,maximum_capacity_bytes:64,maximum_release_bytes:64,maximum_depth:8},WORK,|effect|{
        let boundary=effect.action_boundary();let before=effect.performed_work();let remaining=effect.remaining_work_units().unwrap();
        assert!(effect.admits_recovery(quote,1,boundary).unwrap());
        assert!(effect.admits_recovery(RetirementDemand{depth:9,..quote},1,boundary).unwrap()==false);
        assert!(!effect.admits_recovery(quote,remaining+1,boundary).unwrap());
        let mut narrow=effect.reborrow(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:7,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:2},remaining).unwrap();
        assert!(!narrow.admits_recovery(quote,1,boundary).unwrap());
        assert_eq!(narrow.performed_work(),before);
        RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(semio_framework_value::RetainedCloneStep::Progress(Default::default())),work:0})
    });
}
