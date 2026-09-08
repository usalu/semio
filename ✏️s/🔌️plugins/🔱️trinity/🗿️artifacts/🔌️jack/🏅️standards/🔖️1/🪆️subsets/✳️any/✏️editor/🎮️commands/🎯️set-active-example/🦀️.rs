//! 🔎️ 🔎️ Trinity Jack app command — `set-active-example`.

use crate::op::TrinityGraphMutation;
use crate::JackSnapshot;
use crate::core;
use crate::editor::jack::config::JackConfigMutation;
use semio_framework_plugin::Emit;
use store::ArtifactDsl;

/// 🔎️ Runs a jack query against the fixture, returning `(result_json, forward operations)`; a parse/execute
/// failure yields an error result and no operations (no document mutation).
pub(crate) fn run_jack_query(fixture: &JackSnapshot, query: &str) -> (String, Vec<TrinityGraphMutation>) {
    let graph = match crate::Graph::from_fixture(fixture.clone()) {
        Ok(graph) => graph,
        Err(error) => return (error_result_json(&error.to_string()), Vec::new()),
    };
    let parsed = match core::parse(query) {
        Ok(parsed) => parsed,
        Err(error) => return (error_result_json(&error), Vec::new()),
    };
    match crate::executor::execute(&graph, &parsed) {
        Ok((result, operations)) => (pack::to_json_string(&result), operations),
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
    pack::json!({ "error": message }).to_string()
}
fn fixture_dsl_for_preset(preset_id: &str) -> Option<&'static str> {
    match preset_id {
        "nakagin" | "nakagin-capsule-tower" => Some(crate::editor::jack::NAKAGIN_FIXTURE_DSL),
        "branch-chain" => Some(crate::editor::jack::BRANCH_FIXTURE_DSL),
        _ => None,
    }
}

pub(crate) fn set_active_example(example_id: &str) -> Emit<TrinityGraphMutation, JackConfigMutation> {
    match fixture_dsl_for_preset(example_id).and_then(|dsl| JackSnapshot::parse_dsl(dsl).ok()) {
        Some(next) => {
            let query = preset_query(example_id).to_string();
            let (result_json, _) = run_jack_query(&next, &query);
            Emit {
                effects: vec![crate::editor::jack::reset_document_effect(&next)],
                config_mutations: vec![
                    JackConfigMutation::SetActiveFixture(crate::editor::jack::config::SetActiveFixture { value: example_id.to_string() }),
                    JackConfigMutation::SetCamera(crate::editor::jack::config::SetCamera { camera: next.camera }),
                    JackConfigMutation::SetQuery(crate::editor::jack::config::SetQuery { value: query }),
                    JackConfigMutation::SetResult(crate::editor::jack::config::SetResult { value: result_json }),
                ],
                ..Default::default()
            }
        }
        None => Emit::default(),
    }
}
