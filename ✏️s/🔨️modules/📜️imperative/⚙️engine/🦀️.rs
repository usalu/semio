//! ⚙️ Path/Step data model and the sequential executor that runs a `Path` against a `neural_engine::Registry`.

use neural_engine::{Atom, ColdRetire, Dictionary, Registry, Value, SCHEMA_KEY};
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const MAX_NESTING_DEPTH: usize = 64;
const MAX_LOOP_ITERATIONS: u64 = 200_000;

// #region 🔖️Path
/// 👣️ One ordered side-effect invocation. `serde` is TEST-ONLY (RUNTIME-DEPENDENCY-ELIMINATION-
/// FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01, tenth-seam pass): `params: Dictionary` needs `Dictionary:
/// Serialize`, which lost its own unconditional derive this pass — see `📓️orderedmap-tenth-seam.md`.
/// No oracle test exists for `Step`/`Path`/`EffectLogEntry`/`RunResult` here, so the derive is
/// dropped outright rather than kept cross-crate-invisible under `#[cfg_attr(test, ...)]` (`serde`
/// gated inside `neural_engine`'s own crate never becomes visible from this crate's `cfg(test)` —
/// cfg(test) does not cross a crate boundary).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Step {
    pub id: String,
    pub kind: String,
    #[value(default)]
    pub params: Dictionary,
    #[value(default)]
    pub bodies: BTreeMap<String, Path>,
}

/// 🛤️ Ordered list of steps — position is execution order within a scope. No `serde` — see `Step`
/// above.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Path {
    pub steps: Vec<Step>,
}

/// 🧊️ A step IS the cold boundary of the `params` dictionary it owns — the same statement
/// `neural_engine::ColdOwner` makes, made by the document type itself.
///
/// `neural_engine::Dictionary` drops fail-closed: the FINAL owner of a non-empty pair root must be
/// retired explicitly or the drop panics (`final Dictionary ownership must be explicitly retired or
/// owned by a cold boundary`). A `Step` is cloned, moved into and out of `Path`s, parked inside a
/// composed child's local owner, carried through mutation payloads and edit history, and dropped by
/// the framework wherever a snapshot dies — there is no single call site that could own that
/// retirement, and in the wasm guest a missed one is an abort, not a test failure. Declaring the
/// boundary here retires `params` exactly once, on the last owner, and recurses through `bodies`
/// (each nested `Path`'s `Step`s retire their own) without any caller ceremony.
///
/// Because `Step` now has a `Drop`, its fields can no longer be moved out individually — assign
/// (`step.params = …`) or `std::mem::take` them instead.
impl Drop for Step {
    fn drop(&mut self) {
        neural_engine::ColdRetire::retire_cold(std::mem::take(&mut self.params));
    }
}

/// 🧊️ Kept so `Path`/`Vec<Step>`/`BTreeMap<_, Step>` still satisfy `ColdRetire`; the work itself is
/// [`Step`]'s own `Drop` boundary above, so consuming the step IS the retirement.
impl neural_engine::ColdRetire for Step {
    fn retire_cold(self) {
        drop(self);
    }
}

/// 🧊️ Retires every step of the path.
impl neural_engine::ColdRetire for Path {
    fn retire_cold(self) {
        self.steps.retire_cold();
    }
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
        // 🧊️ The DISPLACED dictionary is retired, never dropped: a bare `self.params = …` drops the
        // previous root in place, which panics as soon as this step was its last owner
        // (`final Dictionary ownership must be explicitly retired or owned by a cold boundary`).
        ColdRetire::retire_cold(std::mem::replace(&mut self.params, patch.clone()));
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
/// 📋️ One recorded side-effect from a step execution. No `serde` — see `Step`'s docstring above.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct EffectLogEntry {
    pub step_id: String,
    pub kind: String,
    pub input: Dictionary,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<Dictionary>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 📦️ Result of running a path. No `serde` — see `Step`'s docstring above.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
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
                        let next = scope.clone().insert("index", Value::Atom(Atom::Integer(index as i64)));
                        replace_scope_cold(scope, next);
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
                let next = merge_output_into_scope(scope, &output);
                replace_scope_cold(scope, next);
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

/// 🧊️ Hands one dictionary owner over to its successor. `Dictionary::drop` panics `final Dictionary
/// ownership must be explicitly retired or owned by a cold boundary` whenever the value being dropped
/// is the LAST owner of a non-empty pair root, so every rebind of a scope — every `*scope = …`, every
/// `merged = merged.merge(…)` — has to retire the displaced root instead of letting it fall out of
/// scope. Measured: every `run`/`stop` verb of `🎬️sequence` and every imperative executor step that
/// merged a non-empty operator output aborted the test binary here.
fn replace_scope_cold(scope: &mut Dictionary, next: Dictionary) {
    std::mem::replace(scope, next).retire_cold();
}

/// 🧊️ [`replace_scope_cold`] for a plain owner rebind.
fn replaced_cold(previous: Dictionary, next: Dictionary) -> Dictionary {
    previous.retire_cold();
    next
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
                    let next = merged.merge(payload);
                    merged = replaced_cold(merged, next);
                    continue;
                }
                if key == "message" || key == "delay" {
                    let next = merged.merge(payload);
                    merged = replaced_cold(merged, next);
                    continue;
                }
            }
            let next = merged.insert(key.clone(), value.clone());
            merged = replaced_cold(merged, next);
        }
    }
    merged
}
// #endregion 🔖️Executor

// #region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests
