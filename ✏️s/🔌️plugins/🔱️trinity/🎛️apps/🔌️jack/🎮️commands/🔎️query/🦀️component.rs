//! 🔎️ Trinity Jack app — jack-query-driven commands (`runQuery`, `loadExampleQuery`,
//! `setActiveExample`, `requestCompletions`, `formatDocument`).

use crate::apps::jack::config::JackConfigMutation;
use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::artifacts::jack::GraphFixture;
use crate::core;
use semio_framework_plugin::{Emit, Fault};
use serde_json::json;
use store::DocumentDsl;

/// 🔎️ Runs a jack query against the fixture, returning `(result_json, forward operations)`; a parse/execute
/// failure yields an error result and no operations (no document mutation).
pub(crate) fn run_jack_query(fixture: &GraphFixture, query: &str) -> (String, Vec<TrinityGraphMutation>) {
    let graph = match crate::artifacts::jack::Graph::from_fixture(fixture.clone()) {
        Ok(graph) => graph,
        Err(error) => return (error_result_json(&error.to_string()), Vec::new()),
    };
    let parsed = match core::parse(query) {
        Ok(parsed) => parsed,
        Err(error) => return (error_result_json(&error), Vec::new()),
    };
    match core::execute(&graph, &parsed) {
        Ok((result, operations)) => (serde_json::to_string(&result).unwrap_or_default(), operations),
        Err(error) => (error_result_json(&error), Vec::new()),
    }
}

fn error_result_json(message: &str) -> String {
    json!({ "error": message }).to_string()
}

pub(crate) fn run_query(fixture: &GraphFixture, query: &Option<String>, current_query: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    let resolved = query.as_deref().filter(|value| !value.trim().is_empty()).map_or_else(|| current_query.to_string(), str::to_string);
    let (result_json, operations) = run_jack_query(fixture, &resolved);
    Ok(Emit {
        document_mutations: operations,
        config_mutations: vec![JackConfigMutation::SetQuery { value: resolved }, JackConfigMutation::SetResult { value: result_json }, JackConfigMutation::SetResultsEngagementInput { value: String::new() }],
        ..Default::default()
    })
}

pub(crate) fn load_example_query(fixture: &GraphFixture, query: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    let (result_json, operations) = run_jack_query(fixture, query);
    Ok(Emit { document_mutations: operations, config_mutations: vec![JackConfigMutation::SetQuery { value: query.to_string() }, JackConfigMutation::SetResult { value: result_json }], ..Default::default() })
}

fn fixture_dsl_for_preset(preset_id: &str) -> Option<&'static str> {
    match preset_id {
        "nakagin" | "nakagin-capsule-tower" => Some(crate::apps::jack::NAKAGIN_FIXTURE_DSL),
        "branch-chain" => Some(crate::apps::jack::BRANCH_FIXTURE_DSL),
        _ => None,
    }
}

pub(crate) fn preset_query(preset_id: &str) -> &'static str {
    match preset_id {
        "branch-chain" => "MATCH (a:Piece)-[r:Connection]->(b:Piece) RETURN a, r, b",
        _ => crate::apps::jack::TRINITY_JACK_DEFAULT_QUERY,
    }
}

pub(crate) fn set_active_example(example_id: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    match fixture_dsl_for_preset(example_id).and_then(|dsl| GraphFixture::parse_dsl(dsl).ok()) {
        Some(next) => {
            let query = preset_query(example_id).to_string();
            let (result_json, _) = run_jack_query(&next, &query);
            Ok(Emit {
                document_mutations: vec![TrinityGraphMutation::SetFixture { fixture: next.clone() }],
                config_mutations: vec![
                    JackConfigMutation::SetActiveFixture { value: example_id.to_string() },
                    JackConfigMutation::SetCamera { camera: next.camera },
                    JackConfigMutation::SetQuery { value: query },
                    JackConfigMutation::SetResult { value: result_json },
                ],
                ..Default::default()
            })
        }
        None => Ok(Emit::default()),
    }
}

pub(crate) fn request_completions(revision: u64) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    Ok(Emit::config(vec![JackConfigMutation::SetRevision { value: revision + 1 }]))
}

pub(crate) fn format_document(jack_query: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    match core::format(jack_query) {
        Ok(formatted) => Ok(Emit::config(vec![JackConfigMutation::SetQuery { value: formatted }])),
        Err(_) => Ok(Emit::default()),
    }
}
