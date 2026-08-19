//! 🧹️ 🧹️ Remodel play app commands command — `clear-tracks`.

use crate::editor::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::artifacts::remodel::mutations::replace_tracks;
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "clear-tracks")]
pub struct ClearTracks {}

pub async fn handle(_payload: &ClearTracks, _doc: &ArtifactView<'_, RemodelSnapshot>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![replace_tracks(Vec::new())]))
}
