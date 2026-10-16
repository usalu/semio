//! 🖥️ framework/products/os/modules/renderer/engine/elements/🐚️Shell/component.rs — wgpu shell
//! chrome implementation for the Shell element, extracted from lib.rs's inline
//! `pub mod shell { ... }` body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired via
//! `#[path = "../../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"] pub mod shell;` in lib.rs in place of
//! the former inline block; the module name `shell` is unchanged, so every existing
//! `crate::shell::...` call site elsewhere in the crate keeps resolving with zero other changes.
//! 🖥️ OS shell chrome — navbar, footer, floating panels, overlays, and studio mode.
//!
//! 🧩 This file keeps the shared imports and re-exports the Shell crates, so every `crate::shell::...` path is unchanged.
//! The chunk folders that remain next to it are `cfg(test)` hosts of the tests that need the monolith fixtures; each reaches
//! the shared imports through `use super::*;`.

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

#[cfg(test)]
#[path = "⏪️time-travel/🦀️.rs"]
mod time_travel;

#[cfg(test)]
#[path = "📎️local-folders/🦀️.rs"]
mod local_folders;

#[cfg(test)]
#[path = "⚓️dock-model-and-measures/🦀️.rs"]
mod dock_model_and_measures;
#[cfg(test)]
pub use dock_model_and_measures::*;

#[cfg(test)]
#[path = "🧾️panel-records/🦀️.rs"]
mod panel_records;
#[cfg(test)]
pub(crate) use panel_records::*;

#[cfg(test)]
#[path = "⚙️settings-and-dock-persistence/🦀️.rs"]
mod settings_and_dock_persistence;

#[cfg(test)]
#[path = "⌨️keyboard-and-palette/🦀️.rs"]
mod keyboard_and_palette;

#[cfg(test)]
#[path = "🧱️chrome-paint-and-navbar/🦀️.rs"]
mod chrome_paint_and_navbar;
#[cfg(test)]
pub use chrome_paint_and_navbar::*;

#[cfg(test)]
#[path = "🔑️storage-prefs-and-i18n/🦀️.rs"]
mod storage_prefs_and_i18n;
#[cfg(test)]
pub use storage_prefs_and_i18n::*;

#[cfg(test)]
#[path = "🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod theme_file_import_and_accessibility;
#[cfg(test)]
pub use theme_file_import_and_accessibility::*;

pub use semio_framework_os_renderer_wgpu_contracts::{try_push_find_item, ActiveShellFindItems, ShellFindItem, ShellFindItemBinding, ShellFindItems, SHELL_FIND_ITEM_CAPACITY, SHELL_FIND_PAYLOAD_BYTES};

//#region 🧬️RendererWp4Components
pub use semio_framework_os_renderer_wgpu_shell_chrome_state::*;
pub use semio_framework_os_renderer_wgpu_shell_surfaces::*;
pub use semio_framework_os_renderer_wgpu_shell_plugins::*;
pub use semio_framework_os_renderer_wgpu_shell_history::*;
pub use semio_framework_os_renderer_wgpu_shell_prefs::*;
pub use semio_framework_os_renderer_wgpu_shell_directory::*;
pub use semio_framework_os_renderer_wgpu_shell_sync::*;
pub use semio_framework_os_renderer_wgpu_shell_docs::*;
pub use semio_framework_os_renderer_wgpu_shell_doc_open::*;
pub use semio_framework_os_renderer_wgpu_shell_windows::*;
pub use semio_framework_os_renderer_wgpu_shell_dock::*;
pub use semio_framework_os_renderer_wgpu_shell_input::*;
pub use semio_framework_os_renderer_wgpu_shell_commands::*;
//#endregion 🧬️RendererWp4Components

//#region 📥️RendererWp4Staging
// 🪓️ Step S0 of PLAN-RENDERER-WGPU-CRATES.md: the ShellState members and WP5-bound free items by crate, one module per (crate, chunk).
pub use semio_framework_os_renderer_wgpu_shell_async::*;
pub use semio_framework_os_renderer_wgpu_shell_state::*;
pub use semio_framework_os_renderer_wgpu_shell_session::*;
pub use semio_framework_os_renderer_wgpu_shell_window_actions::*;
pub use semio_framework_os_renderer_wgpu_shell_actions::*;
pub use semio_framework_os_renderer_wgpu_shell_pointer::*;
pub use semio_framework_os_renderer_wgpu_shell_view_paint::*;
pub use semio_framework_os_renderer_wgpu_shell_view_chrome::*;
pub use semio_framework_os_renderer_wgpu_shell_view_overlays::*;
pub use semio_framework_os_renderer_wgpu_shell_view_panels::*;
pub(crate) use semio_framework_os_renderer_wgpu_host_base::host_io_call;
pub(crate) use shell_vocab::shell_chrome_string;
//#endregion 📥️RendererWp4Staging

#[cfg(test)]
/// 🔎️ The whole Shell module as the one text the unsplit file held: this file and every child split out of it, for the source-text laws. The four modules that were already mounted before the split (`icon_export`, `tree_windows`, `time_travel`, `local_folders`) stay out, as they were out of that text.
pub(crate) const SHELL_SOURCE: &str = concat!(
    include_str!("🦀️.rs"),
    include_str!("⚓️dock-model-and-measures/🦀️.rs"),
    include_str!("🧾️panel-records/🦀️.rs"),
    include_str!("⚙️settings-and-dock-persistence/🦀️.rs"),
    include_str!("⌨️keyboard-and-palette/🦀️.rs"),
    include_str!("🧱️chrome-paint-and-navbar/🦀️.rs"),
    include_str!("🔑️storage-prefs-and-i18n/🦀️.rs"),
    include_str!("🎨️theme-file-import-and-accessibility/🦀️.rs"),
    include_str!("📥️staging/⏳️async/📨️command-dispatch/🦀️.rs"),
    include_str!("📥️staging/⏳️async/🗨️chrome-overlays-and-tour/🦀️.rs"),
    include_str!("📥️staging/⏳️async/🧩️session-and-topology/🦀️.rs"),
    include_str!("📥️staging/🎚️chrome-state/⌨️keyboard-and-palette/🦀️.rs"),
    include_str!("📥️staging/🎚️chrome-state/⏪️time-travel/🦀️.rs"),
    include_str!("📥️staging/🎚️chrome-state/⚓️dock-model-and-measures/🦀️.rs"),
    include_str!("📥️staging/🎚️chrome-state/⚙️settings-and-dock-persistence/🦀️.rs"),
    include_str!("📥️staging/🎚️chrome-state/🎨️theme-file-import-and-accessibility/🦀️.rs"),
    include_str!("📥️staging/🎚️chrome-state/💬️chrome-dialog-and-tooltips/🦀️.rs"),
    include_str!("📥️staging/🎚️chrome-state/🔑️storage-prefs-and-i18n/🦀️.rs"),
    include_str!("📥️staging/🎚️chrome-state/🔖️identity-and-types/🦀️.rs"),
    include_str!("📥️staging/🎚️chrome-state/🖱️pointer-input/🦀️.rs"),
    include_str!("📥️staging/🎚️chrome-state/🖼️chrome-frame-and-main-window/🦀️.rs"),
    include_str!("📥️staging/🎚️chrome-state/🗨️chrome-overlays-and-tour/🦀️.rs"),
    include_str!("📥️staging/🎚️chrome-state/🧰️window-actions-and-commands/🦀️.rs"),
    include_str!("📥️staging/🎚️chrome-state/🧱️chrome-paint-and-navbar/🦀️.rs"),
    include_str!("📥️staging/🫧️surfaces/⌨️keyboard-and-palette/🦀️.rs"),
    include_str!("📥️staging/🫧️surfaces/⚙️settings-and-dock-persistence/🦀️.rs"),
    include_str!("📥️staging/🫧️surfaces/🎓️tutorial-runtime/🦀️.rs"),
    include_str!("📥️staging/🫧️surfaces/🎨️theme-file-import-and-accessibility/🦀️.rs"),
    include_str!("📥️staging/🫧️surfaces/🎬️shell-actions/🦀️.rs"),
    include_str!("📥️staging/🫧️surfaces/🏛️space-admin-and-settle/🦀️.rs"),
    include_str!("📥️staging/🫧️surfaces/🖱️pointer-input/🦀️.rs"),
    include_str!("📥️staging/🫧️surfaces/🖼️chrome-frame-and-main-window/🦀️.rs"),
    include_str!("📥️staging/🫧️surfaces/🗨️chrome-overlays-and-tour/🦀️.rs"),
    include_str!("📥️staging/🫧️surfaces/🧩️session-and-topology/🦀️.rs"),
    include_str!("📥️staging/🫧️surfaces/🧭️navbar-footer-and-overlays/🦀️.rs"),
    include_str!("📥️staging/🫧️surfaces/🧱️chrome-paint-and-navbar/🦀️.rs"),
    include_str!("📥️staging/🫧️surfaces/🧵️shell-async-runners/🦀️.rs"),
    include_str!("📥️staging/🧲️plugins/⚙️settings-and-dock-persistence/🦀️.rs"),
    include_str!("📥️staging/🧲️plugins/🏛️space-admin-and-settle/🦀️.rs"),
    include_str!("📥️staging/🧲️plugins/📨️command-dispatch/🦀️.rs"),
    include_str!("📥️staging/🧲️plugins/🔖️identity-and-types/🦀️.rs"),
    include_str!("📥️staging/🧲️plugins/🐚️shell-parent/🦀️.rs"),
    include_str!("📥️staging/🧲️plugins/🧩️session-and-topology/🦀️.rs"),
    include_str!("📥️staging/🧲️plugins/🧾️panel-records/🦀️.rs"),
    include_str!("📥️staging/⏪️history/⏪️time-travel/🦀️.rs"),
    include_str!("📥️staging/⏪️history/📎️local-folders/🦀️.rs"),
    include_str!("📥️staging/⏪️history/🔑️storage-prefs-and-i18n/🦀️.rs"),
    include_str!("📥️staging/⏪️history/🔖️identity-and-types/🦀️.rs"),
    include_str!("📥️staging/🎛️prefs/⏪️time-travel/🦀️.rs"),
    include_str!("📥️staging/🎛️prefs/⚓️dock-model-and-measures/🦀️.rs"),
    include_str!("📥️staging/🎛️prefs/⚙️settings-and-dock-persistence/🦀️.rs"),
    include_str!("📥️staging/🎛️prefs/🎨️theme-file-import-and-accessibility/🦀️.rs"),
    include_str!("📥️staging/🎛️prefs/📎️local-folders/🦀️.rs"),
    include_str!("📥️staging/🎛️prefs/📨️command-dispatch/🦀️.rs"),
    include_str!("📥️staging/🎛️prefs/🔑️storage-prefs-and-i18n/🦀️.rs"),
    include_str!("📥️staging/🎛️prefs/🔖️identity-and-types/🦀️.rs"),
    include_str!("📥️staging/🎛️prefs/🖼️chrome-frame-and-main-window/🦀️.rs"),
    include_str!("📥️staging/🎛️prefs/🗨️chrome-overlays-and-tour/🦀️.rs"),
    include_str!("📥️staging/🎛️prefs/🧩️session-and-topology/🦀️.rs"),
    include_str!("📥️staging/🎛️prefs/🧰️window-actions-and-commands/🦀️.rs"),
    include_str!("📥️staging/🎛️prefs/🧱️chrome-paint-and-navbar/🦀️.rs"),
    include_str!("📥️staging/🎛️prefs/🧾️panel-records/🦀️.rs"),
    include_str!("📥️staging/📒️directory/⏪️time-travel/🦀️.rs"),
    include_str!("📥️staging/📒️directory/⚙️settings-and-dock-persistence/🦀️.rs"),
    include_str!("📥️staging/📒️directory/🎬️shell-actions/🦀️.rs"),
    include_str!("📥️staging/📒️directory/🏛️space-admin-and-settle/🦀️.rs"),
    include_str!("📥️staging/📒️directory/💬️chrome-dialog-and-tooltips/🦀️.rs"),
    include_str!("📥️staging/📒️directory/📨️command-dispatch/🦀️.rs"),
    include_str!("📥️staging/📒️directory/🔑️storage-prefs-and-i18n/🦀️.rs"),
    include_str!("📥️staging/📒️directory/🔖️identity-and-types/🦀️.rs"),
    include_str!("📥️staging/📒️directory/🖼️chrome-frame-and-main-window/🦀️.rs"),
    include_str!("📥️staging/📒️directory/🗨️chrome-overlays-and-tour/🦀️.rs"),
    include_str!("📥️staging/📒️directory/🧩️session-and-topology/🦀️.rs"),
    include_str!("📥️staging/📒️directory/🧭️navbar-footer-and-overlays/🦀️.rs"),
    include_str!("📥️staging/📒️directory/🧱️chrome-paint-and-navbar/🦀️.rs"),
    include_str!("📥️staging/📒️directory/🧵️shell-async-runners/🦀️.rs"),
    include_str!("📥️staging/📒️directory/🧾️panel-records/🦀️.rs"),
    include_str!("📥️staging/🔃️sync/⚙️settings-and-dock-persistence/🦀️.rs"),
    include_str!("📥️staging/🔃️sync/🎬️shell-actions/🦀️.rs"),
    include_str!("📥️staging/🔃️sync/📎️local-folders/🦀️.rs"),
    include_str!("📥️staging/🔃️sync/📤️icon-export/🦀️.rs"),
    include_str!("📥️staging/🔃️sync/📨️command-dispatch/🦀️.rs"),
    include_str!("📥️staging/🔃️sync/🔖️identity-and-types/🦀️.rs"),
    include_str!("📥️staging/🔃️sync/🖼️chrome-frame-and-main-window/🦀️.rs"),
    include_str!("📥️staging/🔃️sync/🧱️chrome-paint-and-navbar/🦀️.rs"),
    include_str!("📥️staging/📄️docs/⌨️keyboard-and-palette/🦀️.rs"),
    include_str!("📥️staging/📄️docs/⏪️time-travel/🦀️.rs"),
    include_str!("📥️staging/📄️docs/⚙️settings-and-dock-persistence/🦀️.rs"),
    include_str!("📥️staging/📄️docs/🎨️theme-file-import-and-accessibility/🦀️.rs"),
    include_str!("📥️staging/📄️docs/🏛️space-admin-and-settle/🦀️.rs"),
    include_str!("📥️staging/📄️docs/💬️chrome-dialog-and-tooltips/🦀️.rs"),
    include_str!("📥️staging/📄️docs/📎️local-folders/🦀️.rs"),
    include_str!("📥️staging/📄️docs/📨️command-dispatch/🦀️.rs"),
    include_str!("📥️staging/📄️docs/🔖️identity-and-types/🦀️.rs"),
    include_str!("📥️staging/📄️docs/🖱️pointer-input/🦀️.rs"),
    include_str!("📥️staging/📄️docs/🖼️chrome-frame-and-main-window/🦀️.rs"),
    include_str!("📥️staging/📄️docs/🗨️chrome-overlays-and-tour/🦀️.rs"),
    include_str!("📥️staging/📄️docs/🧩️session-and-topology/🦀️.rs"),
    include_str!("📥️staging/📄️docs/🧰️window-actions-and-commands/🦀️.rs"),
    include_str!("📥️staging/📄️docs/🧱️chrome-paint-and-navbar/🦀️.rs"),
    include_str!("📥️staging/📄️docs/🧵️shell-async-runners/🦀️.rs"),
    include_str!("📥️staging/📄️docs/🧾️panel-records/🦀️.rs"),
    include_str!("📥️staging/📂️doc-open/⌨️keyboard-and-palette/🦀️.rs"),
    include_str!("📥️staging/📂️doc-open/⚙️settings-and-dock-persistence/🦀️.rs"),
    include_str!("📥️staging/📂️doc-open/🎨️theme-file-import-and-accessibility/🦀️.rs"),
    include_str!("📥️staging/📂️doc-open/🎬️shell-actions/🦀️.rs"),
    include_str!("📥️staging/📂️doc-open/🏛️space-admin-and-settle/🦀️.rs"),
    include_str!("📥️staging/📂️doc-open/📤️icon-export/🦀️.rs"),
    include_str!("📥️staging/📂️doc-open/📨️command-dispatch/🦀️.rs"),
    include_str!("📥️staging/📂️doc-open/🔑️storage-prefs-and-i18n/🦀️.rs"),
    include_str!("📥️staging/📂️doc-open/🔖️identity-and-types/🦀️.rs"),
    include_str!("📥️staging/📂️doc-open/🖼️chrome-frame-and-main-window/🦀️.rs"),
    include_str!("📥️staging/📂️doc-open/🗨️chrome-overlays-and-tour/🦀️.rs"),
    include_str!("📥️staging/📂️doc-open/🧱️chrome-paint-and-navbar/🦀️.rs"),
    include_str!("📥️staging/🔲️windows/⚓️dock-model-and-measures/🦀️.rs"),
    include_str!("📥️staging/🔲️windows/⚙️settings-and-dock-persistence/🦀️.rs"),
    include_str!("📥️staging/🔲️windows/🏛️space-admin-and-settle/🦀️.rs"),
    include_str!("📥️staging/🔲️windows/📨️command-dispatch/🦀️.rs"),
    include_str!("📥️staging/🔲️windows/🖱️pointer-input/🦀️.rs"),
    include_str!("📥️staging/🔲️windows/🐚️shell-parent/🦀️.rs"),
    include_str!("📥️staging/🔲️windows/🧩️session-and-topology/🦀️.rs"),
    include_str!("📥️staging/🔲️windows/🧰️window-actions-and-commands/🦀️.rs"),
    include_str!("📥️staging/🔲️windows/🧱️chrome-paint-and-navbar/🦀️.rs"),
    include_str!("📥️staging/🔲️windows/🧾️panel-records/🦀️.rs"),
    include_str!("📥️staging/⚓️dock/⌨️keyboard-and-palette/🦀️.rs"),
    include_str!("📥️staging/⚓️dock/⚓️dock-model-and-measures/🦀️.rs"),
    include_str!("📥️staging/⚓️dock/⚙️settings-and-dock-persistence/🦀️.rs"),
    include_str!("📥️staging/⚓️dock/🎓️tutorial-runtime/🦀️.rs"),
    include_str!("📥️staging/⚓️dock/🎬️shell-actions/🦀️.rs"),
    include_str!("📥️staging/⚓️dock/💬️chrome-dialog-and-tooltips/🦀️.rs"),
    include_str!("📥️staging/⚓️dock/🔑️storage-prefs-and-i18n/🦀️.rs"),
    include_str!("📥️staging/⚓️dock/🔖️identity-and-types/🦀️.rs"),
    include_str!("📥️staging/⚓️dock/🖱️pointer-input/🦀️.rs"),
    include_str!("📥️staging/⚓️dock/🖼️chrome-frame-and-main-window/🦀️.rs"),
    include_str!("📥️staging/⚓️dock/🗨️chrome-overlays-and-tour/🦀️.rs"),
    include_str!("📥️staging/⚓️dock/🐚️shell-parent/🦀️.rs"),
    include_str!("📥️staging/⚓️dock/🧩️session-and-topology/🦀️.rs"),
    include_str!("📥️staging/⚓️dock/🧰️window-actions-and-commands/🦀️.rs"),
    include_str!("📥️staging/⚓️dock/🧱️chrome-paint-and-navbar/🦀️.rs"),
    include_str!("📥️staging/⚓️dock/🧾️panel-records/🦀️.rs"),
    include_str!("📥️staging/⚓️dock/🪟️tree-windows/🦀️.rs"),
    include_str!("📥️staging/🕹️input/⌨️keyboard-and-palette/🦀️.rs"),
    include_str!("📥️staging/🕹️input/⏪️time-travel/🦀️.rs"),
    include_str!("📥️staging/🕹️input/⚙️settings-and-dock-persistence/🦀️.rs"),
    include_str!("📥️staging/🕹️input/🎨️theme-file-import-and-accessibility/🦀️.rs"),
    include_str!("📥️staging/🕹️input/🏛️space-admin-and-settle/🦀️.rs"),
    include_str!("📥️staging/🕹️input/💬️chrome-dialog-and-tooltips/🦀️.rs"),
    include_str!("📥️staging/🕹️input/🔑️storage-prefs-and-i18n/🦀️.rs"),
    include_str!("📥️staging/🕹️input/🔖️identity-and-types/🦀️.rs"),
    include_str!("📥️staging/🕹️input/🖱️pointer-input/🦀️.rs"),
    include_str!("📥️staging/🕹️input/🖼️chrome-frame-and-main-window/🦀️.rs"),
    include_str!("📥️staging/🕹️input/🗨️chrome-overlays-and-tour/🦀️.rs"),
    include_str!("📥️staging/🕹️input/🧩️session-and-topology/🦀️.rs"),
    include_str!("📥️staging/🕹️input/🧰️window-actions-and-commands/🦀️.rs"),
    include_str!("📥️staging/🕹️input/🧱️chrome-paint-and-navbar/🦀️.rs"),
    include_str!("📥️staging/🕹️input/🧾️panel-records/🦀️.rs"),
    include_str!("📥️staging/🎹️commands/⌨️keyboard-and-palette/🦀️.rs"),
    include_str!("📥️staging/🎹️commands/🧩️session-and-topology/🦀️.rs"),
    include_str!("📥️staging/🎹️commands/🧰️window-actions-and-commands/🦀️.rs"),
    include_str!("📥️staging/🐙️state/🎨️theme-file-import-and-accessibility/🦀️.rs"),
    include_str!("📥️staging/🐙️state/🔖️identity-and-types/🦀️.rs"),
    include_str!("📥️staging/🐙️state/🧩️session-and-topology/🦀️.rs"),
    include_str!("📥️staging/🚶️session/⌨️keyboard-and-palette/🦀️.rs"),
    include_str!("📥️staging/🚶️session/⏪️time-travel/🦀️.rs"),
    include_str!("📥️staging/🚶️session/⚓️dock-model-and-measures/🦀️.rs"),
    include_str!("📥️staging/🚶️session/⚙️settings-and-dock-persistence/🦀️.rs"),
    include_str!("📥️staging/🚶️session/🎨️theme-file-import-and-accessibility/🦀️.rs"),
    include_str!("📥️staging/🚶️session/🎬️shell-actions/🦀️.rs"),
    include_str!("📥️staging/🚶️session/🏛️space-admin-and-settle/🦀️.rs"),
    include_str!("📥️staging/🚶️session/🐚️shell-parent/🦀️.rs"),
    include_str!("📥️staging/🚶️session/💬️chrome-dialog-and-tooltips/🦀️.rs"),
    include_str!("📥️staging/🚶️session/📎️local-folders/🦀️.rs"),
    include_str!("📥️staging/🚶️session/📨️command-dispatch/🦀️.rs"),
    include_str!("📥️staging/🚶️session/🔖️identity-and-types/🦀️.rs"),
    include_str!("📥️staging/🚶️session/🖱️pointer-input/🦀️.rs"),
    include_str!("📥️staging/🚶️session/🖼️chrome-frame-and-main-window/🦀️.rs"),
    include_str!("📥️staging/🚶️session/🗨️chrome-overlays-and-tour/🦀️.rs"),
    include_str!("📥️staging/🚶️session/🧩️session-and-topology/🦀️.rs"),
    include_str!("📥️staging/🚶️session/🧰️window-actions-and-commands/🦀️.rs"),
    include_str!("📥️staging/🚶️session/🧱️chrome-paint-and-navbar/🦀️.rs"),
    include_str!("📥️staging/🚶️session/🧺️bundle/🦀️.rs"),
    include_str!("📥️staging/🚶️session/🧾️panel-records/🦀️.rs"),
    include_str!("📥️staging/🚶️session/🪟️tree-windows/🦀️.rs"),
    include_str!("📥️staging/🪝️window-actions/⚓️dock-model-and-measures/🦀️.rs"),
    include_str!("📥️staging/🪝️window-actions/⚙️settings-and-dock-persistence/🦀️.rs"),
    include_str!("📥️staging/🪝️window-actions/🎓️tutorial-runtime/🦀️.rs"),
    include_str!("📥️staging/🪝️window-actions/🎨️theme-file-import-and-accessibility/🦀️.rs"),
    include_str!("📥️staging/🪝️window-actions/🎬️shell-actions/🦀️.rs"),
    include_str!("📥️staging/🪝️window-actions/🏛️space-admin-and-settle/🦀️.rs"),
    include_str!("📥️staging/🪝️window-actions/📎️local-folders/🦀️.rs"),
    include_str!("📥️staging/🪝️window-actions/📨️command-dispatch/🦀️.rs"),
    include_str!("📥️staging/🪝️window-actions/🔑️storage-prefs-and-i18n/🦀️.rs"),
    include_str!("📥️staging/🪝️window-actions/🔖️identity-and-types/🦀️.rs"),
    include_str!("📥️staging/🪝️window-actions/🖱️pointer-input/🦀️.rs"),
    include_str!("📥️staging/🪝️window-actions/🗨️chrome-overlays-and-tour/🦀️.rs"),
    include_str!("📥️staging/🪝️window-actions/🧩️session-and-topology/🦀️.rs"),
    include_str!("📥️staging/🪝️window-actions/🧰️window-actions-and-commands/🦀️.rs"),
    include_str!("📥️staging/🪝️window-actions/🧱️chrome-paint-and-navbar/🦀️.rs"),
    include_str!("📥️staging/🪝️window-actions/🧵️shell-async-runners/🦀️.rs"),
    include_str!("📥️staging/🪝️window-actions/🧺️bundle/🦀️.rs"),
    include_str!("📥️staging/🪝️window-actions/🧾️panel-records/🦀️.rs"),
    include_str!("📥️staging/🔀️actions/⌨️keyboard-and-palette/🦀️.rs"),
    include_str!("📥️staging/🔀️actions/⏪️time-travel/🦀️.rs"),
    include_str!("📥️staging/🔀️actions/🎨️theme-file-import-and-accessibility/🦀️.rs"),
    include_str!("📥️staging/🔀️actions/🎬️shell-actions/🦀️.rs"),
    include_str!("📥️staging/🔀️actions/🏛️space-admin-and-settle/🦀️.rs"),
    include_str!("📥️staging/🔀️actions/📎️local-folders/🦀️.rs"),
    include_str!("📥️staging/🔀️actions/📨️command-dispatch/🦀️.rs"),
    include_str!("📥️staging/🔀️actions/🔖️identity-and-types/🦀️.rs"),
    include_str!("📥️staging/🔀️actions/🗨️chrome-overlays-and-tour/🦀️.rs"),
    include_str!("📥️staging/🔀️actions/🧩️session-and-topology/🦀️.rs"),
    include_str!("📥️staging/🔀️actions/🧰️window-actions-and-commands/🦀️.rs"),
    include_str!("📥️staging/🔀️actions/🧱️chrome-paint-and-navbar/🦀️.rs"),
    include_str!("📥️staging/🔀️actions/🧺️bundle/🦀️.rs"),
    include_str!("📥️staging/🔀️actions/🧾️panel-records/🦀️.rs"),
    include_str!("📥️staging/🖐️pointer/⌨️keyboard-and-palette/🦀️.rs"),
    include_str!("📥️staging/🖐️pointer/🎓️tutorial-runtime/🦀️.rs"),
    include_str!("📥️staging/🖐️pointer/🎨️theme-file-import-and-accessibility/🦀️.rs"),
    include_str!("📥️staging/🖐️pointer/🎬️shell-actions/🦀️.rs"),
    include_str!("📥️staging/🖐️pointer/🏛️space-admin-and-settle/🦀️.rs"),
    include_str!("📥️staging/🖐️pointer/📨️command-dispatch/🦀️.rs"),
    include_str!("📥️staging/🖐️pointer/🔑️storage-prefs-and-i18n/🦀️.rs"),
    include_str!("📥️staging/🖐️pointer/🔖️identity-and-types/🦀️.rs"),
    include_str!("📥️staging/🖐️pointer/🖱️pointer-input/🦀️.rs"),
    include_str!("📥️staging/🖐️pointer/🧩️session-and-topology/🦀️.rs"),
    include_str!("📥️staging/🖐️pointer/🧰️window-actions-and-commands/🦀️.rs"),
    include_str!("📥️staging/🖐️pointer/🧱️chrome-paint-and-navbar/🦀️.rs"),
    include_str!("📥️staging/🖐️pointer/🧵️shell-async-runners/🦀️.rs"),
    include_str!("📥️staging/🖐️pointer/🧺️bundle/🦀️.rs"),
    include_str!("📥️staging/🖍️view-paint/⚓️dock-model-and-measures/🦀️.rs"),
    include_str!("📥️staging/🖍️view-paint/🎓️tutorial-runtime/🦀️.rs"),
    include_str!("📥️staging/🖍️view-paint/🎨️theme-file-import-and-accessibility/🦀️.rs"),
    include_str!("📥️staging/🖍️view-paint/🏛️space-admin-and-settle/🦀️.rs"),
    include_str!("📥️staging/🖍️view-paint/🔑️storage-prefs-and-i18n/🦀️.rs"),
    include_str!("📥️staging/🖍️view-paint/🔖️identity-and-types/🦀️.rs"),
    include_str!("📥️staging/🖍️view-paint/🖼️chrome-frame-and-main-window/🦀️.rs"),
    include_str!("📥️staging/🖍️view-paint/🗨️chrome-overlays-and-tour/🦀️.rs"),
    include_str!("📥️staging/🖍️view-paint/🧱️chrome-paint-and-navbar/🦀️.rs"),
    include_str!("📥️staging/🍱️view-chrome/⚓️dock-model-and-measures/🦀️.rs"),
    include_str!("📥️staging/🍱️view-chrome/🎨️theme-file-import-and-accessibility/🦀️.rs"),
    include_str!("📥️staging/🍱️view-chrome/🏛️space-admin-and-settle/🦀️.rs"),
    include_str!("📥️staging/🍱️view-chrome/🖱️pointer-input/🦀️.rs"),
    include_str!("📥️staging/🍱️view-chrome/🖼️chrome-frame-and-main-window/🦀️.rs"),
    include_str!("📥️staging/🍱️view-chrome/🗨️chrome-overlays-and-tour/🦀️.rs"),
    include_str!("📥️staging/🍱️view-chrome/🧭️navbar-footer-and-overlays/🦀️.rs"),
    include_str!("📥️staging/🍱️view-chrome/🧱️chrome-paint-and-navbar/🦀️.rs"),
    include_str!("📥️staging/🍱️view-chrome/🧺️bundle/🦀️.rs"),
    include_str!("📥️staging/🗨️view-overlays/⏪️time-travel/🦀️.rs"),
    include_str!("📥️staging/🗨️view-overlays/🎨️theme-file-import-and-accessibility/🦀️.rs"),
    include_str!("📥️staging/🗨️view-overlays/💬️chrome-dialog-and-tooltips/🦀️.rs"),
    include_str!("📥️staging/🗨️view-overlays/📎️local-folders/🦀️.rs"),
    include_str!("📥️staging/🗨️view-overlays/🔖️identity-and-types/🦀️.rs"),
    include_str!("📥️staging/🗨️view-overlays/🗨️chrome-overlays-and-tour/🦀️.rs"),
    include_str!("📥️staging/🗨️view-overlays/🧭️navbar-footer-and-overlays/🦀️.rs"),
    include_str!("📥️staging/🗨️view-overlays/🧱️chrome-paint-and-navbar/🦀️.rs"),
    include_str!("📥️staging/🗨️view-overlays/🧺️bundle/🦀️.rs"),
    include_str!("📥️staging/🪀️view-panels/⌨️keyboard-and-palette/🦀️.rs"),
    include_str!("📥️staging/🪀️view-panels/🎨️theme-file-import-and-accessibility/🦀️.rs"),
    include_str!("📥️staging/🪀️view-panels/🎬️shell-actions/🦀️.rs"),
    include_str!("📥️staging/🪀️view-panels/📤️icon-export/🦀️.rs"),
    include_str!("📥️staging/🪀️view-panels/🔖️identity-and-types/🦀️.rs"),
    include_str!("📥️staging/🪀️view-panels/🖱️pointer-input/🦀️.rs"),
    include_str!("📥️staging/🪀️view-panels/🖼️chrome-frame-and-main-window/🦀️.rs"),
    include_str!("📥️staging/🪀️view-panels/🗨️chrome-overlays-and-tour/🦀️.rs"),
    include_str!("📥️staging/🪀️view-panels/🧭️navbar-footer-and-overlays/🦀️.rs"),
    include_str!("📥️staging/🪀️view-panels/🧰️window-actions-and-commands/🦀️.rs"),
    include_str!("📥️staging/🪀️view-panels/🧱️chrome-paint-and-navbar/🦀️.rs"),
    include_str!("📥️staging/🪀️view-panels/🧺️bundle/🦀️.rs"),
    include_str!("📥️staging/🎚️chrome-state/🧬️component/🦀️.rs"),
    include_str!("📥️staging/🫧️surfaces/🧬️component/🦀️.rs"),
    include_str!("📥️staging/🧲️plugins/🧬️component/🦀️.rs"),
    include_str!("📥️staging/⏪️history/🧬️component/🦀️.rs"),
    include_str!("📥️staging/🎛️prefs/🧬️component/🦀️.rs"),
    include_str!("📥️staging/📒️directory/🧬️component/🦀️.rs"),
    include_str!("📥️staging/🔃️sync/🧬️component/🦀️.rs"),
    include_str!("📥️staging/📄️docs/🧬️component/🦀️.rs"),
    include_str!("📥️staging/📂️doc-open/🧬️component/🦀️.rs"),
    include_str!("📥️staging/🔲️windows/🧬️component/🦀️.rs"),
    include_str!("📥️staging/⚓️dock/🧬️component/🦀️.rs"),
    include_str!("📥️staging/🕹️input/🧬️component/🦀️.rs"),
    include_str!("📥️staging/🎹️commands/🧬️component/🦀️.rs")
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
