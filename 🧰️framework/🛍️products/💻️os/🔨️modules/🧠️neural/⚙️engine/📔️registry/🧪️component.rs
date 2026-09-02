//! 🧪️ Shared readers, exact final ownership, nested defaults, and domain-specific operator retirement.

use super::*;
use crate::{ColdOwner, Dictionary, EvalError, Operator, OperatorImpl, OperatorInfo, Registry, Schema, ValueRetirement, ValueRetirementStep};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

//#region 🧪️SharedRegistry
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️.json")).unwrap();
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
            match retirement.close_step(1, maximum_bytes).unwrap() {
                ValueRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1 && released_bytes <= maximum_bytes);
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
        if retirement.close_step(1, 1).unwrap() == ValueRetirementStep::Complete { break; }
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
        if let Err(reason) = retirement.close_step(1, 64) {
            assert_eq!(reason, "neural.operator-retirement-not-implemented");
            refused = true;
            break;
        }
    }
    assert!(refused && retirement.operator.is_some() && !retirement.terminal_is_empty());
    retirement.operator.take().unwrap().retire_cold();
    for _ in 0..1000 {
        if retirement.close_step(1, 64).unwrap() == ValueRetirementStep::Complete { break; }
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️.json")).unwrap();
    let drops = Arc::new(AtomicUsize::new(0));
    let mut registry = Registry::new();
    registry.register_operator(OperatorInfo::default(), vec![OperatorImpl { schemas: vec![], operator: Box::new(FaultingOperator { text: fixture["payload"].as_str().unwrap().into(), drops: Arc::clone(&drops) }) }], &[]);
    let (reader, mut supervisor) = SharedRegistry::new(registry);
    drop(reader);
    let fault = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        for _ in 0..1000 { supervisor.close_step(1, 1).unwrap(); }
    }));
    assert!(fault.is_err() && !supervisor.terminal_is_empty());
    assert_eq!(drops.load(Ordering::SeqCst), 0);
    let mut bytes = 0;
    for _ in 0..1000 {
        match supervisor.close_step(1, 1).unwrap() {
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
