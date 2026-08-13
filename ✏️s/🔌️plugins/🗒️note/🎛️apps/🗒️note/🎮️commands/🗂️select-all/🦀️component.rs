//! 🗂️ 🗂️ Note play app commands command — `select-all`.

use crate::apps::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::schema::{block_id, flatten_blocks};
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "select-all")]
pub struct SelectAll {}

pub fn handle(_payload: &SelectAll, doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    let ids: Vec<String> = flatten_blocks(&doc.snapshot.blocks).into_iter().map(|block| block_id(block).into()).collect();
    Ok(Emit::config(vec![NoteConfigMutation::SetSelection { block_ids: ids }]))
}
