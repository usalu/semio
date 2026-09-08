
use super::*;
use neural_engine::{Atom, EvalError, Operator, OperatorImpl, OperatorInfo};

enum TestOperator {
    Set,
    Increment,
    Log,
}

impl Operator for TestOperator {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        match self {
            Self::Set => {
                let key = read_string_param(input, "key").ok_or_else(|| EvalError::MissingInput("key".into()))?;
                Ok(Dictionary::new().insert(key, input.get("value").cloned().unwrap_or(Value::null())))
            }
            Self::Increment => {
                let key = read_string_param(input, "key").ok_or_else(|| EvalError::MissingInput("key".into()))?;
                let current = read_number_param(input, &key).unwrap_or(0.0);
                let by = read_number_param(input, "by").unwrap_or(1.0);
                Ok(Dictionary::new().insert(key, Value::Atom(Atom::Decimal(current + by))))
            }
            Self::Log => Ok(Dictionary::new()),
        }
    }
}

fn register_test_operator(registry: &mut Registry, id: &str, operator: TestOperator) {
    registry.register_operator(OperatorInfo { id: id.into(), ..Default::default() }, vec![OperatorImpl { schemas: vec![], operator: Box::new(operator) }], &[]);
}

fn test_registry() -> Registry {
    let mut registry = Registry::new();
    register_test_operator(&mut registry, "state.set", TestOperator::Set);
    register_test_operator(&mut registry, "state.increment", TestOperator::Increment);
    register_test_operator(&mut registry, "log.print", TestOperator::Log);
    registry.finalize();
    registry
}

#[semio_framework_async_macros::async_test]
async fn executor_runs_steps_in_order() {
    let registry = test_registry();
    let executor = Executor::new(&registry);
    let path = Path {
        steps: vec![
            Step { id: "s1".into(), kind: "state.set".into(), params: Dictionary::new().insert("key", Value::Atom(Atom::String("counter".into()))).insert("value", Value::Atom(Atom::Decimal(0.0))), bodies: BTreeMap::new() },
            Step { id: "s2".into(), kind: "state.increment".into(), params: Dictionary::new().insert("key", Value::Atom(Atom::String("counter".into()))).insert("by", Value::Atom(Atom::Decimal(3.0))), bodies: BTreeMap::new() },
            Step { id: "s3".into(), kind: "log.print".into(), params: Dictionary::new().insert("message", Value::Atom(Atom::String("done".into()))), bodies: BTreeMap::new() },
        ],
    };
    let result = executor.run(&path, &Dictionary::new());
    assert_eq!(result.effects.len(), 3);
    assert!(result.effects.iter().all(|entry| entry.error.is_none()));
    let counter = result.scope.get("counter").and_then(|v| v.as_atom()).and_then(|a| a.as_f64());
    assert_eq!(counter, Some(3.0));
}

#[semio_framework_async_macros::async_test]
async fn executor_runs_control_if_then_branch() {
    let registry = test_registry();
    let executor = Executor::new(&registry);
    let mut bodies = BTreeMap::new();
    bodies.insert(
        "then".into(),
        Path { steps: vec![Step { id: "t1".into(), kind: "state.set".into(), params: Dictionary::new().insert("key", Value::Atom(Atom::String("result".into()))).insert("value", Value::Atom(Atom::String("yes".into()))), bodies: BTreeMap::new() }] },
    );
    let path = Path {
        steps: vec![
            Step { id: "s1".into(), kind: "state.set".into(), params: Dictionary::new().insert("key", Value::Atom(Atom::String("flag".into()))).insert("value", Value::Atom(Atom::Boolean(true))), bodies: BTreeMap::new() },
            Step { id: "s2".into(), kind: "control.if".into(), params: Dictionary::new().insert("key", Value::Atom(Atom::String("flag".into()))), bodies },
        ],
    };
    let result = executor.run(&path, &Dictionary::new());
    let value = result.scope.get("result").and_then(|v| v.as_atom()).and_then(|a| a.as_str());
    assert_eq!(value, Some("yes"));
}

#[semio_framework_async_macros::async_test]
async fn executor_runs_control_repeat() {
    let registry = test_registry();
    let executor = Executor::new(&registry);
    let mut bodies = BTreeMap::new();
    bodies.insert(
        "body".into(),
        Path { steps: vec![Step { id: "b1".into(), kind: "state.increment".into(), params: Dictionary::new().insert("key", Value::Atom(Atom::String("counter".into()))).insert("by", Value::Atom(Atom::Decimal(1.0))), bodies: BTreeMap::new() }] },
    );
    let path = Path {
        steps: vec![
            Step { id: "s1".into(), kind: "state.set".into(), params: Dictionary::new().insert("key", Value::Atom(Atom::String("counter".into()))).insert("value", Value::Atom(Atom::Decimal(0.0))), bodies: BTreeMap::new() },
            Step { id: "s2".into(), kind: "control.repeat".into(), params: Dictionary::new().insert("count", Value::Atom(Atom::Decimal(3.0))), bodies },
        ],
    };
    let result = executor.run(&path, &Dictionary::new());
    let counter = result.scope.get("counter").and_then(|v| v.as_atom()).and_then(|a| a.as_f64());
    assert_eq!(counter, Some(3.0));
}
