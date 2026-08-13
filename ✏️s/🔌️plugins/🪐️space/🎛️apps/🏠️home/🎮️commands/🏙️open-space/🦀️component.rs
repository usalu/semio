//! 🏙️ 🏙️ S Home launcher app command — `open-space`.

use crate::apps::home::config::{HomeConfig, HomeConfigMutation};

use crate::artifacts::home::op::SHomeMutation;
use crate::artifacts::home::SHomeSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, HostEffect};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "open-space")]
pub struct OpenSpace {
    pub space_id: String,
}

pub fn handle(payload: &OpenSpace, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    eprintln!("[DEBUG] home openSpace id={}", payload.space_id);
    Ok(Emit::effect(HostEffect::Navigate { uri: format!("/spaces/{}", payload.space_id) }))
}
