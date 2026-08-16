//! 🔘️ 🔘️ Block 5D play app commands command — `add-grip-kind`.

use crate::editor::block5d::config::{Block5dConfig, Block5dConfigMutation};
use crate::artifacts::block5d::op::Block5dMutation;
use crate::artifacts::block5d::{Block5dGripKind, Block5dSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "addGripKind")]
pub struct AddGripKind {}

pub fn handle(_payload: &AddGripKind, doc: &ArtifactView<'_, Block5dSnapshot>, _cfg: &ConfigView<'_, Block5dConfig>) -> Result<Emit<Block5dMutation, Block5dConfigMutation>, Fault> {
    let id = crate::artifacts::block5d::schema::next_id(doc.snapshot.grip_kinds.iter().map(|kind| kind.id.as_str()), "grip-kind-");
    let grip_kind = Block5dGripKind { id: id.clone(), name: id.clone(), label: id, color: "#888888".into(), default_rope_kind: "rope.link".into() };
    Ok(Emit::mutations(vec![crate::artifacts::block5d::mutations::create_grip_kind(grip_kind)]))
}
