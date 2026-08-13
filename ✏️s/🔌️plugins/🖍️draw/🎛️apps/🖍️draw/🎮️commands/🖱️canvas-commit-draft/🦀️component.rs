//! 🖱️ 🖱️ Draw play app commands command — `canvas-commit-draft`.

use crate::apps::draw::config::{DrawConfig, DrawConfigMutation};
use crate::artifacts::draw::schema::{create_draw_path_layer, create_draw_trace_layer, draw_layer_world_bounds, draw_transform_to_matrix, find_draw_layer, flatten_draw_layers, layer_base, layer_id, layer_to_path_segments};
use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::{DrawCamera, DrawSnapshot, DrawLayerNode, PathSegment};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use crate::apps::draw::commands::canvas_pointer_down::{draw_gesture, finish_gesture_emit, DrawSession};
use serde::{Deserialize, Serialize};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "canvas-commit-draft")]
pub struct CanvasCommitDraft {}

pub fn handle(_payload: &CanvasCommitDraft, doc: &ArtifactView<'_, DrawSnapshot>, cfg: &ConfigView<'_, DrawConfig>, session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let document = doc.snapshot;
    let mut config = cfg.snapshot.clone();
    let emit = session.step_gesture(draw_gesture::Event::CommitDraft, document, &mut config);
    Ok(finish_gesture_emit(emit, cfg.snapshot, &config))
}
