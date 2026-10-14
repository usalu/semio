//! 🏷️ 🏷️ Block 2D play app command command — `patch-node-kind`.

use semio_s_artifact_block_2d_mutations::Block2dMutation;
use semio_s_artifact_block_2d_model::Block2dSnapshot;
use crate::config::{Block2dConfig, Block2dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "patchNodeKind")]
pub struct PatchNodeKind {
    pub field: String,
    pub value: String,
}

pub fn handle(payload: &PatchNodeKind, _doc: &ArtifactView<'_, Block2dSnapshot>, _cfg: &ConfigView<'_, Block2dConfig>) -> Result<Emit<Block2dMutation, Block2dConfigMutation>, Fault> {
    use semio_s_artifact_block_2d_mutations as m;
    let optional = |value: &str| if value.is_empty() { None } else { Some(value.to_string()) };
    let mutation = match payload.field.as_str() {
        "name" => m::rename_node_kind(payload.value.clone()),
        "label" => m::change_node_kind_label(payload.value.clone()),
        "variant" => m::change_node_kind_variant(optional(&payload.value)),
        "description" => m::change_node_kind_description(payload.value.clone()),
        "icon" => m::change_node_kind_icon(optional(&payload.value)),
        "unit" => m::change_node_kind_unit(optional(&payload.value)),
        _ => return Ok(Emit::default()),
    };
    Ok(Emit::mutations(vec![mutation]))
}
