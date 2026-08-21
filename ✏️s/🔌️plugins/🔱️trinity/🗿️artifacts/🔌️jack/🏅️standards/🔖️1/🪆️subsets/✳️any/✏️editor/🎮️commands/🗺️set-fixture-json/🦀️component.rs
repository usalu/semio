//! 🗺️ 🗺️ Trinity Jack app command — `set-fixture-json`.

use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::artifacts::jack::JackSnapshot;
use crate::editor::jack::config::JackConfigMutation;
use semio_framework_plugin::{Emit, Fault};

pub(crate) async fn set_fixture_json(json: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    match JackSnapshot::from_json(json) {
        Ok(next) => Ok(Emit { effects: vec![crate::editor::jack::reset_document_effect(&next)], ..Default::default() }),
        Err(_) => Ok(Emit::default()),
    }
}
