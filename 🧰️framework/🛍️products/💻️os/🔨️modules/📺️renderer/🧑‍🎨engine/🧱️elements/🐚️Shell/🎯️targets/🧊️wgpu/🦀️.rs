//! 🖥️ framework/products/os/modules/renderer/engine/elements/🐚️Shell/component.rs — wgpu shell
//! chrome implementation for the Shell element, extracted from lib.rs's inline
//! `pub mod shell { ... }` body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired via
//! `#[path = "../../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"] pub mod shell;` in lib.rs in place of
//! the former inline block; the module name `shell` is unchanged, so every existing
//! `crate::shell::...` call site elsewhere in the crate keeps resolving with zero other changes.
//! 🖥️ OS shell chrome — navbar, footer, floating panels, overlays, and studio mode.
//!
//! 🧩 This file keeps the shared imports, the extension-ledger types, [`ShellState`] and its `Drop`; every other
//! concern lives in a sibling chunk module wired below, each a `<emoji>️<kebab>/🦀️.rs` folder that reaches the shared
//! imports through `use super::*;` and is re-exported here, so every `crate::shell::...` path is unchanged.

use semio_framework_os_kernel::os_directory::io::text::directory_command_sha256;
use semio_framework_os_config::io::text::mutations::{decode_ui_preferences_config_mutation_json};
#[path = "../../../🏛️ShellHost/🔀️surface-switch/📄️document/🦀️.rs"]
mod document_surface_preparation;

#[cfg(test)]
use crate::dock::{push_window_silhouette_border, DockStackTab};
#[cfg(all(test, not(target_arch = "wasm32")))]
use semio_framework_os_kernel::os_directory::{client::DirectoryTransport, DirectoryCommandOutcomeV1};
use ui_wgpu::wgpu::push_chrome_group_border;
// 🧾️ Un-gated with the shell-owned panel builders themselves: the Command dock, the five Settings
// leaves, the Marketplace leaf, the tool leaves and the chat transcript are all production retained
// panel bodies now, so the whole `UiNode` vocabulary they author has to compile on every target.
use semio_framework_ui_locale::Label;
use ui_wgpu::wgpu::UiButtonNode;
use ui_wgpu::wgpu::UiControlNode;
use ui_wgpu::wgpu::UiFieldNode;
use ui_wgpu::wgpu::UiInputNode;
use ui_wgpu::wgpu::UiNode;
use ui_wgpu::wgpu::UiNumberStepperNode;
use ui_wgpu::wgpu::UiPresence;
use ui_wgpu::wgpu::UiRingNode;
use ui_wgpu::wgpu::UiSectionNode;
use ui_wgpu::wgpu::UiSelectItem;
use ui_wgpu::wgpu::UiSelectNode;
use ui_wgpu::wgpu::UiSliderNode;
use ui_wgpu::wgpu::UiStackNode;
use ui_wgpu::wgpu::UiTextNode;
use ui_wgpu::wgpu::UiToggleNode;
use ui_wgpu::wgpu::UiTreeItemNode;
use ui_wgpu::wgpu::UiTreeNode;
use ui_wgpu::wgpu::UiTreeSectionNode;
use ui_wgpu::wgpu::UiTreeWindow;
use ui_wgpu::wgpu::UiTreeWindowRowExtent;

use crate::dock::{compute_dock_drop_zone, drop_zone_indicator_rect, parse_path, DockDragKind, DockDragPayload, DockDragState, DockDropZone, DockRenderContext, DockState, WindowSilhouette};
use crate::hub_connection::{
    hub_artifact_creation_intent, hub_artifact_creation_terminal, HubArtifactCatalogPhase, HubArtifactCreation, HubArtifactCreationState, HubArtifactOpening, HubConnectionState, HubDocumentRemote, HubLink, HubSessionPresence, FRAMEWORK_HUB_PANEL_ID,
};
use crate::hub_sign_in::{
    hub_connection_id_for_origin, parse_hub_origin, reduce_hub_session, remove_hub_connection, select_hub_connection, serialize_hub_connection_book, upsert_hub_connection, HubConnection, HubConnectionKind, HubSessionEvent, HubSessionPhase,
    HubSignInClientClass, HubSignInCredential, HubSignInErrorCode, HUB_CONNECTION_BOOK_STORAGE_KEY_V1,
};
use crate::interpreter::{begin_ui_document_opportunity, framework_widget_context, render_ui_document_step, RetainedNodeFocusKind, UiDocumentFrameCursor};
use crate::program_bridge::{resolve_playground_app_id, resolve_plugin_host_config, resolve_registry_plugin_id, PluginHostConfig, ProgramBridgeEntry};
use crate::scenes::{toggle_vfs_row_expanded, vfs_selection_for_click, AdmittedSurfaceMap, BlockListChromeLabels, Board2dSurface, NodeGraphSurface, SceneChromeLabels, TiledMapSurface, VirtualFileSystemChromeLabels, SCENE_SURFACE_CAPACITY};
use infinite_world::world::{
    begin_world3d_dynamic_retirement, enqueue_world3d_events, step_world3d_dynamic_retirement, world3d_catalogue_drop_origin, world3d_clear_catalogue_drop_preview, world3d_dynamic_retirement_terminal_is_empty, world3d_update_catalogue_drop_preview,
    World3dState, WorldInteractionIntent, WorldInteractionPhase,
};
use semio_framework::kernel::{UiDirtyScope, UiDirtySection};
use semio_framework::IconName;
use semio_framework::{AppDefinition, PanelGroup, PanelTabDefinition, ViewModel, ViewSessionIdentity};
use semio_framework_ui_viewport::{
    Viewport3dAxonometricHemisphere, Viewport3dAxonometricQuadrant, Viewport3dAxonometricVariant,
    Viewport3dCurvilinearMapping, Viewport3dObliqueVariant, Viewport3dOrthographicView,
    Viewport3dProjectionMode, Viewport3dProjectionOrientation, Viewport3dProjectionSpec,
};
use semio_framework_os_config::opening_config::{
    apply_ui_preferences_config_mutation, mutations::{set_appearance, set_custom_driver, set_custom_theme, set_driver, set_keybinding_override, set_layout, set_locale, set_named_layout, set_terminology, set_theme, UiPreferencesConfigMutation},
    UiAppearance as OsUiAppearance, UiChromeLayout as OsUiChromeLayout, UiDriver as OsUiDriver, UiLocale as OsUiLocale, UiPreferences, UiTheme as OsUiTheme, UserNamedLayout, UI_PREFERENCES_CONFIG_SCHEMA,
};
use semio_framework_os_kernel::os_directory::identity::IdentityEnv;
use semio_framework_os_kernel::os_directory::schema::space_artifact_creation::{SpaceArtifactCreationCatalogV1, SpaceArtifactCreationPhaseV1};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, VecDeque};
use store_sync::sync::{ArtifactActorConfig, ArtifactActorMsg, ArtifactDocumentKey, ArtifactEvent, ArtifactHost, ArtifactMailboxSender, ArtifactSyncStatus, DocumentLinkStatus, PersistenceBinding, RemoteState};
use store_sync::{PresencePeer, PresenceWindowView};
use ui_contract::UiFixedList;
use ui_contract::{SurfaceId, UiDocumentLease, UiText, UI_DOCUMENT_LEASE_ALIASES, UI_DOCUMENT_LEASE_SLOTS};
use ui_wgpu::wgpu::draw_text;
// 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C0/§C3/§C6 (lane 2-D) —
// lane 1-D's Rust directory client, consumed as-is (never re-declared:
// `semio_framework_os_kernel::os_directory` is the single source of truth both this shell and the
// hub's own Rust twin import).
//
// 🌐️ Target-neutral since ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1e: identity, the
// directory command FIFO and Space Administration are pure state machinery over `DirectoryClient<T>`,
// so the platform split lives at `T` — `crate::directory_door::ShellDirectoryTransport` — and not
// around the features themselves. Only the document-sync backbone (`ArtifactHost`) and the retained
// Home projection's WebSocket stream remain native-only below.
use crate::directory_door::{ShellDirectoryClient, ShellDirectoryTransport};
// 🔐️ Slice WG6: the hub workspace's closed verb vocabulary, aliased so the one dispatch arm and
// the surface builder name the same constants rather than two copies of eighteen string literals.
use crate::hub_connection::action as hub_action;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_kernel::os_directory::{
    client::{CanonicalDirectoryEventPageV1, DirectoryBootstrapTransition, DirectoryEventPageAckV1, DirectoryEventPageBootstrapV1, DirectoryStream, DirectoryStreamTurn, DirectoryWsConnection, TransportError},
    identity::{claimed_local_hub_credential, restore_claimed, IdentityOutcome, IdentityStatus},
    schema::{
        DocumentExecutionTargetLeaseFieldsV1, DocumentScope,
    },
    DirectoryStreamMessage,
};
use semio_framework_os_kernel::os_directory::{
    client::{DirectoryClient, DirectoryClientError, LocalHubCredential},
    identity::{actor_id, Identity},
    mint_directory_command_request_id, DirectoryCommand, DirectoryCommandErrorCodeV1, DirectoryCommandReceiptV1, DirectoryCommandRequestV1, DirectoryCommandResultV1, DirectorySessionAuthorityV1, DirectorySpaceAdministrationCapabilitiesV1,
    DirectorySpaceAdministrationInviteRowV1, DirectorySpaceAdministrationMemberRowV1, DirectorySpaceAdministrationPageV1, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility,
};
// 🌀️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-directory-and-run): `DirectoryClient`'s request
// methods now carry an `OperationContext` (cancellation/deadline/trace), and the native transport
// routes through `semio-framework-os-services`'s `HttpPool`/`ComputePool` over ONE shared
// `TokioHostRuntime` — `NativeDirectoryTransport::new()` (zero-arg) is gone, replaced by
// `with_new_http_pool_now` fed the renderer's process-wide pool. No local executor or thread is
// constructed for identity or directory work.
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_actor::{ActorId as DirectoryActorId, PackageId as DirectoryPackageId};
use semio_framework_async::{CancelToken, OperationContext, TraceId};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_async::{Lane, ScopeOwner, WorkerPool};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_services::{ComputePool, TokioHostRuntime};
use ui_wgpu::wgpu::{
    chrome_item_bg, chrome_item_text, paint_retained_glyph_step_flowed, DragAxis, DrawList, FontAtlas, HitKind, HitTarget, IconAtlas, InputState, Level, PointerModifiers, Rect, RetainedGlyphCursor, RetainedGlyphStep, RetainedTextFlow, Rgba, Theme,
    TreeDragState, TreeDropPosition, WidgetInteractionMaps, WindowLayout, WindowStackCorner,
};
use ui_wgpu::wgpu::ActionDescriptor;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_locale::Terminology;
use ui_wgpu::wgpu::UtilityNode;
use ui_wgpu::wgpu::WindowEngagement;
use ui_wgpu::wgpu::WindowMeasure;
use ui_wgpu::wgpu::FRAMEWORK_PANEL_TAB_ARTIFACT_ID;
use ui_wgpu::wgpu::FRAMEWORK_PANEL_TAB_CATALOGUE_ID;
use ui_wgpu::wgpu::FRAMEWORK_PANEL_TAB_HISTORY_ID;
use ui_wgpu::wgpu::FRAMEWORK_PANEL_TAB_INSPECTION_ID;
use ui_wgpu::wgpu::FRAMEWORK_PANEL_TAB_TOOL_RUN_ID;

use semio_framework_value::{DslValue, FromValue, ToValue};
#[cfg(not(target_arch = "wasm32"))]
use protocol::os_directory::client::{DocumentHttpPortDeclarationV1, InstalledServiceContributionV1, InstalledServiceDriverV1, InstalledServiceTurnV1, InstalledServiceStatusV1, DOCUMENT_HTTP_PORT_TOPIC};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_kernel::os_directory::client::document_http::CompiledDocumentHttpPortV1;
use serde_json::Value;

#[path = "📤️icon-export/🦀️.rs"]
mod icon_export;

#[path = "🪟️tree-windows/🦀️.rs"]
mod tree_windows;

#[path = "⏪️time-travel/🦀️.rs"]
mod time_travel;
use time_travel::TimeTravelVerb;

#[path = "📎️local-folders/🦀️.rs"]
mod local_folders;

#[path = "🔖️identity-and-types/🦀️.rs"]
mod identity_and_types;
pub use identity_and_types::*;

#[path = "🧵️shell-async-runners/🦀️.rs"]
mod shell_async_runners;
pub(crate) use shell_async_runners::*;

#[path = "🏛️space-admin-and-settle/🦀️.rs"]
mod space_admin_and_settle;
pub use space_admin_and_settle::*;

#[path = "⚓️dock-model-and-measures/🦀️.rs"]
mod dock_model_and_measures;
pub use dock_model_and_measures::*;

#[path = "🧾️panel-records/🦀️.rs"]
mod panel_records;
pub(crate) use panel_records::*;

#[path = "🧩️session-and-topology/🦀️.rs"]
mod session_and_topology;

#[path = "⚙️settings-and-dock-persistence/🦀️.rs"]
mod settings_and_dock_persistence;

#[path = "🎬️shell-actions/🦀️.rs"]
mod shell_actions;
pub use shell_actions::*;

#[path = "📨️command-dispatch/🦀️.rs"]
mod command_dispatch;

#[path = "🖱️pointer-input/🦀️.rs"]
mod pointer_input;
pub use pointer_input::*;

#[path = "⌨️keyboard-and-palette/🦀️.rs"]
mod keyboard_and_palette;

#[path = "🧱️chrome-paint-and-navbar/🦀️.rs"]
mod chrome_paint_and_navbar;
pub use chrome_paint_and_navbar::*;

#[path = "🧰️window-actions-and-commands/🦀️.rs"]
mod window_actions_and_commands;
pub(crate) use window_actions_and_commands::*;

#[path = "🗨️chrome-overlays-and-tour/🦀️.rs"]
mod chrome_overlays_and_tour;
pub use chrome_overlays_and_tour::*;

#[path = "🎓️tutorial-runtime/🦀️.rs"]
mod tutorial_runtime;
pub use tutorial_runtime::*;

#[path = "🖼️chrome-frame-and-main-window/🦀️.rs"]
mod chrome_frame_and_main_window;
pub(crate) use chrome_frame_and_main_window::*;

#[path = "🧭️navbar-footer-and-overlays/🦀️.rs"]
mod navbar_footer_and_overlays;

#[path = "💬️chrome-dialog-and-tooltips/🦀️.rs"]
mod chrome_dialog_and_tooltips;

#[path = "🔑️storage-prefs-and-i18n/🦀️.rs"]
mod storage_prefs_and_i18n;
pub use storage_prefs_and_i18n::*;

#[path = "🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod theme_file_import_and_accessibility;
pub use theme_file_import_and_accessibility::*;

#[cfg(test)]
/// 🔎️ The whole Shell module as the one text the unsplit file held: this file and every child split out of it, for the source-text laws. The four modules that were already mounted before the split (`icon_export`, `tree_windows`, `time_travel`, `local_folders`) stay out, as they were out of that text.
pub(crate) const SHELL_SOURCE: &str = concat!(
    include_str!("🦀️.rs"),
    include_str!("🔖️identity-and-types/🦀️.rs"),
    include_str!("🧵️shell-async-runners/🦀️.rs"),
    include_str!("🏛️space-admin-and-settle/🦀️.rs"),
    include_str!("⚓️dock-model-and-measures/🦀️.rs"),
    include_str!("🧾️panel-records/🦀️.rs"),
    include_str!("🧩️session-and-topology/🦀️.rs"),
    include_str!("⚙️settings-and-dock-persistence/🦀️.rs"),
    include_str!("🎬️shell-actions/🦀️.rs"),
    include_str!("📨️command-dispatch/🦀️.rs"),
    include_str!("🖱️pointer-input/🦀️.rs"),
    include_str!("⌨️keyboard-and-palette/🦀️.rs"),
    include_str!("🧱️chrome-paint-and-navbar/🦀️.rs"),
    include_str!("🧰️window-actions-and-commands/🦀️.rs"),
    include_str!("🗨️chrome-overlays-and-tour/🦀️.rs"),
    include_str!("🎓️tutorial-runtime/🦀️.rs"),
    include_str!("🖼️chrome-frame-and-main-window/🦀️.rs"),
    include_str!("🧭️navbar-footer-and-overlays/🦀️.rs"),
    include_str!("💬️chrome-dialog-and-tooltips/🦀️.rs"),
    include_str!("🔑️storage-prefs-and-i18n/🦀️.rs"),
    include_str!("🎨️theme-file-import-and-accessibility/🦀️.rs")
);

//#region ShellLifecycle
#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs"]
mod panel_anchor_model_tests;
//#endregion ShellLifecycle

//#region ShellInput
#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-shell-input/🦀️.rs"]
mod shell_input_tests;

#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-shell-boot-isolation/🦀️.rs"]
mod shell_boot_isolation_tests;
//#endregion ShellInput

//#region ActionPanelAndUtilities
//#region 🪟️WindowPaneChrome
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../🧪️tests/🪟️wgpu-window-pane-chrome/🦀️.rs"]
mod window_pane_chrome_tests;
//#endregion 🪟️WindowPaneChrome

#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-command-registry/🦀️.rs"]
mod command_registry_tests;

//#region 🎬️WindowActionsAndSearchPanes
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../🧪️tests/🎬️wgpu-window-actions-search-panes/🦀️.rs"]
mod window_actions_search_pane_tests;
//#endregion 🎬️WindowActionsAndSearchPanes
//#endregion ActionPanelAndUtilities

//#region ShellChrome
//#region 🌓️AppearanceTourAndFooterPillTests
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../🧪️tests/🌓️appearance-tour-and-footer-pills/🦀️.rs"]
mod appearance_tour_and_footer_pill_tests;
//#endregion 🌓️AppearanceTourAndFooterPillTests
//#endregion ShellChrome

//#region ShellTypes
const SHELL_EXTENSION_LEDGER_CAPACITY: usize = 256;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ShellExtensionStoreRecord {
    pub extension_id: String,
    pub directory_name: String,
    pub version: String,
    pub label: String,
    #[serde(rename = "extends")]
    pub extends_host: String,
    pub module_url: String,
    pub package_hash: String,
    pub installed_at: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShellExtensionLoadStatus {
    Available,
    Loaded,
    Failed(String),
}

#[derive(Clone)]
pub struct ShellExtensionProjection {
    pub record: ShellExtensionStoreRecord,
    pub enabled: bool,
    pub load_status: ShellExtensionLoadStatus,
    program: Option<ProgramBridgeEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct WindowIconOverride {
    plugin_id: String,
    app_id: String,
    app_instance_id: u32,
    icon_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct WindowTitleOverride {
    plugin_id: String,
    app_id: String,
    app_instance_id: u32,
    title: String,
}

/// 🪟️ Separates framework layout application from the effective React Mode prop identity.
struct DockInputIdentity {
    source: Option<WindowLayout>,
    mode_layout: Option<WindowLayout>,
    roster: Vec<String>,
}

/// 🧭️ Projects the structural fields observed by React Mode without presentation-only titles.
fn dock_mode_layout_identity(layout: &WindowLayout) -> WindowLayout {
    use ui_wgpu::wgpu::{WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode};
    fn stack(node: &mut WindowLayoutStackNode) {
        node.active_window_kind_id = None;
        for leaf in &mut node.children {
            leaf.title = None;
            leaf.window_kind_id = leaf.instance_id.take().unwrap_or_else(|| leaf.window_kind_id.clone());
            leaf.template_id = None;
        }
    }
    fn axis(node: &mut WindowLayoutAxisNode) {
        for child in &mut node.children {
            match child {
                WindowLayoutChild::Axis(child) => axis(child),
                WindowLayoutChild::Stack(child) => stack(child),
            }
        }
    }
    let mut projected = layout.clone();
    match &mut projected.root {
        WindowLayoutRoot::Axis(node) => axis(node),
        WindowLayoutRoot::Stack(node) => stack(node),
    }
    projected
}

pub struct ShellState {
    pub plugins: Vec<ProgramBridgeEntry>,
    pub extensions: Vec<ShellExtensionProjection>,
    pub plugin_filter: String,
    pub host_capability: Option<&'static PluginHostConfig>,
    pub space_mode: bool,
    pub session: Option<ActiveSession>,
    pub window_ui: HashMap<String, UiDocumentLease>,
    pub panel_documents: HashMap<String, UiDocumentLease>,
    tool_run_panel_runs: std::collections::BTreeSet<u64>,
    pub spawned_ui: Option<UiDocumentLease>,
    pub spawned_session: Option<ActiveSession>,
    app_document_identities: crate::media_slots::DocumentIdentityRegistry,
    closing_documents: ShellDocumentRetirementRegistry,
    pub active_window_id: Option<String>,
    /// 🪟️ The active window the guest's command history was last told about, so a REAL activation is
    /// noted exactly once — the wgpu twin of React's `handleActiveWindowChange` change guard
    /// (`🏛️ShellHost/🟦️.tsx`'s `if (value) noteShellCommand("shell.windowActivate", …)`), which only
    /// fires on a changed value. Re-seeded, never noted, when the session instance changes, because a
    /// successor's landing window is a mount rather than a user activation.
    noted_active_window: Option<(u32, String)>,
    /// 📨️ The active window the program was last told about for host events, so the pane that loses the activation is
    /// blurred exactly once ([`Self::arm_window_blur_host_event`]).
    host_event_active_window: Option<(u32, String)>,
    /// 🧭️ Every anchor's own open/size/active-path chrome, slotted by `PanelAnchor::index` — the
    /// wgpu twin of React's `panels[anchor]` reducer state.
    pub panel_anchors: [PanelAnchorState; 8],
    /// 🧭️ The live per-anchor tab trees, `apply_dock_skeleton(default_dock, dock_override)`.
    pub dock_tabs: ShellDock,
    /// 🗄️ The persisted arrangement override, `None` while the dock equals its computed default.
    pub dock_override: Option<DockSkeleton>,
    /// 🌳️ Branch ids folded shut in an anchor's tab bar, keyed `{anchor}/{branchId}`.
    pub dock_collapsed_branches: HashMap<String, bool>,
    /// 🎯️ The tab id a pointer is dragging between anchors, plus the anchor it started in.
    pub dock_tab_drag: Option<(String, PanelAnchor)>,
    /// 🗄️ The app id whose persisted dock layer is currently loaded — the change detector that reloads
    /// the override on an app switch without clobbering a reveal on every other chrome sync.
    dock_loaded_app_id: Option<String>,
    pub scroll_offsets: HashMap<String, f32>,
    pub overlay_state: OverlayState,
    pub collapsed_sections: HashMap<String, bool>,
    pub tree_open_states: BTreeMap<String, bool>,
    accessibility_focused_control_id: Option<String>,
    pub interaction_selection: HashMap<String, semio_framework::DomainSelection>,
    pub open_selects: HashMap<String, bool>,
    pub context_menu: Option<ContextMenuState>,
    pub search_open: bool,
    pub find_open: bool,
    pub appearance_id: String,
    pub locale_id: String,
    pub terminology_id: String,
    pub right_click: RightClickState,
    pub uri_history: Vec<String>,
    pub uri_index: usize,
    pub open_space_id: Option<String>,
    pub pending_shell_uri_apply: bool,
    pub panel_resize_origin_width: f32,
    pub error: Option<String>,
    /// 🧯 One entry per plugin whose boot activation faulted — the per-plugin status that replaces
    /// the old "any `create_app` failure aborts the whole shell boot" rule. A trapped actor belongs
    /// to exactly one plugin: every OTHER plugin, and the requested app itself, must still boot.
    pub plugin_faults: Vec<ShellPluginFault>,
    /// 🧯 One entry per surface whose document could not be rendered this refresh. The shell keeps
    /// painting every OTHER surface and shows this one as a typed fault card, instead of failing the
    /// boot — see [`ShellSurfaceFault`].
    pub surface_faults: Vec<ShellSurfaceFault>,
    /// 📐️ The content root, in LOGICAL (CSS) pixels — the SAME unit as every chrome constant
    /// (`chrome_px`, `navbar_height`, `footer_height`, `control_height`, the resize-handle hit
    /// widths, every font size), which are read verbatim from the generated `ui_styling` tokens
    /// React's DOM consumes. The surface scale factor never reaches this pair: it sizes the GPU
    /// surface, the projection divisor and the glyph/icon atlas raster, and nothing else. Sizing this
    /// in physical pixels while the constants stayed logical was the HiDPI placement bug — ticket
    /// 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1g (`📓️w1g-dpi-logical-units.md`).
    pub screen_w: f32,
    pub screen_h: f32,
    pub world3d_states: AdmittedSurfaceMap<World3dState>,
    pub world3d_window_ids: HashMap<String, String>,
    /// 🪦 Closed World3d owners, detached from input immediately and drained one bounded step per
    /// frame before Drop. Keys include a shell epoch so reopening the same window id is independent.
    pub retired_world3d_states: VecDeque<(String, World3dState)>,
    /// 🧯️ Closed World3d owners whose `cancel(CaptureLost)` waits for one drive of their world authority before they retire,
    /// with the sightings so far ([`ShellState::world3d_close_cancel_settled`]).
    world3d_close_cancels: BTreeMap<String, u8>,
    component_world3d_retirement: Option<crate::scenes::AdmittedSurfaceCloseOwner<World3dState>>,
    world3d_retirement_epoch: u64,
    world3d_retirement_sequence: u64,
    world3d_retirement_fault: Option<infinite_world::world::WorldDynamicFault>,
    /// 🛑️ Per-World3d-surface compute status, mirrored out of the scene by the same per-frame attach
    /// walk that mirrors bounds. The CANCEL CONTRACT (`cancellable` + `cancelAction`) is read from
    /// here and from nowhere else, so the shell offers a stop affordance without ever learning a
    /// domain verb from code — see `world3d_cancel_affordance`.
    pub world3d_status: HashMap<String, String>,
    /// ⏳️ The last compute-status pill label announced per World3d surface, so the trace prints once
    /// per CHANGE rather than once per frame — the pill itself is recomputed from `world3d_status`
    /// every grant and holds no state of its own.
    pub world3d_status_pill_trace: HashMap<String, String>,
    pub node_graph_states: AdmittedSurfaceMap<NodeGraphSurface>,
    /// 🛍️ The active app instance's APP-STATIC operator/palette catalogue, fetched once per instance
    /// from the reserved `framework.section.catalogue` retained surface and pushed into every live
    /// node-graph engine host. Mirrors the React shell's `appCatalogue`
    /// (`🐚️Shell/🟦️.tsx`) and its `uiRefreshWantsCatalogue` "full scope only" rule: the payload is
    /// app-static, so an instance fetches it exactly once and never on a scene change.
    /// Never carried on a `NodeGraphScene` — see `ArtifactApp::app_catalogue_json`.
    pub app_catalogue_json: String,
    /// 🛍️ Which app instance [`Self::app_catalogue_json`] was fetched for — the cache key that makes
    /// the fetch once-per-instance rather than once-per-refresh.
    pub app_catalogue_instance: Option<u32>,
    pub tiled_map_states: AdmittedSurfaceMap<TiledMapSurface>,
    pub icon_render_states: HashMap<String, World3dState>,
    pub board2d_states: AdmittedSurfaceMap<Board2dSurface>,
    pub dock: DockState,
    pub search_query: String,
    pub search_selected: usize,
    pub find_query: String,
    pub split_resize_path: Option<Vec<usize>>,
    pub split_resize_index: usize,
    pub split_resize_axis_total: f32,
    pub active_example_id: Option<String>,
    pub find_items: ShellFindItems,
    pub find_selected: usize,
    pub engagement_expanded: HashMap<String, bool>,
    pub engagement_activated: HashMap<String, bool>,
    pub measures_folded: HashMap<String, bool>,
    pub measures_expanded: HashMap<String, bool>,
    pub measures_width: HashMap<String, f32>,
    pub measures_resize_origin_width: f32,
    pub engagement_inputs: HashMap<String, String>,
    pub driver_id: String,
    /// 🚗️ Session-local unsaved driver document. Its seven axes immediately replace the live
    /// resolved chrome while the owned preference record remains untouched until Save.
    pub driver_draft: Option<OsUiDriver>,
    /// 🚗️ React's `driverSaveLabel`, cleared after a successful save.
    pub driver_save_label: String,
    pub tree_drag: Option<TreeDragState>,
    pub tree_hovered_id: Option<String>,
    pub widget_maps: WidgetInteractionMaps<ActionDescriptor>,
    widget_maps_staging: WidgetInteractionMaps<ActionDescriptor>,
    pub pending_tree_drag: Option<(String, HashMap<String, String>)>,
    pub tree_drag_origin: (f32, f32),
    pub dock_drag: Option<DockDragState>,
    pub pending_dock_drag: Option<(DockDragPayload, (f32, f32))>,
    /// 🪟️ The tab whose label is held down, selected on release — React's `onSelectTab` fires from
    /// the label BUTTON's click, while a drag can only start from the tab's own grip chip.
    /// 🖱️ The exact chrome control armed by a primary press and eligible to activate only when the
    /// matching release lands on that same control. Shared by dock tab labels and palette rows.
    pub pending_chrome_release: Option<String>,
    chrome_pointer_owner: Option<ui_render::PointerId>,
    /// 🖼️ The render-only dock tree this frame paints, hit-tests and solves drop zones against —
    /// `DockState::render_view` of the committed [`Self::dock`]: the mobile flat stack below React's
    /// breakpoint, the docked-out tree while a drag floats, and the committed tree otherwise. Written
    /// once per frame by `plan_dock_windows`, so the plan, the chrome and the pointer never disagree.
    pub dock_view: DockState,
    /// 🪪️ App instance whose layout was last reconciled into `dock`; commands are refused while a
    /// successor session is mounted but its dock has not crossed `sync_dock` yet.
    dock_instance_owner: Option<(String, u32, String)>,
    /// 🪟️ Effective incoming layout and ordered window IDs last accepted by the dock.
    dock_input_identity: Option<DockInputIdentity>,
    /// 🪟️ Host-owned tree windows of every body this shell paints — what crosses as `ViewModel.tree_windows`.
    tree_windows: tree_windows::TreeWindowScheduler,
    /// 🧠️ Per body, what the observer learned from the guest's answers (`TreeWindowServedMemoryV1`).
    tree_window_served: HashMap<String, Vec<(String, ui_wgpu::wgpu::tree_window::TreeWindowServedMemory)>>,
    pub dock_canvas_bounds: Rect,
    /// 📐️ Separator metric captured with this frame's dock plan so later pointer resize math uses
    /// the same themed physical geometry that paint, drop targets and body publication used.
    pub dock_axis_separator: f32,
    pub dock_drop_tab_bars: Vec<(Vec<usize>, WindowStackCorner, Rect, Vec<f32>)>,
    pub dock_drop_bodies: Vec<(Vec<usize>, Rect, String)>,
    /// 🪟️ The windows this frame's chrome walk paints, in dock order, each with the content rect the
    /// dock solved for it. Written once per frame by `render_main_window_step`'s plan phase and read
    /// by its body loop — the wgpu twin of React's `shellLayout` pane walk. Before this existed the
    /// walk painted `active_window_id` alone over the whole body, so an app whose authored mode
    /// layout places two windows (generation3d's edit mode = Flow + Preview, `[68, 32]`) only ever
    /// laid out one of them and the other's engine surface was never created
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub dock_window_plan: Vec<(String, Rect)>,
    /// 🐛️ Last `[TRACE] ` dock-plan line, so the trace prints on CHANGE rather than once per frame.
    dock_plan_trace: Option<String>,
    pub layout_override: Option<ui_wgpu::wgpu::WindowLayout>,
    pub split_resize_origin: Vec<f32>,
    pub split_resize_secondary_path: Option<Vec<usize>>,
    pub split_resize_secondary_index: usize,
    pub split_resize_secondary_axis_total: f32,
    pub split_resize_secondary_origin: Vec<f32>,
    pub measures_resize_window_id: Option<String>,
    pub deferred_actions: Vec<ActionDescriptor>,
    /// 🪟️ Physical window transfers journal through the ordinary guest action lane, but only after
    /// the Full refresh publishes the required retained body. Direct Display opens intentionally
    /// have no React journal. The shared queue gives every attempted note one admission or refusal.
    window_topology_actions: ui_wgpu::wgpu::BoundedActionQueue,
    /// 🪟️ Required initial-body identities, retry counts and journal outcomes bounded to the action
    /// queue's item capacity so a refusing guest cannot grow host ownership without limit.
    window_topology_publications: VecDeque<WindowTopologyPublication>,
    /// 🧾️ Stable logical identities parallel to the fixed-credit action queue. A later body may
    /// publish before an earlier refusing peer without stealing that peer's action.
    window_topology_action_tokens: VecDeque<u64>,
    next_window_topology_action_token: u64,
    window_topology_refresh_owed: bool,
    /// 📯️ A successful cohort released at least one body-gated journal. This alone permits
    /// deferred dispatch ahead of the next retry; unrelated deferred guest work never delays the
    /// first topology refresh.
    window_topology_journal_dispatch_owed: bool,
    /// 🔁️ Guards `flush_deferred_actions`'s convergence loop against re-entry: a nested flush (one a
    /// dispatched action itself asks for) stays a plain drain so the outer loop keeps the single
    /// authority over how many times the window bodies are re-entered.
    settling: bool,
    /// 🐢️ The union of every [`UiDirtyScope`] declared since the last refresh pass drained it — this
    /// shell's half of React's `createUiRefreshCoalescerV1` owed slot. Producers (a dispatched
    /// action's `InvocationResult::ui_scope`, a command's, an extension answer's, plus whatever the
    /// HOST dirtied by applying that pass's own effects) only DECLARE dirt here; `refresh_ui` is the
    /// one place that takes it. Starts [`UiDirtyScope::None`], so a refresh nobody owes anything to
    /// re-renders nothing at all.
    owed_refresh_scope: UiDirtyScope,
    /// 🫀️ The runtime-owned settle lane's shell half — see [`ShellSettlePump`]. Producers only ARM it
    /// ([`Self::owe_settle`]); the frame loop is the one caller of [`Self::settle_pump_step`].
    settle_pump: ShellSettlePump,
    /// 📥️ Extension invocations the guest asked for, parked until a flush that owns `&mut self`
    /// can run them AND fold their answers' own effects back in — see `flush_deferred_actions`.
    #[cfg(target_arch = "wasm32")]
    pending_extension_invocations: Vec<PendingExtensionInvocation>,
    /// 📤️ File-open requests the guest asked for, parked on the SAME flush for the same reason.
    #[cfg(target_arch = "wasm32")]
    pending_file_opens: Vec<PendingFileOpen>,
    #[cfg(not(target_arch = "wasm32"))]
    shell_io_pending: std::collections::VecDeque<PendingShellIo>,
    /// 📥️ Picked-file imports moving into the session's program chunk by chunk, oldest first ([`ImportTransfer`]).
    pub(crate) import_transfers: VecDeque<ImportTransfer>,
    import_transfer_next: u64,
    pub fullscreen_toggle_requested: bool,
    pub fullscreen_active: bool,
    pub active_utilities: Vec<UtilityNode>,
    /// 🧰️ Host-owned active utility per window kind (never a document field, never a VCS operation).
    /// Replaces the deleted `active_utility_id`/`find_active_utility_id` "first pressed toggle" heuristic.
    pub active_utility_by_window: HashMap<String, String>,
    /// 📇️ Per-window Actions-rail fold state (absent = folded, the default).
    pub action_panel_folded: HashMap<String, bool>,
    /// 🔎️ Per-window Search-pane fold state (absent = folded, the default).
    pub search_panel_folded: HashMap<String, bool>,
    /// 🧰️ Per-window Utilities-rail fold state — React's `Window` `utilityBarFolded`
    /// (`useState(true)`, `🪟️Window/🟦️.tsx:176`), so absent = folded.
    pub utility_bar_folded: HashMap<String, bool>,
    /// 🔀️ Per-window Projection-pane fold state — React's `WorldOrbitProjectionSwitchPane`
    /// `useState(true)` (`🌐️World3dHost/🟦️.tsx:5190`), so absent = folded.
    pub projection_pane_folded: HashMap<String, bool>,
    /// 🔀️ Per-window selected projection template id — the wgpu twin of React's
    /// `externalPendingProjectionSpec`, VIEW STATE ONLY and never dispatched, exactly as
    /// `handleProjectionKindChange` records (`🌐️World3dHost/🟦️.tsx:6574`). Absent =
    /// [`WORLD_PROJECTION_DEFAULT_TEMPLATE_ID`], the template React's own
    /// `worldProjectionDefaults("threePoint")` fallback resolves to for a perspective orbit.
    pub world_projection_template: HashMap<String, String>,
    /// 🖼️ Renderer-local `SetWindowIconContext` state, owned by the exact mounted app instance and
    /// bounded by the same 64-window view-context contract as the dock roster.
    window_icon_overrides: HashMap<String, WindowIconOverride>,
    window_title_overrides: HashMap<String, WindowTitleOverride>,
    /// 📇️ Per-window expanded action id (the accordion-open staged arg form).
    pub action_panel_expanded: HashMap<String, String>,
    /// 📝️ Staged action argument values keyed `"{window_id}:{action_id}"` — edits buffer here
    /// and never dispatch until Execute (Architecture Decision 8, P2).
    pub staged_action_args: HashMap<String, serde_json::Map<String, Value>>,
    /// 🎛️ The command whose staged form the Commands panel should show expanded — the twin of
    /// React's `SET_COMMAND_EXPANDED`, set when an arg-carrying command is picked from the palette
    /// (`command-form:` in `activate_search_item`). Keyed by `command_address_stable_key`.
    pub expanded_command_id: Option<String>,
    /// 📝️ Staged command argument values keyed by `command_address_stable_key` — the twin of
    /// React's `STAGE_COMMAND_ARG`/`RESET_COMMAND_ARGS` buffer, which Execute reads and nothing else
    /// dispatches from.
    pub staged_command_args: HashMap<String, serde_json::Map<String, Value>>,
    /// ⌨️ The shortcut row whose next key event is a CHORD WRITE rather than a shell verb —
    /// React's `keybindingCaptureControlId` (`🏛️ShellHost/🟦️.tsx`).
    pub keybinding_capture_control_id: Option<String>,
    /// 👁️✏️ Pinned default app per `"<dialectCoordinate>/<role>"` — the projection React reads
    /// off `OpeningPreferences`, each value `"<pluginId> <appId>"` or `"none"`.
    pub default_app_pins: HashMap<String, String>,
    /// 🎨️ The unsaved theme document this session is editing — React's `uiThemeDraft`
    /// (`SET_UI_THEME_DRAFT`). `Some` is exactly React's `themeDirty`.
    pub theme_draft: Option<ThemeDocument>,
    /// 🎨️ The theme `Save` box — React's `themeSaveLabel`, session-local exactly as its
    /// `useState` is.
    pub theme_save_label: String,
    /// ⚖️ This session's merge policy — React's `shellState.merge.mergePolicy`, the value the
    /// `framework.settings.mergePolicy` selector writes. `"LaissezFaire" | "Normal" | "Vigilant"`,
    /// React's own option values.
    pub merge_policy: String,
    /// 📤️ Whether a theme IMPORT picker is parked for the gesture lane — a picker is gated on
    /// user activation, so it rides the same narrow door `pending_file_opens` does.
    pub pending_theme_import: bool,
    /// 🖥️ The `framework.display.layout` leaf's save-name box — React's `layoutSaveLabel`
    /// (`📌️ChromePanels/🟦️.tsx`'s `DisplayHostApi`). Session-local, exactly as React's `useState` is.
    pub layout_save_label: String,
    /// 🖥️ Layouts the user saved through `framework.display.save` — React's `displayUserLayouts`, the
    /// `origin: "user"` half of the roster, projected from the `os.config.ui-preferences` log and written back as
    /// `setNamedLayout` events ([`ShellState::load_persisted_named_layouts`]/[`ShellState::persist_named_layouts`]).
    /// The app's own `named_layouts` supply the `origin: "builtin"` half.
    pub user_layouts: Vec<ui_wgpu::wgpu::NamedLayout>,
    /// ⚔️ This session's open first-class conflicts — React's `selectOpenConflicts(shellState)`
    /// over `🐚️Shell`'s `merge.conflicts`. Seeded once per session from
    /// `ProgramBridgeEntry::read_conflicts` and replaced by every `resolve_conflict` reply, which is
    /// React's own `AppCommand::ReadConflicts` seed plus reply-carried roster.
    pub open_conflicts: Vec<ShellConflictRow>,
    /// ⚔️ The conflict row whose diff preview is expanded — React's `selectedConflictId`.
    pub selected_conflict_id: Option<String>,
    /// ⚔️ Whether this session already asked the guest for its conflict roster, so the seed
    /// costs one exchange per session rather than one per frame.
    pub conflicts_seeded: bool,
    /// 🛠️ Live tool measures keyed by TOOL id — React's `toolMeasuresByToolId` ref, read off
    /// [`ProgramBridgeEntry::tool_measures_section`] on the same refresh pass window measures ride.
    pub tool_measures: HashMap<String, Vec<WindowMeasure>>,
    /// 💬️ The `framework.chat` composer's unsent text. The transcript itself is NOT held here:
    /// it is [`crate::agent_bridge::AgentBridgeState::conversation`], the live bridge conversation
    /// (`GatewayToShell::AgentToolCall`/`AgentToolResult`/`ApprovalRequested` in,
    /// `ShellToGateway::AgentMessage` out), exactly as React's `useAgentBridge().conversation` is.
    pub agent_chat_draft: String,
    /// 📱️ The single mobile panel's own chrome — React's `mobilePanelVisible`/`mobilePanelPath`,
    /// the flattened counterpart of the eight `PanelAnchorState`s.
    pub mobile_panel_visible: bool,
    pub mobile_panel_path: Vec<String>,
    pub sync_backbone_uri: Option<String>,
    pub sync_card_kind: Option<String>,
    pub sync_card_draft: String,
    pub sync_card_anchor: Option<(f32, f32)>,
    pub last_envelope_dsl: Option<String>,
    /// 🏛️ Shell-lifetime document-host actor registry, the kernel's `ArtifactHost` on both
    /// targets ([`SHELL_DOCUMENT_TRANSPORTS`] names which transports its actor serves).
    pub document_host: ArtifactHost,
    /// 🧵️ The currently attached document's live actor channel.
    pub sync_channel: Option<ShellSyncChannel>,
    next_sync_binding_generation: u64,
    sync_terminal_fault: Option<String>,
    /// 🚦️ Latest sync health for the active document's status badge.
    pub sync_status: Option<ArtifactSyncStatus>,
    /// 🔌️ The hub link of the document this shell last attached ([`store_sync::sync::DocumentLink`]'s status):
    /// the Sync card speaks it while it is short and after it turned terminal, until the next open.
    pub sync_link: DocumentLinkStatus,
    /// 📶️ Target-neutral remote state for every document still attached to this shell. Native actor
    /// status events and browser-host publications enter through the same bounded projection.
    pub hub_documents: BTreeMap<String, ShellHubRemoteV1>,
    pub sync_bootstrap_progress: Option<(u64, u64, u32, u32)>,
    /// 🛟️ The canonical-pair reseed a rebootstrapping hub document is waiting for ([`ShellState::advance_sync_reseed`]).
    sync_reseed: Option<ShellSyncReseed>,
    /// 🪪️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C3 — the restored-or-
    /// minted session, `None` with no hub env (unchanged local-only behaviour) or before boot's
    /// bootstrap thread reports back. Native only, see this region's header note above.
    pub identity: Option<Identity>,
    /// 🔐️ Exact public `/auth/sessions/me` authority that verified `identity`. This is cleared with
    /// the client and never inferred from a cached identity or a mutable UI boolean.
    pub verified_session_authority: Option<DirectorySessionAuthorityV1>,
    /// 📶️ Set when `mint_or_restore` degraded to the last cached identity because the hub was
    /// unreachable — never blocks, never clears `identity` itself.
    pub identity_offline: bool,
    /// 🌱️ `S_HUB_URL`/`S_USER`/`S_DATA_DIR`, resolved once at `boot()` — `None` means "no hub env",
    /// the sentinel every identity/binding/directory code path below checks first.
    pub identity_env: Option<IdentityEnv>,
    /// 🎭️ Per-process session id — the `{sessionId}` half of contract §C0's `user:{userId}#{sessionId}`
    /// actor grammar, minted once in `ShellState::new` and stable for this process's lifetime.
    pub shell_session_id: String,
    /// ⏳️ Browser identity retry floor — see [`BROWSER_IDENTITY_RETRY_MS`].
    #[cfg(target_arch = "wasm32")]
    identity_retry_at_ms: u64,
    /// 📡️ Non-blocking identity result channel, completed by a retained-waker pool future.
    #[cfg(not(target_arch = "wasm32"))]
    pub identity_bootstrap_rx: Option<std::sync::mpsc::Receiver<Result<IdentityOutcome, String>>>,
    /// 🛑️ Cancellable handle for the finite-turn identity bootstrap future.
    #[cfg(not(target_arch = "wasm32"))]
    identity_bootstrap_task: Option<std::sync::Arc<ShellPoolFuture>>,
    /// 📇️ The hub directory client used to issue `os.directory.*` commands (§C6) — constructed once
    /// identity resolves, holding the session token.
    pub directory_client: Option<std::sync::Arc<ShellDirectoryClient>>,
    /// 🌉️ The MCP agent bridge's TRANSPORT (packet W15e). Holds the socket and the dial ladder; the
    /// consumer it feeds is `chrome_build.agent`, which stays transport-free. Disarmed — and therefore
    /// inert — until a supervisor hands over a config, exactly as React's own `discoverAgentBridgeConfig`
    /// answering `null` leaves its hook `disabled`.
    pub agent_bridge: crate::agent_bridge_door::AgentBridgeTransport,
    /// 🛰️ The scope this shell asks the agent-bridge supervisor with ([`AgentBridgeOfferScopeOwnerV1`]).
    pub agent_bridge_offer_scope: AgentBridgeOfferScopeOwnerV1,
    /// 💡️ The one retained host-owned inference port, live only while its document's execution
    /// target is verified. It is never a document command and nothing it holds is persisted.
    #[cfg(not(target_arch = "wasm32"))]
    inference_port: Option<std::sync::Arc<ShellInferenceRunner>>,
    #[cfg(not(target_arch = "wasm32"))]
    document_services: Vec<InstalledServiceContributionV1>,
    /// 💡️ The last status the retained port published, rendered by the shell's own chrome.
    #[cfg(not(target_arch = "wasm32"))]
    pub inference_port_status: Option<InstalledServiceStatusV1>,
    /// 🎬️ The ONE lazy plugin install this shell runs at a time, retained for exactly as long as it
    /// takes — the wgpu twin of React's `pluginOpInFlightRef` + `SET_PLUGIN_STATUS` pair. Its phase is
    /// what chrome reads; its `cancel` is what [`Self::cancel_plugin_install`] fires. Never persisted,
    /// never a document command.
    pub plugin_install: Option<ShellPluginInstall>,
    /// 🚪️ The ONE document open in flight or settled, advanced by the frame pump
    /// ([`Self::advance_document_opening`]) and painted as its own band with a cancel control.
    pub document_opening: Option<ShellDocumentOpening>,
    icon_export: Option<icon_export::IconExportBatch>,
    /// 🪪️ The verified execution-target lease fields for the open document. Native document opening
    /// retains only a canonical surface-id preference today — `document_socket_surface_from_descriptor`
    /// was deliberately downgraded from a forgeable partial authority by the execution-target-lease
    /// lane — so nothing native fills this in yet and every port refuses with a localized terminal. Both builds retain
    /// the lease a hub document's open admitted and drop it on detach; only the native inference port reads it.
    pub document_execution_target_lease: Option<semio_framework_os_kernel::os_directory::DocumentExecutionTargetLeaseFieldsV1>,
    /// 🏠️ Shell-lifetime Home projection and its fetch → terminal Config receipt → ACK → live owner.
    #[cfg(not(target_arch = "wasm32"))]
    directory_home: Option<DirectoryHomeProjection>,
    /// 🔌️ ONE `NativeDirectoryTransport` (cheap to `.clone()` — see its own doc), shared by every
    /// `DirectoryClient` this shell constructs so they all draw on the SAME `HttpPool` byte budget/
    /// outstanding-request accounting rather than each minting a disjoint pool.
    pub directory_transport: ShellDirectoryTransport,
    /// 🛑️ Shell-lifetime cancellation root shared by identity, requests, and the directory runner.
    pub directory_cancel: CancelToken,
    /// 🎮️ Bounded FIFO of sealed, idempotency-correlated `os.directory.*` operations plus their
    /// transient result slot — flushed opportunistically every frame once `directory_client` exists
    /// (contract §C6: "commands queue in the shell (bounded, in-memory) and flush on reconnect").
    /// Each entry keeps its exact request bytes, so a retry is the SAME command to the hub's
    /// digest-keyed idempotency store and can never mint a second invitation.
    pub directory_commands: ShellDirectoryCommandQueueV1,
    /// 🔐️ The hub workspace state behind the `/hub` overlay — hub book, session phase, space rows,
    /// roster and drafts. Target-neutral: the surface is the same retained tree on both arms, and
    /// only the transport underneath it is split.
    pub hub_workspace: crate::hub_connection::HubWorkspaceState,
    /// 🔐️ The sign-in whose network legs are in flight off the interaction state ([`ShellHubTask`]).
    hub_sign_in_task: Option<ShellHubTask<ShellHubSignInAnswer>>,
    /// 🏘️ The spaces read in flight, on the same kind of task.
    hub_spaces_task: Option<ShellHubTask<ShellHubSpacesAnswer>>,
    pub hub_workspace_open: bool,
    /// 🏛️ The one retained native space-administration operation (fixed capacity: exactly one).
    /// Its page, receipt, and one-shot invite capability are erased by every terminal transition, so
    /// an identity change, a 401/403, or a scoped 4401 leaves nothing administrable behind.
    pub space_administration: Option<ShellSpaceAdministrationOperationV1>,
    /// 🔢️ Monotonic operation epoch; a late turn for a replaced operation is dropped, never applied.
    pub space_administration_epoch: u64,
    /// 👥️ ticket §5 — shell-LOCAL presence roster from the currently attached document's
    /// `ArtifactEvent::Presence`, deliberately NOT folded into the shared kernel `ViewModel`.
    pub presence_peers: Vec<PresencePeer>,
    /// 👥️ The canonical surface id (`surface_app_id`) the CURRENTLY attached document's hub binding
    /// used, if any — `presence_peers` is scoped to this surface; a document opened without a hub
    /// binding (local-only) carries `None` and renders no roster.
    pub presence_surface: Option<String>,
    /// 🪪️ This shell's own hub-admitted presence identity on the attached document — the actor and
    /// palette index the hub's `Session` frame named. Board overlays never paint it, and the local
    /// colour is the hub's, never a guess (React's `publishLocalPresenceActorV1`).
    pub presence_self: Option<(String, u8)>,
    /// 🖱️ The last pointer position the shell saw, in logical screen space — what a board window's
    /// presence view turns into the world point under the pointer on the next heartbeat.
    pub presence_pointer: Option<(f32, f32)>,
    /// 🧾️ ticket §C5 — this session's live history projection (`history_cursor`/`history_entries`
    /// mirror the React shell's `historyProjection` reducer), fed by `ReadHistory`
    /// (session/document mount, `replace=true`) and `AppFrame::Invocation.history_patch` (every
    /// dispatch response, `replace=false`) — see `fold_history_patch`/`observe_invocation_history`.
    pub history_cursor: u64,
    pub history_entries: BTreeMap<String, semio_framework::kernel::HistoryEntry>,
    /// 🧾️ The most recent checkpoint id `HistoryPatch.currentCheckpointId` reported — compared against
    /// its previous value to detect "a checkpoint landed" (§C5 item 6, `TouchArtifact`).
    pub history_current_checkpoint_id: Option<String>,
    /// ⏪️ The live history-edit session `HistoryPatch.timeTravel` last carried (`None` = no session) — what the
    /// time-travel band, the pane chips and the `ui.timeTravel.*` chords read (see `⏪️time-travel`).
    history_time_travel: Option<semio_framework::kernel::HistoryTimeTravel>,
    /// 📡️ The history change `HistoryPatch.reprojection` last said replays before adoption (`None` = none waits) — what the
    /// reprojection band and its live status read (see `⏪️time-travel`, audit W1E-3).
    history_reprojection: Option<semio_framework::kernel::HistoryReprojection>,
    /// 🎯️ Where keyboard focus goes once the session edge that named it reaches the screen, and until when its target
    /// may take before the band fallback (`⏪️time-travel` `TimeTravelFocus`).
    time_travel_focus: Option<(time_travel::TimeTravelFocus, f64)>,
    /// ⏱️ When the native shell last re-read the history for replay progress (`TIME_TRAVEL_POLL_MS` apart).
    #[cfg(not(target_arch = "wasm32"))]
    time_travel_polled_at_ms: f64,
    /// 📁️ This device's remembered folder bindings (`os.config.local-folders`), loaded once and kept current by every
    /// commit (see `📎️local-folders`).
    local_folder_bindings: Option<semio_framework_os_config::opening_config::mutations::LocalFolderBindings>,
    /// 🔁️ The document whose remembered folder the direct reattach already tried, so a failure is offered, not retried.
    local_folder_reattach_tried: Option<String>,
    /// 🚪️ Whether this build reopens a remembered folder by itself (native) or offers it for the person's gesture
    /// (browser) — `📎️local-folders`'s `LOCAL_FOLDER_DIRECT_REATTACH`.
    local_folder_direct_reattach: bool,
    /// 🌐️ The folder- or file-bound document the browser build keeps through the dev host's backbone route.
    #[cfg(any(target_arch = "wasm32", test))]
    host_route_folder: Option<local_folders::HostRouteFolder>,
    /// 📌️ Ms-since-epoch of the last uncommitted-count CHANGE (any direction — mirrors
    /// `AutoCheckinScheduler::notify` resetting its idle timer on every call, not just increases);
    /// `None` whenever nothing is uncommitted.
    pub last_uncommitted_edit_at_ms: Option<i64>,
    /// 📌️ The auto check-in poll's own `pending` latch (`AutoCheckinScheduler`'s twin): set the
    /// instant an auto checkpoint is dispatched, cleared the moment `uncommitted_edit_count` returns
    /// to 0 (a landed checkpoint) — the storm guard `auto_checkin_should_fire` reads.
    pub auto_checkin_pending: bool,
    /// 📌️ §C5 item 6 — set right before ANY checkpoint (auto/explicit/close) THIS shell itself asked
    /// for; cleared once the resulting `history_current_checkpoint_id` change is observed and
    /// `TouchArtifact` fires — distinguishes "a checkpoint we asked for landed" from "the session just
    /// mounted with a pre-existing checkpoint" (mirrors React's `checkpointDispatchedRef`).
    pub checkpoint_dispatched: bool,
    /// 📌️ ticket §C5 item 3 — explicit check-in's own message-prompt dialog: `None` when closed,
    /// `Some(draft)` while open. This hand-painted footer chrome has no generic text-input widget (the
    /// `🟦️Interpreter` pipeline's `UiEvent::TextInput` is deliberately bypassed here, same as
    /// `render_presence_bar`'s own design note) — mirrors the pre-existing `sync_card_kind`/
    /// `sync_card_draft` shell-owned keyboard-routed draft-field idiom instead of inventing a new one.
    pub checkin_dialog_draft: Option<String>,
    /// 📌️ The one hub Check In this shell drives for its mounted hub document: requested the moment
    /// a checkpoint this shell asked for lands, submitted once the document's sync status names an
    /// acknowledged head, then polled to a terminal status (the React twin is the worker's
    /// `driveDocumentCheckIn`). Its status is the footer's hub badge suffix.
    #[cfg(not(target_arch = "wasm32"))]
    pub hub_check_in: Option<ShellHubCheckInV1>,
    pub window_engagements: HashMap<String, WindowEngagement>,
    /// 🎬️ Each live window instance's projected Actions pane document — see
    /// `//#region 🎬️WindowActionsAndSearchPanes`.
    pub window_actions_documents: HashMap<String, UiDocumentLease>,
    /// 🔎️ Each live window instance's projected Search pane document — the same region's twin.
    pub window_search_documents: HashMap<String, UiDocumentLease>,
    /// 🔀️ Each live World window instance's retained Projection tree — the bottom-right Pane body.
    pub window_projection_documents: HashMap<String, UiDocumentLease>,
    /// 🔎️ Per-window possibles-chevron state — React's `Search` `possiblesExpanded`
    /// (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:10771`), so absent = collapsed.
    pub search_possibles_open: HashMap<String, bool>,
    /// 📏️ Each live window instance's projected Measures overlay document — see `//#region 📏️WindowMeasures`.
    pub window_measures_documents: HashMap<String, UiDocumentLease>,
    /// 🪪️ `(revision, generation)` last minted per Measures overlay surface — the ledger
    /// `ui_document_ingress_generation` reads, kept across retirements so a reopened overlay moves forward.
    pub window_measures_minted: HashMap<String, (u64, u64)>,
    pub utility_collection_expanded: HashMap<String, bool>,
    pub contributor_instances: HashMap<String, u32>,
    /// 🖱️ Last-rendered full window content bounds per window id — used to apply the active utility's
    /// cursor while the pointer is inside that window's silhouette (Architecture Decision 8, P5).
    pub window_content_rects: HashMap<String, Rect>,
    /// 🎯️ Control id → the retained body that minted it this frame build, and the rect that body was
    /// painted into. The registry `InputState` keeps is flat and carries no owner, so this is what
    /// tells a document hit (route it into that body's retained tree, in that body's own local
    /// coordinates) from a chrome hit (route it through the shell's own handlers). Panels and dock
    /// windows are one map: both paint a retained document, and only their rect differs
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// 🎯️ Double-buffered against `ui_wgpu::wgpu::HitRegistry` and published in the same step: this
    /// is the owner map of the last COMPLETE chrome walk, so an id `InputState::hit_at` resolves is
    /// always one this map can name. `retained_hit_windows_staging` is the map the walk in progress
    /// is filling.
    pub retained_hit_windows: HashMap<String, (String, Rect)>,
    retained_hit_windows_staging: HashMap<String, (String, Rect)>,
    retained_scene_hits: HashMap<String, (Rect, crate::element_contracts::ScenePointerTarget)>,
    retained_scene_hits_staging: HashMap<String, (Rect, crate::element_contracts::ScenePointerTarget)>,
    presented_input_epoch: u64,
    next_presented_input_epoch: Option<u64>,
    presented_input_candidate: Option<PresentedInputCandidateWitness>,
    presented_input_geometry: PresentedInputGeometry,
    presented_input_geometry_staging: PresentedInputGeometry,
    retained_body_rects_staging: Vec<(String, Rect)>,
    presented_chrome_accessibility: Vec<ui_contract::AccessibilityProjectionNode>,
    /// ♿️ The presented epoch at which `presented_chrome_accessibility` last CHANGED — the generation the
    /// ARIA mirror addresses the chrome by. It advances with the projection, not with every presented
    /// frame, so a mirror that lags one idle frame still addresses the live chrome.
    presented_chrome_accessibility_generation: u64,
    /// 🪟️ The pane-overlay chip rows this frame's windows registered, held back until every window
    /// BODY has been walked and registered before the first docked panel.
    ///
    /// ⚖️ `InputState::hit_at` resolves in reverse registration order, so this deferral is React's
    /// own three-level stack read as an order: a chip outranks the window body and the world surface
    /// it is painted inside (`--z-pane: 20` over `--z-window: 10`) and LOSES to any docked panel
    /// floating over it (`--z-panel: 30`, `🖱️ui/🎨️styling/🖌️ui/🎨️.css:834-836`). React states the
    /// same rule in prose where it mounts the chips — "window pane toggles stay on their authored
    /// anchors behind anchored chrome panels; panels paint above `z-window` and occlude overlap
    /// without shifting pane chrome" (`🪟️Window/🟦️.tsx:200`).
    ///
    /// 🩸️ The rows used to be registered AFTER the panels, which made a covered chip outrank the
    /// panel over it — the opposite of React, where the press lands on the panel. Measured on the
    /// React reference with the Catalogue panel open over the top pane: pressing
    /// `framework.window.puzzle3dMainTop.{engagement,search}.toggle` journalled the catalogue row's
    /// own `addObjectKind` and left every fold closed
    /// (`🗑️generated/parity-run-12/steps.json` steps 9-10, ticket
    /// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w12d-pane-chip-ids-and-projection-toggle.md` §1).
    pane_overlay_hits: Vec<HitTarget<ActionDescriptor>>,
    /// 🏠️ The surface scope the chrome walk has established for this frame, read by
    /// [`Self::render_main_window_step`] through `ui_wgpu::wgpu::shell_floor_paints` — React's
    /// `shellFloorPaints` (`🔨️modules/🏠️shell-floor-presentation/🟦️.ts:14`). `FrameSetup` fills the
    /// whole surface at [`Level::Base`], which makes the main window's own identical base fill a
    /// second paint of the same colour over the same pixels; React suppresses exactly that.
    chrome_floor_scope: Option<ui_wgpu::wgpu::SurfaceScope>,
    /// 👋️ The retained body a pointer move was last routed into, so the move that LEAVES it is
    /// routed there too. `events::EventRouter` is edge-triggered on the moves it receives, so a body
    /// that stops receiving them keeps `NodeFlags::HOVERED` on whatever it last resolved.
    pub retained_hover_window: Option<String>,
    /// 🪟️ Last-rendered dock-stack silhouette per active window id (tabs + gap cutout + controls + body).
    pub window_silhouettes: HashMap<String, WindowSilhouette>,
    /// 🎬️ Active tutorial playback/recording runtime, if any — see `//#region 🎬️Tutorial` (below
    /// `ShellChrome`) for `TutorialRuntime`'s full shape, lifecycle, and the player/recorder it drives.
    pub tutorial: Option<TutorialRuntime>,
    /// 🎬️ Document-track operations queued by a tutorial tick/seek this frame, drained and applied
    /// asynchronously right after `render_chrome` returns (mirrors how `AppRuntime::frame` already defers
    /// `scene_events`/wheel actions through `spawn_app_task` for the same reason: the plugin bridge's
    /// `apply_mutations`/`handle_action` calls are async, but chrome rendering isn't).
    pub tutorial_pending_document_ops: Vec<TutorialPendingDocOp>,
    /// 🪪️ Who issued the dispatch currently crossing [`Self::dispatch_action`], for the chrome
    /// ledger's `origin` column — React's `InputOriginV1`. A chrome press, a keybinding, a palette
    /// entry and a panel row are all `user`; a pointer/camera stream an engine surface derived from
    /// raw input is `gesture`, which is the one origin React never turns into a notice
    /// (`🏛️ShellHost/🎯️input-ledger/🟦️.ts`: "a 60 Hz `setCamera` that the guest refuses would
    /// otherwise toast once per throttle window for as long as the user orbits"). Set by
    /// [`Self::dispatch_gesture_action`] for the frame's deferred surface actions and nothing else.
    dispatch_origin: &'static str,
    chrome_build: ShellChromeBuildState,
    chrome_present: ShellChromePresentState,
}
//#endregion ShellTypes

#[cfg(not(target_arch = "wasm32"))]
impl Drop for ShellState {
    fn drop(&mut self) {
        self.directory_cancel.cancel_now();
        if let Some(task) = self.identity_bootstrap_task.take() {
            task.cancel();
        }
        if let Some(mut home) = self.directory_home.take() {
            if let Some((plugin_id, instance_id)) = home.take_destroy_authority() {
                if let Some(program) = self.plugins.iter().find(|entry| entry.plugin_id == plugin_id) {
                    program.destroy_app(instance_id);
                }
            }
        }
        for pending in &self.shell_io_pending {
            if let Some(task) = &pending.task {
                task.cancel();
            }
        }
    }
}
