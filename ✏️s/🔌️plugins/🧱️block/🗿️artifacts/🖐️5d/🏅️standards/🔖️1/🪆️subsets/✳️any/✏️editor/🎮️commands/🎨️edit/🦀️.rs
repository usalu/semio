//! 🎨️ 🎨️ Block 5D play app commands command — `edit`.

use semio_s_artifact_block_5d_mutations::Block5dMutation;
use semio_s_artifact_block_5d_model::Block5dSnapshot;
use crate::config::{Block5dConfig, Block5dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "edit")]
pub struct Edit {
    pub text: String,
}

pub fn handle(payload: &Edit, doc: &ArtifactView<'_, Block5dSnapshot>, _cfg: &ConfigView<'_, Block5dConfig>) -> Result<Emit<Block5dMutation, Block5dConfigMutation>, Fault> {
    match semio_framework_pack_json::from_json_str::<Block5dSnapshot>(&payload.text, semio_framework_pack_json::JsonMemberPolicy::Reject) {
        Ok(document) if &document != doc.snapshot => Ok(Emit::effect(crate::commands::set_active_example::load_document_effect(&document))),
        _ => Ok(Emit::default()),
    }
}
