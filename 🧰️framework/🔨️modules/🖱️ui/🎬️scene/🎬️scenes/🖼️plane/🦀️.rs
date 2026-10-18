//! 🖼️ The flat-surface scene payloads (canvas, paint, board, tiled map, ink canvas) with the canvas framing and snapshot lease.
//!
//! Carved out of the former `semio-framework-ui-scene` scenes module by ticket `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`.

use protocol::value::{DslValue, FromValue, ToValue, ValueError};
use semio_framework_ui_scene_doc::{
    deserialize_non_empty_interaction_domain_text, scene_lane_hash, scene_pack_wire, value_decode, value_decode_default, value_decode_option, value_push, value_push_if_nonempty, value_push_option, SceneDoc, SceneLanePayload, SceneLaneRef,
};
use semio_framework_ui_scene_pack::{from_bytes, to_bytes, PackError};
use serde::{Deserialize, Serialize};
#[path = "../../📷️framing/🦀️.rs"]
mod framing;
pub use framing::{Canvas2dFrameCamera, Canvas2dFraming};
#[path = "../../🖼️canvas2d-snapshot/🦀️.rs"]
mod canvas2d_snapshot;
pub use canvas2d_snapshot::{
    canvas2d_snapshot_abort_write, canvas2d_snapshot_abort_write_step, canvas2d_snapshot_admit_page, canvas2d_snapshot_begin, canvas2d_snapshot_begin_close, canvas2d_snapshot_close_step, canvas2d_snapshot_seal, canvas2d_snapshot_terminal_is_empty,
    canvas2d_snapshot_with_page, canvas2d_snapshot_write_terminal_is_empty, Canvas2dRejectedSnapshotPage, Canvas2dSnapshotDescriptor, Canvas2dSnapshotFault, Canvas2dSnapshotLease, Canvas2dSnapshotPage, Canvas2dSnapshotWriteToken,
    CANVAS2D_SNAPSHOT_CAPACITY, CANVAS2D_SNAPSHOT_PAGE_BYTE_CAPACITY, CANVAS2D_SNAPSHOT_PAGE_CAPACITY,
};

#[path = "🔲️canvas2d-scene/🦀️.rs"]
mod canvas2d_scene;
pub use canvas2d_scene::{
    Canvas2dScene, Canvas2dSceneLane, CANVAS2D_SCENE_LANE_BODY_KEYS, CANVAS2D_SCENE_LANE_FIELDS,
    CANVAS2D_SCENE_LANE_KEY_PREFIX, CANVAS2D_SCENE_LANE_NAMES, CANVAS2D_SCENE_LANE_OPTIONAL,
};

#[path = "🎨️paint2d-scene/🦀️.rs"]
mod paint2d_scene;
pub use paint2d_scene::{
    Paint2dScene, Paint2dSceneLane, PAINT2D_SCENE_LANE_BODY_KEYS, PAINT2D_SCENE_LANE_FIELDS,
    PAINT2D_SCENE_LANE_KEY_PREFIX, PAINT2D_SCENE_LANE_NAMES, PAINT2D_SCENE_LANE_OPTIONAL,
};

#[path = "🗺️tiled-map-scene/🦀️.rs"]
mod tiled_map_scene;
pub use tiled_map_scene::{
    tiled_map_default_hover_json, tiled_map_default_layer_stroke_scale_json,
    tiled_map_default_layer_visibility_json, tiled_map_default_lod_mode,
    tiled_map_default_render_mode, tiled_map_default_selection_json,
    tiled_map_default_selection_method, tiled_map_default_selection_mode,
    tiled_map_default_tile_url_template, tiled_map_default_vector_style,
    tiled_map_default_vector_tile_url_template, TiledMapScene, TiledMapSceneLane,
    TILEDMAP_SCENE_LANE_BODY_KEYS, TILEDMAP_SCENE_LANE_FIELDS, TILEDMAP_SCENE_LANE_KEY_PREFIX,
    TILEDMAP_SCENE_LANE_NAMES, TILEDMAP_SCENE_LANE_OPTIONAL,
};

#[path = "🎲️board2d-scene/🦀️.rs"]
mod board2d_scene;
pub use board2d_scene::{
    board2d_default_brush_weights_json, board2d_default_glyph_catalogs_json,
    board2d_default_grid_factor, board2d_default_highlighted_ids_json, board2d_default_lod_mode,
    board2d_default_placement_compatibility_json, board2d_default_selection_json,
    board2d_default_selection_method, board2d_default_true, board2d_highlights_nothing,
    Board2dScene, Board2dSceneLane, BOARD2D_SCENE_LANE_BODY_KEYS, BOARD2D_SCENE_LANE_FIELDS,
    BOARD2D_SCENE_LANE_KEY_PREFIX, BOARD2D_SCENE_LANE_NAMES, BOARD2D_SCENE_LANE_OPTIONAL,
};

#[path = "🖋️ink-canvas-scene/🦀️.rs"]
mod ink_canvas_scene;
pub use ink_canvas_scene::{
    ink_canvas_default_selection_json, InkCanvasInteractionDomain, InkCanvasScene,
};

#[cfg(test)]
#[path = "../../🧪️tests/🔬️plane-unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "../../🧪️tests/🔬️plane-value-round-trip/🦀️.rs"]
mod value_round_trip_tests;
