//! 🏙️ 🏙️ S Home launcher app command — `open-space`.

use crate::editor::home::config::{HomeConfig, HomeConfigMutation};

use crate::artifacts::home::op::SHomeMutation;
use crate::artifacts::home::SHomeSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "open-space")]
pub struct OpenSpace {
    pub space_id: String,
}

pub fn handle(payload: &OpenSpace, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    eprintln!("[DEBUG] home openSpace id={}", payload.space_id);
    Ok(Emit::effect(Effect::Navigate { uri: format!("/spaces/{}", payload.space_id) }))
}
