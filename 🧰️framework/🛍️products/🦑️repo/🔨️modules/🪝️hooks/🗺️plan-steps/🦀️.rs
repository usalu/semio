//! 🪜️ Plan steps: extraction and merging of agent plan steps.
//!
//! Holds the `🗺️PlanSteps` region of the crate root; the root re-exports its items.

use super::*;

//#region 🗺️PlanSteps
/// 🔁️ One step of a plan / task list update event.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HookPlanStep {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "status", default)]
    pub status: String,
}

pub(super) fn plan_source(input: Option<&Value>, tool_args: &str) -> Option<Map<String, Value>> {
    let data = match input {
        Some(value) => value.as_object().cloned().or_else(|| parse_tool_args(tool_args)),
        None => parse_tool_args(tool_args),
    }?;
    match data.get("tool_input") {
        Some(Value::Object(tool_input)) => Some(tool_input.clone()),
        Some(_) => None,
        None => Some(data),
    }
}

/// 🗺️ Reads the plan steps out of a payload: VS Code's `todoList` entries (`title` + `status`) and
/// the neutral `steps` entries (`name` + `status`), in that order, from `tool_input` when present.
pub fn extract_plan_steps(input: Option<&Value>, tool_args: &str) -> Vec<HookPlanStep> {
    let Some(source) = plan_source(input, tool_args) else { return Vec::new() };
    let mut steps = Vec::new();
    if let Some(Value::Array(entries)) = source.get("todoList") {
        for entry in entries {
            let Some(entry) = entry.as_object() else { continue };
            let name = entry.get("title").and_then(Value::as_str).unwrap_or_default();
            let status = entry.get("status").and_then(Value::as_str).unwrap_or_default();
            if !name.is_empty() {
                steps.push(HookPlanStep { name: name.to_string(), status: status.to_string() });
            }
        }
    }
    if let Some(Value::Array(entries)) = source.get("steps") {
        for entry in entries {
            let Some(entry) = entry.as_object() else { continue };
            let name = entry.get("name").and_then(Value::as_str).unwrap_or_default();
            let status = entry.get("status").and_then(Value::as_str).unwrap_or_default();
            if !name.is_empty() {
                steps.push(HookPlanStep { name: name.to_string(), status: status.to_string() });
            }
        }
    }
    steps
}

/// 🔀️ Folds an incoming plan into the recorded one: an incoming step keeps whatever timestamps it
/// already had, gains `ideated` the first time it is seen, `started` when it goes in progress and
/// `completed` when it finishes after having started; a recorded step the incoming plan dropped is
/// abandoned unless it already carries a lifecycle timestamp.
pub fn merge_plan_steps(existing: &[TicketAgentPlanStep], incoming: &[HookPlanStep], second: &str) -> Vec<TicketAgentPlanStep> {
    let mut by_name: BTreeMap<&str, &TicketAgentPlanStep> = BTreeMap::new();
    for step in existing {
        by_name.insert(step.name.as_str(), step);
    }
    let mut active: BTreeSet<String> = BTreeSet::new();
    let mut merged: Vec<TicketAgentPlanStep> = Vec::with_capacity(existing.len() + incoming.len());
    for step in incoming {
        let name = step.name.trim();
        if name.is_empty() {
            continue;
        }
        let mut resolved = by_name.get(name).map_or_else(|| TicketAgentPlanStep {
            id: String::new(),
            name: String::new(),
            description: String::new(),
            status: String::new(),
            ideated: String::new(),
            started: String::new(),
            completed: String::new(),
            abandoned: String::new(),
        }, |found| (*found).clone());
        resolved.name = name.to_string();
        resolved.status = step.status.clone();
        if resolved.ideated.is_empty() {
            resolved.ideated = second.to_string();
        }
        match step.status.trim().to_lowercase().as_str() {
            "in-progress" | "in_progress" | "started" => {
                if resolved.started.is_empty() {
                    resolved.started = second.to_string();
                }
            }
            "completed" | "done" if !resolved.started.is_empty() && resolved.completed.is_empty() => {
                resolved.completed = second.to_string();
            }
            _ => {}
        }
        active.insert(name.to_string());
        merged.push(resolved);
    }
    for step in existing {
        if active.contains(&step.name) || step.name.is_empty() {
            continue;
        }
        let mut resolved = step.clone();
        if !resolved.abandoned.is_empty() || !resolved.completed.is_empty() || !resolved.started.is_empty() {
            merged.push(resolved);
            continue;
        }
        if resolved.ideated.is_empty() {
            resolved.ideated = second.to_string();
        }
        resolved.abandoned = second.to_string();
        merged.push(resolved);
    }
    merged
}
//#endregion 🗺️PlanSteps
