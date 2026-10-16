//! 🎬️ Product scene payloads for the semantic UI contract's `Component::Surface(SurfaceProps)` — the 15 `SceneDoc` structs each embedded product surface carries as an opaque pack-encoded `SurfaceDoc.bytes` blob, plus the generic 3D scene math shared by every mesh and world viewport.
//!
//! This crate is the thin facade over the scene producers `semio-framework-ui-scene-pack`, `-doc`, `-surface`, `-modelling`, `-world3d`, `-node-graph`, `-plane`, `-views` and the math producers `-algebra`, `-camera`, `-mesh`, `-pass` and `-picking`; it re-exports exactly the names its consumers use, bit for bit the surface of the former monolith. Wasm-safe by construction, product- and renderer-neutral, sync by decree (ruling E6).

#[path = "../../📐️math/🦀️.rs"]
pub mod math;
pub use semio_framework_ui_scene_doc::{scene_lane_hash, SceneDoc, SceneLanePayload, SceneLaneRef};
pub use semio_framework_ui_scene_modelling::{
    format_world3d_legend_value, project_world3d_annotations, world3d_modelling_from_lane_text, world3d_modelling_lane_text, world3d_modelling_strings, world3d_sub_element_paint, World3dAngle, World3dAnnotation, World3dAnnotationLayer,
    World3dColorRamp, World3dDimension, World3dHighlight, World3dLabelAlign, World3dLeader, World3dLegendLine, World3dLegendLines, World3dLegendTick, World3dMarker, World3dMarkerShape, World3dModellingOptions, World3dModellingStrings,
    World3dPickGranularity, World3dPickTargets, World3dProjectedAnnotation, World3dProjectedArc, World3dProjectedArrow, World3dProjectedLabel, World3dProjectedMarker, World3dScalarDomain, World3dScalarField, World3dScalarLegend, World3dScalarRange,
    World3dSection, World3dSectionCap, World3dSubElementDefaults, World3dSubElementPaint, World3dSubElementStyle, World3dText, World3dTone, World3dVec3, WORLD3D_ANGLE_RADIUS_PX_DEFAULT, WORLD3D_ANNOTATIONS_MAX, WORLD3D_ANNOTATION_ARROW_PX,
    WORLD3D_ANNOTATION_LABEL_GAP_PX, WORLD3D_ANNOTATION_MARKER_PX, WORLD3D_ANNOTATION_STROKE_PX, WORLD3D_LEGEND_TICKS_DEFAULT, WORLD3D_SCALAR_NO_DATA_RGB, WORLD3D_SCALAR_VALUES_MAX,
};
pub use semio_framework_ui_scene_node_graph::{
    NodeGraphEdgeRecord, NodeGraphFindItem, NodeGraphHover, NodeGraphInteractionDomain, NodeGraphNodeRecord, NodeGraphOperatorChannelRecord, NodeGraphOperatorRecord, NodeGraphOperatorVariadicRecord, NodeGraphPortRecord, NodeGraphScene,
    NodeGraphSceneLane, NODE_GRAPH_SCENE_LANE_BODY_KEYS, NODE_GRAPH_SCENE_LANE_FIELDS, NODE_GRAPH_SCENE_LANE_NAMES,
};
pub use semio_framework_ui_scene_pack as pack;
pub use semio_framework_ui_scene_plane::{
    board2d_default_brush_weights_json, board2d_default_glyph_catalogs_json, board2d_default_grid_factor, board2d_default_highlighted_ids_json, board2d_default_lod_mode, board2d_default_placement_compatibility_json, board2d_default_selection_json,
    board2d_default_selection_method, board2d_default_true, board2d_highlights_nothing, canvas2d_snapshot_abort_write, canvas2d_snapshot_abort_write_step, canvas2d_snapshot_admit_page, canvas2d_snapshot_begin, canvas2d_snapshot_begin_close,
    canvas2d_snapshot_close_step, canvas2d_snapshot_seal, canvas2d_snapshot_terminal_is_empty, canvas2d_snapshot_with_page, canvas2d_snapshot_write_terminal_is_empty, ink_canvas_default_selection_json, tiled_map_default_hover_json,
    tiled_map_default_layer_stroke_scale_json, tiled_map_default_layer_visibility_json, tiled_map_default_lod_mode, tiled_map_default_render_mode, tiled_map_default_selection_json, tiled_map_default_selection_method,
    tiled_map_default_selection_mode, tiled_map_default_tile_url_template, tiled_map_default_vector_style, tiled_map_default_vector_tile_url_template, Board2dScene, Board2dSceneLane, Canvas2dFrameCamera, Canvas2dFraming,
    Canvas2dRejectedSnapshotPage, Canvas2dScene, Canvas2dSceneLane, Canvas2dSnapshotDescriptor, Canvas2dSnapshotFault, Canvas2dSnapshotLease, Canvas2dSnapshotPage, Canvas2dSnapshotWriteToken, InkCanvasInteractionDomain, InkCanvasScene, Paint2dScene,
    Paint2dSceneLane, TiledMapScene, TiledMapSceneLane, BOARD2D_SCENE_LANE_BODY_KEYS, BOARD2D_SCENE_LANE_FIELDS, BOARD2D_SCENE_LANE_KEY_PREFIX, BOARD2D_SCENE_LANE_NAMES, BOARD2D_SCENE_LANE_OPTIONAL, CANVAS2D_SCENE_LANE_BODY_KEYS,
    CANVAS2D_SCENE_LANE_FIELDS, CANVAS2D_SCENE_LANE_KEY_PREFIX, CANVAS2D_SCENE_LANE_NAMES, CANVAS2D_SCENE_LANE_OPTIONAL, CANVAS2D_SNAPSHOT_CAPACITY, CANVAS2D_SNAPSHOT_PAGE_BYTE_CAPACITY, CANVAS2D_SNAPSHOT_PAGE_CAPACITY, PAINT2D_SCENE_LANE_BODY_KEYS,
    PAINT2D_SCENE_LANE_FIELDS, PAINT2D_SCENE_LANE_KEY_PREFIX, PAINT2D_SCENE_LANE_NAMES, PAINT2D_SCENE_LANE_OPTIONAL, TILEDMAP_SCENE_LANE_BODY_KEYS, TILEDMAP_SCENE_LANE_FIELDS, TILEDMAP_SCENE_LANE_KEY_PREFIX, TILEDMAP_SCENE_LANE_NAMES,
    TILEDMAP_SCENE_LANE_OPTIONAL,
};
pub use semio_framework_ui_scene_surface::{decode, encode, SurfaceDocError, SurfaceEncodeError};
pub use semio_framework_ui_scene_views::{
    apply_draft_changes, compose_draft_step, rebase_text_edits, write_draft_changes_json_into, AppliedTextSplice, BlockListBlock, BlockListPaletteEntry, BlockListScene, BlockListSelectionTarget, BlockListStep, DiffViewScene, DraftChange,
    DraftChangesJsonCursor, EventFeedScene, GraphTimelineScene, IconRenderScene, LocatedTextSplice, TableScene, TextEditorScene, TextSplice, TextSpliceComposition, VirtualFileSystemScene, DRAFT_SPLICES_ARGUMENT, TABLE_COLUMNS_LANE_KEY,
    TABLE_ROWS_LANE_KEY, TEXT_EDITOR_BUFFER_LANE_KEY, TEXT_EDITOR_TYPING_BUFFER_ARG, TEXT_EDITOR_TYPING_COMMIT_ARG, TEXT_EDITOR_TYPING_IDLE_MS, TEXT_SPLICE_CONTEXT_SCALARS, TEXT_SPLICE_MIN_TWO_SIDED_SCALARS,
};
pub use semio_framework_ui_scene_world3d::{
    world3d_default_meshes_json, world3d_default_selection_json, world3d_snapshot_abort_write, world3d_snapshot_abort_write_step, world3d_snapshot_admit_page, world3d_snapshot_begin, world3d_snapshot_begin_close, world3d_snapshot_claim_draw_permit,
    world3d_snapshot_close_step, world3d_snapshot_recover_lease, world3d_snapshot_recover_page, world3d_snapshot_recover_write, world3d_snapshot_recovery_close_step, world3d_snapshot_seal, world3d_snapshot_terminal_is_empty,
    world3d_snapshot_with_page, world3d_snapshot_write_terminal_is_empty, World3dPresentation, World3dPresentationClear, World3dRejectedSnapshotPage, World3dScene, World3dSceneLane, World3dSnapshotDescriptor, World3dSnapshotDrawPermit,
    World3dSnapshotFault, World3dSnapshotItem, World3dSnapshotLease, World3dSnapshotPage, World3dSnapshotPageKind, World3dSnapshotSpan, World3dSnapshotWriteToken, WORLD3D_SCENE_LANE_BODY_KEYS, WORLD3D_SCENE_LANE_FIELDS,
    WORLD3D_SCENE_LANE_KEY_PREFIX, WORLD3D_SCENE_LANE_NAMES, WORLD3D_SCENE_LANE_OPTIONAL, WORLD3D_SNAPSHOT_CAPACITY, WORLD3D_SNAPSHOT_DRAW_BYTE_CAPACITY, WORLD3D_SNAPSHOT_DRAW_CAPACITY, WORLD3D_SNAPSHOT_DRAW_INSTANCE_CAPACITY,
    WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY, WORLD3D_SNAPSHOT_PAGE_CAPACITY, WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY,
};

#[cfg(test)]
#[path = "../../🧪️tests/🔬️scene-catalog-unit/🦀️.rs"]
mod tests;
