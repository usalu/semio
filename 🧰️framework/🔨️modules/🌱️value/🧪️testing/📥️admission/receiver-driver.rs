//! 🧭️ One native receiver is driven end to end through genuine original effects, Source dispatch and a finite wallet.
use semio_framework_value_native_owned::*;
use semio_framework_value_native_owned::owned::{NativeAdvanceSelection,NativeReceiver};
use semio_framework_value_testing_admission::{EffectBank,session_admit,session_driven as driven,session_nested,session_open,session_run as bank_run};
use semio_framework_value_testing_admission::driven as bank_driven;
use semio_framework_value_testing_allocator::{observe_retirement_allocations as observe,refuse_one_allocation_on_this_thread};
use semio_framework_value_native_owned::native_decoding::{OriginalEffectReply,OriginalEffectReplyComplete,OriginalReceivingPrimary};

const MAXIMUM:u64=1<<20;
const RETIRE_BODY:usize=4096;
const STALL:u32=1024;
const CLOSE_STEPS:u32=8192;
const PENDING_SETTLE:u64=4096;
thread_local!{static IDLE:std::cell::Cell<u32>=const{std::cell::Cell::new(0)};}
use semio_framework_value_native_owned::retirement::{RetireOwned,controlled::ControlledRetirement};
use semio_framework_value_native_owned::retained_clone::{RetainedCloneRef,RetainedCloneSource};
pub fn bank()->EffectBank{EffectBank::new(semio_framework_value_testing_admission::Wallet::new(RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:1<<10},1<<30))}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
/// 🪢️ Settles one reply-shaped action into a poll.
pub fn settle<E:From<ValueError>>(effect:&mut RetainedAllocationEffect<'_>,run:impl for<'r> FnOnce(&mut RetainedAllocationEffect<'_>,OriginalEffectReply<'r,RetainedCloneStep,E>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,E>,ValueError>)->RetainedWorkPoll<RetainedCloneStep,E>{
    let mut slot=None;let ran=OriginalEffectReply::borrow(&mut slot).and_then(|reply|run(&mut *effect,reply).map(|_|()));let work=effect.performed_work();
    match(ran,slot.take()){(Err(error),_)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work}),(Ok(()),Some(result))=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work}),(Ok(()),None)=>RetainedWorkPoll::Pending{work}}
}
fn post<E:std::fmt::Debug>(bank:&mut EffectBank,g:RetainedCloneGrant,heap:(usize,usize),poll:RetainedWorkPoll<RetainedCloneStep,E>)->Result<(),String>{post_work(bank,g,heap,poll,&mut 0)}
fn post_work<E:std::fmt::Debug>(bank:&mut EffectBank,g:RetainedCloneGrant,heap:(usize,usize),poll:RetainedWorkPoll<RetainedCloneStep,E>,work:&mut u128)->Result<(),String>{
    match poll{RetainedWorkPoll::Pending{work:pending}=>{*work+=pending;Ok(())},RetainedWorkPoll::Complete(turn)=>{*work+=turn.work;match turn.result{Ok(step)=>{bank.post(step.progress(),g,heap);Ok(())},Err(error)=>Err(format!("{error:?}"))}}}
}
fn denials(g:RetainedCloneGrant,d:RetirementDemand)->Vec<RetainedCloneGrant>{
    [Some(RetainedCloneGrant{maximum_items:0,..g}),(d.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..g}),(d.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes.saturating_sub(1),..g}),(d.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:d.release_bytes.saturating_sub(1),..g}),(d.depth>0).then_some(RetainedCloneGrant{maximum_depth:d.depth.saturating_sub(1),..g})].into_iter().flatten().collect()
}
#[track_caller]fn refused<E:std::fmt::Debug>(poll:RetainedWorkPoll<RetainedCloneStep,E>,heap:(usize,usize)){
    assert_eq!(heap,(0,0),"denied turn allocated");
    match poll{RetainedWorkPoll::Complete(turn)=>{assert_eq!(turn.work,0,"denied turn worked");match turn.result{Ok(step)=>assert_eq!(step.progress(),Default::default(),"denied turn progressed"),Err(error)=>panic!("denied turn failed: {error:?}")}},RetainedWorkPoll::Pending{..}=>panic!("denied turn parked")}
}
/// 📥️ Admits one genuine Source.
pub fn admit<I:RetireOwned+Sync+'static>(bank:&mut EffectBank,value:I)->RetainedCloneSource<I>{let g=bank.loan(1,RETIRE_BODY);match semio_framework_value_testing_admission::admit_owned(bank,value,(),g).result{Ok((source,_))=>source,Err(_)=>panic!("fixture source admission refused")}}
fn faulted<E:std::fmt::Debug+RetireOwned>(bank:&mut EffectBank,poll:RetainedWorkPoll<RetainedCloneStep,E>,heap:(usize,usize),attempts:usize,work:u128){
    assert_eq!(attempts,1,"refused allocation not requested once");assert_eq!(heap,(0,0),"refused allocation left heap");
    match poll{RetainedWorkPoll::Complete(turn)=>{assert_eq!(turn.work,work,"refused turn worked differently");match turn.result{Err(error)=>retire(error,bank),Ok(step)=>panic!("refused allocation did not fail: {step:?}")}},RetainedWorkPoll::Pending{..}=>panic!("refused allocation parked")}
}
/// 🧵️ Runs one synchronous funded turn.
pub fn funded<T,E>(bank:&mut EffectBank,grant:RetainedCloneGrant,maximum:u64,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<T,E>)->(RetainedWorkOutcome<T,E>,Option<usize>,(usize,usize)){
    session_open(bank);let((poll,actual),heap)=observe(||bank_run(bank,grant,maximum,|effect|RetainedWorkPoll::Complete(operation(effect))));
    match poll{RetainedWorkPoll::Complete(turn)=>(turn,actual,heap),RetainedWorkPoll::Pending{..}=>panic!("a synchronous funded turn stayed pending")}
}
/// 🛬️ Receives one value, or cancels after `cut` turns: every turn is first denied on each demanded axis, then funded.
pub fn receive_cut<I:?Sized,T,R:NativeReceiver<I,T>>(receiver:&mut R,source:RetainedCloneRef<'_,I>,bank:&mut EffectBank,body:usize,cut:Option<u64>)->Result<Option<T>,String>{receive_faulted(receiver,source,bank,body,cut,None)}
/// 🚫️ Like [`receive_cut`]; the first allocating turn has its one allocation refused after exactly `Some(work)` work.
pub fn receive_faulted<I:?Sized,T,R:NativeReceiver<I,T>>(receiver:&mut R,source:RetainedCloneRef<'_,I>,bank:&mut EffectBank,body:usize,cut:Option<u64>,fault:Option<u128>)->Result<Option<T>,String>{receive_traced(receiver,source,bank,body,cut,fault,&mut Trace::default())}
struct AdmitReset;
impl Drop for AdmitReset{fn drop(&mut self){session_admit(None)}}
/// 🧾️ Records per-turn work and the quote shape of every funded turn; clearing `admit` refuses the next cancellation checkpoint.
#[derive(Debug,Default)]
pub struct Trace{pub advance:Vec<u128>,pub shapes:Vec<[usize;5]>,pub take:u128,pub admit:Option<std::sync::Arc<std::sync::atomic::AtomicBool>>}
/// 🧾️ Like [`receive_faulted`], recording work in `trace`.
pub fn receive_traced<I:?Sized,T,R:NativeReceiver<I,T>>(receiver:&mut R,source:RetainedCloneRef<'_,I>,bank:&mut EffectBank,body:usize,cut:Option<u64>,fault:Option<u128>,trace:&mut Trace)->Result<Option<T>,String>{
    let pointer=std::ptr::from_ref(source.get());let mut pending_fault=fault;session_open(bank);
    session_admit(trace.admit.clone());let _reset=AdmitReset;
    for turn in 0..1_000_000u64{
        if let Some(cut)=cut{if turn>=cut{if !receiver.advance_pending(){break}assert!(turn<cut.saturating_add(PENDING_SETTLE),"receiver stayed pending {PENDING_SETTLE} turns after its cut: {:?}",receiver.select_advance(source,body,MAXIMUM).map_err(|error|format!("{error:?}")))}}
        if receiver.output_ready(){break}
        assert!(IDLE.get()<STALL,"receiver stalled: {} consecutive funded turns performed no work",IDLE.get());
        let selection=receiver.select_advance(source,body,MAXIMUM).map_err(|error|format!("{error:?}"))?;
        let demand=receiver.demands(source,body,MAXIMUM).map_err(|error|format!("{error:?}"))?;let g=grant(demand);
        trace.shapes.push([demand.copy_bytes,demand.capacity_bytes,demand.release_bytes,demand.depth,match selection{NativeAdvanceSelection::Nested=>usize::MAX,NativeAdvanceSelection::Local{work}=>work as usize}]);
        for denied in denials(g,demand){
            match selection{
                NativeAdvanceSelection::Nested=>{let(poll,heap)=observe(||session_nested(bank,denied,|control,witness|receiver.advance_nested(source,control,witness,body,MAXIMUM,denied)));refused(poll,heap);},
                NativeAdvanceSelection::Local{work}=>{let((poll,_),heap)=observe(||driven::<OriginalReceivingPrimary>(bank,denied,work.max(1),|effect,reply,recover|if recover{receiver.recover_advance_local(source,body,effect,reply)}else{receiver.advance_local(source,body,effect,reply)}));refused::<OriginalReceivingPrimary>(poll,heap);},
            }
            assert_eq!(receiver.demands(source,body,MAXIMUM).map_err(|error|format!("{error:?}"))?,demand,"denied turn changed quote");assert_eq!(std::ptr::from_ref(source.get()),pointer,"denied turn moved Source");
        }
        if let(Some(work),true)=(pending_fault,demand.capacity_bytes>0){
            pending_fault=None;
            match selection{
                NativeAdvanceSelection::Nested=>{let((poll,attempts),heap)=observe(||refuse_one_allocation_on_this_thread(||session_nested(bank,g,|control,witness|receiver.advance_nested(source,control,witness,body,MAXIMUM,g))));if attempts==0{pending_fault=Some(work);let mut spent=0;let posted=post_work(bank,g,heap,poll,&mut spent);trace.advance.push(spent);IDLE.set(if spent==0{IDLE.get()+1}else{0});posted?;continue}faulted(bank,poll,heap,attempts,work);},
                NativeAdvanceSelection::Local{work:funds}=>{let(((poll,_),attempts),heap)=observe(||refuse_one_allocation_on_this_thread(||driven::<OriginalReceivingPrimary>(bank,g,funds.max(1),|effect,reply,recover|if recover{receiver.recover_advance_local(source,body,effect,reply)}else{receiver.advance_local(source,body,effect,reply)})));if attempts==0{pending_fault=Some(work);let mut spent=0;let posted=post_work(bank,g,heap,poll,&mut spent);trace.advance.push(spent);IDLE.set(if spent==0{IDLE.get()+1}else{0});posted?;continue}faulted(bank,poll,heap,attempts,work);},
            }
            continue;
        }
        match selection{
            NativeAdvanceSelection::Nested=>{let(poll,heap)=observe(||session_nested(bank,g,|control,witness|receiver.advance_nested(source,control,witness,body,MAXIMUM,g)));{let mut work=0;let posted=post_work(bank,g,heap,poll,&mut work);trace.advance.push(work);posted?;}},
            NativeAdvanceSelection::Local{work}=>{let((poll,_),heap)=observe(||driven::<OriginalReceivingPrimary>(bank,g,work.max(1),|effect,reply,recover|if recover{receiver.recover_advance_local(source,body,effect,reply)}else{receiver.advance_local(source,body,effect,reply)}));{let mut work=0;let posted=post_work(bank,g,heap,poll,&mut work);trace.advance.push(work);posted?;}},
        }
        IDLE.set(if trace.advance.last()==Some(&0){IDLE.get()+1}else{0});
    }
    if cut.is_some(){return Ok(None)}
    assert!(receiver.output_ready(),"receiver stalled before its output was ready");
    let demand=receiver.take_demand().map_err(|error|format!("{error:?}"))?;let work=receiver.take_work_demand().map_err(|error|format!("{error:?}"))?;let g=grant(demand);let mut output=None;
    for denied in denials(g,demand){let((poll,_),heap)=observe(||driven::<OriginalReceivingPrimary>(bank,denied,work.max(1),|effect,reply,recover|if recover{receiver.recover_take(&mut output,effect,reply)}else{receiver.take(&mut output,effect,reply)}));refused(poll,heap);assert!(output.is_none()&&receiver.output_ready(),"denied take lost output");}
    let(mut work,mut g)=(work,g);
    for _ in 0..1024{let((poll,_),heap)=observe(||driven::<OriginalReceivingPrimary>(bank,g,work.max(1),|effect,reply,recover|if recover{receiver.recover_take(&mut output,effect,reply)}else{receiver.take(&mut output,effect,reply)}));let settled=matches!(&poll,RetainedWorkPoll::Complete(_));post_work(bank,g,heap,poll,&mut trace.take)?;if output.is_some()||settled&&!receiver.output_ready(){break}if settled{let demand=receiver.take_demand().map_err(|error|format!("{error:?}"))?;work=receiver.take_work_demand().map_err(|error|format!("{error:?}"))?;g=grant(demand)}}
    output.map(Some).ok_or_else(||"receiver published nothing".to_owned())
}
/// ✂️ Asserts that a cut receive published nothing.
pub fn none<T>(received:Result<Option<T>,String>)->Result<(),String>{received.map(|published|assert!(published.is_none(),"a cut receive publishes nothing"))}
/// ♻️ Closes the receiver through funded steps, denying each axis first.
pub fn close<I:?Sized,T,R:NativeReceiver<I,T>>(receiver:&mut R,bank:&mut EffectBank,body:usize){
    session_open(bank);
    for _ in 0..CLOSE_STEPS{
        if receiver.terminal_is_empty(){return}
        assert!(!receiver.requires_original_birth()&&!receiver.requires_original_control(),"close loan not lent");
        if !receiver.is_closing(){
            let d=receiver.begin_close_demand().unwrap();let w=receiver.begin_close_work_demand().unwrap();let g=grant(d);
            for denied in denials(g,d){let((poll,_),heap)=observe(||driven::<OriginalRetirementFailure>(bank,denied,w.max(1),|effect,reply,recover|if recover{receiver.recover_begin_close_into(effect,reply)}else{receiver.begin_close_into(effect,reply)}));refused(poll,heap);assert_eq!(receiver.begin_close_demand().unwrap(),d,"denied close start changed quote");}
            let((poll,_),heap)=observe(||driven::<OriginalRetirementFailure>(bank,g,w.max(1),|effect,reply,recover|if recover{receiver.recover_begin_close_into(effect,reply)}else{receiver.begin_close_into(effect,reply)}));post(bank,g,heap,poll).unwrap();
        }else{
            let d=receiver.close_demands(body).unwrap();let w=receiver.close_work_demand(body).unwrap();let g=grant(d);
            for denied in denials(g,d){let((poll,_),heap)=observe(||driven::<OriginalRetirementFailure>(bank,denied,w.max(1),|effect,reply,recover|if recover{receiver.recover_close_step(body,effect,reply)}else{receiver.close_step(body,effect,reply)}));refused(poll,heap);assert_eq!(receiver.close_demands(body).unwrap(),d,"denied close step changed quote");}
            let((poll,_),heap)=observe(||driven::<OriginalRetirementFailure>(bank,g,w.max(1),|effect,reply,recover|if recover{receiver.recover_close_step(body,effect,reply)}else{receiver.close_step(body,effect,reply)}));post(bank,g,heap,poll).unwrap();
        }
    }
    panic!("receiver failed its funded close within {CLOSE_STEPS} steps");
}
/// 🗑️ Retires one owned value through its controlled retirement until terminal-empty.
pub fn retire<T:RetireOwned>(value:T,bank:&mut EffectBank){
    let mut owner=ControlledRetirement::new(value).map_err(|(error,_)|error).unwrap();
    for _ in 0..1_000_000{
        if owner.terminal_is_empty(){return}
        let d=owner.next_demand(RETIRE_BODY).unwrap();let w=owner.next_computational_work(RETIRE_BODY).unwrap();let g=RetainedCloneGrant{maximum_copy_bytes:RETIRE_BODY.max(d.copy_bytes),..grant(d)};
        let((poll,_),heap)=observe(||bank_driven::<OriginalRetirementFailure>(bank,g,w.max(1),|effect,reply,recover|if recover{owner.recover_step_into(RETIRE_BODY,effect,reply)}else{owner.step_into(RETIRE_BODY,effect,reply)}));post(bank,g,heap,poll).unwrap();
    }
    panic!("owned value failed its funded retirement");
}
/// 🗑️ Closes the genuine Source through its own paid close until terminal-empty.
pub fn close_source<I:RetireOwned+Sync+'static>(mut source:RetainedCloneSource<I>,bank:&mut EffectBank){
    for _ in 0..1_000_000{
        if source.terminal_is_empty(){return}
        let d=source.next_close_demand(RETIRE_BODY).unwrap();let w=source.next_close_work_demand(RETIRE_BODY).unwrap();let g=RetainedCloneGrant{maximum_copy_bytes:RETIRE_BODY.max(d.copy_bytes),..grant(d)};
        let((poll,_),heap)=observe(||bank_driven::<OriginalRetirementFailure>(bank,g,w.max(1),|effect,reply,recover|if recover{source.recover_close_into(RETIRE_BODY,effect,reply)}else{source.close_into(RETIRE_BODY,effect,reply)}));post(bank,g,heap,poll).unwrap();
    }
    panic!("Source failed its funded close");
}
/// 🔁️ One case at one cut: receive or cancel, then close every owner.
pub fn lifecycle<I:RetireOwned+Sync+'static,T:RetireOwned,R:NativeReceiver<I,T>>(mut receiver:R,source:RetainedCloneSource<I>,bank:&mut EffectBank,body:usize,cut:Option<u64>,inspect:impl FnOnce(&T))->Result<bool,String>{
    let received=receive_cut(&mut receiver,source.borrow(),bank,body,cut);
    let outcome=match received{Ok(Some(output))=>{inspect(&output);retire(output,bank);Ok(true)},Ok(None)=>Ok(false),Err(error)=>Err(error)};
    close(&mut receiver,bank,body);assert!(receiver.terminal_is_empty());drop(receiver);close_source(source,bank);outcome
}

