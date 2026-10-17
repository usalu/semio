//! 🔎️ 🔎️ Trinity Jack app command — `set-active-example`.

use crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
use semio_framework_plugin::{Emit, NoConfigMutation};
use pack::snapshot::ArtifactDsl;

pub fn preset_query(preset_id: &str) -> &'static str {
    match preset_id {
        "branch-chain" => "MATCH (a:Piece)-[r:Connection]->(b:Piece) RETURN a, r, b",
        _ => crate::standards::v1::subsets::any::schema::graph::TRINITY_JACK_DEFAULT_QUERY,
    }
}

/// 🎬️ The authored composition supplies each explicit example with its original selected child.
pub fn set_active_example<C:crate::standards::v1::subsets::any::schema::composition::JackComposition>(example_id:&str)->Emit<TrinityGraphMutation,NoConfigMutation>{
 match C::example_snapshot(example_id){Some(mut next)=>{next.query=preset_query(example_id).into();Emit{effects:vec![crate::editor::jack::reset_document_effect(&next)],..Default::default()}},None=>Emit::default()}
}
