//! 🧮️ Cold and retained selection share the declared numeric-order and homogeneous signature law.
use crate::{OperatorImpl, OperatorPlanCursor, OperatorPlanStep, Registry};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, ValueError};
use semio_os_kernel_neural_engine_model::{Atom, ColdOwner, Dictionary, EvalError, Operator, OperatorInfo, OperatorPlanAdmission, Value, VariadicSpec};
use std::sync::atomic::{AtomicUsize, Ordering};

static SELECTED: AtomicUsize = AtomicUsize::new(0);

struct Marker(usize);
impl Operator for Marker {
    fn evaluate(&self, _: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(Dictionary::new().insert("marker", Value::Atom(Atom::Integer(self.0 as i64))))
    }
    fn step_plan(&self, input: Dictionary, grant: RetainedCloneGrant) -> Result<(OperatorPlanAdmission, RetainedCloneProgress), (EvalError, Dictionary)> {
        let admitted = OperatorPlanAdmission::immediate(input, grant)?;
        SELECTED.store(self.0, Ordering::Relaxed);
        Ok(admitted)
    }
    fn next_plan_copy_byte_demand(&self, _: &Dictionary) -> Result<usize, ValueError> { Ok(0) }
    fn next_plan_capacity_byte_demand(&self, _: &Dictionary, _: usize) -> Result<usize, ValueError> { Ok(0) }
    fn next_plan_release_byte_demand(&self, _: &Dictionary) -> Result<usize, ValueError> { Ok(0) }
    fn next_plan_depth_demand(&self, _: &Dictionary) -> Result<usize, ValueError> { Ok(1) }
}

#[test]
fn original_variadic_signature_matches_cold_selection_without_allocating() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/variadic-signature.json")).unwrap();
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1, maximum_depth: 1, ..Default::default() };
    for row in fixture["cases"].as_array().unwrap() {
        let label = row["case"].as_str().unwrap();
        let expected = row["expected"].as_u64();
        let mut implementations = fixture["implementations"].as_array().unwrap().iter().map(|entry| OperatorImpl {
            schemas: serde_json::from_value(entry["schemas"].clone()).unwrap(),
            operator: Box::new(Marker(entry["marker"].as_u64().unwrap() as usize)),
        }).collect::<Vec<_>>();
        if row["fallback"].as_bool().unwrap() {
            implementations.push(OperatorImpl { schemas: Vec::new(), operator: Box::new(Marker(4)) });
        }
        let id = "fixture.variadic";
        let mut registry = Registry::new();
        registry.register_operator(OperatorInfo {
            id: id.into(),
            variadic_input: Some(VariadicSpec { slot_key: "items".into(), min: row["min"].as_u64().unwrap() as usize, max: row["max"].as_u64().map(|value| value as usize) }),
            ..Default::default()
        }, implementations, &[]);
        let registry = ColdOwner::new(registry);
        let mut items = Dictionary::new();
        for (key, schema) in row["items"].as_object().unwrap() {
            let value = match schema.as_str() {
                Some(schema) => Value::Dictionary(Dictionary::with_schema(schema)),
                None => Value::Atom(Atom::Null),
            };
            items = items.insert(key, value);
        }
        let original = Dictionary::new().insert("items", Value::Dictionary(items));
        let actual = registry.dispatch_cold(id, original.clone()).ok().and_then(|output| match output.get("marker") {
            Some(Value::Atom(Atom::Integer(value))) => Some(*value as u64),
            _ => None,
        });
        assert_eq!(actual, expected, "{label}: cold selection");

        let identity = original.original_root_identity();
        let mut input = Some(original);
        let mut cursor = OperatorPlanCursor::new();
        SELECTED.store(0, Ordering::Relaxed);
        let mut final_selection = None;
        for _ in 0..4096 {
            let quote = registry.next_dispatch_job_demands(id, &input, &cursor, 1).unwrap();
            if quote.copy_bytes > 0 {
                let denied = RetainedCloneGrant { maximum_copy_bytes: 0, ..grant };
                assert!(matches!(registry.dispatch_job(id, &mut input, &mut cursor, denied), Ok(OperatorPlanStep::Working)));
                assert_eq!(cursor.step_progress(), Default::default());
            }
            let (result, allocations) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| registry.dispatch_job(id, &mut input, &mut cursor, grant));
            assert_eq!((allocations.requested_bytes, allocations.released_bytes), (0, 0));
            assert!(cursor.step_progress().fits(grant), "{label}");
            match result {
                Ok(OperatorPlanStep::Working) => assert_eq!(input.as_ref().unwrap().original_root_identity(), identity),
                Ok(OperatorPlanStep::Admitted(OperatorPlanAdmission::Immediate(output))) => {
                    assert_eq!(output.original_root_identity(), identity);
                    assert!(input.is_none());
                    drop(ColdOwner::new(output));
                    final_selection = Some(Some(SELECTED.load(Ordering::Relaxed) as u64));
                    break;
                }
                Err(_) => {
                    assert_eq!(input.as_ref().unwrap().original_root_identity(), identity);
                    drop(ColdOwner::new(input.take().unwrap()));
                    final_selection = Some(None);
                    break;
                }
                _ => panic!("{label}: unexpected admission"),
            }
        }
        assert_eq!(final_selection, Some(expected), "{label}: retained selection");
    }
}
