//! 🧩️ Wgpu engine-surface attach — the ONE production seam that constructs the engine host behind
//! every `SurfaceKind` the wgpu shell can paint, generalised from the node-graph-only seam of
//! `📓️wgpu-node-graph-2026-09-10.md` §2.
//!
//! Three of the four kinds composite through a vello texture and attach in `ENGINE_SURFACES`
//! (`sync_engine_scene`); `World3d` attaches against the shell's own `world3d_states` and paints a
//! real 3-D pass into the window's retained draw list. All four are recorded through the same drained
//! `EngineSurfaceRegistration` list, which is what the shell mirrors into its pointer-dispatch maps.
//!
//! The `World3d` payload is generation3d's own `hexagonal-mushroom-column` preview scene, committed as
//! data in `🔣️.json` with its provenance; the action payloads are pinned against React's `World3dHost`
//! (`🧱️elements/🌍️World3dHost/🟦️.tsx`), the other implementation of this same host.

use super::*;
use crate::surface_fixtures::*;
use semio_framework_os_renderer_wgpu_engine_attach::drop_engine_surface;
use infinite_world::world::{
    begin_world3d_dynamic_retirement, enqueue_world3d_event, step_world3d_draw_rebuild, step_world3d_dynamic_retirement, step_world3d_interaction, step_world3d_scene_bridge, step_world3d_snapshot, world3d_dynamic_retirement_terminal_is_empty,
    World3dBuildContext, World3dSceneBridgeStep, World3dSnapshotApplyStep, World3dState, WorldCursorWakeAuthority, WorldDrawRebuildStep, WorldInteractionAuthorityStep, WorldInteractionIntent,
};
use semio_framework_ui_declaration::ui::SurfaceKind;
use semio_framework_ui_draw_contract::{DrawList, IconAtlas};
use semio_framework_ui_layout::layout::UiPresence;
use semio_framework_ui_scene_plane::{Board2dScene, TiledMapScene};
use semio_framework_ui_scene_world3d::World3dScene;
use semio_framework_ui_text::FontAtlas;
use ui_wgpu::wgpu::InputState;

include!("🗃️fixtures-and-retained-map-records/🦀️.rs");

include!("🚪️engine-close-and-world3d-fixtures/🦀️.rs");

include!("🌐️world3d-window-pointer-and-orbit-laws/🦀️.rs");

include!("🗺️tiled-map-and-board-attach-laws/🦀️.rs");

include!("🤏️two-touch-gestures-and-hover/🦀️.rs");

include!("♻️retained-map-gesture-lifecycle-and-bridge/🦀️.rs");
