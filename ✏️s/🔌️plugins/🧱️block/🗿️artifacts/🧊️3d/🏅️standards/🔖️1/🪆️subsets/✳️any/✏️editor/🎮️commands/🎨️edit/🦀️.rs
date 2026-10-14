//! 🎨️ 🎨️ Block 3D play app commands command — `edit`.

use semio_s_artifact_block_3d_mutations::Block3dMutation;
use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_editor_config::{Block3dConfig, Block3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "edit")]
pub struct Edit {
    pub text: String,
}

pub fn handle(payload: &Edit, doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    match semio_framework_pack_json::from_json_str::<Block3dSnapshot>(&payload.text, semio_framework_pack_json::JsonMemberPolicy::Reject) {
        Ok(document) if &document != doc.snapshot => Ok(Emit::effect(crate::set_active_example::load_document_effect(&document))),
        _ => Ok(Emit::default()),
    }
}
