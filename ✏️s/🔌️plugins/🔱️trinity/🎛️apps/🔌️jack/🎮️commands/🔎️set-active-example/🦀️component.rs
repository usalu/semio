//! 🔎️ 🔎️ Trinity Jack app command — `set-active-example`.

use crate::apps::jack::config::JackConfigMutation;
use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::artifacts::jack::JackSnapshot;
use crate::core;
use semio_framework_plugin::{Emit, Fault};
use serde_json::json;
use store::ArtifactDsl;

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
        _ => crate::apps::jack::TRINITY_JACK_DEFAULT_QUERY,
    }
}

fn error_result_json(message: &str) -> String {
    json!({ "error": message }).to_string()
}
fn fixture_dsl_for_preset(preset_id: &str) -> Option<&'static str> {
    match preset_id {
        "nakagin" | "nakagin-capsule-tower" => Some(crate::apps::jack::NAKAGIN_FIXTURE_DSL),
        "branch-chain" => Some(crate::apps::jack::BRANCH_FIXTURE_DSL),
        _ => None,
    }
}

pub(crate) fn set_active_example(example_id: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    match fixture_dsl_for_preset(example_id).and_then(|dsl| JackSnapshot::parse_dsl(dsl).ok()) {
        Some(next) => {
            let query = preset_query(example_id).to_string();
            let (result_json, _) = run_jack_query(&next, &query);
            Ok(Emit {
                effects: vec![crate::apps::jack::reset_document_effect(&next)],
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
