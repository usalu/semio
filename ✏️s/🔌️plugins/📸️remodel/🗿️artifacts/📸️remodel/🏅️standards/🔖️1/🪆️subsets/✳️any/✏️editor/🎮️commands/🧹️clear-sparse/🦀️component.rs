//! 🧹️ 🧹️ Remodel play app commands command — `clear-sparse`.

use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::artifacts::remodel::mutations::replace_sparse;
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "clear-sparse")]
pub struct ClearSparse {}

pub async fn handle(_payload: &ClearSparse, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![replace_sparse(None)]))
}
