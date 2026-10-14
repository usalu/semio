//! 🔎️ 🔎️ Trinity Jack app command — `set-active-example`.

use crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
use crate::JackSnapshot;
use semio_framework_plugin::{Emit, NoConfigMutation};
use pack::snapshot::ArtifactDsl;

pub(crate) fn preset_query(preset_id: &str) -> &'static str {
    match preset_id {
        "branch-chain" => "MATCH (a:Piece)-[r:Connection]->(b:Piece) RETURN a, r, b",
        _ => crate::TRINITY_JACK_DEFAULT_QUERY,
    }
}

/// 🎬️ The authored composition supplies each explicit example with its original selected child.
pub(crate) fn set_active_example<C:crate::JackComposition>(example_id:&str)->Emit<TrinityGraphMutation,NoConfigMutation>{
 match C::example_snapshot(example_id){Some(mut next)=>{next.query=preset_query(example_id).into();Emit{effects:vec![crate::editor::jack::reset_document_effect(&next)],..Default::default()}},None=>Emit::default()}
}
