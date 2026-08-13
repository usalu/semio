//! 🔘️ 🔘️ Block 2D play app commands command — `add-handle-kind`.

use crate::apps::block2d::config::{Block2dConfig, Block2dConfigMutation};
use crate::artifacts::block2d::op::Block2dMutation;
use crate::artifacts::block2d::{Block2dHandleKind, Block2dSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "addHandleKind")]
pub struct AddHandleKind {}

pub fn handle(_payload: &AddHandleKind, doc: &ArtifactView<'_, Block2dSnapshot>, _cfg: &ConfigView<'_, Block2dConfig>) -> Result<Emit<Block2dMutation, Block2dConfigMutation>, Fault> {
    let id = crate::artifacts::block2d::schema::next_id(doc.snapshot.handle_kinds.iter().map(|kind| kind.id.as_str()), "handle-kind-");
    let handle_kind = Block2dHandleKind { id: id.clone(), name: id.clone(), label: id, color: "#888888".into(), default_wire_kind: "cable.link".into() };
    Ok(Emit::mutations(vec![crate::artifacts::block2d::mutations::create_handle_kind(handle_kind)]))
}
