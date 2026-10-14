//! 🔘️ 🔘️ Block 2D play app commands command — `add-handle-kind`.

use semio_s_artifact_block_2d_mutations::Block2dMutation;
use semio_s_artifact_block_2d_model::{Block2dHandleKind, Block2dSnapshot};
use crate::config::{Block2dConfig, Block2dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "addHandleKind")]
pub struct AddHandleKind {}

pub fn handle(_payload: &AddHandleKind, doc: &ArtifactView<'_, Block2dSnapshot>, _cfg: &ConfigView<'_, Block2dConfig>) -> Result<Emit<Block2dMutation, Block2dConfigMutation>, Fault> {
    let id = semio_s_artifact_block_2d_schema::next_id(doc.snapshot.handle_kinds.iter().map(|kind| kind.id.as_str()), "handle-kind-");
    let handle_kind = Block2dHandleKind { id: id.clone(), name: id.clone(), label: id, color: "#888888".into(), default_wire_kind: "cable.link".into() };
    Ok(Emit::mutations(vec![semio_s_artifact_block_2d_mut_handles::create_handle_kind(handle_kind)]))
}
