//! 👁️ 👁️ Draw play app commands command — `select-all`.

use crate::apps::draw::commands::canvas_pointer_down::DrawSession;
use crate::apps::draw::config::{DrawConfig, DrawConfigMutation};
use crate::artifacts::draw::schema::{flatten_draw_layers, layer_id};
use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::{DrawCamera, DrawSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "select-all")]
pub struct SelectAll {}

pub fn handle(_payload: &SelectAll, doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let document = doc.snapshot;
    let ids = flatten_draw_layers(&document.layers).into_iter().map(|layer| layer_id(layer).to_string()).collect();
    Ok(Emit::config(vec![DrawConfigMutation::SetSelection { ids }]))
}
