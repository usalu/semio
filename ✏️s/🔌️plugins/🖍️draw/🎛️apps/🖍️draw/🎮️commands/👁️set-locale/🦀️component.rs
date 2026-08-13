//! 👁️ 👁️ Draw play app commands command — `set-locale`.

use crate::apps::draw::commands::canvas_pointer_down::DrawSession;
use crate::apps::draw::config::{DrawConfig, DrawConfigMutation};
use crate::artifacts::draw::schema::{flatten_draw_layers, layer_id};
use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::{DrawCamera, DrawSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "locale")]
pub struct SetLocale {
    pub value: String,
}

pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    Ok(Emit::config(vec![DrawConfigMutation::SetLocale { value: payload.value.clone() }]))
}
