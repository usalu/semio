//! 🔎️ 🔎️ Trinity Jack app command — `set-active-example`.

use crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
use crate::JackSnapshot;
use semio_framework_plugin::{Emit, NoConfigMutation};
use store::ArtifactDsl;

pub(crate) fn preset_query(preset_id: &str) -> &'static str {
    match preset_id {
        "branch-chain" => "MATCH (a:Piece)-[r:Connection]->(b:Piece) RETURN a, r, b",
        _ => crate::TRINITY_JACK_DEFAULT_QUERY,
    }
}

/// 🎬️ The ids this verb answers. `crate::examples::demo::ID` is the one the SHELL ever sends: the
/// navbar example picker dispatches a REGISTERED example id, and `demo` is the only example this
/// subset registers. Without it every navbar pick resolved to `None` and loaded nothing at all, with
/// no fault anywhere (`📓️b3b-trinity-wfc-puzzle.md`). The catalogue panel's own fixture rows keep
/// sending `nakagin`/`branch-chain`, which name the same asset.
fn example_dsl_for_preset(preset_id: &str) -> Option<&'static str> {
    match preset_id {
        "nakagin" | "nakagin-capsule-tower" => Some(crate::editor::jack::NAKAGIN_EXAMPLE_DSL),
        "branch-chain" => Some(crate::editor::jack::BRANCH_EXAMPLE_DSL),
        id if id == crate::examples::demo::ID => Some(crate::editor::jack::NAKAGIN_EXAMPLE_DSL),
        _ => None,
    }
}

pub(crate) fn set_active_example(example_id: &str) -> Emit<TrinityGraphMutation, NoConfigMutation> {
    match example_dsl_for_preset(example_id).and_then(|dsl| JackSnapshot::parse_dsl(dsl).ok()) {
        Some(parsed) => {
            let next = JackSnapshot { query: preset_query(example_id).into(), ..parsed };
            Emit { effects: vec![crate::editor::jack::reset_document_effect(&next)], ..Default::default() }
        }
        None => Emit::default(),
    }
}
