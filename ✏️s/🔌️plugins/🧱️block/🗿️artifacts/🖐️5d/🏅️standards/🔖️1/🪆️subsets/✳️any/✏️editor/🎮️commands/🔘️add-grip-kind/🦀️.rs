//! 🔘️ 🔘️ Block 5D play app commands command — `add-grip-kind`.

use crate::standards::v1::subsets::any::schema::mutations::text::Block5dMutation;
use crate::{Block5dGripKind, Block5dSnapshot};
use crate::editor::block5d::config::{Block5dConfig, Block5dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "addGripKind")]
pub struct AddGripKind {}

pub fn handle(_payload: &AddGripKind, doc: &ArtifactView<'_, Block5dSnapshot>, _cfg: &ConfigView<'_, Block5dConfig>) -> Result<Emit<Block5dMutation, Block5dConfigMutation>, Fault> {
    let id = crate::standards::v1::subsets::any::schema::next_id(doc.snapshot.grip_kinds.iter().map(|kind| kind.id.as_str()), "grip-kind-");
    let grip_kind = Block5dGripKind { id: id.clone(), name: id.clone(), label: id, color: "#888888".into(), default_rope_kind: "rope.link".into() };
    Ok(Emit::mutations(vec![crate::standards::v1::subsets::any::schema::mutations::create_grip_kind(grip_kind)]))
}
