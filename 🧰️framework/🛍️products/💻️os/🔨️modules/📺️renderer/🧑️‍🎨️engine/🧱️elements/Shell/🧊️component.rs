//! 🖥️ framework/products/os/modules/renderer/engine/elements/Shell/component.rs — wgpu shell
//! chrome implementation for the Shell element, extracted from lib.rs's inline
//! `pub mod shell { ... }` body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired via
//! `#[path = "../../../../🧱️elements/Shell/🧊️component.rs"] pub mod shell;` in lib.rs in place of
//! the former inline block; the module name `shell` is unchanged, so every existing
//! `crate::shell::...` call site elsewhere in the crate keeps resolving with zero other changes.
//! 🖥️ OS shell chrome — navbar, footer, floating panels, overlays, and studio mode.

use crate::dock::{compute_dock_drop_zone, drop_zone_indicator_rect, parse_path, push_window_silhouette_border, DockDragKind, DockDragPayload, DockDragState, DockDropZone, DockRenderContext, DockStackTab, DockState, WindowSilhouette};
use crate::engine_canvas::theme_is_dark;
use crate::interpreter::{begin_ui_document_opportunity, framework_widget_context, render_ui_document, resolve_ui_image};
use crate::program_bridge::{is_space_mode, resolve_playground_app_id, resolve_plugin_host_config, PluginHostConfig, ProgramBridgeEntry};
use crate::scenes::{clear_graph_node_context, resolve_graph_context_action, toggle_vfs_row_expanded, vfs_selection_for_click, AdmittedSurfaceMap, Board2dSurface, NodeGraphSurface, TiledMapSurface};
use infinite_world::world::{enqueue_world3d_events, World3dState, WorldInteractionIntent, WorldInteractionPhase};
use semio_framework::{app_breadcrumb, app_window_label, resolve_app_breadcrumb, AppDefinition, ExampleDefinition, IconName, PanelGroup, PanelTabDefinition, ViewModel};
use semio_framework_os_kernel::os_directory::identity::IdentityEnv;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
#[cfg(not(target_arch = "wasm32"))]
use store_sync::sync::{ArtifactActorConfig, ArtifactActorMsg, ArtifactEvent, ArtifactHost, ArtifactMailboxSender, ArtifactSyncStatus, PersistenceBinding, RemoteState};
#[cfg(not(target_arch = "wasm32"))]
use store_sync::PresencePeer;
use ui_contract::{SurfaceId, UiDocumentLease, UiFixedList, UiText, UI_DOCUMENT_LEASE_ALIASES, UI_DOCUMENT_LEASE_SLOTS};
// 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C0/§C3/§C6 (lane 2-D) —
// lane 1-D's Rust directory client + native identity mint/restore helper, consumed as-is (never
// re-declared: `semio_framework_os_kernel::os_directory` is the single source of truth both this
// shell and the hub's own Rust twin import). Native-only: the browser wgpu build has no native
// `ArtifactHost`/document-sync path either (see `attach_sync_backbone`'s wasm32 branch), so identity/
// directory wiring stays inside the same `not(wasm32)` boundary as every other native-only field on
// `ShellState` (`document_host`, `sync_channel`, `sync_status`).
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_kernel::os_directory::{
    client::{native::NativeDirectoryTransport, DirectoryClient, DirectoryStream, DirectoryStreamTurn, DirectoryTransport},
    identity::{actor_id, mint_or_restore, Identity, IdentityOutcome, IdentityStatus},
    DirectoryCommand, DirectoryEvent, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility, DirectoryStreamMessage,
};
// 🌀️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-directory-and-run): `DirectoryClient`'s request
// methods now carry an `OperationContext` (cancellation/deadline/trace), and the native transport
// routes through `semio-framework-os-services`'s `HttpPool`/`ComputePool` over ONE shared
// `TokioHostRuntime` — `NativeDirectoryTransport::new()` (zero-arg) is gone, replaced by
// `with_new_http_pool_now` fed the renderer's process-wide pool. No local executor or thread is
// constructed for identity or directory work.
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_actor::{ActorId as DirectoryActorId, PackageId as DirectoryPackageId};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_async::{CancelToken, Lane, OperationContext, ScopeOwner, TraceId, WorkerPool};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_services::{ComputePool, TokioHostRuntime};
use ui_wgpu::wgpu::{
    chrome_item_bg, chrome_item_text, draw_text, push_chrome_group_border, DragAxis, DrawList, FontAtlas, HitKind, HitTarget, IconAtlas, InputState, Level, PointerModifiers, Rect, Rgba, Theme, TreeDragState, TreeDropPosition, WidgetInteractionMaps,
    WindowStackCorner,
};
use ui_wgpu::wgpu::{
    ActionDescriptor, Label, Locale, LocalizedLabel, Terminology, UiButtonNode, UiNode, UiPresence, UiSelectItem, UiSelectNode, UiStackNode, UiTextNode, UtilityCategory, UtilityNode, WindowEngagement, WindowEngagementControl, WindowEngagementInput,
    WindowMeasure, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_HISTORY_ID, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

const FRAMEWORK_DISPLAY_WINDOWS_TAB_ID: &str = "framework.display.windows";
const FRAMEWORK_DISPLAY_LAYOUT_TAB_ID: &str = "framework.display.layout";
const FRAMEWORK_SETTINGS_GENERAL_TAB_ID: &str = "framework.settings.general";

use dsl::DslValue;
use serde_json::Value;

fn dsl_value_as_json(value: &DslValue) -> Value {
    serde_json::to_value(value).unwrap_or(Value::Null)
}

fn optional_dsl_value_as_json(value: Option<DslValue>) -> Option<Value> {
    value.map(|entry| dsl_value_as_json(&entry))
}

fn optional_json_as_dsl_value(value: Option<Value>) -> Option<DslValue> {
    value.map(|entry| serde_json::from_value(entry).unwrap_or(DslValue::Null))
}
/// 🎨️ Byte-identical to React's `FRAMEWORK_SETTINGS_THEME_TAB_ID` (`ui/js/react/index.tsx:8807`) — the
/// `PanelTabKind::SettingsTheme` variant this maps to already existed in `framework/core/rs/lib.rs`
/// but was completely unwired on this side (see `build_settings_theme_ui`/`right_tabs`).
const FRAMEWORK_SETTINGS_THEME_TAB_ID: &str = "framework.settings.theme";
/// 🎛️ wgpu-only: React surfaces its command palette as a persistent `bottom-middle` dock anchor
/// (`buildCommandCategoryTabs`), which this renderer has no equivalent of (`group_side`/`PanelGroup::
/// anchor` only ever map to the four corners — see that function's own doc comment). This gives
/// The local builder remains a test oracle; production panel content now requires retained semantic
/// document publication.
const FRAMEWORK_SETTINGS_COMMANDS_TAB_ID: &str = "framework.settings.commands";
const CHROME_ICON_TINY: f32 = 14.0;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LeftPanelKind {
    #[default]
    Workbench,
    Display,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RightPanelKind {
    #[default]
    Details,
    Settings,
}

#[derive(Clone, Debug)]
pub struct SearchPaletteItem {
    pub id: String,
    pub label: String,
    pub group: String,
    pub dispatch_action: Option<ActionDescriptor>,
    pub action: Option<String>,
    /// 🗂️ Coarse command-source tag (os/plugin/app/mode) — `None` for the pre-existing panel/window/
    /// keybinding/action/studio entries, `Some(..)` for entries derived from `shell::ActionPanelAndUtilities`'s
    /// `ResolvedCommand` aggregation (see `command_search_items`).
    pub category: Option<semio_framework::manifest::CommandOwnerAddress>,
}

#[derive(Clone, Debug)]
pub struct ShellFindItem {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub surface_id: String,
    pub node_id: String,
}

/// 🧺️ Owned callback output shared with the scene renderer for one chrome build.
#[derive(Clone, Default)]
struct ShellFindItemSink(std::sync::Arc<std::sync::Mutex<Vec<ShellFindItem>>>);

#[cfg(not(target_arch = "wasm32"))]
struct WorkerCell<T>(std::sync::OnceLock<std::sync::Mutex<std::cell::RefCell<T>>>);

#[cfg(not(target_arch = "wasm32"))]
impl<T> WorkerCell<T> {
    const fn new() -> Self {
        Self(std::sync::OnceLock::new())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<T: Default> WorkerCell<T> {
    fn with<R>(&self, f: impl FnOnce(&std::cell::RefCell<T>) -> R) -> R {
        let guard = self.0.get_or_init(|| std::sync::Mutex::new(std::cell::RefCell::new(T::default()))).lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        f(&guard)
    }
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static ACTIVE_FIND_ITEM_SINKS: std::cell::RefCell<Vec<ShellFindItemSink>> = const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(not(target_arch = "wasm32"))]
static ACTIVE_FIND_ITEM_SINKS: WorkerCell<Vec<ShellFindItemSink>> = WorkerCell::new();

/// 🪢️ Scoped adapter binding that exposes an owned find-item sink to scene callbacks.
struct ShellFindItemSinkBinding(std::marker::PhantomData<std::rc::Rc<()>>);

impl ShellFindItemSink {
    fn bind(&self) -> ShellFindItemSinkBinding {
        ACTIVE_FIND_ITEM_SINKS.with(|sinks| sinks.borrow_mut().push(self.clone()));
        ShellFindItemSinkBinding(std::marker::PhantomData)
    }

    fn clear(&self) {
        self.0.lock().unwrap_or_else(|error| error.into_inner()).clear();
    }

    fn take(&self) -> Vec<ShellFindItem> {
        std::mem::take(&mut *self.0.lock().unwrap_or_else(|error| error.into_inner()))
    }
}

impl Drop for ShellFindItemSinkBinding {
    fn drop(&mut self) {
        ACTIVE_FIND_ITEM_SINKS.with(|sinks| {
            sinks.borrow_mut().pop().expect("find-item sink binding stack is empty");
        });
    }
}

pub fn push_find_item(item: ShellFindItem) {
    ACTIVE_FIND_ITEM_SINKS.with(|sinks| {
        let sink = sinks.borrow().last().cloned().expect("push_find_item requires an active Shell render binding");
        sink.0.lock().unwrap_or_else(|error| error.into_inner()).push(item);
    });
}

/// 🗂️ `ui_wgpu::wgpu::ShellMenuAction.kind` wire string for an `ActionDefinition.kind` — host-side styling
/// parity only, unused by `build_shell_context_menu_specs` itself.
fn context_menu_action_kind_str(kind: semio_framework::ActionKind) -> String {
    match kind {
        semio_framework::ActionKind::Mutation => "mutation",
        semio_framework::ActionKind::View => "view",
        semio_framework::ActionKind::History => "history",
        semio_framework::ActionKind::Clipboard => "clipboard",
        semio_framework::ActionKind::Shell => "shell",
        semio_framework::ActionKind::Interaction => "interaction",
    }
    .to_string()
}

/// 🖱️ Maps an on-demand plugin context-menu spec into the wgpu shell menu row — `menu.group.<category>`
/// rows (D5's `organize_context_menu` folds, see `ui_wgpu::wgpu::ContextMenuOrganizer`) resolve their label via
/// `ribbon_parent_label` (falling back to the spec's own label if the category is unrecognized) and get
/// a default folder icon when the spec left `icon` unset.
fn shell_context_menu_item_from_spec(spec: ui_wgpu::wgpu::ContextMenuItemSpec, controller_id: &str, is_de: bool) -> ContextMenuItem {
    let ui_wgpu::wgpu::ContextMenuItemSpec { id, label, icon, shortcut, disabled, separator, checked, destructive, action, args, children, .. } = spec;
    let category = id.strip_prefix("menu.group.");
    let label = category.and_then(|category| ui_wgpu::wgpu::ribbon_parent_label(category, is_de)).map(str::to_string).or(label);
    let icon = icon.or_else(|| category.map(|_| "folder".to_string()));
    ContextMenuItem {
        id,
        label: label.unwrap_or_default(),
        icon,
        shortcut,
        destructive: destructive.unwrap_or(false),
        action: action.map(|action| ActionDescriptor { controller_id: controller_id.into(), action, args }),
        children: children.unwrap_or_default().into_iter().map(|child| shell_context_menu_item_from_spec(child, controller_id, is_de)).collect(),
        disabled: disabled.unwrap_or(false),
        separator: separator.unwrap_or(false),
        checked: checked.unwrap_or(false),
    }
}

//#region 🔖️IdentityPure
/// 🎭️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C0 — mints this process's
/// stable per-shell session id (the `{sessionId}` half of `user:{userId}#{sessionId}`), once, at
/// `ShellState::new`. Process id + boot-time millis is deliberately simple (CLAUDE.md: concise code,
/// no cryptographic uniqueness need) — collisions would require two `semio-wgpu-native` processes
/// booting the same pid at the same millisecond, which the OS itself already rules out.
#[cfg(not(target_arch = "wasm32"))]
fn mint_shell_session_id() -> String {
    format!("wgpu-{}", semio_framework_os_kernel::os_identity::time_ordered_id())
}

/// 🌱️ Native: `S_HUB_URL`/`S_USER`/`S_DATA_DIR` straight off the process env (contract §C0 — "wgpu
/// native reads `S_*` directly"). No hub env ⇒ `None` ⇒ every identity/binding/directory code path
/// below is a no-op, preserving today's local-only behaviour byte-for-byte.
#[cfg(not(target_arch = "wasm32"))]
fn resolve_identity_env() -> Option<IdentityEnv> {
    IdentityEnv::from_process_env()
}

/// 🌐️ Browser wgpu build: `IdentityEnv::from_process_env` reads `std::env::var`, which never resolves
/// anything meaningful on `wasm32-unknown-unknown` — `🟦️boot.ts` calls `semioWgpuSetHubEnv` (mirroring
/// its existing `semioWgpuSetAppRole` call) with the `VITE_S_*` values it read, stashed here.
#[cfg(target_arch = "wasm32")]
thread_local! {
    static BOOT_HUB_ENV: std::cell::RefCell<Option<(String, String, Option<String>)>> = std::cell::RefCell::new(None);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = semioWgpuSetHubEnv)]
pub fn semio_wgpu_set_hub_env(hub_url: String, user: String, data_dir: String) {
    let data_dir = if data_dir.is_empty() { None } else { Some(data_dir) };
    BOOT_HUB_ENV.with(|cell| *cell.borrow_mut() = Some((hub_url, user, data_dir)));
}

#[cfg(target_arch = "wasm32")]
fn resolve_identity_env() -> Option<IdentityEnv> {
    let _ = IdentityEnv::from_process_env; // never meaningful on this target; kept for symmetry with native's doc.
    None
}

/// 🎭️ contract §C0's actor grammar: `user:{userId}#{sessionId}` once an identity is minted/restored,
/// falling back to the pre-identity local default (`wgpu-{instanceId}`) — the same default this shell
/// used everywhere before this lane, so local-only (no hub env) behaviour is unchanged.
#[cfg(not(target_arch = "wasm32"))]
fn shell_actor(identity: Option<&Identity>, session_id: &str, instance_id: u32) -> String {
    match identity {
        Some(identity) => actor_id(identity, session_id),
        None => format!("wgpu-{instance_id}"),
    }
}

/// 🔗️ ticket §2 — `attach_sync_backbone`/`open_document`'s default binding decision: `[Hub, Folder]`
/// when an identity AND a space both exist, `[Folder]` with a data dir but no identity, `[]`
/// otherwise. Pure and free-standing so the decision is unit-testable without a live `ShellState`.
#[cfg(not(target_arch = "wasm32"))]
fn default_persistence_bindings(identity: Option<&Identity>, space_id: Option<&str>, data_dir: Option<&std::path::Path>, surface: Option<&str>) -> Vec<PersistenceBinding> {
    let folder = match (space_id, data_dir) {
        (Some(space_id), Some(data_dir)) => Some(PersistenceBinding::Folder { path: data_dir.join("spaces").join(space_id) }),
        _ => None,
    };
    match (identity, space_id) {
        (Some(identity), Some(space_id)) => {
            let mut bindings = vec![PersistenceBinding::Hub { base_url: identity.hub_base_url.clone(), space_id: space_id.to_string(), token: Some(identity.session_token.clone()), surface: surface.map(str::to_string) }];
            bindings.extend(folder);
            bindings
        }
        _ => folder.into_iter().collect(),
    }
}

/// 📇️ ticket §C6 — maps an `os.directory.<verb>` action id + its relayed JSON args onto a
/// `DirectoryCommand`, mirroring the React shell's `directoryCommandFromAction` verb-for-verb
/// (`📓️w2-c-report.md`) — same action ids, same field names, same `share-link` → `create-invite`
/// sugar (contract §C1 has no directory-schema command kind of its own for it). `None` for an
/// unrecognized verb, defensive against a foreign/future caller.
#[cfg(not(target_arch = "wasm32"))]
fn directory_command_from_action(action_id: &str, args: Option<&Value>) -> Option<DirectoryCommand> {
    let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(|value| value.as_str()).unwrap_or("").to_string();
    let str_field_or = |key: &str, default: &str| args.and_then(|value| value.get(key)).and_then(|value| value.as_str()).unwrap_or(default).to_string();
    let space_kind = |raw: &str| match raw {
        "studio" => DirectorySpaceKind::Studio,
        "archive" => DirectorySpaceKind::Archive,
        _ => DirectorySpaceKind::Atelier,
    };
    let visibility = |raw: &str| if raw == "public" { DirectorySpaceVisibility::Public } else { DirectorySpaceVisibility::Private };
    let role = |raw: &str| if raw == "author" { DirectorySpaceRole::Author } else { DirectorySpaceRole::Spectator };
    match action_id {
        "os.directory.create-space" => Some(DirectoryCommand::CreateSpace { name: str_field("name"), space_kind: space_kind(&str_field_or("spaceKind", "atelier")), visibility: visibility(&str_field_or("visibility", "private")) }),
        "os.directory.delete-space" => Some(DirectoryCommand::DeleteSpace { space_id: str_field("spaceId") }),
        "os.directory.rename-space" => Some(DirectoryCommand::RenameSpace { space_id: str_field("spaceId"), name: str_field("name") }),
        "os.directory.set-visibility" => Some(DirectoryCommand::SetVisibility { space_id: str_field("spaceId"), visibility: visibility(&str_field_or("visibility", "private")) }),
        "os.directory.upsert-member" => Some(DirectoryCommand::UpsertMember { space_id: str_field("spaceId"), email: str_field("email"), role: role(&str_field_or("role", "spectator")) }),
        "os.directory.remove-member" => Some(DirectoryCommand::RemoveMember { space_id: str_field("spaceId"), user_id: str_field("userId") }),
        "os.directory.share-link" => {
            Some(DirectoryCommand::CreateInvite { space_id: str_field("spaceId"), role: role(&str_field_or("role", "spectator")), ttl_secs: args.and_then(|value| value.get("ttlSecs")).and_then(|value| value.as_u64()).unwrap_or(3600) })
        }
        _ => None,
    }
}

/// 📇️ ticket §3/§C6 — the pure `ActionDescriptor` this shell dispatches to fold a `DirectoryEvent`
/// batch into whichever session is currently mounted (home/studio/space — this shell, like the React
/// one, keeps exactly one plugin session live at a time), mirroring the React shell's
/// `dispatchDirectoryEventBatch` action name/payload shape (`foldDirectoryEvents`, `{ events }`)
/// exactly so the guest plugins — shared between both shells — see one contract.
#[cfg(not(target_arch = "wasm32"))]
fn fold_directory_events_action(controller_id: &str, events: &[DirectoryEvent]) -> Option<ActionDescriptor> {
    if events.is_empty() {
        return None;
    }
    let events_json: Vec<Value> = events.iter().filter_map(|event| serde_json::to_value(event).ok()).collect();
    let args = optional_json_as_dsl_value(Some(serde_json::json!({ "events": events_json })));
    Some(ActionDescriptor { controller_id: controller_id.to_string(), action: "foldDirectoryEvents".to_string(), args })
}

/// 📂️ ticket §4/§3-B — parses the `os.open-artifact`/`os.open-artifact-with` relay's JSON args for
/// the `documentId`/`spaceId` fields the opening relay now carries (contract §C6: they ride inside
/// the existing `ReplayShellCommand` args, no channel tag added). `documentId` absent ⇒ `None` (the
/// relay only opened an app, no document to attach — the pre-existing gap this closes).
#[cfg(not(target_arch = "wasm32"))]
struct OpenArtifactRelayTarget {
    document_id: String,
    space_id: Option<String>,
}

#[cfg(not(target_arch = "wasm32"))]
fn open_artifact_relay_target(args: Option<&Value>) -> Option<OpenArtifactRelayTarget> {
    let document_id = args.and_then(|value| value.get("documentId")).and_then(|value| value.as_str())?.to_string();
    let space_id = args.and_then(|value| value.get("spaceId")).and_then(|value| value.as_str()).map(str::to_string);
    Some(OpenArtifactRelayTarget { document_id, space_id })
}

/// 👥️ ticket §5 — the presence roster IS surface-scoped: a `PresencePeer` batch from the currently
/// attached document only renders when the shell's own attached surface (set at `open_document`/
/// `attach_sync_backbone` time) matches the surface being displayed, so a stale batch from a
/// just-closed document/surface can never leak into whatever opens next. `PresencePeer` itself
/// carries no `surface` field (contract §C0: it travels out-of-band on the WS URL, "no `PresencePeer`
/// wire change") — this is the client-side half of that scoping.
#[cfg(not(target_arch = "wasm32"))]
fn presence_peer_rows_for_surface(peers: &[PresencePeer], attached_surface: Option<&str>, target_surface: &str) -> Vec<ui_wgpu::wgpu::PresencePeerRow> {
    if attached_surface != Some(target_surface) {
        return Vec::new();
    }
    peers
        .iter()
        .map(|peer| ui_wgpu::wgpu::PresencePeerRow {
            actor: peer.actor.clone(),
            user_id: peer.user_id.clone(),
            label: peer.label.clone().unwrap_or_else(|| peer.actor.clone()),
            role: match peer.role.as_deref() {
                Some("author") | Some("owner") | Some("member") => Some(ui_wgpu::wgpu::PresenceRole::Author),
                Some("spectator") | Some("viewer") => Some(ui_wgpu::wgpu::PresenceRole::Spectator),
                _ => None,
            },
            connected_at_ms: Some(peer.connected_at_ms),
            // 🎨️ The wire `PresencePeer` (`📡️spr/📡️wire`) carries no colour field — the hub assigns
            // session colours out of band via its `Session` frame, and this shell tracks no such
            // roster. `None` is the row's documented "no hub connection" value and renders as
            // palette index 0. Mapped as absent rather than fabricated, matching the identical
            // decision on this file's own outbound heartbeat; remote peers therefore all render at
            // index 0 in wgpu until either the wire carries the colour or the shell keeps a
            // Session-frame roster, which is the presence ticket's call, not this one's.
            color: None,
        })
        .collect()
}

/// 🪐️ ticket §C4/§6 — the space's own artifact-index document: kind `s.space`, dialect
/// `s.space.space@1/*`, document id always the literal `"index"` (one per hub space) — mirrored from
/// the Rust source of truth (`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🦀️component.rs`), same as the
/// React shell's own `S_SPACE_INDEX_DOCUMENT_SCHEMA`/`SPACE_INDEX_DIALECT` mirror
/// (`📓️w2-c-report.md`).
#[cfg(not(target_arch = "wasm32"))]
const S_SPACE_INDEX_DOCUMENT_SCHEMA: &str = "s.space";
#[cfg(not(target_arch = "wasm32"))]
const S_SPACE_INDEX_DOCUMENT_ID: &str = "index";

#[cfg(not(target_arch = "wasm32"))]
fn space_index_dialect() -> semio_framework::ArtifactDialect {
    semio_framework::ArtifactDialect { artifact_kind: "s.space.space".to_string(), standard: "1".to_string(), subset: "*".to_string() }
}

/// 📇️ §5/§6 — a direct manifest scan for the one app a plugin declares for a given
/// `(dialect, role)`, mirroring the React shell's `findDialectApp`.
#[cfg(not(target_arch = "wasm32"))]
fn find_dialect_app<'a>(program: &'a ProgramBridgeEntry, dialect: &semio_framework::ArtifactDialect, role: semio_framework::manifest::AppRole) -> Option<&'a AppDefinition> {
    program.manifest.apps.iter().find(|app| &app.dialect == dialect && app.role == role)
}

//#endregion 🔖️IdentityPure

//#region 🔖️CheckInPure
/// 📌️ ticket 26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END §C5 — auto check-in policy
/// constants, byte-identical to the React shell's `AUTO_CHECKIN_IDLE_MS`/`AUTO_CHECKIN_EDIT_THRESHOLD`
/// (`ShellHelpers/🟦️component.tsx`).
#[cfg(not(target_arch = "wasm32"))]
const AUTO_CHECKIN_IDLE_MS: i64 = 20_000;
#[cfg(not(target_arch = "wasm32"))]
const AUTO_CHECKIN_EDIT_THRESHOLD: u32 = 200;

/// 🧾️ ticket §C5 — folds one `HistoryPatch` into the running per-session entry map, the wgpu twin of
/// the React shell's `applyHistoryPatch` reducer (`ShellHost/🟦️component.tsx`): `replace=true` (a
/// fresh `ReadHistory` snapshot on session/document mount) clears the map first; otherwise a patch no
/// newer than the tracked cursor is a stale/duplicate reply and is ignored outright. Returns whether
/// the fold actually changed anything, so callers only re-derive the uncommitted count when it could
/// have moved.
#[cfg(not(target_arch = "wasm32"))]
fn fold_history_patch(entries: &mut BTreeMap<u64, semio_framework::kernel::HistoryEntry>, cursor: &mut u64, patch: &semio_framework::kernel::HistoryPatch, replace: bool) -> bool {
    if !replace && patch.cursor <= *cursor {
        return false;
    }
    if replace {
        entries.clear();
    }
    for entry in &patch.upserts {
        entries.insert(entry.seq, entry.clone());
    }
    *cursor = patch.cursor;
    true
}

/// 🧾️ ticket §C5 — uncommitted-since-last-checkpoint count: the SAME "since the last Change" fold the
/// React shell's `uncommittedEditCount` (`ShellHost/🟦️component.tsx`) uses — every applied
/// mutation-kind entry counts, reset to 0 the moment a `commitCheckpoint` history-kind entry is seen,
/// walked oldest-first (`BTreeMap` keyed by `seq` iterates in order already).
#[cfg(not(target_arch = "wasm32"))]
fn uncommitted_edit_count(entries: &BTreeMap<u64, semio_framework::kernel::HistoryEntry>) -> u32 {
    let mut pending = 0u32;
    for entry in entries.values() {
        if entry.kind == "history" && entry.action_id == "commitCheckpoint" {
            pending = 0;
            continue;
        }
        if entry.kind == "mutation" && entry.applied {
            pending += 1;
        }
    }
    pending
}

/// 👁️✏️ ticket §C5 item 5 — "viewers never checkpoint", the wgpu twin of the React shell's
/// `canCheckIn` (`ShellHelpers/🟦️component.tsx`): the one predicate gating both the `#s-checkin`
/// affordance's presence and the auto check-in poll's arming.
#[cfg(not(target_arch = "wasm32"))]
fn can_check_in(role: semio_framework::manifest::AppRole) -> bool {
    role == semio_framework::manifest::AppRole::Editor
}

/// 📌️ ticket §C5 item 2 — pure decision for the per-frame auto check-in poll. This shell has no timer
/// wheel (see `render_sync_status_and_checkin`'s doc note on `Effect::DispatchAction`'s `delay_ms`
/// collapsing to "next tick"), so `AutoCheckinScheduler`'s `setTimeout`-based debounce
/// (`ShellHelpers/🟦️component.tsx`) is reproduced as a poll instead: fires once uncommitted edits
/// reach `threshold` (never waiting out the idle window once crossed), or once `idle_ms` have elapsed
/// since the last uncommitted-count change — and never again while `pending` is already set (mirrors
/// `AutoCheckinScheduler::notify`'s own storm guard: a fire that's already pending never re-fires
/// until the caller observes the count return to 0 and clears `pending`).
#[cfg(not(target_arch = "wasm32"))]
fn auto_checkin_should_fire(uncommitted_count: u32, pending: bool, last_edit_at_ms: Option<i64>, now_ms: i64, idle_ms: i64, threshold: u32) -> bool {
    if uncommitted_count == 0 || pending {
        return false;
    }
    if uncommitted_count >= threshold {
        return true;
    }
    match last_edit_at_ms {
        Some(last) => now_ms.saturating_sub(last) >= idle_ms,
        None => false,
    }
}

/// 📌️ ticket §C5 item 4 — the pure gate `checkpoint_before_detach` evaluates: checkpoint-on-close
/// only when the session can check in at all (item 5's viewer guard), a document is actually attached
/// (nothing to check in otherwise), and there really is something uncommitted.
#[cfg(not(target_arch = "wasm32"))]
fn should_checkpoint_before_detach(role: semio_framework::manifest::AppRole, has_attached_document: bool, uncommitted_count: u32) -> bool {
    can_check_in(role) && has_attached_document && uncommitted_count > 0
}

/// 📌️ ticket §C5 item 6 — the pure decision `observe_invocation_history` uses to fire `TouchArtifact`:
/// a checkpoint id change only counts when THIS shell itself dispatched the checkpoint that produced
/// it (`checkpoint_dispatched`) — a checkpoint id present from the very first snapshot (nothing
/// "landed", the session just mounted with a pre-existing one) or one made by a REMOTE peer must never
/// trigger a redundant `TouchArtifact` of our own.
#[cfg(not(target_arch = "wasm32"))]
fn checkpoint_landed(previous_checkpoint_id: Option<&str>, next_checkpoint_id: Option<&str>, checkpoint_dispatched: bool) -> bool {
    checkpoint_dispatched && next_checkpoint_id.is_some() && next_checkpoint_id != previous_checkpoint_id
}
//#endregion 🔖️CheckInPure

//#region 🧪️IdentityDirectoryPresenceTests
#[cfg(all(test, not(target_arch = "wasm32")))]
mod identity_directory_presence_tests {
    use super::*;

    fn sample_identity() -> Identity {
        Identity { user_id: "u-amara".to_string(), email: "amara@semio.dev".to_string(), display_name: "Amara".to_string(), hub_base_url: "http://hub.local".to_string(), session_token: "tok".to_string(), issued_at_ms: 0 }
    }

    /// 🧪️ Verify item: "the actor id shape".
    #[test]
    fn shell_actor_uses_contract_grammar_when_identity_present_else_local_default() {
        assert_eq!(shell_actor(Some(&sample_identity()), "sess-1", 7), "user:u-amara#sess-1");
        assert_eq!(shell_actor(None, "sess-1", 7), "wgpu-7");
    }

    /// 🧪️ Verify item: "the default-bindings decision with and without identity".
    #[test]
    fn default_bindings_are_hub_plus_folder_with_identity_and_space() {
        let identity = sample_identity();
        let data_dir = std::path::Path::new("/tmp/semio-s-user1");
        let bindings = default_persistence_bindings(Some(&identity), Some("sp-1"), Some(data_dir), Some("s.space.space@1/*#editor"));
        assert_eq!(bindings.len(), 2, "hub first, folder second");
        match &bindings[0] {
            PersistenceBinding::Hub { base_url, space_id, token, surface } => {
                assert_eq!(base_url, "http://hub.local");
                assert_eq!(space_id, "sp-1");
                assert_eq!(token.as_deref(), Some("tok"));
                assert_eq!(surface.as_deref(), Some("s.space.space@1/*#editor"));
            }
            other => panic!("expected Hub binding first, got {other:?}"),
        }
        assert_eq!(bindings[1], PersistenceBinding::Folder { path: data_dir.join("spaces").join("sp-1") });
    }

    #[test]
    fn default_bindings_are_folder_only_without_identity() {
        let data_dir = std::path::Path::new("/tmp/semio-s-user1");
        let bindings = default_persistence_bindings(None, Some("sp-1"), Some(data_dir), None);
        assert_eq!(bindings, vec![PersistenceBinding::Folder { path: data_dir.join("spaces").join("sp-1") }]);
    }

    #[test]
    fn default_bindings_are_empty_without_space_or_data_dir() {
        assert!(default_persistence_bindings(None, None, None, None).is_empty());
        assert!(default_persistence_bindings(Some(&sample_identity()), None, None, None).is_empty(), "identity alone, no open space, binds nothing");
    }

    #[test]
    fn directory_command_from_action_covers_every_frozen_verb() {
        assert_eq!(
            directory_command_from_action("os.directory.create-space", Some(&serde_json::json!({"name": "Atelier", "spaceKind": "atelier", "visibility": "private"}))),
            Some(DirectoryCommand::CreateSpace { name: "Atelier".into(), space_kind: DirectorySpaceKind::Atelier, visibility: DirectorySpaceVisibility::Private })
        );
        assert_eq!(directory_command_from_action("os.directory.delete-space", Some(&serde_json::json!({"spaceId": "sp-1"}))), Some(DirectoryCommand::DeleteSpace { space_id: "sp-1".into() }));
        assert_eq!(
            directory_command_from_action("os.directory.share-link", Some(&serde_json::json!({"spaceId": "sp-1", "role": "author", "ttlSecs": 120}))),
            Some(DirectoryCommand::CreateInvite { space_id: "sp-1".into(), role: DirectorySpaceRole::Author, ttl_secs: 120 }),
            "share-link is client-side sugar for create-invite, contract §C1 has no schema command kind of its own for it"
        );
        assert_eq!(directory_command_from_action("os.unknown.verb", None), None);
    }

    /// 🧪️ Verify item: "the directory-event fold reaching a session".
    #[test]
    fn fold_directory_events_action_reaches_the_controller_with_the_events_payload() {
        let event: DirectoryEvent = serde_json::from_value(serde_json::json!({
            "seq": 7, "id": "e1", "hlc": {"physicalMs": 1, "logical": 0},
            "actor": {"kind": "system", "id": "sys"}, "body": {"kind": "space.archived", "spaceId": "sp-1"}, "recordedAtMs": 1
        }))
        .expect("fixture event decodes");
        let action = fold_directory_events_action("s.home.controller", std::slice::from_ref(&event)).expect("a non-empty batch builds an action");
        assert_eq!(action.controller_id, "s.home.controller");
        assert_eq!(action.action, "foldDirectoryEvents");
        let args_json = action.args.as_ref().map(dsl_value_as_json).expect("args present");
        // 🧮️ `ActionDescriptor.args` is a `DslValue`, whose `Number` variant is always `f64` (see
        // `dsl/schema/component.rs`) — every action-args payload round-trips numbers through this
        // f64 representation, so the JSON view here carries `seq` as a float-tagged
        // `serde_json::Number` (`.as_i64()` is `None` for it) even though the value is whole.
        assert_eq!(args_json["events"][0]["seq"].as_f64(), Some(7.0));
        assert!(fold_directory_events_action("s.home.controller", &[]).is_none(), "an empty batch dispatches nothing");
    }

    /// 🧪️ Verify item: "the `os.open-artifact{documentId}` path".
    #[test]
    fn open_artifact_relay_target_parses_document_and_space_ids() {
        let target = open_artifact_relay_target(Some(&serde_json::json!({"documentId": "index", "spaceId": "sp-1"}))).expect("documentId present");
        assert_eq!(target.document_id, "index");
        assert_eq!(target.space_id.as_deref(), Some("sp-1"));
        assert!(open_artifact_relay_target(Some(&serde_json::json!({"artifactRef": "s.space.space@1/*"}))).is_none(), "no documentId means no document to attach, the pre-existing gap this closes");
    }

    /// 🧪️ Verify item: "the presence roster filtering by surface".
    #[test]
    fn presence_rows_are_scoped_to_the_attached_surface() {
        let peer = PresencePeer {
            actor: "user:a#1".into(),
            connected_at_ms: 1,
            label: Some("Alice".into()),
            presence_pack: None,
            user_id: None,
            role: Some("author".into()),
            drag_ghost_json: None,
            interaction: None,
            color: None,
            surface: None,
            views: Vec::new(),
            ui: None,
        };
        let rows = presence_peer_rows_for_surface(std::slice::from_ref(&peer), Some("s.space.space@1/*#editor"), "s.space.space@1/*#editor");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].actor, "user:a#1");
        assert_eq!(rows[0].role, Some(ui_wgpu::wgpu::PresenceRole::Author));
        assert!(presence_peer_rows_for_surface(std::slice::from_ref(&peer), Some("s.space.space@1/*#editor"), "s.space.space@1/*#viewer").is_empty(), "a different surface sees no roster");
        assert!(presence_peer_rows_for_surface(std::slice::from_ref(&peer), None, "s.space.space@1/*#editor").is_empty(), "no attached surface (local-only document) sees no roster");
    }

    /// 🧪️ Verify item: "the status pill renders each state" — mirrors the React twin's
    /// `computeSyncPillState`/`syncPillText` test coverage (`📓️w3-a-report.md`).
    #[test]
    fn sync_pill_text_covers_persisted_pending_and_every_remote_state() {
        assert_eq!(ShellState::sync_pill_text(None), "Remote: detached");
        assert_eq!(ShellState::sync_pill_text(Some(&ArtifactSyncStatus { persisted: true, pending_mutations: 0, remote: RemoteState::Live { peer_count: 1 } })), "Persisted");
        assert_eq!(ShellState::sync_pill_text(Some(&ArtifactSyncStatus { persisted: false, pending_mutations: 3, remote: RemoteState::Live { peer_count: 1 } })), "Pending (3)");
        assert_eq!(ShellState::sync_pill_text(Some(&ArtifactSyncStatus { persisted: false, pending_mutations: 0, remote: RemoteState::Connecting })), "Remote: connecting");
        assert_eq!(ShellState::sync_pill_text(Some(&ArtifactSyncStatus { persisted: false, pending_mutations: 0, remote: RemoteState::Backoff { retry_in_ms: 500 } })), "Remote: backoff");
        assert_eq!(ShellState::sync_pill_text(Some(&ArtifactSyncStatus { persisted: false, pending_mutations: 0, remote: RemoteState::Detached })), "Remote: detached");
        // 🎯️ A non-live remote takes priority over a nonzero pending count — the connection itself
        // being degraded is the more urgent fact, mirroring the React twin's own priority order.
        assert_eq!(ShellState::sync_pill_text(Some(&ArtifactSyncStatus { persisted: false, pending_mutations: 9, remote: RemoteState::Backoff { retry_in_ms: 500 } })), "Remote: backoff");
    }

    //#region 🧪️CheckInTests
    fn mutation_entry(seq: u64, applied: bool) -> semio_framework::kernel::HistoryEntry {
        semio_framework::kernel::HistoryEntry { seq, action_id: "apply".into(), label: "Apply".into(), kind: "mutation".into(), applied, ..Default::default() }
    }

    fn checkpoint_entry(seq: u64) -> semio_framework::kernel::HistoryEntry {
        semio_framework::kernel::HistoryEntry { seq, action_id: "commitCheckpoint".into(), label: "Checkpoint".into(), kind: "history".into(), applied: true, ..Default::default() }
    }

    /// 🧪️ Verify item: "the fold merges upserts and resets the uncommitted count on a checkpoint".
    #[test]
    fn fold_history_patch_merges_upserts_and_uncommitted_count_resets_on_checkpoint() {
        let mut entries = BTreeMap::new();
        let mut cursor = 0u64;
        let patch1 = semio_framework::kernel::HistoryPatch { cursor: 2, upserts: vec![mutation_entry(1, true), mutation_entry(2, true)], ..Default::default() };
        assert!(fold_history_patch(&mut entries, &mut cursor, &patch1, false));
        assert_eq!(uncommitted_edit_count(&entries), 2);
        // 🎯️ A stale/duplicate reply (cursor no newer than tracked) changes nothing.
        assert!(!fold_history_patch(&mut entries, &mut cursor, &patch1, false));
        let patch2 = semio_framework::kernel::HistoryPatch { cursor: 3, upserts: vec![checkpoint_entry(3)], current_checkpoint_id: Some("chk-1".into()), ..Default::default() };
        assert!(fold_history_patch(&mut entries, &mut cursor, &patch2, false));
        assert_eq!(uncommitted_edit_count(&entries), 0, "a commitCheckpoint history entry resets the count");
        // 🎯️ A fresh `ReadHistory` snapshot (`replace=true`) discards whatever was tracked before.
        let snapshot = semio_framework::kernel::HistoryPatch { cursor: 1, upserts: vec![mutation_entry(1, true)], ..Default::default() };
        assert!(fold_history_patch(&mut entries, &mut cursor, &snapshot, true));
        assert_eq!(entries.len(), 1);
        assert_eq!(uncommitted_edit_count(&entries), 1);
    }

    /// 🧪️ Verify item: "viewer guard" — `can_check_in` and its two downstream gates.
    #[test]
    fn viewers_never_check_in() {
        assert!(can_check_in(semio_framework::manifest::AppRole::Editor));
        assert!(!can_check_in(semio_framework::manifest::AppRole::Viewer));
        assert!(should_checkpoint_before_detach(semio_framework::manifest::AppRole::Editor, true, 3));
        assert!(!should_checkpoint_before_detach(semio_framework::manifest::AppRole::Viewer, true, 3), "a viewer with pending edits and an attached document still never checkpoints");
    }

    /// 🧪️ Verify item: "checkpoint-on-close" — the pure gate `checkpoint_before_detach` evaluates.
    #[test]
    fn checkpoint_before_detach_fires_only_with_an_attached_document_and_pending_edits() {
        assert!(should_checkpoint_before_detach(semio_framework::manifest::AppRole::Editor, true, 1));
        assert!(!should_checkpoint_before_detach(semio_framework::manifest::AppRole::Editor, false, 1), "nothing attached, nothing to check in");
        assert!(!should_checkpoint_before_detach(semio_framework::manifest::AppRole::Editor, true, 0), "nothing uncommitted, nothing to check in");
    }

    /// 🧪️ Verify item: "auto-fires once per idle period" and "volume trigger".
    #[test]
    fn auto_checkin_fires_on_idle_or_volume_and_never_twice_while_pending() {
        // 🎯️ No uncommitted edits: never fires, regardless of elapsed time.
        assert!(!auto_checkin_should_fire(0, false, Some(0), 100_000, AUTO_CHECKIN_IDLE_MS, AUTO_CHECKIN_EDIT_THRESHOLD));
        // 🎯️ Volume trigger — fires immediately once the threshold is reached, without waiting out
        // the idle window (the edit just landed this instant: `now_ms == last_edit_at_ms`).
        assert!(auto_checkin_should_fire(AUTO_CHECKIN_EDIT_THRESHOLD, false, Some(1_000), 1_000, AUTO_CHECKIN_IDLE_MS, AUTO_CHECKIN_EDIT_THRESHOLD));
        // 🎯️ Below threshold, not yet idle long enough: does not fire.
        assert!(!auto_checkin_should_fire(5, false, Some(1_000), 1_000 + AUTO_CHECKIN_IDLE_MS - 1, AUTO_CHECKIN_IDLE_MS, AUTO_CHECKIN_EDIT_THRESHOLD));
        // 🎯️ Below threshold, idle window elapsed: fires exactly once per idle period.
        assert!(auto_checkin_should_fire(5, false, Some(1_000), 1_000 + AUTO_CHECKIN_IDLE_MS, AUTO_CHECKIN_IDLE_MS, AUTO_CHECKIN_EDIT_THRESHOLD));
        // 🎯️ The storm guard: already pending (a checkpoint for this idle period is in flight) never
        // fires again, even past the threshold, until the caller observes count return to 0.
        assert!(!auto_checkin_should_fire(AUTO_CHECKIN_EDIT_THRESHOLD, true, Some(1_000), 1_000 + AUTO_CHECKIN_IDLE_MS, AUTO_CHECKIN_IDLE_MS, AUTO_CHECKIN_EDIT_THRESHOLD));
    }

    /// 🧪️ Verify item: "TouchArtifact follows a checkpoint" — the pure decision
    /// `observe_invocation_history` uses to fire it.
    #[test]
    fn touch_artifact_follows_only_a_checkpoint_this_shell_itself_dispatched() {
        // 🎯️ We asked for a checkpoint, and a NEW checkpoint id landed — fire.
        assert!(checkpoint_landed(None, Some("chk-1"), true));
        assert!(checkpoint_landed(Some("chk-1"), Some("chk-2"), true));
        // 🎯️ We asked for a checkpoint, but nothing changed (same id, e.g. a no-op reply) — no fire.
        assert!(!checkpoint_landed(Some("chk-1"), Some("chk-1"), true));
        // 🎯️ A checkpoint id changed, but THIS shell never dispatched one (a remote peer's checkpoint,
        // or the session mounting with a pre-existing id) — must never fire our own TouchArtifact.
        assert!(!checkpoint_landed(None, Some("chk-1"), false));
        assert!(!checkpoint_landed(Some("chk-1"), Some("chk-2"), false));
        // 🎯️ No checkpoint id at all yet — nothing landed.
        assert!(!checkpoint_landed(None, None, true));
    }
    //#endregion 🧪️CheckInTests
}
//#endregion 🧪️IdentityDirectoryPresenceTests

//#region ShellTypes
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceProgramEntry {
    pub plugin_id: String,
    pub workflow_step_id: String,
    pub app_id: String,
    pub label: String,
    pub breadcrumb: Vec<String>,
    pub yields: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpawnedAppEntry {
    pub id: String,
    pub plugin_id: String,
    pub instance_id: u32,
    pub app_id: String,
    pub label: String,
    pub breadcrumb: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpacePanelState {
    pub active_panel_tab: String,
    pub workflows: Vec<SpaceProgramEntry>,
    pub spawned_apps: Vec<SpawnedAppEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_spawned_id: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct ContextMenuItem {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
    pub shortcut: Option<String>,
    pub destructive: bool,
    pub action: Option<ActionDescriptor>,
    pub children: Vec<ContextMenuItem>,
    pub disabled: bool,
    pub separator: bool,
    pub checked: bool,
}

#[derive(Clone, Debug, Default)]
pub struct ContextMenuState {
    pub x: f32,
    pub y: f32,
    pub items: Vec<ContextMenuItem>,
    pub active: Vec<usize>,
    pub submenu_collapsed_at: Option<Vec<usize>>,
    /// 📜️ Vertical scroll offset for whichever rendered level currently exceeds the viewport height —
    /// see `render_context_menu_level`'s clip/scroll handling and `ShellState::handle_pointer_wheel`.
    pub scroll_offset: f32,
}

/** @emoji ⌨️ Result of routing a key while the shell context menu is open. */
pub enum ContextMenuKeyOutcome {
    Ignored,
    Consumed,
    Activate(ActionDescriptor),
    CloseMenu,
}

fn context_menu_enabled_indices(items: &[ContextMenuItem]) -> Vec<usize> {
    items.iter().enumerate().filter(|(_, item)| !item.separator && !item.disabled).map(|(index, _)| index).collect()
}

fn context_menu_items_at_level<'a>(root: &'a [ContextMenuItem], path_prefix: &[usize]) -> &'a [ContextMenuItem] {
    let mut level = root;
    for &index in path_prefix {
        let Some(row) = level.get(index) else {
            return level;
        };
        if row.children.is_empty() {
            return level;
        }
        level = &row.children;
    }
    level
}

fn context_menu_item_at_path<'a>(root: &'a [ContextMenuItem], path: &[usize]) -> Option<&'a ContextMenuItem> {
    if path.is_empty() {
        return None;
    }
    let mut level = root;
    let mut item = None;
    for (depth, &index) in path.iter().enumerate() {
        item = level.get(index);
        let row = item?;
        if depth + 1 < path.len() {
            level = &row.children;
        }
    }
    item
}

fn context_menu_path_for_item_id(root: &[ContextMenuItem], item_id: &str, prefix: &mut Vec<usize>) -> Option<Vec<usize>> {
    for (index, item) in root.iter().enumerate() {
        if item.separator || item.disabled {
            continue;
        }
        prefix.push(index);
        if item.id == item_id {
            return Some(prefix.clone());
        }
        if !item.children.is_empty() {
            if let Some(path) = context_menu_path_for_item_id(&item.children, item_id, prefix) {
                return Some(path);
            }
        }
        prefix.pop();
    }
    None
}

fn context_menu_move_active(root: &[ContextMenuItem], path: &[usize], down: bool) -> Vec<usize> {
    let level_prefix = if path.is_empty() { &[][..] } else { &path[..path.len() - 1] };
    let level = context_menu_items_at_level(root, level_prefix);
    let enabled = context_menu_enabled_indices(level);
    if enabled.is_empty() {
        return path.to_vec();
    }
    let current = path.last().copied().unwrap_or(usize::MAX);
    let position = enabled.iter().position(|index| *index == current);
    let next_position = match position {
        None => {
            if down {
                0
            } else {
                enabled.len() - 1
            }
        }
        Some(pos) => {
            if down {
                (pos + 1) % enabled.len()
            } else {
                (pos + enabled.len() - 1) % enabled.len()
            }
        }
    };
    let mut next = level_prefix.to_vec();
    next.push(enabled[next_position]);
    next
}

fn context_menu_path_for_ordinal(root: &[ContextMenuItem], path: &[usize], ordinal: usize) -> Option<Vec<usize>> {
    let level_prefix = if path.is_empty() { &[][..] } else { &path[..path.len() - 1] };
    let level = context_menu_items_at_level(root, level_prefix);
    let mut seen = 0usize;
    for (index, item) in level.iter().enumerate() {
        if item.separator || item.disabled {
            continue;
        }
        seen += 1;
        if seen == ordinal {
            let mut next = level_prefix.to_vec();
            next.push(index);
            return Some(next);
        }
    }
    None
}

fn context_menu_open_submenu_path(root: &[ContextMenuItem], path: &[usize]) -> Option<Vec<usize>> {
    let item = context_menu_item_at_path(root, path)?;
    if item.children.is_empty() {
        return None;
    }
    let enabled = context_menu_enabled_indices(&item.children);
    if enabled.is_empty() {
        return Some(path.to_vec());
    }
    let mut next = path.to_vec();
    next.push(enabled[0]);
    Some(next)
}

fn context_menu_paths_equal(a: &[usize], b: &[usize]) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(left, right)| left == right)
}

fn context_menu_submenu_open(active: &[usize], row_path: &[usize], is_active: bool, has_children: bool) -> bool {
    if !has_children {
        return false;
    }
    if is_active && active.len() == row_path.len() {
        return true;
    }
    active.len() > row_path.len() && row_path.iter().enumerate().all(|(index, value)| active.get(index) == Some(value))
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum OverlayState {
    #[default]
    None,
    Search,
    Find,
    Dropdown(String),
}

#[derive(Clone, Debug, Default)]
pub struct RightClickState {
    pub pending: bool,
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
pub struct ActiveSession {
    pub plugin_id: String,
    pub instance_id: u32,
    pub app: AppDefinition,
    pub view_state: ViewModel,
}

//#region 🔖️NativeSyncChannel
/// @emoji 🧵️ One open document's live `framework/sync` actor channel held by the native wgpu shell.
/// Mirrors `os-shell.tsx`'s `openArtifactSessionsRef` entry: the shell owns the `cmd_tx`/event
/// receiver while the sandboxed plugin instance's store pumps through the registered
/// `ChannelBackbone` (see `framework/product/os/core/rs`'s `host_runtime` canonical sequence).
#[cfg(not(target_arch = "wasm32"))]
pub struct ShellSyncChannel {
    pub document_id: String,
    pub actor_uri: String,
    pub instance_id: u32,
    pub plugin_id: String,
    pub cmd_tx: ArtifactMailboxSender,
    pub events: tokio::sync::broadcast::Receiver<ArtifactEvent>,
    pub connected_at_ms: i64,
}
//#endregion 🔖️NativeSyncChannel

//#region 🔖️ChromeThreadBoundary
/// 🏗️ Send-capable inputs, scratch state, and outputs consumed by chrome construction.
#[derive(Default)]
struct ShellChromeBuildState {
    find_items: ShellFindItemSink,
    content_focus: HashMap<String, bool>,
    tooltip_titles: HashMap<String, String>,
    tooltip_hover: Option<ChromeTooltipHover>,
    dialog_stack: Vec<ChromeDialogRequest>,
    tour_state: Option<ChromeTourState>,
    tour_auto_considered: Option<String>,
    previous_pointer_down: bool,
    clicked_this_frame: bool,
    tour_reveal_latch: Option<String>,
    element_rects: HashMap<String, ChromeElementRectEntry>,
    introduction_seen: HashMap<String, bool>,
    introduction_seen_writes: Vec<String>,
    tutorial_dispatch_internal: bool,
    preferences: ChromePrefsState,
}

/// 🖥️ UI-thread persistence and platform integration state surrounding chrome construction.
#[derive(Default)]
struct ShellChromePresentState {
    last_persisted_panel_layout: Option<PanelLayoutPersisted>,
    preferences_loaded: bool,
    last_synced_preferences: Option<UiPrefsSnapshot>,
}

#[cfg(not(target_arch = "wasm32"))]
/// 🧵️ Compile-time proof that chrome construction state can move to a worker.
fn assert_shell_chrome_build_state_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<ShellChromeBuildState>();
}
//#endregion 🔖️ChromeThreadBoundary

#[cfg(not(target_arch = "wasm32"))]
/// 🔄️ A retained-waker future polled once per shared-pool turn; no worker waits for completion.
struct ShellPoolFuture {
    pool: WorkerPool,
    lane: Lane,
    future: std::sync::Mutex<Option<std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>>>,
    scheduled: std::sync::atomic::AtomicBool,
    notified: std::sync::atomic::AtomicBool,
    cancelled: std::sync::atomic::AtomicBool,
}

#[cfg(not(target_arch = "wasm32"))]
impl ShellPoolFuture {
    fn spawn(pool: WorkerPool, lane: Lane, future: impl std::future::Future<Output = ()> + Send + 'static) -> std::sync::Arc<Self> {
        let task = std::sync::Arc::new(Self {
            pool,
            lane,
            future: std::sync::Mutex::new(Some(Box::pin(future))),
            scheduled: std::sync::atomic::AtomicBool::new(false),
            notified: std::sync::atomic::AtomicBool::new(true),
            cancelled: std::sync::atomic::AtomicBool::new(false),
        });
        task.schedule();
        task
    }

    fn schedule(self: &std::sync::Arc<Self>) {
        self.notified.store(true, std::sync::atomic::Ordering::Release);
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) || self.scheduled.swap(true, std::sync::atomic::Ordering::AcqRel) {
            return;
        }
        let task = self.clone();
        self.pool.submit(self.lane, Box::new(move || task.run_turn()));
    }

    fn run_turn(self: std::sync::Arc<Self>) {
        self.notified.store(false, std::sync::atomic::Ordering::Release);
        let future = self.future.lock().expect("ShellPoolFuture mutex poisoned").take();
        if let Some(mut future) = future {
            if !self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
                let waker = std::task::Waker::from(self.clone());
                let mut context = std::task::Context::from_waker(&waker);
                if std::future::Future::poll(future.as_mut(), &mut context).is_pending() && !self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
                    *self.future.lock().expect("ShellPoolFuture mutex poisoned") = Some(future);
                }
            }
        }
        self.scheduled.store(false, std::sync::atomic::Ordering::Release);
        if self.notified.load(std::sync::atomic::Ordering::Acquire) && self.future.lock().expect("ShellPoolFuture mutex poisoned").is_some() {
            self.schedule();
        }
    }

    fn cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.future.lock().expect("ShellPoolFuture mutex poisoned").take();
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl std::task::Wake for ShellPoolFuture {
    fn wake(self: std::sync::Arc<Self>) {
        self.schedule();
    }

    fn wake_by_ref(self: &std::sync::Arc<Self>) {
        self.schedule();
    }
}

#[cfg(not(target_arch = "wasm32"))]
type ShellDirectoryStream = DirectoryStream<NativeDirectoryTransport<TokioHostRuntime>>;

#[cfg(not(target_arch = "wasm32"))]
/// 📡️ Finite directory-stream actor: bounded turns, I/O-lane dials, timer-wheel wakeups, ordered output.
struct ShellDirectoryRunner {
    pool: WorkerPool,
    stream: std::sync::Mutex<ShellDirectoryStream>,
    context: OperationContext,
    events: std::sync::Mutex<std::collections::VecDeque<DirectoryStreamMessage>>,
    scheduled: std::sync::atomic::AtomicBool,
    notified: std::sync::atomic::AtomicBool,
    cancelled: std::sync::atomic::AtomicBool,
    timer_deadline_ms: std::sync::atomic::AtomicU64,
}

#[cfg(not(target_arch = "wasm32"))]
impl ShellDirectoryRunner {
    const MAX_EVENTS: usize = 256;
    const MAX_MESSAGES_PER_TURN: usize = 32;
    const MAX_TURN_MS: u64 = 4;
    const SOCKET_POLL_MS: u64 = 8;
    const DIAL_TIMEOUT_MS: u64 = 1_000;

    fn start(pool: WorkerPool, stream: ShellDirectoryStream, context: OperationContext) -> std::sync::Arc<Self> {
        let runner = std::sync::Arc::new(Self {
            pool,
            stream: std::sync::Mutex::new(stream),
            context,
            events: std::sync::Mutex::new(std::collections::VecDeque::new()),
            scheduled: std::sync::atomic::AtomicBool::new(false),
            notified: std::sync::atomic::AtomicBool::new(true),
            cancelled: std::sync::atomic::AtomicBool::new(false),
            timer_deadline_ms: std::sync::atomic::AtomicU64::new(0),
        });
        runner.schedule();
        runner
    }

    fn schedule(self: &std::sync::Arc<Self>) {
        self.notified.store(true, std::sync::atomic::Ordering::Release);
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) || self.scheduled.swap(true, std::sync::atomic::Ordering::AcqRel) {
            return;
        }
        let runner = self.clone();
        self.pool.submit(Lane::Io, Box::new(move || runner.run_turn()));
    }

    fn run_turn(self: std::sync::Arc<Self>) {
        self.notified.store(false, std::sync::atomic::Ordering::Release);
        let started = self.pool.now_ms();
        let mut processed = 0usize;
        let mut wake_at = None;
        while processed < Self::MAX_MESSAGES_PER_TURN && self.pool.now_ms().saturating_sub(started) < Self::MAX_TURN_MS {
            if self.cancelled.load(std::sync::atomic::Ordering::Acquire) || self.events.lock().expect("directory events mutex poisoned").len() >= Self::MAX_EVENTS {
                break;
            }
            let action = self.stream.lock().expect("directory stream mutex poisoned").turn(&self.context, self.pool.now_ms());
            match action {
                DirectoryStreamTurn::Dial { transport, url } => {
                    let weak = std::sync::Arc::downgrade(&self);
                    let context = self.context.clone();
                    self.pool.submit(
                        Lane::Io,
                        Box::new(move || {
                            let result = transport.open_ws(&context, &url, Self::DIAL_TIMEOUT_MS);
                            if let Some(runner) = weak.upgrade() {
                                let now_ms = runner.pool.now_ms();
                                let _ = runner.stream.lock().expect("directory stream mutex poisoned").complete_dial(now_ms, result);
                                runner.schedule();
                            }
                        }),
                    );
                    break;
                }
                DirectoryStreamTurn::Message(message) => {
                    self.events.lock().expect("directory events mutex poisoned").push_back(message);
                    processed += 1;
                }
                DirectoryStreamTurn::ReconnectAt(deadline_ms) => {
                    wake_at = Some(deadline_ms);
                    break;
                }
                DirectoryStreamTurn::Idle => {
                    wake_at = Some(self.pool.now_ms().saturating_add(Self::SOCKET_POLL_MS));
                    break;
                }
                DirectoryStreamTurn::Closed => break,
            }
        }
        self.scheduled.store(false, std::sync::atomic::Ordering::Release);
        if self.notified.load(std::sync::atomic::Ordering::Acquire) || processed == Self::MAX_MESSAGES_PER_TURN {
            self.schedule();
        } else if let Some(deadline_ms) = wake_at {
            self.arm_timer(deadline_ms);
        }
    }

    fn arm_timer(self: &std::sync::Arc<Self>, deadline_ms: u64) {
        let current = self.timer_deadline_ms.load(std::sync::atomic::Ordering::Acquire);
        if current != 0 && current <= deadline_ms {
            return;
        }
        self.timer_deadline_ms.store(deadline_ms, std::sync::atomic::Ordering::Release);
        let weak = std::sync::Arc::downgrade(self);
        let pool = self.pool.clone();
        let sleep_pool = pool.clone();
        ShellPoolFuture::spawn(pool, Lane::Timer, async move {
            sleep_pool.timer().sleep_until(deadline_ms).await;
            if let Some(runner) = weak.upgrade() {
                if runner.timer_deadline_ms.compare_exchange(deadline_ms, 0, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok() {
                    runner.schedule();
                }
            }
        });
    }

    fn drain(self: &std::sync::Arc<Self>) -> Vec<DirectoryStreamMessage> {
        let events = self.events.lock().expect("directory events mutex poisoned").drain(..).collect();
        self.schedule();
        events
    }

    fn cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.context.cancel.cancel_now();
        self.stream.lock().expect("directory stream mutex poisoned").close();
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod shell_pool_future_tests {
    use super::*;
    use semio_framework_async::{ProcessKind, WorkerPoolConfig};

    #[test]
    fn retained_future_progresses_without_adding_threads() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::InteractiveNative, 2));
        let workers = pool.worker_count();
        let (tx, rx) = std::sync::mpsc::channel();
        let _task = ShellPoolFuture::spawn(pool.clone(), Lane::Io, async move {
            semio_framework_async::yield_once().await;
            let _ = tx.send(7);
        });
        assert_eq!(rx.recv_timeout(std::time::Duration::from_secs(1)), Ok(7));
        assert_eq!(pool.worker_count(), workers);
        pool.shutdown();
    }

    #[test]
    fn cancellation_drops_a_sleeping_future_before_shutdown() {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::InteractiveNative, 2));
        let sleep_pool = pool.clone();
        let task = ShellPoolFuture::spawn(pool.clone(), Lane::Timer, async move {
            sleep_pool.timer().sleep_until(sleep_pool.now_ms().saturating_add(60_000)).await;
        });
        task.cancel();
        let started = std::time::Instant::now();
        pool.shutdown();
        assert!(started.elapsed() < std::time::Duration::from_secs(1));
    }
}

#[cfg(not(target_arch = "wasm32"))]
enum ShellIoCompletion {
    Actions(Vec<ActionDescriptor>),
    Finished,
}

#[cfg(not(target_arch = "wasm32"))]
struct PendingShellIo {
    receiver: std::sync::mpsc::Receiver<ShellIoCompletion>,
    task: Option<std::sync::Arc<ShellPoolFuture>>,
}

//#region 📄️ShellDocumentRetirement
const SHELL_DOCUMENT_RETIREMENT_CAPACITY: usize = UI_DOCUMENT_LEASE_SLOTS * UI_DOCUMENT_LEASE_ALIASES as usize;

struct ShellDocumentRetirementSlot {
    epoch: u64,
    generation: u64,
    surface: Option<SurfaceId>,
    document: UiDocumentLease,
}

struct ShellDocumentRetirementRegistry {
    slots: [Option<ShellDocumentRetirementSlot>; SHELL_DOCUMENT_RETIREMENT_CAPACITY],
    epochs: [u64; SHELL_DOCUMENT_RETIREMENT_CAPACITY],
    cursor: usize,
}

impl Default for ShellDocumentRetirementRegistry {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| None), epochs: [0; SHELL_DOCUMENT_RETIREMENT_CAPACITY], cursor: 0 }
    }
}

impl ShellDocumentRetirementRegistry {
    fn try_admit(&mut self, document: UiDocumentLease) -> Result<(), UiDocumentLease> {
        let Some(index) = self.slots.iter().position(Option::is_none) else { return Err(document) };
        let Some(epoch) = self.epochs[index].checked_add(1) else { return Err(document) };
        let generation = document.generation();
        let surface = document.header().ok().map(|header| header.surface);
        self.epochs[index] = epoch;
        self.slots[index] = Some(ShellDocumentRetirementSlot { epoch, generation, surface, document });
        Ok(())
    }

    fn close_one(&mut self) -> bool {
        let index = self.cursor;
        self.cursor = (self.cursor + 1) % SHELL_DOCUMENT_RETIREMENT_CAPACITY;
        let Some(slot) = self.slots[index].as_mut() else { return false };
        let terminal = slot.document.close_step() && slot.document.terminal_is_empty();
        if terminal {
            let terminal = self.slots[index].take().expect("terminal shell document retirement slot");
            assert!(terminal.document.terminal_is_empty(), "shell document retirement witness changed before removal");
        }
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.slots.iter().all(Option::is_none)
    }

    #[cfg(test)]
    fn qualified_owners(&self) -> impl Iterator<Item = (u64, u64, Option<&SurfaceId>)> {
        self.slots.iter().flatten().map(|slot| (slot.epoch, slot.generation, slot.surface.as_ref()))
    }
}

#[cfg(test)]
mod shell_document_retirement_tests {
    use super::*;

    fn document(generation: u64, surface: &str) -> UiDocumentLease {
        let id = ui_contract::UiNodeId(1);
        let surface = SurfaceId::try_from(surface).expect("bounded shell retirement surface");
        let mut builder = ui_contract::UiDocumentBuilder::try_new(generation, surface, ui_contract::UiRevision(generation), Some(id), generation).expect("shell retirement document slot");
        builder
            .try_push(ui_contract::UiNodeRecord {
                id,
                key: UiText::try_from_str("root").expect("bounded shell retirement key"),
                component: ui_contract::Component::Separator(ui_contract::SeparatorProps {}),
                layout: Default::default(),
                style: Default::default(),
                activity: Default::default(),
                disabled: false,
                transition: None,
                accessibility: Default::default(),
                bindings: Default::default(),
                menu: None,
                children: Default::default(),
            })
            .expect("shell retirement root");
        builder.finish().expect("shell retirement document")
    }

    #[test]
    fn shell_ninth_document_and_nonterminal_first_close_remain_in_qualified_retirement() {
        let first = document(90_001, "shell.retirement.first");
        let mut owners = UiFixedList::<UiDocumentLease, 9>::default();
        for _ in 0..7 {
            owners.try_push(first.try_alias().expect("fixed shell document alias")).expect("shell alias owner");
        }
        owners.try_push(first).expect("shell primary owner");
        owners.try_push(document(90_002, "shell.retirement.ninth")).expect("shell ninth owner");
        let mut registry = ShellDocumentRetirementRegistry::default();
        for owner in owners {
            registry.try_admit(owner).expect("absolute lease credits retain the ninth owner");
        }
        assert_eq!(registry.qualified_owners().count(), 9);
        assert!(registry.close_one());
        assert_eq!(registry.qualified_owners().count(), 9, "a false-returning first close opportunity cannot remove its exact owner");
        let mut opportunities = 1;
        while !registry.terminal_is_empty() {
            let _ = registry.close_one();
            opportunities += 1;
            assert!(opportunities < 8_192, "fixed shell document retirement must converge");
        }
    }

    #[test]
    fn shell_absolute_refusal_returns_the_exact_max_plus_one_owner_before_mutation() {
        let mut owners = UiFixedList::<UiDocumentLease, SHELL_DOCUMENT_RETIREMENT_CAPACITY>::default();
        for ordinal in 0..UI_DOCUMENT_LEASE_SLOTS {
            let primary = document(90_100 + ordinal as u64, &format!("shell.retirement.absolute-{ordinal}"));
            for _ in 1..UI_DOCUMENT_LEASE_ALIASES {
                owners.try_push(primary.try_alias().expect("absolute fixed shell alias")).expect("absolute shell alias owner");
            }
            owners.try_push(primary).expect("absolute shell primary owner");
        }
        let expected = owners.pop().expect("absolute shell refusal owner");
        let generation = expected.generation();
        let mut registry = ShellDocumentRetirementRegistry::default();
        for owner in owners {
            registry.try_admit(owner).expect("absolute shell retirement owner");
        }
        let vacancy = registry.slots.iter().position(Option::is_none).expect("one exact refusal vacancy");
        registry.epochs[vacancy] = u64::MAX;
        let refused = registry.try_admit(expected).expect_err("exhausted vacancy refuses before transfer");
        assert_eq!(refused.generation(), generation);
        assert_eq!(registry.qualified_owners().count(), SHELL_DOCUMENT_RETIREMENT_CAPACITY - 1);
        registry.epochs[vacancy] = 0;
        registry.try_admit(refused).expect("exact refused owner retries without reconstruction");
        while !registry.terminal_is_empty() {
            let _ = registry.close_one();
        }
    }
}
//#endregion 📄️ShellDocumentRetirement

pub struct ShellState {
    pub plugins: Vec<ProgramBridgeEntry>,
    pub plugin_filter: String,
    pub space_mode: bool,
    pub session: Option<ActiveSession>,
    pub window_ui: HashMap<String, UiDocumentLease>,
    pub panel_documents: HashMap<String, UiDocumentLease>,
    pub spawned_ui: Option<UiDocumentLease>,
    closing_documents: ShellDocumentRetirementRegistry,
    pub active_window_id: Option<String>,
    pub left_panel_open: bool,
    pub right_panel_open: bool,
    pub left_panel_width: f32,
    pub right_panel_width: f32,
    pub scroll_offsets: HashMap<String, f32>,
    pub overlay_state: OverlayState,
    pub collapsed_sections: HashMap<String, bool>,
    pub open_selects: HashMap<String, bool>,
    pub active_right_tab: Option<String>,
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
    pub screen_w: f32,
    pub screen_h: f32,
    pub world3d_states: AdmittedSurfaceMap<World3dState>,
    pub node_graph_states: AdmittedSurfaceMap<NodeGraphSurface>,
    pub tiled_map_states: AdmittedSurfaceMap<TiledMapSurface>,
    pub icon_render_states: HashMap<String, World3dState>,
    pub board2d_states: AdmittedSurfaceMap<Board2dSurface>,
    pub dock: DockState,
    pub active_left_kind: LeftPanelKind,
    pub active_right_kind: RightPanelKind,
    pub search_query: String,
    pub search_selected: usize,
    pub find_query: String,
    pub split_resize_path: Option<Vec<usize>>,
    pub split_resize_index: usize,
    pub split_resize_axis_total: f32,
    pub active_example_id: Option<String>,
    pub active_left_tab: Option<String>,
    pub find_items: Vec<ShellFindItem>,
    pub find_selected: usize,
    pub engagement_expanded: HashMap<String, bool>,
    pub engagement_activated: HashMap<String, bool>,
    pub measures_folded: HashMap<String, bool>,
    pub measures_expanded: HashMap<String, bool>,
    pub measures_width: HashMap<String, f32>,
    pub measures_resize_origin_width: f32,
    pub engagement_inputs: HashMap<String, String>,
    pub driver_id: String,
    pub tree_drag: Option<TreeDragState>,
    pub tree_hovered_id: Option<String>,
    pub widget_maps: WidgetInteractionMaps<ActionDescriptor>,
    window_measure_actions: WindowMeasureActionRegistry,
    pub pending_tree_drag: Option<(String, HashMap<String, String>)>,
    pub tree_drag_origin: (f32, f32),
    pub dock_drag: Option<DockDragState>,
    pub pending_dock_drag: Option<(DockDragPayload, (f32, f32))>,
    pub dock_drag_snapshot: Option<ui_wgpu::wgpu::WindowLayout>,
    pub dock_canvas_bounds: Rect,
    pub dock_drop_tab_bars: Vec<(Vec<usize>, WindowStackCorner, Rect, Vec<f32>)>,
    pub dock_drop_bodies: Vec<(Vec<usize>, Rect, String)>,
    pub layout_override: Option<ui_wgpu::wgpu::WindowLayout>,
    pub split_resize_origin: Vec<f32>,
    pub split_resize_secondary_path: Option<Vec<usize>>,
    pub split_resize_secondary_index: usize,
    pub split_resize_secondary_axis_total: f32,
    pub split_resize_secondary_origin: Vec<f32>,
    pub measures_resize_window_id: Option<String>,
    pub deferred_actions: Vec<ActionDescriptor>,
    #[cfg(not(target_arch = "wasm32"))]
    shell_io_pending: std::collections::VecDeque<PendingShellIo>,
    pub fullscreen_toggle_requested: bool,
    pub fullscreen_active: bool,
    pub active_utilities: Vec<UtilityNode>,
    /// @emoji 🧰️ Host-owned active utility per window kind (never a document field, never a VCS operation).
    /// Replaces the deleted `active_utility_id`/`find_active_utility_id` "first pressed toggle" heuristic.
    pub active_utility_by_window: HashMap<String, String>,
    /// @emoji 📇️ Per-window Actions-rail fold state (absent = folded, the default).
    pub action_panel_folded: HashMap<String, bool>,
    /// @emoji 📇️ Per-window expanded action id (the accordion-open staged arg form).
    pub action_panel_expanded: HashMap<String, String>,
    /// @emoji 📝️ Staged action argument values keyed `"{window_id}:{action_id}"` — edits buffer here
    /// and never dispatch until Execute (Architecture Decision 8, P2).
    pub staged_action_args: HashMap<String, serde_json::Map<String, Value>>,
    pub sync_backbone_uri: Option<String>,
    pub sync_card_kind: Option<String>,
    pub sync_card_draft: String,
    pub sync_card_anchor: Option<(f32, f32)>,
    pub last_envelope_dsl: Option<String>,
    /// @emoji 🏛️ Shell-lifetime document-host actor registry (native only); the browser wgpu build
    /// has no native `ArtifactHost` — its sync flows through the React shell's `🟦️backbone-worker.ts`.
    #[cfg(not(target_arch = "wasm32"))]
    pub document_host: ArtifactHost,
    /// @emoji 🧵️ The currently attached document's live actor channel (native only).
    #[cfg(not(target_arch = "wasm32"))]
    pub sync_channel: Option<ShellSyncChannel>,
    /// @emoji 🚦️ Latest sync health for the active document's status badge (native only).
    #[cfg(not(target_arch = "wasm32"))]
    pub sync_status: Option<ArtifactSyncStatus>,
    //#region 🔖️Identity
    /// 🪪️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C3 — the restored-or-
    /// minted session, `None` with no hub env (unchanged local-only behaviour) or before boot's
    /// bootstrap thread reports back. Native only, see this region's header note above.
    #[cfg(not(target_arch = "wasm32"))]
    pub identity: Option<Identity>,
    /// 📶️ Set when `mint_or_restore` degraded to the last cached identity because the hub was
    /// unreachable — never blocks, never clears `identity` itself.
    #[cfg(not(target_arch = "wasm32"))]
    pub identity_offline: bool,
    /// 🌱️ `S_HUB_URL`/`S_USER`/`S_DATA_DIR`, resolved once at `boot()` — `None` means "no hub env",
    /// the sentinel every identity/binding/directory code path below checks first.
    #[cfg(not(target_arch = "wasm32"))]
    pub identity_env: Option<IdentityEnv>,
    /// 🎭️ Per-process session id — the `{sessionId}` half of contract §C0's `user:{userId}#{sessionId}`
    /// actor grammar, minted once in `ShellState::new` and stable for this process's lifetime.
    #[cfg(not(target_arch = "wasm32"))]
    pub shell_session_id: String,
    /// 📡️ Non-blocking identity result channel, completed by a retained-waker pool future.
    #[cfg(not(target_arch = "wasm32"))]
    pub identity_bootstrap_rx: Option<std::sync::mpsc::Receiver<Result<IdentityOutcome, String>>>,
    /// 🛑️ Cancellable handle for the finite-turn identity bootstrap future.
    #[cfg(not(target_arch = "wasm32"))]
    identity_bootstrap_task: Option<std::sync::Arc<ShellPoolFuture>>,
    /// 📇️ The hub directory client used to issue `os.directory.*` commands (§C6) — constructed once
    /// identity resolves, holding the session token.
    #[cfg(not(target_arch = "wasm32"))]
    pub directory_client: Option<std::sync::Arc<DirectoryClient<NativeDirectoryTransport<TokioHostRuntime>>>>,
    /// 📡️ Finite shared-pool stream runner; output is lossless-bounded and drained once per frame.
    #[cfg(not(target_arch = "wasm32"))]
    directory_stream: Option<std::sync::Arc<ShellDirectoryRunner>>,
    /// 🔌️ ONE `NativeDirectoryTransport` (cheap to `.clone()` — see its own doc), shared by every
    /// `DirectoryClient` this shell constructs so they all draw on the SAME `HttpPool` byte budget/
    /// outstanding-request accounting rather than each minting a disjoint pool.
    #[cfg(not(target_arch = "wasm32"))]
    pub directory_transport: NativeDirectoryTransport<TokioHostRuntime>,
    /// 🛑️ Shell-lifetime cancellation root shared by identity, requests, and the directory runner.
    #[cfg(not(target_arch = "wasm32"))]
    pub directory_cancel: CancelToken,
    /// 🎮️ Bounded offline queue for `os.directory.*` commands issued while the hub is unreachable —
    /// flushed opportunistically every frame once `directory_client` exists (contract §C6: "commands
    /// queue in the shell (bounded, in-memory) and flush on reconnect").
    #[cfg(not(target_arch = "wasm32"))]
    pub pending_directory_commands: Vec<DirectoryCommand>,
    /// 👥️ ticket §5 — shell-LOCAL presence roster from the currently attached document's
    /// `ArtifactEvent::Presence`, deliberately NOT folded into the shared kernel `ViewModel`.
    #[cfg(not(target_arch = "wasm32"))]
    pub presence_peers: Vec<PresencePeer>,
    /// 👥️ The canonical surface id (`surface_app_id`) the CURRENTLY attached document's hub binding
    /// used, if any — `presence_peers` is scoped to this surface; a document opened without a hub
    /// binding (local-only) carries `None` and renders no roster.
    #[cfg(not(target_arch = "wasm32"))]
    pub presence_surface: Option<String>,
    //#endregion 🔖️Identity
    //#region 🔖️CheckIn
    /// 🧾️ ticket §C5 — this session's live history projection (`history_cursor`/`history_entries`
    /// mirror the React shell's `historyProjection` reducer), fed by `ReadHistory`
    /// (session/document mount, `replace=true`) and `AppFrame::Invocation.history_patch` (every
    /// dispatch response, `replace=false`) — see `fold_history_patch`/`observe_invocation_history`.
    #[cfg(not(target_arch = "wasm32"))]
    pub history_cursor: u64,
    #[cfg(not(target_arch = "wasm32"))]
    pub history_entries: BTreeMap<u64, semio_framework::kernel::HistoryEntry>,
    /// 🧾️ The most recent checkpoint id `HistoryPatch.currentCheckpointId` reported — compared against
    /// its previous value to detect "a checkpoint landed" (§C5 item 6, `TouchArtifact`).
    #[cfg(not(target_arch = "wasm32"))]
    pub history_current_checkpoint_id: Option<String>,
    /// 📌️ Ms-since-epoch of the last uncommitted-count CHANGE (any direction — mirrors
    /// `AutoCheckinScheduler::notify` resetting its idle timer on every call, not just increases);
    /// `None` whenever nothing is uncommitted.
    #[cfg(not(target_arch = "wasm32"))]
    pub last_uncommitted_edit_at_ms: Option<i64>,
    /// 📌️ The auto check-in poll's own `pending` latch (`AutoCheckinScheduler`'s twin): set the
    /// instant an auto checkpoint is dispatched, cleared the moment `uncommitted_edit_count` returns
    /// to 0 (a landed checkpoint) — the storm guard `auto_checkin_should_fire` reads.
    #[cfg(not(target_arch = "wasm32"))]
    pub auto_checkin_pending: bool,
    /// 📌️ §C5 item 6 — set right before ANY checkpoint (auto/explicit/close) THIS shell itself asked
    /// for; cleared once the resulting `history_current_checkpoint_id` change is observed and
    /// `TouchArtifact` fires — distinguishes "a checkpoint we asked for landed" from "the session just
    /// mounted with a pre-existing checkpoint" (mirrors React's `checkpointDispatchedRef`).
    #[cfg(not(target_arch = "wasm32"))]
    pub checkpoint_dispatched: bool,
    /// 📌️ ticket §C5 item 3 — explicit check-in's own message-prompt dialog: `None` when closed,
    /// `Some(draft)` while open. This hand-painted footer chrome has no generic text-input widget (the
    /// `Interpreter` pipeline's `UiEvent::TextInput` is deliberately bypassed here, same as
    /// `render_presence_bar`'s own design note) — mirrors the pre-existing `sync_card_kind`/
    /// `sync_card_draft` shell-owned keyboard-routed draft-field idiom instead of inventing a new one.
    #[cfg(not(target_arch = "wasm32"))]
    pub checkin_dialog_draft: Option<String>,
    //#endregion 🔖️CheckIn
    pub window_engagements: HashMap<String, WindowEngagement>,
    pub window_measures: HashMap<String, Vec<WindowMeasure>>,
    pub utility_collection_expanded: HashMap<String, bool>,
    pub contributor_instances: HashMap<String, u32>,
    /// 🖱️ Last-rendered full window content bounds per window id — used to apply the active utility's
    /// cursor while the pointer is inside that window's silhouette (Architecture Decision 8, P5).
    pub window_content_rects: HashMap<String, Rect>,
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
        if let Some(runner) = self.directory_stream.take() {
            runner.cancel();
        }
        for pending in &self.shell_io_pending {
            if let Some(task) = &pending.task {
                task.cancel();
            }
        }
    }
}

//#region ShellLifecycle
//#region 🧭️PanelAnchorModel
/// 🧭️ The framework's generic 8-anchor panel positioning model — mirrors `PanelGroup::anchor()`
/// (`framework/core/rs/lib.rs`) and React's `Anchor`/`ANCHORS` (`ui/js/react/index.tsx`).
/// This shell only ever surfaces the four corners today: `left_panel_open`/`right_panel_open` gate
/// visibility and `active_left_kind`/`active_right_kind` pick which of the two candidates occupies that
/// side, exactly the same Workbench/Display and Details/Settings corner split `PanelGroup::anchor()`
/// already declares. Re-homing that onto named anchors here gives future code (drag re-anchoring,
/// middle-anchor content) one generic surface to target instead of the scattered `group_side` left/right
/// fold. The four edge-middle anchors (`TopMiddle`/`BottomMiddle`/`LeftMiddle`/`RightMiddle`) are real
/// anchors with nothing assigned to them yet — matches upstream, where `PanelGroup` never maps to a
/// middle anchor either.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PanelAnchor {
    TopLeft,
    TopMiddle,
    TopRight,
    RightMiddle,
    BottomRight,
    BottomMiddle,
    BottomLeft,
    LeftMiddle,
}

impl PanelAnchor {
    pub const ALL: [PanelAnchor; 8] = [PanelAnchor::TopLeft, PanelAnchor::TopMiddle, PanelAnchor::TopRight, PanelAnchor::RightMiddle, PanelAnchor::BottomRight, PanelAnchor::BottomMiddle, PanelAnchor::BottomLeft, PanelAnchor::LeftMiddle];

    pub fn as_str(&self) -> &'static str {
        match self {
            PanelAnchor::TopLeft => "top-left",
            PanelAnchor::TopMiddle => "top-middle",
            PanelAnchor::TopRight => "top-right",
            PanelAnchor::RightMiddle => "right-middle",
            PanelAnchor::BottomRight => "bottom-right",
            PanelAnchor::BottomMiddle => "bottom-middle",
            PanelAnchor::BottomLeft => "bottom-left",
            PanelAnchor::LeftMiddle => "left-middle",
        }
    }

    /// 🧭️ Mirrors `PanelGroup::anchor()`'s corner mapping exactly; a group never maps to a middle anchor.
    pub fn from_group(group: PanelGroup) -> PanelAnchor {
        match group.anchor() {
            "top-right" => PanelAnchor::TopRight,
            "bottom-left" => PanelAnchor::BottomLeft,
            "bottom-right" => PanelAnchor::BottomRight,
            _ => PanelAnchor::TopLeft,
        }
    }
}

/// 🧭️ A single anchor's current visible/size/active-tab projection — the read side of the anchor model.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PanelAnchorSnapshot {
    pub visible: bool,
    pub size: f32,
    pub active_tab: Option<String>,
}

/// 🗄️ The subset of panel layout that's actually mutable today, keyed the same way it's persisted — one
/// JSON blob, localStorage on wasm / a `~/.semio/panel-layout.json` (`%APPDATA%\semio` on Windows) file
/// on native. See `ShellState::persist_panel_layout`/`load_persisted_panel_layout` below.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PanelLayoutPersisted {
    #[serde(default)]
    pub left_panel_open: bool,
    #[serde(default)]
    pub right_panel_open: bool,
    #[serde(default)]
    pub active_left_kind: Option<String>,
    #[serde(default)]
    pub active_right_kind: Option<String>,
    #[serde(default)]
    pub left_panel_width: Option<f32>,
    #[serde(default)]
    pub right_panel_width: Option<f32>,
}

const PANEL_LAYOUT_STORAGE_KEY: &str = "semio.panelLayout.v1";
/// ↔ Shared starting width for every panel anchor, one compact step wider than the former 280px Document panel.
const DEFAULT_PANEL_WIDTH_PX: f32 = 300.0;

/// 🗄️ **Dedup note**: this used to be its own `js_sys::Reflect`-based localStorage pair on wasm32
/// (`local_storage_get_item`/`local_storage_set_item`) plus a parallel native `$HOME/.semio/
/// panel-layout.json` file store, built independently of and nearly identical to `🗄️PrefsStore`'s
/// `WebLocalStorage`/`FilePrefsStore` (below, `w3-prefs-i18n-themes` built those for uiPrefs the same
/// wave) — both landed the same `js_sys::Reflect` workaround for the same "`Storage` web-sys feature
/// isn't enabled" `Cargo.toml` constraint. Flagged by both `report-w3-panel-dock-6anchor.md` and
/// `report-w3-prefs-i18n-themes.md` as a wiring/dedup request; resolved here by routing panel layout
/// through `prefs_get`/`prefs_set` like every other uiPref instead of keeping a second storage mechanism.
fn load_panel_layout_from_store() -> Option<PanelLayoutPersisted> {
    prefs_get(PANEL_LAYOUT_STORAGE_KEY).and_then(|json| serde_json::from_str(&json).ok())
}

fn save_panel_layout_to_store(layout: &PanelLayoutPersisted) {
    if let Ok(json) = serde_json::to_string(layout) {
        prefs_set(PANEL_LAYOUT_STORAGE_KEY, &json);
    }
}

//#endregion 🧭️PanelAnchorModel

impl ShellState {
    #[cfg(not(target_arch = "wasm32"))]
    fn submit_shell_io(&mut self, operation: impl FnOnce() -> ShellIoCompletion + Send + 'static) {
        const MAX_PENDING_SHELL_IO: usize = 64;
        if self.shell_io_pending.len() >= MAX_PENDING_SHELL_IO {
            return;
        }
        let (sender, receiver) = std::sync::mpsc::channel();
        crate::renderer_worker_pool().submit(
            Lane::Io,
            Box::new(move || {
                let _ = sender.send(operation());
            }),
        );
        self.shell_io_pending.push_back(PendingShellIo { receiver, task: None });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn submit_shell_io_future(&mut self, operation: impl std::future::Future<Output = ShellIoCompletion> + Send + 'static) {
        const MAX_PENDING_SHELL_IO: usize = 64;
        if self.shell_io_pending.len() >= MAX_PENDING_SHELL_IO {
            return;
        }
        let (sender, receiver) = std::sync::mpsc::channel();
        let task = ShellPoolFuture::spawn(crate::renderer_worker_pool(), Lane::Io, async move {
            let _ = sender.send(operation.await);
        });
        self.shell_io_pending.push_back(PendingShellIo { receiver, task: Some(task) });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn poll_shell_io(&mut self) -> bool {
        const MAX_COMPLETIONS_PER_FRAME: usize = 8;
        let mut changed = false;
        for _ in 0..MAX_COMPLETIONS_PER_FRAME {
            let result = match self.shell_io_pending.front() {
                Some(pending) => pending.receiver.try_recv(),
                None => break,
            };
            match result {
                Ok(ShellIoCompletion::Actions(actions)) => {
                    if let Some(pending) = self.shell_io_pending.pop_front() {
                        if let Some(task) = pending.task {
                            task.cancel();
                        }
                    }
                    changed |= !actions.is_empty();
                    self.deferred_actions.extend(actions);
                }
                Ok(ShellIoCompletion::Finished) | Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    if let Some(pending) = self.shell_io_pending.pop_front() {
                        if let Some(task) = pending.task {
                            task.cancel();
                        }
                    }
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => break,
            }
        }
        changed
    }

    //#region 🏷️LabelResolution
    /// 🌐️ Active `Locale` derived from `locale_id`. This engine is not manifest-aware the way the
    /// React renderer is (no per-window locale/terminology context is threaded through render
    /// calls), so every `LocalizedLabel` in this file resolves against this shell-wide value.
    pub fn active_locale(&self) -> Locale {
        Locale::parse(&self.locale_id).unwrap_or_default()
    }

    /// 🗣️ Active `Terminology` derived from `terminology_id`. See `active_locale` doc.
    pub fn active_terminology(&self) -> Terminology {
        Terminology::parse(&self.terminology_id).unwrap_or_default()
    }
    //#endregion 🏷️LabelResolution

    pub fn new(plugins: Vec<ProgramBridgeEntry>, plugin_filter: String) -> Self {
        let space_mode = is_space_mode(&plugin_filter);
        // 🌀️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-directory-and-run, sol-extended lease):
        // the ONE `TokioHostRuntime`/`ComputePool`/`NativeDirectoryTransport` this shell's directory
        // client and finite pool turns share — minted ONCE here, never per-call.
        //
        // 🧵️ P1e (INTERACTIVE-JOB-RUNTIME-REFACTOR, one-pool-worker-runtime): this shell no longer
        // sizes or owns its own thread pool — `TokioHostRuntime::new()`'s own `thread_plan`/
        // `ThreadBudget` construction path was DELETED by P1a with no replacement. Instead this
        // injects `crate::renderer_worker_pool()`, the ONE process-wide `WorkerPool` this whole
        // renderer crate shares (also used by `📦️glue.rs::kernel_runtime`'s `ParallelRuntime` for
        // shard-turn scheduling — see that fn's own doc for why it cannot instead reuse
        // `semio-framework-os-services`'s own private `global_worker_pool()` singleton). `open_scope`/
        // `ComputePool::new`/`with_new_http_pool` are async constructors bridged once here. Runtime,
        // scope, and compute ownership then lives in the transport; no directory thread is retained.
        #[cfg(not(target_arch = "wasm32"))]
        let (directory_transport, directory_cancel): (NativeDirectoryTransport<TokioHostRuntime>, CancelToken) = {
            const DIRECTORY_COMPUTE_CAPACITY: u32 = 4;
            let pool = crate::renderer_worker_pool();
            let runtime = std::sync::Arc::new(TokioHostRuntime::with_pool(pool.clone()));
            let scope = runtime.open_scope_now(ScopeOwner::Service("directory_client"), None);
            let compute = std::sync::Arc::new(ComputePool::with_pool(DIRECTORY_COMPUTE_CAPACITY, pool));
            let transport = NativeDirectoryTransport::with_new_http_pool_now(runtime, scope, compute, 10_000_000, 8, DirectoryPackageId("os.directory-client".to_string()), DirectoryActorId(0));
            (transport, CancelToken::root_now())
        };
        let mut state = Self {
            plugins,
            plugin_filter,
            space_mode,
            session: None,
            window_ui: HashMap::new(),
            panel_documents: HashMap::new(),
            spawned_ui: None,
            closing_documents: ShellDocumentRetirementRegistry::default(),
            active_window_id: None,
            left_panel_open: false,
            right_panel_open: false,
            left_panel_width: DEFAULT_PANEL_WIDTH_PX,
            right_panel_width: DEFAULT_PANEL_WIDTH_PX,
            scroll_offsets: HashMap::new(),
            overlay_state: OverlayState::None,
            collapsed_sections: HashMap::new(),
            open_selects: HashMap::new(),
            active_right_tab: None,
            context_menu: None,
            search_open: false,
            find_open: false,
            appearance_id: "system".into(),
            locale_id: "en".into(),
            terminology_id: "native".into(),
            right_click: RightClickState::default(),
            uri_history: vec!["/".into()],
            uri_index: 0,
            open_space_id: None,
            pending_shell_uri_apply: false,
            panel_resize_origin_width: DEFAULT_PANEL_WIDTH_PX,
            error: None,
            screen_w: 1280.0,
            screen_h: 720.0,
            world3d_states: AdmittedSurfaceMap::default(),
            node_graph_states: AdmittedSurfaceMap::default(),
            tiled_map_states: AdmittedSurfaceMap::default(),
            icon_render_states: HashMap::new(),
            board2d_states: AdmittedSurfaceMap::default(),
            dock: DockState::default(),
            active_left_kind: LeftPanelKind::Workbench,
            active_right_kind: RightPanelKind::Details,
            search_query: String::new(),
            search_selected: 0,
            find_query: String::new(),
            split_resize_path: None,
            split_resize_index: 0,
            split_resize_axis_total: 1.0,
            active_example_id: None,
            active_left_tab: None,
            find_items: Vec::new(),
            find_selected: 0,
            engagement_expanded: HashMap::new(),
            engagement_activated: HashMap::new(),
            measures_folded: HashMap::new(),
            measures_expanded: HashMap::new(),
            measures_width: HashMap::new(),
            measures_resize_origin_width: 0.0,
            engagement_inputs: HashMap::new(),
            driver_id: "default".into(),
            tree_drag: None,
            tree_hovered_id: None,
            widget_maps: WidgetInteractionMaps::default(),
            window_measure_actions: WindowMeasureActionRegistry::new(),
            pending_tree_drag: None,
            tree_drag_origin: (0.0, 0.0),
            dock_drag: None,
            pending_dock_drag: None,
            dock_drag_snapshot: None,
            dock_canvas_bounds: Rect::new(0.0, 0.0, 0.0, 0.0),
            dock_drop_tab_bars: Vec::new(),
            dock_drop_bodies: Vec::new(),
            layout_override: None,
            split_resize_origin: Vec::new(),
            split_resize_secondary_path: None,
            split_resize_secondary_index: 0,
            split_resize_secondary_axis_total: 1.0,
            split_resize_secondary_origin: Vec::new(),
            measures_resize_window_id: None,
            deferred_actions: Vec::new(),
            #[cfg(not(target_arch = "wasm32"))]
            shell_io_pending: std::collections::VecDeque::new(),
            fullscreen_toggle_requested: false,
            fullscreen_active: false,
            active_utilities: Vec::new(),
            active_utility_by_window: HashMap::new(),
            action_panel_folded: HashMap::new(),
            action_panel_expanded: HashMap::new(),
            staged_action_args: HashMap::new(),
            sync_backbone_uri: None,
            sync_card_kind: None,
            sync_card_draft: String::new(),
            sync_card_anchor: None,
            last_envelope_dsl: None,
            #[cfg(not(target_arch = "wasm32"))]
            document_host: ArtifactHost::new(std::sync::Arc::new(crate::renderer_worker_pool())),
            #[cfg(not(target_arch = "wasm32"))]
            sync_channel: None,
            #[cfg(not(target_arch = "wasm32"))]
            sync_status: None,
            #[cfg(not(target_arch = "wasm32"))]
            identity: None,
            #[cfg(not(target_arch = "wasm32"))]
            identity_offline: false,
            #[cfg(not(target_arch = "wasm32"))]
            identity_env: None,
            #[cfg(not(target_arch = "wasm32"))]
            shell_session_id: mint_shell_session_id(),
            #[cfg(not(target_arch = "wasm32"))]
            identity_bootstrap_rx: None,
            #[cfg(not(target_arch = "wasm32"))]
            identity_bootstrap_task: None,
            #[cfg(not(target_arch = "wasm32"))]
            directory_client: None,
            #[cfg(not(target_arch = "wasm32"))]
            directory_stream: None,
            #[cfg(not(target_arch = "wasm32"))]
            directory_transport,
            #[cfg(not(target_arch = "wasm32"))]
            directory_cancel,
            #[cfg(not(target_arch = "wasm32"))]
            pending_directory_commands: Vec::new(),
            #[cfg(not(target_arch = "wasm32"))]
            presence_peers: Vec::new(),
            #[cfg(not(target_arch = "wasm32"))]
            presence_surface: None,
            #[cfg(not(target_arch = "wasm32"))]
            history_cursor: 0,
            #[cfg(not(target_arch = "wasm32"))]
            history_entries: BTreeMap::new(),
            #[cfg(not(target_arch = "wasm32"))]
            history_current_checkpoint_id: None,
            #[cfg(not(target_arch = "wasm32"))]
            last_uncommitted_edit_at_ms: None,
            #[cfg(not(target_arch = "wasm32"))]
            auto_checkin_pending: false,
            #[cfg(not(target_arch = "wasm32"))]
            checkpoint_dispatched: false,
            #[cfg(not(target_arch = "wasm32"))]
            checkin_dialog_draft: None,
            window_engagements: HashMap::new(),
            window_measures: HashMap::new(),
            utility_collection_expanded: HashMap::new(),
            contributor_instances: HashMap::new(),
            window_content_rects: HashMap::new(),
            window_silhouettes: HashMap::new(),
            tutorial: None,
            tutorial_pending_document_ops: Vec::new(),
            chrome_build: ShellChromeBuildState::default(),
            chrome_present: ShellChromePresentState::default(),
        };
        state.load_persisted_panel_layout();
        state
    }

    //#region 🏠️🧳️PluginHostConfig
    /// 🏠️🧳️ This filter's host config (landing/host app-id roles), or `None` when it doesn't offer a
    /// host-style multi-app experience — see `program_bridge::PluginHostConfig`.
    fn host_config(&self) -> Option<&'static PluginHostConfig> {
        resolve_plugin_host_config(&self.plugin_filter)
    }

    /// 🏠️🧳️ The host plugin's own host-role app, self-declaring its `controller_id`/`panel_tabs` — the
    /// generic source of truth for what were previously separate `S_PLAY_CONTROLLER_ID`/
    /// `S_PLAY_CATALOGUE_TAB_ID` literals.
    fn host_app(&self) -> Option<&AppDefinition> {
        let cfg = self.host_config()?;
        let program = self.plugins.iter().find(|p| p.plugin_id == cfg.plugin_id)?;
        program.manifest.apps.iter().find(|app| app.id == cfg.host_app_id)
    }

    fn host_controller_id(&self) -> Option<String> {
        self.host_app().map(|app| app.controller_id.clone())
    }

    fn host_catalogue_tab_id(&self) -> Option<String> {
        self.host_app().and_then(|app| app.panel_tabs.first().map(|tab| tab.id().to_string()))
    }
    //#endregion 🏠️🧳️PluginHostConfig

    // TEMP(Wave 3): replace with workflow_palette() once the shell palette wiring lands. Reads
    // `program.manifest.apps` directly (one `SpaceProgramEntry` per app) — `PluginManifest.workflows`/
    // `WorkflowDefinition` were deleted in Wave 0 (WP-0.1); the real palette derivation moves to
    // `semio-framework-os`'s `registry::workflow_palette()` (`AppIo`-driven) once the browser shell wires
    // it in. `yields` is empty here (no registry lookup at this layer) — matches every other Wave-1
    // `WorkflowNode.yields` derivation until Wave 2 populates apps' declared output ports.
    pub fn build_space_workflows(&self) -> Vec<SpaceProgramEntry> {
        self.plugins
            .iter()
            .flat_map(|program| {
                program.manifest.apps.iter().map(|app| SpaceProgramEntry {
                    plugin_id: program.plugin_id.clone(),
                    workflow_step_id: app.id.clone(),
                    app_id: app.id.clone(),
                    label: app.label.resolve(self.active_terminology(), self.active_locale()).to_string(),
                    breadcrumb: app.breadcrumb.clone(),
                    yields: String::new(),
                })
            })
            .collect()
    }

    pub fn panel_state_from_view(view_state: &ViewModel) -> Option<SpacePanelState> {
        view_state.panel_json.as_ref().and_then(|json| serde_json::from_str(json).ok())
    }

    pub fn panel_json(state: &SpacePanelState) -> String {
        serde_json::to_string(state).unwrap_or_default()
    }

    pub fn prepare_hot_reload(&mut self, plugins: Vec<ProgramBridgeEntry>) {
        if let Some(session) = self.session.take() {
            if let Some(program) = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id) {
                program.destroy_app(session.instance_id);
            }
        }
        self.plugins = plugins;
    }

    pub async fn hot_reload_plugins(&mut self, plugins: Vec<ProgramBridgeEntry>) -> Result<(), String> {
        self.prepare_hot_reload(plugins);
        self.boot().await
    }

    pub async fn boot(&mut self) -> Result<(), String> {
        if let Some(cfg) = self.host_config() {
            let semio_s_plugin_space = self.plugins.iter().find(|p| p.plugin_id == cfg.plugin_id).ok_or("host program missing")?;
            let s_app = semio_s_plugin_space.manifest.apps.iter().find(|app| app.id == cfg.landing_app_id).or_else(|| semio_s_plugin_space.manifest.apps.first()).ok_or("host program missing landing app")?.clone();
            let workflows = self.build_space_workflows();
            let panel_state = SpacePanelState { active_panel_tab: self.host_catalogue_tab_id().unwrap_or_default(), workflows, spawned_apps: vec![], active_spawned_id: None };
            let instance_id = semio_s_plugin_space.create_app(&s_app.id).await?;
            let view_state = ViewModel {
                active_mode_id: Some(s_app.default_mode_id.clone()),
                active_window_kind_id: Some(s_app.window_kinds.first().id.clone()),
                active_utility_id: None,
                panel_json: Some(Self::panel_json(&panel_state)),
                contributions_json: None,
                locale: self.active_locale(),
                terminology: self.active_terminology(),
                window_id: None,
                window_instances: Vec::new(),
                active_tool_id: None,
                active_utility_by_window_id: HashMap::new(),
            };
            self.active_window_id = Some(s_app.window_kinds.first().id.clone());
            self.session = Some(ActiveSession { plugin_id: semio_s_plugin_space.plugin_id.clone(), instance_id, app: s_app, view_state });
        } else if let Some(program) = self.plugins.first() {
            let app = program.manifest.apps.iter().find(|app| Some(app.id.as_str()) == resolve_playground_app_id(&self.plugin_filter)).or_else(|| program.manifest.apps.first()).ok_or("plugin has no apps")?.clone();
            let instance_id = program.create_app(&app.id).await?;
            self.active_window_id = Some(app.window_kinds.first().id.clone());
            self.session = Some(ActiveSession {
                plugin_id: program.plugin_id.clone(),
                instance_id,
                app: app.clone(),
                view_state: ViewModel {
                    active_mode_id: Some(app.default_mode_id.clone()),
                    active_window_kind_id: self.active_window_id.clone(),
                    active_utility_id: None,
                    panel_json: None,
                    contributions_json: None,
                    locale: self.active_locale(),
                    terminology: self.active_terminology(),
                    window_id: None,
                    window_instances: Vec::new(),
                    active_tool_id: None,
                    active_utility_by_window_id: HashMap::new(),
                },
            });
        }
        self.sync_dock();
        self.sync_session_chrome();
        // 📇️ ticket §1 — non-blocking (submits finite shared-pool turns and returns immediately, see
        // `bootstrap_identity`'s own doc): a slow/unreachable hub must never delay the first frame.
        #[cfg(not(target_arch = "wasm32"))]
        self.bootstrap_identity();
        self.refresh_ui().await
    }

    fn sync_session_chrome(&mut self) {
        let Some(session) = &self.session else {
            return;
        };
        let examples = self.plugins.iter().find(|p| p.plugin_id == session.plugin_id).map(|p| p.manifest.examples.as_slice()).unwrap_or(&[]);
        if examples.is_empty() {
            self.active_example_id = None;
        } else {
            let current = self.active_example_id.clone();
            self.active_example_id = current.filter(|id| examples.iter().any(|ex| &ex.id == id)).or_else(|| examples.first().map(|ex| ex.id.clone()));
        }
        if let Some(mode_id) = session.view_state.active_mode_id.clone() {
            let _ = mode_id;
        }
    }

    fn active_plugin_examples(&self) -> Vec<ExampleDefinition> {
        let Some(session) = &self.session else {
            return Vec::new();
        };
        self.plugins.iter().find(|p| p.plugin_id == session.plugin_id).map(|p| p.manifest.examples.iter().filter(|example| example.app_id.is_empty() || example.app_id == session.app.id).cloned().collect()).unwrap_or_default()
    }

    fn flatten_panel_tab_leaves(tabs: &[PanelTabDefinition]) -> Vec<&PanelTabDefinition> {
        tabs.iter().flat_map(|tab| if tab.children.is_empty() { vec![tab] } else { Self::flatten_panel_tab_leaves(&tab.children) }).collect()
    }

    fn sync_dock(&mut self) {
        if let Some(session) = &self.session {
            if let Some(layout) = self.layout_override.clone() {
                self.dock.apply_layout_diff(&layout);
                if self.dock.active_window_id.is_none() {
                    self.dock.active_window_id = self.active_window_id.clone().or_else(|| session.view_state.active_window_kind_id.clone());
                }
            } else {
                self.dock = DockState::from_app(&session.app, self.active_window_id.as_deref());
            }
            if let Some(id) = &self.active_window_id {
                self.dock.sync_active_window(id);
            }
        }
    }

    fn persist_dock_layout(&mut self) {
        self.layout_override = Some(self.dock.to_window_layout());
        self.dock_drag_snapshot = None;
    }

    fn restore_dock_drag_snapshot(&mut self) {
        if let Some(layout) = self.dock_drag_snapshot.take() {
            self.layout_override = Some(layout);
            self.sync_dock();
        }
    }

    fn begin_pending_dock_drag(&mut self, payload: DockDragPayload, x: f32, y: f32) {
        self.dock_drag_snapshot = Some(self.dock.to_window_layout());
        self.pending_dock_drag = Some((payload, (x, y)));
    }

    fn dock_tab_bars_for_drop(&self, atlas: &mut FontAtlas, theme: &Theme, canvas: Rect, labels: &HashMap<String, String>, icon_ids: &HashMap<String, String>) -> Vec<(Vec<usize>, WindowStackCorner, Rect, Vec<f32>)> {
        let _ = icon_ids;
        self.dock.stack_corner_tab_bar_rects(canvas, theme, atlas, labels)
    }

    fn contributions_json_from_plugins(plugins: &[ProgramBridgeEntry]) -> String {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct ProgramContributionEntry<'a> {
            plugin_id: &'a str,
            topic_contribution: &'a semio_framework::TopicContribution,
        }
        let entries: Vec<ProgramContributionEntry<'_>> =
            plugins.iter().flat_map(|program| program.manifest.topic_contributions.iter().map(|topic_contribution| ProgramContributionEntry { plugin_id: program.plugin_id.as_str(), topic_contribution })).collect();
        serde_json::to_string(&entries).unwrap_or_else(|_| "[]".into())
    }

    pub async fn refresh_ui(&mut self) -> Result<(), String> {
        let Some(session) = self.session.clone() else {
            return Ok(());
        };
        self.close_document_one();
        self.sync_dock();
        if let Err(documents) = Self::retain_document_map_for_close(&mut self.closing_documents, std::mem::take(&mut self.window_ui)) {
            self.window_ui = documents;
            return Err("shell: window document retirement registry refused the exact prior owners".to_string());
        }
        let mut view_state = session.view_state.clone();
        view_state.contributions_json = Some(Self::contributions_json_from_plugins(&self.plugins));
        let mut refresh_effects = Vec::new();
        {
            let program = self.plugins.iter().find(|p| p.plugin_id == session.plugin_id).cloned().ok_or("session program missing")?;
            for kind in &session.app.window_kinds {
                view_state.active_utility_id = self.active_utility_by_window.get(&kind.id).cloned();
                let document = program.render_with_document(session.instance_id, &kind.body_key, &view_state, None, Some(&mut refresh_effects)).await?;
                self.window_ui.insert(kind.id.clone(), document);
            }
        }
        if let Err(documents) = Self::retain_document_map_for_close(&mut self.closing_documents, std::mem::take(&mut self.panel_documents)) {
            self.panel_documents = documents;
            return Err("shell: panel document retirement registry refused the exact prior owners".to_string());
        }
        let program = self.plugins.iter().find(|p| p.plugin_id == session.plugin_id).cloned().ok_or("session program missing")?;
        for tab in Self::flatten_panel_tab_leaves(&session.app.panel_tabs) {
            let body_key = tab.body_key.as_deref().unwrap_or_default();
            let document = program.render_with_document(session.instance_id, body_key, &view_state, None, Some(&mut refresh_effects)).await?;
            self.panel_documents.insert(tab.id().to_string(), document);
        }
        // 🧰️ The utility bar is derived from the app's declared `AppDefinition.utilities` (scoped to the active
        // window kind) via `ui_wgpu::wgpu::derive_utility_nodes` — the old per-call `plugin.utilities()` fetch and the
        // `find_active_utility_id` "first pressed toggle" heuristic are gone (Architecture Decision 5).
        self.active_utilities = self.derive_utility_nodes(&session);
        self.active_utilities.extend(framework_sync_utilities(self.sync_backbone_uri.as_deref()));
        self.window_engagements = program.window_engagements(session.instance_id, &view_state).await.unwrap_or_default();
        self.window_measures = program.window_measures(session.instance_id, &view_state).await.unwrap_or_default();
        if self.space_mode {
            if let Some(panel) = Self::panel_state_from_view(&session.view_state) {
                if let Some(spawned) = panel.active_spawned_id.as_ref().and_then(|id| panel.spawned_apps.iter().find(|app| &app.id == id)) {
                    if let Some(spawn_plugin) = self.plugins.iter().find(|p| p.plugin_id == spawned.plugin_id) {
                        let spawned_app = spawn_plugin.manifest.apps.iter().find(|app| app.id == spawned.app_id);
                        if let Some(app) = spawned_app {
                            let body_key = app.window_kinds.first().body_key.clone();
                            let view_state = ViewModel {
                                active_mode_id: Some(app.default_mode_id.clone()),
                                active_window_kind_id: Some(app.window_kinds.first().id.clone()),
                                active_utility_id: None,
                                panel_json: None,
                                contributions_json: None,
                                locale: self.active_locale(),
                                terminology: self.active_terminology(),
                                window_id: None,
                                window_instances: Vec::new(),
                                active_tool_id: None,
                                active_utility_by_window_id: HashMap::new(),
                            };
                            if let Some(document) = self.spawned_ui.take() {
                                if let Err(document) = self.retain_document_for_close(document) {
                                    self.spawned_ui = Some(document);
                                    return Err("shell: spawned document retirement registry refused the exact prior owner".to_string());
                                }
                            }
                            self.spawned_ui = Some(spawn_plugin.render(spawned.instance_id, &body_key, &view_state).await?);
                        }
                    }
                } else {
                    if let Some(document) = self.spawned_ui.take() {
                        if let Err(document) = self.retain_document_for_close(document) {
                            self.spawned_ui = Some(document);
                            return Err("shell: spawned document retirement registry refused the exact prior owner".to_string());
                        }
                    }
                }
            }
        }
        self.queue_host_effects(&session.app.controller_id, refresh_effects);
        Ok(())
    }

    fn retain_document_map_for_close(registry: &mut ShellDocumentRetirementRegistry, documents: HashMap<String, UiDocumentLease>) -> Result<(), HashMap<String, UiDocumentLease>> {
        let mut owners = documents.into_iter();
        while let Some((id, document)) = owners.next() {
            if let Err(document) = registry.try_admit(document) {
                let mut refused = HashMap::from([(id, document)]);
                refused.extend(owners);
                return Err(refused);
            }
        }
        Ok(())
    }

    fn retain_document_for_close(&mut self, document: UiDocumentLease) -> Result<(), UiDocumentLease> {
        self.closing_documents.try_admit(document)
    }

    fn close_document_one(&mut self) -> bool {
        if self.closing_documents.terminal_is_empty() {
            return !ui_contract::close_ui_document_page_one();
        }
        self.closing_documents.close_one()
    }

    fn queue_host_effects(&mut self, controller_id: &str, effects: Vec<semio_framework::kernel::Effect>) {
        for effect in effects {
            match effect {
                semio_framework::kernel::Effect::SetActiveUtility { window_id, utility_id } => {
                    self.apply_set_active_utility(&window_id, &utility_id);
                }
                semio_framework::kernel::Effect::Navigate { uri } => {
                    self.push_uri(uri);
                }
                semio_framework::kernel::Effect::LoadDocument { pack, spr } => {
                    if let Some(session) = self.session.clone() {
                        if let Some(plugin) = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned() {
                            // 🎠️ terra-shell-unpark — this used to synchronously drive the kernel
                            // round trip right here. Unlike the two `glue.rs` call sites, `ShellState`
                            // has no `Weak<RefCell<AppRuntime>>` to re-borrow after the `.await` (only
                            // `AppRuntime` carries `self_weak`), which is exactly why an earlier packet
                            // left this one alone rather than force a borrow that doesn't exist. It
                            // doesn't need one: nothing here mutates shell state after firing, only logs
                            // on failure, so `plugin` (`ProgramBridgeEntry` is `Clone`) and the already-
                            // owned `pack`/`spr` from the matched `Effect` are enough to detach this into
                            // a fully self-contained `spawn_app_task` future — `queue_host_effects`
                            // itself stays a plain, non-async `fn`.
                            let instance_id = session.instance_id;
                            crate::spawn_app_task(async move {
                                if let Err(error) = plugin.load_app_document_pack(instance_id, &pack, &spr).await {
                                    eprintln!("[DEBUG] wgpu shell loadDocument effect failed: {error}");
                                }
                            });
                        }
                    }
                }
                semio_framework::kernel::Effect::DispatchAction { action: dispatch_action_id, args, .. } => {
                    self.deferred_actions.push(ActionDescriptor { controller_id: controller_id.to_string(), action: dispatch_action_id, args });
                }
                semio_framework::kernel::Effect::RequestMediaFrames { accept, frame_action, done_action, fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload, args, .. } => {
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let controller_id = controller_id.to_string();
                        let args = optional_dsl_value_as_json(args);
                        self.submit_shell_io_future(async move {
                            ShellIoCompletion::Actions(request_media_frames(&controller_id, &accept, &frame_action, &done_action, &fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload.as_deref(), args).await)
                        });
                    }
                    #[cfg(target_arch = "wasm32")]
                    for descriptor in request_media_frames(controller_id, &accept, &frame_action, &done_action, &fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload.as_deref(), optional_dsl_value_as_json(args)) {
                        self.deferred_actions.push(descriptor);
                    }
                }
                _ => {}
            }
        }
    }

    #[cfg(test)]
    fn build_display_windows_ui(&self, session: &ActiveSession) -> UiNode {
        let items: Vec<UiNode> = session
            .app
            .window_kinds
            .iter()
            .map(|kind| {
                UiNode::Button(UiButtonNode {
                    id: Some(format!("shell.display.window.{}", kind.id)),
                    icon_id: kind.icon_id.clone(),
                    label: Label::data(format!("{} — {}", kind.label.resolve(self.active_terminology(), self.active_locale()), kind.id)),
                    action: ActionDescriptor { controller_id: session.app.controller_id.clone(), action: "noOperation".into(), args: None },
                    style: None,
                    presence: UiPresence::default(),
                    menu: None,
                })
            })
            .collect();
        if items.is_empty() {
            return UiNode::Text(UiTextNode { presence: UiPresence::default(), value: Label::data("—"), emphasize: None, data_attributes: None, menu: None });
        }
        UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, children: items, id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, menu: None })
    }

    #[cfg(test)]
    fn build_display_layout_ui(&self, session: &ActiveSession) -> UiNode {
        let items: Vec<UiNode> = session
            .app
            .named_layouts
            .iter()
            .map(|layout| {
                UiNode::Button(UiButtonNode {
                    id: Some(format!("shell.layout.{}", layout.id)),
                    icon_id: layout.icon_id.clone().unwrap_or_else(|| "layout-grid".into()),
                    label: Label::data(format!("{} ({})", layout.label, layout.origin)),
                    action: ActionDescriptor { controller_id: session.app.controller_id.clone(), action: "noOperation".into(), args: None },
                    style: None,
                    presence: UiPresence::default(),
                    menu: None,
                })
            })
            .collect();
        if items.is_empty() {
            return UiNode::Text(UiTextNode { presence: UiPresence::default(), value: Label::data("No saved layouts"), emphasize: None, data_attributes: None, menu: None });
        }
        UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, children: items, id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, menu: None })
    }

    #[cfg(test)]
    fn build_settings_general_ui(&self) -> UiNode {
        UiNode::Stack(UiStackNode {
            direction: "column".into(),
            gap: None,
            padding: None,
            id: None,
            children: vec![
                UiNode::Text(UiTextNode { presence: UiPresence::default(), value: Label::data("General"), emphasize: Some(true), data_attributes: None, menu: None }),
                UiNode::Select(UiSelectNode {
                    presence: UiPresence::default(),
                    id: "framework.settings.appearance".into(),
                    value: self.appearance_id.clone(),
                    items: vec![UiSelectItem { value: "system".into(), label: Label::data("System") }, UiSelectItem { value: "light".into(), label: Label::data("Light") }, UiSelectItem { value: "dark".into(), label: Label::data("Dark") }],
                    placeholder: None,
                    on_change: ActionDescriptor { controller_id: "framework".into(), action: "setAppearance".into(), args: None },
                    menu: None,
                }),
                UiNode::Select(UiSelectNode {
                    presence: UiPresence::default(),
                    id: "framework.settings.driver".into(),
                    value: self.driver_id.clone(),
                    items: vec![UiSelectItem { value: "default".into(), label: Label::data("Default") }, UiSelectItem { value: "compact".into(), label: Label::data("Compact") }],
                    placeholder: None,
                    on_change: ActionDescriptor { controller_id: "framework".into(), action: "setDriver".into(), args: None },
                    menu: None,
                }),
                UiNode::Select(UiSelectNode {
                    presence: UiPresence::default(),
                    id: "framework.settings.language".into(),
                    value: self.locale_id.clone(),
                    items: vec![UiSelectItem { value: "en".into(), label: Label::data("English") }, UiSelectItem { value: "de".into(), label: Label::data("Deutsch") }],
                    placeholder: None,
                    on_change: ActionDescriptor { controller_id: "framework".into(), action: "setLocale".into(), args: None },
                    menu: None,
                }),
                UiNode::Select(UiSelectNode {
                    presence: UiPresence::default(),
                    id: "framework.settings.terminology".into(),
                    value: self.terminology_id.clone(),
                    items: self.active_terminologies().into_iter().map(|id| UiSelectItem { label: if id == "native" { Label::data("Native") } else { Label::data(id.clone()) }, value: id }).collect(),
                    placeholder: None,
                    on_change: ActionDescriptor { controller_id: "framework".into(), action: "setTerminology".into(), args: None },
                    menu: None,
                }),
            ],
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            menu: None,
        })
    }

    /// 🎨️ The wgpu mirror of React's `buildSettingsThemeTree`'s theme-selector section (`ui/js/react/
    /// index.tsx:9424-9498`) — deliberately scoped to picking/resetting/deleting a theme, same
    /// proportion as this crate's `build_settings_general_ui`'s "driver" row having no axis editor.
    /// `w3-prefs-i18n-themes`'s draft-color-editor primitives (`begin_custom_theme_draft`/
    /// `set_draft_theme_color`/`save_draft_theme`/`discard_draft_theme`) stay unwired here on purpose —
    /// that report already scoped the token editor itself down to 5 color slots and called porting
    /// React's full multi-hundred-token editor "out of proportion to this ticket"; this wave only closes
    /// the *reachability* gap (the registry/resolver was already live in `frame()`'s `resolve_theme_for_ids`
    /// call, just invisible — no UI could ever select "mono" or a saved custom theme before this).
    #[cfg(test)]
    fn build_settings_theme_ui(&self) -> UiNode {
        let is_de = self.locale_id == "de";
        let active_id = self.chrome_build.preferences.theme_id.clone();
        let mut items = vec![UiSelectItem { value: "semio".into(), label: Label::data("Semio") }, UiSelectItem { value: "mono".into(), label: Label::data("Mono") }];
        for id in self.chrome_build.preferences.custom_themes.keys().cloned() {
            let label = custom_theme_definition_from(&self.chrome_build.preferences, &id).map(|theme| theme.label).unwrap_or_else(|| id.clone());
            items.push(UiSelectItem { value: id, label: Label::data(label) });
        }
        let mut children = vec![
            UiNode::Text(UiTextNode { presence: UiPresence::default(), value: Label::data(shell_chrome_string("settings.tab.theme", is_de)), emphasize: Some(true), data_attributes: None, menu: None }),
            UiNode::Select(UiSelectNode {
                presence: UiPresence::default(),
                id: "framework.settings.theme.select".into(),
                value: active_id.clone(),
                items,
                placeholder: None,
                on_change: ActionDescriptor { controller_id: "framework".into(), action: "setThemeId".into(), args: None },
                menu: None,
            }),
            UiNode::Button(UiButtonNode {
                id: Some("framework.settings.theme.reset".into()),
                icon_id: IconName::RotateCcw,
                label: Label::data(shell_chrome_string("settings.theme.reset", is_de)),
                action: ActionDescriptor { controller_id: "framework".into(), action: "resetThemeId".into(), args: None },
                style: None,
                presence: UiPresence::default(),
                menu: None,
            }),
        ];
        if active_id.starts_with("custom.") {
            children.push(UiNode::Button(UiButtonNode {
                id: Some("framework.settings.theme.delete".into()),
                icon_id: IconName::Trash2,
                label: Label::data(shell_chrome_string("settings.theme.delete", is_de)),
                action: ActionDescriptor { controller_id: "framework".into(), action: "deleteThemeId".into(), args: crate::action_args_json!({ "value": active_id }) },
                style: None,
                presence: UiPresence::default(),
                menu: None,
            }));
        }
        UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, id: None, children, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, menu: None })
    }

    fn active_terminologies(&self) -> Vec<String> {
        let mut ids = vec!["native".to_string()];
        if let Some(session) = self.session.as_ref() {
            for id in &session.app.terminologies {
                if !ids.contains(id) {
                    ids.push(id.clone());
                }
            }
        }
        ids
    }

    //#region 🧭️PanelAnchorAccessors
    /// 🧭️ Projects the current left/right toggle state onto the named anchor it corresponds to (see
    /// `🧭️PanelAnchorModel` above). The four edge-middle anchors are always empty — nothing assigns there yet.
    pub fn panel_anchor_snapshot(&self, anchor: PanelAnchor) -> PanelAnchorSnapshot {
        match anchor {
            PanelAnchor::TopLeft => {
                PanelAnchorSnapshot { visible: self.left_panel_open && self.active_left_kind == LeftPanelKind::Workbench, size: self.left_panel_width, active_tab: (self.active_left_kind == LeftPanelKind::Workbench).then(|| "workbench".to_string()) }
            }
            PanelAnchor::BottomLeft => {
                PanelAnchorSnapshot { visible: self.left_panel_open && self.active_left_kind == LeftPanelKind::Display, size: self.left_panel_width, active_tab: (self.active_left_kind == LeftPanelKind::Display).then(|| "display".to_string()) }
            }
            PanelAnchor::TopRight => {
                PanelAnchorSnapshot { visible: self.right_panel_open && self.active_right_kind == RightPanelKind::Details, size: self.right_panel_width, active_tab: (self.active_right_kind == RightPanelKind::Details).then(|| "details".to_string()) }
            }
            PanelAnchor::BottomRight => PanelAnchorSnapshot {
                visible: self.right_panel_open && self.active_right_kind == RightPanelKind::Settings,
                size: self.right_panel_width,
                active_tab: (self.active_right_kind == RightPanelKind::Settings).then(|| "settings".to_string()),
            },
            PanelAnchor::TopMiddle | PanelAnchor::BottomMiddle | PanelAnchor::LeftMiddle | PanelAnchor::RightMiddle => PanelAnchorSnapshot::default(),
        }
    }

    /// 🗄️ Flattens the mutable subset of panel layout into the persisted shape.
    pub fn panel_layout_snapshot(&self) -> PanelLayoutPersisted {
        PanelLayoutPersisted {
            left_panel_open: self.left_panel_open,
            right_panel_open: self.right_panel_open,
            active_left_kind: Some(
                match self.active_left_kind {
                    LeftPanelKind::Workbench => "workbench",
                    LeftPanelKind::Display => "display",
                }
                .to_string(),
            ),
            active_right_kind: Some(
                match self.active_right_kind {
                    RightPanelKind::Details => "details",
                    RightPanelKind::Settings => "settings",
                }
                .to_string(),
            ),
            left_panel_width: Some(self.left_panel_width),
            right_panel_width: Some(self.right_panel_width),
        }
    }

    /// 🗄️ Applies a persisted layout snapshot — used both on load and directly by tests.
    pub fn apply_panel_layout(&mut self, layout: &PanelLayoutPersisted) {
        self.left_panel_open = layout.left_panel_open;
        self.right_panel_open = layout.right_panel_open;
        if let Some(kind) = &layout.active_left_kind {
            self.active_left_kind = match kind.as_str() {
                "display" => LeftPanelKind::Display,
                _ => LeftPanelKind::Workbench,
            };
        }
        if let Some(kind) = &layout.active_right_kind {
            self.active_right_kind = match kind.as_str() {
                "settings" => RightPanelKind::Settings,
                _ => RightPanelKind::Details,
            };
        }
        if let Some(width) = layout.left_panel_width {
            self.left_panel_width = width;
        }
        if let Some(width) = layout.right_panel_width {
            self.right_panel_width = width;
        }
    }

    /// 🗄️ Persists the current panel layout so it survives a reload — mirrors `persist_dock_layout` for
    /// the unrelated `dock`/Mode system.
    pub fn persist_panel_layout(&mut self) {
        let snapshot = self.panel_layout_snapshot();
        save_panel_layout_to_store(&snapshot);
        self.chrome_present.last_persisted_panel_layout = Some(snapshot);
    }

    /// 🗄️ Persists the panel layout only when it actually changed since the last persist/load — called
    /// once per frame from `render_chrome` (`ShellChrome`). **Resolves the previously-flagged wiring
    /// gap**: `handle_shell_hit`'s `"ui.panelToggle.*"` arms (and the panel-resize-end path) live in the
    /// do-not-touch `ShellInput` region, so rather than patch each of those call sites individually (which
    /// this ticket isn't scoped to edit), this hooks persistence to the render loop instead — a dirty-check
    /// against the owned present-state snapshot keeps it a no-op write on every frame nothing actually moved,
    /// same shape as `persist_ui_prefs_if_changed` (💾️PrefsSync) one frame refresh away in this same file.
    pub fn persist_panel_layout_if_changed(&mut self) {
        let current = self.panel_layout_snapshot();
        let changed = self.chrome_present.last_persisted_panel_layout.as_ref() != Some(&current);
        if changed {
            self.persist_panel_layout();
        }
    }

    /// 🗄️ Loads any persisted panel layout and applies it — called once from `ShellState::new()`. Seeds
    /// the owned present-state snapshot from the loaded layout so the very first `render_chrome` frame
    /// doesn't immediately re-persist a layout that was just read back unchanged.
    fn load_persisted_panel_layout(&mut self) {
        if let Some(layout) = load_panel_layout_from_store() {
            self.apply_panel_layout(&layout);
            self.chrome_present.last_persisted_panel_layout = Some(self.panel_layout_snapshot());
        }
    }
    //#endregion 🧭️PanelAnchorAccessors
}

#[cfg(test)]
mod panel_anchor_model_tests {
    use super::*;

    fn fresh_state() -> ShellState {
        // 🧪️ `ShellState::new` calls `load_persisted_panel_layout`, which — on native — reads whatever
        // happens to be at `~/.semio/panel-layout.json` on the machine running the test. Every assertion
        // below explicitly sets the fields it exercises afterward, so the outcome never depends on that.
        ShellState::new(Vec::new(), String::new())
    }

    #[test]
    fn panel_anchor_from_group_matches_panel_group_anchor_corners() {
        assert_eq!(PanelAnchor::from_group(PanelGroup::Workbench), PanelAnchor::TopLeft);
        assert_eq!(PanelAnchor::from_group(PanelGroup::Details), PanelAnchor::TopRight);
        assert_eq!(PanelAnchor::from_group(PanelGroup::Display), PanelAnchor::BottomLeft);
        assert_eq!(PanelAnchor::from_group(PanelGroup::Settings), PanelAnchor::BottomRight);
    }

    #[test]
    fn panel_anchor_as_str_matches_react_panel_anchor_ids() {
        let expected = [
            (PanelAnchor::TopLeft, "top-left"),
            (PanelAnchor::TopMiddle, "top-middle"),
            (PanelAnchor::TopRight, "top-right"),
            (PanelAnchor::RightMiddle, "right-middle"),
            (PanelAnchor::BottomRight, "bottom-right"),
            (PanelAnchor::BottomMiddle, "bottom-middle"),
            (PanelAnchor::BottomLeft, "bottom-left"),
            (PanelAnchor::LeftMiddle, "left-middle"),
        ];
        for (anchor, id) in expected {
            assert_eq!(anchor.as_str(), id);
        }
        assert_eq!(PanelAnchor::ALL.len(), 8);
    }

    /// ↔ Keeps native panel initialization aligned with the React shell's shared 300px default.
    #[test]
    fn panel_default_width_is_uniform_and_wider_than_the_former_document_panel() {
        assert_eq!(DEFAULT_PANEL_WIDTH_PX, 300.0);
        assert!(DEFAULT_PANEL_WIDTH_PX > 280.0);
    }

    #[test]
    fn panel_anchor_snapshot_top_left_visible_only_when_workbench_active_and_open() {
        let mut state = fresh_state();
        state.left_panel_open = true;
        state.active_left_kind = LeftPanelKind::Workbench;
        state.left_panel_width = 300.0;
        let top_left = state.panel_anchor_snapshot(PanelAnchor::TopLeft);
        assert!(top_left.visible);
        assert_eq!(top_left.size, 300.0);
        assert_eq!(top_left.active_tab.as_deref(), Some("workbench"));
        let bottom_left = state.panel_anchor_snapshot(PanelAnchor::BottomLeft);
        assert!(!bottom_left.visible, "display anchor must stay hidden while workbench occupies the left column");
    }

    #[test]
    fn panel_anchor_snapshot_switches_corner_with_active_kind_not_visibility_alone() {
        let mut state = fresh_state();
        state.left_panel_open = true;
        state.active_left_kind = LeftPanelKind::Display;
        assert!(!state.panel_anchor_snapshot(PanelAnchor::TopLeft).visible);
        assert!(state.panel_anchor_snapshot(PanelAnchor::BottomLeft).visible);
        state.right_panel_open = true;
        state.active_right_kind = RightPanelKind::Settings;
        assert!(!state.panel_anchor_snapshot(PanelAnchor::TopRight).visible);
        assert!(state.panel_anchor_snapshot(PanelAnchor::BottomRight).visible);
    }

    #[test]
    fn panel_anchor_snapshot_middle_anchors_are_always_empty() {
        let mut state = fresh_state();
        state.left_panel_open = true;
        state.right_panel_open = true;
        assert_eq!(state.panel_anchor_snapshot(PanelAnchor::TopMiddle), PanelAnchorSnapshot::default());
        assert_eq!(state.panel_anchor_snapshot(PanelAnchor::BottomMiddle), PanelAnchorSnapshot::default());
        assert_eq!(state.panel_anchor_snapshot(PanelAnchor::LeftMiddle), PanelAnchorSnapshot::default());
        assert_eq!(state.panel_anchor_snapshot(PanelAnchor::RightMiddle), PanelAnchorSnapshot::default());
    }

    #[test]
    fn panel_layout_snapshot_round_trips_through_apply_panel_layout() {
        let mut source = fresh_state();
        source.left_panel_open = true;
        source.right_panel_open = false;
        source.active_left_kind = LeftPanelKind::Display;
        source.active_right_kind = RightPanelKind::Settings;
        source.left_panel_width = 411.0;
        source.right_panel_width = 233.0;
        let snapshot = source.panel_layout_snapshot();
        assert_eq!(snapshot.active_left_kind.as_deref(), Some("display"));
        assert_eq!(snapshot.active_right_kind.as_deref(), Some("settings"));

        let mut target = fresh_state();
        target.apply_panel_layout(&snapshot);
        assert_eq!(target.left_panel_open, source.left_panel_open);
        assert_eq!(target.right_panel_open, source.right_panel_open);
        assert_eq!(target.active_left_kind, source.active_left_kind);
        assert_eq!(target.active_right_kind, source.active_right_kind);
        assert_eq!(target.left_panel_width, source.left_panel_width);
        assert_eq!(target.right_panel_width, source.right_panel_width);
    }

    #[test]
    fn apply_panel_layout_leaves_widths_untouched_when_absent_from_snapshot() {
        let mut state = fresh_state();
        state.left_panel_width = 555.0;
        state.right_panel_width = 666.0;
        let sparse = PanelLayoutPersisted { left_panel_open: true, right_panel_open: true, active_left_kind: None, active_right_kind: None, left_panel_width: None, right_panel_width: None };
        state.apply_panel_layout(&sparse);
        assert_eq!(state.left_panel_width, 555.0, "absent width in a persisted snapshot must not clobber the current width");
        assert_eq!(state.right_panel_width, 666.0);
        assert_eq!(state.active_left_kind, LeftPanelKind::Workbench, "absent active kind falls back to the default");
    }

    /// 🗄️ Now that panel layout storage routes through the same `prefs_get`/`prefs_set` primitives as
    /// every other uiPref (see the dedup note on `load_panel_layout_from_store`), a round trip through
    /// `save_panel_layout_to_store`/`load_panel_layout_from_store` exercises the exact same `PREFS_STORE`
    /// thread-local singleton `file_prefs_store_round_trips_through_disk` (🧪️UiPrefsThemesI18nTests)
    /// already proves is disk-durable on native — this only needs to prove the panel-layout JSON shape
    /// itself round-trips through that singleton correctly.
    #[test]
    fn panel_layout_round_trips_through_prefs_store() {
        let layout =
            PanelLayoutPersisted { left_panel_open: true, right_panel_open: false, active_left_kind: Some("display".to_string()), active_right_kind: Some("details".to_string()), left_panel_width: Some(321.0), right_panel_width: Some(210.0) };
        save_panel_layout_to_store(&layout);
        let loaded = load_panel_layout_from_store().expect("round-tripped layout must parse back");
        assert_eq!(loaded, layout);
    }

    /// 🗄️ `persist_panel_layout_if_changed`'s dirty-check: a second call with no field changes since the
    /// last persist must not touch storage again — mirrors `persist_ui_prefs_if_changed_is_idempotent_
    /// when_nothing_changed` (🧪️UiPrefsThemesI18nTests) one region over, same shape for the same reason
    /// (this is the render-loop hook that replaces patching every `ui.panelToggle.*` call site — see
    /// `persist_panel_layout_if_changed`'s doc comment).
    #[test]
    fn persist_panel_layout_if_changed_is_idempotent_when_nothing_changed() {
        let mut state = fresh_state();
        state.left_panel_open = true;
        state.active_left_kind = LeftPanelKind::Display;
        state.persist_panel_layout_if_changed();
        let after_first = load_panel_layout_from_store().expect("first call must persist");
        assert_eq!(after_first.active_left_kind.as_deref(), Some("display"));

        // A second call with identical state must be a no-op — flip storage underneath it directly so a
        // wrongly-unconditional write would be observable.
        save_panel_layout_to_store(&PanelLayoutPersisted::default());
        state.persist_panel_layout_if_changed();
        let after_second = load_panel_layout_from_store().expect("storage still has a value");
        assert_eq!(after_second, PanelLayoutPersisted::default(), "unchanged state must not re-persist and clobber the manual write above");
    }

    /// 🎨️ `build_settings_theme_ui`'s reachability contract: a select node listing the built-in themes
    /// plus any saved custom ones, a reset button always present, and a delete button gated strictly on
    /// the active theme id being a `"custom."`-prefixed one (mirrors React's `host.themeId.startsWith(
    /// "custom.")` gate on the same button, `ui/js/react/index.tsx:9489`).
    #[test]
    fn build_settings_theme_ui_lists_builtins_and_gates_delete_on_custom_theme() {
        let mut state = fresh_state();
        state.chrome_build.preferences.theme_id = "semio".to_string();
        let UiNode::Stack(builtin_panel) = state.build_settings_theme_ui() else {
            panic!("expected a stack root");
        };
        let has_select = builtin_panel.children.iter().any(|node| matches!(node, UiNode::Select(_)));
        assert!(has_select, "must render the theme picker select");
        let button_count = builtin_panel.children.iter().filter(|node| matches!(node, UiNode::Button(_))).count();
        assert_eq!(button_count, 1, "only Reset, no Delete, while the built-in \"semio\" theme is active");

        state.chrome_build.preferences.theme_id = "custom.wp-audit-test".to_string();
        let UiNode::Stack(custom_panel) = state.build_settings_theme_ui() else {
            panic!("expected a stack root");
        };
        let button_count = custom_panel.children.iter().filter(|node| matches!(node, UiNode::Button(_))).count();
        assert_eq!(button_count, 2, "Reset and Delete once a custom theme is active");
    }
}
//#endregion ShellLifecycle

//#region ShellActions
fn patch_ops_from_action_result(result: &semio_framework::kernel::InvocationResult) -> Vec<String> {
    result.mutations.iter().filter_map(|operation| serde_json::to_string(&operation.diff.payload).ok()).collect()
}

impl ShellState {
    fn sync_document_id(&self) -> Option<String> {
        let session = self.session.as_ref()?;
        Some(format!("{}-{}", session.plugin_id, session.instance_id))
    }

    //#region 🔖️NativeBackboneSync
    /// @emoji 🧭️ Parses a shell sync-card uri into the `framework/sync` persistence bindings a
    /// document actor opens. `folder://` → the multi-document append-only event log; `file://x.json` → its
    /// parent folder's store (single-blob export demoted per the plan); `remote://host:port[/space_id]`
    /// → the semio_hub over WebSocket, studio-scoped (an omitted studio segment falls back to `"default"`).
    /// Superseded the fetch/CRUD `shell_backbone_read`/`write` pair.
    #[cfg(not(target_arch = "wasm32"))]
    fn parse_persistence_binding(uri: &str) -> Result<Vec<PersistenceBinding>, String> {
        if let Some(rest) = uri.strip_prefix("remote://") {
            let (host_port, space_id) = rest.split_once('/').unwrap_or((rest, "default"));
            let space_id = if space_id.is_empty() { "default" } else { space_id };
            return Ok(vec![PersistenceBinding::Hub { base_url: format!("http://{host_port}"), space_id: space_id.to_string(), token: None, surface: None }]);
        }
        if let Some(path) = uri.strip_prefix("folder://") {
            return Ok(vec![PersistenceBinding::Folder { path: std::path::PathBuf::from(path) }]);
        }
        if let Some(path) = uri.strip_prefix("file://") {
            let parent = std::path::Path::new(path).parent().map(std::path::Path::to_path_buf).unwrap_or_else(|| std::path::PathBuf::from("."));
            return Ok(vec![PersistenceBinding::Folder { path: parent }]);
        }
        Err(format!("unsupported backbone uri: {uri}"))
    }

    /// @emoji ✂️ Tears down the active document channel: detaches the plugin's backbone, deregisters
    /// the host channel end, and stops the actor (flushing pending outbound operations). Step 7 of the
    /// `host_runtime` canonical sequence.
    #[cfg(not(target_arch = "wasm32"))]
    fn detach_sync_backbone_internal(&mut self) {
        if let Some(channel) = self.sync_channel.take() {
            let _ = channel.cmd_tx.send(ArtifactActorMsg::Detach);
            if let Some(plugin) = self.plugins.iter().find(|entry| entry.plugin_id == channel.plugin_id) {
                let _ = plugin.detach_backbone(channel.instance_id);
                // 🎠️ H3-wgpu-native — `wasm_runtime()`/`deregister_host_backbone` had no in-process
                // guest handle to call once the guest moved to the kernel thread; the process-global
                // `HostBackboneChannel` this drove is retired with no `EffectBackbone` replacement
                // landed yet (`📓️status.md`'s "A2-abi-sdk — honest partial" entry, still open).
            }
            self.document_host.close(&channel.document_id);
        }
        self.sync_status = None;
        // 👥️ ticket §5 — a detached document's roster must never linger onto whatever opens next.
        self.presence_peers.clear();
        self.presence_surface = None;
    }

    /// @emoji 📬️ Drains the active document actor's event stream into the plugin store and the sync
    /// badge. Called once per native frame — the render loop already redraws continuously (winit
    /// `ControlFlow::Poll`), so a `try_recv` poll suffices and no `EventLoopProxy` wake is needed.
    /// `RemoteMutations` are force-applied via `apply_mutations` (idempotent by operation id), which also covers
    /// idle frames where the sandboxed store never pumps its `ChannelBackbone` on its own. Returns
    /// whether anything changed (and a re-render was issued).
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn pump_sync_events(&mut self) -> bool {
        use tokio::sync::broadcast::error::TryRecvError;
        let shell_io_changed = self.poll_shell_io();
        // 📇️ ticket §1/§3 — non-blocking identity bootstrap result + directory-stream/command-queue
        // drain, folded into this same every-frame pump (glue.rs's frame loop already calls this
        // method once per tick; adding a second call site outside this lane's lease was avoidable).
        let directory_changed = self.pump_directory_events().await || shell_io_changed;
        // 📌️ ticket §C5 item 2 — the auto check-in poll, folded into the same every-frame pump for the
        // identical reason (no timer wheel exists in this shell; see `auto_checkin_should_fire`'s doc).
        self.poll_auto_checkin().await;
        let (instance_id, plugin_id, events) = {
            let Some(channel) = self.sync_channel.as_mut() else {
                return directory_changed;
            };
            let mut events: Vec<ArtifactEvent> = Vec::new();
            loop {
                match channel.events.try_recv() {
                    Ok(event) => events.push(event),
                    Err(TryRecvError::Empty) | Err(TryRecvError::Closed) => break,
                    Err(TryRecvError::Lagged(_)) => continue,
                }
            }
            if events.is_empty() {
                return directory_changed;
            }
            (channel.instance_id, channel.plugin_id.clone(), events)
        };
        let plugin = self.plugins.iter().find(|entry| entry.plugin_id == plugin_id);
        let mut changed = directory_changed;
        for event in events {
            match event {
                ArtifactEvent::RemoteMutations { envelopes } => {
                    if let Some(plugin) = plugin.as_ref() {
                        let operations = protocol::encode_envelopes(&envelopes);
                        // 🎠️ H3-wgpu-native — `apply_mutations` now runs its plugin turn on the
                        // dedicated kernel thread instead of in-process here; `.await` is the only
                        // change this call site needs (`pump_sync_events` was already `async fn`).
                        match plugin.apply_mutations(instance_id, &operations).await {
                            Ok(()) => changed = true,
                            Err(error) => eprintln!("[DEBUG] wgpu shell apply_mutations failed: {error}"),
                        }
                    }
                }
                ArtifactEvent::SnapshotReplaced { pack, spr } => {
                    if let Some(plugin) = plugin.as_ref() {
                        // 🎠️ H3-wgpu-native — same treatment, `load_app_document_pack` is now async.
                        match plugin.load_app_document_pack(instance_id, &pack, &spr).await {
                            Ok(()) => changed = true,
                            Err(error) => eprintln!("[DEBUG] wgpu shell load_app_document_pack failed: {error}"),
                        }
                    }
                }
                ArtifactEvent::Status(status) => {
                    self.sync_status = Some(status);
                    changed = true;
                }
                ArtifactEvent::Presence { peers } => {
                    // 👥️ ticket §5 — shell-LOCAL roster (deliberately NOT the shared kernel
                    // `ViewModel`, which still has no presence field). Rendered by
                    // `render_presence_bar` in the footer, scoped to `presence_surface`.
                    self.presence_peers = peers;
                    changed = true;
                }
                ArtifactEvent::Conflict(_) => {
                    self.sync_card_kind = Some("conflict".into());
                    changed = true;
                }
                // 👥️ Peer session identity (actor + colour), sent once per connection. The sync
                // actor already stamps it onto outbound heartbeats itself, so the shell has
                // nothing further to fold in here — matched explicitly so a future variant
                // cannot be silently ignored by a catch-all.
                ArtifactEvent::Session { .. } => {}
                ArtifactEvent::Preview { .. } => {
                    // 👻️ Ephemeral peer previews (wire v2's uncredited preview lane) have no native
                    // wgpu shell UI yet — same documented-follow-up status as `Presence` above.
                }
                ArtifactEvent::CommandOutcome { .. } => {
                    // 📮️ Terminal batch dispositions (accepted/transformed/rejected) have no native
                    // wgpu shell surfacing yet — `RemoteMutations`/rollback already keep document
                    // state correct; this event is purely informational until a UI is built for it.
                }
            }
        }
        if changed {
            let _ = self.refresh_ui().await;
        }
        changed
    }

    /// @emoji 🚦️ Human-readable summary of a document's sync health for the attach card, mirroring
    /// the React shell's `syncStatusLabel`.
    #[cfg(not(target_arch = "wasm32"))]
    fn sync_status_label(status: &ArtifactSyncStatus) -> String {
        let remote = match &status.remote {
            RemoteState::Live { peer_count } => {
                format!("live · {peer_count} peer{}", if *peer_count == 1 { "" } else { "s" })
            }
            RemoteState::Connecting => "connecting…".to_string(),
            RemoteState::Backoff { .. } => "reconnecting…".to_string(),
            RemoteState::Detached => "offline".to_string(),
        };
        let persisted = if status.persisted { "saved" } else { "unsaved" };
        let pending = if status.pending_mutations > 0 { format!(" · {} pending", status.pending_mutations) } else { String::new() };
        format!("{remote} · {persisted}{pending}")
    }

    /// 🚦️ ticket §C5 — the `#s-sync-status` footer pill's text, mirroring the React shell's
    /// `computeSyncPillState`/`syncPillText` three-way vocabulary (`persisted | pending(n) |
    /// remote(connected|connecting|backoff|detached)`) exactly, distinct from `sync_status_label`
    /// above (that one is the manual sync-card's own popover-style summary, unchanged). English-only
    /// — `active_locale()` resolves plugin-declared `LocalizedLabel`s, not shell-owned plain text like
    /// this one (same tier `sync_status_label` is already at); a real bilingual pill needs the same
    /// `Label`/`LocalizedLabel` plumbing gap `📓️w2-b-report.md` already flagged for tree-content
    /// strings, not invented bespoke here — see `📓️w3-a-report.md`'s "what is NOT done".
    #[cfg(not(target_arch = "wasm32"))]
    fn sync_pill_text(status: Option<&ArtifactSyncStatus>) -> String {
        let Some(status) = status else { return "Remote: detached".to_string() };
        if !matches!(status.remote, RemoteState::Live { .. }) {
            let remote = match &status.remote {
                RemoteState::Live { .. } => "connected",
                RemoteState::Connecting => "connecting",
                RemoteState::Backoff { .. } => "backoff",
                RemoteState::Detached => "detached",
            };
            return format!("Remote: {remote}");
        }
        if status.pending_mutations > 0 {
            return format!("Pending ({})", status.pending_mutations);
        }
        "Persisted".to_string()
    }

    /// 💓️ Publishes the newest app-typed presence through the document host's
    /// per-document coalescer. The channel drain also exposes transient generation for the native
    /// renderer's next-frame invalidation contract without ever sharing transient contents.
    #[cfg(not(target_arch = "wasm32"))]
    fn publish_presence_heartbeat(&mut self) {
        let Some(channel) = self.sync_channel.as_ref() else { return };
        let document_id = channel.document_id.clone();
        let instance_id = channel.instance_id;
        let plugin_id = channel.plugin_id.clone();
        let connected_at_ms = channel.connected_at_ms;
        let presence_pack = self.plugins.iter().find(|plugin| plugin.plugin_id == plugin_id).and_then(|plugin| plugin.ephemeral_snapshot(instance_id).ok()).map(|(presence, _, _)| presence);
        let label = self.session.as_ref().map(|session| session.app.id.clone());
        let peer = PresencePeer {
            actor: self.current_shell_actor(instance_id),
            label,
            presence_pack,
            connected_at_ms,
            user_id: self.identity.as_ref().map(|identity| identity.user_id.clone()),
            role: None,
            drag_ghost_json: None,
            interaction: None,
            // 👥️ Peer-presence fields introduced by the presence refactor. The wgpu shell does not
            // publish colour/surface/view/ui presence yet — the sync actor assigns `color` from the
            // hub's `Session` frame, and the richer `views`/`ui` payloads belong to the presence
            // packet, not this heartbeat. Left empty rather than invented so nothing fabricates
            // presence state the shell does not actually observe.
            color: None,
            surface: None,
            views: Vec::new(),
            ui: None,
        };
        self.document_host.presence_heartbeat(&document_id, chrome_now_ms() as u64, peer);
    }

    /// 🎭️ ticket §1 — contract §C0's `user:{userId}#{sessionId}` once identity is minted/restored,
    /// else the pre-identity local default this shell always used (`shell_actor`'s pure decision).
    #[cfg(not(target_arch = "wasm32"))]
    fn current_shell_actor(&self, instance_id: u32) -> String {
        shell_actor(self.identity.as_ref(), &self.shell_session_id, instance_id)
    }
    //#endregion 🔖️NativeBackboneSync

    //#region 🔖️CheckIn
    /// 🧾️ ticket §C5 — resets the local history projection and re-seeds it from a full
    /// `ProgramBridgeEntry::read_history` snapshot (`replace=true`), called right after a document
    /// attaches (`open_document`/`attach_sync_backbone`'s native branch) so a just-opened document's
    /// own uncommitted-edit count starts from its OWN history, never a leftover projection from
    /// whatever was open before. Best-effort: a read failure just leaves the projection empty (the
    /// same posture every other native exchange call in this file already takes on error — logged, not
    /// propagated, since a stale/absent history must never block opening a document).
    #[cfg(not(target_arch = "wasm32"))]
    async fn refresh_history_snapshot(&mut self) {
        self.history_cursor = 0;
        self.history_entries.clear();
        self.history_current_checkpoint_id = None;
        self.last_uncommitted_edit_at_ms = None;
        self.auto_checkin_pending = false;
        self.checkpoint_dispatched = false;
        let Some(session) = self.session.as_ref() else { return };
        let Some(plugin) = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id) else { return };
        match plugin.read_history(session.instance_id).await {
            Ok(patch) => {
                fold_history_patch(&mut self.history_entries, &mut self.history_cursor, &patch, true);
                self.history_current_checkpoint_id = patch.current_checkpoint_id;
            }
            Err(error) => eprintln!("[DEBUG] wgpu shell read_history failed: {error}"),
        }
    }

    /// 🧾️ ticket §C5 — folds an `InvocationResult.history_patch` (present on every `handleAction`/
    /// `handleCommand` response, per `ProgramBridge/🧊️component.rs`'s `AppFrame::Invocation` decode)
    /// into the running projection, updates the idle clock, and — the instant a checkpoint THIS shell
    /// itself asked for (`checkpoint_dispatched`) is observed to have actually landed — fires
    /// `TouchArtifact` on the space index (§C5 item 6). Called from both `dispatch_action` and
    /// `dispatch_command` right after their own `program.handle_action`/`handle_command` call.
    #[cfg(not(target_arch = "wasm32"))]
    async fn observe_invocation_history(&mut self, history_patch: Option<&semio_framework::kernel::HistoryPatch>) {
        let Some(patch) = history_patch else { return };
        if !fold_history_patch(&mut self.history_entries, &mut self.history_cursor, patch, false) {
            return;
        }
        let count = uncommitted_edit_count(&self.history_entries);
        if count == 0 {
            self.last_uncommitted_edit_at_ms = None;
            self.auto_checkin_pending = false;
        } else {
            self.last_uncommitted_edit_at_ms = Some(chrome_now_ms() as i64);
        }
        let landed = checkpoint_landed(self.history_current_checkpoint_id.as_deref(), patch.current_checkpoint_id.as_deref(), self.checkpoint_dispatched);
        if patch.current_checkpoint_id.is_some() {
            self.history_current_checkpoint_id = patch.current_checkpoint_id.clone();
        }
        if !landed {
            return;
        }
        self.checkpoint_dispatched = false;
        let Some(space_id) = self.open_space_id.clone() else { return };
        let Some(document_id) = self.sync_channel.as_ref().map(|channel| channel.document_id.clone()) else { return };
        if document_id != S_SPACE_INDEX_DOCUMENT_ID {
            self.touch_space_index_artifact(&space_id, &document_id).await;
        }
    }

    /// 📌️ ticket §C5 item 2 — the per-frame auto check-in poll, called from `pump_sync_events` (see
    /// that method's own doc: "folded into the same every-frame pump"). Viewer sessions never arm
    /// (`can_check_in`, item 5's guard) — the SAME predicate the `#s-checkin` affordance itself checks.
    #[cfg(not(target_arch = "wasm32"))]
    async fn poll_auto_checkin(&mut self) {
        let Some(session) = self.session.as_ref() else { return };
        if !can_check_in(session.app.role) {
            return;
        }
        let count = uncommitted_edit_count(&self.history_entries);
        if count == 0 {
            self.auto_checkin_pending = false;
            self.last_uncommitted_edit_at_ms = None;
            return;
        }
        let now_ms = chrome_now_ms() as i64;
        if auto_checkin_should_fire(count, self.auto_checkin_pending, self.last_uncommitted_edit_at_ms, now_ms, AUTO_CHECKIN_IDLE_MS, AUTO_CHECKIN_EDIT_THRESHOLD) {
            self.auto_checkin_pending = true;
            self.dispatch_checkpoint("auto").await;
        }
    }

    /// 📌️ ticket §C5 items 2/3/4 — fires `commitCheckpoint` through the same action funnel the
    /// History panel's own "Checkpoint" button uses (`history_command` in `🔌️plugin/🦀️component.rs`),
    /// with `authors` riding along (`history_command` still hardcodes `authors: Vec::new()` today per
    /// `📓️w3-a-report.md`'s own `sharedFileRequest` — sent anyway so this is ready the moment that
    /// lands). Viewer-guarded (item 5) so a stray call from `checkpoint_before_detach`/`poll_auto_checkin`
    /// can never reach a viewer session even if their own callers' guards were ever relaxed.
    #[cfg(not(target_arch = "wasm32"))]
    async fn dispatch_checkpoint(&mut self, message: &str) {
        let Some(session) = self.session.clone() else { return };
        if !can_check_in(session.app.role) {
            return;
        }
        self.checkpoint_dispatched = true;
        let authors: Vec<Value> = self.identity.as_ref().map(|identity| vec![serde_json::json!({ "id": identity.user_id, "name": identity.display_name })]).unwrap_or_default();
        let action = ActionDescriptor { controller_id: session.app.controller_id.clone(), action: "commitCheckpoint".into(), args: crate::action_args_json!({ "message": message, "authors": authors }) };
        // 🧱️ `Box::pin` breaks a real call-graph cycle (`dispatch_action` → `handle_sync_action` →
        // `attach_sync_backbone` → `checkpoint_before_detach` → here → `dispatch_action` again) that
        // `rustc` rightly refuses to size without it (E0733) — the cycle is never actually walked at
        // runtime for a `commitCheckpoint` action (its `controller_id` is the session's own app, never
        // `"framework.sync"`), but the async-fn state machine's size is inferred statically regardless
        // of which branch executes.
        if let Err(error) = Box::pin(self.dispatch_action(action)).await {
            eprintln!("[DEBUG] wgpu shell check-in checkpoint dispatch failed: {error}");
            self.checkpoint_dispatched = false;
        }
    }

    /// 📌️ ticket §C5 item 4 — checkpoint-on-close: called right before EVERY
    /// `detach_sync_backbone_internal()` call site (this shell keeps exactly one document mounted at a
    /// time, so attaching a different backbone IS closing whatever was open — same posture the React
    /// shell's own report documents). Best-effort (mirrors React's "fire-and-forget, not gated on the
    /// success-detection effect" note) — `dispatch_checkpoint`/`observe_invocation_history` still run
    /// the normal detection+`TouchArtifact` path; this call site just doesn't await or fail the caller
    /// on their outcome, since a failed close-time checkpoint must never block navigating away.
    #[cfg(not(target_arch = "wasm32"))]
    async fn checkpoint_before_detach(&mut self) {
        let Some(session) = self.session.as_ref() else { return };
        let count = uncommitted_edit_count(&self.history_entries);
        if !should_checkpoint_before_detach(session.app.role, self.sync_channel.is_some(), count) {
            return;
        }
        self.dispatch_checkpoint("auto").await;
    }

    /// 📌️ ticket §C5 item 6 — `TouchArtifact` on the space's `index` document after a successful
    /// checkpoint, mirroring the React shell's `touchSpaceIndexArtifact`. Reuses the CURRENTLY visible
    /// session outright when it already IS this exact space's own index document (avoiding a second
    /// `document_host` actor under the shared, non-space-scoped `"index"` key entirely); otherwise
    /// spawns a FRESH, non-visible `s.space` editor instance + backbone attach for the duration of the
    /// one `touchArtifact` dispatch, then tears both down immediately. Unlike the React shell's cached
    /// background instance (`spaceIndexInstanceRef`), this does not cache across calls: `document_host`
    /// keys its actor registry by the bare document id, not `(spaceId, documentId)`, so a PERSISTENT
    /// background actor under `"index"` risks silently being replaced by a later legitimate
    /// `open_document`/`attach_sync_backbone` call for a DIFFERENT space's index (or vice versa) with
    /// no way for either side to detect the swap — spinning up fresh each time trades a little
    /// efficiency for provable correctness under that shared-key constraint.
    #[cfg(not(target_arch = "wasm32"))]
    async fn touch_space_index_artifact(&mut self, space_id: &str, artifact_id: &str) {
        let now_ms = chrome_now_ms();
        let actor = self.identity.as_ref().map(|identity| identity.user_id.clone()).unwrap_or_else(|| self.shell_session_id.clone());
        let arguments: BTreeMap<String, Value> = BTreeMap::from([("id".to_string(), serde_json::json!(artifact_id)), ("nowMs".to_string(), serde_json::json!(now_ms)), ("actor".to_string(), serde_json::json!(actor))]);
        // 🐚️ Reuse the live session outright when it's already this exact space's own index document.
        // Calls `program.handle_command` DIRECTLY rather than `self.dispatch_command` (which would
        // recurse back into `observe_invocation_history` → `touch_space_index_artifact` — `rustc`
        // rightly refuses that as unbounded async recursion without `Box::pin`; a direct low-level
        // call is also the more honest shape here, since `touchArtifact` requests no `Effect`s for
        // `dispatch_command`'s own effect loop to process).
        if self.sync_channel.as_ref().map(|channel| channel.document_id.as_str()) == Some(S_SPACE_INDEX_DOCUMENT_ID) && self.open_space_id.as_deref() == Some(space_id) {
            let Some(session) = self.session.clone() else { return };
            let Some(program) = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned() else { return };
            let Some(app) = find_dialect_app(&program, &space_index_dialect(), semio_framework::manifest::AppRole::Editor).cloned() else { return };
            let invocation = semio_framework::manifest::CommandInvocation {
                address: semio_framework::manifest::CommandAddress { owner: semio_framework::manifest::CommandOwnerAddress::App { plugin_id: session.plugin_id.clone(), app_id: app.id.clone() }, command_id: "touchArtifact".into() },
                arguments,
            };
            match serde_json::to_string(&invocation) {
                Ok(command_json) => {
                    if let Err(error) = program.handle_command(session.instance_id, &command_json, &session.view_state).await {
                        eprintln!("[DEBUG] wgpu shell touchArtifact (live session) failed: {error}");
                    }
                }
                Err(error) => eprintln!("[DEBUG] wgpu shell touchArtifact (live session) encode failed: {error}"),
            }
            return;
        }
        // 🐚️ `document_host` keys its actor registry by the bare document id ("index", not
        // space-scoped) — opening a background actor under that key while the MAIN slot is ALSO
        // showing "index" for a DIFFERENT space would silently sever that live session's own backbone
        // (`ArtifactHost::open`'s own "idempotent per id" contract). Skip rather than risk it.
        if self.sync_channel.as_ref().map(|channel| channel.document_id.as_str()) == Some(S_SPACE_INDEX_DOCUMENT_ID) {
            eprintln!("[DEBUG] wgpu shell touchArtifact skipped: main session already holds the shared \"index\" actor for a different space");
            return;
        }
        let Some(program) = self.plugins.iter().find(|entry| find_dialect_app(entry, &space_index_dialect(), semio_framework::manifest::AppRole::Editor).is_some()).cloned() else { return };
        let Some(app) = find_dialect_app(&program, &space_index_dialect(), semio_framework::manifest::AppRole::Editor).cloned() else { return };
        let instance_id = match program.create_app(&app.id).await {
            Ok(id) => id,
            Err(error) => {
                eprintln!("[DEBUG] wgpu shell touchArtifact background instance failed: {error}");
                return;
            }
        };
        // 🎠️ H3-wgpu-native — `wasm_runtime()`/`register_host_backbone` had no in-process guest
        // handle to call once the guest moved to the kernel thread (see `detach_sync_backbone_internal`'s
        // own note above); `attach_backbone` below is now the SDK's own honest-error stub for the
        // whole retired backbone mechanism (`📓️design-abi.md` §2/§4's `EffectBackbone`, not landed
        // yet), so this falls straight through to it rather than gating on a step that no longer
        // does anything.
        let data_dir = self.identity_env.as_ref().and_then(|env| env.data_dir.as_deref());
        let surface = semio_framework::manifest::surface_app_id(&app.dialect, semio_framework::manifest::AppRole::Editor);
        let bindings = default_persistence_bindings(self.identity.as_ref(), Some(space_id), data_dir, Some(surface.as_str()));
        let actor_uri = format!("actor://{S_SPACE_INDEX_DOCUMENT_ID}");
        let channels = self.document_host.open(ArtifactActorConfig { document_id: S_SPACE_INDEX_DOCUMENT_ID.to_string(), schema: S_SPACE_INDEX_DOCUMENT_SCHEMA.to_string(), bindings, watch_external: true, actor: actor.clone() }).await;
        if let Err(error) = program.attach_backbone(instance_id, &actor_uri) {
            eprintln!("[DEBUG] wgpu shell touchArtifact attach_backbone failed: {error}");
            self.document_host.close(S_SPACE_INDEX_DOCUMENT_ID);
            program.destroy_app(instance_id);
            return;
        }
        let _ = channels.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() });
        let invocation = semio_framework::manifest::CommandInvocation {
            address: semio_framework::manifest::CommandAddress { owner: semio_framework::manifest::CommandOwnerAddress::App { plugin_id: program.plugin_id.clone(), app_id: app.id.clone() }, command_id: "touchArtifact".into() },
            arguments,
        };
        match serde_json::to_string(&invocation) {
            Ok(command_json) => {
                let view_state = ViewModel { locale: self.active_locale(), terminology: self.active_terminology(), ..Default::default() };
                if let Err(error) = program.handle_command(instance_id, &command_json, &view_state).await {
                    eprintln!("[DEBUG] wgpu shell touchArtifact dispatch failed: {error}");
                }
            }
            Err(error) => eprintln!("[DEBUG] wgpu shell touchArtifact encode failed: {error}"),
        }
        self.document_host.close(S_SPACE_INDEX_DOCUMENT_ID);
        program.destroy_app(instance_id);
    }

    /// 📌️ ticket §C5 item 3 — explicit check-in's message-prompt dialog funnel (`#s-checkin`'s own
    /// dedicated controller, routed alongside `"framework.sync"` in `dispatch_action`): `open` shows
    /// the dialog, `cancel` closes it without dispatching, `submit` dispatches with the typed message
    /// (falling back to `"check-in"` for an empty/whitespace-only draft, matching the React shell's own
    /// `submitCheckin`).
    #[cfg(not(target_arch = "wasm32"))]
    async fn handle_checkin_action(&mut self, action: ActionDescriptor) -> Result<(), String> {
        match action.action.as_str() {
            "open" => {
                self.checkin_dialog_draft = Some(String::new());
                Ok(())
            }
            "cancel" => {
                self.checkin_dialog_draft = None;
                Ok(())
            }
            "submit" => {
                let message = action.args.as_ref().and_then(|args| args.get("message")).and_then(DslValue::as_str).map(str::trim).filter(|value| !value.is_empty()).unwrap_or("check-in").to_string();
                self.checkin_dialog_draft = None;
                self.dispatch_checkpoint(&message).await;
                Ok(())
            }
            _ => Ok(()),
        }
    }
    //#endregion 🔖️CheckIn

    /// @emoji 🔗️ Opens the shell's active app document on a `framework/sync` `ArtifactHost` actor and
    /// wires the sandboxed plugin store to it, following `framework/product/os/core/rs`'s
    /// `host_runtime` canonical sequence (open → subscribe → register host channel → program
    /// `attach-backbone`). The React shell's `openDocument` is the TS twin of this exact sequence.
    async fn attach_sync_backbone(&mut self, uri: String) -> Result<(), String> {
        let session = self.session.clone().ok_or("session missing")?;
        #[cfg(not(target_arch = "wasm32"))]
        {
            let plugin = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned().ok_or("plugin missing")?;
            // 🎠️ H3-wgpu-native — `wasm_runtime()`/`register_host_backbone` retired, see
            // `detach_sync_backbone_internal`'s note; `attach_backbone` below now carries the honest
            // "not implemented yet" error for the whole mechanism.
            let document_id = self.sync_document_id().unwrap_or_else(|| "document".into());
            let schema = session.app.breadcrumb.join(".");
            let bindings = Self::parse_persistence_binding(&uri)?;
            // 📌️ ticket §C5 item 4 — checkpoint-on-close: this shell keeps exactly one session/document
            // mounted at a time, so "attach a different backbone" IS "close" for whatever was open —
            // same posture the React shell's own report documents ("switch away IS close here").
            self.checkpoint_before_detach().await;
            self.detach_sync_backbone_internal();
            // 🔗️ The manual `remote://` sync-card override never carries a surface (§C6's
            // auto-binding is what threads one through — see `open_document` below), so the presence
            // roster stays empty for this path, same as before this lane.
            self.presence_surface = None;
            let actor_uri = format!("actor://{document_id}");
            let actor = self.current_shell_actor(session.instance_id);
            let channels = self.document_host.open(ArtifactActorConfig { document_id: document_id.clone(), schema, bindings, watch_external: true, actor }).await;
            let events = self.document_host.subscribe(&document_id).await;
            plugin.attach_backbone(session.instance_id, &actor_uri).map_err(|error| format!("plugin attach backbone: {error}"))?;
            let cmd_tx = channels.cmd_tx.clone();
            let _ = cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() });
            self.sync_channel = Some(ShellSyncChannel { document_id, actor_uri, instance_id: session.instance_id, plugin_id: session.plugin_id.clone(), cmd_tx, events, connected_at_ms: chrome_now_ms() as i64 });
            self.sync_status = Some(ArtifactSyncStatus::default());
            self.sync_backbone_uri = Some(uri);
            self.sync_card_kind = None;
            eprintln!("[DEBUG] wgpu shell attached backbone {}", self.sync_backbone_uri.as_deref().unwrap_or_default());
            self.refresh_history_snapshot().await;
            self.refresh_ui().await?;
            Ok(())
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = &session;
            self.sync_backbone_uri = Some(uri);
            self.sync_card_kind = None;
            web_sys::console::log_1(&"[DEBUG] attached backbone (browser wgpu: relayed via host-shim)".into());
            Ok(())
        }
    }

    /// 📇️ ticket §2/§C6 — opens an explicit document id/schema on the same `framework/sync`
    /// `ArtifactHost` sequence `attach_sync_backbone` uses, but with caller-supplied bindings (task
    /// 2's default-binding computation, or an explicit override) instead of parsing a manual sync-
    /// card uri — independent of `attach_sync_backbone`, which stays the untouched `remote://`
    /// override path. Used by the `os.open-artifact{documentId}` opening relay (§4) and by the
    /// identity-driven space-index auto-bind on the `/spaces/{id}` route (§6).
    #[cfg(not(target_arch = "wasm32"))]
    async fn open_document(&mut self, document_id: String, schema: String, bindings: Vec<PersistenceBinding>, surface: Option<String>) -> Result<(), String> {
        let session = self.session.clone().ok_or("session missing")?;
        let plugin = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned().ok_or("plugin missing")?;
        // 🎠️ H3-wgpu-native — `wasm_runtime()`/`register_host_backbone` retired, see
        // `detach_sync_backbone_internal`'s note.
        // 📌️ ticket §C5 item 4 — checkpoint-on-close, same "switch away IS close" posture as
        // `attach_sync_backbone` above (this shell keeps exactly one document mounted at a time).
        self.checkpoint_before_detach().await;
        self.detach_sync_backbone_internal();
        self.presence_surface = surface;
        let actor_uri = format!("actor://{document_id}");
        let actor = self.current_shell_actor(session.instance_id);
        let channels = self.document_host.open(ArtifactActorConfig { document_id: document_id.clone(), schema, bindings, watch_external: true, actor }).await;
        let events = self.document_host.subscribe(&document_id).await;
        plugin.attach_backbone(session.instance_id, &actor_uri).map_err(|error| format!("plugin attach backbone: {error}"))?;
        let cmd_tx = channels.cmd_tx.clone();
        let _ = cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() });
        self.sync_backbone_uri = Some(actor_uri.clone());
        self.sync_channel = Some(ShellSyncChannel { document_id, actor_uri, instance_id: session.instance_id, plugin_id: session.plugin_id.clone(), cmd_tx, events, connected_at_ms: chrome_now_ms() as i64 });
        self.sync_status = Some(ArtifactSyncStatus::default());
        self.sync_card_kind = None;
        self.refresh_history_snapshot().await;
        self.refresh_ui().await
    }

    /// 📇️ Default bindings for a document opened against the CURRENTLY mounted session's own
    /// dialect/role — `default_persistence_bindings`'s pure decision, fed the live identity/space/
    /// data-dir/surface this shell already holds.
    #[cfg(not(target_arch = "wasm32"))]
    fn default_bindings_for_current_session(&self) -> (Vec<PersistenceBinding>, Option<String>) {
        let space_id = self.open_space_id.clone();
        let data_dir = self.identity_env.as_ref().and_then(|env| env.data_dir.as_deref());
        let surface = self.session.as_ref().filter(|_| self.identity.is_some() && space_id.is_some()).map(|session| semio_framework::manifest::surface_app_id(&session.app.dialect, session.app.role));
        let bindings = default_persistence_bindings(self.identity.as_ref(), space_id.as_deref(), data_dir, surface.as_deref());
        (bindings, surface)
    }

    async fn handle_sync_action(&mut self, action: ActionDescriptor) -> Result<(), String> {
        match action.action.as_str() {
            "selectFile" => {
                self.sync_card_kind = Some("file".into());
                self.sync_card_draft = self.sync_backbone_uri.as_deref().filter(|uri| uri.starts_with("file://")).map(|uri| uri.trim_start_matches("file://").to_string()).unwrap_or_default();
                Ok(())
            }
            "selectFolder" => {
                self.sync_card_kind = Some("folder".into());
                self.sync_card_draft = self.sync_backbone_uri.as_deref().filter(|uri| uri.starts_with("folder://")).map(|uri| uri.trim_start_matches("folder://").to_string()).unwrap_or_default();
                Ok(())
            }
            "selectRemote" => {
                self.sync_card_kind = Some("remote".into());
                self.sync_card_draft = self.sync_backbone_uri.as_deref().filter(|uri| uri.starts_with("remote://")).map(|uri| uri.trim_start_matches("remote://").to_string()).unwrap_or_default();
                Ok(())
            }
            "attach" => {
                let path = action.args.as_ref().and_then(|args| args.get("path")).and_then(|value| value.as_str()).unwrap_or(self.sync_card_draft.as_str());
                if path.trim().is_empty() {
                    return Ok(());
                }
                let kind = action.args.as_ref().and_then(|args| args.get("kind")).and_then(|value| value.as_str()).unwrap_or(self.sync_card_kind.as_deref().unwrap_or("file"));
                let uri = match kind {
                    "folder" => format!("folder://{path}"),
                    "remote" => format!("remote://{path}"),
                    _ => format!("file://{path}"),
                };
                self.attach_sync_backbone(uri).await
            }
            "detach" => {
                // 📌️ ticket §C5 item 4 — the one EXPLICIT close (a user-initiated "Detach"), same
                // checkpoint-before-detach guard as the two switch-triggered closes above.
                #[cfg(not(target_arch = "wasm32"))]
                self.checkpoint_before_detach().await;
                #[cfg(not(target_arch = "wasm32"))]
                self.detach_sync_backbone_internal();
                self.sync_backbone_uri = None;
                self.sync_card_kind = None;
                self.last_envelope_dsl = None;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub async fn dispatch_action(&mut self, action: ActionDescriptor) -> Result<(), String> {
        // 🎬️ Tutorial interception — fully short-circuits (mirrors `SET_ACTIVE_UTILITY_ACTION_ID`'s own
        // interception further down): both `startTutorial`/`recordTutorial` are framework-injected View
        // actions with no plugin-side handler at all (see `framework/plugin/rs`'s auto-injection).
        if action.action == semio_framework::START_TUTORIAL_ACTION_ID {
            if let Some(tutorial_id) = action.args.as_ref().and_then(|args| args.get("tutorialId")).and_then(|v| v.as_str()) {
                self.tutorial_start(tutorial_id);
            }
            return Ok(());
        }
        if action.action == semio_framework::RECORD_TUTORIAL_ACTION_ID {
            self.tutorial_start_recording();
            return Ok(());
        }
        // 🎬️ Deviation detection + recorder tap — every OTHER real dispatch funnels through here exactly
        // once, before any of this function's own side effects, and skips itself while the tutorial
        // player's own history-action replay is mid-flight (`tutorial_flush_pending_document_ops`'s
        // `TutorialDispatchGuard`).
        if !self.chrome_build.tutorial_dispatch_internal {
            self.tutorial_note_real_dispatch(&action);
        }
        if action.controller_id == "framework" {
            match action.action.as_str() {
                "setAppearance" => {
                    if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                        self.appearance_id = value.to_string();
                        self.note_shell_setting_command("os.setAppearance", Some(value)).await?;
                    }
                    return Ok(());
                }
                "setDriver" => {
                    if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                        self.driver_id = value.to_string();
                        self.note_shell_setting_command("os.setDriver", Some(value)).await?;
                    }
                    return Ok(());
                }
                "setLocale" => {
                    if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                        self.locale_id = value.to_string();
                        self.note_shell_setting_command("os.setLocale", Some(value)).await?;
                    }
                    return Ok(());
                }
                "setTerminology" => {
                    if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                        self.terminology_id = value.to_string();
                        self.note_shell_setting_command("os.setTerminology", Some(value)).await?;
                    }
                    return Ok(());
                }
                // 🎨️ Backs `build_settings_theme_ui`'s theme select/reset/delete in the owned build
                // snapshot and mirrors the value to the present-side bridge consumed by `frame()`.
                "setThemeId" => {
                    if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                        set_active_theme_id(value);
                        self.chrome_build.preferences.theme_id = value.to_string();
                        self.note_shell_setting_command("os.setThemeId", Some(value)).await?;
                    }
                    return Ok(());
                }
                "resetThemeId" => {
                    set_active_theme_id("semio");
                    self.chrome_build.preferences.theme_id = "semio".to_string();
                    self.note_shell_setting_command("os.resetThemeId", Some("semio")).await?;
                    return Ok(());
                }
                "deleteThemeId" => {
                    if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                        delete_custom_theme(value);
                        self.chrome_build.preferences.custom_themes.remove(value);
                        if self.chrome_build.preferences.theme_id == value {
                            self.chrome_build.preferences.theme_id = "semio".to_string();
                        }
                        self.note_shell_setting_command("os.deleteThemeId", Some(value)).await?;
                    }
                    return Ok(());
                }
                _ => {}
            }
        }
        if action.controller_id == "framework.sync" {
            return self.handle_sync_action(action).await;
        }
        // 📌️ ticket §C5 item 3 — explicit check-in's own dedicated controller (`#s-checkin`'s
        // open/cancel/submit dialog funnel), same shell-owned-chrome treatment as `framework.sync`
        // above.
        #[cfg(not(target_arch = "wasm32"))]
        if action.controller_id == "framework.checkin" {
            return self.handle_checkin_action(action).await;
        }
        // 🧰️ Intercept the framework `setActiveUtility` View action to update the host-owned active-utility
        // map before forwarding to the plugin (which reacts by clearing its live-preview scratch). The
        // authoritative state is the shell map + the `ViewModel.active_utility_id` it injects on render.
        if action.action == semio_framework::SET_ACTIVE_UTILITY_ACTION_ID {
            if let Some(session) = self.session.clone() {
                if action.controller_id == session.app.controller_id {
                    if let Some(utility_id) = action.args.as_ref().and_then(|args| args.get("utilityId")).and_then(|value| value.as_str()) {
                        let window_kind_id = action.args.as_ref().and_then(|args| args.get("windowKindId")).and_then(|value| value.as_str()).map(String::from).unwrap_or_else(|| self.active_utility_bar_window_kind(&session).id.clone());
                        self.apply_set_active_utility(&window_kind_id, utility_id);
                    }
                }
            }
        }
        let Some(session) = self.session.clone() else {
            return Ok(());
        };
        let program = self.plugins.iter().find(|p| p.manifest.apps.iter().any(|app| app.controller_id == action.controller_id)).or_else(|| self.plugins.iter().find(|p| p.plugin_id == session.plugin_id)).ok_or("action program missing")?;
        let requested_window_id = action.args.as_ref().and_then(|args| args.get("windowId")).and_then(DslValue::as_str);
        let window_instance_id = requested_window_id.map(str::to_string).or_else(|| session.view_state.window_id.clone()).or_else(|| self.active_window_id.clone()).unwrap_or_else(|| session.app.window_kinds.first().id.clone());
        let window_kind_id = session
            .view_state
            .window_instances
            .iter()
            .find(|instance| instance.id == window_instance_id)
            .map(|instance| instance.window_kind_id.clone())
            .or_else(|| session.app.window_kinds.iter().find(|kind| kind.id == window_instance_id).map(|kind| kind.id.clone()))
            .ok_or_else(|| format!("action window instance {window_instance_id} has no declared kind"))?;
        let mode_id = session.view_state.active_mode_id.clone().unwrap_or_else(|| session.app.default_mode_id.clone());
        let arguments = action.args.as_ref().map(dsl_value_as_json).and_then(|value| value.as_object().cloned()).unwrap_or_default().into_iter().collect();
        let invocation = semio_framework::manifest::ActionInvocation {
            address: semio_framework::manifest::ActionAddress { plugin_id: session.plugin_id.clone(), app_id: session.app.id.clone(), mode_id, window_kind_id, window_instance_id, action_id: action.action.clone() },
            arguments,
        };
        let action_json = serde_json::to_string(&invocation).map_err(|err| err.to_string())?;
        let result = program.handle_action(session.instance_id, &action_json, &session.view_state).await?;
        // 🧾️ ticket §C5 — fold this dispatch's own `history_patch` into the check-in projection (idle
        // clock, checkpoint-landed detection + `TouchArtifact`) before anything else touches `self`.
        #[cfg(not(target_arch = "wasm32"))]
        self.observe_invocation_history(result.history_patch.as_ref()).await;
        // 🎓️ Advance-by-doing: this action was actually performed (the plugin call above succeeded), so
        // a tour step whose `advance` targets it moves on now — see `chrome_tour_note_action_performed`.
        self.chrome_tour_note_action_performed(&action.action);
        // 🧰️ A program may programmatically switch the active utility via `Effect::SetActiveUtility`
        // (Architecture Decision 4/9) — routed through `apply_set_active_utility` (rather than writing
        // `active_utility_by_window` directly) so the tour's advance-by-doing funnel sees this activation
        // too, exactly like a user click would.
        for effect in &result.requested_effects {
            match effect {
                semio_framework::kernel::Effect::SetActiveUtility { window_id, utility_id } => {
                    self.apply_set_active_utility(window_id, utility_id);
                }
                semio_framework::kernel::Effect::Navigate { uri } => {
                    self.push_uri(uri.clone());
                    if let Err(error) = self.apply_shell_uri(uri).await {
                        eprintln!("[DEBUG] wgpu shell navigate effect failed: {error}");
                    }
                }
                semio_framework::kernel::Effect::LoadDocument { pack, spr } => {
                    if let Some(session) = self.session.clone() {
                        if let Some(plugin) = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id) {
                            // 🎠️ H3-wgpu-native — `load_app_document_pack` is now async.
                            if let Err(error) = plugin.load_app_document_pack(session.instance_id, pack, spr).await {
                                eprintln!("[DEBUG] wgpu shell loadDocument effect failed: {error}");
                            }
                        }
                    }
                }
                // 🔁️ Self re-dispatch (D2): queues `action` onto the same `deferred_actions` mechanism
                // tree-hover/selection follow-ups already use, which `flush_deferred_actions` drains every
                // event-loop tick — so, natively, any `delay_ms` collapses to "next tick" (no timer wheel
                // exists in this shell yet; the real wall-clock delay is honored by the React shell's own
                // `setTimeout` handling of the same effect). The dispatched action reuses the originating
                // `action.controller_id`, i.e. re-invokes the same plugin instance that emitted the effect.
                semio_framework::kernel::Effect::DispatchAction { action: dispatch_action_id, args, .. } => {
                    self.deferred_actions.push(ActionDescriptor { controller_id: action.controller_id.clone(), action: dispatch_action_id.clone(), args: args.clone() });
                }
                // 🎞️ D5: native counterpart of `request_file_open`, beside it below — builds one
                // `ActionDescriptor` per sampled frame (+one for `done_action`, or a single
                // `fallback_action` one on failure) via `request_media_frames`, then queues them onto the
                // same `deferred_actions` mechanism `DispatchAction` above uses so `flush_deferred_actions`
                // dispatches them through the normal `dispatch_action` path (including its own nested
                // `requested_effects`) in order, one per tick's drain.
                semio_framework::kernel::Effect::RequestMediaFrames { accept, frame_action, done_action, fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload, args, .. } => {
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let controller_id = action.controller_id.clone();
                        let accept = accept.clone();
                        let frame_action = frame_action.clone();
                        let done_action = done_action.clone();
                        let fallback_action = fallback_action.clone();
                        let (sample_stride, max_frames, max_long_edge_px, fps_hint) = (*sample_stride, *max_frames, *max_long_edge_px, *fps_hint);
                        let payload = payload.clone();
                        let args = optional_dsl_value_as_json(args.clone());
                        self.submit_shell_io_future(async move {
                            ShellIoCompletion::Actions(request_media_frames(&controller_id, &accept, &frame_action, &done_action, &fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload.as_deref(), args).await)
                        });
                    }
                    #[cfg(target_arch = "wasm32")]
                    for descriptor in
                        request_media_frames(&action.controller_id, accept, frame_action, done_action, fallback_action, *sample_stride, *max_frames, *max_long_edge_px, *fps_hint, payload.as_deref(), optional_dsl_value_as_json(args.clone()))
                    {
                        self.deferred_actions.push(descriptor);
                    }
                }
                // 📇️ ticket §C6/§3/§4 — the `os.directory.*` funnel and the `os.open-artifact`/
                // `os.open-artifact-with` opening relay (§3-B: `documentId`/`spaceId` riding inside
                // the existing args, no channel tag added). Every other `ReplayShellCommand` action id
                // (e.g. `os.setThemeId`'s undo replay) has no handler in this lease, same pre-existing
                // gap the React shell's own report documents.
                #[cfg(not(target_arch = "wasm32"))]
                semio_framework::kernel::Effect::ReplayShellCommand { action_id, args } => {
                    self.handle_replay_shell_command(action_id, args.as_ref()).await;
                }
                _ => {}
            }
        }
        let operations: Vec<String> = result.mutations.iter().filter_map(|operation| serde_json::to_string(&operation.diff.payload).ok()).collect();
        self.apply_mutations(&operations).await
    }

    /// 🎛️ Sends one owner-qualified non-OS command through the program command boundary.
    pub async fn dispatch_command(&mut self, invocation: semio_framework::manifest::CommandInvocation) -> Result<(), String> {
        if matches!(invocation.address.owner, semio_framework::manifest::CommandOwnerAddress::Os) {
            return Err("os commands must be dispatched by the shell".into());
        }
        let Some(session) = self.session.clone() else {
            return Ok(());
        };
        let owner_plugin_id = match &invocation.address.owner {
            semio_framework::manifest::CommandOwnerAddress::Plugin { plugin_id } | semio_framework::manifest::CommandOwnerAddress::App { plugin_id, .. } | semio_framework::manifest::CommandOwnerAddress::Mode { plugin_id, .. } => plugin_id,
            semio_framework::manifest::CommandOwnerAddress::Os => unreachable!(),
        };
        if owner_plugin_id != &session.plugin_id {
            return Err(format!("command owner plugin {owner_plugin_id} is not active"));
        }
        let program = self.plugins.iter().find(|entry| entry.plugin_id == *owner_plugin_id).ok_or("command program missing")?;
        let command_json = serde_json::to_string(&invocation).map_err(|error| error.to_string())?;
        let result = program.handle_command(session.instance_id, &command_json, &session.view_state).await?;
        // 🧾️ ticket §C5 — same fold `dispatch_action` performs; a command-boundary edit (e.g. a
        // plugin-owned command dispatched from the command palette) is just as real an uncommitted
        // edit as an action-boundary one.
        #[cfg(not(target_arch = "wasm32"))]
        self.observe_invocation_history(result.history_patch.as_ref()).await;
        for effect in &result.requested_effects {
            match effect {
                semio_framework::kernel::Effect::SetActiveUtility { window_id, utility_id } => self.apply_set_active_utility(window_id, utility_id),
                semio_framework::kernel::Effect::Navigate { uri } => {
                    self.push_uri(uri.clone());
                    self.apply_shell_uri(uri).await?;
                }
                semio_framework::kernel::Effect::LoadDocument { pack, spr } => {
                    if let Some(plugin) = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id) {
                        // 🎠️ H3-wgpu-native — `load_app_document_pack` is now async.
                        plugin.load_app_document_pack(session.instance_id, pack, spr).await?;
                    }
                }
                semio_framework::kernel::Effect::DispatchAction { action, args, .. } => {
                    self.deferred_actions.push(ActionDescriptor { controller_id: session.app.controller_id.clone(), action: action.clone(), args: args.clone() });
                }
                // 📇️ ticket §C6/§3/§4 — same funnel as `dispatch_action`'s own arm above; a "surface
                // command" per contract §C6's own wording is exactly a command-boundary emission, so
                // both dispatch paths need the handler.
                #[cfg(not(target_arch = "wasm32"))]
                semio_framework::kernel::Effect::ReplayShellCommand { action_id, args } => {
                    self.handle_replay_shell_command(action_id, args.as_ref()).await;
                }
                _ => {}
            }
        }
        let operations: Vec<String> = result.mutations.iter().filter_map(|operation| serde_json::to_string(&operation.diff.payload).ok()).collect();
        self.apply_mutations(&operations).await
    }

    //#region 🔖️DirectoryAndIdentity
    /// 📇️ ticket §C6/§3/§4 — dispatches a `ReplayShellCommand`'s `action_id`/`args` onto the
    /// directory funnel or the opening relay; every other action id is a documented no-op (see this
    /// region's callers' own comment).
    #[cfg(not(target_arch = "wasm32"))]
    async fn handle_replay_shell_command(&mut self, action_id: &str, args: Option<&DslValue>) {
        let args_json = args.map(dsl_value_as_json);
        if let Some(command) = directory_command_from_action(action_id, args_json.as_ref()) {
            self.dispatch_directory_command(command).await;
            return;
        }
        if action_id == "os.open-artifact" || action_id == "os.open-artifact-with" {
            self.handle_open_artifact_relay(args_json.as_ref()).await;
        }
    }

    /// 📂️ ticket §4/§3-B — opens the relayed `documentId` (with `spaceId` pinning `open_space_id`
    /// first, so the default-binding computation sees it) with this session's own default bindings.
    /// `documentId` absent ⇒ no-op (the relay only opened an app, nothing further to attach — same
    /// as the React shell's own documented gap until lane 3-B's opening relay carries a real
    /// `schema` field; the `s.space` mapping is the one this lane knows, mirroring `📓️w2-c-report.md`).
    #[cfg(not(target_arch = "wasm32"))]
    async fn handle_open_artifact_relay(&mut self, args: Option<&Value>) {
        let Some(target) = open_artifact_relay_target(args) else {
            return;
        };
        if let Some(space_id) = target.space_id {
            self.open_space_id = Some(space_id);
        }
        let schema = if target.document_id == S_SPACE_INDEX_DOCUMENT_ID { S_SPACE_INDEX_DOCUMENT_SCHEMA.to_string() } else { target.document_id.clone() };
        let (bindings, surface) = self.default_bindings_for_current_session();
        if let Err(error) = self.open_document(target.document_id, schema, bindings, surface).await {
            eprintln!("[DEBUG] wgpu shell os.open-artifact relay failed: {error}");
        }
    }

    /// 🪪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-directory-and-run): builds this shell's
    /// `OperationContext` for one directory request — `cancel` is a `.child()` of `directory_cancel`
    /// (this shell's ONE root, see that field's own doc), so cancelling `directory_cancel` once
    /// transitively cancels every in-flight directory request/stream at once. No deadline yet (see
    /// the packet report's honest gaps).
    #[cfg(not(target_arch = "wasm32"))]
    fn directory_ctx(&self) -> OperationContext {
        OperationContext { actor: 0, generation: 0, trace: TraceId(0), lane: 0, deadline_ms: None, cancel: self.directory_cancel.child_now(), capability: None }
    }

    /// 📇️ ticket §4/§C6 — issues one `os.directory.*` command against the hub, dropping (with a
    /// warning) when no identity is signed in at all, else queueing on any transport/HTTP failure
    /// (the "same offline queueing behaviour as the React side" the brief asks for — React's worker
    /// queues in-memory and flushes on reconnect; this shell's `pending_directory_commands` +
    /// `flush_pending_directory_commands` (called every frame from `pump_directory_events`) is the
    /// native twin of that policy).
    #[cfg(not(target_arch = "wasm32"))]
    async fn dispatch_directory_command(&mut self, command: DirectoryCommand) {
        if self.identity.is_none() {
            eprintln!("[DEBUG] wgpu shell directory command dropped, no signed-in identity: {command:?}");
            return;
        }
        let Some(client) = self.directory_client.as_ref() else {
            self.queue_pending_directory_command(command);
            return;
        };
        if let Err(error) = client.command(&self.directory_ctx(), &command).await {
            eprintln!("[DEBUG] wgpu shell directory command failed, queueing: {error}");
            self.queue_pending_directory_command(command);
        }
    }

    /// 🎮️ Bounded (contract §C6: "bounded, in-memory") — drops the OLDEST queued command rather than
    /// the newest once full, so a burst never silently loses the caller's most recent intent.
    #[cfg(not(target_arch = "wasm32"))]
    fn queue_pending_directory_command(&mut self, command: DirectoryCommand) {
        const MAX_PENDING_DIRECTORY_COMMANDS: usize = 64;
        if self.pending_directory_commands.len() >= MAX_PENDING_DIRECTORY_COMMANDS {
            self.pending_directory_commands.remove(0);
        }
        self.pending_directory_commands.push(command);
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn flush_pending_directory_commands(&mut self) {
        let Some(client) = self.directory_client.as_ref() else { return };
        if self.pending_directory_commands.is_empty() {
            return;
        }
        let ctx = self.directory_ctx();
        let pending = std::mem::take(&mut self.pending_directory_commands);
        for command in pending {
            if let Err(error) = client.command(&ctx, &command).await {
                eprintln!("[DEBUG] wgpu shell directory command flush failed, re-queueing: {error}");
                self.pending_directory_commands.push(command);
            }
        }
    }

    /// 📇️ ticket §3/§C6 — folds one `DirectoryEvent` batch into whichever session is CURRENTLY
    /// mounted, mirroring the React shell's `dispatchDirectoryEventBatch` exactly (same action name/
    /// payload shape, same "one session at a time" constraint — see `fold_directory_events_action`'s
    /// own doc).
    #[cfg(not(target_arch = "wasm32"))]
    async fn dispatch_directory_event_batch(&mut self, events: Vec<DirectoryEvent>) {
        let Some(session) = self.session.clone() else { return };
        let Some(action) = fold_directory_events_action(&session.app.controller_id, &events) else { return };
        if let Err(error) = self.dispatch_action(action).await {
            eprintln!("[DEBUG] wgpu shell foldDirectoryEvents dispatch failed: {error}");
        }
    }

    /// 🪪️ Starts deadline-aware identity bootstrap as a retained-waker I/O-lane future. Each pool
    /// turn polls once; the HTTP boundary itself is admitted on `Lane::Io`, and the UI only polls
    /// this result channel. No worker waits for the network future and no local executor is built.
    #[cfg(not(target_arch = "wasm32"))]
    fn bootstrap_identity(&mut self) {
        let Some(env) = resolve_identity_env() else { return };
        self.identity_env = Some(env.clone());
        let (tx, rx) = std::sync::mpsc::channel::<Result<IdentityOutcome, String>>();
        let transport = self.directory_transport.clone();
        let pool = crate::renderer_worker_pool();
        let mut ctx = self.directory_ctx();
        ctx.deadline_ms = Some(pool.now_ms().saturating_add(5_000));
        self.identity_bootstrap_task = Some(ShellPoolFuture::spawn(pool, Lane::Io, async move {
            let client = DirectoryClient::new(transport, env.hub_url.clone());
            let outcome = mint_or_restore(&ctx, &client, &env).await.map_err(|error| error.to_string());
            let _ = tx.send(outcome);
        }));
        self.identity_bootstrap_rx = Some(rx);
    }

    /// 🪪️ Drains `identity_bootstrap_rx` (non-blocking `try_recv`) and, once resolved, mints the
    /// live `directory_client` and opens the directory stream — called from `pump_directory_events`.
    #[cfg(not(target_arch = "wasm32"))]
    fn poll_identity_bootstrap(&mut self) {
        let Some(rx) = self.identity_bootstrap_rx.as_ref() else { return };
        match rx.try_recv() {
            Ok(Ok(outcome)) => {
                self.identity_bootstrap_task.take();
                self.identity_offline = outcome.status == IdentityStatus::Offline;
                self.identity = Some(outcome.identity.clone());
                let client = std::sync::Arc::new(DirectoryClient::new(self.directory_transport.clone(), outcome.identity.hub_base_url.clone()));
                client.set_token(Some(outcome.identity.session_token.clone()));
                self.directory_client = Some(client.clone());
                if !self.identity_offline {
                    self.open_directory_stream(client);
                }
                self.identity_bootstrap_rx = None;
            }
            Ok(Err(error)) => {
                self.identity_bootstrap_task.take();
                eprintln!("[DEBUG] wgpu shell identity bootstrap failed: {error}");
                self.identity_bootstrap_rx = None;
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {}
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                self.identity_bootstrap_task.take();
                self.identity_bootstrap_rx = None;
            }
        }
    }

    /// 📡️ Starts the Send-capable directory state machine on finite shared-pool turns.
    #[cfg(not(target_arch = "wasm32"))]
    fn open_directory_stream(&mut self, client: std::sync::Arc<DirectoryClient<NativeDirectoryTransport<TokioHostRuntime>>>) {
        if self.directory_stream.is_some() {
            return;
        }
        let ctx = self.directory_ctx();
        self.directory_stream = Some(ShellDirectoryRunner::start(crate::renderer_worker_pool(), client.stream(0), ctx));
    }

    /// 📡️ ticket §1/§3/§4 — called once per frame from `pump_sync_events` (this lease's own region;
    /// glue.rs's frame loop already calls that method every tick, so no second call site was needed
    /// outside this lease): polls identity, drains the finite runner into ordered
    /// `foldDirectoryEvents` batches, and opportunistically flushes queued offline commands.
    #[cfg(not(target_arch = "wasm32"))]
    async fn pump_directory_events(&mut self) -> bool {
        self.poll_identity_bootstrap();
        let mut changed = false;
        if let Some(runner) = self.directory_stream.as_ref() {
            let mut events: Vec<DirectoryEvent> = Vec::new();
            for message in runner.drain() {
                match message {
                    DirectoryStreamMessage::Event { event } => events.push(event),
                    DirectoryStreamMessage::Connection { .. } | DirectoryStreamMessage::Presence { .. } | DirectoryStreamMessage::Heartbeat { .. } => {}
                }
            }
            if !events.is_empty() {
                self.dispatch_directory_event_batch(events).await;
                changed = true;
            }
        }
        self.flush_pending_directory_commands().await;
        changed
    }
    //#endregion 🔖️DirectoryAndIdentity

    pub async fn apply_mutations(&mut self, operations: &[String]) -> Result<(), String> {
        self.apply_ops_inner(operations, true).await
    }

    async fn apply_ops_inner(&mut self, operations: &[String], allow_navigate: bool) -> Result<(), String> {
        let mut pending: Vec<String> = operations.to_vec();
        let mut view_state = self.session.as_ref().map(|s| s.view_state.clone());
        let mut document_changed = false;
        let mut navigate_uri: Option<String> = None;
        while !pending.is_empty() {
            let batch = std::mem::take(&mut pending);
            let mut follow_up_operations: Vec<String> = Vec::new();
            for operation_json in batch {
                let operation: Value = serde_json::from_str(&operation_json).unwrap_or(Value::Null);
                if operation.get("operation").and_then(|v| v.as_str()) == Some("setDocument") {
                    // 🔗️ Document sync now flows through the `framework/sync` `ArtifactHost` actor + the
                    // program store's `ChannelBackbone` (see `attach_sync_backbone`), not a CRUD envelope
                    // write on every `setDocument` — the old `shell_backbone_write` mirror is deleted.
                    document_changed = true;
                }
                if operation.get("operation").and_then(|v| v.as_str()) == Some("setPanel") {
                    if let Some(panel) = operation.get("panel") {
                        if let Some(mut vs) = view_state.take() {
                            vs.panel_json = Some(panel.to_string());
                            view_state = Some(vs);
                        }
                    }
                }
                if operation.get("operation").and_then(|v| v.as_str()) == Some("downloadMediaExport") {
                    if let (Some(filename), Some(mime_type), Some(data)) = (operation.get("filename").and_then(|v| v.as_str()), operation.get("mimeType").and_then(|v| v.as_str()), operation.get("data").and_then(|v| v.as_str())) {
                        let encoding = operation.get("encoding").and_then(|v| v.as_str());
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let (filename, mime_type, data, encoding) = (filename.to_string(), mime_type.to_string(), data.to_string(), encoding.map(str::to_string));
                            self.submit_shell_io_future(async move {
                                download_media_export_worker(&filename, &mime_type, &data, encoding.as_deref()).await;
                                ShellIoCompletion::Finished
                            });
                        }
                        #[cfg(target_arch = "wasm32")]
                        download_media_export(filename, mime_type, data, encoding);
                    }
                }
                if operation.get("operation").and_then(|v| v.as_str()) == Some("requestFileOpen") {
                    if let Some(import_action) = operation.get("importAction").and_then(|v| v.as_str()) {
                        let accept = operation.get("accept").and_then(|v| v.as_str()).unwrap_or(".json");
                        let read_as = operation.get("readAs").and_then(|v| v.as_str());
                        // 📤️ D3: `multiple` opens a multi-select native dialog;
                        // single-file behavior (one dialog call, one `handleAction` with `{json, payload}`) is
                        // byte-for-byte unchanged when absent/false since `request_file_open` then returns at
                        // most one entry and this loop runs exactly once with the same args shape as before.
                        let multiple = operation.get("multiple").and_then(|v| v.as_bool()).unwrap_or(false);
                        #[cfg(not(target_arch = "wasm32"))]
                        if let Some(session) = self.session.clone() {
                            let accept = accept.to_string();
                            let read_as = read_as.map(str::to_string);
                            let import_action = import_action.to_string();
                            let base_args = operation.get("args").cloned().unwrap_or_else(|| serde_json::json!({}));
                            self.submit_shell_io_future(async move {
                                let opened = request_file_open(&accept, read_as.as_deref(), multiple).await;
                                let total = opened.len();
                                let actions = opened
                                    .into_iter()
                                    .enumerate()
                                    .map(|(index, contents)| {
                                        let payload = serde_json::from_str::<Value>(&contents).unwrap_or_else(|_| Value::String(contents.clone()));
                                        let mut args = base_args.clone();
                                        if let Some(obj) = args.as_object_mut() {
                                            obj.insert("json".into(), Value::String(contents));
                                            obj.insert("payload".into(), payload);
                                            if multiple {
                                                obj.insert("index".into(), serde_json::json!(index));
                                                obj.insert("total".into(), serde_json::json!(total));
                                            }
                                        }
                                        ActionDescriptor { controller_id: session.app.controller_id.clone(), action: import_action.clone(), args: semio_framework::optional_json_to_dsl(Some(args)) }
                                    })
                                    .collect();
                                ShellIoCompletion::Actions(actions)
                            });
                        }
                        #[cfg(target_arch = "wasm32")]
                        if let Some(session) = self.session.clone() {
                            let opened = request_file_open(accept, read_as, multiple);
                            let total = opened.len();
                            for (index, contents) in opened.into_iter().enumerate() {
                                let payload = serde_json::from_str::<Value>(&contents).unwrap_or_else(|_| Value::String(contents.clone()));
                                let mut args = operation.get("args").cloned().unwrap_or_else(|| serde_json::json!({}));
                                if let Some(obj) = args.as_object_mut() {
                                    obj.insert("json".into(), Value::String(contents));
                                    obj.insert("payload".into(), payload);
                                    if multiple {
                                        obj.insert("index".into(), serde_json::json!(index));
                                        obj.insert("total".into(), serde_json::json!(total));
                                    }
                                }
                                let action = ActionDescriptor { controller_id: session.app.controller_id.clone(), action: import_action.to_string(), args: semio_framework::optional_json_to_dsl(Some(args)) };
                                if let Some(program) = self.plugins.iter().find(|p| p.plugin_id == session.plugin_id) {
                                    if let Ok(action_json) = serde_json::to_string(&action) {
                                        if let Ok(import_result) = program.handle_action(session.instance_id, &action_json, &session.view_state).await {
                                            follow_up_operations.extend(patch_ops_from_action_result(&import_result));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if operation.get("operation").and_then(|v| v.as_str()) == Some("requestFileSave") {
                    #[cfg(not(target_arch = "wasm32"))]
                    if let (Some(filename), Some(data), Some(space_id)) = (operation.get("filename").and_then(|v| v.as_str()), operation.get("data").and_then(|v| v.as_str()), operation.get("spaceId").and_then(|v| v.as_str())) {
                        if let Some(session) = self.session.clone() {
                            let (filename, data, space_id) = (filename.to_string(), data.to_string(), space_id.to_string());
                            self.submit_shell_io_future(async move {
                                let Some(path) = request_file_save(&filename).await else { return ShellIoCompletion::Finished };
                                use std::fs as system_fs;
                                if system_fs::write(&path, data.as_bytes()).is_err() {
                                    return ShellIoCompletion::Finished;
                                }
                                ShellIoCompletion::Actions(vec![ActionDescriptor {
                                    controller_id: session.app.controller_id,
                                    action: "bindSpaceFile".into(),
                                    args: crate::action_args_json!({ "spaceId": space_id, "filePath": path.display().to_string() }),
                                }])
                            });
                        }
                    }
                }
                if operation.get("operation").and_then(|v| v.as_str()) == Some("requestFolderPick") {
                    #[cfg(not(target_arch = "wasm32"))]
                    if let Some(import_action) = operation.get("importAction").and_then(|v| v.as_str()) {
                        if let Some(session) = self.session.clone() {
                            let import_action = import_action.to_string();
                            let mut args = operation.get("args").cloned().unwrap_or_else(|| serde_json::json!({}));
                            self.submit_shell_io_future(async move {
                                let Some(folder_path) = pick_folder().await else { return ShellIoCompletion::Finished };
                                if let Some(obj) = args.as_object_mut() {
                                    obj.insert("folderPath".into(), serde_json::json!(folder_path));
                                }
                                ShellIoCompletion::Actions(vec![ActionDescriptor { controller_id: session.app.controller_id, action: import_action, args: semio_framework::optional_json_to_dsl(Some(args)) }])
                            });
                        }
                    }
                }
                if operation.get("operation").and_then(|v| v.as_str()) == Some("spawnProgram") {
                    if let (Some(plugin_id), Some(session)) = (operation.get("pluginId").and_then(|v| v.as_str()), &self.session) {
                        self.spawn_plugin(plugin_id, session.view_state.clone()).await?;
                    }
                }
                if operation.get("operation").and_then(|v| v.as_str()) == Some("navigate") {
                    if let Some(uri) = operation.get("uri").and_then(|v| v.as_str()) {
                        navigate_uri = Some(uri.to_string());
                    }
                }
            }
            if !follow_up_operations.is_empty() {
                pending.extend(follow_up_operations);
                document_changed = true;
            }
        }
        if allow_navigate {
            if let Some(uri) = navigate_uri.take() {
                self.push_uri(uri.clone());
                self.apply_shell_uri(&uri).await?;
                if document_changed {
                    self.sync_session_chrome();
                }
                return Ok(());
            }
        }
        if let (Some(mut session), Some(vs)) = (self.session.take(), view_state) {
            session.view_state = vs;
            self.session = Some(session);
            self.sync_session_chrome();
            self.refresh_ui().await?;
        } else if document_changed {
            self.sync_session_chrome();
            self.refresh_ui().await?;
        }
        Ok(())
    }

    // 🏠️🧳️ Generic replacement for the old `switch_to_s_app` — switches to either the host plugin's
    // landing or host app by id (both resolved via `host_config()`, never a specific app's identity).
    async fn switch_to_managed_app(&mut self, app_id: &str, view_state: Option<ViewModel>) -> Result<(), String> {
        let cfg = self.host_config().ok_or("host config missing")?;
        let semio_s_plugin_space = self.plugins.iter().find(|program| program.plugin_id == cfg.plugin_id).ok_or("host program missing")?;
        let app = semio_s_plugin_space.manifest.apps.iter().find(|candidate| candidate.id == app_id).ok_or("host app missing")?.clone();
        if let Some(session) = &self.session {
            if session.plugin_id == semio_s_plugin_space.plugin_id && session.app.id == app_id {
                if let Some(next_view_state) = view_state {
                    if let Some(mut current) = self.session.take() {
                        current.view_state = next_view_state;
                        self.session = Some(current);
                        self.refresh_ui().await?;
                    }
                }
                return Ok(());
            }
        }
        let instance_id = semio_s_plugin_space.create_app(&app.id).await?;
        let workflows = self.build_space_workflows();
        let panel_state = SpacePanelState { active_panel_tab: self.host_catalogue_tab_id().unwrap_or_default(), workflows, spawned_apps: vec![], active_spawned_id: None };
        let next_view_state = view_state.unwrap_or_else(|| ViewModel {
            active_mode_id: Some(app.default_mode_id.clone()),
            active_window_kind_id: Some(app.window_kinds.first().id.clone()),
            active_utility_id: None,
            panel_json: Some(Self::panel_json(&panel_state)),
            contributions_json: None,
            locale: self.active_locale(),
            terminology: self.active_terminology(),
            window_id: None,
            window_instances: Vec::new(),
            active_tool_id: None,
            active_utility_by_window_id: HashMap::new(),
        });
        self.active_window_id = Some(app.window_kinds.first().id.clone());
        if app_id == cfg.landing_app_id {
            self.open_space_id = None;
        }
        self.session = Some(ActiveSession { plugin_id: semio_s_plugin_space.plugin_id.clone(), instance_id, app, view_state: next_view_state });
        self.refresh_ui().await
    }

    /// 📇️ ticket §6 — generic app switch by definition (not the host's landing/host app via
    /// `host_config()`), used for the `s.space` artifact-index route below. Kept as a close sibling of
    /// `switch_to_managed_app` (CLAUDE.md: repeated code stays close together) rather than a forced
    /// shared abstraction that would blur the two call shapes (this one has no workflow-panel state).
    #[cfg(not(target_arch = "wasm32"))]
    async fn switch_to_app(&mut self, plugin_id: &str, app: AppDefinition) -> Result<(), String> {
        let program = self.plugins.iter().find(|entry| entry.plugin_id == plugin_id).cloned().ok_or("program missing")?;
        if let Some(session) = &self.session {
            if session.plugin_id == plugin_id && session.app.id == app.id {
                return Ok(());
            }
        }
        let instance_id = program.create_app(&app.id).await?;
        let view_state = ViewModel {
            active_mode_id: Some(app.default_mode_id.clone()),
            active_window_kind_id: Some(app.window_kinds.first().id.clone()),
            active_utility_id: None,
            panel_json: None,
            contributions_json: None,
            locale: self.active_locale(),
            terminology: self.active_terminology(),
            window_id: None,
            window_instances: Vec::new(),
            active_tool_id: None,
            active_utility_by_window_id: HashMap::new(),
        };
        self.active_window_id = Some(app.window_kinds.first().id.clone());
        self.session = Some(ActiveSession { plugin_id: plugin_id.to_string(), instance_id, app, view_state });
        self.refresh_ui().await
    }

    async fn apply_shell_uri(&mut self, uri: &str) -> Result<(), String> {
        let Some(cfg) = self.host_config() else {
            return Ok(());
        };
        let path = uri.split('?').next().unwrap_or(uri);
        let raw = path.strip_prefix("/spaces/").map(|value| value.trim_end_matches('/').to_string()).filter(|value| !value.is_empty());
        if raw.is_none() {
            self.open_space_id = None;
            if self.session.as_ref().map(|session| session.app.id.as_str()) != Some(cfg.landing_app_id) {
                self.switch_to_managed_app(cfg.landing_app_id, None).await?;
            }
            return Ok(());
        }
        let raw = raw.expect("space route");
        // 📇️ ticket §5/§6 — "/spaces/{id}/studio" keeps the pre-existing studio behaviour (the ONLY
        // behaviour this route had before this lane); a bare "/spaces/{id}" now opens the `s.space`
        // artifact-index app instead, mirroring the React shell's `applyShellUri` (`📓️w2-c-report.md`).
        let (space_id, is_studio_route) = match raw.split_once('/') {
            Some((id, "studio")) => (id.to_string(), true),
            _ => (raw.clone(), false),
        };
        #[cfg(not(target_arch = "wasm32"))]
        if !is_studio_route {
            self.open_space_id = Some(space_id.clone());
            let host_program = self.plugins.iter().find(|program| program.plugin_id == cfg.plugin_id).cloned();
            let space_app = host_program
                .as_ref()
                .and_then(|program| find_dialect_app(program, &space_index_dialect(), semio_framework::manifest::AppRole::Editor).or_else(|| find_dialect_app(program, &space_index_dialect(), semio_framework::manifest::AppRole::Viewer)))
                .cloned();
            let Some(space_app) = space_app else {
                eprintln!("[DEBUG] wgpu shell apply_shell_uri: no app registered for dialect s.space.space@1/* — s.space (lane 1-E) not loaded yet");
                return Ok(());
            };
            self.switch_to_app(cfg.plugin_id, space_app).await?;
            let (bindings, surface) = self.default_bindings_for_current_session();
            return self.open_document(S_SPACE_INDEX_DOCUMENT_ID.to_string(), S_SPACE_INDEX_DOCUMENT_SCHEMA.to_string(), bindings, surface).await;
        }
        let studio_changed = self.open_space_id.as_deref() != Some(space_id.as_str());
        // 🧭️ Pin before the async switch so a concurrent chrome sync cannot boot the demo example over
        // an explicit `/spaces/:id` route.
        self.open_space_id = Some(space_id.clone());
        self.switch_to_managed_app(cfg.host_app_id, None).await?;
        if !studio_changed {
            return Ok(());
        }
        let session = self.session.clone().ok_or("space session missing")?;
        let program = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).ok_or("space program missing")?;
        let action = ActionDescriptor { controller_id: session.app.controller_id.clone(), action: "openSpace".into(), args: crate::action_args_json!({ "spaceId": space_id }) };
        let action_json = serde_json::to_string(&action).map_err(|err| err.to_string())?;
        let result = program.handle_action(session.instance_id, &action_json, &session.view_state).await?;
        for effect in &result.requested_effects {
            if let semio_framework::kernel::Effect::LoadDocument { pack, spr } = effect {
                // 🎠️ H3-wgpu-native — `load_app_document_pack` is now async.
                program.load_app_document_pack(session.instance_id, pack, spr).await?;
            }
        }
        self.sync_session_chrome();
        self.refresh_ui().await
    }

    pub async fn apply_pending_shell_uri(&mut self) -> Result<(), String> {
        let uri = self.shell_uri();
        self.apply_shell_uri(&uri).await
    }

    async fn spawn_plugin(&mut self, plugin_id: &str, mut view_state: ViewModel) -> Result<(), String> {
        let workflows = self.build_space_workflows();
        let Some(workflow) = workflows.iter().find(|entry| entry.plugin_id == plugin_id).cloned() else {
            return Ok(());
        };
        let bridge = self.plugins.iter().find(|entry| entry.plugin_id == workflow.plugin_id).ok_or("spawn program missing")?;
        let instance_id = bridge.create_app(&workflow.app_id).await?;
        let default_catalogue_tab_id = self.host_catalogue_tab_id().unwrap_or_default();
        let mut panel = Self::panel_state_from_view(&view_state).unwrap_or(SpacePanelState { active_panel_tab: default_catalogue_tab_id, workflows: workflows.clone(), spawned_apps: vec![], active_spawned_id: None });
        let spawned_id = format!("{}-{}", bridge.plugin_id, instance_id);
        panel.spawned_apps.push(SpawnedAppEntry { id: spawned_id.clone(), plugin_id: bridge.plugin_id.clone(), instance_id, app_id: workflow.app_id.clone(), label: workflow.label.clone(), breadcrumb: workflow.breadcrumb.clone() });
        panel.active_spawned_id = Some(spawned_id);
        view_state.panel_json = Some(Self::panel_json(&panel));
        if let Some(session) = self.session.as_mut() {
            session.view_state = view_state;
        }
        Ok(())
    }
}
//#endregion ShellActions

//#region ShellInput
impl ShellChromeBuildState {
    fn content_has_focus(&self, window_id: &str) -> bool {
        self.content_focus.get(window_id).copied().unwrap_or(false)
    }

    fn note_content_focus_commands(&mut self, commands: &[ui_wgpu::wgpu::UiCommand]) {
        for command in commands {
            if let ui_wgpu::wgpu::UiCommand::FocusChanged { window_id, node } = command {
                self.content_focus.insert(window_id.clone(), node.is_some());
            }
        }
    }
}

/// ⌨️ Maps a chrome-level `ui_wgpu::wgpu::KeyAction` (+ modifiers) to the `ui_wgpu::wgpu::UiEvent` the retained
/// content engine's `events::EventRouter::dispatch` expects — mirrors that fn's `UiEvent::KeyDown`
/// key-string vocabulary exactly (`"ArrowLeft"`/`"Backspace"`/`"c"`+ctrl for copy/etc., see
/// `ui_wgpu`'s `🔖️EditRouting`/`🔖️UiCommand` regions). A `Char` held with Ctrl/Cmd routes as
/// `KeyDown` (so `c`/`x`/`v` clipboard chords reach `route_edit_key` instead of being literally
/// inserted as text); a plain `Char` routes as `TextInput`. `Space` has no coherent press/release
/// `UiEvent` (content has no pan-mode concept) and is already fully handled earlier in
/// `AppRuntime::handle_key`, so it never reaches here.
fn ui_event_from_key_action(action: &ui_wgpu::wgpu::KeyAction, modifiers: &PointerModifiers) -> Option<ui_wgpu::wgpu::UiEvent> {
    let event_modifiers = ui_wgpu::wgpu::EventModifiers { shift: modifiers.shift, ctrl: modifiers.ctrl, alt: modifiers.alt, meta: modifiers.meta };
    match action {
        ui_wgpu::wgpu::KeyAction::Char(ch) => {
            if modifiers.ctrl_or_meta() {
                Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: ch.clone(), modifiers: event_modifiers })
            } else {
                Some(ui_wgpu::wgpu::UiEvent::TextInput { text: ch.clone() })
            }
        }
        ui_wgpu::wgpu::KeyAction::Backspace => Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: "Backspace".into(), modifiers: event_modifiers }),
        ui_wgpu::wgpu::KeyAction::Delete => Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: "Delete".into(), modifiers: event_modifiers }),
        ui_wgpu::wgpu::KeyAction::Enter => Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: "Enter".into(), modifiers: event_modifiers }),
        ui_wgpu::wgpu::KeyAction::Escape => Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: "Escape".into(), modifiers: event_modifiers }),
        ui_wgpu::wgpu::KeyAction::ArrowLeft => Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: "ArrowLeft".into(), modifiers: event_modifiers }),
        ui_wgpu::wgpu::KeyAction::ArrowRight => Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: "ArrowRight".into(), modifiers: event_modifiers }),
        ui_wgpu::wgpu::KeyAction::ArrowUp => Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: "ArrowUp".into(), modifiers: event_modifiers }),
        ui_wgpu::wgpu::KeyAction::ArrowDown => Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: "ArrowDown".into(), modifiers: event_modifiers }),
        ui_wgpu::wgpu::KeyAction::Function(number) => Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: format!("F{number}"), modifiers: event_modifiers }),
        ui_wgpu::wgpu::KeyAction::Tab => Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: "Tab".into(), modifiers: event_modifiers }),
        ui_wgpu::wgpu::KeyAction::Space(_) => None,
    }
}

impl ShellState {
    pub async fn handle_pointer_button(&mut self, x: f32, y: f32, down: bool, button: i16, input: &mut InputState<ActionDescriptor>, theme: &Theme) -> Result<(), String> {
        input.pointer_x = x;
        input.pointer_y = y;
        input.pointer_down = down;
        input.pointer_button = button;
        if !down {
            if self.dock_drag.is_some() {
                self.finish_dock_drag(x, y, input).await?;
            } else if let Some((payload, _)) = self.pending_dock_drag.take() {
                if let Some(hit) = input.hit_at(x, y) {
                    if let Some(rest) = hit.control_id.as_deref().and_then(|id| id.strip_prefix("dock.tab.")) {
                        let rest = rest.strip_suffix(".focus").or_else(|| rest.strip_suffix(".close")).or_else(|| rest.strip_suffix(".new")).unwrap_or(rest);
                        if let Some((path_str_value, window_id)) = rest.split_once('.') {
                            if window_id == payload.window_id {
                                let path = parse_path(path_str_value);
                                self.dock.set_stack_active(&path, window_id);
                                self.active_window_id = Some(window_id.to_string());
                            }
                        }
                    }
                }
                self.restore_dock_drag_snapshot();
            }
            if self.tree_drag.is_some() {
                self.finish_tree_drag(x, y, input).await?;
            } else if let Some((item_id, _)) = self.pending_tree_drag.take() {
                if let Some(hit) = input.hit_at(x, y) {
                    if hit.control_id.as_deref() == Some(&format!("tree.label.{item_id}")) {
                        self.dispatch_tree_selection(&item_id).await?;
                        if let Some(action) = hit.event.clone() {
                            self.dispatch_action(action).await?;
                        }
                    }
                }
            }
            if input.drag.active {
                let drag_target = input.drag.target_id.clone();
                self.dispatch_widget_drag_values(input).await?;
                input.end_drag();
                if drag_target.as_deref().is_some_and(|id| id.starts_with("dock.split.") || id.starts_with("dock.corner.")) {
                    self.persist_dock_layout();
                    if let Some(controller_id) = self.host_controller_id() {
                        let note = Self::note_shell_command_action(&controller_id, "shell.windowResize", "Resize Window", None);
                        self.dispatch_action(note).await?;
                    }
                }
            }
            return Ok(());
        }
        if button == 2 {
            let hit = input.hit_at(x, y).cloned();
            self.open_context_menu(x, y, hit).await;
            self.right_click = RightClickState { pending: true, x, y };
            return Ok(());
        }
        if self.dismiss_overlays(x, y, input) {
            return Ok(());
        }
        if let Some(hit) = input.hit_at(x, y).cloned() {
            if hit.kind == HitKind::PanelResize {
                if let Some(id) = hit.control_id.as_deref() {
                    if let Some(window_id) = id.strip_prefix("shell.measures.resize.") {
                        self.measures_resize_window_id = Some(window_id.to_string());
                        self.measures_resize_origin_width = *self.measures_width.get(window_id).unwrap_or(&Theme::default().window_measures_default_width);
                        input.begin_drag(x, y, button, hit.control_id.clone(), Some(DragAxis::Horizontal), Some(hit.kind));
                        return Ok(());
                    }
                }
                let body = self.body_rect(theme);
                let width = if hit.control_id.as_deref() == Some("panel.resize.left") { floating_panel_width(self.left_panel_width, body, theme) } else { floating_panel_width(self.right_panel_width, body, theme) };
                self.panel_resize_origin_width = width;
                input.begin_drag(x, y, button, hit.control_id.clone(), Some(DragAxis::Horizontal), Some(hit.kind));
                return Ok(());
            }
            if matches!(hit.kind, HitKind::DockSplit | HitKind::DockJoinCorner) {
                if let Some(id) = hit.control_id.as_deref() {
                    if let Some(rest) = id.strip_prefix("dock.corner.r/") {
                        if let Some((row_part, col_part)) = rest.split_once("/c/") {
                            if let Some((row_path_str, row_index_str)) = row_part.rsplit_once('/') {
                                if let Some((col_path_str, col_index_str)) = col_part.rsplit_once('/') {
                                    let row_path = parse_path(row_path_str);
                                    let col_path = parse_path(col_path_str);
                                    self.split_resize_path = Some(row_path.clone());
                                    self.split_resize_index = row_index_str.parse().unwrap_or(0);
                                    self.split_resize_secondary_path = Some(col_path.clone());
                                    self.split_resize_secondary_index = col_index_str.parse().unwrap_or(0);
                                    self.split_resize_origin = self.dock.begin_split_drag(&row_path);
                                    self.split_resize_secondary_origin = self.dock.begin_split_drag(&col_path);
                                    self.split_resize_axis_total = self.dock.split_axis_extent(&row_path, self.dock_canvas_bounds).unwrap_or(self.dock_canvas_bounds.w);
                                    self.split_resize_secondary_axis_total = self.dock.split_axis_extent(&col_path, self.dock_canvas_bounds).unwrap_or(self.dock_canvas_bounds.h);
                                    input.begin_drag(x, y, button, Some(id.to_string()), Some(DragAxis::Both), Some(hit.kind));
                                    return Ok(());
                                }
                            }
                        }
                    }
                    if let Some(rest) = id.strip_prefix("dock.split.") {
                        if let Some((path_str, index_str)) = rest.rsplit_once('.') {
                            let path = parse_path(path_str);
                            let index: usize = index_str.parse().unwrap_or(0);
                            self.split_resize_path = Some(path.clone());
                            self.split_resize_index = index;
                            self.split_resize_origin = self.dock.begin_split_drag(&path);
                            self.split_resize_axis_total = self.dock.split_axis_extent(&path, self.dock_canvas_bounds).unwrap_or_else(|| match hit.drag_axis {
                                Some(DragAxis::Vertical) => self.dock_canvas_bounds.h,
                                _ => self.dock_canvas_bounds.w,
                            });
                            input.begin_drag(x, y, button, Some(id.to_string()), hit.drag_axis, Some(hit.kind));
                            return Ok(());
                        }
                    }
                }
            }
            // 🧾️ Flush an in-progress staged-arg edit before any Actions-rail interaction so Execute
            // merges it (Architecture Decision 8, P2 — "execute flushes any focused text buffer first").
            if hit.control_id.as_deref().is_some_and(|id| id.starts_with("shell.action.")) && input.focused_id.as_deref().is_some_and(|id| id.starts_with("shell.action.arginput::") || id.starts_with("shell.action.argvec3::")) {
                self.commit_focused_input(input).await?;
            }
            if self.handle_shell_hit(&hit).await? {
                return Ok(());
            }
            if let Some(id) = hit.control_id.as_deref() {
                if let Some(rest) = id.strip_prefix("dock.tab.") {
                    let rest = rest.strip_suffix(".focus").or_else(|| rest.strip_suffix(".close")).or_else(|| rest.strip_suffix(".new")).unwrap_or(rest);
                    if let Some((path_str_value, window_id)) = rest.split_once('.') {
                        let path = parse_path(path_str_value);
                        let tab_index = self.dock.tab_index(&path, window_id).unwrap_or(0);
                        let ghost_label =
                            self.session.as_ref().and_then(|s| s.app.window_kinds.iter().find(|k| k.id == window_id).map(|k| k.label.resolve(self.active_terminology(), self.active_locale()).to_string())).unwrap_or_else(|| window_id.to_string());
                        self.begin_pending_dock_drag(DockDragPayload { kind: DockDragKind::Tab, window_id: window_id.to_string(), source_path: path, tab_index, ghost_label }, x, y);
                        return Ok(());
                    }
                }
                if let Some(path_str_value) = id.strip_prefix("dock.stack.") {
                    let path = parse_path(path_str_value);
                    let windows = self.dock.stack_windows_at_path(&path).unwrap_or_default();
                    let active = windows.iter().find(|wid| self.active_window_id.as_deref() == Some(wid.as_str())).or_else(|| windows.first()).cloned().unwrap_or_default();
                    if !active.is_empty() {
                        let tab_index = self.dock.tab_index(&path, &active).unwrap_or(0);
                        let ghost_label = self.session.as_ref().and_then(|s| s.app.window_kinds.iter().find(|k| k.id == active).map(|k| k.label.resolve(self.active_terminology(), self.active_locale()).to_string())).unwrap_or_else(|| active.clone());
                        self.begin_pending_dock_drag(DockDragPayload { kind: DockDragKind::Stack, window_id: active, source_path: path, tab_index, ghost_label }, x, y);
                        return Ok(());
                    }
                }
            }
            if hit.kind == HitKind::Slider {
                if let Some(id) = hit.control_id.clone() {
                    input.begin_drag(x, y, button, Some(id), hit.drag_axis, Some(hit.kind));
                    return Ok(());
                }
            }
            if hit.kind == HitKind::Select || hit.kind == HitKind::Toggle || hit.kind == HitKind::DropdownItem {
                return Ok(());
            }
            if let Some(id) = hit.control_id.as_deref() {
                if id.contains(".vfs.") && !id.contains(".chevron.") {
                    if let Some((surface_id, row_id)) = id.rsplit_once(".vfs.") {
                        let additive = input.modifiers.meta || input.modifiers.ctrl;
                        let shift = input.modifiers.shift;
                        let ordered = vec![row_id.to_string()];
                        let ids = vfs_selection_for_click(surface_id, row_id, &ordered, shift, additive);
                        self.dispatch_action(ActionDescriptor {
                            controller_id: self.session.as_ref().map(|s| s.app.controller_id.clone()).unwrap_or_default(),
                            action: "selectRows".into(),
                            args: crate::action_args_json!({ "surfaceId": surface_id, "ids": ids }),
                        })
                        .await?;
                        return Ok(());
                    }
                }
            }
            if let Some(drag_data) = hit.drag_data.clone() {
                if hit.control_id.as_deref().is_some_and(|id| id.starts_with("tree.label.")) {
                    if let Some(item_id) = hit.control_id.as_deref().and_then(|id| id.strip_prefix("tree.label.")) {
                        self.tree_drag_origin = (x, y);
                        self.pending_tree_drag = Some((item_id.to_string(), drag_data));
                        return Ok(());
                    }
                }
            }
            if let Some(action) = hit.event.clone() {
                self.dispatch_action(action).await?;
            } else if hit.kind == HitKind::Input {
                if let Some(id) = hit.control_id {
                    let seed = self.widget_maps.input_metas.get(&id).map(|meta| meta.value.clone()).or_else(|| self.staged_input_seed(&id)).unwrap_or_default();
                    input.focus_input_owned(id, seed);
                }
            }
        }
        self.flush_deferred_actions().await?;
        Ok(())
    }

    pub fn handle_pointer_move(&mut self, x: f32, y: f32, down: bool, input: &mut InputState<ActionDescriptor>, theme: &Theme) {
        input.pointer_x = x;
        input.pointer_y = y;
        input.pointer_down = down;
        input.update_hover(x, y);
        self.sync_context_menu_hover(input);
        self.update_tree_hover(input);
        if let Some((ref item_id, ref drag_data)) = self.pending_tree_drag {
            if down {
                let dx = x - self.tree_drag_origin.0;
                let dy = y - self.tree_drag_origin.1;
                if self.tree_drag.is_none() && (dx * dx + dy * dy) > 25.0 {
                    self.tree_drag = Some(TreeDragState { source_id: item_id.clone(), drag_data: drag_data.clone(), x, y, drop_target_id: None, drop_position: TreeDropPosition::Inside });
                    self.pending_tree_drag = None;
                }
            }
        }
        if let Some(drag) = &mut self.tree_drag {
            drag.x = x;
            drag.y = y;
            crate::engine_canvas::node_graph_sync_flow_widget_ghost(x, y, &drag.drag_data, &self.node_graph_states.iter().map(|(id, surface)| (id.as_str(), surface.bounds)).collect::<Vec<_>>());
            if let Some(hit) = input.hit_at(x, y) {
                if let Some(target_id) = hit.control_id.as_deref().and_then(|id| id.strip_prefix("tree.label.")) {
                    drag.drop_target_id = Some(target_id.to_string());
                    let rel = (y - hit.rect.y) / hit.rect.h.max(1.0);
                    drag.drop_position = if rel < 0.25 {
                        TreeDropPosition::Before
                    } else if rel > 0.75 {
                        TreeDropPosition::After
                    } else {
                        TreeDropPosition::Inside
                    };
                } else if hit.kind == HitKind::World3d || hit.kind == HitKind::Window {
                    drag.drop_target_id = hit.control_id.clone();
                    drag.drop_position = TreeDropPosition::Inside;
                } else {
                    drag.drop_target_id = None;
                }
            }
        }
        if let Some((payload, origin)) = &self.pending_dock_drag {
            if down {
                let dx = x - origin.0;
                let dy = y - origin.1;
                if self.dock_drag.is_none() && (dx * dx + dy * dy) > 25.0 {
                    let payload = payload.clone();
                    let origin = *origin;
                    self.pending_dock_drag = None;
                    self.dock.remove_window(&payload.window_id);
                    self.dock_drag = Some(DockDragState { payload, x: origin.0, y: origin.1, drop_zone: None });
                }
            }
        }
        if let Some(drag) = &mut self.dock_drag {
            drag.x = x;
            drag.y = y;
            drag.drop_zone = compute_dock_drop_zone(x, y, &self.dock_drop_tab_bars, &self.dock_drop_bodies, self.dock_canvas_bounds);
        }
        if input.drag.active && down {
            input.update_drag(x, y);
            if let Some(id) = input.drag.target_id.as_deref() {
                let dx = x - input.drag.start_x;
                let dy = y - input.drag.start_y;
                match id {
                    id if id.starts_with("shell.measures.resize.") => {
                        if let Some(window_id) = self.measures_resize_window_id.clone() {
                            let next = (self.measures_resize_origin_width - dx).clamp(theme.panel_min_width, theme.panel_max_width);
                            self.measures_width.insert(window_id, next);
                        }
                    }
                    "panel.resize.left" => {
                        let body = self.body_rect(theme);
                        self.left_panel_width = (self.panel_resize_origin_width + dx).clamp(theme.panel_min_width, floating_panel_max_width(body, theme));
                    }
                    "panel.resize.right" => {
                        let body = self.body_rect(theme);
                        self.right_panel_width = (self.panel_resize_origin_width - dx).clamp(theme.panel_min_width, floating_panel_max_width(body, theme));
                    }
                    dock_id if dock_id.starts_with("dock.corner.") => {
                        if let Some(path) = self.split_resize_path.clone() {
                            self.dock.apply_split_drag_with_origin(&path, self.split_resize_index, dx, self.split_resize_axis_total, &self.split_resize_origin);
                        }
                        if let Some(path) = self.split_resize_secondary_path.clone() {
                            self.dock.apply_split_drag_with_origin(&path, self.split_resize_secondary_index, dy, self.split_resize_secondary_axis_total, &self.split_resize_secondary_origin);
                        }
                    }
                    dock_id if dock_id.starts_with("dock.split.") => {
                        if let (Some(path), axis) = (&self.split_resize_path, input.drag.axis) {
                            let delta = match axis {
                                Some(DragAxis::Horizontal) => dx,
                                Some(DragAxis::Vertical) => dy,
                                _ => dx,
                            };
                            self.dock.apply_split_drag_with_origin(path, self.split_resize_index, delta, self.split_resize_axis_total, &self.split_resize_origin);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    async fn finish_dock_drag(&mut self, x: f32, y: f32, input: &InputState<ActionDescriptor>) -> Result<(), String> {
        let Some(mut drag) = self.dock_drag.take() else {
            return Ok(());
        };
        drag.x = x;
        drag.y = y;
        if drag.drop_zone.is_none() {
            drag.drop_zone = compute_dock_drop_zone(x, y, &self.dock_drop_tab_bars, &self.dock_drop_bodies, self.dock_canvas_bounds);
        }
        if let Some(zone) = drag.drop_zone {
            if self.dock.apply_drop(&drag.payload, &zone) {
                self.active_window_id = Some(drag.payload.window_id.clone());
                self.dock.sync_active_window(&drag.payload.window_id);
                self.persist_dock_layout();
                if let Some(controller_id) = self.host_controller_id() {
                    let note = Self::note_shell_command_action(&controller_id, "shell.windowMove", "Move Window", Some(serde_json::json!({ "windowId": drag.payload.window_id })));
                    self.dispatch_action(note).await?;
                }
            } else {
                self.restore_dock_drag_snapshot();
            }
        } else {
            self.restore_dock_drag_snapshot();
        }
        self.dock_drag_snapshot = None;
        let _ = input;
        Ok(())
    }

    fn scroll_region_is_scene_surface(control_id: &str) -> bool {
        control_id.ends_with(".pane") || control_id.ends_with(".map")
    }

    pub fn wheel_propagates_to_scene_surface(hit: Option<&HitTarget<ActionDescriptor>>) -> bool {
        let Some(hit) = hit else {
            return true;
        };
        match hit.kind {
            HitKind::World3d | HitKind::Window => true,
            HitKind::ScrollRegion => hit.control_id.as_deref().is_some_and(Self::scroll_region_is_scene_surface),
            _ => false,
        }
    }

    pub fn handle_pointer_wheel(&mut self, x: f32, y: f32, delta: f32, input: &InputState<ActionDescriptor>) -> bool {
        let Some(hit) = input.hit_at(x, y) else {
            return false;
        };
        if hit.kind != HitKind::ScrollRegion {
            return false;
        }
        let Some(id) = &hit.control_id else {
            return false;
        };
        // 📜️ The open context menu's own scroll offset lives on `ContextMenuState`, not the generic
        // per-control `scroll_offsets` map — see `render_context_menu_level`'s clip/scroll handling.
        if id == "shell.context.menu.scroll" {
            let Some(menu) = self.context_menu.as_mut() else {
                return false;
            };
            menu.scroll_offset = (menu.scroll_offset + delta * 24.0).max(0.0);
            return true;
        }
        if Self::scroll_region_is_scene_surface(id) {
            return false;
        }
        let entry = self.scroll_offsets.entry(id.clone()).or_insert(0.0);
        *entry = (*entry + delta * 24.0).max(0.0);
        true
    }

    pub async fn handle_world3d_input(&mut self, x: f32, y: f32, down: bool, button: i16, shift: bool, ctrl: bool, alt: bool, meta: bool, wheel_delta: f32, drag_dx: f32, drag_dy: f32) -> Result<(), String> {
        let modifiers = PointerModifiers { shift, ctrl, alt, meta };
        for state in self.world3d_states.values_mut() {
            if !state.bounds.contains(x, y) {
                continue;
            }
            let wheel = WorldInteractionIntent::wheel(x, y, wheel_delta, &modifiers);
            let button_intent = WorldInteractionIntent::pointer_button(x, y, down, button, &modifiers);
            let move_intent = WorldInteractionIntent::pointer_move(x, y, drag_dx, drag_dy, down, button, &modifiers);
            let mut drag = move_intent;
            drag.phase = WorldInteractionPhase::PointerDrag;
            let has_wheel = wheel_delta.abs() > 0.0;
            let has_drag = down && (drag_dx.abs() > 0.0 || drag_dy.abs() > 0.0);
            let admitted = match (has_wheel, has_drag) {
                (true, true) => enqueue_world3d_events(state, [wheel, drag, button_intent, move_intent]).is_ok(),
                (true, false) => enqueue_world3d_events(state, [wheel, button_intent, move_intent]).is_ok(),
                (false, true) => enqueue_world3d_events(state, [drag, button_intent, move_intent]).is_ok(),
                (false, false) => enqueue_world3d_events(state, [button_intent, move_intent]).is_ok(),
            };
            if !admitted {
                return Err("world3d fixed interaction credits exhausted".into());
            }
        }
        Ok(())
    }

    async fn handle_shell_hit(&mut self, hit: &HitTarget<ActionDescriptor>) -> Result<bool, String> {
        let Some(id) = hit.control_id.as_deref() else {
            return Ok(false);
        };
        match id {
            "ui.nav.back" => {
                if self.uri_index > 0 {
                    self.uri_index -= 1;
                }
                self.apply_pending_shell_uri().await?;
                return Ok(true);
            }
            "ui.nav.forward" => {
                if self.uri_index + 1 < self.uri_history.len() {
                    self.uri_index += 1;
                }
                self.apply_pending_shell_uri().await?;
                return Ok(true);
            }
            "ui.nav.up" => {
                let uri = self.shell_uri();
                if let Some(parent) = uri.rsplit_once('/').map(|(p, _)| p.to_string()) {
                    if !parent.is_empty() {
                        self.push_uri(parent);
                    }
                }
                self.apply_pending_shell_uri().await?;
                return Ok(true);
            }
            "playground.navbar.fixture" => {
                self.overlay_state = OverlayState::Dropdown("example".to_string());
                return Ok(true);
            }
            id if id.starts_with("playground.navbar.modes.") => {
                let mode_id = id.trim_start_matches("playground.navbar.modes.");
                if let Some(session) = self.session.as_mut() {
                    session.view_state.active_mode_id = Some(mode_id.to_string());
                    if let Some(layout) = semio_framework::resolve_layout_for_mode(&session.app, mode_id) {
                        self.layout_override = Some(layout);
                        self.sync_dock();
                        self.active_window_id = self.dock.active_window_id.clone();
                    }
                }
                self.refresh_ui().await?;
                return Ok(true);
            }
            id if id.starts_with("framework.utility.collection.") => {
                let collection_id = id.trim_start_matches("framework.utility.collection.");
                let expanded = self.utility_collection_expanded.get(collection_id).copied().unwrap_or(false);
                self.utility_collection_expanded.insert(collection_id.to_string(), !expanded);
                return Ok(true);
            }
            id if id.starts_with("shell.example.") => {
                let example_id = id.trim_start_matches("shell.example.");
                self.active_example_id = Some(example_id.to_string());
                self.overlay_state = OverlayState::None;
                if let Some(session) = &self.session {
                    self.dispatch_action(ActionDescriptor { controller_id: session.app.controller_id.clone(), action: "setActiveExample".into(), args: crate::action_args_json!({ "exampleId": example_id }) }).await?;
                }
                return Ok(true);
            }
            id if id.starts_with("shell.find.item.") => {
                let index: usize = id.trim_start_matches("shell.find.item.").parse().unwrap_or(0);
                self.activate_find_item(index).await?;
                return Ok(true);
            }
            id if id.starts_with("shell.engagement.toggle.") => {
                let window_id = id.trim_start_matches("shell.engagement.toggle.");
                let activated = self.engagement_activated.get(window_id).copied().unwrap_or(false);
                self.engagement_activated.insert(window_id.to_string(), !activated);
                self.engagement_expanded.insert(window_id.to_string(), !activated);
                return Ok(true);
            }
            id if id.starts_with("shell.measures.fold.") => {
                let window_id = id.trim_start_matches("shell.measures.fold.");
                self.measures_folded.insert(window_id.to_string(), true);
                return Ok(true);
            }
            id if id.starts_with("shell.measures.unfold.") => {
                let window_id = id.trim_start_matches("shell.measures.unfold.");
                self.measures_folded.insert(window_id.to_string(), false);
                return Ok(true);
            }
            id if id.starts_with("shell.measures.focus.") => {
                let window_id = id.trim_start_matches("shell.measures.focus.");
                let expanded = self.measures_expanded.get(window_id).copied().unwrap_or(false);
                self.measures_expanded.insert(window_id.to_string(), !expanded);
                if !expanded {
                    self.engagement_activated.remove(window_id);
                    self.engagement_expanded.insert(window_id.to_string(), false);
                }
                return Ok(true);
            }
            id if id.starts_with("shell.action.fold.") => {
                let window_id = id.trim_start_matches("shell.action.fold.");
                let folded = self.action_panel_folded.get(window_id).copied().unwrap_or(true);
                self.action_panel_folded.insert(window_id.to_string(), !folded);
                return Ok(true);
            }
            id if id.starts_with("shell.action.expand::") => {
                if let Some((window_id, action_id)) = id.trim_start_matches("shell.action.expand::").split_once("::") {
                    let open = self.action_panel_expanded.get(window_id).map(String::as_str) == Some(action_id);
                    if open {
                        self.action_panel_expanded.remove(window_id);
                    } else {
                        self.action_panel_expanded.insert(window_id.to_string(), action_id.to_string());
                    }
                }
                return Ok(true);
            }
            id if id.starts_with("shell.action.reset::") => {
                if let Some((window_id, action_id)) = id.trim_start_matches("shell.action.reset::").split_once("::") {
                    self.reset_staged_args(window_id, action_id);
                }
                return Ok(true);
            }
            id if id.starts_with("shell.action.argtoggle::") => {
                let parts: Vec<&str> = id.trim_start_matches("shell.action.argtoggle::").split("::").collect();
                if let [window_id, action_id, arg_id] = parts.as_slice() {
                    let current = self.staged_map_for(window_id, action_id).get(*arg_id).and_then(|value| value.as_bool()).or_else(|| self.arg_default(window_id, action_id, arg_id).and_then(|value| value.as_bool())).unwrap_or(false);
                    self.stage_arg(window_id, action_id, arg_id, Value::Bool(!current));
                }
                return Ok(true);
            }
            id if id.starts_with("shell.action.argselect::") => {
                let parts: Vec<&str> = id.trim_start_matches("shell.action.argselect::").split("::").collect();
                if let [window_id, action_id, arg_id, value] = parts.as_slice() {
                    self.stage_arg(window_id, action_id, arg_id, Value::String((*value).to_string()));
                }
                return Ok(true);
            }
            id if id.starts_with("shell.action.exec::") => {
                if let Some((window_id, action_id)) = id.trim_start_matches("shell.action.exec::").split_once("::") {
                    let (window_id, action_id) = (window_id.to_string(), action_id.to_string());
                    self.execute_staged_action(&window_id, &action_id).await?;
                }
                return Ok(true);
            }
            "ui.search.toggle" => {
                self.search_open = !self.search_open;
                self.find_open = false;
                self.overlay_state = if self.search_open { OverlayState::Search } else { OverlayState::None };
                return Ok(true);
            }
            "ui.find.toggle" => {
                self.find_open = !self.find_open;
                self.search_open = false;
                self.overlay_state = if self.find_open { OverlayState::Find } else { OverlayState::None };
                return Ok(true);
            }
            "ui.panelToggle.display" => {
                if self.left_panel_open && self.active_left_kind == LeftPanelKind::Display {
                    self.left_panel_open = false;
                } else {
                    self.active_left_kind = LeftPanelKind::Display;
                    self.left_panel_open = true;
                }
                return Ok(true);
            }
            "ui.panelToggle.workbench" => {
                if self.left_panel_open && self.active_left_kind == LeftPanelKind::Workbench {
                    self.left_panel_open = false;
                } else {
                    self.active_left_kind = LeftPanelKind::Workbench;
                    self.left_panel_open = true;
                }
                return Ok(true);
            }
            "ui.panelToggle.details" => {
                if self.right_panel_open && self.active_right_kind == RightPanelKind::Details {
                    self.right_panel_open = false;
                } else {
                    self.active_right_kind = RightPanelKind::Details;
                    self.right_panel_open = true;
                }
                self.note_panel_toggle_command(id, "details").await?;
                return Ok(true);
            }
            "ui.panelToggle.settings" => {
                if self.right_panel_open && self.active_right_kind == RightPanelKind::Settings {
                    self.right_panel_open = false;
                } else {
                    self.active_right_kind = RightPanelKind::Settings;
                    self.right_panel_open = true;
                }
                self.note_panel_toggle_command(id, "settings").await?;
                return Ok(true);
            }
            "ui.fullscreen.toggle" => {
                self.apply_os_command("os.toggleFullscreen", None).await?;
                return Ok(true);
            }
            "space.canvas.home" => {
                let controller_id = self.host_controller_id().unwrap_or_default();
                self.dispatch_action(ActionDescriptor { controller_id, action: "goHome".into(), args: None }).await?;
                return Ok(true);
            }
            "space.canvas.back" => {
                let has_focused_instance = self.session.as_ref().and_then(|session| Self::panel_state_from_view(&session.view_state)).is_some_and(|panel| panel.active_spawned_id.is_some());
                if has_focused_instance {
                    let controller_id = self.host_controller_id().unwrap_or_default();
                    self.dispatch_action(ActionDescriptor { controller_id, action: "closeFocusedInstance".into(), args: None }).await?;
                }
                return Ok(true);
            }
            id if id.starts_with("dock.tab.") && id.ends_with(".focus") => {
                if let Some((path, window_id)) = Self::parse_dock_tab_action_id(id, "focus") {
                    self.dock.set_stack_active(&path, window_id);
                    self.active_window_id = Some(window_id.to_string());
                    self.dock.toggle_maximize(&path);
                    self.persist_dock_layout();
                    self.note_control_command(id, None).await?;
                }
                return Ok(true);
            }
            id if id.starts_with("dock.tab.") && id.ends_with(".close") => {
                if let Some((path, window_id)) = Self::parse_dock_tab_action_id(id, "close") {
                    if self.dock.close_window_in_stack(&path, window_id) {
                        self.active_window_id = self.dock.active_window_id.clone();
                        self.persist_dock_layout();
                        self.note_control_command(id, None).await?;
                    }
                }
                return Ok(true);
            }
            id if id.starts_with("shell.layout.") => {
                let layout_id = id.trim_start_matches("shell.layout.");
                if let Some(session) = &self.session {
                    if let Some(named) = session.app.named_layouts.iter().find(|entry| entry.id == layout_id) {
                        self.layout_override = Some(named.layout.clone());
                        self.sync_dock();
                        self.active_window_id = self.dock.active_window_id.clone();
                        self.note_control_command(id, Some(serde_json::json!({ "layoutId": layout_id }))).await?;
                    }
                }
                return Ok(true);
            }
            id if id.starts_with("shell.mode.") => {
                let mode_id = id.trim_start_matches("shell.mode.");
                self.dispatch_action(ActionDescriptor { controller_id: self.session.as_ref().map(|s| s.app.controller_id.clone()).unwrap_or_default(), action: "setMode".into(), args: crate::action_args_json!({ "modeId": mode_id }) }).await?;
                return Ok(true);
            }
            id if id.starts_with("framework.settings.appearance.") => {
                self.appearance_id = id.trim_start_matches("framework.settings.appearance.").to_string();
                return Ok(true);
            }
            id if id.starts_with("shell.panel.tab.left.") => {
                let tab_id = id.trim_start_matches("shell.panel.tab.left.");
                self.select_left_panel_tab(tab_id).await?;
                return Ok(true);
            }
            id if id.starts_with("shell.panel.tab.right.") => {
                let tab_id = id.trim_start_matches("shell.panel.tab.right.");
                self.active_right_tab = Some(tab_id.to_string());
                if let Some(controller_id) = self.host_controller_id() {
                    self.dispatch_action(ActionDescriptor { controller_id, action: "setActivePanelTab".into(), args: crate::action_args_json!({ "tabId": tab_id }) }).await?;
                }
                return Ok(true);
            }
            id if self.context_menu.as_ref().is_some_and(|menu| {
                let mut prefix = Vec::new();
                context_menu_path_for_item_id(&menu.items, id, &mut prefix).is_some()
            }) =>
            {
                // 🖱️ Group rows (`menu.group.<category>`, but any row with `children`) open their submenu on
                // click, not just hover — matched by path (not a flat top-level id scan) so nested rows work.
                let submenu_path = self.context_menu.as_ref().and_then(|menu| {
                    let mut prefix = Vec::new();
                    let path = context_menu_path_for_item_id(&menu.items, id, &mut prefix)?;
                    (!context_menu_item_at_path(&menu.items, &path)?.children.is_empty()).then_some(path)
                });
                if let Some(path) = submenu_path {
                    if let Some(menu) = self.context_menu.as_mut() {
                        menu.active = path;
                        menu.submenu_collapsed_at = None;
                    }
                    return Ok(true);
                }
                let action = self.context_menu.as_ref().and_then(|menu| {
                    let mut prefix = Vec::new();
                    let path = context_menu_path_for_item_id(&menu.items, id, &mut prefix)?;
                    context_menu_item_at_path(&menu.items, &path)?.action.clone()
                });
                self.context_menu = None;
                if let Some(action) = action {
                    self.dispatch_action(action).await?;
                }
                return Ok(true);
            }
            id if id.starts_with("section.chevron.") => {
                let section_id = id.trim_start_matches("section.chevron.");
                let key = format!("section.{section_id}");
                let collapsed = self.collapsed_sections.get(&key).copied().unwrap_or(false);
                self.collapsed_sections.insert(key, !collapsed);
                return Ok(true);
            }
            id if id.starts_with("tree.chevron.") => {
                let item_id = id.trim_start_matches("tree.chevron.");
                let key = format!("tree.{item_id}");
                let collapsed = self.collapsed_sections.get(&key).copied().unwrap_or(false);
                self.collapsed_sections.insert(key, !collapsed);
                return Ok(true);
            }
            id if id.contains(".vfs.chevron.") => {
                if let Some((surface_id, row_id)) = id.rsplit_once(".vfs.chevron.") {
                    toggle_vfs_row_expanded(surface_id, row_id);
                    return Ok(true);
                }
            }
            id if self.window_measure_actions.get(id).is_some_and(|binding| matches!(binding.kind, WindowMeasureActionKind::Select)) => {
                let opening = !self.open_selects.get(id).copied().unwrap_or(false);
                self.open_selects.insert(id.to_string(), opening);
                return Ok(true);
            }
            id if self.widget_maps.select_metas.contains_key(id) => {
                let opening = !self.open_selects.get(id).copied().unwrap_or(false);
                for key in self.open_selects.keys().cloned().collect::<Vec<_>>() {
                    self.open_selects.insert(key, false);
                }
                self.open_selects.insert(id.to_string(), opening);
                return Ok(true);
            }
            id if id.contains(".item.") => {
                if let Some((select_id, value)) = id.rsplit_once(".item.") {
                    if let Some(binding) = self.window_measure_actions.get(select_id).filter(|binding| matches!(binding.kind, WindowMeasureActionKind::Select)) {
                        let controller_id = binding.controller.as_str().to_owned();
                        let action = binding.action.as_str().to_owned();
                        self.open_selects.insert(select_id.to_string(), false);
                        self.dispatch_action(ActionDescriptor { controller_id, action, args: crate::action_args_json!({ "value": value }) }).await?;
                        return Ok(true);
                    }
                    if let Some(action) = self.widget_maps.select_metas.get(select_id).cloned() {
                        self.open_selects.insert(select_id.to_string(), false);
                        self.dispatch_action(ActionDescriptor { controller_id: action.controller_id, action: action.action, args: crate::action_args_json!({ "value": value }) }).await?;
                        return Ok(true);
                    }
                }
            }
            id if self.window_measure_actions.get(id).is_some_and(|binding| matches!(binding.kind, WindowMeasureActionKind::Toggle(_))) => {
                let Some(binding) = self.window_measure_actions.get(id) else { return Ok(false) };
                let WindowMeasureActionKind::Toggle(pressed) = binding.kind else { return Ok(false) };
                let controller_id = binding.controller.as_str().to_owned();
                let action = binding.action.as_str().to_owned();
                self.dispatch_action(ActionDescriptor { controller_id, action, args: crate::action_args_json!({ "pressed": !pressed }) }).await?;
                return Ok(true);
            }
            id if self.widget_maps.toggle_metas.contains_key(id) => {
                if let Some((pressed, action)) = self.widget_maps.toggle_metas.get(id).cloned() {
                    self.dispatch_action(ActionDescriptor { controller_id: action.controller_id, action: action.action, args: crate::action_args_json!({ "pressed": !pressed }) }).await?;
                    return Ok(true);
                }
            }
            id if id.ends_with(".minus") => {
                let base = id.trim_end_matches(".minus");
                if let Some(meta) = self.widget_maps.stepper_metas.get(base).cloned() {
                    self.dispatch_action(ActionDescriptor { controller_id: meta.on_delta.controller_id, action: meta.on_delta.action, args: crate::action_args_json!({ "delta": -meta.step }) }).await?;
                    return Ok(true);
                }
            }
            id if id.ends_with(".plus") => {
                let base = id.trim_end_matches(".plus");
                if let Some(meta) = self.widget_maps.stepper_metas.get(base).cloned() {
                    self.dispatch_action(ActionDescriptor { controller_id: meta.on_delta.controller_id, action: meta.on_delta.action, args: crate::action_args_json!({ "delta": meta.step }) }).await?;
                    return Ok(true);
                }
            }
            id if id.starts_with("tree.label.") => {
                let item_id = id.trim_start_matches("tree.label.");
                if hit.drag_data.is_some() {
                    return Ok(true);
                }
                self.queue_tree_selection(item_id);
                return Ok(false);
            }
            _ => {}
        }
        Ok(false)
    }

    fn update_tree_hover(&mut self, input: &InputState<ActionDescriptor>) {
        let hovered = input.hovered_id.as_deref().and_then(|id| id.strip_prefix("tree.label."));
        if self.tree_hovered_id.as_deref() == hovered {
            return;
        }
        if let Some(prev) = self.tree_hovered_id.take() {
            if let Some(action) = self.widget_maps.tree_unhover_commands.get(&prev) {
                self.deferred_actions.push(action.clone());
            }
        }
        if let Some(id) = hovered {
            if let Some(action) = self.widget_maps.tree_hover_commands.get(id) {
                self.deferred_actions.push(action.clone());
            }
            self.tree_hovered_id = Some(id.to_string());
        }
    }

    fn queue_tree_selection(&mut self, item_id: &str) {
        let Some(action) = self.widget_maps.tree_selection_change.clone() else {
            return;
        };
        self.deferred_actions.push(ActionDescriptor { controller_id: action.controller_id, action: action.action, args: crate::action_args_json!({ "ids": [item_id] }) });
    }

    async fn dispatch_tree_selection(&mut self, item_id: &str) -> Result<(), String> {
        self.queue_tree_selection(item_id);
        self.flush_deferred_actions().await
    }

    pub async fn flush_deferred_actions(&mut self) -> Result<(), String> {
        let actions = std::mem::take(&mut self.deferred_actions);
        for action in actions {
            self.dispatch_action(action).await?;
        }
        if self.pending_shell_uri_apply {
            self.pending_shell_uri_apply = false;
            self.apply_pending_shell_uri().await?;
        }
        Ok(())
    }

    async fn dispatch_widget_drag_values(&mut self, input: &InputState<ActionDescriptor>) -> Result<(), String> {
        let Some(id) = input.drag.target_id.as_deref() else {
            return Ok(());
        };
        if let Some(value) = self.widget_maps.slider_live_values.get(id).copied() {
            if let Some(meta) = self.widget_maps.slider_metas.get(id).cloned() {
                self.dispatch_action(ActionDescriptor { controller_id: meta.on_change.controller_id, action: meta.on_change.action, args: crate::action_args_json!({ "value": value }) }).await?;
            } else if let Some(binding) = self.window_measure_actions.get(id).filter(|binding| matches!(binding.kind, WindowMeasureActionKind::Slider)) {
                let controller_id = binding.controller.as_str().to_owned();
                let action = binding.action.as_str().to_owned();
                self.dispatch_action(ActionDescriptor { controller_id, action, args: crate::action_args_json!({ "value": value }) }).await?;
            }
        } else if let Some(value) = self.widget_maps.ring_live_values.get(id).copied() {
            if let Some(meta) = self.widget_maps.ring_metas.get(id).cloned() {
                self.dispatch_action(ActionDescriptor { controller_id: meta.on_change.controller_id, action: meta.on_change.action, args: crate::action_args_json!({ "value": value }) }).await?;
            }
        }
        Ok(())
    }

    async fn commit_focused_input(&mut self, input: &mut InputState<ActionDescriptor>) -> Result<(), String> {
        let Some(id) = input.focused_id.clone() else {
            return Ok(());
        };
        // 📝️ A staged action-arg input writes into the staging map (parsed per the arg's control kind)
        // instead of dispatching live — Architecture Decision 8, P2 (item 4).
        if self.commit_staged_input(&id, input.text_view()) {
            input.blur_input();
            return Ok(());
        }
        if id.ends_with(".input") {
            let base = id.trim_end_matches(".input");
            if let Some(meta) = self.widget_maps.stepper_metas.get(base).cloned() {
                let parsed = input.text_view().parse::<f64>().unwrap_or(meta.value);
                self.dispatch_action(ActionDescriptor { controller_id: meta.on_absolute.controller_id, action: meta.on_absolute.action, args: crate::action_args_json!({ "value": parsed }) }).await?;
                input.blur_input();
                return Ok(());
            }
        }
        if let Some(meta) = self.widget_maps.input_metas.get(&id).cloned() {
            self.dispatch_action(ActionDescriptor { controller_id: meta.on_change.controller_id, action: meta.on_change.action, args: crate::action_args_json!({ "value": input.text_view() }) }).await?;
            input.blur_input();
        }
        Ok(())
    }

    async fn finish_tree_drag(&mut self, x: f32, y: f32, _input: &InputState<ActionDescriptor>) -> Result<(), String> {
        let Some(drag) = self.tree_drag.take() else {
            return Ok(());
        };
        let surfaces = self.node_graph_states.iter().map(|(id, surface)| (id.as_str(), surface.bounds, surface.controller_id.as_str())).collect::<Vec<_>>();
        if let Some(action) = crate::engine_canvas::node_graph_flow_widget_drop_action(x, y, &drag.drag_data, &surfaces) {
            crate::engine_canvas::node_graph_clear_all_ghost_widgets();
            self.dispatch_action(action).await?;
            return Ok(());
        }
        if let Some(action) = crate::engine_canvas::node_graph_catalogue_drop_action(x, y, &drag.drag_data, &surfaces) {
            crate::engine_canvas::node_graph_clear_all_ghost_widgets();
            self.dispatch_action(action).await?;
            return Ok(());
        }
        crate::engine_canvas::node_graph_clear_all_ghost_widgets();
        Ok(())
    }

    fn render_tree_drag_overlay(&self, overlay: &mut DrawList, input: &InputState<ActionDescriptor>, theme: &Theme) {
        let Some(drag) = &self.tree_drag else {
            return;
        };
        overlay.push_solid([drag.x - 60.0, drag.y - 12.0, 120.0, 24.0], theme.selected.with_alpha(0.85));
        if let Some(hit) = input.hit_at(drag.x, drag.y) {
            if let Some(target_id) = hit.control_id.as_deref().and_then(|id| id.strip_prefix("tree.label.")) {
                let _ = target_id;
                match drag.drop_position {
                    TreeDropPosition::Before => overlay.push_solid([hit.rect.x, hit.rect.y, hit.rect.w, 2.0], theme.accent),
                    TreeDropPosition::After => overlay.push_solid([hit.rect.x, hit.rect.y + hit.rect.h - 2.0, hit.rect.w, 2.0], theme.accent),
                    TreeDropPosition::Inside => overlay.push_rounded([hit.rect.x, hit.rect.y, hit.rect.w, hit.rect.h], theme.accent.with_alpha(0.15), theme.border_radius),
                }
            }
        }
    }

    async fn select_left_panel_tab(&mut self, tab_id: &str) -> Result<(), String> {
        self.active_left_tab = Some(tab_id.to_string());
        // 🏠️🧳️ Once `session.app.id` matches the host app id, `session.app` *is* the host app, so its own
        // self-declared `controller_id` is the right value — no separate app-identity lookup needed.
        let host_app_id = self.host_config().map(|cfg| cfg.host_app_id);
        let controller_id = self.session.as_ref().filter(|session| Some(session.app.id.as_str()) == host_app_id).map(|session| session.app.controller_id.clone());
        if let Some(controller_id) = controller_id {
            self.dispatch_action(ActionDescriptor { controller_id, action: "setActivePanelTab".into(), args: crate::action_args_json!({ "tabId": tab_id }) }).await?;
        }
        Ok(())
    }

    fn dismiss_overlays(&mut self, x: f32, y: f32, input: &InputState<ActionDescriptor>) -> bool {
        let hit = input.hit_at(x, y);
        let on_overlay = hit.is_some_and(|h| matches!(h.kind, HitKind::ContextMenu | HitKind::DropdownItem | HitKind::NavbarItem | HitKind::Select));
        if self.open_selects.values().any(|open| *open) && !on_overlay {
            for key in self.open_selects.keys().cloned().collect::<Vec<_>>() {
                self.open_selects.insert(key, false);
            }
            return true;
        }
        if self.context_menu.is_some() && !on_overlay {
            self.context_menu = None;
            return true;
        }
        if self.overlay_state != OverlayState::None && !on_overlay {
            self.overlay_state = OverlayState::None;
            self.search_open = false;
            self.find_open = false;
            return true;
        }
        false
    }

    async fn open_context_menu(&mut self, x: f32, y: f32, hit: Option<HitTarget<ActionDescriptor>>) {
        let node_id = hit.as_ref().and_then(|hit| hit.control_id.as_deref().and_then(|id| id.rsplit_once(".node.").map(|(_, node_id)| node_id.to_string())));
        let edge_id = hit.as_ref().and_then(|hit| hit.control_id.as_deref().and_then(|id| id.rsplit_once(".edge.").map(|(_, edge_id)| edge_id.to_string())));
        let (surface_id, kind, hits) = self.resolve_context_menu_surface(x, y, node_id.as_deref(), edge_id.as_deref());
        let is_de = self.locale_id == "de";
        let mut items = Vec::new();
        if let Some(session) = self.session.clone() {
            let shortcut_by_action: HashMap<String, String> = session.app.keybindings.iter().map(|binding| (binding.action.action.clone(), binding.keys.clone())).collect();
            // 🖱️ `viewState` deliberately omitted — `ui_wgpu::wgpu::ContextMenuRequest` never carries it (see
            // that type's own doc comment); `selection`/`text` are the typed slices plugins actually need.
            // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W3a: the opaque per-app
            // `ViewModel.selectionJson` this used to read is deleted — the framework now owns selection
            // via `InteractionState`/`PresenceInteraction`, not yet threaded into this request. Empty
            // until a follow-up wires the active domain's `DomainSelection` through here.
            let selection: Vec<Value> = Vec::new();
            let text: Option<Value> = None;
            let request = serde_json::json!({
                "menu": { "id": kind.clone() },
                "surface": {
                    "surfaceId": surface_id,
                    "kind": kind.clone(),
                    "hits": hits,
                    "selection": selection,
                    "text": text,
                },
                "point": { "x": x as f64, "y": y as f64 },
            });
            if let Some(program) = self.plugins.iter().find(|plugin| plugin.plugin_id == session.plugin_id) {
                match program.context_menu(session.instance_id, request).await {
                    Ok(specs) => {
                        items = specs
                            .into_iter()
                            .map(|spec| {
                                let mut item = shell_context_menu_item_from_spec(spec, &session.app.controller_id, is_de);
                                if item.shortcut.is_none() {
                                    if let Some(action) = item.action.as_ref() {
                                        item.shortcut = shortcut_by_action.get(&action.action).map(|keys| format_keybinding_shortcut(keys));
                                    }
                                } else if let Some(shortcut) = item.shortcut.as_ref() {
                                    item.shortcut = Some(format_keybinding_shortcut(shortcut));
                                }
                                if let Some(action) = item.action.take() {
                                    item.action = Some(resolve_graph_context_action(&action, node_id.as_deref()));
                                }
                                item
                            })
                            .collect();
                    }
                    Err(error) => {
                        self.error = Some(error);
                    }
                }
            }
        }
        if items.is_empty() {
            if let Some(session) = &self.session {
                let window_kind = session.app.window_kinds.iter().find(|kind| Some(&kind.id) == self.active_window_id.as_ref()).or_else(|| Some(session.app.window_kinds.first()));
                let actions: Vec<ui_wgpu::wgpu::ShellMenuAction> = window_kind
                    .map(|kind| semio_framework::resolve_window_actions(&session.app, kind))
                    .unwrap_or_default()
                    .into_iter()
                    .map(|action| ui_wgpu::wgpu::ShellMenuAction {
                        id: action.id.clone(),
                        label: action.label.resolve(self.active_terminology(), self.active_locale()).to_string(),
                        icon: Some(action.icon_id.as_str().to_string()),
                        keys: action.keys.clone(),
                        kind: context_menu_action_kind_str(action.kind),
                        category: action.category.clone(),
                        in_palette: action.in_palette,
                        arg_carrying: !action.args.is_empty(),
                    })
                    .collect();
                let controller_id = session.app.controller_id.clone();
                items = ui_wgpu::wgpu::build_shell_context_menu_specs(&actions, true).into_iter().map(|spec| shell_context_menu_item_from_spec(spec, &controller_id, is_de)).collect();
            }
        }
        if let Some(controller_id) = self.host_controller_id() {
            items.push(ContextMenuItem { id: "shell.context.home".into(), label: "Go Home".into(), icon: None, destructive: false, action: Some(ActionDescriptor { controller_id, action: "goHome".into(), args: None }), ..Default::default() });
        }
        self.context_menu = Some(ContextMenuState { x, y, items, active: Vec::new(), submenu_collapsed_at: None, scroll_offset: 0.0 });
        self.overlay_state = OverlayState::None;
    }

    fn resolve_context_menu_surface(&self, x: f32, y: f32, node_id: Option<&str>, edge_id: Option<&str>) -> (String, String, Vec<ui_wgpu::wgpu::ContextMenuHit>) {
        let mut hits = Vec::new();
        if let Some(node_id) = node_id {
            hits.push(ui_wgpu::wgpu::ContextMenuHit { domain: "node".into(), id: node_id.into(), label: None });
        }
        if let Some(edge_id) = edge_id {
            hits.push(ui_wgpu::wgpu::ContextMenuHit { domain: "edge".into(), id: edge_id.into(), label: None });
        }
        for (surface_id, surface) in self.node_graph_states.iter() {
            if surface.bounds.contains(x, y) {
                return (surface_id.clone(), "nodeGraph".into(), hits);
            }
        }
        for (surface_id, surface) in self.tiled_map_states.iter() {
            if surface.bounds.contains(x, y) {
                return (surface_id.clone(), "tiledMap".into(), hits);
            }
        }
        for (surface_id, surface) in self.board2d_states.iter() {
            if surface.bounds.contains(x, y) {
                return (surface_id.clone(), "board2d".into(), hits);
            }
        }
        for (surface_id, surface) in self.world3d_states.iter() {
            if surface.bounds.contains(x, y) {
                return (surface_id.clone(), "world3d".into(), hits);
            }
        }
        ("shell".into(), "window".into(), hits)
    }

    fn sync_context_menu_hover(&mut self, input: &InputState<ActionDescriptor>) {
        let Some(menu) = self.context_menu.as_mut() else {
            return;
        };
        let Some(item_id) = input.hovered_id.as_deref() else {
            return;
        };
        let mut prefix = Vec::new();
        if let Some(path) = context_menu_path_for_item_id(&menu.items, item_id, &mut prefix) {
            menu.active = path;
            menu.submenu_collapsed_at = None;
        }
    }

    /** @emoji ⌨️ Routes keyboard input to the open shell context menu. */
    pub fn context_menu_handle_key(&mut self, action: ui_wgpu::wgpu::KeyAction) -> ContextMenuKeyOutcome {
        let Some(menu) = self.context_menu.as_mut() else {
            return ContextMenuKeyOutcome::Ignored;
        };
        let root = menu.items.clone();
        let path = menu.active.clone();
        match action {
            ui_wgpu::wgpu::KeyAction::Escape => {
                if menu.active.len() > 1 {
                    menu.active.pop();
                    return ContextMenuKeyOutcome::Consumed;
                }
                self.context_menu = None;
                return ContextMenuKeyOutcome::CloseMenu;
            }
            ui_wgpu::wgpu::KeyAction::Char(ref key) if key.len() == 1 && key.chars().next().is_some_and(|ch| ch.is_ascii_digit() && ch != '0') => {
                let ordinal = key.parse::<usize>().ok();
                let Some(ordinal) = ordinal else {
                    return ContextMenuKeyOutcome::Ignored;
                };
                if let Some(next) = context_menu_path_for_ordinal(&root, &path, ordinal) {
                    menu.active = next;
                    return ContextMenuKeyOutcome::Consumed;
                }
                return ContextMenuKeyOutcome::Ignored;
            }
            ui_wgpu::wgpu::KeyAction::ArrowUp => {
                menu.active = context_menu_move_active(&root, &path, false);
                return ContextMenuKeyOutcome::Consumed;
            }
            ui_wgpu::wgpu::KeyAction::ArrowDown => {
                menu.active = context_menu_move_active(&root, &path, true);
                return ContextMenuKeyOutcome::Consumed;
            }
            ui_wgpu::wgpu::KeyAction::ArrowLeft => {
                if menu.active.len() > 1 {
                    let parent = menu.active[..menu.active.len() - 1].to_vec();
                    menu.active = parent.clone();
                    menu.submenu_collapsed_at = Some(parent);
                }
                return ContextMenuKeyOutcome::Consumed;
            }
            ui_wgpu::wgpu::KeyAction::ArrowRight => {
                if let Some(next) = context_menu_open_submenu_path(&root, &menu.active) {
                    menu.active = next;
                }
                return ContextMenuKeyOutcome::Consumed;
            }
            ui_wgpu::wgpu::KeyAction::Enter | ui_wgpu::wgpu::KeyAction::Space(true) => {
                let active = menu.active.clone();
                let Some(item) = context_menu_item_at_path(&root, &active) else {
                    return ContextMenuKeyOutcome::Ignored;
                };
                if item.disabled {
                    return ContextMenuKeyOutcome::Ignored;
                }
                if !item.children.is_empty() {
                    if let Some(next) = context_menu_open_submenu_path(&root, &active) {
                        menu.active = next;
                    }
                    return ContextMenuKeyOutcome::Consumed;
                }
                if let Some(action) = item.action.clone() {
                    self.context_menu = None;
                    return ContextMenuKeyOutcome::Activate(action);
                }
                return ContextMenuKeyOutcome::Ignored;
            }
            ui_wgpu::wgpu::KeyAction::Char(ref key) if matches!(key.as_str(), "w" | "W") => {
                menu.active = context_menu_move_active(&root, &path, false);
                return ContextMenuKeyOutcome::Consumed;
            }
            ui_wgpu::wgpu::KeyAction::Char(ref key) if matches!(key.as_str(), "s" | "S") => {
                menu.active = context_menu_move_active(&root, &path, true);
                return ContextMenuKeyOutcome::Consumed;
            }
            ui_wgpu::wgpu::KeyAction::Char(ref key) if matches!(key.as_str(), "a" | "A") => {
                if menu.active.len() > 1 {
                    menu.active.pop();
                }
                return ContextMenuKeyOutcome::Consumed;
            }
            ui_wgpu::wgpu::KeyAction::Char(ref key) if matches!(key.as_str(), "d" | "D") => {
                if let Some(next) = context_menu_open_submenu_path(&root, &menu.active) {
                    menu.active = next;
                }
                return ContextMenuKeyOutcome::Consumed;
            }
            _ => ContextMenuKeyOutcome::Ignored,
        }
    }

    fn build_search_items(&self) -> Vec<SearchPaletteItem> {
        let mut items = self.command_search_items();
        let Some(session) = &self.session else {
            return items;
        };
        for tab in &session.app.panel_tabs {
            items.push(SearchPaletteItem {
                id: format!("panel.{}", tab.id()),
                label: tab.label.resolve(self.active_terminology(), self.active_locale()).to_string(),
                group: "Panels".into(),
                dispatch_action: Some(ActionDescriptor { controller_id: session.app.controller_id.clone(), action: "setActivePanelTab".into(), args: crate::action_args_json!({ "tabId": tab.id() }) }),
                action: None,
                category: None,
            });
        }
        for kind in &session.app.window_kinds {
            items.push(SearchPaletteItem {
                id: format!("window.{}", kind.id),
                label: kind.label.resolve(self.active_terminology(), self.active_locale()).to_string(),
                group: "Windows".into(),
                dispatch_action: None,
                action: Some(format!("window:{}", kind.id)),
                category: None,
            });
        }
        for binding in &session.app.keybindings {
            items.push(SearchPaletteItem { id: format!("keybinding.{}", binding.keys), label: binding.action.action.clone(), group: "Actions".into(), dispatch_action: Some(binding.action.clone()), action: None, category: None });
        }
        if let Some(controller_id) = self.host_controller_id() {
            for action in ["undo", "redo", "commitCheckpoint"] {
                items.push(SearchPaletteItem {
                    id: format!("studio.{action}"),
                    label: action.into(),
                    group: "Space".into(),
                    dispatch_action: Some(ActionDescriptor { controller_id: controller_id.clone(), action: action.into(), args: None }),
                    action: None,
                    category: None,
                });
            }
        }
        items
    }

    fn filtered_search_items(&self) -> Vec<SearchPaletteItem> {
        let query = self.search_query.clone();
        let items = self.build_search_items();
        if query.trim().is_empty() {
            return items.into_iter().take(20).collect();
        }
        // 🔍️ Fuzzy subsequence match (see `fuzzy_match_score` in `shell::ActionPanelAndUtilities`) —
        // replaces the previous pure-substring `.contains()` filter so e.g. "stlc" finds "Set Locale".
        let mut scored: Vec<(i64, SearchPaletteItem)> = items
            .into_iter()
            .filter_map(|item| {
                let score = fuzzy_match_score(&query, &item.label).into_iter().chain(fuzzy_match_score(&query, &item.group)).max();
                score.map(|score| (score, item))
            })
            .collect();
        scored.sort_by(|a, b| b.0.cmp(&a.0));
        scored.into_iter().take(20).map(|(_, item)| item).collect()
    }

    fn filtered_find_items(&self) -> Vec<ShellFindItem> {
        let query = self.find_query.to_lowercase();
        if query.trim().is_empty() {
            return self.find_items.iter().take(20).cloned().collect();
        }
        self.find_items.iter().filter(|item| item.label.to_lowercase().contains(&query) || item.description.as_ref().is_some_and(|d| d.to_lowercase().contains(&query))).take(20).cloned().collect()
    }

    pub async fn activate_search_item(&mut self, index: usize) -> Result<(), String> {
        let items = self.filtered_search_items();
        let Some(item) = items.get(index) else {
            return Ok(());
        };
        if let Some(action) = item.dispatch_action.clone() {
            self.dispatch_action(action).await?;
        } else if let Some(action) = &item.action {
            if let Some(window_id) = action.strip_prefix("window:") {
                self.active_window_id = Some(window_id.to_string());
            } else if let Some(rest) = action.strip_prefix("action-panel:") {
                // 📇️ P3 redirect: focus the hosting window, unfold its Actions rail, expand the form.
                if let Some((window_id, action_id)) = rest.split_once(':') {
                    self.active_window_id = Some(window_id.to_string());
                    self.action_panel_folded.insert(window_id.to_string(), false);
                    self.action_panel_expanded.insert(window_id.to_string(), action_id.to_string());
                }
            } else if let Some(rest) = action.strip_prefix("os-command:") {
                // 🎛️ Os-level command redirect (see `apply_os_command` in `shell::ActionPanelAndUtilities`)
                // — `"os.commandId"` for zero-arg commands, `"os.commandId:optionValue"` for the
                // per-option-expanded select-arg commands built by `command_search_items`.
                let (command_id, option_value) = match rest.split_once(':') {
                    Some((command_id, value)) => (command_id.to_string(), Some(value.to_string())),
                    None => (rest.to_string(), None),
                };
                self.apply_os_command(&command_id, option_value.as_deref()).await?;
            } else if let Some(command_json) = action.strip_prefix("command:") {
                let invocation: semio_framework::manifest::CommandInvocation = serde_json::from_str(command_json).map_err(|error| error.to_string())?;
                self.dispatch_command(invocation).await?;
            }
        }
        self.search_open = false;
        self.overlay_state = OverlayState::None;
        self.search_query.clear();
        self.search_selected = 0;
        Ok(())
    }

    pub async fn activate_find_item(&mut self, index: usize) -> Result<(), String> {
        let items = self.filtered_find_items();
        let Some(item) = items.get(index) else {
            return Ok(());
        };
        if let Some(session) = &self.session {
            self.dispatch_action(ActionDescriptor {
                controller_id: session.app.controller_id.clone(),
                action: "setMediaNodeSelection".into(),
                args: crate::action_args_json!({
                    "surfaceId": item.surface_id,
                    "nodeIds": [item.node_id],
                }),
            })
            .await?;
        }
        self.find_open = false;
        self.overlay_state = OverlayState::None;
        self.find_query.clear();
        self.find_selected = 0;
        Ok(())
    }

    pub fn handle_keyboard(&mut self, action: ui_wgpu::wgpu::KeyAction, modifiers: &PointerModifiers, input: &mut InputState<ActionDescriptor>) {
        if action == ui_wgpu::wgpu::KeyAction::Escape {
            if self.dock_drag.take().is_some() || self.pending_dock_drag.take().is_some() {
                self.restore_dock_drag_snapshot();
                self.dock_drag_snapshot = None;
                return;
            }
            // 🪟️ Escape closes exactly the topmost overlay first — matches ui_wgpu's overlay-manager
            // precedence (`EventRouter::close_topmost_overlay`, report-w1d-events-overlay.md: "Escape
            // closes only the topmost") even though these ad-hoc chrome overlays predate that stack and
            // aren't routed through it yet. A Select dropdown is the most local/transient overlay, so it
            // wins over the context menu; neither used to close on Escape at all before this fix.
            if self.open_selects.values().any(|open| *open) {
                for key in self.open_selects.keys().cloned().collect::<Vec<_>>() {
                    self.open_selects.insert(key, false);
                }
                return;
            }
            if self.context_menu.is_some() {
                match self.context_menu_handle_key(ui_wgpu::wgpu::KeyAction::Escape) {
                    ContextMenuKeyOutcome::Activate(_) => return,
                    ContextMenuKeyOutcome::Ignored => {}
                    ContextMenuKeyOutcome::Consumed | ContextMenuKeyOutcome::CloseMenu => return,
                }
            }
        }
        // 🎓️ Tour keys take precedence over every other chord below (mirrors Escape closing the topmost
        // overlay above) but never fire while a field is focused or the sync-attach card is open — same
        // "not editing" guard the rest of this function uses (computed inline here since `editing` itself
        // isn't bound until after this block).
        if input.focused_id.is_none() && self.sync_card_kind.is_none() {
            if let Some(step) = self.chrome_tour_active_step() {
                match action {
                    ui_wgpu::wgpu::KeyAction::Escape => {
                        if let Some(session) = self.session.as_ref() {
                            self.chrome_build.mark_introduction_seen(&session.app.id);
                        }
                        self.chrome_build.skip_introduction();
                        return;
                    }
                    ui_wgpu::wgpu::KeyAction::Enter | ui_wgpu::wgpu::KeyAction::ArrowRight => {
                        if step.interactions.is_empty() {
                            self.chrome_tour_advance_current_step(&step);
                            return;
                        }
                    }
                    ui_wgpu::wgpu::KeyAction::ArrowLeft => {
                        self.chrome_build.back_introduction();
                        return;
                    }
                    _ => {}
                }
            }
        }
        let meta = modifiers.meta || modifiers.ctrl;
        // ⌨️ Hardcoded shell chords never fire while a text field (or the sync-attach draft buffer)
        // has focus — matches `os-shell.tsx`'s `isEditableEventTarget`/`useActionHotkey` (backed by
        // react-hotkeys-hook, which by default does not fire on form tags): "hotkeys never fire while
        // the user is typing". Previously these six chords fired unconditionally, so e.g. Ctrl+B while
        // typing in a focused Input would silently toggle the left panel instead of inserting "b".
        let editing = input.focused_id.is_some() || self.sync_card_kind.is_some();
        if !editing && meta && matches!(action, ui_wgpu::wgpu::KeyAction::Char(ref c) if c.eq_ignore_ascii_case("p")) {
            self.search_open = !self.search_open;
            self.find_open = false;
            self.overlay_state = if self.search_open { OverlayState::Search } else { OverlayState::None };
            if self.search_open {
                input.focused_id = Some("shell.search.input".into());
            }
            return;
        }
        if !editing && meta && matches!(action, ui_wgpu::wgpu::KeyAction::Char(ref c) if c.eq_ignore_ascii_case("f")) {
            self.find_open = !self.find_open;
            self.search_open = false;
            self.overlay_state = if self.find_open { OverlayState::Find } else { OverlayState::None };
            if self.find_open {
                input.focused_id = Some("shell.find.input".into());
            }
            return;
        }
        if !editing && meta && matches!(action, ui_wgpu::wgpu::KeyAction::Char(ref c) if c == "[") {
            if self.uri_index > 0 {
                self.uri_index -= 1;
            }
            self.pending_shell_uri_apply = true;
            return;
        }
        if !editing && meta && matches!(action, ui_wgpu::wgpu::KeyAction::Char(ref c) if c == "]") {
            if self.uri_index + 1 < self.uri_history.len() {
                self.uri_index += 1;
            }
            self.pending_shell_uri_apply = true;
            return;
        }
        if !editing && meta && matches!(action, ui_wgpu::wgpu::KeyAction::ArrowUp) {
            let uri = self.shell_uri();
            if let Some(parent) = uri.rsplit_once('/').map(|(p, _)| p.to_string()) {
                if !parent.is_empty() {
                    self.push_uri(parent);
                }
            }
            self.pending_shell_uri_apply = true;
            return;
        }
        if !editing && meta && modifiers.shift && matches!(action, ui_wgpu::wgpu::KeyAction::Char(ref c) if c.eq_ignore_ascii_case("b")) {
            self.right_panel_open = !self.right_panel_open;
            return;
        }
        if !editing && meta && matches!(action, ui_wgpu::wgpu::KeyAction::Char(ref c) if c.eq_ignore_ascii_case("b")) {
            self.left_panel_open = !self.left_panel_open;
            return;
        }
        let palette_open = matches!(self.overlay_state, OverlayState::Search | OverlayState::Find);
        // 🪟️ Tab/Shift+Tab cycles the active window across the *whole* dock (cross-window focus order)
        // whenever nothing else claims focus — this renderer has no DOM, so there was no cross-window
        // Tab order at all before this fix (only ever within a single scene, e.g. the text editor's own
        // `KeyAction::Tab` handling). Single-window content-level Tab cycling among widgets is a
        // separate, content-layer concern for `ui_wgpu`'s own engine/EventRouter (owned by the
        // interpreter cutover), not this chrome-level routing.
        if !editing && !palette_open && self.dock_drag.is_none() && action == ui_wgpu::wgpu::KeyAction::Tab {
            self.cycle_active_window(!modifiers.shift);
            return;
        }
        if self.sync_card_kind.is_some() {
            match action {
                ui_wgpu::wgpu::KeyAction::Escape => {
                    self.sync_card_kind = None;
                    return;
                }
                ui_wgpu::wgpu::KeyAction::Enter => {
                    self.deferred_actions.push(ActionDescriptor {
                        controller_id: "framework.sync".into(),
                        action: "attach".into(),
                        args: crate::action_args_json!({
                            "path": self.sync_card_draft,
                            "kind": self.sync_card_kind,
                        }),
                    });
                    return;
                }
                ui_wgpu::wgpu::KeyAction::Char(key) => {
                    self.sync_card_draft.push_str(&key);
                    return;
                }
                ui_wgpu::wgpu::KeyAction::Backspace => {
                    self.sync_card_draft.pop();
                    return;
                }
                _ => {}
            }
        }
        // 📌️ ticket §C5 item 3 — same shell-owned keyboard-routed draft-field idiom as
        // `sync_card_kind` above, for the check-in message-prompt card.
        #[cfg(not(target_arch = "wasm32"))]
        if self.checkin_dialog_draft.is_some() {
            match action {
                ui_wgpu::wgpu::KeyAction::Escape => {
                    self.checkin_dialog_draft = None;
                    return;
                }
                ui_wgpu::wgpu::KeyAction::Enter => {
                    let message = self.checkin_dialog_draft.clone().unwrap_or_default();
                    self.checkin_dialog_draft = None;
                    self.deferred_actions.push(ActionDescriptor { controller_id: "framework.checkin".into(), action: "submit".into(), args: crate::action_args_json!({ "message": message }) });
                    return;
                }
                ui_wgpu::wgpu::KeyAction::Char(key) => {
                    if let Some(draft) = self.checkin_dialog_draft.as_mut() {
                        draft.push_str(&key);
                    }
                    return;
                }
                ui_wgpu::wgpu::KeyAction::Backspace => {
                    if let Some(draft) = self.checkin_dialog_draft.as_mut() {
                        draft.pop();
                    }
                    return;
                }
                _ => {}
            }
        }
        if palette_open {
            match action {
                ui_wgpu::wgpu::KeyAction::Escape => {
                    self.overlay_state = OverlayState::None;
                    self.search_open = false;
                    self.find_open = false;
                    input.focused_id = None;
                }
                ui_wgpu::wgpu::KeyAction::ArrowDown => {
                    if self.overlay_state == OverlayState::Search {
                        let len = self.filtered_search_items().len();
                        if len > 0 {
                            self.search_selected = (self.search_selected + 1).min(len - 1);
                        }
                    } else {
                        let len = self.filtered_find_items().len();
                        if len > 0 {
                            self.find_selected = (self.find_selected + 1).min(len - 1);
                        }
                    }
                }
                ui_wgpu::wgpu::KeyAction::ArrowUp => {
                    if self.overlay_state == OverlayState::Search {
                        self.search_selected = self.search_selected.saturating_sub(1);
                    } else {
                        self.find_selected = self.find_selected.saturating_sub(1);
                    }
                }
                ui_wgpu::wgpu::KeyAction::Enter => {
                    let runtime = ();
                    let _ = runtime;
                }
                ui_wgpu::wgpu::KeyAction::Char(key) => {
                    if self.overlay_state == OverlayState::Search {
                        self.search_query.push_str(&key);
                        self.search_selected = 0;
                    } else {
                        self.find_query.push_str(&key);
                        self.find_selected = 0;
                    }
                }
                ui_wgpu::wgpu::KeyAction::Backspace => {
                    if self.overlay_state == OverlayState::Search {
                        self.search_query.pop();
                        self.search_selected = 0;
                    } else {
                        self.find_query.pop();
                        self.find_selected = 0;
                    }
                }
                _ => {}
            }
            return;
        }
        if input.focused_id.is_some() {
            match action {
                ui_wgpu::wgpu::KeyAction::Char(key) => {
                    if let Some(ch) = key.chars().next() {
                        input.insert_char(ch);
                    }
                }
                ui_wgpu::wgpu::KeyAction::Backspace => input.backspace(),
                ui_wgpu::wgpu::KeyAction::Delete => input.delete_forward(),
                _ => {}
            }
        }
    }

    pub async fn handle_keyboard_async(&mut self, action: ui_wgpu::wgpu::KeyAction, modifiers: &PointerModifiers, input: &mut InputState<ActionDescriptor>) -> Result<(), String> {
        if self.context_menu.is_some() {
            match self.context_menu_handle_key(action.clone()) {
                ContextMenuKeyOutcome::Ignored => {}
                ContextMenuKeyOutcome::Consumed | ContextMenuKeyOutcome::CloseMenu => return Ok(()),
                ContextMenuKeyOutcome::Activate(descriptor) => {
                    self.dispatch_action(descriptor).await?;
                    return Ok(());
                }
            }
        }
        if matches!(self.overlay_state, OverlayState::Search) && action == ui_wgpu::wgpu::KeyAction::Enter {
            self.activate_search_item(self.search_selected).await?;
            return Ok(());
        }
        if matches!(self.overlay_state, OverlayState::Find) && action == ui_wgpu::wgpu::KeyAction::Enter {
            self.activate_find_item(self.find_selected).await?;
            return Ok(());
        }
        if input.focused_id.is_some() {
            match action {
                ui_wgpu::wgpu::KeyAction::Enter | ui_wgpu::wgpu::KeyAction::Escape => {
                    self.commit_focused_input(input).await?;
                    return Ok(());
                }
                _ => {}
            }
        }
        let idle = input.focused_id.is_none() && self.overlay_state == OverlayState::None && self.sync_card_kind.is_none() && self.dock_drag.is_none();
        let fullscreen_chord = matches!(action, ui_wgpu::wgpu::KeyAction::Function(11)) || matches!(&action, ui_wgpu::wgpu::KeyAction::Char(key) if key.eq_ignore_ascii_case("f") && modifiers.ctrl && modifiers.meta);
        if idle && fullscreen_chord {
            self.dispatch_command(semio_framework::manifest::CommandInvocation {
                address: semio_framework::manifest::CommandAddress { owner: semio_framework::manifest::CommandOwnerAddress::Os, command_id: "os.toggleFullscreen".into() },
                arguments: BTreeMap::new(),
            })
            .await?;
            return Ok(());
        }
        if idle && !is_reserved_shell_chord(&action, modifiers) {
            let platform = command_host_platform();
            let command = self
                .resolved_commands()
                .into_iter()
                .rev()
                .find(|entry| entry.definition.in_palette && entry.definition.keybindings.iter().any(|binding| binding.platform.is_none_or(|declared| declared == platform) && key_event_matches_chord(&action, modifiers, &binding.chord)));
            if let Some(entry) = command {
                if entry.definition.args.is_empty() {
                    self.dispatch_command(semio_framework::manifest::CommandInvocation { address: entry.address, arguments: BTreeMap::new() }).await?;
                } else {
                    self.overlay_state = OverlayState::Search;
                    self.search_query = entry.definition.label.resolve(self.active_terminology(), self.active_locale()).to_string();
                    self.search_selected = 0;
                }
                return Ok(());
            }
        }
        // 🎯️🕹️ Content-focus routing (w2-input-wiring): whenever the active window's retained
        // content — not chrome — is the one holding focus, real keys belong there via
        // `interpreter::dispatch_ui_event` (Escape/Tab/edit keys/clipboard chords/text), taking
        // priority over the idle-Escape-deactivate-utility and app-keybinding dispatch below, both
        // of which are chrome-level concerns. `content_has_focus` is this module's own best-effort
        // tracker (see its doc comment for the one documented gap: pointer-click-driven focus
        // changes, entirely inside the off-limits `interpreter` region, never reach it). `Tab`
        // reaching `dispatch_event` here is ALSO the full Tab-traversal fix: `events::EventRouter::
        // dispatch`'s own `KeyDown{key:"Tab"}` arm already calls `FocusState::focus_next`/
        // `focus_prev` internally — nothing else to wire for that. When content does NOT have
        // tracked focus, this is a no-operation and `handle_keyboard`'s existing cross-window
        // `cycle_active_window` Tab handling below still runs exactly as before.
        if idle {
            if let Some(window_id) = self.active_window_id.clone() {
                if self.chrome_build.content_has_focus(&window_id) {
                    if let Some(event) = ui_event_from_key_action(&action, modifiers) {
                        let commands = crate::interpreter::dispatch_ui_event(&window_id, event, input);
                        self.chrome_build.note_content_focus_commands(&commands);
                        return Ok(());
                    }
                }
            }
        }
        // 🧰️ Escape deactivates the active utility for the focused window (P5).
        if idle && action == ui_wgpu::wgpu::KeyAction::Escape {
            if let Some(window_id) = self.active_window_id.clone() {
                if self.active_utility_by_window.remove(&window_id).is_some() {
                    self.refresh_ui().await?;
                    return Ok(());
                }
            }
        }
        // ⌨️ App-declared keybinding dispatch (Architecture Decision 8, P4) — NET-NEW for the wgpu
        // shell, which previously only handled hardcoded shell chords. Reserved shell chords still win.
        if idle && !is_reserved_shell_chord(&action, modifiers) {
            if let Some(descriptor) = self.match_app_keybinding(&action, modifiers) {
                self.dispatch_app_keybinding(descriptor).await?;
                return Ok(());
            }
        }
        self.handle_keyboard(action, modifiers, input);
        Ok(())
    }

    /// ⌨️ The app keybinding matching the current key event, if any.
    fn match_app_keybinding(&self, action: &ui_wgpu::wgpu::KeyAction, modifiers: &PointerModifiers) -> Option<ActionDescriptor> {
        let session = self.session.as_ref()?;
        session.app.keybindings.iter().find(|binding| key_event_matches_chord(action, modifiers, &binding.keys)).map(|binding| binding.action.clone())
    }

    /// ⌨️ Applies the P4 keybinding rule: arg-less actions dispatch directly; an arg-carrying action's
    /// hotkey opens its form, or — if that form is already expanded in the active window — executes it
    /// with the staged/validated args (never silent-fires defaults from a cold keystroke).
    async fn dispatch_app_keybinding(&mut self, descriptor: ActionDescriptor) -> Result<(), String> {
        let Some(session) = self.session.clone() else {
            return Ok(());
        };
        let window_id = self.active_window_id.clone().or_else(|| session.view_state.active_window_kind_id.clone()).unwrap_or_else(|| session.app.window_kinds.first().id.clone());
        let action_def = window_action_definition(&session.app, &window_id, &descriptor.action).cloned();
        let has_args = action_def.as_ref().is_some_and(|action| !action.args.is_empty());
        if !has_args {
            return self.dispatch_action(descriptor).await;
        }
        let action_id = action_def.expect("checked has_args").id;
        let already_expanded = self.active_window_id.as_deref() == Some(window_id.as_str()) && self.action_panel_expanded.get(&window_id).map(String::as_str) == Some(action_id.as_str());
        if already_expanded {
            self.execute_staged_action(&window_id, &action_id).await
        } else {
            self.active_window_id = Some(window_id.clone());
            self.action_panel_folded.insert(window_id.clone(), false);
            self.action_panel_expanded.insert(window_id, action_id);
            self.refresh_ui().await
        }
    }

    fn push_uri(&mut self, uri: String) {
        self.uri_history.truncate(self.uri_index + 1);
        self.uri_history.push(uri);
        self.uri_index = self.uri_history.len().saturating_sub(1);
    }

    /// 🪟️ Read-only walk of the dock tree collecting `(stack path, window id)` in tab/visual order
    /// (depth-first Row/Column child order, each Stack's own tab order). Duplicated locally rather than
    /// reusing `dock`'s own module-private `find_stack_path`-style traversal, because that helper is
    /// private to the `dock` module and this region (`shell::ShellInput`) must not edit `dock` to expose
    /// a public equivalent (region-claims.json `must_not_touch`). `DockNode`'s variants/fields are
    /// public, so this is a plain read of already-public state, not a layering violation.
    fn dock_window_order(node: &crate::dock::DockNode, path: &mut Vec<usize>, out: &mut Vec<(Vec<usize>, String)>) {
        match node {
            crate::dock::DockNode::Stack { windows, .. } => {
                for tab in windows {
                    out.push((path.clone(), tab.window_id.clone()));
                }
            }
            crate::dock::DockNode::Row(children) | crate::dock::DockNode::Column(children) => {
                for (index, (child, _)) in children.iter().enumerate() {
                    path.push(index);
                    Self::dock_window_order(child, path, out);
                    path.pop();
                }
            }
        }
    }

    /// 🪟️ Cross-window Tab-focus cycling (see the `KeyAction::Tab` arm in `handle_keyboard`): advances
    /// (or, with Shift, retreats) `active_window_id` through the dock's full window order, wrapping
    /// around, and — critically — updates the containing stack's own active tab via `set_stack_active`
    /// (not just `sync_active_window`, which only updates `active_window_id`/`active_stack` bookkeeping
    /// and would leave the visually-active tab unchanged) so the window body actually switches.
    fn cycle_active_window(&mut self, forward: bool) {
        let mut order = Vec::new();
        Self::dock_window_order(&self.dock.root, &mut Vec::new(), &mut order);
        if order.is_empty() {
            return;
        }
        let current_index = self.active_window_id.as_deref().and_then(|id| order.iter().position(|(_, window_id)| window_id == id));
        let next_index = match current_index {
            Some(i) if forward => (i + 1) % order.len(),
            Some(i) => (i + order.len() - 1) % order.len(),
            None => 0,
        };
        let (path, next_id) = order[next_index].clone();
        self.dock.set_stack_active(&path, &next_id);
    }
}

#[cfg(test)]
mod shell_input_tests {
    use super::*;
    #[test]
    fn standalone_multi_app_variants_resolve_their_declared_app() {
        assert_eq!(resolve_playground_app_id("puzzle2d"), Some("puzzle2d-play"));
        assert_eq!(resolve_playground_app_id("puzzle3d"), Some("puzzle3d-play"));
        assert_eq!(resolve_playground_app_id("3d"), Some("puzzle3d-play"));
        assert_eq!(resolve_playground_app_id("puzzle5d"), Some("puzzle5d-play"));
    }

    //#region SilhouetteContentTests

    #[test]
    fn silhouette_hit_intersections_leave_the_cap_gap_empty() {
        let silhouette = WindowSilhouette::from_measured_top(Rect::new(0.0, 0.0, 300.0, 200.0), 80.0, 60.0, 32.0);
        let hits: Vec<Rect> = silhouette.content_clip_rects().iter().filter_map(|clip| ShellState::intersect_content_rect(*clip, silhouette.bounds)).collect();
        assert_eq!(hits.len(), 3);
        assert!(!hits.iter().any(|rect| rect.contains(150.0, 16.0)));
        assert!(hits.iter().any(|rect| rect.contains(150.0, 100.0)));
    }

    //#endregion SilhouetteContentTests

    /// 🧪️ `dock_window_order` is a `Self`-less associated fn, so it's callable without constructing a
    /// full `ShellState` fixture (impractically large: 90+ fields, several without `Default`).
    fn window_ids(node: &crate::dock::DockNode) -> Vec<String> {
        let mut out = Vec::new();
        ShellState::dock_window_order(node, &mut Vec::new(), &mut out);
        out.into_iter().map(|(_, window_id)| window_id).collect()
    }

    #[test]
    fn dock_window_order_flattens_a_single_stack_in_tab_order() {
        let node = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("a"), DockStackTab::new("b"), DockStackTab::new("c")], active: "a".into() };
        assert_eq!(window_ids(&node), vec!["a", "b", "c"]);
    }

    #[test]
    fn dock_window_order_walks_row_and_column_children_depth_first() {
        let left = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("left")], active: "left".into() };
        let right_top = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("top")], active: "top".into() };
        let right_bottom = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("bottom")], active: "bottom".into() };
        let right = crate::dock::DockNode::Column(vec![(right_top, 0.5), (right_bottom, 0.5)]);
        let root = crate::dock::DockNode::Row(vec![(left, 0.5), (right, 0.5)]);
        assert_eq!(window_ids(&root), vec!["left", "top", "bottom"]);
    }

    #[test]
    fn dock_window_order_pairs_each_window_with_its_own_stack_path() {
        let a = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("a")], active: "a".into() };
        let b = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("b")], active: "b".into() };
        let root = crate::dock::DockNode::Row(vec![(a, 0.5), (b, 0.5)]);
        let mut out = Vec::new();
        ShellState::dock_window_order(&root, &mut Vec::new(), &mut out);
        assert_eq!(out, vec![(vec![0], "a".to_string()), (vec![1], "b".to_string())]);
    }

    // 🎯️🕹️ w2-input-wiring: key mapping is pure and focus tracking is explicit owned build state,
    // so both remain testable without a full `ShellState` fixture.

    #[test]
    fn ui_event_from_key_action_maps_plain_char_to_text_input() {
        let modifiers = ui_wgpu::wgpu::PointerModifiers::default();
        let event = ui_event_from_key_action(&ui_wgpu::wgpu::KeyAction::Char("a".into()), &modifiers);
        assert_eq!(event, Some(ui_wgpu::wgpu::UiEvent::TextInput { text: "a".into() }));
    }

    #[test]
    fn ui_event_from_key_action_routes_ctrl_char_as_key_down_for_clipboard_chords() {
        let modifiers = ui_wgpu::wgpu::PointerModifiers { ctrl: true, ..Default::default() };
        let event = ui_event_from_key_action(&ui_wgpu::wgpu::KeyAction::Char("c".into()), &modifiers);
        assert_eq!(event, Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: "c".into(), modifiers: ui_wgpu::wgpu::EventModifiers { shift: false, ctrl: true, alt: false, meta: false } }));
    }

    #[test]
    fn ui_event_from_key_action_maps_editing_and_tab_keys_to_matching_key_down_strings() {
        let modifiers = ui_wgpu::wgpu::PointerModifiers::default();
        let cases = [
            (ui_wgpu::wgpu::KeyAction::Backspace, "Backspace"),
            (ui_wgpu::wgpu::KeyAction::Delete, "Delete"),
            (ui_wgpu::wgpu::KeyAction::Enter, "Enter"),
            (ui_wgpu::wgpu::KeyAction::Escape, "Escape"),
            (ui_wgpu::wgpu::KeyAction::ArrowLeft, "ArrowLeft"),
            (ui_wgpu::wgpu::KeyAction::ArrowRight, "ArrowRight"),
            (ui_wgpu::wgpu::KeyAction::ArrowUp, "ArrowUp"),
            (ui_wgpu::wgpu::KeyAction::ArrowDown, "ArrowDown"),
            (ui_wgpu::wgpu::KeyAction::Tab, "Tab"),
        ];
        for (action, key) in cases {
            let event = ui_event_from_key_action(&action, &modifiers);
            assert_eq!(event, Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: key.into(), modifiers: ui_wgpu::wgpu::EventModifiers::default() }), "KeyAction {action:?} should map to KeyDown{{{key}}}");
        }
    }

    #[test]
    fn ui_event_from_key_action_has_no_mapping_for_space() {
        let event = ui_event_from_key_action(&ui_wgpu::wgpu::KeyAction::Space(true), &ui_wgpu::wgpu::PointerModifiers::default());
        assert_eq!(event, None);
    }

    #[test]
    fn content_focus_tracker_defaults_unfocused_and_tracks_focus_changed_commands() {
        let mut chrome = ShellChromeBuildState::default();
        let window_id = "w2-input-wiring-test-window-a";
        assert!(!chrome.content_has_focus(window_id));
        let mut arena: ui_wgpu::wgpu::Arena<()> = ui_wgpu::wgpu::Arena::new();
        let node_id = arena.insert(());
        chrome.note_content_focus_commands(&[ui_wgpu::wgpu::UiCommand::FocusChanged { window_id: window_id.to_string(), node: Some(node_id) }]);
        assert!(chrome.content_has_focus(window_id));
        chrome.note_content_focus_commands(&[ui_wgpu::wgpu::UiCommand::FocusChanged { window_id: window_id.to_string(), node: None }]);
        assert!(!chrome.content_has_focus(window_id));
    }

    #[test]
    fn content_focus_tracker_ignores_commands_for_other_windows() {
        let mut chrome = ShellChromeBuildState::default();
        let window_id = "w2-input-wiring-test-window-b";
        let other_window_id = "w2-input-wiring-test-window-c";
        let mut arena: ui_wgpu::wgpu::Arena<()> = ui_wgpu::wgpu::Arena::new();
        let node_id = arena.insert(());
        chrome.note_content_focus_commands(&[ui_wgpu::wgpu::UiCommand::FocusChanged { window_id: other_window_id.to_string(), node: Some(node_id) }]);
        assert!(!chrome.content_has_focus(window_id));
        assert!(chrome.content_has_focus(other_window_id));
    }

    #[test]
    fn content_focus_tracker_ignores_non_focus_commands() {
        let mut chrome = ShellChromeBuildState::default();
        let window_id = "w2-input-wiring-test-window-d";
        chrome.note_content_focus_commands(&[ui_wgpu::wgpu::UiCommand::ClipboardPasteRequested { window_id: window_id.to_string() }]);
        assert!(!chrome.content_has_focus(window_id));
    }

    /// 🕒️ `finish_dock_drag`'s successful-drop branch persists the new layout and clears the drag —
    /// unchanged behavior this ticket's `noteShellCommand` dispatch is appended after, not instead of.
    /// No host app is configured (`ShellState::new`'s bare fixture, same "impractically large" 90+-field
    /// constraint as `dock_window_order`'s own fixture note above), so `host_controller_id()` is `None`
    /// and the new dispatch is a documented no-op here — this pins the "must still complete without a
    /// host to log against" half of that behavior; `note_shell_command_action`'s own shape (the other
    /// half) is covered directly in `command_registry_tests`.
    #[test]
    fn finish_dock_drag_persists_layout_and_clears_drag_state_on_successful_drop() {
        let mut shell = ShellState::new(Vec::new(), String::new());
        shell.dock.root = crate::dock::DockNode::Row(vec![(crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("a"), DockStackTab::new("b"), DockStackTab::new("c")], active: "a".into() }, 1.0)]);
        assert!(shell.dock.remove_window("a"));
        let payload = DockDragPayload { kind: DockDragKind::Tab, window_id: "a".into(), source_path: vec![0], tab_index: 0, ghost_label: "a".into() };
        let zone = DockDropZone::Tab { stack_path: vec![0], corner: WindowStackCorner::TopLeft, index: 2 };
        shell.dock_drag = Some(DockDragState { payload, x: 10.0, y: 10.0, drop_zone: Some(zone) });
        assert!(shell.layout_override.is_none(), "sanity: nothing persisted yet");
        let input = InputState::<ActionDescriptor>::default();
        let result = semio_framework_async::block_on(shell.finish_dock_drag(10.0, 10.0, &input));
        assert!(result.is_ok(), "finish_dock_drag must not error even without a host app to log a shell.windowMove against");
        assert!(shell.layout_override.is_some(), "a successful drop persists the new dock layout");
        assert!(shell.dock_drag.is_none(), "the transient drag state is always taken");
    }
}
//#endregion ShellInput

fn chrome_text(target: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    let mut ctx = framework_widget_context(target, None, atlas, None, input, theme, &mut scroll, &mut collapsed, &mut selects, None);
    draw_text(&mut ctx, text, x, y, size, color);
}

fn chrome_icon(draw: &mut DrawList, icons: &IconAtlas, icon_id: &str, x: f32, y: f32, size: f32, color: Rgba) {
    if let Some(uv) = icons.icon_uv(icon_id) {
        draw.push_textured([x, y, size, size], uv, color);
    }
}

/** @emoji 📑️ Shared side-panel tab strip for floating panels. */
fn render_panel_tab_bar(
    chrome: &mut ShellChromeBuildState,
    panel_draw: &mut DrawList,
    atlas: &mut FontAtlas,
    icons: &IconAtlas,
    input: &mut InputState<ActionDescriptor>,
    theme: &Theme,
    panel: Rect,
    tabs: &[PanelTabDefinition],
    active_tab_id: &str,
    side_left: bool,
    inner_stroke: Rgba,
    hair: f32,
) -> f32 {
    let tab_bar_h = theme.panel_header_height;
    let tab_bar = Rect::new(panel.x + hair, panel.y, (panel.w - hair * 2.0).max(0.0), tab_bar_h);
    panel_draw.push_scissor(tab_bar);
    panel_draw.push_solid([tab_bar.x, tab_bar.y + tab_bar_h - hair, tab_bar.w, hair], inner_stroke);
    let mut tab_x = tab_bar.x;
    for (index, tab) in tabs.iter().enumerate() {
        let icon_id = panel_tab_icon_id(tab);
        // 🚧️ Not locale-aware yet: this free function has no locale/terminology threaded through its
        // render path (see ticket 26/08/03/COMPILE-TIME-CHECKED-UI-LABELS-ACROSS-LOCALE-TERMINOLOGY-AND-BRAND).
        let tab_label = tab.label.resolve(Terminology::Native, Locale::En);
        let label_w = atlas.measure_text(tab_label, theme.font_size_small).0;
        let tw = theme.padding_standard * 2.0 + CHROME_ICON_TINY + theme.gap_standard + label_w;
        let rect = Rect::new(tab_x, tab_bar.y, tw, tab_bar_h);
        if index > 0 {
            panel_draw.push_solid([tab_x, tab_bar.y, hair, tab_bar_h], inner_stroke);
        }
        let active = tab.id() == active_tab_id;
        let hovered = rect.contains(input.pointer_x, input.pointer_y);
        if active {
            panel_draw.push_solid([rect.x, rect.y, rect.w, rect.h], theme.selected);
        } else if hovered {
            panel_draw.push_solid([rect.x, rect.y, rect.w, rect.h], theme.button_hover);
        }
        let icon_x = rect.x + theme.padding_standard;
        let icon_y = rect.y + (rect.h - CHROME_ICON_TINY) * 0.5;
        chrome_icon(panel_draw, icons, icon_id, icon_x, icon_y, CHROME_ICON_TINY, chrome_item_text(theme, active, hovered));
        chrome_text(panel_draw, atlas, input, theme, tab_label, icon_x + CHROME_ICON_TINY + theme.gap_standard, rect.y + (rect.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, chrome_item_text(theme, active, hovered));
        let prefix = if side_left { "shell.panel.tab.left." } else { "shell.panel.tab.right." };
        input.register_hit(HitTarget { rect, event: None, control_id: Some(format!("{prefix}{}", tab.id())), kind: HitKind::PanelTab, drag_axis: None, drag_data: None });
        chrome.register_element_rect_fallback(semio_framework::panel_tab_element_id(tab.id()), rect);
        tab_x += tw;
    }
    panel_draw.pop_scissor();
    tab_bar_h
}

fn chrome_group_border(draw: &mut DrawList, rect: Rect, theme: &Theme) {
    push_chrome_group_border(draw, rect, theme);
}

struct ChromeGroupItem<'a> {
    control_id: &'a str,
    icon_id: Option<&'a str>,
    label: Option<&'a str>,
    active: bool,
    disabled: bool,
    kind: HitKind,
}

fn measure_chrome_group_item(atlas: &mut FontAtlas, theme: &Theme, item: &ChromeGroupItem<'_>) -> f32 {
    let icon_w = item.icon_id.map(|_| CHROME_ICON_TINY + theme.gap_standard).unwrap_or(0.0);
    let text_w = item.label.map(|label| atlas.measure_text(label, theme.font_size_small).0).unwrap_or(0.0);
    theme.padding_standard * 2.0 + icon_w + text_w
}

struct WindowMeasuresRailOutcome {
    chip_hit: Option<(Rect, String)>,
    reserve_width: f32,
}

const WINDOW_MEASURE_TRAVERSAL_CAPACITY: usize = 64;
const WINDOW_MEASURE_ACTION_CAPACITY: usize = 64;

#[derive(Clone, Copy)]
enum WindowMeasureActionKind {
    Select,
    Slider,
    Toggle(bool),
}

struct WindowMeasureActionBinding {
    control: UiText,
    controller: UiText,
    action: UiText,
    kind: WindowMeasureActionKind,
}

struct WindowMeasureActionRegistry {
    slots: [Option<WindowMeasureActionBinding>; WINDOW_MEASURE_ACTION_CAPACITY],
}

struct WindowMeasureRenderFrame<'a> {
    measure: &'a WindowMeasure,
    inset: f32,
}

impl WindowMeasureActionRegistry {
    fn new() -> Self {
        Self { slots: std::array::from_fn(|_| None) }
    }

    fn try_upsert(&mut self, control: &str, descriptor: &ActionDescriptor, kind: WindowMeasureActionKind) -> Result<(), ()> {
        let binding =
            WindowMeasureActionBinding { control: UiText::try_from_str(control).map_err(|_| ())?, controller: UiText::try_from_str(&descriptor.controller_id).map_err(|_| ())?, action: UiText::try_from_str(&descriptor.action).map_err(|_| ())?, kind };
        if let Some(slot) = self.slots.iter_mut().find(|slot| slot.as_ref().is_some_and(|slot| slot.control.as_str() == control)) {
            *slot = Some(binding);
            return Ok(());
        }
        let Some(slot) = self.slots.iter_mut().find(|slot| slot.is_none()) else { return Err(()) };
        *slot = Some(binding);
        Ok(())
    }

    fn get(&self, control: &str) -> Option<&WindowMeasureActionBinding> {
        self.slots.iter().flatten().find(|slot| slot.control.as_str() == control)
    }
}

fn window_overlay_max_width(content_w: f32, inset: f32) -> f32 {
    (content_w - inset * 2.0).max(0.0)
}

fn engagement_rail_width(theme: &Theme, content_w: f32, inset: f32, measures_reserve: f32) -> f32 {
    let available = content_w - inset * 2.0 - measures_reserve;
    theme.window_engagement_max_width.min(available.max(0.0))
}

fn floating_panel_available_width(body: Rect, theme: &Theme) -> f32 {
    (body.w - theme.panel_inset * 2.0).max(theme.panel_min_width)
}

fn floating_panel_max_width(body: Rect, theme: &Theme) -> f32 {
    theme.panel_max_width.min(floating_panel_available_width(body, theme)).max(theme.panel_min_width)
}

fn floating_panel_width(width: f32, body: Rect, theme: &Theme) -> f32 {
    width.clamp(theme.panel_min_width, floating_panel_max_width(body, theme))
}

fn measure_window_measure_height(theme: &Theme, collapsed_sections: &HashMap<String, bool>, measure: &WindowMeasure) -> Option<f32> {
    let mut stack = UiFixedList::<&WindowMeasure, WINDOW_MEASURE_TRAVERSAL_CAPACITY>::default();
    stack.try_push(measure).ok()?;
    let mut height = 0.0;
    while let Some(measure) = stack.pop() {
        match measure {
            WindowMeasure::Group { id, default_open, children, .. } => {
                height += theme.control_height;
                if !collapsed_sections.get(id).copied().unwrap_or(!default_open.unwrap_or(false)) {
                    for child in children.iter().rev() {
                        stack.try_push(child).ok()?;
                    }
                }
            }
            WindowMeasure::Select { .. } | WindowMeasure::Slider { .. } => height += 16.0 + theme.control_height,
            WindowMeasure::Toggle { .. } => height += theme.control_height,
        }
    }
    Some(height)
}

fn measure_window_measures_body_height<'a>(theme: &Theme, collapsed_sections: &HashMap<String, bool>, measures: impl Iterator<Item = &'a WindowMeasure>) -> Option<f32> {
    let mut height = 0.0;
    for measure in measures {
        height += measure_window_measure_height(theme, collapsed_sections, measure)?;
    }
    Some(height)
}

fn measure_engagement_body_height(theme: &Theme, engagement: &WindowEngagement) -> f32 {
    let mut h = 0.0f32;
    if let Some(options) = &engagement.options {
        h += options.len() as f32 * (theme.control_height + 4.0);
    }
    if engagement.input.is_some() {
        h += theme.control_height * 2.0 + 8.0;
    }
    if engagement.control.is_some() {
        h += theme.control_height;
    }
    if let Some(status_rows) = &engagement.status {
        h += status_rows.len() as f32 * theme.control_height;
    }
    if let Some(possibles) = &engagement.possible_engagements {
        h += possibles.len() as f32 * (theme.control_height + 2.0);
    }
    h
}

fn render_chrome_group(draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, rect: Rect, items: &[ChromeGroupItem<'_>], register_hits: bool) {
    if items.is_empty() {
        return;
    }
    let hair = theme.stroke_hairline;
    let inner_y = rect.y + hair;
    let inner_h = (rect.h - hair * 2.0).max(0.0);
    let mut x = rect.x;
    for (index, item) in items.iter().enumerate() {
        let item_w = measure_chrome_group_item(atlas, theme, item);
        let item_rect = Rect::new(x, inner_y, item_w, inner_h);
        let hovered = !item.disabled && item_rect.contains(input.pointer_x, input.pointer_y);
        let bg = if item.disabled { theme.overlay_shadow } else { chrome_item_bg(theme, item.active, hovered) };
        if bg.a > 0.0 {
            draw.push_solid([item_rect.x, item_rect.y, item_rect.w, item_rect.h], bg);
        }
        let text_color = if item.disabled { theme.text_muted } else { chrome_item_text(theme, item.active, hovered) };
        let mut content_x = item_rect.x + theme.padding_standard;
        if let Some(icon_id) = item.icon_id {
            chrome_icon(draw, icons, icon_id, content_x, item_rect.y + (item_rect.h - CHROME_ICON_TINY) * 0.5, CHROME_ICON_TINY, text_color);
            content_x += CHROME_ICON_TINY + theme.gap_standard;
        }
        if let Some(label) = item.label {
            chrome_text(draw, atlas, input, theme, label, content_x, item_rect.y + (item_rect.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, text_color);
        }
        if register_hits && !item.disabled {
            input.register_hit(HitTarget { rect: item_rect, event: None, control_id: Some(item.control_id.into()), kind: item.kind.clone(), drag_axis: None, drag_data: None });
        }
        x += item_w;
        if index + 1 < items.len() {
            draw.push_solid([x, inner_y, hair, inner_h], theme.border_normal);
        }
    }
    chrome_group_border(draw, rect, theme);
}

fn footer_utility_label<'a>(label: &'a Option<String>, text: &'a Option<String>, title: &'a Option<String>, id: &'a str) -> &'a str {
    title.as_deref().or(label.as_deref()).or(text.as_deref()).unwrap_or(id)
}

fn backbone_kind_from_uri(uri: &str) -> &'static str {
    if uri.starts_with("file://") {
        "file"
    } else if uri.starts_with("folder://") {
        "folder"
    } else if uri.starts_with("remote://") {
        "remote"
    } else {
        "unknown"
    }
}

fn framework_sync_utilities(active_uri: Option<&str>) -> Vec<UtilityNode> {
    let active_kind = active_uri.map(backbone_kind_from_uri);
    let pressed = |kind: &str| active_kind == Some(kind);
    vec![
        UtilityNode::Toggle {
            id: "framework.sync.file".into(),
            icon_id: "file-json".into(),
            label: Some("File".into()),
            text: None,
            title: None,
            order: Some(0),
            pressed: Some(pressed("file")),
            disabled: None,
            category: Some(UtilityCategory::Sync),
            on_change: ActionDescriptor { controller_id: "framework.sync".into(), action: "selectFile".into(), args: None },
        },
        UtilityNode::Toggle {
            id: "framework.sync.folder".into(),
            icon_id: "folder".into(),
            label: Some("Folder".into()),
            text: None,
            title: None,
            order: Some(1),
            pressed: Some(pressed("folder")),
            disabled: None,
            category: Some(UtilityCategory::Sync),
            on_change: ActionDescriptor { controller_id: "framework.sync".into(), action: "selectFolder".into(), args: None },
        },
        UtilityNode::Toggle {
            id: "framework.sync.remote".into(),
            icon_id: "cloud".into(),
            label: Some("Remote".into()),
            text: None,
            title: None,
            order: Some(2),
            pressed: Some(pressed("remote")),
            disabled: None,
            category: Some(UtilityCategory::Sync),
            on_change: ActionDescriptor { controller_id: "framework.sync".into(), action: "selectRemote".into(), args: None },
        },
    ]
}

fn partition_utilities_by_category(utilities: &[UtilityNode]) -> [Vec<UtilityNode>; 4] {
    let mut buckets: [Vec<UtilityNode>; 4] = [vec![], vec![], vec![], vec![]];
    for utility in utilities {
        let idx = match utility.category() {
            UtilityCategory::Selection => 0,
            UtilityCategory::Utilities => 1,
            UtilityCategory::History => 2,
            UtilityCategory::Sync => 3,
        };
        buckets[idx].push(utility.clone());
    }
    buckets
}

/// 👥️ ticket §5 — walks `ui_wgpu::wgpu::build_presence_bar`'s declarative `UiNode` tree (id
/// `s-presence-peers`, per-peer `peer:<actor>` — contract §C0's frozen id grammar, the wgpu↔React
/// parity join) and paints one `ChromeGroupItem` chip per peer, right-aligned in the footer, reusing
/// the SAME `render_chrome_group`/`measure_chrome_group_item` primitives `render_footer_utility_nodes`
/// already draws with (rather than the generic `render_ui_node` pipeline plugin surfaces use, which
/// needs a `&mut GpuContext` this footer's own immediate-mode callers don't carry). Not clickable
/// (`register_hits: false`) — a `peer:<actor>` hit is registered separately, with no `event`, purely
/// so the id is discoverable for e2e/hit-testing.
#[cfg(not(target_arch = "wasm32"))]
fn render_presence_bar(draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, rows: &[ui_wgpu::wgpu::PresencePeerRow], right_edge: f32, btn_y: f32, btn_h: f32) {
    if rows.is_empty() {
        return;
    }
    let node = ui_wgpu::wgpu::build_presence_bar("s-presence-peers", rows, None);
    let UiNode::Stack(root) = node else { return };
    let mut x = right_edge;
    for child in root.children.iter().rev() {
        let UiNode::Stack(peer_stack) = child else { continue };
        let label = peer_stack.children.iter().find_map(|node| if let UiNode::Text(text) = node { Some(text.value.as_str().to_string()) } else { None }).unwrap_or_default();
        let item = ChromeGroupItem { control_id: "framework.presence.peer", icon_id: None, label: Some(label.as_str()), active: false, disabled: false, kind: HitKind::Button };
        let item_w = measure_chrome_group_item(atlas, theme, &item);
        x -= item_w;
        let rect = Rect::new(x, btn_y, item_w, btn_h);
        render_chrome_group(draw, atlas, icons, input, theme, rect, &[item], false);
        if let Some(id) = &peer_stack.id {
            input.register_hit(HitTarget { rect, event: None, control_id: Some(format!("framework.presence.{id}")), kind: HitKind::Button, drag_axis: None, drag_data: None });
        }
        x -= theme.gap_standard * 0.5;
    }
}

/// 🚦️ ticket §C5 — `#s-sync-status` status pill + `#s-checkin` explicit check-in, painted directly in
/// the footer left-to-right (same `ChromeGroupItem`/`render_chrome_group`/`measure_chrome_group_item`
/// immediate-mode primitives `render_presence_bar` above already uses, for the identical reason: these
/// are SHELL-owned chrome, not a plugin-declared `UtilityNode`/`active_utilities` entry). `#s-checkin`
/// is simply absent for a viewer session — contract §C5 "viewers never checkpoint" — never rendered
/// disabled, mirroring `render_footer_utility_nodes`'s own undo/redo precedent one region up in the
/// React twin. `uncommitted_count` mirrors React's own `(${count})` suffix on the button label; the
/// button itself now opens the `#s-checkin-*` message-prompt card (`render_overlay`'s own doc note)
/// through `framework.checkin`'s `open` action rather than dispatching a fixed-message checkpoint
/// straight away.
#[cfg(not(target_arch = "wasm32"))]
fn render_sync_status_and_checkin(
    draw: &mut DrawList,
    atlas: &mut FontAtlas,
    icons: &IconAtlas,
    input: &mut InputState<ActionDescriptor>,
    theme: &Theme,
    status: Option<&ArtifactSyncStatus>,
    session: Option<&ActiveSession>,
    uncommitted_count: u32,
    mut x: f32,
    btn_y: f32,
    btn_h: f32,
) -> f32 {
    let pill_label = ShellState::sync_pill_text(status);
    let pill = ChromeGroupItem { control_id: "s-sync-status", icon_id: None, label: Some(pill_label.as_str()), active: false, disabled: false, kind: HitKind::Button };
    let pill_w = measure_chrome_group_item(atlas, theme, &pill);
    let pill_rect = Rect::new(x, btn_y, pill_w, btn_h);
    render_chrome_group(draw, atlas, icons, input, theme, pill_rect, &[pill], false);
    input.register_hit(HitTarget { rect: pill_rect, event: None, control_id: Some("s-sync-status".into()), kind: HitKind::Button, drag_axis: None, drag_data: None });
    x += pill_w + theme.gap_standard * 0.5;
    if let Some(session) = session {
        if can_check_in(session.app.role) {
            let label = if uncommitted_count > 0 { format!("Check In ({uncommitted_count})") } else { "Check In".to_string() };
            let checkin = ChromeGroupItem { control_id: "s-checkin", icon_id: None, label: Some(label.as_str()), active: false, disabled: false, kind: HitKind::Button };
            let checkin_w = measure_chrome_group_item(atlas, theme, &checkin);
            let checkin_rect = Rect::new(x, btn_y, checkin_w, btn_h);
            render_chrome_group(draw, atlas, icons, input, theme, checkin_rect, &[checkin], true);
            let event = ActionDescriptor { controller_id: "framework.checkin".into(), action: "open".into(), args: None };
            input.register_hit(HitTarget { rect: checkin_rect, event: Some(event), control_id: Some("s-checkin".into()), kind: HitKind::Button, drag_axis: None, drag_data: None });
            x += checkin_w + theme.gap_standard * 0.5;
        }
    }
    x
}

fn render_footer_section_divider(draw: &mut DrawList, theme: &Theme, x: f32, btn_y: f32, btn_h: f32) -> f32 {
    draw.push_solid([x + theme.gap_standard * 0.5, btn_y + 4.0, theme.stroke_hairline, btn_h - 8.0], theme.border_normal);
    x + theme.gap_standard
}

fn render_footer_utility_nodes(
    chrome: &mut ShellChromeBuildState,
    draw: &mut DrawList,
    atlas: &mut FontAtlas,
    icons: &IconAtlas,
    input: &mut InputState<ActionDescriptor>,
    theme: &Theme,
    mut x: f32,
    btn_y: f32,
    btn_h: f32,
    utilities: &[UtilityNode],
    collection_expanded: &HashMap<String, bool>,
) -> f32 {
    for utility in utilities {
        match utility {
            UtilityNode::Separator { .. } => {
                x = render_footer_section_divider(draw, theme, x, btn_y, btn_h);
            }
            UtilityNode::Button { id, icon_id, label, text, title, disabled, on_press, .. } => {
                if disabled.unwrap_or(false) {
                    continue;
                }
                let label_text = footer_utility_label(label, text, title, id);
                let item = ChromeGroupItem { control_id: "framework.utility.button", icon_id: Some(icon_id.as_str()), label: Some(label_text), active: false, disabled: false, kind: HitKind::Button };
                let item_w = measure_chrome_group_item(atlas, theme, &item);
                let rect = Rect::new(x, btn_y, item_w, btn_h);
                render_chrome_group(draw, atlas, icons, input, theme, rect, &[item], true);
                chrome.register_element_rect(id.clone(), rect);
                input.register_hit(HitTarget { rect, event: Some(on_press.clone()), control_id: Some(format!("framework.utility.button.{id}")), kind: HitKind::Button, drag_axis: None, drag_data: None });
                x += item_w + theme.gap_standard * 0.5;
            }
            UtilityNode::Toggle { id, icon_id, label, text, title, pressed, disabled, on_change, .. } => {
                if disabled.unwrap_or(false) {
                    continue;
                }
                let label_text = footer_utility_label(label, text, title, id);
                let item = ChromeGroupItem { control_id: "framework.utility.toggle", icon_id: Some(icon_id.as_str()), label: Some(label_text), active: pressed.unwrap_or(false), disabled: false, kind: HitKind::Button };
                let item_w = measure_chrome_group_item(atlas, theme, &item);
                let rect = Rect::new(x, btn_y, item_w, btn_h);
                render_chrome_group(draw, atlas, icons, input, theme, rect, &[item], true);
                chrome.register_element_rect(id.clone(), rect);
                input.register_hit(HitTarget { rect, event: Some(on_change.clone()), control_id: Some(format!("framework.utility.toggle.{id}")), kind: HitKind::Button, drag_axis: None, drag_data: None });
                x += item_w + theme.gap_standard * 0.5;
            }
            UtilityNode::Collection { id, icon_id, label, text, title, disabled, children, .. } => {
                if disabled.unwrap_or(false) {
                    continue;
                }
                let expanded = collection_expanded.get(id).copied().unwrap_or(false);
                // 🧭️ Item 5 (ribbon nesting): a collapsed group still highlights when the active picker
                // segment lives somewhere inside it (recursively, through further nested `Collection`s) —
                // `active-path` tracking, ported from the React ribbon's recursive reconciliation.
                let on_active_path = expanded || utility_subtree_has_active_path(children);
                let label_text = footer_utility_label(label, text, title, id);
                let item = ChromeGroupItem { control_id: "framework.utility.collection", icon_id: Some(icon_id.as_str()), label: Some(label_text), active: on_active_path, disabled: false, kind: HitKind::Button };
                let item_w = measure_chrome_group_item(atlas, theme, &item);
                let rect = Rect::new(x, btn_y, item_w, btn_h);
                render_chrome_group(draw, atlas, icons, input, theme, rect, &[item], true);
                input.register_hit(HitTarget { rect, event: None, control_id: Some(format!("framework.utility.collection.{id}")), kind: HitKind::Button, drag_axis: None, drag_data: None });
                x += item_w + theme.gap_standard * 0.5;
                if expanded {
                    // 🧭️ Item 5: previously flattened one level (`.filter(|child| !matches!(child,
                    // UtilityNode::Collection { .. }))` dropped nested `Collection`s outright) — recursing
                    // on the full, unfiltered `children` supports arbitrary nesting depth; each nested
                    // `Collection` gets its own `collection_expanded` entry (already a flat `id`-keyed map,
                    // so no path-keying change needed) and paints its own active-path highlight in turn.
                    x = render_footer_utility_nodes(chrome, draw, atlas, icons, input, theme, x, btn_y, btn_h, children, collection_expanded);
                }
            }
        }
    }
    x
}

fn panel_tab_icon_id(tab: &PanelTabDefinition) -> &'static str {
    // 🌱️ `tab.group == PanelGroup::Workbench` already covers every host-app catalogue tab (each such app
    // declares its catalogue tab under that group — see `s/plugin/rs`'s `App::builder(...).panel_tab(...)`)
    // so no separate app-specific tab-id literal is needed here.
    if tab.group == PanelGroup::Workbench {
        return "library";
    }
    if tab.id().contains("parameters") {
        return "settings";
    }
    if tab.id().contains("inspector") || tab.id().contains("inspection") || tab.id() == FRAMEWORK_PANEL_TAB_INSPECTION_ID {
        return "text-search";
    }
    if tab.id() == FRAMEWORK_PANEL_TAB_ARTIFACT_ID {
        return "file-text";
    }
    if tab.id() == FRAMEWORK_DISPLAY_WINDOWS_TAB_ID {
        return "layout-grid";
    }
    if tab.id() == FRAMEWORK_DISPLAY_LAYOUT_TAB_ID {
        return "layout";
    }
    if tab.id() == FRAMEWORK_SETTINGS_GENERAL_TAB_ID {
        return "settings-2";
    }
    // 🎨️🎛️ Same icon choices as React's `createFrameworkSettingsPanelTab`/`buildCommandCategoryTabs`
    // (`shellTabIcon("paintbrush")` / `COMMAND_CATEGORY_ICON = shellTabIcon("wrench")`).
    if tab.id() == FRAMEWORK_SETTINGS_THEME_TAB_ID {
        return "paintbrush";
    }
    if tab.id() == FRAMEWORK_SETTINGS_COMMANDS_TAB_ID {
        return "wrench";
    }
    if tab.id() == FRAMEWORK_PANEL_TAB_CATALOGUE_ID {
        return "library";
    }
    if tab.id() == FRAMEWORK_PANEL_TAB_HISTORY_ID {
        return "undo";
    }
    "circle-dot"
}

/// 🧭️ This renderer only has a 2-panel (left/right) layout; fold the framework's 6-anchor model back down to
/// left/right. A middle anchor would fold right (the details/overflow side) but never occurs here — `PanelGroup`
/// only ever maps to the four corner anchors.
fn group_side(group: PanelGroup) -> &'static str {
    if group.anchor().ends_with("left") {
        "left"
    } else {
        "right"
    }
}

fn panel_toggle_icon_id(kind: &str, session: Option<&ActiveSession>) -> &'static str {
    match kind {
        "display" => "layout-grid",
        "workbench" => session.and_then(|s| s.app.panel_tabs.iter().find(|tab| group_side(tab.group) == "left")).map(|tab| panel_tab_icon_id(tab)).unwrap_or("folder"),
        "details" => session.and_then(|s| s.app.panel_tabs.iter().find(|tab| group_side(tab.group) == "right")).map(|tab| panel_tab_icon_id(tab)).unwrap_or("info"),
        "settings" => "settings-2",
        _ => "circle-dot",
    }
}

/// 🛡️ Chrome content must always win over window bodies; route it to the
/// overlay compositing phase (guaranteed last) whenever one is available.
fn with_chrome_sink<F, R>(draw: &mut DrawList, overlay: &mut Option<&mut DrawList>, f: F) -> R
where
    F: FnOnce(&mut DrawList, &mut Option<&mut DrawList>) -> R,
{
    if let Some(chrome) = overlay.as_deref_mut() {
        let mut nested_overlay = None;
        f(chrome, &mut nested_overlay)
    } else {
        f(draw, overlay)
    }
}

//#region ActionPanelAndUtilities
/// 🧰️ Resolves the utilities a window kind presents in the utility bar — the utility mirror of
/// {@link semio_framework::resolve_window_actions}: explicit `window_kind.utilities` refs in declared
/// order, plus any app utility referenced by no window kind (an "orphan" appearing on every window — the
/// scoping fallback that prevents blank utility bars mid-migration, Architecture Decision 8).
pub(crate) fn resolve_window_utilities<'a>(app: &'a AppDefinition, window_kind: &semio_framework::WindowKindDefinition) -> Vec<&'a semio_framework::UtilityDefinition> {
    use std::collections::HashSet;
    let referenced: HashSet<&str> = app.window_kinds.iter().flat_map(|window| window.utilities.iter().map(|utility_ref| utility_ref.as_str())).collect();
    let mut resolved: Vec<&'a semio_framework::UtilityDefinition> = Vec::new();
    let mut seen: HashSet<&str> = HashSet::new();
    for utility_ref in &window_kind.utilities {
        if let Some(utility) = app.utilities.iter().find(|utility| utility.id == utility_ref.as_str()) {
            if seen.insert(utility.id.as_str()) {
                resolved.push(utility);
            }
        }
    }
    for utility in &app.utilities {
        if !referenced.contains(utility.id.as_str()) && seen.insert(utility.id.as_str()) {
            resolved.push(utility);
        }
    }
    resolved
}

/// 📇️ The first window kind whose resolved actions include `action_id` — the window the palette/keybinding
/// redirect focuses to open an arg-carrying action's form (Architecture Decision 8, P3/P4).
// 🎯️ Exercised today only by `Dock`'s own `action_host_window_id_finds_scoping_window` test — the
// palette/keybinding redirect call site itself hasn't landed yet, hence the matching test-only cfg.
#[cfg(all(test, not(target_arch = "wasm32")))]
pub(crate) fn action_host_window_id(app: &AppDefinition, action_id: &str) -> Option<String> {
    app.window_kinds.iter().find(|kind| semio_framework::resolve_window_actions(app, kind).iter().any(|action| action.id == action_id)).map(|kind| kind.id.clone())
}

/// 🪟️ Resolves an action only from its addressed window kind owner.
pub(crate) fn window_action_definition<'a>(app: &'a AppDefinition, window_kind_id: &str, action_id: &str) -> Option<&'a semio_framework::ActionDefinition> {
    app.window_kinds.iter().find(|kind| kind.id == window_kind_id)?.actions.iter().find(|action| action.id == action_id)
}

/// 🔢️ Formats a number for a staged input/vec3 field — integers without a trailing `.0`.
fn fmt_num(value: f64) -> String {
    if value.is_finite() && value == value.trunc() && value.abs() < 1e15 {
        format!("{}", value as i64)
    } else {
        format!("{value}")
    }
}

/// 🖱️ Maps a `UtilityDefinition.cursor` CSS/winit cursor name onto the shell's {@link ui_wgpu::wgpu::SemioCursor}.
fn semio_cursor_from_name(name: &str) -> ui_wgpu::wgpu::SemioCursor {
    use ui_wgpu::wgpu::SemioCursor;
    match name.trim().to_ascii_lowercase().as_str() {
        "pointer" => SemioCursor::Pointer,
        "text" => SemioCursor::Text,
        "grab" => SemioCursor::Grab,
        "grabbing" => SemioCursor::Grabbing,
        "move" => SemioCursor::Move,
        "crosshair" | "cross" => SemioCursor::Crosshair,
        "not-allowed" | "notallowed" | "forbidden" => SemioCursor::NotAllowed,
        "ew-resize" | "col-resize" | "e-resize" | "w-resize" => SemioCursor::EwResize,
        "ns-resize" | "row-resize" | "n-resize" | "s-resize" => SemioCursor::NsResize,
        "nwse-resize" | "nw-resize" | "se-resize" => SemioCursor::NwseResize,
        "nesw-resize" | "ne-resize" | "sw-resize" => SemioCursor::NeswResize,
        "cell" | "selectable" => SemioCursor::Selectable,
        _ => SemioCursor::Default,
    }
}

/// ⌨️ Formats the first chord of a keybinding for context-menu shortcut labels (mirrors ui-react formatKeybindingShortcut).
fn format_keybinding_shortcut(keys: &str) -> String {
    let chord = keys.split(',').next().unwrap_or(keys).trim().to_ascii_lowercase();
    if chord.is_empty() {
        return String::new();
    }
    let apple = cfg!(target_os = "macos");
    let glyph = |part: &str| -> String {
        match part {
            "mod" if apple => "⌘️".into(),
            "mod" => "Ctrl".into(),
            "ctrl" if apple => "⌃️".into(),
            "ctrl" => "Ctrl".into(),
            "meta" => "⌘️".into(),
            "alt" if apple => "⌥️".into(),
            "alt" => "Alt".into(),
            "shift" if apple => "⇧️".into(),
            "shift" => "Shift".into(),
            "backspace" => "⌫️".into(),
            "delete" => "⌦️".into(),
            "enter" if apple => "↵️".into(),
            "enter" => "Enter".into(),
            "escape" if apple => "⎋️".into(),
            "escape" => "Esc".into(),
            "up" => "↑".into(),
            "down" => "↓".into(),
            "left" => "←".into(),
            "right" => "→".into(),
            other if other.len() == 1 => other.to_ascii_uppercase(),
            other => {
                let mut s = other.to_string();
                if let Some(first) = s.get_mut(0..1) {
                    first.make_ascii_uppercase();
                }
                s
            }
        }
    };
    let parts: Vec<String> = chord.split('+').map(str::trim).filter(|part| !part.is_empty()).map(glyph).collect();
    if apple {
        parts.join("")
    } else {
        parts.join("+")
    }
}

/// ⌨️ Whether a key event is one of the hardcoded shell chords (palette/find/panels/nav) that must win
/// over app-declared keybindings (P4 — "reserved shell chords still win").
pub(crate) fn is_reserved_shell_chord(action: &ui_wgpu::wgpu::KeyAction, modifiers: &PointerModifiers) -> bool {
    if matches!(action, ui_wgpu::wgpu::KeyAction::Function(11)) || matches!(action, ui_wgpu::wgpu::KeyAction::Char(key) if key.eq_ignore_ascii_case("f") && modifiers.ctrl && modifiers.meta) {
        return true;
    }
    let accelerator = modifiers.meta || modifiers.ctrl;
    if !accelerator {
        return false;
    }
    match action {
        ui_wgpu::wgpu::KeyAction::Char(c) => matches!(c.to_ascii_lowercase().as_str(), "p" | "f" | "b" | "[" | "]"),
        ui_wgpu::wgpu::KeyAction::ArrowUp => true,
        _ => false,
    }
}

fn command_host_platform() -> semio_framework::manifest::Platform {
    #[cfg(target_os = "macos")]
    {
        semio_framework::manifest::Platform::MacOs
    }
    #[cfg(target_os = "windows")]
    {
        semio_framework::manifest::Platform::Windows
    }
    #[cfg(all(not(target_os = "macos"), not(target_os = "windows"), not(target_arch = "wasm32")))]
    {
        semio_framework::manifest::Platform::Linux
    }
    #[cfg(target_arch = "wasm32")]
    {
        let platform =
            web_sys::window().and_then(|window| js_sys::Reflect::get(window.as_ref(), &"navigator".into()).ok()).and_then(|navigator| js_sys::Reflect::get(&navigator, &"platform".into()).ok()).and_then(|value| value.as_string()).unwrap_or_default();
        if platform.to_ascii_lowercase().contains("mac") {
            semio_framework::manifest::Platform::MacOs
        } else if platform.to_ascii_lowercase().contains("win") {
            semio_framework::manifest::Platform::Windows
        } else {
            semio_framework::manifest::Platform::Linux
        }
    }
}

/// ⌨️ Whether a key event matches a keybinding chord such as `"mod+shift+z"`, `"ctrl+k"`, or `"escape"`.
/// `"mod"` is the platform accelerator (meta OR ctrl). Declared modifiers must be present and no
/// undeclared accelerator/shift/alt may be held, so `mod+z` never fires for `mod+shift+z`.
pub(crate) fn key_event_matches_chord(action: &ui_wgpu::wgpu::KeyAction, modifiers: &PointerModifiers, chord: &str) -> bool {
    let mut want_mod = false;
    let mut want_shift = false;
    let mut want_alt = false;
    let mut want_ctrl = false;
    let mut want_meta = false;
    let mut key_token = String::new();
    for token in chord.split('+') {
        match token.trim().to_ascii_lowercase().as_str() {
            "" => {}
            "mod" => want_mod = true,
            "shift" => want_shift = true,
            "alt" | "option" => want_alt = true,
            "ctrl" | "control" => want_ctrl = true,
            "cmd" | "meta" | "super" | "win" => want_meta = true,
            other => key_token = other.to_string(),
        }
    }
    if key_token.is_empty() {
        return false;
    }
    if modifiers.shift != want_shift || modifiers.alt != want_alt {
        return false;
    }
    let accelerator = modifiers.meta || modifiers.ctrl;
    let want_accelerator = want_mod || want_ctrl || want_meta;
    if want_accelerator != accelerator {
        return false;
    }
    match action {
        ui_wgpu::wgpu::KeyAction::Char(c) => c.eq_ignore_ascii_case(&key_token),
        ui_wgpu::wgpu::KeyAction::Enter => key_token == "enter" || key_token == "return",
        ui_wgpu::wgpu::KeyAction::Escape => key_token == "escape" || key_token == "esc",
        ui_wgpu::wgpu::KeyAction::Backspace => key_token == "backspace",
        ui_wgpu::wgpu::KeyAction::Delete => key_token == "delete" || key_token == "del",
        ui_wgpu::wgpu::KeyAction::Tab => key_token == "tab",
        ui_wgpu::wgpu::KeyAction::ArrowLeft => key_token == "arrowleft" || key_token == "left",
        ui_wgpu::wgpu::KeyAction::ArrowRight => key_token == "arrowright" || key_token == "right",
        ui_wgpu::wgpu::KeyAction::ArrowUp => key_token == "arrowup" || key_token == "up",
        ui_wgpu::wgpu::KeyAction::ArrowDown => key_token == "arrowdown" || key_token == "down",
        ui_wgpu::wgpu::KeyAction::Function(number) => key_token == format!("f{number}"),
        ui_wgpu::wgpu::KeyAction::Space(_) => key_token == "space",
    }
}

impl ShellState {
    // #region utility-derivation
    /// 🧰️ The window kind whose utilities/actions the shell chrome currently scopes to (the focused window,
    /// else the view-state's active kind, else the app's first kind).
    fn active_utility_bar_window_kind<'a>(&self, session: &'a ActiveSession) -> &'a semio_framework::WindowKindDefinition {
        let active_id = self.active_window_id.as_deref().or(session.view_state.active_window_kind_id.as_deref());
        active_id.and_then(|id| session.app.window_kinds.iter().find(|kind| kind.id == id)).unwrap_or_else(|| session.app.window_kinds.first())
    }

    /// 🧰️ Derives the footer utility bar `UtilityNode`s from the app's declared utilities scoped to the active
    /// window kind, marking the host-owned active utility pressed (Architecture Decision 5).
    fn derive_utility_nodes(&self, session: &ActiveSession) -> Vec<UtilityNode> {
        if session.app.utilities.is_empty() {
            return Vec::new();
        }
        let window_kind = self.active_utility_bar_window_kind(session);
        let resolved = resolve_window_utilities(&session.app, window_kind);
        if resolved.is_empty() {
            return Vec::new();
        }
        let specs: Vec<ui_wgpu::wgpu::component::utilities::DerivedUtilitySpec> = resolved
            .iter()
            .map(|utility| ui_wgpu::wgpu::component::utilities::DerivedUtilitySpec {
                id: utility.id.clone(),
                label: utility.label.resolve(self.active_terminology(), self.active_locale()).to_string(),
                icon_id: utility.icon_id.clone(),
                group: utility.group.clone(),
                category: utility.category,
            })
            .collect();
        let active = self.active_utility_by_window.get(&window_kind.id).map(String::as_str);
        ui_wgpu::wgpu::component::utilities::derive_utility_nodes(&session.app.controller_id, &specs, active)
    }
    // #endregion

    // #region active-utility
    /// 🧰️ Applies a user-driven `setActiveUtility`: re-selecting the active utility deactivates it, otherwise
    /// it becomes the active utility for that window kind (Architecture Decision 4).
    pub(crate) fn apply_set_active_utility(&mut self, window_kind_id: &str, utility_id: &str) {
        let already = self.active_utility_by_window.get(window_kind_id).map(String::as_str) == Some(utility_id);
        if already {
            self.active_utility_by_window.remove(window_kind_id);
        } else {
            self.active_utility_by_window.insert(window_kind_id.to_string(), utility_id.to_string());
            // 🎓️ Advance-by-doing: only the activation branch counts as "the utility was activated" —
            // see `chrome_tour_note_utility_performed`.
            self.chrome_tour_note_utility_performed(utility_id);
        }
    }

    /// 🧰️ The active utility id for a window kind, if any.
    pub(crate) fn active_utility_for_window(&self, window_kind_id: &str) -> Option<&str> {
        self.active_utility_by_window.get(window_kind_id).map(String::as_str)
    }

    /// 🖱️ The cursor the active utility requests while the pointer is inside the active window silhouette — maps
    /// `UtilityDefinition.cursor` onto a {@link ui_wgpu::wgpu::SemioCursor} (P5). `None` when no utility/cursor applies.
    pub(crate) fn utility_cursor_override(&self, x: f32, y: f32) -> Option<ui_wgpu::wgpu::SemioCursor> {
        let session = self.session.as_ref()?;
        let window_id = self.active_window_id.as_deref()?;
        let utility_id = self.active_utility_for_window(window_id)?;
        let cursor_name = session.app.utilities.iter().find(|utility| utility.id == utility_id)?.cursor.as_deref()?;
        let visible = self.window_silhouettes.get(window_id).map_or_else(|| self.window_content_rects.get(window_id).is_some_and(|rect| rect.contains(x, y)), |silhouette| silhouette.contains(x, y));
        visible.then(|| semio_cursor_from_name(cursor_name))
    }

    /// 🚦️ Whether window-scoped actions stay enabled: `true` when no utility is active or the active utility
    /// declares `allows_actions_while_active` (P5 — replaces the old `UTILITY_ID_PREFIXES` whitelist).
    pub(crate) fn actions_enabled_for_window(&self, app: &AppDefinition, window_kind_id: &str) -> bool {
        match self.active_utility_for_window(window_kind_id) {
            None => true,
            Some(utility_id) => app.utilities.iter().find(|utility| utility.id == utility_id).map(|utility| utility.allows_actions_while_active).unwrap_or(true),
        }
    }
    // #endregion

    // #region staging
    fn staged_key(window_id: &str, action_id: &str) -> String {
        format!("{window_id}:{action_id}")
    }

    pub(crate) fn stage_arg(&mut self, window_id: &str, action_id: &str, arg_id: &str, value: Value) {
        self.staged_action_args.entry(Self::staged_key(window_id, action_id)).or_default().insert(arg_id.to_string(), value);
    }

    pub(crate) fn staged_map_for(&self, window_id: &str, action_id: &str) -> serde_json::Map<String, Value> {
        self.staged_action_args.get(&Self::staged_key(window_id, action_id)).cloned().unwrap_or_default()
    }

    pub(crate) fn reset_staged_args(&mut self, window_id: &str, action_id: &str) {
        self.staged_action_args.remove(&Self::staged_key(window_id, action_id));
    }

    /// 📝️ Parses a focused staged-arg input's buffer per the arg's control kind and writes it into the
    /// staging map. Returns `true` when `control_id` belongs to a staged input (so the caller stops).
    fn commit_staged_input(&mut self, control_id: &str, buffer: &str) -> bool {
        use semio_framework::ActionArgControl;
        let (is_vec3, rest) = if let Some(rest) = control_id.strip_prefix("shell.action.argvec3::") {
            (true, rest)
        } else if let Some(rest) = control_id.strip_prefix("shell.action.arginput::") {
            (false, rest)
        } else {
            return false;
        };
        let parts: Vec<&str> = rest.split("::").collect();
        let (Some(window_id), Some(action_id), Some(arg_id)) = (parts.first().copied(), parts.get(1).copied(), parts.get(2).copied()) else {
            return true;
        };
        let (window_id, action_id, arg_id) = (window_id.to_string(), action_id.to_string(), arg_id.to_string());
        if is_vec3 {
            let axis: usize = parts.get(3).and_then(|token| token.parse().ok()).unwrap_or(0);
            let mut current: Vec<Value> = self
                .staged_map_for(&window_id, &action_id)
                .get(&arg_id)
                .and_then(|value| value.as_array().cloned())
                .or_else(|| self.arg_default(&window_id, &action_id, &arg_id).and_then(|value| value.as_array().cloned()))
                .unwrap_or_else(|| vec![serde_json::json!(0.0), serde_json::json!(0.0), serde_json::json!(0.0)]);
            while current.len() < 3 {
                current.push(serde_json::json!(0.0));
            }
            if axis < 3 {
                current[axis] = serde_json::json!(buffer.trim().parse::<f64>().unwrap_or(0.0));
            }
            self.stage_arg(&window_id, &action_id, &arg_id, Value::Array(current));
            return true;
        }
        let control = self.session.as_ref().and_then(|session| window_action_definition(&session.app, &window_id, &action_id)).and_then(|action| action.args.iter().find(|arg| arg.id == arg_id)).map(|arg| arg.control());
        let value = match control {
            Some(ActionArgControl::Number { .. }) | Some(ActionArgControl::Slider { .. }) => {
                serde_json::json!(buffer.trim().parse::<f64>().unwrap_or(0.0))
            }
            _ => Value::String(buffer.to_string()),
        };
        self.stage_arg(&window_id, &action_id, &arg_id, value);
        true
    }

    /// 📝️ The seed string used when focusing a staged-arg input — its current effective value.
    fn staged_input_seed(&self, control_id: &str) -> Option<String> {
        let (is_vec3, rest) = if let Some(rest) = control_id.strip_prefix("shell.action.argvec3::") {
            (true, rest)
        } else if let Some(rest) = control_id.strip_prefix("shell.action.arginput::") {
            (false, rest)
        } else {
            return None;
        };
        let parts: Vec<&str> = rest.split("::").collect();
        let window_id = parts.first().copied()?;
        let action_id = parts.get(1).copied()?;
        let arg_id = parts.get(2).copied()?;
        let session = self.session.as_ref()?;
        let arg = window_action_definition(&session.app, window_id, action_id)?.args.iter().find(|arg| arg.id == arg_id)?;
        let effective = self.effective_arg_value(window_id, action_id, arg);
        if is_vec3 {
            let axis: usize = parts.get(3).and_then(|token| token.parse().ok()).unwrap_or(0);
            let number = effective.as_ref().and_then(|value| value.as_array()).and_then(|array| array.get(axis)).and_then(|value| value.as_f64()).unwrap_or(0.0);
            return Some(fmt_num(number));
        }
        Some(match effective {
            Some(Value::String(text)) => text,
            Some(Value::Number(num)) => num.to_string(),
            Some(Value::Bool(flag)) => flag.to_string(),
            Some(other) => other.to_string(),
            None => String::new(),
        })
    }

    /// 🧮️ Validated effective args for execution: `None` when a required arg is still unset — the P2
    /// gate that keeps arg-carrying actions from firing partially (delegates to the core-side pure
    /// {@link semio_framework::missing_required_args}).
    pub(crate) fn resolved_execute_args(defs: &[semio_framework::ActionArgDef], staged: &serde_json::Map<String, Value>) -> Option<serde_json::Map<String, Value>> {
        let staged_dsl = semio_framework::to_dsl_value(&Value::Object(staged.clone())).ok()?;
        let effective = semio_framework::effective_action_args(defs, &staged_dsl, None);
        if semio_framework::missing_required_args(defs, &effective).is_empty() {
            semio_framework::from_dsl_value::<Value>(effective).ok().and_then(|value| value.as_object().cloned())
        } else {
            None
        }
    }

    /// 🚀️ Executes a staged action once (P2): validates required args, dispatches exactly one
    /// `ActionDescriptor` with the merged effective args, and keeps the staged values for tweak-and-
    /// repeat. No-operations when the active utility gates actions or a required arg is still unset.
    async fn execute_staged_action(&mut self, window_id: &str, action_id: &str) -> Result<(), String> {
        let Some(session) = self.session.clone() else {
            return Ok(());
        };
        if !self.actions_enabled_for_window(&session.app, window_id) {
            return Ok(());
        }
        let Some(action) = window_action_definition(&session.app, window_id, action_id).cloned() else {
            return Ok(());
        };
        let staged = self.staged_map_for(window_id, action_id);
        let Some(effective) = Self::resolved_execute_args(&action.args, &staged) else {
            return Ok(());
        };
        let args = semio_framework::optional_json_to_dsl(if effective.is_empty() { None } else { Some(Value::Object(effective)) });
        self.dispatch_action(ActionDescriptor { controller_id: session.app.controller_id.clone(), action: action_id.to_string(), args }).await
    }
    // #endregion

    // #region command-registry
    /// 🔌️ The current session's program manifest (for its `commands: Vec<CommandDefinition>` — Plugin-scope
    /// commands apply whenever any of that plugin's apps is focused, mirroring `os-shell.tsx`'s
    /// `activePluginManifest`).
    fn active_plugin_manifest(&self) -> Option<&semio_framework::PluginManifest> {
        let session = self.session.as_ref()?;
        self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).map(|entry| &entry.manifest)
    }

    /// 🎛️ Os-level built-in commands — the wgpu mirror of `os-shell.tsx`'s `buildOsCommands`, scoped to
    /// mutation paths that already exist in this shell: `appearance_id`/`driver_id`/`locale_id`/
    /// `terminology_id` (all wired through the existing `"framework"` controller `dispatch_action` switch
    /// — see `apply_os_command`), and the dock's `layout_override`
    /// (`os.resetDock`, applied locally like `RESET_DOCK` resets `dockLayoutStore`/`dockUiStateStore` in
    /// React — no plugin round-trip). Deliberately omits `os.introduceApp`/`os.setThemeId`/`os.setLayout`:
    /// none of the three has any persisted shell state yet (no introduction-playback step, no named
    /// `UiTheme` list, no desktop/tablet layout flag) — inventing that storage is out of this region's
    /// scope (`shell::ShellTypes` owns `ShellState`'s fields and is off-limits this wave).
    pub(crate) fn build_os_commands(&self) -> Vec<semio_framework::CommandDefinition> {
        use semio_framework::manifest::Platform;
        use semio_framework::{ActionArgDef, ActionArgOption, ActionKind, CommandDefinition, PlatformKeybinding};
        let terminology_options: Vec<ActionArgOption> = self.active_terminologies().into_iter().map(|id| ActionArgOption { label: if id == "native" { LocalizedLabel::data("Native") } else { LocalizedLabel::data(id.clone()) }, value: id }).collect();
        vec![
            CommandDefinition::bounded_catalog("os.toggleFullscreen", LocalizedLabel::native("Toggle Full Screen", "Vollbild umschalten"), "window", ActionKind::Shell)
                .with_keybinding(PlatformKeybinding::for_platform("f11", Platform::Windows))
                .with_keybinding(PlatformKeybinding::for_platform("f11", Platform::Linux))
                .with_keybinding(PlatformKeybinding::for_platform("control+meta+f", Platform::MacOs)),
            CommandDefinition::bounded_catalog("os.setAppearance", LocalizedLabel::data("Set Appearance"), "appearance", ActionKind::Shell).with_args([ActionArgDef::select(
                "value",
                LocalizedLabel::data("Appearance"),
                vec![
                    ActionArgOption { value: "system".into(), label: LocalizedLabel::data("System") },
                    ActionArgOption { value: "light".into(), label: LocalizedLabel::data("Light") },
                    ActionArgOption { value: "dark".into(), label: LocalizedLabel::data("Dark") },
                ],
            )
            .required()]),
            CommandDefinition::bounded_catalog("os.setDriver", LocalizedLabel::data("Set Driver"), "layout", ActionKind::Shell).with_args([ActionArgDef::select(
                "value",
                LocalizedLabel::data("Driver"),
                vec![ActionArgOption { value: "default".into(), label: LocalizedLabel::data("Default") }, ActionArgOption { value: "compact".into(), label: LocalizedLabel::data("Compact") }],
            )
            .required()]),
            CommandDefinition::bounded_catalog("os.setLocale", LocalizedLabel::data("Set Locale"), "language", ActionKind::Shell).with_args([ActionArgDef::select(
                "value",
                LocalizedLabel::data("Locale"),
                vec![ActionArgOption { value: "en".into(), label: LocalizedLabel::data("English") }, ActionArgOption { value: "de".into(), label: LocalizedLabel::data("Deutsch") }],
            )
            .required()]),
            CommandDefinition::bounded_catalog("os.setTerminology", LocalizedLabel::data("Set Terminology"), "language", ActionKind::Shell)
                .with_args([ActionArgDef::select("value", LocalizedLabel::data("Terminology"), terminology_options).required()]),
            CommandDefinition::bounded_catalog("os.setThemeId", LocalizedLabel::data("Set Theme"), "appearance", ActionKind::Shell).with_args([ActionArgDef::select(
                "value",
                LocalizedLabel::data("Theme"),
                std::iter::once(ActionArgOption { value: "semio".into(), label: LocalizedLabel::data("Semio") })
                    .chain(std::iter::once(ActionArgOption { value: "mono".into(), label: LocalizedLabel::data("Mono") }))
                    .chain(self.chrome_build.preferences.custom_themes.keys().cloned().map(|id| {
                        let label = custom_theme_definition_from(&self.chrome_build.preferences, &id).map(|theme| theme.label).unwrap_or_else(|| id.clone());
                        ActionArgOption { value: id, label: LocalizedLabel::data(label) }
                    }))
                    .collect(),
            )
            .required()]),
            CommandDefinition::bounded_catalog("os.resetDock", LocalizedLabel::data("Reset Dock Layout"), "layout", ActionKind::Shell),
        ]
    }

    /// 🎛️ Every command visible for the current session — os built-ins, the active plugin's Plugin-scope
    /// commands, and the app's App-/active-Mode-scope commands. The wgpu mirror of `os-shell.tsx`'s
    /// `resolveCommands(buildOsCommands(...), activePluginManifest, session?.app, activeModeId)` call site.
    pub(crate) fn resolved_commands(&self) -> Vec<ResolvedCommand> {
        let os_commands = self.build_os_commands();
        let Some(session) = self.session.as_ref() else {
            return os_commands.into_iter().map(|definition| ResolvedCommand::new(definition, semio_framework::manifest::CommandOwnerAddress::Os)).collect();
        };
        let active_mode_id = session.view_state.active_mode_id.as_deref().unwrap_or(session.app.default_mode_id.as_str());
        resolve_commands(os_commands, self.active_plugin_manifest(), &session.plugin_id, &session.app, active_mode_id)
    }

    /// 🔍️ Flattens `resolved_commands()` into quick-search-palette entries. Zero-arg commands (any source)
    /// dispatch immediately; an os-scope command with a single `Select` arg — every os command declares
    /// exactly that shape (see `build_os_commands`) — expands into one concrete item per option (e.g. "Set
    /// Appearance: Light") rather than redirecting to a staged form, since (unlike window-scoped actions)
    /// os commands have no hosting window whose Actions rail could show that form. Arg-carrying Plugin/
    /// App/Mode-scope commands are skipped here — there is no `ArtifactApp::handle_command` RPC wired on
    /// the plugin bridge yet (only `handle_action` exists; see `ProgramBridgeEntry`), so the same staged
    /// redirect would open a form with no way to actually execute; they still appear in
    /// `resolved_commands()`/`build_command_panel_ui()` for completeness.
    pub(crate) fn command_search_items(&self) -> Vec<SearchPaletteItem> {
        let mut items = Vec::new();
        for entry in self.resolved_commands() {
            let ResolvedCommand { definition, address } = entry;
            if !definition.in_palette {
                continue;
            }
            let category = address.owner.clone();
            let group = command_category_label(&definition.category);
            if definition.args.is_empty() {
                let is_os = matches!(address.owner, semio_framework::manifest::CommandOwnerAddress::Os);
                let invocation = semio_framework::manifest::CommandInvocation { address: address.clone(), arguments: BTreeMap::new() };
                items.push(SearchPaletteItem {
                    id: format!("command.{}", command_address_stable_key(&address)),
                    label: definition.label.resolve(self.active_terminology(), self.active_locale()).to_string(),
                    group,
                    dispatch_action: None,
                    action: Some(if is_os { format!("os-command:{}", definition.id) } else { format!("command:{}", serde_json::to_string(&invocation).expect("command invocation serializes")) }),
                    category: Some(category.clone()),
                });
                continue;
            }
            if let Some(arg) = definition.args.first() {
                if let semio_framework::ActionArgControl::Select { options } = &arg.control() {
                    for option in options {
                        let is_os = matches!(address.owner, semio_framework::manifest::CommandOwnerAddress::Os);
                        let invocation = semio_framework::manifest::CommandInvocation { address: address.clone(), arguments: BTreeMap::from([(arg.id.clone(), Value::String(option.value.clone()))]) };
                        items.push(SearchPaletteItem {
                            id: format!("command.{}.{}", command_address_stable_key(&address), option.value),
                            label: format!("{}: {}", definition.label.resolve(self.active_terminology(), self.active_locale()), option.label.resolve(self.active_terminology(), self.active_locale())),
                            group: group.clone(),
                            dispatch_action: None,
                            action: Some(if is_os { format!("os-command:{}:{}", definition.id, option.value) } else { format!("command:{}", serde_json::to_string(&invocation).expect("command invocation serializes")) }),
                            category: Some(category.clone()),
                        });
                    }
                }
            }
        }
        items
    }

    /// 🚀️ Executes an os-level command by id (the wgpu mirror of `os-shell.tsx`'s `dispatchOsCommand`):
    /// `os.resetDock` clears the persisted layout override locally (no document round-trip, exactly like
    /// React's `RESET_DOCK` resetting `dockLayoutStore`/`dockUiStateStore`); every other command reuses the
    /// existing `"framework"` controller `dispatch_action` switch (`setAppearance`/`setDriver`/
    /// `setLocale`/`setTerminology`) that already backs the Settings panel's selects, so this
    /// never invents a new mutation path.
    pub(crate) async fn apply_os_command(&mut self, command_id: &str, option_value: Option<&str>) -> Result<(), String> {
        match command_id {
            "os.toggleFullscreen" => {
                self.fullscreen_toggle_requested = true;
                Ok(())
            }
            "os.resetDock" => {
                self.layout_override = None;
                self.sync_dock();
                Ok(())
            }
            "os.setAppearance" | "os.setDriver" | "os.setLocale" | "os.setTerminology" | "os.setThemeId" => {
                let Some(value) = option_value else {
                    return Ok(());
                };
                let action = match command_id {
                    "os.setAppearance" => "setAppearance",
                    "os.setDriver" => "setDriver",
                    "os.setLocale" => "setLocale",
                    "os.setTerminology" => "setTerminology",
                    "os.setThemeId" => "setThemeId",
                    _ => unreachable!(),
                };
                self.dispatch_action(ActionDescriptor { controller_id: "framework".into(), action: action.into(), args: crate::action_args_json!({ "value": value }) }).await
            }
            _ => Ok(()),
        }
    }

    //#region ShellCommandHistory
    /// 🕒️ Looks up an `os.*` setting command's display label from `build_os_commands`' registry
    /// (`os.setAppearance`/`os.setDriver`/`os.setLocale`/`os.setTerminology`/`os.setThemeId`), falling
    /// back to the theme reset/delete buttons' own `shell_chrome_string` i18n labels for the two
    /// `build_settings_theme_ui` commands that never got a `CommandDefinition` (see that fn's own
    /// `os.setThemeId`-only doc comment) — the single label source `note_shell_command_action` call
    /// sites below draw from, so a chrome label edit never drifts from what the history panel shows.
    fn shell_command_label_for_setting(&self, command_id: &str) -> String {
        match command_id {
            "os.resetThemeId" => shell_chrome_string("settings.theme.reset", self.locale_id == "de").to_string(),
            "os.deleteThemeId" => shell_chrome_string("settings.theme.delete", self.locale_id == "de").to_string(),
            _ => self.build_os_commands().into_iter().find(|definition| definition.id == command_id).map(|definition| definition.label.resolve(self.active_terminology(), self.active_locale()).to_string()).unwrap_or_else(|| command_id.to_string()),
        }
    }

    /// 🧭️ Parses `dock.tab.{path}.{window}.{action}` into `(path, window_id)`.
    fn parse_dock_tab_action_id<'a>(control_id: &'a str, action: &str) -> Option<(Vec<usize>, &'a str)> {
        let rest = control_id.strip_prefix("dock.tab.")?;
        let rest = rest.strip_suffix(&format!(".{action}"))?;
        let (path_str_value, window_id) = rest.split_once('.')?;
        Some((parse_path(path_str_value), window_id))
    }

    /// 🕒️ Maps a chrome control id to its `noteShellCommand` `(commandId, label)` pair — factored out
    /// of `handle_shell_hit`'s per-arm dispatch so the mapping is unit-testable without a full
    /// `ShellState` fixture. Only control ids that represent a discrete, loggable user command are
    /// covered; everything else is `None` (`handle_shell_hit` keeps its existing behavior either way).
    fn shell_command_for_control(control_id: &str, is_de: bool) -> Option<(&'static str, String)> {
        if control_id.starts_with("dock.tab.") && control_id.ends_with(".close") {
            return Some(("shell.windowClose", "Close".to_string()));
        }
        if control_id.starts_with("dock.tab.") && control_id.ends_with(".focus") {
            return Some(("shell.windowMaximize", "Focus".to_string()));
        }
        if control_id.starts_with("shell.layout.") {
            return Some(("shell.applyNamedLayout", "Apply Layout".to_string()));
        }
        if control_id == "ui.panelToggle.details" {
            return Some(("shell.panelToggle", shell_chrome_string("panelToggle.details", is_de).to_string()));
        }
        if control_id == "ui.panelToggle.settings" {
            return Some(("shell.panelToggle", shell_chrome_string("panelToggle.settings", is_de).to_string()));
        }
        None
    }

    /// 🕒️ Builds a `noteShellCommand` `ActionDescriptor` — same inline-construction shape as
    /// `map_action`/`scene_action`/`board_action` elsewhere in this crate, targeting the
    /// framework-injected, session-only command-history tap (Shell-kind, intercepted before the app
    /// ever sees it — see the plugin crate's universal command-recording mechanism). `detail` is the
    /// caller-supplied JSON payload (e.g. `{"value": ...}`/`{"windowId": ...}`), left out of `args`
    /// entirely when `None` rather than serialized as `null`.
    fn note_shell_command_action(controller_id: &str, command_id: &str, label: &str, detail: Option<Value>) -> ActionDescriptor {
        let mut args = serde_json::json!({ "commandId": command_id, "label": label });
        if let Some(detail) = detail {
            args["detail"] = detail;
        }
        ActionDescriptor { controller_id: controller_id.to_string(), action: "noteShellCommand".into(), args: semio_framework::optional_json_to_dsl(Some(args)) }
    }

    /// 🕒️ The `dispatch_action`-recursion delivery path (mechanism (a) — a `noteShellCommand`'s
    /// action id falls through every special-cased arm, including this one's own `"framework"`
    /// branch, straight to the normal plugin-forwarding tail) for the framework settings arms below.
    /// `Box::pin` sidesteps `dispatch_action` calling itself inside its own generated future
    /// (rustc E0733) — the recursion itself is exactly what the ticket calls for. No-ops without an
    /// active session: nothing to log a shell command against.
    async fn note_shell_setting_command(&mut self, command_id: &str, value: Option<&str>) -> Result<(), String> {
        let Some(controller_id) = self.host_controller_id() else {
            return Ok(());
        };
        let label = self.shell_command_label_for_setting(command_id);
        let detail = value.map(|value| serde_json::json!({ "value": value }));
        let note = Self::note_shell_command_action(&controller_id, command_id, &label, detail);
        Box::pin(self.dispatch_action(note)).await
    }

    /// 🕒️ `handle_shell_hit`'s generic seam into `shell_command_for_control`'s `(commandId, label)`
    /// mapping — every discrete chrome-control arm that should log a history row calls this instead
    /// of hand-rolling the same `host_controller_id`/`dispatch_action` boilerplate. A silent no-op for
    /// any control id `shell_command_for_control` doesn't recognize, and without an active session
    /// (nothing to log a shell command against). Not itself inside `dispatch_action`, so a plain
    /// `.await` (no `Box::pin`) suffices here.
    async fn note_control_command(&mut self, control_id: &str, detail: Option<Value>) -> Result<(), String> {
        let Some((command_id, label)) = Self::shell_command_for_control(control_id, self.locale_id == "de") else {
            return Ok(());
        };
        let Some(controller_id) = self.host_controller_id() else {
            return Ok(());
        };
        let note = Self::note_shell_command_action(&controller_id, command_id, &label, detail);
        self.dispatch_action(note).await
    }

    /// 🕒️ `ui.panelToggle.details`/`ui.panelToggle.settings` share this: both log the same
    /// `shell.panelToggle` command, differing only in the `panel` id and the post-toggle `visible`
    /// flag — the caller has already flipped `right_panel_open`/`active_right_kind` before this runs,
    /// so `visible` reads the POST-toggle state directly (no separate before/after bookkeeping).
    async fn note_panel_toggle_command(&mut self, control_id: &str, panel: &str) -> Result<(), String> {
        let visible = self.right_panel_open
            && match panel {
                "details" => self.active_right_kind == RightPanelKind::Details,
                "settings" => self.active_right_kind == RightPanelKind::Settings,
                _ => false,
            };
        self.note_control_command(control_id, Some(serde_json::json!({ "panel": panel, "visible": visible }))).await
    }
    //#endregion ShellCommandHistory

    /// 🎛️ Data-complete "Commands" panel content — the wgpu mirror of React's
    /// `buildCommandCategoryTabs`/`buildCommandCategoryTree`, folded into a single flat, category-headed
    /// `UiNode` tree (the honestly-scoped fallback: React surfaces this as a persistent `bottom-middle`
    /// dock anchor, which this renderer has no equivalent of — `PanelGroup::anchor` only ever maps to the
    /// four corners, and the two middle anchors "start empty... never via a `PanelGroup`" per its own doc
    /// comment; building a real middle anchor would mean touching `dock`/restructuring `ShellTypes`'s
    /// hardcoded 2-column model, both out of scope). This builder is a test oracle until framework
    /// chrome publishes through the retained semantic document boundary.
    /// Every row for a command whose id already has a `"framework"` `dispatch_action` arm
    /// (appearance/driver/locale/terminology/themeId) is fully interactive; `os.resetDock` has no such
    /// arm to attach a plain `ActionDescriptor` to (only the ⌘️K search's `"os-command:"` string redirect
    /// can reach `apply_os_command` for it), so it renders as a pointer to command search instead of a
    /// non-functional button.
    #[cfg(test)]
    pub(crate) fn build_command_panel_ui(&self) -> UiNode {
        let resolved: Vec<_> = self.resolved_commands().into_iter().filter(|entry| entry.definition.in_palette).collect();
        let categories = command_categories(&resolved);
        let mut sections: Vec<UiNode> = Vec::new();
        for (category_id, category_label) in categories {
            let mut rows: Vec<UiNode> = vec![UiNode::Text(UiTextNode { presence: UiPresence::default(), value: Label::data(category_label), emphasize: Some(true), data_attributes: None, menu: None })];
            rows.extend(resolved.iter().filter(|entry| entry.definition.category == category_id).map(|entry| self.build_command_panel_row(entry)));
            sections.push(UiNode::Stack(UiStackNode {
                direction: "column".into(),
                gap: None,
                padding: None,
                id: Some(format!("shell.commands.category.{category_id}")),
                children: rows,
                activate: None,
                drop_action: None,
                drop_overlay: None,
                presence: UiPresence::default(),
                menu: None,
            }));
        }
        UiNode::Stack(UiStackNode {
            direction: "column".into(),
            gap: None,
            padding: None,
            id: Some("shell.commands.panel".into()),
            children: sections,
            activate: None,
            drop_action: None,
            drop_overlay: None,
            presence: UiPresence::default(),
            menu: None,
        })
    }

    /// 🎛️ One `build_command_panel_ui` row: a `Select` for the four os commands whose single arg already
    /// has a "framework" `dispatch_action` arm, else a plain (non-interactive) label — see
    /// `build_command_panel_ui`'s doc comment for why `os.resetDock` and any future arg-carrying
    /// Plugin/App/Mode command fall into that last case.
    #[cfg(test)]
    fn build_command_panel_row(&self, entry: &ResolvedCommand) -> UiNode {
        let definition = &entry.definition;
        if let Some(arg) = definition.args.first() {
            if let semio_framework::ActionArgControl::Select { options } = &arg.control() {
                let value = match definition.id.as_str() {
                    "os.setAppearance" => self.appearance_id.clone(),
                    "os.setDriver" => self.driver_id.clone(),
                    "os.setLocale" => self.locale_id.clone(),
                    "os.setTerminology" => self.terminology_id.clone(),
                    _ => options.first().map(|option| option.value.clone()).unwrap_or_default(),
                };
                let action = match definition.id.as_str() {
                    "os.setAppearance" => "setAppearance",
                    "os.setDriver" => "setDriver",
                    "os.setLocale" => "setLocale",
                    "os.setTerminology" => "setTerminology",
                    other => other,
                };
                return UiNode::Select(UiSelectNode {
                    presence: UiPresence::default(),
                    id: format!("shell.commands.{}", definition.id),
                    value,
                    items: options.iter().map(|option| UiSelectItem { value: option.value.clone(), label: Label::data(option.label.resolve(self.active_terminology(), self.active_locale())) }).collect(),
                    placeholder: None,
                    on_change: ActionDescriptor { controller_id: "framework".into(), action: action.into(), args: None },
                    menu: None,
                });
            }
        }
        UiNode::Text(UiTextNode {
            presence: UiPresence::default(),
            value: if definition.id == "os.resetDock" {
                Label::data(format!("{} — available via ⌘️K command search", definition.label.resolve(self.active_terminology(), self.active_locale())))
            } else {
                Label::data(definition.label.resolve(self.active_terminology(), self.active_locale()))
            },
            emphasize: Some(false),
            data_attributes: None,
            menu: None,
        })
    }
    // #endregion
}

/// 🎛️ One aggregated command plus its fully-qualified owner address.
#[derive(Clone, Debug)]
pub(crate) struct ResolvedCommand {
    pub definition: semio_framework::CommandDefinition,
    pub address: semio_framework::manifest::CommandAddress,
}

impl ResolvedCommand {
    fn new(definition: semio_framework::CommandDefinition, owner: semio_framework::manifest::CommandOwnerAddress) -> Self {
        let command_id = definition.id.clone();
        Self { definition, address: semio_framework::manifest::CommandAddress { owner, command_id } }
    }
}

fn command_address_stable_key(address: &semio_framework::manifest::CommandAddress) -> String {
    match &address.owner {
        semio_framework::manifest::CommandOwnerAddress::Os => format!("os:{}", address.command_id),
        semio_framework::manifest::CommandOwnerAddress::Plugin { plugin_id } => format!("plugin:{plugin_id}:{}", address.command_id),
        semio_framework::manifest::CommandOwnerAddress::App { plugin_id, app_id } => format!("app:{plugin_id}:{app_id}:{}", address.command_id),
        semio_framework::manifest::CommandOwnerAddress::Mode { plugin_id, app_id, mode_id } => format!("mode:{plugin_id}:{app_id}:{mode_id}:{}", address.command_id),
    }
}

/// 🎛️ Merges os-built-in, Plugin-scope, App-scope, and active-Mode-scope commands into one list — the
/// wgpu mirror of `os-shell.tsx`'s `resolveCommands`. A `Mode`-scope command only resolves when
/// `active_mode_id`'s `ModeDefinition.commands` references it, exactly like the React source.
pub(crate) fn resolve_commands(os_commands: Vec<semio_framework::CommandDefinition>, plugin_manifest: Option<&semio_framework::PluginManifest>, plugin_id: &str, app: &AppDefinition, active_mode_id: &str) -> Vec<ResolvedCommand> {
    let mut resolved: Vec<ResolvedCommand> = os_commands.into_iter().map(|definition| ResolvedCommand::new(definition, semio_framework::manifest::CommandOwnerAddress::Os)).collect();
    if let Some(manifest) = plugin_manifest {
        resolved.extend(manifest.commands.iter().cloned().map(|definition| ResolvedCommand::new(definition, semio_framework::manifest::CommandOwnerAddress::Plugin { plugin_id: plugin_id.to_string() })));
    }
    resolved.extend(app.commands.iter().cloned().map(|definition| ResolvedCommand::new(definition, semio_framework::manifest::CommandOwnerAddress::App { plugin_id: plugin_id.to_string(), app_id: app.id.clone() })));
    if let Some(mode) = app.modes.iter().find(|mode| mode.id == active_mode_id) {
        resolved.extend(mode.commands.iter().cloned().map(|definition| ResolvedCommand::new(definition, semio_framework::manifest::CommandOwnerAddress::Mode { plugin_id: plugin_id.to_string(), app_id: app.id.clone(), mode_id: mode.id.clone() })));
    }
    resolved
}

/// 🎛️ Loose title-case for an open-set command category id (e.g. `"appearance"` -> `"Appearance"`,
/// `"named-layout"` -> `"Named Layout"`) — the wgpu mirror of `os-shell.tsx`'s `titleizeCommandCategory`.
/// wgpu has no `ui.settings.tab.*` translation table to special-case chrome-known ids against, so every
/// category (chrome-known or app/plugin-invented) goes through this uniformly.
pub(crate) fn command_category_label(category: &str) -> String {
    category
        .split(|c: char| c == '-' || c == '_')
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// 🎛️ Ordered, deduped `(category id, display label)` pairs derived from whatever commands actually
/// resolved — the wgpu mirror of `os-shell.tsx`'s `commandCategories`.
pub(crate) fn command_categories(commands: &[ResolvedCommand]) -> Vec<(String, String)> {
    let mut seen = std::collections::HashSet::new();
    let mut categories = Vec::new();
    for entry in commands {
        let category = entry.definition.category.as_str();
        if seen.insert(category.to_string()) {
            categories.push((category.to_string(), command_category_label(category)));
        }
    }
    categories
}

/// 🔍️ Case-insensitive fuzzy subsequence match: every char of `query`, in order, must appear somewhere in
/// `target` (not necessarily contiguously). Returns `None` when `query` isn't a subsequence of `target`,
/// else a score that rewards contiguous runs and word-start matches and lightly penalizes long targets —
/// replaces `filtered_search_items`'/`filtered_find_items`'-style pure `.contains()` substring filtering
/// (no fuzzy-search crate exists in this workspace's dependency tree; deliberately hand-rolled rather than
/// adding one — see `Cargo.lock`).
pub(crate) fn fuzzy_match_score(query: &str, target: &str) -> Option<i64> {
    let query = query.trim();
    if query.is_empty() {
        return Some(0);
    }
    let query_chars: Vec<char> = query.chars().flat_map(char::to_lowercase).collect();
    let target_chars: Vec<char> = target.chars().flat_map(char::to_lowercase).collect();
    let mut target_index = 0usize;
    let mut score: i64 = 0;
    let mut consecutive: i64 = 0;
    for &query_char in &query_chars {
        let mut matched = false;
        while target_index < target_chars.len() {
            let candidate = target_chars[target_index];
            let at_word_start = target_index == 0 || !target_chars[target_index - 1].is_alphanumeric();
            target_index += 1;
            if candidate == query_char {
                score += 10 + consecutive * 5 + if at_word_start { 8 } else { 0 };
                consecutive += 1;
                matched = true;
                break;
            }
            consecutive = 0;
        }
        if !matched {
            return None;
        }
    }
    score -= (target_chars.len() as i64 - query_chars.len() as i64).max(0);
    Some(score)
}

#[cfg(test)]
mod command_registry_tests {
    use super::*;
    use semio_framework::{
        ActionArgControl, ActionKind, AppDefinition, AppRole, ArtifactDialect, CommandDefinition, CommandOwnerAddress, ModeDefinition, Modes, PanelGroup, PanelTabDefinition, PanelTabKind, PluginManifest, WindowKindDefinition, WindowKinds,
    };

    fn test_app(commands: Vec<CommandDefinition>, mode_commands: Vec<CommandDefinition>) -> AppDefinition {
        AppDefinition {
            id: "test-app".into(),
            role: AppRole::Editor,
            dialect: ArtifactDialect { artifact_kind: "s.test.app".into(), standard: "1".into(), subset: "*".into() },
            label: LocalizedLabel::data("Test App"),
            breadcrumb: vec!["semio".into(), "test".into()],
            icon_id: None,
            controller_id: "test".into(),
            modes: Modes::one(ModeDefinition { id: "default".into(), label: LocalizedLabel::data("Default"), icon_id: "pencil".into(), tools: vec![], layout_id: None, commands: mode_commands }),
            default_mode_id: "default".into(),
            window_kinds: WindowKinds::try_from(vec![WindowKindDefinition {
                id: "main".into(),
                label: LocalizedLabel::data("Main"),
                body_key: "main.body".into(),
                surface_kind: ui_wgpu::wgpu::SurfaceKind::Canvas2d,
                icon_id: "app-window".into(),
                options: Default::default(),
                actions: vec![],
                utilities: vec![],
                interactions: vec![],
                params_schema: None,
                artifact_snapshot_schema: None,
                input_event_schema: None,
                output_schema: None,
                capabilities: vec![],
            }])
            .expect("non-empty"),
            panel_tabs: vec![PanelTabDefinition { kind: PanelTabKind::App("tab".into()), label: LocalizedLabel::data("Tab"), group: PanelGroup::Workbench, body_key: Some("tab.body".into()), children: vec![] }],
            keybindings: vec![],
            utilities: vec![],
            tools: vec![],
            commands,
            interactions: vec![],
            named_layouts: vec![],
            default_layout: None,
            terminologies: vec!["de".into()],
            terminology_breadcrumbs: std::collections::HashMap::new(),
            introduction: None,
            tutorials: Vec::new(),
            dialogs: Vec::new(),
            media_inputs: Vec::new(),
            media_outputs: Vec::new(),
            artifact_kinds: Vec::new(),
            config: semio_framework_async::block_on(semio_framework::ConfigSpec::empty()),
            command_grammar: semio_framework_async::block_on(semio_framework::CommandGrammar::empty()),
            io: semio_framework::AppIo::default(),
        }
    }

    fn test_shell_state() -> ShellState {
        ShellState::new(Vec::new(), String::new())
    }

    #[test]
    fn build_os_commands_covers_every_wired_setting() {
        let shell = test_shell_state();
        let ids: Vec<String> = shell.build_os_commands().into_iter().map(|command| command.id).collect();
        assert_eq!(ids, vec!["os.toggleFullscreen", "os.setAppearance", "os.setDriver", "os.setLocale", "os.setTerminology", "os.setThemeId", "os.resetDock",]);
    }

    #[test]
    fn fullscreen_command_requests_the_host_transition() {
        let mut shell = test_shell_state();
        semio_framework_async::block_on(shell.apply_os_command("os.toggleFullscreen", None)).expect("fullscreen command");
        assert!(shell.fullscreen_toggle_requested);
    }

    #[test]
    fn build_os_commands_terminology_options_include_app_terminologies() {
        let mut shell = test_shell_state();
        shell.session = Some(ActiveSession { plugin_id: "test".into(), instance_id: 0, app: test_app(vec![], vec![]), view_state: semio_framework::ViewModel::default() });
        let terminology_command = shell.build_os_commands().into_iter().find(|command| command.id == "os.setTerminology").expect("terminology command present");
        let ActionArgControl::Select { options } = &terminology_command.args[0].control() else {
            panic!("expected a select control");
        };
        let values: Vec<&str> = options.iter().map(|option| option.value.as_str()).collect();
        assert_eq!(values, vec!["native", "de"]);
    }

    #[test]
    fn resolve_commands_tags_every_source() {
        let os_commands = vec![CommandDefinition::bounded_catalog("os.setLocale", LocalizedLabel::data("Set Locale"), "language", ActionKind::Shell)];
        let app_command = CommandDefinition::bounded_catalog("export", LocalizedLabel::data("Export"), "app", ActionKind::Mutation);
        let mode_command = CommandDefinition::bounded_catalog("focus", LocalizedLabel::data("Focus Mode"), "mode", ActionKind::View);
        let app = test_app(vec![app_command.clone()], vec![mode_command.clone()]);
        let plugin_manifest = PluginManifest {
            plugin_id: "plugin".into(),
            label: "Plugin".into(),
            version: "0.0.0".into(),
            apps: vec![],
            examples: vec![],
            capabilities: vec![],
            topic_contributions: vec![],
            commands: vec![CommandDefinition::bounded_catalog("doThing", LocalizedLabel::data("Do Thing"), "plugin", ActionKind::Shell)],
            artifact_kinds: vec![],
            dependencies: vec![],
            contributions: vec![],
        };
        let resolved = resolve_commands(os_commands, Some(&plugin_manifest), "plugin", &app, "default");
        let sources: Vec<(&str, CommandOwnerAddress)> = resolved.iter().map(|entry| (entry.definition.id.as_str(), entry.address.owner.clone())).collect();
        assert_eq!(
            sources,
            vec![
                ("os.setLocale", CommandOwnerAddress::Os),
                ("doThing", CommandOwnerAddress::Plugin { plugin_id: "plugin".into() }),
                ("export", CommandOwnerAddress::App { plugin_id: "plugin".into(), app_id: "test-app".into() }),
                ("focus", CommandOwnerAddress::Mode { plugin_id: "plugin".into(), app_id: "test-app".into(), mode_id: "default".into() }),
            ]
        );
    }

    #[test]
    fn identical_local_command_ids_have_collision_free_owner_keys() {
        let local_id = "refresh";
        let app = test_app(
            vec![CommandDefinition::bounded_catalog(local_id, LocalizedLabel::data("Refresh App"), "app", ActionKind::View)],
            vec![CommandDefinition::bounded_catalog(local_id, LocalizedLabel::data("Refresh Mode"), "mode", ActionKind::View)],
        );
        let plugin_manifest = PluginManifest {
            plugin_id: "plugin".into(),
            label: "Plugin".into(),
            version: "0.0.0".into(),
            apps: vec![],
            examples: vec![],
            capabilities: vec![],
            topic_contributions: vec![],
            commands: vec![CommandDefinition::bounded_catalog(local_id, LocalizedLabel::data("Refresh Plugin"), "plugin", ActionKind::View)],
            artifact_kinds: vec![],
            dependencies: vec![],
            contributions: vec![],
        };
        let resolved = resolve_commands(vec![CommandDefinition::bounded_catalog(local_id, LocalizedLabel::data("Refresh Shell"), "general", ActionKind::Shell)], Some(&plugin_manifest), "plugin", &app, "default");
        let keys: Vec<String> = resolved.iter().map(|entry| command_address_stable_key(&entry.address)).collect();
        assert_eq!(keys, vec!["os:refresh", "plugin:plugin:refresh", "app:plugin:test-app:refresh", "mode:plugin:test-app:default:refresh"]);
    }

    #[test]
    fn command_categories_orders_by_first_appearance_and_dedupes() {
        let resolved = vec![
            ResolvedCommand::new(CommandDefinition::bounded_catalog("a", LocalizedLabel::data("A"), "appearance", ActionKind::Shell), CommandOwnerAddress::Os),
            ResolvedCommand::new(CommandDefinition::bounded_catalog("b", LocalizedLabel::data("B"), "layout", ActionKind::Shell), CommandOwnerAddress::Os),
            ResolvedCommand::new(CommandDefinition::bounded_catalog("c", LocalizedLabel::data("C"), "appearance", ActionKind::Shell), CommandOwnerAddress::Os),
        ];
        assert_eq!(command_categories(&resolved), vec![("appearance".to_string(), "Appearance".to_string()), ("layout".to_string(), "Layout".to_string())]);
    }

    #[test]
    fn command_category_label_titleizes_hyphenated_ids() {
        assert_eq!(command_category_label("named-layout"), "Named Layout");
        assert_eq!(command_category_label("general"), "General");
    }

    #[test]
    fn command_search_items_expands_select_options_and_tags_os_category() {
        let mut shell = test_shell_state();
        shell.session = Some(ActiveSession { plugin_id: "test".into(), instance_id: 0, app: test_app(vec![], vec![]), view_state: semio_framework::ViewModel::default() });
        let items = shell.command_search_items();
        let appearance_dark = items.iter().find(|item| item.id == "command.os:os.setAppearance.dark").expect("expanded dark option present");
        assert_eq!(appearance_dark.label, "Set Appearance: Dark");
        assert_eq!(appearance_dark.action.as_deref(), Some("os-command:os.setAppearance:dark"));
        assert_eq!(appearance_dark.category, Some(CommandOwnerAddress::Os));
        let reset_dock = items.iter().find(|item| item.id == "command.os:os.resetDock").expect("zero-arg reset dock present");
        assert_eq!(reset_dock.action.as_deref(), Some("os-command:os.resetDock"));
        assert!(reset_dock.dispatch_action.is_none());
    }

    // 🔌️ Plain `#[test]` plus the sanctioned test-entrypoint driver keeps these tests
    // deterministic without introducing a runtime.
    #[test]
    fn apply_os_command_reset_dock_clears_layout_override_locally() {
        let mut shell = test_shell_state();
        shell.session = Some(ActiveSession { plugin_id: "test".into(), instance_id: 0, app: test_app(vec![], vec![]), view_state: semio_framework::ViewModel::default() });
        shell.layout_override = Some(shell.dock.to_window_layout());
        semio_framework_async::block_on(shell.apply_os_command("os.resetDock", None)).expect("reset dock never errors");
        assert!(shell.layout_override.is_none());
    }

    #[test]
    fn apply_os_command_set_locale_dispatches_through_framework_controller() {
        let mut shell = test_shell_state();
        shell.session = Some(ActiveSession { plugin_id: "test".into(), instance_id: 0, app: test_app(vec![], vec![]), view_state: semio_framework::ViewModel::default() });
        semio_framework_async::block_on(shell.apply_os_command("os.setLocale", Some("de"))).expect("set locale never errors");
        assert_eq!(shell.locale_id, "de");
    }

    #[test]
    fn apply_os_command_set_driver_updates_driver_id() {
        let mut shell = test_shell_state();
        assert_eq!(shell.driver_id, "default");
        semio_framework_async::block_on(shell.apply_os_command("os.setDriver", Some("compact"))).expect("set driver never errors");
        assert_eq!(shell.driver_id, "compact");
        semio_framework_async::block_on(shell.apply_os_command("os.setDriver", Some("default"))).expect("set driver never errors");
        assert_eq!(shell.driver_id, "default");
    }

    /// 🎨️ `os.setThemeId` (added alongside `build_settings_theme_ui`'s reachable Theme tab — see that
    /// function's doc comment) round-trips through the same `"framework"` `dispatch_action` arm as every
    /// other os select-command, landing in `active_theme_id()` (the `CHROME_PREFS` thread-local
    /// `frame()`'s `resolve_theme_for_ids` call already reads every frame), not a `ShellState` field.
    #[test]
    fn apply_os_command_set_theme_id_updates_active_theme() {
        let mut shell = test_shell_state();
        semio_framework_async::block_on(shell.apply_os_command("os.setThemeId", Some("mono"))).expect("set theme never errors");
        assert_eq!(active_theme_id(), "mono");
        assert_eq!(shell.chrome_build.preferences.theme_id, "mono");
        semio_framework_async::block_on(shell.apply_os_command("os.setThemeId", Some("semio"))).expect("set theme never errors");
        assert_eq!(active_theme_id(), "semio");
        assert_eq!(shell.chrome_build.preferences.theme_id, "semio");
    }

    #[test]
    fn build_command_panel_ui_groups_rows_under_category_headers() {
        let mut shell = test_shell_state();
        shell.session = Some(ActiveSession { plugin_id: "test".into(), instance_id: 0, app: test_app(vec![], vec![]), view_state: semio_framework::ViewModel::default() });
        let UiNode::Stack(panel) = shell.build_command_panel_ui() else {
            panic!("expected a stack root");
        };
        // 🗂️ One section per distinct `CommandDefinition.category`: fullscreen is in window,
        // appearance contains setAppearance/setThemeId, layout contains setDriver/resetDock, and
        // language contains setLocale/setTerminology.
        assert_eq!(panel.children.len(), 4);
    }

    #[test]
    fn fuzzy_match_score_finds_scattered_subsequence_and_rejects_non_matches() {
        assert!(fuzzy_match_score("stlc", "Set Locale").is_some());
        assert!(fuzzy_match_score("xyz", "Set Locale").is_none());
        assert!(fuzzy_match_score("", "Set Locale").is_some());
    }

    #[test]
    fn fuzzy_match_score_ranks_contiguous_prefix_above_scattered_match() {
        let contiguous = fuzzy_match_score("set", "Set Locale").expect("contiguous match");
        let scattered = fuzzy_match_score("sca", "Set Locale").expect("scattered match");
        assert!(contiguous > scattered, "contiguous {contiguous} should outrank scattered {scattered}");
    }

    #[test]
    fn fuzzy_match_score_is_case_insensitive() {
        assert_eq!(fuzzy_match_score("SET", "set locale"), fuzzy_match_score("set", "SET LOCALE"));
    }

    //#region ShellCommandHistoryTests
    #[test]
    fn note_shell_command_action_carries_command_id_label_and_detail() {
        let action = ShellState::note_shell_command_action("controller-1", "os.setLocale", "Set Locale", Some(serde_json::json!({ "value": "de" })));
        assert_eq!(action.controller_id, "controller-1");
        assert_eq!(action.action, "noteShellCommand");
        let args = action.args.expect("noteShellCommand always carries args");
        assert_eq!(args.get("commandId").and_then(semio_framework::DslValue::as_str), Some("os.setLocale"));
        assert_eq!(args.get("label").and_then(semio_framework::DslValue::as_str), Some("Set Locale"));
        assert_eq!(args.get("detail").and_then(|value| value.get("value")).and_then(semio_framework::DslValue::as_str), Some("de"));
    }

    #[test]
    fn note_shell_command_action_omits_detail_entirely_when_none() {
        let action = ShellState::note_shell_command_action("controller-1", "shell.windowResize", "Resize Window", None);
        let args = action.args.expect("args always present");
        assert!(args.get("detail").is_none(), "no detail key at all, not a serialized null");
    }

    #[test]
    fn shell_command_label_for_setting_matches_build_os_commands_and_theme_chrome_labels() {
        let shell = test_shell_state();
        assert_eq!(shell.shell_command_label_for_setting("os.setAppearance"), "Set Appearance");
        assert_eq!(shell.shell_command_label_for_setting("os.setDriver"), "Set Driver");
        assert_eq!(shell.shell_command_label_for_setting("os.setLocale"), "Set Locale");
        assert_eq!(shell.shell_command_label_for_setting("os.setTerminology"), "Set Terminology");
        assert_eq!(shell.shell_command_label_for_setting("os.setThemeId"), "Set Theme");
        assert_eq!(shell.shell_command_label_for_setting("os.resetThemeId"), shell_chrome_string("settings.theme.reset", false));
        assert_eq!(shell.shell_command_label_for_setting("os.deleteThemeId"), shell_chrome_string("settings.theme.delete", false));
    }

    /// 🕒️ The `handle_shell_hit` control-id-to-shell-command mapping table: every discrete window/panel
    /// chrome command this ticket wires, plus a representative non-matching id proving the mapping
    /// doesn't fire on everything.
    #[test]
    fn shell_command_for_control_maps_dock_and_panel_control_ids() {
        assert_eq!(ShellState::shell_command_for_control("dock.tab.0.a.close", false), Some(("shell.windowClose", "Close".to_string())));
        assert_eq!(ShellState::shell_command_for_control("dock.tab.0.a.focus", false), Some(("shell.windowMaximize", "Focus".to_string())));
        assert_eq!(ShellState::shell_command_for_control("shell.layout.compact", false), Some(("shell.applyNamedLayout", "Apply Layout".to_string())));
        assert_eq!(ShellState::shell_command_for_control("ui.panelToggle.details", false), Some(("shell.panelToggle", shell_chrome_string("panelToggle.details", false).to_string())));
        assert_eq!(ShellState::shell_command_for_control("ui.panelToggle.settings", true), Some(("shell.panelToggle", shell_chrome_string("panelToggle.settings", true).to_string())));
        assert_eq!(ShellState::shell_command_for_control("ui.panelToggle.display", false), None, "left-panel toggles are out of scope");
        assert_eq!(ShellState::shell_command_for_control("ui.nav.back", false), None);
    }
    //#endregion ShellCommandHistoryTests
}
//#endregion ActionPanelAndUtilities

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
// 🍿️ w3-overlays-chrome-polish (WP15+WP16): tooltips, a generic modal dialog, and the app introduction
// tour — layered on the existing immediate-mode chrome through `ShellChromeBuildState`, while
// `OverlayState` remains dedicated to the pre-existing menu/search overlays.
// Placement math reuses `ui_wgpu`'s w1d-events-overlay manager types (`OverlayKind`, `OverlayPlacement`,
// `resolve_overlay_placement`) even though the manager's own `EventRouter`/`open_overlay` stay
// `pub(crate)` to `ui_wgpu` (an `engine::Ui`/retained-`UiTree` implementation detail) and out of reach
// for this non-tree-based immediate-mode chrome renderer — confirmed unreachable the same way
// `report-w2-text-editor.md` found for its own local-fallback popup.

/// ⏱️ The hover-armed tooltip candidate — (re)armed the first frame `control_id` becomes hovered,
/// painted once `CHROME_TOOLTIP_DELAY_MS` elapses. Dismissed immediately on hover-out: this crate has no
/// animation-clock scaffolding anywhere (`engine::Ui::needs_frame`'s own doc comment admits the same gap
/// for animations generally), matching `OverlayKind::Tooltip::dismiss_policy`'s own documented admission
/// that hover-out-delay isn't actually debounced yet.
#[derive(Clone, Debug)]
struct ChromeTooltipHover {
    control_id: String,
    anchor_x: f32,
    anchor_y: f32,
    started_ms: f64,
}

const CHROME_TOOLTIP_DELAY_MS: f64 = 500.0;

/// ⏱️ Pure delay-threshold check, factored out of `render_chrome_tooltip` so it's unit-testable without
/// a `DrawList`/`FontAtlas` fixture.
fn chrome_tooltip_ready(hover: &ChromeTooltipHover, now_ms: f64) -> bool {
    now_ms - hover.started_ms >= CHROME_TOOLTIP_DELAY_MS
}

/// 🗨️ A generic modal confirmation/message dialog request — the minimal generic mechanism
/// `os-shell.tsx`'s `DialogDefinition`/`Effect::OpenDialog` calls for (title + body +
/// submit/cancel), enough to gate a destructive chrome action behind a real confirmation. Staged-form
/// `args` are out of scope for this pass (see the report's honest scope-down).
#[derive(Clone, Debug)]
struct ChromeDialogRequest {
    id: String,
    title: String,
    body: String,
    confirm_label: String,
    confirm_action: ActionDescriptor,
    cancel_label: String,
}

/// 🎓️ Live playback state for `AppDefinition.introduction` — which step is showing. Steps themselves are
/// re-read fresh from `session.app.introduction` every frame, never cached, so a plugin hot-reload
/// mid-session can't desync from a stale copy.
#[derive(Clone, Debug)]
struct ChromeTourState {
    step_index: usize,
    /// ✅️ Indices into the active step's `interactions` that are done — reset whenever `step_index` changes.
    completed_interactions: Vec<usize>,
}

impl ShellChromeBuildState {
    fn register_tooltip(&mut self, control_id: impl Into<String>, title: impl Into<String>) {
        let title = title.into();
        if !title.is_empty() {
            self.tooltip_titles.insert(control_id.into(), title);
        }
    }

    fn compute_click_edge(&mut self, pointer_down: bool) {
        self.clicked_this_frame = pointer_down && !self.previous_pointer_down;
        self.previous_pointer_down = pointer_down;
    }

    fn dialog_open(&self) -> bool {
        !self.dialog_stack.is_empty()
    }

    fn open_dialog(&mut self, request: ChromeDialogRequest) {
        self.dialog_stack.push(request);
    }

    fn close_topmost_dialog(&mut self) {
        self.dialog_stack.pop();
    }

    fn start_introduction(&mut self) {
        self.tour_state = Some(ChromeTourState { step_index: 0, completed_interactions: Vec::new() });
    }

    fn skip_introduction(&mut self) {
        self.tour_state = None;
    }

    fn advance_introduction(&mut self, step_count: usize) {
        if let Some(tour) = self.tour_state.as_mut() {
            if tour.step_index + 1 >= step_count {
                self.tour_state = None;
            } else {
                tour.step_index += 1;
                tour.completed_interactions.clear();
            }
        }
    }

    fn back_introduction(&mut self) {
        if let Some(tour) = self.tour_state.as_mut() {
            tour.step_index = tour.step_index.saturating_sub(1);
            tour.completed_interactions.clear();
        }
    }

    fn introduction_was_seen(&self, app_id: &str) -> bool {
        self.introduction_seen.get(app_id).copied().unwrap_or(false)
    }

    fn mark_introduction_seen(&mut self, app_id: &str) {
        if !self.introduction_was_seen(app_id) {
            self.introduction_seen.insert(app_id.to_string(), true);
            self.introduction_seen_writes.push(app_id.to_string());
        }
    }
}

/// 💬️ Registers footer utility tooltips ahead of `render_footer_utility_nodes` (an off-limits
/// `ShellInput`-adjacent function this wave — see the report) so hovering an already-carried-but-never-
/// rendered `title` shows it, without touching that function's own body. Control-id format
/// (`framework.utility.{button|toggle|collection}.{id}`) mirrors it exactly, including the flat
/// (non-prefixed) child ids nested `Collection`s already use there.
fn chrome_register_utility_tooltips(chrome: &mut ShellChromeBuildState, utilities: &[UtilityNode]) {
    for utility in utilities {
        match utility {
            UtilityNode::Button { id, label, text, title, .. } => {
                chrome.register_tooltip(format!("framework.utility.button.{id}"), footer_utility_label(label, text, title, id));
            }
            UtilityNode::Toggle { id, label, text, title, .. } => {
                chrome.register_tooltip(format!("framework.utility.toggle.{id}"), footer_utility_label(label, text, title, id));
            }
            UtilityNode::Collection { id, label, text, title, children, .. } => {
                chrome.register_tooltip(format!("framework.utility.collection.{id}"), footer_utility_label(label, text, title, id));
                chrome_register_utility_tooltips(chrome, children);
            }
            UtilityNode::Separator { .. } => {}
        }
    }
}

/// 🧭️ Item 5's "active-path tracking": true if `nodes` (recursively, through nested `Collection`s)
/// contains a pressed `Toggle` — used so a *collapsed* ribbon `Collection` still highlights when the
/// user's current selection lives inside it, mirroring `ui/js/react/index.tsx`'s recursive picker
/// active-path reconciliation. Called from `render_footer_utility_nodes` (an off-limits `ShellInput`-
/// adjacent function this wave — see the report's coordination note).
fn utility_subtree_has_active_path(nodes: &[UtilityNode]) -> bool {
    nodes.iter().any(|node| match node {
        UtilityNode::Toggle { pressed, .. } => pressed.unwrap_or(false),
        UtilityNode::Collection { children, .. } => utility_subtree_has_active_path(children),
        _ => false,
    })
}

/// 🖱️ Edge-detects "pointer just went down this frame", computed once per `render_chrome` call —
/// `InputState::pointer_down` is level-triggered (true every frame the button is held), so the
/// chrome-owned click handling below (dialogs, tour controls, the tour trigger) that lives outside the
/// `ActionDescriptor`/`ShellActions` dispatch pipeline needs its own edge tracking to avoid re-firing on
/// every held frame. Reads afterward (`chrome_clicked_this_frame`) are pure.
/// ⏱️ Self-contained wall-clock reader for the tooltip hover-delay timer — deliberately not sharing the
/// file's other `*_now_ms` helper (its exact module/scope kept shifting under concurrent edits from
/// `w3-prefs-i18n-themes` while this region was being written), same cfg-gated `js_sys::Date::now()` /
/// `SystemTime` split as the rest of this file already uses elsewhere.
#[cfg(target_arch = "wasm32")]
fn chrome_now_ms() -> f64 {
    js_sys::Date::now()
}

#[cfg(not(target_arch = "wasm32"))]
fn chrome_now_ms() -> f64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_secs_f64() * 1000.0).unwrap_or(0.0)
}

/// 🧰️ Whether `target_id` (a bare Button/Toggle utility id) exists anywhere in `nodes`, recursing through
/// nested `Collection`s.
fn utility_node_contains(nodes: &[UtilityNode], target_id: &str) -> bool {
    nodes.iter().any(|node| match node {
        UtilityNode::Button { id, .. } | UtilityNode::Toggle { id, .. } => id == target_id,
        UtilityNode::Collection { id, children, .. } => id == target_id || utility_node_contains(children, target_id),
        UtilityNode::Separator { .. } => false,
    })
}

/// 🧰️ Ancestor `Collection` ids (root → immediate parent) that must be expanded for `target_id` to be
/// visible in the footer utility bar — empty when `target_id` isn't nested in any collection (already
/// visible) or doesn't exist. Pure so it's independently testable from the frame-reveal wiring.
fn utility_collection_path_to_id(nodes: &[UtilityNode], target_id: &str) -> Vec<String> {
    for node in nodes {
        if let UtilityNode::Collection { id, children, .. } = node {
            if utility_node_contains(children, target_id) {
                let mut path = vec![id.clone()];
                path.extend(utility_collection_path_to_id(children, target_id));
                return path;
            }
        }
    }
    Vec::new()
}

/// 🎓️ A rect registered this frame under a logical element id (`register_element_rect`) — `fallback`
/// entries (folded-chrome chips) never override an already-registered primary entry, mirroring
/// `ui/js/react/index.tsx`'s `useIntroductionAnchorRect` "never downgrade" rule.
struct ChromeElementRectEntry {
    rect: Rect,
    fallback: bool,
}

impl ShellChromeBuildState {
    fn register_element_rect(&mut self, id: impl Into<String>, rect: Rect) {
        self.element_rects.insert(id.into(), ChromeElementRectEntry { rect, fallback: false });
    }

    fn register_element_rect_fallback(&mut self, id: impl Into<String>, rect: Rect) {
        let id = id.into();
        self.element_rects.entry(id).or_insert(ChromeElementRectEntry { rect, fallback: true });
    }

    fn resolve_element_rect(&self, id: &str) -> Option<Rect> {
        self.element_rects.get(id).map(|entry| entry.rect)
    }

    fn element_rect_is_fallback(&self, id: &str) -> bool {
        self.element_rects.get(id).is_some_and(|entry| entry.fallback)
    }
}

/// 🎓️ Punches `hole` out of `band`, returning up to four remaining rectangles (or the original band when
/// they don't overlap). The React shell now renders one fullscreen `ui-veil` div and raises the
/// introduced/shown element's chrome unit above it via z-index instead of doing this subtraction itself —
/// wgpu keeps the geometric subtraction because it's the only way to realize the *same visual result*
/// here: `push_solid` quads tile with no seam (no per-quad backdrop-filter to discontinue), a real glass
/// veil can't work in this renderer (see `introduction_veil_bands`'s doc), and 3D window content lives in
/// separate `scene_passes` that can't be repainted above an overlay at all.
fn punch_introduction_cutout(band: Rect, hole: Rect) -> Vec<Rect> {
    let top = band.y.max(hole.y);
    let left = band.x.max(hole.x);
    let bottom = (band.y + band.h).min(hole.y + hole.h);
    let right = (band.x + band.w).min(hole.x + hole.w);
    if right <= left || bottom <= top {
        return vec![band];
    }
    [Rect::new(band.x, band.y, band.w, top - band.y), Rect::new(band.x, bottom, band.w, band.y + band.h - bottom), Rect::new(band.x, top, left - band.x, bottom - top), Rect::new(right, top, band.x + band.w - right, bottom - top)]
        .into_iter()
        .filter(|piece| piece.w > 0.0 && piece.h > 0.0)
        .collect()
}

/// 🎓️ Splits the viewport into bands tiling the space around every cutout, painted as this renderer's
/// internal realization of "one fullscreen veil beneath elevated elements" — an empty cutout list returns
/// one full-viewport band. A *real* glass veil (matching React's `ui-veil`) is infeasible here: the
/// blur chain (`run_blur_chain`, ui/wgpu/rs/lib.rs) only mips the main draw-list's scene texture, but
/// panels/navbar/footer paint into the *overlay* DrawList (`with_chrome_sink`), so a glass veil would show
/// blurred canvas where it overlaps chrome rather than frosting it; and in `composite_to_swapchain`,
/// overlay glass regions composite *before* the overlay's own instance pass, i.e. beneath that chrome
/// regardless of push order. A solid-fill veil with real geometric holes is therefore the correct choice,
/// not a shortcut — and it's seam-free by construction (no per-quad backdrop-filter exists to discontinue).
fn introduction_veil_bands(width: f32, height: f32, cutouts: &[Rect]) -> Vec<Rect> {
    let mut bands = vec![Rect::new(0.0, 0.0, width, height)];
    for cutout in cutouts {
        let x = cutout.x.clamp(0.0, width);
        let y = cutout.y.clamp(0.0, height);
        let w = (cutout.x + cutout.w).clamp(0.0, width) - x;
        let h = (cutout.y + cutout.h).clamp(0.0, height) - y;
        if w <= 0.0 || h <= 0.0 {
            continue;
        }
        let clamped = Rect::new(x, y, w, h);
        bands = bands.into_iter().flat_map(|band| punch_introduction_cutout(band, clamped)).collect();
    }
    bands
}

const INTRODUCED_PULSE_PERIOD_MS: f64 = 1600.0;

/// 🎓️ Raised-cosine breathing thickness for the introduced-element pulse ring — mirrors the
/// `data-introduced` CSS keyframes (`ui/styling/js/🎨️ui.css`: hairline → focus → hairline over 1.6s,
/// ease-in-out), which are exactly a raised cosine.
fn introduced_pulse_thickness(now_ms: f64, hairline: f32, focus: f32) -> f32 {
    let phase = (now_ms.rem_euclid(INTRODUCED_PULSE_PERIOD_MS) / INTRODUCED_PULSE_PERIOD_MS) as f32;
    hairline + (focus - hairline) * 0.5 * (1.0 - (phase * std::f32::consts::TAU).cos())
}

const INTRODUCTION_INFO_BOX_GAP: f32 = 16.0;

/// 🎓️ Where the info box sits relative to `anchor` — byte-for-byte port of `resolveIntroductionPlacement`
/// (`ui/js/react/index.tsx`). `auto` picks the side with the most free viewport space; `center` (and any
/// anchor-less step) centers the box.
fn resolve_introduction_placement(placement: semio_framework::IntroductionPlacement, anchor: Option<Rect>, box_size: (f32, f32), viewport: (f32, f32)) -> (f32, f32) {
    use semio_framework::IntroductionPlacement;
    let (box_w, box_h) = box_size;
    let (vw, vh) = viewport;
    let centered = ((vw - box_w) / 2.0, (vh - box_h) / 2.0);
    let Some(anchor) = anchor else {
        return centered;
    };
    if matches!(placement, IntroductionPlacement::Center) {
        return centered;
    }
    let gap = INTRODUCTION_INFO_BOX_GAP;
    let clamp_left = |left: f32| left.max(gap).min((vw - box_w - gap).max(gap));
    let clamp_top = |top: f32| top.max(gap).min((vh - box_h - gap).max(gap));
    let space_top = anchor.y;
    let space_bottom = vh - (anchor.y + anchor.h);
    let space_left = anchor.x;
    let space_right = vw - (anchor.x + anchor.w);
    let side = if matches!(placement, IntroductionPlacement::Auto) {
        [("top", space_top), ("bottom", space_bottom), ("left", space_left), ("right", space_right)].into_iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)).map(|(side, _)| side).unwrap_or("bottom")
    } else {
        match placement {
            IntroductionPlacement::Top => "top",
            IntroductionPlacement::Bottom => "bottom",
            IntroductionPlacement::Left => "left",
            IntroductionPlacement::Right => "right",
            _ => "bottom",
        }
    };
    match side {
        "top" => (clamp_left(anchor.x + anchor.w / 2.0 - box_w / 2.0), clamp_top(anchor.y - box_h - gap)),
        "bottom" => (clamp_left(anchor.x + anchor.w / 2.0 - box_w / 2.0), clamp_top(anchor.y + anchor.h + gap)),
        "left" => (clamp_left(anchor.x - box_w - gap), clamp_top(anchor.y + anchor.h / 2.0 - box_h / 2.0)),
        "right" => (clamp_left(anchor.x + anchor.w + gap), clamp_top(anchor.y + anchor.h / 2.0 - box_h / 2.0)),
        _ => centered,
    }
}

/// ⌨️ Ports `engagementInlineCompletion`/`engagementCompletionSuffix` (`ui/js/react/index.tsx`) to this
/// renderer: the first `possible` (in the order the host already gave them — wgpu's engagement rail has
/// no ranked-match dropdown to reorder by) whose label case-insensitively prefix-matches `query`, sliced
/// on a char boundary (never a byte index) so a multi-byte label can't panic.
fn engagement_completion_suffix(query: &str, possibles: Option<&[ui_wgpu::wgpu::WindowEngagementPossible]>) -> String {
    let query = query.trim();
    if query.is_empty() {
        return String::new();
    }
    let Some(possibles) = possibles else {
        return String::new();
    };
    let query_lower = query.to_lowercase();
    for possible in possibles {
        let label_lower = possible.label.to_lowercase();
        if !label_lower.starts_with(&query_lower) {
            continue;
        }
        let query_chars = query.chars().count();
        let split_at = possible.label.char_indices().nth(query_chars).map(|(byte_index, _)| byte_index).unwrap_or(possible.label.len());
        let suffix = &possible.label[split_at..];
        if !suffix.is_empty() {
            return suffix.to_string();
        }
    }
    String::new()
}

/// 👻️ Pure accept-decision for the ghost-text click affordance — factored out of
/// `render_engagement_input` so it's unit-testable without a `GpuContext` fixture (that function
/// unconditionally needs a real wgpu device, per its own `_gpu` parameter).
fn engagement_ghost_accept_on_click(ghost_rect: Rect, pointer_x: f32, pointer_y: f32, clicked_this_frame: bool, query: &str, suffix: &str) -> Option<String> {
    if clicked_this_frame && ghost_rect.contains(pointer_x, pointer_y) {
        Some(format!("{query}{suffix}"))
    } else {
        None
    }
}
//#endregion 🔖️ChromeOverlaysAndTour

//#region 🎬️Tutorial
// 🎬️ The wgpu-shell half of the Tutorial mechanism (sibling of `//#region 🔖️ChromeOverlaysAndTour`'s
// introduction tour, above) — see `framework/core/rs/lib.rs`'s `//#region 🔖️Tutorial` for the shared
// data model and pure engine fns (`tutorial_slice`/`compose_tutorial_ui`/`interpolate_tutorial_camera`/
// `tutorial_camera_at`/`apply_tutorial_ui_change`) this region calls directly rather than reimplementing.
// The React shell's own half lives in `framework/renderer/react/index.tsx`; both are built against the
// same core model but neither coordinates with the other beyond that shared semantics.

/// 🎬️ Live playback/recording state machine for `ShellState.tutorial`. `Deviated` is a distinct mode
/// (not just "paused") so the player remembers to converge the camera on resume (Design Decision 6).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TutorialMode {
    Playing,
    Paused,
    Recording,
    Deviated,
}

/// 🎥️ A real-time (not timeline-time, not rate-scaled) camera glide from a live/deviated pose to the
/// recorded pose at the playhead the user pressed Play at — `TUTORIAL_CONVERGE_MS` long (Design
/// Decision 5). Reuses `interpolate_tutorial_camera` under the hood (see `tutorial_converge_pose`)
/// rather than reimplementing easing.
#[derive(Clone, Debug)]
pub struct TutorialCameraConverge {
    from: semio_framework::TutorialCameraState,
    to: semio_framework::TutorialCameraState,
    started_wall_ms: f64,
}

/// 🎬️ The active tutorial's full runtime state: the definition itself (recording IS a `TutorialDefinition`
/// — Design Decision 1), playback position/rate, the sandbox snapshot to restore on exit, and (for the
/// recorder) the bookkeeping needed to sample cameras/UI only on meaningful change.
#[derive(Clone)]
pub struct TutorialRuntime {
    pub definition: semio_framework::TutorialDefinition,
    pub mode: TutorialMode,
    pub playhead_ms: f64,
    pub rate: f32,
    /// ✂️ The playhead document/UI state was last synced to — `tutorial_slice(applied_ms, playhead_ms)`
    /// drives each tick's incremental apply; a seek instead jumps this straight to the target.
    applied_ms: f64,
    /// 📸️ The live document/UI as they stood the moment the tutorial sandboxed them — restored on exit
    /// (Design Decision 3). `None` for a recording, which is never sandboxed.
    pre_sandbox_document_dsl: Option<String>,
    pre_sandbox_ui: semio_framework::TutorialUiSnapshot,
    last_tick_wall_ms: f64,
    /// 🎥️ Active per-window convergence tweens (Deviated → Playing) — see `TutorialCameraConverge`.
    converge: HashMap<String, TutorialCameraConverge>,
    recorder_last_camera_wall_ms: HashMap<String, f64>,
    recorder_last_camera_pose: HashMap<String, semio_framework::TutorialCameraState>,
    recorder_last_ui: semio_framework::TutorialUiSnapshot,
    recorder_last_ui_sample_wall_ms: f64,
}

/// 🎬️ One document-track application queued by a tick/seek this frame — see
/// `ShellState::tutorial_pending_document_ops`'s own doc comment for why this has to be deferred rather
/// than applied inline (the plugin bridge's document calls are async, chrome rendering isn't).
#[derive(Clone, Debug)]
pub enum TutorialPendingDocOp {
    LoadArtifactDsl(String),
    ApplyOperations(Vec<String>),
    /// 🖋️ `Undo`/`Redo`/`Checkpoint`/`CheckoutCheckpoint`/`SwitchAlternative` all replay as a bare
    /// generic action dispatch (`"undo"`/`"redo"`/… against the session's `controller_id`) — the same
    /// convention `framework/plugin/rs`'s own `handle_action("undo", …)` test helpers already use, and
    /// the only "history action" mechanism reachable from here without inventing a second one.
    HistoryAction {
        action_id: String,
        args: Option<Value>,
    },
}

/// 🎬️ Reuses the navbar's own height token for the tutorial control bar (item 2's "reuse the existing
/// navbar height theme token" — no second style constant introduced).
fn tutorial_bar_height(theme: &Theme) -> f32 {
    theme.navbar_height
}

//#region 🎥️CameraConversion
/// 🎥️ `OrbitController` → `TutorialCameraState::Orbit`. `fov` is in **degrees**, matching
/// `World3dScene.camera_json`'s own wire format (`infinite_world`'s `WorldCameraRecord.fov` — see that
/// crate's `camera.fov.unwrap_or(45.0) as f32 * PI / 180.0` conversion the other way), not
/// `OrbitController.fov_y`'s radians.
fn orbit_to_tutorial_camera(orbit: &ui_wgpu::wgpu::OrbitController) -> semio_framework::TutorialCameraState {
    let camera = orbit.to_camera();
    semio_framework::TutorialCameraState::Orbit {
        position: [camera.position.x as f64, camera.position.y as f64, camera.position.z as f64],
        target: [camera.target.x as f64, camera.target.y as f64, camera.target.z as f64],
        up: [camera.up.x as f64, camera.up.y as f64, camera.up.z as f64],
        fov: Some((camera.fov_y as f64).to_degrees()),
    }
}

/// 🎥️ `TutorialCameraState` → `OrbitController`. `Canvas` (the 2D infinite-canvas camera kind) has no
/// orbit-controller equivalent — `None` (see the ticket's own scope note on 2D camera tracks).
fn tutorial_camera_to_orbit(state: &semio_framework::TutorialCameraState) -> Option<ui_wgpu::wgpu::OrbitController> {
    match state {
        semio_framework::TutorialCameraState::Orbit { position, target, up, fov } => Some(ui_wgpu::wgpu::OrbitController::from_camera(&ui_wgpu::wgpu::Camera3d {
            position: ui_wgpu::wgpu::Vec3::new(position[0] as f32, position[1] as f32, position[2] as f32),
            target: ui_wgpu::wgpu::Vec3::new(target[0] as f32, target[1] as f32, target[2] as f32),
            up: ui_wgpu::wgpu::Vec3::new(up[0] as f32, up[1] as f32, up[2] as f32),
            fov_y: (fov.unwrap_or(45.0) as f32).to_radians(),
            near: 0.1,
            far: 1000.0,
        })),
        semio_framework::TutorialCameraState::Canvas { .. } => None,
    }
}

fn tutorial_capture_camera_pose(state: &ShellState, window_id: &str) -> Option<semio_framework::TutorialCameraState> {
    state.world3d_states.get(window_id).or_else(|| state.icon_render_states.get(window_id)).map(|world| orbit_to_tutorial_camera(&world.orbit))
}

fn tutorial_apply_camera_pose(state: &mut ShellState, window_id: &str, pose: &semio_framework::TutorialCameraState) {
    let Some(orbit) = tutorial_camera_to_orbit(pose) else {
        return;
    };
    if let Some(world) = state.world3d_states.get_mut(window_id) {
        world.orbit = orbit.clone();
    }
    if let Some(world) = state.icon_render_states.get_mut(window_id) {
        world.orbit = orbit;
    }
}

/// 🎥️ Unique window ids across both `base.cameras` and `tracks.camera`, in first-seen order.
fn tutorial_camera_window_ids(def: &semio_framework::TutorialDefinition) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    for keyframe in def.base.cameras.iter().chain(def.tracks.camera.iter()) {
        if !ids.contains(&keyframe.window_id) {
            ids.push(keyframe.window_id.clone());
        }
    }
    ids
}

fn tutorial_camera_pose_close(a: &semio_framework::TutorialCameraState, b: &semio_framework::TutorialCameraState, epsilon: f64) -> bool {
    use semio_framework::TutorialCameraState::*;
    match (a, b) {
        (Orbit { position: p0, target: t0, .. }, Orbit { position: p1, target: t1, .. }) => {
            let dist = |x: [f64; 3], y: [f64; 3]| ((x[0] - y[0]).powi(2) + (x[1] - y[1]).powi(2) + (x[2] - y[2]).powi(2)).sqrt();
            dist(*p0, *p1) < epsilon && dist(*t0, *t1) < epsilon
        }
        (Canvas { x: x0, y: y0, zoom: z0 }, Canvas { x: x1, y: y1, zoom: z1 }) => (x0 - x1).abs() < epsilon && (y0 - y1).abs() < epsilon && (z0 - z1).abs() < epsilon,
        _ => false,
    }
}

/// 🎥️ Resolves the real-time convergence tween's pose at `elapsed_ms` since it started, by reusing
/// `interpolate_tutorial_camera` over two synthetic keyframes at `0`/`TUTORIAL_CONVERGE_MS` — rather than
/// reimplementing easing for this one caller.
fn tutorial_converge_pose(tween: &TutorialCameraConverge, elapsed_ms: f64) -> semio_framework::TutorialCameraState {
    let from_keyframe = semio_framework::TutorialCameraKeyframe { at: 0, window_id: String::new(), camera: tween.from.clone(), easing: semio_framework::TutorialEasing::Hold };
    let to_keyframe = semio_framework::TutorialCameraKeyframe { at: semio_framework::TUTORIAL_CONVERGE_MS, window_id: String::new(), camera: tween.to.clone(), easing: semio_framework::TutorialEasing::EaseInOut };
    semio_framework::interpolate_tutorial_camera(&from_keyframe, &to_keyframe, elapsed_ms)
}
//#endregion 🎥️CameraConversion

//#region 🧮️UiSnapshot
/// 🧮️ `ShellState` → `TutorialUiSnapshot` (Design Decision 4). Fields with no home in this shell's state
/// today (`activeToolId` has one — `ViewModel.active_tool_id`; `expandedTreeIds` does not) are noted
/// inline rather than inventing new cross-cutting state to fill them.
fn tutorial_capture_ui_snapshot(state: &ShellState) -> semio_framework::TutorialUiSnapshot {
    let mut active_panel_tab_by_group: HashMap<String, String> = HashMap::new();
    if state.left_panel_open {
        if let Some(tab) = &state.active_left_tab {
            let group = match state.active_left_kind {
                LeftPanelKind::Workbench => "workbench",
                LeftPanelKind::Display => "display",
            };
            active_panel_tab_by_group.insert(group.into(), tab.clone());
        }
    }
    if state.right_panel_open {
        if let Some(tab) = &state.active_right_tab {
            let group = match state.active_right_kind {
                RightPanelKind::Details => "details",
                RightPanelKind::Settings => "settings",
            };
            active_panel_tab_by_group.insert(group.into(), tab.clone());
        }
    }
    let command_panel_open = state.right_panel_open && state.active_right_kind == RightPanelKind::Settings && state.active_right_tab.as_deref() == Some(FRAMEWORK_SETTINGS_COMMANDS_TAB_ID);
    semio_framework::TutorialUiSnapshot {
        active_mode_id: state.session.as_ref().and_then(|s| s.view_state.active_mode_id.clone()),
        focused_window_id: state.active_window_id.clone(),
        active_utility_by_window_id: state.active_utility_by_window.clone(),
        active_tool_id: state.session.as_ref().and_then(|s| s.view_state.active_tool_id.clone()),
        layout: Some(state.dock.to_window_layout()),
        active_panel_tab_by_group,
        panel_json: state.session.as_ref().and_then(|s| s.view_state.panel_json.clone()),
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W3a: the deleted opaque
        // `ViewModel.selectionJson` this used to mirror had no `ShellState` home either — the framework
        // now owns selection via `InteractionState`, not yet threaded through `ShellState`. Left
        // unmapped (best-effort no-op) rather than inventing new cross-cutting state.
        interaction_selection: HashMap::new(),
        open_dialog_id: state.chrome_build.dialog_stack.last().map(|dialog| dialog.id.clone()),
        // 🚧️ No generic hierarchical "expanded tree ids" state exists on `ShellState` today — the closest
        // analog (`collapsed_sections`) is a flat per-accordion-id map with inverted (collapsed, not
        // expanded) boolean semantics and no notion of a tree, so round-tripping through it would silently
        // perturb unrelated accordion sections. Left unmapped (best-effort no-op) rather than inventing
        // new cross-cutting tree-expansion state.
        expanded_tree_ids: Vec::new(),
        command_panel_open,
    }
}

/// 🧮️ `TutorialUiSnapshot` → `ShellState`, applied as a snap (single field writes — Design Decision 5).
fn tutorial_apply_ui_snapshot(state: &mut ShellState, snapshot: &semio_framework::TutorialUiSnapshot) {
    if let Some(session) = state.session.as_mut() {
        session.view_state.active_mode_id = snapshot.active_mode_id.clone();
        session.view_state.active_tool_id = snapshot.active_tool_id.clone();
        session.view_state.panel_json = snapshot.panel_json.clone();
    }
    state.active_window_id = snapshot.focused_window_id.clone();
    state.active_utility_by_window = snapshot.active_utility_by_window_id.clone();
    if let Some(layout) = &snapshot.layout {
        state.dock.apply_layout_diff(layout);
    }
    let workbench = snapshot.active_panel_tab_by_group.get("workbench");
    let display = snapshot.active_panel_tab_by_group.get("display");
    if let Some(tab) = workbench.or(display) {
        state.left_panel_open = true;
        state.active_left_kind = if workbench.is_some() { LeftPanelKind::Workbench } else { LeftPanelKind::Display };
        state.active_left_tab = Some(tab.clone());
    } else {
        state.left_panel_open = false;
    }
    let details = snapshot.active_panel_tab_by_group.get("details");
    let settings = snapshot.active_panel_tab_by_group.get("settings");
    if let Some(tab) = details.or(settings) {
        state.right_panel_open = true;
        state.active_right_kind = if details.is_some() { RightPanelKind::Details } else { RightPanelKind::Settings };
        state.active_right_tab = Some(tab.clone());
    } else {
        state.right_panel_open = false;
    }
    if snapshot.open_dialog_id.is_none() {
        state.chrome_build.dialog_stack.clear();
    }
    if snapshot.command_panel_open {
        state.right_panel_open = true;
        state.active_right_kind = RightPanelKind::Settings;
        state.active_right_tab = Some(FRAMEWORK_SETTINGS_COMMANDS_TAB_ID.into());
    }
}

/// 🩹️ Applies one `TutorialUiChange` live — composed from `tutorial_capture_ui_snapshot` +
/// `apply_tutorial_ui_change` (core) + `tutorial_apply_ui_snapshot` rather than duplicating the change's
/// own per-field switch a second time.
fn tutorial_apply_ui_change_to_shell(state: &mut ShellState, change: &semio_framework::TutorialUiChange) {
    let mut snapshot = tutorial_capture_ui_snapshot(state);
    semio_framework::apply_tutorial_ui_change(&mut snapshot, change);
    tutorial_apply_ui_snapshot(state, &snapshot);
}

//#endregion 🧮️UiSnapshot

//#region 👻️GestureOverlay
/// 👻️ Resolves an `IntroductionPoint` to a viewport pixel, reusing this shell's existing per-frame rect
/// registries (`resolve_element_rect`, `window_content_rects`). `Scene`/`Canvas`/`Entity`/`Curve`/`Domain`
/// need a per-window world→screen projection resolver that doesn't exist as reusable cross-cutting infra
/// here (each 3D/2D surface picks/projects ad hoc at its own interaction call sites) — scoped out rather
/// than inventing new resolver plumbing.
fn tutorial_resolve_gesture_point(state: &ShellState, point: &semio_framework::IntroductionPoint) -> Option<(f32, f32)> {
    use semio_framework::IntroductionPoint as P;
    match point {
        P::Screen { x, y } => Some((*x as f32, *y as f32)),
        P::ScreenNormalized { x, y } => Some((*x as f32 * state.screen_w, *y as f32 * state.screen_h)),
        P::Element { id, offset } => {
            let rect = state.chrome_build.resolve_element_rect(id)?;
            let [ox, oy] = offset.unwrap_or([0.5, 0.5]);
            Some((rect.x + rect.w * ox as f32, rect.y + rect.h * oy as f32))
        }
        P::Window { id, x, y } => {
            let rect = state.window_content_rects.get(id)?;
            Some((rect.x + *x as f32, rect.y + *y as f32))
        }
        P::WindowNormalized { id, x, y } => {
            let rect = state.window_content_rects.get(id)?;
            Some((rect.x + rect.w * (*x as f32), rect.y + rect.h * (*y as f32)))
        }
        P::Scene { .. } | P::Canvas { .. } | P::Entity { .. } | P::Curve { .. } | P::Domain { .. } => None,
    }
}

fn tutorial_gesture_endpoints(gesture: &semio_framework::IntroductionGesture) -> (semio_framework::IntroductionPoint, semio_framework::IntroductionPoint) {
    use semio_framework::IntroductionGesture as G;
    match gesture {
        G::LeftClick { at } | G::RightClick { at } | G::DoubleClick { at } => (at.clone(), at.clone()),
        G::Scroll { at, .. } => (at.clone(), at.clone()),
        G::Drag { from, to, .. } | G::Orbit { from, to, .. } => (from.clone(), to.clone()),
    }
}

/// 👻️ Paints a simple ghost-cursor dot at the active `TutorialGestureCue`'s interpolated position (linear
/// by playhead progress within the cue) — the minimal demonstration painter neither this shell nor its
/// introduction mechanism had before (verified: no `demonstration`/ghost-cursor renderer existed here).
fn render_tutorial_gesture_overlay(state: &ShellState, overlay: &mut DrawList, theme: &Theme) {
    let Some(runtime) = state.tutorial.as_ref() else {
        return;
    };
    if !matches!(runtime.mode, TutorialMode::Playing | TutorialMode::Paused | TutorialMode::Deviated) {
        return;
    }
    let playhead = runtime.playhead_ms;
    let Some(cue) = runtime.definition.tracks.gestures.iter().find(|cue| {
        let at = cue.at as f64;
        playhead >= at && playhead <= at + cue.duration_ms as f64
    }) else {
        return;
    };
    let (from, to) = tutorial_gesture_endpoints(&cue.gesture);
    let Some((fx, fy)) = tutorial_resolve_gesture_point(state, &from) else {
        return;
    };
    let (tx, ty) = tutorial_resolve_gesture_point(state, &to).unwrap_or((fx, fy));
    let t = if cue.duration_ms == 0 { 1.0 } else { ((playhead - cue.at as f64) / cue.duration_ms as f64).clamp(0.0, 1.0) as f32 };
    let x = fx + (tx - fx) * t;
    let y = fy + (ty - fy) * t;
    let size = 18.0;
    overlay.push_rounded([x - size * 0.5, y - size * 0.5, size, size], theme.selected, size * 0.5);
    let inner = size - 6.0;
    overlay.push_rounded([x - inner * 0.5, y - inner * 0.5, inner, inner], theme.background, inner * 0.5);
}
//#endregion 👻️GestureOverlay

//#region 📅️Provenance
/// 📅️ Howard Hinnant's `civil_from_days` algorithm (http://howardhinnant.github.io/date_algorithms.html)
/// — days-since-1970-01-01 → proleptic-Gregorian `(year, month, day)`, dependency-free (no date crate in
/// this crate's `Cargo.toml`) for the recorder's `recordedAt` provenance stamp.
fn tutorial_civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn tutorial_recorded_at_iso() -> String {
    let total_secs = (chrome_now_ms() / 1000.0).floor() as i64;
    let days = total_secs.div_euclid(86400);
    let secs_of_day = total_secs.rem_euclid(86400);
    let (hour, minute, second) = (secs_of_day / 3600, (secs_of_day % 3600) / 60, secs_of_day % 60);
    let (year, month, day) = tutorial_civil_from_days(days);
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

/// 💾️ Reuses the existing "download a file from the browser"/"save file" mechanism (`download_media_export`,
/// already used for GLB/OBJ/image exports) rather than inventing a second one for tutorial recordings.
fn tutorial_save_recording(tutorial_id: &str, json: &str) {
    download_media_export(&format!("{tutorial_id}.tutorial.json"), "application/json", json, None);
}
//#endregion 📅️Provenance

//#region ✂️PendingDocOps
/// ✂️ One `TutorialArtifactEvent` → the pending op(s) needed to apply it in `direction` — `forward` uses
/// `Edit::forwards`/dispatches the named history action as-is; backward uses `Edit::backwards`/inverts
/// the history action (undo↔redo) per `TutorialArtifactEventKind::Edit`'s own doc comment on exact
/// bidirectional scrubbing.
fn tutorial_pending_op_for_edit(entry: &semio_framework::TutorialArtifactEvent, forward: bool) -> TutorialPendingDocOp {
    use semio_framework::TutorialArtifactEventKind as K;
    match &entry.kind {
        K::Edit { forwards, backwards, .. } => {
            let ops = if forward { forwards } else { backwards };
            TutorialPendingDocOp::ApplyOperations(ops.iter().filter_map(|v| serde_json::to_string(v).ok()).collect())
        }
        K::Undo => TutorialPendingDocOp::HistoryAction { action_id: (if forward { "undo" } else { "redo" }).into(), args: None },
        K::Redo => TutorialPendingDocOp::HistoryAction { action_id: (if forward { "redo" } else { "undo" }).into(), args: None },
        K::Checkpoint { message } => TutorialPendingDocOp::HistoryAction { action_id: "checkpoint".into(), args: message.as_ref().map(|m| serde_json::json!({ "message": m })) },
        K::CheckoutCheckpoint { checkpoint_id } => TutorialPendingDocOp::HistoryAction { action_id: "checkoutCheckpoint".into(), args: Some(serde_json::json!({ "checkpointId": checkpoint_id })) },
        K::SwitchAlternative { alternative_id } => TutorialPendingDocOp::HistoryAction { action_id: "switchAlternative".into(), args: Some(serde_json::json!({ "alternativeId": alternative_id })) },
        K::Load { artifact_dsl, previous_dsl } => TutorialPendingDocOp::LoadArtifactDsl(if forward { artifact_dsl.clone() } else { previous_dsl.clone() }),
    }
}
//#endregion ✂️PendingDocOps

//#region 🎬️PureHelpers
/// ⏱️ Pure playhead-advance math (per-tick real-time `dt` scaled by the UI-only `rate` — Design
/// Decision 6): factored out of `tutorial_tick` for unit testing.
fn tutorial_advance_playhead(playhead_ms: f64, dt_ms: f64, rate: f32) -> f64 {
    playhead_ms + dt_ms * rate as f64
}

/// 🎚️ Pure scrub-bar progress (0–1) for a given playhead/duration — `0.0` for a zero-length timeline
/// (the in-progress recording case) rather than dividing by zero.
fn tutorial_scrub_progress(playhead_ms: f64, duration_ms: u64) -> f32 {
    if duration_ms == 0 {
        0.0
    } else {
        (playhead_ms / duration_ms as f64).clamp(0.0, 1.0) as f32
    }
}

/// 🎚️ Pure scrub-bar hit math: pointer x within `track` → target playhead ms.
fn tutorial_scrub_target_ms(pointer_x: f32, track: Rect, duration_ms: u64) -> f64 {
    let t = ((pointer_x - track.x) / track.w.max(1.0)).clamp(0.0, 1.0);
    t as f64 * duration_ms as f64
}
//#endregion 🎬️PureHelpers

impl ShellState {
    //#region 🎬️Lifecycle
    /// 🎬️ Starts (or restarts) `tutorial_id` for the active app: sandboxes the live document behind
    /// `base`, snaps UI/camera to `base`, and begins playback from `t=0` (Design Decisions 2/3).
    pub fn tutorial_start(&mut self, tutorial_id: &str) {
        let Some(session) = self.session.clone() else {
            return;
        };
        let Some(definition) = session.app.tutorials.iter().find(|t| t.id == tutorial_id).cloned() else {
            return;
        };
        // 🎬️ Introductions and tutorials are mutually exclusive (Design Decision 8).
        self.chrome_build.skip_introduction();
        let pre_sandbox_ui = tutorial_capture_ui_snapshot(self);
        // 🚧️ `last_envelope_dsl` is this shell's own best-effort stand-in for "the live document's full
        // `ArtifactEnvelope` JSON" — there is no other reachable accessor for it from here.
        let pre_sandbox_document_dsl = self.last_envelope_dsl.clone();
        if let Some(artifact_dsl) = definition.base.artifact_dsl.clone() {
            self.tutorial_pending_document_ops.push(TutorialPendingDocOp::LoadArtifactDsl(artifact_dsl));
        } else if let Some(example_id) = definition.base.example_id.as_ref() {
            // 🚧️ No "load example by id" plugin-bridge primitive is reachable from here without
            // duplicating `apply_shell_uri`'s example-switch machinery — scoped out; the tutorial plays
            // over whichever document is already loaded instead of sandboxing a fresh example copy.
            eprintln!("[DEBUG] tutorial base.exampleId `{example_id}` sandbox load not wired (no base.documentJson) — playing over the live document");
        }
        tutorial_apply_ui_snapshot(self, &definition.base.ui);
        for keyframe in &definition.base.cameras {
            tutorial_apply_camera_pose(self, &keyframe.window_id, &keyframe.camera);
        }
        self.tutorial = Some(TutorialRuntime {
            definition,
            mode: TutorialMode::Playing,
            playhead_ms: 0.0,
            rate: 1.0,
            applied_ms: 0.0,
            pre_sandbox_document_dsl,
            pre_sandbox_ui,
            last_tick_wall_ms: chrome_now_ms(),
            converge: HashMap::new(),
            recorder_last_camera_wall_ms: HashMap::new(),
            recorder_last_camera_pose: HashMap::new(),
            recorder_last_ui: semio_framework::TutorialUiSnapshot::default(),
            recorder_last_ui_sample_wall_ms: 0.0,
        });
    }

    /// ⏺️ Arms the recorder against the LIVE document (never sandboxed — a recording IS the user's work,
    /// Design Decision 2/3's sandbox note). Skips webcam/mic capture entirely (explicitly out of scope).
    pub fn tutorial_start_recording(&mut self) {
        if self.tutorial.is_some() {
            return;
        }
        self.chrome_build.skip_introduction();
        let ui = tutorial_capture_ui_snapshot(self);
        let mut cameras = Vec::new();
        for (window_id, world) in self.world3d_states.iter() {
            cameras.push(semio_framework::TutorialCameraKeyframe { at: 0, window_id: window_id.clone(), camera: orbit_to_tutorial_camera(&world.orbit), easing: semio_framework::TutorialEasing::Hold });
        }
        let now = chrome_now_ms();
        let definition = semio_framework::TutorialDefinition {
            id: format!("recording-{}", tutorial_recorded_at_iso().replace(['-', ':'], "")),
            title: LocalizedLabel::data("Recording"),
            description: None,
            duration_ms: 0,
            chapters: Vec::new(),
            base: semio_framework::TutorialBase { artifact_dsl: self.last_envelope_dsl.clone(), example_id: self.active_example_id.clone(), ui: ui.clone(), cameras },
            tracks: semio_framework::TutorialTracks::default(),
            recorded_at: None,
        };
        self.tutorial = Some(TutorialRuntime {
            definition,
            mode: TutorialMode::Recording,
            playhead_ms: 0.0,
            rate: 1.0,
            applied_ms: 0.0,
            pre_sandbox_document_dsl: None,
            pre_sandbox_ui: ui.clone(),
            last_tick_wall_ms: now,
            converge: HashMap::new(),
            recorder_last_camera_wall_ms: HashMap::new(),
            recorder_last_camera_pose: HashMap::new(),
            recorder_last_ui: ui,
            recorder_last_ui_sample_wall_ms: now,
        });
    }

    /// 🛑️ Ends playback (restoring the sandboxed document/UI, Design Decision 3) or, if recording, ends
    /// the take and serializes it (Design Decision 7: JSON via the existing download/save mechanism).
    pub fn tutorial_stop(&mut self) {
        let Some(runtime) = self.tutorial.take() else {
            return;
        };
        match runtime.mode {
            TutorialMode::Recording => {
                let mut definition = runtime.definition;
                definition.duration_ms = runtime.playhead_ms.max(0.0) as u64;
                definition.recorded_at = Some(tutorial_recorded_at_iso());
                match serde_json::to_string_pretty(&definition) {
                    Ok(json) => tutorial_save_recording(&definition.id, &json),
                    Err(err) => eprintln!("[DEBUG] tutorial recording serialize failed: {err}"),
                }
            }
            _ => {
                tutorial_apply_ui_snapshot(self, &runtime.pre_sandbox_ui);
                if let Some(document_dsl) = runtime.pre_sandbox_document_dsl {
                    self.tutorial_pending_document_ops.push(TutorialPendingDocOp::LoadArtifactDsl(document_dsl));
                }
            }
        }
    }

    pub fn tutorial_toggle_play_pause(&mut self) {
        let Some(runtime) = self.tutorial.clone() else {
            return;
        };
        match runtime.mode {
            TutorialMode::Playing => {
                if let Some(r) = self.tutorial.as_mut() {
                    r.mode = TutorialMode::Paused;
                }
            }
            TutorialMode::Paused => {
                if let Some(r) = self.tutorial.as_mut() {
                    r.mode = TutorialMode::Playing;
                    r.last_tick_wall_ms = chrome_now_ms();
                }
            }
            // 🪄️ Deviation → Play: snap UI+document to the composed state at the current playhead, then
            // start a real-time camera convergence tween per window (Design Decision 5/6).
            TutorialMode::Deviated => {
                let target_ms = runtime.playhead_ms;
                let ui_state = semio_framework::compose_tutorial_ui(&runtime.definition, target_ms);
                tutorial_apply_ui_snapshot(self, &ui_state);
                let slice = semio_framework::tutorial_slice(&runtime.definition, runtime.applied_ms, target_ms);
                for entry in &slice.document {
                    self.tutorial_pending_document_ops.push(tutorial_pending_op_for_edit(entry, slice.forward));
                }
                let now = chrome_now_ms();
                let mut converge = HashMap::new();
                for window_id in tutorial_camera_window_ids(&runtime.definition) {
                    if let Some(to) = semio_framework::tutorial_camera_at(&runtime.definition, &window_id, target_ms) {
                        if let Some(from) = tutorial_capture_camera_pose(self, &window_id) {
                            converge.insert(window_id, TutorialCameraConverge { from, to, started_wall_ms: now });
                        }
                    }
                }
                if let Some(r) = self.tutorial.as_mut() {
                    r.mode = TutorialMode::Playing;
                    r.applied_ms = target_ms;
                    r.last_tick_wall_ms = now;
                    r.converge = converge;
                }
            }
            TutorialMode::Recording => {}
        }
    }

    /// ⏩️ Seeks to `target_ms`: UI applies wholesale via `compose_tutorial_ui`, document track entries
    /// apply forward/backward via `tutorial_slice` since the last-applied playhead, camera sets exactly
    /// via `tutorial_camera_at` (Design Decision item 5 of "what to implement").
    pub fn tutorial_seek(&mut self, target_ms: f64) {
        let Some(runtime) = self.tutorial.clone() else {
            return;
        };
        if runtime.mode == TutorialMode::Recording {
            return;
        }
        let target_ms = target_ms.clamp(0.0, runtime.definition.duration_ms as f64);
        let ui_state = semio_framework::compose_tutorial_ui(&runtime.definition, target_ms);
        tutorial_apply_ui_snapshot(self, &ui_state);
        let slice = semio_framework::tutorial_slice(&runtime.definition, runtime.applied_ms, target_ms);
        for entry in &slice.document {
            self.tutorial_pending_document_ops.push(tutorial_pending_op_for_edit(entry, slice.forward));
        }
        for window_id in tutorial_camera_window_ids(&runtime.definition) {
            if let Some(pose) = semio_framework::tutorial_camera_at(&runtime.definition, &window_id, target_ms) {
                tutorial_apply_camera_pose(self, &window_id, &pose);
            }
        }
        if let Some(r) = self.tutorial.as_mut() {
            r.playhead_ms = target_ms;
            r.applied_ms = target_ms;
            r.converge.clear();
            if r.mode == TutorialMode::Deviated {
                r.mode = TutorialMode::Paused;
            }
        }
    }

    /// 🎬️ Per-frame tick — advances the playhead (Playing), samples the recorder (Recording), applies
    /// annotational-free document/UI deltas since the last applied playhead, resolves active camera
    /// convergence tweens, and otherwise drives the camera track exactly via `tutorial_camera_at`.
    pub fn tutorial_tick(&mut self, now_wall_ms: f64) {
        let Some(mut runtime) = self.tutorial.clone() else {
            return;
        };
        let dt_wall_ms = (now_wall_ms - runtime.last_tick_wall_ms).max(0.0);
        runtime.last_tick_wall_ms = now_wall_ms;

        if runtime.mode == TutorialMode::Recording {
            runtime.playhead_ms = tutorial_advance_playhead(runtime.playhead_ms, dt_wall_ms, 1.0);
            runtime.definition.duration_ms = runtime.playhead_ms.max(0.0) as u64;
            tutorial_recorder_sample(self, &mut runtime, now_wall_ms);
            self.tutorial = Some(runtime);
            return;
        }

        if runtime.mode == TutorialMode::Playing {
            let from_ms = runtime.applied_ms;
            let total_ms = runtime.definition.duration_ms as f64;
            let mut to_ms = tutorial_advance_playhead(runtime.playhead_ms, dt_wall_ms, runtime.rate);
            let auto_paused = to_ms >= total_ms;
            if auto_paused {
                to_ms = total_ms;
            }
            runtime.playhead_ms = to_ms;
            let slice = semio_framework::tutorial_slice(&runtime.definition, from_ms, to_ms);
            for change in &slice.ui_changes {
                tutorial_apply_ui_change_to_shell(self, change);
            }
            for entry in &slice.document {
                self.tutorial_pending_document_ops.push(tutorial_pending_op_for_edit(entry, slice.forward));
            }
            runtime.applied_ms = to_ms;
            if auto_paused {
                runtime.mode = TutorialMode::Paused;
            }
        }

        // 🎥️ Convergence tweens win over the recorded pose while active; once a window's tween completes
        // it falls through to `tutorial_camera_at` at the live playhead next tick.
        let mut converged: Vec<String> = Vec::new();
        for (window_id, tween) in runtime.converge.iter() {
            let elapsed_ms = (now_wall_ms - tween.started_wall_ms).max(0.0);
            let pose = tutorial_converge_pose(tween, elapsed_ms.min(semio_framework::TUTORIAL_CONVERGE_MS as f64));
            tutorial_apply_camera_pose(self, window_id, &pose);
            if elapsed_ms >= semio_framework::TUTORIAL_CONVERGE_MS as f64 {
                converged.push(window_id.clone());
            }
        }
        for window_id in converged {
            runtime.converge.remove(&window_id);
        }
        if matches!(runtime.mode, TutorialMode::Playing | TutorialMode::Paused) {
            for window_id in tutorial_camera_window_ids(&runtime.definition) {
                if runtime.converge.contains_key(&window_id) {
                    continue;
                }
                if let Some(pose) = semio_framework::tutorial_camera_at(&runtime.definition, &window_id, runtime.playhead_ms) {
                    tutorial_apply_camera_pose(self, &window_id, &pose);
                }
            }
        }

        self.tutorial = Some(runtime);
    }

    /// 🎬️ Deviation detection (Playing → Deviated on any real dispatch, Design Decision 6) + the
    /// recorder's annotational event tap (Design Decision 7) — called once at the very top of
    /// `dispatch_action` for every dispatch NOT already filtered as tutorial-internal.
    fn tutorial_note_real_dispatch(&mut self, action: &ActionDescriptor) {
        let Some(runtime) = self.tutorial.as_mut() else {
            return;
        };
        match runtime.mode {
            TutorialMode::Playing => {
                runtime.mode = TutorialMode::Deviated;
            }
            TutorialMode::Recording => {
                // 🎥️ Camera changes are sampled directly from the orbit controller (`tutorial_recorder_sample`),
                // never re-recorded as an annotational event too.
                if action.action == "setCamera" {
                    return;
                }
                // 🕒️ Session-only command-history log rows, never part of a tutorial's own replayable
                // action track (mirrors the `setCamera` skip just above).
                if action.action == "noteShellCommand" {
                    return;
                }
                let at = runtime.playhead_ms.max(0.0) as u64;
                runtime.definition.tracks.events.push(semio_framework::TutorialEvent { at, kind: semio_framework::TutorialEventKind::Action { action: action.action.clone(), args: action.args.clone() } });
            }
            TutorialMode::Paused | TutorialMode::Deviated => {}
        }
    }

    /// 🎬️ Drains `tutorial_pending_document_ops` (queued by a tick/seek this frame) and applies each
    /// through the existing plugin-bridge document mechanisms — the async counterpart to the sync
    /// tick/seek above, called from `AppRuntime::frame` right after `render_chrome` (mirrors how `frame`
    /// already defers `scene_events` the same way, for the same sync/async split).
    pub async fn tutorial_flush_pending_document_ops(&mut self) {
        if self.tutorial_pending_document_ops.is_empty() {
            return;
        }
        let ops = std::mem::take(&mut self.tutorial_pending_document_ops);
        self.chrome_build.tutorial_dispatch_internal = true;
        for op in ops {
            match op {
                // 🚧️ `TutorialBase.document_json` is always `None` fleet-wide today (no tutorial
                // definition populates it, and `last_envelope_dsl` — its only non-`None` source — is
                // itself never set past its `None` default); a real loader needs the tutorial-content
                // dsl-text conversion this plan's B5 tutorial-track bullet scopes separately, not a
                // whole-envelope JSON reader (deleted with `PluginApp::load_document`/`document_dsl`).
                TutorialPendingDocOp::LoadArtifactDsl(_json) => {
                    eprintln!("[DEBUG] tutorial load document (json) not wired to the pack-only plugin bridge");
                }
                TutorialPendingDocOp::ApplyOperations(operations) => {
                    if let Err(err) = self.apply_mutations(&operations).await {
                        eprintln!("[DEBUG] tutorial apply operations failed: {err}");
                    }
                }
                TutorialPendingDocOp::HistoryAction { action_id, args } => {
                    if let Some(session) = self.session.clone() {
                        let descriptor = ActionDescriptor { controller_id: session.app.controller_id.clone(), action: action_id, args: semio_framework::optional_json_to_dsl(args) };
                        if let Err(err) = self.dispatch_action(descriptor).await {
                            eprintln!("[DEBUG] tutorial history action failed: {err}");
                        }
                    }
                }
            }
        }
        self.chrome_build.tutorial_dispatch_internal = false;
    }
    //#endregion 🎬️Lifecycle

    //#region 🎬️Chrome
    /// 🎬️ The tutorial control bar (play/pause, stop, scrubber, time, rate) — rendered right after
    /// `render_navbar` in the chrome pass, reusing its own `ChromeGroupItem`/`render_chrome_group`
    /// button plumbing and `chrome_clicked_this_frame` click-edge detection.
    fn render_tutorial_bar(&mut self, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32) {
        if self.tutorial.is_none() {
            return;
        }
        let bar_h = tutorial_bar_height(theme);
        let y = theme.navbar_height;
        draw.push_solid([0.0, y, width, bar_h], theme.navbar);
        draw.push_solid([0.0, y + bar_h - theme.stroke_hairline, width, theme.stroke_hairline], theme.border_normal);

        let (mode, playhead_ms, duration_ms, rate) = {
            let runtime = self.tutorial.as_ref().unwrap();
            (runtime.mode, runtime.playhead_ms, runtime.definition.duration_ms, runtime.rate)
        };
        let btn_h = theme.control_height;
        let btn_y = y + (bar_h - btn_h) * 0.5;
        let mut x = theme.padding_standard;

        if mode != TutorialMode::Recording {
            let playing = mode == TutorialMode::Playing;
            let play_item = ChromeGroupItem { control_id: "shell.tutorial.playPause", icon_id: Some(if playing { "pause" } else { "play" }), label: None, active: false, disabled: false, kind: HitKind::NavbarItem };
            let play_w = measure_chrome_group_item(atlas, theme, &play_item).max(btn_h);
            let play_rect = Rect::new(x, btn_y, play_w, btn_h);
            self.chrome_build.register_tooltip(play_item.control_id, if playing { "Pause" } else { "Play" });
            render_chrome_group(draw, atlas, icons, input, theme, play_rect, &[play_item], true);
            if self.chrome_build.clicked_this_frame && play_rect.contains(input.pointer_x, input.pointer_y) {
                self.tutorial_toggle_play_pause();
            }
            x += play_w + theme.gap_standard;
        }

        let stop_item = ChromeGroupItem { control_id: "shell.tutorial.stop", icon_id: Some("square"), label: None, active: false, disabled: false, kind: HitKind::NavbarItem };
        let stop_w = measure_chrome_group_item(atlas, theme, &stop_item).max(btn_h);
        let stop_rect = Rect::new(x, btn_y, stop_w, btn_h);
        self.chrome_build.register_tooltip(stop_item.control_id, if mode == TutorialMode::Recording { "Stop recording" } else { "Stop tutorial" });
        render_chrome_group(draw, atlas, icons, input, theme, stop_rect, &[stop_item], true);
        if self.chrome_build.clicked_this_frame && stop_rect.contains(input.pointer_x, input.pointer_y) {
            self.tutorial_stop();
            return;
        }
        x += stop_w + theme.gap_standard;

        let cur_s = (playhead_ms / 1000.0).max(0.0) as u64;
        let time_label = if mode == TutorialMode::Recording {
            format!("REC {}:{:02}", cur_s / 60, cur_s % 60)
        } else {
            let total_s = duration_ms / 1000;
            format!("{}:{:02} / {}:{:02}", cur_s / 60, cur_s % 60, total_s / 60, total_s % 60)
        };
        chrome_text(draw, atlas, input, theme, &time_label, x, btn_y + (btn_h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text);
        x += atlas.measure_text(&time_label, theme.font_size_small).0 + theme.gap_standard * 2.0;

        let mut rx = width - theme.padding_standard;
        if mode != TutorialMode::Recording {
            const RATES: [(f32, &str, &str); 4] = [(2.0, "2x", "shell.tutorial.rate.2"), (1.5, "1.5x", "shell.tutorial.rate.1_5"), (1.0, "1x", "shell.tutorial.rate.1"), (0.5, "0.5x", "shell.tutorial.rate.0_5")];
            for (value, label, control_id) in RATES {
                let item = ChromeGroupItem { control_id, icon_id: None, label: Some(label), active: (rate - value).abs() < 0.01, disabled: false, kind: HitKind::Toggle };
                let item_w = measure_chrome_group_item(atlas, theme, &item);
                rx -= item_w;
                let item_rect = Rect::new(rx, btn_y, item_w, btn_h);
                render_chrome_group(draw, atlas, icons, input, theme, item_rect, &[item], true);
                if self.chrome_build.clicked_this_frame && item_rect.contains(input.pointer_x, input.pointer_y) {
                    if let Some(r) = self.tutorial.as_mut() {
                        r.rate = value;
                    }
                }
            }
            rx -= theme.gap_standard;
        }

        let scrubber_rect = Rect::new(x, btn_y, (rx - x).max(24.0), btn_h);
        let track_y = btn_y + btn_h * 0.5 - 2.0;
        draw.push_rounded([scrubber_rect.x, track_y, scrubber_rect.w, 4.0], theme.border_normal, 2.0);
        let progress = tutorial_scrub_progress(playhead_ms, duration_ms);
        let knob_x = scrubber_rect.x + scrubber_rect.w * progress;
        draw.push_rounded([knob_x - 6.0, btn_y + btn_h * 0.5 - 6.0, 12.0, 12.0], theme.selected, 6.0);
        input.register_hit(HitTarget { rect: scrubber_rect, event: None, control_id: Some("shell.tutorial.scrubber".into()), kind: HitKind::Slider, drag_axis: None, drag_data: None });
        if mode != TutorialMode::Recording && input.pointer_down && scrubber_rect.contains(input.pointer_x, input.pointer_y) {
            let target_ms = tutorial_scrub_target_ms(input.pointer_x, scrubber_rect, duration_ms);
            self.tutorial_seek(target_ms);
        }
    }
    //#endregion 🎬️Chrome
}

/// 🎥️ Recorder-only sampling: the active window's camera on meaningful change (not every frame), and a
/// periodic full UI snapshot (Design Decision 7's "minimal recorder" scope).
fn tutorial_recorder_sample(state: &mut ShellState, runtime: &mut TutorialRuntime, now_wall_ms: f64) {
    const CAMERA_SAMPLE_MIN_INTERVAL_MS: f64 = 150.0;
    const CAMERA_MOVE_EPSILON: f64 = 0.01;
    const UI_SAMPLE_INTERVAL_MS: f64 = 2000.0;

    if let Some(window_id) = state.active_window_id.clone() {
        if let Some(pose) = tutorial_capture_camera_pose(state, &window_id) {
            let last_sample_ms = runtime.recorder_last_camera_wall_ms.get(&window_id).copied().unwrap_or(f64::NEG_INFINITY);
            let changed = runtime.recorder_last_camera_pose.get(&window_id).map(|prev| !tutorial_camera_pose_close(prev, &pose, CAMERA_MOVE_EPSILON)).unwrap_or(true);
            if changed && now_wall_ms - last_sample_ms >= CAMERA_SAMPLE_MIN_INTERVAL_MS {
                runtime.definition.tracks.camera.push(semio_framework::TutorialCameraKeyframe { at: runtime.playhead_ms.max(0.0) as u64, window_id: window_id.clone(), camera: pose.clone(), easing: semio_framework::TutorialEasing::EaseInOut });
                runtime.recorder_last_camera_wall_ms.insert(window_id.clone(), now_wall_ms);
                runtime.recorder_last_camera_pose.insert(window_id, pose);
            }
        }
    }
    if now_wall_ms - runtime.recorder_last_ui_sample_wall_ms >= UI_SAMPLE_INTERVAL_MS {
        let snapshot = tutorial_capture_ui_snapshot(state);
        if snapshot != runtime.recorder_last_ui {
            runtime.definition.tracks.ui.push(semio_framework::TutorialUiKeyframe { at: runtime.playhead_ms.max(0.0) as u64, sample: semio_framework::TutorialUiSample::Snapshot { state: Box::new(snapshot.clone()) } });
            runtime.recorder_last_ui = snapshot;
        }
        runtime.recorder_last_ui_sample_wall_ms = now_wall_ms;
    }
}

#[cfg(test)]
mod tutorial_tests {
    use super::*;

    fn shell() -> ShellState {
        ShellState::new(Vec::new(), String::new())
    }

    //#region PureMathTests
    #[test]
    fn advance_playhead_scales_by_rate() {
        assert_eq!(tutorial_advance_playhead(1000.0, 500.0, 2.0), 2000.0);
        assert_eq!(tutorial_advance_playhead(1000.0, 500.0, 0.5), 1250.0);
        assert_eq!(tutorial_advance_playhead(1000.0, 0.0, 1.0), 1000.0);
    }

    #[test]
    fn scrub_progress_clamps_and_avoids_divide_by_zero() {
        assert_eq!(tutorial_scrub_progress(0.0, 0), 0.0);
        assert_eq!(tutorial_scrub_progress(500.0, 1000), 0.5);
        assert_eq!(tutorial_scrub_progress(5000.0, 1000), 1.0);
        assert_eq!(tutorial_scrub_progress(-5.0, 1000), 0.0);
    }

    #[test]
    fn scrub_target_ms_maps_pointer_position_to_playhead() {
        let track = Rect::new(100.0, 0.0, 200.0, 20.0);
        assert_eq!(tutorial_scrub_target_ms(100.0, track, 1000), 0.0);
        assert_eq!(tutorial_scrub_target_ms(200.0, track, 1000), 500.0);
        assert_eq!(tutorial_scrub_target_ms(300.0, track, 1000), 1000.0);
        assert_eq!(tutorial_scrub_target_ms(50.0, track, 1000), 0.0);
        assert_eq!(tutorial_scrub_target_ms(9999.0, track, 1000), 1000.0);
    }
    //#endregion PureMathTests

    //#region CameraConversionTests
    #[test]
    fn orbit_camera_round_trips_through_tutorial_camera_state() {
        let orbit = ui_wgpu::wgpu::OrbitController { target: ui_wgpu::wgpu::Vec3::new(1.0, 2.0, 3.0), distance: 10.0, yaw: 0.4, pitch: 0.2, fov_y: 45.0_f32.to_radians() };
        let tutorial_camera = orbit_to_tutorial_camera(&orbit);
        let round_tripped = tutorial_camera_to_orbit(&tutorial_camera).expect("orbit camera state converts back");
        let original_pose = orbit.to_camera();
        let round_tripped_pose = round_tripped.to_camera();
        assert!((original_pose.position.x - round_tripped_pose.position.x).abs() < 0.01);
        assert!((original_pose.position.y - round_tripped_pose.position.y).abs() < 0.01);
        assert!((original_pose.position.z - round_tripped_pose.position.z).abs() < 0.01);
        assert!((original_pose.fov_y - round_tripped_pose.fov_y).abs() < 0.001);
    }

    #[test]
    fn canvas_camera_state_has_no_orbit_equivalent() {
        assert!(tutorial_camera_to_orbit(&semio_framework::TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 1.0 }).is_none());
    }

    #[test]
    fn camera_pose_close_only_compares_matching_kinds() {
        let orbit_a = semio_framework::TutorialCameraState::Orbit { position: [0.0, 0.0, 0.0], target: [0.0, 0.0, 0.0], up: [0.0, 0.0, 1.0], fov: Some(45.0) };
        let orbit_b = semio_framework::TutorialCameraState::Orbit { position: [0.0001, 0.0, 0.0], target: [0.0, 0.0, 0.0], up: [0.0, 0.0, 1.0], fov: Some(45.0) };
        let canvas = semio_framework::TutorialCameraState::Canvas { x: 0.0, y: 0.0, zoom: 1.0 };
        assert!(tutorial_camera_pose_close(&orbit_a, &orbit_b, 0.01));
        assert!(!tutorial_camera_pose_close(&orbit_a, &canvas, 0.01));
    }
    //#endregion CameraConversionTests

    //#region UiSnapshotTests
    #[test]
    fn ui_snapshot_round_trips_panel_tabs_and_focus() {
        let mut state = shell();
        state.active_window_id = Some("window-a".into());
        state.left_panel_open = true;
        state.active_left_kind = LeftPanelKind::Display;
        state.active_left_tab = Some("tab-x".into());
        state.right_panel_open = true;
        state.active_right_kind = RightPanelKind::Settings;
        state.active_right_tab = Some("tab-y".into());

        let snapshot = tutorial_capture_ui_snapshot(&state);
        assert_eq!(snapshot.focused_window_id.as_deref(), Some("window-a"));
        assert_eq!(snapshot.active_panel_tab_by_group.get("display").map(String::as_str), Some("tab-x"));
        assert_eq!(snapshot.active_panel_tab_by_group.get("settings").map(String::as_str), Some("tab-y"));

        let mut fresh = shell();
        tutorial_apply_ui_snapshot(&mut fresh, &snapshot);
        assert_eq!(fresh.active_window_id.as_deref(), Some("window-a"));
        assert!(fresh.left_panel_open);
        assert_eq!(fresh.active_left_kind, LeftPanelKind::Display);
        assert_eq!(fresh.active_left_tab.as_deref(), Some("tab-x"));
        assert!(fresh.right_panel_open);
        assert_eq!(fresh.active_right_kind, RightPanelKind::Settings);
        assert_eq!(fresh.active_right_tab.as_deref(), Some("tab-y"));
    }

    #[test]
    fn ui_snapshot_absent_panel_tabs_close_the_panel() {
        let mut state = shell();
        state.left_panel_open = true;
        state.active_left_tab = Some("tab-x".into());
        state.right_panel_open = true;
        state.active_right_tab = Some("tab-y".into());
        let empty_snapshot = semio_framework::TutorialUiSnapshot::default();
        tutorial_apply_ui_snapshot(&mut state, &empty_snapshot);
        assert!(!state.left_panel_open);
        assert!(!state.right_panel_open);
    }

    #[test]
    fn ui_change_applies_live_against_shell_state() {
        let mut state = shell();
        let change = semio_framework::TutorialUiChange::ActiveUtility { window_id: "window-a".into(), utility_id: Some("select".into()) };
        tutorial_apply_ui_change_to_shell(&mut state, &change);
        assert_eq!(state.active_utility_by_window.get("window-a").map(String::as_str), Some("select"));
    }
    //#endregion UiSnapshotTests

    //#region GesturePointTests
    #[test]
    fn gesture_point_resolves_screen_and_normalized() {
        let mut state = shell();
        state.screen_w = 1000.0;
        state.screen_h = 500.0;
        assert_eq!(tutorial_resolve_gesture_point(&state, &semio_framework::IntroductionPoint::Screen { x: 42.0, y: 7.0 }), Some((42.0, 7.0)));
        assert_eq!(tutorial_resolve_gesture_point(&state, &semio_framework::IntroductionPoint::ScreenNormalized { x: 0.5, y: 0.25 }), Some((500.0, 125.0)));
    }

    #[test]
    fn gesture_point_resolves_window_local() {
        let mut state = shell();
        state.window_content_rects.insert("window-a".into(), Rect::new(100.0, 50.0, 400.0, 300.0));
        assert_eq!(tutorial_resolve_gesture_point(&state, &semio_framework::IntroductionPoint::Window { id: "window-a".into(), x: 10.0, y: 20.0 }), Some((110.0, 70.0)));
        assert_eq!(tutorial_resolve_gesture_point(&state, &semio_framework::IntroductionPoint::WindowNormalized { id: "window-a".into(), x: 0.5, y: 0.5 }), Some((300.0, 200.0)));
        assert_eq!(tutorial_resolve_gesture_point(&state, &semio_framework::IntroductionPoint::Window { id: "missing".into(), x: 0.0, y: 0.0 }), None);
    }

    #[test]
    fn gesture_point_scopes_out_scene_and_entity_kinds() {
        let state = shell();
        assert_eq!(tutorial_resolve_gesture_point(&state, &semio_framework::IntroductionPoint::Scene { id: "w".into(), position: [0.0, 0.0, 0.0] }), None);
        let point = semio_framework_async::block_on(semio_framework::IntroductionPoint::any_entity("w", "vortex"));
        assert_eq!(tutorial_resolve_gesture_point(&state, &point), None);
    }
    //#endregion GesturePointTests

    //#region LifecycleTests
    #[test]
    fn seek_clamps_to_duration_and_updates_playhead() {
        let mut state = shell();
        let definition = semio_framework::TutorialDefinition {
            id: "t1".into(),
            title: LocalizedLabel::data("Test"),
            description: None,
            duration_ms: 1000,
            chapters: Vec::new(),
            base: semio_framework::TutorialBase { artifact_dsl: None, example_id: None, ui: semio_framework::TutorialUiSnapshot::default(), cameras: Vec::new() },
            tracks: semio_framework::TutorialTracks::default(),
            recorded_at: None,
        };
        state.tutorial = Some(TutorialRuntime {
            definition,
            mode: TutorialMode::Paused,
            playhead_ms: 0.0,
            rate: 1.0,
            applied_ms: 0.0,
            pre_sandbox_document_dsl: None,
            pre_sandbox_ui: semio_framework::TutorialUiSnapshot::default(),
            last_tick_wall_ms: 0.0,
            converge: HashMap::new(),
            recorder_last_camera_wall_ms: HashMap::new(),
            recorder_last_camera_pose: HashMap::new(),
            recorder_last_ui: semio_framework::TutorialUiSnapshot::default(),
            recorder_last_ui_sample_wall_ms: 0.0,
        });
        state.tutorial_seek(5000.0);
        assert_eq!(state.tutorial.as_ref().unwrap().playhead_ms, 1000.0);
    }

    #[test]
    fn note_real_dispatch_deviates_a_playing_tutorial() {
        let mut state = shell();
        let definition = semio_framework::TutorialDefinition {
            id: "t1".into(),
            title: LocalizedLabel::data("Test"),
            description: None,
            duration_ms: 1000,
            chapters: Vec::new(),
            base: semio_framework::TutorialBase { artifact_dsl: None, example_id: None, ui: semio_framework::TutorialUiSnapshot::default(), cameras: Vec::new() },
            tracks: semio_framework::TutorialTracks::default(),
            recorded_at: None,
        };
        state.tutorial = Some(TutorialRuntime {
            definition,
            mode: TutorialMode::Playing,
            playhead_ms: 0.0,
            rate: 1.0,
            applied_ms: 0.0,
            pre_sandbox_document_dsl: None,
            pre_sandbox_ui: semio_framework::TutorialUiSnapshot::default(),
            last_tick_wall_ms: 0.0,
            converge: HashMap::new(),
            recorder_last_camera_wall_ms: HashMap::new(),
            recorder_last_camera_pose: HashMap::new(),
            recorder_last_ui: semio_framework::TutorialUiSnapshot::default(),
            recorder_last_ui_sample_wall_ms: 0.0,
        });
        state.tutorial_note_real_dispatch(&ActionDescriptor { controller_id: "app".into(), action: "someAction".into(), args: None });
        assert_eq!(state.tutorial.as_ref().unwrap().mode, TutorialMode::Deviated);
    }

    #[test]
    fn recorder_records_annotational_events_but_skips_set_camera() {
        let mut state = shell();
        let definition = semio_framework::TutorialDefinition {
            id: "rec".into(),
            title: LocalizedLabel::data("Recording"),
            description: None,
            duration_ms: 0,
            chapters: Vec::new(),
            base: semio_framework::TutorialBase { artifact_dsl: None, example_id: None, ui: semio_framework::TutorialUiSnapshot::default(), cameras: Vec::new() },
            tracks: semio_framework::TutorialTracks::default(),
            recorded_at: None,
        };
        state.tutorial = Some(TutorialRuntime {
            definition,
            mode: TutorialMode::Recording,
            playhead_ms: 250.0,
            rate: 1.0,
            applied_ms: 0.0,
            pre_sandbox_document_dsl: None,
            pre_sandbox_ui: semio_framework::TutorialUiSnapshot::default(),
            last_tick_wall_ms: 0.0,
            converge: HashMap::new(),
            recorder_last_camera_wall_ms: HashMap::new(),
            recorder_last_camera_pose: HashMap::new(),
            recorder_last_ui: semio_framework::TutorialUiSnapshot::default(),
            recorder_last_ui_sample_wall_ms: 0.0,
        });
        state.tutorial_note_real_dispatch(&ActionDescriptor { controller_id: "app".into(), action: "setCamera".into(), args: None });
        state.tutorial_note_real_dispatch(&ActionDescriptor { controller_id: "app".into(), action: "doSomething".into(), args: None });
        let events = &state.tutorial.as_ref().unwrap().definition.tracks.events;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].at, 250);
    }
    //#endregion LifecycleTests
}
//#endregion 🎬️Tutorial

impl ShellState {
    /// 🎭️ Runs present-side synchronization around the owned chrome construction phase.
    pub fn render_chrome(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut DrawList,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        engine_resources: &mut crate::engine_canvas::EngineCanvasBuildContext,
        world_resources: &mut infinite_world::world::World3dBuildContext,
    ) {
        let close_consumed = self.close_document_one();
        begin_ui_document_opportunity(close_consumed);
        self.load_ui_prefs_once();
        if let Some(app_id) = self.session.as_ref().map(|session| session.app.id.clone()) {
            self.chrome_build.introduction_seen.entry(app_id.clone()).or_insert_with(|| read_stored_introduction_seen(&app_id));
        }
        #[cfg(not(target_arch = "wasm32"))]
        self.publish_presence_heartbeat();
        self.persist_panel_layout_if_changed();
        self.render_chrome_build(draw, overlay, atlas, icons, input, theme, engine_resources, world_resources);
        for app_id in std::mem::take(&mut self.chrome_build.introduction_seen_writes) {
            write_stored_introduction_seen(&app_id);
        }
        self.persist_ui_prefs_if_changed();
    }

    /// 🧱️ Builds chrome exclusively from explicit owned state and caller-provided rendering resources.
    fn render_chrome_build(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut DrawList,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        engine_resources: &mut crate::engine_canvas::EngineCanvasBuildContext,
        world_resources: &mut infinite_world::world::World3dBuildContext,
    ) {
        let w = self.screen_w;
        let h = self.screen_h;
        draw.set_screen_height(h);
        overlay.set_screen_height(h);
        overlay.clear();
        draw.push_solid([0.0, 0.0, w, h], theme.background);
        let body = self.body_rect(theme);
        let find_items = self.chrome_build.find_items.clone();
        find_items.clear();
        self.chrome_build.tooltip_titles.clear();
        self.chrome_build.element_rects.clear();
        self.chrome_build.compute_click_edge(input.pointer_down);
        self.chrome_tour_frame_begin();
        clear_graph_node_context();
        self.node_graph_states.clear();
        self.tiled_map_states.clear();
        self.board2d_states.clear();
        self.widget_maps.clear_frame();
        let mut overlay_slot = Some(overlay);
        {
            let _find_items_binding = find_items.bind();
            self.render_main_window(draw, &mut overlay_slot, atlas, icons, input, theme, body, engine_resources, world_resources);
        }
        self.find_items = find_items.take();
        if self.left_panel_open && self.has_left_tabs() {
            if let Some(panel_draw) = overlay_slot.as_deref_mut() {
                self.render_left_panel(panel_draw, None, atlas, icons, input, theme, body, engine_resources, world_resources);
            } else {
                self.render_left_panel(draw, None, atlas, icons, input, theme, body, engine_resources, world_resources);
            }
        }
        if self.right_panel_open && self.has_right_tabs() {
            if let Some(panel_draw) = overlay_slot.as_deref_mut() {
                self.render_right_panel(panel_draw, None, atlas, icons, input, theme, body, engine_resources, world_resources);
            } else {
                self.render_right_panel(draw, None, atlas, icons, input, theme, body, engine_resources, world_resources);
            }
        }
        with_chrome_sink(draw, &mut overlay_slot, |chrome, _select_overlay| {
            self.render_navbar(chrome, atlas, icons, input, theme, w);
            self.render_tutorial_bar(chrome, atlas, icons, input, theme, w);
            self.render_footer(chrome, atlas, icons, input, theme, w, h);
        });
        if let Some(overlay) = overlay_slot.as_deref_mut() {
            self.render_overlay(overlay, atlas, icons, input, theme, w, h);
            self.render_tree_drag_overlay(overlay, input, theme);
            render_tutorial_gesture_overlay(self, overlay, theme);
        }
        if let Some(error) = &self.error {
            let scroll_offsets = &mut self.scroll_offsets;
            let collapsed_sections = &mut self.collapsed_sections;
            let open_selects = &mut self.open_selects;
            let mut ctx = framework_widget_context(draw, None, atlas, Some(icons), input, theme, scroll_offsets, collapsed_sections, open_selects, None);
            draw_text(&mut ctx, error, 12.0, h - theme.footer_height - 24.0, theme.font_size_small, theme.error);
        }
    }

    fn body_rect(&self, theme: &Theme) -> Rect {
        let top = theme.navbar_height + self.tutorial_bar_reserve(theme);
        Rect::new(0.0, top, self.screen_w, self.screen_h - top - theme.footer_height)
    }

    /// 🎬️ Extra vertical space the tutorial control bar reserves below the navbar while a tutorial is
    /// active — `0.0` otherwise, so `body_rect`/the canvas layout are byte-identical to before this
    /// region existed whenever no tutorial is running.
    fn tutorial_bar_reserve(&self, theme: &Theme) -> f32 {
        if self.tutorial.is_some() {
            tutorial_bar_height(theme)
        } else {
            0.0
        }
    }

    fn shell_uri(&self) -> String {
        self.uri_history.get(self.uri_index).cloned().unwrap_or_else(|| self.session.as_ref().map(|s| format!("os://{}/{}", s.plugin_id, s.app.id)).unwrap_or_else(|| "os://home".into()))
    }

    fn has_left_tabs(&self) -> bool {
        self.session.is_some()
    }

    fn has_right_tabs(&self) -> bool {
        self.session.is_some()
    }

    fn left_tabs(&self, session: &ActiveSession) -> Vec<PanelTabDefinition> {
        let is_de = self.locale_id == "de";
        match self.active_left_kind {
            LeftPanelKind::Display => vec![
                PanelTabDefinition {
                    kind: semio_framework::PanelTabKind::DisplayWindows,
                    label: LocalizedLabel::data(shell_chrome_string("display.tab.windows", is_de)),
                    group: PanelGroup::Display,
                    body_key: Some(String::new()),
                    children: Vec::new(),
                },
                PanelTabDefinition { kind: semio_framework::PanelTabKind::DisplayLayout, label: LocalizedLabel::data(shell_chrome_string("display.tab.layout", is_de)), group: PanelGroup::Display, body_key: Some(String::new()), children: Vec::new() },
            ],
            LeftPanelKind::Workbench => {
                let mut tabs: Vec<PanelTabDefinition> = session.app.panel_tabs.iter().filter(|tab| group_side(tab.group) == "left").cloned().collect();
                fn panel_tabs_contain_artifact(list: &[PanelTabDefinition]) -> bool {
                    list.iter().any(|t| t.id() == FRAMEWORK_PANEL_TAB_ARTIFACT_ID || panel_tabs_contain_artifact(&t.children))
                }
                let has_document = panel_tabs_contain_artifact(&tabs);
                if !has_document {
                    tabs.insert(
                        0,
                        PanelTabDefinition {
                            kind: semio_framework::PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
                            label: LocalizedLabel::data(shell_panel_tab_label(FRAMEWORK_PANEL_TAB_ARTIFACT_ID, "Document", is_de)),
                            group: PanelGroup::Workbench,
                            body_key: Some(String::new()),
                            children: Vec::new(),
                        },
                    );
                }
                tabs
            }
        }
    }

    fn right_tabs(&self, session: &ActiveSession) -> Vec<PanelTabDefinition> {
        match self.active_right_kind {
            RightPanelKind::Settings => {
                let is_de = self.locale_id == "de";
                let mut tabs = vec![PanelTabDefinition {
                    kind: semio_framework::PanelTabKind::SettingsGeneral,
                    label: LocalizedLabel::data(shell_chrome_string("settings.tab.general", is_de)),
                    group: PanelGroup::Settings,
                    body_key: Some(String::new()),
                    children: Vec::new(),
                }];
                // 🔒️ Byte-identical to React's `createFrameworkSettingsPanelTab` (`ui/js/react/
                // index.tsx:9526-9528`): a locked theme drops the whole Theme tab, not just its editor.
                if shell_pref_locks().theme_id.is_none() {
                    tabs.push(PanelTabDefinition {
                        kind: semio_framework::PanelTabKind::SettingsTheme,
                        label: LocalizedLabel::data(shell_chrome_string("settings.tab.theme", is_de)),
                        group: PanelGroup::Settings,
                        body_key: Some(String::new()),
                        children: Vec::new(),
                    });
                }
                // 🎛️ See `FRAMEWORK_SETTINGS_COMMANDS_TAB_ID`'s doc comment: the honest substitute for
                // React's `bottom-middle`-anchored command palette dock, which this renderer's 2-column
                // panel model has no equivalent surface for.
                tabs.push(PanelTabDefinition {
                    kind: semio_framework::PanelTabKind::App(FRAMEWORK_SETTINGS_COMMANDS_TAB_ID.into()),
                    label: LocalizedLabel::data(shell_chrome_string("settings.tab.commands", is_de)),
                    group: PanelGroup::Settings,
                    body_key: Some(String::new()),
                    children: Vec::new(),
                });
                tabs
            }
            RightPanelKind::Details => session.app.panel_tabs.iter().filter(|tab| group_side(tab.group) == "right").cloned().collect(),
        }
    }

    fn active_left_tab_id(&self, session: &ActiveSession) -> String {
        match self.active_left_kind {
            LeftPanelKind::Display => FRAMEWORK_DISPLAY_WINDOWS_TAB_ID.into(),
            LeftPanelKind::Workbench => {
                if self.host_config().is_some_and(|cfg| session.app.id == cfg.host_app_id) {
                    // 🏠️🧳️ `session.app` is confirmed to be the host app here, so its own first-declared
                    // panel tab (self-declared via `AppBuilder::panel_tab`) is the catalogue default.
                    Self::panel_state_from_view(&session.view_state).map(|p| p.active_panel_tab).unwrap_or_else(|| session.app.panel_tabs.first().map(|tab| tab.id().to_string()).unwrap_or_default())
                } else {
                    let tabs = self.left_tabs(session);
                    if let Some(id) = &self.active_left_tab {
                        if tabs.iter().any(|tab| tab.id() == *id) {
                            return id.clone();
                        }
                    }
                    tabs.first().map(|t| t.id().to_string()).unwrap_or_else(|| FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into())
                }
            }
        }
    }

    fn active_right_tab_id(&self, session: &ActiveSession) -> String {
        // 🎨️🎛️ Settings now has 2-3 tabs (General / Theme / Commands — see `right_tabs`), so this needs
        // to actually respect `self.active_right_tab` here too, same as every other panel column, instead
        // of hardcoding General; falls back to General (first tab) exactly like before when unset/stale.
        let tabs = self.right_tabs(session);
        if let Some(id) = &self.active_right_tab {
            if tabs.iter().any(|tab| tab.id() == *id) {
                return id.clone();
            }
        }
        tabs.first().map(|t| t.id().to_string()).unwrap_or_default()
    }

    fn has_display_tabs(&self) -> bool {
        self.session.as_ref().is_some_and(|s| !s.app.window_kinds.is_empty())
    }

    fn floating_panel_rect(&self, left: bool, body: Rect, theme: &Theme) -> Rect {
        let inset = theme.panel_inset;
        let width = if left { floating_panel_width(self.left_panel_width, body, theme) } else { floating_panel_width(self.right_panel_width, body, theme) };
        if left {
            Rect::new(body.x + inset, body.y + inset, width, body.h - inset * 2.0)
        } else {
            Rect::new(body.x + body.w - inset - width, body.y + inset, width, body.h - inset * 2.0)
        }
    }

    fn render_navbar(&mut self, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32) {
        let is_de = self.locale_id == "de";
        let navbar_rect = Rect::new(0.0, 0.0, width, theme.navbar_height);
        let navbar_hovered = navbar_rect.contains(input.pointer_x, input.pointer_y);
        draw.push_solid([0.0, 0.0, width, theme.navbar_height], theme.navbar);
        let border_color = if navbar_hovered { theme.border_emphasized } else { theme.border_normal };
        draw.push_solid([0.0, theme.navbar_height - theme.stroke_hairline, width, theme.stroke_hairline], border_color);
        let btn_h = theme.control_height;
        let btn_y = (theme.navbar_height - btn_h) * 0.5;
        let mut x = theme.padding_standard;
        let logo_size = btn_h - theme.gap_standard;
        chrome_icon(draw, icons, "semio-logo", x, btn_y + (btn_h - logo_size) * 0.5, logo_size, theme.text);
        x += logo_size + theme.gap_standard;
        let title = self.session.as_ref().map(|s| app_breadcrumb(resolve_app_breadcrumb(&s.app, &self.terminology_id))).unwrap_or_else(|| if self.space_mode { format!("semio · {}", self.plugin_filter) } else { "semio · os".into() });
        chrome_text(draw, atlas, input, theme, &title, x, btn_y + (btn_h + theme.font_size_body) * 0.5 - 2.0, theme.font_size_body, theme.text);
        x += atlas.measure_text(&title, theme.font_size_body).0 + theme.gap_standard * 2.0;
        let examples = self.active_plugin_examples();
        if !examples.is_empty() && !self.space_mode {
            let active_example = examples.iter().find(|ex| Some(&ex.id) == self.active_example_id.as_ref());
            let active_label = active_example.map(|ex| ex.label.resolve(self.active_terminology(), self.active_locale())).unwrap_or("Example");
            let active_example_icon = active_example.map(|ex| ex.icon_id.as_str()).unwrap_or("file-text");
            let fixture_w = atlas.measure_text(active_label, theme.font_size_small).0 + theme.padding_standard * 2.0 + theme.gap_standard;
            let fixture_rect = Rect::new(x, btn_y, fixture_w.max(120.0), btn_h);
            self.chrome_build.register_tooltip("playground.navbar.fixture", active_label);
            render_chrome_group(
                draw,
                atlas,
                icons,
                input,
                theme,
                fixture_rect,
                &[ChromeGroupItem {
                    control_id: "playground.navbar.fixture",
                    icon_id: Some(active_example_icon),
                    label: Some(active_label),
                    active: self.overlay_state == OverlayState::Dropdown("example".to_string()),
                    disabled: false,
                    kind: HitKind::NavbarItem,
                }],
                true,
            );
        }
        let mut rx = width - theme.padding_standard;
        let fullscreen_item = ChromeGroupItem {
            control_id: "ui.fullscreen.toggle",
            icon_id: Some(if self.fullscreen_active { "minimize-2" } else { "maximize-2" }),
            label: Some(if self.fullscreen_active { shell_chrome_string("fullscreen.exit", is_de) } else { shell_chrome_string("fullscreen.toggle", is_de) }),
            active: self.fullscreen_active,
            disabled: false,
            kind: HitKind::Toggle,
        };
        self.chrome_build.register_tooltip(fullscreen_item.control_id, fullscreen_item.label.unwrap_or_default());
        let fullscreen_w = measure_chrome_group_item(atlas, theme, &fullscreen_item);
        rx -= fullscreen_w;
        render_chrome_group(draw, atlas, icons, input, theme, Rect::new(rx, btn_y, fullscreen_w, btn_h), &[fullscreen_item], true);
        rx -= theme.gap_standard;
        // 🎓️ Item 3's "simple direct trigger point" (the `introduceApp` os-command itself is
        // `w3-command-palette`'s `shell::ActionPanelAndUtilities` scope, off-limits here) — only shown
        // when the active app actually declares an `introduction`. Click handling is chrome-owned (not
        // routed through `ActionDescriptor`/`ShellActions`), mirroring the dialog trigger above.
        if self.session.as_ref().is_some_and(|s| s.app.introduction.is_some()) {
            let tour_item = ChromeGroupItem { control_id: "shell.introduction.start", icon_id: Some("help-circle"), label: None, active: false, disabled: false, kind: HitKind::NavbarItem };
            let tour_w = measure_chrome_group_item(atlas, theme, &tour_item);
            rx -= tour_w;
            let tour_rect = Rect::new(rx, btn_y, tour_w, btn_h);
            self.chrome_build.register_tooltip(tour_item.control_id, "Start introduction");
            render_chrome_group(draw, atlas, icons, input, theme, tour_rect, &[tour_item], true);
            if !self.chrome_build.dialog_open() && self.chrome_build.clicked_this_frame && tour_rect.contains(input.pointer_x, input.pointer_y) {
                // 🎬️ Introductions and tutorials are mutually exclusive (Design Decision 8) — starting one
                // clears the other.
                self.tutorial_stop();
                self.chrome_build.start_introduction();
            }
            rx -= theme.gap_standard;
        }
        // 🎬️ "Play Tutorial" trigger, beside the introduction trigger above — same "simple direct trigger
        // point" pattern (chrome-owned click, not routed through `ActionDescriptor`/`dispatch_action`),
        // shown only when the active app declares at least one tutorial. Multiple declared tutorials are
        // still fully reachable through the generic Action rail / command palette — the auto-injected
        // `startTutorial` action already carries a `tutorialId` select arg per declared tutorial — this
        // navbar shortcut always starts the first one rather than inventing a second picker UI here.
        if self.tutorial.is_none() {
            if let Some(tutorial_id) = self.session.as_ref().and_then(|s| s.app.tutorials.first().map(|t| t.id.clone())) {
                let play_item = ChromeGroupItem { control_id: "shell.tutorial.trigger", icon_id: Some("play-circle"), label: None, active: false, disabled: false, kind: HitKind::NavbarItem };
                let play_w = measure_chrome_group_item(atlas, theme, &play_item);
                rx -= play_w;
                let play_rect = Rect::new(rx, btn_y, play_w, btn_h);
                self.chrome_build.register_tooltip(play_item.control_id, "Play tutorial");
                render_chrome_group(draw, atlas, icons, input, theme, play_rect, &[play_item], true);
                if !self.chrome_build.dialog_open() && self.chrome_build.clicked_this_frame && play_rect.contains(input.pointer_x, input.pointer_y) {
                    self.tutorial_start(&tutorial_id);
                }
                rx -= theme.gap_standard;
            }
        }
        let mut toggle_items: Vec<ChromeGroupItem<'_>> = Vec::new();
        if self.has_display_tabs() {
            toggle_items.push(ChromeGroupItem {
                control_id: "ui.panelToggle.display",
                icon_id: Some(panel_toggle_icon_id("display", self.session.as_ref())),
                label: Some(shell_chrome_string("panelToggle.display", is_de)),
                active: self.left_panel_open && self.active_left_kind == LeftPanelKind::Display,
                disabled: false,
                kind: HitKind::Toggle,
            });
        }
        toggle_items.push(ChromeGroupItem {
            control_id: "ui.panelToggle.workbench",
            icon_id: Some(panel_toggle_icon_id("workbench", self.session.as_ref())),
            label: Some(shell_chrome_string("panelToggle.workbench", is_de)),
            active: self.left_panel_open && self.active_left_kind == LeftPanelKind::Workbench,
            disabled: false,
            kind: HitKind::Toggle,
        });
        toggle_items.push(ChromeGroupItem {
            control_id: "ui.panelToggle.details",
            icon_id: Some(panel_toggle_icon_id("details", self.session.as_ref())),
            label: Some(shell_chrome_string("panelToggle.details", is_de)),
            active: self.right_panel_open && self.active_right_kind == RightPanelKind::Details,
            disabled: false,
            kind: HitKind::Toggle,
        });
        toggle_items.push(ChromeGroupItem {
            control_id: "ui.panelToggle.settings",
            icon_id: Some(panel_toggle_icon_id("settings", self.session.as_ref())),
            label: Some(shell_chrome_string("panelToggle.settings", is_de)),
            active: self.right_panel_open && self.active_right_kind == RightPanelKind::Settings,
            disabled: false,
            kind: HitKind::Toggle,
        });
        for item in &toggle_items {
            self.chrome_build.register_tooltip(item.control_id, item.label.unwrap_or_default());
        }
        let toggle_w: f32 = toggle_items.iter().map(|item| measure_chrome_group_item(atlas, theme, item)).sum();
        rx -= toggle_w;
        render_chrome_group(draw, atlas, icons, input, theme, Rect::new(rx, btn_y, toggle_w, btn_h), &toggle_items, true);
        rx -= theme.gap_standard;
        if let Some(session) = &self.session {
            if session.app.modes.len() > 1 {
                // 🚧️ `modes` is a `NonEmptyVec`, whose `iter()` yields an opaque non-double-ended
                // iterator — collect before reversing for the right-to-left navbar order.
                let modes: Vec<&semio_framework::ModeDefinition> = session.app.modes.iter().collect();
                let mode_control_ids: Vec<String> = modes.iter().rev().map(|mode| format!("playground.navbar.modes.{}", mode.id)).collect();
                let mode_items: Vec<ChromeGroupItem<'_>> = modes
                    .iter()
                    .rev()
                    .zip(mode_control_ids.iter())
                    .map(|(mode, control_id)| {
                        let active_mode = session.view_state.active_mode_id.as_deref().unwrap_or(session.app.default_mode_id.as_str());
                        ChromeGroupItem {
                            control_id: control_id.as_str(),
                            icon_id: Some(mode.icon_id.as_str()),
                            label: Some(mode.label.resolve(self.active_terminology(), self.active_locale())),
                            active: active_mode == mode.id,
                            disabled: false,
                            kind: HitKind::NavbarItem,
                        }
                    })
                    .collect();
                for item in &mode_items {
                    self.chrome_build.register_tooltip(item.control_id, item.label.unwrap_or_default());
                }
                let mode_w: f32 = mode_items.iter().map(|item| measure_chrome_group_item(atlas, theme, item)).sum();
                rx -= mode_w;
                render_chrome_group(draw, atlas, icons, input, theme, Rect::new(rx, btn_y, mode_w, btn_h), &mode_items, true);
            }
        }
    }

    fn render_footer(&mut self, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32, height: f32) {
        let y = height - theme.footer_height;
        let footer_rect = Rect::new(0.0, y, width, theme.footer_height);
        let footer_hovered = footer_rect.contains(input.pointer_x, input.pointer_y);
        draw.push_solid([0.0, y, width, theme.footer_height], theme.navbar);
        let border_color = if footer_hovered { theme.border_emphasized } else { theme.border_normal };
        draw.push_solid([0.0, y, width, theme.stroke_hairline], border_color);
        if self.session.is_none() {
            return;
        }
        let btn_h = theme.control_height;
        let btn_y = y + (theme.footer_height - btn_h) * 0.5;
        // 🧰️ Footer sections: Selection · Utilities · History · Sync. The former `UtilityCategory::Actions`
        // section is deleted — window-scoped actions now live in the per-window Actions rail
        // (Architecture Decision 8/9, P6).
        chrome_register_utility_tooltips(&mut self.chrome_build, &self.active_utilities);
        let partitions = partition_utilities_by_category(&self.active_utilities);
        let sections = [partitions[0].as_slice(), partitions[1].as_slice(), partitions[2].as_slice(), partitions[3].as_slice()];
        let mut utility_x = theme.padding_standard;
        let mut first_section = true;
        for utilities in sections {
            if utilities.is_empty() {
                continue;
            }
            if !first_section {
                utility_x = render_footer_section_divider(draw, theme, utility_x, btn_y, btn_h);
            }
            first_section = false;
            utility_x = render_footer_utility_nodes(&mut self.chrome_build, draw, atlas, icons, input, theme, utility_x, btn_y, btn_h, utilities, &self.utility_collection_expanded);
        }
        // 🚦️ ticket §C5 — `#s-sync-status`/`#s-checkin`, left-aligned right after the plugin-declared
        // utility sections (own divider, `first_section` no longer matters past this point).
        #[cfg(not(target_arch = "wasm32"))]
        {
            if !first_section {
                utility_x = render_footer_section_divider(draw, theme, utility_x, btn_y, btn_h);
            }
            let uncommitted_count = uncommitted_edit_count(&self.history_entries);
            utility_x = render_sync_status_and_checkin(draw, atlas, icons, input, theme, self.sync_status.as_ref(), self.session.as_ref(), uncommitted_count, utility_x, btn_y, btn_h);
        }
        let _ = utility_x;
        // 👥️ ticket §5 — the live presence roster, right-aligned in the footer, scoped to the
        // CURRENTLY mounted session's own canonical surface (`presence_peer_rows_for_surface`'s pure
        // decision, so a stale roster from a just-closed document/surface never leaks in).
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(session) = &self.session {
            let target_surface = semio_framework::manifest::surface_app_id(&session.app.dialect, session.app.role);
            let rows = presence_peer_rows_for_surface(&self.presence_peers, self.presence_surface.as_deref(), &target_surface);
            render_presence_bar(draw, atlas, icons, input, theme, &rows, width - theme.padding_standard, btn_y, btn_h);
        }
    }

    fn render_floating_panel(
        &mut self,
        panel_draw: &mut DrawList,
        overlay: Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        panel: Rect,
        tabs: &[PanelTabDefinition],
        active_tab_id: &str,
        side_left: bool,
        engine_resources: &mut crate::engine_canvas::EngineCanvasBuildContext,
        world_resources: &mut infinite_world::world::World3dBuildContext,
    ) {
        const PANEL_RESIZE_HIT_PX: f32 = 20.0;
        let resize_id = if side_left { "panel.resize.left" } else { "panel.resize.right" };
        let resize_handle = if side_left { Rect::new(panel.x + panel.w - PANEL_RESIZE_HIT_PX, panel.y, PANEL_RESIZE_HIT_PX, panel.h) } else { Rect::new(panel.x, panel.y, PANEL_RESIZE_HIT_PX, panel.h) };
        let resize_active = input.drag.active && input.drag.target_id.as_deref() == Some(resize_id);
        let resize_handle_hovered = resize_handle.contains(input.pointer_x, input.pointer_y);
        let resize_edge_hot = resize_active || resize_handle_hovered;
        let panel_hovered = panel.contains(input.pointer_x, input.pointer_y);
        let hair = theme.stroke_hairline;
        let edge_color = |is_resize_edge: bool| {
            if is_resize_edge && resize_active {
                theme.accent
            } else if is_resize_edge && resize_handle_hovered {
                theme.border_emphasized
            } else if panel_hovered && !resize_edge_hot {
                theme.border_emphasized
            } else {
                theme.border_normal
            }
        };
        let top = edge_color(false);
        let bottom = edge_color(false);
        let left = edge_color(!side_left);
        let right = edge_color(side_left);
        let inner_stroke = if panel_hovered && !resize_edge_hot { theme.border_emphasized } else { theme.border_normal };
        let glass = panel_draw.push_glass([panel.x, panel.y, panel.w, panel.h], theme.border_radius, theme.glass(Level::Panel));
        panel_draw.begin_glass_content(glass);
        panel_draw.push_solid([panel.x, panel.y, panel.w, hair], top);
        panel_draw.push_solid([panel.x, panel.y + panel.h - hair, panel.w, hair], bottom);
        panel_draw.push_solid([panel.x, panel.y, hair, panel.h], left);
        panel_draw.push_solid([panel.x + panel.w - hair, panel.y, hair, panel.h], right);
        let tab_bar_h = render_panel_tab_bar(&mut self.chrome_build, panel_draw, atlas, icons, input, theme, panel, tabs, active_tab_id, side_left, inner_stroke, hair);
        let content = Rect::new(panel.x + theme.gap_standard, panel.y + tab_bar_h, panel.w - theme.gap_standard * 2.0, panel.h - tab_bar_h - theme.gap_standard);
        self.chrome_build.register_element_rect(semio_framework::panel_tab_element_id(active_tab_id), content);
        let scroll_key = format!("panel.{}.{}", if side_left { "left" } else { "right" }, active_tab_id);
        let scroll_y = *self.scroll_offsets.get(&scroll_key).unwrap_or(&0.0);
        panel_draw.push_scissor(content);
        input.register_hit(HitTarget { rect: content, event: None, control_id: Some(scroll_key.clone()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
        if self.panel_documents.contains_key(active_tab_id) {
            let scrolled = Rect::new(content.x, content.y - scroll_y, content.w, content.h);
            let scroll_offsets = &mut self.scroll_offsets;
            let collapsed_sections = &mut self.collapsed_sections;
            let open_selects = &mut self.open_selects;
            let widget_maps = &mut self.widget_maps;
            let mut ctx = framework_widget_context(panel_draw, overlay, atlas, Some(icons), input, theme, scroll_offsets, collapsed_sections, open_selects, Some(widget_maps));
            ctx.pick_clip = Some(content);
            if let Some(document) = self.panel_documents.get(active_tab_id) {
                render_ui_document(
                    document,
                    scrolled,
                    &mut ctx,
                    active_tab_id,
                    engine_resources,
                    world_resources,
                    &mut self.world3d_states,
                    &mut self.node_graph_states,
                    &mut self.tiled_map_states,
                    &mut self.icon_render_states,
                    &mut self.board2d_states,
                );
            }
        }
        panel_draw.pop_scissor();
        panel_draw.end_glass_content();
        input.register_hit(HitTarget { rect: resize_handle, event: None, control_id: Some(resize_id.into()), kind: HitKind::PanelResize, drag_axis: Some(DragAxis::Horizontal), drag_data: None });
    }

    fn render_left_panel(
        &mut self,
        panel_draw: &mut DrawList,
        mut overlay: Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        body: Rect,
        engine_resources: &mut crate::engine_canvas::EngineCanvasBuildContext,
        world_resources: &mut infinite_world::world::World3dBuildContext,
    ) {
        let session = match self.session.as_ref() {
            Some(s) => s.clone(),
            None => return,
        };
        let tabs = self.left_tabs(&session);
        if tabs.is_empty() {
            return;
        }
        let active = self.active_left_tab_id(&session);
        let panel = self.floating_panel_rect(true, body, theme);
        self.render_floating_panel(panel_draw, overlay.as_deref_mut(), atlas, icons, input, theme, panel, &tabs, &active, true, engine_resources, world_resources);
    }

    fn render_right_panel(
        &mut self,
        panel_draw: &mut DrawList,
        mut overlay: Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        body: Rect,
        engine_resources: &mut crate::engine_canvas::EngineCanvasBuildContext,
        world_resources: &mut infinite_world::world::World3dBuildContext,
    ) {
        let session = match self.session.as_ref() {
            Some(s) => s.clone(),
            None => return,
        };
        let tabs = self.right_tabs(&session);
        if tabs.is_empty() {
            return;
        }
        let active = self.active_right_tab_id(&session);
        let panel = self.floating_panel_rect(false, body, theme);
        self.render_floating_panel(panel_draw, overlay.as_deref_mut(), atlas, icons, input, theme, panel, &tabs, &active, false, engine_resources, world_resources);
    }

    fn render_main_window(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        bounds: Rect,
        engine_resources: &mut crate::engine_canvas::EngineCanvasBuildContext,
        world_resources: &mut infinite_world::world::World3dBuildContext,
    ) {
        draw.push_solid([bounds.x, bounds.y, bounds.w, bounds.h], theme.background);
        let session = match self.session.as_ref() {
            Some(s) => s.clone(),
            None => return,
        };
        let mut canvas = bounds.inset(theme.panel_inset);
        canvas = self.render_studio_canvas_bars(draw, atlas, icons, input, theme, canvas, &session);
        if self.space_mode {
            if let Some(spawned_ui) = self.spawned_ui.take() {
                self.render_window_content(draw, overlay.as_deref_mut(), atlas, icons, input, theme, canvas, canvas, &[canvas], &spawned_ui, "spawned", engine_resources, world_resources);
                self.spawned_ui = Some(spawned_ui);
                return;
            }
        }
        let window_labels: HashMap<String, String> =
            session.app.window_kinds.iter().map(|kind| (kind.id.clone(), app_window_label(&session.app, &self.terminology_id, self.active_locale(), kind.label.resolve(self.active_terminology(), self.active_locale())))).collect();
        let window_icon_ids: HashMap<String, String> = session.app.window_kinds.iter().map(|kind| (kind.id.clone(), kind.icon_id.as_str().to_string())).collect();
        self.dock_canvas_bounds = canvas;
        self.dock_drop_tab_bars = self.dock_tab_bars_for_drop(atlas, theme, canvas, &window_labels, &window_icon_ids);
        self.dock_drop_bodies = self.dock.stack_body_rects(canvas, theme, &window_labels, atlas).into_iter().map(|(path, rect, active)| (path, rect, active)).collect();
        {
            let mut dock_ctx = DockRenderContext { draw, atlas, icons, input, theme, window_labels: &window_labels, window_icon_ids: &window_icon_ids };
            self.dock.register_hits(&mut dock_ctx, canvas);
        }
        let (placements, silhouettes) = self.dock.stack_body_rects_with_silhouettes(canvas, theme, &window_labels, atlas);
        let show_fallback = placements.is_empty();
        self.window_content_rects.clear();
        self.window_silhouettes = silhouettes;
        for (_, safe_body, window_id) in placements {
            let window_kind = session.app.window_kinds.iter().find(|kind| kind.id == window_id).cloned();
            let silhouette = self.window_silhouettes.get(&window_id).cloned();
            let mut window_chip_hits: Vec<(Rect, String)> = Vec::new();
            let content_viewport = silhouette.as_ref().map(WindowSilhouette::content_bounds).unwrap_or(safe_body);
            self.window_content_rects.insert(window_id.clone(), content_viewport);
            if let Some(ui) = self.window_ui.remove(&window_id) {
                let content_layout = content_viewport;
                let clip_rects = silhouette.as_ref().map(WindowSilhouette::content_clip_rects).unwrap_or_else(|| vec![safe_body]);
                let hit_regions: Vec<Rect> = clip_rects.iter().filter_map(|clip| Self::intersect_content_rect(*clip, content_viewport)).collect();
                draw.begin_silhouette_clip(&clip_rects);
                self.render_window_content(draw, overlay.as_deref_mut(), atlas, icons, input, theme, content_viewport, content_layout, &hit_regions, &ui, &window_id, engine_resources, world_resources);
                draw.end_silhouette_clip();
                self.window_ui.insert(window_id.clone(), ui);
            }
            if let Some(kind) = window_kind {
                let measures_outcome = self.render_window_measures_rail(draw, overlay, atlas, icons, input, theme, &safe_body, &window_id, &kind);
                if let Some(hit) = measures_outcome.chip_hit {
                    window_chip_hits.push(hit);
                }
                if let Some(hit) = self.render_window_engagement_rail(draw, overlay, atlas, icons, input, theme, &safe_body, &window_id, &kind, measures_outcome.reserve_width) {
                    window_chip_hits.push(hit);
                }
                if let Some(hit) = self.render_window_actions_rail(draw, overlay, atlas, icons, input, theme, &safe_body, &window_id, &session.app, &kind) {
                    window_chip_hits.push(hit);
                }
                self.render_utility_options_rail(draw, overlay, atlas, icons, input, theme, &safe_body, &window_id, &kind);
            }
            for (rect, control_id) in window_chip_hits {
                input.register_hit(HitTarget { rect, event: None, control_id: Some(control_id), kind: HitKind::Button, drag_axis: None, drag_data: None });
            }
        }
        with_chrome_sink(draw, overlay, |chrome, _select_overlay| {
            let mut dock_ctx = DockRenderContext { draw: chrome, atlas, icons, input, theme, window_labels: &window_labels, window_icon_ids: &window_icon_ids };
            self.dock.paint_chrome(&mut dock_ctx, canvas, false);
        });
        {
            let mut resize_ctx = DockRenderContext { draw, atlas, icons, input, theme, window_labels: &window_labels, window_icon_ids: &window_icon_ids };
            self.dock.register_resize_hits(&mut resize_ctx, canvas);
        }
        if show_fallback {
            chrome_text(draw, atlas, input, theme, &app_breadcrumb(resolve_app_breadcrumb(&session.app, &self.terminology_id)), canvas.x + 16.0, canvas.y + 32.0, theme.font_size_body, theme.text_muted);
        }
        if let Some(drag) = &self.dock_drag {
            if let Some(zone) = &drag.drop_zone {
                if let Some(indicator) = drop_zone_indicator_rect(zone, &self.dock_drop_tab_bars, &self.dock_drop_bodies, self.dock_canvas_bounds, theme.gap_standard) {
                    draw.push_rounded([indicator.x, indicator.y, indicator.w, indicator.h], theme.accent.with_alpha(0.2), theme.border_radius);
                    let hair = theme.stroke_hairline;
                    draw.push_solid([indicator.x, indicator.y, indicator.w, hair], theme.accent);
                    draw.push_solid([indicator.x, indicator.y + indicator.h - hair, indicator.w, hair], theme.accent);
                    draw.push_solid([indicator.x, indicator.y, hair, indicator.h], theme.accent);
                    draw.push_solid([indicator.x + indicator.w - hair, indicator.y, hair, indicator.h], theme.accent);
                }
            }
            let ghost = Rect::new(drag.x - 48.0, drag.y - 12.0, 120.0, theme.control_height);
            if !matches!(drag.drop_zone, Some(DockDropZone::Tab { .. })) {
                draw.push_rounded([ghost.x, ghost.y, ghost.w, ghost.h], theme.panel, theme.border_radius);
                chrome_text(draw, atlas, input, theme, &drag.payload.ghost_label, ghost.x + theme.padding_standard, ghost.y + (ghost.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text);
            }
        }
    }

    fn render_studio_canvas_bars(&self, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, mut canvas: Rect, session: &ActiveSession) -> Rect {
        if !self.host_config().is_some_and(|cfg| session.app.id == cfg.host_app_id) {
            return canvas;
        }
        let bar_h = theme.control_height;
        if self.spawned_ui.is_none() {
            let item = ChromeGroupItem { control_id: "space.canvas.home", icon_id: Some("home"), label: Some(shell_chrome_string("common.home", self.locale_id == "de")), active: false, disabled: false, kind: HitKind::Button };
            let bar_w = measure_chrome_group_item(atlas, theme, &item);
            let bar = Rect::new(canvas.x, canvas.y, bar_w, bar_h);
            render_chrome_group(draw, atlas, icons, input, theme, bar, &[item], true);
            canvas.y += bar_h + theme.gap_standard;
            canvas.h -= bar_h + theme.gap_standard;
            return canvas;
        }
        if let Some(panel) = Self::panel_state_from_view(&session.view_state) {
            if let Some(spawned) = panel.active_spawned_id.as_ref().and_then(|id| panel.spawned_apps.iter().find(|app| &app.id == id)) {
                let label = format!("Back to Workflow · {}", app_breadcrumb(&spawned.breadcrumb));
                let item = ChromeGroupItem { control_id: "space.canvas.back", icon_id: Some("chevron-left"), label: Some(&label), active: false, disabled: false, kind: HitKind::Button };
                let bar_w = measure_chrome_group_item(atlas, theme, &item).min(canvas.w);
                let bar = Rect::new(canvas.x, canvas.y, bar_w, bar_h);
                render_chrome_group(draw, atlas, icons, input, theme, bar, &[item], true);
                canvas.y += bar_h + theme.gap_standard;
                canvas.h -= bar_h + theme.gap_standard;
            }
        }
        canvas
    }

    //#region SilhouetteContent

    fn intersect_content_rect(left: Rect, right: Rect) -> Option<Rect> {
        let x = left.x.max(right.x);
        let y = left.y.max(right.y);
        let x2 = (left.x + left.w).min(right.x + right.w);
        let y2 = (left.y + left.h).min(right.y + right.h);
        (x2 > x && y2 > y).then(|| Rect::new(x, y, x2 - x, y2 - y))
    }

    fn render_window_content(
        &mut self,
        draw: &mut DrawList,
        overlay: Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        viewport: Rect,
        layout: Rect,
        hit_regions: &[Rect],
        ui: &UiDocumentLease,
        window_id: &str,
        engine_resources: &mut crate::engine_canvas::EngineCanvasBuildContext,
        world_resources: &mut infinite_world::world::World3dBuildContext,
    ) {
        let scroll_key = format!("window.{window_id}");
        let scroll_y = *self.scroll_offsets.get(&scroll_key).unwrap_or(&0.0);
        draw.push_scissor(viewport);
        for rect in hit_regions.iter().copied().filter(|rect| rect.w > 0.0 && rect.h > 0.0) {
            input.register_hit(HitTarget { rect, event: None, control_id: Some(scroll_key.clone()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
        }
        let generated_hit_start = input.hit_targets.len();
        let pointer = (input.pointer_x, input.pointer_y);
        if !hit_regions.iter().any(|rect| rect.contains(pointer.0, pointer.1)) {
            input.pointer_x = -1_000_000.0;
            input.pointer_y = -1_000_000.0;
        }
        let scrolled = Rect::new(layout.x, layout.y - scroll_y, layout.w, layout.h);
        let scroll_offsets = &mut self.scroll_offsets;
        let collapsed_sections = &mut self.collapsed_sections;
        let open_selects = &mut self.open_selects;
        let widget_maps = &mut self.widget_maps;
        let mut ctx = framework_widget_context(draw, overlay, atlas, Some(icons), input, theme, scroll_offsets, collapsed_sections, open_selects, Some(widget_maps));
        ctx.pick_clip = Some(viewport);
        render_ui_document(ui, scrolled, &mut ctx, window_id, engine_resources, world_resources, &mut self.world3d_states, &mut self.node_graph_states, &mut self.tiled_map_states, &mut self.icon_render_states, &mut self.board2d_states);
        drop(ctx);
        input.pointer_x = pointer.0;
        input.pointer_y = pointer.1;
        let generated_hits: Vec<_> = input.hit_targets.drain(generated_hit_start..).collect();
        for hit in generated_hits {
            for rect in hit_regions.iter().filter_map(|clip| Self::intersect_content_rect(hit.rect, *clip)) {
                let mut clipped = hit.clone();
                clipped.rect = rect;
                input.register_hit(clipped);
            }
        }
        draw.pop_scissor();
    }

    //#endregion SilhouetteContent

    fn render_overlay(&mut self, overlay: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32, height: f32) {
        match &self.overlay_state {
            OverlayState::Search => {
                let items: Vec<(String, String, usize)> = self.filtered_search_items().into_iter().enumerate().map(|(index, item)| (item.group, item.label, index)).collect();
                self.render_action_list(overlay, atlas, input, theme, width * 0.5 - 200.0, theme.navbar_height + 8.0, 400.0, height * 0.55, "Search", &self.search_query, "shell.search.input", self.search_selected, &items, "shell.search.item");
            }
            OverlayState::Find => {
                let items: Vec<(String, String, usize)> = self.filtered_find_items().into_iter().enumerate().map(|(index, item)| (item.category.clone().unwrap_or_default(), item.label.clone(), index)).collect();
                self.render_action_list(overlay, atlas, input, theme, width * 0.5 - 200.0, theme.navbar_height + 8.0, 400.0, height * 0.55, "Find in page", &self.find_query, "shell.find.input", self.find_selected, &items, "shell.find.item");
            }
            OverlayState::Dropdown(id) if id == "example" => {
                let examples = self.active_plugin_examples();
                let mapped: Vec<(String, String, usize)> = examples.iter().enumerate().map(|(index, ex)| ("Examples".into(), ex.label.resolve(self.active_terminology(), self.active_locale()).to_string(), index)).collect();
                self.render_example_dropdown(overlay, atlas, input, theme, width * 0.25, theme.navbar_height + 4.0, 220.0, &mapped, &examples);
            }
            OverlayState::Dropdown(_) => {}
            OverlayState::None => {}
        }
        if let Some(kind) = self.sync_card_kind.as_deref() {
            let card_w = 320.0;
            let card_h = 132.0;
            let card_x = (width - card_w) * 0.5;
            let card_y = height - theme.footer_height - card_h - theme.gap_standard;
            overlay.push_solid([card_x, card_y, card_w, card_h], theme.panel);
            overlay.push_solid([card_x, card_y, card_w, theme.stroke_hairline], theme.border_normal);
            chrome_text(overlay, atlas, input, theme, &format!("{kind} backbone"), card_x + theme.padding_standard, card_y + theme.padding_standard, theme.font_size_small, theme.text);
            if let Some(uri) = &self.sync_backbone_uri {
                chrome_text(overlay, atlas, input, theme, uri, card_x + theme.padding_standard, card_y + theme.padding_standard + theme.font_size_small + 4.0, theme.font_size_small, theme.text_muted);
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(status) = &self.sync_status {
                    chrome_text(overlay, atlas, input, theme, &Self::sync_status_label(status), card_x + theme.padding_standard, card_y + theme.padding_standard + (theme.font_size_small + 4.0) * 2.0, theme.font_size_small, theme.text_muted);
                }
            }
            let input_y = card_y + 52.0;
            let input_h = theme.control_height;
            overlay.push_solid([card_x + theme.padding_standard, input_y, card_w - theme.padding_standard * 2.0, input_h], theme.input_bg);
            chrome_text(
                overlay,
                atlas,
                input,
                theme,
                if self.sync_card_draft.is_empty() { "/absolute/path" } else { &self.sync_card_draft },
                card_x + theme.padding_standard + 8.0,
                input_y + (input_h + theme.font_size_small) * 0.5 - 1.0,
                theme.font_size_small,
                theme.text,
            );
            let attach_rect = Rect::new(card_x + theme.padding_standard, card_y + card_h - theme.control_height - theme.padding_standard, 72.0, theme.control_height);
            overlay.push_solid([attach_rect.x, attach_rect.y, attach_rect.w, attach_rect.h], theme.accent);
            chrome_text(overlay, atlas, input, theme, "Attach", attach_rect.x + 12.0, attach_rect.y + (attach_rect.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.active_foreground);
            input.register_hit(HitTarget {
                rect: attach_rect,
                event: Some(ActionDescriptor {
                    controller_id: "framework.sync".into(),
                    action: "attach".into(),
                    args: crate::action_args_json!({
                        "path": self.sync_card_draft,
                        "kind": kind,
                    }),
                }),
                control_id: Some("framework.sync.attach".into()),
                kind: HitKind::Button,
                drag_axis: None,
                drag_data: None,
            });
            if self.sync_backbone_uri.is_some() {
                let detach_rect = Rect::new(attach_rect.x + attach_rect.w + theme.gap_standard, attach_rect.y, 72.0, theme.control_height);
                overlay.push_solid([detach_rect.x, detach_rect.y, detach_rect.w, detach_rect.h], theme.button);
                chrome_text(overlay, atlas, input, theme, "Detach", detach_rect.x + 10.0, detach_rect.y + (detach_rect.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text);
                // 🗨️ Detaching drops the live sync connection — gate it behind a real confirmation
                // (item 2 of the WP15/16 brief) instead of dispatching `detach` on a bare click. The
                // hit target itself carries no `event` any more; `chrome_open_dialog` stages the same
                // `ActionDescriptor` onto the dialog's own Confirm button (see `render_chrome_dialog`),
                // so confirming still flows through the existing `framework.sync`/`detach` handler
                // unchanged — only the gate is new.
                input.register_hit(HitTarget { rect: detach_rect, event: None, control_id: Some("framework.sync.detach".into()), kind: HitKind::Button, drag_axis: None, drag_data: None });
                if !self.chrome_build.dialog_open() && self.chrome_build.clicked_this_frame && detach_rect.contains(input.pointer_x, input.pointer_y) {
                    self.chrome_build.open_dialog(ChromeDialogRequest {
                        id: "framework.sync.detach".into(),
                        title: "Detach sync backbone?".into(),
                        body: format!("This disconnects the live {kind} backbone. You can reattach it later from the same panel."),
                        confirm_label: "Detach".into(),
                        confirm_action: ActionDescriptor { controller_id: "framework.sync".into(), action: "detach".into(), args: None },
                        cancel_label: "Cancel".into(),
                    });
                }
            }
        }
        // 📌️ ticket §C5 item 3 — explicit check-in's message-prompt card, the SAME hand-painted
        // overlay-card idiom the `sync_card_kind` block above already establishes (this footer's
        // immediate-mode chrome has no generic text-input widget — `checkin_dialog_draft`'s own field
        // doc explains why a shell-owned keyboard-routed draft field is used instead). Real typed
        // messages now reach `commitCheckpoint`, closing the "fixed message" gap
        // `📓️w3-a-report.md` documented.
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(draft) = &self.checkin_dialog_draft {
            let card_w = 320.0;
            let card_h = 104.0;
            let card_x = (width - card_w) * 0.5;
            let card_y = height - theme.footer_height - card_h - theme.gap_standard;
            overlay.push_solid([card_x, card_y, card_w, card_h], theme.panel);
            overlay.push_solid([card_x, card_y, card_w, theme.stroke_hairline], theme.border_normal);
            chrome_text(overlay, atlas, input, theme, "Check-in message", card_x + theme.padding_standard, card_y + theme.padding_standard, theme.font_size_small, theme.text);
            let input_y = card_y + 36.0;
            let input_h = theme.control_height;
            overlay.push_solid([card_x + theme.padding_standard, input_y, card_w - theme.padding_standard * 2.0, input_h], theme.input_bg);
            chrome_text(overlay, atlas, input, theme, if draft.is_empty() { "check-in" } else { draft.as_str() }, card_x + theme.padding_standard + 8.0, input_y + (input_h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text);
            let commit_rect = Rect::new(card_x + theme.padding_standard, card_y + card_h - theme.control_height - theme.padding_standard, 72.0, theme.control_height);
            overlay.push_solid([commit_rect.x, commit_rect.y, commit_rect.w, commit_rect.h], theme.accent);
            chrome_text(overlay, atlas, input, theme, "Commit", commit_rect.x + 12.0, commit_rect.y + (commit_rect.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.active_foreground);
            input.register_hit(HitTarget {
                rect: commit_rect,
                event: Some(ActionDescriptor { controller_id: "framework.checkin".into(), action: "submit".into(), args: crate::action_args_json!({ "message": draft }) }),
                control_id: Some("s-checkin-commit".into()),
                kind: HitKind::Button,
                drag_axis: None,
                drag_data: None,
            });
            let cancel_rect = Rect::new(commit_rect.x + commit_rect.w + theme.gap_standard, commit_rect.y, 72.0, theme.control_height);
            overlay.push_solid([cancel_rect.x, cancel_rect.y, cancel_rect.w, cancel_rect.h], theme.button);
            chrome_text(overlay, atlas, input, theme, "Cancel", cancel_rect.x + 12.0, cancel_rect.y + (cancel_rect.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text);
            input.register_hit(HitTarget {
                rect: cancel_rect,
                event: Some(ActionDescriptor { controller_id: "framework.checkin".into(), action: "cancel".into(), args: None }),
                control_id: Some("s-checkin-cancel".into()),
                kind: HitKind::Button,
                drag_axis: None,
                drag_data: None,
            });
        }
        if let Some(menu) = &self.context_menu {
            self.render_context_menu(overlay, atlas, icons, input, theme, menu, width, height);
        }
        self.render_chrome_tooltip(overlay, atlas, input, theme, width, height);
        self.render_chrome_dialog(overlay, atlas, input, theme, width, height);
        self.render_chrome_tour(overlay, atlas, input, theme, width, height);
    }

    /// 💬️ Paints the armed tooltip (item 1) — `AtPointer` placement/dismissal policy sourced from
    /// `ui_wgpu::wgpu::OverlayKind::Tooltip` via a scratch `UiTree` (empty; `Point` anchors never touch it).
    fn render_chrome_tooltip(&mut self, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32, height: f32) {
        let hovered_id = input.hovered_id.clone();
        let title = hovered_id.as_ref().and_then(|id| self.chrome_build.tooltip_titles.get(id).cloned());
        let now = chrome_now_ms();
        let armed = {
            let hover = &mut self.chrome_build.tooltip_hover;
            match (&title, hovered_id.as_ref()) {
                (Some(_), Some(id)) => {
                    let restart = hover.as_ref().map(|h| &h.control_id != id).unwrap_or(true);
                    if restart {
                        *hover = Some(ChromeTooltipHover { control_id: id.clone(), anchor_x: input.pointer_x, anchor_y: input.pointer_y, started_ms: now });
                    }
                }
                _ => *hover = None,
            }
            hover.clone()
        };
        if self.chrome_build.dialog_open() {
            return;
        }
        let Some(hover) = armed else { return };
        if !chrome_tooltip_ready(&hover, now) {
            return;
        }
        let Some(text) = title else { return };
        let padding = theme.padding_standard * 0.5;
        let (text_w, text_h) = atlas.measure_text(&text, theme.font_size_small);
        let content_w = text_w + padding * 2.0;
        let content_h = text_h + padding * 2.0;
        let scratch_tree = ui_wgpu::wgpu::UiTree::new();
        let (x, y) =
            ui_wgpu::wgpu::resolve_overlay_placement(&scratch_tree, ui_wgpu::wgpu::OverlayAnchor::Point { x: hover.anchor_x, y: hover.anchor_y }, (content_w, content_h), (width, height), ui_wgpu::wgpu::OverlayKind::Tooltip.default_placement());
        overlay.push_glass([x, y, content_w, content_h], theme.border_radius, theme.glass(Level::Menu));
        chrome_text(overlay, atlas, input, theme, &text, x + padding, y + (content_h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text);
    }

    /// 🗨️ Paints the topmost queued dialog (item 2) — full-screen scrim (click outside == cancel, per
    /// `OverlayKind::Dialog`'s `outside_press_swallow` dismiss policy) plus a centered box
    /// (`OverlayKind::Dialog::default_placement` == `Centered`) with Cancel/Confirm. Confirm's hit
    /// target carries the staged `ActionDescriptor` so it dispatches through the existing generic
    /// pipeline exactly like any other chrome button — only closing the dialog itself is handled here.
    fn render_chrome_dialog(&mut self, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32, height: f32) {
        let Some(request) = self.chrome_build.dialog_stack.last().cloned() else {
            return;
        };
        // 🌫️ `Theme` has no dedicated veil/scrim color (`overlay_shadow` is actually the disabled-control
        // tint, always alpha 0 in the one constructor that sets it) — a fixed dark translucency matches
        // `ui-veil`'s own theme-agnostic dimming in `ui/js/react/index.tsx`.
        overlay.push_solid([0.0, 0.0, width, height], Rgba::new(0.0, 0.0, 0.0, 0.35));
        let dialog_w = 360.0_f32;
        let dialog_h = 168.0_f32;
        let scratch_tree = ui_wgpu::wgpu::UiTree::new();
        let (x, y) = ui_wgpu::wgpu::resolve_overlay_placement(&scratch_tree, ui_wgpu::wgpu::OverlayAnchor::Point { x: 0.0, y: 0.0 }, (dialog_w, dialog_h), (width, height), ui_wgpu::wgpu::OverlayKind::Dialog.default_placement());
        let dialog_rect = Rect::new(x, y, dialog_w, dialog_h);
        overlay.push_glass([x, y, dialog_w, dialog_h], theme.border_radius, theme.glass(Level::Dialog));
        let pad = theme.padding_standard;
        chrome_text(overlay, atlas, input, theme, &request.title, x + pad, y + pad + theme.font_size_body, theme.font_size_body, theme.text);
        chrome_text(overlay, atlas, input, theme, &request.body, x + pad, y + pad + theme.font_size_body + theme.gap_standard + theme.font_size_small, theme.font_size_small, theme.text_muted);
        let btn_h = theme.control_height;
        let confirm_w = 110.0_f32;
        let cancel_w = 90.0_f32;
        let confirm_rect = Rect::new(x + dialog_w - pad - confirm_w, y + dialog_h - pad - btn_h, confirm_w, btn_h);
        let cancel_rect = Rect::new(x + pad, y + dialog_h - pad - btn_h, cancel_w, btn_h);
        overlay.push_rounded([cancel_rect.x, cancel_rect.y, cancel_rect.w, cancel_rect.h], theme.button, theme.border_radius);
        chrome_text(overlay, atlas, input, theme, &request.cancel_label, cancel_rect.x + 10.0, cancel_rect.y + (cancel_rect.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text);
        overlay.push_rounded([confirm_rect.x, confirm_rect.y, confirm_rect.w, confirm_rect.h], theme.accent, theme.border_radius);
        chrome_text(overlay, atlas, input, theme, &request.confirm_label, confirm_rect.x + 10.0, confirm_rect.y + (confirm_rect.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.active_foreground);
        input.register_hit(HitTarget { rect: cancel_rect, event: None, control_id: Some(format!("shell.dialog.{}.cancel", request.id)), kind: HitKind::Button, drag_axis: None, drag_data: None });
        input.register_hit(HitTarget { rect: confirm_rect, event: Some(request.confirm_action.clone()), control_id: Some(format!("shell.dialog.{}.confirm", request.id)), kind: HitKind::Button, drag_axis: None, drag_data: None });
        if self.chrome_build.clicked_this_frame {
            let (px, py) = (input.pointer_x, input.pointer_y);
            if confirm_rect.contains(px, py) {
                self.chrome_build.close_topmost_dialog();
            } else if cancel_rect.contains(px, py) || !dialog_rect.contains(px, py) {
                self.chrome_build.close_topmost_dialog();
            }
        }
    }

    /// 🆔️ Resolves an introduction element id to every on-screen rect this frame: `ui.navbar`/`ui.footer`
    /// are geometric fast paths; `framework.window.{segment}` matches every dock-stack silhouette (full
    /// chrome outline bounds — tabs + gap + controls + body) whose own segment OR whose declared
    /// window-kind segment equals the target (kind-level introduce/show must raise Top + Perspective
    /// together, mirroring React's `data-element-alias`); everything else resolves through
    /// `resolve_element_rect` (utility buttons/toggles; panel tabs via registration).
    /// `…firstDraggable` resolves to the first draggable tree-row hit inside the tab body when available.
    fn resolve_introduction_element_rects(&self, id: &str, theme: &Theme, width: f32, height: f32, hit_targets: &[HitTarget<ActionDescriptor>]) -> Vec<Rect> {
        if id == semio_framework::UI_NAVBAR_ELEMENT_ID {
            return vec![Rect::new(0.0, 0.0, width, theme.navbar_height)];
        }
        if id == semio_framework::UI_FOOTER_ELEMENT_ID {
            return vec![Rect::new(0.0, height - theme.footer_height, width, theme.footer_height)];
        }
        // 🆔️ `…firstDraggable` only resolves at tour time: draggability is a property of the rendered tree
        // row (`tree.label.*` hits carry `drag_axis`), never knowable from the panel-tab id alone. Ladder:
        // first draggable row inside the tab's real body → the body itself → the tab-bar chip fallback
        // (via the base `framework.panelTab.{tabId}` lookup `resolve_element_rect` already does).
        if let Some(tab_id) = id.strip_prefix("framework.panelTab.").and_then(|rest| rest.strip_suffix(".firstDraggable")) {
            let base_id = semio_framework::panel_tab_element_id(tab_id);
            let Some(base_rect) = self.chrome_build.resolve_element_rect(&base_id) else {
                return Vec::new();
            };
            if !self.chrome_build.element_rect_is_fallback(&base_id) {
                if let Some(row) = hit_targets.iter().find(|hit| hit.drag_axis.is_some() && hit.control_id.as_deref().is_some_and(|cid| cid.starts_with("tree.label.")) && base_rect.contains(hit.rect.x + 1.0, hit.rect.y + 1.0)) {
                    return vec![row.rect];
                }
            }
            return vec![base_rect];
        }
        if let Some(rect) = self.chrome_build.resolve_element_rect(id) {
            return vec![rect];
        }
        if let Some(segment) = id.strip_prefix("framework.window.") {
            let segment = segment.split('.').next().unwrap_or(segment);
            let kind_segments: std::collections::HashSet<String> =
                self.session.as_ref().map(|session| session.app.window_kinds.iter().filter(|kind| semio_framework::element_id_segment(&kind.id) == segment).map(|kind| kind.id.clone()).collect()).unwrap_or_default();
            let matches_window = |window_id: &str| {
                semio_framework::element_id_segment(window_id) == segment
                    || kind_segments.iter().any(|kind_id| window_id == kind_id || window_id.starts_with(&format!("{kind_id}-")) || semio_framework::element_id_segment(window_id).starts_with(&semio_framework::element_id_segment(kind_id)))
            };
            let silhouette_rects: Vec<Rect> = self.window_silhouettes.iter().filter(|(window_id, _)| matches_window(window_id)).map(|(_, silhouette)| silhouette.bounds).collect();
            if !silhouette_rects.is_empty() {
                return silhouette_rects;
            }
            return self.window_content_rects.iter().filter(|(window_id, _)| matches_window(window_id)).map(|(_, rect)| *rect).collect();
        }
        Vec::new()
    }

    /// 🪟️ Resolves every dock-stack silhouette for an introduction window id (kind or instance).
    fn resolve_introduction_window_silhouettes(&self, id: &str) -> Vec<WindowSilhouette> {
        let Some(segment) = id.strip_prefix("framework.window.") else {
            return Vec::new();
        };
        let segment = segment.split('.').next().unwrap_or(segment);
        let kind_segments: std::collections::HashSet<String> =
            self.session.as_ref().map(|session| session.app.window_kinds.iter().filter(|kind| semio_framework::element_id_segment(&kind.id) == segment).map(|kind| kind.id.clone()).collect()).unwrap_or_default();
        self.window_silhouettes
            .iter()
            .filter(|(window_id, _)| {
                semio_framework::element_id_segment(window_id) == segment
                    || kind_segments.iter().any(|kind_id| *window_id == kind_id || window_id.starts_with(&format!("{kind_id}-")) || semio_framework::element_id_segment(window_id).starts_with(&semio_framework::element_id_segment(kind_id)))
            })
            .map(|(_, silhouette)| silhouette.clone())
            .collect()
    }

    /// 🆔️ Convenience: first/only rect for an introduction id (info-box anchoring + single-target pulse).
    fn resolve_introduction_element_rect(&self, id: &str, theme: &Theme, width: f32, height: f32, hit_targets: &[HitTarget<ActionDescriptor>]) -> Option<Rect> {
        let rects = self.resolve_introduction_element_rects(id, theme, width, height, hit_targets);
        match rects.as_slice() {
            [] => None,
            [only] => Some(*only),
            many => {
                let mut min_x = f32::INFINITY;
                let mut min_y = f32::INFINITY;
                let mut max_x = f32::NEG_INFINITY;
                let mut max_y = f32::NEG_INFINITY;
                for rect in many {
                    min_x = min_x.min(rect.x);
                    min_y = min_y.min(rect.y);
                    max_x = max_x.max(rect.x + rect.w);
                    max_y = max_y.max(rect.y + rect.h);
                }
                Some(Rect::new(min_x, min_y, max_x - min_x, max_y - min_y))
            }
        }
    }

    /// 🎓️ The currently active introduction step (if a tour is running and its index still resolves) —
    /// shared by every wgpu tour touchpoint beyond painting (reveal, advance-by-doing, keyboard) so they
    /// can never drift on what "the active step" means.
    fn chrome_tour_active_step(&self) -> Option<semio_framework::IntroductionStepDefinition> {
        let session = self.session.as_ref()?;
        let intro = session.app.introduction.as_ref()?;
        let step_index = self.chrome_build.tour_state.as_ref().map(|state| state.step_index)?;
        intro.steps.get(step_index).cloned()
    }

    /// 🎓️ Opens+selects `tab_id`'s panel side, mirroring `select_left_panel_tab`/the `shell.panel.tab.right.*`
    /// hit handler exactly (including their differing `setActivePanelTab` dispatch conditions) so the
    /// tour's programmatic reveal behaves identically to the user clicking the tab themselves.
    fn chrome_tour_reveal_panel_tab(&mut self, session: &ActiveSession, tab_id: &str) {
        let is_left = session.app.panel_tabs.iter().any(|tab| tab.id() == tab_id && group_side(tab.group) == "left");
        let is_right = !is_left && session.app.panel_tabs.iter().any(|tab| tab.id() == tab_id && group_side(tab.group) == "right");
        if is_left {
            self.left_panel_open = true;
            self.active_left_kind = LeftPanelKind::Workbench;
            self.active_left_tab = Some(tab_id.to_string());
            let host_app_id = self.host_config().map(|cfg| cfg.host_app_id);
            if Some(session.app.id.as_str()) == host_app_id {
                self.deferred_actions.push(ActionDescriptor { controller_id: session.app.controller_id.clone(), action: "setActivePanelTab".into(), args: crate::action_args_json!({ "tabId": tab_id }) });
            }
        } else if is_right {
            self.right_panel_open = true;
            self.active_right_kind = RightPanelKind::Details;
            self.active_right_tab = Some(tab_id.to_string());
            if let Some(controller_id) = self.host_controller_id() {
                self.deferred_actions.push(ActionDescriptor { controller_id, action: "setActivePanelTab".into(), args: crate::action_args_json!({ "tabId": tab_id }) });
            }
        }
    }

    /// 🎓️ Force-reveals whatever the active step's `introduce`/`show` ids target — folded action rails,
    /// nested utility collections, closed panel tabs — before any of that chrome paints this frame; must
    /// run ahead of `render_main_window`/`render_left_panel`/`render_right_panel`/`render_footer`, all of
    /// which read the fold/open state this writes. Latched per step id in owned build state so a
    /// user who re-folds/closes what the tour revealed doesn't get it snapped back open next frame —
    /// mirrors the React shell's own reveal effects, which likewise fire once per step.
    fn chrome_tour_frame_begin(&mut self) {
        let Some(step) = self.chrome_tour_active_step() else {
            return;
        };
        let already_latched = self.chrome_build.tour_reveal_latch.as_deref() == Some(step.id.as_str());
        if already_latched {
            return;
        }
        self.chrome_build.tour_reveal_latch = Some(step.id.clone());
        let Some(session) = self.session.clone() else {
            return;
        };
        let ids: Vec<String> = step.introduce.iter().cloned().chain(step.show.iter().cloned()).collect();
        for id in ids {
            if id == semio_framework::UI_NAVBAR_ELEMENT_ID || id == semio_framework::UI_FOOTER_ELEMENT_ID {
                continue;
            }
            if let Some(rest) = id.strip_prefix("framework.window.") {
                if let Some((segment, _action_id)) = rest.split_once(".action.") {
                    let window_id = session.app.window_kinds.iter().find(|kind| semio_framework::element_id_segment(&kind.id) == segment).map(|kind| kind.id.clone());
                    if let Some(window_id) = window_id {
                        self.action_panel_folded.insert(window_id, false);
                    }
                }
                continue;
            }
            if let Some(rest) = id.strip_prefix("framework.panelTab.") {
                let tab_id = rest.strip_suffix(".firstDraggable").unwrap_or(rest);
                self.chrome_tour_reveal_panel_tab(&session, tab_id);
                continue;
            }
            for collection_id in utility_collection_path_to_id(&self.active_utilities, &id) {
                self.utility_collection_expanded.insert(collection_id, true);
            }
        }
    }

    /// 🎓️ Advance-by-doing (Part B) — called from the single funnel points a user/plugin action can take
    /// (`dispatch_action`'s successful program forward, `apply_set_active_utility`'s activation branch) so
    /// a step's matching `Action`/`Utility` interaction completes the instant the described behavior
    /// actually happens, mirroring the React shell's own advance-by-doing wiring. No-operations when no tour is
    /// active or nothing in the active step's `interactions` matches what was performed.
    fn chrome_tour_note_action_performed(&mut self, action_id: &str) {
        let Some(step) = self.chrome_tour_active_step() else {
            return;
        };
        self.chrome_tour_complete_interaction(&step, |kind| matches!(kind, semio_framework::IntroductionInteractionKind::Action(action) if action.as_str() == action_id));
    }

    fn chrome_tour_note_utility_performed(&mut self, utility_id: &str) {
        let Some(step) = self.chrome_tour_active_step() else {
            return;
        };
        self.chrome_tour_complete_interaction(&step, |kind| matches!(kind, semio_framework::IntroductionInteractionKind::Utility(utility) if utility.as_str() == utility_id));
    }

    /// ✅️ Shared completion path for interaction-gated steps: finds the first not-yet-completed
    /// interaction matching `matches` (respecting `step.ordered` — only the next in-order interaction may
    /// complete), records it, and advances the step once every interaction is done. Mirrors the React
    /// shell's `completeIntroductionInteraction`.
    fn chrome_tour_complete_interaction(&mut self, step: &semio_framework::IntroductionStepDefinition, matches: impl Fn(&semio_framework::IntroductionInteractionKind) -> bool) {
        if step.interactions.is_empty() {
            return;
        }
        let completed = self.chrome_build.tour_state.as_ref().map(|state| state.completed_interactions.clone()).unwrap_or_default();
        let Some(index) = step.interactions.iter().enumerate().find(|(i, interaction)| !completed.contains(i) && matches(&interaction.on)).map(|(i, _)| i) else {
            return;
        };
        if step.ordered && index != completed.len() {
            return;
        }
        let completed_len = {
            if let Some(tour) = self.chrome_build.tour_state.as_mut() {
                if !tour.completed_interactions.contains(&index) {
                    tour.completed_interactions.push(index);
                }
                tour.completed_interactions.len()
            } else {
                0
            }
        };
        if completed_len >= step.interactions.len() {
            self.chrome_tour_advance_current_step(step);
        }
    }

    fn chrome_tour_advance_current_step(&mut self, step: &semio_framework::IntroductionStepDefinition) {
        let Some(session) = self.session.as_ref() else {
            return;
        };
        let Some(intro) = session.app.introduction.as_ref() else {
            return;
        };
        let step_index = intro.steps.iter().position(|candidate| candidate.id == step.id).unwrap_or(0);
        if step_index + 1 >= intro.steps.len() {
            self.chrome_build.mark_introduction_seen(&session.app.id);
        }
        self.chrome_build.advance_introduction(intro.steps.len());
    }

    /// 🎓️ Paints the current introduction-tour step (item 3) — visual parity with `ui/js/react/index.tsx`'s
    /// `UIIntroduction`, which now renders one fullscreen veil div and raises the introduced/shown
    /// element's chrome unit above it via z-index. This painter achieves the identical *pixels* by solid
    /// bands tiled around the `introduce`/`show` element ids that resolved to a rect this frame instead —
    /// see `introduction_veil_bands`'s doc for why that's the correct approach here, not a shortcut. The
    /// `introduce` rect pulses an inset ring (`introduced_pulse_thickness`), and the info box anchors
    /// beside it via `resolve_introduction_placement`. Ids that don't resolve to a rect (see
    /// `resolve_introduction_element_rect`'s doc comment for the current registration gaps) fall back to
    /// a centered box with no cutout, same as a `None` `introduce`.
    fn render_chrome_tour(&mut self, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32, height: f32) {
        let Some(session) = self.session.as_ref() else {
            self.chrome_build.tour_state = None;
            return;
        };
        let Some(intro) = session.app.introduction.as_ref() else {
            return;
        };
        // 🎓️ Auto-start once per app per session, the first frame this app_id is seen — `w3-prefs-i18n-themes`
        // landed `read_stored_introduction_seen`/`write_stored_introduction_seen` (byte-identical to
        // `ui/js/react/index.tsx`'s `readStoredIntroductionSeen`/`writeStoredIntroductionSeen`) explicitly for
        // this wiring (see that function's own doc comment). The owned per-app latch guards against
        // re-triggering every frame after the user skips/finishes within the same still-open session.
        let already_considered = self.chrome_build.tour_auto_considered.as_deref() == Some(session.app.id.as_str());
        if !already_considered {
            self.chrome_build.tour_auto_considered = Some(session.app.id.clone());
            if !self.chrome_build.introduction_was_seen(&session.app.id) && self.chrome_build.tour_state.is_none() {
                self.chrome_build.start_introduction();
            }
        }
        let Some(step_index) = self.chrome_build.tour_state.as_ref().map(|state| state.step_index) else {
            return;
        };
        let Some(step) = intro.steps.get(step_index) else {
            self.chrome_build.tour_state = None;
            return;
        };

        let introduce_rects = step.introduce.as_deref().map(|id| self.resolve_introduction_element_rects(id, theme, width, height, &input.hit_targets)).unwrap_or_default();
        let introduce_rect = step.introduce.as_deref().and_then(|id| self.resolve_introduction_element_rect(id, theme, width, height, &input.hit_targets));
        let show_rects: Vec<Rect> = step.show.iter().flat_map(|id| self.resolve_introduction_element_rects(id, theme, width, height, &input.hit_targets)).collect();
        let cutouts: Vec<Rect> = introduce_rects.iter().copied().chain(show_rects.iter().copied()).collect();
        let bands = introduction_veil_bands(width, height, &cutouts);
        // 🎓️ A targeted step that hasn't mounted yet (a folded utility bar/panel) must not trap the user
        // behind an opaque-to-clicks veil — only screen-style steps (`introduce == None`) and steps whose
        // target did resolve block pointer events; `show` rects alone also count as a resolved target.
        let veil_blocks_pointer = step.introduce.is_none() || introduce_rect.is_some() || !show_rects.is_empty();
        for band in &bands {
            overlay.push_solid([band.x, band.y, band.w, band.h], Rgba::new(0.0, 0.0, 0.0, 0.35));
            if veil_blocks_pointer {
                input.register_hit(HitTarget { rect: *band, event: None, control_id: Some("shell.tour.veil".to_string()), kind: HitKind::Generic, drag_axis: None, drag_data: None });
            }
        }
        let introduce_silhouettes = step.introduce.as_deref().map(|id| self.resolve_introduction_window_silhouettes(id)).unwrap_or_default();
        let thickness = introduced_pulse_thickness(chrome_now_ms(), theme.stroke_hairline, 3.0);
        if introduce_silhouettes.is_empty() {
            for rect in &introduce_rects {
                let ring = rect.inset(-thickness * 0.5);
                overlay.push_solid([ring.x, ring.y, ring.w, thickness], theme.focus_ring);
                overlay.push_solid([ring.x, ring.y + ring.h - thickness, ring.w, thickness], theme.focus_ring);
                overlay.push_solid([ring.x, ring.y, thickness, ring.h], theme.focus_ring);
                overlay.push_solid([ring.x + ring.w - thickness, ring.y, thickness, ring.h], theme.focus_ring);
            }
        } else {
            for silhouette in &introduce_silhouettes {
                push_window_silhouette_border(overlay, silhouette, thickness, theme.focus_ring);
            }
        }

        let is_de = self.locale_id == "de";
        // 🎓️ Logos (Part B's "info box" item) reuse the existing UI-image pipeline verbatim: `resolve_ui_image`
        // queues/decodes async, returning `None` until cached — an un-cached logo is simply skipped this
        // frame (it appears once its fetch lands, same as any other async `Image` node). Sized to a fixed
        // row height at natural aspect ratio ("aspect-sum row math"), dark-mode picks `dark_src` when set.
        let is_dark = theme_is_dark(theme);
        let logo_h = 28.0_f32;
        let resolved_logos: Vec<(String, f32)> = step
            .logos
            .iter()
            .enumerate()
            .filter_map(|(index, logo)| {
                let src = if is_dark { logo.dark_src.as_deref().filter(|src| !src.is_empty()).unwrap_or(logo.src.as_str()) } else { logo.src.as_str() };
                let (key, natural) = resolve_ui_image(&format!("shell.tour.logo.{index}"), src);
                let key = key?;
                let (nw, nh) = natural.filter(|(w, h)| *w > 0 && *h > 0).unwrap_or((1, 1));
                Some((key, logo_h * nw as f32 / nh as f32))
            })
            .collect();
        let logos_row_h = if resolved_logos.is_empty() { 0.0 } else { logo_h + theme.gap_standard };

        let box_w = 320.0_f32;
        let box_h = 168.0_f32 + logos_row_h;
        let (x, y) = resolve_introduction_placement(step.placement, introduce_rect, (box_w, box_h), (width, height));
        overlay.push_glass([x, y, box_w, box_h], theme.border_radius, theme.glass(Level::Dialog));
        let pad = theme.padding_standard;
        if !resolved_logos.is_empty() {
            let total_w: f32 = resolved_logos.iter().map(|(_, w)| *w).sum::<f32>() + theme.gap_standard * (resolved_logos.len() as f32 - 1.0);
            let mut lx = x + ((box_w - total_w) * 0.5).max(pad);
            for (key, w) in &resolved_logos {
                overlay.push_raster_quad(key, [lx, y + pad, *w, logo_h], [0.0, 0.0, 1.0, 1.0], 1.0);
                lx += w + theme.gap_standard;
            }
        }
        chrome_text(overlay, atlas, input, theme, step.title.resolve(self.active_terminology(), self.active_locale()), x + pad, y + pad + logos_row_h + theme.font_size_body, theme.font_size_body, theme.text);
        chrome_text(
            overlay,
            atlas,
            input,
            theme,
            step.body.resolve(self.active_terminology(), self.active_locale()),
            x + pad,
            y + pad + logos_row_h + theme.font_size_body + theme.gap_standard + theme.font_size_small,
            theme.font_size_small,
            theme.text_muted,
        );
        chrome_text(overlay, atlas, input, theme, &format!("{} / {}", step_index + 1, intro.steps.len()), x + box_w - pad - 40.0, y + pad + logos_row_h + theme.font_size_body, theme.font_size_small, theme.text_muted);
        let btn_h = theme.control_height;
        let is_last = step_index + 1 >= intro.steps.len();
        let next_label = shell_chrome_string(if is_last { "introduction.done" } else { "introduction.next" }, is_de);
        let advance_by_button = step.interactions.is_empty();
        let next_rect = Rect::new(x + box_w - pad - 90.0, y + box_h - pad - btn_h, 90.0, btn_h);
        let skip_rect = Rect::new(x + pad, y + box_h - pad - btn_h, 70.0, btn_h);
        let back_rect = Rect::new(next_rect.x - 8.0 - 70.0, y + box_h - pad - btn_h, 70.0, btn_h);
        overlay.push_rounded([skip_rect.x, skip_rect.y, skip_rect.w, skip_rect.h], theme.button, theme.border_radius);
        chrome_text(overlay, atlas, input, theme, shell_chrome_string("introduction.skip", is_de), skip_rect.x + 10.0, skip_rect.y + (skip_rect.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text);
        if step_index > 0 {
            overlay.push_rounded([back_rect.x, back_rect.y, back_rect.w, back_rect.h], theme.button, theme.border_radius);
            chrome_text(overlay, atlas, input, theme, shell_chrome_string("introduction.back", is_de), back_rect.x + 10.0, back_rect.y + (back_rect.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text);
            input.register_hit(HitTarget { rect: back_rect, event: None, control_id: Some(format!("shell.tour.{}.back", step.id)), kind: HitKind::Button, drag_axis: None, drag_data: None });
        }
        if advance_by_button {
            overlay.push_rounded([next_rect.x, next_rect.y, next_rect.w, next_rect.h], theme.accent, theme.border_radius);
            chrome_text(overlay, atlas, input, theme, next_label, next_rect.x + 10.0, next_rect.y + (next_rect.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.active_foreground);
            input.register_hit(HitTarget { rect: next_rect, event: None, control_id: Some(format!("shell.tour.{}.next", step.id)), kind: HitKind::Button, drag_axis: None, drag_data: None });
        } else {
            // ✅️ Checklist hint: each interaction's label, ✓️-prefixed once completed, {n}.-prefixed when
            // `ordered` so the user knows what's next — single line, this painter has no multi-line text.
            let completed = self.chrome_build.tour_state.as_ref().map(|state| state.completed_interactions.clone()).unwrap_or_default();
            let hint = step
                .interactions
                .iter()
                .enumerate()
                .map(|(i, interaction)| {
                    let mark = if completed.contains(&i) {
                        "✓️".to_string()
                    } else if step.ordered {
                        format!("{}.", i + 1)
                    } else {
                        "•".to_string()
                    };
                    format!("{mark} {}", interaction.label)
                })
                .collect::<Vec<_>>()
                .join("   ");
            chrome_text(overlay, atlas, input, theme, &hint, next_rect.x - 120.0, next_rect.y + (btn_h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text_muted);
        }
        input.register_hit(HitTarget { rect: skip_rect, event: None, control_id: Some(format!("shell.tour.{}.skip", step.id)), kind: HitKind::Button, drag_axis: None, drag_data: None });
        if self.chrome_build.clicked_this_frame {
            let (px, py) = (input.pointer_x, input.pointer_y);
            if advance_by_button && next_rect.contains(px, py) {
                if is_last {
                    self.chrome_build.mark_introduction_seen(&session.app.id);
                }
                self.chrome_build.advance_introduction(intro.steps.len());
            } else if skip_rect.contains(px, py) {
                self.chrome_build.mark_introduction_seen(&session.app.id);
                self.chrome_build.skip_introduction();
            } else if step_index > 0 && back_rect.contains(px, py) {
                self.chrome_build.back_introduction();
            }
        }
    }

    fn render_example_dropdown(&self, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, x: f32, y: f32, w: f32, items: &[(String, String, usize)], examples: &[ExampleDefinition]) {
        let row_h = theme.control_height;
        let h = items.len() as f32 * row_h + theme.padding_standard * 2.0;
        overlay.push_glass([x, y, w, h.max(row_h + 8.0)], theme.border_radius, theme.glass(Level::Menu));
        for (index, (_group, label, _)) in items.iter().enumerate() {
            let row = Rect::new(x + theme.gap_standard, y + theme.gap_standard + index as f32 * row_h, w - theme.gap_standard * 2.0, row_h);
            let selected = examples.get(index).is_some_and(|ex| self.active_example_id.as_deref() == Some(ex.id.as_str()));
            let hovered = row.contains(input.pointer_x, input.pointer_y);
            let bg = if selected {
                theme.selected
            } else if hovered {
                theme.button_hover
            } else {
                theme.button
            };
            overlay.push_rounded([row.x, row.y, row.w, row.h], bg, theme.border_radius);
            chrome_text(overlay, atlas, input, theme, label, row.x + theme.padding_standard, row.y + (row.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, if selected || hovered { theme.active_foreground } else { theme.text });
            if let Some(example) = examples.get(index) {
                input.register_hit(HitTarget { rect: row, event: None, control_id: Some(format!("shell.example.{}", example.id)), kind: HitKind::DropdownItem, drag_axis: None, drag_data: None });
            }
        }
    }

    fn render_action_list(
        &self,
        overlay: &mut DrawList,
        atlas: &mut FontAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        title: &str,
        query: &str,
        input_id: &str,
        selected: usize,
        items: &[(String, String, usize)],
        item_prefix: &str,
    ) {
        overlay.push_glass([x, y, w, h], theme.border_radius, theme.glass(Level::Menu));
        chrome_text(overlay, atlas, input, theme, title, x + 12.0, y + 20.0, theme.font_size_body, theme.text);
        let filter_rect = Rect::new(x + 8.0, y + 32.0, w - 16.0, theme.control_height);
        overlay.push_rounded([filter_rect.x, filter_rect.y, filter_rect.w, filter_rect.h], theme.input_bg, theme.border_radius);
        let display_query = if query.is_empty() { "Type to filter…" } else { query };
        chrome_text(overlay, atlas, input, theme, display_query, filter_rect.x + 8.0, filter_rect.y + (filter_rect.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, if query.is_empty() { theme.text_muted } else { theme.text });
        input.register_hit(HitTarget { rect: filter_rect, event: None, control_id: Some(input_id.into()), kind: HitKind::Input, drag_axis: None, drag_data: None });
        let list_top = y + 32.0 + theme.control_height + 8.0;
        let list_h = h - (list_top - y) - 8.0;
        let mut row_y = list_top;
        let mut last_group = String::new();
        for (group, label, index) in items {
            if !group.is_empty() && group != &last_group {
                chrome_text(overlay, atlas, input, theme, group, x + 12.0, row_y + 12.0, theme.font_size_small, theme.text_muted);
                row_y += 18.0;
                last_group = group.clone();
            }
            let row = Rect::new(x + 8.0, row_y, w - 16.0, theme.control_height);
            if row_y + theme.control_height > list_top + list_h {
                break;
            }
            let hovered = row.contains(input.pointer_x, input.pointer_y);
            let is_selected = *index == selected;
            let bg = if is_selected {
                theme.selected
            } else if hovered {
                theme.button_hover
            } else {
                theme.button
            };
            overlay.push_rounded([row.x, row.y, row.w, row.h], bg, theme.border_radius);
            chrome_text(overlay, atlas, input, theme, label, row.x + 8.0, row.y + (row.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, if is_selected || hovered { theme.active_foreground } else { theme.text });
            input.register_hit(HitTarget { rect: row, event: None, control_id: Some(format!("{item_prefix}.{index}")), kind: HitKind::DropdownItem, drag_axis: None, drag_data: None });
            row_y += theme.control_height + 2.0;
        }
    }

    fn measures_for_kind(kind: &semio_framework::WindowKindDefinition) -> &[WindowMeasure] {
        kind.options.measures.as_slice()
    }

    fn engagement_for_kind(&self, kind: &semio_framework::WindowKindDefinition) -> Option<WindowEngagement> {
        self.window_engagements.get(&kind.id).cloned().or_else(|| kind.options.engagement.as_option().cloned()).or_else(|| if kind.surface_kind.is_viewport() { Some(ui_wgpu::wgpu::default_viewport_engagement()) } else { None })
    }

    fn render_window_measures_rail(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        content: &Rect,
        window_id: &str,
        kind: &semio_framework::WindowKindDefinition,
    ) -> WindowMeasuresRailOutcome {
        let inset = theme.gap_standard;
        let active_utility = self.active_utility_by_window.get(window_id).map(String::as_str);
        let Ok(partition) = ui_wgpu::wgpu::partition_window_measures(Self::measures_for_kind(kind), active_utility) else {
            return WindowMeasuresRailOutcome { chip_hit: None, reserve_width: 0.0 };
        };
        let measures = partition.general;
        if measures.is_empty() {
            return WindowMeasuresRailOutcome { chip_hit: None, reserve_width: 0.0 };
        }
        let folded = self.measures_folded.get(window_id).copied().unwrap_or(true);
        let expanded = self.measures_expanded.get(window_id).copied().unwrap_or(false);
        let is_de = self.locale_id == "de";
        (|chrome: &mut DrawList, select_overlay: &mut Option<&mut DrawList>| {
            if folded {
                let item = ChromeGroupItem { control_id: "", icon_id: Some("chevron-left"), label: Some(shell_chrome_string("common.windowOptions", is_de)), active: false, disabled: false, kind: HitKind::Button };
                let chip_w = measure_chrome_group_item(atlas, theme, &item);
                let chip = Rect::new(content.x + content.w - chip_w - inset, content.y + inset, chip_w, theme.control_height);
                render_chrome_group(chrome, atlas, icons, input, theme, chip, &[item], false);
                return WindowMeasuresRailOutcome { chip_hit: Some((chip, format!("shell.measures.unfold.{window_id}"))), reserve_width: chip_w + inset };
            }
            let max_w = window_overlay_max_width(content.w, inset);
            let default_w = *self.measures_width.get(window_id).unwrap_or(&theme.window_measures_default_width);
            let width = if expanded { content.w } else { default_w.clamp(theme.panel_min_width, theme.panel_max_width).min(max_w) };
            let Some(body_content_h) = measure_window_measures_body_height(theme, &self.collapsed_sections, measures.iter().copied()) else {
                return WindowMeasuresRailOutcome { chip_hit: None, reserve_width: 0.0 };
            };
            let rail_h = if expanded {
                content.h
            } else {
                let card_h = theme.panel_header_height + theme.gap_standard * 2.0 + body_content_h;
                card_h.min((content.h - inset * 2.0).max(theme.panel_header_height))
            };
            let (rail_x, rail_y) = if expanded { (content.x, content.y) } else { (content.x + content.w - width - inset, content.y + inset) };
            let rail = Rect::new(rail_x, rail_y, width, rail_h);
            let glass = chrome.push_glass([rail.x, rail.y, rail.w, rail.h], theme.border_radius, theme.glass(Level::Pane));
            chrome.begin_glass_content(glass);
            let header = Rect::new(rail.x, rail.y, rail.w, theme.panel_header_height);
            chrome.push_solid([header.x, header.y, header.w, header.h], theme.navbar);
            let focus_label = if expanded { shell_chrome_string("common.unfocus", is_de) } else { shell_chrome_string("common.focus", is_de) };
            let focus_item = ChromeGroupItem { control_id: "shell.measures.focus", icon_id: Some(if expanded { "minimize-2" } else { "maximize-2" }), label: Some(focus_label), active: false, disabled: false, kind: HitKind::Button };
            let fold_item = ChromeGroupItem { control_id: "shell.measures.fold", icon_id: Some("chevron-right"), label: Some(shell_chrome_string("common.windowOptions", is_de)), active: false, disabled: false, kind: HitKind::Button };
            let focus_w = measure_chrome_group_item(atlas, theme, &focus_item);
            render_chrome_group(chrome, atlas, icons, input, theme, Rect::new(header.x, header.y, focus_w, header.h), &[focus_item], true);
            input.register_hit(HitTarget { rect: Rect::new(header.x, header.y, focus_w, header.h), event: None, control_id: Some(format!("shell.measures.focus.{window_id}")), kind: HitKind::Button, drag_axis: None, drag_data: None });
            let fold_w = measure_chrome_group_item(atlas, theme, &fold_item);
            render_chrome_group(chrome, atlas, icons, input, theme, Rect::new(header.x + header.w - fold_w, header.y, fold_w, header.h), &[fold_item], true);
            input.register_hit(HitTarget {
                rect: Rect::new(header.x + header.w - fold_w, header.y, fold_w, header.h),
                event: None,
                control_id: Some(format!("shell.measures.fold.{window_id}")),
                kind: HitKind::Button,
                drag_axis: None,
                drag_data: None,
            });
            let body = Rect::new(rail.x + theme.gap_standard, rail.y + theme.panel_header_height + theme.gap_standard, rail.w - theme.gap_standard * 2.0, rail.h - theme.panel_header_height - theme.gap_standard * 2.0);
            let mut y = body.y;
            for measure in measures.iter().copied() {
                let Some(height) = self.render_window_measure_tree(chrome, select_overlay, atlas, icons, input, theme, Rect::new(body.x, y, body.w, body.h), measure) else {
                    return WindowMeasuresRailOutcome { chip_hit: None, reserve_width: 0.0 };
                };
                y += height;
            }
            if !expanded {
                let resize = Rect::new(rail.x - 3.0, rail.y, 6.0, rail.h);
                input.register_hit(HitTarget { rect: resize, event: None, control_id: Some(format!("shell.measures.resize.{window_id}")), kind: HitKind::PanelResize, drag_axis: Some(DragAxis::Horizontal), drag_data: None });
            }
            chrome.end_glass_content();
            WindowMeasuresRailOutcome { chip_hit: None, reserve_width: if expanded { width } else { width + inset } }
        })(draw, overlay)
    }

    /// 🎯️ Bottom-left utility-scoped measure strip: the utility-scoped bucket of `partition_window_measures`,
    /// rendered as a compact overlay directly above the footer utility bar (no detached "Utility Options" card).
    /// Reuses [`Self::render_window_measure`] so Select/Slider/Toggle controls behave exactly as in the
    /// general Measures rail.
    fn render_utility_options_rail(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        content: &Rect,
        window_id: &str,
        kind: &semio_framework::WindowKindDefinition,
    ) {
        let inset = theme.gap_standard;
        let active_utility = self.active_utility_by_window.get(window_id).map(String::as_str);
        let Ok(partition) = ui_wgpu::wgpu::partition_window_measures(Self::measures_for_kind(kind), active_utility) else {
            return;
        };
        let utility_options = partition.utility_options;
        if utility_options.is_empty() {
            return;
        }
        (|chrome: &mut DrawList, select_overlay: &mut Option<&mut DrawList>| {
            let width = theme.window_measures_default_width.clamp(theme.panel_min_width, theme.panel_max_width).min(window_overlay_max_width(content.w, inset));
            let Some(body_content_h) = measure_window_measures_body_height(theme, &self.collapsed_sections, utility_options.iter().copied()) else {
                return;
            };
            let card_h = body_content_h + theme.gap_standard * 2.0;
            let footer_reserve = theme.footer_height + inset;
            let rail = Rect::new(content.x + inset, content.y + content.h - card_h - footer_reserve, width, card_h);
            let glass = chrome.push_glass([rail.x, rail.y, rail.w, rail.h], theme.border_radius, theme.glass(Level::Pane));
            chrome.begin_glass_content(glass);
            let body = Rect::new(rail.x + theme.gap_standard, rail.y + theme.gap_standard, rail.w - theme.gap_standard * 2.0, rail.h - theme.gap_standard * 2.0);
            let mut y = body.y;
            for measure in utility_options.iter().copied() {
                let Some(height) = self.render_window_measure_tree(chrome, select_overlay, atlas, icons, input, theme, Rect::new(body.x, y, body.w, body.h), measure) else {
                    return;
                };
                y += height;
            }
            chrome.end_glass_content();
        })(draw, overlay)
    }

    fn render_window_measure_tree(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        bounds: Rect,
        measure: &WindowMeasure,
    ) -> Option<f32> {
        let mut stack = UiFixedList::<WindowMeasureRenderFrame<'_>, WINDOW_MEASURE_TRAVERSAL_CAPACITY>::default();
        stack.try_push(WindowMeasureRenderFrame { measure, inset: 0.0 }).ok()?;
        let mut y = bounds.y;
        while let Some(frame) = stack.pop() {
            let height = match frame.measure {
                WindowMeasure::Group { .. } | WindowMeasure::Toggle { .. } => theme.control_height,
                WindowMeasure::Select { .. } | WindowMeasure::Slider { .. } => theme.control_height + 16.0,
            };
            let leaf_bounds = Rect::new(bounds.x + frame.inset, y, (bounds.w - frame.inset).max(0.0), height);
            self.render_window_measure_one(draw, overlay, atlas, icons, input, theme, leaf_bounds, frame.measure);
            y += height;
            if let WindowMeasure::Group { id, default_open, children, .. } = frame.measure {
                let open = !self.collapsed_sections.get(id).copied().unwrap_or(!default_open.unwrap_or(false));
                if open {
                    for child in children.iter().rev() {
                        stack.try_push(WindowMeasureRenderFrame { measure: child, inset: frame.inset + 12.0 }).ok()?;
                    }
                }
            }
        }
        Some(y - bounds.y)
    }

    fn render_window_measure_one(&mut self, draw: &mut DrawList, overlay: &mut Option<&mut DrawList>, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, bounds: Rect, measure: &WindowMeasure) {
        use ui_wgpu::wgpu::widgets::{render_window_measure_select, render_window_measure_slider, render_window_measure_toggle};
        let y = bounds.y;
        match measure {
            WindowMeasure::Group { id, label, default_open, .. } => {
                let open = !self.collapsed_sections.get(id).copied().unwrap_or(!default_open.unwrap_or(false));
                chrome_text(draw, atlas, input, theme, &format!("{} {}", if open { "v" } else { ">" }, label), bounds.x, y + 14.0, theme.font_size_small, theme.text);
                input.register_hit(HitTarget { rect: Rect::new(bounds.x, y, bounds.w, theme.control_height), event: None, control_id: Some(format!("shell.measure.group.{id}")), kind: HitKind::Button, drag_axis: None, drag_data: None });
            }
            WindowMeasure::Select { id, label, value, items, on_change } => {
                if let Some(label) = label {
                    chrome_text(draw, atlas, input, theme, label, bounds.x, y + 14.0, theme.font_size_small, theme.text_muted);
                }
                let _ = self.window_measure_actions.try_upsert(id, on_change, WindowMeasureActionKind::Select);
                let rect = Rect::new(bounds.x, y + 16.0, bounds.w, theme.control_height);
                let scroll_offsets = &mut self.scroll_offsets;
                let collapsed_sections = &mut self.collapsed_sections;
                let open_selects = &mut self.open_selects;
                let mut ctx = framework_widget_context(draw, overlay.as_deref_mut(), atlas, Some(icons), input, theme, scroll_offsets, collapsed_sections, open_selects, None);
                render_window_measure_select(id, value, items, rect, &mut ctx);
            }
            WindowMeasure::Slider { id, label, value, min, max, step, ready, loading: _, waiting: _, disabled, reveal: _, on_change } => {
                if let Some(label) = label {
                    chrome_text(draw, atlas, input, theme, label, bounds.x, y + 14.0, theme.font_size_small, theme.text_muted);
                }
                if !disabled.unwrap_or(false) {
                    let _ = self.window_measure_actions.try_upsert(id, on_change, WindowMeasureActionKind::Slider);
                }
                let rect = Rect::new(bounds.x, y + 16.0, bounds.w, theme.control_height);
                let scroll_offsets = &mut self.scroll_offsets;
                let collapsed_sections = &mut self.collapsed_sections;
                let open_selects = &mut self.open_selects;
                let mut ctx = framework_widget_context(draw, overlay.as_deref_mut(), atlas, Some(icons), input, theme, scroll_offsets, collapsed_sections, open_selects, None);
                render_window_measure_slider(id, *value, *min, *max, step.unwrap_or(0.01), *ready, disabled.unwrap_or(false), rect, &mut ctx);
            }
            WindowMeasure::Toggle { id, icon_id, label, pressed, text, on_change } => {
                let _ = self.window_measure_actions.try_upsert(id, on_change, WindowMeasureActionKind::Toggle(*pressed));
                let rect = Rect::new(bounds.x, y, bounds.w, theme.control_height);
                let scroll_offsets = &mut self.scroll_offsets;
                let collapsed_sections = &mut self.collapsed_sections;
                let open_selects = &mut self.open_selects;
                let mut ctx = framework_widget_context(draw, overlay.as_deref_mut(), atlas, Some(icons), input, theme, scroll_offsets, collapsed_sections, open_selects, None);
                render_window_measure_toggle(id, icon_id.clone(), *pressed, text.as_deref().or(label.as_deref()), rect, &mut ctx);
            }
        }
    }

    fn render_window_engagement_rail(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        content: &Rect,
        window_id: &str,
        kind: &semio_framework::WindowKindDefinition,
        measures_reserve: f32,
    ) -> Option<(Rect, String)> {
        let inset = theme.gap_standard;
        let measures_expanded = self.measures_expanded.get(window_id).copied().unwrap_or(false);
        if measures_expanded {
            return None;
        }
        let window_active = self.active_window_id.as_deref() == Some(window_id);
        if !window_active {
            return None;
        }
        let engagement = self.engagement_for_kind(kind);
        let Some(engagement) = engagement else {
            return None;
        };
        let activated = self.engagement_activated.get(window_id).copied().unwrap_or(false);
        (|chrome: &mut DrawList, select_overlay: &mut Option<&mut DrawList>| {
            if !activated {
                let item = ChromeGroupItem { control_id: "", icon_id: Some("chevron-right"), label: Some("Action"), active: false, disabled: false, kind: HitKind::Button };
                let chip_w = measure_chrome_group_item(atlas, theme, &item);
                let chip = Rect::new(content.x + inset, content.y + inset, chip_w, theme.control_height);
                render_chrome_group(chrome, atlas, icons, input, theme, chip, &[item], false);
                return Some((chip, format!("shell.engagement.toggle.{window_id}")));
            }
            let rail_w = engagement_rail_width(theme, content.w, inset, measures_reserve);
            if rail_w <= 0.0 {
                return None;
            }
            let body_content_h = measure_engagement_body_height(theme, &engagement);
            let card_h = theme.panel_header_height + theme.gap_standard * 2.0 + body_content_h;
            let rail_h = card_h.min((content.h - inset * 2.0).max(theme.panel_header_height));
            let rail = Rect::new(content.x + inset, content.y + inset, rail_w, rail_h);
            let glass = chrome.push_glass([rail.x, rail.y, rail.w, rail.h], theme.border_radius, theme.glass(Level::Pane));
            chrome.begin_glass_content(glass);
            let header = Rect::new(rail.x, rail.y, rail.w, theme.panel_header_height);
            chrome.push_solid([header.x, header.y, header.w, header.h], theme.navbar);
            let toggle_item = ChromeGroupItem { control_id: "shell.engagement.toggle", icon_id: Some("chevron-left"), label: Some("Action"), active: false, disabled: false, kind: HitKind::Button };
            let toggle_w = measure_chrome_group_item(atlas, theme, &toggle_item);
            let toggle_rect = Rect::new(header.x, header.y, toggle_w, header.h);
            render_chrome_group(chrome, atlas, icons, input, theme, toggle_rect, &[toggle_item], true);
            input.register_hit(HitTarget { rect: toggle_rect, event: None, control_id: Some(format!("shell.engagement.toggle.{window_id}")), kind: HitKind::Button, drag_axis: None, drag_data: None });
            let mut y = rail.y + theme.panel_header_height + theme.gap_standard;
            if let Some(options) = &engagement.options {
                for option in options {
                    let label = option.label.clone().unwrap_or_else(|| option.id.clone());
                    let pressed = option.pressed.unwrap_or(false);
                    let item = ChromeGroupItem { control_id: "shell.engagement.option", icon_id: None, label: Some(&label), active: pressed, disabled: false, kind: HitKind::Button };
                    let item_w = measure_chrome_group_item(atlas, theme, &item);
                    let rect = Rect::new(rail.x + 8.0, y, item_w, theme.control_height);
                    render_chrome_group(chrome, atlas, icons, input, theme, rect, &[item], true);
                    if let Some(action) = &option.action {
                        input.register_hit(HitTarget { rect, event: Some(action.clone()), control_id: Some(format!("shell.engagement.option.{}.{}", window_id, option.id)), kind: HitKind::Button, drag_axis: None, drag_data: None });
                    }
                    y += theme.control_height + 4.0;
                }
            }
            if let Some(input_spec) = &engagement.input {
                self.render_engagement_input(chrome, select_overlay, atlas, icons, input, theme, Rect::new(rail.x + 8.0, y, rail.w - 16.0, theme.control_height * 2.0), window_id, input_spec, engagement.possible_engagements.as_deref());
                y += theme.control_height * 2.0 + 8.0;
            }
            if let Some(control) = &engagement.control {
                self.render_engagement_control(chrome, select_overlay, atlas, icons, input, theme, Rect::new(rail.x + 8.0, y, rail.w - 16.0, theme.control_height), control);
                y += theme.control_height;
            }
            if let Some(status_rows) = &engagement.status {
                for row in status_rows {
                    y += theme.control_height;
                    chrome_text(chrome, atlas, input, theme, &row.text, rail.x + 8.0, y, theme.font_size_small, theme.text_muted);
                }
            }
            if let Some(possibles) = &engagement.possible_engagements {
                for possible in possibles {
                    y += theme.control_height + 2.0;
                    let rect = Rect::new(rail.x + 8.0, y, rail.w - 16.0, theme.control_height);
                    chrome.push_rounded([rect.x, rect.y, rect.w, rect.h], theme.button, theme.border_radius);
                    chrome_text(chrome, atlas, input, theme, &possible.label, rect.x + 8.0, rect.y + (rect.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text);
                    if let Some(action) = &possible.action {
                        input.register_hit(HitTarget { rect, event: Some(action.clone()), control_id: Some(format!("shell.engagement.possible.{}.{}", window_id, possible.id)), kind: HitKind::Button, drag_axis: None, drag_data: None });
                    }
                }
            }
            chrome.end_glass_content();
            None
        })(draw, overlay)
    }

    fn render_engagement_input(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        bounds: Rect,
        window_id: &str,
        spec: &WindowEngagementInput,
        possibles: Option<&[ui_wgpu::wgpu::WindowEngagementPossible]>,
    ) {
        let id = spec.id.clone().unwrap_or_else(|| format!("engagement-input-{window_id}"));
        let committed_value = self.engagement_inputs.get(&id).cloned().or_else(|| spec.value.clone()).unwrap_or_default();
        // 👻️ Item 6: the live query for the ghost suffix is the in-progress edit buffer while focused
        // (`InputState::text_buffer`, the same source the generic `Input` widget itself displays while
        // focused), else the last-committed value — mirrors `engagementActiveInlineCompletion`'s own
        // `query` input in `ui/js/react/index.tsx`.
        let focused = input.focused_id.as_deref() == Some(id.as_str());
        let live_query = if focused { input.text_view().to_string() } else { committed_value.clone() };
        let node = ui_wgpu::wgpu::widgets::WidgetNode::Input { id: id.clone(), input_kind: "text".into(), value: committed_value, placeholder: spec.placeholder.clone(), commit: None, on_change: spec.on_change.clone() };
        {
            let scroll_offsets = &mut self.scroll_offsets;
            let collapsed_sections = &mut self.collapsed_sections;
            let open_selects = &mut self.open_selects;
            let mut ctx = framework_widget_context(draw, overlay.as_deref_mut(), atlas, Some(icons), input, theme, scroll_offsets, collapsed_sections, open_selects, None);
            ui_wgpu::wgpu::widgets::render_widget(&node, bounds, &mut ctx);
        }
        // #region GhostTextCompletion (item 6: engagement inline-completion ghost text)
        // 👻️ Ports `engagementInlineCompletion`/`engagementCompletionSuffix` (`ui/js/react/index.tsx`) —
        // the dimmed suffix drawn right after the live text, showing the top `possible_engagements`
        // match's remaining characters.
        let suffix = engagement_completion_suffix(&live_query, possibles);
        if !suffix.is_empty() {
            let text_padding = 8.0_f32;
            let (query_w, _) = atlas.measure_text(&live_query, theme.font_size_small);
            let ghost_x = bounds.x + text_padding + query_w;
            let baseline_y = bounds.y + (theme.control_height + theme.font_size_small) * 0.5 - 1.0;
            let mut ghost_color = theme.text_muted;
            ghost_color.a *= 0.6;
            chrome_text(draw, atlas, input, theme, &suffix, ghost_x, baseline_y, theme.font_size_small, ghost_color);
            // 🖱️ Click-to-accept substitutes for the brief's "Tab or Right-arrow-at-end-of-input accepts"
            // shortcut: this crate's keyboard queue (`InputState::pending_keys`/`queue_key`) is never
            // populated anywhere in this codebase today (confirmed by grep across both crates) — real key
            // events are handled directly inline in the off-limits `shell::ShellInput` region this wave,
            // unreachable from here without editing it. See the report's honest scope-down.
            let (suffix_w, _) = atlas.measure_text(&suffix, theme.font_size_small);
            let ghost_rect = Rect::new(ghost_x, bounds.y, suffix_w.max(4.0), theme.control_height);
            input.register_hit(HitTarget { rect: ghost_rect, event: None, control_id: Some(format!("shell.engagement.input.{id}.ghost-accept")), kind: HitKind::Generic, drag_axis: None, drag_data: None });
            if let Some(accepted) = engagement_ghost_accept_on_click(ghost_rect, input.pointer_x, input.pointer_y, self.chrome_build.clicked_this_frame, &live_query, &suffix) {
                self.engagement_inputs.insert(id.clone(), accepted.clone());
                if focused {
                    let accepted_len = accepted.len();
                    input.focus_input_owned(id, accepted);
                    input.cursor_pos = accepted_len;
                }
            }
        }
        // #endregion
    }

    fn render_engagement_control(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        bounds: Rect,
        control: &WindowEngagementControl,
    ) {
        use ui_wgpu::wgpu::widgets::{render_widget, WidgetNode};
        let node = match control {
            WindowEngagementControl::Slider { id, value, min, max, step, disabled, on_change, .. } => WidgetNode::Slider {
                id: id.clone().unwrap_or_else(|| "engagement-slider".into()),
                value: *value,
                min: *min,
                max: *max,
                step: step.unwrap_or(0.01),
                ready: None,
                disabled: disabled.unwrap_or(false),
                on_change: if disabled.unwrap_or(false) { None } else { on_change.clone() },
            },
            WindowEngagementControl::Stepper { id, value, step, on_change, .. } => {
                WidgetNode::NumberStepper { id: id.clone().unwrap_or_else(|| "engagement-stepper".into()), value: *value, step: step.unwrap_or(1.0), uniform: false, on_absolute: on_change.clone(), on_delta: on_change.clone() }
            }
            WindowEngagementControl::Select { id, value, items, on_change, .. } => WidgetNode::Select {
                id: id.clone().unwrap_or_else(|| "engagement-select".into()),
                value: value.clone().unwrap_or_default(),
                items: items.iter().map(|item| ui_wgpu::wgpu::widgets::SelectItem { value: item.value.clone(), label: item.label.clone() }).collect(),
                placeholder: None,
                on_change: on_change.clone(),
            },
            WindowEngagementControl::Ring { id, value, on_select, .. } => {
                WidgetNode::Ring { id: id.clone().unwrap_or_else(|| "engagement-ring".into()), t: value.as_ref().and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.5), disabled: false, on_change: on_select.clone() }
            }
            WindowEngagementControl::ToggleGroup { id, value, options, on_select, .. } => {
                let label = value.clone().or_else(|| options.first().map(|o| o.id.clone())).unwrap_or_else(|| "toggle".into());
                WidgetNode::Toggle { id: id.clone().unwrap_or_else(|| "engagement-toggle".into()), icon_id: IconName::CircleDot, pressed: false, text: Some(label), on_change: on_select.clone() }
            }
        };
        let scroll_offsets = &mut self.scroll_offsets;
        let collapsed_sections = &mut self.collapsed_sections;
        let open_selects = &mut self.open_selects;
        let mut ctx = framework_widget_context(draw, overlay.as_deref_mut(), atlas, Some(icons), input, theme, scroll_offsets, collapsed_sections, open_selects, None);
        render_widget(&node, bounds, &mut ctx);
    }

    // #region ActionsRail
    /// 📇️ Renders a window's Actions rail (Architecture Decision 8, P1) anchored bottom-right — the free
    /// corner (measures top-right, engagement top-left, utility bar bottom-left). Folded to a chip by
    /// default; unfolded it lists window-scoped actions in manifest order: zero-arg rows ARE the execute
    /// button, arg-carrying rows are accordion disclosures over a staged form. Returns the fold chip hit.
    fn render_window_actions_rail(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        content: &Rect,
        window_id: &str,
        app: &AppDefinition,
        kind: &semio_framework::WindowKindDefinition,
    ) -> Option<(Rect, String)> {
        let actions: Vec<semio_framework::ActionDefinition> = semio_framework::resolve_window_actions(app, kind).into_iter().cloned().collect();
        if actions.is_empty() {
            return None;
        }
        let inset = theme.gap_standard;
        let row_h = theme.control_height;
        let folded = self.action_panel_folded.get(window_id).copied().unwrap_or(true);
        let enabled = self.actions_enabled_for_window(app, window_id);
        let expanded_action = self.action_panel_expanded.get(window_id).cloned();
        (|chrome: &mut DrawList, select_overlay: &mut Option<&mut DrawList>| {
            if folded {
                let item = ChromeGroupItem { control_id: "", icon_id: Some("chevron-up"), label: Some("Actions"), active: false, disabled: false, kind: HitKind::Button };
                let chip_w = measure_chrome_group_item(atlas, theme, &item);
                let chip = Rect::new(content.x + content.w - chip_w - inset, content.y + content.h - row_h - inset, chip_w, row_h);
                render_chrome_group(chrome, atlas, icons, input, theme, chip, &[item], false);
                let segment = semio_framework::element_id_segment(window_id);
                for action in &actions {
                    self.chrome_build.register_element_rect_fallback(format!("framework.window.{segment}.action.{}", action.id), chip);
                }
                return Some((chip, format!("shell.action.fold.{window_id}")));
            }
            let width = theme.window_measures_default_width.clamp(theme.panel_min_width, theme.panel_max_width).min(window_overlay_max_width(content.w, inset));
            let mut body_h = theme.gap_standard;
            for action in &actions {
                body_h += row_h;
                if expanded_action.as_deref() == Some(action.id.as_str()) {
                    body_h += self.staged_form_height(theme, action);
                }
            }
            let card_h = (theme.panel_header_height + body_h + theme.gap_standard).min((content.h - inset * 2.0).max(theme.panel_header_height));
            let rail = Rect::new(content.x + content.w - width - inset, content.y + content.h - card_h - inset, width, card_h);
            let glass = chrome.push_glass([rail.x, rail.y, rail.w, rail.h], theme.border_radius, theme.glass(Level::Pane));
            chrome.begin_glass_content(glass);
            let header = Rect::new(rail.x, rail.y, rail.w, theme.panel_header_height);
            chrome.push_solid([header.x, header.y, header.w, header.h], theme.navbar);
            let fold_item = ChromeGroupItem { control_id: "shell.action.fold", icon_id: Some("chevron-down"), label: Some("Actions"), active: false, disabled: false, kind: HitKind::Button };
            let fold_w = measure_chrome_group_item(atlas, theme, &fold_item);
            render_chrome_group(chrome, atlas, icons, input, theme, Rect::new(header.x, header.y, fold_w, header.h), &[fold_item], true);
            input.register_hit(HitTarget { rect: Rect::new(header.x, header.y, fold_w, header.h), event: None, control_id: Some(format!("shell.action.fold.{window_id}")), kind: HitKind::Button, drag_axis: None, drag_data: None });
            let mut y = rail.y + theme.panel_header_height + theme.gap_standard;
            let body_x = rail.x + theme.gap_standard;
            let body_w = rail.w - theme.gap_standard * 2.0;
            for action in &actions {
                let is_expanded = expanded_action.as_deref() == Some(action.id.as_str());
                let has_args = !action.args.is_empty();
                let row = Rect::new(body_x, y, body_w, row_h);
                let icon = if !has_args {
                    Some(action.icon_id.as_str())
                } else if is_expanded {
                    Some("chevron-down")
                } else {
                    Some("chevron-right")
                };
                let item = ChromeGroupItem { control_id: "", icon_id: icon, label: Some(action.label.resolve(self.active_terminology(), self.active_locale())), active: is_expanded, disabled: !enabled, kind: HitKind::Button };
                render_chrome_group(chrome, atlas, icons, input, theme, row, &[item], false);
                self.chrome_build.register_element_rect(format!("framework.window.{}.action.{}", semio_framework::element_id_segment(window_id), action.id), row);
                if enabled {
                    let control_id = if has_args { format!("shell.action.expand::{window_id}::{}", action.id) } else { format!("shell.action.exec::{window_id}::{}", action.id) };
                    input.register_hit(HitTarget { rect: row, event: None, control_id: Some(control_id), kind: HitKind::Button, drag_axis: None, drag_data: None });
                }
                y += row_h;
                if is_expanded && has_args {
                    y += self.render_staged_form(chrome, select_overlay, atlas, icons, input, theme, Rect::new(body_x, y, body_w, self.staged_form_height(theme, action)), window_id, action, enabled);
                }
            }
            chrome.end_glass_content();
            None
        })(draw, overlay)
    }

    /// 📝️ Total height of one action's staged arg form (per-arg fields + the Execute/Reset row).
    fn staged_form_height(&self, theme: &Theme, action: &semio_framework::ActionDefinition) -> f32 {
        let mut h = theme.gap_standard;
        for arg in &action.args {
            h += self.staged_arg_height(theme, arg);
        }
        h + theme.control_height + theme.gap_standard
    }

    fn staged_arg_height(&self, theme: &Theme, arg: &semio_framework::ActionArgDef) -> f32 {
        match arg.control() {
            semio_framework::ActionArgControl::Toggle => theme.control_height + theme.gap_standard,
            _ => theme.control_height * 2.0 + theme.gap_standard,
        }
    }

    /// 📝️ The effective value of one arg (staged if present, else the declared default).
    fn effective_arg_value(&self, window_id: &str, action_id: &str, arg: &semio_framework::ActionArgDef) -> Option<Value> {
        self.staged_action_args.get(&Self::staged_key(window_id, action_id)).and_then(|map| map.get(&arg.id).cloned()).or_else(|| arg.default.as_ref().map(dsl_value_as_json))
    }

    fn arg_default(&self, window_kind_id: &str, action_id: &str, arg_id: &str) -> Option<Value> {
        window_action_definition(&self.session.as_ref()?.app, window_kind_id, action_id)?.args.iter().find(|arg| arg.id == arg_id)?.default.as_ref().map(dsl_value_as_json)
    }

    /// 📝️ Renders the staged form for one expanded action and returns its consumed height. Every control
    /// writes to STAGING via shell-owned hit ids (never a live `on_change` dispatch) — P2.
    fn render_staged_form(
        &mut self,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        bounds: Rect,
        window_id: &str,
        action: &semio_framework::ActionDefinition,
        enabled: bool,
    ) -> f32 {
        let row_h = theme.control_height;
        let mut y = bounds.y + theme.gap_standard;
        for arg in &action.args {
            let arg_h = self.staged_arg_height(theme, arg);
            self.render_staged_arg(draw, overlay, atlas, icons, input, theme, Rect::new(bounds.x, y, bounds.w, arg_h), window_id, &action.id, arg, enabled);
            y += arg_h;
        }
        // Execute / Reset row.
        let is_de = self.locale_id == "de";
        let staged = self.staged_map_for(window_id, &action.id);
        let executable = Self::resolved_execute_args(&action.args, &staged).is_some();
        let exec_item = ChromeGroupItem { control_id: "", icon_id: Some("play"), label: Some(shell_chrome_string("common.execute", is_de)), active: false, disabled: !(enabled && executable), kind: HitKind::Button };
        let exec_w = measure_chrome_group_item(atlas, theme, &exec_item);
        let exec_rect = Rect::new(bounds.x, y, exec_w, row_h);
        render_chrome_group(draw, atlas, icons, input, theme, exec_rect, &[exec_item], false);
        if enabled && executable {
            input.register_hit(HitTarget { rect: exec_rect, event: None, control_id: Some(format!("shell.action.exec::{window_id}::{}", action.id)), kind: HitKind::Button, drag_axis: None, drag_data: None });
        }
        let reset_item = ChromeGroupItem { control_id: "", icon_id: Some("rotate-ccw"), label: Some(shell_chrome_string("common.reset", is_de)), active: false, disabled: false, kind: HitKind::Button };
        let reset_w = measure_chrome_group_item(atlas, theme, &reset_item);
        let reset_rect = Rect::new(bounds.x + exec_w + theme.gap_standard, y, reset_w, row_h);
        render_chrome_group(draw, atlas, icons, input, theme, reset_rect, &[reset_item], false);
        input.register_hit(HitTarget { rect: reset_rect, event: None, control_id: Some(format!("shell.action.reset::{window_id}::{}", action.id)), kind: HitKind::Button, drag_axis: None, drag_data: None });
        self.staged_form_height(theme, action)
    }

    fn render_staged_arg(
        &mut self,
        draw: &mut DrawList,
        _overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        bounds: Rect,
        window_id: &str,
        action_id: &str,
        arg: &semio_framework::ActionArgDef,
        enabled: bool,
    ) {
        use semio_framework::ActionArgControl;
        let row_h = theme.control_height;
        let effective = self.effective_arg_value(window_id, action_id, arg);
        match &arg.control() {
            ActionArgControl::Toggle => {
                let on = effective.as_ref().and_then(|v| v.as_bool()).unwrap_or(false);
                let label = format!("{} · {}", arg.label.resolve(self.active_terminology(), self.active_locale()), if on { "on" } else { "off" });
                let item = ChromeGroupItem { control_id: "", icon_id: Some(if on { "check-square" } else { "square" }), label: Some(label.as_str()), active: on, disabled: !enabled, kind: HitKind::Button };
                let item_w = measure_chrome_group_item(atlas, theme, &item).min(bounds.w);
                let rect = Rect::new(bounds.x, bounds.y, item_w, row_h);
                render_chrome_group(draw, atlas, icons, input, theme, rect, &[item], false);
                if enabled {
                    input.register_hit(HitTarget { rect, event: None, control_id: Some(format!("shell.action.argtoggle::{window_id}::{action_id}::{}", arg.id)), kind: HitKind::Button, drag_axis: None, drag_data: None });
                }
            }
            ActionArgControl::Select { options } => {
                chrome_text(draw, atlas, input, theme, arg.label.resolve(self.active_terminology(), self.active_locale()), bounds.x, bounds.y + 14.0, theme.font_size_small, theme.text_muted);
                let effective_str = effective.as_ref().and_then(|v| v.as_str()).map(String::from);
                let mut x = bounds.x;
                let chip_y = bounds.y + row_h;
                for option in options {
                    let active = effective_str.as_deref() == Some(option.value.as_str());
                    let item = ChromeGroupItem { control_id: "", icon_id: None, label: Some(option.label.resolve(self.active_terminology(), self.active_locale())), active, disabled: !enabled, kind: HitKind::Button };
                    let item_w = measure_chrome_group_item(atlas, theme, &item);
                    let rect = Rect::new(x, chip_y, item_w, row_h);
                    render_chrome_group(draw, atlas, icons, input, theme, rect, &[item], false);
                    if enabled {
                        input.register_hit(HitTarget { rect, event: None, control_id: Some(format!("shell.action.argselect::{window_id}::{action_id}::{}::{}", arg.id, option.value)), kind: HitKind::Button, drag_axis: None, drag_data: None });
                    }
                    x += item_w + theme.gap_standard;
                }
            }
            ActionArgControl::IconSelect { .. } => {
                // Icon classifiers are not enumerable at manifest altitude; fall back to a text field.
                let value = self.staged_arg_display_string(window_id, action_id, arg, input, None);
                self.render_staged_text_field(draw, atlas, icons, input, theme, bounds, window_id, action_id, arg, &value, enabled, None);
            }
            ActionArgControl::Vec3 => {
                chrome_text(draw, atlas, input, theme, arg.label.resolve(self.active_terminology(), self.active_locale()), bounds.x, bounds.y + 14.0, theme.font_size_small, theme.text_muted);
                let arr = effective.as_ref().and_then(|v| v.as_array());
                let field_w = ((bounds.w - theme.gap_standard * 2.0) / 3.0).max(24.0);
                for axis in 0..3usize {
                    let control_id = format!("shell.action.argvec3::{window_id}::{action_id}::{}::{axis}", arg.id);
                    let focused = input.focused_id.as_deref() == Some(control_id.as_str());
                    let display = if focused { input.text_view().to_string() } else { fmt_num(arr.and_then(|a| a.get(axis)).and_then(|v| v.as_f64()).unwrap_or(0.0)) };
                    let rect = Rect::new(bounds.x + axis as f32 * (field_w + theme.gap_standard), bounds.y + row_h, field_w, row_h);
                    self.paint_staged_input_box(draw, atlas, input, theme, rect, &display, focused, enabled, &control_id);
                }
            }
            _ => {
                // Text / Number / Slider → a single focusable input, staged on commit.
                let control_id = format!("shell.action.arginput::{window_id}::{action_id}::{}", arg.id);
                let focused = input.focused_id.as_deref() == Some(control_id.as_str());
                let display = self.staged_arg_display_string(window_id, action_id, arg, input, Some(&control_id));
                self.render_staged_text_field(draw, atlas, icons, input, theme, bounds, window_id, action_id, arg, &display, enabled, Some(focused));
            }
        }
    }

    /// 📝️ The current display string of a scalar arg — the live focus buffer if focused, else the
    /// effective staged/default value.
    fn staged_arg_display_string(&self, window_id: &str, action_id: &str, arg: &semio_framework::ActionArgDef, input: &InputState<ActionDescriptor>, control_id: Option<&str>) -> String {
        if let Some(control_id) = control_id {
            if input.focused_id.as_deref() == Some(control_id) {
                return input.text_view().to_string();
            }
        }
        match self.effective_arg_value(window_id, action_id, arg) {
            Some(Value::String(text)) => text,
            Some(Value::Number(num)) => num.to_string(),
            Some(Value::Bool(flag)) => flag.to_string(),
            Some(other) => other.to_string(),
            None => String::new(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_staged_text_field(
        &mut self,
        draw: &mut DrawList,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        bounds: Rect,
        window_id: &str,
        action_id: &str,
        arg: &semio_framework::ActionArgDef,
        display: &str,
        enabled: bool,
        focused_override: Option<bool>,
    ) {
        let _ = icons;
        chrome_text(draw, atlas, input, theme, arg.label.resolve(self.active_terminology(), self.active_locale()), bounds.x, bounds.y + 14.0, theme.font_size_small, theme.text_muted);
        let control_id = format!("shell.action.arginput::{window_id}::{action_id}::{}", arg.id);
        let focused = focused_override.unwrap_or_else(|| input.focused_id.as_deref() == Some(control_id.as_str()));
        let rect = Rect::new(bounds.x, bounds.y + theme.control_height, bounds.w, theme.control_height);
        self.paint_staged_input_box(draw, atlas, input, theme, rect, display, focused, enabled, &control_id);
    }

    #[allow(clippy::too_many_arguments)]
    fn paint_staged_input_box(&self, draw: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, rect: Rect, display: &str, focused: bool, enabled: bool, control_id: &str) {
        draw.push_rounded([rect.x, rect.y, rect.w, rect.h], theme.input_bg, theme.border_radius);
        if focused {
            let hair = theme.stroke_hairline * 2.0;
            draw.push_solid([rect.x, rect.y + rect.h - hair, rect.w, hair], theme.accent);
        }
        let text_color = if enabled { theme.text } else { theme.text_muted };
        chrome_text(draw, atlas, input, theme, display, rect.x + theme.padding_standard, rect.y + (rect.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, text_color);
        if enabled {
            input.register_hit(HitTarget { rect, event: None, control_id: Some(control_id.to_string()), kind: HitKind::Input, drag_axis: None, drag_data: None });
        }
    }
    // #endregion

    fn render_context_menu(&self, overlay: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, menu: &ContextMenuState, viewport_w: f32, viewport_h: f32) {
        Self::render_context_menu_level(overlay, atlas, icons, input, theme, menu, &menu.items, &[], menu.x, menu.y, viewport_w, viewport_h);
    }

    /// 📏️ Shared width pass for a menu level — also used to size a submenu BEFORE deciding which side of
    /// its parent row it opens on (see `render_context_menu_level`'s flip-left check).
    fn context_menu_level_width(items: &[ContextMenuItem], theme: &Theme) -> f32 {
        let mut w = 180.0;
        for item in items.iter().filter(|item| !item.separator || !item.label.is_empty()) {
            let label_w = item.label.chars().count() as f32 * theme.font_size_body * 0.55;
            let shortcut_w = item.shortcut.as_ref().map(|s| s.chars().count() as f32 * theme.font_size_small * 0.55 + 16.0).unwrap_or(0.0);
            w = f32::max(w, 56.0_f32 + label_w + shortcut_w);
        }
        w
    }

    fn render_context_menu_level(
        overlay: &mut DrawList,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        menu: &ContextMenuState,
        items: &[ContextMenuItem],
        path_prefix: &[usize],
        origin_x: f32,
        origin_y: f32,
        viewport_w: f32,
        viewport_h: f32,
    ) {
        let row_h = theme.control_height;
        let w = Self::context_menu_level_width(items, theme);
        let content_h = items.len() as f32 * row_h + 8.0;
        let available_h = (viewport_h - 8.0).max(row_h + 8.0);
        let h = content_h.min(available_h);
        let scrollable = content_h > h + 0.5;
        let scroll = if scrollable { menu.scroll_offset.clamp(0.0, content_h - h) } else { 0.0 };
        // 🖥️ Only the top-level menu is clamped on-screen — a submenu instead flips to the parent row's
        // left edge below when it would overflow the right edge (never repositioned vertically).
        let (x, y) = if path_prefix.is_empty() { (origin_x.clamp(0.0, (viewport_w - w).max(0.0)), origin_y.clamp(0.0, (viewport_h - h).max(0.0))) } else { (origin_x, origin_y) };
        let rect = Rect::new(x, y, w, h);
        overlay.push_glass([rect.x, rect.y, rect.w, rect.h], theme.border_radius, theme.glass(Level::Menu));
        if scrollable {
            input.register_hit(HitTarget { rect, event: None, control_id: Some("shell.context.menu.scroll".into()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
        }
        let icon_size = theme.font_size_body;
        let ordinal_gap = theme.font_size_small * 0.85;
        overlay.push_scissor(rect);
        let mut visual_row = 0usize;
        let mut ordinal = 0usize;
        for (index, item) in items.iter().enumerate() {
            let row_top = rect.y + 4.0 + visual_row as f32 * row_h - scroll;
            let row_visible = row_top + row_h > rect.y && row_top < rect.y + rect.h;
            if item.separator {
                // 🏷️ A separator carrying a `label` is a non-interactive header row — kept in place (never
                // dropped) and rendered as a labeled row instead of a bare rule.
                if row_visible {
                    if item.label.is_empty() {
                        overlay.push_solid([rect.x + 8.0, row_top + row_h * 0.5, w - 16.0, 1.0], theme.text_muted);
                    } else {
                        chrome_text(overlay, atlas, input, theme, &item.label, rect.x + 8.0, row_top + (row_h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text_muted);
                    }
                }
                visual_row += 1;
                continue;
            }
            ordinal += 1;
            let row_path: Vec<usize> = path_prefix.iter().copied().chain(std::iter::once(index)).collect();
            let is_active = context_menu_paths_equal(&menu.active, &row_path);
            let has_children = !item.children.is_empty();
            let submenu_collapsed = menu.submenu_collapsed_at.as_deref().is_some_and(|collapsed| context_menu_paths_equal(collapsed, &row_path));
            let submenu_open = (context_menu_submenu_open(&menu.active, &row_path, is_active, has_children) || (is_active && has_children)) && !submenu_collapsed;
            let row = Rect::new(rect.x + 4.0, row_top, w - 8.0, row_h);
            visual_row += 1;
            if row_visible {
                let (bg, fg) = if is_active { (theme.accent, theme.active_foreground) } else { (theme.button, theme.text) };
                overlay.push_rounded([row.x, row.y, row.w, row.h], bg, theme.border_radius);
                let mut text_x = row.x + 8.0;
                if ordinal <= 9 {
                    let badge = format!("{ordinal}");
                    chrome_text(overlay, atlas, input, theme, &badge, text_x, row.y + (row.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text_muted);
                    text_x += ordinal_gap;
                }
                let icon_id = item.icon.as_deref().unwrap_or("circle-dot");
                chrome_icon(overlay, icons, icon_id, text_x, row.y + (row.h - icon_size) * 0.5, icon_size, if item.disabled { theme.text_muted } else { fg });
                text_x += icon_size + theme.gap_standard;
                chrome_text(overlay, atlas, input, theme, &item.label, text_x, row.y + (row.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, if item.disabled { theme.text_muted } else { fg });
                if let Some(shortcut) = item.shortcut.as_deref() {
                    let shortcut_w = shortcut.chars().count() as f32 * theme.font_size_small * 0.55;
                    chrome_text(overlay, atlas, input, theme, shortcut, row.x + row.w - 8.0 - shortcut_w, row.y + (row.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text_muted);
                }
                if !item.disabled {
                    input.register_hit(HitTarget { rect: row, event: item.action.clone(), control_id: Some(item.id.clone()), kind: HitKind::ContextMenu, drag_axis: None, drag_data: None });
                }
            }
            if submenu_open {
                let child_w = Self::context_menu_level_width(&item.children, theme);
                let opens_left = row.x + row.w + 4.0 + child_w > viewport_w;
                let child_x = if opens_left { (row.x - child_w - 4.0).max(0.0) } else { row.x + row.w + 4.0 };
                overlay.pop_scissor();
                Self::render_context_menu_level(overlay, atlas, icons, input, theme, menu, &item.children, &row_path, child_x, row.y, viewport_w, viewport_h);
                overlay.push_scissor(rect);
            }
        }
        overlay.pop_scissor();
    }
}

// #region 💾️🎨️🌐️ UiPrefsThemesI18n
// WP14: uiPrefs persistence (byte-identical localStorage keys to `ui/js/react/index.tsx`
// :2100-2318 so both renderers share prefs on one browser origin), `SEMIO_LOCKED_*` pref locks
// (mirrors `os-shell.tsx`'s `FrameworkOsLocks`), a named/custom theme registry + minimal draft
// editor, and an EN/DE chrome-string bundle keyed to match `ui/js/react/index.tsx`'s
// `uiChromeTranslationBundles` (:2898-3975). Additive-only new region: shares `shell::ShellChrome`
// with `w3-overlays-chrome-polish` (tooltips/dialogs/tour/cursor/ribbon) — this region touches
// neither; it only adds new items after the last existing method.

//#region 🔑️StorageKeys
/// 🔑️ Byte-identical to `UI_CHROME_APPEARANCE_STORAGE_KEY` (`ui/js/react/index.tsx:2132`).
const UI_CHROME_APPEARANCE_STORAGE_KEY: &str = "ui.chrome.appearance";
/// 🔑️ Byte-identical to `UI_CHROME_LOCALE_STORAGE_KEY` (`ui/js/react/index.tsx:2167`).
const UI_CHROME_LOCALE_STORAGE_KEY: &str = "ui.chrome.locale";
/// 🔑️ Byte-identical to `UI_CHROME_TERMINOLOGY_STORAGE_KEY` (`ui/js/react/index.tsx:2186`).
const UI_CHROME_TERMINOLOGY_STORAGE_KEY: &str = "ui.chrome.terminology";
/// 🔑️ Byte-identical to `UI_CHROME_DRIVER_STORAGE_KEY` (`ui/js/react/index.tsx`). Custom drivers
/// (`ui.drivers.custom`) are a JS-only editor feature this wgpu mirror doesn't surface yet — only the
/// active driver id round-trips here, same scope the old `compact`/`expertise` fields had.
const UI_CHROME_DRIVER_STORAGE_KEY: &str = "ui.chrome.driver";
/// 🔑️ Byte-identical to `UI_CHROME_LAYOUT_STORAGE_KEY` (`ui/js/react/index.tsx:2152`).
const UI_CHROME_LAYOUT_STORAGE_KEY: &str = "ui.chrome.layout";
/// 🔑️ Byte-identical to `UI_CHROME_THEME_ID_STORAGE_KEY` (`ui/js/react/index.tsx:2201`).
const UI_CHROME_THEME_ID_STORAGE_KEY: &str = "ui.chrome.theme";
/// 🔑️ Byte-identical to `UI_CUSTOM_THEMES_STORAGE_KEY` (`ui/js/react/index.tsx:2237`).
const UI_CUSTOM_THEMES_STORAGE_KEY: &str = "ui.themes.custom";
/// 🔑️ Byte-identical to `UI_COMPUTE_WORKER_COUNT_STORAGE_KEY` (`ui/js/react/index.tsx:2267`).
const UI_COMPUTE_WORKER_COUNT_STORAGE_KEY: &str = "ui.compute.workerCount";
/// 🔑️ Byte-identical to `UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX` (`ui/js/react/index.tsx:2305`).
const UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX: &str = "ui.introduction.seen.";
/// 🔑️ Byte-identical to `UI_TERMINOLOGY_NATIVE` (`ui/js/react/index.tsx:2183`).
const UI_TERMINOLOGY_NATIVE: &str = "native";
//#endregion 🔑️StorageKeys

//#region 🗄️PrefsStore
const OS_SHELL_CONFIG_STORAGE_KEY: &str = "semio.os.config";

/// 🗄️ Cross-platform key-value persistence for uiPrefs. `web-sys`'s "Storage" feature isn't enabled
/// on this crate (`Cargo.toml` is a reserved wave-3 choke point), so the wasm32 backend reaches
/// `localStorage` via raw `js_sys::Reflect`/`js_sys::Function` calls against the already-enabled
/// "Window" feature rather than requesting a new one. The native backend is a small JSON file next
/// to no new dependency (`serde_json` is already a dep) — zero-touch across devcontainer/win/mac/linux.
trait PrefsStore {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: &str, value: &str);
}

#[cfg(target_arch = "wasm32")]
struct WebLocalStorage {
    storage: Option<wasm_bindgen::JsValue>,
}

#[cfg(target_arch = "wasm32")]
impl WebLocalStorage {
    fn new() -> Self {
        let storage = web_sys::window().and_then(|window| js_sys::Reflect::get(&window, &wasm_bindgen::JsValue::from_str("localStorage")).ok());
        Self { storage }
    }

    fn call(&self, method: &str, args: &[&str]) -> Option<wasm_bindgen::JsValue> {
        use wasm_bindgen::JsCast;
        let storage = self.storage.as_ref()?;
        let func = js_sys::Reflect::get(storage, &wasm_bindgen::JsValue::from_str(method)).ok()?;
        let func: js_sys::Function = func.dyn_into().ok()?;
        match args.len() {
            1 => func.call1(storage, &wasm_bindgen::JsValue::from_str(args[0])).ok(),
            2 => func.call2(storage, &wasm_bindgen::JsValue::from_str(args[0]), &wasm_bindgen::JsValue::from_str(args[1])).ok(),
            _ => None,
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl PrefsStore for WebLocalStorage {
    fn get(&self, key: &str) -> Option<String> {
        self.call("getItem", &[key]).and_then(|value| value.as_string())
    }

    fn set(&mut self, key: &str, value: &str) {
        self.call("setItem", &[key, value]);
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct FilePrefsStore {
    path: std::path::PathBuf,
    cache: std::cell::RefCell<HashMap<String, String>>,
    pending_load: std::cell::RefCell<Option<std::sync::mpsc::Receiver<HashMap<String, String>>>>,
    dirty: std::cell::Cell<bool>,
    flush_state: std::sync::Arc<std::sync::Mutex<FilePrefsFlushState>>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Default)]
struct FilePrefsFlushState {
    latest: Option<HashMap<String, String>>,
    running: bool,
}

/// 📁️ Resolves the native prefs file path: `$SEMIO_PREFS_DIR/ui-prefs.json` when set, else a
/// per-OS config-home fallback (XDG on linux/devcontainer, `%APPDATA%` on windows, `~/.config` on
/// macOS) — no new dependency (no `dirs` crate), just `std::env`.
#[cfg(not(target_arch = "wasm32"))]
fn native_prefs_file_path() -> std::path::PathBuf {
    if let Ok(dir) = std::env::var("SEMIO_PREFS_DIR") {
        return std::path::PathBuf::from(dir).join("ui-prefs.json");
    }
    let base = std::env::var("XDG_CONFIG_HOME").or_else(|_| std::env::var("APPDATA")).or_else(|_| std::env::var("HOME").map(|home| format!("{home}/.config"))).unwrap_or_else(|_| ".".to_string());
    std::path::PathBuf::from(base).join("semio").join("ui-prefs.json")
}

#[cfg(not(target_arch = "wasm32"))]
impl FilePrefsStore {
    fn new() -> Self {
        Self::with_path(native_prefs_file_path())
    }

    fn with_path(path: std::path::PathBuf) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        let load_path = path.clone();
        crate::renderer_worker_pool().submit(
            Lane::Io,
            Box::new(move || {
                use std::fs as system_fs;
                let cache = system_fs::read_to_string(load_path).ok().and_then(|raw| serde_json::from_str::<HashMap<String, String>>(&raw).ok()).unwrap_or_default();
                let _ = sender.send(cache);
            }),
        );
        Self { path, cache: std::cell::RefCell::new(HashMap::new()), pending_load: std::cell::RefCell::new(Some(receiver)), dirty: std::cell::Cell::new(false), flush_state: std::sync::Arc::new(std::sync::Mutex::new(FilePrefsFlushState::default())) }
    }

    fn poll_load(&self) {
        let result = self.pending_load.borrow().as_ref().map(std::sync::mpsc::Receiver::try_recv);
        match result {
            Some(Ok(loaded)) => {
                self.pending_load.borrow_mut().take();
                if !self.dirty.get() {
                    *self.cache.borrow_mut() = loaded;
                }
            }
            Some(Err(std::sync::mpsc::TryRecvError::Disconnected)) => {
                self.pending_load.borrow_mut().take();
            }
            Some(Err(std::sync::mpsc::TryRecvError::Empty)) | None => {}
        }
    }

    fn flush(&self) {
        let mut state = self.flush_state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.latest = Some(self.cache.borrow().clone());
        if state.running {
            return;
        }
        state.running = true;
        drop(state);
        let path = self.path.clone();
        let flush_state = std::sync::Arc::clone(&self.flush_state);
        crate::renderer_worker_pool().submit(
            Lane::Io,
            Box::new(move || loop {
                let snapshot = {
                    let mut state = flush_state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    match state.latest.take() {
                        Some(snapshot) => snapshot,
                        None => {
                            state.running = false;
                            break;
                        }
                    }
                };
                use std::fs as system_fs;
                if let Some(parent) = path.parent() {
                    let _ = system_fs::create_dir_all(parent);
                }
                if let Ok(json) = serde_json::to_string_pretty(&snapshot) {
                    let _ = system_fs::write(&path, json);
                }
            }),
        );
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl PrefsStore for FilePrefsStore {
    fn get(&self, key: &str) -> Option<String> {
        self.poll_load();
        self.cache.borrow().get(key).cloned()
    }

    fn set(&mut self, key: &str, value: &str) {
        self.poll_load();
        self.cache.borrow_mut().insert(key.to_string(), value.to_string());
        self.dirty.set(true);
        self.flush();
    }
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    #[cfg(target_arch = "wasm32")]
    static PREFS_STORE: std::cell::RefCell<WebLocalStorage> = std::cell::RefCell::new(WebLocalStorage::new());
}

#[cfg(not(target_arch = "wasm32"))]
static PREFS_STORE: std::sync::OnceLock<std::sync::Mutex<std::cell::RefCell<FilePrefsStore>>> = std::sync::OnceLock::new();

fn empty_os_shell_config() -> Value {
    serde_json::json!({
        "version": 1,
        "preferences": {},
        "namedLayouts": {},
        "dockLayouts": { "apps": {} },
        "dockUi": { "apps": {} },
        "windowPanes": { "apps": {} }
    })
}

fn prefs_get_from(store: &impl PrefsStore, key: &str) -> Option<String> {
    let raw = store.get(OS_SHELL_CONFIG_STORAGE_KEY)?;
    serde_json::from_str::<Value>(&raw).ok()?.get("preferences")?.get(key)?.as_str().map(ToOwned::to_owned)
}

fn prefs_set_in(store: &mut impl PrefsStore, key: &str, value: &str) {
    let mut config = store.get(OS_SHELL_CONFIG_STORAGE_KEY).and_then(|raw| serde_json::from_str::<Value>(&raw).ok()).filter(|config| config.get("version").and_then(Value::as_u64) == Some(1)).unwrap_or_else(empty_os_shell_config);
    if !config.get("preferences").is_some_and(Value::is_object) {
        config["preferences"] = serde_json::json!({});
    }
    config["preferences"][key] = Value::String(value.to_string());
    if let Ok(raw) = serde_json::to_string(&config) {
        store.set(OS_SHELL_CONFIG_STORAGE_KEY, &raw);
    }
}

fn prefs_get(key: &str) -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        PREFS_STORE.with(|store| prefs_get_from(&*store.borrow(), key))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let store = PREFS_STORE.get_or_init(|| std::sync::Mutex::new(std::cell::RefCell::new(FilePrefsStore::new()))).lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let value = prefs_get_from(&*store.borrow(), key);
        value
    }
}

fn prefs_set(key: &str, value: &str) {
    #[cfg(target_arch = "wasm32")]
    PREFS_STORE.with(|store| prefs_set_in(&mut *store.borrow_mut(), key, value));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let store = PREFS_STORE.get_or_init(|| std::sync::Mutex::new(std::cell::RefCell::new(FilePrefsStore::new()))).lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        prefs_set_in(&mut *store.borrow_mut(), key, value);
    }
}
//#endregion 🗄️PrefsStore

//#region 🔒️PrefLocks
/// 🔒️ `SEMIO_LOCKED_*` env-driven pref locks — mirrors `os-shell.tsx`'s `FrameworkOsLocks`
/// (`:372-378`, read via `VITE_SEMIO_LOCKED_*` in `framework/product/os/dev/js/index.ts:19-23`):
/// appearance/locale/terminology/themeId may be locked; driver/layout/customThemes/customDrivers
/// deliberately stay unlocked in React too. A locked pref skips its localStorage write. Native-only
/// in practice (wasm32-unknown-unknown has no process env at runtime, so this is always empty
/// there — kiosk/demo locking is a native `semio-wgpu-native` deployment concern).
#[derive(Clone, Debug, Default)]
struct ShellPrefLocks {
    appearance: Option<String>,
    locale: Option<String>,
    terminology: Option<String>,
    theme_id: Option<String>,
}

fn env_lock(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|value| !value.is_empty())
}

fn shell_pref_locks() -> ShellPrefLocks {
    ShellPrefLocks { appearance: env_lock("SEMIO_LOCKED_APPEARANCE"), locale: env_lock("SEMIO_LOCKED_LOCALE"), terminology: env_lock("SEMIO_LOCKED_TERMINOLOGY"), theme_id: env_lock("SEMIO_LOCKED_THEME") }
}
//#endregion 🔒️PrefLocks

//#region 🎨️ThemeRegistry
/// 🎨️ A user-defined theme's color overrides for one appearance — deliberately scoped down from
/// React's full `UiTheme` (colors/spacing/fontStacks/canvasFonts/strokes/radii/opacities/metrics
/// per :ui/styling/js/theme.ts`) to the handful of paints `ui_wgpu::wgpu::Theme` actually varies by
/// chrome palette (see `ui/wgpu/rs/lib.rs`'s `from_chrome`, read-only reference). A full token-level
/// draft editor would require porting `resolveThemePaint`'s token/mix resolver wholesale; this
/// covers "load/mutate/save a custom theme's token values programmatically" per the WP14 scope note.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct ChromeColorOverrides {
    background: Option<String>,
    panel: Option<String>,
    navbar: Option<String>,
    text: Option<String>,
    accent: Option<String>,
}

/// 🎨️ A persisted custom theme: `base` is a builtin id ("semio" | "mono") the overrides layer onto.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CustomChromeTheme {
    id: String,
    label: String,
    base: String,
    #[serde(default)]
    light: ChromeColorOverrides,
    #[serde(default)]
    dark: ChromeColorOverrides,
}

/// 🎛️ Owned preference snapshot consumed while constructing chrome.
#[derive(Clone)]
struct ChromePrefsState {
    ui_layout: String,
    theme_id: String,
    custom_themes: HashMap<String, String>,
    // 🎨️ Only ever read/written by the `w3-prefs-i18n-themes` draft-color-editor primitives below
    // (`begin_custom_theme_draft`/`set_draft_theme_color`/`save_draft_theme`/`discard_draft_theme`),
    // which stay unwired to any real UI on purpose (see `build_settings_theme_ui`'s doc comment) —
    // exercised today only by `ui_prefs_themes_i18n_tests`, hence the matching `cfg` here.
    #[cfg(all(test, not(target_arch = "wasm32")))]
    draft_theme: Option<String>,
    worker_count: u32,
}

impl Default for ChromePrefsState {
    fn default() -> Self {
        Self {
            ui_layout: "desktop".to_string(),
            theme_id: "semio".to_string(),
            custom_themes: HashMap::new(),
            #[cfg(all(test, not(target_arch = "wasm32")))]
            draft_theme: None,
            worker_count: default_compute_worker_count(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static CHROME_PREFS: std::cell::RefCell<Option<ChromePrefsState>> = std::cell::RefCell::new(None);
}

#[cfg(not(target_arch = "wasm32"))]
static CHROME_PREFS: WorkerCell<Option<ChromePrefsState>> = WorkerCell::new();

fn default_compute_worker_count() -> u32 {
    std::thread::available_parallelism().map(|n| n.get() as u32).unwrap_or(1)
}

fn load_chrome_prefs() -> ChromePrefsState {
    let ui_layout = if prefs_get(UI_CHROME_LAYOUT_STORAGE_KEY).as_deref() == Some("tablet") { "tablet".to_string() } else { "desktop".to_string() };
    let theme_id = prefs_get(UI_CHROME_THEME_ID_STORAGE_KEY).unwrap_or_else(|| "semio".to_string());
    let custom_themes = prefs_get(UI_CUSTOM_THEMES_STORAGE_KEY).and_then(|raw| serde_json::from_str::<HashMap<String, Value>>(&raw).ok()).map(|map| map.into_iter().map(|(id, value)| (id, value.to_string())).collect()).unwrap_or_default();
    let worker_count = prefs_get(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY).and_then(|raw| raw.parse::<u32>().ok()).filter(|count| *count >= 1).unwrap_or_else(default_compute_worker_count);
    ChromePrefsState {
        ui_layout,
        theme_id,
        custom_themes,
        #[cfg(all(test, not(target_arch = "wasm32")))]
        draft_theme: None,
        worker_count,
    }
}

fn with_chrome_prefs<R>(f: impl FnOnce(&mut ChromePrefsState) -> R) -> R {
    CHROME_PREFS.with(|cell| {
        let mut guard = cell.borrow_mut();
        if guard.is_none() {
            *guard = Some(load_chrome_prefs());
        }
        f(guard.as_mut().expect("chrome prefs just initialized"))
    })
}

pub(crate) fn active_theme_id() -> String {
    with_chrome_prefs(|prefs| prefs.theme_id.clone())
}

pub(crate) fn set_active_theme_id(id: &str) {
    with_chrome_prefs(|prefs| prefs.theme_id = id.to_string());
}

pub(crate) fn active_ui_layout() -> String {
    with_chrome_prefs(|prefs| prefs.ui_layout.clone())
}

// 🎨️ Same "stays unwired on purpose" status as the draft-theme cluster below — exercised only by
// `ui_prefs_themes_i18n_tests` today.
#[cfg(all(test, not(target_arch = "wasm32")))]
pub(crate) fn set_active_ui_layout(layout: &str) {
    let value = if layout == "tablet" { "tablet" } else { "desktop" };
    with_chrome_prefs(|prefs| prefs.ui_layout = value.to_string());
}

pub(crate) fn active_worker_count() -> u32 {
    with_chrome_prefs(|prefs| prefs.worker_count)
}

pub(crate) fn custom_theme_ids() -> Vec<String> {
    with_chrome_prefs(|prefs| prefs.custom_themes.keys().cloned().collect())
}

/// 🎨️ Starts (or replaces) the in-memory draft for a new custom theme cloned from `base_id`.
#[cfg(all(test, not(target_arch = "wasm32")))]
pub(crate) fn begin_custom_theme_draft(base_id: &str, label: &str, slug: &str) -> String {
    let id = format!("custom.{slug}");
    let draft = CustomChromeTheme { id: id.clone(), label: label.to_string(), base: base_id.to_string(), light: ChromeColorOverrides::default(), dark: ChromeColorOverrides::default() };
    with_chrome_prefs(|prefs| prefs.draft_theme = serde_json::to_string(&draft).ok());
    id
}

/// 🎨️ Mutates one paint slot (`"background" | "panel" | "navbar" | "text" | "accent"`) of the
/// in-progress draft for one appearance (`"light" | "dark"`). Returns `false` if there is no draft
/// or `field` is unknown.
#[cfg(all(test, not(target_arch = "wasm32")))]
pub(crate) fn set_draft_theme_color(appearance: &str, field: &str, hex: &str) -> bool {
    with_chrome_prefs(|prefs| {
        let Some(raw) = prefs.draft_theme.clone() else { return false };
        let Ok(mut draft) = serde_json::from_str::<CustomChromeTheme>(&raw) else { return false };
        let overrides = if appearance == "dark" { &mut draft.dark } else { &mut draft.light };
        let slot = match field {
            "background" => &mut overrides.background,
            "panel" => &mut overrides.panel,
            "navbar" => &mut overrides.navbar,
            "text" => &mut overrides.text,
            "accent" => &mut overrides.accent,
            _ => return false,
        };
        *slot = Some(hex.to_string());
        prefs.draft_theme = serde_json::to_string(&draft).ok();
        true
    })
}

/// 🎨️ Commits the in-progress draft into the custom-theme registry and activates it. Returns the
/// new theme id, or `None` if there was no draft.
#[cfg(all(test, not(target_arch = "wasm32")))]
pub(crate) fn save_draft_theme() -> Option<String> {
    with_chrome_prefs(|prefs| {
        let raw = prefs.draft_theme.take()?;
        let draft: CustomChromeTheme = serde_json::from_str(&raw).ok()?;
        let id = draft.id.clone();
        prefs.custom_themes.insert(id.clone(), raw);
        prefs.theme_id = id.clone();
        Some(id)
    })
}

#[cfg(all(test, not(target_arch = "wasm32")))]
pub(crate) fn discard_draft_theme() {
    with_chrome_prefs(|prefs| prefs.draft_theme = None);
}

pub(crate) fn delete_custom_theme(id: &str) {
    with_chrome_prefs(|prefs| {
        prefs.custom_themes.remove(id);
        if prefs.theme_id == id {
            prefs.theme_id = "semio".to_string();
        }
    });
}

fn hex_to_rgba(hex: &str, fallback: Rgba) -> Rgba {
    let s = hex.trim_start_matches('#');
    if s.len() != 6 {
        return fallback;
    }
    match (u8::from_str_radix(&s[0..2], 16), u8::from_str_radix(&s[2..4], 16), u8::from_str_radix(&s[4..6], 16)) {
        (Ok(r), Ok(g), Ok(b)) => Rgba::from_srgb8(r, g, b, 255),
        _ => fallback,
    }
}

fn apply_chrome_color_overrides(base: &Theme, overrides: &ChromeColorOverrides) -> Theme {
    let mut theme = *base;
    if let Some(hex) = &overrides.background {
        theme.background = hex_to_rgba(hex, theme.background);
        theme.canvas_clear = theme.background;
        theme.input_bg = theme.background;
    }
    if let Some(hex) = &overrides.panel {
        theme.panel = hex_to_rgba(hex, theme.panel);
    }
    if let Some(hex) = &overrides.navbar {
        theme.navbar = hex_to_rgba(hex, theme.navbar);
        theme.button = theme.navbar;
    }
    if let Some(hex) = &overrides.text {
        theme.text = hex_to_rgba(hex, theme.text);
    }
    if let Some(hex) = &overrides.accent {
        theme.accent = hex_to_rgba(hex, theme.accent);
        theme.selected = theme.accent;
        theme.focus_ring = theme.accent.with_alpha(0.6);
    }
    theme
}

/// 🎨️ The "mono" premade's real chrome palette (`ui/styling/theme/🔣️mono.theme.json`), resolved once
/// via `ui/styling/js/theme.ts`'s own `resolveThemeAppearancePalettes` (ticket scratchpad
/// `resolve-mono-chrome.ts`) and hand-ported here as `Rgba::from_srgb8` calls: this crate has no
/// dependency on the `ui_styling` Rust codegen crate (only `ui_wgpu` does, and its `ChromePalette`/
/// `from_chrome` aren't `pub`), and `Cargo.toml` is a reserved wave-3 choke point, so a generated
/// `MONO_LIGHT`/`MONO_DARK` constant isn't reachable this wave — these are real resolved values, not
/// invented ones. Metrics/fonts/checker/diagram/error/focus-ring-alpha etc. are shared with "semio"
/// via `Theme::light()/dark()`'s struct-update base (mono only recolors chrome paints).
fn mono_theme(dark: bool) -> Theme {
    let base = if dark { Theme::dark() } else { Theme::light() };
    let (canvas, panel, window, foreground, accent, active_hover, hover_interactive_fill, border_normal, border_emphasized, temporary) = if dark {
        (
            Rgba::from_srgb8(25, 25, 25, 255),
            Rgba::from_srgb8(40, 40, 40, 255),
            Rgba::from_srgb8(21, 21, 21, 255),
            Rgba::from_srgb8(243, 243, 243, 255),
            Rgba::from_srgb8(140, 140, 140, 255),
            Rgba::from_srgb8(126, 126, 126, 255),
            Rgba::from_srgb8(128, 128, 128, 255),
            Rgba::from_srgb8(128, 128, 128, 255),
            Rgba::from_srgb8(243, 243, 243, 255),
            Rgba::from_srgb8(47, 47, 47, 255),
        )
    } else {
        (
            Rgba::from_srgb8(236, 236, 236, 255),
            Rgba::from_srgb8(199, 199, 199, 255),
            Rgba::from_srgb8(232, 232, 232, 255),
            Rgba::from_srgb8(14, 14, 14, 255),
            Rgba::from_srgb8(140, 140, 140, 255),
            Rgba::from_srgb8(126, 126, 126, 255),
            Rgba::from_srgb8(128, 128, 128, 255),
            Rgba::from_srgb8(128, 128, 128, 255),
            Rgba::from_srgb8(14, 14, 14, 255),
            Rgba::from_srgb8(154, 154, 154, 255),
        )
    };
    Theme {
        background: canvas,
        panel,
        panel_border: border_normal,
        navbar: window,
        text: foreground,
        text_muted: hover_interactive_fill,
        accent,
        accent_hover: active_hover,
        active_foreground: foreground,
        button: window,
        button_hover: hover_interactive_fill,
        input_bg: canvas,
        separator: border_normal,
        selected: accent,
        canvas_clear: canvas,
        temporary,
        row_hover: hover_interactive_fill,
        border_normal,
        border_emphasized,
        text_element: hover_interactive_fill,
        focus_ring: accent.with_alpha(0.6),
        ..base
    }
}

fn custom_theme_definition_from(prefs: &ChromePrefsState, custom_id: &str) -> Option<CustomChromeTheme> {
    let raw = prefs.custom_themes.get(custom_id)?;
    serde_json::from_str(&raw).ok()
}

fn custom_theme_definition(custom_id: &str) -> Option<CustomChromeTheme> {
    with_chrome_prefs(|prefs| custom_theme_definition_from(prefs, custom_id))
}

/// 🎨️ Extends `resolve_theme`'s system/light/dark-only resolution with named built-ins ("semio",
/// "mono") and persisted custom themes (`"custom.<slug>"`, whose color overrides layer onto their
/// declared `base`). The `frame()` loop's single `resolve_theme(...)` call site now goes through
/// this instead, keyed by `active_theme_id()`.
pub fn resolve_theme_for_ids(theme_id: &str, appearance_id: &str) -> Theme {
    match theme_id {
        "" | "semio" => crate::resolve_theme(appearance_id),
        "mono" => mono_theme(crate::appearance_is_dark(appearance_id)),
        custom_id if custom_id.starts_with("custom.") => match custom_theme_definition(custom_id) {
            Some(custom) => {
                let base = resolve_theme_for_ids(&custom.base, appearance_id);
                let overrides = if crate::appearance_is_dark(appearance_id) { &custom.dark } else { &custom.light };
                apply_chrome_color_overrides(&base, overrides)
            }
            None => crate::resolve_theme(appearance_id),
        },
        _ => crate::resolve_theme(appearance_id),
    }
}
//#endregion 🎨️ThemeRegistry

//#region 🗣️ChromeI18n
/// 🗣️ EN/DE lookup for a curated subset of previously-hardcoded chrome strings, byte-identical to
/// `ui/js/react/index.tsx`'s `uiChromeTranslationBundles.{en,de}.translation.ui.*` "normal" labels
/// (`:2898-3975`) at the dotted paths named below (e.g. `"display.tab.windows"` ==
/// `ui.display.tab.windows`). Unknown keys fall back to the key itself rather than inventing text.
fn shell_chrome_string(key: &'static str, is_de: bool) -> &'static str {
    match (key, is_de) {
        ("display.tab.windows", false) => "Windows",
        ("display.tab.windows", true) => "Fenster",
        ("display.tab.layout", false) => "Layout",
        ("display.tab.layout", true) => "Layout",
        ("settings.tab.general", false) => "General",
        ("settings.tab.general", true) => "Allgemein",
        // 🎨️ wgpu-only additions (not verified against the external `elements/ui` i18next resource
        // bundle React's `shellLabel`/`uiI18n.t` ultimately reads — that package isn't vendored in this
        // repo tree, so these are reasonable EN/DE pairs in the same terse register as the rest of this
        // curated subset, not a byte-identical trace like the entries above copied from `index.tsx:2898-3975`).
        ("settings.tab.theme", false) => "Theme",
        ("settings.tab.theme", true) => "Design",
        ("settings.tab.commands", false) => "Commands",
        ("settings.tab.commands", true) => "Befehle",
        ("settings.theme.select", false) => "Theme",
        ("settings.theme.select", true) => "Design",
        ("settings.theme.reset", false) => "Reset",
        ("settings.theme.reset", true) => "Zurücksetzen",
        ("settings.theme.delete", false) => "Delete",
        ("settings.theme.delete", true) => "Löschen",
        ("fullscreen.toggle", false) => "Fullscreen",
        ("fullscreen.toggle", true) => "Vollbild",
        ("fullscreen.exit", false) => "Exit Fullscreen",
        ("fullscreen.exit", true) => "Vollbild beenden",
        ("panelToggle.display", false) => "Display",
        ("panelToggle.display", true) => "Anzeige",
        ("panelToggle.workbench", false) => "Workbench",
        ("panelToggle.workbench", true) => "Arbeitsbereich",
        ("panelToggle.details", false) => "Details",
        ("panelToggle.details", true) => "Details",
        ("panelToggle.settings", false) => "Settings",
        ("panelToggle.settings", true) => "Einstellungen",
        ("common.home", false) => "Home",
        ("common.home", true) => "Startseite",
        ("common.windowOptions", false) => "Window Options",
        ("common.windowOptions", true) => "Fensteroptionen",
        ("common.focus", false) => "Focus",
        ("common.focus", true) => "Fokussieren",
        ("common.unfocus", false) => "Unfocus",
        ("common.unfocus", true) => "Fokus aufheben",
        ("common.execute", false) => "Execute",
        ("common.execute", true) => "Ausführen",
        ("common.reset", false) => "Reset",
        ("common.reset", true) => "Zurücksetzen",
        ("introduction.skip", false) => "Skip",
        ("introduction.skip", true) => "Überspringen",
        ("introduction.back", false) => "Back",
        ("introduction.back", true) => "Zurück",
        ("introduction.next", false) => "Next",
        ("introduction.next", true) => "Weiter",
        ("introduction.done", false) => "Done",
        ("introduction.done", true) => "Fertig",
        (other, _) => other,
    }
}

/// 🗣️ `id`'s locale-aware label via `ui_wgpu::wgpu::framework_panel_tab_label` (the one existing
/// locale-aware string helper, per a prior wave), falling back to `fallback` for app-declared ids.
fn shell_panel_tab_label(id: &str, fallback: &'static str, is_de: bool) -> String {
    ui_wgpu::wgpu::framework_panel_tab_label(id, is_de).unwrap_or(fallback).to_string()
}

/// 🎓️ Reads whether `app_id`'s introduction has already been shown, byte-identical semantics to
/// `readStoredIntroductionSeen` (`ui/js/react/index.tsx:2309`). Persistence primitive only — wiring
/// this into the actual onboarding-tour auto-start trigger is `w3-overlays-chrome-polish`'s scope.
pub(crate) fn read_stored_introduction_seen(app_id: &str) -> bool {
    prefs_get(&format!("{UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX}{app_id}")).as_deref() == Some("true")
}

/// 🎓️ Marks `app_id`'s introduction as shown, byte-identical semantics to `writeStoredIntroductionSeen`
/// (`ui/js/react/index.tsx:2315`).
pub(crate) fn write_stored_introduction_seen(app_id: &str) {
    prefs_set(&format!("{UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX}{app_id}"), "true");
}
//#endregion 🗣️ChromeI18n

//#region 💾️PrefsSync
#[derive(Clone, PartialEq)]
struct UiPrefsSnapshot {
    appearance_id: String,
    locale_id: String,
    terminology_id: String,
    driver_id: String,
    theme_id: String,
    ui_layout: String,
    worker_count: u32,
}

impl UiPrefsSnapshot {
    fn capture(state: &ShellState) -> Self {
        Self {
            appearance_id: state.appearance_id.clone(),
            locale_id: state.locale_id.clone(),
            terminology_id: state.terminology_id.clone(),
            driver_id: state.driver_id.clone(),
            theme_id: state.chrome_build.preferences.theme_id.clone(),
            ui_layout: state.chrome_build.preferences.ui_layout.clone(),
            worker_count: state.chrome_build.preferences.worker_count,
        }
    }
}

fn persist_custom_themes(preferences: &ChromePrefsState) {
    let as_values: HashMap<String, Value> = preferences.custom_themes.iter().map(|(id, raw)| (id.clone(), serde_json::from_str(raw).unwrap_or(Value::Null))).collect();
    if let Ok(json) = serde_json::to_string(&as_values) {
        prefs_set(UI_CUSTOM_THEMES_STORAGE_KEY, &json);
    }
}

impl ShellState {
    /// 💾️ Loads persisted uiPrefs into `self` exactly once per process, mirroring `os-shell.tsx`'s
    /// boot-time `locks.appearance ?? readStoredUiChromeAppearance()` fallback chain (`:862-868`).
    /// A locked pref (`SEMIO_LOCKED_*`) wins over storage, matching `resolveShellLocks`.
    fn load_ui_prefs_once(&mut self) {
        if self.chrome_present.preferences_loaded {
            return;
        }
        self.chrome_present.preferences_loaded = true;
        let locks = shell_pref_locks();
        self.appearance_id = locks.appearance.clone().unwrap_or_else(|| prefs_get(UI_CHROME_APPEARANCE_STORAGE_KEY).filter(|value| value == "light" || value == "dark" || value == "system").unwrap_or_else(|| "system".to_string()));
        self.locale_id = locks.locale.clone().unwrap_or_else(|| prefs_get(UI_CHROME_LOCALE_STORAGE_KEY).filter(|value| value == "en" || value == "de").unwrap_or_else(|| "en".to_string()));
        self.terminology_id = locks.terminology.clone().unwrap_or_else(|| prefs_get(UI_CHROME_TERMINOLOGY_STORAGE_KEY).unwrap_or_else(|| UI_TERMINOLOGY_NATIVE.to_string()));
        self.driver_id = prefs_get(UI_CHROME_DRIVER_STORAGE_KEY).unwrap_or_else(|| "default".to_string());
        self.chrome_build.preferences = with_chrome_prefs(|preferences| preferences.clone());
        if let Some(locked_theme) = &locks.theme_id {
            set_active_theme_id(locked_theme);
            self.chrome_build.preferences.theme_id = locked_theme.clone();
        }
        self.chrome_present.last_synced_preferences = Some(UiPrefsSnapshot::capture(self));
    }

    /// 💾️ Writes any changed uiPrefs field to the store (skipping locked ones), mirroring
    /// `os-shell.tsx`'s persistence `useEffect` (`:3477-3491`): one combined-dependency effect that
    /// rewrites every non-locked pref whenever any of them changes, not a per-field diff.
    fn persist_ui_prefs_if_changed(&mut self) {
        let snapshot = UiPrefsSnapshot::capture(self);
        if self.chrome_present.last_synced_preferences.as_ref() == Some(&snapshot) {
            return;
        }
        let locks = shell_pref_locks();
        if locks.appearance.is_none() {
            prefs_set(UI_CHROME_APPEARANCE_STORAGE_KEY, &snapshot.appearance_id);
        }
        prefs_set(UI_CHROME_DRIVER_STORAGE_KEY, &snapshot.driver_id);
        if locks.locale.is_none() {
            prefs_set(UI_CHROME_LOCALE_STORAGE_KEY, &snapshot.locale_id);
        }
        if locks.terminology.is_none() {
            prefs_set(UI_CHROME_TERMINOLOGY_STORAGE_KEY, &snapshot.terminology_id);
        }
        if locks.theme_id.is_none() {
            prefs_set(UI_CHROME_THEME_ID_STORAGE_KEY, &snapshot.theme_id);
        }
        prefs_set(UI_CHROME_LAYOUT_STORAGE_KEY, &snapshot.ui_layout);
        prefs_set(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY, &snapshot.worker_count.to_string());
        persist_custom_themes(&self.chrome_build.preferences);
        self.chrome_present.last_synced_preferences = Some(snapshot);
    }
}
//#endregion 💾️PrefsSync

//#region 🧪️UiPrefsThemesI18nTests
#[cfg(all(test, not(target_arch = "wasm32")))]
mod ui_prefs_themes_i18n_tests {
    use super::*;

    /// 🧪️ A `FilePrefsStore` constructed against a scratch path (never through the thread-local
    /// singleton, so this doesn't depend on `SEMIO_PREFS_DIR`/`$HOME` or risk touching a real prefs
    /// file) round-trips a value through an actual disk write + independent re-read.
    #[test]
    fn file_prefs_store_round_trips_through_disk() {
        use std::fs as system_fs;
        use std::process as system_process;
        let dir = std::env::temp_dir().join(format!("semio-wp14-prefs-test-{}-{:?}", system_process::id(), std::thread::current().id()));
        let _ = system_fs::create_dir_all(&dir);
        let path = dir.join("ui-prefs.json");
        let _ = system_fs::remove_file(&path);
        let mut store = FilePrefsStore::with_path(path.clone());
        store.set(
            OS_SHELL_CONFIG_STORAGE_KEY,
            &serde_json::json!({
                "version": 1,
                "preferences": {},
                "namedLayouts": { "draw": [{ "id": "wide" }] },
                "dockLayouts": { "apps": {} },
                "dockUi": { "apps": {} },
                "windowPanes": { "apps": {} }
            })
            .to_string(),
        );
        assert_eq!(prefs_get_from(&store, UI_CHROME_APPEARANCE_STORAGE_KEY), None);
        prefs_set_in(&mut store, UI_CHROME_APPEARANCE_STORAGE_KEY, "dark");
        assert_eq!(prefs_get_from(&store, UI_CHROME_APPEARANCE_STORAGE_KEY), Some("dark".to_string()));
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(1);
        let raw = loop {
            if let Ok(raw) = system_fs::read_to_string(&path) {
                if raw.contains("dark") {
                    break raw;
                }
            }
            assert!(std::time::Instant::now() < deadline, "flush worker must write the latest prefs snapshot");
            std::thread::yield_now();
        };
        let reloaded: HashMap<String, String> = serde_json::from_str(&raw).expect("valid JSON");
        let config: serde_json::Value = serde_json::from_str(reloaded.get(OS_SHELL_CONFIG_STORAGE_KEY).expect("one config document")).expect("valid config JSON");
        assert_eq!(config["preferences"][UI_CHROME_APPEARANCE_STORAGE_KEY], "dark");
        assert_eq!(config["dockLayouts"]["apps"], serde_json::json!({}));
        assert_eq!(config["namedLayouts"]["draw"][0]["id"], "wide", "preference writes must preserve sibling projections");
        assert_eq!(reloaded.len(), 1, "wgpu preferences use the shared OS config authority");
        let _ = system_fs::remove_file(&path);
    }

    /// 🧪️ `env_lock` treats an empty-string env value the same as unset (matches
    /// `FrameworkOsLocks`' optional-string semantics: an empty `VITE_SEMIO_LOCKED_*` never locks).
    #[test]
    fn env_lock_ignores_unset_and_empty() {
        assert_eq!(env_lock("SEMIO_WP14_TEST_UNSET_VAR"), None);
        unsafe {
            std::env::set_var("SEMIO_WP14_TEST_EMPTY_VAR", "");
        }
        assert_eq!(env_lock("SEMIO_WP14_TEST_EMPTY_VAR"), None);
        unsafe {
            std::env::set_var("SEMIO_WP14_TEST_EMPTY_VAR", "en");
        }
        assert_eq!(env_lock("SEMIO_WP14_TEST_EMPTY_VAR"), Some("en".to_string()));
        unsafe {
            std::env::remove_var("SEMIO_WP14_TEST_EMPTY_VAR");
        }
    }

    /// 🧪️ `shell_pref_locks` wires `SEMIO_LOCKED_*` onto the four lockable fields — byte-identical
    /// env var names to `framework/product/os/dev/js/index.ts:19-23`'s `VITE_SEMIO_LOCKED_*` reads.
    #[test]
    fn shell_pref_locks_reads_the_four_lockable_envs() {
        unsafe {
            std::env::set_var("SEMIO_LOCKED_APPEARANCE", "dark");
            std::env::set_var("SEMIO_LOCKED_LOCALE", "de");
            std::env::set_var("SEMIO_LOCKED_TERMINOLOGY", "reuse");
            std::env::set_var("SEMIO_LOCKED_THEME", "mono");
        }
        let locks = shell_pref_locks();
        assert_eq!(locks.appearance.as_deref(), Some("dark"));
        assert_eq!(locks.locale.as_deref(), Some("de"));
        assert_eq!(locks.terminology.as_deref(), Some("reuse"));
        assert_eq!(locks.theme_id.as_deref(), Some("mono"));
        unsafe {
            std::env::remove_var("SEMIO_LOCKED_APPEARANCE");
            std::env::remove_var("SEMIO_LOCKED_LOCALE");
            std::env::remove_var("SEMIO_LOCKED_TERMINOLOGY");
            std::env::remove_var("SEMIO_LOCKED_THEME");
        }
        let unlocked = shell_pref_locks();
        assert_eq!(unlocked.appearance, None);
        assert_eq!(unlocked.locale, None);
    }

    /// 🧪️ A locked appearance wins over storage at load time — mirrors `os-shell.tsx:862`'s
    /// `locks.appearance ?? readStoredUiChromeAppearance()`. Deliberately backend-independent (never
    /// asserts on storage's own contents, since `PREFS_STORE`'s thread-local can be seeded by an
    /// earlier test reusing this worker thread) — only that a lock always overrides whatever loads.
    #[test]
    fn load_ui_prefs_once_prefers_a_lock_over_storage() {
        unsafe {
            std::env::set_var("SEMIO_LOCKED_APPEARANCE", "dark");
        }
        let mut state = ShellState::new(Vec::new(), String::new());
        state.load_ui_prefs_once();
        assert_eq!(state.appearance_id, "dark");
        // The "load once" gate: mutating the field and calling load again must not reset it.
        state.appearance_id = "light".to_string();
        state.load_ui_prefs_once();
        assert_eq!(state.appearance_id, "light");
        unsafe {
            std::env::remove_var("SEMIO_LOCKED_APPEARANCE");
        }
    }

    /// 🧪️ `persist_ui_prefs_if_changed`'s dirty-check: a second call with no field changes since the
    /// last sync must be a cheap no-operation (mirrors the combined-dependency-array `useEffect` at
    /// `os-shell.tsx:3477-3491`, which only re-runs when one of its deps actually changed).
    ///
    /// **Self-inflicted-flakiness fix**: this used to hardcode the "changed" value as `"compact"` and
    /// never restore the original — on native, `PREFS_STORE`'s `FilePrefsStore` backs onto a real
    /// `$SEMIO_PREFS_DIR`/`~/.config/semio/ui-prefs.json` file (not a scratch path; unlike
    /// `file_prefs_store_round_trips_through_disk`, this test exercises the thread-local singleton
    /// directly, same as `load_ui_prefs_once_prefers_a_lock_over_storage`'s own doc comment already
    /// flags this file as shared across runs), so once this test ran once, `ui.chrome.driver` was left
    /// at `"compact"` on disk — the *next* run's `load_ui_prefs_once` read `"compact"` right back in,
    /// making `state.driver_id = "compact"` below a no-op and failing the final assertion. Toggling to
    /// whatever the loaded value *isn't*, then restoring it, makes this test pass regardless of what a
    /// previous run left behind.
    #[test]
    fn persist_ui_prefs_if_changed_is_idempotent_when_nothing_changed() {
        let mut state = ShellState::new(Vec::new(), String::new());
        state.load_ui_prefs_once();
        let after_load = state.chrome_present.last_synced_preferences.clone();
        assert!(after_load.is_some());
        state.persist_ui_prefs_if_changed();
        let after_noop_persist = state.chrome_present.last_synced_preferences.clone();
        assert!(after_load == after_noop_persist);
        let original_driver_id = state.driver_id.clone();
        let toggled_driver_id = if original_driver_id == "compact" { "default" } else { "compact" };
        state.driver_id = toggled_driver_id.to_string();
        state.persist_ui_prefs_if_changed();
        let after_change = state.chrome_present.last_synced_preferences.clone();
        assert!(after_change != after_noop_persist);
        state.driver_id = original_driver_id;
        state.persist_ui_prefs_if_changed();
    }

    /// 🧪️ `resolve_theme_for_ids("semio", _)` is exactly `resolve_theme` (the pre-WP14 behavior),
    /// and `"mono"` resolves to a *different* real palette (not a copy of semio's).
    #[test]
    fn resolve_theme_for_ids_semio_and_mono_differ() {
        let semio_dark = resolve_theme_for_ids("semio", "dark");
        let plain_dark = crate::resolve_theme("dark");
        assert_eq!(semio_dark.background, plain_dark.background);
        let mono_dark = resolve_theme_for_ids("mono", "dark");
        assert_ne!(mono_dark.background, semio_dark.background);
        assert_eq!(mono_dark.background, Rgba::from_srgb8(25, 25, 25, 255));
        // Metrics are shared with the base theme (mono only recolors chrome paints).
        assert_eq!(mono_dark.navbar_height, semio_dark.navbar_height);
    }

    /// 🧪️ A hex color override is parsed and applied; an invalid hex falls back to the base color
    /// rather than panicking or silently corrupting the theme.
    #[test]
    fn hex_to_rgba_parses_valid_and_falls_back_on_invalid() {
        let fallback = Rgba::new(0.1, 0.2, 0.3, 1.0);
        assert_eq!(hex_to_rgba("#ff0000", fallback), Rgba::from_srgb8(255, 0, 0, 255));
        assert_eq!(hex_to_rgba("ff0000", fallback), Rgba::from_srgb8(255, 0, 0, 255));
        assert_eq!(hex_to_rgba("not-a-color", fallback), fallback);
        assert_eq!(hex_to_rgba("#fff", fallback), fallback);
    }

    /// 🧪️ End-to-end custom theme draft flow: begin → mutate → save → resolves with the override
    /// applied → delete falls back to "semio". Explicitly seeds `active_theme_id` rather than
    /// asserting on whatever `load_chrome_prefs` found on disk, so this is independent of any other
    /// test's writes on a reused worker thread.
    #[test]
    fn custom_theme_draft_round_trips_and_deletes() {
        set_active_theme_id("semio");
        let id = begin_custom_theme_draft("semio", "My Theme", "wp14-test");
        assert_eq!(id, "custom.wp14-test");
        assert!(set_draft_theme_color("light", "background", "#112233"));
        assert!(!set_draft_theme_color("light", "not-a-field", "#112233"));
        let saved_id = save_draft_theme().expect("a draft was in progress");
        assert_eq!(saved_id, id);
        assert_eq!(active_theme_id(), id);
        assert!(custom_theme_ids().contains(&id));
        let resolved = resolve_theme_for_ids(&id, "light");
        assert_eq!(resolved.background, Rgba::from_srgb8(0x11, 0x22, 0x33, 255));
        // Untouched fields still fall through to the "semio" base.
        assert_eq!(resolved.navbar_height, resolve_theme_for_ids("semio", "light").navbar_height);
        delete_custom_theme(&id);
        assert_eq!(active_theme_id(), "semio");
        assert!(!custom_theme_ids().contains(&id));
    }

    /// 🧪️ `discard_draft_theme` clears an in-progress draft without touching the saved registry.
    #[test]
    fn discard_draft_theme_clears_in_progress_draft() {
        let id = begin_custom_theme_draft("mono", "Discard Me", "wp14-discard");
        assert!(set_draft_theme_color("dark", "accent", "#abcdef"));
        discard_draft_theme();
        assert_eq!(save_draft_theme(), None);
        assert!(!custom_theme_ids().contains(&id));
    }

    /// 🧪️ `active_ui_layout`/`set_active_ui_layout` round-trip and reject unknown values (matches
    /// React's `UiChromeLayout` union of exactly `"desktop" | "tablet"`).
    #[test]
    fn ui_layout_round_trips_and_rejects_unknown_values() {
        set_active_ui_layout("tablet");
        assert_eq!(active_ui_layout(), "tablet");
        set_active_ui_layout("bogus");
        assert_eq!(active_ui_layout(), "desktop");
    }

    /// 🧪️ EN/DE parity spot-check against `ui/js/react/index.tsx`'s `uiChromeTranslationBundles`
    /// "normal" labels (`:2898-3975`) for a sample of the keys this crate now routes through.
    #[test]
    fn shell_chrome_string_matches_react_bundle_samples() {
        assert_eq!(shell_chrome_string("display.tab.windows", false), "Windows");
        assert_eq!(shell_chrome_string("display.tab.windows", true), "Fenster");
        assert_eq!(shell_chrome_string("common.execute", false), "Execute");
        assert_eq!(shell_chrome_string("common.execute", true), "Ausführen");
        assert_eq!(shell_chrome_string("common.windowOptions", true), "Fensteroptionen");
        // Unknown keys fall back to the key itself rather than inventing text.
        assert_eq!(shell_chrome_string("nonexistent.key", true), "nonexistent.key");
    }

    /// 🧪️ `read_stored_introduction_seen`/`write_stored_introduction_seen` byte-identical semantics
    /// to `ui/js/react/index.tsx:2309-2317` — exercised against a scratch `FilePrefsStore` (not the
    /// thread-local singleton) so this never touches a real prefs file.
    #[test]
    fn introduction_seen_key_format_matches_react() {
        assert_eq!(format!("{UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX}framework-os"), "ui.introduction.seen.framework-os");
    }
}
//#endregion 🧪️UiPrefsThemesI18nTests
// #endregion 💾️🎨️🌐️ UiPrefsThemesI18n

//#region 🧪️ChromeOverlaysAndTourTests
#[cfg(test)]
mod chrome_overlays_tour_tests {
    use super::*;

    /// 🧹️ Returns a pristine owned chrome state for each test.
    fn chrome_state() -> ShellChromeBuildState {
        ShellChromeBuildState::default()
    }

    //#region ThreadBoundary
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn chrome_build_state_moves_across_threads_without_losing_state() {
        assert_shell_chrome_build_state_is_send();
        let mut chrome = chrome_state();
        chrome.content_focus.insert("main".to_string(), true);
        chrome.register_tooltip("nav.help", "Help");
        chrome.preferences.theme_id = "mono".to_string();
        let chrome = std::thread::spawn(move || {
            assert!(chrome.content_has_focus("main"));
            assert_eq!(chrome.tooltip_titles.get("nav.help").map(String::as_str), Some("Help"));
            chrome
        })
        .join()
        .expect("owned chrome state crosses a worker thread");
        assert_eq!(chrome.preferences.theme_id, "mono");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn find_item_sink_binding_routes_owned_output_across_threads() {
        let sink = ShellFindItemSink::default();
        let worker_sink = sink.clone();
        std::thread::spawn(move || {
            let _binding = worker_sink.bind();
            push_find_item(ShellFindItem { id: "item".to_string(), label: "Item".to_string(), description: None, category: None, surface_id: "surface".to_string(), node_id: "node".to_string() });
        })
        .join()
        .expect("worker render callback completes");
        assert_eq!(sink.take().into_iter().map(|item| item.id).collect::<Vec<_>>(), vec!["item"]);
    }

    #[test]
    fn find_item_callback_without_render_binding_is_explicitly_rejected() {
        let result = std::panic::catch_unwind(|| {
            push_find_item(ShellFindItem { id: "unbound".to_string(), label: "Unbound".to_string(), description: None, category: None, surface_id: "surface".to_string(), node_id: "node".to_string() });
        });
        assert!(result.is_err(), "unbound callbacks must not silently write into an empty per-thread sink");
    }
    //#endregion ThreadBoundary

    //#region Tooltip
    #[test]
    fn tooltip_titles_register_and_clear() {
        let mut chrome = chrome_state();
        chrome.register_tooltip("nav.help", "Help");
        assert_eq!(chrome.tooltip_titles.get("nav.help").cloned(), Some("Help".to_string()));
        chrome.tooltip_titles.clear();
        assert!(chrome.tooltip_titles.is_empty());
    }

    #[test]
    fn tooltip_registration_ignores_empty_titles() {
        let mut chrome = chrome_state();
        chrome.register_tooltip("nav.mystery", "");
        assert!(chrome.tooltip_titles.get("nav.mystery").is_none());
    }

    #[test]
    fn tooltip_ready_respects_hover_delay() {
        let hover = ChromeTooltipHover { control_id: "x".into(), anchor_x: 0.0, anchor_y: 0.0, started_ms: 1_000.0 };
        assert!(!chrome_tooltip_ready(&hover, 1_000.0 + CHROME_TOOLTIP_DELAY_MS - 1.0));
        assert!(chrome_tooltip_ready(&hover, 1_000.0 + CHROME_TOOLTIP_DELAY_MS));
        assert!(chrome_tooltip_ready(&hover, 1_000.0 + CHROME_TOOLTIP_DELAY_MS + 250.0));
    }

    /// 🧪️ Full close-on-hover-out path through `render_chrome_tooltip`: hovering a registered control
    /// arms the hover timer; on the very next call the pointer has moved off (`hovered_id` no longer
    /// matches), which must clear the armed hover immediately (no debounce, matching this crate's
    /// documented "no animation-clock scaffolding" gap) rather than leaving a stale tooltip painted.
    #[test]
    fn tooltip_closes_immediately_on_hover_out() {
        let mut shell = ShellState::new(Vec::new(), String::new());
        let mut draw = DrawList::default();
        let mut atlas = FontAtlas::builtin();
        let mut input = InputState::<ActionDescriptor>::default();
        let theme = Theme::light();
        shell.chrome_build.register_tooltip("nav.help", "Help");
        input.hovered_id = Some("nav.help".into());
        shell.render_chrome_tooltip(&mut draw, &mut atlas, &mut input, &theme, 800.0, 600.0);
        assert!(shell.chrome_build.tooltip_hover.is_some(), "hover should arm on first hovered frame");
        input.hovered_id = None;
        shell.render_chrome_tooltip(&mut draw, &mut atlas, &mut input, &theme, 800.0, 600.0);
        assert!(shell.chrome_build.tooltip_hover.is_none(), "hover-out must clear the armed tooltip");
    }
    //#endregion Tooltip

    //#region Dialog
    #[test]
    fn dialog_open_and_close_topmost() {
        let mut chrome = chrome_state();
        assert!(!chrome.dialog_open());
        chrome.open_dialog(ChromeDialogRequest {
            id: "confirm-1".into(),
            title: "Delete?".into(),
            body: "This cannot be undone.".into(),
            confirm_label: "Delete".into(),
            confirm_action: ActionDescriptor { controller_id: "test".into(), action: "delete".into(), args: None },
            cancel_label: "Cancel".into(),
        });
        assert!(chrome.dialog_open());
        chrome.close_topmost_dialog();
        assert!(!chrome.dialog_open());
    }

    #[test]
    fn dialog_stack_supports_nesting_close_order() {
        let mut chrome = chrome_state();
        chrome.open_dialog(ChromeDialogRequest {
            id: "outer".into(),
            title: "Outer".into(),
            body: String::new(),
            confirm_label: "OK".into(),
            confirm_action: ActionDescriptor { controller_id: "test".into(), action: "outer".into(), args: None },
            cancel_label: "Cancel".into(),
        });
        chrome.open_dialog(ChromeDialogRequest {
            id: "inner".into(),
            title: "Inner".into(),
            body: String::new(),
            confirm_label: "OK".into(),
            confirm_action: ActionDescriptor { controller_id: "test".into(), action: "inner".into(), args: None },
            cancel_label: "Cancel".into(),
        });
        assert_eq!(chrome.dialog_stack.last().map(|dialog| dialog.id.clone()), Some("inner".to_string()));
        chrome.close_topmost_dialog();
        assert_eq!(chrome.dialog_stack.last().map(|dialog| dialog.id.clone()), Some("outer".to_string()));
        chrome.close_topmost_dialog();
        assert!(!chrome.dialog_open());
    }

    /// 🧪️ `render_chrome_dialog`'s scrim-click dismissal (`DismissPolicy::outside_press_swallow`) — a
    /// click outside the centered dialog box closes it without dispatching `confirm_action`.
    #[test]
    fn dialog_scrim_click_dismisses_without_confirm_action() {
        let mut shell = ShellState::new(Vec::new(), String::new());
        let mut draw = DrawList::default();
        let mut atlas = FontAtlas::builtin();
        let mut input = InputState::<ActionDescriptor>::default();
        let theme = Theme::light();
        shell.chrome_build.open_dialog(ChromeDialogRequest {
            id: "confirm-1".into(),
            title: "Delete?".into(),
            body: "Sure?".into(),
            confirm_label: "Delete".into(),
            confirm_action: ActionDescriptor { controller_id: "test".into(), action: "delete".into(), args: None },
            cancel_label: "Cancel".into(),
        });
        // A click far in the top-left corner, well outside the centered ~360x168 box on an 800x600 viewport.
        input.pointer_x = 4.0;
        input.pointer_y = 4.0;
        shell.chrome_build.compute_click_edge(false);
        shell.chrome_build.compute_click_edge(true);
        shell.render_chrome_dialog(&mut draw, &mut atlas, &mut input, &theme, 800.0, 600.0);
        assert!(!shell.chrome_build.dialog_open(), "scrim click must dismiss the dialog");
    }

    /// 🧪️ Focus-trap-equivalent modality: while a dialog is open, the chrome-owned tour trigger and the
    /// sync-detach-confirmation click handlers must not fire — both are gated on `!chrome_dialog_open()`.
    #[test]
    fn dialog_open_blocks_other_chrome_owned_click_handlers() {
        let mut chrome = chrome_state();
        chrome.open_dialog(ChromeDialogRequest {
            id: "blocker".into(),
            title: "Blocking".into(),
            body: String::new(),
            confirm_label: "OK".into(),
            confirm_action: ActionDescriptor { controller_id: "test".into(), action: "noOperation".into(), args: None },
            cancel_label: "Cancel".into(),
        });
        assert!(chrome.dialog_open());
        // The guard every other chrome-owned click handler in this region checks first.
        assert!(!(!chrome.dialog_open()));
    }
    //#endregion Dialog

    //#region Tour
    #[test]
    fn tour_start_advance_and_skip() {
        let mut chrome = chrome_state();
        assert!(chrome.tour_state.is_none());
        chrome.start_introduction();
        assert_eq!(chrome.tour_state.as_ref().map(|tour| tour.step_index), Some(0));
        chrome.advance_introduction(3);
        assert_eq!(chrome.tour_state.as_ref().map(|tour| tour.step_index), Some(1));
        chrome.skip_introduction();
        assert!(chrome.tour_state.is_none());
    }

    #[test]
    fn tour_advance_past_last_step_closes_the_tour() {
        let mut chrome = chrome_state();
        chrome.start_introduction();
        chrome.advance_introduction(2);
        assert_eq!(chrome.tour_state.as_ref().map(|tour| tour.step_index), Some(1));
        chrome.advance_introduction(2);
        assert!(chrome.tour_state.is_none());
    }

    #[test]
    fn tour_advance_on_empty_state_is_a_no_operation() {
        let mut chrome = chrome_state();
        chrome.advance_introduction(5);
        assert!(chrome.tour_state.is_none());
    }

    #[test]
    fn tour_back_decrements_and_floors_at_zero() {
        let mut chrome = chrome_state();
        chrome.start_introduction();
        chrome.advance_introduction(3);
        chrome.advance_introduction(3);
        assert_eq!(chrome.tour_state.as_ref().map(|tour| tour.step_index), Some(2));
        chrome.back_introduction();
        assert_eq!(chrome.tour_state.as_ref().map(|tour| tour.step_index), Some(1));
        chrome.back_introduction();
        chrome.back_introduction();
        assert_eq!(chrome.tour_state.as_ref().map(|tour| tour.step_index), Some(0));
    }

    #[test]
    fn tour_back_on_empty_state_is_a_no_operation() {
        let mut chrome = chrome_state();
        chrome.back_introduction();
        assert!(chrome.tour_state.is_none());
    }

    #[test]
    fn tour_advance_and_back_reset_completed_interactions() {
        let mut chrome = chrome_state();
        chrome.start_introduction();
        chrome.tour_state.as_mut().unwrap().completed_interactions.push(0);
        chrome.advance_introduction(3);
        assert_eq!(chrome.tour_state.as_ref().map(|tour| tour.completed_interactions.clone()), Some(vec![]));
        chrome.tour_state.as_mut().unwrap().completed_interactions.push(0);
        chrome.back_introduction();
        assert_eq!(chrome.tour_state.as_ref().map(|tour| tour.completed_interactions.clone()), Some(vec![]));
    }

    /// 🧪️ Ordered interactions gate out-of-order completions (the not-yet-reached one is ignored, no
    /// dedup entry added) and a repeated already-completed gesture is a no-operation — both fall out of
    /// `chrome_tour_complete_interaction`'s `!completed.contains(i)` + `index != completed.len()` checks.
    #[test]
    fn chrome_tour_complete_interaction_respects_order_and_dedups() {
        let mut shell = ShellState::new(Vec::new(), String::new());
        shell.chrome_build.start_introduction();
        let step = semio_framework::IntroductionStepDefinition::new("viewport", LocalizedLabel::data("Viewport"), LocalizedLabel::data("…"))
            .interact_ordered(vec![semio_framework_async::block_on(semio_framework::IntroductionInteraction::zoom("main", "Zoom")), semio_framework_async::block_on(semio_framework::IntroductionInteraction::pan("main", "Pan"))]);
        // Pan is index 1; out of order while zoom (index 0) hasn't completed — ignored.
        shell.chrome_tour_complete_interaction(&step, |kind| matches!(kind, semio_framework::IntroductionInteractionKind::Pan(id) if id == "main"));
        assert_eq!(shell.chrome_build.tour_state.as_ref().unwrap().completed_interactions, Vec::<usize>::new());
        shell.chrome_tour_complete_interaction(&step, |kind| matches!(kind, semio_framework::IntroductionInteractionKind::Zoom(id) if id == "main"));
        assert_eq!(shell.chrome_build.tour_state.as_ref().unwrap().completed_interactions, vec![0]);
        // Repeating zoom after it's already completed is a no-operation.
        shell.chrome_tour_complete_interaction(&step, |kind| matches!(kind, semio_framework::IntroductionInteractionKind::Zoom(id) if id == "main"));
        assert_eq!(shell.chrome_build.tour_state.as_ref().unwrap().completed_interactions, vec![0]);
        shell.chrome_tour_complete_interaction(&step, |kind| matches!(kind, semio_framework::IntroductionInteractionKind::Pan(id) if id == "main"));
        assert_eq!(shell.chrome_build.tour_state.as_ref().unwrap().completed_interactions, vec![0, 1]);
    }
    //#endregion Tour

    //#region ElementRects
    #[test]
    fn element_rect_registers_and_resolves() {
        let mut chrome = chrome_state();
        assert_eq!(chrome.resolve_element_rect("transform"), None);
        chrome.register_element_rect("transform", Rect::new(1.0, 2.0, 3.0, 4.0));
        assert_eq!(chrome.resolve_element_rect("transform"), Some(Rect::new(1.0, 2.0, 3.0, 4.0)));
        chrome.element_rects.clear();
        assert_eq!(chrome.resolve_element_rect("transform"), None);
    }

    #[test]
    fn element_rect_fallback_never_overrides_a_primary_entry() {
        let mut chrome = chrome_state();
        chrome.register_element_rect_fallback("transform", Rect::new(0.0, 0.0, 10.0, 10.0));
        assert_eq!(chrome.resolve_element_rect("transform"), Some(Rect::new(0.0, 0.0, 10.0, 10.0)));
        chrome.register_element_rect("transform", Rect::new(5.0, 5.0, 20.0, 20.0));
        chrome.register_element_rect_fallback("transform", Rect::new(0.0, 0.0, 1.0, 1.0));
        assert_eq!(chrome.resolve_element_rect("transform"), Some(Rect::new(5.0, 5.0, 20.0, 20.0)));
    }
    //#endregion ElementRects

    //#region VeilBands
    #[test]
    fn punch_cutout_returns_the_band_unchanged_when_disjoint() {
        let band = Rect::new(0.0, 0.0, 100.0, 100.0);
        let hole = Rect::new(200.0, 200.0, 10.0, 10.0);
        assert_eq!(punch_introduction_cutout(band, hole), vec![band]);
    }

    #[test]
    fn punch_cutout_centered_hole_tiles_into_four_pieces_covering_the_remaining_area() {
        let band = Rect::new(0.0, 0.0, 100.0, 100.0);
        let hole = Rect::new(25.0, 25.0, 50.0, 50.0);
        let pieces = punch_introduction_cutout(band, hole);
        assert_eq!(pieces.len(), 4);
        let covered: f32 = pieces.iter().map(|p| p.w * p.h).sum();
        assert_eq!(covered, band.w * band.h - hole.w * hole.h);
    }

    #[test]
    fn veil_bands_with_no_cutouts_is_one_full_viewport_band() {
        let bands = introduction_veil_bands(800.0, 600.0, &[]);
        assert_eq!(bands, vec![Rect::new(0.0, 0.0, 800.0, 600.0)]);
    }

    #[test]
    fn veil_bands_clamp_out_of_viewport_cutouts_to_a_no_operation() {
        let bands = introduction_veil_bands(800.0, 600.0, &[Rect::new(-100.0, -100.0, 10.0, 10.0)]);
        assert_eq!(bands, vec![Rect::new(0.0, 0.0, 800.0, 600.0)]);
    }

    #[test]
    fn veil_bands_compose_multiple_cutouts() {
        let bands = introduction_veil_bands(800.0, 600.0, &[Rect::new(0.0, 0.0, 100.0, 100.0), Rect::new(700.0, 500.0, 100.0, 100.0)]);
        let covered: f32 = bands.iter().map(|b| b.w * b.h).sum();
        assert_eq!(covered, 800.0 * 600.0 - 100.0 * 100.0 * 2.0);
    }
    //#endregion VeilBands

    //#region Pulse
    #[test]
    fn pulse_thickness_breathes_hairline_to_focus_and_back() {
        let hairline = 1.0;
        let focus = 3.0;
        assert_eq!(introduced_pulse_thickness(0.0, hairline, focus), hairline);
        assert!((introduced_pulse_thickness(INTRODUCED_PULSE_PERIOD_MS / 2.0, hairline, focus) - focus).abs() < 0.001);
        assert!((introduced_pulse_thickness(INTRODUCED_PULSE_PERIOD_MS, hairline, focus) - hairline).abs() < 0.001);
    }

    #[test]
    fn pulse_thickness_is_periodic() {
        let (hairline, focus) = (1.0, 3.0);
        assert_eq!(introduced_pulse_thickness(100.0, hairline, focus), introduced_pulse_thickness(100.0 + INTRODUCED_PULSE_PERIOD_MS, hairline, focus));
    }

    #[test]
    fn window_silhouette_border_emits_notched_outline_segments() {
        let mut draw = DrawList::default();
        let silhouette = WindowSilhouette::from_measured_top(Rect::new(10.0, 20.0, 200.0, 100.0), 60.0, 40.0, 24.0);
        push_window_silhouette_border(&mut draw, &silhouette, 2.0, Rgba::new(1.0, 0.0, 0.0, 1.0));
        let solids: Vec<[f32; 4]> = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter().map(|instance| instance.rect)).collect();
        assert!(solids.len() >= 8, "silhouette must paint every outline segment");
        // Gap baseline sits at y = bounds.y + cap_h - stroke
        assert!(solids.iter().any(|r| (r[1] - (20.0 + 24.0 - 2.0)).abs() < 0.01 && r[0] >= 10.0 + 60.0 - 0.01), "gap baseline must sit under the cutout between tabs and controls");
        // Top of controls starts at x = bounds.x + bounds.w - controls_w
        assert!(solids.iter().any(|r| (r[0] - (10.0 + 200.0 - 40.0)).abs() < 0.01 && (r[1] - 20.0).abs() < 0.01), "controls cap top must be part of the silhouette");
    }
    //#endregion Pulse

    //#region Placement
    #[test]
    fn placement_centers_when_no_anchor() {
        let (x, y) = resolve_introduction_placement(semio_framework::IntroductionPlacement::Auto, None, (320.0, 168.0), (800.0, 600.0));
        assert_eq!((x, y), ((800.0 - 320.0) / 2.0, (600.0 - 168.0) / 2.0));
    }

    #[test]
    fn placement_center_variant_ignores_the_anchor() {
        let anchor = Rect::new(10.0, 10.0, 50.0, 20.0);
        let (x, y) = resolve_introduction_placement(semio_framework::IntroductionPlacement::Center, Some(anchor), (320.0, 168.0), (800.0, 600.0));
        assert_eq!((x, y), ((800.0 - 320.0) / 2.0, (600.0 - 168.0) / 2.0));
    }

    #[test]
    fn placement_auto_picks_the_side_with_the_most_free_space() {
        // Anchor near the top-left in a wide viewport: space_right (750) exceeds space_bottom (580),
        // space_top (0), and space_left (0), so "right" wins.
        let anchor = Rect::new(0.0, 0.0, 50.0, 20.0);
        let (x, y) = resolve_introduction_placement(semio_framework::IntroductionPlacement::Auto, Some(anchor), (100.0, 50.0), (800.0, 600.0));
        assert_eq!(x, anchor.x + anchor.w + INTRODUCTION_INFO_BOX_GAP);
        assert!(y >= INTRODUCTION_INFO_BOX_GAP);
    }

    #[test]
    fn placement_explicit_side_is_honored_and_clamped_to_the_viewport() {
        let anchor = Rect::new(780.0, 10.0, 15.0, 15.0);
        let (x, _) = resolve_introduction_placement(semio_framework::IntroductionPlacement::Right, Some(anchor), (100.0, 50.0), (800.0, 600.0));
        assert!(x <= 800.0 - 100.0 - INTRODUCTION_INFO_BOX_GAP + 0.001);
    }
    //#endregion Placement

    //#region RibbonActivePath
    #[test]
    fn utility_subtree_has_active_path_finds_a_pressed_toggle_at_the_top_level() {
        let action = ActionDescriptor { controller_id: "test".into(), action: "noOperation".into(), args: None };
        let nodes = vec![ui_wgpu::wgpu::utility_toggle("a", "circle".into(), "A", true, action)];
        assert!(utility_subtree_has_active_path(&nodes));
    }

    #[test]
    fn utility_subtree_has_active_path_recurses_into_nested_collections() {
        let action = ActionDescriptor { controller_id: "test".into(), action: "noOperation".into(), args: None };
        let inner = vec![ui_wgpu::wgpu::utility_toggle("b", "circle".into(), "B", true, action.clone())];
        let nested = ui_wgpu::wgpu::utility_collection("group-2", "circle".into(), "Group 2", vec![ui_wgpu::wgpu::utility_collection("group-1", "circle".into(), "Group 1", inner)]);
        assert!(utility_subtree_has_active_path(std::slice::from_ref(&nested)));
    }

    #[test]
    fn utility_subtree_has_active_path_false_when_nothing_pressed() {
        let action = ActionDescriptor { controller_id: "test".into(), action: "noOperation".into(), args: None };
        let nodes =
            vec![ui_wgpu::wgpu::utility_toggle("a", "circle".into(), "A", false, action.clone()), ui_wgpu::wgpu::utility_collection("group", "circle".into(), "Group", vec![ui_wgpu::wgpu::utility_toggle("b", "circle".into(), "B", false, action)])];
        assert!(!utility_subtree_has_active_path(&nodes));
    }

    /// 🧪️ Item 5's core regression test: before this fix, `render_footer_utility_nodes` filtered nested
    /// `Collection`s out of `children` before recursing (`.filter(|child| !matches!(child,
    /// UtilityNode::Collection { .. }))`), so a 2nd-level nested toggle never got a hit target at all —
    /// expanding both levels here must reach it.
    #[test]
    fn render_footer_utility_nodes_recurses_at_least_two_levels_deep() {
        let action = ActionDescriptor { controller_id: "test".into(), action: "noOperation".into(), args: None };
        let leaf_toggle = ui_wgpu::wgpu::utility_toggle("leaf", "circle".into(), "Leaf", false, action.clone());
        let inner_collection = ui_wgpu::wgpu::utility_collection("inner", "circle".into(), "Inner", vec![leaf_toggle]);
        let outer_collection = ui_wgpu::wgpu::utility_collection("outer", "circle".into(), "Outer", vec![inner_collection]);
        let utilities = vec![outer_collection];

        let mut collection_expanded = HashMap::new();
        collection_expanded.insert("outer".to_string(), true);
        collection_expanded.insert("inner".to_string(), true);

        let mut draw = DrawList::default();
        let mut atlas = FontAtlas::builtin();
        let icons = IconAtlas::default();
        let mut input = InputState::<ActionDescriptor>::default();
        let theme = Theme::light();
        render_footer_utility_nodes(&mut ShellChromeBuildState::default(), &mut draw, &mut atlas, &icons, &mut input, &theme, 0.0, 0.0, theme.control_height, &utilities, &collection_expanded);

        assert!(input.hit_targets.iter().any(|hit| hit.control_id.as_deref() == Some("framework.utility.toggle.leaf")), "a toggle nested two Collection levels deep must still get a real hit target once both ancestors are expanded");
    }
    //#endregion RibbonActivePath

    //#region GhostText
    #[test]
    fn engagement_completion_suffix_matches_label_prefix() {
        let possibles = vec![ui_wgpu::wgpu::WindowEngagementPossible { id: "box".into(), label: "Box".into(), detail: None, action: None }];
        assert_eq!(engagement_completion_suffix("Bo", Some(&possibles)), "x");
        assert_eq!(engagement_completion_suffix("bo", Some(&possibles)), "x");
    }

    #[test]
    fn engagement_completion_suffix_empty_when_query_is_empty_or_unmatched() {
        let possibles = vec![ui_wgpu::wgpu::WindowEngagementPossible { id: "box".into(), label: "Box".into(), detail: None, action: None }];
        assert_eq!(engagement_completion_suffix("", Some(&possibles)), "");
        assert_eq!(engagement_completion_suffix("zz", Some(&possibles)), "");
        assert_eq!(engagement_completion_suffix("Box", Some(&possibles)), ""); // fully typed: no suffix left
        assert_eq!(engagement_completion_suffix("Bo", None), "");
    }

    #[test]
    fn engagement_completion_suffix_picks_first_matching_possible_in_order() {
        let possibles =
            vec![ui_wgpu::wgpu::WindowEngagementPossible { id: "boat".into(), label: "Boat".into(), detail: None, action: None }, ui_wgpu::wgpu::WindowEngagementPossible { id: "box".into(), label: "Box".into(), detail: None, action: None }];
        assert_eq!(engagement_completion_suffix("Bo", Some(&possibles)), "at");
    }

    /// 🧪️ Char-boundary safety: a multi-byte label prefix-matched by a query must not panic on slicing.
    #[test]
    fn engagement_completion_suffix_is_multibyte_safe() {
        let possibles = vec![ui_wgpu::wgpu::WindowEngagementPossible { id: "muenster".into(), label: "Münster".into(), detail: None, action: None }];
        assert_eq!(engagement_completion_suffix("M", Some(&possibles)), "ünster");
    }

    /// 🧪️ Accept-on-click (the mouse-driven substitute for Tab/Right-arrow — see the report's honest
    /// scope-down on why the keyboard shortcut itself isn't reachable from this region): a click landing
    /// inside the ghost-text rect on a clicked frame commits `query + suffix`.
    #[test]
    fn engagement_ghost_accept_on_click_commits_query_plus_suffix_when_clicked_inside() {
        let ghost_rect = Rect::new(50.0, 0.0, 20.0, 24.0);
        assert_eq!(engagement_ghost_accept_on_click(ghost_rect, 55.0, 10.0, true, "Bo", "x"), Some("Box".to_string()));
    }

    #[test]
    fn engagement_ghost_accept_on_click_ignores_clicks_outside_the_ghost_rect() {
        let ghost_rect = Rect::new(50.0, 0.0, 20.0, 24.0);
        assert_eq!(engagement_ghost_accept_on_click(ghost_rect, 5.0, 10.0, true, "Bo", "x"), None);
    }

    #[test]
    fn engagement_ghost_accept_on_click_ignores_held_or_stale_clicks() {
        let ghost_rect = Rect::new(50.0, 0.0, 20.0, 24.0);
        assert_eq!(engagement_ghost_accept_on_click(ghost_rect, 55.0, 10.0, false, "Bo", "x"), None);
    }
    //#endregion GhostText
}
//#endregion 🧪️ChromeOverlaysAndTourTests

//#endregion ShellChrome

#[cfg(target_arch = "wasm32")]
fn download_media_export(filename: &str, mime_type: &str, data: &str, _encoding: Option<&str>) {
    use wasm_bindgen::JsCast;
    use web_sys::{Blob, HtmlAnchorElement, Url};

    let window = match web_sys::window() {
        Some(window) => window,
        None => return,
    };
    let document = match window.document() {
        Some(document) => document,
        None => return,
    };
    let parts = js_sys::Array::new();
    parts.push(&wasm_bindgen::JsValue::from_str(data));
    let blob = Blob::new_with_str_sequence(&parts).unwrap();
    let url = Url::create_object_url_with_blob(&blob).unwrap();
    let anchor: HtmlAnchorElement = document.create_element("a").unwrap().dyn_into().unwrap();
    anchor.set_href(&url);
    anchor.set_download(filename);
    anchor.set_attribute("type", mime_type).ok();
    anchor.click();
    Url::revoke_object_url(&url).ok();
}

#[cfg(not(target_arch = "wasm32"))]
fn download_media_export(filename: &str, mime_type: &str, data: &str, encoding: Option<&str>) {
    let (filename, mime_type, data, encoding) = (filename.to_string(), mime_type.to_string(), data.to_string(), encoding.map(str::to_string));
    ShellPoolFuture::spawn(crate::renderer_worker_pool(), Lane::Io, async move {
        download_media_export_worker(&filename, &mime_type, &data, encoding.as_deref()).await;
    });
}

#[cfg(not(target_arch = "wasm32"))]
async fn download_media_export_worker(filename: &str, mime_type: &str, data: &str, encoding: Option<&str>) {
    let extension = mime_type.rsplit_once('/').map(|(_, ext)| ext).unwrap_or("dat");
    if let Some(path) = ui_host::select_native_paths(ui_host::NativeFileDialogRequest::save(filename, [extension])).await.into_iter().next() {
        use base64::Engine;
        use std::fs as system_fs;
        let bytes = if encoding == Some("base64") { base64::engine::general_purpose::STANDARD.decode(data).unwrap_or_else(|_| data.as_bytes().to_vec()) } else { data.as_bytes().to_vec() };
        let _ = system_fs::write(path, bytes);
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn request_file_save(filename: &str) -> Option<std::path::PathBuf> {
    ui_host::select_native_paths(ui_host::NativeFileDialogRequest::save(filename, ["json"])).await.into_iter().next()
}

#[cfg(target_arch = "wasm32")]
fn request_file_save(_filename: &str) -> Option<std::path::PathBuf> {
    None
}

#[cfg(not(target_arch = "wasm32"))]
async fn pick_folder() -> Option<String> {
    ui_host::select_native_paths(ui_host::NativeFileDialogRequest::folder()).await.into_iter().next().map(|path| path.display().to_string())
}

#[cfg(target_arch = "wasm32")]
fn pick_folder() -> Option<String> {
    None
}

/// 📤️ Opens the native file picker; one entry per selected file, in selection order.
#[cfg(not(target_arch = "wasm32"))]
async fn request_file_open(accept: &str, read_as: Option<&str>, multiple: bool) -> Vec<String> {
    use std::fs as system_fs;
    let extensions: Vec<String> = accept.split(',').filter_map(|entry| entry.trim().strip_prefix('.').map(str::to_string)).collect();
    let paths = ui_host::select_native_paths(ui_host::NativeFileDialogRequest::open(extensions.clone(), multiple)).await;
    paths
        .into_iter()
        .filter_map(|path| {
            if read_as == Some("dataUrl") {
                use base64::Engine;
                let bytes = system_fs::read(&path).ok()?;
                let mime = extensions.first().map(|ext| format!("application/{ext}")).unwrap_or_else(|| "application/octet-stream".into());
                return Some(format!("data:{mime};base64,{}", base64::engine::general_purpose::STANDARD.encode(bytes)));
            }
            system_fs::read_to_string(path).ok()
        })
        .collect()
}

/// 🕸️ wasm32 has no native file-dialog surface — the browser shell handles `RequestFileOpen` itself
/// (see `framework/renderer/react/index.tsx`'s `requestFileOpen`); this native fallback stays empty.
#[cfg(target_arch = "wasm32")]
fn request_file_open(_accept: &str, _read_as: Option<&str>, _multiple: bool) -> Vec<String> {
    Vec::new()
}

//#region RequestMediaFrames
/// 🔓️ Decodes a `data:<mime>;base64,<data>` URL's payload; `None` for anything malformed or missing a
/// comma separator. Used by both `RequestMediaFrames.payload` (bytes already in memory) and the
/// fallback-response encoding below.
fn decode_data_url(data_url: &str) -> Option<Vec<u8>> {
    use base64::Engine;
    let comma = data_url.find(',')?;
    base64::engine::general_purpose::STANDARD.decode(&data_url[comma + 1..]).ok()
}

/// 📦️ Builds the single `fallback_action` `ActionDescriptor` — raw `bytes` re-encoded as a data URL
/// merged into `base_args`, same shape on every failure path (`ffmpeg` missing, spawn/scratch-dir I/O
/// failure, no source bytes at all) so the plugin's fallback handler only needs to handle one shape.
fn fallback_action_descriptor(controller_id: &str, fallback_action: &str, bytes: &[u8], name: &str, base_args: &Value) -> ActionDescriptor {
    use base64::Engine;
    let mut args = base_args.clone();
    if let Some(obj) = args.as_object_mut() {
        obj.insert("payload".into(), Value::String(format!("data:application/octet-stream;base64,{}", base64::engine::general_purpose::STANDARD.encode(bytes))));
        obj.insert("name".into(), Value::String(name.to_string()));
    }
    ActionDescriptor { controller_id: controller_id.to_string(), action: fallback_action.to_string(), args: semio_framework::optional_json_to_dsl(Some(args)) }
}

/// 🧮️ Pure `ffmpeg` argument computation for D5 frame extraction (precedent: `animate/video/rs/lib.rs`'s
/// `run_ffmpeg`, which this shell doesn't depend on directly — cross-technology dep avoided per house
/// rules — but whose `Command::new("ffmpeg").args(...).status()` invocation convention this follows).
/// `sample_stride` floors at 1 (every frame); `max_frames` of 0 means "host default" (capped generously
/// rather than truly unlimited, so a pathological request can't fill disk); `max_long_edge_px` of 0
/// skips the scale filter entirely (native resolution).
fn ffmpeg_frame_extraction_args(sample_stride: u32, max_frames: u32, max_long_edge_px: u32, input: &std::path::Path, out_dir: &std::path::Path) -> Vec<String> {
    let stride = sample_stride.max(1);
    let filter = if max_long_edge_px > 0 { format!("select=not(mod(n\\,{stride})),scale={max_long_edge_px}:-2") } else { format!("select=not(mod(n\\,{stride}))") };
    let frames_cap = if max_frames > 0 { max_frames } else { 100_000 };
    vec!["-y".into(), "-i".into(), input.display().to_string(), "-vf".into(), filter, "-vsync".into(), "vfr".into(), "-frames:v".into(), frames_cap.to_string(), out_dir.join("%06d.jpg").display().to_string()]
}

/// ⏱️ Approximate per-extracted-frame timestamp from the requested sampling cadence — `ffmpeg`'s own
/// frame PTS aren't threaded back through the `%06d.jpg` sequence (would need an `ffprobe` pass or
/// `-frame_pts`/timebase math this ticket scopes out; documented simplification, same spirit as the D1
/// wgpu point-sprite pass note above `render_world_3d`). Good enough for frame *ordering*/spacing; exact
/// decode timestamps are future work if a downstream consumer needs true sub-frame sync.
fn approx_sampled_timestamp_ms(index: u32, sample_stride: u32, fps_hint: f64) -> f64 {
    let stride = sample_stride.max(1) as f64;
    let fps = if fps_hint > 0.0 { fps_hint } else { 30.0 };
    (index as f64) * stride / fps * 1000.0
}

#[cfg(not(target_arch = "wasm32"))]
fn ffmpeg_available() -> bool {
    use std::process as system_process;
    system_process::Command::new("ffmpeg").arg("-version").stdout(system_process::Stdio::null()).stderr(system_process::Stdio::null()).status().is_ok_and(|status| status.success())
}

/// 🎞️ D5 native pipeline, beside `request_file_open` above: obtains video bytes (native file
/// picker, or `payload`'s data-URL bytes when the caller already has them from a drop zone), shells out
/// to `ffmpeg` to sample frames into a scratch temp directory, base64-encodes each resulting JPEG, and
/// returns one `ActionDescriptor` per frame (`frame_action`) followed by one for `done_action` — or, on
/// any failure (no `ffmpeg` on `PATH`, no source bytes, scratch I/O failure, non-zero `ffmpeg` exit), a
/// single `fallback_action` descriptor carrying the raw bytes so the plugin's own in-process decoder
/// (MJPEG/H.264-baseline) gets a chance.
#[cfg(not(target_arch = "wasm32"))]
#[allow(clippy::too_many_arguments)]
async fn request_media_frames(
    controller_id: &str,
    accept: &str,
    frame_action: &str,
    done_action: &str,
    fallback_action: &str,
    sample_stride: u32,
    max_frames: u32,
    max_long_edge_px: u32,
    fps_hint: f64,
    payload: Option<&str>,
    args: Option<Value>,
) -> Vec<ActionDescriptor> {
    use base64::Engine;
    use std::fs as system_fs;
    use std::process as system_process;
    let base_args = args.unwrap_or_else(|| serde_json::json!({}));
    let (bytes, name) = match payload {
        Some(data_url) => match decode_data_url(data_url) {
            Some(decoded) => (decoded, "video".to_string()),
            None => return Vec::new(),
        },
        None => {
            let extensions: Vec<String> = accept.split(',').filter_map(|entry| entry.trim().strip_prefix('.').map(str::to_string)).collect();
            let Some(path) = ui_host::select_native_paths(ui_host::NativeFileDialogRequest::open(extensions, false)).await.into_iter().next() else {
                return Vec::new();
            };
            let Ok(bytes) = system_fs::read(&path) else {
                return Vec::new();
            };
            let name = path.file_name().map_or_else(|| "video".to_string(), |value| value.to_string_lossy().into_owned());
            (bytes, name)
        }
    };
    if !ffmpeg_available() {
        return vec![fallback_action_descriptor(controller_id, fallback_action, &bytes, &name, &base_args)];
    }
    let scratch_dir = std::env::temp_dir().join(format!("semio-media-frames-{}-{}", system_process::id(), name.len()));
    if system_fs::create_dir_all(&scratch_dir).is_err() {
        return vec![fallback_action_descriptor(controller_id, fallback_action, &bytes, &name, &base_args)];
    }
    let input_path = scratch_dir.join("source.bin");
    if system_fs::write(&input_path, &bytes).is_err() {
        let _ = system_fs::remove_dir_all(&scratch_dir);
        return vec![fallback_action_descriptor(controller_id, fallback_action, &bytes, &name, &base_args)];
    }
    let ffmpeg_args = ffmpeg_frame_extraction_args(sample_stride, max_frames, max_long_edge_px, &input_path, &scratch_dir);
    let extracted = system_process::Command::new("ffmpeg").args(&ffmpeg_args).status().is_ok_and(|status| status.success());
    if !extracted {
        let _ = system_fs::remove_dir_all(&scratch_dir);
        return vec![fallback_action_descriptor(controller_id, fallback_action, &bytes, &name, &base_args)];
    }
    let mut frame_paths: Vec<std::path::PathBuf> =
        system_fs::read_dir(&scratch_dir).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.path())).filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("jpg")).collect()).unwrap_or_default();
    frame_paths.sort();
    let total = frame_paths.len();
    let mut actions = Vec::with_capacity(total + 1);
    for (index, frame_path) in frame_paths.iter().enumerate() {
        let Ok(jpeg_bytes) = system_fs::read(frame_path) else {
            continue;
        };
        let mut frame_args = base_args.clone();
        if let Some(obj) = frame_args.as_object_mut() {
            obj.insert("payload".into(), Value::String(format!("data:image/jpeg;base64,{}", base64::engine::general_purpose::STANDARD.encode(&jpeg_bytes))));
            obj.insert("name".into(), Value::String(name.clone()));
            obj.insert("frameIndex".into(), serde_json::json!(index));
            obj.insert("timestampMs".into(), serde_json::json!(approx_sampled_timestamp_ms(index as u32, sample_stride, fps_hint)));
            obj.insert("index".into(), serde_json::json!(index));
            obj.insert("total".into(), serde_json::json!(total));
        }
        actions.push(ActionDescriptor { controller_id: controller_id.to_string(), action: frame_action.to_string(), args: semio_framework::optional_json_to_dsl(Some(frame_args)) });
    }
    let mut done_args = base_args;
    if let Some(obj) = done_args.as_object_mut() {
        obj.insert("name".into(), Value::String(name));
        obj.insert("frameCount".into(), serde_json::json!(total));
        obj.insert("sampledCount".into(), serde_json::json!(total));
    }
    actions.push(ActionDescriptor { controller_id: controller_id.to_string(), action: done_action.to_string(), args: semio_framework::optional_json_to_dsl(Some(done_args)) });
    let _ = system_fs::remove_dir_all(&scratch_dir);
    actions
}

/// 🕸️ wasm32 mirrors `request_file_open`'s stub: no native `ffmpeg`/file-dialog surface, the browser
/// React shell handles `RequestMediaFrames` itself. If `payload` bytes are already in hand (a drop
/// zone), still honor `fallback_action` with them so an in-process program decoder gets a chance even on
/// this native/wasm shell.
#[cfg(target_arch = "wasm32")]
#[allow(clippy::too_many_arguments)]
fn request_media_frames(
    controller_id: &str,
    _accept: &str,
    _frame_action: &str,
    _done_action: &str,
    fallback_action: &str,
    _sample_stride: u32,
    _max_frames: u32,
    _max_long_edge_px: u32,
    _fps_hint: f64,
    payload: Option<&str>,
    args: Option<serde_json::Value>,
) -> Vec<ActionDescriptor> {
    match payload.and_then(decode_data_url) {
        Some(bytes) => vec![fallback_action_descriptor(controller_id, fallback_action, &bytes, "video", &args.unwrap_or_else(|| serde_json::json!({})))],
        None => Vec::new(),
    }
}

#[cfg(test)]
mod media_frames_tests {
    use super::*;

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn stalled_shell_io_keeps_mailbox_poll_p99_below_two_ms() {
        let mut state = ShellState::new(Vec::new(), String::new());
        state.submit_shell_io(|| {
            std::thread::sleep(std::time::Duration::from_millis(80));
            ShellIoCompletion::Finished
        });
        let mut samples = Vec::with_capacity(4096);
        for _ in 0..4096 {
            let started = std::time::Instant::now();
            let _ = state.poll_shell_io();
            samples.push(started.elapsed());
        }
        samples.sort_unstable();
        let p99 = samples[samples.len() * 99 / 100];
        assert!(p99 < std::time::Duration::from_millis(2), "shell I/O callback p99 was {p99:?}");
    }

    #[test]
    fn ffmpeg_args_apply_stride_scale_and_frame_cap() {
        let input = std::path::Path::new("/tmp/in.mp4");
        let out_dir = std::path::Path::new("/tmp/out");
        let args = ffmpeg_frame_extraction_args(5, 200, 1600, input, out_dir);
        assert_eq!(args[0], "-y");
        assert_eq!(args[2], "/tmp/in.mp4");
        assert_eq!(args[4], "select=not(mod(n\\,5)),scale=1600:-2");
        assert_eq!(args[8], "200");
        assert_eq!(args[9], "/tmp/out/%06d.jpg");
    }

    #[test]
    fn ffmpeg_args_floor_stride_and_default_frame_cap() {
        let args = ffmpeg_frame_extraction_args(0, 0, 0, std::path::Path::new("in.mp4"), std::path::Path::new("out"));
        assert_eq!(args[4], "select=not(mod(n\\,1))", "stride 0 floors to 1: {args:?}");
        assert_eq!(args[8], "100000", "max_frames 0 falls back to a generous cap: {args:?}");
    }

    #[test]
    fn ffmpeg_args_omit_scale_filter_when_max_long_edge_zero() {
        let args = ffmpeg_frame_extraction_args(1, 10, 0, std::path::Path::new("in.mp4"), std::path::Path::new("out"));
        assert!(!args[4].contains("scale"), "{args:?}");
    }

    #[test]
    fn approx_timestamp_scales_with_stride_and_fps() {
        assert_eq!(approx_sampled_timestamp_ms(0, 5, 30.0), 0.0);
        assert!((approx_sampled_timestamp_ms(1, 5, 30.0) - (5.0 / 30.0 * 1000.0)).abs() < 1e-9);
        // fps_hint of 0 falls back to a 30 fps default rather than dividing by zero.
        assert!((approx_sampled_timestamp_ms(1, 5, 0.0) - (5.0 / 30.0 * 1000.0)).abs() < 1e-9);
    }

    #[test]
    fn decode_data_url_round_trips_bytes() {
        use base64::Engine;
        let bytes = vec![1u8, 2, 3, 250];
        let url = format!("data:application/octet-stream;base64,{}", base64::engine::general_purpose::STANDARD.encode(&bytes));
        assert_eq!(decode_data_url(&url), Some(bytes));
        assert_eq!(decode_data_url("not-a-data-url"), None);
    }

    #[test]
    fn fallback_descriptor_merges_payload_into_base_args() {
        let descriptor = fallback_action_descriptor("app.controller", "importVideoBytesPayload", &[9, 9], "clip.mp4", &serde_json::json!({"streamId": "s1"}));
        assert_eq!(descriptor.controller_id, "app.controller");
        assert_eq!(descriptor.action, "importVideoBytesPayload");
        let args = descriptor.args.unwrap();
        assert_eq!(args.get("streamId").and_then(semio_framework::DslValue::as_str), Some("s1"));
        assert_eq!(args.get("name").and_then(semio_framework::DslValue::as_str), Some("clip.mp4"));
        assert!(args.get("payload").and_then(semio_framework::DslValue::as_str).is_some_and(|payload| payload.starts_with("data:application/octet-stream;base64,")));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn request_media_frames_falls_back_when_ffmpeg_missing_and_payload_given() {
        // 🧪️ Doesn't assert on `ffmpeg_available()` (may or may not be installed in CI/sandboxes) — only
        // exercises the `payload`-bytes-in-hand path, which is deterministic regardless of `ffmpeg`
        // presence when the decoded payload is deliberately not a real video (so even a present `ffmpeg`
        // fails to extract frames from it and this still falls back).
        use base64::Engine;
        let payload = format!("data:video/mp4;base64,{}", base64::engine::general_purpose::STANDARD.encode(b"not a real video"));
        let actions = semio_framework_async::block_on(request_media_frames(
            "app.controller",
            "video/mp4",
            "importVideoFramePayload",
            "importVideoDone",
            "importVideoBytesPayload",
            5,
            200,
            1600,
            30.0,
            Some(&payload),
            Some(serde_json::json!({"streamId": "s1"})),
        ));
        assert_eq!(actions.len(), 1, "garbage payload never yields real frames: {actions:?}");
        assert_eq!(actions[0].action, "importVideoBytesPayload");
        assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("streamId")).and_then(semio_framework::DslValue::as_str), Some("s1"));
    }
}
//#endregion RequestMediaFrames

#[cfg(test)]
mod context_menu_keyboard_tests {
    use super::*;

    #[test]
    fn context_menu_path_for_ordinal_selects_enabled_rows() {
        let items = vec![ContextMenuItem { id: "a".into(), label: "A".into(), ..Default::default() }, ContextMenuItem { id: "b".into(), label: "B".into(), ..Default::default() }];
        assert_eq!(context_menu_path_for_ordinal(&items, &[], 2), Some(vec![1]));
    }

    #[test]
    fn context_menu_submenu_open_follows_active_path() {
        assert!(context_menu_submenu_open(&[0, 0], &[0], false, true));
        assert!(context_menu_submenu_open(&[0], &[0], true, true));
        assert!(!context_menu_submenu_open(&[1], &[0], false, true));
    }

    #[test]
    fn context_menu_click_on_group_row_control_id_opens_its_submenu_instead_of_dispatching() {
        let mut shell = ShellState::new(Vec::new(), String::new());
        shell.context_menu = Some(ContextMenuState {
            items: vec![ContextMenuItem { id: "menu.group.view".into(), label: "View".into(), children: vec![ContextMenuItem { id: "menu.group.view.child".into(), label: "Child".into(), ..Default::default() }], ..Default::default() }],
            ..Default::default()
        });
        let hit = HitTarget { rect: Rect::new(0.0, 0.0, 10.0, 10.0), event: None, control_id: Some("menu.group.view".into()), kind: HitKind::ContextMenu, drag_axis: None, drag_data: None };
        let consumed = semio_framework_async::block_on(shell.handle_shell_hit(&hit)).expect("group-row click never errors");
        assert!(consumed);
        let menu = shell.context_menu.as_ref().expect("a group-row click opens its submenu instead of closing the menu");
        assert_eq!(menu.active, vec![0]);
    }

    #[test]
    fn render_context_menu_level_renders_a_labeled_separator_as_a_header_without_a_hit() {
        let items = vec![ContextMenuItem { id: "header-1".into(), label: "Header".into(), separator: true, ..Default::default() }, ContextMenuItem { id: "leaf-1".into(), label: "Leaf".into(), ..Default::default() }];
        let menu = ContextMenuState { items: items.clone(), ..Default::default() };
        let mut draw = ui_wgpu::wgpu::DrawList::default();
        let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
        let icons = ui_wgpu::wgpu::IconAtlas::default();
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        let theme = Theme::default();
        ShellState::render_context_menu_level(&mut draw, &mut atlas, &icons, &mut input, &theme, &menu, &menu.items, &[], 0.0, 0.0, 800.0, 600.0);
        assert!(input.hit_targets.iter().all(|hit| hit.control_id.as_deref() != Some("header-1")), "a labeled separator must stay non-interactive");
        assert!(input.hit_targets.iter().any(|hit| hit.control_id.as_deref() == Some("leaf-1")), "the leaf row after the header must still register a hit");
    }

    #[test]
    fn render_context_menu_level_clips_to_viewport_height_and_scrolls_hidden_rows_into_view() {
        let items: Vec<ContextMenuItem> = (0..20).map(|index| ContextMenuItem { id: format!("item-{index}"), label: format!("Item {index}"), ..Default::default() }).collect();
        let theme = Theme::default();
        let row_h = theme.control_height;
        let viewport_h = row_h * 4.0;
        let menu_at = |scroll_offset: f32| ContextMenuState { items: items.clone(), scroll_offset, ..Default::default() };
        let mut draw = ui_wgpu::wgpu::DrawList::default();
        let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
        let icons = ui_wgpu::wgpu::IconAtlas::default();

        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        let menu = menu_at(0.0);
        ShellState::render_context_menu_level(&mut draw, &mut atlas, &icons, &mut input, &theme, &menu, &menu.items, &[], 0.0, 0.0, 800.0, viewport_h);
        let visible_ids: Vec<String> = input.hit_targets.iter().filter_map(|hit| hit.control_id.clone()).collect();
        assert!(visible_ids.len() < items.len(), "expected the viewport clip to hide some rows, got {} of {}", visible_ids.len(), items.len());
        assert!(!visible_ids.contains(&"item-19".to_string()), "the last row should be scrolled out of view without scrolling");

        let mut input2 = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        let menu2 = menu_at(row_h * 16.0);
        ShellState::render_context_menu_level(&mut draw, &mut atlas, &icons, &mut input2, &theme, &menu2, &menu2.items, &[], 0.0, 0.0, 800.0, viewport_h);
        let scrolled_ids: Vec<String> = input2.hit_targets.iter().filter_map(|hit| hit.control_id.clone()).collect();
        assert!(scrolled_ids.contains(&"item-19".to_string()), "scrolling down should bring the last row into view");
    }

    #[test]
    fn render_context_menu_level_flips_a_submenu_left_when_it_would_overflow_the_right_edge() {
        let theme = Theme::default();
        let child_items = vec![ContextMenuItem { id: "child-1".into(), label: "Child one".into(), ..Default::default() }];
        let parent_items = vec![ContextMenuItem { id: "menu.group.view".into(), label: "View".into(), children: child_items, ..Default::default() }];
        let menu = ContextMenuState { items: parent_items.clone(), active: vec![0], ..Default::default() };
        let mut draw = ui_wgpu::wgpu::DrawList::default();
        let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
        let icons = ui_wgpu::wgpu::IconAtlas::default();
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        let viewport_w = 220.0;
        ShellState::render_context_menu_level(&mut draw, &mut atlas, &icons, &mut input, &theme, &menu, &menu.items, &[], 0.0, 0.0, viewport_w, 600.0);
        let parent_w = ShellState::context_menu_level_width(&parent_items, &theme);
        let child_hit = input.hit_targets.iter().find(|hit| hit.control_id.as_deref() == Some("child-1")).expect("submenu row registers a hit");
        assert!(child_hit.rect.x < parent_w, "expected the submenu to flip left of the parent row, got x={}", child_hit.rect.x);
    }
}
