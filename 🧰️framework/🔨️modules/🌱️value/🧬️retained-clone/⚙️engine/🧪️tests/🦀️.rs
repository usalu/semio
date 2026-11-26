//! 🧪️ Fixed native leaves use real finite Work and preserve actual destructor events.
use super::*;
use semio_framework_value_control_contract::{RetainedAllocationEffect,RetainedCloneStep,RetainedCloneProgress,RetirementDemand,ValueError,OriginalEffectReply,OriginalEffectReplyComplete,RetainedWorkPoll,RetainedWorkOutcome,RetainedCloneGrant};
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
fn bank(law:&serde_json::Value)->EffectBank{EffectBank::new(Wallet::from_profile(&serde_json::json!({"items":32,"copyBytes":65536,"capacityBytes":65536,"releaseBytes":65536,"depth":8,"fuel":law["fuel"]})))}

#[test]
fn clone_backing_separates_reservation_initialization_and_cancellation(){
    type Payload=[u8;2048];
    let law:serde_json::Value=serde_json::from_str(include_str!("copy-release.json")).unwrap();
    let mut bank=bank(&law);
    let bytes=law["cloneBacking"]["payloadBytes"].as_u64().unwrap() as usize;
    let maximum=law["cloneBacking"]["maximumTurnBytes"].as_u64().unwrap() as usize;
    let mut backing=CloneBacking::<Payload>::default();
    let mut output=None;
    let mut turn=|demand:RetirementDemand,work,action:&mut dyn FnMut(&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>|{
        assert!(demand.copy_bytes+demand.capacity_bytes+demand.release_bytes<=maximum);
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
        let ((poll,actual),heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||bank.run(grant,work,|effect|{
            let before=effect.performed_work();
            let result=action(effect);
            RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:effect.performed_work()-before})
        }));
        let RetainedWorkPoll::Complete(outcome)=poll else{panic!("clone backing left a pending reply")};
        let progress=outcome.result.unwrap().progress();
        assert_eq!(outcome.work,u128::from(progress.copied_items as u64));
        assert_eq!(actual,(progress.retained_capacity_bytes>0).then_some(progress.retained_capacity_bytes));
        assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));
        bank.take_admitted();
        bank.post(progress,grant,heap);
        progress
    };
    let reserve=backing.birth_demand();
    let mut short=reserve;
    short.capacity_bytes-=1;
    assert_eq!(turn(short,1,&mut |effect|backing.birth(&mut output,||panic!("denied reserve cannot initialize"),effect)),Default::default());
    assert!(!backing.is_reserved());
    assert_eq!(turn(reserve,1,&mut |effect|backing.birth(&mut output,||panic!("raw reserve cannot initialize"),effect)).retained_capacity_bytes,bytes);
    assert!(backing.is_reserved()&&output.is_none());
    let release=backing.release_demand();
    assert_eq!(turn(release,0,&mut |effect|backing.release(effect)),Default::default());
    assert!(backing.is_reserved());
    assert_eq!(turn(release,1,&mut |effect|backing.release(effect)).released_bytes,bytes);
    assert!(!backing.is_reserved());
    turn(reserve,1,&mut |effect|backing.birth(&mut output,||panic!("raw reserve cannot initialize"),effect));
    let initialize=backing.birth_demand();
    let mut short=initialize;
    short.copy_bytes-=1;
    assert_eq!(turn(short,1,&mut |effect|backing.birth(&mut output,||panic!("denied initialize cannot write"),effect)),Default::default());
    assert!(backing.is_reserved()&&output.is_none());
    assert_eq!(turn(initialize,1,&mut |effect|backing.birth(&mut output,||[17;2048],effect)).copied_bytes,initialize.copy_bytes);
    assert!(!backing.is_reserved());
    assert!(output.as_deref().unwrap().iter().all(|value|*value==17));
    let release=unsafe{RetainedCursorRelease::native(|_:&Payload|true)};
    assert_eq!(turn(release.boxed_demand(),1,&mut |effect|{
        let mut reply=None;
        release.boxed_into(&mut output,effect,OriginalEffectReply::borrow(&mut reply)?)?;
        reply.unwrap()
    }).released_bytes,bytes);
    assert!(output.is_none());
}
fn native(bank:&mut EffectBank,demand:RetirementDemand,maximum:u64,operation:impl for<'r>FnOnce(&mut RetainedAllocationEffect<'_>,OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>)->RetainedCloneProgress{let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let mut reply=None;let((poll,marker),heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||bank.run(grant,maximum,|effect|{let before=effect.performed_work();let result=operation(effect,OriginalEffectReply::borrow(&mut reply).unwrap());let result=match result{Ok(complete)=>{drop(complete);reply.take().unwrap()},Err(error)=>Err(error)};RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:effect.performed_work()-before})}));assert_eq!(marker,None);let RetainedWorkPoll::Complete(turn)=poll else{panic!("native leaf cannot defer a Source reply")};let progress=turn.result.unwrap().progress();assert_eq!(bank.take_admitted(),if turn.work==0{None}else{Some(true)});bank.post(progress,grant,heap);progress}
#[test]
fn copy_witness_publishes_only_funded_resident_arrays_and_reserved_vectors(){let law:serde_json::Value=serde_json::from_str(include_str!("copy-release.json")).unwrap();let source:[u64;3]=law["values"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()).collect::<Vec<_>>().try_into().unwrap();let witness=RetainedCopy::<u64>::new();let mut bank=bank(&law);let mut output=None;let demand=witness.array_demand::<3>().unwrap();let work=witness.array_work::<3>().unwrap();assert_eq!(work,law["arrayWork"].as_u64().unwrap());let denied=native(&mut bank,demand,0,|effect,reply|witness.array_into(&source,&mut output,effect,reply));assert_eq!(denied,RetainedCloneProgress::default());assert!(output.is_none());let mut short=demand;short.copy_bytes-=1;native(&mut bank,short,work,|effect,reply|witness.array_into(&source,&mut output,effect,reply));assert!(output.is_none());let progress=native(&mut bank,demand,work,|effect,reply|witness.array_into(&source,&mut output,effect,reply));assert_eq!(progress.copied_bytes,demand.copy_bytes);assert_eq!(output.unwrap(),source.clone());let nested=witness.array_witness::<3>();let mut nested_output=None;native(&mut bank,nested.array_demand::<1>().unwrap(),nested.array_work::<1>().unwrap(),|effect,reply|nested.array_into(&[source],&mut nested_output,effect,reply));assert_eq!(nested_output,Some([source]));let mut vector=Vec::with_capacity(source.len());let capacity=vector.capacity();native(&mut bank,witness.extend_demand(source.len()).unwrap(),witness.extend_work(source.len()).unwrap(),|effect,reply|witness.extend_into(&source,&mut vector,effect,reply));assert_eq!(vector,source.to_vec());assert_eq!(vector.capacity(),capacity);let mut prefix=NativeArrayPrefix::<u64,3>::new();let mut input=Some(source[0]);native(&mut bank,NativeArrayPrefix::<u64,3>::element_demand().unwrap(),1,|effect,reply|prefix.push_from(&mut input,effect,reply));assert!(input.is_none());assert_eq!(prefix.as_slice(),&source[..1]);let mut popped=None;native(&mut bank,NativeArrayPrefix::<u64,3>::element_demand().unwrap(),1,|effect,reply|prefix.pop_into(&mut popped,effect,reply));assert_eq!(popped,Some(source[0]));assert!(prefix.is_empty());let first=law["prefix"]["first"].as_u64().unwrap()as usize;let second=law["prefix"]["second"].as_u64().unwrap()as usize;assert_eq!(witness.prefix_work(first).unwrap(),law["prefix"]["firstWork"].as_u64().unwrap());native(&mut bank,witness.prefix_demand(first).unwrap(),witness.prefix_work(first).unwrap()-1,|effect,reply|witness.prefix_into(&source,&mut prefix,first,effect,reply));assert_eq!(prefix.initialized(),0);native(&mut bank,witness.prefix_demand(first).unwrap(),witness.prefix_work(first).unwrap(),|effect,reply|witness.prefix_into(&source,&mut prefix,first,effect,reply));assert_eq!(prefix.initialized(),first);native(&mut bank,witness.prefix_demand(second).unwrap(),witness.prefix_work(second).unwrap(),|effect,reply|witness.prefix_into(&source,&mut prefix,second,effect,reply));assert_eq!(prefix.initialized(),source.len());let mut published=None;native(&mut bank,witness.prefix_publication_demand::<3>().unwrap(),law["prefix"]["publicationWork"].as_u64().unwrap(),|effect,reply|witness.publish_prefix_into(&mut prefix,&mut published,effect,reply));assert_eq!(published,Some(source));assert_eq!(prefix.initialized(),0);}
#[test]
fn explicit_native_cursor_release_runs_the_real_terminal_destructor(){use std::sync::atomic::{AtomicUsize,Ordering};static DROPS:AtomicUsize=AtomicUsize::new(0);struct Cursor{empty:bool}impl Drop for Cursor{fn drop(&mut self){assert!(self.empty);DROPS.fetch_add(1,Ordering::SeqCst);}}let law:serde_json::Value=serde_json::from_str(include_str!("copy-release.json")).unwrap();DROPS.store(0,Ordering::SeqCst);let release=unsafe{RetainedCursorRelease::native(|cursor:&Cursor|cursor.empty)};let mut slot=Some(Cursor{empty:true});let mut bank=bank(&law);native(&mut bank,release.inline_demand(),0,|effect,reply|release.inline_into(&mut slot,effect,reply));assert!(slot.is_some());assert_eq!(DROPS.load(Ordering::SeqCst),0);native(&mut bank,release.inline_demand(),release.inline_work(),|effect,reply|release.inline_into(&mut slot,effect,reply));assert!(slot.is_none());assert_eq!(DROPS.load(Ordering::SeqCst),law["dropEvents"].as_u64().unwrap()as usize);let mut boxed=Some(Box::new(Cursor{empty:true}));let progress=native(&mut bank,release.boxed_demand(),release.inline_work(),|effect,reply|release.boxed_into(&mut boxed,effect,reply));assert_eq!(progress.released_bytes,std::mem::size_of::<Cursor>());assert!(boxed.is_none());assert_eq!(DROPS.load(Ordering::SeqCst),law["dropEvents"].as_u64().unwrap()as usize+1);}

#[test]fn copy_witness_drives_the_actual_source_bound_array_backing(){let law:serde_json::Value=serde_json::from_str(include_str!("copy-release.json")).unwrap();let mut bank=bank(&law);let mut backing=NativeArrayBacking::<(),3>::new();native(&mut bank,NativeArrayBacking::<(),3>::reserve_demand().unwrap(),1,|effect,reply|backing.reserve_into(effect,reply));let witness=RetainedCopy::<()>::new();let d=witness.backing_copy_demand(3).unwrap();let work=witness.backing_copy_work(3).unwrap();native(&mut bank,d,work,|effect,reply|witness.copy_into_backing(&[();3],3,&mut backing,effect,reply));assert_eq!(backing.as_slice().unwrap().len(),3);for _ in 0..3{let mut output=None;native(&mut bank,NativeArrayBacking::<(),3>::element_demand().unwrap(),1,|effect,reply|backing.pop_into(&mut output,effect,reply));assert_eq!(output,Some(()))}native(&mut bank,backing.release_demand().unwrap(),1,|effect,reply|backing.release_into(effect,reply));assert!(backing.terminal_is_empty());native(&mut bank,NativeArrayBacking::<(),3>::reserve_demand().unwrap(),1,|effect,reply|backing.reserve_into(effect,reply));native(&mut bank,witness.backing_copy_demand(3).unwrap(),witness.backing_copy_work(3).unwrap(),|effect,reply|witness.copy_into_backing(&[();3],3,&mut backing,effect,reply));native(&mut bank,witness.backing_discard_demand(3).unwrap(),witness.backing_discard_work(3).unwrap(),|effect,reply|witness.discard_from_backing(3,&mut backing,effect,reply));assert!(backing.as_slice().unwrap().is_empty());native(&mut bank,backing.release_demand().unwrap(),1,|effect,reply|backing.release_into(effect,reply));assert!(backing.terminal_is_empty());}
