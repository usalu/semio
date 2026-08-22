//! ⚙️ Path/Step data model and the sequential executor that runs a `Path` against a `neural_engine::Registry`.

use neural_engine::{Atom, Dictionary, Registry, Value, SCHEMA_KEY};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const MAX_NESTING_DEPTH: usize = 64;
const MAX_LOOP_ITERATIONS: u64 = 200_000;

// #region 🔖️Path
/// 👣️ One ordered side-effect invocation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Step {
    pub id: String,
    pub kind: String,
    #[serde(default)]
    pub params: Dictionary,
    #[serde(default)]
    pub bodies: BTreeMap<String, Path>,
}

/// 🛤️ Ordered list of steps — position is execution order within a scope.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Path {
    pub steps: Vec<Step>,
}

impl Path {
    pub fn new() -> Self {
        Self::default()
    }
}

impl protocol::Identified<String> for Step {
    fn id(&self) -> &String {
        &self.id
    }
}

/// @emoji 🩹️ `protocol::Patchable`'s split shape: `apply_patch` mutates only (no returned inverse —
/// `protocol_command::invert_collection_operation` recomputes the inverse from a prior snapshot via
/// `diff_patch` instead); `diff_patch` reports `None` when `params` is unchanged, matching this same
/// full-replace semantics as `vcs::Patchable`'s impl above.
impl protocol::Patchable<Dictionary> for Step {
    fn apply_patch(&mut self, patch: &Dictionary) {
        self.params = patch.clone();
    }

    fn diff_patch(&self, other: &Self) -> Option<Dictionary> {
        if self.params == other.params {
            None
        } else {
            Some(other.params.clone())
        }
    }
}
// #endregion 🔖️Path

// #region 🔖️EffectLog
/// 📋️ One recorded side-effect from a step execution.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectLogEntry {
    pub step_id: String,
    pub kind: String,
    pub input: Dictionary,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<Dictionary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 📦️ Result of running a path.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunResult {
    pub scope: Dictionary,
    pub effects: Vec<EffectLogEntry>,
}
// #endregion 🔖️EffectLog

// #region 🔖️Executor
/// ▶️ Sequential executor over an imperative path.
pub struct Executor<'a> {
    registry: &'a Registry,
}

impl<'a> Executor<'a> {
    pub fn new(registry: &'a Registry) -> Self {
        Self { registry }
    }

    /// Runs steps strictly in list order; merges each output into scope; halts on first error.
    pub fn run(&self, path: &Path, seed: &Dictionary) -> RunResult {
        let mut scope = seed.clone();
        let mut effects = Vec::new();
        self.run_steps(&path.steps, &mut scope, &mut effects, 0);
        RunResult { scope, effects }
    }

    fn run_steps(&self, steps: &[Step], scope: &mut Dictionary, effects: &mut Vec<EffectLogEntry>, depth: usize) {
        if depth > MAX_NESTING_DEPTH {
            effects.push(EffectLogEntry { step_id: String::new(), kind: "control.depth".into(), input: Dictionary::new(), output: None, error: Some(format!("nesting depth exceeded {MAX_NESTING_DEPTH}")) });
            return;
        }
        for step in steps {
            if let Some(halt) = self.run_step(step, scope, effects, depth) {
                if halt {
                    break;
                }
            }
        }
    }

    fn run_step(&self, step: &Step, scope: &mut Dictionary, effects: &mut Vec<EffectLogEntry>, depth: usize) -> Option<bool> {
        match step.kind.as_str() {
            "control.if" => {
                let key = read_string_param(&step.params, "key").unwrap_or_default();
                let condition = read_scope_bool(scope, &key);
                let slot = if condition { "then" } else { "else" };
                let input = scope.merge(&step.params);
                effects.push(EffectLogEntry { step_id: step.id.clone(), kind: step.kind.clone(), input, output: Some(Dictionary::new().insert("branch", Value::Atom(Atom::String(slot.into())))), error: None });
                if let Some(body) = step.bodies.get(slot) {
                    self.run_steps(&body.steps, scope, effects, depth + 1);
                }
                return None;
            }
            "control.while" => {
                let key = read_string_param(&step.params, "key").unwrap_or_default();
                let mut iterations = 0u64;
                while read_scope_bool(scope, &key) {
                    iterations += 1;
                    if iterations > MAX_LOOP_ITERATIONS {
                        effects.push(EffectLogEntry { step_id: step.id.clone(), kind: step.kind.clone(), input: scope.merge(&step.params), output: None, error: Some(format!("while loop exceeded {MAX_LOOP_ITERATIONS} iterations")) });
                        return Some(true);
                    }
                    if let Some(body) = step.bodies.get("body") {
                        self.run_steps(&body.steps, scope, effects, depth + 1);
                    }
                }
                return None;
            }
            "control.repeat" => {
                let count = read_number_param(&step.params, "count").unwrap_or(0.0).max(0.0) as u64;
                let capped = count.min(MAX_LOOP_ITERATIONS);
                if count > MAX_LOOP_ITERATIONS {
                    effects.push(EffectLogEntry { step_id: step.id.clone(), kind: step.kind.clone(), input: scope.merge(&step.params), output: None, error: Some(format!("repeat count capped at {MAX_LOOP_ITERATIONS}")) });
                }
                if let Some(body) = step.bodies.get("body") {
                    for index in 0..capped {
                        *scope = scope.clone().insert("index", Value::Atom(Atom::Integer(index as i64)));
                        self.run_steps(&body.steps, scope, effects, depth + 1);
                    }
                }
                return None;
            }
            _ => {}
        }
        let input = scope.merge(&step.params);
        match self.registry.dispatch(&step.kind, &input) {
            Ok(output) => {
                *scope = merge_output_into_scope(scope, &output);
                effects.push(EffectLogEntry { step_id: step.id.clone(), kind: step.kind.clone(), input, output: Some(output), error: None });
                None
            }
            Err(err) => {
                effects.push(EffectLogEntry { step_id: step.id.clone(), kind: step.kind.clone(), input, output: None, error: Some(err.to_string()) });
                Some(true)
            }
        }
    }
}

/// 🔑️ Shared with `crate::compiler` — both the executor and the text emitter read `key`/`count` params
/// the same way, so this stays `pub(crate)` rather than duplicated.
pub(crate) fn read_string_param(params: &Dictionary, key: &str) -> Option<String> {
    params.get(key).and_then(|v| v.as_atom()).and_then(|a| a.as_str()).map(str::to_string)
}

pub(crate) fn read_number_param(params: &Dictionary, key: &str) -> Option<f64> {
    params.get(key).and_then(|v| v.as_atom()).and_then(|a| a.as_f64())
}

fn read_scope_bool(scope: &Dictionary, key: &str) -> bool {
    scope.get(key).and_then(|v| v.as_atom()).and_then(|a| a.as_bool()).unwrap_or(false)
}

fn merge_output_into_scope(scope: &Dictionary, output: &Dictionary) -> Dictionary {
    let mut merged = scope.clone();
    for key in output.keys() {
        if key == SCHEMA_KEY {
            continue;
        }
        if let Some(value) = output.get(key) {
            if let Some(payload) = value.as_dictionary() {
                if payload.len() == 1 && payload.get(SCHEMA_KEY).is_some() {
                    merged = merged.merge(payload);
                    continue;
                }
                if key == "message" || key == "delay" {
                    merged = merged.merge(payload);
                    continue;
                }
            }
            merged = merged.insert(key.clone(), value.clone());
        }
    }
    merged
}
// #endregion 🔖️Executor

// #region 🧪️Tests
#[cfg(test)]
mod tests {
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
            Path {
                steps: vec![Step { id: "t1".into(), kind: "state.set".into(), params: Dictionary::new().insert("key", Value::Atom(Atom::String("result".into()))).insert("value", Value::Atom(Atom::String("yes".into()))), bodies: BTreeMap::new() }],
            },
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
}
// #endregion 🧪️Tests
