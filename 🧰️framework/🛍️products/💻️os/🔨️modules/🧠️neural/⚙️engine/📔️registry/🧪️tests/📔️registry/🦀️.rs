//! 🧪️ Shared readers, exact final ownership, nested defaults, and domain-specific operator retirement.

use super::*;
use crate::{ColdOwner, Dictionary, EvalError, Operator, OperatorImpl, OperatorInfo, Registry, Schema, ValueRetirement, ValueRetirementStep};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

//#region 🧪️SharedRegistry
#[derive(Clone,Copy,Default)]
struct ShellObservation { recording:bool,count:usize,addresses:[usize;8],sizes:[usize;8],released:usize }
thread_local! {static SHELL_OBSERVATION:std::cell::Cell<ShellObservation>=const {std::cell::Cell::new(ShellObservation {recording:false,count:0,addresses:[0;8],sizes:[0;8],released:0})};}
thread_local! {static OWNERSHIP_OBSERVATION:std::cell::Cell<Option<(usize,usize)>>=const {std::cell::Cell::new(None)};}
pub(crate) fn observe_ownership<T>(operation:impl FnOnce()->T)->(T,usize,usize) {
    OWNERSHIP_OBSERVATION.with(|state|assert!(state.replace(Some((0,0))).is_none()));
    let value=operation();
    let (allocated,released)=OWNERSHIP_OBSERVATION.with(|state|state.replace(None).unwrap());
    (value,allocated,released)
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

#[test]
fn registry_physical_shells_retire_only_under_exact_allocator_byte_grants() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let law=&fixture["physicalShell"];
    SHELL_OBSERVATION.with(|state|state.set(ShellObservation {recording:true,..Default::default()}));let (reader,mut retirement)=SharedRegistry::new(Registry::new());let original=SHELL_OBSERVATION.with(|state|{let mut observed=state.get();observed.recording=false;state.set(observed);observed});
    let alias=reader.clone();assert_eq!(reader.owner_identity(),alias.owner_identity());assert_eq!(serde_json::to_value(&reader.schemas).unwrap(),serde_json::json!({}));drop(reader);let shared_frozen=SHELL_OBSERVATION.with(|state|state.get().released);drop(alias);let ungranted=SHELL_OBSERVATION.with(|state|state.get().released);
    let demand=*original.sizes[..original.count].iter().max().unwrap();let undersized=*original.sizes[..original.count].iter().min().unwrap()-1;let zero=retirement.close_step(0,demand).unwrap();let zero_bytes=retirement.close_step(1,0).unwrap();let before_small=SHELL_OBSERVATION.with(|state|state.get().released);let mut undersized_blocked=false;for _ in 0..16 {if retirement.close_step(1,undersized).unwrap()==ValueRetirementStep::Blocked {undersized_blocked=true;break;}if retirement.terminal_is_empty(){break;}}let after_small=SHELL_OBSERVATION.with(|state|state.get().released);
    let mut charged=0;let mut receipts_exact=true;for _ in 0..100 {if retirement.terminal_is_empty(){break;}let before=SHELL_OBSERVATION.with(|state|state.get().released);if let ValueRetirementStep::Pending {released_bytes,..}=retirement.close_step(1,demand).unwrap(){charged+=released_bytes;receipts_exact&=released_bytes==SHELL_OBSERVATION.with(|state|state.get().released)-before;}}
    let physical=SHELL_OBSERVATION.with(|state|state.get().released);assert!(retirement.terminal_is_empty());assert_eq!(original.count,law["allocations"].as_u64().unwrap()as usize);assert_eq!(shared_frozen,0);assert_eq!(ungranted,law["releasedBeforeGrant"].as_u64().unwrap()as usize);assert_eq!(zero,ValueRetirementStep::Blocked);assert_eq!(zero_bytes,ValueRetirementStep::Blocked);assert_eq!(before_small,after_small);assert!(undersized_blocked);assert!(receipts_exact);assert_eq!(charged,physical);assert_eq!(physical,original.sizes[..original.count].iter().sum::<usize>());println!("[DEBUG] Existing registry shells allocatorBytes={physical}, chargedBytes={charged}, zero and undersized grants frozen");
}

#[test]
fn registry_execution_owner_identity_survives_the_original_source_reader() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let law=&fixture["executionOwner"];let bytes=law["retirementGrantBytes"].as_u64().unwrap()as usize;
    let mut a=Registry::new();a.register_schema(serde_json::from_value::<Schema>(fixture["schema"].clone()).unwrap());let mut b=Registry::new();b.register_schema(serde_json::from_value::<Schema>(fixture["schema"].clone()).unwrap());let (a,mut a_retirement)=SharedRegistry::new(a);let (b,mut b_retirement)=SharedRegistry::new(b);let a_source=a.clone();let b_source=b.clone();
    assert_eq!(a.owner_identity()==a_source.owner_identity(),law["sameReaderSameIdentity"].as_bool().unwrap());assert_eq!(a.owner_identity()!=b.owner_identity(),law["distinctOwnerDifferentIdentity"].as_bool().unwrap());let a_identity=a.owner_identity();drop(a);assert_eq!(a_source.owner_identity(),a_identity);assert_eq!(serde_json::to_value(a_source.schema("sample").unwrap()).unwrap(),fixture["schema"]);assert_eq!(a_retirement.close_step(1,bytes).unwrap(),ValueRetirementStep::Blocked);drop(a_source);while !a_retirement.terminal_is_empty(){let grant=bytes.max(a_retirement.next_close_byte_demand());if let ValueRetirementStep::Pending {released_bytes,..}=a_retirement.close_step(1,grant).unwrap(){assert!(released_bytes<=grant);}}
    assert_eq!(serde_json::to_value(b_source.schema("sample").unwrap()).unwrap(),fixture["schema"]);assert_eq!(b.owner_identity(),b_source.owner_identity());assert_eq!(b_retirement.close_step(1,bytes).unwrap(),ValueRetirementStep::Blocked);drop(b);drop(b_source);while !b_retirement.terminal_is_empty(){let grant=bytes.max(b_retirement.next_close_byte_demand());if let ValueRetirementStep::Pending {released_bytes,..}=b_retirement.close_step(1,grant).unwrap(){assert!(released_bytes<=grant);}}println!("[DEBUG] Original registry source readers preserve exact execution ownership across independent three-byte domain retirement and admitted physical shells");
}

struct OwnedOperator { text: String, drops: Arc<AtomicUsize> }
impl Operator for OwnedOperator {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> { Ok(input.clone()) }
    fn retirement_is_empty(&self) -> bool { self.text.is_empty() }
    fn retire_step(&mut self, maximum_items: usize, maximum_bytes: usize, values: &mut ValueRetirement) -> Result<ValueRetirementStep, &'static str> {
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(ValueRetirementStep::Blocked); }
        values.text(std::mem::take(&mut self.text));
        Ok(ValueRetirementStep::Complete)
    }
}
impl Drop for OwnedOperator { fn drop(&mut self) { assert!(self.text.is_empty()); self.drops.fetch_add(1, Ordering::SeqCst); } }

#[test]
fn registry_fixture_preserves_readers_and_retires_every_exact_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let mut expected_bytes = None;
    for grant in fixture["grants"].as_array().unwrap() {
        let maximum_bytes = grant.as_u64().unwrap() as usize;
        let drops = Arc::new(AtomicUsize::new(0));
        let mut registry = Registry::new();
        registry.register_schema(serde_json::from_value::<Schema>(fixture["schema"].clone()).unwrap());
        registry.register_operator(
            serde_json::from_value::<OperatorInfo>(fixture["operator"].clone()).unwrap(),
            vec![OperatorImpl {
                schemas: serde_json::from_value(fixture["implementationSchemas"].clone()).unwrap(),
                operator: Box::new(OwnedOperator { text: fixture["payload"].as_str().unwrap().into(), drops: Arc::clone(&drops) }),
            }],
            &["sample"],
        );
        registry.finalize();
        let (root, mut retirement) = SharedRegistry::new(registry);
        let readers = [root.clone(), root.clone()];
        assert_eq!(serde_json::to_value(root.schema("sample").unwrap()).unwrap(), fixture["schema"]);
        assert_eq!(serde_json::to_value(root.operator_info("fixture.echo").unwrap()).unwrap(), fixture["operator"]);
        assert_eq!(retirement.close_step(1, maximum_bytes).unwrap(), ValueRetirementStep::Blocked);
        drop(root);
        let [first, last] = readers;
        drop(first);
        assert_eq!(retirement.close_step(1, maximum_bytes).unwrap(), ValueRetirementStep::Blocked);
        assert_eq!(serde_json::to_value(last.schema("sample").unwrap()).unwrap(), fixture["schema"]);
        drop(last);
        assert_eq!(drops.load(Ordering::SeqCst), fixture["expected"]["dropsBeforeRetirement"].as_u64().unwrap() as usize);
        assert_eq!(retirement.close_step(0, maximum_bytes).unwrap(), ValueRetirementStep::Blocked);
        assert_eq!(retirement.close_step(1, 0).unwrap(), ValueRetirementStep::Blocked);
        let mut bytes = 0;
        for _ in 0..100_000 {
            let grant=maximum_bytes.max(retirement.next_close_byte_demand());
            match retirement.close_step(1, grant).unwrap() {
                ValueRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1 && released_bytes <= grant);
                    bytes += released_bytes;
                }
                ValueRetirementStep::Complete => break,
                ValueRetirementStep::Blocked => panic!("unique registry did not advance"),
            }
        }
        assert!(retirement.terminal_is_empty());
        assert_eq!(retirement.close_step(1, maximum_bytes).unwrap(), ValueRetirementStep::Complete);
        assert!(bytes > fixture["payload"].as_str().unwrap().len());
        if let Some(expected) = expected_bytes { assert_eq!(bytes, expected); } else { expected_bytes = Some(bytes); }
        assert_eq!(drops.load(Ordering::SeqCst), fixture["expected"]["dropsAfterRetirement"].as_u64().unwrap() as usize);
    }
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn final_registry_reader_handoff_is_exact_across_workers() {
    let mut registry = Registry::new();
    registry.register_schema(Schema { id: "shared".into(), ..Default::default() });
    let (root, mut retirement) = SharedRegistry::new(registry);
    let readers: Vec<_> = (0..8).map(|_| root.clone()).collect();
    drop(root);
    let workers: Vec<_> = readers.into_iter().map(|reader| std::thread::spawn(move || {
        assert_eq!(reader.schema("shared").unwrap().id, "shared");
        drop(reader);
    })).collect();
    for worker in workers { worker.join().unwrap(); }
    for _ in 0..1000 {
        if retirement.close_step(1,retirement.next_close_byte_demand().max(1)).unwrap() == ValueRetirementStep::Complete { break; }
    }
    assert!(retirement.terminal_is_empty());
}

#[test]
fn raw_registry_cold_boundary_remains_explicit() {
    let mut registry = Registry::new();
    registry.register_schema(Schema { id: "cold".into(), ..Default::default() });
    drop(ColdOwner::new(registry));
}

struct UnspecifiedOperator { _text: String }
impl Operator for UnspecifiedOperator { fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> { Ok(input.clone()) } }

#[test]
fn dynamic_payload_without_retirement_authority_is_retained_and_rejected() {
    let mut registry = Registry::new();
    registry.register_operator(OperatorInfo { id: "unspecified".into(), ..Default::default() }, vec![OperatorImpl { schemas: vec![], operator: Box::new(UnspecifiedOperator { _text: "owned".into() }) }], &[]);
    let (root, mut retirement) = SharedRegistry::new(registry);
    drop(root);
    let mut refused = false;
    for _ in 0..1000 {
        if let Err(reason) = retirement.close_step(1,retirement.next_close_byte_demand().max(64)) {
            assert_eq!(reason, "neural.operator-retirement-not-implemented");
            refused = true;
            break;
        }
    }
    assert!(refused && retirement.operator.is_some() && !retirement.terminal_is_empty());
    retirement.operator.take().unwrap().retire_cold();
    for _ in 0..1000 {
        if retirement.close_step(1,retirement.next_close_byte_demand().max(64)).unwrap() == ValueRetirementStep::Complete { break; }
    }
    assert!(retirement.terminal_is_empty());
}

struct FaultingOperator { text: String, drops: Arc<AtomicUsize> }
impl Operator for FaultingOperator {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> { Ok(input.clone()) }
    fn retirement_is_empty(&self) -> bool { self.text.is_empty() }
    fn retire_step(&mut self, _: usize, _: usize, values: &mut ValueRetirement) -> Result<ValueRetirementStep, &'static str> {
        values.text(std::mem::take(&mut self.text));
        panic!("fixture fault after payload handoff");
    }
}
impl Drop for FaultingOperator { fn drop(&mut self) { assert!(self.text.is_empty()); self.drops.fetch_add(1, Ordering::SeqCst); } }

#[test]
fn supervising_cursor_recovers_operator_fault_after_exact_payload_handoff() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let drops = Arc::new(AtomicUsize::new(0));
    let mut registry = Registry::new();
    registry.register_operator(OperatorInfo::default(), vec![OperatorImpl { schemas: vec![], operator: Box::new(FaultingOperator { text: fixture["payload"].as_str().unwrap().into(), drops: Arc::clone(&drops) }) }], &[]);
    let (reader, mut supervisor) = SharedRegistry::new(registry);
    drop(reader);
    let fault = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        for _ in 0..1000 { supervisor.close_step(1,supervisor.next_close_byte_demand().max(1)).unwrap(); }
    }));
    assert!(fault.is_err() && !supervisor.terminal_is_empty());
    assert_eq!(drops.load(Ordering::SeqCst), 0);
    let mut bytes = 0;
    for _ in 0..1000 {
        match supervisor.close_step(1,supervisor.next_close_byte_demand().max(1)).unwrap() {
            ValueRetirementStep::Pending { released_bytes, .. } => bytes += released_bytes,
            ValueRetirementStep::Complete => break,
            ValueRetirementStep::Blocked => panic!("recovered supervisor must make progress"),
        }
    }
    assert!(supervisor.terminal_is_empty());
    assert_eq!(bytes, fixture["payload"].as_str().unwrap().len());
    assert_eq!(drops.load(Ordering::SeqCst), 1);
}
//#endregion 🧪️SharedRegistry
