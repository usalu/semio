//! 🗺️ 🗺️ Trinity Jack app command — `set-fixture-json`.

use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::artifacts::jack::JackSnapshot;
use crate::editor::jack::config::JackConfigMutation;
use semio_framework_plugin::Emit;

pub(crate) fn set_fixture_json(json: &str) -> Emit<TrinityGraphMutation, JackConfigMutation> {
    match JackSnapshot::from_json(json) {
        Ok(next) => Emit { effects: vec![crate::editor::jack::reset_document_effect(&next)], ..Default::default() },
        Err(_) => Emit::default(),
    }
}
