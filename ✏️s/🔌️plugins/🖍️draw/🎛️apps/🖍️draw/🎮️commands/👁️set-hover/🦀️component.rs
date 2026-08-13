//! 👁️ 👁️ Draw play app commands command — `set-hover`.

use crate::apps::draw::commands::canvas_pointer_down::DrawSession;
use crate::apps::draw::config::{DrawConfig, DrawConfigMutation};
use crate::artifacts::draw::schema::{flatten_draw_layers, layer_id};
use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::{DrawCamera, DrawSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-hover")]
pub struct SetHover {
    pub id: Option<String>,
}

pub fn handle(payload: &SetHover, _doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    Ok(Emit::config(vec![DrawConfigMutation::SetHovered { id: payload.id.clone() }]))
}
