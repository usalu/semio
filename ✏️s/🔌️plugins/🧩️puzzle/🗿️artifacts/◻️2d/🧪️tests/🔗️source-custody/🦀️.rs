//! 🔗️ Native fixtures admit real public source custody and observe its exact final lifecycle.
use semio_framework_value::retained_clone::{RetainedOwnedProjection,RetainedOwnedProjectionAdmission,RetainedClone,RetainedCloneBorrowAuthority,RetainedCloneCursor,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneSource,RetainedCloneStep};
use semio_framework_value::retirement::arc_custody::ArcLease;
use semio_framework_value::{OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll,ValueError,ValueRefusalKind};
use semio_framework_value_clone_source_testing::{admit_owned,admit_projected,drive};
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
use std::cell::RefCell;

const CPU:u64=1048576;
const BODY:usize=4096;
const STOCK:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1048576,maximum_copy_bytes:16777216,maximum_capacity_bytes:16777216,maximum_release_bytes:33554432,maximum_depth:256};
thread_local!{static BANK:RefCell<Option<EffectBank>>=const{RefCell::new(None)};}

fn fresh()->EffectBank{EffectBank::new(Wallet::new(RetainedCloneGrant{maximum_items:usize::MAX,maximum_copy_bytes:usize::MAX,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},1<<62))}

fn lent<R>(operation:impl FnOnce(&mut EffectBank)->R)->R{let mut bank=BANK.with(|slot|slot.borrow_mut().take()).unwrap_or_else(fresh);let result=operation(&mut bank);BANK.with(|slot|*slot.borrow_mut()=Some(bank));result}

/// 🪢️ Runs one synchronous operation inside one funded original effect and returns its value.
pub fn run<R>(grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R{
    let mut output=None;
    let (poll,_)=lent(|bank|{let outcome=bank.run(grant,CPU,|effect|{let boundary=effect.performed_boundary();output=Some(operation(effect));let work=effect.performed_since(boundary).unwrap();RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work})});assert_ne!(bank.take_admitted(),Some(false));outcome});
    let RetainedWorkPoll::Complete(turn)=poll else{panic!("synchronous fixture turn stayed pending")};
    turn.result.unwrap();
    output.take().unwrap()
}

/// 🎞️ Drives one poll-shaped call under a real original action; the physical heap receipt equals the recorded progress.
pub fn driven(grant:RetainedCloneGrant,call:impl FnMut(&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll)->RetainedWorkOutcome<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>{
    drive::warm();
    drive::drive(grant,CPU,call)
}

/// ♻️ Drives one close turn under a real original action and reports its recorded step.
pub fn close_turn(grant:RetainedCloneGrant,call:impl FnMut(&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll)->Result<RetainedCloneStep,ValueError>{
    driven(grant,call).result.map_err(|failure|failure.primary().map_or_else(||ValueError::invariant("original close failed"),|error|ValueError::literal(error.kind,"original close failed")))
}

/// 📤️ Transfers the finished value out of a clone cursor through one real original reply.
pub fn take<T:RetainedClone,C:RetainedCloneCursor<T>>(cursor:&mut C)->Option<T>{
    let demand=cursor.take_demand().unwrap();
    let grant=grant_of(demand.copy_bytes,demand.capacity_bytes,demand.release_bytes,demand.depth);
    let mut output=None;
    drive::warm();
    let outcome=drive::drive_reply::<RetainedCloneStep,ValueError>(grant,CPU,|result:&Result<RetainedCloneStep,ValueError>|match result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()},|effect,reply,recover|if recover{cursor.recover_take(&mut output,effect,reply)}else{cursor.take(&mut output,effect,reply)});
    let progress:RetainedCloneProgress=outcome.result.unwrap().progress();
    assert!(progress.fits(grant));
    output
}

/// 🎟️ Advances a clone cursor over one lent source through one real original reply and reports its recorded step.
pub fn advance<T:RetainedClone,C:RetainedCloneCursor<T>>(cursor:&mut C,source:RetainedCloneRef<'_,T>,body:usize,maximum:u64,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
    drive::warm();
    let outcome=drive::drive_reply::<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>(grant,CPU,|result:&Result<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>|match result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()},|effect,reply,recover|if recover{cursor.recover_advance_into(source,body,maximum,effect,reply)}else{cursor.advance_into(source,body,maximum,effect,reply)});
    outcome.result.map_err(|failure|failure.primary().map_or_else(||ValueError::invariant("original advance failed"),|error|ValueError::literal(error.kind,"original advance failed")))
}

/// 🪢️ Publishes one retained owned projection of a lent child through two real original replies.
pub fn project<T:?Sized+Sync+'static>(child:RetainedCloneRef<'_,T>)->RetainedOwnedProjection<T>{
    let mut admission=RetainedOwnedProjectionAdmission::<T>::default();
    let demand=admission.demand(child).unwrap();
    let grant=grant_of(demand.copy_bytes,demand.capacity_bytes,demand.release_bytes,demand.depth);
    drive::warm();
    let outcome=drive::drive_reply::<RetainedCloneStep,ValueError>(grant,CPU,|result:&Result<RetainedCloneStep,ValueError>|match result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()},|effect,reply,recover|if recover{admission.recover_admit(reply)}else{admission.admit(child,effect,reply)});
    assert!(outcome.result.unwrap().progress().fits(grant));
    let demand=admission.take_demand().unwrap();
    let grant=grant_of(demand.copy_bytes,demand.capacity_bytes,demand.release_bytes,demand.depth);
    let mut output=None;
    drive::warm();
    let outcome=drive::drive_reply::<RetainedCloneStep,ValueError>(grant,CPU,|result:&Result<RetainedCloneStep,ValueError>|match result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()},|effect,reply,recover|if recover{admission.recover_take(reply)}else{admission.take(&mut output,effect,reply)});
    assert!(outcome.result.unwrap().progress().fits(grant));
    output.expect("projection published")
}

/// 🚦️ Runs the admitted close-start of a clone cursor through one real original reply.
pub fn begin_close<T:RetainedClone,C:RetainedCloneCursor<T>>(cursor:&mut C){
    let demand=cursor.begin_close_demand().unwrap();
    let grant=grant_of(demand.copy_bytes,demand.capacity_bytes,demand.release_bytes,demand.depth);
    drive::warm();
    let outcome=drive::drive_reply::<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>(grant,CPU,|result:&Result<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>|match result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()},|effect,reply,recover|if recover{cursor.recover_begin_close_into(effect,reply)}else{cursor.begin_close_into(effect,reply)});
    assert!(outcome.result.unwrap().progress().fits(grant));
}

fn grant_of(copy:usize,capacity:usize,release:usize,depth:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth}}

fn drain<C>(cursor:&mut C,maximum_turns:usize,demands:impl Fn(&C)->(usize,usize,usize,usize),step:fn(&mut C,&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll,terminal:impl Fn(&C)->bool,what:&str){
    for _ in 0..maximum_turns {
        if terminal(cursor){return;}
        let (copy,capacity,release,depth)=demands(cursor);
        assert!(copy+capacity+release<=4096);
        let grant=grant_of(copy,capacity,release,depth);
        let zero=driven(RetainedCloneGrant::default(),|effect|step(cursor,effect));
        assert_eq!(zero.result.unwrap().progress(),Default::default());
        for axis in 0..4 {
            let mut below=grant;
            let demand=match axis {0=>&mut below.maximum_copy_bytes,1=>&mut below.maximum_capacity_bytes,2=>&mut below.maximum_release_bytes,_=>&mut below.maximum_depth};
            if *demand==0 {continue;}
            *demand-=1;
            let outcome=driven(below,|effect|step(cursor,effect));
            match outcome.result {Ok(step)=>assert_eq!(step.progress(),Default::default()),Err(error)=>assert!(axis==3&&error.primary().is_some_and(|error|error.kind==ValueRefusalKind::DepthLimit))}
            assert_eq!(demands(cursor),(copy,capacity,release,depth));
        }
        let outcome=driven(grant,|effect|step(cursor,effect));
        assert!(outcome.result.unwrap().progress().fits(grant));
    }
    panic!("{what} did not close under its pure demands");
}

/// 🔗️ A unit-authority custody that lends any externally owned value under its lease: the alias identity is the value's own address.
/// The lent reference never outlives the borrow of both the custody and the value (the returned ref carries `'a`).
pub struct Custody(RetainedCloneBorrowAuthority<()>);

impl Custody {
    pub fn borrow<'a, T: ?Sized + 'static>(&'a self, value: &'a T) -> RetainedCloneRef<'a, T> {
        let lent: &'static T = unsafe { &*(value as *const T) };
        self.0.borrow(0, move |_| lent)
    }
}

impl std::ops::Deref for Custody {
    type Target = RetainedCloneBorrowAuthority<()>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Custody {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn admit() -> Custody {
    let capacity=RetainedCloneBorrowAuthority::<()>::constructor_capacity_bytes();
    assert!(capacity>0&&capacity<=4096);
    let (source,progress)=lent(|bank|match admit_owned(bank,(),(),STOCK).result{Ok(admitted)=>admitted,Err(_)=>panic!("unit source custody admission refused")});
    assert!(progress.fits(STOCK));
    Custody(RetainedCloneBorrowAuthority::from_admitted_source(source))
}

pub fn close(source:&mut Custody) {
    drain(source,100000,|source|{let copy=source.next_close_copy_byte_demand().unwrap();(copy,source.next_close_capacity_byte_demand(copy).unwrap(),source.next_close_release_byte_demand().unwrap(),source.next_close_depth_demand().unwrap())},|source,effect|source.close_step(BODY,effect),|source|source.terminal_is_empty(),"original source custody");
}

/// 🔗️ Admits one owned value as a retained source under one funded original admission.
pub fn admit_owner<T:semio_framework_value::retirement::RetireOwned+Sync+'static>(owner:T,grant:RetainedCloneGrant)->Result<(RetainedCloneSource<T>,RetainedCloneProgress),ValueError>{
    lent(|bank|admit_owned(bank,owner,(),grant).result.map_err(|_|ValueError::invariant("owned source admission refused")))
}

pub fn admit_source<T:semio_framework_value::retirement::RetireOwned+Sync+Send+'static>(original:std::sync::Arc<T>) -> RetainedCloneSource<T,ArcLease<T>> {
    let capacity=RetainedCloneSource::<T,ArcLease<T>>::constructor_capacity_bytes::<()>();
    assert!(capacity>0&&capacity<=4096);
    let (source,progress)=lent(|bank|match admit_projected::<T,ArcLease<T>,()>(bank,ArcLease(Some(original)),(),STOCK).result{Ok(admitted)=>admitted,Err(_)=>panic!("snapshot source custody admission refused")});
    assert!(progress.fits(STOCK));
    source
}

pub fn close_source<T:semio_framework_value::retirement::RetireOwned+Sync+Send+'static>(source:&mut RetainedCloneSource<T,ArcLease<T>>) {
    drain(source,100000,|source|{let copy=source.next_close_copy_byte_demand().unwrap();(copy,source.next_close_capacity_byte_demand(copy).unwrap(),source.next_close_release_byte_demand().unwrap(),source.next_close_depth_demand().unwrap())},|source,effect|source.close_step(BODY,effect),|source|source.terminal_is_empty(),"original snapshot source custody");
}

pub fn close_cursor<C>(cursor:&mut C,maximum_turns:usize,demands:fn(&C)->(usize,usize,usize,usize),step:fn(&mut C,&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll,terminal:fn(&C)->bool) {
    drain(cursor,maximum_turns,demands,step,terminal,"native domain controller");
}
