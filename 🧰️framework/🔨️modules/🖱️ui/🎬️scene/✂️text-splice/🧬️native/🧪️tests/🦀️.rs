//! 🧪️ Actual original composition matches the scalar contract with bounded denial and physical cancellation closure.
use super::*;
use std::alloc::GlobalAlloc as _;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation};
use semio_framework_value::{RetainedCloneGrant,retirement::RetireOwned};
use semio_ui_scene::{TextSplice,TextSpliceComposition};
struct Refusing;
thread_local!{static REFUSE:std::cell::Cell<usize>=const{std::cell::Cell::new(0)}}
/// 🛡️ SAFETY: every method forwards the unchanged layout to the sound `HeapWitness` allocator or returns null for the one refused size, which the `GlobalAlloc` contract permits.
unsafe impl std::alloc::GlobalAlloc for Refusing{
 unsafe fn alloc(&self,layout:std::alloc::Layout)->*mut u8{if REFUSE.try_with(|size|size.get()==layout.size()).unwrap_or(false){return std::ptr::null_mut()}unsafe{semio_framework_trace::HeapWitness.alloc(layout)}}
 unsafe fn dealloc(&self,pointer:*mut u8,layout:std::alloc::Layout){unsafe{semio_framework_trace::HeapWitness.dealloc(pointer,layout)}}
 unsafe fn alloc_zeroed(&self,layout:std::alloc::Layout)->*mut u8{if REFUSE.try_with(|size|size.get()==layout.size()).unwrap_or(false){return std::ptr::null_mut()}unsafe{semio_framework_trace::HeapWitness.alloc_zeroed(layout)}}
 unsafe fn realloc(&self,pointer:*mut u8,layout:std::alloc::Layout,size:usize)->*mut u8{unsafe{semio_framework_trace::HeapWitness.realloc(pointer,layout,size)}}
}
#[global_allocator]static HEAP:Refusing=Refusing;
const TURN:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096};
fn fixture_caller()->OriginalJobFixtureCaller{OriginalJobFixtureCaller::admit(OperationId(6491),Generation(73),RetainedCloneGrant{maximum_items:1<<26,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:4096},semio_framework_job::default_now_us,u64::MAX,1<<26)}
fn turn<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,grant:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(grant,decoder,run));let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));result.unwrap()
}
fn view(s:&TextSplice)->TextSpliceView<'_>{TextSpliceView{start:s.start,deleted:&s.deleted,insert:&s.insert,before:&s.before,after:&s.after}}
fn json(s:&OriginalTextSpliceComposition)->serde_json::Value{match s{OriginalTextSpliceComposition::Cancelled=>serde_json::json!({"kind":"cancelled"}),OriginalTextSpliceComposition::Disjoint=>serde_json::json!({"kind":"disjoint"}),OriginalTextSpliceComposition::Composed(s)=>serde_json::json!({"kind":"composed","splice":{"start":s.start,"deleted":s.deleted,"insert":s.insert,"before":s.before,"after":s.after}})}}
fn expected(s:TextSpliceComposition)->serde_json::Value{match s{TextSpliceComposition::Cancelled=>serde_json::json!({"kind":"cancelled"}),TextSpliceComposition::Disjoint=>serde_json::json!({"kind":"disjoint"}),TextSpliceComposition::Composed(s)=>serde_json::json!({"kind":"composed","splice":s})}}
fn denied(d:RetirementDemand)->Vec<RetainedCloneGrant>{let mut grants=vec![RetainedCloneGrant{maximum_items:0,..TURN}];if d.copy_bytes>0{grants.push(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..TURN})}if d.capacity_bytes>0{grants.push(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..TURN})}if d.depth>0{grants.push(RetainedCloneGrant{maximum_depth:d.depth-1,..TURN})}grants}
fn retire<T:RetireOwned+Send+'static>(value:T,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){let mut value=ControlledRetirement::retain(value);for i in 0..1000000{if value.terminal_is_empty(){return}let d=value.next_demand(TURN.maximum_copy_bytes).unwrap();turn(caller,decoder,TURN,|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|g|value.step(g))?;Ok(())}));assert!(i<999999)}}
fn close(owner:&mut Option<Box<OriginalTextSpliceComposer>>,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){
 for i in 0..1000000{if owner.is_none(){return}turn(caller,decoder,TURN,|cx|cx.with_original_retirement(|cx|{if owner.as_ref().unwrap().terminal_is_empty(){let d=OriginalTextSpliceComposer::release_demands();close_original_retained_step(cx,d,|_|{owner.take();Ok(RetainedCloneStep::Complete(receipt(d)))})?;}else{owner.as_mut().unwrap().close_step(cx)?;}Ok(())}));assert!(i<999999)}
}
fn finish(mut caller:OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){while !decoder.terminal_is_empty(){let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(TURN,|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(TURN.maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|decoder.close_step(g))?;Ok(())})));result.unwrap();let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));}caller.retire(TURN);}
fn run(a:&TextSplice,b:&TextSplice,context:usize,probe:bool)->serde_json::Value{
 let mut caller=fixture_caller();let mut decoder=OriginalJobDecodeOwner::empty();let mut owner=None;let a_ptr=a.insert.as_ptr();let b_ptr=b.insert.as_ptr();
 if probe{for grant in denied(OriginalTextSpliceComposer::birth_demands()){assert!(!turn(&mut caller,&mut decoder,grant,|cx|OriginalTextSpliceComposer::admit(view(a),view(b),context,cx,&mut owner)));assert!(owner.is_none());assert_eq!(caller.receipt(),Default::default());}}
 while owner.is_none(){turn(&mut caller,&mut decoder,TURN,|cx|OriginalTextSpliceComposer::admit(view(a),view(b),context,cx,&mut owner));assert!(owner.is_some()||caller.receipt()!=RetainedCloneProgress::default());}
 if probe{let mut foreign_caller=fixture_caller();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||foreign_caller.turn_with_decoder(TURN,&mut decoder,|cx|{assert!(owner.as_mut().unwrap().advance(view(a),view(b),cx).is_err());Ok(())}));if result.is_err(){assert_eq!(foreign_caller.receipt(),RetainedCloneProgress::default())}assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));foreign_caller.retire(TURN);let mut foreign=OriginalJobDecodeOwner::empty();turn(&mut caller,&mut foreign,TURN,|cx|{assert!(owner.as_mut().unwrap().advance(view(a),view(b),cx).is_err());Ok(())});assert!(foreign.terminal_is_empty());let mut changed=b.clone();changed.start=changed.start.wrapping_add(1);turn(&mut caller,&mut decoder,TURN,|cx|{assert!(owner.as_mut().unwrap().advance(view(a),view(&changed),cx).is_err());Ok(())});assert_eq!(caller.receipt(),Default::default());}
 for i in 0..1000000{if owner.as_ref().unwrap().ready(){break}if probe{let d=owner.as_ref().unwrap().demands(view(a),view(b),TURN.maximum_copy_bytes).unwrap();for grant in denied(d){assert_eq!(turn(&mut caller,&mut decoder,grant,|cx|owner.as_mut().unwrap().advance(view(a),view(b),cx)),OriginalTextSpliceAdvance::Unfunded);assert_eq!(caller.receipt(),Default::default());}}
 assert_ne!(turn(&mut caller,&mut decoder,TURN,|cx|owner.as_mut().unwrap().advance(view(a),view(b),cx)),OriginalTextSpliceAdvance::Unfunded);assert!(i<999999);}
 let mut result=None;let d=owner.as_ref().unwrap().take_demands().unwrap();if probe{for grant in denied(d){assert!(!turn(&mut caller,&mut decoder,grant,|cx|owner.as_mut().unwrap().take_into(&mut result,cx)));assert!(result.is_none());assert_eq!(caller.receipt(),Default::default());}}
 assert!(turn(&mut caller,&mut decoder,TURN,|cx|owner.as_mut().unwrap().take_into(&mut result,cx)));let actual=json(result.as_ref().unwrap());assert_eq!(a.insert.as_ptr(),a_ptr);assert_eq!(b.insert.as_ptr(),b_ptr);close(&mut owner,&mut caller,&mut decoder);retire(result,&mut caller,&mut decoder);finish(caller,&mut decoder);actual
}
#[test]
fn original_text_splice_matches_fixture_typing_runs_and_generated_cold_algebras(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/✂️text-splice/🔣️.json")).unwrap();let spec:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in fixture["compositions"].as_array().unwrap(){let a:TextSplice=serde_json::from_value(row["net"].clone()).unwrap();let b:TextSplice=serde_json::from_value(row["next"].clone()).unwrap();let actual=run(&a,&b,32,true);assert_eq!(actual,row["composition"]);assert_eq!(actual,expected(a.then(&b,32)));}
 for row in fixture["typingRuns"].as_array().unwrap(){let texts:Vec<String>=serde_json::from_value(row["texts"].clone()).unwrap();let mut net:Option<TextSplice>=None;let mut runs=Vec::new();for pair in texts.windows(2){let Some(edit)=TextSplice::from_edit(&pair[0],&pair[1],32)else{continue};if let Some(old)=net.take(){let actual=run(&old,&edit,32,false);assert_eq!(actual,expected(old.then(&edit,32)));match actual["kind"].as_str().unwrap(){"composed"=>net=Some(serde_json::from_value(actual["splice"].clone()).unwrap()),"cancelled"=>{},_=>{runs.push(old);net=Some(edit)}}}else{net=Some(edit)}}if let Some(net)=net{runs.push(net)}assert_eq!(serde_json::to_value(runs).unwrap(),row["runs"]);}
 let alphabet=["a","b","😀","é","́"];let mut random=spec["generative"]["seed"].as_u64().unwrap();let next_random=|state:&mut u64|{*state=state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);*state};let text=|state:&mut u64|{let n=(next_random(state)%25)as usize;(0..n).map(|_|alphabet[(next_random(state)%5)as usize]).collect::<String>()};
 let mut completed=0;while completed<spec["generative"]["cases"].as_u64().unwrap(){let old=text(&mut random);let middle=text(&mut random);let new=text(&mut random);let context=(next_random(&mut random)%41)as usize;let(Some(a),Some(b))=(TextSplice::from_edit(&old,&middle,context),TextSplice::from_edit(&middle,&new,context))else{continue};let actual=run(&a,&b,context,false);assert_eq!(actual,expected(a.then(&b,context)));completed+=1;}
}
#[test]
fn original_text_splice_cancel_closes_every_partial_owner_without_changing_inputs(){
 let spec:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let old="a".repeat(1024);let middle=format!("{}{}{}","a".repeat(512),"😀".repeat(512),"a".repeat(512));let new=format!("{}{}{}{}","a".repeat(512),"😀".repeat(512),"é".repeat(512),"a".repeat(512));let a=TextSplice::from_edit(&old,&middle,32).unwrap();let b=TextSplice::from_edit(&middle,&new,32).unwrap();let pointers=(a.insert.as_ptr(),b.insert.as_ptr());
 for cut in spec["cancelCuts"].as_array().unwrap(){let mut caller=fixture_caller();let mut decoder=OriginalJobDecodeOwner::empty();let mut owner=None;while owner.is_none(){turn(&mut caller,&mut decoder,TURN,|cx|OriginalTextSpliceComposer::admit(view(&a),view(&b),32,cx,&mut owner));}for _ in 0..cut.as_u64().unwrap(){if owner.as_ref().unwrap().ready(){break}turn(&mut caller,&mut decoder,TURN,|cx|owner.as_mut().unwrap().advance(view(&a),view(&b),cx));}caller.cancel().cancel_now();close(&mut owner,&mut caller,&mut decoder);assert_eq!((a.insert.as_ptr(),b.insert.as_ptr()),pointers);finish(caller,&mut decoder);}
}

static EXPIRY_CLOCK:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1);
fn expiry_clock()->Option<u64>{Some(EXPIRY_CLOCK.load(std::sync::atomic::Ordering::SeqCst))}
#[test]
fn original_text_splice_expired_source_closes_without_renewing_its_authority(){
 EXPIRY_CLOCK.store(1,std::sync::atomic::Ordering::SeqCst);let mut caller=OriginalJobFixtureCaller::admit(OperationId(6492),Generation(73),RetainedCloneGrant{maximum_items:1<<26,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:4096},expiry_clock,10,1<<26);let mut decoder=OriginalJobDecodeOwner::empty();let a=TextSplice{start:0,deleted:String::new(),insert:"😀".repeat(512),before:String::new(),after:String::new()};let b=TextSplice{start:512,deleted:String::new(),insert:"é".repeat(512),before:"😀".repeat(32),after:String::new()};let mut owner=None;
 while owner.is_none(){turn(&mut caller,&mut decoder,TURN,|cx|OriginalTextSpliceComposer::admit(view(&a),view(&b),32,cx,&mut owner));}
 for _ in 0..1500{assert!(!owner.as_ref().unwrap().ready());turn(&mut caller,&mut decoder,TURN,|cx|owner.as_mut().unwrap().advance(view(&a),view(&b),cx));}
 let phase=(owner.as_ref().unwrap().cursor.phase,owner.as_ref().unwrap().cursor.field,owner.as_ref().unwrap().cursor.byte);EXPIRY_CLOCK.store(11,std::sync::atomic::Ordering::SeqCst);
 turn(&mut caller,&mut decoder,TURN,|cx|{if let Ok(ready)=owner.as_mut().unwrap().advance(view(&a),view(&b),cx){assert_ne!(ready,OriginalTextSpliceAdvance::Ready)}Ok(())});assert_eq!(caller.receipt(),Default::default());assert_eq!((owner.as_ref().unwrap().cursor.phase,owner.as_ref().unwrap().cursor.field,owner.as_ref().unwrap().cursor.byte),phase);close(&mut owner,&mut caller,&mut decoder);finish(caller,&mut decoder);
}
#[test]
fn original_text_splice_allocator_exhaustion_refuses_admission_instead_of_aborting(){
 let a=TextSplice{start:0,deleted:String::new(),insert:"x".into(),before:String::new(),after:String::new()};let mut caller=fixture_caller();let mut decoder=OriginalJobDecodeOwner::empty();let mut owner=None;
 REFUSE.with(|size|size.set(size_of::<OriginalTextSpliceComposer>()));let result=caller.turn_with_decoder(TURN,&mut decoder,|cx|OriginalTextSpliceComposer::admit(view(&a),view(&a),32,cx,&mut owner));REFUSE.with(|size|size.set(0));
 assert!(result.is_err(),"refused allocation must surface as a refusal: {:?}",result.as_ref().map(|_|()));assert!(owner.is_none());finish(caller,&mut decoder);
}
