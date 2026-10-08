//! 🧪️ Shared readers, exact final ownership, nested defaults, and domain-specific operator retirement.

use super::*;
use crate::{ColdOwner, Dictionary, EvalError, Operator, OperatorImpl, OperatorInfo, Registry, Schema, ValueRetirement,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

//#region 🧪️SharedRegistry
#[derive(Clone,Copy,Default)]
struct ShellObservation { recording:bool,count:usize,addresses:[usize;8],sizes:[usize;8],released:usize }
thread_local! {static SHELL_OBSERVATION:std::cell::Cell<ShellObservation>=const {std::cell::Cell::new(ShellObservation {recording:false,count:0,addresses:[0;8],sizes:[0;8],released:0})};}
thread_local! {static OWNERSHIP_OBSERVATION:std::cell::Cell<Option<(usize,usize)>>=const {std::cell::Cell::new(None)};}
struct OwnershipObservationScope {parent:Option<(usize,usize)>,active:bool}
impl OwnershipObservationScope {
    fn finish(&mut self)->(usize,usize) {
        let result=OWNERSHIP_OBSERVATION.with(|state|{let result=state.replace(None).unwrap();state.set(self.parent.map(|(allocated,released)|(allocated+result.0,released+result.1)));result});
        self.active=false;result
    }
}
impl Drop for OwnershipObservationScope {fn drop(&mut self){if self.active {self.finish();}}}
pub(crate) fn observe_ownership<T>(operation:impl FnOnce()->T)->(T,usize,usize) {
    let parent=OWNERSHIP_OBSERVATION.with(|state|state.replace(Some((0,0))));
    let mut scope=OwnershipObservationScope {parent,active:true};
    let value=operation();
    let (allocated,released)=scope.finish();
    (value,allocated,released)
}
#[test]
fn nested_original_system_observations_preserve_child_and_aggregate_receipts() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧵️retirement/🧫️fixtures/🎮️typed-owners/🔣️.json")).unwrap();
    let bytes=fixture["cases"][0]["reservedCapacity"].as_u64().unwrap()as usize;
    let ((text,child_birth,child_free),birth,freed)=observe_ownership(||observe_ownership(||String::with_capacity(bytes)));
    assert_eq!((birth,freed),(child_birth,child_free));assert_eq!((birth,freed),(text.capacity(),0));
    let (((),child_birth,child_free),birth,freed)=observe_ownership(||observe_ownership(||drop(text)));
    assert_eq!((birth,freed),(child_birth,child_free));assert_eq!((birth,freed),(0,bytes));
    let fault=std::panic::catch_unwind(||observe_ownership(||observe_ownership(||panic!("original observation fault"))));
    assert!(fault.is_err());assert_eq!(observe_ownership(||()).1,0);
}
struct ObservedSystem;
unsafe impl std::alloc::GlobalAlloc for ObservedSystem {
    unsafe fn alloc(&self,layout:std::alloc::Layout)->*mut u8 {
        let pointer=unsafe {std::alloc::GlobalAlloc::alloc(&std::alloc::System,layout)};
        if !pointer.is_null(){let _=SHELL_OBSERVATION.try_with(|state|{let mut observed=state.get();if observed.recording&&observed.count<observed.addresses.len(){let index=observed.count;observed.addresses[index]=pointer as usize;observed.sizes[index]=layout.size();observed.count+=1;state.set(observed);}});}
        if !pointer.is_null(){let _=OWNERSHIP_OBSERVATION.try_with(|state|{if let Some((allocated,released))=state.get(){state.set(Some((allocated+layout.size(),released)));}});}
        pointer
    }
    unsafe fn dealloc(&self,pointer:*mut u8,layout:std::alloc::Layout) {
        let _=OWNERSHIP_OBSERVATION.try_with(|state|{if let Some((allocated,released))=state.get(){state.set(Some((allocated,released+layout.size())));}});
        let _=SHELL_OBSERVATION.try_with(|state|{let mut observed=state.get();if let Some(index)=observed.addresses.iter().position(|address|*address==pointer as usize){observed.addresses[index]=0;observed.released+=layout.size();state.set(observed);}});
        unsafe {std::alloc::GlobalAlloc::dealloc(&std::alloc::System,pointer,layout)};
    }
}
#[global_allocator]
static OBSERVED_SYSTEM:ObservedSystem=ObservedSystem;

fn registry_grant(owner:&RegistryRetirement,copy:usize)->Result<RetainedCloneGrant,ValueError> {
    Ok(RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy)?,maximum_release_bytes:owner.next_release_byte_demand()?,maximum_depth:owner.next_depth_demand()?})
}
fn drain_registry(owner:&mut RegistryRetirement,copy:usize,maximum_turns:usize)->usize {
    let mut released=0;
    for turn in 0..maximum_turns {
        if owner.terminal_is_empty(){return released;}
        let grant=registry_grant(owner,copy).unwrap();let (step,born,freed)=observe_ownership(||owner.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(born,freed));released+=freed;
        assert!(step.progress().copied_items!=0||matches!(step,RetainedCloneStep::Complete(_)),"original registry grant stalled on turn {turn}");
    }
    panic!("original registry did not close");
}
#[test]
fn registry_physical_shells_retire_only_under_exact_allocator_byte_grants() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let law=&fixture["physicalShell"];
    SHELL_OBSERVATION.with(|state|state.set(ShellObservation {recording:true,..Default::default()}));let (reader,mut retirement)=SharedRegistry::new(Registry::new());let original=SHELL_OBSERVATION.with(|state|{let mut observed=state.get();observed.recording=false;state.set(observed);observed});
    let alias=reader.clone();assert_eq!(reader.owner_identity(),alias.owner_identity());assert_eq!(serde_json::to_value(&reader.schemas).unwrap(),serde_json::json!({}));
    let grant=registry_grant(&retirement,1).unwrap();let (waiting,born,freed)=observe_ownership(||retirement.close_step(grant).unwrap());assert_eq!(waiting.progress(),Default::default());assert_eq!((born,freed),(0,0));
    drop(reader);assert_eq!(SHELL_OBSERVATION.with(|state|state.get().released),0);drop(alias);assert_eq!(SHELL_OBSERVATION.with(|state|state.get().released),law["releasedBeforeGrant"].as_u64().unwrap()as usize);
    let mut charged=0;let mut denied=0;
    for _ in 0..100000 {
        if retirement.terminal_is_empty(){break;}let grant=registry_grant(&retirement,1).unwrap();
        let (zero,a,r)=observe_ownership(||retirement.close_step(RetainedCloneGrant {maximum_items:0,..grant}).unwrap());assert_eq!(zero.progress(),RetainedCloneProgress::default());assert_eq!((a,r),(0,0));
        if grant.maximum_release_bytes>0&&grant.maximum_capacity_bytes==0&&retirement.next_copy_byte_demand().unwrap()==0 {
            let (short,a,r)=observe_ownership(||retirement.close_step(RetainedCloneGrant {maximum_release_bytes:grant.maximum_release_bytes-1,..grant}).unwrap());assert_eq!(short.progress(),RetainedCloneProgress::default());assert_eq!((a,r),(0,0));denied+=1;
        }
        let (step,a,r)=observe_ownership(||retirement.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(a,r));charged+=r;
    }
    let physical=SHELL_OBSERVATION.with(|state|state.get().released);assert!(retirement.terminal_is_empty());assert_eq!(original.count,law["allocations"].as_u64().unwrap()as usize);assert!(denied>=2);assert_eq!(physical,original.sizes[..original.count].iter().sum::<usize>());assert!(charged>=physical);eprintln!("[DEBUG] Original registry shells={physical}, full physical closure={charged}, short whole releases denied={denied}");
}
#[test]
fn registry_execution_owner_identity_survives_the_original_source_reader() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let law=&fixture["executionOwner"];let copy=law["retirementGrantBytes"].as_u64().unwrap()as usize;
    let mut a=Registry::new();a.register_schema(serde_json::from_value::<Schema>(fixture["schema"].clone()).unwrap());let mut b=Registry::new();b.register_schema(serde_json::from_value::<Schema>(fixture["schema"].clone()).unwrap());let (a,mut a_retirement)=SharedRegistry::new(a);let (b,mut b_retirement)=SharedRegistry::new(b);let a_source=a.clone();let b_source=b.clone();
    assert_eq!(a.owner_identity()==a_source.owner_identity(),law["sameReaderSameIdentity"].as_bool().unwrap());assert_eq!(a.owner_identity()!=b.owner_identity(),law["distinctOwnerDifferentIdentity"].as_bool().unwrap());let identity=a.owner_identity();
    let mut a_lease=a.into_retirement();let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};let (step,born,freed)=observe_ownership(||a_lease.close_step(grant).unwrap());assert!(matches!(step,RetainedCloneStep::Complete(_)));assert_eq!((born,freed),(0,0));assert_eq!(a_source.owner_identity(),identity);assert_eq!(serde_json::to_value(a_source.schema("sample").unwrap()).unwrap(),fixture["schema"]);
    let grant=registry_grant(&a_retirement,copy).unwrap();assert_eq!(a_retirement.close_step(grant).unwrap().progress(),Default::default());drop(a_source);drain_registry(&mut a_retirement,copy,law["maximumRetirementTurns"].as_u64().unwrap()as usize);
    assert_eq!(b.owner_identity(),b_source.owner_identity());assert_eq!(serde_json::to_value(b_source.schema("sample").unwrap()).unwrap(),fixture["schema"]);drop(b);drop(b_source);drain_registry(&mut b_retirement,copy,law["maximumRetirementTurns"].as_u64().unwrap()as usize);
    eprintln!("[DEBUG] Original registry identity/reader lease retained through exact final source authority");
}
struct OwnedOperator {text:String,drops:Arc<AtomicUsize>}
impl Operator for OwnedOperator {
    fn evaluate(&self,input:&Dictionary)->Result<Dictionary,EvalError>{Ok(input.clone())}
    fn retirement_is_empty(&self)->bool{self.text.capacity()==0}
    fn next_retire_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
    fn next_retire_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_retire_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
    fn next_retire_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(!self.retirement_is_empty()))}
    fn retire_step(&mut self,grant:RetainedCloneGrant,values:&mut ValueRetirement)->Result<RetainedCloneStep,ValueError> {
        match values.text(std::mem::take(&mut self.text),grant){Ok(progress)=>Ok(RetainedCloneStep::Complete(progress)),Err((error,text))=>{self.text=text;Err(error)}}
    }
}
impl Drop for OwnedOperator{fn drop(&mut self){assert_eq!(self.text.capacity(),0);self.drops.fetch_add(1,Ordering::SeqCst);}}
#[test]
fn registry_fixture_preserves_readers_and_retires_every_exact_owner() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let mut expected_bytes=None;
    for copy in fixture["grants"].as_array().unwrap(){
        let copy=copy.as_u64().unwrap()as usize;let drops=Arc::new(AtomicUsize::new(0));let mut registry=Registry::new();registry.register_schema(serde_json::from_value::<Schema>(fixture["schema"].clone()).unwrap());registry.register_operator(serde_json::from_value::<OperatorInfo>(fixture["operator"].clone()).unwrap(),vec![OperatorImpl {schemas:serde_json::from_value(fixture["implementationSchemas"].clone()).unwrap(),operator:Box::new(OwnedOperator {text:fixture["payload"].as_str().unwrap().into(),drops:Arc::clone(&drops)})}],&["sample"]);registry.finalize();
        let (root,mut retirement)=SharedRegistry::new(registry);let readers=[root.clone(),root.clone()];assert_eq!(serde_json::to_value(root.schema("sample").unwrap()).unwrap(),fixture["schema"]);assert_eq!(serde_json::to_value(root.operator_info("fixture.echo").unwrap()).unwrap(),fixture["operator"]);
        let grant=registry_grant(&retirement,copy).unwrap();assert_eq!(retirement.close_step(grant).unwrap().progress(),Default::default());drop(root);let [first,last]=readers;drop(first);let grant=registry_grant(&retirement,copy).unwrap();assert_eq!(retirement.close_step(grant).unwrap().progress(),Default::default());assert_eq!(serde_json::to_value(last.schema("sample").unwrap()).unwrap(),fixture["schema"]);drop(last);assert_eq!(drops.load(Ordering::SeqCst),fixture["expected"]["dropsBeforeRetirement"].as_u64().unwrap()as usize);
        let bytes=drain_registry(&mut retirement,copy,100000);assert!(bytes>fixture["payload"].as_str().unwrap().len());if let Some(expected)=expected_bytes{assert_eq!(bytes,expected);}else{expected_bytes=Some(bytes);}assert_eq!(drops.load(Ordering::SeqCst),fixture["expected"]["dropsAfterRetirement"].as_u64().unwrap()as usize);
    }
}
#[test]
#[cfg(not(target_arch="wasm32"))]
fn final_registry_reader_handoff_is_exact_across_workers() {
    let mut registry=Registry::new();registry.register_schema(Schema {id:"shared".into(),..Default::default()});let (root,mut retirement)=SharedRegistry::new(registry);let readers:Vec<_>=(0..8).map(|_|root.clone()).collect();drop(root);let workers:Vec<_>=readers.into_iter().map(|reader|std::thread::spawn(move||{assert_eq!(reader.schema("shared").unwrap().id,"shared");drop(reader);})).collect();for worker in workers{worker.join().unwrap();}drain_registry(&mut retirement,1,100000);assert!(retirement.terminal_is_empty());
}
#[test]
fn raw_registry_cold_boundary_remains_explicit(){let mut registry=Registry::new();registry.register_schema(Schema {id:"cold".into(),..Default::default()});drop(ColdOwner::new(registry));}
struct UnspecifiedOperator{_text:String}
impl Operator for UnspecifiedOperator{fn evaluate(&self,input:&Dictionary)->Result<Dictionary,EvalError>{Ok(input.clone())}}
#[test]
fn dynamic_payload_without_retirement_authority_is_retained_and_rejected() {
    let text=String::from("owned");let pointer=text.as_ptr();let original=UnspecifiedOperator {_text:text};assert_eq!(original._text.as_ptr(),pointer);
    let mut owner=OperatorRetirement {operator:Some(Box::new(original)),values:Default::default()};
    let (result,born,freed)=observe_ownership(||owner.next_depth_demand());assert_eq!(result.unwrap_err().kind,ValueRefusalKind::UnsupportedOwner);assert_eq!((born,freed),(0,0));assert!(!owner.terminal_is_empty());assert!(owner.operator.is_some());
    owner.operator.take().unwrap().retire_cold();assert!(owner.terminal_is_empty());
}
struct FaultingOperator{text:String,drops:Arc<AtomicUsize>}
impl Operator for FaultingOperator {
    fn evaluate(&self,input:&Dictionary)->Result<Dictionary,EvalError>{Ok(input.clone())}
    fn retirement_is_empty(&self)->bool{self.text.capacity()==0}
    fn next_retire_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
    fn next_retire_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_retire_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
    fn next_retire_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(!self.retirement_is_empty()))}
    fn retire_step(&mut self,grant:RetainedCloneGrant,values:&mut ValueRetirement)->Result<RetainedCloneStep,ValueError> {values.text(std::mem::take(&mut self.text),grant).map_err(|(error,text)|{self.text=text;error})?;panic!("fixture fault after payload handoff");}
}
impl Drop for FaultingOperator{fn drop(&mut self){assert_eq!(self.text.capacity(),0);self.drops.fetch_add(1,Ordering::SeqCst);}}
#[test]
fn supervising_cursor_recovers_operator_fault_after_exact_payload_handoff() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let drops=Arc::new(AtomicUsize::new(0));let mut registry=Registry::new();registry.register_operator(OperatorInfo::default(),vec![OperatorImpl {schemas:vec![],operator:Box::new(FaultingOperator {text:fixture["payload"].as_str().unwrap().into(),drops:Arc::clone(&drops)})}],&[]);let (reader,mut supervisor)=SharedRegistry::new(registry);drop(reader);
    let fault=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{for _ in 0..100000{let grant=registry_grant(&supervisor,1).unwrap();supervisor.close_step(grant).unwrap();}}));assert!(fault.is_err()&&!supervisor.terminal_is_empty());assert_eq!(drops.load(Ordering::SeqCst),0);let bytes=drain_registry(&mut supervisor,1,100000);assert!(bytes>=fixture["payload"].as_str().unwrap().len());assert_eq!(drops.load(Ordering::SeqCst),1);
}
//#endregion 🧪️SharedRegistry
