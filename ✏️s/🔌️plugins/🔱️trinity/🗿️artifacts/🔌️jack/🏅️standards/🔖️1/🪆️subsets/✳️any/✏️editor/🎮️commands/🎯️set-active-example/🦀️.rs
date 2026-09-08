//! 🔎️ 🔎️ Trinity Jack app command — `set-active-example`.

use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use crate::JackSnapshot;
use crate::editor::jack::config::JackConfigMutation;
use semio_framework_plugin::Emit;
use store::ArtifactDsl;

pub(crate) fn preset_query(preset_id: &str) -> &'static str {
    match preset_id {
        "branch-chain" => "MATCH (a:Piece)-[r:Connection]->(b:Piece) RETURN a, r, b",
        _ => crate::editor::jack::TRINITY_JACK_DEFAULT_QUERY,
    }
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
            Emit {
                effects: vec![crate::editor::jack::reset_document_effect(&next)],
                config_mutations: vec![
                    JackConfigMutation::SetCamera(crate::editor::jack::config::SetCamera { camera: next.camera }),
                    JackConfigMutation::SetQuery(crate::editor::jack::config::SetQuery { value: query }),
                ],
                ..Default::default()
            }
        }
        None => Emit::default(),
    }
}
