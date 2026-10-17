//! ▶️ Jack query runner: parses a text query, executes it against an in-memory graph, applies its graph effects and encodes the typed result under physical byte admission.

use crate::standards::v1::subsets::any::schema::ast::QueryResult;
use crate::standards::v1::subsets::any::schema::executor::execute;
use crate::standards::v1::subsets::any::schema::language_service::{example_graph_snapshot, parse};
use crate::standards::v1::subsets::any::schema::memory::Graph;
use crate::standards::v1::subsets::any::schema::operations::apply_graph_effects;

/// ▶️ Parses and executes a text query, publishing its semantic graph effects.
pub fn run(graph: &mut Graph, source: &str) -> Result<QueryResult, String> {
    let query = parse(source)?;
    let (result, effects) = execute(graph, &query)?;
    apply_graph_effects(graph, &effects).map_err(|error| error.to_string())?;
    Ok(result)
}

/// 📤️ Encodes a typed query result under caller-owned physical progress and byte admission.
pub fn encode_query_result_json(result: &QueryResult, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<String, semio_framework_value::ValueError> {
    semio_framework_pack_json::to_json_string_controlled(result, control)
}

/// ▶️ Runs a text query and emits its response under physical JSON byte admission.
pub fn run_json_controlled(graph: &mut Graph, source: &str, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<String, String> {
    let result = run(graph, source)?;
    encode_query_result_json(&result, control).map_err(|error| error.to_string())
}

/// ▶️ Executes a text query with the default response byte grant.
pub fn run_json(graph: &mut Graph, source: &str) -> Result<String, String> {
    let mut progress = |_| true;
    let mut control = semio_framework_value::NativeEncodeControl::new(1_048_576, &mut progress);
    run_json_controlled(graph, source, &mut control)
}


/// 📤️ Encodes the typed language-service example at its physical text boundary.
pub fn example_graph_snapshot_json() -> String {
    semio_framework_pack_json::to_json_string(&example_graph_snapshot())
}
