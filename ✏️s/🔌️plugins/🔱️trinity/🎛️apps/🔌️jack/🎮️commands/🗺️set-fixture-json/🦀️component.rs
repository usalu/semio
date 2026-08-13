//! 🗺️ 🗺️ Trinity Jack app command — `set-fixture-json`.

use crate::apps::jack::config::JackConfigMutation;
use crate::artifacts::jack::mutations::{delete_node, move_node, rename_node};
use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::artifacts::jack::{JackSnapshot, Node};
use semio_framework_plugin::{Emit, Fault};

pub(crate) fn set_fixture_json(json: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    match JackSnapshot::from_json(json) {
        Ok(next) => Ok(Emit { effects: vec![crate::apps::jack::reset_document_effect(&next)], ..Default::default() }),
        Err(_) => Ok(Emit::default()),
    }
}
