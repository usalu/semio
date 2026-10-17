//! 🗺️ 🗺️ Trinity Jack app command — `load-document-json`.

use crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
use semio_framework_plugin::{Emit, NoConfigMutation};

pub fn load_document_json(json: &str) -> Emit<TrinityGraphMutation, NoConfigMutation> {
    match JackSnapshot::from_json(json) {
        Ok(next) => Emit { effects: vec![crate::editor::jack::reset_document_effect(&next)], ..Default::default() },
        Err(_) => Emit::default(),
    }
}
