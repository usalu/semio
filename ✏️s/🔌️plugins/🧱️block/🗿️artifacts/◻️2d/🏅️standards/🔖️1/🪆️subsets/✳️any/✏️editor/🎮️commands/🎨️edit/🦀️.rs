//! 🎨️ 🎨️ Block 2D play app commands command — `edit`.

use semio_s_artifact_block_2d_mutations::Block2dMutation;
use semio_s_artifact_block_2d_model::Block2dSnapshot;
use crate::config::{Block2dConfig, Block2dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "edit")]
pub struct Edit {
    pub text: String,
}

pub fn handle(payload: &Edit, doc: &ArtifactView<'_, Block2dSnapshot>, _cfg: &ConfigView<'_, Block2dConfig>) -> Result<Emit<Block2dMutation, Block2dConfigMutation>, Fault> {
    match semio_framework_pack_json::from_json_str::<Block2dSnapshot>(&payload.text, semio_framework_pack_json::JsonMemberPolicy::Reject) {
        Ok(document) if &document != doc.snapshot => Ok(Emit::effect(crate::commands::set_active_example::load_document_effect(&document))),
        _ => Ok(Emit::default()),
    }
}
