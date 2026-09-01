//! 🌱️ 🌱️ Block 2D play app commands command — `add-handle`.

use crate::artifacts::block2d::op::Block2dMutation;
use crate::artifacts::block2d::{Block2dHandleTemplate, Block2dSnapshot};
use crate::editor::block2d::config::{Block2dConfig, Block2dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "addHandle")]
pub struct AddHandle {}

pub async fn handle(_payload: &AddHandle, doc: &ArtifactView<'_, Block2dSnapshot>, _cfg: &ConfigView<'_, Block2dConfig>) -> Result<Emit<Block2dMutation, Block2dConfigMutation>, Fault> {
    let Some(handle_kind_id) = doc.snapshot.handle_kinds.first().map(|kind| kind.id.clone()) else {
        return Ok(Emit::default());
    };
    let id = crate::artifacts::block2d::schema::next_id(doc.snapshot.handles.iter().map(|handle| handle.id.as_str()), "handle-");
    let handle = Block2dHandleTemplate { id, handle_kind: handle_kind_id, angle: 0.0, radius: 0.36 };
    Ok(Emit::mutations(vec![crate::artifacts::block2d::mutations::create_handle(handle)]))
}
