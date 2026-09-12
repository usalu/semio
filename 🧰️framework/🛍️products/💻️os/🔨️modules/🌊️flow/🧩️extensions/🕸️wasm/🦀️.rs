//! 🔌️ Shared wasm extension glue for flow modules.

use neural_engine::{inject_channel_defaults, ColdOwner, ColdRetire, Dictionary, OperatorInfo, Registry, Schema};
use semio_framework_os_kernel::{DslValue, FromValue, ToValue};

// #region 🔖️Manifest
/// 📋️ `flow.extension` manifest encoded through the first-party value contract.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct FlowExtensionManifest {
    pub schema: String,
    pub id: String,
    pub name: String,
    pub version: String,
    pub activation_events: Vec<String>,
    pub contributes: FlowExtensionContributes,
}

/// 🎁️ Contributed extension surface.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct FlowExtensionContributes {
    pub schemas: Vec<Schema>,
    pub operators: Vec<OperatorInfo>,
    #[value(default)]
    pub widgets: Vec<FlowExtensionWidget>,
    #[value(default)]
    pub commands: Vec<FlowExtensionCommand>,
    #[value(default)]
    pub settings: Vec<FlowExtensionSetting>,
}

/// 🧩️ Declared widget contribution.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct FlowExtensionWidget {
    pub kind: String,
    pub name: String,
    pub summary: String,
}

/// ⌘️ Declared command contribution.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct FlowExtensionCommand {
    pub id: String,
    pub title: String,
}

/// ⚙️ Declared setting contribution. `default` is a `DslValue` directly (was `serde_json::Value`) —
/// the same tenth-seam pass; no bridge needed since `DslValue` already implements `ToValue`/`FromValue`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct FlowExtensionSetting {
    pub id: String,
    #[value(rename = "type")]
    pub setting_type: String,
    pub default: DslValue,
    pub description: String,
}

/// 🔢️ Builds the `DslValue` for a [`FlowExtensionSetting::default`] of a whole number — factored
/// out so extension crates never need to construct one themselves.
pub fn integer_setting_default(value: i64) -> DslValue {
    value.to_value()
}

/// 📦️ Builds a `flow.extension` JSON manifest from registry catalogue metadata.
#[allow(clippy::too_many_arguments, reason = "manifest needs id+name+version+registry+activation_events+widgets+commands+settings together; a params struct would ripple into every flow/module/*/rs call site outside this ticket's scope")]
pub fn build_manifest_json(id: &str, name: &str, version: &str, registry: &Registry, activation_events: Vec<String>, widgets: Vec<FlowExtensionWidget>, commands: Vec<FlowExtensionCommand>, settings: Vec<FlowExtensionSetting>) -> String {
    let mut manifest = FlowExtensionManifest {
        schema: "flow.extension".into(),
        id: id.into(),
        name: name.into(),
        version: version.into(),
        activation_events,
        contributes: FlowExtensionContributes { schemas: registry.schema_catalogue(), operators: registry.operator_catalogue(), widgets, commands, settings },
    };
    let encoded = crate::os_pack::json::to_json_string(&manifest);
    std::mem::take(&mut manifest.contributes.schemas).retire_cold();
    std::mem::take(&mut manifest.contributes.operators).retire_cold();
    encoded
}
// #endregion 🔖️Manifest

// #region 🔖️Evaluate
/// 🧮️ Evaluates an operator and returns JSON dictionary or `{ "error": ... }`.
pub fn evaluate_json(registry: &Registry, kind_id: &str, input_json: &str) -> String {
    let input: Dictionary = match crate::os_pack::json::from_json_str(input_json) {
        Ok(d) => d,
        Err(err) => return crate::os_pack::json::to_json_string(&DslValue::object([("error".to_string(), err.to_string().to_value())])),
    };
    let input = ColdOwner::new(match registry.operator_info(kind_id) {
        Some(info) => inject_channel_defaults(input, info),
        None => input,
    });
    match registry.dispatch(kind_id, &input) {
        Ok(out) => crate::os_pack::json::to_json_string(&*ColdOwner::new(out)),
        Err(err) => crate::os_pack::json::to_json_string(&DslValue::object([("error".to_string(), err.to_string().to_value())])),
    }
}

/// 🧮️ Evaluates a neural tree as a function and returns the out dictionary JSON or `{ "error": ... }`.
pub fn evaluate_function_json(registry: &Registry, tree_json: &str, in_dict_json: &str) -> String {
    let tree: neural_engine::Tree = match crate::os_pack::json::from_json_str(tree_json) {
        Ok(tree) => tree,
        Err(err) => return crate::os_pack::json::to_json_string(&DslValue::object([("error".to_string(), err.to_string().to_value())])),
    };
    let tree = ColdOwner::new(tree);
    let in_dict: Dictionary = match crate::os_pack::json::from_json_str(in_dict_json) {
        Ok(dict) => dict,
        Err(err) => return crate::os_pack::json::to_json_string(&DslValue::object([("error".to_string(), err.to_string().to_value())])),
    };
    let evaluator = neural_engine::Evaluator::new(registry);
    let in_dict = ColdOwner::new(in_dict);
    match evaluator.evaluate_function(&tree, &in_dict) {
        Ok(out) => crate::os_pack::json::to_json_string(&*ColdOwner::new(out)),
        Err(err) => crate::os_pack::json::to_json_string(&DslValue::object([("error".to_string(), err.to_string().to_value())])),
    }
}
/// 🔀️ Parses a WIT `extension::invoke` "evaluate" capability request and answers ONE budgeted
/// round trip of it — factored out of every flow extension's own guest bundle so those crates
/// never need `serde_json`/`serde::Deserialize` themselves
/// (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`),
/// and so the budget is declared in ONE place for all ten of them.
///
/// ⏱️ Request: `{operatorId, inputJson, nodeHash?, budget?, wallMicros?}`. Answer: the
/// [`EVALUATE_ENVELOPE_SCHEMA`] envelope. An operator with no sub-structure
/// ([`neural_engine::Operator::step_plan`] answering `None` — which is every operator but the three
/// brep set operations) completes inside the first round trip and is indistinguishable from the
/// unbudgeted call it replaces. An operator that DOES offer a job is retained by
/// `(operatorId, nodeHash)` and resumed by the next identical request, exactly the way a budgeted
/// `tessellate` is resumed by the next `flowEvalTick`
/// (ticket `26/09/09/PROCEDURAL-3D-END-TO-END`, `📓️extension-evaluate-budget-2026-09-12.md`).
pub fn evaluate_invoke_json(registry: &Registry, request: &[u8]) -> Result<Vec<u8>, String> {
    let text = std::str::from_utf8(request).map_err(|error| error.to_string())?;
    let request: EvaluateRequest = crate::os_pack::json::from_json_str(text).map_err(|error| error.to_string())?;
    Ok(evaluate_step_envelope_json(registry, &request).into_bytes())
}

/// 📇️ The `evaluate` capability's request, as the wire declares it.
#[derive(FromValue)]
#[value(rename_all = "camelCase")]
pub struct EvaluateRequest {
    pub operator_id: String,
    pub input_json: String,
    /// 🪪️ The requester's own identity for this evaluation — the key a retained job is resumed by.
    /// Zero (the default) means "do not retain": every round trip starts a fresh job, which is what
    /// an unaddressed caller (a test, an export bridge) wants.
    #[value(default)]
    pub node_hash: u64,
    /// ⏱️ Units ONE step may spend before the round trip re-checks its wall deadline.
    #[value(default)]
    pub budget: u64,
    /// ⌛️ Wall-clock allowance for the WHOLE round trip, in microseconds.
    #[value(default)]
    pub wall_micros: u64,
}

/// ⏱️ Units one `evaluate` STEP spends before the round trip re-checks its wall deadline — the
/// granularity at which a cancel can land, never the round trip's own bound. Mirrors
/// `TESSELLATE_STEP_BUDGET`'s reasoning: units are not time.
pub const EVALUATE_STEP_BUDGET: usize = 8;

/// ⌛️ Wall-clock allowance for one `evaluate` round trip when the requester names none. Chosen
/// well under the host shard watchdog's 16 s silence threshold AND under the 5 s ordinary
/// heartbeat window, so a worker driving a budgeted evaluation is never mistaken for a dead one
/// (`🎭️actor/📮️shard-client/🟦️.ts`'s `SHARD_LIVENESS_POLICY`).
pub const EVALUATE_STEP_WALL_MICROS: u64 = 2_000_000;

/// 🧾️ The shape every `evaluate` answer takes. Declared here as one string so the Rust law, the
/// TypeScript twin and the fixture all name the same fields.
pub const EVALUATE_ENVELOPE_SCHEMA: &str = "{done,cancellable,phase,unitsDone,unitsTotal,outputJson}";

/// ⏱️ One budgeted `evaluate` ROUND TRIP as the extension-boundary JSON envelope.
///
/// `budget` bounds ONE step in operator-defined units; `wall_micros` bounds the whole round trip:
/// the call keeps stepping while the job is still working AND the deadline has not passed. Both
/// are needed because units are not time.
pub fn evaluate_step_envelope_json(registry: &Registry, request: &EvaluateRequest) -> String {
    use crate::os_pack::json::{object, Value};
    let budget = if request.budget == 0 { EVALUATE_STEP_BUDGET } else { request.budget as usize };
    let wall_micros = if request.wall_micros == 0 { EVALUATE_STEP_WALL_MICROS } else { request.wall_micros };
    let complete = |output_json: String, units: usize| {
        object([
            ("done".to_string(), Value::Bool(true)),
            ("cancellable".to_string(), Value::Bool(false)),
            ("phase".to_string(), Value::String("complete".to_string())),
            ("unitsDone".to_string(), Value::from(units as u64)),
            ("unitsTotal".to_string(), Value::from(units as u64)),
            ("outputJson".to_string(), Value::String(output_json)),
        ])
    };
    let key = (request.operator_id.clone(), request.node_hash);
    let retained = (request.node_hash != 0).then(|| take_evaluation_job(&key)).flatten();
    let mut job = match retained {
        Some(job) => job,
        None => {
            let input = match parsed_operator_input(registry, &request.operator_id, &request.input_json) {
                Ok(input) => input,
                Err(message) => return crate::os_pack::json::to_string(&complete(error_output_json(&message), 1)),
            };
            match registry.dispatch_job(&request.operator_id, &input) {
                Err(error) => return crate::os_pack::json::to_string(&complete(error_output_json(&error.to_string()), 1)),
                Ok(None) => {
                    let output = evaluate_json(registry, &request.operator_id, &request.input_json);
                    return crate::os_pack::json::to_string(&complete(output, 1));
                }
                Ok(Some(job)) => job,
            }
        }
    };
    let deadline = semio_framework_job::default_now_us().map(|now| now.saturating_add(wall_micros));
    let envelope = loop {
        match job.step(budget) {
            Err(error) => break complete(error_output_json(&error.to_string()), job.progress().units_done),
            Ok(neural_engine::OperatorJobStep::Cancelled(progress)) => {
                break object([
                    ("done".to_string(), Value::Bool(true)),
                    ("cancellable".to_string(), Value::Bool(false)),
                    ("phase".to_string(), Value::String("cancelled".to_string())),
                    ("unitsDone".to_string(), Value::from(progress.units_done as u64)),
                    ("unitsTotal".to_string(), Value::from(progress.units_total as u64)),
                    ("outputJson".to_string(), Value::String(String::new())),
                ])
            }
            Ok(neural_engine::OperatorJobStep::Done(output)) => {
                let units = job.progress().units_total;
                break match registry.finish_job(&request.operator_id, output) {
                    Ok(output) => complete(crate::os_pack::json::to_json_string(&*ColdOwner::new(output)), units),
                    Err(error) => complete(error_output_json(&error.to_string()), units),
                };
            }
            Ok(neural_engine::OperatorJobStep::Working(progress)) => {
                let expired = deadline.is_none_or(|deadline| semio_framework_job::default_now_us().is_none_or(|now| now >= deadline));
                if !expired {
                    continue;
                }
                if request.node_hash != 0 {
                    retain_evaluation_job(key.clone(), job);
                }
                break object([
                    ("done".to_string(), Value::Bool(false)),
                    ("cancellable".to_string(), Value::Bool(true)),
                    ("phase".to_string(), Value::String(progress.phase.to_string())),
                    ("unitsDone".to_string(), Value::from(progress.units_done as u64)),
                    ("unitsTotal".to_string(), Value::from(progress.units_total as u64)),
                    ("outputJson".to_string(), Value::String(String::new())),
                ]);
            }
        }
    };
    crate::os_pack::json::to_string(&envelope)
}

/// 🧮️ The operator input `dispatch`/`dispatch_job` both see: the request's `inputJson` parsed and
/// given this operator's declared channel defaults. Factored out so the budgeted path and the
/// one-shot path cannot disagree about what the operator was asked.
fn parsed_operator_input(registry: &Registry, operator_id: &str, input_json: &str) -> Result<ColdOwner<Dictionary>, String> {
    let input: Dictionary = crate::os_pack::json::from_json_str(input_json).map_err(|error| error.to_string())?;
    Ok(ColdOwner::new(match registry.operator_info(operator_id) {
        Some(info) => inject_channel_defaults(input, info),
        None => input,
    }))
}

/// 💥️ The `{"error": …}` output body a refused evaluation carries — the SAME shape
/// [`evaluate_json`] has always produced, so a budgeted failure and a one-shot failure are
/// indistinguishable to the requester's node cache.
fn error_output_json(message: &str) -> String {
    crate::os_pack::json::to_json_string(&DslValue::object([("error".to_string(), message.to_value())]))
}
// #endregion 🔖️Evaluate

// #region ⏱️EvaluationJobs

/// ⏱️ Retained resumable evaluations keyed by `(operatorId, nodeHash)`. Bounded: a new job past the
/// ceiling evicts the least recently stepped one rather than growing without limit — the same
/// policy `brep_geometry`'s tessellation registry states, for the same reason.
const EVALUATION_JOB_CAPACITY: usize = 16;

struct RetainedEvaluation {
    job: Box<dyn neural_engine::OperatorJob>,
    last_step: u64,
}

#[derive(Default)]
struct EvaluationJobRegistry {
    jobs: std::collections::HashMap<(String, u64), RetainedEvaluation>,
    clock: u64,
}

static EVALUATION_JOBS: std::sync::OnceLock<std::sync::Mutex<EvaluationJobRegistry>> = std::sync::OnceLock::new();

fn evaluation_jobs() -> &'static std::sync::Mutex<EvaluationJobRegistry> {
    EVALUATION_JOBS.get_or_init(|| std::sync::Mutex::new(EvaluationJobRegistry::default()))
}

/// 🔁️ Takes the retained job for `key` out of the registry, if one is parked there. Taking rather
/// than borrowing is deliberate: a step runs without the registry lock held, so a second concurrent
/// request for the same key starts its own job instead of deadlocking on ours.
fn take_evaluation_job(key: &(String, u64)) -> Option<Box<dyn neural_engine::OperatorJob>> {
    let mut registry = evaluation_jobs().lock().ok()?;
    registry.jobs.remove(key).map(|retained| retained.job)
}

/// 🅿️ Parks a still-working job for the next round trip to resume.
fn retain_evaluation_job(key: (String, u64), job: Box<dyn neural_engine::OperatorJob>) {
    let Ok(mut registry) = evaluation_jobs().lock() else { return };
    if registry.jobs.len() >= EVALUATION_JOB_CAPACITY {
        if let Some(oldest) = registry.jobs.iter().min_by_key(|(_, retained)| retained.last_step).map(|(key, _)| key.clone()) {
            registry.jobs.remove(&oldest);
        }
    }
    registry.clock += 1;
    let clock = registry.clock;
    registry.jobs.insert(key, RetainedEvaluation { job, last_step: clock });
}

/// 🛑️ Retires the parked evaluation of `operator_id` at `node_hash`. Returns true when a job was
/// actually retired — a cancel for an already-finished or never-started evaluation is a no-op, not
/// a fault, exactly as `cancel_tessellation` states.
pub fn cancel_evaluation(operator_id: &str, node_hash: u64) -> bool {
    let Ok(mut registry) = evaluation_jobs().lock() else { return false };
    match registry.jobs.remove(&(operator_id.to_string(), node_hash)) {
        Some(mut retained) => {
            retained.job.cancel();
            true
        }
        None => false,
    }
}

/// 🛑️ Retires every parked evaluation — the whole-registry half of the cancel gesture, for a
/// requester that cannot name the `(operatorId, nodeHash)` pair it wants stopped.
pub fn cancel_all_evaluations() -> usize {
    let Ok(mut registry) = evaluation_jobs().lock() else { return 0 };
    let count = registry.jobs.len();
    for (_, retained) in registry.jobs.iter_mut() {
        retained.job.cancel();
    }
    registry.jobs.clear();
    count
}

/// 📈️ Progress of the parked evaluation of `operator_id` at `node_hash`, if one is retained.
pub fn evaluation_progress(operator_id: &str, node_hash: u64) -> Option<neural_engine::OperatorProgress> {
    let registry = evaluation_jobs().lock().ok()?;
    registry.jobs.get(&(operator_id.to_string(), node_hash)).map(|retained| retained.job.progress())
}

// #endregion ⏱️EvaluationJobs

// #region 🔖️TopicContribution
/// 🗺️ Builds the `"flow.extension"` topic contribution one host app (`flow-play`,
/// `procedural3d-play`) consumes — factored out of every flow extension's own guest bundle so those
/// crates never need `serde_json` themselves. See
/// `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs::TopicContribution`.
pub fn flow_extension_topic_contribution(app_id: &str, extension_id: &str, label: &str, icon_id: &str, manifest_json: &str) -> semio_framework::TopicContribution {
    semio_framework::TopicContribution::new(
        "flow.extension",
        DslValue::object([
            ("appId".to_string(), DslValue::String(app_id.to_string())),
            ("extensionId".to_string(), DslValue::String(extension_id.to_string())),
            ("label".to_string(), DslValue::String(label.to_string())),
            ("iconId".to_string(), DslValue::String(icon_id.to_string())),
            ("manifestJson".to_string(), DslValue::String(manifest_json.to_string())),
        ]),
    )
}
// #endregion 🔖️TopicContribution

// #region 🔖️Command
/// ⌘️ Stub command handler returning acknowledgement JSON.
pub fn command_json(command_id: &str, args_json: &str) -> String {
    crate::os_pack::json::to_json_string(&DslValue::object([
        ("ok".to_string(), true.to_value()),
        ("commandId".to_string(), command_id.to_value()),
        ("args".to_string(), args_json.to_value()),
    ]))
}
// #endregion 🔖️Command

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
