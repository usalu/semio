//! 🏙️ 🏙️ S Home launcher app command — `open-space`.

use crate::editor::home::config::{HomeConfig, HomeConfigMutation};

use crate::artifacts::home::op::SHomeMutation;
use crate::artifacts::home::SHomeSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, Effect};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "open-space")]
pub struct OpenSpace {
    pub space_id: String,
}

pub async fn handle(payload: &OpenSpace, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    eprintln!("[DEBUG] home openSpace id={}", payload.space_id);
    Ok(Emit::effect(Effect::Navigate { uri: format!("/spaces/{}", payload.space_id) }))
}
