//! 🏷️ 🏷️ Block 3D play app command command — `patch-object-kind`.

use crate::editor::block3d::config::{Block3dConfig, Block3dConfigMutation};
use crate::artifacts::block3d::op::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "patchObjectKind")]
pub struct PatchObjectKind {
    pub field: String,
    pub value: String,
}

pub fn handle(payload: &PatchObjectKind, _doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    use crate::artifacts::block3d::mutations as m;
    let optional = |value: &str| if value.is_empty() { None } else { Some(value.to_string()) };
    let mutation = match payload.field.as_str() {
        "name" => m::rename_object_kind(payload.value.clone()),
        "label" => m::change_object_kind_label(payload.value.clone()),
        "variant" => m::change_object_kind_variant(optional(&payload.value)),
        "description" => m::change_object_kind_description(payload.value.clone()),
        "icon" => m::change_object_kind_icon(optional(&payload.value)),
        "unit" => m::change_object_kind_unit(optional(&payload.value)),
        _ => return Ok(Emit::default()),
    };
    Ok(Emit::mutations(vec![mutation]))
}
