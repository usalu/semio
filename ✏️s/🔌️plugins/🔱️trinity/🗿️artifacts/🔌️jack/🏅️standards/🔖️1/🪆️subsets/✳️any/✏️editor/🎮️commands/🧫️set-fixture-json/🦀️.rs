//! 🗺️ 🗺️ Trinity Jack app command — `set-fixture-json`.

use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use crate::JackSnapshot;
use semio_framework_plugin::{Emit, NoConfigMutation};

pub(crate) fn set_fixture_json(json: &str) -> Emit<TrinityGraphMutation, NoConfigMutation> {
    match JackSnapshot::from_json(json) {
        Ok(next) => Emit { effects: vec![crate::editor::jack::reset_document_effect(&next)], ..Default::default() },
        Err(_) => Emit::default(),
    }
}
