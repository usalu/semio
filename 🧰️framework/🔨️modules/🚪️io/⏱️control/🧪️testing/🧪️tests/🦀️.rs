use super::*;
use semio_framework_value::{DslValue,FromValue,retirement::{RetireOwned,controlled::ControlledRetirement},ValueRefusalKind};
#[global_allocator]
static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
fn retire<T:RetireOwned>(value:T,grant:RetainedCloneGrant){let mut owner=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("original fixture retirement"));for _ in 0..1_000_000{if owner.terminal_is_empty(){return}assert!(owner.step(grant).unwrap().progress().fits(grant));}panic!("fixture retirement stalled");}
#[test]
fn scoped_original_decoding_preserves_budget_cancellation_and_closes_its_actual_recipient(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let g=&law["grant"];let n=|key:&str|usize::try_from(g[key].as_u64().unwrap()).unwrap();let grant=RetainedCloneGrant{maximum_items:n("items"),maximum_copy_bytes:n("copy"),maximum_capacity_bytes:n("capacity"),maximum_release_bytes:n("release"),maximum_depth:n("depth")};
 for row in law["cases"].as_array().unwrap(){let text=row["unit"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap()as usize);let input=DslValue::String(text.clone());let cancel=row["cancelAt"].as_u64().map(|value|value as usize);let mut observed=false;let mut observer=|progress:NativeDecodeProgress|{let accepted=cancel.is_none_or(|at|progress.completed<at);observed|=!accepted;accepted};let(result,receipt)=with_decoding(row["maximum"].as_u64().unwrap()as usize,grant,&mut observer,|control|{if row["returnSlot"].as_bool().unwrap(){control.with_retirement_owner::<String,semio_framework_value::ValueError>(std::mem::size_of::<ControlledRetirement<String>>(),|control|{let mut output=String::new();match control.copy_text_into(&text,&mut output){Ok(())=>(Ok(output),None),Err(error)=>(Err(error),Some(Box::new(ControlledRetirement::new(output).unwrap())))}})}else{String::from_value_controlled(&input,control)}});assert!(receipt.ledger!=0&&receipt.recipient!=0);assert!(receipt.retirement.fits(grant));match row["expected"].as_str().unwrap(){"ok"=>{let value=result.unwrap();assert_eq!(value,text);assert!(!observed);retire(value,grant);},"limit"=>assert_eq!(result.unwrap_err().kind,ValueRefusalKind::OwnershipLimit),"cancel"=>{assert_eq!(result.unwrap_err().kind,ValueRefusalKind::Canceled);assert!(observed);},_=>unreachable!()}if row["returnSlot"].as_bool().unwrap(){assert!(receipt.semantic_owned_bytes>0);assert!(receipt.retirement_steps>0);}retire(input,grant);retire(text,grant);}
}

use std::sync::{Arc,Mutex,atomic::{AtomicBool,AtomicUsize,Ordering}};
use semio_framework_value::{ErasedSnapshotRetirement,RetainedCloneStep,ValueError};
struct Watched{owner:ControlledRetirement<String>,events:Arc<Mutex<Vec<&'static str>>>,released:Arc<AtomicUsize>}
impl ErasedSnapshotRetirement for Watched{
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let step=self.owner.close_step(grant)?;self.released.fetch_add(step.progress().released_bytes,Ordering::SeqCst);Ok(step)}
 fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_copy_byte_demand()}
 fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.owner.next_capacity_byte_demand(body)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_release_byte_demand()}
 fn next_depth_demand(&self)->Result<usize,ValueError>{self.owner.next_depth_demand()}
 fn next_demand(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{self.owner.next_demand(body)}
}
impl Drop for Watched{fn drop(&mut self){assert!(self.owner.terminal_is_empty(),"panic payload cursor was not closed");self.events.lock().unwrap().push("closed");}}
#[test]
fn semantic_and_observer_panics_resume_the_same_payload_only_after_original_owner_closure(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let g=&law["grant"];let n=|key:&str|g[key].as_u64().unwrap()as usize;let grant=RetainedCloneGrant{maximum_items:n("items"),maximum_copy_bytes:n("copy"),maximum_capacity_bytes:n("capacity"),maximum_release_bytes:n("release"),maximum_depth:n("depth")};
 for row in law["panicCases"].as_array().unwrap(){
  let events=Arc::new(Mutex::new(Vec::with_capacity(3)));let released=Arc::new(AtomicUsize::new(0));let charged=Arc::new(AtomicUsize::new(0));let payload=Arc::new(17usize);let signal=payload.clone();let log=events.clone();let retirement=released.clone();let debits=charged.clone();let text=row["text"].as_str().unwrap();let observer_panic=row["phase"]=="observer";let reserved=row["frontier"]=="reserved";
  let result=std::panic::catch_unwind(move||{
   let armed=Arc::new(AtomicBool::new(false));let callback_armed=armed.clone();let callback_signal=signal.clone();let mut observer=move|_:NativeDecodeProgress|{if observer_panic&&callback_armed.load(Ordering::SeqCst){std::panic::panic_any(callback_signal.clone());}true};
   with_decoding(1<<20,grant,&mut observer,|control|{
    let fail=|control:&mut NativeDecodeControl<'_>|->!{armed.store(true,Ordering::SeqCst);if observer_panic{control.checkpoint().unwrap();}std::panic::panic_any(signal.clone())};
    if reserved{
     let _=control.with_retirement_owner::<(),ValueError>(std::mem::size_of::<Watched>(),|control|{debits.store(control.owned_bytes(),Ordering::SeqCst);log.lock().unwrap().push("reserved");fail(control)});
     unreachable!();
    }
    let error=control.with_retirement_owner::<(),ValueError>(std::mem::size_of::<Watched>(),|control|{let value=control.copy_text(text).unwrap();let owner=Watched{owner:ControlledRetirement::new(value).unwrap(),events:log.clone(),released:retirement};(Err(ValueError::literal(ValueRefusalKind::InvalidValue,"actual original owner retained")),Some(Box::new(owner)))}).unwrap_err();
    assert_eq!(error.kind,ValueRefusalKind::InvalidValue);assert!(control.has_retirement_owner());debits.store(control.owned_bytes(),Ordering::SeqCst);log.lock().unwrap().push("retained");fail(control);
   });
  });
  let actual=result.expect_err("the original panic must resume").downcast::<Arc<usize>>().expect("original panic payload type");assert!(Arc::ptr_eq(&actual,&payload));events.lock().unwrap().push("resumed");assert_eq!(*events.lock().unwrap(),row["events"].as_array().unwrap().iter().map(|value|value.as_str().unwrap()).collect::<Vec<_>>());assert!(charged.load(Ordering::SeqCst)>=std::mem::size_of::<Watched>());if reserved{assert_eq!(released.load(Ordering::SeqCst),0);}else{assert!(released.load(Ordering::SeqCst)>=text.len());}
 }
}
#[test]
fn empty_scoped_control_keeps_original_identity_without_a_retirement_turn(){
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:64};let mut yes=|_|true;let(identity,receipt)=with_decoding(0,grant,&mut yes,|control|(control.ledger_identity(),control.retirement_recipient_identity().unwrap()));assert_eq!(identity,(receipt.ledger,receipt.recipient));assert_eq!(receipt.semantic_owned_bytes,0);assert_eq!(receipt.retirement_steps,0);assert_eq!(receipt.retirement,Default::default());
}

#[test]
fn exact_semantic_allocation_limit_keeps_independent_original_cleanup_funding(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let g=&law["grant"];let n=|key:&str|g[key].as_u64().unwrap()as usize;let grant=RetainedCloneGrant{maximum_items:n("items"),maximum_copy_bytes:n("copy"),maximum_capacity_bytes:n("capacity"),maximum_release_bytes:n("release"),maximum_depth:n("depth")};let count=law["cleanup"]["rows"].as_u64().unwrap()as usize;let text=law["cleanup"]["text"].as_str().unwrap();type Cursor=ControlledRetirement<Vec<Vec<String>>>;let exact=std::mem::size_of::<Cursor>()+count*(std::mem::size_of::<Vec<String>>()+std::mem::size_of::<String>()+text.len());
 for case in law["cleanup"]["cases"].as_array().unwrap(){let maximum=exact-case["shortBy"].as_u64().unwrap()as usize;let mut yes=|_|true;let(result,receipt)=with_decoding(maximum,grant,&mut yes,|control|control.with_retirement_owner::<(),ValueError>(std::mem::size_of::<Cursor>(),|control|{let mut rows=Vec::new();let result=(||{rows=control.allocate_vec::<Vec<String>>(count)?;for _ in 0..count{rows.push(control.allocate_vec::<String>(1)?);rows.last_mut().unwrap().push(control.copy_text(text)?);}Ok(())})();(result,Some(Box::new(ControlledRetirement::new(rows).unwrap())))}));match case["expected"].as_str().unwrap(){"ok"=>{result.unwrap();assert_eq!(receipt.semantic_owned_bytes,exact);},"limit"=>assert_eq!(result.unwrap_err().kind,ValueRefusalKind::OwnershipLimit),_=>unreachable!()}assert!(receipt.semantic_owned_bytes<=maximum);assert!(receipt.retirement_steps>0);assert!(receipt.retirement.retained_capacity_bytes>0);assert!(receipt.retirement.fits(grant));}
}
