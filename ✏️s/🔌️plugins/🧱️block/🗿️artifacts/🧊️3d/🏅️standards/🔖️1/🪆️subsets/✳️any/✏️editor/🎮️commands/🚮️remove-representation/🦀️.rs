//! 🚮️ Block 3D play app command — `remove-representation`.

use crate::standards::v1::subsets::any::schema::mutations::text::Block3dMutation;
use crate::Block3dSnapshot;
use crate::editor::block3d::config::{Block3dConfig, Block3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "removeRepresentation")]
pub struct RemoveRepresentation {
    pub id: String,
}

pub fn handle(payload: &RemoveRepresentation, _doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![crate::standards::v1::subsets::any::schema::mutations::delete_representation(payload.id.clone())]))
}
