//! 🖥️ framework/products/os/modules/renderer/engine/elements/🐚️Shell/component.rs — wgpu shell
//! chrome implementation for the Shell element, extracted from lib.rs's inline
//! `pub mod shell { ... }` body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired via
//! `#[path = "../../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"] pub mod shell;` in lib.rs in place of
//! the former inline block; the module name `shell` is unchanged, so every existing
//! `crate::shell::...` call site elsewhere in the crate keeps resolving with zero other changes.
//! 🖥️ OS shell chrome — navbar, footer, floating panels, overlays, and studio mode.

#[cfg(test)]
use crate::dock::{push_window_silhouette_border, DockDropZone, DockStackTab};
#[cfg(all(test, not(target_arch = "wasm32")))]
use semio_framework_os_kernel::os_directory::{client::DirectoryTransport, directory_command_sha256, DirectoryCommandOutcomeV1};
#[cfg(test)]
use ui_wgpu::wgpu::{push_chrome_group_border, Label, UiButtonNode, UiNode, UiPresence, UiSelectItem, UiSelectNode, UiStackNode, UiTextNode};

use crate::dock::{compute_dock_drop_zone, parse_path, DockDragKind, DockDragPayload, DockDragState, DockState, WindowSilhouette};
use crate::interpreter::{begin_ui_document_opportunity, framework_widget_context, render_ui_document_step, UiDocumentFrameCursor};
use crate::program_bridge::{is_space_mode, resolve_playground_app_id, resolve_plugin_host_config, PluginHostConfig, ProgramBridgeEntry};
use crate::scenes::{toggle_vfs_row_expanded, vfs_selection_for_click, AdmittedSurfaceMap, Board2dSurface, NodeGraphSurface, TiledMapSurface};
use infinite_world::world::{enqueue_world3d_events, World3dState, WorldInteractionIntent, WorldInteractionPhase};
#[cfg(test)]
use ui_wgpu::wgpu::draw_text;
use semio_framework::{AppDefinition, PanelGroup, PanelTabDefinition, ViewModel};
use semio_framework_os_config::opening_config::{
    apply_ui_preferences_config_mutation, decode_ui_preferences_config_mutation_json,
    mutations::{set_appearance, set_custom_driver, set_custom_theme, set_driver, set_keybinding_override, set_layout, set_locale, set_terminology, set_theme, UiPreferencesConfigMutation},
    UiAppearance as OsUiAppearance, UiChromeLayout as OsUiChromeLayout, UiDriver as OsUiDriver, UiLocale as OsUiLocale, UiPreferences, UiTheme as OsUiTheme, UI_PREFERENCES_CONFIG_SCHEMA,
};
#[cfg(test)]
use semio_framework::IconName;
use semio_framework_os_kernel::os_directory::identity::IdentityEnv;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
#[cfg(not(target_arch = "wasm32"))]
use store_sync::sync::{ArtifactActorConfig, ArtifactActorMsg, ArtifactDocumentKey, ArtifactEvent, ArtifactHost, ArtifactMailboxSender, ArtifactSyncStatus, PersistenceBinding, RemoteState};
#[cfg(not(target_arch = "wasm32"))]
use store_sync::PresencePeer;
use ui_contract::{SurfaceId, UiDocumentLease, UiText, UI_DOCUMENT_LEASE_ALIASES, UI_DOCUMENT_LEASE_SLOTS};
#[cfg(test)]
use ui_contract::UiFixedList;
// 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C0/§C3/§C6 (lane 2-D) —
// lane 1-D's Rust directory client + native identity mint/restore helper, consumed as-is (never
// re-declared: `semio_framework_os_kernel::os_directory` is the single source of truth both this
// shell and the hub's own Rust twin import). Native-only: the browser wgpu build has no native
// `ArtifactHost`/document-sync path either (see `attach_sync_backbone`'s wasm32 branch), so identity/
// directory wiring stays inside the same `not(wasm32)` boundary as every other native-only field on
// `ShellState` (`document_host`, `sync_channel`, `sync_status`).
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_kernel::os_directory::{
    client::{
        native::NativeDirectoryTransport, CanonicalDirectoryEventPageV1, DirectoryBootstrapTransition, DirectoryClient, DirectoryClientError, DirectoryEventPageAckV1, DirectoryEventPageBootstrapV1, DirectoryStream, DirectoryStreamTurn, DirectoryWsConnection, TransportError,
    },
    identity::{actor_id, claimed_local_hub_credential, restore_claimed, Identity, IdentityOutcome, IdentityStatus},
    mint_directory_command_request_id,
    schema::{
        reduce_gis_map_inference_port_v1, DocumentExecutionTargetLeaseFieldsV1, DocumentScope, GisMapInferenceApprovalRequestV1, GisMapInferenceJobRequestV1, GisMapInferencePortCodeV1, GisMapInferencePortEventV1, GisMapInferencePortPhaseV1,
        GisMapInferencePortStatusV1, GIS_MAP_INFERENCE_SERVICE_ID,
    },
    DirectoryCommand, DirectoryCommandErrorCodeV1, DirectoryCommandReceiptV1, DirectoryCommandRequestV1, DirectoryCommandResultV1, DirectorySpaceAdministrationCapabilitiesV1, DirectorySpaceAdministrationInviteRowV1,
    DirectorySpaceAdministrationMemberRowV1, DirectorySpaceAdministrationPageV1, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility, DirectoryStreamMessage,
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
    chrome_item_bg, chrome_item_text, paint_retained_glyph_step, DragAxis, DrawList, FontAtlas, HitKind, HitTarget, IconAtlas, InputState, Level, PointerModifiers, Rect, RetainedGlyphCursor, RetainedGlyphStep,
    Rgba, Theme, TreeDragState, TreeDropPosition, WidgetInteractionMaps, WindowStackCorner,
};
use ui_wgpu::wgpu::{
    ActionDescriptor, Locale, LocalizedLabel, Terminology, UtilityCategory, UtilityNode, WindowEngagement,
    WindowMeasure, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_HISTORY_ID, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
};

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

const SHELL_FIND_ITEM_CAPACITY: usize = 256;
const SHELL_FIND_PAYLOAD_BYTES: usize = 1024 * 1024;

/// 🧺️ Fixed FIFO ownership for one generation of scene find items.
pub struct ShellFindItems {
    slots: Box<[Option<ShellFindItem>; SHELL_FIND_ITEM_CAPACITY]>,
    head: usize,
    len: usize,
    payload_bytes: usize,
    generation: u64,
}

impl Default for ShellFindItems {
    fn default() -> Self {
        Self { slots: Box::new([const { None }; SHELL_FIND_ITEM_CAPACITY]), head: 0, len: 0, payload_bytes: 0, generation: 1 }
    }
}

impl ShellFindItems {
    fn item_bytes(item: &ShellFindItem) -> Option<usize> {
        item.id
            .capacity()
            .checked_add(item.label.capacity())?
            .checked_add(item.description.as_ref().map_or(0, String::capacity))?
            .checked_add(item.category.as_ref().map_or(0, String::capacity))?
            .checked_add(item.surface_id.capacity())?
            .checked_add(item.node_id.capacity())
    }

    fn try_push_at(&mut self, generation: u64, item: ShellFindItem) -> Result<(), ShellFindItem> {
        let Some(bytes) = Self::item_bytes(&item) else { return Err(item) };
        let Some(next_bytes) = self.payload_bytes.checked_add(bytes) else { return Err(item) };
        if generation != self.generation || self.len == self.slots.len() || next_bytes > SHELL_FIND_PAYLOAD_BYTES {
            return Err(item);
        }
        let Some(index) = self.head.checked_add(self.len).map(|index| index % self.slots.len()) else { return Err(item) };
        let Some(next_len) = self.len.checked_add(1) else { return Err(item) };
        if self.slots[index].is_some() {
            return Err(item);
        }
        self.slots[index] = Some(item);
        self.len = next_len;
        self.payload_bytes = next_bytes;
        Ok(())
    }

    fn try_push(&mut self, item: ShellFindItem) -> Result<(), ShellFindItem> {
        self.try_push_at(self.generation, item)
    }

    fn pop_front(&mut self) -> Option<ShellFindItem> {
        if self.len == 0 {
            return None;
        }
        let item = self.slots[self.head].take()?;
        let bytes = Self::item_bytes(&item)?;
        self.head = self.head.checked_add(1).map(|index| index % self.slots.len())?;
        self.len = self.len.checked_sub(1)?;
        self.payload_bytes = self.payload_bytes.checked_sub(bytes)?;
        Some(item)
    }

    fn begin_next_generation(&mut self) -> bool {
        if self.len != 0 {
            return false;
        }
        let Some(generation) = self.generation.checked_add(1) else { return false };
        self.head = 0;
        self.generation = generation;
        true
    }

    fn iter(&self) -> impl Iterator<Item = &ShellFindItem> {
        (0..self.len).filter_map(move |offset| self.head.checked_add(offset).map(|index| index % self.slots.len()).and_then(|index| self.slots[index].as_ref()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.payload_bytes == 0 && self.slots.iter().all(Option::is_none)
    }

    fn close_step(&mut self) -> bool {
        if self.pop_front().is_some() {
            return false;
        }
        self.terminal_is_empty()
    }
}

#[derive(Clone, Copy)]
struct ActiveShellFindItems {
    pointer: std::ptr::NonNull<ShellFindItems>,
    generation: u64,
}

thread_local! {
    static ACTIVE_FIND_ITEMS: std::cell::Cell<Option<ActiveShellFindItems>> = const { std::cell::Cell::new(None) };
}

/// 🪢️ Non-nesting single-worker binding for the exact retained collector.
struct ShellFindItemBinding {
    active: ActiveShellFindItems,
}

impl ShellFindItems {
    fn bind(&mut self) -> Result<ShellFindItemBinding, ()> {
        let active = ActiveShellFindItems { pointer: std::ptr::NonNull::from(&mut *self), generation: self.generation };
        ACTIVE_FIND_ITEMS.with(|slot| {
            if slot.get().is_some() {
                return Err(());
            }
            slot.set(Some(active));
            Ok(ShellFindItemBinding { active })
        })
    }
}

impl Drop for ShellFindItemBinding {
    fn drop(&mut self) {
        ACTIVE_FIND_ITEMS.with(|slot| {
            if slot.get().is_some_and(|active| active.pointer == self.active.pointer && active.generation == self.active.generation) {
                slot.set(None);
            }
        });
    }
}

/// 📥️ Transfers one exact callback item or returns it unchanged on stale/full/unbound refusal.
pub fn try_push_find_item(item: ShellFindItem) -> Result<(), ShellFindItem> {
    ACTIVE_FIND_ITEMS.with(|slot| {
        let Some(active) = slot.get() else { return Err(item) };
        let collector = unsafe { active.pointer.as_ptr().as_mut() };
        let Some(collector) = collector else { return Err(item) };
        collector.try_push_at(active.generation, item)
    })
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

fn scope_context_menu_items(items: &mut [ContextMenuItem], window_id: &str) {
    for item in items {
        if let Some(action) = item.action.as_mut() {
            let mut args = match action.args.take() { Some(DslValue::Object(entries)) => entries, _ => Vec::new() };
            args.retain(|(key, _)| key != "windowId");
            args.push(("windowId".into(), DslValue::String(window_id.into())));
            action.args = Some(DslValue::Object(args));
        }
        scope_context_menu_items(&mut item.children, window_id);
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

#[cfg(target_arch = "wasm32")]
thread_local! {
    /// 🌐️ Browser boot environment supplied by `semioWgpuSetHubEnv` from the TypeScript host.
    static BOOT_HUB_ENV: std::cell::RefCell<Option<(String, String, Option<String>)>> = std::cell::RefCell::new(None);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = semioWgpuSetHubEnv)]
pub fn semio_wgpu_set_hub_env(hub_url: String, user: String, data_dir: String) {
    let data_dir = if data_dir.is_empty() { None } else { Some(data_dir) };
    BOOT_HUB_ENV.with(|cell| *cell.borrow_mut() = Some((hub_url, user, data_dir)));
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
            let mut bindings = vec![PersistenceBinding::Hub { base_url: identity.hub_base_url.clone(), space_id: space_id.to_string(), surface: surface.map(str::to_string) }];
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

/// 📂️ Parses the schema-first opening relay shared with the React shell. String and numeric
/// role forms normalize identically; a surface-suffixed artifact ref must agree with the role; app
/// and document coordinates are all-or-nothing pairs.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug)]
struct OpenArtifactRelayTarget {
    artifact_ref: String,
    dialect: semio_framework::ArtifactDialect,
    role: semio_framework::AppRole,
    plugin_id: Option<String>,
    app_id: Option<String>,
    document_id: Option<String>,
    space_id: Option<String>,
    schema: Option<String>,
}

#[cfg(not(target_arch = "wasm32"))]
fn open_artifact_relay_target(action_id: &str, args: Option<&Value>) -> Result<OpenArtifactRelayTarget, &'static str> {
    let args = args.and_then(Value::as_object).ok_or("opening.invalid-args")?;
    let text = |field: &str| args.get(field).and_then(Value::as_str).filter(|value| !value.trim().is_empty()).map(str::to_string);
    let raw_artifact_ref = text("artifactRef").ok_or("opening.invalid-artifact-ref")?;
    let (dialect, surface_role) = if raw_artifact_ref.contains('#') {
        let (dialect, role) = semio_framework::parse_surface_app_id(&raw_artifact_ref).map_err(|_| "opening.invalid-artifact-ref")?;
        (dialect, Some(role))
    } else {
        (semio_framework::ArtifactDialect::parse_coordinate(&raw_artifact_ref).map_err(|_| "opening.invalid-artifact-ref")?, None)
    };
    let wire_role = match args.get("role") {
        None => None,
        Some(Value::Number(number)) if number.as_u64() == Some(0) => Some(semio_framework::AppRole::Viewer),
        Some(Value::Number(number)) if number.as_u64() == Some(1) => Some(semio_framework::AppRole::Editor),
        Some(Value::String(role)) if role == "viewer" => Some(semio_framework::AppRole::Viewer),
        Some(Value::String(role)) if role == "editor" => Some(semio_framework::AppRole::Editor),
        _ => return Err("opening.invalid-role"),
    };
    if surface_role.is_some() && wire_role.is_some() && surface_role != wire_role {
        return Err("opening.role-mismatch");
    }
    let role = wire_role.or(surface_role).unwrap_or(semio_framework::AppRole::Editor);
    let plugin_id = text("pluginId");
    let app_id = text("appId");
    if plugin_id.is_some() != app_id.is_some() {
        return Err("opening.partial-app-ref");
    }
    if action_id == "os.open-artifact-with" && plugin_id.is_none() {
        return Err("opening.explicit-app-required");
    }
    if let Some(app_id) = &app_id {
        let (app_dialect, app_role) = semio_framework::parse_surface_app_id(app_id).map_err(|_| "opening.app-mismatch")?;
        if app_dialect != dialect || app_role != role {
            return Err("opening.app-mismatch");
        }
    }
    let document_id = text("documentId");
    let schema = text("schema");
    if document_id.is_some() != schema.is_some() {
        return Err("opening.partial-document-ref");
    }
    Ok(OpenArtifactRelayTarget { artifact_ref: dialect.to_coordinate(), dialect, role, plugin_id, app_id, document_id, space_id: text("spaceId"), schema })
}

/// 👥️ Projects only Hub-normalized peers for the shell's currently attached surface.
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
fn presence_peer_rows_for_surface(peers: &[PresencePeer], attached_surface: Option<&str>, target_surface: &str) -> Vec<ui_wgpu::wgpu::PresencePeerRow> {
    if attached_surface != Some(target_surface) {
        return Vec::new();
    }
    peers
        .iter()
        .filter(|peer| peer.surface.as_deref() == Some(target_surface))
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
            color: peer.color,
        })
        .collect()
}

/// 🪐️ ticket §C4/§6 — the space's own artifact-index document: kind `s.space`, dialect
/// `s.space.space@1/*`, document id always the literal `"index"` (one per hub space) — mirrored from
/// the Rust source of truth (`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🦀️.rs`), same as the
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

#[cfg(not(target_arch = "wasm32"))]
/// 🎯 Resolves the descriptor-bound canonical surface id this local selection may request. It is a
/// preference, never an authority: a local WGPU selection verifies no execution-target bytes and so
/// can never mint a `DocumentExecutionTargetLeaseFieldsV1`.
fn document_socket_surface_from_descriptor(plugin_id: &str, package_id: Option<&str>, manifest: &semio_framework::PluginManifest, app: &AppDefinition, window_kind_id: &str) -> Result<String, String> {
    let _package_id = package_id.filter(|value| !value.is_empty()).ok_or_else(|| "document open requires a verified package descriptor".to_string())?;
    if plugin_id != manifest.plugin_id || manifest.version.is_empty() || !manifest.apps.iter().any(|candidate| candidate.id == app.id) || !app.window_kinds.iter().any(|window| window.id == window_kind_id) {
        return Err("document open local plugin selection is not descriptor-bound".into());
    }
    Ok(semio_framework::manifest::surface_app_id(&app.dialect, app.role))
}

#[cfg(not(target_arch = "wasm32"))]
fn wgpu_document_socket_surface(program: &ProgramBridgeEntry, app: &AppDefinition, window_kind_id: &str) -> Result<String, String> {
    document_socket_surface_from_descriptor(&program.plugin_id, program.package_id.as_deref(), &program.manifest, app, window_kind_id)
}

#[cfg(not(target_arch = "wasm32"))]
fn bind_wgpu_document_socket_surface(host: &ArtifactHost, document_id: &str, artifact_schema: &str, bindings: &[PersistenceBinding], program: &ProgramBridgeEntry, app: &AppDefinition, window_kind_id: &str) -> Result<(), String> {
    let mut hub_spaces = bindings.iter().filter_map(|binding| match binding {
        PersistenceBinding::Hub { space_id, .. } => Some(space_id.as_str()),
        PersistenceBinding::Folder { .. } => None,
    });
    let Some(space_id) = hub_spaces.next() else {
        return Ok(());
    };
    if hub_spaces.any(|candidate| candidate != space_id) {
        return Err("document open cannot span hub spaces".into());
    }
    if app.io.document_schema != artifact_schema {
        return Err("document open local artifact schema does not match the selected app".into());
    }
    let surface_id = wgpu_document_socket_surface(program, app, window_kind_id)?;
    if bindings.iter().any(|binding| matches!(binding, PersistenceBinding::Hub { surface: Some(selected), .. } if selected != &surface_id)) {
        return Err("document open local surface does not match the hub binding".into());
    }
    let _ = (host, space_id, document_id);
    Ok(())
}

//#endregion 🔖️IdentityPure

//#region 🔖️CheckInPure
/// 📌️ ticket 26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END §C5 — auto check-in policy
/// constants, byte-identical to the React shell's `AUTO_CHECKIN_IDLE_MS`/`AUTO_CHECKIN_EDIT_THRESHOLD`
/// (`ShellHelpers/🟦️.tsx`).
#[cfg(not(target_arch = "wasm32"))]
const AUTO_CHECKIN_IDLE_MS: i64 = 20_000;
#[cfg(not(target_arch = "wasm32"))]
const AUTO_CHECKIN_EDIT_THRESHOLD: u32 = 200;

/// 🧾️ ticket §C5 — folds one `HistoryPatch` into the running per-session entry map, the wgpu twin of
/// the React shell's `applyHistoryPatch` reducer (`ShellHost/🟦️.tsx`): `replace=true` (a
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
/// React shell's `uncommittedEditCount` (`ShellHost/🟦️.tsx`) uses — every applied
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
/// `canCheckIn` (`ShellHelpers/🟦️.tsx`): the one predicate gating both the `#s-checkin`
/// affordance's presence and the auto check-in poll's arming.
#[cfg(not(target_arch = "wasm32"))]
fn can_check_in(role: semio_framework::manifest::AppRole) -> bool {
    role == semio_framework::manifest::AppRole::Editor
}

/// 📌️ ticket §C5 item 2 — pure decision for the per-frame auto check-in poll. This shell has no timer
/// wheel (see `render_sync_status_and_checkin`'s doc note on `Effect::DispatchAction`'s `delay_ms`
/// collapsing to "next tick"), so `AutoCheckinScheduler`'s `setTimeout`-based debounce
/// (`ShellHelpers/🟦️.tsx`) is reproduced as a poll instead: fires once uncommitted edits
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
#[path = "../../🧪️tests/🔬️wgpu-identity-directory-presence/🦀️.rs"]
mod identity_directory_presence_tests;
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

#[cfg(test)]
fn context_menu_paths_equal(a: &[usize], b: &[usize]) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(left, right)| left == right)
}

#[cfg(test)]
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
/// `ChannelBackbone` (see `framework/product/os/core/rs`'s `ArtifactHost` canonical sequence).
#[cfg(not(target_arch = "wasm32"))]
pub struct ShellSyncChannel {
    pub document_id: String,
    pub document_key: ArtifactDocumentKey,
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
    find_items: ShellFindItems,
    content_focus: HashMap<String, bool>,
    tooltip_titles: HashMap<String, String>,
    tooltip_hover: Option<ChromeTooltipHover>,
    dialog_stack: Vec<ChromeDialogRequest>,
    tour_state: Option<ChromeTourState>,
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
    maintenance: ShellChromeMaintenance,
}

const SHELL_CHROME_IO_FIELD_BYTES: usize = 4 * 1024;

#[derive(Default)]
struct ShellChromeMaintenance {
    load_requested: bool,
    load_phase: u8,
    introduction_read: Option<String>,
    introduction_write: Option<String>,
    layout_requested: bool,
    presence_requested: bool,
    persist_requested: bool,
    persist_phase: u8,
}

impl ShellChromeMaintenance {
    fn pending(&self) -> bool {
        self.load_requested || self.introduction_read.is_some() || self.introduction_write.is_some() || self.layout_requested || self.presence_requested || self.persist_requested
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
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
fn close_unowned_directory_dial<T: DirectoryWsConnection>(result: Result<T, TransportError>) {
    if let Ok(mut connection) = result {
        connection.close();
    }
}

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
    terminal: std::sync::atomic::AtomicBool,
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
            terminal: std::sync::atomic::AtomicBool::new(false),
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
                DirectoryStreamTurn::Dial { client, since } => {
                    let weak = std::sync::Arc::downgrade(&self);
                    let context = self.context.clone();
                    self.pool.submit(
                        Lane::Io,
                        Box::new(move || {
                            let result = client.open_stream_ws(&context, since, Self::DIAL_TIMEOUT_MS).map_err(|error| TransportError::Io(error.to_string()));
                            if let Some(runner) = weak.upgrade() {
                                let now_ms = runner.pool.now_ms();
                                let _ = runner.stream.lock().expect("directory stream mutex poisoned").complete_dial(now_ms, result);
                                runner.schedule();
                            } else {
                                close_unowned_directory_dial(result);
                            }
                        }),
                    );
                    break;
                }
                DirectoryStreamTurn::DialScoped { .. } | DirectoryStreamTurn::Revoked(_) => {
                    self.terminal.store(true, std::sync::atomic::Ordering::Release);
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
                DirectoryStreamTurn::Closed => {
                    if !self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
                        self.terminal.store(true, std::sync::atomic::Ordering::Release);
                    }
                    break;
                }
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

    fn take_terminal(&self) -> bool {
        self.terminal.swap(false, std::sync::atomic::Ordering::AcqRel)
    }

    fn cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.context.cancel.cancel_now();
        self.stream.lock().expect("directory stream mutex poisoned").close();
    }
}

//#region 💡️InferencePort
/// 💡️ Pure finite driver for one host-owned ephemeral inference port. It performs NO I/O: every
/// turn returns the single bounded action the shell should take next, and every completed action is
/// folded back through the shared `reduce_gis_map_inference_port_v1` reducer the browser worker also
/// runs. It refuses to leave `Idle` at all unless the document's execution-target lease is verified,
/// it never has more than one action in flight, it never invents a phase the server has not
/// reported, and a terminal phase is hard — no later turn or completion can move it.
#[cfg(not(target_arch = "wasm32"))]
pub struct GisMapInferenceDriverV1 {
    scope: DocumentScope,
    status: GisMapInferencePortStatusV1,
    lease_verified: bool,
    intent: Option<GisMapInferenceIntentV1>,
    in_flight: bool,
    turns: u32,
    next_poll_at_ms: u64,
}

/// 🎬 One operator intent the driver may still be holding.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GisMapInferenceIntentV1 {
    Propose,
    Cancel,
    Approve,
}

/// 🎯 The single bounded action one turn asks for.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GisMapInferenceTurnV1 {
    Idle,
    WaitUntil(u64),
    Submit,
    Poll { job_id: String, after: u64 },
    Cancel { job_id: String },
    Approve { job_id: String, proposal_hash: String },
    Terminal,
}

#[cfg(not(target_arch = "wasm32"))]
impl GisMapInferenceDriverV1 {
    /// 🔁 Highest number of poll turns one job may take before it is reported indeterminate.
    pub const MAX_POLL_TURNS: u32 = 240;
    /// ⏱ Bounded poll cadence — a timer-armed reschedule, never a busy loop.
    pub const POLL_INTERVAL_MS: u64 = 750;
    /// ⏳ Lifetime one submitted job asks the hub for.
    pub const JOB_LIFETIME_MS: u64 = 60_000;

    /// 🆕 Builds one driver. `lease_verified` is the caller's own answer to "does this document own a
    /// verified, live execution target right now" — the driver never infers it.
    pub fn new(scope: DocumentScope, lease_verified: bool) -> Self {
        Self { scope, status: GisMapInferencePortStatusV1::default(), lease_verified, intent: None, in_flight: false, turns: 0, next_poll_at_ms: 0 }
    }

    pub fn scope(&self) -> &DocumentScope {
        &self.scope
    }

    pub fn status(&self) -> &GisMapInferencePortStatusV1 {
        &self.status
    }

    /// 🎬 Records one operator intent. A terminal port accepts none.
    pub fn intend(&mut self, intent: GisMapInferenceIntentV1) {
        if self.status.phase.terminal() {
            return;
        }
        if intent == GisMapInferenceIntentV1::Cancel {
            self.apply(&GisMapInferencePortEventV1::Cancel);
        }
        self.intent = Some(intent);
    }

    /// 🧮 Folds one exact answer back through the shared reducer and releases the in-flight slot.
    pub fn complete(&mut self, event: &GisMapInferencePortEventV1) {
        self.in_flight = false;
        self.apply(event);
    }

    fn apply(&mut self, event: &GisMapInferencePortEventV1) {
        self.status = reduce_gis_map_inference_port_v1(&self.status, event);
    }

    /// 🔄 One bounded turn. It asks for at most one action, never two, and arms a timer instead of
    /// spinning whenever the only remaining work is the next poll.
    pub fn turn(&mut self, now_ms: u64) -> GisMapInferenceTurnV1 {
        if self.status.phase.terminal() {
            return GisMapInferenceTurnV1::Terminal;
        }
        if !self.lease_verified {
            self.apply(&GisMapInferencePortEventV1::LeaseUnverified);
            return GisMapInferenceTurnV1::Terminal;
        }
        if self.in_flight {
            return GisMapInferenceTurnV1::WaitUntil(now_ms.saturating_add(Self::POLL_INTERVAL_MS));
        }
        match self.intent.take() {
            Some(GisMapInferenceIntentV1::Propose) if self.status.phase == GisMapInferencePortPhaseV1::Idle => {
                self.apply(&GisMapInferencePortEventV1::Start);
                self.in_flight = true;
                return GisMapInferenceTurnV1::Submit;
            }
            Some(GisMapInferenceIntentV1::Cancel) => {
                if let Some(job_id) = self.status.job_id.clone() {
                    self.in_flight = true;
                    return GisMapInferenceTurnV1::Cancel { job_id };
                }
            }
            // ✅️ The shared reducer owns the whole approve admission (offered, a matching bounded
            // preview, no pending cancel): the driver applies the intent FIRST and only asks for the
            // call when that reducer actually entered `Approving`, so this native path is structurally
            // incapable of approving something the browser port would refuse.
            Some(GisMapInferenceIntentV1::Approve) => {
                let before = self.status.phase;
                self.apply(&GisMapInferencePortEventV1::Approve);
                if before != self.status.phase && self.status.phase == GisMapInferencePortPhaseV1::Approving {
                    if let (Some(job_id), Some(proposal_hash)) = (self.status.job_id.clone(), self.status.proposal_hash.clone()) {
                        self.in_flight = true;
                        return GisMapInferenceTurnV1::Approve { job_id, proposal_hash };
                    }
                }
            }
            _ => {}
        }
        let Some(job_id) = self.status.job_id.clone() else {
            return GisMapInferenceTurnV1::Idle;
        };
        if self.turns >= Self::MAX_POLL_TURNS {
            self.apply(&GisMapInferencePortEventV1::Failed(GisMapInferencePortCodeV1::Transport));
            return GisMapInferenceTurnV1::Terminal;
        }
        if now_ms < self.next_poll_at_ms {
            return GisMapInferenceTurnV1::WaitUntil(self.next_poll_at_ms);
        }
        self.turns += 1;
        self.in_flight = true;
        self.next_poll_at_ms = now_ms.saturating_add(Self::POLL_INTERVAL_MS);
        GisMapInferenceTurnV1::Poll { job_id, after: self.status.cursor }
    }
}

/// 💡️ Finite inference-port actor: bounded turns, I/O-lane calls through the native
/// `DirectoryClient`, timer-wheel wakeups and ordered status output — the exact discipline
/// `ShellDirectoryRunner` above already follows for the directory stream. No blocking call ever
/// enters the render loop, and a cancel is a hard terminal.
#[cfg(not(target_arch = "wasm32"))]
struct ShellInferenceRunner {
    pool: WorkerPool,
    client: std::sync::Arc<DirectoryClient<NativeDirectoryTransport<TokioHostRuntime>>>,
    context: OperationContext,
    driver: std::sync::Mutex<GisMapInferenceDriverV1>,
    statuses: std::sync::Mutex<std::collections::VecDeque<GisMapInferencePortStatusV1>>,
    scheduled: std::sync::atomic::AtomicBool,
    notified: std::sync::atomic::AtomicBool,
    cancelled: std::sync::atomic::AtomicBool,
    timer_deadline_ms: std::sync::atomic::AtomicU64,
}

#[cfg(not(target_arch = "wasm32"))]
impl ShellInferenceRunner {
    const MAX_STATUSES: usize = 64;
    const MAX_ACTIONS_PER_TURN: usize = 4;
    const MAX_TURN_MS: u64 = 4;

    fn start(pool: WorkerPool, client: std::sync::Arc<DirectoryClient<NativeDirectoryTransport<TokioHostRuntime>>>, context: OperationContext, driver: GisMapInferenceDriverV1) -> std::sync::Arc<Self> {
        let runner = std::sync::Arc::new(Self {
            pool,
            client,
            context,
            driver: std::sync::Mutex::new(driver),
            statuses: std::sync::Mutex::new(std::collections::VecDeque::new()),
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

    fn intend(self: &std::sync::Arc<Self>, intent: GisMapInferenceIntentV1) {
        self.driver.lock().expect("inference driver mutex poisoned").intend(intent);
        self.publish();
        self.schedule();
    }

    fn publish(self: &std::sync::Arc<Self>) {
        let status = self.driver.lock().expect("inference driver mutex poisoned").status().clone();
        let mut statuses = self.statuses.lock().expect("inference statuses mutex poisoned");
        if statuses.back() == Some(&status) {
            return;
        }
        if statuses.len() >= Self::MAX_STATUSES {
            statuses.pop_front();
        }
        statuses.push_back(status);
    }

    fn run_turn(self: std::sync::Arc<Self>) {
        self.notified.store(false, std::sync::atomic::Ordering::Release);
        let started = self.pool.now_ms();
        let mut processed = 0usize;
        let mut wake_at = None;
        while processed < Self::MAX_ACTIONS_PER_TURN && self.pool.now_ms().saturating_sub(started) < Self::MAX_TURN_MS {
            if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
                break;
            }
            let action = self.driver.lock().expect("inference driver mutex poisoned").turn(self.pool.now_ms());
            self.publish();
            match action {
                GisMapInferenceTurnV1::Idle | GisMapInferenceTurnV1::Terminal => break,
                GisMapInferenceTurnV1::WaitUntil(deadline_ms) => {
                    wake_at = Some(deadline_ms);
                    break;
                }
                other => {
                    self.submit_action(other);
                    processed += 1;
                }
            }
        }
        self.scheduled.store(false, std::sync::atomic::Ordering::Release);
        if self.notified.load(std::sync::atomic::Ordering::Acquire) {
            self.schedule();
        } else if let Some(deadline_ms) = wake_at {
            self.arm_timer(deadline_ms);
        }
    }

    /// 📡 Submits exactly one bounded call on the I/O lane and folds its exact answer back.
    fn submit_action(self: &std::sync::Arc<Self>, action: GisMapInferenceTurnV1) {
        let weak = std::sync::Arc::downgrade(self);
        let client = self.client.clone();
        let context = self.context.clone();
        let scope = self.driver.lock().expect("inference driver mutex poisoned").scope().clone();
        ShellPoolFuture::spawn(self.pool.clone(), Lane::Io, async move {
            let outcome = {
                match action {
                    GisMapInferenceTurnV1::Submit => {
                        let request = GisMapInferenceJobRequestV1 {
                            schema: "semio.hub.inference-request/v1".to_string(),
                            version: 1,
                            request_id: mint_directory_command_request_id(),
                            service_id: GIS_MAP_INFERENCE_SERVICE_ID.to_string(),
                            policy_version: 1,
                            lifetime_ms: GisMapInferenceDriverV1::JOB_LIFETIME_MS,
                        };
                        client.submit_gis_map_inference_job(&context, &scope, &request).await.map(GisMapInferencePortEventV1::Receipt)
                    }
                    GisMapInferenceTurnV1::Poll { job_id, after } => client.read_gis_map_inference_events(&context, &scope, &job_id, after).await.map(GisMapInferencePortEventV1::Page),
                    GisMapInferenceTurnV1::Cancel { job_id } => client.cancel_gis_map_inference_job(&context, &scope, &job_id).await.map(GisMapInferencePortEventV1::Page),
                    GisMapInferenceTurnV1::Approve { job_id, proposal_hash } => {
                        let request = GisMapInferenceApprovalRequestV1 { schema: "semio.hub.inference-approval/v1".to_string(), version: 1, job_id, proposal_hash };
                        client.approve_gis_map_inference_job(&context, &scope, &request).await.map(GisMapInferencePortEventV1::Approval)
                    }
                    GisMapInferenceTurnV1::Idle | GisMapInferenceTurnV1::WaitUntil(_) | GisMapInferenceTurnV1::Terminal => Ok(GisMapInferencePortEventV1::Clear),
                }
            };
            let event = outcome.unwrap_or_else(GisMapInferencePortEventV1::Failed);
            if let Some(runner) = weak.upgrade() {
                runner.driver.lock().expect("inference driver mutex poisoned").complete(&event);
                runner.publish();
                runner.schedule();
            }
        });
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

    fn drain(self: &std::sync::Arc<Self>) -> Vec<GisMapInferencePortStatusV1> {
        self.statuses.lock().expect("inference statuses mutex poisoned").drain(..).collect()
    }

    fn cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.context.cancel.cancel_now();
    }
}
//#endregion 💡️InferencePort

#[cfg(not(target_arch = "wasm32"))]
type ShellDirectoryPageResult = (u64, Result<CanonicalDirectoryEventPageV1, DirectoryClientError>);

#[cfg(not(target_arch = "wasm32"))]
enum ShellDirectoryHomePublicationOutcome {
    Published(DirectoryEventPageAckV1),
    Rejected(String),
    Terminal(String),
}

#[cfg(not(target_arch = "wasm32"))]
type ShellDirectoryHomePublicationResult = (u64, u32, DirectoryEventPageAckV1, ShellDirectoryHomePublicationOutcome);

#[cfg(not(target_arch = "wasm32"))]
struct DirectoryHomeProjection {
    plugin_id: String,
    instance_id: u32,
    app: AppDefinition,
    view_state: ViewModel,
    bootstrap: DirectoryEventPageBootstrapV1,
    page_task: Option<std::sync::Arc<ShellPoolFuture>>,
    page_cancel: Option<CancelToken>,
    page_rx: Option<std::sync::mpsc::Receiver<ShellDirectoryPageResult>>,
    home_task: Option<std::sync::Arc<ShellPoolFuture>>,
    home_rx: Option<std::sync::mpsc::Receiver<ShellDirectoryHomePublicationResult>>,
    stream: Option<std::sync::Arc<ShellDirectoryRunner>>,
    retry_at_ms: u64,
    closed: bool,
    destroy_authority: bool,
}

#[cfg(not(target_arch = "wasm32"))]
impl DirectoryHomeProjection {
    const RETRY_DELAY_MS: u64 = 50;

    fn new(plugin_id: String, instance_id: u32, app: AppDefinition, view_state: ViewModel) -> Result<Self, DirectoryClientError> {
        Ok(Self {
            plugin_id,
            instance_id,
            app,
            view_state,
            bootstrap: DirectoryEventPageBootstrapV1::new(1, 0)?,
            page_task: None,
            page_cancel: None,
            page_rx: None,
            home_task: None,
            home_rx: None,
            stream: None,
            retry_at_ms: 0,
            closed: false,
            destroy_authority: true,
        })
    }

    fn is_instance(&self, plugin_id: &str, instance_id: u32) -> bool {
        self.plugin_id == plugin_id && self.instance_id == instance_id
    }

    fn active_session(&self) -> ActiveSession {
        ActiveSession { plugin_id: self.plugin_id.clone(), instance_id: self.instance_id, app: self.app.clone(), view_state: self.view_state.clone() }
    }

    fn cancel_io(&mut self) {
        if let Some(cancel) = self.page_cancel.take() {
            cancel.cancel_now();
        }
        if let Some(task) = self.page_task.take() {
            task.cancel();
        }
        self.page_rx.take();
        if let Some(task) = self.home_task.take() {
            task.cancel();
        }
        self.home_rx.take();
        if let Some(stream) = self.stream.take() {
            stream.cancel();
        }
    }

    fn begin_epoch(&mut self, after: u64) -> Result<u64, DirectoryClientError> {
        self.cancel_io();
        self.bootstrap.close();
        let epoch = self.bootstrap.bootstrap_epoch().checked_add(1).ok_or_else(|| DirectoryClientError::Decode("native directory bootstrap epoch exhausted".into()))?;
        self.bootstrap = DirectoryEventPageBootstrapV1::new(epoch, after)?;
        self.retry_at_ms = 0;
        self.closed = false;
        Ok(epoch)
    }

    fn retry(&mut self, now_ms: u64, receipt_sha256: &str) -> Result<u64, DirectoryClientError> {
        let after = self.bootstrap.reject(self.bootstrap.bootstrap_epoch(), receipt_sha256)?;
        self.retry_at_ms = now_ms.saturating_add(Self::RETRY_DELAY_MS);
        Ok(after)
    }

    fn wake(&mut self, rebootstrap: bool) -> Result<Option<u64>, DirectoryClientError> {
        if rebootstrap {
            return self.begin_epoch(0).map(Some);
        }
        self.cancel_io();
        Ok(self.bootstrap.wake(false))
    }

    fn close(&mut self) {
        self.cancel_io();
        self.bootstrap.close();
        self.closed = true;
    }

    fn take_destroy_authority(&mut self) -> Option<(String, u32)> {
        self.close();
        if !self.destroy_authority {
            return None;
        }
        self.destroy_authority = false;
        Some((self.plugin_id.clone(), self.instance_id))
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn terminal_directory_home_ack(result: &semio_framework::kernel::InvocationResult, bootstrap_epoch: u64) -> Result<DirectoryEventPageAckV1, String> {
    const RECEIPT_SCHEMA: &str = "semio.space.home.directory-projection-receipt.v1";
    let mut acknowledgement = None;
    for effect in &result.requested_effects {
        let semio_framework::kernel::Effect::PublishEvent { topic, payload } = effect else {
            return Err("retained Home directory publication emitted a non-receipt effect".into());
        };
        if topic != RECEIPT_SCHEMA || acknowledgement.is_some() {
            return Err("retained Home directory publication emitted an unexpected receipt".into());
        }
        let value = store::pack_rt::decode_wire_value(payload).map_err(|error| error.to_string())?;
        let DslValue::Object(fields) = &value else {
            return Err("retained Home directory receipt is not an object".into());
        };
        let expected_keys = ["schema", "sessionBindingSha256", "authorizationGeneration", "throughSeqInclusive", "receiptSha256"];
        if fields.len() != expected_keys.len() || !fields.iter().all(|(key, _)| expected_keys.contains(&key.as_str())) {
            return Err("retained Home directory receipt has an inexact field set".into());
        }
        let field = |key: &str| value.get(key).ok_or_else(|| format!("retained Home directory receipt lacks {key}"));
        if field("schema")?.as_str() != Some(RECEIPT_SCHEMA) {
            return Err("retained Home directory receipt schema mismatch".into());
        }
        acknowledgement = Some(DirectoryEventPageAckV1 {
            bootstrap_epoch,
            session_binding_sha256: field("sessionBindingSha256")?.as_str().ok_or("retained Home directory receipt binding is not text")?.to_string(),
            authorization_generation: field("authorizationGeneration")?.as_u64().ok_or("retained Home directory receipt generation is not an unsigned integer")?,
            through_seq_inclusive: field("throughSeqInclusive")?.as_u64().ok_or("retained Home directory receipt frontier is not an unsigned integer")?,
            receipt_sha256: field("receiptSha256")?.as_str().ok_or("retained Home directory page receipt is not text")?.to_string(),
        });
    }
    if !result.mutations.is_empty() {
        return Err("retained Home directory publication escaped its Config-only lane".into());
    }
    acknowledgement.ok_or_else(|| "retained Home directory publication returned no terminal receipt".into())
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../🧪️tests/🔬️wgpu-shell-pool-future/🦀️.rs"]
mod shell_pool_future_tests;

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
#[path = "../../🧪️tests/🔬️wgpu-shell-document-retirement/🦀️.rs"]
mod shell_document_retirement_tests;
//#endregion 📄️ShellDocumentRetirement

//#region 🏛️SpaceAdministration
/// 🏛️ Closed lifecycle of the one shell-owned native space-administration operation — the WGPU twin
/// of the browser worker's `DirectoryAdministrationOperation`. Every terminal phase has already
/// erased the page, the receipt, and any invite capability.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShellSpaceAdministrationPhaseV1 {
    Loading,
    Ready,
    Submitting,
    Receipt,
    Refreshing,
    Cancelled,
    Denied,
    Stale,
    Failed,
}

#[cfg(not(target_arch = "wasm32"))]
impl ShellSpaceAdministrationPhaseV1 {
    /// 🏁️ A terminal phase never advances again and holds no page, receipt, or capability.
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Cancelled | Self::Denied | Self::Stale | Self::Failed)
    }

    /// 🚦️ Dispatch is admitted only from a settled `Ready` page — never while a receipt is pending.
    pub fn is_dispatchable(self) -> bool {
        matches!(self, Self::Ready)
    }
}

/// 🎬️ The one bounded step the driver may take next. `Idle` means the operation is waiting on the
/// renderer; `Closed` means it has been retired and must be dropped.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, PartialEq)]
pub enum ShellSpaceAdministrationTurnV1 {
    FetchPage { cursor: Option<String> },
    Submit { request: DirectoryCommandRequestV1 },
    Idle,
    Closed,
}

/// 🏛️ The one retained native administration operation (fixed capacity: exactly one per shell).
/// It owns one canonical page, at most one command request/receipt, and at most one one-shot invite
/// capability. Administration mutations never auto-retry: an indeterminate transport is terminal
/// `Failed` ("unknown outcome / refresh required"), so a `create-invite` can never mint twice.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug)]
pub struct ShellSpaceAdministrationOperationV1 {
    operation_epoch: u64,
    space_id: String,
    phase: ShellSpaceAdministrationPhaseV1,
    page: Option<DirectorySpaceAdministrationPageV1>,
    canonical_json: Option<String>,
    receipt_sha256: Option<String>,
    invite_capability: Option<String>,
    pending_request: Option<DirectoryCommandRequestV1>,
    pending_cursor: Option<String>,
    fetch_requested: bool,
    code: Option<DirectoryCommandErrorCodeV1>,
}

#[cfg(not(target_arch = "wasm32"))]
impl ShellSpaceAdministrationOperationV1 {
    /// 🆕️ Opens the one operation for exactly one space; the first turn fetches its page.
    pub fn open(operation_epoch: u64, space_id: &str) -> Self {
        Self {
            operation_epoch,
            space_id: space_id.to_string(),
            phase: ShellSpaceAdministrationPhaseV1::Loading,
            page: None,
            canonical_json: None,
            receipt_sha256: None,
            invite_capability: None,
            pending_request: None,
            pending_cursor: None,
            fetch_requested: true,
            code: None,
        }
    }

    pub fn operation_epoch(&self) -> u64 {
        self.operation_epoch
    }
    pub fn space_id(&self) -> &str {
        &self.space_id
    }
    pub fn phase(&self) -> ShellSpaceAdministrationPhaseV1 {
        self.phase
    }
    pub fn page(&self) -> Option<&DirectorySpaceAdministrationPageV1> {
        self.page.as_ref()
    }
    pub fn canonical_json(&self) -> Option<&str> {
        self.canonical_json.as_deref()
    }
    pub fn receipt_sha256(&self) -> Option<&str> {
        self.receipt_sha256.as_deref()
    }
    pub fn code(&self) -> Option<DirectoryCommandErrorCodeV1> {
        self.code
    }
    /// 🎁️ `true` while the worker still holds one unacknowledged one-shot invite capability.
    pub fn invite_capability_pending(&self) -> bool {
        self.invite_capability.is_some()
    }

    /// 🛂️ The ONLY authority a renderer may consult: the page's own server-filled capability flags.
    pub fn capabilities(&self) -> Option<DirectorySpaceAdministrationCapabilitiesV1> {
        self.page.as_ref().and_then(DirectorySpaceAdministrationPageV1::capabilities)
    }

    /// 🎬️ Returns the one bounded step the driver may take next.
    pub fn turn(&mut self) -> ShellSpaceAdministrationTurnV1 {
        if self.phase.is_terminal() {
            return ShellSpaceAdministrationTurnV1::Closed;
        }
        if let Some(request) = self.pending_request.clone() {
            self.phase = ShellSpaceAdministrationPhaseV1::Submitting;
            return ShellSpaceAdministrationTurnV1::Submit { request };
        }
        if self.fetch_requested {
            self.fetch_requested = false;
            self.phase = if self.page.is_some() { ShellSpaceAdministrationPhaseV1::Refreshing } else { ShellSpaceAdministrationPhaseV1::Loading };
            return ShellSpaceAdministrationTurnV1::FetchPage { cursor: self.pending_cursor.take() };
        }
        ShellSpaceAdministrationTurnV1::Idle
    }

    /// 📄️ Requests exactly one page read; `cursor` advances precisely the window it was issued for.
    pub fn request_page(&mut self, cursor: Option<String>) -> bool {
        if self.phase.is_terminal() || self.pending_request.is_some() {
            return false;
        }
        self.pending_cursor = cursor;
        self.fetch_requested = true;
        true
    }

    /// 📮️ Admits exactly one command request; a second one before the receipt is refused.
    pub fn request_command(&mut self, request: DirectoryCommandRequestV1) -> bool {
        if !self.phase.is_dispatchable() || self.pending_request.is_some() || self.fetch_requested {
            return false;
        }
        self.pending_request = Some(request);
        true
    }

    /// 📥️ Applies exactly one canonical page whose space matches this operation.
    pub fn apply_page(&mut self, canonical_json: String, page: DirectorySpaceAdministrationPageV1) -> bool {
        if self.phase.is_terminal() || page.space_id() != self.space_id {
            return false;
        }
        self.page = Some(page);
        self.canonical_json = Some(canonical_json);
        self.phase = ShellSpaceAdministrationPhaseV1::Ready;
        true
    }

    /// 🧾️ Applies exactly one accepted server receipt, then schedules the mandatory page refresh.
    pub fn apply_receipt(&mut self, receipt: &DirectoryCommandReceiptV1) -> bool {
        let Some(request) = self.pending_request.take() else { return false };
        if request.request_id != receipt.request_id || self.phase.is_terminal() {
            return false;
        }
        self.receipt_sha256 = Some(receipt.receipt_sha256.clone());
        self.invite_capability = match &receipt.result {
            DirectoryCommandResultV1::Invite { invite_token } => Some(invite_token.clone()),
            DirectoryCommandResultV1::None => None,
        };
        self.phase = ShellSpaceAdministrationPhaseV1::Receipt;
        self.fetch_requested = true;
        self.pending_cursor = None;
        true
    }

    /// 🎁️ Hands the one-shot invite capability over exactly once and erases it in the same turn.
    pub fn acknowledge_capability(&mut self) -> Option<String> {
        self.invite_capability.take()
    }

    /// 🧯️ Erases page, receipt, and capability, then settles one terminal phase exactly once.
    pub fn terminate(&mut self, phase: ShellSpaceAdministrationPhaseV1, code: Option<DirectoryCommandErrorCodeV1>) {
        if self.phase.is_terminal() {
            return;
        }
        self.page = None;
        self.canonical_json = None;
        self.receipt_sha256 = None;
        self.invite_capability = None;
        self.pending_request = None;
        self.pending_cursor = None;
        self.fetch_requested = false;
        self.phase = if phase.is_terminal() { phase } else { ShellSpaceAdministrationPhaseV1::Failed };
        self.code = code;
    }

    /// 🚦️ Maps one page-read rejection onto the closed terminal vocabulary; 401/403 clears the pane.
    pub fn fail_page(&mut self, error: &DirectoryClientError) {
        let (phase, code) = match error {
            DirectoryClientError::Unauthorized => (ShellSpaceAdministrationPhaseV1::Denied, DirectoryCommandErrorCodeV1::Unauthorized),
            DirectoryClientError::Cancelled => (ShellSpaceAdministrationPhaseV1::Cancelled, DirectoryCommandErrorCodeV1::Cancelled),
            DirectoryClientError::Http { status: 403 | 404, .. } => (ShellSpaceAdministrationPhaseV1::Denied, DirectoryCommandErrorCodeV1::Forbidden),
            DirectoryClientError::Http { status: 409 | 410, .. } => (ShellSpaceAdministrationPhaseV1::Stale, DirectoryCommandErrorCodeV1::StaleSession),
            _ => (ShellSpaceAdministrationPhaseV1::Failed, DirectoryCommandErrorCodeV1::Transport),
        };
        self.terminate(phase, Some(code));
    }

    /// 🚦️ Maps one command rejection onto the closed terminal vocabulary. Nothing is ever retried:
    /// an indeterminate outcome must be resolved by an operator refresh, never by a silent reissue.
    pub fn fail_command(&mut self, code: DirectoryCommandErrorCodeV1) {
        let phase = match code {
            DirectoryCommandErrorCodeV1::Unauthorized | DirectoryCommandErrorCodeV1::Forbidden => ShellSpaceAdministrationPhaseV1::Denied,
            DirectoryCommandErrorCodeV1::StaleSession => ShellSpaceAdministrationPhaseV1::Stale,
            DirectoryCommandErrorCodeV1::Cancelled => ShellSpaceAdministrationPhaseV1::Cancelled,
            _ => ShellSpaceAdministrationPhaseV1::Failed,
        };
        self.terminate(phase, Some(code));
    }
}

/// 🛂️ An owner row can never be removed: the server rejects it, so the control is disabled before
/// dispatch rather than offered and then refused.
#[cfg(not(target_arch = "wasm32"))]
pub fn shell_space_administration_member_removable(row: &DirectorySpaceAdministrationMemberRowV1, capabilities: Option<DirectorySpaceAdministrationCapabilitiesV1>) -> bool {
    capabilities.is_some_and(|capabilities| capabilities.remove_member) && !row.owner
}

/// 🎟️ A revoked or accepted invitation is terminal; only a live one may be revoked.
#[cfg(not(target_arch = "wasm32"))]
pub fn shell_space_administration_invite_revocable(row: &DirectorySpaceAdministrationInviteRowV1, capabilities: Option<DirectorySpaceAdministrationCapabilitiesV1>) -> bool {
    capabilities.is_some_and(|capabilities| capabilities.revoke_invite) && !row.revoked && !row.accepted
}

/// 🗣️ Every visible administration string, in both languages, with no default: the caller passes the
/// active locale and gets the exact pair member. Mirrors the React pane's own bundle one-for-one.
#[cfg(not(target_arch = "wasm32"))]
pub fn shell_space_administration_label(key: &str, german: bool) -> &'static str {
    match (key, german) {
        ("title", false) => "Space administration",
        ("title", true) => "Space-Verwaltung",
        ("members", false) => "Members",
        ("members", true) => "Mitglieder",
        ("invites", false) => "Invitations",
        ("invites", true) => "Einladungen",
        ("role", false) => "Role",
        ("role", true) => "Rolle",
        ("author", false) => "Author",
        ("author", true) => "Autor",
        ("spectator", false) => "Spectator",
        ("spectator", true) => "Betrachter",
        ("owner", false) => "Owner",
        ("owner", true) => "Eigentümer",
        ("remove", false) => "Remove member",
        ("remove", true) => "Mitglied entfernen",
        ("issue", false) => "Issue invitation",
        ("issue", true) => "Einladung ausstellen",
        ("revoke", false) => "Revoke invitation",
        ("revoke", true) => "Einladung widerrufen",
        ("copy", false) => "Copy invitation link",
        ("copy", true) => "Einladungslink kopieren",
        ("more", false) => "Show more",
        ("more", true) => "Mehr anzeigen",
        ("loading", false) => "Loading the administration page…",
        ("loading", true) => "Verwaltungsseite wird geladen…",
        ("ready", false) => "Administration page is current.",
        ("ready", true) => "Verwaltungsseite ist aktuell.",
        ("submitting", false) => "Waiting for the server receipt…",
        ("submitting", true) => "Warte auf die Serverquittung…",
        ("receipt", false) => "Server receipt accepted.",
        ("receipt", true) => "Serverquittung angenommen.",
        ("refreshing", false) => "Refreshing after the receipt…",
        ("refreshing", true) => "Aktualisierung nach der Quittung…",
        ("cancelled", false) => "Administration was cancelled.",
        ("cancelled", true) => "Verwaltung wurde abgebrochen.",
        ("denied", false) => "Access to this space was withdrawn.",
        ("denied", true) => "Der Zugriff auf diesen Space wurde entzogen.",
        ("stale", false) => "This session changed; reopen administration.",
        ("stale", true) => "Diese Sitzung hat sich geändert; Verwaltung erneut öffnen.",
        ("failed", false) => "Unknown outcome — refresh required before retrying.",
        ("failed", true) => "Unbekanntes Ergebnis — vor einem erneuten Versuch aktualisieren.",
        _ => "",
    }
}

/// 🗣️ The status line one phase renders, in the active language.
#[cfg(not(target_arch = "wasm32"))]
pub fn shell_space_administration_status(phase: ShellSpaceAdministrationPhaseV1, german: bool) -> &'static str {
    let key = match phase {
        ShellSpaceAdministrationPhaseV1::Loading => "loading",
        ShellSpaceAdministrationPhaseV1::Ready => "ready",
        ShellSpaceAdministrationPhaseV1::Submitting => "submitting",
        ShellSpaceAdministrationPhaseV1::Receipt => "receipt",
        ShellSpaceAdministrationPhaseV1::Refreshing => "refreshing",
        ShellSpaceAdministrationPhaseV1::Cancelled => "cancelled",
        ShellSpaceAdministrationPhaseV1::Denied => "denied",
        ShellSpaceAdministrationPhaseV1::Stale => "stale",
        ShellSpaceAdministrationPhaseV1::Failed => "failed",
    };
    shell_space_administration_label(key, german)
}

/// 🎛️ One keyboard-reachable administration control the WGPU pane renders. `enabled` is derived
/// solely from the server's own capability flags and the operation's settled phase, so the native
/// renderer offers exactly the affordances the React pane does — and neither derives authority from
/// a locally stored role.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShellSpaceAdministrationControlV1 {
    pub control_id: String,
    pub label: &'static str,
    pub value: String,
    pub enabled: bool,
}

/// 🖼️ Builds the pane's complete control set from the canonical page alone. A `member`/`public`
/// page yields no administration control at all, because it structurally carries no capabilities.
#[cfg(not(target_arch = "wasm32"))]
pub fn shell_space_administration_controls(operation: &ShellSpaceAdministrationOperationV1, german: bool) -> Vec<ShellSpaceAdministrationControlV1> {
    let mut controls = Vec::new();
    controls.push(ShellSpaceAdministrationControlV1 {
        control_id: "os.space-administration.status".into(),
        label: shell_space_administration_status(operation.phase(), german),
        value: operation.receipt_sha256().unwrap_or_default().to_string(),
        enabled: false,
    });
    let Some(page) = operation.page() else { return controls };
    let capabilities = operation.capabilities();
    let dispatchable = operation.phase().is_dispatchable();
    if let DirectorySpaceAdministrationPageV1::Member { members, .. } | DirectorySpaceAdministrationPageV1::Author { members, .. } = page {
        for row in &members.rows {
            controls.push(ShellSpaceAdministrationControlV1 {
                control_id: format!("os.space-administration.role.{}", row.user_id),
                label: shell_space_administration_label("role", german),
                value: shell_space_administration_label(if row.role == DirectorySpaceRole::Author { "author" } else { "spectator" }, german).to_string(),
                enabled: dispatchable && capabilities.is_some_and(|capabilities| capabilities.upsert_member),
            });
            controls.push(ShellSpaceAdministrationControlV1 {
                control_id: format!("os.space-administration.remove.{}", row.user_id),
                label: shell_space_administration_label("remove", german),
                value: if row.owner { shell_space_administration_label("owner", german).to_string() } else { String::new() },
                enabled: dispatchable && shell_space_administration_member_removable(row, capabilities),
            });
        }
        if let Some(cursor) = &members.next_cursor {
            controls.push(ShellSpaceAdministrationControlV1 { control_id: "os.space-administration.members.more".into(), label: shell_space_administration_label("more", german), value: cursor.clone(), enabled: dispatchable });
        }
    }
    if let DirectorySpaceAdministrationPageV1::Author { invites, .. } = page {
        controls.push(ShellSpaceAdministrationControlV1 {
            control_id: "os.space-administration.invite.issue".into(),
            label: shell_space_administration_label("issue", german),
            value: String::new(),
            enabled: dispatchable && capabilities.is_some_and(|capabilities| capabilities.create_invite),
        });
        if operation.invite_capability_pending() {
            controls.push(ShellSpaceAdministrationControlV1 { control_id: "os.space-administration.invite.copy".into(), label: shell_space_administration_label("copy", german), value: String::new(), enabled: true });
        }
        for row in &invites.rows {
            controls.push(ShellSpaceAdministrationControlV1 {
                control_id: format!("os.space-administration.invite.revoke.{}", row.invite_id),
                label: shell_space_administration_label("revoke", german),
                value: row.invite_id.clone(),
                enabled: dispatchable && shell_space_administration_invite_revocable(row, capabilities),
            });
        }
        if let Some(cursor) = &invites.next_cursor {
            controls.push(ShellSpaceAdministrationControlV1 { control_id: "os.space-administration.invites.more".into(), label: shell_space_administration_label("more", german), value: cursor.clone(), enabled: dispatchable });
        }
    }
    controls
}
//#endregion 🏛️SpaceAdministration

/// 🎮️ Bounded (contract §C6 "bounded, in-memory") native directory-command FIFO depth.
#[cfg(not(target_arch = "wasm32"))]
const MAX_PENDING_DIRECTORY_COMMANDS: usize = 64;
/// 🧾️ Bounded transient command-result slot depth.
#[cfg(not(target_arch = "wasm32"))]
const MAX_DIRECTORY_COMMAND_RESULTS: usize = 64;
/// ⏳️ Finite per-command deadline; a hung hub can never retain a command turn forever.
#[cfg(not(target_arch = "wasm32"))]
const DIRECTORY_COMMAND_DEADLINE_MS: u64 = 5_000;

/// 🧾️ One transient native command completion: either the authoritative receipt or a closed
/// terminal code. Never a raw server body, and never a logged capability.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, PartialEq)]
pub enum NativeDirectoryCommandResultV1 {
    Receipt(DirectoryCommandReceiptV1),
    Failed(DirectoryCommandErrorCodeV1),
}

/// 🎮️ The shell's owned, bounded FIFO of sealed directory-command operations plus their transient
/// request-id-keyed result slot. Every policy this transport owes — fixed capacity, byte-identical
/// retry of the head, stop-at-the-first-transient-fault, and proceed past a terminal auth/conflict
/// failure — lives here so it is provable without a renderer, a surface, or a live hub.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Default)]
pub struct NativeDirectoryCommandQueueV1 {
    pending: Vec<DirectoryCommandRequestV1>,
    results: Vec<(String, NativeDirectoryCommandResultV1)>,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeDirectoryCommandQueueV1 {
    /// 📥️ Admits one sealed operation, answering the NEWEST intent with a terminal `capacity`
    /// result when full rather than silently discarding an older one the caller believes it issued.
    pub fn admit(&mut self, request: DirectoryCommandRequestV1) {
        let rejection = if request.validate().is_err() {
            Some(DirectoryCommandErrorCodeV1::Invalid)
        } else if self.pending.len() >= MAX_PENDING_DIRECTORY_COMMANDS {
            Some(DirectoryCommandErrorCodeV1::Capacity)
        } else {
            None
        };
        match rejection {
            Some(code) => self.retain(&request.request_id, NativeDirectoryCommandResultV1::Failed(code)),
            None => self.pending.push(request),
        }
    }

    /// 🚀️ Puts one freshly issued operation at the head so an interactive command is not stuck
    /// behind an offline backlog it never chose.
    pub fn admit_first(&mut self, request: DirectoryCommandRequestV1) {
        let request_id = request.request_id.clone();
        self.admit(request);
        if let Some(index) = self.pending.iter().position(|entry| entry.request_id == request_id) {
            let entry = self.pending.remove(index);
            self.pending.insert(0, entry);
        }
    }

    /// 👀️ The exact sealed request the next turn must re-send byte-identically.
    pub fn head(&self) -> Option<&DirectoryCommandRequestV1> {
        self.pending.first()
    }

    /// 🏁️ Applies one turn's outcome. Returns `true` while the FIFO may proceed: a transient fault
    /// retains the head and stops it; every terminal code produces a result and advances.
    pub fn settle(&mut self, outcome: Result<DirectoryCommandReceiptV1, DirectoryCommandErrorCodeV1>) -> bool {
        let Some(request) = self.pending.first().map(|request| request.request_id.clone()) else {
            return false;
        };
        match outcome {
            Err(code) if code.is_transient() => false,
            Err(code) => {
                self.pending.remove(0);
                self.retain(&request, NativeDirectoryCommandResultV1::Failed(code));
                true
            }
            Ok(receipt) => {
                self.pending.remove(0);
                self.retain(&request, NativeDirectoryCommandResultV1::Receipt(receipt));
                true
            }
        }
    }

    /// 🧾️ Reads one retained transient result for the issuing surface.
    pub fn result(&self, request_id: &str) -> Option<&NativeDirectoryCommandResultV1> {
        self.results.iter().find(|(id, _)| id == request_id).map(|(_, result)| result)
    }

    /// 🔢️ Operations still awaiting a live turn.
    pub fn pending(&self) -> usize {
        self.pending.len()
    }

    fn retain(&mut self, request_id: &str, result: NativeDirectoryCommandResultV1) {
        self.results.retain(|(id, _)| id != request_id);
        if self.results.len() >= MAX_DIRECTORY_COMMAND_RESULTS {
            self.results.remove(0);
        }
        self.results.push((request_id.to_string(), result));
    }
}

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
    pub tree_drag: Option<TreeDragState>,
    pub tree_hovered_id: Option<String>,
    pub widget_maps: WidgetInteractionMaps<ActionDescriptor>,
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
    /// has no native `ArtifactHost` — its sync flows through the React shell's `🧵️backbone-worker.ts`.
    #[cfg(not(target_arch = "wasm32"))]
    pub document_host: ArtifactHost,
    /// @emoji 🧵️ The currently attached document's live actor channel (native only).
    #[cfg(not(target_arch = "wasm32"))]
    pub sync_channel: Option<ShellSyncChannel>,
    /// @emoji 🚦️ Latest sync health for the active document's status badge (native only).
    #[cfg(not(target_arch = "wasm32"))]
    pub sync_status: Option<ArtifactSyncStatus>,
    #[cfg(not(target_arch = "wasm32"))]
    pub sync_bootstrap_progress: Option<(u64, u64, u32, u32)>,
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
    /// 💡️ The one retained host-owned inference port, live only while its document's execution
    /// target is verified. It is never a document command and nothing it holds is persisted.
    #[cfg(not(target_arch = "wasm32"))]
    inference_port: Option<std::sync::Arc<ShellInferenceRunner>>,
    /// 💡️ The last status the retained port published, rendered by the shell's own chrome.
    #[cfg(not(target_arch = "wasm32"))]
    pub inference_port_status: Option<GisMapInferencePortStatusV1>,
    /// 🪪️ The verified execution-target lease fields for the open document. Native document opening
    /// retains only a canonical surface-id preference today — `document_socket_surface_from_descriptor`
    /// was deliberately downgraded from a forgeable partial authority by the execution-target-lease
    /// lane — so nothing native fills this in yet and every port refuses with a localized terminal.
    #[cfg(not(target_arch = "wasm32"))]
    pub document_execution_target_lease: Option<DocumentExecutionTargetLeaseFieldsV1>,
    /// 🏠️ Shell-lifetime Home projection and its fetch → terminal Config receipt → ACK → live owner.
    #[cfg(not(target_arch = "wasm32"))]
    directory_home: Option<DirectoryHomeProjection>,
    /// 🔌️ ONE `NativeDirectoryTransport` (cheap to `.clone()` — see its own doc), shared by every
    /// `DirectoryClient` this shell constructs so they all draw on the SAME `HttpPool` byte budget/
    /// outstanding-request accounting rather than each minting a disjoint pool.
    #[cfg(not(target_arch = "wasm32"))]
    pub directory_transport: NativeDirectoryTransport<TokioHostRuntime>,
    /// 🛑️ Shell-lifetime cancellation root shared by identity, requests, and the directory runner.
    #[cfg(not(target_arch = "wasm32"))]
    pub directory_cancel: CancelToken,
    /// 🎮️ Bounded FIFO of sealed, idempotency-correlated `os.directory.*` operations plus their
    /// transient result slot — flushed opportunistically every frame once `directory_client` exists
    /// (contract §C6: "commands queue in the shell (bounded, in-memory) and flush on reconnect").
    /// Each entry keeps its exact request bytes, so a retry is the SAME command to the hub's
    /// digest-keyed idempotency store and can never mint a second invitation.
    #[cfg(not(target_arch = "wasm32"))]
    pub directory_commands: NativeDirectoryCommandQueueV1,
    /// 🏛️ The one retained native space-administration operation (fixed capacity: exactly one).
    /// Its page, receipt, and one-shot invite capability are erased by every terminal transition, so
    /// an identity change, a 401/403, or a scoped 4401 leaves nothing administrable behind.
    #[cfg(not(target_arch = "wasm32"))]
    pub space_administration: Option<ShellSpaceAdministrationOperationV1>,
    /// 🔢️ Monotonic operation epoch; a late turn for a replaced operation is dropped, never applied.
    #[cfg(not(target_arch = "wasm32"))]
    pub space_administration_epoch: u64,
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
    /// `🟦️Interpreter` pipeline's `UiEvent::TextInput` is deliberately bypassed here, same as
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
fn decode_panel_layout_field(raw: &str) -> Option<PanelLayoutPersisted> {
    if raw.len() > SHELL_CHROME_IO_FIELD_BYTES {
        return None;
    }
    let mut fields = raw.splitn(7, '\t');
    let left_panel_open = fields.next()?.parse::<u8>().ok()? == 1;
    let right_panel_open = fields.next()?.parse::<u8>().ok()? == 1;
    let active_left_kind = match fields.next()? {
        "" => None,
        value => Some(value.to_string()),
    };
    let active_right_kind = match fields.next()? {
        "" => None,
        value => Some(value.to_string()),
    };
    let left_panel_width = match fields.next()? {
        "" => None,
        value => Some(value.parse::<f32>().ok()?),
    };
    let right_panel_width = match fields.next()? {
        "" => None,
        value => Some(value.parse::<f32>().ok()?),
    };
    if fields.next().is_some() {
        return None;
    }
    Some(PanelLayoutPersisted { left_panel_open, right_panel_open, active_left_kind, active_right_kind, left_panel_width, right_panel_width })
}

fn encode_panel_layout_field(layout: &PanelLayoutPersisted) -> Option<String> {
    let left_kind = layout.active_left_kind.as_deref().unwrap_or("");
    let right_kind = layout.active_right_kind.as_deref().unwrap_or("");
    if left_kind.contains('\t') || right_kind.contains('\t') {
        return None;
    }
    let encoded = format!(
        "{}\t{}\t{}\t{}\t{}\t{}",
        u8::from(layout.left_panel_open),
        u8::from(layout.right_panel_open),
        left_kind,
        right_kind,
        layout.left_panel_width.map(|value| value.to_string()).unwrap_or_default(),
        layout.right_panel_width.map(|value| value.to_string()).unwrap_or_default()
    );
    (encoded.len() <= SHELL_CHROME_IO_FIELD_BYTES).then_some(encoded)
}

fn load_panel_layout_from_store() -> Option<PanelLayoutPersisted> {
    prefs_get(PANEL_LAYOUT_STORAGE_KEY).and_then(|raw| decode_panel_layout_field(&raw))
}

fn save_panel_layout_to_store(layout: &PanelLayoutPersisted) {
    if let Some(encoded) = encode_panel_layout_field(layout) {
        prefs_set(PANEL_LAYOUT_STORAGE_KEY, &encoded);
    }
}

//#endregion 🧭️PanelAnchorModel

impl ShellState {
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(test)]
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
        // renderer crate shares (also used by `🦀️.rs::kernel_runtime`'s `ParallelRuntime` for
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
        #[cfg(not(target_arch = "wasm32"))]
        let document_host = {
            let host = ArtifactHost::new(std::sync::Arc::new(crate::renderer_worker_pool()));
            if let Some(credential) = claimed_local_hub_credential("native") {
                host.set_local_hub_credential(credential);
            }
            host
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
            find_items: ShellFindItems::default(),
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
            document_host,
            #[cfg(not(target_arch = "wasm32"))]
            sync_channel: None,
            #[cfg(not(target_arch = "wasm32"))]
            sync_status: None,
            #[cfg(not(target_arch = "wasm32"))]
            sync_bootstrap_progress: None,
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
            inference_port: None,
            #[cfg(not(target_arch = "wasm32"))]
            inference_port_status: None,
            #[cfg(not(target_arch = "wasm32"))]
            document_execution_target_lease: None,
            #[cfg(not(target_arch = "wasm32"))]
            directory_home: None,
            #[cfg(not(target_arch = "wasm32"))]
            directory_transport,
            #[cfg(not(target_arch = "wasm32"))]
            directory_cancel,
            #[cfg(not(target_arch = "wasm32"))]
            directory_commands: NativeDirectoryCommandQueueV1::default(),
            #[cfg(not(target_arch = "wasm32"))]
            space_administration: None,
            #[cfg(not(target_arch = "wasm32"))]
            space_administration_epoch: 0,
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
        #[cfg(not(target_arch = "wasm32"))]
        {
            let retained_home = self.directory_home.as_ref().map(|home| (home.plugin_id.clone(), home.instance_id));
            if let Some(mut home) = self.directory_home.take() {
                if let Some((plugin_id, instance_id)) = home.take_destroy_authority() {
                    if let Some(program) = self.plugins.iter().find(|entry| entry.plugin_id == plugin_id) {
                        program.destroy_app(instance_id);
                    }
                }
            }
            if let Some(session) = self.session.take() {
                if retained_home.as_ref().is_none_or(|(plugin_id, instance_id)| plugin_id != &session.plugin_id || *instance_id != session.instance_id) {
                    if let Some(program) = self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id) {
                        program.destroy_app(session.instance_id);
                    }
                }
            }
        }
        #[cfg(target_arch = "wasm32")]
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
            let session = ActiveSession { plugin_id: semio_s_plugin_space.plugin_id.clone(), instance_id, app: s_app, view_state };
            #[cfg(not(target_arch = "wasm32"))]
            {
                self.directory_home = Some(DirectoryHomeProjection::new(session.plugin_id.clone(), session.instance_id, session.app.clone(), session.view_state.clone()).map_err(|error| error.to_string())?);
            }
            self.session = Some(session);
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

    fn live_view_state(&self, session: &ActiveSession) -> ViewModel {
        let mut view_state = session.view_state.clone();
        view_state.locale = self.active_locale();
        view_state.terminology = self.active_terminology();
        view_state.contributions_json = Some(Self::contributions_json_from_plugins(&self.plugins));
        view_state.window_instances = self
            .dock
            .window_instances()
            .into_iter()
            .map(|(id, window_kind_id)| semio_framework::ViewWindowInstance { id, window_kind_id })
            .collect();
        view_state.active_utility_by_window_id = self.active_utility_by_window.clone();
        view_state
    }

    fn live_window_kind_id<'a>(&'a self, session: &'a ActiveSession, window_id: &str) -> Option<&'a str> {
        self.dock
            .window_kind_id(window_id)
            .or_else(|| session.app.window_kinds.iter().find(|kind| kind.id == window_id).map(|kind| kind.id.as_str()))
    }

    fn context_window_instance_id(&self, x: f32, y: f32) -> Option<&str> {
        self.dock_drop_bodies.iter().find(|(_, bounds, _)| bounds.contains(x, y)).map(|(_, _, window_id)| window_id.as_str())
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
        let view_state = self.live_view_state(&session);
        let live_windows = self.dock.window_instances();
        let mut refresh_effects = Vec::new();
        {
            let program = self.plugins.iter().find(|p| p.plugin_id == session.plugin_id).cloned().ok_or("session program missing")?;
            for (window_id, window_kind_id) in live_windows {
                let kind = session.app.window_kinds.iter().find(|kind| kind.id == window_kind_id).ok_or_else(|| format!("window kind '{}' is absent from the app", window_kind_id))?;
                let window_view = view_state.for_window_instance(&window_id).ok_or_else(|| format!("window '{}' is absent from the live view", window_id))?;
                let document = program.render_with_document(session.instance_id, &window_id, &kind.body_key, &window_view, None, Some(&mut refresh_effects)).await?;
                self.window_ui.insert(window_id, document);
            }
        }
        if let Err(documents) = Self::retain_document_map_for_close(&mut self.closing_documents, std::mem::take(&mut self.panel_documents)) {
            self.panel_documents = documents;
            return Err("shell: panel document retirement registry refused the exact prior owners".to_string());
        }
        let program = self.plugins.iter().find(|p| p.plugin_id == session.plugin_id).cloned().ok_or("session program missing")?;
        let panel_view = view_state.for_panel();
        for tab in Self::flatten_panel_tab_leaves(&session.app.panel_tabs) {
            let Some(body_key) = tab.body_key.as_deref() else { continue };
            let document = program.render_with_document(session.instance_id, tab.id(), body_key, &panel_view, None, Some(&mut refresh_effects)).await?;
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
                    if let Some(spawn_plugin) = self.plugins.iter().find(|p| p.plugin_id == spawned.plugin_id).cloned() {
                        let spawned_app = spawn_plugin.manifest.apps.iter().find(|app| app.id == spawned.app_id).cloned();
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
                                window_id: Some(spawned.id.clone()),
                                window_instances: vec![semio_framework::ViewWindowInstance { id: spawned.id.clone(), window_kind_id: app.window_kinds.first().id.clone() }],
                                active_tool_id: None,
                                active_utility_by_window_id: HashMap::new(),
                            };
                            if let Some(document) = self.spawned_ui.take() {
                                if let Err(document) = self.retain_document_for_close(document) {
                                    self.spawned_ui = Some(document);
                                    return Err("shell: spawned document retirement registry refused the exact prior owner".to_string());
                                }
                            }
                            self.spawned_ui = Some(spawn_plugin.render(spawned.instance_id, &spawned.id, &body_key, &view_state).await?);
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
                // 💡️ Slice D — the host-owned ephemeral inference port. The program named only an
                // intent; this shell resolves the document scope, checks the execution-target lease,
                // and drives the whole lifecycle itself. It never writes the document.
                #[cfg(not(target_arch = "wasm32"))]
                semio_framework::kernel::Effect::RequestInferenceProposal { .. } => {
                    self.open_inference_port();
                }
                _ => {}
            }
        }
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
#[path = "../../🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs"]
mod panel_anchor_model_tests;
//#endregion ShellLifecycle

//#region ShellActions
fn patch_ops_from_action_result(result: &semio_framework::kernel::InvocationResult) -> Vec<String> {
    result.mutations.iter().filter_map(|operation| serde_json::to_string(&operation.diff.payload).ok()).collect()
}

impl ShellState {
    #[cfg(not(target_arch = "wasm32"))]
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
            return Ok(vec![PersistenceBinding::Hub { base_url: format!("http://{host_port}"), space_id: space_id.to_string(), surface: None }]);
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
    /// `ArtifactHost` canonical sequence.
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
            self.document_host.close_key(&channel.document_key);
        }
        self.sync_status = None;
        self.sync_bootstrap_progress = None;
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
                    self.sync_bootstrap_progress = None;
                    if let Some(plugin) = plugin.as_ref() {
                        // 🎠️ H3-wgpu-native — same treatment, `load_app_document_pack` is now async.
                        match plugin.load_app_document_pack(instance_id, &pack, &spr).await {
                            Ok(()) => changed = true,
                            Err(error) => eprintln!("[DEBUG] wgpu shell load_app_document_pack failed: {error}"),
                        }
                    }
                }
                ArtifactEvent::BootstrapProgress { received_bytes, total_bytes, received_chunks, total_chunks } => {
                    self.sync_bootstrap_progress = Some((received_bytes, total_bytes, received_chunks, total_chunks));
                    changed = true;
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



    /// 🚦️ ticket §C5 — the `#s-sync-status` footer pill's text, mirroring the React shell's
    /// `computeSyncPillState`/`syncPillText` three-way vocabulary (`persisted | pending(n) |
    /// remote(connected|connecting|backoff|detached)`) exactly, distinct from `sync_status_label`
    /// above (that one is the manual sync-card's own popover-style summary, unchanged). English-only
    /// — `active_locale()` resolves plugin-declared `LocalizedLabel`s, not shell-owned plain text like
    /// this one (same tier `sync_status_label` is already at); a real bilingual pill needs the same
    /// `Label`/`LocalizedLabel` plumbing gap `📓️w2-b-report.md` already flagged for tree-content
    /// strings, not invented bespoke here — see `📓️w3-a-report.md`'s "what is NOT done".
    #[cfg(not(target_arch = "wasm32"))]
    fn sync_pill_text(status: Option<&ArtifactSyncStatus>, progress: Option<&(u64, u64, u32, u32)>) -> String {
        if let Some((received_bytes, total_bytes, received_chunks, total_chunks)) = progress {
            return format!("Recovering {received_bytes}/{total_bytes} bytes · {received_chunks}/{total_chunks} chunks");
        }
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
    /// `handleCommand` response, per `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`'s `AppFrame::Invocation` decode)
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
    /// History panel's own "Checkpoint" button uses (`history_command` in `🔌️plugin/🦀️.rs`),
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
        let arguments: BTreeMap<String, DslValue> = BTreeMap::from([("id".to_string(), DslValue::String(artifact_id.to_string())), ("nowMs".to_string(), DslValue::float(now_ms)), ("actor".to_string(), DslValue::String(actor.clone()))]);
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
            let command_json = dsl::os_pack::json::to_json_string(&invocation);
            if let Err(error) = program.handle_command(session.instance_id, &command_json, &session.view_state).await {
                eprintln!("[DEBUG] wgpu shell touchArtifact (live session) failed: {error}");
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
        if let Err(error) = bind_wgpu_document_socket_surface(&self.document_host, S_SPACE_INDEX_DOCUMENT_ID, S_SPACE_INDEX_DOCUMENT_SCHEMA, &bindings, &program, &app, &app.window_kinds.first().id) {
            eprintln!("[DEBUG] wgpu shell touchArtifact document admission failed: {error}");
            program.destroy_app(instance_id);
            return;
        }
        let channels = self.document_host.open(ArtifactActorConfig { document_id: S_SPACE_INDEX_DOCUMENT_ID.to_string(), schema: S_SPACE_INDEX_DOCUMENT_SCHEMA.to_string(), bindings, watch_external: true, actor: actor.clone() }).await;
        if let Err(error) = program.attach_backbone(instance_id, &actor_uri) {
            eprintln!("[DEBUG] wgpu shell touchArtifact attach_backbone failed: {error}");
            self.document_host.close_key(&channels.document_key);
            program.destroy_app(instance_id);
            return;
        }
        let _ = channels.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() });
        let invocation = semio_framework::manifest::CommandInvocation {
            address: semio_framework::manifest::CommandAddress { owner: semio_framework::manifest::CommandOwnerAddress::App { plugin_id: program.plugin_id.clone(), app_id: app.id.clone() }, command_id: "touchArtifact".into() },
            arguments,
        };
        let command_json = dsl::os_pack::json::to_json_string(&invocation);
        let view_state = ViewModel { locale: self.active_locale(), terminology: self.active_terminology(), ..Default::default() };
        if let Err(error) = program.handle_command(instance_id, &command_json, &view_state).await {
            eprintln!("[DEBUG] wgpu shell touchArtifact dispatch failed: {error}");
        }
        self.document_host.close_key(&channels.document_key);
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
    /// `ArtifactHost` canonical sequence (open → subscribe → register host channel → program
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
            let schema = session.app.io.document_schema.clone();
            let bindings = Self::parse_persistence_binding(&uri)?;
            let window_id = self.active_window_id.as_deref().or(session.view_state.window_id.as_deref()).unwrap_or_else(|| session.app.window_kinds.first().id.as_str());
            let window_kind_id = self.live_window_kind_id(&session, window_id).unwrap_or_else(|| session.app.window_kinds.first().id.as_str()).to_string();
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
            bind_wgpu_document_socket_surface(&self.document_host, &document_id, &schema, &bindings, &plugin, &session.app, &window_kind_id)?;
            let channels = self.document_host.open(ArtifactActorConfig { document_id: document_id.clone(), schema, bindings, watch_external: true, actor }).await;
            let events = self.document_host.subscribe_key(&channels.document_key).await;
            plugin.attach_backbone(session.instance_id, &actor_uri).map_err(|error| format!("plugin attach backbone: {error}"))?;
            let cmd_tx = channels.cmd_tx.clone();
            let _ = cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() });
            self.sync_channel = Some(ShellSyncChannel { document_id, document_key: channels.document_key, actor_uri, instance_id: session.instance_id, plugin_id: session.plugin_id.clone(), cmd_tx, events, connected_at_ms: chrome_now_ms() as i64 });
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
        let window_id = self.active_window_id.as_deref().or(session.view_state.window_id.as_deref()).unwrap_or_else(|| session.app.window_kinds.first().id.as_str());
        let window_kind_id = self.live_window_kind_id(&session, window_id).unwrap_or_else(|| session.app.window_kinds.first().id.as_str()).to_string();
        // 🎠️ H3-wgpu-native — `wasm_runtime()`/`register_host_backbone` retired, see
        // `detach_sync_backbone_internal`'s note.
        // 📌️ ticket §C5 item 4 — checkpoint-on-close, same "switch away IS close" posture as
        // `attach_sync_backbone` above (this shell keeps exactly one document mounted at a time).
        self.checkpoint_before_detach().await;
        self.detach_sync_backbone_internal();
        self.presence_surface = surface;
        let actor_uri = format!("actor://{document_id}");
        let actor = self.current_shell_actor(session.instance_id);
        bind_wgpu_document_socket_surface(&self.document_host, &document_id, &schema, &bindings, &plugin, &session.app, &window_kind_id)?;
        let channels = self.document_host.open(ArtifactActorConfig { document_id: document_id.clone(), schema, bindings, watch_external: true, actor }).await;
        let events = self.document_host.subscribe_key(&channels.document_key).await;
        plugin.attach_backbone(session.instance_id, &actor_uri).map_err(|error| format!("plugin attach backbone: {error}"))?;
        let cmd_tx = channels.cmd_tx.clone();
        let _ = cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() });
        self.sync_backbone_uri = Some(actor_uri.clone());
        self.sync_channel = Some(ShellSyncChannel { document_id, document_key: channels.document_key, actor_uri, instance_id: session.instance_id, plugin_id: session.plugin_id.clone(), cmd_tx, events, connected_at_ms: chrome_now_ms() as i64 });
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
        // 🎬️ Tutorial playback remains shell-local. Recording is armed only after the plugin's shared
        // retained `recordTutorial` route accepts and commits below.
        if action.action == semio_framework::START_TUTORIAL_ACTION_ID {
            if let Some(tutorial_id) = action.args.as_ref().and_then(|args| args.get("tutorialId")).and_then(|v| v.as_str()) {
                self.tutorial_start(tutorial_id);
            }
            return Ok(());
        }
        let record_tutorial_after_acceptance = action.action == semio_framework::RECORD_TUTORIAL_ACTION_ID;
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
                        let window_id = action
                            .args
                            .as_ref()
                            .and_then(|args| args.get("windowId"))
                            .and_then(|value| value.as_str())
                            .map(String::from)
                            .or_else(|| self.active_window_id.clone())
                            .unwrap_or_else(|| self.active_utility_bar_window_kind(&session).id.clone());
                        self.apply_set_active_utility(&window_id, utility_id);
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
        let live_view_state = self.live_view_state(&session);
        let window_kind_id = live_view_state
            .window_instances
            .iter()
            .find(|instance| instance.id == window_instance_id)
            .map(|instance| instance.window_kind_id.clone())
            .or_else(|| session.app.window_kinds.iter().find(|kind| kind.id == window_instance_id).map(|kind| kind.id.clone()))
            .ok_or_else(|| format!("action window instance {window_instance_id} has no declared kind"))?;
        let mode_id = session.view_state.active_mode_id.clone().unwrap_or_else(|| session.app.default_mode_id.clone());
        let arguments = action.args.as_ref().and_then(DslValue::as_object).map(|entries| entries.iter().cloned().collect()).unwrap_or_default();
        let invocation = semio_framework::manifest::ActionInvocation {
            address: semio_framework::manifest::ActionAddress { plugin_id: session.plugin_id.clone(), app_id: session.app.id.clone(), mode_id, window_kind_id, window_instance_id, action_id: action.action.clone() },
            arguments,
        };
        let action_json = dsl::os_pack::json::to_json_string(&invocation);
        let result = program.handle_action(session.instance_id, &action_json, &live_view_state).await?;
        // 🧾️ ticket §C5 — fold this dispatch's own `history_patch` into the check-in projection (idle
        // clock, checkpoint-landed detection + `TouchArtifact`) before anything else touches `self`.
        #[cfg(not(target_arch = "wasm32"))]
        self.observe_invocation_history(result.history_patch.as_ref()).await;
        if record_tutorial_after_acceptance {
            self.tutorial_start_recording();
        }
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
                // 💡️ Slice D — same host-owned inference port funnel as `queue_host_effects`.
                #[cfg(not(target_arch = "wasm32"))]
                semio_framework::kernel::Effect::RequestInferenceProposal { .. } => {
                    self.open_inference_port();
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
        let command_json = dsl::os_pack::json::to_json_string(&invocation);
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
                // 💡️ Slice D — a command-boundary emission opens the same one host-owned port.
                #[cfg(not(target_arch = "wasm32"))]
                semio_framework::kernel::Effect::RequestInferenceProposal { .. } => {
                    self.open_inference_port();
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
        if action_id == "os.directory.open-administration" {
            let space_id = args_json.as_ref().and_then(|args| args.get("spaceId")).and_then(Value::as_str).unwrap_or_default().to_string();
            self.open_space_administration(&space_id);
            return;
        }
        if let Some(command) = directory_command_from_action(action_id, args_json.as_ref()) {
            self.dispatch_directory_command(command).await;
            return;
        }
        if action_id == "os.open-artifact" || action_id == "os.open-artifact-with" {
            self.handle_open_artifact_relay(action_id, args_json.as_ref()).await;
        }
    }

    /// 🏛️ Installs the one retained administration operation for exactly one space, retiring any
    /// predecessor first. Nothing is administrable until the hub's own canonical page has landed.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn open_space_administration(&mut self, space_id: &str) {
        if space_id.trim().is_empty() || self.identity.is_none() {
            return;
        }
        self.close_space_administration();
        self.space_administration_epoch = self.space_administration_epoch.saturating_add(1);
        self.space_administration = Some(ShellSpaceAdministrationOperationV1::open(self.space_administration_epoch, space_id));
    }

    /// 🧯️ Retires the operation: page, receipt, and capability are erased before anything observes it.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn close_space_administration(&mut self) {
        if let Some(operation) = self.space_administration.as_mut() {
            operation.terminate(ShellSpaceAdministrationPhaseV1::Cancelled, Some(DirectoryCommandErrorCodeV1::Cancelled));
        }
        self.space_administration = None;
    }

    /// 🎁️ Hands the one-shot invite capability to the caller exactly once, erasing it in the same turn.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn acknowledge_space_administration_capability(&mut self) -> Option<String> {
        self.space_administration.as_mut().and_then(ShellSpaceAdministrationOperationV1::acknowledge_capability)
    }

    /// 🎬️ Drives exactly one bounded administration turn against the real `DirectoryClient`. Returns
    /// `true` while the operation still has work, so the frame loop can pump it without spinning.
    /// A mutation is never retried: an indeterminate transport settles `Failed`, and only an exact
    /// server receipt advances the operation, always followed by a mandatory page refresh.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn pump_space_administration(&mut self) -> bool {
        let Some(client) = self.directory_client.clone() else { return false };
        let epoch = self.space_administration_epoch;
        let Some(turn) = self.space_administration.as_mut().map(ShellSpaceAdministrationOperationV1::turn) else { return false };
        match turn {
            ShellSpaceAdministrationTurnV1::Idle => false,
            ShellSpaceAdministrationTurnV1::Closed => {
                self.space_administration = None;
                false
            }
            ShellSpaceAdministrationTurnV1::FetchPage { cursor } => {
                let space_id = self.space_administration.as_ref().map(|operation| operation.space_id().to_string()).unwrap_or_default();
                let result = client.space_administration_page(&self.directory_command_ctx(), &space_id, cursor.as_deref()).await;
                let Some(operation) = self.space_administration.as_mut().filter(|operation| operation.operation_epoch() == epoch) else { return false };
                match result {
                    Ok(page) => {
                        let canonical = page.canonical_json().to_string();
                        if !operation.apply_page(canonical, page.page().clone()) {
                            operation.terminate(ShellSpaceAdministrationPhaseV1::Failed, Some(DirectoryCommandErrorCodeV1::Invalid));
                        }
                    }
                    Err(error) => operation.fail_page(&error),
                }
                true
            }
            ShellSpaceAdministrationTurnV1::Submit { request } => {
                let result = client.command(&self.directory_command_ctx(), &request).await;
                let Some(operation) = self.space_administration.as_mut().filter(|operation| operation.operation_epoch() == epoch) else { return false };
                match result {
                    Ok(canonical) => {
                        if !operation.apply_receipt(&canonical.receipt) {
                            operation.terminate(ShellSpaceAdministrationPhaseV1::Failed, Some(DirectoryCommandErrorCodeV1::Invalid));
                        }
                    }
                    Err(code) => operation.fail_command(code),
                }
                true
            }
        }
    }

    /// 📂️ Opens the relay's exact `documentId`/`schema` pair with `spaceId` pinning
    /// `open_space_id` first so default binding computation sees it. An app-only relay is a no-op.
    #[cfg(not(target_arch = "wasm32"))]
    async fn handle_open_artifact_relay(&mut self, action_id: &str, args: Option<&Value>) {
        let target = match open_artifact_relay_target(action_id, args) {
            Ok(target) => target,
            Err(error) => {
                eprintln!("[DEBUG] wgpu shell os.open-artifact relay rejected: {error}");
                return;
            }
        };
        if let Some(space_id) = target.space_id {
            self.open_space_id = Some(space_id);
        }
        let (Some(document_id), Some(schema)) = (target.document_id, target.schema) else {
            return;
        };
        let (bindings, surface) = self.default_bindings_for_current_session();
        if let Err(error) = self.open_document(document_id, schema, bindings, surface).await {
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

    /// 🛑️ One command-scoped context: a fresh `directory_cancel` child plus a finite deadline, so a
    /// hung hub can never retain a command turn forever. Cancelling the wait does NOT cancel a
    /// command already past its server linearization point — the operation becomes indeterminate and
    /// keeps its request id for an explicit later resolution, never a silently fresh one.
    #[cfg(not(target_arch = "wasm32"))]
    fn directory_command_ctx(&self) -> OperationContext {
        let mut context = self.directory_ctx();
        context.deadline_ms = Some(crate::renderer_worker_pool().now_ms().saturating_add(DIRECTORY_COMMAND_DEADLINE_MS));
        context
    }

    //#region 💡️InferencePort
    /// 🪪️ The native precondition: a port may only start while this shell retains a VERIFIED, live
    /// execution-target lease for the open document. Native document opening currently retains only
    /// a canonical surface-id preference — `document_socket_surface_from_descriptor` was deliberately
    /// downgraded from a forgeable partial authority to exactly that by the execution-target-lease
    /// lane — so no native path can mint `DocumentExecutionTargetLeaseFieldsV1` yet and this answers
    /// `false`. That is an honest refusal, not a stub: the port reports a localized terminal instead
    /// of running against an unverified target. It becomes real when the native lease lands.
    #[cfg(not(target_arch = "wasm32"))]
    fn execution_target_lease_verified(&self) -> bool {
        self.document_execution_target_lease.is_some()
    }

    /// 🗺️ The exact hub scope of the one open document, or `None` when nothing hub-scoped is open.
    #[cfg(not(target_arch = "wasm32"))]
    fn inference_document_scope(&self) -> Option<DocumentScope> {
        let space_id = self.open_space_id.clone()?;
        let document_id = self.sync_channel.as_ref()?.document_id.clone();
        Some(DocumentScope { space_id, document_id })
    }

    /// 💡️ Opens the one retained port, replacing any predecessor. A missing scope, identity, client
    /// or verified lease publishes a localized terminal instead of starting anything.
    #[cfg(not(target_arch = "wasm32"))]
    fn open_inference_port(&mut self) {
        self.cancel_inference_port();
        let Some(scope) = self.inference_document_scope() else {
            self.inference_port_status = Some(GisMapInferencePortStatusV1 { phase: GisMapInferencePortPhaseV1::Failed, code: Some(GisMapInferencePortCodeV1::NotFound), ..GisMapInferencePortStatusV1::default() });
            return;
        };
        let mut driver = GisMapInferenceDriverV1::new(scope, self.execution_target_lease_verified());
        driver.intend(GisMapInferenceIntentV1::Propose);
        let Some(client) = self.directory_client.clone().filter(|_| self.identity.is_some()) else {
            let status = reduce_gis_map_inference_port_v1(driver.status(), &GisMapInferencePortEventV1::Failed(GisMapInferencePortCodeV1::Unavailable));
            self.inference_port_status = Some(status);
            return;
        };
        if !self.execution_target_lease_verified() {
            let status = reduce_gis_map_inference_port_v1(driver.status(), &GisMapInferencePortEventV1::LeaseUnverified);
            self.inference_port_status = Some(status);
            return;
        }
        let context = self.directory_ctx();
        self.inference_port_status = Some(driver.status().clone());
        self.inference_port = Some(ShellInferenceRunner::start(crate::renderer_worker_pool(), client, context, driver));
    }

    /// 🔄️ One bounded drain per frame; the runner never spins and never blocks the render loop.
    #[cfg(not(target_arch = "wasm32"))]
    fn pump_inference_port(&mut self) -> bool {
        let Some(runner) = self.inference_port.clone() else { return false };
        let statuses = runner.drain();
        let Some(latest) = statuses.into_iter().next_back() else { return false };
        let terminal = latest.phase.terminal();
        self.inference_port_status = Some(latest);
        if terminal {
            runner.cancel();
            self.inference_port = None;
        }
        true
    }

    /// 🛑️ Asks the retained port to cancel. The phase does NOT move here: only the server's own next
    /// answer may report `cancelled`, so a hub that refuses a cancel is never misreported as honouring it.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn cancel_inference_proposal(&mut self) {
        if let Some(runner) = self.inference_port.as_ref() {
            runner.intend(GisMapInferenceIntentV1::Cancel);
        }
    }

    /// ✅️ Approves exactly the offered proposal, echoing back the hash the server itself published.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn approve_inference_proposal(&mut self) {
        if let Some(runner) = self.inference_port.as_ref() {
            runner.intend(GisMapInferenceIntentV1::Approve);
        }
    }

    /// 🛑️ Hard terminal: cancels every in-flight call and retires the runner.
    #[cfg(not(target_arch = "wasm32"))]
    fn cancel_inference_port(&mut self) {
        if let Some(runner) = self.inference_port.take() {
            runner.cancel();
        }
    }
    //#endregion 💡️InferencePort

    /// 📇️ ticket §4/§C6 — seals one `os.directory.*` command under a fresh idempotency correlation
    /// and issues it against the hub, dropping it when no identity is signed in at all, else leaving
    /// it queued on a transient failure (the "same offline queueing behaviour as the React side" the
    /// brief asks for — React's worker queues in-memory and flushes on reconnect; this shell's
    /// `directory_commands` + `flush_pending_directory_commands` (called every frame from
    /// `pump_directory_events`) is the native twin of that policy).
    #[cfg(not(target_arch = "wasm32"))]
    async fn dispatch_directory_command(&mut self, command: DirectoryCommand) {
        if self.identity.is_none() {
            return;
        }
        self.directory_commands.admit_first(DirectoryCommandRequestV1::new(mint_directory_command_request_id(), command));
        self.flush_pending_directory_commands().await;
    }

    /// ♻️ Drives the FIFO in order: a transient fault retains the head and its byte-identical sealed
    /// request and stops the queue; a terminal auth/conflict/validation failure produces a result and
    /// lets the queue proceed, so one forbidden command can never wedge every later one.
    #[cfg(not(target_arch = "wasm32"))]
    async fn flush_pending_directory_commands(&mut self) {
        let Some(client) = self.directory_client.clone() else { return };
        while let Some(request) = self.directory_commands.head().cloned() {
            let outcome = client.command(&self.directory_command_ctx(), &request).await.map(|canonical| canonical.receipt);
            if !self.directory_commands.settle(outcome) {
                break;
            }
        }
    }

    /// 🏠️ Begins one bounded canonical-page fetch owned by the retained Home bootstrap epoch.
    #[cfg(not(target_arch = "wasm32"))]
    fn start_directory_event_page_fetch(&mut self) {
        if self.identity_offline {
            return;
        }
        let Some(client) = self.directory_client.clone() else { return };
        let pool = crate::renderer_worker_pool();
        let now_ms = pool.now_ms();
        let Some((epoch, after)) = self
            .directory_home
            .as_ref()
            .and_then(|home| (!home.closed && home.page_task.is_none() && home.home_task.is_none() && home.stream.is_none() && now_ms >= home.retry_at_ms).then_some((home.bootstrap.bootstrap_epoch(), home.bootstrap.after())))
        else {
            return;
        };
        let mut context = self.directory_ctx();
        context.deadline_ms = Some(now_ms.saturating_add(5_000));
        let cancel = context.cancel.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        let task = ShellPoolFuture::spawn(pool, Lane::Io, async move {
            let result = client.event_page(&context, after).await;
            let _ = tx.send((epoch, result));
        });
        if let Some(home) = self.directory_home.as_mut().filter(|home| home.bootstrap.bootstrap_epoch() == epoch && !home.closed) {
            home.page_cancel = Some(cancel);
            home.page_rx = Some(rx);
            home.page_task = Some(task);
        } else {
            cancel.cancel_now();
            task.cancel();
        }
    }

    /// 🧾️ Invokes the stored Home instance and accepts only its terminal Config publication receipt.
    #[cfg(not(target_arch = "wasm32"))]
    fn start_directory_home_publication(&mut self, page: CanonicalDirectoryEventPageV1) -> Result<(), String> {
        let canonical_json = page.canonical_json().to_string();
        let (epoch, instance_id, program, app, view_state, expected) = {
            let home = self.directory_home.as_mut().ok_or("retained Home projection is unavailable")?;
            let expected = home.bootstrap.present(page).map_err(|error| error.to_string())?;
            let view_state = self.session.as_ref().filter(|session| home.is_instance(&session.plugin_id, session.instance_id)).map(|session| session.view_state.clone()).unwrap_or_else(|| home.view_state.clone());
            let program = self.plugins.iter().find(|entry| entry.plugin_id == home.plugin_id).cloned().ok_or("retained Home program is unavailable")?;
            (home.bootstrap.bootstrap_epoch(), home.instance_id, program, home.app.clone(), view_state, expected)
        };
        let window_kind_id = app.window_kinds.first().id.clone();
        let window_instance_id = view_state.window_id.clone().unwrap_or_else(|| window_kind_id.clone());
        let invocation = semio_framework::manifest::ActionInvocation {
            address: semio_framework::manifest::ActionAddress {
                plugin_id: program.plugin_id.clone(),
                app_id: app.id.clone(),
                mode_id: view_state.active_mode_id.clone().unwrap_or_else(|| app.default_mode_id.clone()),
                window_kind_id,
                window_instance_id,
                action_id: "applyDirectoryEventPage".into(),
            },
            arguments: BTreeMap::from([("pageJson".into(), DslValue::String(canonical_json))]),
        };
        let action_json = dsl::os_pack::json::to_json_string(&invocation);
        let (tx, rx) = std::sync::mpsc::channel();
        let expected_for_task = expected.clone();
        let pool = crate::renderer_worker_pool();
        let task = ShellPoolFuture::spawn(pool, Lane::Io, async move {
            let outcome = match program.handle_action(instance_id, &action_json, &view_state).await {
                Err(error) => ShellDirectoryHomePublicationOutcome::Rejected(error),
                Ok(result) => match terminal_directory_home_ack(&result, epoch) {
                    Ok(actual) if actual == expected_for_task => ShellDirectoryHomePublicationOutcome::Published(actual),
                    Ok(_) => ShellDirectoryHomePublicationOutcome::Terminal("retained Home directory acknowledgement does not match the pending page".into()),
                    Err(error) => ShellDirectoryHomePublicationOutcome::Terminal(error),
                },
            };
            let _ = tx.send((epoch, instance_id, expected_for_task, outcome));
        });
        let home = self.directory_home.as_mut().ok_or("retained Home projection retired before publication admission")?;
        if home.bootstrap.bootstrap_epoch() != epoch || home.instance_id != instance_id || home.closed {
            task.cancel();
            return Err("retained Home projection changed before publication admission".into());
        }
        home.home_rx = Some(rx);
        home.home_task = Some(task);
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn poll_directory_page_result(&mut self) -> Option<ShellDirectoryPageResult> {
        let home = self.directory_home.as_mut()?;
        let result = match home.page_rx.as_ref()?.try_recv() {
            Ok(result) => Some(result),
            Err(std::sync::mpsc::TryRecvError::Empty) => None,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => Some((home.bootstrap.bootstrap_epoch(), Err(DirectoryClientError::Transport(TransportError::Io("directory page task disconnected".into()))))),
        }?;
        home.page_rx.take();
        home.page_cancel.take();
        home.page_task.take();
        Some(result)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn poll_directory_home_publication(&mut self) -> Option<ShellDirectoryHomePublicationResult> {
        let home = self.directory_home.as_mut()?;
        let result = match home.home_rx.as_ref()?.try_recv() {
            Ok(result) => Some(result),
            Err(std::sync::mpsc::TryRecvError::Empty) => None,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => Some((
                home.bootstrap.bootstrap_epoch(),
                home.instance_id,
                DirectoryEventPageAckV1 { bootstrap_epoch: home.bootstrap.bootstrap_epoch(), session_binding_sha256: String::new(), authorization_generation: 0, through_seq_inclusive: home.bootstrap.after(), receipt_sha256: String::new() },
                ShellDirectoryHomePublicationOutcome::Terminal("retained Home publication task disconnected".into()),
            )),
        }?;
        home.home_rx.take();
        home.home_task.take();
        Some(result)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn open_acknowledged_directory_stream(&mut self, since: u64) -> Result<(), String> {
        let client = self.directory_client.clone().ok_or("directory client is unavailable")?;
        let stream = client.stream_acknowledged(since).map_err(|error| error.to_string())?;
        let context = self.directory_ctx();
        let runner = ShellDirectoryRunner::start(crate::renderer_worker_pool(), stream, context);
        let home = self.directory_home.as_mut().ok_or("retained Home projection is unavailable")?;
        if home.closed || home.stream.is_some() {
            runner.cancel();
            return Err("retained Home stream admission raced with retirement".into());
        }
        home.stream = Some(runner);
        Ok(())
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
            let outcome = restore_claimed(&ctx, transport, "native").await.map_err(|error| error.to_string());
            let _ = tx.send(outcome);
        }));
        self.identity_bootstrap_rx = Some(rx);
    }

    /// 🪪️ Drains `identity_bootstrap_rx` and starts a fresh zero-frontier retained-Home epoch.
    #[cfg(not(target_arch = "wasm32"))]
    fn poll_identity_bootstrap(&mut self) {
        let Some(rx) = self.identity_bootstrap_rx.as_ref() else { return };
        match rx.try_recv() {
            Ok(Ok(outcome)) => {
                self.identity_bootstrap_task.take();
                self.identity_offline = outcome.status == IdentityStatus::Offline;
                self.identity = Some(outcome.identity.clone());
                let client = std::sync::Arc::new(DirectoryClient::authenticated(self.directory_transport.clone(), outcome.credential.clone()));
                self.document_host.set_local_hub_credential(outcome.credential);
                self.document_host.set_hub_socket_grant_source(client.clone());
                self.directory_client = Some(client);
                if let Some(home) = self.directory_home.as_mut() {
                    if let Err(error) = home.begin_epoch(0) {
                        home.close();
                        self.error = Some(error.to_string());
                    }
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

    /// 📡️ Advances the finite fetch → terminal Home receipt → exact ACK → acknowledged-live owner.
    #[cfg(not(target_arch = "wasm32"))]
    async fn pump_directory_events(&mut self) -> bool {
        self.poll_identity_bootstrap();
        let inference_changed = self.pump_inference_port();
        // 🏛️ One bounded administration turn per frame: the retained operation never spins, because
        // `pump_space_administration` answers `false` the moment it reaches `Idle` or a terminal phase.
        let mut changed = self.pump_space_administration().await || inference_changed;
        let runner = self.directory_home.as_ref().and_then(|home| home.stream.clone());
        if let Some(runner) = runner {
            if runner.take_terminal() {
                if let Some(home) = self.directory_home.as_mut() {
                    let _ = home.begin_epoch(0);
                }
                self.directory_client = None;
                self.identity = None;
                self.bootstrap_identity();
                changed = true;
            }
            let mut dirty = false;
            let mut rebootstrap = false;
            for message in runner.drain() {
                match message {
                    DirectoryStreamMessage::Event { .. } | DirectoryStreamMessage::Heartbeat { .. } => dirty = true,
                    DirectoryStreamMessage::Connection { .. } | DirectoryStreamMessage::Presence { .. } => {}
                    DirectoryStreamMessage::RebootstrapRequired { .. } => rebootstrap = true,
                }
            }
            if dirty || rebootstrap {
                if let Some(home) = self.directory_home.as_mut() {
                    match home.wake(rebootstrap) {
                        Ok(Some(_)) => changed = true,
                        Ok(None) => {}
                        Err(error) => {
                            home.close();
                            self.error = Some(error.to_string());
                        }
                    }
                }
            }
        }

        if let Some((epoch, result)) = self.poll_directory_page_result() {
            let current = self.directory_home.as_ref().is_some_and(|home| !home.closed && home.bootstrap.bootstrap_epoch() == epoch);
            if current {
                match result {
                    Ok(page) => {
                        if let Err(error) = self.start_directory_home_publication(page) {
                            if let Some(home) = self.directory_home.as_mut() {
                                home.close();
                            }
                            self.error = Some(error);
                        }
                    }
                    Err(DirectoryClientError::Unauthorized) => {
                        if let Some(home) = self.directory_home.as_mut() {
                            let _ = home.begin_epoch(0);
                        }
                        self.directory_client = None;
                        self.identity = None;
                        self.bootstrap_identity();
                    }
                    Err(DirectoryClientError::Transport(TransportError::Io(_)) | DirectoryClientError::Transport(TransportError::DeadlineExceeded) | DirectoryClientError::Http { status: 500..=599, .. }) => {
                        if let Some(home) = self.directory_home.as_mut() {
                            home.retry_at_ms = crate::renderer_worker_pool().now_ms().saturating_add(DirectoryHomeProjection::RETRY_DELAY_MS);
                        }
                    }
                    Err(DirectoryClientError::Cancelled | DirectoryClientError::Transport(TransportError::Cancelled)) => {}
                    Err(_) => {
                        if let Some(home) = self.directory_home.as_mut() {
                            home.close();
                        }
                        self.error = Some("directory event-page response was terminally rejected".into());
                    }
                }
                changed = true;
            }
        }

        if let Some((epoch, instance_id, expected, outcome)) = self.poll_directory_home_publication() {
            let current = self.directory_home.as_ref().is_some_and(|home| !home.closed && home.bootstrap.bootstrap_epoch() == epoch && home.instance_id == instance_id);
            if current {
                match outcome {
                    ShellDirectoryHomePublicationOutcome::Published(acknowledgement) => {
                        let transition = self.directory_home.as_mut().expect("checked retained Home").bootstrap.acknowledge(&acknowledgement).map_err(|error| error.to_string());
                        match transition {
                            Ok(DirectoryBootstrapTransition::Fetch { .. }) => {}
                            Ok(DirectoryBootstrapTransition::Live { since }) => {
                                if let Err(error) = self.open_acknowledged_directory_stream(since) {
                                    if let Some(home) = self.directory_home.as_mut() {
                                        home.close();
                                    }
                                    self.error = Some(error);
                                }
                            }
                            Err(error) => {
                                if let Some(home) = self.directory_home.as_mut() {
                                    home.close();
                                }
                                self.error = Some(error);
                            }
                        }
                    }
                    ShellDirectoryHomePublicationOutcome::Rejected(error) => {
                        if let Some(home) = self.directory_home.as_mut() {
                            if error.contains("directory-event-page-rebootstrap-required") {
                                let _ = home.begin_epoch(0);
                            } else if home.retry(crate::renderer_worker_pool().now_ms(), &expected.receipt_sha256).is_err() {
                                home.close();
                                self.error = Some("retained Home directory rejection no longer matches the pending page".into());
                            }
                        }
                    }
                    ShellDirectoryHomePublicationOutcome::Terminal(error) => {
                        if let Some(home) = self.directory_home.as_mut() {
                            home.close();
                        }
                        self.error = Some(error);
                    }
                }
                changed = true;
            }
        }

        self.start_directory_event_page_fetch();
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
            let follow_up_operations: Vec<String> = Vec::new();
            #[cfg(target_arch = "wasm32")]
            let mut follow_up_operations = follow_up_operations;
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
        #[cfg(not(target_arch = "wasm32"))]
        if app_id == cfg.landing_app_id {
            if let Some(home) = self.directory_home.as_mut().filter(|home| home.plugin_id == semio_s_plugin_space.plugin_id && home.app.id == app_id) {
                if let Some(next_view_state) = view_state {
                    home.view_state = next_view_state;
                }
                let session = home.active_session();
                self.active_window_id = Some(session.app.window_kinds.first().id.clone());
                self.open_space_id = None;
                self.session = Some(session);
                return self.refresh_ui().await;
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let (Some(home), Some(current)) = (self.directory_home.as_mut(), self.session.as_ref()) {
            if home.is_instance(&current.plugin_id, current.instance_id) {
                home.view_state = current.view_state.clone();
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
        if let (Some(home), Some(current)) = (self.directory_home.as_mut(), self.session.as_ref()) {
            if home.is_instance(&current.plugin_id, current.instance_id) {
                home.view_state = current.view_state.clone();
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
        let space_route = match raw.split_once('/') {
            Some((id, "studio")) => (id.to_string(), true),
            _ => (raw.clone(), false),
        };
        let space_id = space_route.0;
        #[cfg(not(target_arch = "wasm32"))]
        if !space_route.1 {
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
                        let window_kind_id = self.dock.window_kind_id(window_id).unwrap_or(window_id).to_string();
                        let ghost_label = self.session.as_ref().and_then(|s| s.app.window_kinds.iter().find(|k| k.id == window_kind_id).map(|k| k.label.resolve(self.active_terminology(), self.active_locale()).to_string())).unwrap_or_else(|| window_id.to_string());
                        self.begin_pending_dock_drag(DockDragPayload { kind: DockDragKind::Tab, window_id: window_id.to_string(), window_kind_id, source_path: path, tab_index, ghost_label }, x, y);
                        return Ok(());
                    }
                }
                if let Some(path_str_value) = id.strip_prefix("dock.stack.") {
                    let path = parse_path(path_str_value);
                    let windows = self.dock.stack_windows_at_path(&path).unwrap_or_default();
                    let active = windows.iter().find(|wid| self.active_window_id.as_deref() == Some(wid.as_str())).or_else(|| windows.first()).cloned().unwrap_or_default();
                    if !active.is_empty() {
                        let tab_index = self.dock.tab_index(&path, &active).unwrap_or(0);
                        let window_kind_id = self.dock.window_kind_id(&active).unwrap_or(&active).to_string();
                        let ghost_label = self.session.as_ref().and_then(|s| s.app.window_kinds.iter().find(|k| k.id == window_kind_id).map(|k| k.label.resolve(self.active_terminology(), self.active_locale()).to_string())).unwrap_or_else(|| active.clone());
                        self.begin_pending_dock_drag(DockDragPayload { kind: DockDragKind::Stack, window_id: active, window_kind_id, source_path: path, tab_index, ghost_label }, x, y);
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
                    if let Some(action) = self.widget_maps.select_metas.get(select_id).cloned() {
                        self.open_selects.insert(select_id.to_string(), false);
                        self.dispatch_action(ActionDescriptor { controller_id: action.controller_id, action: action.action, args: crate::action_args_json!({ "value": value }) }).await?;
                        return Ok(true);
                    }
                }
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

    fn render_tree_drag_overlay_node(&self, overlay: &mut DrawList, input: &InputState<ActionDescriptor>, theme: &Theme) {
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
        let window_instance_id = self.context_window_instance_id(x, y).map(str::to_string);
        let is_de = self.locale_id == "de";
        let mut items = Vec::new();
        if let Some(session) = self.session.clone() {
            let shortcut_by_action: HashMap<String, String> = session.app.keybindings.iter().map(|binding| (binding.action.action.clone(), binding.keys.clone())).collect();
            let view_state = self.live_view_state(&session);
            let selection: Vec<Value> = Vec::new();
            let text: Option<Value> = None;
            let request = serde_json::json!({
                "menu": { "id": kind.clone() },
                "viewState": view_state,
                "windowInstanceId": window_instance_id,
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
                let window_kind = window_instance_id
                    .as_deref()
                    .and_then(|window_id| self.live_window_kind_id(session, window_id))
                    .and_then(|window_kind_id| session.app.window_kinds.iter().find(|kind| kind.id == window_kind_id));
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
        if let Some(window_id) = window_instance_id.as_deref() {
            scope_context_menu_items(&mut items, window_id);
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
                let invocation: semio_framework::manifest::CommandInvocation = dsl::os_pack::json::from_json_str(command_json).map_err(|error| error.to_string())?;
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
#[path = "../../🧪️tests/🔬️wgpu-shell-input/🦀️.rs"]
mod shell_input_tests;
//#endregion ShellInput

/// ✍️ Immediate-mode Shell chrome text — the non-retained sibling of {@link chrome_text_step}, used by
/// the eleven overlay/dropdown/status painters below that draw a whole scalar in one pass. It was
/// `#[cfg(test)]`-gated, which made every one of those production call sites `E0425` on every target
/// (the reason this crate compiled on neither `wasm32-unknown-unknown` nor natively).
#[cfg(test)]
fn chrome_text(target: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    let mut ctx = framework_widget_context(target, None, atlas, None, input, theme, &mut scroll, &mut collapsed, &mut selects, None);
    draw_text(&mut ctx, text, x, y, size, color);
}

/// ✍️ Advances one Shell text scalar and at most one pre-admitted glyph.
fn chrome_text_step(target: &mut DrawList, atlas: &mut FontAtlas, text: &str, x: f32, y: f32, width: f32, size: f32, color: Rgba, cursor: &mut RetainedGlyphCursor) -> RetainedGlyphStep {
    paint_retained_glyph_step(text, Rect::new(x, y - size, width.max(1.0), size), size, color, atlas, target, cursor)
}

fn chrome_text_complete_step(target: &mut DrawList, atlas: &mut FontAtlas, text: &str, x: f32, y: f32, width: f32, size: f32, color: Rgba, cursor: &mut RetainedGlyphCursor) -> Result<bool, ()> {
    match chrome_text_step(target, atlas, text, x, y, width, size, color, cursor) {
        RetainedGlyphStep::Pending => Ok(false),
        RetainedGlyphStep::Complete => {
            cursor.reset();
            Ok(true)
        }
        RetainedGlyphStep::Fault => Err(()),
    }
}

#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-retained-chrome-text-laws/🦀️.rs"]
mod retained_chrome_text_laws;

fn chrome_icon(draw: &mut DrawList, icons: &IconAtlas, icon_id: &str, x: f32, y: f32, size: f32, color: Rgba) {
    if let Some(uv) = icons.icon_uv(icon_id) {
        draw.push_textured([x, y, size, size], uv, color);
    }
}



#[cfg(test)]
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RetainedChromeGroupStep {
    Pending,
    Complete,
    Fault,
}

/// 📐️ Computes the conservative fixed-cost width admitted for one retained chrome item.
fn retained_chrome_group_item_width(theme: &Theme, item: &ChromeGroupItem<'_>) -> Option<f32> {
    let label_bytes = item.label.map(str::len).unwrap_or(0);
    if label_bytes > ui_wgpu::wgpu::RETAINED_NODE_TEXT_MAX_BYTES {
        return None;
    }
    let icon_w = item.icon_id.map(|_| CHROME_ICON_TINY + theme.gap_standard).unwrap_or(0.0);
    Some(theme.padding_standard * 2.0 + icon_w + label_bytes as f32 * theme.font_size_small * 0.6)
}

/// 🧱️ Emits at most one retained chrome item scalar, glyph, hit, or border per grant.
fn render_retained_chrome_group_item_step(
    group_phase: &mut u8,
    glyph: &mut RetainedGlyphCursor,
    draw: &mut DrawList,
    atlas: &mut FontAtlas,
    icons: &IconAtlas,
    input: &mut InputState<ActionDescriptor>,
    theme: &Theme,
    rect: Rect,
    item: &ChromeGroupItem<'_>,
    register_hit: bool,
) -> RetainedChromeGroupStep {
    let hovered = !item.disabled && rect.contains(input.pointer_x, input.pointer_y);
    let color = if item.disabled { theme.text_muted } else { chrome_item_text(theme, item.active, hovered) };
    match *group_phase {
        0 => {
            let background = if item.disabled { theme.overlay_shadow } else { chrome_item_bg(theme, item.active, hovered) };
            if background.a > 0.0 {
                draw.push_solid([rect.x, rect.y, rect.w, rect.h], background);
            }
            *group_phase = 1;
        }
        1 => {
            if let Some(icon_id) = item.icon_id {
                chrome_icon(draw, icons, icon_id, rect.x + theme.padding_standard, rect.y + (rect.h - CHROME_ICON_TINY) * 0.5, CHROME_ICON_TINY, color);
            }
            *group_phase = 2;
        }
        2 => {
            if let Some(label) = item.label {
                if label.len() > ui_wgpu::wgpu::RETAINED_NODE_TEXT_MAX_BYTES {
                    *group_phase = 0;
                    glyph.reset();
                    return RetainedChromeGroupStep::Fault;
                }
                let icon_w = item.icon_id.map(|_| CHROME_ICON_TINY + theme.gap_standard).unwrap_or(0.0);
                let x = rect.x + theme.padding_standard + icon_w;
                let width = (rect.x + rect.w - theme.padding_standard - x).max(1.0);
                match chrome_text_complete_step(draw, atlas, label, x, rect.y + (rect.h + theme.font_size_small) * 0.5 - 1.0, width, theme.font_size_small, color, glyph) {
                    Ok(false) => return RetainedChromeGroupStep::Pending,
                    Ok(true) => {}
                    Err(()) => {
                        *group_phase = 0;
                        glyph.reset();
                        return RetainedChromeGroupStep::Fault;
                    }
                }
            }
            *group_phase = 3;
        }
        3 => {
            draw.push_solid([rect.x, rect.y, rect.w, theme.stroke_hairline], theme.border_normal);
            *group_phase = 4;
        }
        4 => {
            draw.push_solid([rect.x, rect.y + rect.h - theme.stroke_hairline, rect.w, theme.stroke_hairline], theme.border_normal);
            *group_phase = 5;
        }
        5 => {
            draw.push_solid([rect.x, rect.y, theme.stroke_hairline, rect.h], theme.border_normal);
            *group_phase = 6;
        }
        6 => {
            draw.push_solid([rect.x + rect.w - theme.stroke_hairline, rect.y, theme.stroke_hairline, rect.h], theme.border_normal);
            *group_phase = 7;
        }
        7 => {
            if register_hit && !item.disabled {
                input.register_hit(HitTarget { rect, event: None, control_id: Some(item.control_id.into()), kind: item.kind.clone(), drag_axis: None, drag_data: None });
            }
            *group_phase = 8;
        }
        _ => {
            *group_phase = 0;
            glyph.reset();
            return RetainedChromeGroupStep::Complete;
        }
    }
    RetainedChromeGroupStep::Pending
}

#[cfg(test)]
fn measure_chrome_group_item(atlas: &mut FontAtlas, theme: &Theme, item: &ChromeGroupItem<'_>) -> f32 {
    let icon_w = item.icon_id.map(|_| CHROME_ICON_TINY + theme.gap_standard).unwrap_or(0.0);
    let text_w = item.label.map(|label| atlas.measure_text(label, theme.font_size_small).0).unwrap_or(0.0);
    theme.padding_standard * 2.0 + icon_w + text_w
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







#[cfg(test)]
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
        #[cfg(test)]
        {
            if let Some(label) = item.label {
                chrome_text(draw, atlas, input, theme, label, content_x, item_rect.y + (item_rect.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, text_color);
            }
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

#[cfg(test)]
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
    cursor: &mut ShellChromeChildCursor,
    draw: &mut DrawList,
    atlas: &mut FontAtlas,
    icons: &IconAtlas,
    input: &mut InputState<ActionDescriptor>,
    theme: &Theme,
    status: Option<&ArtifactSyncStatus>,
    progress: Option<&(u64, u64, u32, u32)>,
    session: Option<&ActiveSession>,
    uncommitted_count: u32,
    x: f32,
    btn_y: f32,
    btn_h: f32,
) -> Result<Option<f32>, ()> {
    if cursor.item == 0 {
        if cursor.window.is_none() {
            cursor.window = Some(UiText::try_from_string(ShellState::sync_pill_text(status, progress)).map_err(|_| ())?);
        }
        let Some(label) = cursor.window.as_ref() else { return Err(()) };
        let item = ChromeGroupItem { control_id: "s-sync-status", icon_id: None, label: Some(label.as_str()), active: false, disabled: false, kind: HitKind::Button };
        if cursor.rect.is_none() {
            let width = retained_chrome_group_item_width(theme, &item).ok_or(())?;
            cursor.rect = Some(Rect::new(x, btn_y, width, btn_h));
        }
        let Some(rect) = cursor.rect else { return Err(()) };
        match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, false) {
            RetainedChromeGroupStep::Pending => return Ok(None),
            RetainedChromeGroupStep::Fault => return Err(()),
            RetainedChromeGroupStep::Complete => {}
        }
        input.register_hit(HitTarget { rect, event: None, control_id: Some("s-sync-status".into()), kind: HitKind::Button, drag_axis: None, drag_data: None });
        cursor.x = x + rect.w + theme.gap_standard * 0.5;
        cursor.window = None;
        cursor.rect = None;
        cursor.item = 1;
        return Ok(None);
    }
    if !session.is_some_and(|session| can_check_in(session.app.role)) {
        return Ok(Some(cursor.x));
    }
    if cursor.window.is_none() {
        let label = if uncommitted_count > 0 { format!("Check In ({uncommitted_count})") } else { "Check In".to_string() };
        cursor.window = Some(UiText::try_from_string(label).map_err(|_| ())?);
    }
    let Some(label) = cursor.window.as_ref() else { return Err(()) };
    let item = ChromeGroupItem { control_id: "s-checkin", icon_id: None, label: Some(label.as_str()), active: false, disabled: false, kind: HitKind::Button };
    if cursor.rect.is_none() {
        let width = retained_chrome_group_item_width(theme, &item).ok_or(())?;
        cursor.rect = Some(Rect::new(cursor.x, btn_y, width, btn_h));
    }
    let Some(rect) = cursor.rect else { return Err(()) };
    match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, false) {
        RetainedChromeGroupStep::Pending => return Ok(None),
        RetainedChromeGroupStep::Fault => return Err(()),
        RetainedChromeGroupStep::Complete => {}
    }
    let event = ActionDescriptor { controller_id: "framework.checkin".into(), action: "open".into(), args: None };
    input.register_hit(HitTarget { rect, event: Some(event), control_id: Some("s-checkin".into()), kind: HitKind::Button, drag_axis: None, drag_data: None });
    cursor.window = None;
    cursor.rect = None;
    Ok(Some(cursor.x + rect.w + theme.gap_standard * 0.5))
}

fn render_footer_section_divider(draw: &mut DrawList, theme: &Theme, x: f32, btn_y: f32, btn_h: f32) -> f32 {
    draw.push_solid([x + theme.gap_standard * 0.5, btn_y + 4.0, theme.stroke_hairline, btn_h - 8.0], theme.border_normal);
    x + theme.gap_standard
}

fn footer_utility_at_path<'utility>(roots: &'utility [UtilityNode], path: &[u16; 64], depth: usize) -> Option<&'utility UtilityNode> {
    let mut siblings = roots;
    for level in 0..=depth {
        let node = siblings.get(path[level] as usize)?;
        if level == depth {
            return Some(node);
        }
        let UtilityNode::Collection { children, .. } = node else { return None };
        siblings = children;
    }
    None
}

fn footer_utility_sibling_count(roots: &[UtilityNode], path: &[u16; 64], depth: usize) -> Option<usize> {
    if depth == 0 {
        return Some(roots.len());
    }
    let parent = footer_utility_at_path(roots, path, depth - 1)?;
    let UtilityNode::Collection { children, .. } = parent else { return None };
    Some(children.len())
}

fn render_footer_utility_node(
    cursor: &mut ShellChromeChildCursor,
    chrome: &mut ShellChromeBuildState,
    draw: &mut DrawList,
    atlas: &mut FontAtlas,
    icons: &IconAtlas,
    input: &mut InputState<ActionDescriptor>,
    theme: &Theme,
    x: f32,
    btn_y: f32,
    btn_h: f32,
    utility: &UtilityNode,
    collection_expanded: &HashMap<String, bool>,
) -> Result<Option<(f32, bool)>, ()> {
    match utility {
        UtilityNode::Separator { .. } => Ok(Some((render_footer_section_divider(draw, theme, x, btn_y, btn_h), false))),
        UtilityNode::Button { id, icon_id, label, text, title, disabled, on_press, .. } if !disabled.unwrap_or(false) => {
            let item = ChromeGroupItem { control_id: "framework.utility.button", icon_id: Some(icon_id.as_str()), label: Some(footer_utility_label(label, text, title, id)), active: false, disabled: false, kind: HitKind::Button };
            if cursor.rect.is_none() {
                let width = retained_chrome_group_item_width(theme, &item).ok_or(())?;
                cursor.rect = Some(Rect::new(x, btn_y, width, btn_h));
            }
            let Some(rect) = cursor.rect else { return Err(()) };
            match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, false) {
                RetainedChromeGroupStep::Pending => return Ok(None),
                RetainedChromeGroupStep::Fault => return Err(()),
                RetainedChromeGroupStep::Complete => {}
            }
            chrome.register_element_rect(id.clone(), rect);
            input.register_hit(HitTarget { rect, event: Some(on_press.clone()), control_id: Some(format!("framework.utility.button.{id}")), kind: HitKind::Button, drag_axis: None, drag_data: None });
            cursor.rect = None;
            Ok(Some((x + rect.w + theme.gap_standard * 0.5, false)))
        }
        UtilityNode::Toggle { id, icon_id, label, text, title, pressed, disabled, on_change, .. } if !disabled.unwrap_or(false) => {
            let item = ChromeGroupItem { control_id: "framework.utility.toggle", icon_id: Some(icon_id.as_str()), label: Some(footer_utility_label(label, text, title, id)), active: pressed.unwrap_or(false), disabled: false, kind: HitKind::Toggle };
            if cursor.rect.is_none() {
                let width = retained_chrome_group_item_width(theme, &item).ok_or(())?;
                cursor.rect = Some(Rect::new(x, btn_y, width, btn_h));
            }
            let Some(rect) = cursor.rect else { return Err(()) };
            match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, false) {
                RetainedChromeGroupStep::Pending => return Ok(None),
                RetainedChromeGroupStep::Fault => return Err(()),
                RetainedChromeGroupStep::Complete => {}
            }
            chrome.register_element_rect(id.clone(), rect);
            input.register_hit(HitTarget { rect, event: Some(on_change.clone()), control_id: Some(format!("framework.utility.toggle.{id}")), kind: HitKind::Toggle, drag_axis: None, drag_data: None });
            cursor.rect = None;
            Ok(Some((x + rect.w + theme.gap_standard * 0.5, false)))
        }
        UtilityNode::Collection { id, icon_id, label, text, title, disabled, children, .. } if !disabled.unwrap_or(false) => {
            let expanded = collection_expanded.get(id).copied().unwrap_or(false);
            let item = ChromeGroupItem { control_id: "framework.utility.collection", icon_id: Some(icon_id.as_str()), label: Some(footer_utility_label(label, text, title, id)), active: expanded, disabled: false, kind: HitKind::Button };
            if cursor.rect.is_none() {
                let width = retained_chrome_group_item_width(theme, &item).ok_or(())?;
                cursor.rect = Some(Rect::new(x, btn_y, width, btn_h));
            }
            let Some(rect) = cursor.rect else { return Err(()) };
            match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, false) {
                RetainedChromeGroupStep::Pending => return Ok(None),
                RetainedChromeGroupStep::Fault => return Err(()),
                RetainedChromeGroupStep::Complete => {}
            }
            input.register_hit(HitTarget { rect, event: None, control_id: Some(format!("framework.utility.collection.{id}")), kind: HitKind::Button, drag_axis: None, drag_data: None });
            cursor.rect = None;
            Ok(Some((x + rect.w + theme.gap_standard * 0.5, expanded && !children.is_empty())))
        }
        UtilityNode::Button { .. } | UtilityNode::Toggle { .. } | UtilityNode::Collection { .. } => Ok(Some((x, false))),
    }
}

#[cfg(test)]
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
// 🎯️ Exercised today only by `🛰️Dock`'s own `action_host_window_id_finds_scoping_window` test — the
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
        let active_id = self.active_window_id.as_deref().or(session.view_state.window_id.as_deref());
        active_id
            .and_then(|id| self.live_window_kind_id(session, id))
            .and_then(|kind_id| session.app.window_kinds.iter().find(|kind| kind.id == kind_id))
            .unwrap_or_else(|| session.app.window_kinds.first())
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
        let active = self.active_window_id.as_deref().and_then(|window_id| self.active_utility_by_window.get(window_id)).map(String::as_str);
        ui_wgpu::wgpu::component::utilities::derive_utility_nodes(&session.app.controller_id, &specs, active)
    }
    // #endregion

    // #region active-utility
    /// 🧰️ Applies a user-driven `setActiveUtility`: re-selecting the active utility deactivates it, otherwise
    /// it becomes the active utility for that window kind (Architecture Decision 4).
    pub(crate) fn apply_set_active_utility(&mut self, window_id: &str, utility_id: &str) {
        let already = self.active_utility_by_window.get(window_id).map(String::as_str) == Some(utility_id);
        if already {
            self.active_utility_by_window.remove(window_id);
        } else {
            self.active_utility_by_window.insert(window_id.to_string(), utility_id.to_string());
            // 🎓️ Advance-by-doing: only the activation branch counts as "the utility was activated" —
            // see `chrome_tour_note_utility_performed`.
            self.chrome_tour_note_utility_performed(utility_id);
        }
    }

    /// 🧰️ The active utility id for a window kind, if any.
    pub(crate) fn active_utility_for_window(&self, window_id: &str) -> Option<&str> {
        self.active_utility_by_window.get(window_id).map(String::as_str)
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
    pub(crate) fn actions_enabled_for_window(&self, app: &AppDefinition, window_id: &str) -> bool {
        match self.active_utility_for_window(window_id) {
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

    /// 🧮️ Resolves required presence and current catalog membership before native staged execution.
    pub(crate) fn resolved_execute_args(defs: &[semio_framework::ActionArgDef], staged: &serde_json::Map<String, Value>) -> Option<serde_json::Map<String, Value>> {
        let staged_dsl = DslValue::from(Value::Object(staged.clone()));
        let effective = semio_framework::effective_action_args(defs, &staged_dsl, None);
        if semio_framework::unresolved_action_args(defs, &effective).is_empty() {
            Value::from(&effective).as_object().cloned()
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
            CommandDefinition::new("os.toggleFullscreen", LocalizedLabel::native("Toggle Full Screen", "Vollbild umschalten"), "window", "code", ActionKind::Shell)
                .with_keybinding(PlatformKeybinding::for_platform("f11", Platform::Windows))
                .with_keybinding(PlatformKeybinding::for_platform("f11", Platform::Linux))
                .with_keybinding(PlatformKeybinding::for_platform("control+meta+f", Platform::MacOs)),
            CommandDefinition::new("os.setAppearance", LocalizedLabel::data("Set Appearance"), "appearance", "settings", ActionKind::Shell).with_args([ActionArgDef::select(
                "value",
                LocalizedLabel::data("Appearance"),
                vec![
                    ActionArgOption { value: "system".into(), label: LocalizedLabel::data("System") },
                    ActionArgOption { value: "light".into(), label: LocalizedLabel::data("Light") },
                    ActionArgOption { value: "dark".into(), label: LocalizedLabel::data("Dark") },
                ],
            )
            .required()]),
            CommandDefinition::new("os.setDriver", LocalizedLabel::data("Set Driver"), "layout", "settings", ActionKind::Shell).with_args([ActionArgDef::select(
                "value",
                LocalizedLabel::data("Driver"),
                vec![ActionArgOption { value: "default".into(), label: LocalizedLabel::data("Default") }, ActionArgOption { value: "compact".into(), label: LocalizedLabel::data("Compact") }],
            )
            .required()]),
            CommandDefinition::new("os.setLocale", LocalizedLabel::data("Set Locale"), "language", "settings", ActionKind::Shell).with_args([ActionArgDef::select(
                "value",
                LocalizedLabel::data("Locale"),
                vec![ActionArgOption { value: "en".into(), label: LocalizedLabel::data("English") }, ActionArgOption { value: "de".into(), label: LocalizedLabel::data("Deutsch") }],
            )
            .required()]),
            CommandDefinition::new("os.setTerminology", LocalizedLabel::data("Set Terminology"), "language", "settings", ActionKind::Shell)
                .with_args([ActionArgDef::select("value", LocalizedLabel::data("Terminology"), terminology_options).required()]),
            CommandDefinition::new("os.setThemeId", LocalizedLabel::data("Set Theme"), "appearance", "settings", ActionKind::Shell).with_args([ActionArgDef::select(
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
            CommandDefinition::new("os.resetDock", LocalizedLabel::data("Reset Dock Layout"), "layout", "panel-left", ActionKind::Shell),
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
                    action: Some(if is_os { format!("os-command:{}", definition.id) } else { format!("command:{}", dsl::os_pack::json::to_json_string(&invocation)) }),
                    category: Some(category.clone()),
                });
                continue;
            }
            if let Some(arg) = definition.args.first() {
                if let semio_framework::ActionArgControl::Select { options } = &arg.control() {
                    for option in options {
                        let is_os = matches!(address.owner, semio_framework::manifest::CommandOwnerAddress::Os);
                        let invocation = semio_framework::manifest::CommandInvocation { address: address.clone(), arguments: BTreeMap::from([(arg.id.clone(), DslValue::String(option.value.clone()))]) };
                        items.push(SearchPaletteItem {
                            id: format!("command.{}.{}", command_address_stable_key(&address), option.value),
                            label: format!("{}: {}", definition.label.resolve(self.active_terminology(), self.active_locale()), option.label.resolve(self.active_terminology(), self.active_locale())),
                            group: group.clone(),
                            dispatch_action: None,
                            action: Some(if is_os { format!("os-command:{}:{}", definition.id, option.value) } else { format!("command:{}", dsl::os_pack::json::to_json_string(&invocation)) }),
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
#[cfg(test)]
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
#[path = "../../🧪️tests/🔬️wgpu-command-registry/🦀️.rs"]
mod command_registry_tests;
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
    #[cfg(test)]
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

    #[cfg(test)]
    fn open_dialog(&mut self, request: ChromeDialogRequest) {
        self.dialog_stack.push(request);
    }

    fn close_topmost_dialog(&mut self) {
        self.dialog_stack.pop();
    }

    #[cfg(test)]
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



/// 🧭️ Item 5's "active-path tracking": true if `nodes` (recursively, through nested `Collection`s)
/// contains a pressed `Toggle` — used so a *collapsed* ribbon `Collection` still highlights when the
/// user's current selection lives inside it, mirroring `ui/js/react/index.tsx`'s recursive picker
/// active-path reconciliation. Called from `render_footer_utility_nodes` (an off-limits `ShellInput`-
/// adjacent function this wave — see the report's coordination note).
#[cfg(test)]
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
}

impl ShellChromeBuildState {
    fn register_element_rect(&mut self, id: impl Into<String>, rect: Rect) {
        self.element_rects.insert(id.into(), ChromeElementRectEntry { rect });
    }

    #[cfg(test)]
    fn register_element_rect_fallback(&mut self, id: impl Into<String>, rect: Rect) {
        let id = id.into();
        self.element_rects.entry(id).or_insert(ChromeElementRectEntry { rect });
    }

    fn resolve_element_rect(&self, id: &str) -> Option<Rect> {
        self.element_rects.get(id).map(|entry| entry.rect)
    }


}

/// 🎓️ Punches `hole` out of `band`, returning up to four remaining rectangles (or the original band when
/// they don't overlap). The React shell now renders one fullscreen `ui-veil` div and raises the
/// introduced/shown element's chrome unit above it via z-index instead of doing this subtraction itself —
/// wgpu keeps the geometric subtraction because it's the only way to realize the *same visual result*
/// here: `push_solid` quads tile with no seam (no per-quad backdrop-filter to discontinue), a real glass
/// veil can't work in this renderer (see `introduction_veil_bands`'s doc), and 3D window content lives in
/// separate `scene_passes` that can't be repainted above an overlay at all.
#[cfg(test)]
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
#[cfg(test)]
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

#[cfg(test)]
const INTRODUCED_PULSE_PERIOD_MS: f64 = 1600.0;

/// 🎓️ Raised-cosine breathing thickness for the introduced-element pulse ring — mirrors the
/// `data-introduced` CSS keyframes (`ui/styling/js/🎨️ui.css`: hairline → focus → hairline over 1.6s,
/// ease-in-out), which are exactly a raised cosine.
#[cfg(test)]
fn introduced_pulse_thickness(now_ms: f64, hairline: f32, focus: f32) -> f32 {
    let phase = (now_ms.rem_euclid(INTRODUCED_PULSE_PERIOD_MS) / INTRODUCED_PULSE_PERIOD_MS) as f32;
    hairline + (focus - hairline) * 0.5 * (1.0 - (phase * std::f32::consts::TAU).cos())
}

#[cfg(test)]
const INTRODUCTION_INFO_BOX_GAP: f32 = 16.0;

/// 🎓️ Where the info box sits relative to `anchor` — byte-for-byte port of `resolveIntroductionPlacement`
/// (`ui/js/react/index.tsx`). `auto` picks the side with the most free viewport space; `center` (and any
/// anchor-less step) centers the box.
#[cfg(test)]
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
#[cfg(test)]
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
#[cfg(test)]
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
fn render_tutorial_gesture_overlay_node(state: &ShellState, overlay: &mut DrawList, theme: &Theme) {
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
        K::Load { document_dsl, previous_dsl } => TutorialPendingDocOp::LoadArtifactDsl(if forward { document_dsl.clone() } else { previous_dsl.clone() }),
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
        if let Some(document_dsl) = definition.base.document_dsl.clone() {
            self.tutorial_pending_document_ops.push(TutorialPendingDocOp::LoadArtifactDsl(document_dsl));
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
            base: semio_framework::TutorialBase { document_dsl: self.last_envelope_dsl.clone(), example_id: self.active_example_id.clone(), ui: ui.clone(), cameras },
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
#[path = "../../🧪️tests/🔬️wgpu-tutorial/🦀️.rs"]
mod tutorial_tests;
//#endregion 🎬️Tutorial

pub(crate) struct ShellChromeFrameCursor {
    phase: ShellChromeFramePhase,
    setup: u8,
    child: ShellChromeChildCursor,
}

/// 🧭️ Fixed 64-deep footer-utility tree cursor path. A newtype rather than a bare `[u16; 64]`
/// field because std implements `Default` for arrays only up to length 32, and
/// `ShellChromeChildCursor` must stay derivable — the bound is on the array, not on the cursor.
struct ShellUtilityPath([u16; 64]);

impl Default for ShellUtilityPath {
    fn default() -> Self {
        Self([0; 64])
    }
}

#[derive(Default)]
struct ShellChromeChildCursor {
    phase: u16,
    item: usize,
    scalar: usize,
    x: f32,
    right: f32,
    flag: bool,
    fault: bool,
    group_phase: u8,
    document: UiDocumentFrameCursor,
    window: Option<UiText>,
    rect: Option<Rect>,
    path: ShellUtilityPath,
    depth: usize,
    find_rejected: Option<ShellFindItem>,
    glyph: RetainedGlyphCursor,
}

#[derive(Clone, Copy, Default)]
enum ShellChromeFramePhase {
    #[default]
    CloseDocument,
    LoadPreferences,
    IntroductionRead,
    Presence,
    PersistLayout,
    FrameSetup,
    MainWindow,
    LeftPanel,
    RightPanel,
    Navbar,
    TutorialBar,
    Footer,
    Overlay,
    TreeDrag,
    TutorialGesture,
    Error,
    IntroductionWrite,
    PersistPreferences,
    Complete,
}

impl Default for ShellChromeFrameCursor {
    fn default() -> Self {
        Self { phase: ShellChromeFramePhase::default(), setup: 0, child: ShellChromeChildCursor::default() }
    }
}

impl ShellChromeFrameCursor {
    pub(crate) fn terminal_is_complete(&self) -> bool {
        matches!(self.phase, ShellChromeFramePhase::Complete)
    }

    fn advance(&mut self, phase: ShellChromeFramePhase) {
        self.phase = phase;
        self.child = ShellChromeChildCursor::default();
    }
}

impl ShellState {
    /// 🎭️ Advances one retained chrome child per worker opportunity.
    pub(crate) fn render_chrome_step(&mut self, cursor: &mut ShellChromeFrameCursor, draw: &mut DrawList, overlay: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme) -> bool {
        let w = self.screen_w;
        let h = self.screen_h;
        let body = self.body_rect(theme);
        match cursor.phase {
            ShellChromeFramePhase::CloseDocument => {
                let close_consumed = self.close_document_one();
                begin_ui_document_opportunity(close_consumed);
                cursor.phase = ShellChromeFramePhase::LoadPreferences;
            }
            ShellChromeFramePhase::LoadPreferences => {
                self.request_chrome_preferences_load();
                cursor.phase = ShellChromeFramePhase::IntroductionRead;
            }
            ShellChromeFramePhase::IntroductionRead => {
                self.request_introduction_read();
                cursor.phase = ShellChromeFramePhase::Presence;
            }
            ShellChromeFramePhase::Presence => {
                self.request_presence_preview();
                cursor.phase = ShellChromeFramePhase::PersistLayout;
            }
            ShellChromeFramePhase::PersistLayout => {
                self.request_panel_layout_persist();
                cursor.phase = ShellChromeFramePhase::FrameSetup;
            }
            ShellChromeFramePhase::FrameSetup => {
                match cursor.setup {
                    0 => {
                        draw.set_screen_height(h);
                    }
                    1 => overlay.set_screen_height(h),
                    2 => draw.push_solid([0.0, 0.0, w, h], theme.background),
                    3 => {
                        if self.chrome_build.find_items.pop_front().is_some() {
                            return false;
                        }
                        if !self.chrome_build.find_items.begin_next_generation() {
                            cursor.phase = ShellChromeFramePhase::Complete;
                            return false;
                        }
                    }
                    4 => {
                        if self.find_items.pop_front().is_some() {
                            return false;
                        }
                        if !self.find_items.begin_next_generation() {
                            cursor.phase = ShellChromeFramePhase::Complete;
                            return false;
                        }
                    }
                    5 => {
                        if self.chrome_build.tooltip_titles.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    6 => {
                        if self.chrome_build.element_rects.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    7 => {
                        if self.widget_maps.input_metas.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    8 => {
                        if self.widget_maps.select_metas.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    9 => {
                        if self.widget_maps.toggle_metas.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    10 => {
                        if self.widget_maps.slider_metas.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    11 => {
                        if self.widget_maps.stepper_metas.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    12 => {
                        if self.widget_maps.ring_metas.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    13 => {
                        if self.widget_maps.slider_live_values.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    14 => {
                        if self.widget_maps.ring_live_values.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    15 => {
                        if self.widget_maps.tree_hover_commands.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    16 => {
                        if self.widget_maps.tree_unhover_commands.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    17 => {
                        self.widget_maps.tree_selection_change = None;
                    }
                    18 => {
                        self.chrome_build.compute_click_edge(input.pointer_down);
                    }
                    19 => {
                        self.chrome_tour_frame_begin();
                    }
                    20 => {
                        cursor.phase = ShellChromeFramePhase::MainWindow;
                        return false;
                    }
                    _ => return false,
                }
                cursor.setup += 1;
            }
            ShellChromeFramePhase::MainWindow => {
                if cursor.child.find_rejected.take().is_some() {
                    return false;
                }
                if let Some(item) = self.chrome_build.find_items.pop_front() {
                    if let Err(item) = self.find_items.try_push(item) {
                        cursor.child.find_rejected = Some(item);
                    }
                    return false;
                }
                let binding = match self.chrome_build.find_items.bind() {
                    Ok(binding) => binding,
                    Err(()) => return false,
                };
                let mut overlay_slot = Some(overlay);
                let complete = self.render_main_window_step(&mut cursor.child, draw, &mut overlay_slot, atlas, icons, input, theme, body);
                drop(binding);
                if !complete {
                    return false;
                }
                if !self.chrome_build.find_items.terminal_is_empty() {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::LeftPanel);
            }
            ShellChromeFramePhase::LeftPanel => {
                if self.left_panel_open && self.has_left_tabs() && !self.render_panel_step(&mut cursor.child, true, overlay, None, atlas, icons, input, theme, body) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::RightPanel);
            }
            ShellChromeFramePhase::RightPanel => {
                if self.right_panel_open && self.has_right_tabs() && !self.render_panel_step(&mut cursor.child, false, overlay, None, atlas, icons, input, theme, body) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::Navbar);
            }
            ShellChromeFramePhase::Navbar => {
                if !self.render_navbar_step(&mut cursor.child, overlay, atlas, icons, input, theme, w) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::TutorialBar);
            }
            ShellChromeFramePhase::TutorialBar => {
                if !self.render_tutorial_bar_step(&mut cursor.child, overlay, atlas, icons, input, theme, w) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::Footer);
            }
            ShellChromeFramePhase::Footer => {
                if !self.render_footer_step(&mut cursor.child, overlay, atlas, icons, input, theme, w, h) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::Overlay);
            }
            ShellChromeFramePhase::Overlay => {
                if !self.render_overlay_step(&mut cursor.child, overlay, atlas, icons, input, theme, w, h) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::TreeDrag);
            }
            ShellChromeFramePhase::TreeDrag => {
                self.render_tree_drag_overlay_node(overlay, input, theme);
                cursor.advance(ShellChromeFramePhase::TutorialGesture);
            }
            ShellChromeFramePhase::TutorialGesture => {
                render_tutorial_gesture_overlay_node(self, overlay, theme);
                cursor.advance(ShellChromeFramePhase::Error);
            }
            ShellChromeFramePhase::Error => {
                if let Some(error) = &self.error {
                    match chrome_text_complete_step(draw, atlas, error, 12.0, h - theme.footer_height - 24.0, (w - 24.0).max(1.0), theme.font_size_small, theme.error, &mut cursor.child.glyph) {
                        Ok(false) => return false,
                        Ok(true) => {}
                        Err(()) => cursor.child.glyph.reset(),
                    }
                }
                cursor.advance(ShellChromeFramePhase::IntroductionWrite);
            }
            ShellChromeFramePhase::IntroductionWrite => {
                if self.chrome_present.maintenance.introduction_write.is_some() {
                    cursor.phase = ShellChromeFramePhase::PersistPreferences;
                } else if let Some(app_id) = self.chrome_build.introduction_seen_writes.pop() {
                    if app_id.capacity() > SHELL_CHROME_IO_FIELD_BYTES {
                        cursor.phase = ShellChromeFramePhase::PersistPreferences;
                    } else {
                        self.chrome_present.maintenance.introduction_write = Some(app_id);
                    }
                } else {
                    cursor.phase = ShellChromeFramePhase::PersistPreferences;
                }
            }
            ShellChromeFramePhase::PersistPreferences => {
                self.request_chrome_preferences_persist();
                cursor.phase = ShellChromeFramePhase::Complete;
            }
            ShellChromeFramePhase::Complete => return true,
        }
        cursor.terminal_is_complete()
    }

    /// 📨️ Coalesces the persisted chrome load without touching platform storage in the frame step.
    fn request_chrome_preferences_load(&mut self) {
        if !self.chrome_present.preferences_loaded {
            self.chrome_present.maintenance.load_requested = true;
        }
    }

    /// 🎓️ Coalesces one bounded introduction lookup for the shared I/O lane.
    fn request_introduction_read(&mut self) {
        if self.chrome_present.maintenance.introduction_read.is_some() {
            return;
        }
        let Some(app_id) = self.session.as_ref().map(|session| session.app.id.as_str()) else { return };
        if app_id.len() <= SHELL_CHROME_IO_FIELD_BYTES && !self.chrome_build.introduction_seen.contains_key(app_id) {
            self.chrome_present.maintenance.introduction_read = Some(app_id.to_string());
        }
    }

    /// 🗄️ Coalesces one bounded panel-layout page for the shared I/O lane.
    fn request_panel_layout_persist(&mut self) {
        self.chrome_present.maintenance.layout_requested = true;
    }

    /// 💓️ Coalesces one bounded presence preview page for the shared I/O lane.
    fn request_presence_preview(&mut self) {
        self.chrome_present.maintenance.presence_requested = true;
    }

    /// 💾️ Coalesces changed preference fields for the shared I/O lane.
    fn request_chrome_preferences_persist(&mut self) {
        self.chrome_present.maintenance.persist_requested = true;
    }

    /// 🧵️ Reports whether the frame has retained chrome I/O or preview work to resume.
    pub(crate) fn chrome_maintenance_pending(&self) -> bool {
        self.chrome_present.maintenance.pending()
    }

    /// 🎨️ Exposes the already-loaded theme id without synchronously opening preference storage.
    pub(crate) fn active_theme_id_for_frame(&self) -> &str {
        &self.chrome_build.preferences.theme_id
    }

    /// ⏭️ Advances one bounded field or preview page and retains every remaining owner.
    pub(crate) fn advance_chrome_maintenance_step(&mut self) -> bool {
        if self.chrome_present.maintenance.load_requested {
            self.advance_chrome_preferences_load_step();
        } else if let Some(app_id) = self.chrome_present.maintenance.introduction_read.take() {
            let key = format!("{UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX}{app_id}");
            let seen = prefs_get_bounded(&key).as_deref() == Some("true");
            self.chrome_build.introduction_seen.insert(app_id, seen);
        } else if let Some(app_id) = self.chrome_present.maintenance.introduction_write.take() {
            let key = format!("{UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX}{app_id}");
            prefs_set_bounded(&key, "true");
        } else if self.chrome_present.maintenance.layout_requested {
            self.advance_panel_layout_persist_step();
        } else if self.chrome_present.maintenance.presence_requested {
            self.advance_presence_preview_step();
        } else if self.chrome_present.maintenance.persist_requested {
            self.advance_chrome_preferences_persist_step();
        }
        self.chrome_present.maintenance.pending()
    }

    fn advance_chrome_preferences_load_step(&mut self) {
        let phase = self.chrome_present.maintenance.load_phase;
        match phase {
            0 => {
                let preferences = read_ui_preferences();
                let custom_themes = custom_themes_from(&preferences);
                self.appearance_id = env_lock("SEMIO_LOCKED_APPEARANCE").unwrap_or_else(|| appearance_id(preferences.appearance));
                self.locale_id = env_lock("SEMIO_LOCKED_LOCALE").unwrap_or_else(|| locale_id(preferences.locale));
                self.terminology_id = env_lock("SEMIO_LOCKED_TERMINOLOGY").or(preferences.terminology).unwrap_or_else(|| UI_TERMINOLOGY_NATIVE.to_string());
                self.driver_id = preferences.driver_id.unwrap_or_else(|| "default".to_string());
                self.chrome_build.preferences.custom_drivers = preferences.custom_drivers;
                self.chrome_build.preferences.ui_layout = match preferences.layout { Some(OsUiChromeLayout::Tablet) => "tablet", _ => "desktop" }.to_string();
                self.chrome_build.preferences.theme_id = env_lock("SEMIO_LOCKED_THEME").or(preferences.theme_id).unwrap_or_else(|| "semio".to_string());
                self.chrome_build.preferences.custom_themes = custom_themes;
                self.chrome_build.preferences.keybinding_overrides = preferences.keybinding_overrides;
                let loaded = self.chrome_build.preferences.clone();
                with_chrome_prefs(|current| *current = loaded);
            }
            1 => {
                self.chrome_build.preferences.worker_count = prefs_get_bounded(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY).and_then(|raw| raw.parse::<u32>().ok()).filter(|count| *count >= 1).unwrap_or_else(default_compute_worker_count);
            }
            _ => {
                self.chrome_present.preferences_loaded = true;
                self.chrome_present.maintenance.load_requested = false;
                self.chrome_present.maintenance.load_phase = 0;
                self.chrome_present.last_synced_preferences = Some(UiPrefsSnapshot::capture(self));
                return;
            }
        }
        self.chrome_present.maintenance.load_phase = phase.saturating_add(1);
    }

    fn advance_panel_layout_persist_step(&mut self) {
        let snapshot = self.panel_layout_snapshot();
        if self.chrome_present.last_persisted_panel_layout.as_ref() != Some(&snapshot) {
            if let Some(raw) = encode_panel_layout_field(&snapshot) {
                prefs_set_bounded(PANEL_LAYOUT_STORAGE_KEY, &raw);
                self.chrome_present.last_persisted_panel_layout = Some(snapshot);
            }
        }
        self.chrome_present.maintenance.layout_requested = false;
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn advance_presence_preview_step(&mut self) {
        self.chrome_present.maintenance.presence_requested = false;
        let Some(channel) = self.sync_channel.as_ref() else { return };
        if channel.document_id.len() > SHELL_CHROME_IO_FIELD_BYTES || channel.plugin_id.len() > SHELL_CHROME_IO_FIELD_BYTES {
            return;
        }
        let actor = self.current_shell_actor(channel.instance_id);
        if actor.len() > SHELL_CHROME_IO_FIELD_BYTES {
            return;
        }
        let _document_id = channel.document_id.clone();
        let connected_at_ms = channel.connected_at_ms;
        let label = self.session.as_ref().map(|session| session.app.id.clone()).filter(|value| value.len() <= SHELL_CHROME_IO_FIELD_BYTES);
        let user_id = self.identity.as_ref().map(|identity| identity.user_id.clone()).filter(|value| value.len() <= SHELL_CHROME_IO_FIELD_BYTES);
        let peer = PresencePeer { actor, label, presence_pack: None, connected_at_ms, user_id, role: None, drag_ghost_json: None, interaction: None, color: None, surface: None, views: Vec::new(), ui: None };
        self.document_host.presence_heartbeat_key(&channel.document_key, chrome_now_ms() as u64, peer);
    }

    #[cfg(target_arch = "wasm32")]
    fn advance_presence_preview_step(&mut self) {
        self.chrome_present.maintenance.presence_requested = false;
    }

    fn advance_chrome_preferences_persist_step(&mut self) {
        let phase = self.chrome_present.maintenance.persist_phase;
        let snapshot = UiPrefsSnapshot::capture(self);
        match phase {
            0 if self.chrome_present.last_synced_preferences.as_ref() != Some(&snapshot) => {
                persist_ui_preferences(self, self.chrome_present.last_synced_preferences.as_ref());
            }
            1 if self.chrome_present.last_synced_preferences.as_ref().map(|synced| synced.worker_count) != Some(snapshot.worker_count) => {
                prefs_set_bounded(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY, &snapshot.worker_count.to_string());
            }
            0..=1 => {}
            _ => {
                self.chrome_present.last_synced_preferences = Some(snapshot);
                self.chrome_present.maintenance.persist_requested = false;
                self.chrome_present.maintenance.persist_phase = 0;
                return;
            }
        }
        self.chrome_present.maintenance.persist_phase = phase.saturating_add(1);
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

    fn render_main_window_step(&mut self, cursor: &mut ShellChromeChildCursor, draw: &mut DrawList, overlay: &mut Option<&mut DrawList>, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, bounds: Rect) -> bool {
        match cursor.phase {
            0 => {
                draw.push_solid([bounds.x, bounds.y, bounds.w, bounds.h], theme.background);
                cursor.rect = Some(bounds.inset(theme.panel_inset));
                cursor.phase = 1;
            }
            1 => {
                let window = if self.space_mode && self.spawned_ui.is_some() { UiText::try_from_str("spawned") } else { self.active_window_id.as_deref().and_then(UiText::try_from_str) };
                cursor.window = window;
                cursor.phase = 2;
            }
            2 => {
                let Some(rect) = cursor.rect else {
                    cursor.phase = u16::MAX;
                    return false;
                };
                draw.push_scissor(rect);
                cursor.phase = 3;
            }
            3 => {
                let Some(rect) = cursor.rect else {
                    cursor.phase = u16::MAX;
                    return false;
                };
                let control_id = cursor.window.as_ref().map_or("window", UiText::as_str);
                input.register_hit(HitTarget { rect, event: None, control_id: Some(control_id.into()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
                cursor.phase = 4;
            }
            4 => {
                let Some(window) = cursor.window.as_ref() else {
                    cursor.phase = 5;
                    return false;
                };
                let document = if window.as_str() == "spawned" { self.spawned_ui.as_ref() } else { self.window_ui.get(window.as_str()) }.and_then(|lease| lease.try_alias().ok());
                let Some(document) = document else {
                    cursor.phase = 5;
                    return false;
                };
                let Some(rect) = cursor.rect else {
                    cursor.phase = u16::MAX;
                    return false;
                };
                let scroll_offsets = &mut self.scroll_offsets;
                let collapsed_sections = &mut self.collapsed_sections;
                let open_selects = &mut self.open_selects;
                let widget_maps = &mut self.widget_maps;
                let mut ctx = framework_widget_context(draw, overlay.as_deref_mut(), atlas, Some(icons), input, theme, scroll_offsets, collapsed_sections, open_selects, Some(widget_maps));
                ctx.pick_clip = Some(rect);
                if !render_ui_document_step(&mut cursor.document, &document, rect, &mut ctx, window.as_str()) {
                    return false;
                }
                cursor.phase = 5;
            }
            5 => {
                draw.pop_scissor();
                cursor.phase = 6;
            }
            6 => return true,
            _ => return false,
        }
        false
    }

    fn render_panel_step(&mut self, cursor: &mut ShellChromeChildCursor, left: bool, panel_draw: &mut DrawList, overlay: Option<&mut DrawList>, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, body: Rect) -> bool {
        const PANEL_RESIZE_HIT_PX: f32 = 20.0;
        match cursor.phase {
            0 => {
                cursor.rect = Some(self.floating_panel_rect(left, body, theme));
                let active = if left { self.active_left_tab.as_deref().unwrap_or(FRAMEWORK_PANEL_TAB_ARTIFACT_ID) } else { self.active_right_tab.as_deref().unwrap_or(FRAMEWORK_SETTINGS_GENERAL_TAB_ID) };
                cursor.window = UiText::try_from_str(active);
                cursor.phase = 1;
            }
            1 => {
                let Some(panel) = cursor.rect else {
                    cursor.phase = u16::MAX;
                    return false;
                };
                cursor.scalar = panel_draw.push_glass([panel.x, panel.y, panel.w, panel.h], theme.border_radius, theme.glass(Level::Panel));
                cursor.phase = 2;
            }
            2 => {
                panel_draw.begin_glass_content(cursor.scalar);
                cursor.phase = 3;
            }
            3..=6 => {
                let Some(panel) = cursor.rect else {
                    cursor.phase = u16::MAX;
                    return false;
                };
                let hair = theme.stroke_hairline;
                let rect = match cursor.phase {
                    3 => [panel.x, panel.y, panel.w, hair],
                    4 => [panel.x, panel.y + panel.h - hair, panel.w, hair],
                    5 => [panel.x, panel.y, hair, panel.h],
                    _ => [panel.x + panel.w - hair, panel.y, hair, panel.h],
                };
                panel_draw.push_solid(rect, theme.border_normal);
                cursor.phase += 1;
            }
            7 => {
                let Some(panel) = cursor.rect else {
                    cursor.phase = u16::MAX;
                    return false;
                };
                let content = panel.inset(theme.gap_standard);
                panel_draw.push_scissor(content);
                cursor.phase = 8;
            }
            8 => {
                let Some(window) = cursor.window.as_ref() else {
                    cursor.phase = 9;
                    return false;
                };
                let Some(document) = self.panel_documents.get(window.as_str()).and_then(|lease| lease.try_alias().ok()) else {
                    cursor.phase = 9;
                    return false;
                };
                let Some(panel) = cursor.rect else {
                    cursor.phase = u16::MAX;
                    return false;
                };
                let content = panel.inset(theme.gap_standard);
                let scroll_offsets = &mut self.scroll_offsets;
                let collapsed_sections = &mut self.collapsed_sections;
                let open_selects = &mut self.open_selects;
                let widget_maps = &mut self.widget_maps;
                let mut ctx = framework_widget_context(panel_draw, overlay, atlas, Some(icons), input, theme, scroll_offsets, collapsed_sections, open_selects, Some(widget_maps));
                ctx.pick_clip = Some(content);
                if !render_ui_document_step(&mut cursor.document, &document, content, &mut ctx, window.as_str()) {
                    return false;
                }
                cursor.phase = 9;
            }
            9 => {
                panel_draw.pop_scissor();
                cursor.phase = 10;
            }
            10 => {
                panel_draw.end_glass_content();
                cursor.phase = 11;
            }
            11 => {
                let Some(panel) = cursor.rect else {
                    cursor.phase = u16::MAX;
                    return false;
                };
                let resize_id = if left { "panel.resize.left" } else { "panel.resize.right" };
                let resize_handle = if left { Rect::new(panel.x + panel.w - PANEL_RESIZE_HIT_PX, panel.y, PANEL_RESIZE_HIT_PX, panel.h) } else { Rect::new(panel.x, panel.y, PANEL_RESIZE_HIT_PX, panel.h) };
                input.register_hit(HitTarget { rect: resize_handle, event: None, control_id: Some(resize_id.into()), kind: HitKind::PanelResize, drag_axis: Some(DragAxis::Horizontal), drag_data: None });
                cursor.phase = 12;
            }
            12 => return true,
            _ => return false,
        }
        false
    }

    fn render_navbar_step(&mut self, cursor: &mut ShellChromeChildCursor, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32) -> bool {
        let btn_h = theme.control_height;
        let btn_y = (theme.navbar_height - btn_h) * 0.5;
        match cursor.phase {
            0 => {
                draw.push_solid([0.0, 0.0, width, theme.navbar_height], theme.navbar);
                cursor.x = theme.padding_standard;
                cursor.right = width - theme.padding_standard;
                cursor.phase = 1;
            }
            1 => {
                let hovered = Rect::new(0.0, 0.0, width, theme.navbar_height).contains(input.pointer_x, input.pointer_y);
                draw.push_solid([0.0, theme.navbar_height - theme.stroke_hairline, width, theme.stroke_hairline], if hovered { theme.border_emphasized } else { theme.border_normal });
                cursor.phase = 2;
            }
            2 => {
                let size = btn_h - theme.gap_standard;
                chrome_icon(draw, icons, "semio-logo", cursor.x, btn_y + (btn_h - size) * 0.5, size, theme.text);
                cursor.x += size + theme.gap_standard;
                cursor.phase = 3;
            }
            3 => {
                let title = self.session.as_ref().map_or("semio · os", |session| session.app.id.as_str());
                match chrome_text_complete_step(draw, atlas, title, cursor.x, btn_y + (btn_h + theme.font_size_body) * 0.5 - 2.0, (width - cursor.x).max(1.0), theme.font_size_body, theme.text, &mut cursor.glyph) {
                    Ok(false) => return false,
                    Ok(true) => {}
                    Err(()) => {
                        self.error = Some("Shell navbar text exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
                cursor.phase = 4;
            }
            4 => {
                let is_de = self.locale_id == "de";
                let item = ChromeGroupItem {
                    control_id: "ui.fullscreen.toggle",
                    icon_id: Some(if self.fullscreen_active { "minimize-2" } else { "maximize-2" }),
                    label: Some(if self.fullscreen_active { shell_chrome_string("fullscreen.exit", is_de) } else { shell_chrome_string("fullscreen.toggle", is_de) }),
                    active: self.fullscreen_active,
                    disabled: false,
                    kind: HitKind::Toggle,
                };
                if cursor.rect.is_none() {
                    let Some(item_w) = retained_chrome_group_item_width(theme, &item) else {
                        self.error = Some("Shell fullscreen item exceeded the retained chrome boundary".to_string());
                        cursor.phase = 5;
                        return false;
                    };
                    cursor.right -= item_w;
                    cursor.rect = Some(Rect::new(cursor.right, btn_y, item_w, btn_h));
                }
                let Some(rect) = cursor.rect else { return false };
                match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, true) {
                    RetainedChromeGroupStep::Pending => return false,
                    RetainedChromeGroupStep::Complete => {}
                    RetainedChromeGroupStep::Fault => self.error = Some("Shell fullscreen item exceeded the retained glyph boundary".to_string()),
                }
                cursor.rect = None;
                cursor.phase = 5;
            }
            5 => {
                let is_de = self.locale_id == "de";
                let display = self.has_display_tabs();
                let item = match (display, cursor.item) {
                    (true, 0) => Some(ChromeGroupItem {
                        control_id: "ui.panelToggle.display",
                        icon_id: Some(panel_toggle_icon_id("display", self.session.as_ref())),
                        label: Some(shell_chrome_string("panelToggle.display", is_de)),
                        active: self.left_panel_open && self.active_left_kind == LeftPanelKind::Display,
                        disabled: false,
                        kind: HitKind::Toggle,
                    }),
                    (true, 1) | (false, 0) => Some(ChromeGroupItem {
                        control_id: "ui.panelToggle.workbench",
                        icon_id: Some(panel_toggle_icon_id("workbench", self.session.as_ref())),
                        label: Some(shell_chrome_string("panelToggle.workbench", is_de)),
                        active: self.left_panel_open && self.active_left_kind == LeftPanelKind::Workbench,
                        disabled: false,
                        kind: HitKind::Toggle,
                    }),
                    (true, 2) | (false, 1) => Some(ChromeGroupItem {
                        control_id: "ui.panelToggle.details",
                        icon_id: Some(panel_toggle_icon_id("details", self.session.as_ref())),
                        label: Some(shell_chrome_string("panelToggle.details", is_de)),
                        active: self.right_panel_open && self.active_right_kind == RightPanelKind::Details,
                        disabled: false,
                        kind: HitKind::Toggle,
                    }),
                    (true, 3) | (false, 2) => Some(ChromeGroupItem {
                        control_id: "ui.panelToggle.settings",
                        icon_id: Some(panel_toggle_icon_id("settings", self.session.as_ref())),
                        label: Some(shell_chrome_string("panelToggle.settings", is_de)),
                        active: self.right_panel_open && self.active_right_kind == RightPanelKind::Settings,
                        disabled: false,
                        kind: HitKind::Toggle,
                    }),
                    _ => None,
                };
                if let Some(item) = item {
                    if cursor.rect.is_none() {
                        let Some(item_w) = retained_chrome_group_item_width(theme, &item) else {
                            self.error = Some("Shell panel toggle exceeded the retained chrome boundary".to_string());
                            cursor.phase = 6;
                            return false;
                        };
                        cursor.right -= item_w;
                        cursor.rect = Some(Rect::new(cursor.right, btn_y, item_w, btn_h));
                    }
                    let Some(rect) = cursor.rect else { return false };
                    match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, true) {
                        RetainedChromeGroupStep::Pending => return false,
                        RetainedChromeGroupStep::Complete => {}
                        RetainedChromeGroupStep::Fault => self.error = Some("Shell panel toggle exceeded the retained glyph boundary".to_string()),
                    }
                    cursor.rect = None;
                    cursor.item += 1;
                    return false;
                }
                cursor.phase = 6;
            }
            6 => return true,
            _ => return false,
        }
        false
    }

    fn render_tutorial_bar_step(&mut self, cursor: &mut ShellChromeChildCursor, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32) -> bool {
        let Some(runtime) = self.tutorial.as_ref() else { return true };
        let mode = runtime.mode;
        let playhead_ms = runtime.playhead_ms;
        let duration_ms = runtime.definition.duration_ms;
        let rate = runtime.rate;
        let bar_h = tutorial_bar_height(theme);
        let y = theme.navbar_height;
        let btn_h = theme.control_height;
        let btn_y = y + (bar_h - btn_h) * 0.5;
        match cursor.phase {
            0 => {
                draw.push_solid([0.0, y, width, bar_h], theme.navbar);
                cursor.x = theme.padding_standard;
                cursor.right = width - theme.padding_standard;
                cursor.phase = 1;
            }
            1 => {
                draw.push_solid([0.0, y + bar_h - theme.stroke_hairline, width, theme.stroke_hairline], theme.border_normal);
                cursor.phase = 2;
            }
            2 if mode != TutorialMode::Recording => {
                let playing = mode == TutorialMode::Playing;
                let item = ChromeGroupItem { control_id: "shell.tutorial.playPause", icon_id: Some(if playing { "pause" } else { "play" }), label: None, active: false, disabled: false, kind: HitKind::NavbarItem };
                if cursor.rect.is_none() {
                    let Some(item_w) = retained_chrome_group_item_width(theme, &item) else {
                        cursor.phase = 9;
                        return false;
                    };
                    cursor.rect = Some(Rect::new(cursor.x, btn_y, item_w.max(btn_h), btn_h));
                }
                let Some(rect) = cursor.rect else { return false };
                match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, true) {
                    RetainedChromeGroupStep::Pending => return false,
                    RetainedChromeGroupStep::Complete => {}
                    RetainedChromeGroupStep::Fault => self.error = Some("Shell tutorial play item exceeded the retained glyph boundary".to_string()),
                }
                if self.chrome_build.clicked_this_frame && rect.contains(input.pointer_x, input.pointer_y) {
                    self.tutorial_toggle_play_pause();
                }
                cursor.x += rect.w + theme.gap_standard;
                cursor.rect = None;
                cursor.phase = 3;
            }
            2 => cursor.phase = 3,
            3 => {
                let item = ChromeGroupItem { control_id: "shell.tutorial.stop", icon_id: Some("square"), label: None, active: false, disabled: false, kind: HitKind::NavbarItem };
                if cursor.rect.is_none() {
                    let Some(item_w) = retained_chrome_group_item_width(theme, &item) else {
                        cursor.phase = 9;
                        return false;
                    };
                    cursor.rect = Some(Rect::new(cursor.x, btn_y, item_w.max(btn_h), btn_h));
                }
                let Some(rect) = cursor.rect else { return false };
                match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, true) {
                    RetainedChromeGroupStep::Pending => return false,
                    RetainedChromeGroupStep::Complete => {}
                    RetainedChromeGroupStep::Fault => self.error = Some("Shell tutorial stop item exceeded the retained glyph boundary".to_string()),
                }
                if self.chrome_build.clicked_this_frame && rect.contains(input.pointer_x, input.pointer_y) {
                    self.tutorial_stop();
                    cursor.rect = None;
                    cursor.phase = 9;
                    return false;
                }
                cursor.x += rect.w + theme.gap_standard;
                cursor.rect = None;
                cursor.phase = 4;
            }
            4 => {
                let current = (playhead_ms / 1000.0).max(0.0) as u64;
                let label = if mode == TutorialMode::Recording {
                    format!("REC {}:{:02}", current / 60, current % 60)
                } else {
                    let total = duration_ms / 1000;
                    format!("{}:{:02} / {}:{:02}", current / 60, current % 60, total / 60, total % 60)
                };
                match chrome_text_complete_step(draw, atlas, &label, cursor.x, btn_y + (btn_h + theme.font_size_small) * 0.5 - 1.0, (cursor.right - cursor.x).max(1.0), theme.font_size_small, theme.text, &mut cursor.glyph) {
                    Ok(false) => return false,
                    Ok(true) => {}
                    Err(()) => {
                        self.error = Some("Shell tutorial-bar text exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
                cursor.x += theme.font_size_small * 13.0 + theme.gap_standard * 2.0;
                cursor.phase = 5;
            }
            5 if mode != TutorialMode::Recording => {
                const RATES: [(f32, &str, &str); 4] = [(2.0, "2x", "shell.tutorial.rate.2"), (1.5, "1.5x", "shell.tutorial.rate.1_5"), (1.0, "1x", "shell.tutorial.rate.1"), (0.5, "0.5x", "shell.tutorial.rate.0_5")];
                let Some((value, label, control_id)) = RATES.get(cursor.item).copied() else {
                    cursor.phase = 6;
                    return false;
                };
                let item = ChromeGroupItem { control_id, icon_id: None, label: Some(label), active: (rate - value).abs() < 0.01, disabled: false, kind: HitKind::Toggle };
                if cursor.rect.is_none() {
                    let Some(item_w) = retained_chrome_group_item_width(theme, &item) else {
                        self.error = Some("Shell tutorial rate exceeded the retained chrome boundary".to_string());
                        cursor.phase = 6;
                        return false;
                    };
                    cursor.right -= item_w;
                    cursor.rect = Some(Rect::new(cursor.right, btn_y, item_w, btn_h));
                }
                let Some(rect) = cursor.rect else { return false };
                match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, true) {
                    RetainedChromeGroupStep::Pending => return false,
                    RetainedChromeGroupStep::Complete => {}
                    RetainedChromeGroupStep::Fault => self.error = Some("Shell tutorial rate exceeded the retained glyph boundary".to_string()),
                }
                if self.chrome_build.clicked_this_frame && rect.contains(input.pointer_x, input.pointer_y) {
                    if let Some(runtime) = self.tutorial.as_mut() {
                        runtime.rate = value;
                    }
                }
                cursor.rect = None;
                cursor.item += 1;
            }
            5 => cursor.phase = 6,
            6 => {
                let rect = Rect::new(cursor.x, btn_y, (cursor.right - cursor.x).max(24.0), btn_h);
                cursor.rect = Some(rect);
                draw.push_rounded([rect.x, btn_y + btn_h * 0.5 - 2.0, rect.w, 4.0], theme.border_normal, 2.0);
                cursor.phase = 7;
            }
            7 => {
                let Some(rect) = cursor.rect else {
                    cursor.phase = u16::MAX;
                    return false;
                };
                let knob_x = rect.x + rect.w * tutorial_scrub_progress(playhead_ms, duration_ms);
                draw.push_rounded([knob_x - 6.0, btn_y + btn_h * 0.5 - 6.0, 12.0, 12.0], theme.selected, 6.0);
                cursor.phase = 8;
            }
            8 => {
                let Some(rect) = cursor.rect else {
                    cursor.phase = u16::MAX;
                    return false;
                };
                input.register_hit(HitTarget { rect, event: None, control_id: Some("shell.tutorial.scrubber".into()), kind: HitKind::Slider, drag_axis: None, drag_data: None });
                if mode != TutorialMode::Recording && input.pointer_down && rect.contains(input.pointer_x, input.pointer_y) {
                    self.tutorial_seek(tutorial_scrub_target_ms(input.pointer_x, rect, duration_ms));
                }
                cursor.phase = 9;
            }
            9 => return true,
            _ => return false,
        }
        false
    }

    fn render_footer_step(&mut self, cursor: &mut ShellChromeChildCursor, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32, height: f32) -> bool {
        let y = height - theme.footer_height;
        let btn_h = theme.control_height;
        let btn_y = y + (theme.footer_height - btn_h) * 0.5;
        match cursor.phase {
            0 => {
                draw.push_solid([0.0, y, width, theme.footer_height], theme.navbar);
                cursor.x = theme.padding_standard;
                cursor.phase = 1;
            }
            1 => {
                let hovered = Rect::new(0.0, y, width, theme.footer_height).contains(input.pointer_x, input.pointer_y);
                draw.push_solid([0.0, y, width, theme.stroke_hairline], if hovered { theme.border_emphasized } else { theme.border_normal });
                cursor.phase = 2;
            }
            2 if self.session.is_some() => {
                if cursor.scalar == 0 {
                    let Some(utility) = footer_utility_at_path(&self.active_utilities, &cursor.path.0, cursor.depth) else {
                        cursor.phase = 3;
                        return false;
                    };
                    let x = cursor.x;
                    let result = render_footer_utility_node(cursor, &mut self.chrome_build, draw, atlas, icons, input, theme, x, btn_y, btn_h, utility, &self.utility_collection_expanded);
                    let Some((next_x, descend)) = (match result {
                        Ok(result) => result,
                        Err(()) => {
                            self.error = Some("Shell footer utility exceeded the retained chrome boundary".to_string());
                            cursor.phase = 4;
                            return false;
                        }
                    }) else {
                        return false;
                    };
                    cursor.x = next_x;
                    cursor.flag = descend;
                    cursor.scalar = 1;
                    return false;
                }
                if cursor.flag {
                    let Some(next_depth) = cursor.depth.checked_add(1).filter(|depth| *depth < cursor.path.0.len()) else {
                        cursor.phase = u16::MAX;
                        return false;
                    };
                    cursor.depth = next_depth;
                    cursor.path.0[next_depth] = 0;
                    cursor.flag = false;
                    cursor.scalar = 0;
                    return false;
                }
                let sibling_count = footer_utility_sibling_count(&self.active_utilities, &cursor.path.0, cursor.depth).unwrap_or(0);
                let next = cursor.path.0[cursor.depth] as usize + 1;
                if next < sibling_count {
                    cursor.path.0[cursor.depth] = next as u16;
                    cursor.scalar = 0;
                    return false;
                }
                if cursor.depth > 0 {
                    cursor.depth -= 1;
                    return false;
                }
                cursor.phase = 3;
            }
            2 => cursor.phase = 4,
            3 => {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let count = uncommitted_edit_count(&self.history_entries);
                    let x = cursor.x;
                    match render_sync_status_and_checkin(cursor, draw, atlas, icons, input, theme, self.sync_status.as_ref(), self.sync_bootstrap_progress.as_ref(), self.session.as_ref(), count, x, btn_y, btn_h) {
                        Ok(Some(next_x)) => cursor.x = next_x,
                        Ok(None) => return false,
                        Err(()) => {
                            self.error = Some("Shell sync footer exceeded the retained chrome boundary".to_string());
                            cursor.phase = 4;
                            return false;
                        }
                    }
                }
                cursor.phase = 4;
            }
            4 => return true,
            _ => return false,
        }
        false
    }

    fn render_overlay_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32, height: f32) -> bool {
        match cursor.phase {
            0 => {
                if matches!(self.overlay_state, OverlayState::Search | OverlayState::Find | OverlayState::Dropdown(_)) {
                    let rect = Rect::new(width * 0.5 - 200.0, theme.navbar_height + 8.0, 400.0, height * 0.55);
                    overlay.push_glass([rect.x, rect.y, rect.w, rect.h], theme.border_radius, theme.glass(Level::Menu));
                    cursor.rect = Some(rect);
                }
                cursor.phase = 1;
            }
            1 => {
                let label = match &self.overlay_state {
                    OverlayState::Search => Some(("Search", self.search_query.as_str())),
                    OverlayState::Find => Some(("Find in page", self.find_query.as_str())),
                    OverlayState::Dropdown(id) if id == "example" => Some(("Examples", "")),
                    _ => None,
                };
                if let (Some(rect), Some((title, query))) = (cursor.rect, label) {
                    let text = match cursor.item {
                        0 => Some((title, rect.y + theme.padding_standard + theme.font_size_body, theme.font_size_body, theme.text)),
                        1 if !query.is_empty() => Some((query, rect.y + theme.padding_standard * 2.0 + theme.font_size_body * 2.0, theme.font_size_small, theme.text_muted)),
                        1 => {
                            cursor.item = 2;
                            return false;
                        }
                        _ => None,
                    };
                    if let Some((text, y, size, color)) = text {
                        match chrome_text_complete_step(overlay, atlas, text, rect.x + theme.padding_standard, y, (rect.w - theme.padding_standard * 2.0).max(1.0), size, color, &mut cursor.glyph) {
                            Ok(false) => return false,
                            Ok(true) => {}
                            Err(()) => {
                                self.error = Some("Shell overlay text exceeded the retained glyph boundary".to_string());
                                cursor.glyph.reset();
                            }
                        }
                        cursor.item += 1;
                        return false;
                    }
                }
                cursor.item = 0;
                cursor.phase = 2;
            }
            2 => {
                if !self.render_context_menu_step(cursor, overlay, atlas, icons, input, theme, width, height) {
                    return false;
                }
                cursor.scalar = 0;
                cursor.item = 0;
                cursor.phase = 3;
            }
            3 => {
                if !self.render_chrome_tooltip_step(cursor, overlay, atlas, input, theme, width, height) {
                    return false;
                }
                cursor.scalar = 0;
                cursor.item = 0;
                cursor.phase = 4;
            }
            4 => {
                if !self.render_chrome_dialog_step(cursor, overlay, atlas, input, theme, width, height) {
                    return false;
                }
                cursor.scalar = 0;
                cursor.item = 0;
                cursor.phase = 5;
            }
            5 => {
                if !self.render_chrome_tour_step(cursor, overlay, atlas, input, theme, width, height) {
                    return false;
                }
                cursor.phase = 6;
            }
            6 => return true,
            _ => return false,
        }
        false
    }

    fn render_context_menu_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, viewport_w: f32, viewport_h: f32) -> bool {
        let Some(menu) = self.context_menu.as_ref() else { return true };
        let row_h = theme.control_height;
        let menu_w = 240.0_f32.min(viewport_w.max(0.0));
        let menu_h = (menu.items.len() as f32 * row_h + 8.0).min((viewport_h - 8.0).max(row_h + 8.0));
        let rect = Rect::new(menu.x.clamp(0.0, (viewport_w - menu_w).max(0.0)), menu.y.clamp(0.0, (viewport_h - menu_h).max(0.0)), menu_w, menu_h);
        match cursor.scalar {
            0 => {
                overlay.push_glass([rect.x, rect.y, rect.w, rect.h], theme.border_radius, theme.glass(Level::Menu));
                cursor.rect = Some(rect);
                cursor.scalar = 1;
            }
            1 => {
                overlay.push_scissor(rect);
                cursor.scalar = 2;
            }
            2 => {
                let Some(item) = menu.items.get(cursor.item) else {
                    cursor.scalar = 3;
                    return false;
                };
                let row = Rect::new(rect.x + 4.0, rect.y + 4.0 + cursor.item as f32 * row_h - menu.scroll_offset, rect.w - 8.0, row_h);
                if row.y + row.h > rect.y && row.y < rect.y + rect.h {
                    if item.separator {
                        overlay.push_solid([row.x + 4.0, row.y + row.h * 0.5, row.w - 8.0, theme.stroke_hairline], theme.text_muted);
                    } else {
                        if !cursor.flag {
                            let active = menu.active.first().copied() == Some(cursor.item);
                            overlay.push_rounded([row.x, row.y, row.w, row.h], if active { theme.accent } else { theme.button }, theme.border_radius);
                            let icon = item.icon.as_deref().unwrap_or("circle-dot");
                            chrome_icon(overlay, icons, icon, row.x + 8.0, row.y + (row.h - theme.font_size_body) * 0.5, theme.font_size_body, if item.disabled { theme.text_muted } else { theme.text });
                            cursor.flag = true;
                            return false;
                        }
                        match chrome_text_complete_step(
                            overlay,
                            atlas,
                            &item.label,
                            row.x + theme.font_size_body + theme.gap_standard * 2.0,
                            row.y + (row.h + theme.font_size_small) * 0.5 - 1.0,
                            (row.w - theme.font_size_body - theme.gap_standard * 3.0).max(1.0),
                            theme.font_size_small,
                            if item.disabled { theme.text_muted } else { theme.text },
                            &mut cursor.glyph,
                        ) {
                            Ok(false) => return false,
                            Ok(true) => {}
                            Err(()) => {
                                cursor.fault = true;
                                cursor.glyph.reset();
                            }
                        }
                        cursor.flag = false;
                        if !item.disabled {
                            input.register_hit(HitTarget { rect: row, event: item.action.clone(), control_id: Some(item.id.clone()), kind: HitKind::ContextMenu, drag_axis: None, drag_data: None });
                        }
                    }
                }
                cursor.item += 1;
            }
            3 => {
                overlay.pop_scissor();
                cursor.scalar = 4;
            }
            4 => {
                if cursor.fault {
                    self.error = Some("Shell context-menu text exceeded the retained glyph boundary".to_string());
                    cursor.fault = false;
                }
                return true;
            }
            _ => return false,
        }
        false
    }

    fn render_chrome_tooltip_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32, height: f32) -> bool {
        if self.chrome_build.dialog_open() {
            return true;
        }
        match cursor.scalar {
            0 => {
                let Some(id) = input.hovered_id.as_deref() else {
                    self.chrome_build.tooltip_hover = None;
                    return true;
                };
                let Some(title) = self.chrome_build.tooltip_titles.get(id) else {
                    self.chrome_build.tooltip_hover = None;
                    return true;
                };
                let now = chrome_now_ms();
                let restart = self.chrome_build.tooltip_hover.as_ref().is_none_or(|hover| hover.control_id != id);
                if restart {
                    self.chrome_build.tooltip_hover = Some(ChromeTooltipHover { control_id: id.into(), anchor_x: input.pointer_x, anchor_y: input.pointer_y, started_ms: now });
                    return true;
                }
                let Some(hover) = self.chrome_build.tooltip_hover.as_ref() else { return true };
                if !chrome_tooltip_ready(hover, now) {
                    return true;
                }
                let Some(text) = UiText::try_from_str(title) else { return true };
                let padding = theme.padding_standard * 0.5;
                let text_w = text.as_str().len() as f32 * theme.font_size_small * 0.6;
                let text_h = theme.font_size_small * 1.25;
                cursor.rect =
                    Some(Rect::new((hover.anchor_x + 12.0).clamp(0.0, (width - text_w - padding * 2.0).max(0.0)), (hover.anchor_y + 18.0).clamp(0.0, (height - text_h - padding * 2.0).max(0.0)), text_w + padding * 2.0, text_h + padding * 2.0));
                cursor.window = Some(text);
                cursor.scalar = 1;
            }
            1 => {
                let Some(rect) = cursor.rect else { return true };
                overlay.push_glass([rect.x, rect.y, rect.w, rect.h], theme.border_radius, theme.glass(Level::Menu));
                cursor.scalar = 2;
            }
            2 => {
                if let (Some(rect), Some(text)) = (cursor.rect, cursor.window.as_ref()) {
                    let padding = theme.padding_standard * 0.5;
                    match chrome_text_complete_step(overlay, atlas, text.as_str(), rect.x + padding, rect.y + (rect.h + theme.font_size_small) * 0.5 - 1.0, (rect.w - padding * 2.0).max(1.0), theme.font_size_small, theme.text, &mut cursor.glyph) {
                        Ok(false) => return false,
                        Ok(true) => {}
                        Err(()) => {
                            self.error = Some("Shell tooltip text exceeded the retained glyph boundary".to_string());
                            cursor.glyph.reset();
                        }
                    }
                }
                cursor.scalar = 3;
            }
            3 => return true,
            _ => return false,
        }
        false
    }

    fn render_chrome_dialog_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32, height: f32) -> bool {
        if cursor.flag {
            self.error = Some("Shell dialog text exceeded the retained glyph boundary".to_string());
            cursor.flag = false;
            return true;
        }
        let Some(request) = self.chrome_build.dialog_stack.last() else { return true };
        let dialog = Rect::new((width - 360.0) * 0.5, (height - 168.0) * 0.5, 360.0, 168.0);
        let pad = theme.padding_standard;
        let confirm = Rect::new(dialog.x + dialog.w - pad - 110.0, dialog.y + dialog.h - pad - theme.control_height, 110.0, theme.control_height);
        let cancel = Rect::new(dialog.x + pad, dialog.y + dialog.h - pad - theme.control_height, 90.0, theme.control_height);
        let text = match cursor.scalar {
            2 => Some((request.title.as_str(), dialog.x + pad, dialog.y + pad + theme.font_size_body, dialog.w - pad * 2.0, theme.font_size_body, theme.text)),
            3 => Some((request.body.as_str(), dialog.x + pad, dialog.y + pad + theme.font_size_body + theme.gap_standard + theme.font_size_small, dialog.w - pad * 2.0, theme.font_size_small, theme.text_muted)),
            5 => Some((request.cancel_label.as_str(), cancel.x + 10.0, cancel.y + (cancel.h + theme.font_size_small) * 0.5 - 1.0, cancel.w - 20.0, theme.font_size_small, theme.text)),
            7 => Some((request.confirm_label.as_str(), confirm.x + 10.0, confirm.y + (confirm.h + theme.font_size_small) * 0.5 - 1.0, confirm.w - 20.0, theme.font_size_small, theme.active_foreground)),
            _ => None,
        };
        if let Some((value, x, y, text_width, size, color)) = text {
            match chrome_text_complete_step(overlay, atlas, value, x, y, text_width, size, color, &mut cursor.glyph) {
                Ok(false) => return false,
                Ok(true) => {}
                Err(()) => {
                    cursor.glyph.reset();
                    cursor.flag = true;
                    return false;
                }
            }
            cursor.scalar += 1;
            return false;
        }
        match cursor.scalar {
            0 => overlay.push_solid([0.0, 0.0, width, height], Rgba::new(0.0, 0.0, 0.0, 0.35)),
            1 => {
                overlay.push_glass([dialog.x, dialog.y, dialog.w, dialog.h], theme.border_radius, theme.glass(Level::Dialog));
            }
            4 => overlay.push_rounded([cancel.x, cancel.y, cancel.w, cancel.h], theme.button, theme.border_radius),
            6 => overlay.push_rounded([confirm.x, confirm.y, confirm.w, confirm.h], theme.accent, theme.border_radius),
            8 => input.register_hit(HitTarget { rect: cancel, event: None, control_id: Some(format!("shell.dialog.{}.cancel", request.id)), kind: HitKind::Button, drag_axis: None, drag_data: None }),
            9 => input.register_hit(HitTarget { rect: confirm, event: Some(request.confirm_action.clone()), control_id: Some(format!("shell.dialog.{}.confirm", request.id)), kind: HitKind::Button, drag_axis: None, drag_data: None }),
            10 => {
                if self.chrome_build.clicked_this_frame {
                    let (x, y) = (input.pointer_x, input.pointer_y);
                    if confirm.contains(x, y) || cancel.contains(x, y) || !dialog.contains(x, y) {
                        self.chrome_build.close_topmost_dialog();
                    }
                }
            }
            11 => return true,
            _ => return false,
        }
        cursor.scalar += 1;
        false
    }

    fn render_chrome_tour_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32, height: f32) -> bool {
        if cursor.flag {
            self.error = Some("Shell tour text exceeded the retained glyph boundary".to_string());
            cursor.flag = false;
            return true;
        }
        let Some(session) = self.session.as_ref() else {
            self.chrome_build.tour_state = None;
            return true;
        };
        let Some(introduction) = session.app.introduction.as_ref() else { return true };
        let Some(step_index) = self.chrome_build.tour_state.as_ref().map(|state| state.step_index) else { return true };
        let Some(step) = introduction.steps.get(step_index) else { return true };
        let rect = Rect::new((width - 320.0) * 0.5, (height - 168.0) * 0.5, 320.0, 168.0);
        let pad = theme.padding_standard;
        let skip = Rect::new(rect.x + pad, rect.y + rect.h - pad - theme.control_height, 70.0, theme.control_height);
        let next = Rect::new(rect.x + rect.w - pad - 90.0, rect.y + rect.h - pad - theme.control_height, 90.0, theme.control_height);
        let text = match cursor.scalar {
            2 => Some((step.title.resolve(self.active_terminology(), self.active_locale()), rect.x + pad, rect.y + pad + theme.font_size_body, rect.w - pad * 2.0, theme.font_size_body, theme.text)),
            3 => Some((step.body.resolve(self.active_terminology(), self.active_locale()), rect.x + pad, rect.y + pad + theme.font_size_body + theme.gap_standard + theme.font_size_small, rect.w - pad * 2.0, theme.font_size_small, theme.text_muted)),
            5 => Some((shell_chrome_string("introduction.skip", self.locale_id == "de"), skip.x + 10.0, skip.y + (skip.h + theme.font_size_small) * 0.5 - 1.0, skip.w - 20.0, theme.font_size_small, theme.text)),
            7 if step.interactions.is_empty() => Some((
                shell_chrome_string(if step_index + 1 == introduction.steps.len() { "introduction.done" } else { "introduction.next" }, self.locale_id == "de"),
                next.x + 10.0,
                next.y + (next.h + theme.font_size_small) * 0.5 - 1.0,
                next.w - 20.0,
                theme.font_size_small,
                theme.active_foreground,
            )),
            _ => None,
        };
        if let Some((value, x, y, text_width, size, color)) = text {
            match chrome_text_complete_step(overlay, atlas, value, x, y, text_width, size, color, &mut cursor.glyph) {
                Ok(false) => return false,
                Ok(true) => {}
                Err(()) => {
                    cursor.glyph.reset();
                    cursor.flag = true;
                    return false;
                }
            }
            cursor.scalar += 1;
            return false;
        }
        match cursor.scalar {
            0 => overlay.push_solid([0.0, 0.0, width, height], Rgba::new(0.0, 0.0, 0.0, 0.35)),
            1 => {
                overlay.push_glass([rect.x, rect.y, rect.w, rect.h], theme.border_radius, theme.glass(Level::Dialog));
            }
            4 => overlay.push_rounded([skip.x, skip.y, skip.w, skip.h], theme.button, theme.border_radius),
            6 if step.interactions.is_empty() => overlay.push_rounded([next.x, next.y, next.w, next.h], theme.accent, theme.border_radius),
            8 => input.register_hit(HitTarget { rect: skip, event: None, control_id: Some("shell.tour.skip".into()), kind: HitKind::Button, drag_axis: None, drag_data: None }),
            9 if step.interactions.is_empty() => input.register_hit(HitTarget { rect: next, event: None, control_id: Some("shell.tour.next".into()), kind: HitKind::Button, drag_axis: None, drag_data: None }),
            10 => {
                if self.chrome_build.clicked_this_frame {
                    if next.contains(input.pointer_x, input.pointer_y) && step.interactions.is_empty() {
                        self.chrome_build.advance_introduction(introduction.steps.len());
                    } else if skip.contains(input.pointer_x, input.pointer_y) {
                        self.chrome_build.skip_introduction();
                    }
                }
            }
            11 => return true,
            _ => {}
        }
        cursor.scalar += 1;
        false
    }















    //#region SilhouetteContent

    #[cfg(test)]
    fn intersect_content_rect(left: Rect, right: Rect) -> Option<Rect> {
        let x = left.x.max(right.x);
        let y = left.y.max(right.y);
        let x2 = (left.x + left.w).min(right.x + right.w);
        let y2 = (left.y + left.h).min(right.y + right.h);
        (x2 > x && y2 > y).then(|| Rect::new(x, y, x2 - x, y2 - y))
    }



    //#endregion SilhouetteContent



    /// 💬️ Paints the armed tooltip (item 1) — `AtPointer` placement/dismissal policy sourced from
    /// `ui_wgpu::wgpu::OverlayKind::Tooltip` via a scratch `UiTree` (empty; `Point` anchors never touch it).
    #[cfg(test)]
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
    #[cfg(test)]
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

























    // #region ActionsRail






    /// 📝️ The effective value of one arg (staged if present, else the declared default).
    fn effective_arg_value(&self, window_id: &str, action_id: &str, arg: &semio_framework::ActionArgDef) -> Option<Value> {
        self.staged_action_args.get(&Self::staged_key(window_id, action_id)).and_then(|map| map.get(&arg.id).cloned()).or_else(|| arg.default.as_ref().map(dsl_value_as_json))
    }

    fn arg_default(&self, window_kind_id: &str, action_id: &str, arg_id: &str) -> Option<Value> {
        window_action_definition(&self.session.as_ref()?.app, window_kind_id, action_id)?.args.iter().find(|arg| arg.id == arg_id)?.default.as_ref().map(dsl_value_as_json)
    }










    // #endregion



    /// 📏️ Shared width pass for a menu level — also used to size a submenu BEFORE deciding which side of
    /// its parent row it opens on (see `render_context_menu_level`'s flip-left check).
    #[cfg(test)]
    fn context_menu_level_width(items: &[ContextMenuItem], theme: &Theme) -> f32 {
        let mut w = 180.0;
        for item in items.iter().filter(|item| !item.separator || !item.label.is_empty()) {
            let label_w = item.label.chars().count() as f32 * theme.font_size_body * 0.55;
            let shortcut_w = item.shortcut.as_ref().map(|s| s.chars().count() as f32 * theme.font_size_small * 0.55 + 16.0).unwrap_or(0.0);
            w = f32::max(w, 56.0_f32 + label_w + shortcut_w);
        }
        w
    }

    #[cfg(test)]
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
#[cfg(any(target_arch = "wasm32", test))]
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

#[cfg(all(not(target_arch = "wasm32"), test))]
struct FilePrefsStore {
    path: std::path::PathBuf,
    cache: std::cell::RefCell<HashMap<String, String>>,
    pending_load: std::cell::RefCell<Option<std::sync::mpsc::Receiver<HashMap<String, String>>>>,
    dirty: std::cell::Cell<bool>,
    flush_state: std::sync::Arc<std::sync::Mutex<FilePrefsFlushState>>,
}

#[cfg(all(not(target_arch = "wasm32"), test))]
#[derive(Default)]
struct FilePrefsFlushState {
    latest: Option<HashMap<String, String>>,
    running: bool,
}

const OS_SHELL_CONFIG_MAX_BYTES: usize = 64 * 1024;
#[cfg(test)]
const OS_SHELL_PREFS_FILE_MAX_BYTES: usize = OS_SHELL_CONFIG_MAX_BYTES + 4 * 1024;

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

#[cfg(all(not(target_arch = "wasm32"), test))]
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
                use std::io::Read;
                let cache = system_fs::File::open(load_path)
                    .ok()
                    .filter(|file| file.metadata().ok().is_some_and(|metadata| metadata.len() <= OS_SHELL_PREFS_FILE_MAX_BYTES as u64))
                    .and_then(|file| {
                        let mut raw = String::new();
                        file.take((OS_SHELL_PREFS_FILE_MAX_BYTES + 1) as u64).read_to_string(&mut raw).ok()?;
                        (raw.len() <= OS_SHELL_PREFS_FILE_MAX_BYTES).then_some(raw)
                    })
                    .and_then(|raw| serde_json::from_str::<HashMap<String, String>>(&raw).ok())
                    .unwrap_or_default();
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

#[cfg(all(not(target_arch = "wasm32"), test))]
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

#[cfg(all(not(target_arch = "wasm32"), test))]
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

#[cfg(test)]
fn prefs_get_from(store: &impl PrefsStore, key: &str) -> Option<String> {
    let raw = store.get(OS_SHELL_CONFIG_STORAGE_KEY)?;
    if raw.len() > OS_SHELL_CONFIG_MAX_BYTES {
        return None;
    }
    serde_json::from_str::<Value>(&raw).ok()?.get("preferences")?.get(key)?.as_str().map(ToOwned::to_owned)
}

#[cfg(test)]
fn prefs_set_in(store: &mut impl PrefsStore, key: &str, value: &str) {
    let mut config = store
        .get(OS_SHELL_CONFIG_STORAGE_KEY)
        .filter(|raw| raw.len() <= OS_SHELL_CONFIG_MAX_BYTES)
        .and_then(|raw| serde_json::from_str::<Value>(&raw).ok())
        .filter(|config| config.get("version").and_then(Value::as_u64) == Some(1))
        .unwrap_or_else(empty_os_shell_config);
    if !config.get("preferences").is_some_and(Value::is_object) {
        config["preferences"] = serde_json::json!({});
    }
    config["preferences"][key] = Value::String(value.to_string());
    if let Ok(raw) = serde_json::to_string(&config) {
        if raw.len() <= OS_SHELL_CONFIG_MAX_BYTES {
            store.set(OS_SHELL_CONFIG_STORAGE_KEY, &raw);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn native_pref_field_path(key: &str) -> Option<std::path::PathBuf> {
    if key.len() > SHELL_CHROME_IO_FIELD_BYTES {
        return None;
    }
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    key.hash(&mut hasher);
    Some(native_prefs_file_path().with_file_name("ui-prefs.fields").join(format!("{:016x}.field", hasher.finish())))
}

#[cfg(not(target_arch = "wasm32"))]
fn native_pref_read_page(key: &str) -> Option<String> {
    let path = native_pref_field_path(key)?;
    let max_bytes = if key == OS_SHELL_CONFIG_STORAGE_KEY { OS_SHELL_CONFIG_MAX_BYTES } else { SHELL_CHROME_IO_FIELD_BYTES };
    let page = semio_framework_os_services::storage_worker_read_fixed_file_page(&path, max_bytes).ok()?;
    String::from_utf8(page).ok()
}

#[cfg(not(target_arch = "wasm32"))]
fn native_pref_write_page(key: &str, value: &str) -> bool {
    let Some(path) = native_pref_field_path(key) else { return false };
    let max_bytes = if key == OS_SHELL_CONFIG_STORAGE_KEY { OS_SHELL_CONFIG_MAX_BYTES } else { SHELL_CHROME_IO_FIELD_BYTES };
    if value.len() > max_bytes {
        return false;
    }
    semio_framework_os_services::storage_worker_write_fixed_file_page(&path, value.as_bytes(), max_bytes).is_ok()
}

fn raw_prefs_get(key: &str) -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        PREFS_STORE.with(|store| store.borrow().get(key))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        native_pref_read_page(key)
    }
}

fn raw_prefs_set(key: &str, value: &str) {
    #[cfg(target_arch = "wasm32")]
    PREFS_STORE.with(|store| store.borrow_mut().set(key, value));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = native_pref_write_page(key, value);
    }
}

fn prefs_get(key: &str) -> Option<String> {
    let raw = raw_prefs_get(OS_SHELL_CONFIG_STORAGE_KEY)?;
    if raw.len() > OS_SHELL_CONFIG_MAX_BYTES {
        return None;
    }
    serde_json::from_str::<Value>(&raw).ok()?.get("preferences")?.get(key)?.as_str().map(ToOwned::to_owned)
}

fn prefs_set(key: &str, value: &str) {
    let mut config = raw_prefs_get(OS_SHELL_CONFIG_STORAGE_KEY)
        .filter(|raw| raw.len() <= OS_SHELL_CONFIG_MAX_BYTES)
        .and_then(|raw| serde_json::from_str::<Value>(&raw).ok())
        .filter(|config| config.get("version").and_then(Value::as_u64) == Some(1))
        .unwrap_or_else(empty_os_shell_config);
    if !config.get("preferences").is_some_and(Value::is_object) {
        config["preferences"] = serde_json::json!({});
    }
    config["preferences"][key] = Value::String(value.to_string());
    if let Ok(raw) = serde_json::to_string(&config) {
        if raw.len() <= OS_SHELL_CONFIG_MAX_BYTES {
            raw_prefs_set(OS_SHELL_CONFIG_STORAGE_KEY, &raw);
        }
    }
}

fn prefs_get_bounded(key: &str) -> Option<String> {
    if key.len() > SHELL_CHROME_IO_FIELD_BYTES {
        return None;
    }
    prefs_get(key).filter(|value| value.len() <= SHELL_CHROME_IO_FIELD_BYTES)
}

fn prefs_set_bounded(key: &str, value: &str) {
    if key.len() <= SHELL_CHROME_IO_FIELD_BYTES && value.len() <= SHELL_CHROME_IO_FIELD_BYTES {
        prefs_set(key, value);
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
#[cfg(test)]
struct ShellPrefLocks {
    appearance: Option<String>,
    locale: Option<String>,
    terminology: Option<String>,
    theme_id: Option<String>,
}

fn env_lock(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|value| !value.is_empty())
}

#[cfg(test)]
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
    custom_drivers: HashMap<String, OsUiDriver>,
    custom_themes: HashMap<String, String>,
    keybinding_overrides: HashMap<String, String>,
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
            custom_drivers: HashMap::new(),
            custom_themes: HashMap::new(),
            keybinding_overrides: HashMap::new(),
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
static CHROME_PREFS: crate::interpreter::WorkerCell<Option<ChromePrefsState>> = crate::interpreter::WorkerCell::new();

fn default_compute_worker_count() -> u32 {
    std::thread::available_parallelism().map(|n| n.get() as u32).unwrap_or(1)
}

fn project_chrome_prefs(preferences: UiPreferences, worker_count: u32) -> ChromePrefsState {
    let custom_drivers = preferences.custom_drivers;
    let ui_layout = match preferences.layout { Some(OsUiChromeLayout::Tablet) => "tablet", _ => "desktop" }.to_string();
    let theme_id = preferences.theme_id.unwrap_or_else(|| "semio".to_string());
    let custom_themes = preferences.custom_themes.into_iter().map(|(id, theme)| (id, theme.config.to_string())).collect();
    let keybinding_overrides = preferences.keybinding_overrides;
    ChromePrefsState {
        ui_layout,
        theme_id,
        custom_drivers,
        custom_themes,
        keybinding_overrides,
        #[cfg(all(test, not(target_arch = "wasm32")))]
        draft_theme: None,
        worker_count,
    }
}

fn load_chrome_prefs() -> ChromePrefsState {
    let worker_count = prefs_get(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY).and_then(|raw| raw.parse::<u32>().ok()).filter(|count| *count >= 1).unwrap_or_else(default_compute_worker_count);
    project_chrome_prefs(read_ui_preferences(), worker_count)
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

#[cfg(test)]
pub(crate) fn active_theme_id() -> String {
    with_chrome_prefs(|prefs| prefs.theme_id.clone())
}

pub(crate) fn set_active_theme_id(id: &str) {
    with_chrome_prefs(|prefs| prefs.theme_id = id.to_string());
}

#[cfg(test)]
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


#[cfg(test)]
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

/// 🎨️ The "mono" premade's real chrome palette (`ui/styling/theme/🔣️.json`), resolved once
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




//#endregion 🗣️ChromeI18n

//#region 💾️PrefsSync
#[derive(Clone, Default, PartialEq)]
struct UiPrefsSnapshot {
    appearance_id: String,
    locale_id: String,
    terminology_id: String,
    driver_id: String,
    theme_id: String,
    ui_layout: String,
    custom_drivers: HashMap<String, OsUiDriver>,
    custom_themes: HashMap<String, String>,
    keybinding_overrides: HashMap<String, String>,
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
            custom_drivers: state.chrome_build.preferences.custom_drivers.clone(),
            custom_themes: state.chrome_build.preferences.custom_themes.clone(),
            keybinding_overrides: state.chrome_build.preferences.keybinding_overrides.clone(),
            worker_count: state.chrome_build.preferences.worker_count,
        }
    }
}

#[derive(Clone, Default)]
struct UiPreferencesEventLog {
    version: u8,
    events: Vec<UiPreferencesConfigMutation>,
}

fn decode_ui_preferences_event_log(raw: &str) -> Option<UiPreferencesEventLog> {
    let value: Value = serde_json::from_str(raw).ok()?;
    if value.get("version").and_then(Value::as_u64) != Some(1) {
        return None;
    }
    let events = value
        .get("events")?
        .as_array()?
        .iter()
        .map(|event| decode_ui_preferences_config_mutation_json(&event.to_string()))
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    Some(UiPreferencesEventLog { version: 1, events })
}

fn encode_ui_preferences_event_log(log: &UiPreferencesEventLog) -> String {
    let events: Vec<Value> = log.events.iter().map(|event| dsl_value_as_json(&dsl::ToValue::to_value(event))).collect();
    serde_json::json!({ "version": log.version, "events": events }).to_string()
}

fn read_ui_preferences_event_log() -> UiPreferencesEventLog {
    prefs_get(UI_PREFERENCES_CONFIG_SCHEMA)
        .and_then(|raw| decode_ui_preferences_event_log(&raw))
        .unwrap_or_else(|| UiPreferencesEventLog { version: 1, events: Vec::new() })
}

fn replay_ui_preferences(log: &UiPreferencesEventLog) -> UiPreferences {
    let mut preferences = UiPreferences::default();
    for mutation in &log.events {
        apply_ui_preferences_config_mutation(&mut preferences, mutation).expect("canonical OS UI-preferences mutation must apply");
    }
    preferences
}

fn read_ui_preferences() -> UiPreferences {
    replay_ui_preferences(&read_ui_preferences_event_log())
}

fn appearance_id(value: Option<OsUiAppearance>) -> String {
    match value {
        Some(OsUiAppearance::Light) => "light",
        Some(OsUiAppearance::Dark) => "dark",
        _ => "system",
    }
    .to_string()
}

fn locale_id(value: Option<OsUiLocale>) -> String {
    match value {
        Some(OsUiLocale::De) => "de",
        _ => "en",
    }
    .to_string()
}

fn custom_themes_from(preferences: &UiPreferences) -> HashMap<String, String> {
    preferences
        .custom_themes
        .iter()
        .filter_map(|(id, theme)| {
            let mut config = theme.config.as_object()?.clone();
            config.insert("id".to_string(), Value::String(theme.theme_id.clone()));
            config.insert("label".to_string(), Value::String(theme.label.clone()));
            Some((id.clone(), Value::Object(config).to_string()))
        })
        .collect()
}

fn canonical_custom_themes(preferences: &ChromePrefsState) -> HashMap<String, OsUiTheme> {
    preferences
        .custom_themes
        .iter()
        .map(|(id, raw)| {
            let mut config = serde_json::from_str::<Value>(raw).unwrap_or_else(|_| Value::Object(Default::default()));
            let label = config.get("label").and_then(Value::as_str).unwrap_or(id).to_string();
            if let Some(object) = config.as_object_mut() {
                object.remove("id");
                object.remove("label");
            }
            (id.clone(), OsUiTheme { theme_id: id.clone(), label, config })
        })
        .collect()
}

fn persist_ui_preferences(state: &ShellState, previous: Option<&UiPrefsSnapshot>) {
    let mut log = read_ui_preferences_event_log();
    let changed = |current: &str, prior: fn(&UiPrefsSnapshot) -> &str| previous.is_none_or(|snapshot| current != prior(snapshot));
    if env_lock("SEMIO_LOCKED_APPEARANCE").is_none() && changed(&state.appearance_id, |snapshot| &snapshot.appearance_id) {
        log.events.push(set_appearance(Some(match state.appearance_id.as_str() { "light" => OsUiAppearance::Light, "dark" => OsUiAppearance::Dark, _ => OsUiAppearance::System })));
    }
    if changed(&state.chrome_build.preferences.ui_layout, |snapshot| &snapshot.ui_layout) {
        log.events.push(set_layout(Some(if state.chrome_build.preferences.ui_layout == "tablet" { OsUiChromeLayout::Tablet } else { OsUiChromeLayout::Desktop })));
    }
    if changed(&state.driver_id, |snapshot| &snapshot.driver_id) {
        log.events.push(set_driver(Some(state.driver_id.clone())));
    }
    if previous.is_none_or(|snapshot| snapshot.custom_drivers != state.chrome_build.preferences.custom_drivers) {
        let before = previous.map(|snapshot| &snapshot.custom_drivers).cloned().unwrap_or_default();
        let after = &state.chrome_build.preferences.custom_drivers;
        let ids: std::collections::BTreeSet<String> = before.keys().chain(after.keys()).cloned().collect();
        let projected = replay_ui_preferences(&log);
        for id in ids {
            let next = after.get(&id).cloned();
            if projected.custom_drivers.get(&id) != next.as_ref() {
                log.events.push(set_custom_driver(id, next));
            }
        }
    }
    if env_lock("SEMIO_LOCKED_LOCALE").is_none() && changed(&state.locale_id, |snapshot| &snapshot.locale_id) {
        log.events.push(set_locale(Some(if state.locale_id == "de" { OsUiLocale::De } else { OsUiLocale::En })));
    }
    if env_lock("SEMIO_LOCKED_TERMINOLOGY").is_none() && changed(&state.terminology_id, |snapshot| &snapshot.terminology_id) {
        log.events.push(set_terminology(Some(state.terminology_id.clone())));
    }
    if env_lock("SEMIO_LOCKED_THEME").is_none() && changed(&state.chrome_build.preferences.theme_id, |snapshot| &snapshot.theme_id) {
        log.events.push(set_theme(Some(state.chrome_build.preferences.theme_id.clone())));
    }
    if previous.is_none_or(|snapshot| snapshot.custom_themes != state.chrome_build.preferences.custom_themes) {
        let before = previous.map(|snapshot| &snapshot.custom_themes).cloned().unwrap_or_default();
        let after = canonical_custom_themes(&state.chrome_build.preferences);
        let ids: std::collections::BTreeSet<String> = before.keys().chain(after.keys()).cloned().collect();
        let projected = replay_ui_preferences(&log);
        for id in ids {
            let next = after.get(&id).cloned();
            if projected.custom_themes.get(&id) != next.as_ref() {
                log.events.push(set_custom_theme(id, next));
            }
        }
    }
    if previous.is_none_or(|snapshot| snapshot.keybinding_overrides != state.chrome_build.preferences.keybinding_overrides) {
        let before = previous.map(|snapshot| &snapshot.keybinding_overrides).cloned().unwrap_or_default();
        let after = &state.chrome_build.preferences.keybinding_overrides;
        let ids: std::collections::BTreeSet<String> = before.keys().chain(after.keys()).cloned().collect();
        let projected = replay_ui_preferences(&log);
        for id in ids {
            let next = after.get(&id).cloned();
            if projected.keybinding_overrides.get(&id) != next.as_ref() {
                log.events.push(set_keybinding_override(id, next));
            }
        }
    }
    prefs_set(UI_PREFERENCES_CONFIG_SCHEMA, &encode_ui_preferences_event_log(&log));
}

impl ShellState {
    /// 💾️ Loads persisted uiPrefs into `self` exactly once per process, mirroring `os-shell.tsx`'s
    /// boot-time `locks.appearance ?? readStoredUiChromeAppearance()` fallback chain (`:862-868`).
    /// A locked pref (`SEMIO_LOCKED_*`) wins over storage, matching `resolveShellLocks`.
    #[cfg(test)]
    fn load_ui_prefs_once(&mut self) {
        if self.chrome_present.preferences_loaded {
            return;
        }
        self.chrome_present.preferences_loaded = true;
        let locks = shell_pref_locks();
        let preferences = read_ui_preferences();
        self.appearance_id = locks.appearance.clone().unwrap_or_else(|| appearance_id(preferences.appearance));
        self.locale_id = locks.locale.clone().unwrap_or_else(|| locale_id(preferences.locale));
        self.terminology_id = locks.terminology.clone().or(preferences.terminology).unwrap_or_else(|| UI_TERMINOLOGY_NATIVE.to_string());
        self.driver_id = preferences.driver_id.unwrap_or_else(|| "default".to_string());
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
    #[cfg(test)]
    fn persist_ui_prefs_if_changed(&mut self) {
        let snapshot = UiPrefsSnapshot::capture(self);
        if self.chrome_present.last_synced_preferences.as_ref() == Some(&snapshot) {
            return;
        }
        persist_ui_preferences(self, self.chrome_present.last_synced_preferences.as_ref());
        prefs_set(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY, &snapshot.worker_count.to_string());
        self.chrome_present.last_synced_preferences = Some(snapshot);
    }
}
//#endregion 💾️PrefsSync

//#region 🧪️UiPrefsThemesI18nTests
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../🧪️tests/🔬️wgpu-ui-prefs-themes-i18n/🦀️.rs"]
mod ui_prefs_themes_i18n_tests;
//#endregion 🧪️UiPrefsThemesI18nTests
// #endregion 💾️🎨️🌐️ UiPrefsThemesI18n

//#region 🧪️ChromeOverlaysAndTourTests
#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-chrome-overlays-tour/🦀️.rs"]
mod chrome_overlays_tour_tests;
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


#[cfg(not(target_arch = "wasm32"))]
async fn pick_folder() -> Option<String> {
    ui_host::select_native_paths(ui_host::NativeFileDialogRequest::folder()).await.into_iter().next().map(|path| path.display().to_string())
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
#[cfg(any(not(target_arch = "wasm32"), test))]
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
#[cfg(any(not(target_arch = "wasm32"), test))]
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
    args: Option<Value>,
) -> Vec<ActionDescriptor> {
    match payload.and_then(decode_data_url) {
        Some(bytes) => vec![fallback_action_descriptor(controller_id, fallback_action, &bytes, "video", &args.unwrap_or_else(|| serde_json::json!({})))],
        None => Vec::new(),
    }
}

#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-media-frames/🦀️.rs"]
mod media_frames_tests;
//#endregion RequestMediaFrames

#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-context-menu-keyboard/🦀️.rs"]
mod context_menu_keyboard_tests;
