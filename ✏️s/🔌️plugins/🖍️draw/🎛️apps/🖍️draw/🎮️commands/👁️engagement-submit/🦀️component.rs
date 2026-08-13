//! 👁️ 👁️ Draw play app commands command — `engagement-submit`.

use crate::apps::draw::commands::canvas_pointer_down::DrawSession;
use crate::apps::draw::config::{DrawConfig, DrawConfigMutation};
use crate::artifacts::draw::schema::{flatten_draw_layers, layer_id};
use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::{DrawCamera, DrawSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "engagement-submit")]
pub struct EngagementSubmit {
    pub value: Option<String>,
}

/// ✏️ Renames the single selected layer to the submitted engagement-input text (or the config's
/// own in-progress `engagement_input` if the caller doesn't pass one) — the one `Config`-only
/// row that actually mutates the document, mirroring the pre-migration behaviour exactly.
pub fn handle(payload: &EngagementSubmit, _doc: &ArtifactView<'_, DrawSnapshot>, cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let config = cfg.snapshot;
    let value = payload.value.clone().unwrap_or_else(|| config.engagement_input.clone());
    let value = value.trim();
    if value.is_empty() || config.selected_ids.len() != 1 {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![crate::artifacts::draw::mutations::rename_layer(config.selected_ids[0].clone(), value.into())]))
}
