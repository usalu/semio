//! 🖱️ Declarative UI components (default) and retained-mode wgpu engine (feature "wgpu-engine").

#[path = "../../../../../🖼️assets/🔣️icons/🤖️generated/🦀️icon_name.rs"]
mod icon_name_gen;

pub use icon_name_gen::IconName;

//#region 🔖️UiAxes
#[path = "🤖️generated.rs"]
mod ui_axes_gen;

pub use ui_axes_gen::{Locale, Terminology};
//#endregion 🔖️UiAxes

//#region 🔖️Label
#[path = "🦀️label.rs"]
mod label_impl;
pub use label_impl::*;
//#endregion 🔖️Label

// #region component
// 🧩️ Declarative UI component model (declarative `UiNode` tree, scene records, `SurfaceKind`, `WindowLayout`/`WindowEngagement`/`WindowMeasure`, `UtilityNode`) — moved verbatim from framework/core/rs/lib.rs; JSON wire format is byte-identical to the pre-move version (see the inline `*_wire_format_tests` mods). Ungated (default features) so wasm32-wasip2 program builds stay dependency-clean; must never reference `semio_framework`.
#[path = "🦀️component.rs"]
pub mod component;
// #endregion component

#[cfg(feature = "wgpu-engine")]
#[path = "🦀️arena.rs"]
pub mod arena;

#[cfg(feature = "wgpu-engine")]
#[path = "🦀️tree.rs"]
pub mod tree;

#[cfg(feature = "wgpu-engine")]
#[path = "🦀️reconcile.rs"]
pub mod reconcile;

#[cfg(feature = "wgpu-engine")]
#[path = "🦀️chrome.rs"]
pub mod chrome;

#[cfg(feature = "wgpu-engine")]
#[path = "🦀️cursor.rs"]
pub mod cursor;

#[cfg(feature = "wgpu-engine")]
#[path = "🦀️draw.rs"]
pub mod draw;

#[path = "🦀️geometry.rs"]
pub mod geometry;

/// 🗺️ Reusable minimap-navigator layout math — panel/viewport placement, content-fit checks, and
/// screen<->world mapping for a bottom-right pannable-camera minimap widget (wgpu parity with the dag
/// board's `MinimapWidget`). Relocated (as pure geometry, not the paint call) from
/// `♾️infinite/🎲️board/directed/🕸️dag`'s private `impl` methods — see
/// `.🦑️repo/🎫️tickets/26/08/05/FRAMEWORK-BUILDER-PASSTHROUGHS-APP-COMMANDS-MACRO-WIDGET-EXTRACTION`.
///
/// Deliberately NOT nested inside `widgets` (that module is `#[cfg(feature = "wgpu-engine")]`, pulling in
/// wgpu/winit/parley/kernel_3d_scene): this math has zero rendering-backend dependency, so it lives at
/// the crate's lightweight (default-feature) tier instead, letting a vello/canvas-based consumer like the
/// dag board depend on `ui_wgpu` WITHOUT the heavyweight `engine` feature. The dag board's own
/// `paint_minimap_widget` (the actual `vello::Scene` fill/stroke calls, keyed off DAG-specific node types)
/// stays where it is — that part is genuinely backend- and app-specific, not portable geometry.
#[path = "🦀️minimap.rs"]
pub mod minimap;

#[cfg(feature = "wgpu-engine")]
#[path = "🦀️gpu.rs"]
pub mod gpu;

#[cfg(feature = "wgpu-engine")]
#[path = "🦀️prepared.rs"]
pub mod prepared;

#[cfg(feature = "wgpu-engine")]
#[path = "🦀️input.rs"]
pub mod input;

#[cfg(feature = "wgpu-engine")]
#[path = "🦀️action.rs"]
pub mod action;

#[cfg(feature = "wgpu-engine")]
#[path = "🦀️layout.rs"]
pub mod layout;

#[cfg(feature = "wgpu-engine")]
#[path = "🦀️flex.rs"]
pub mod flex;

#[cfg(feature = "wgpu-engine")]
#[path = "🦀️mounted_layout.rs"]
pub mod mounted_layout;

#[cfg(feature = "wgpu-engine")]
#[path = "🦀️shaders.rs"]
pub mod shaders;

#[cfg(feature = "wgpu-engine")]
#[path = "🦀️text.rs"]
pub mod text;

#[path = "🦀️theme.rs"]
pub mod theme;

#[cfg(feature = "wgpu-engine")]
#[path = "🦀️paint.rs"]
pub mod paint;

#[cfg(feature = "wgpu-engine")]
#[path = "🦀️events.rs"]
pub mod events;

#[cfg(feature = "wgpu-engine")]
#[path = "🦀️scene_slots.rs"]
pub mod scene_slots;

#[cfg(feature = "wgpu-engine")]
#[path = "🦀️shell.rs"]
pub mod shell;

#[cfg(feature = "wgpu-engine")]
#[path = "🦀️engine.rs"]
pub mod engine;

#[cfg(feature = "wgpu-engine")]
#[path = "../../../../🧱️elements/☑️Select/🎯️targets/🧊️wgpu/🦀️.rs"]
mod select;

#[cfg(feature = "wgpu-engine")]
#[path = "../../../../🧱️elements/🔘️Button/🧊️component.rs"]
mod button;

#[cfg(feature = "wgpu-engine")]
#[path = "../../../../🧱️elements/✏️Input/🎯️targets/🧊️wgpu/🦀️.rs"]
mod input_element;

#[cfg(feature = "wgpu-engine")]
#[path = "../../../../🧱️elements/🎚️Toggle/🧊️component.rs"]
mod toggle;

#[cfg(feature = "wgpu-engine")]
#[path = "../../../../🧱️elements/🔑️KeyValue/🧊️component.rs"]
mod key_value;

#[cfg(feature = "wgpu-engine")]
#[path = "../../../../🧱️elements/🎚️Slider/🧊️component.rs"]
mod slider;

#[cfg(feature = "wgpu-engine")]
#[path = "../../../../🧱️elements/🪜️Stepper/🧊️component.rs"]
mod stepper;

#[cfg(feature = "wgpu-engine")]
#[path = "../../../../🧱️elements/⭕️Ring/🧊️component.rs"]
mod ring;

#[cfg(feature = "wgpu-engine")]
#[path = "../../../../🧱️elements/🔣️IconSelector/🧊️component.rs"]
mod icon_selector;

#[cfg(feature = "wgpu-engine")]
#[path = "../../../../🧱️elements/🪵️Tree/🧊️component.rs"]
mod tree_element;

// 👥️ `PresenceBar` (ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS lane 2-F) builds
// a plain `UiNode` tree via `component::ui`'s declarative types, not a `widgets`/`WidgetContext` chrome
// renderer like its neighbours above — so it needs only the light `wgpu` feature, never `wgpu-engine`.
#[cfg(feature = "wgpu")]
#[path = "../../../../🧱️elements/👥️PresenceBar/🧊️component.rs"]
pub mod presence_bar;

#[cfg(feature = "wgpu-engine")]
#[path = "🦀️widgets.rs"]
pub mod widgets;

#[cfg(all(feature = "wgpu-engine", not(target_os = "wasi")))]
#[path = "🦀️host.rs"]
pub mod host;

// #region re-exports
// 🧩️ Always available: declarative component types + engine-agnostic primitives (default features).
pub use component::layout::{
    build_shell_context_menu_specs, collect_window_kind_ids_from_layout, create_default_layout, create_named_layout, create_stack_layout, create_tab_stack_layout, create_window_layout, default_viewport_engagement, even_window_layout,
    framework_panel_tab_label, merge_named_layouts, organize_context_menu, partition_window_measures, ribbon_parent_label, ActionDescriptor, MeasureSelectItem, NamedLayout, ShellMenuAction, StyleSpec, WindowEngagement, WindowEngagementControl,
    WindowEngagementInput, WindowEngagementOption, WindowEngagementPossible, WindowEngagementSlot, WindowEngagementStatus, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode,
    WindowMeasure, WindowOptions, WindowStackCorner, FRAMEWORK_HISTORY_BODY_KEY, FRAMEWORK_PANEL_TAB_ARTIFACT_ICON_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HISTORY_ICON_ID, FRAMEWORK_PANEL_TAB_HISTORY_ID, FRAMEWORK_PANEL_TAB_HISTORY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID, FRAMEWORK_PANEL_TAB_PARAMETERS_ID, FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL, RIBBON_PARENT_CATEGORIES,
};
pub use component::ui::*;
pub use component::utilities::{utility_button, utility_collection, utility_separator, utility_toggle, UtilityCategory, UtilityNode};
pub use geometry::Rect;
#[cfg(feature = "wgpu")]
pub use presence_bar::{build_presence_bar, build_presence_bar_localized, presence_color, presence_css_var, PresenceAppearance, PresenceHsl, PresencePeerRow, PresenceRole, PRESENCE_BAR_DEFAULT_MAX};
pub use theme::{GlassStyle, Level, Rgba, Theme};

// 🖥️ Retained-mode engine surface (feature = "wgpu-engine" only).
#[cfg(feature = "wgpu-engine")]
pub use arena::{Arena, NodeId};
#[cfg(all(feature = "wgpu-engine", target_arch = "wasm32"))]
pub use cursor::apply_canvas_cursor;
#[cfg(all(feature = "wgpu-engine", not(target_os = "wasi")))]
pub use cursor::apply_window_cursor;
#[cfg(feature = "wgpu-engine")]
pub use cursor::{resolve_semio_cursor, CursorDragState, SemioCursor};
#[cfg(feature = "wgpu-engine")]
pub use draw::{ear_clip_polygon, mesh_content_version, paint_selection_marquee, DrawList, IconAtlas, MeshGpuTable, RasterTextureAdmission, RasterTextureStageFault, RasterTextureTable, RasterTextureWitness, MESH_GPU_KEEP_VERSION_CAPACITY};
#[cfg(feature = "wgpu-engine")]
pub use tree::{EditState, LayoutBucket, Node, NodeFlags, NodeKey, PaintBucket, UiTree, WidgetSpec, WidgetState};
// 🪟️🫳️🖱️ W2 wiring: `w1d-events-overlay`'s overlay/drag-drop/scroll types, previously reachable only
// via `crate::events::*` (the module itself is `pub`, just not curated into this flattened surface)
// — `EventRouter` itself stays `pub(crate)` (an `engine::Ui` implementation detail; drive it via
// `Ui::dispatch_event`), but the data these `UiCommand`s/the host's own drag-ghost rendering need are
// now part of the crate's curated public API like every other `events` type already was.
#[cfg(feature = "wgpu-engine")]
pub use events::{resolve_overlay_placement, CaptureKind, DismissPolicy, DragGhost, DragPayload, DragSession, EventModifiers, ImeEvent, OpenOverlay, OverlayAnchor, OverlayKind, OverlayPlacement, PointerButton, ScrollAxis, UiCommand, UiEvent};
#[cfg(feature = "wgpu-engine")]
pub use scene_slots::{SceneHost, ScenePaintCursor, ScenePaintStep, SceneSlot, SlotContent};
#[cfg(feature = "wgpu-engine")]
pub use shell::{Shell, ShellEvent};
// 🧵️ W2 wiring: the retained-mode façade itself (`engine::Ui` — `apply_tree`/`frame`/
// `dispatch_event`/`needs_frame`/`drain_commands`) was never re-exported at all before this pass;
// this is the actual public entry point a host drives per tick, per `report-w0-engine-facade.md`'s
// own closing wiring request.
#[cfg(feature = "wgpu-engine")]
pub use action::{
    checked_action_string_bytes, BoundedAction, BoundedActionBatchReservation, BoundedActionBuilder, BoundedActionClaim, BoundedActionClaimBatch, BoundedActionFault, BoundedActionQueue, BoundedActionReservation, BoundedClaimedActionDraft,
    BoundedClaimedActionReservation, PreparedClaimedAction, PreparedClaimedActionBatch, ACTION_ITEM_BYTE_CAPACITY, ACTION_STRING_BYTE_CAPACITY,
};
#[cfg(feature = "wgpu-engine")]
pub use chrome::{chrome_item_bg, chrome_item_text, item_bg, item_text, measure_action_item, push_chrome_border, push_chrome_group_border, push_control_border, push_icon, push_window_cap_border, ICON_TINY};
#[cfg(feature = "wgpu-engine")]
pub use engine::{SurfaceLane, Ui, UiFrameStep, UiLayoutStep};
#[cfg(all(feature = "wgpu-engine", not(target_os = "wasi")))]
pub use gpu::schedule_frame;
#[cfg(feature = "wgpu-engine")]
pub use gpu::GpuContext;
#[cfg(all(feature = "wgpu-engine", target_arch = "wasm32", not(target_os = "wasi")))]
pub use host::{clipboard_read_text, clipboard_write_text, dispatch_window_event, modifiers_from_winit, pointer_coords, WindowInputState};
#[cfg(all(feature = "wgpu-engine", not(target_arch = "wasm32"), not(target_os = "wasi")))]
pub use host::{dispatch_window_event, modifiers_from_winit, pointer_coords, ClipboardIoJob, WindowInputState};
#[cfg(feature = "wgpu-engine")]
pub use input::{DragAxis, DragState, HitKind, HitTarget, InputState, KeyAction, PointerCallbacks, PointerModifiers, TreeDragState, TreeDropPosition};
#[cfg(feature = "wgpu-engine")]
pub use paint::{paint_retained_glyph_step, RetainedGlyphCursor, RetainedGlyphStep, RETAINED_NODE_TEXT_MAX_BYTES};
#[cfg(all(feature = "wgpu-engine", target_arch = "wasm32"))]
pub use prepared::OffscreenPresentToken;
#[cfg(feature = "wgpu-engine")]
pub use prepared::{
    PreparedPresenterWitness, PreparedRasterGeneration, PreparedRasterPages, PreparedRasterProducer, PreparedRasterProducerStep, PreparedRasterRejected, PreparedRasterReservation, PreparedRenderEviction, PreparedRenderGate, PreparedRenderInput,
    PreparedRenderJob, PreparedRenderLimits, PreparedRenderPacket, PreparedRenderReceiver, PreparedRenderRejection, PreparedRenderReplacement, PreparedRenderUpload, PreparedRenderUsage, RenderDirective, UiPresentToken, PREPARED_RASTER_PAGE_BYTES,
};
#[cfg(feature = "wgpu-engine")]
// 🎬️ Relocated out of this crate into `semio-framework-ui-scene`'s `math` module (ticket
// 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME packet `scene-surface`; previously relocated
// verbatim from `🧰️framework/🔨️modules/🧊️3d/🎬️scene/🦀️component.rs` per ticket
// 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave MESH). `kernel_3d_scene`
// stays the name every existing call site (`draw.rs`, `widgets.rs`) already uses — this is now an
// alias onto the scene crate's module rather than a `#[path]` file mount.
#[cfg(feature = "wgpu-engine")]
pub use ui_scene::math as kernel_3d_scene;

#[cfg(feature = "wgpu-engine")]
pub use kernel_3d_scene::{
    aabb_intersects_frustum, axis_rotate_angle, frustum_planes, grid_placement_anchor, gumball_axis_drag_plane_normal, gumball_extent, gumball_eye, gumball_project_ray_onto_axis, interpolate_mesh_uv, lod_from_camera_distance,
    lod_progressive_grid_layers, marquee_is_crossing_from_path, mesh3d_abort, mesh3d_abort_step, mesh3d_allocate_step, mesh3d_begin, mesh3d_begin_close, mesh3d_close_step, mesh3d_read_write_u32, mesh3d_read_write_vec3, mesh3d_seal,
    mesh3d_terminal_is_empty, mesh3d_update_vec3, mesh3d_write_edge, mesh3d_write_u32, mesh3d_write_vec2, mesh3d_write_vec3, mesh3d_write_vec4, pick_closest_mesh_url, point_in_polygon, project_point, quat_from_basis, ray_aabb_slab,
    ray_pick_instance, ray_pick_mesh_detail, ray_plane_point, ray_segment_distance, rect_contains, rotate_vector, screen_segment_distance, screen_select_components, screen_select_instances, transform_aabb, vec3_from_f64, Camera3d, Instance3d,
    LineDraw3d, LineVertex3d, Mat4, Mat4Math, Mesh3dFault, Mesh3dField, Mesh3dItem, Mesh3dItemCursor, Mesh3dLease, Mesh3dPageCursor, Mesh3dSchema, Mesh3dWriteToken, OrbitController, SceneDraw3d, ScenePass3d, TexturedDraw3d, TexturedInstance3d, Vec3,
    Vec3Math,
};
#[cfg(feature = "wgpu-engine")]
pub use layout::{gap_for_token, layout_horizontal, layout_vertical, padding_for_token};
#[cfg(feature = "wgpu-engine")]
pub use text::{fetch_font_bytes, FontAtlas};
#[cfg(feature = "wgpu-engine")]
pub use widgets::{
    draw_icon, draw_text, draw_text_overlay, draw_text_wrapped, measure_widget, render_scroll_region, render_widget, wrap_text, ControlNode, InputMeta, KeyValueEntry, RingMeta, SelectItem, SliderMeta, StepperMeta, TreeItem, TreeItemAction,
    TreeSection, WidgetContext, WidgetInteractionMaps, WidgetNode,
};
// #endregion re-exports
