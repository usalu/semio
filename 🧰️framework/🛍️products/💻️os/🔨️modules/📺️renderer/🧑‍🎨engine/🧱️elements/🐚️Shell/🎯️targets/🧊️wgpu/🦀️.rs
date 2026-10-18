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
#[cfg(target_arch = "wasm32")]
pub(crate) use semio_framework_os_renderer_wgpu_host_base::host_io_call;
pub(crate) use shell_vocab::shell_chrome_string;
//#endregion 📥️RendererWp4Staging

//#region ShellLifecycle
//#endregion ShellLifecycle

//#region ShellInput

//#endregion ShellInput

//#region ActionPanelAndUtilities
//#region 🪟️WindowPaneChrome
//#endregion 🪟️WindowPaneChrome

//#region 🎬️WindowActionsAndSearchPanes
//#endregion 🎬️WindowActionsAndSearchPanes
//#endregion ActionPanelAndUtilities

//#region ShellChrome
//#region 🌓️AppearanceTourAndFooterPillTests
//#endregion 🌓️AppearanceTourAndFooterPillTests
//#endregion ShellChrome
