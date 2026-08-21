//! 🏷️ 🏷️ Block 2D play app command command — `patch-node-kind`.

use crate::artifacts::block2d::op::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;
use crate::editor::block2d::config::{Block2dConfig, Block2dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "patchNodeKind")]
pub struct PatchNodeKind {
    pub field: String,
    pub value: String,
}

pub async fn handle(payload: &PatchNodeKind, _doc: &ArtifactView<'_, Block2dSnapshot>, _cfg: &ConfigView<'_, Block2dConfig>) -> Result<Emit<Block2dMutation, Block2dConfigMutation>, Fault> {
    use crate::artifacts::block2d::mutations as m;
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
