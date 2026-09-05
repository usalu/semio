//! 🌱️ 🌱️ Block 5D play app commands command — `add-grip`.

use crate::artifacts::block5d::op::Block5dMutation;
use crate::artifacts::block5d::{Block5dGripTemplate, Block5dSnapshot};
use crate::editor::block5d::config::{Block5dConfig, Block5dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "addGrip")]
pub struct AddGrip {}

pub fn handle(_payload: &AddGrip, doc: &ArtifactView<'_, Block5dSnapshot>, _cfg: &ConfigView<'_, Block5dConfig>) -> Result<Emit<Block5dMutation, Block5dConfigMutation>, Fault> {
    let Some(grip_kind_id) = doc.snapshot.grip_kinds.first().map(|kind| kind.id.clone()) else {
        return Ok(Emit::default());
    };
    let id = crate::artifacts::block5d::schema::next_id(doc.snapshot.grips.iter().map(|grip| grip.id.as_str()), "grip-");
    let grip = Block5dGripTemplate { id, grip_kind: grip_kind_id, angle: 0.0, radius_2d: 0.36, position: [0.0, 0.0, 0.0], direction: [0.0, 1.0, 0.0], radius_3d: 0.36 };
    Ok(Emit::mutations(vec![crate::artifacts::block5d::mutations::create_grip(grip)]))
}
