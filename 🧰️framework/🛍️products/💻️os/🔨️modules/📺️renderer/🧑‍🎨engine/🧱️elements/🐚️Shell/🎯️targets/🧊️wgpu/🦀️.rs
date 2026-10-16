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

//#region 🧬️RendererWp4Components
#[path = "📥️staging/🎚️chrome-state/🧬️component/🦀️.rs"]
mod staged_chrome_state__component;
pub use staged_chrome_state__component::*;
#[path = "📥️staging/🫧️surfaces/🧬️component/🦀️.rs"]
mod staged_surfaces__component;
pub use staged_surfaces__component::*;
#[path = "📥️staging/🧲️plugins/🧬️component/🦀️.rs"]
mod staged_plugins__component;
pub use staged_plugins__component::*;
#[path = "📥️staging/⏪️history/🧬️component/🦀️.rs"]
mod staged_history__component;
pub use staged_history__component::*;
#[path = "📥️staging/🎛️prefs/🧬️component/🦀️.rs"]
mod staged_prefs__component;
pub use staged_prefs__component::*;
#[path = "📥️staging/📒️directory/🧬️component/🦀️.rs"]
mod staged_directory__component;
pub use staged_directory__component::*;
#[path = "📥️staging/🔃️sync/🧬️component/🦀️.rs"]
mod staged_sync__component;
pub use staged_sync__component::*;
#[path = "📥️staging/📄️docs/🧬️component/🦀️.rs"]
mod staged_docs__component;
pub use staged_docs__component::*;
#[path = "📥️staging/📂️doc-open/🧬️component/🦀️.rs"]
mod staged_doc_open__component;
pub use staged_doc_open__component::*;
#[path = "📥️staging/🔲️windows/🧬️component/🦀️.rs"]
mod staged_windows__component;
pub use staged_windows__component::*;
#[path = "📥️staging/⚓️dock/🧬️component/🦀️.rs"]
mod staged_dock__component;
pub use staged_dock__component::*;
#[path = "📥️staging/🕹️input/🧬️component/🦀️.rs"]
mod staged_input__component;
pub use staged_input__component::*;
#[path = "📥️staging/🎹️commands/🧬️component/🦀️.rs"]
mod staged_commands__component;
pub use staged_commands__component::*;
//#endregion 🧬️RendererWp4Components

//#region 📥️RendererWp4Staging
// 🪓️ Step S0 of PLAN-RENDERER-WGPU-CRATES.md: the ShellState members and WP5-bound free items by crate, one module per (crate, chunk).
#[path = "📥️staging/⏳️async/📨️command-dispatch/🦀️.rs"]
mod staged_async__command_dispatch;
pub(crate) use staged_async__command_dispatch::*;
#[path = "📥️staging/⏳️async/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_async__chrome_overlays_and_tour;
pub use staged_async__chrome_overlays_and_tour::*;
#[path = "📥️staging/⏳️async/🧩️session-and-topology/🦀️.rs"]
mod staged_async__session_and_topology;
pub(crate) use staged_async__session_and_topology::*;
#[path = "📥️staging/🎚️chrome-state/⌨️keyboard-and-palette/🦀️.rs"]
mod staged_chrome_state__keyboard_and_palette;
#[path = "📥️staging/🎚️chrome-state/⏪️time-travel/🦀️.rs"]
mod staged_chrome_state__time_travel;
pub(crate) use staged_chrome_state__time_travel::*;
#[path = "📥️staging/🎚️chrome-state/⚓️dock-model-and-measures/🦀️.rs"]
mod staged_chrome_state__dock_model_and_measures;
pub use staged_chrome_state__dock_model_and_measures::*;
#[path = "📥️staging/🎚️chrome-state/⚙️settings-and-dock-persistence/🦀️.rs"]
mod staged_chrome_state__settings_and_dock_persistence;
#[path = "📥️staging/🎚️chrome-state/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_chrome_state__theme_file_import_and_accessibility;
pub use staged_chrome_state__theme_file_import_and_accessibility::*;
#[path = "📥️staging/🎚️chrome-state/💬️chrome-dialog-and-tooltips/🦀️.rs"]
mod staged_chrome_state__chrome_dialog_and_tooltips;
#[path = "📥️staging/🎚️chrome-state/🔑️storage-prefs-and-i18n/🦀️.rs"]
mod staged_chrome_state__storage_prefs_and_i18n;
pub use staged_chrome_state__storage_prefs_and_i18n::*;
#[path = "📥️staging/🎚️chrome-state/🔖️identity-and-types/🦀️.rs"]
mod staged_chrome_state__identity_and_types;
pub use staged_chrome_state__identity_and_types::*;
#[path = "📥️staging/🎚️chrome-state/🖱️pointer-input/🦀️.rs"]
mod staged_chrome_state__pointer_input;
pub use staged_chrome_state__pointer_input::*;
#[path = "📥️staging/🎚️chrome-state/🖼️chrome-frame-and-main-window/🦀️.rs"]
mod staged_chrome_state__chrome_frame_and_main_window;
#[path = "📥️staging/🎚️chrome-state/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_chrome_state__chrome_overlays_and_tour;
pub use staged_chrome_state__chrome_overlays_and_tour::*;
#[path = "📥️staging/🎚️chrome-state/🧰️window-actions-and-commands/🦀️.rs"]
mod staged_chrome_state__window_actions_and_commands;
pub(crate) use staged_chrome_state__window_actions_and_commands::*;
#[path = "📥️staging/🎚️chrome-state/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_chrome_state__chrome_paint_and_navbar;
pub use staged_chrome_state__chrome_paint_and_navbar::*;
#[path = "📥️staging/🫧️surfaces/⌨️keyboard-and-palette/🦀️.rs"]
mod staged_surfaces__keyboard_and_palette;
#[path = "📥️staging/🫧️surfaces/⚙️settings-and-dock-persistence/🦀️.rs"]
mod staged_surfaces__settings_and_dock_persistence;
#[path = "📥️staging/🫧️surfaces/🎓️tutorial-runtime/🦀️.rs"]
mod staged_surfaces__tutorial_runtime;
pub use staged_surfaces__tutorial_runtime::*;
#[path = "📥️staging/🫧️surfaces/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_surfaces__theme_file_import_and_accessibility;
pub use staged_surfaces__theme_file_import_and_accessibility::*;
#[path = "📥️staging/🫧️surfaces/🎬️shell-actions/🦀️.rs"]
mod staged_surfaces__shell_actions;
#[path = "📥️staging/🫧️surfaces/🏛️space-admin-and-settle/🦀️.rs"]
mod staged_surfaces__space_admin_and_settle;
pub use staged_surfaces__space_admin_and_settle::*;
#[path = "📥️staging/🫧️surfaces/🖱️pointer-input/🦀️.rs"]
mod staged_surfaces__pointer_input;
pub use staged_surfaces__pointer_input::*;
#[path = "📥️staging/🫧️surfaces/🖼️chrome-frame-and-main-window/🦀️.rs"]
mod staged_surfaces__chrome_frame_and_main_window;
#[path = "📥️staging/🫧️surfaces/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_surfaces__chrome_overlays_and_tour;
#[path = "📥️staging/🫧️surfaces/🧩️session-and-topology/🦀️.rs"]
mod staged_surfaces__session_and_topology;
#[path = "📥️staging/🫧️surfaces/🧭️navbar-footer-and-overlays/🦀️.rs"]
mod staged_surfaces__navbar_footer_and_overlays;
#[path = "📥️staging/🫧️surfaces/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_surfaces__chrome_paint_and_navbar;
pub use staged_surfaces__chrome_paint_and_navbar::*;
#[path = "📥️staging/🫧️surfaces/🧵️shell-async-runners/🦀️.rs"]
mod staged_surfaces__shell_async_runners;
pub(crate) use staged_surfaces__shell_async_runners::*;
#[path = "📥️staging/🧲️plugins/⚙️settings-and-dock-persistence/🦀️.rs"]
mod staged_plugins__settings_and_dock_persistence;
#[path = "📥️staging/🧲️plugins/🏛️space-admin-and-settle/🦀️.rs"]
mod staged_plugins__space_admin_and_settle;
pub use staged_plugins__space_admin_and_settle::*;
#[path = "📥️staging/🧲️plugins/📨️command-dispatch/🦀️.rs"]
mod staged_plugins__command_dispatch;
pub(crate) use staged_plugins__command_dispatch::*;
#[path = "📥️staging/🧲️plugins/🔖️identity-and-types/🦀️.rs"]
mod staged_plugins__identity_and_types;
pub use staged_plugins__identity_and_types::*;
#[path = "📥️staging/🧲️plugins/🐚️shell-parent/🦀️.rs"]
mod staged_plugins__shell_parent;
pub(crate) use staged_plugins__shell_parent::*;
#[path = "📥️staging/🧲️plugins/🧩️session-and-topology/🦀️.rs"]
mod staged_plugins__session_and_topology;
pub(crate) use staged_plugins__session_and_topology::*;
#[path = "📥️staging/🧲️plugins/🧾️panel-records/🦀️.rs"]
mod staged_plugins__panel_records;
pub(crate) use staged_plugins__panel_records::*;
#[path = "📥️staging/⏪️history/⏪️time-travel/🦀️.rs"]
mod staged_history__time_travel;
#[path = "📥️staging/⏪️history/📎️local-folders/🦀️.rs"]
mod staged_history__local_folders;
pub(crate) use staged_history__local_folders::*;
#[path = "📥️staging/⏪️history/🔑️storage-prefs-and-i18n/🦀️.rs"]
mod staged_history__storage_prefs_and_i18n;
pub use staged_history__storage_prefs_and_i18n::*;
#[path = "📥️staging/⏪️history/🔖️identity-and-types/🦀️.rs"]
mod staged_history__identity_and_types;
pub use staged_history__identity_and_types::*;
#[path = "📥️staging/🎛️prefs/⏪️time-travel/🦀️.rs"]
mod staged_prefs__time_travel;
pub(crate) use staged_prefs__time_travel::*;
#[path = "📥️staging/🎛️prefs/⚓️dock-model-and-measures/🦀️.rs"]
mod staged_prefs__dock_model_and_measures;
pub use staged_prefs__dock_model_and_measures::*;
#[path = "📥️staging/🎛️prefs/⚙️settings-and-dock-persistence/🦀️.rs"]
mod staged_prefs__settings_and_dock_persistence;
#[path = "📥️staging/🎛️prefs/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_prefs__theme_file_import_and_accessibility;
pub use staged_prefs__theme_file_import_and_accessibility::*;
#[path = "📥️staging/🎛️prefs/📎️local-folders/🦀️.rs"]
mod staged_prefs__local_folders;
pub(crate) use staged_prefs__local_folders::*;
#[path = "📥️staging/🎛️prefs/📨️command-dispatch/🦀️.rs"]
mod staged_prefs__command_dispatch;
#[path = "📥️staging/🎛️prefs/🔑️storage-prefs-and-i18n/🦀️.rs"]
mod staged_prefs__storage_prefs_and_i18n;
pub use staged_prefs__storage_prefs_and_i18n::*;
#[path = "📥️staging/🎛️prefs/🔖️identity-and-types/🦀️.rs"]
mod staged_prefs__identity_and_types;
pub use staged_prefs__identity_and_types::*;
#[path = "📥️staging/🎛️prefs/🖼️chrome-frame-and-main-window/🦀️.rs"]
mod staged_prefs__chrome_frame_and_main_window;
#[path = "📥️staging/🎛️prefs/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_prefs__chrome_overlays_and_tour;
pub use staged_prefs__chrome_overlays_and_tour::*;
#[path = "📥️staging/🎛️prefs/🧩️session-and-topology/🦀️.rs"]
mod staged_prefs__session_and_topology;
#[path = "📥️staging/🎛️prefs/🧰️window-actions-and-commands/🦀️.rs"]
mod staged_prefs__window_actions_and_commands;
pub(crate) use staged_prefs__window_actions_and_commands::*;
#[path = "📥️staging/🎛️prefs/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_prefs__chrome_paint_and_navbar;
pub use staged_prefs__chrome_paint_and_navbar::*;
#[path = "📥️staging/🎛️prefs/🧾️panel-records/🦀️.rs"]
mod staged_prefs__panel_records;
pub(crate) use staged_prefs__panel_records::*;
#[path = "📥️staging/📒️directory/⏪️time-travel/🦀️.rs"]
mod staged_directory__time_travel;
pub(crate) use staged_directory__time_travel::*;
#[path = "📥️staging/📒️directory/⚙️settings-and-dock-persistence/🦀️.rs"]
mod staged_directory__settings_and_dock_persistence;
#[path = "📥️staging/📒️directory/🎬️shell-actions/🦀️.rs"]
mod staged_directory__shell_actions;
#[path = "📥️staging/📒️directory/🏛️space-admin-and-settle/🦀️.rs"]
mod staged_directory__space_admin_and_settle;
pub use staged_directory__space_admin_and_settle::*;
#[path = "📥️staging/📒️directory/💬️chrome-dialog-and-tooltips/🦀️.rs"]
mod staged_directory__chrome_dialog_and_tooltips;
#[path = "📥️staging/📒️directory/📨️command-dispatch/🦀️.rs"]
mod staged_directory__command_dispatch;
#[path = "📥️staging/📒️directory/🔑️storage-prefs-and-i18n/🦀️.rs"]
mod staged_directory__storage_prefs_and_i18n;
pub use staged_directory__storage_prefs_and_i18n::*;
#[path = "📥️staging/📒️directory/🔖️identity-and-types/🦀️.rs"]
mod staged_directory__identity_and_types;
pub use staged_directory__identity_and_types::*;
#[path = "📥️staging/📒️directory/🖼️chrome-frame-and-main-window/🦀️.rs"]
mod staged_directory__chrome_frame_and_main_window;
#[path = "📥️staging/📒️directory/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_directory__chrome_overlays_and_tour;
#[path = "📥️staging/📒️directory/🧩️session-and-topology/🦀️.rs"]
mod staged_directory__session_and_topology;
#[path = "📥️staging/📒️directory/🧭️navbar-footer-and-overlays/🦀️.rs"]
mod staged_directory__navbar_footer_and_overlays;
#[path = "📥️staging/📒️directory/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_directory__chrome_paint_and_navbar;
pub use staged_directory__chrome_paint_and_navbar::*;
#[path = "📥️staging/📒️directory/🧵️shell-async-runners/🦀️.rs"]
mod staged_directory__shell_async_runners;
pub(crate) use staged_directory__shell_async_runners::*;
#[path = "📥️staging/📒️directory/🧾️panel-records/🦀️.rs"]
mod staged_directory__panel_records;
pub(crate) use staged_directory__panel_records::*;
#[path = "📥️staging/🔃️sync/⚙️settings-and-dock-persistence/🦀️.rs"]
mod staged_sync__settings_and_dock_persistence;
#[path = "📥️staging/🔃️sync/🎬️shell-actions/🦀️.rs"]
mod staged_sync__shell_actions;
#[path = "📥️staging/🔃️sync/📎️local-folders/🦀️.rs"]
mod staged_sync__local_folders;
#[path = "📥️staging/🔃️sync/📤️icon-export/🦀️.rs"]
mod staged_sync__icon_export;
pub(crate) use staged_sync__icon_export::*;
#[path = "📥️staging/🔃️sync/📨️command-dispatch/🦀️.rs"]
mod staged_sync__command_dispatch;
#[path = "📥️staging/🔃️sync/🔖️identity-and-types/🦀️.rs"]
mod staged_sync__identity_and_types;
pub use staged_sync__identity_and_types::*;
#[path = "📥️staging/🔃️sync/🖼️chrome-frame-and-main-window/🦀️.rs"]
mod staged_sync__chrome_frame_and_main_window;
#[path = "📥️staging/🔃️sync/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_sync__chrome_paint_and_navbar;
pub use staged_sync__chrome_paint_and_navbar::*;
#[path = "📥️staging/📄️docs/⌨️keyboard-and-palette/🦀️.rs"]
mod staged_docs__keyboard_and_palette;
pub(crate) use staged_docs__keyboard_and_palette::*;
#[path = "📥️staging/📄️docs/⏪️time-travel/🦀️.rs"]
mod staged_docs__time_travel;
pub(crate) use staged_docs__time_travel::*;
#[path = "📥️staging/📄️docs/⚙️settings-and-dock-persistence/🦀️.rs"]
mod staged_docs__settings_and_dock_persistence;
pub(crate) use staged_docs__settings_and_dock_persistence::*;
#[path = "📥️staging/📄️docs/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_docs__theme_file_import_and_accessibility;
pub use staged_docs__theme_file_import_and_accessibility::*;
#[path = "📥️staging/📄️docs/🏛️space-admin-and-settle/🦀️.rs"]
mod staged_docs__space_admin_and_settle;
pub use staged_docs__space_admin_and_settle::*;
#[path = "📥️staging/📄️docs/💬️chrome-dialog-and-tooltips/🦀️.rs"]
mod staged_docs__chrome_dialog_and_tooltips;
#[path = "📥️staging/📄️docs/📎️local-folders/🦀️.rs"]
mod staged_docs__local_folders;
pub(crate) use staged_docs__local_folders::*;
#[path = "📥️staging/📄️docs/📨️command-dispatch/🦀️.rs"]
mod staged_docs__command_dispatch;
#[path = "📥️staging/📄️docs/🔖️identity-and-types/🦀️.rs"]
mod staged_docs__identity_and_types;
pub use staged_docs__identity_and_types::*;
#[path = "📥️staging/📄️docs/🖱️pointer-input/🦀️.rs"]
mod staged_docs__pointer_input;
#[path = "📥️staging/📄️docs/🖼️chrome-frame-and-main-window/🦀️.rs"]
mod staged_docs__chrome_frame_and_main_window;
#[path = "📥️staging/📄️docs/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_docs__chrome_overlays_and_tour;
pub use staged_docs__chrome_overlays_and_tour::*;
#[path = "📥️staging/📄️docs/🧩️session-and-topology/🦀️.rs"]
mod staged_docs__session_and_topology;
#[path = "📥️staging/📄️docs/🧰️window-actions-and-commands/🦀️.rs"]
mod staged_docs__window_actions_and_commands;
pub(crate) use staged_docs__window_actions_and_commands::*;
#[path = "📥️staging/📄️docs/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_docs__chrome_paint_and_navbar;
pub use staged_docs__chrome_paint_and_navbar::*;
#[path = "📥️staging/📄️docs/🧵️shell-async-runners/🦀️.rs"]
mod staged_docs__shell_async_runners;
pub(crate) use staged_docs__shell_async_runners::*;
#[path = "📥️staging/📄️docs/🧾️panel-records/🦀️.rs"]
mod staged_docs__panel_records;
pub(crate) use staged_docs__panel_records::*;
#[path = "📥️staging/📂️doc-open/⌨️keyboard-and-palette/🦀️.rs"]
mod staged_doc_open__keyboard_and_palette;
#[path = "📥️staging/📂️doc-open/⚙️settings-and-dock-persistence/🦀️.rs"]
mod staged_doc_open__settings_and_dock_persistence;
#[path = "📥️staging/📂️doc-open/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_doc_open__theme_file_import_and_accessibility;
pub use staged_doc_open__theme_file_import_and_accessibility::*;
#[path = "📥️staging/📂️doc-open/🎬️shell-actions/🦀️.rs"]
mod staged_doc_open__shell_actions;
#[path = "📥️staging/📂️doc-open/🏛️space-admin-and-settle/🦀️.rs"]
mod staged_doc_open__space_admin_and_settle;
pub use staged_doc_open__space_admin_and_settle::*;
#[path = "📥️staging/📂️doc-open/📤️icon-export/🦀️.rs"]
mod staged_doc_open__icon_export;
pub(crate) use staged_doc_open__icon_export::*;
#[path = "📥️staging/📂️doc-open/📨️command-dispatch/🦀️.rs"]
mod staged_doc_open__command_dispatch;
#[path = "📥️staging/📂️doc-open/🔑️storage-prefs-and-i18n/🦀️.rs"]
mod staged_doc_open__storage_prefs_and_i18n;
pub use staged_doc_open__storage_prefs_and_i18n::*;
#[path = "📥️staging/📂️doc-open/🔖️identity-and-types/🦀️.rs"]
mod staged_doc_open__identity_and_types;
pub use staged_doc_open__identity_and_types::*;
#[path = "📥️staging/📂️doc-open/🖼️chrome-frame-and-main-window/🦀️.rs"]
mod staged_doc_open__chrome_frame_and_main_window;
#[path = "📥️staging/📂️doc-open/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_doc_open__chrome_overlays_and_tour;
#[path = "📥️staging/📂️doc-open/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_doc_open__chrome_paint_and_navbar;
pub use staged_doc_open__chrome_paint_and_navbar::*;
#[path = "📥️staging/🔲️windows/⚓️dock-model-and-measures/🦀️.rs"]
mod staged_windows__dock_model_and_measures;
pub use staged_windows__dock_model_and_measures::*;
#[path = "📥️staging/🔲️windows/⚙️settings-and-dock-persistence/🦀️.rs"]
mod staged_windows__settings_and_dock_persistence;
#[path = "📥️staging/🔲️windows/🏛️space-admin-and-settle/🦀️.rs"]
mod staged_windows__space_admin_and_settle;
pub use staged_windows__space_admin_and_settle::*;
#[path = "📥️staging/🔲️windows/📨️command-dispatch/🦀️.rs"]
mod staged_windows__command_dispatch;
#[path = "📥️staging/🔲️windows/🖱️pointer-input/🦀️.rs"]
mod staged_windows__pointer_input;
pub use staged_windows__pointer_input::*;
#[path = "📥️staging/🔲️windows/🐚️shell-parent/🦀️.rs"]
mod staged_windows__shell_parent;
pub(crate) use staged_windows__shell_parent::*;
#[path = "📥️staging/🔲️windows/🧩️session-and-topology/🦀️.rs"]
mod staged_windows__session_and_topology;
#[path = "📥️staging/🔲️windows/🧰️window-actions-and-commands/🦀️.rs"]
mod staged_windows__window_actions_and_commands;
pub(crate) use staged_windows__window_actions_and_commands::*;
#[path = "📥️staging/🔲️windows/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_windows__chrome_paint_and_navbar;
pub use staged_windows__chrome_paint_and_navbar::*;
#[path = "📥️staging/🔲️windows/🧾️panel-records/🦀️.rs"]
mod staged_windows__panel_records;
pub(crate) use staged_windows__panel_records::*;
#[path = "📥️staging/⚓️dock/⌨️keyboard-and-palette/🦀️.rs"]
mod staged_dock__keyboard_and_palette;
pub(crate) use staged_dock__keyboard_and_palette::*;
#[path = "📥️staging/⚓️dock/⚓️dock-model-and-measures/🦀️.rs"]
mod staged_dock__dock_model_and_measures;
pub use staged_dock__dock_model_and_measures::*;
#[path = "📥️staging/⚓️dock/⚙️settings-and-dock-persistence/🦀️.rs"]
mod staged_dock__settings_and_dock_persistence;
pub(crate) use staged_dock__settings_and_dock_persistence::*;
#[path = "📥️staging/⚓️dock/🎓️tutorial-runtime/🦀️.rs"]
mod staged_dock__tutorial_runtime;
pub use staged_dock__tutorial_runtime::*;
#[path = "📥️staging/⚓️dock/🎬️shell-actions/🦀️.rs"]
mod staged_dock__shell_actions;
#[path = "📥️staging/⚓️dock/💬️chrome-dialog-and-tooltips/🦀️.rs"]
mod staged_dock__chrome_dialog_and_tooltips;
#[path = "📥️staging/⚓️dock/🔑️storage-prefs-and-i18n/🦀️.rs"]
mod staged_dock__storage_prefs_and_i18n;
pub use staged_dock__storage_prefs_and_i18n::*;
#[path = "📥️staging/⚓️dock/🔖️identity-and-types/🦀️.rs"]
mod staged_dock__identity_and_types;
pub use staged_dock__identity_and_types::*;
#[path = "📥️staging/⚓️dock/🖱️pointer-input/🦀️.rs"]
mod staged_dock__pointer_input;
pub(crate) use staged_dock__pointer_input::*;
#[path = "📥️staging/⚓️dock/🖼️chrome-frame-and-main-window/🦀️.rs"]
mod staged_dock__chrome_frame_and_main_window;
#[path = "📥️staging/⚓️dock/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_dock__chrome_overlays_and_tour;
#[path = "📥️staging/⚓️dock/🐚️shell-parent/🦀️.rs"]
mod staged_dock__shell_parent;
pub(crate) use staged_dock__shell_parent::*;
#[path = "📥️staging/⚓️dock/🧩️session-and-topology/🦀️.rs"]
mod staged_dock__session_and_topology;
#[path = "📥️staging/⚓️dock/🧰️window-actions-and-commands/🦀️.rs"]
mod staged_dock__window_actions_and_commands;
#[path = "📥️staging/⚓️dock/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_dock__chrome_paint_and_navbar;
pub use staged_dock__chrome_paint_and_navbar::*;
#[path = "📥️staging/⚓️dock/🧾️panel-records/🦀️.rs"]
mod staged_dock__panel_records;
pub(crate) use staged_dock__panel_records::*;
#[path = "📥️staging/⚓️dock/🪟️tree-windows/🦀️.rs"]
mod staged_dock__tree_windows;
pub(crate) use staged_dock__tree_windows::*;
#[path = "📥️staging/🕹️input/⌨️keyboard-and-palette/🦀️.rs"]
mod staged_input__keyboard_and_palette;
#[path = "📥️staging/🕹️input/⏪️time-travel/🦀️.rs"]
mod staged_input__time_travel;
pub(crate) use staged_input__time_travel::*;
#[path = "📥️staging/🕹️input/⚙️settings-and-dock-persistence/🦀️.rs"]
mod staged_input__settings_and_dock_persistence;
#[path = "📥️staging/🕹️input/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_input__theme_file_import_and_accessibility;
pub use staged_input__theme_file_import_and_accessibility::*;
#[path = "📥️staging/🕹️input/🏛️space-admin-and-settle/🦀️.rs"]
mod staged_input__space_admin_and_settle;
pub use staged_input__space_admin_and_settle::*;
#[path = "📥️staging/🕹️input/💬️chrome-dialog-and-tooltips/🦀️.rs"]
mod staged_input__chrome_dialog_and_tooltips;
#[path = "📥️staging/🕹️input/🔑️storage-prefs-and-i18n/🦀️.rs"]
mod staged_input__storage_prefs_and_i18n;
pub use staged_input__storage_prefs_and_i18n::*;
#[path = "📥️staging/🕹️input/🔖️identity-and-types/🦀️.rs"]
mod staged_input__identity_and_types;
pub use staged_input__identity_and_types::*;
#[path = "📥️staging/🕹️input/🖱️pointer-input/🦀️.rs"]
mod staged_input__pointer_input;
pub use staged_input__pointer_input::*;
#[path = "📥️staging/🕹️input/🖼️chrome-frame-and-main-window/🦀️.rs"]
mod staged_input__chrome_frame_and_main_window;
pub(crate) use staged_input__chrome_frame_and_main_window::*;
#[path = "📥️staging/🕹️input/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_input__chrome_overlays_and_tour;
pub use staged_input__chrome_overlays_and_tour::*;
#[path = "📥️staging/🕹️input/🧩️session-and-topology/🦀️.rs"]
mod staged_input__session_and_topology;
#[path = "📥️staging/🕹️input/🧰️window-actions-and-commands/🦀️.rs"]
mod staged_input__window_actions_and_commands;
pub(crate) use staged_input__window_actions_and_commands::*;
#[path = "📥️staging/🕹️input/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_input__chrome_paint_and_navbar;
pub use staged_input__chrome_paint_and_navbar::*;
#[path = "📥️staging/🕹️input/🧾️panel-records/🦀️.rs"]
mod staged_input__panel_records;
pub(crate) use staged_input__panel_records::*;
#[path = "📥️staging/🎹️commands/⌨️keyboard-and-palette/🦀️.rs"]
mod staged_commands__keyboard_and_palette;
#[path = "📥️staging/🎹️commands/🧩️session-and-topology/🦀️.rs"]
mod staged_commands__session_and_topology;
#[path = "📥️staging/🎹️commands/🧰️window-actions-and-commands/🦀️.rs"]
mod staged_commands__window_actions_and_commands;
pub(crate) use staged_commands__window_actions_and_commands::*;
#[path = "📥️staging/🐙️state/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_state__theme_file_import_and_accessibility;
pub use staged_state__theme_file_import_and_accessibility::*;
#[path = "📥️staging/🐙️state/🔖️identity-and-types/🦀️.rs"]
mod staged_state__identity_and_types;
pub use staged_state__identity_and_types::*;
#[path = "📥️staging/🐙️state/🧩️session-and-topology/🦀️.rs"]
mod staged_state__session_and_topology;
#[path = "📥️staging/🚶️session/⌨️keyboard-and-palette/🦀️.rs"]
mod staged_session__keyboard_and_palette;
#[path = "📥️staging/🚶️session/⏪️time-travel/🦀️.rs"]
mod staged_session__time_travel;
pub use staged_session__time_travel::*;
#[path = "📥️staging/🚶️session/⚓️dock-model-and-measures/🦀️.rs"]
mod staged_session__dock_model_and_measures;
pub use staged_session__dock_model_and_measures::*;
#[path = "📥️staging/🚶️session/⚙️settings-and-dock-persistence/🦀️.rs"]
mod staged_session__settings_and_dock_persistence;
pub use staged_session__settings_and_dock_persistence::*;
#[path = "📥️staging/🚶️session/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_session__theme_file_import_and_accessibility;
pub use staged_session__theme_file_import_and_accessibility::*;
#[path = "📥️staging/🚶️session/🎬️shell-actions/🦀️.rs"]
mod staged_session__shell_actions;
#[path = "📥️staging/🚶️session/🏛️space-admin-and-settle/🦀️.rs"]
mod staged_session__space_admin_and_settle;
pub use staged_session__space_admin_and_settle::*;
#[path = "📥️staging/🚶️session/🐚️shell-parent/🦀️.rs"]
mod staged_session__shell_parent;
pub use staged_session__shell_parent::*;
#[path = "📥️staging/🚶️session/💬️chrome-dialog-and-tooltips/🦀️.rs"]
mod staged_session__chrome_dialog_and_tooltips;
#[path = "📥️staging/🚶️session/📎️local-folders/🦀️.rs"]
mod staged_session__local_folders;
#[path = "📥️staging/🚶️session/📨️command-dispatch/🦀️.rs"]
mod staged_session__command_dispatch;
#[path = "📥️staging/🚶️session/🔖️identity-and-types/🦀️.rs"]
mod staged_session__identity_and_types;
pub use staged_session__identity_and_types::*;
#[path = "📥️staging/🚶️session/🖱️pointer-input/🦀️.rs"]
mod staged_session__pointer_input;
#[path = "📥️staging/🚶️session/🖼️chrome-frame-and-main-window/🦀️.rs"]
mod staged_session__chrome_frame_and_main_window;
#[path = "📥️staging/🚶️session/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_session__chrome_overlays_and_tour;
pub use staged_session__chrome_overlays_and_tour::*;
#[path = "📥️staging/🚶️session/🧩️session-and-topology/🦀️.rs"]
mod staged_session__session_and_topology;
pub use staged_session__session_and_topology::*;
#[path = "📥️staging/🚶️session/🧰️window-actions-and-commands/🦀️.rs"]
mod staged_session__window_actions_and_commands;
pub use staged_session__window_actions_and_commands::*;
#[path = "📥️staging/🚶️session/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_session__chrome_paint_and_navbar;
pub use staged_session__chrome_paint_and_navbar::*;
#[path = "📥️staging/🚶️session/🧺️bundle/🦀️.rs"]
mod staged_session__bundle;
pub use staged_session__bundle::*;
#[path = "📥️staging/🚶️session/🧾️panel-records/🦀️.rs"]
mod staged_session__panel_records;
pub use staged_session__panel_records::*;
#[path = "📥️staging/🚶️session/🪟️tree-windows/🦀️.rs"]
mod staged_session__tree_windows;
#[path = "📥️staging/🪝️window-actions/⚓️dock-model-and-measures/🦀️.rs"]
mod staged_window_actions__dock_model_and_measures;
pub use staged_window_actions__dock_model_and_measures::*;
#[path = "📥️staging/🪝️window-actions/⚙️settings-and-dock-persistence/🦀️.rs"]
mod staged_window_actions__settings_and_dock_persistence;
#[path = "📥️staging/🪝️window-actions/🎓️tutorial-runtime/🦀️.rs"]
mod staged_window_actions__tutorial_runtime;
pub use staged_window_actions__tutorial_runtime::*;
#[path = "📥️staging/🪝️window-actions/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_window_actions__theme_file_import_and_accessibility;
pub use staged_window_actions__theme_file_import_and_accessibility::*;
#[path = "📥️staging/🪝️window-actions/🎬️shell-actions/🦀️.rs"]
mod staged_window_actions__shell_actions;
pub use staged_window_actions__shell_actions::*;
#[path = "📥️staging/🪝️window-actions/🏛️space-admin-and-settle/🦀️.rs"]
mod staged_window_actions__space_admin_and_settle;
pub use staged_window_actions__space_admin_and_settle::*;
#[path = "📥️staging/🪝️window-actions/📎️local-folders/🦀️.rs"]
mod staged_window_actions__local_folders;
pub use staged_window_actions__local_folders::*;
#[path = "📥️staging/🪝️window-actions/📨️command-dispatch/🦀️.rs"]
mod staged_window_actions__command_dispatch;
#[path = "📥️staging/🪝️window-actions/🔑️storage-prefs-and-i18n/🦀️.rs"]
mod staged_window_actions__storage_prefs_and_i18n;
pub use staged_window_actions__storage_prefs_and_i18n::*;
#[path = "📥️staging/🪝️window-actions/🔖️identity-and-types/🦀️.rs"]
mod staged_window_actions__identity_and_types;
pub use staged_window_actions__identity_and_types::*;
#[path = "📥️staging/🪝️window-actions/🖱️pointer-input/🦀️.rs"]
mod staged_window_actions__pointer_input;
pub use staged_window_actions__pointer_input::*;
#[path = "📥️staging/🪝️window-actions/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_window_actions__chrome_overlays_and_tour;
pub use staged_window_actions__chrome_overlays_and_tour::*;
#[path = "📥️staging/🪝️window-actions/🧩️session-and-topology/🦀️.rs"]
mod staged_window_actions__session_and_topology;
#[path = "📥️staging/🪝️window-actions/🧰️window-actions-and-commands/🦀️.rs"]
mod staged_window_actions__window_actions_and_commands;
pub use staged_window_actions__window_actions_and_commands::*;
#[path = "📥️staging/🪝️window-actions/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_window_actions__chrome_paint_and_navbar;
pub use staged_window_actions__chrome_paint_and_navbar::*;
#[path = "📥️staging/🪝️window-actions/🧵️shell-async-runners/🦀️.rs"]
mod staged_window_actions__shell_async_runners;
pub use staged_window_actions__shell_async_runners::*;
#[path = "📥️staging/🪝️window-actions/🧺️bundle/🦀️.rs"]
mod staged_window_actions__bundle;
pub use staged_window_actions__bundle::*;
#[path = "📥️staging/🪝️window-actions/🧾️panel-records/🦀️.rs"]
mod staged_window_actions__panel_records;
pub use staged_window_actions__panel_records::*;
#[path = "📥️staging/🔀️actions/⌨️keyboard-and-palette/🦀️.rs"]
mod staged_actions__keyboard_and_palette;
#[path = "📥️staging/🔀️actions/⏪️time-travel/🦀️.rs"]
mod staged_actions__time_travel;
#[path = "📥️staging/🔀️actions/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_actions__theme_file_import_and_accessibility;
pub use staged_actions__theme_file_import_and_accessibility::*;
#[path = "📥️staging/🔀️actions/🎬️shell-actions/🦀️.rs"]
mod staged_actions__shell_actions;
pub use staged_actions__shell_actions::*;
#[path = "📥️staging/🔀️actions/🏛️space-admin-and-settle/🦀️.rs"]
mod staged_actions__space_admin_and_settle;
pub use staged_actions__space_admin_and_settle::*;
#[path = "📥️staging/🔀️actions/📎️local-folders/🦀️.rs"]
mod staged_actions__local_folders;
#[path = "📥️staging/🔀️actions/📨️command-dispatch/🦀️.rs"]
mod staged_actions__command_dispatch;
#[path = "📥️staging/🔀️actions/🔖️identity-and-types/🦀️.rs"]
mod staged_actions__identity_and_types;
pub use staged_actions__identity_and_types::*;
#[path = "📥️staging/🔀️actions/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_actions__chrome_overlays_and_tour;
pub use staged_actions__chrome_overlays_and_tour::*;
#[path = "📥️staging/🔀️actions/🧩️session-and-topology/🦀️.rs"]
mod staged_actions__session_and_topology;
#[path = "📥️staging/🔀️actions/🧰️window-actions-and-commands/🦀️.rs"]
mod staged_actions__window_actions_and_commands;
pub use staged_actions__window_actions_and_commands::*;
#[path = "📥️staging/🔀️actions/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_actions__chrome_paint_and_navbar;
pub use staged_actions__chrome_paint_and_navbar::*;
#[path = "📥️staging/🔀️actions/🧺️bundle/🦀️.rs"]
mod staged_actions__bundle;
pub use staged_actions__bundle::*;
#[path = "📥️staging/🔀️actions/🧾️panel-records/🦀️.rs"]
mod staged_actions__panel_records;
pub use staged_actions__panel_records::*;
#[path = "📥️staging/🖐️pointer/⌨️keyboard-and-palette/🦀️.rs"]
mod staged_pointer__keyboard_and_palette;
pub use staged_pointer__keyboard_and_palette::*;
#[path = "📥️staging/🖐️pointer/🎓️tutorial-runtime/🦀️.rs"]
mod staged_pointer__tutorial_runtime;
#[path = "📥️staging/🖐️pointer/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_pointer__theme_file_import_and_accessibility;
#[path = "📥️staging/🖐️pointer/🎬️shell-actions/🦀️.rs"]
mod staged_pointer__shell_actions;
#[path = "📥️staging/🖐️pointer/🏛️space-admin-and-settle/🦀️.rs"]
mod staged_pointer__space_admin_and_settle;
pub use staged_pointer__space_admin_and_settle::*;
#[path = "📥️staging/🖐️pointer/📨️command-dispatch/🦀️.rs"]
mod staged_pointer__command_dispatch;
#[path = "📥️staging/🖐️pointer/🔑️storage-prefs-and-i18n/🦀️.rs"]
mod staged_pointer__storage_prefs_and_i18n;
pub use staged_pointer__storage_prefs_and_i18n::*;
#[path = "📥️staging/🖐️pointer/🔖️identity-and-types/🦀️.rs"]
mod staged_pointer__identity_and_types;
pub use staged_pointer__identity_and_types::*;
#[path = "📥️staging/🖐️pointer/🖱️pointer-input/🦀️.rs"]
mod staged_pointer__pointer_input;
pub use staged_pointer__pointer_input::*;
#[path = "📥️staging/🖐️pointer/🧩️session-and-topology/🦀️.rs"]
mod staged_pointer__session_and_topology;
#[path = "📥️staging/🖐️pointer/🧰️window-actions-and-commands/🦀️.rs"]
mod staged_pointer__window_actions_and_commands;
pub use staged_pointer__window_actions_and_commands::*;
#[path = "📥️staging/🖐️pointer/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_pointer__chrome_paint_and_navbar;
pub use staged_pointer__chrome_paint_and_navbar::*;
#[path = "📥️staging/🖐️pointer/🧵️shell-async-runners/🦀️.rs"]
mod staged_pointer__shell_async_runners;
pub use staged_pointer__shell_async_runners::*;
#[path = "📥️staging/🖐️pointer/🧺️bundle/🦀️.rs"]
mod staged_pointer__bundle;
pub use staged_pointer__bundle::*;
#[path = "📥️staging/🖍️view-paint/⚓️dock-model-and-measures/🦀️.rs"]
mod staged_view_paint__dock_model_and_measures;
pub use staged_view_paint__dock_model_and_measures::*;
#[path = "📥️staging/🖍️view-paint/🎓️tutorial-runtime/🦀️.rs"]
mod staged_view_paint__tutorial_runtime;
pub use staged_view_paint__tutorial_runtime::*;
#[path = "📥️staging/🖍️view-paint/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_view_paint__theme_file_import_and_accessibility;
pub use staged_view_paint__theme_file_import_and_accessibility::*;
#[path = "📥️staging/🖍️view-paint/🏛️space-admin-and-settle/🦀️.rs"]
mod staged_view_paint__space_admin_and_settle;
pub use staged_view_paint__space_admin_and_settle::*;
#[path = "📥️staging/🖍️view-paint/🔑️storage-prefs-and-i18n/🦀️.rs"]
mod staged_view_paint__storage_prefs_and_i18n;
pub use staged_view_paint__storage_prefs_and_i18n::*;
#[path = "📥️staging/🖍️view-paint/🔖️identity-and-types/🦀️.rs"]
mod staged_view_paint__identity_and_types;
pub use staged_view_paint__identity_and_types::*;
#[path = "📥️staging/🖍️view-paint/🖼️chrome-frame-and-main-window/🦀️.rs"]
mod staged_view_paint__chrome_frame_and_main_window;
pub use staged_view_paint__chrome_frame_and_main_window::*;
#[path = "📥️staging/🖍️view-paint/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_view_paint__chrome_overlays_and_tour;
pub use staged_view_paint__chrome_overlays_and_tour::*;
#[path = "📥️staging/🖍️view-paint/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_view_paint__chrome_paint_and_navbar;
pub use staged_view_paint__chrome_paint_and_navbar::*;
#[path = "📥️staging/🍱️view-chrome/⚓️dock-model-and-measures/🦀️.rs"]
mod staged_view_chrome__dock_model_and_measures;
#[path = "📥️staging/🍱️view-chrome/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_view_chrome__theme_file_import_and_accessibility;
pub use staged_view_chrome__theme_file_import_and_accessibility::*;
#[path = "📥️staging/🍱️view-chrome/🏛️space-admin-and-settle/🦀️.rs"]
mod staged_view_chrome__space_admin_and_settle;
pub use staged_view_chrome__space_admin_and_settle::*;
#[path = "📥️staging/🍱️view-chrome/🖱️pointer-input/🦀️.rs"]
mod staged_view_chrome__pointer_input;
#[path = "📥️staging/🍱️view-chrome/🖼️chrome-frame-and-main-window/🦀️.rs"]
mod staged_view_chrome__chrome_frame_and_main_window;
pub use staged_view_chrome__chrome_frame_and_main_window::*;
#[path = "📥️staging/🍱️view-chrome/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_view_chrome__chrome_overlays_and_tour;
pub use staged_view_chrome__chrome_overlays_and_tour::*;
#[path = "📥️staging/🍱️view-chrome/🧭️navbar-footer-and-overlays/🦀️.rs"]
mod staged_view_chrome__navbar_footer_and_overlays;
#[path = "📥️staging/🍱️view-chrome/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_view_chrome__chrome_paint_and_navbar;
pub use staged_view_chrome__chrome_paint_and_navbar::*;
#[path = "📥️staging/🍱️view-chrome/🧺️bundle/🦀️.rs"]
mod staged_view_chrome__bundle;
pub use staged_view_chrome__bundle::*;
#[path = "📥️staging/🗨️view-overlays/⏪️time-travel/🦀️.rs"]
mod staged_view_overlays__time_travel;
pub use staged_view_overlays__time_travel::*;
#[path = "📥️staging/🗨️view-overlays/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_view_overlays__theme_file_import_and_accessibility;
pub use staged_view_overlays__theme_file_import_and_accessibility::*;
#[path = "📥️staging/🗨️view-overlays/💬️chrome-dialog-and-tooltips/🦀️.rs"]
mod staged_view_overlays__chrome_dialog_and_tooltips;
pub use staged_view_overlays__chrome_dialog_and_tooltips::*;
#[path = "📥️staging/🗨️view-overlays/📎️local-folders/🦀️.rs"]
mod staged_view_overlays__local_folders;
pub use staged_view_overlays__local_folders::*;
#[path = "📥️staging/🗨️view-overlays/🔖️identity-and-types/🦀️.rs"]
mod staged_view_overlays__identity_and_types;
pub use staged_view_overlays__identity_and_types::*;
#[path = "📥️staging/🗨️view-overlays/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_view_overlays__chrome_overlays_and_tour;
pub use staged_view_overlays__chrome_overlays_and_tour::*;
#[path = "📥️staging/🗨️view-overlays/🧭️navbar-footer-and-overlays/🦀️.rs"]
mod staged_view_overlays__navbar_footer_and_overlays;
#[path = "📥️staging/🗨️view-overlays/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_view_overlays__chrome_paint_and_navbar;
pub use staged_view_overlays__chrome_paint_and_navbar::*;
#[path = "📥️staging/🗨️view-overlays/🧺️bundle/🦀️.rs"]
mod staged_view_overlays__bundle;
pub use staged_view_overlays__bundle::*;
#[path = "📥️staging/🪀️view-panels/⌨️keyboard-and-palette/🦀️.rs"]
mod staged_view_panels__keyboard_and_palette;
#[path = "📥️staging/🪀️view-panels/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_view_panels__theme_file_import_and_accessibility;
pub use staged_view_panels__theme_file_import_and_accessibility::*;
#[path = "📥️staging/🪀️view-panels/🎬️shell-actions/🦀️.rs"]
mod staged_view_panels__shell_actions;
#[path = "📥️staging/🪀️view-panels/📤️icon-export/🦀️.rs"]
mod staged_view_panels__icon_export;
#[path = "📥️staging/🪀️view-panels/🔖️identity-and-types/🦀️.rs"]
mod staged_view_panels__identity_and_types;
pub use staged_view_panels__identity_and_types::*;
#[path = "📥️staging/🪀️view-panels/🖱️pointer-input/🦀️.rs"]
mod staged_view_panels__pointer_input;
#[path = "📥️staging/🪀️view-panels/🖼️chrome-frame-and-main-window/🦀️.rs"]
mod staged_view_panels__chrome_frame_and_main_window;
pub use staged_view_panels__chrome_frame_and_main_window::*;
#[path = "📥️staging/🪀️view-panels/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_view_panels__chrome_overlays_and_tour;
pub use staged_view_panels__chrome_overlays_and_tour::*;
#[path = "📥️staging/🪀️view-panels/🧭️navbar-footer-and-overlays/🦀️.rs"]
mod staged_view_panels__navbar_footer_and_overlays;
#[path = "📥️staging/🪀️view-panels/🧰️window-actions-and-commands/🦀️.rs"]
mod staged_view_panels__window_actions_and_commands;
#[path = "📥️staging/🪀️view-panels/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_view_panels__chrome_paint_and_navbar;
pub use staged_view_panels__chrome_paint_and_navbar::*;
#[path = "📥️staging/🪀️view-panels/🧺️bundle/🦀️.rs"]
mod staged_view_panels__bundle;
pub use staged_view_panels__bundle::*;
pub(crate) use semio_framework_os_renderer_wgpu_host_base::host_io_call;
#[cfg(test)]
#[path = "../../🧪️tests/🧰️section-fixtures/🦀️.rs"]
pub(crate) mod section_fixtures;
pub(crate) use shell_vocab::shell_chrome_string;
//#endregion 📥️RendererWp4Staging

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

//#region ShellTypes

/// 🐙️ The shell state: exactly the fifteen component structs, which own the fields and the methods of the former god object.
pub struct ShellState {
    pub chrome: ChromeState,
    pub surfaces: SurfaceState,
    pub tutorial: TutorialState,
    pub settle: SettleState,
    pub plugins: PluginState,
    pub history: HistoryState,
    pub prefs: PrefsState,
    pub directory: DirectoryState,
    pub sync: SyncState,
    pub docs: DocumentState,
    pub opening: OpeningState,
    pub windows: WindowState,
    pub dock: ShellDockState,
    pub input: ShellInputState,
    pub commands: CommandState,
}
//#endregion ShellTypes

#[cfg(not(target_arch = "wasm32"))]
impl Drop for ShellState {
    fn drop(&mut self) {
        self.directory.directory_cancel.cancel_now();
        if let Some(task) = self.directory.identity_bootstrap_task.take() {
            task.cancel();
        }
        if let Some(mut home) = self.directory.directory_home.take() {
            if let Some((plugin_id, instance_id)) = home.take_destroy_authority() {
                if let Some(program) = self.plugins.plugins.iter().find(|entry| entry.plugin_id == plugin_id) {
                    program.destroy_app(instance_id);
                }
            }
        }
        for pending in &self.settle.shell_io_pending {
            if let Some(task) = &pending.task {
                task.cancel();
            }
        }
    }
}
