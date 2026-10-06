//! 🏙️ 🏙️ S Home launcher app command — `open-space`.

use crate::editor::home::config::{HomeConfig, HomeConfigMutation};

use crate::standards::v1::subsets::any::schema::mutations::SHomeMutation;
use crate::SHomeSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "open-space")]
pub struct OpenSpace {
    pub space_id: String,
}

pub fn handle(payload: &OpenSpace, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    Ok(Emit::effect(Effect::Navigate { uri: format!("/spaces/{}", payload.space_id) }))
}
