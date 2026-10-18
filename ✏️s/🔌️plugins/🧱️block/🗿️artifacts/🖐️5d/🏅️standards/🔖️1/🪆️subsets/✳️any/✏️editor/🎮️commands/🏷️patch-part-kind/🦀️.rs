//! 🏷️ 🏷️ Block 5D play app command command — `patch-part-kind`.

use semio_s_artifact_block_5d_mutations::Block5dMutation;
use semio_s_artifact_block_5d_model::Block5dSnapshot;
use crate::config::{Block5dConfig, Block5dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_block_5d_mut_part_kind::rename_part_kind::rename_part_kind;
use semio_s_artifact_block_5d_mut_part_kind::change_part_kind_label::change_part_kind_label;
use semio_s_artifact_block_5d_mut_part_kind::change_part_kind_variant::change_part_kind_variant;
use semio_s_artifact_block_5d_mut_part_kind::change_part_kind_description::change_part_kind_description;
use semio_s_artifact_block_5d_mut_part_kind::change_part_kind_icon::change_part_kind_icon;
use semio_s_artifact_block_5d_mut_part_kind::change_part_kind_unit::change_part_kind_unit;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "patchPartKind")]
pub struct PatchPartKind {
    pub field: String,
    pub value: String,
}

pub fn handle(payload: &PatchPartKind, _doc: &ArtifactView<'_, Block5dSnapshot>, _cfg: &ConfigView<'_, Block5dConfig>) -> Result<Emit<Block5dMutation, Block5dConfigMutation>, Fault> {
    use semio_s_artifact_block_5d_mutations as m;
    let optional = |value: &str| if value.is_empty() { None } else { Some(value.to_string()) };
    let mutation = match payload.field.as_str() {
        "name" => m::rename_part_kind(payload.value.clone()),
        "label" => m::change_part_kind_label(payload.value.clone()),
        "variant" => m::change_part_kind_variant(optional(&payload.value)),
        "description" => m::change_part_kind_description(payload.value.clone()),
        "icon" => m::change_part_kind_icon(optional(&payload.value)),
        "unit" => m::change_part_kind_unit(optional(&payload.value)),
        _ => return Ok(Emit::default()),
    };
    Ok(Emit::mutations(vec![mutation]))
}
