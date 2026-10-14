//! 🔘️ 🔘️ Block 5D play app commands command — `add-grip-kind`.

use semio_s_artifact_block_5d_mutations::Block5dMutation;
use semio_s_artifact_block_5d_model::{Block5dGripKind, Block5dSnapshot};
use crate::config::{Block5dConfig, Block5dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "addGripKind")]
pub struct AddGripKind {}

pub fn handle(_payload: &AddGripKind, doc: &ArtifactView<'_, Block5dSnapshot>, _cfg: &ConfigView<'_, Block5dConfig>) -> Result<Emit<Block5dMutation, Block5dConfigMutation>, Fault> {
    let id = semio_s_artifact_block_5d_schema::next_id(doc.snapshot.grip_kinds.iter().map(|kind| kind.id.as_str()), "grip-kind-");
    let grip_kind = Block5dGripKind { id: id.clone(), name: id.clone(), label: id, color: "#888888".into(), default_rope_kind: "rope.link".into() };
    Ok(Emit::mutations(vec![semio_s_artifact_block_5d_mut_grip_kinds::create_grip_kind(grip_kind)]))
}
