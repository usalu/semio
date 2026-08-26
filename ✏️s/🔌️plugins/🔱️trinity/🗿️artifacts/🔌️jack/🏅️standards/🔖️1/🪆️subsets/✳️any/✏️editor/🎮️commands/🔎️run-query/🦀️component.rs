//! 🔎️ 🔎️ Trinity Jack app command — `run-query`.

use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::artifacts::jack::JackSnapshot;
use crate::core;
use crate::editor::jack::config::JackConfigMutation;
use semio_framework_plugin::{Emit, Fault};
use serde_json::json;

/// 🔎️ Runs a jack query against the fixture, returning `(result_json, forward operations)`; a parse/execute
/// failure yields an error result and no operations (no document mutation).
pub(crate) fn run_jack_query(fixture: &JackSnapshot, query: &str) -> (String, Vec<TrinityGraphMutation>) {
    let graph = match crate::artifacts::jack::Graph::from_fixture(fixture.clone()) {
        Ok(graph) => graph,
        Err(error) => return (error_result_json(&error.to_string()), Vec::new()),
    };
    let parsed = match core::parse(query) {
        Ok(parsed) => parsed,
        Err(error) => return (error_result_json(&error), Vec::new()),
    };
    match crate::executor::execute(&graph, &parsed) {
        Ok((result, operations)) => (serde_json::to_string(&result).unwrap_or_default(), operations),
        Err(error) => (error_result_json(&error), Vec::new()),
    }
}

pub(crate) fn preset_query(preset_id: &str) -> &'static str {
    match preset_id {
        "branch-chain" => "MATCH (a:Piece)-[r:Connection]->(b:Piece) RETURN a, r, b",
        _ => crate::editor::jack::TRINITY_JACK_DEFAULT_QUERY,
    }
}

fn error_result_json(message: &str) -> String {
    json!({ "error": message }).to_string()
}

pub(crate) fn run_query(fixture: &JackSnapshot, query: &Option<String>, current_query: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    let resolved = query.as_deref().filter(|value| !value.trim().is_empty()).map_or_else(|| current_query.to_string(), str::to_string);
    let (result_json, operations) = run_jack_query(fixture, &resolved);
    Ok(Emit {
        artifact_mutations: operations,
        config_mutations: vec![JackConfigMutation::SetQuery { value: resolved }, JackConfigMutation::SetResult { value: result_json }, JackConfigMutation::SetResultsEngagementInput { value: String::new() }],
        ..Default::default()
    })
}
