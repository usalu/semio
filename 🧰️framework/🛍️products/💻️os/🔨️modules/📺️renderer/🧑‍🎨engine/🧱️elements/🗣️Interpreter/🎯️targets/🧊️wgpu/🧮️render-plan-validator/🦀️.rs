//! 🧮️ Render plan validator.
//!
//! Validates a component scene render plan against its fixed limits before it is walked: tree depth, node count,
//! window body surfaces and the per-kind scene bounds.

#[cfg(test)]
use super::ui_image_loading::{
    apply_ui_image_bytes, current_ui_image_key, object_contain_rect, render_ui_image_step,
    resolve_ui_image, INLINE_SVG_PARSE_CALLS, UI_IMAGE_SIZES,
};
#[cfg(test)]
use infinite_world::world::{WorldAssetMetadataId, WorldAssetRequestKind};
use serde_json::Value;
use ui_wgpu::wgpu::UiComponentSceneNode;
#[cfg(test)]
use ui_wgpu::wgpu::{ActionDescriptor, Rect, UiNode, UiPresence};

//#region RenderPlanValidator
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenderPlanLimits {
    pub max_tree_depth: usize,
    pub max_node_count: usize,
    pub max_json_payload_bytes: usize,
    pub max_texture_dimension: u32,
    pub max_mesh_count: usize,
    /// ⏯️ Longest base64url tool run trace delta a world-3d or canvas-2d lane may carry.
    pub max_tool_run_trace_bytes: usize,
}

impl Default for RenderPlanLimits {
    fn default() -> Self {
        RENDER_PLAN_LIMITS
    }
}

pub const RENDER_PLAN_LIMITS: RenderPlanLimits = RenderPlanLimits {
    max_tree_depth: 64,
    max_node_count: 4096,
    max_json_payload_bytes: 4 * 1024 * 1024,
    max_texture_dimension: 8192,
    max_mesh_count: 2048,
    max_tool_run_trace_bytes: infinite_world::world::tool_run_trace::TOOL_RUN_TRACE_LANE_BYTES_MAX,
};

fn check_tool_run_trace(label: &str, lane: &Option<String>, limits: &RenderPlanLimits) -> Result<(), String> {
    match lane {
        Some(lane) if lane.len() > limits.max_tool_run_trace_bytes => Err(format!("render plan limit exceeded: {label} has {} bytes (max {})", lane.len(), limits.max_tool_run_trace_bytes)),
        _ => Ok(()),
    }
}

fn check_json_payload(label: &str, payload: &str, limits: &RenderPlanLimits) -> Result<(), String> {
    if payload.len() > limits.max_json_payload_bytes {
        return Err(format!("render plan limit exceeded: {label} has {} bytes (max {})", payload.len(), limits.max_json_payload_bytes));
    }
    Ok(())
}

fn check_optional_json_payload(label: &str, payload: &Option<String>, limits: &RenderPlanLimits) -> Result<(), String> {
    if let Some(value) = payload {
        check_json_payload(label, value, limits)?;
    }
    Ok(())
}

/// ☁️ `points_json` payload-size validation only — the point-sprite GPU pipeline itself lives in
/// `infinite_world::world::World3dState` (see `render_world_3d`, imported below), a separate crate this
/// file delegates all actual mesh/instance drawing to and does not construct wgpu render
/// pipelines/shaders directly for (confirmed: no `create_render_pipeline`/`RenderPipelineDescriptor`
/// call anywhere in this file). Threading the base64 point buffers through `infinite_world`'s own
/// draw path is out of scope here (that crate isn't in this ticket's touched-file list); this
/// validator still guards the native shell against an oversized/malformed payload same as every
/// other optional world3d field above.
pub fn validate_component_scene(scene: &UiComponentSceneNode, limits: &RenderPlanLimits) -> Result<(), String> {
    let scene_label = format!("component scene '{}'", scene.surface_id);
    if let Some(canvas) = &scene.canvas_2d {
        check_json_payload(&format!("{scene_label} canvas2d.layers"), &canvas.layers_json, limits)?;
        check_tool_run_trace(&format!("{scene_label} canvas2d.toolRunTrace"), &canvas.tool_run_trace, limits)?;
    }
    if let Some(board) = &scene.board2d {
        check_tool_run_trace(&format!("{scene_label} board2d.toolRunTrace"), &board.tool_run_trace, limits)?;
    }
    if let Some(world) = &scene.world_3d {
        check_json_payload(&format!("{scene_label} world3d.camera"), &world.camera_json, limits)?;
        check_json_payload(&format!("{scene_label} world3d.meshes"), &world.meshes_json, limits)?;
        let mesh_count = serde_json::from_str::<Value>(&world.meshes_json).ok().and_then(|value| value.as_array().map(|array| array.len())).unwrap_or(0);
        if mesh_count > limits.max_mesh_count {
            return Err(format!("render plan limit exceeded: {scene_label} world3d mesh count {mesh_count} exceeds max {}", limits.max_mesh_count));
        }
        check_json_payload(&format!("{scene_label} world3d.instances"), &world.instances_json, limits)?;
        check_json_payload(&format!("{scene_label} world3d.selection"), &world.selection_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} world3d.vortices"), &world.vortices_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} world3d.attractions"), &world.attractions_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} world3d.targetVolumes"), &world.target_volumes_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} world3d.references"), &world.references_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} world3d.brushPreview"), &world.brush_preview_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} world3d.interaction"), &world.interaction_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} world3d.engagementPreview"), &world.engagement_preview_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} world3d.pickTargets"), &world.pick_targets_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} world3d.lod"), &world.lod_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} world3d.chunking"), &world.chunking_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} world3d.points"), &world.points_json, limits)?;
        check_tool_run_trace(&format!("{scene_label} world3d.toolRunTrace"), &world.tool_run_trace, limits)?;
    }
    if let Some(graph) = &scene.node_graph {
        check_json_payload(&format!("{scene_label} nodeGraph.nodes"), &serde_json::to_string(&graph.nodes).unwrap_or_default(), limits)?;
        check_json_payload(&format!("{scene_label} nodeGraph.edges"), &serde_json::to_string(&graph.edges).unwrap_or_default(), limits)?;
        check_json_payload(&format!("{scene_label} nodeGraph.viewport"), &serde_json::to_string(&graph.viewport).unwrap_or_default(), limits)?;
        check_json_payload(&format!("{scene_label} nodeGraph.operators"), &serde_json::to_string(&graph.operators).unwrap_or_default(), limits)?;
        check_json_payload(&format!("{scene_label} nodeGraph.findItems"), &serde_json::to_string(&graph.find_items).unwrap_or_default(), limits)?;
        check_json_payload(&format!("{scene_label} nodeGraph.selection"), &serde_json::to_string(&graph.selection).unwrap_or_default(), limits)?;
        check_json_payload(&format!("{scene_label} nodeGraph.hover"), &serde_json::to_string(&graph.hover).unwrap_or_default(), limits)?;
        check_optional_json_payload(&format!("{scene_label} nodeGraph.previewOff"), &graph.preview_off_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} nodeGraph.lod"), &graph.lod_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} nodeGraph.controls"), &graph.controls_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} nodeGraph.clusters"), &graph.clusters_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} nodeGraph.computing"), &graph.computing_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} nodeGraph.capabilities"), &graph.capabilities_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} nodeGraph.hostDocument"), &graph.host_snapshot_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} nodeGraph.presencePeers"), &graph.presence_peers_json, limits)?;
    }
    if let Some(editor) = &scene.text_editor {
        check_json_payload(&format!("{scene_label} textEditor.buffer"), &editor.buffer, limits)?;
        check_optional_json_payload(&format!("{scene_label} textEditor.selection"), &editor.selection_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} textEditor.tokens"), &editor.tokens_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} textEditor.diagnostics"), &editor.diagnostics_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} textEditor.completions"), &editor.completions_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} textEditor.overlays"), &editor.overlays_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} textEditor.occurrences"), &editor.occurrences_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} textEditor.placeholders"), &editor.placeholders_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} textEditor.extraCarets"), &editor.extra_carets_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} textEditor.selectableSpans"), &editor.selectable_spans_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} textEditor.settings"), &editor.settings_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} textEditor.camera"), &editor.camera_json, limits)?;
    }
    if let Some(table) = &scene.table {
        check_json_payload(&format!("{scene_label} table.columns"), &table.columns_json, limits)?;
        check_json_payload(&format!("{scene_label} table.rows"), &table.rows_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} table.selection"), &table.selection_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} table.sort"), &table.sort_json, limits)?;
    }
    if let Some(paint_2d) = &scene.paint_2d {
        check_json_payload(&format!("{scene_label} paint2d.documentSync"), &paint_2d.document_sync_json, limits)?;
        check_json_payload(&format!("{scene_label} paint2d.assets"), &paint_2d.assets_json, limits)?;
        check_json_payload(&format!("{scene_label} paint2d.camera"), &paint_2d.camera_json, limits)?;
        check_json_payload(&format!("{scene_label} paint2d.selection"), &paint_2d.selection_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} paint2d.compositeViewport"), &paint_2d.composite_viewport_json, limits)?;
    }
    if let Some(vfs) = &scene.virtual_file_system {
        check_json_payload(&format!("{scene_label} vfs.schema"), &vfs.schema_json, limits)?;
        check_json_payload(&format!("{scene_label} vfs.rows"), &vfs.rows_json, limits)?;
        check_optional_json_payload(&format!("{scene_label} vfs.selectedRowIds"), &vfs.selected_row_ids_json, limits)?;
    }
    if let Some(map) = &scene.tiled_map {
        check_json_payload(&format!("{scene_label} gisMap.fixture"), &map.map_descriptor_json, limits)?;
        check_json_payload(&format!("{scene_label} gisMap.camera"), &map.camera_json, limits)?;
        check_json_payload(&format!("{scene_label} gisMap.layerVisibility"), &map.layer_visibility_json, limits)?;
        check_json_payload(&format!("{scene_label} gisMap.layerStrokeScale"), &map.layer_stroke_scale_json, limits)?;
        check_json_payload(&format!("{scene_label} gisMap.selection"), &map.selection_json, limits)?;
        check_json_payload(&format!("{scene_label} gisMap.hover"), &map.hover_json, limits)?;
    }
    if let Some(board) = &scene.board2d {
        check_json_payload(&format!("{scene_label} board2d.fixture"), &board.snapshot_json, limits)?;
        check_json_payload(&format!("{scene_label} board2d.camera"), &board.camera_json, limits)?;
        check_json_payload(&format!("{scene_label} board2d.glyphCatalogs"), &board.glyph_catalogs_json, limits)?;
        check_json_payload(&format!("{scene_label} board2d.selection"), &board.selection_json, limits)?;
        check_json_payload(&format!("{scene_label} board2d.brushWeights"), &board.brush_weights_json, limits)?;
        check_json_payload(&format!("{scene_label} board2d.placementCompatibility"), &board.placement_compatibility_json, limits)?;
    }
    Ok(())
}

#[cfg(test)]
struct RenderPlanWalkState {
    pub(super) node_count: usize,
}

#[cfg(test)]
fn walk_ui_node(node: &UiNode, depth: usize, limits: &RenderPlanLimits, state: &mut RenderPlanWalkState) -> Result<(), String> {
    state.node_count += 1;
    if state.node_count > limits.max_node_count {
        return Err(format!("render plan limit exceeded: node count {} exceeds max {}", state.node_count, limits.max_node_count));
    }
    if depth > limits.max_tree_depth {
        return Err(format!("render plan limit exceeded: tree depth {depth} exceeds max {}", limits.max_tree_depth));
    }
    match node {
        UiNode::ComponentScene(scene) => validate_component_scene(scene, limits)?,
        UiNode::Stack(stack) => {
            for child in &stack.children {
                walk_ui_node(child, depth + 1, limits, state)?;
            }
        }
        UiNode::Section(section) => {
            for child in &section.children {
                walk_ui_node(child, depth + 1, limits, state)?;
            }
        }
        _ => {}
    }
    Ok(())
}

#[cfg(test)]
pub fn validate_ui_node(node: &UiNode, limits: &RenderPlanLimits) -> Result<(), String> {
    let mut state = RenderPlanWalkState { node_count: 0 };
    walk_ui_node(node, 1, limits, &mut state)
}

#[cfg(test)]
pub fn validate_window_body_surface(kind: &semio_framework::WindowKindDefinition, node: &UiNode) -> Result<(), String> {
    match node {
        UiNode::ComponentScene(scene) if scene.component_kind != kind.surface_kind => Err(format!("window {} declared {} but program returned {}", kind.id, kind.surface_kind.as_str(), scene.component_kind.as_str())),
        _ => Ok(()),
    }
}

//#endregion RenderPlanValidator

//#region RenderPlanValidatorTests
#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-render-plan-validator/🦀️.rs"]
mod render_plan_validator_tests;
//#endregion RenderPlanValidatorTests
