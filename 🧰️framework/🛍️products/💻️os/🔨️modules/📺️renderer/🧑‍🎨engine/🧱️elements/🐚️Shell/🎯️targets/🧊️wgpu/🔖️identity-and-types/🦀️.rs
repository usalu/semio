//! 🔖️ Identity, replay refusal, check-in and chrome type vocabulary.
//!
//! The pure half of the wgpu shell: framework tab/panel ids, find items, context-menu helpers, directory and hub
//! bootstrap helpers, document open/install phases, replay refusal, conflict and check-in laws, and the
//! chrome-thread build/present state types every other shell chunk builds on.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

pub(crate) const FRAMEWORK_DISPLAY_WINDOWS_TAB_ID: &str = "framework.display.windows";
const COMPOSE_WINDOW_TEMPLATE_MIME: &str = "application/x-compose-window-template";
pub(crate) const FRAMEWORK_DISPLAY_LAYOUT_TAB_ID: &str = "framework.display.layout";
pub(crate) const FRAMEWORK_SETTINGS_GENERAL_TAB_ID: &str = "framework.settings.general";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct WindowTemplateDragPayload {
    pub(crate) window_kind_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) template_id: Option<String>,
}

pub(crate) fn window_template_drag_data(window_kind_id: &str, template_id: Option<&str>) -> HashMap<String, String> {
    let payload = WindowTemplateDragPayload { window_kind_id: window_kind_id.to_string(), template_id: template_id.map(str::to_string) };
    HashMap::from([(COMPOSE_WINDOW_TEMPLATE_MIME.to_string(), serde_json::to_string(&payload).expect("window template payload is infallible"))])
}

pub(crate) fn decode_window_template_drag(data: &HashMap<String, String>) -> Option<WindowTemplateDragPayload> {
    serde_json::from_str(data.get(COMPOSE_WINDOW_TEMPLATE_MIME)?).ok()
}

pub(crate) fn dsl_value_as_json(value: &DslValue) -> Value {
    serde_json::to_value(value).unwrap_or(Value::Null)
}

pub(crate) fn optional_dsl_value_as_json(value: Option<DslValue>) -> Option<Value> {
    value.map(|entry| dsl_value_as_json(&entry))
}

/// 🎨️ Byte-identical to React's `FRAMEWORK_SETTINGS_THEME_TAB_ID` (`ui/js/react/index.tsx:8807`) — the
/// `PanelTabKind::SettingsTheme` variant this maps to already existed in `framework/core/rs/lib.rs`
/// but was completely unwired on this side (see `build_settings_theme_ui`/`right_tabs`).
pub(crate) const FRAMEWORK_SETTINGS_THEME_TAB_ID: &str = "framework.settings.theme";
pub(crate) const FRAMEWORK_CHAT_PANEL_ID: &str = "framework.chat";
/// 💬️ Byte-identical to React's own `<ol id="framework.chat.feed">` (`💬️AgentChatPanel/🟦️.tsx`) — the
/// transcript's container, addressed as one node by a probe and by the accessibility mirror.
pub(crate) const FRAMEWORK_CHAT_FEED_ID: &str = "framework.chat.feed";
/// 📃️ How many conversation rows the panel body PAINTS. React's feed scrolls and pins itself to the
/// bottom on every new entry, so the human sees the newest window; this is that window's size on a
/// target whose panel body is a fixed-credit layout tree. The bridge keeps up to
/// `AGENT_CONVERSATION_MAX_ENTRIES` (200) behind it — nothing is discarded, only unpainted.
pub(crate) const AGENT_CHAT_VISIBLE_ENTRIES: usize = 24;
/// 🌳️ Byte-identical to React's `FRAMEWORK_CATEGORY_DISPLAY_ID` (`🛠️ShellHelpers/🟦️.tsx:224`) — the
/// bottom-left anchor's Display branch, the one anchor whose app tabs nest under a category root.
pub(crate) const FRAMEWORK_CATEGORY_DISPLAY_ID: &str = "framework.category.display";
/// 🎛️ Byte-identical to React's `FRAMEWORK_CATEGORY_COMMAND_ID` — the bottom-middle Command branch.
pub(crate) const FRAMEWORK_CATEGORY_COMMAND_ID: &str = "framework.category.command";
/// 🛠️ Byte-identical to React's `FRAMEWORK_CATEGORY_TOOL_ID` — the bottom-middle Tool branch, ordered
/// left of Command. wgpu has no mode-level tool leaves to nest under it yet (`📓️w1h-…` remaining gaps).
pub(crate) const FRAMEWORK_CATEGORY_TOOL_ID: &str = "framework.category.tool";
/// 🔄️ Byte-identical to React's `frameworkSyncTab` leaf id (`🏛️ShellHost/🟦️.tsx`'s `"s-sync-status"`),
/// the same id the footer check-in pill already registers.
pub(crate) const FRAMEWORK_SYNC_PANEL_TAB_ID: &str = "s-sync-status";
/// 🧵️ Byte-identical to React's Task Manager panel and row-action controller id.
pub(crate) const FRAMEWORK_TASK_MANAGER_PANEL_ID: &str = "os.task-manager";
/// 🎛️ One command category's leaf id under the Command branch — React's `command.category.<id>`.
pub(crate) const FRAMEWORK_COMMAND_CATEGORY_TAB_PREFIX: &str = "command.category.";
/// ⚙️ Byte-identical to React's `FRAMEWORK_SETTINGS_PANEL_ID`/`FRAMEWORK_SETTINGS_BRANCH_ID`
/// (`📌️ChromePanels/🟦️.tsx`, `🛠️ShellHelpers/🟦️.tsx`) — the bottom-right Settings BRANCH whose children
/// are the five framework settings leaves, with the app's own Settings-group tabs nested ahead of them.
pub(crate) const FRAMEWORK_SETTINGS_PANEL_ID: &str = "framework.settings";
pub(crate) const FRAMEWORK_SETTINGS_KEYBINDINGS_TAB_ID: &str = "framework.settings.keybindings";
pub(crate) const FRAMEWORK_SETTINGS_DEFAULT_APPS_TAB_ID: &str = "framework.settings.default-apps";
pub(crate) const FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID: &str = "framework.settings.conflicts";
/// 🛍️ Byte-identical to React's `FRAMEWORK_MARKETPLACE_TAB_ID` — the bottom-right Marketplace leaf.
pub(crate) const FRAMEWORK_MARKETPLACE_TAB_ID: &str = "framework.marketplace";
/// 🛍️ The Marketplace's windowed roster section — React's `framework.marketplace.source.<sourceId>` for this renderer's one source.
pub(crate) const MARKETPLACE_LOCAL_SECTION_ID: &str = "framework.marketplace.source.local";
/// 🛍️ The node ledger one Marketplace window may materialise — the body-wide tree window budget less the windowed section itself.
pub(crate) const MARKETPLACE_WINDOW_NODE_BUDGET: usize = ui_contract::TREE_WINDOW_BODY_NODE_BUDGET - 1;
/// 🛠️ One mode tool's leaf id under the Tool branch — React's `tool.<id>` (`🛠️ShellHelpers/🟦️.tsx`'s
/// `buildToolTabs`/`toolIdFromPanelTabId`). The leaf tab IS the activation control.
pub(crate) const FRAMEWORK_TOOL_PANEL_TAB_PREFIX: &str = "tool.";
/// 📱️ Byte-identical to React's synthetic mobile "App" tab (`🏛️ShellHost/🟦️.tsx`'s `mobilePanelTabs`).
pub(crate) const FRAMEWORK_MOBILE_APP_TAB_ID: &str = "framework.mobile.app";
pub(crate) const CHROME_ICON_TINY: f32 = ui_wgpu::wgpu::chrome::ICON_TINY;
pub(crate) const PANEL_CHROME_ICON_TINY: f32 = ui_wgpu::wgpu::chrome::ICON_TINY;
pub(crate) const PANEL_CHROME_GRIP_SIZE: f32 = ui_wgpu::wgpu::chrome::ICON_TREE_ROW;
/// 🫳️ The insert-preview pill's width — React's `panelTabInsertPreviewClass`'s `w-0.5`
/// (`🧭️PanelTabBar/🟦️.tsx`), the 2 logical px Tailwind's `0.5` step resolves to.
pub(crate) const TAB_INSERT_PREVIEW_WIDTH_PX: f32 = 2.0;

#[derive(Clone, Debug)]
pub struct SearchPaletteItem {
    pub id: String,
    pub label: String,
    /// 🔍️ The palette's secondary line — a command's resolved chords, exactly what React's `UISearch`
    /// item puts there (`description: commandKeybindingChords(...)`). Ranked at weight 1 (see
    /// `filtered_search_items`), so typing a chord finds its command.
    pub description: Option<String>,
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

pub(crate) const SHELL_FIND_ITEM_CAPACITY: usize = 256;
const SHELL_FIND_PAYLOAD_BYTES: usize = 1024 * 1024;

/// 🧺️ Fixed FIFO ownership for one generation of scene find items.
pub struct ShellFindItems {
    pub(crate) slots: Box<[Option<ShellFindItem>; SHELL_FIND_ITEM_CAPACITY]>,
    pub(crate) head: usize,
    pub(crate) len: usize,
    payload_bytes: usize,
    pub(crate) generation: u64,
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

    pub(crate) fn try_push_at(&mut self, generation: u64, item: ShellFindItem) -> Result<(), ShellFindItem> {
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

    pub(crate) fn try_push(&mut self, item: ShellFindItem) -> Result<(), ShellFindItem> {
        self.try_push_at(self.generation, item)
    }

    pub(crate) fn pop_front(&mut self) -> Option<ShellFindItem> {
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

    pub(crate) fn begin_next_generation(&mut self) -> bool {
        if self.len != 0 {
            return false;
        }
        let Some(generation) = self.generation.checked_add(1) else { return false };
        self.head = 0;
        self.generation = generation;
        true
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &ShellFindItem> {
        (0..self.len).filter_map(move |offset| self.head.checked_add(offset).map(|index| index % self.slots.len()).and_then(|index| self.slots[index].as_ref()))
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.payload_bytes == 0 && self.slots.iter().all(Option::is_none)
    }

    pub(crate) fn close_step(&mut self) -> bool {
        if self.pop_front().is_some() {
            return false;
        }
        self.terminal_is_empty()
    }
}

#[derive(Clone, Copy)]
struct ActiveShellFindItems {
    pub(crate) pointer: std::ptr::NonNull<ShellFindItems>,
    pub(crate) generation: u64,
}

thread_local! {
    static ACTIVE_FIND_ITEMS: std::cell::Cell<Option<ActiveShellFindItems>> = const { std::cell::Cell::new(None) };
}

/// 🪢️ Non-nesting single-worker binding for the exact retained collector.
struct ShellFindItemBinding {
    pub(crate) active: ActiveShellFindItems,
}

impl ShellFindItems {
    pub(crate) fn bind(&mut self) -> Result<ShellFindItemBinding, ()> {
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
pub(crate) fn context_menu_action_kind_str(kind: semio_framework::ActionKind) -> String {
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
/// `context_menu_group_label` (falling back to the spec's own label if the category is unrecognized) and get
/// a default folder icon when the spec left `icon` unset.
///
/// 🗂️ `context_menu_group_label`, not `ribbon_parent_label`: the overflow row `organize_context_menu`
/// synthesizes (`menu.group.more`) is the one group id outside the closed 20-id ribbon-parent taxonomy,
/// so resolving through the taxonomy alone rendered it with an EMPTY label. Its React twin resolves the
/// same id through `ui.contextMenu.more` (`🛠️ShellHelpers/🟦️.tsx`'s `contextMenuGroupLabel`).
pub(crate) fn shell_context_menu_item_from_spec(spec: ui_wgpu::wgpu::ContextMenuItemSpec, controller_id: &str, is_de: bool) -> ContextMenuItem {
    let ui_wgpu::wgpu::ContextMenuItemSpec { id, label, icon, shortcut, disabled, reason, separator, checked, destructive, action, args, children, .. } = spec;
    let is_group_row = id.starts_with(ui_wgpu::wgpu::CONTEXT_MENU_GROUP_ID_PREFIX);
    let label = ui_wgpu::wgpu::context_menu_group_label(&id, is_de).map(str::to_string).or(label);
    let icon = icon.or_else(|| is_group_row.then(|| "folder".to_string()));
    ContextMenuItem {
        id,
        label: label.unwrap_or_default(),
        icon,
        shortcut,
        destructive: destructive.unwrap_or(false),
        action: action.map(|action| ActionDescriptor { controller_id: controller_id.into(), action, args }),
        children: children.unwrap_or_default().into_iter().map(|child| shell_context_menu_item_from_spec(child, controller_id, is_de)).collect(),
        disabled: disabled.unwrap_or(false),
        reason,
        separator: separator.unwrap_or(false),
        checked: checked.unwrap_or(false),
    }
}

/// 🪟️ Binds one action to the window INSTANCE whose body declared it, through the `windowId`
/// argument `dispatch_action` resolves `ActionAddress::window_instance_id` from.
///
/// Without it the address falls back to the focused window, and the guest refuses an action that
/// window's kind does not own — measured on 6118 as
/// `handle_action promise failed: window kind procedural-main does not own action addGeneration`
/// for a press on the Generations window's own `Add Generation` row
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-input-hit-runtime-2026-09-13.md` §10.3).
/// Context menus and retained window bodies share this one rule.
/// 🪟️ The window instance a plugin action is dispatched in. A retained body stamps its own surface id
/// as `windowId`; a PANEL body is not a window, so its action runs in the active window's context
/// (React's rule — panel verbs are app-level), exactly like an action that names no window.
pub(crate) fn action_window_instance_id(requested: Option<&str>, panel_leaves: &[&str], view_window: Option<&str>, active_window: Option<&str>, first_window_kind: &str) -> String {
    requested.filter(|id| !panel_leaves.contains(id)).or(view_window).or(active_window).unwrap_or(first_window_kind).to_string()
}

pub(crate) fn scope_action_to_window(action: &mut ActionDescriptor, window_id: &str) {
    let mut args = match action.args.take() {
        Some(semio_framework_value::DslValue::Object(entries)) => entries,
        _ => Vec::new(),
    };
    args.retain(|(key, _)| key != "windowId");
    args.push(("windowId".into(), semio_framework_value::DslValue::String(window_id.into())));
    action.args = Some(semio_framework_value::DslValue::Object(args));
}

pub(crate) fn scope_context_menu_items(items: &mut [ContextMenuItem], window_id: &str) {
    for item in items {
        if let Some(action) = item.action.as_mut() {
            scope_action_to_window(action, window_id);
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
pub(crate) fn mint_shell_session_id() -> String {
    format!("wgpu-{}", semio_framework_os_kernel::os_identity::time_ordered_id())
}

/// 🌱️ Native: `S_HUB_URL`/`S_USER`/`S_DATA_DIR` straight off the process env (contract §C0 — "wgpu
/// native reads `S_*` directly"). No hub env ⇒ `None` ⇒ every identity/binding/directory code path
/// below is a no-op, preserving today's local-only behaviour byte-for-byte.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn resolve_identity_env() -> Option<IdentityEnv> {
    IdentityEnv::from_process_env()
}

/// 🌱️ Browser: the same decision from the boot environment the TypeScript host handed over
/// (`semioWgpuSetHubEnv`). A page that was never told about a hub stays local-only, exactly as a
/// native process without `S_*` does.
#[cfg(target_arch = "wasm32")]
pub(crate) fn resolve_identity_env() -> Option<IdentityEnv> {
    BOOT_HUB_ENV.with(|cell| cell.borrow().as_ref().map(|(_, _, data_dir)| IdentityEnv { data_dir: data_dir.as_ref().map(std::path::PathBuf::from) }))
}

/// 📇️ The base the shell's `DirectoryClient` prefixes every hub path with.
///
/// ⚖️ Native talks to the hub origin directly (the credential's own `hub_origin`, filled in by
/// `DirectoryClient::authenticated`, so this is only the pre-identity default). The browser goes
/// through the SAME same-origin reverse proxy prefix the React shell's worker uses
/// (`requestBaseUrl: "/_semio/hub"`, `🏪️store/👷️worker/🟦️.ts`) — same-origin is what lets the
/// session cookie travel at all, so the two renderers cannot disagree about where the hub is.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn shell_directory_request_base_url() -> String {
    std::env::var("S_HUB_URL").ok().filter(|value| !value.trim().is_empty()).unwrap_or_default()
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn shell_directory_request_base_url() -> String {
    "/_semio/hub".to_string()
}

/// 🖥️ This shell session's device-instance id, reduced to the hub's own admission grammar
/// (`^[A-Za-z0-9._:-]+$`, 128 bytes). It is derived from the session id rather than from anything
/// about the human — never a fingerprint — and a session id carrying a character the route would
/// refuse is filtered rather than rejected, so a sign-in can never fail on a local id format.
pub(crate) fn shell_hub_device_instance_id(session_id: &str) -> String {
    let filtered: String = session_id.chars().filter(|character| character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | ':' | '-')).take(crate::hub_sign_in::HUB_SIGN_IN_DEVICE_INSTANCE_MAX_BYTES).collect();
    if filtered.is_empty() {
        "wgpu-shell".to_string()
    } else {
        filtered
    }
}

/// 🔐️ The hub origin the hub workspace's local-bootstrap entry names — slice WG6's twin of AU3
/// §4.4's `hubBootstrapOriginV1`. It is the HUB's own origin, never the page's: a `POST` to the UI
/// origin reaches the SPA's index route and answers HTML, which is why a sign-in could not have
/// worked from React's shell until AU3 separated the two. A declared origin that is not a bare
/// scheme+authority is refused rather than trimmed, and an undeclared hub falls back to the request
/// base url the directory client already uses.
pub(crate) fn shell_hub_bootstrap_origin() -> String {
    let declared = shell_directory_request_base_url();
    crate::hub_sign_in::parse_hub_origin(&declared).unwrap_or(declared)
}

/// 🌐️ The native directory client's network byte budget per minute. It carries the largest bounded answer the
/// client takes — a hub document's execution-target component (`DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES`,
/// 64 MiB) — twice over: at the old 10 MB a 17.7 MB block component was refused mid-body
/// (`os.directory-client exhausted its network_bytes_per_min budget`, gate run 23 on hub 7800 B2).
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const SHELL_DIRECTORY_NETWORK_BYTES_PER_MINUTE: u64 = 2 * semio_framework_os_kernel::os_directory::DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES;

/// 🔌️ The one transport the shell's directory client is built on for this target.
#[cfg(target_arch = "wasm32")]
pub(crate) fn shell_directory_transport() -> ShellDirectoryTransport {
    ShellDirectoryTransport::default()
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
pub(crate) fn shell_actor(identity: Option<&Identity>, session_id: &str, instance_id: u32) -> String {
    match identity {
        Some(identity) => actor_id(identity, session_id),
        None => format!("wgpu-{instance_id}"),
    }
}

/// 🔗️ ticket §2 — `attach_sync_backbone`/`open_document`'s default binding decision: `[Hub, Folder]`
/// when an identity AND a space both exist, `[Folder]` with a data dir but no identity, `[]`
/// otherwise. Pure and free-standing so the decision is unit-testable without a live `ShellState`.
pub(crate) fn default_persistence_bindings(identity: Option<&Identity>, space_id: Option<&str>, data_dir: Option<&std::path::Path>, surface: Option<&str>) -> Vec<PersistenceBinding> {
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

/// 🏛️ The persistence transports the document host this shell build links actually serves. Both
/// builds link the kernel's `ArtifactHost`; its native actor owns the folder event log and the hub
/// WebSocket, while its browser actor has no filesystem and dials its hub through the page's socket door.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ShellDocumentTransports {
    pub(crate) folder: bool,
    pub(crate) hub: bool,
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) const SHELL_DOCUMENT_TRANSPORTS: ShellDocumentTransports = ShellDocumentTransports { folder: true, hub: true };
#[cfg(target_arch = "wasm32")]
pub(crate) const SHELL_DOCUMENT_TRANSPORTS: ShellDocumentTransports = ShellDocumentTransports { folder: false, hub: true };

/// 🛂️ Refuses, before any actor exists, a binding the host would accept and then never serve: an
/// unserved folder or hub binding would hand the plugin a document that silently persists nowhere.
/// An empty binding set is the declared `EphemeralLocalOnly` class and every host serves it.
pub(crate) fn document_bindings_admitted(transports: ShellDocumentTransports, bindings: &[PersistenceBinding]) -> Result<(), &'static str> {
    for binding in bindings {
        match binding {
            PersistenceBinding::Hub { .. } if !transports.hub => return Err("document-binding.hub-transport-unavailable"),
            PersistenceBinding::Folder { .. } if !transports.folder => return Err("document-binding.folder-unavailable"),
            _ => {}
        }
    }
    Ok(())
}

/// 📇️ ticket §C6 — maps an `os.directory.<verb>` action id + its relayed JSON args onto a
/// `DirectoryCommand`, mirroring the React shell's `directoryCommandFromAction` verb-for-verb
/// (`📓️w2-c-report.md`) — same action ids, same field names, same `share-link` → `create-invite`
/// sugar (contract §C1 has no directory-schema command kind of its own for it). `None` for an
/// unrecognized verb, defensive against a foreign/future caller.
pub(crate) fn directory_command_from_action(action_id: &str, args: Option<&Value>) -> Option<DirectoryCommand> {
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
#[derive(Debug)]
pub(crate) struct OpenArtifactRelayTarget {
    pub(crate) artifact_ref: String,
    pub(crate) dialect: semio_framework_artifact_reference::ArtifactDialect,
    pub(crate) role: semio_framework::AppRole,
    pub(crate) plugin_id: Option<String>,
    pub(crate) app_id: Option<String>,
    pub(crate) document_id: Option<String>,
    pub(crate) space_id: Option<String>,
    pub(crate) schema: Option<String>,
}

pub(crate) fn open_artifact_relay_target(action_id: &str, args: Option<&Value>) -> Result<OpenArtifactRelayTarget, &'static str> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    let args = args.and_then(Value::as_object).ok_or("opening.invalid-args")?;
    let text = |field: &str| args.get(field).and_then(Value::as_str).filter(|value| !value.trim().is_empty()).map(str::to_string);
    let raw_artifact_ref = text("artifactRef").ok_or("opening.invalid-artifact-ref")?;
    let (dialect, surface_role) = if raw_artifact_ref.contains('#') {
        let (dialect, role) = semio_framework::parse_surface_app_id(&raw_artifact_ref).map_err(|_| "opening.invalid-artifact-ref")?;
        (dialect, Some(role))
    } else {
        (semio_framework_artifact_reference::ArtifactDialect::parse_coordinate(&raw_artifact_ref).map_err(|_| "opening.invalid-artifact-ref")?, None)
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

/// 👤️ The name a peer's roster shows for this shell — the twin of React's `presenceClientIdentity`
/// (`🛠️ShellHelpers/🟦️.tsx`): the signed-in person's display name, else a guest named after the actor's tail.
pub(crate) fn shell_presence_label(identity: Option<&Identity>, actor: &str) -> String {
    match identity.map(|identity| identity.display_name.trim()).filter(|name| !name.is_empty()) {
        Some(name) => name.to_string(),
        None => format!("Guest {}", actor.chars().rev().take(4).collect::<Vec<_>>().into_iter().rev().collect::<String>().to_uppercase()),
    }
}

/// 👥️ Projects only Hub-normalized peers for the shell's currently attached surface.
pub(crate) fn presence_peer_rows_for_surface(peers: &[PresencePeer], attached_surface: Option<&str>, target_surface: &str) -> Vec<ui_wgpu::wgpu::PresencePeerRow> {
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
            is_agent: matches!(peer.principal_kind, Some(store_sync::os_spr::PresencePrincipalKind::Agent)),
            activity: None,
        })
        .collect()
}

/// 🪐️ ticket §C4/§6 — the space's own artifact-index document: its id is always the literal `"index"`
/// (one per hub space). Which plugin app edits it, and the document schema it is opened with, are named
/// by that plugin's own `os.space-index` topic contribution ([`program_space_index`]), never by this shell.
pub(crate) const SPACE_INDEX_DOCUMENT_ID: &str = "index";

/// 📇️ The space index a program names through its `os.space-index` topic contribution.
pub(crate) fn program_space_index(program: &ProgramBridgeEntry) -> Option<semio_framework::manifest::SpaceIndexContribution> {
    semio_framework::manifest::SpaceIndexContribution::from_topic_contributions(&program.manifest.topic_contributions)
}

/// 🪐️ The app of `role` that serves the space index `program` names.
pub(crate) fn find_space_index_app(program: &ProgramBridgeEntry, role: semio_framework::manifest::AppRole) -> Option<&AppDefinition> {
    let index = program_space_index(program)?;
    find_dialect_app(program, &index.dialect, role)
}

/// 🪐️ Whether `session` is the app that serves the space index its own program names.
pub(crate) fn session_is_space_index(session: &ActiveSession, plugins: &[ProgramBridgeEntry]) -> bool {
    plugins.iter().find(|program| program.plugin_id == session.plugin_id).and_then(program_space_index).is_some_and(|index| index.dialect == session.app.dialect)
}

//#region 📣️ReplayRefusal
/// 📣️ Every way this shell refuses a guest's `replayShellCommand` — a durable gesture the user made — named by the ONE
/// schema-first vocabulary React's ShellHost reads too (`🏛️ShellHost/📣️replay-refusal/🔣️.json`): a refusal is a warning notice
/// in the shell's tongue carrying its fault code, never a silent drop.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum ReplayRefusalReasonV1 {
    SignInRequired,
    SpaceRequired,
    SpaceIndexRequired,
    InvalidRequest,
    RouterNotReady,
    OpenRejected,
    ViewOnlyAccess,
    UnroutedCommand,
    LocalCatalogUnavailable,
}

/// 🗣️ One reason's notice text per supported locale.
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReplayRefusalLabelV1 {
    pub en: String,
    pub de: String,
}

/// 🧪️ One shared vector both shells' laws answer: a reason in a locale reads `text` under `code`.
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReplayRefusalVectorV1 {
    pub reason: ReplayRefusalReasonV1,
    pub locale: String,
    pub text: String,
    pub code: String,
}

/// 📜️ The shared vocabulary file.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct ReplayRefusalVocabularyV1 {
    pub schema: String,
    pub description: String,
    pub code_prefix: String,
    pub reasons: BTreeMap<String, ReplayRefusalLabelV1>,
    pub vectors: Vec<ReplayRefusalVectorV1>,
}

/// 📜️ The shared replay-refusal vocabulary, parsed once from the file both shells read.
pub(crate) static REPLAY_REFUSAL_V1: std::sync::LazyLock<ReplayRefusalVocabularyV1> =
    std::sync::LazyLock::new(|| serde_json::from_str(include_str!("../../../../🏛️ShellHost/📣️replay-refusal/🔣️.json")).expect("the shared replay-refusal vocabulary parses"));

impl ReplayRefusalReasonV1 {
    pub(crate) const ALL: [Self; 9] =
        [Self::SignInRequired, Self::SpaceRequired, Self::SpaceIndexRequired, Self::InvalidRequest, Self::RouterNotReady, Self::OpenRejected, Self::ViewOnlyAccess, Self::UnroutedCommand, Self::LocalCatalogUnavailable];

    /// 🔑️ The reason's vocabulary key — its own kebab-case wire name.
    pub(crate) fn key(self) -> String {
        serde_json::to_value(self).ok().and_then(|value| value.as_str().map(str::to_owned)).unwrap_or_default()
    }

    /// 🗣️ The notice text in `locale`; only an unknown locale reads English.
    pub(crate) fn notice_text(self, locale: &str) -> String {
        REPLAY_REFUSAL_V1.reasons.get(&self.key()).map(|label| if locale == "de" { label.de.clone() } else { label.en.clone() }).unwrap_or_default()
    }

    /// 🩺️ The fault code the notice carries, so a probe, a law and the console name the same gate.
    pub(crate) fn code(self) -> String {
        format!("{}{}", REPLAY_REFUSAL_V1.code_prefix, self.key())
    }
}

/// 🌱️ The one kind an `os.create-space-artifact` replay may create — ShellHost's `spaceArtifactCreationRequestFromAction`
/// over this shell's creation catalog: exactly `{kindChoice, name}`, the choice a canonical (decode ∘ encode)
/// `ArtifactKindChoice` equal to one kind the ready catalog of `space_id` offers. Answers `(kindId, name)`.
pub(crate) async fn space_artifact_creation_replay_choice(args: Option<&Value>, catalog: Option<&SpaceArtifactCreationCatalogV1>, space_id: &str) -> Result<(String, String), ReplayRefusalReasonV1> {
    let invalid = ReplayRefusalReasonV1::InvalidRequest;
    let args = args.and_then(Value::as_object).filter(|args| args.len() == 2).ok_or(invalid)?;
    let (Some(choice), Some(name)) = (args.get("kindChoice").and_then(Value::as_str), args.get("name").and_then(Value::as_str)) else { return Err(invalid) };
    let decoded = semio_framework::manifest::decode_artifact_kind_choice(choice).await.map_err(|_| invalid)?;
    if semio_framework::manifest::encode_artifact_kind_choice(&decoded).await != choice {
        return Err(invalid);
    }
    let catalog = catalog.filter(|catalog| catalog.space_id == space_id).ok_or(ReplayRefusalReasonV1::RouterNotReady)?;
    for kind in &catalog.kinds {
        let offered = semio_framework::manifest::ArtifactKindChoice {
            kind_id: kind.kind_id.clone(),
            schema: kind.schema.clone(),
            dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: kind.dialect.artifact_kind.clone(), standard: kind.dialect.standard.clone(), subset: kind.dialect.subset.clone() },
            label: LocalizedLabel::native(&kind.label.en, &kind.label.de),
        };
        if semio_framework::manifest::encode_artifact_kind_choice(&offered).await == choice {
            return Ok((kind.kind_id.clone(), name.to_string()));
        }
    }
    Err(invalid)
}
//#endregion 📣️ReplayRefusal

/// 📇️ §5/§6 — a direct manifest scan for the one app a plugin declares for a given
/// `(dialect, role)`, mirroring the React shell's `findDialectApp`.
pub(crate) fn find_dialect_app<'a>(program: &'a ProgramBridgeEntry, dialect: &semio_framework_artifact_reference::ArtifactDialect, role: semio_framework::manifest::AppRole) -> Option<&'a AppDefinition> {
    program.manifest.apps.iter().find(|app| &app.dialect == dialect && app.role == role)
}

/// 🎬️ The phase a lazy plugin install is in, the one value chrome reads while it runs. `Failed`
/// keeps the message so a refused open explains itself instead of vanishing into the debug log. Both
/// targets run the same phase machine — only the step that actually acquires the module differs
/// ([`ShellState::acquire_plugin_program`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShellPluginInstallPhase {
    Resolving,
    Loading,
    Cancelled,
    Failed(String),
}

/// 🚪️ Where the one frame-pumped document open stands: a hub document's component is being resolved by
/// the serving catalog generation, the guest's app instance is being created, the document's canonical
/// checkpoint pair is being fetched and loaded into it, or the open settled as cancelled or failed — a
/// settled record stays until the user closes its band, like a settled plugin install.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShellDocumentOpenPhase {
    Resolving,
    Instantiating,
    Seeding,
    Cancelled,
    Failed(String),
}

/// 📨️ What one detached open step answered: a hub document's resolved owner — the plugin and app its lease
/// names, the program to mount (`None` when the local one already is the lease's component) and the
/// checkpoint the lease authorizes — an app instance, or a seeded document with the canonical pair its
/// guest now holds (`None` for a document bound to no hub).
pub(crate) enum ShellDocumentOpenAnswer {
    #[cfg(not(target_arch = "wasm32"))]
    Resolved {
        plugin_id: String,
        app_id: String,
        program: Option<ProgramBridgeEntry>,
        checkpoint: semio_framework_os_kernel::os_directory::DocumentOpenCheckpointV1,
    },
    Instantiated(u32),
    Seeded(Option<semio_framework_os_kernel::os_directory::CanonicalCheckpointPairV1>),
}

/// ⏳️ The resolution step a hub document open is on, as its band reads it ([`ShellDocumentOpening::resolve_step`]).
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn document_open_resolve_step_code(step: semio_framework_os_kernel::os_directory::client::ExecutionTargetModuleStep) -> u8 {
    use semio_framework_os_kernel::os_directory::client::ExecutionTargetModuleStep;
    match step {
        ExecutionTargetModuleStep::Lease => 0,
        ExecutionTargetModuleStep::Component => 1,
        ExecutionTargetModuleStep::Descriptor => 2,
        ExecutionTargetModuleStep::Verified => 3,
    }
}

/// 🎯️ The document half of an open: what the relay asked for, and — once a hub resolution read the
/// document's lease — the checkpoint that lease authorizes.
pub(crate) struct ShellDocumentOpenTarget {
    pub(crate) document_id: String,
    pub(crate) schema: String,
    pub(crate) checkpoint: Option<semio_framework_os_kernel::os_directory::DocumentOpenCheckpointV1>,
}

/// 🛟️ How long a rebootstrapping document waits before asking the hub for its pair again after a transient refusal.
pub(crate) const SYNC_RESEED_RETRY_MS: f64 = 1_000.0;

/// 🛟️ One rebootstrap reseed: the control the actor raised, the pair fetch in flight, and when to ask again after a failure.
pub(crate) struct ShellSyncReseed {
    pub(crate) control: semio_framework_os_kernel::os_directory::RebootstrapRequired,
    pub(crate) pending: Option<ShellDetached<Result<semio_framework_os_kernel::os_directory::CanonicalCheckpointPairV1, String>>>,
    pub(crate) retry_at_ms: f64,
}

/// 🪢️ What a hub document's seed step asks: the document's canonical checkpoint pair, admitted only as the
/// checkpoint its execution-target lease authorizes, over the shell's signed-in directory client.
#[derive(Clone)]
pub(crate) struct ShellHubDocumentSeed {
    pub(crate) client: std::sync::Arc<ShellDirectoryClient>,
    pub(crate) ctx: OperationContext,
    pub(crate) scope: semio_framework_os_kernel::os_directory::DocumentScope,
    pub(crate) checkpoint: semio_framework_os_kernel::os_directory::DocumentOpenCheckpointV1,
}

/// 🧷️ An open whose host half is prepared — the prior document retired, the socket surface and
/// execution target bound — waiting for the guest to hold the document before its actor binds.
pub(crate) struct ShellPreparedDocumentOpen {
    pub(crate) document_id: String,
    pub(crate) schema: String,
    pub(crate) bindings: Vec<PersistenceBinding>,
    pub(crate) backbone_uri: Option<String>,
    pub(crate) seed: Option<ShellHubDocumentSeed>,
    pub(crate) plugin: ProgramBridgeEntry,
    pub(crate) session: ActiveSession,
}

/// 🚪️ The one document open this shell runs at a time (`os.open-artifact`), advanced by the frame pump
/// ([`ShellState::advance_document_opening`]). Every guest turn it waits on — the app instance, the
/// genesis load — runs detached ([`ShellDetached`]), so the frame loop keeps building frames, painting
/// this open's band (phase, step, elapsed, cancel) and taking input while the guest works; before, the
/// whole open held the shell for ~17 s in a debug build (ticket 26/09/23 slice WG8).
pub struct ShellDocumentOpening {
    pub label: String,
    pub phase: ShellDocumentOpenPhase,
    pub cancel_requested: bool,
    pub started_at_ms: f64,
    pub(crate) plugin_id: String,
    pub(crate) app_id: String,
    pub(crate) document: Option<ShellDocumentOpenTarget>,
    pub(crate) pending: Option<ShellDetached<Result<ShellDocumentOpenAnswer, crate::program_bridge::ProgramFault>>>,
    pub(crate) prepared: Option<ShellPreparedDocumentOpen>,
    /// 🛑️ The token the detached hub resolution's requests check; cancelling the open cancels it.
    pub(crate) cancel: Option<CancelToken>,
    /// ⏳️ The resolution step the detached hub resolution last reported (lease, component, descriptor, verified).
    pub(crate) resolve_step: std::sync::Arc<std::sync::atomic::AtomicU8>,
}

impl ShellDocumentOpening {
    /// 🚪️ An open whose first detached step is `pending`, in `phase`.
    pub(crate) fn new(label: String, phase: ShellDocumentOpenPhase, plugin_id: String, app_id: String, document: Option<ShellDocumentOpenTarget>, pending: Option<ShellDetached<Result<ShellDocumentOpenAnswer, crate::program_bridge::ProgramFault>>>) -> Self {
        Self { label, phase, cancel_requested: false, started_at_ms: chrome_now_ms(), plugin_id, app_id, document, pending, prepared: None, cancel: None, resolve_step: std::sync::Arc::default() }
    }

    /// ⏳️ Whether a step is still out; a settled record only waits for its band to be closed.
    pub fn running(&self) -> bool {
        matches!(self.phase, ShellDocumentOpenPhase::Resolving | ShellDocumentOpenPhase::Instantiating | ShellDocumentOpenPhase::Seeding)
    }
}

/// 🎬️ One retained lazy plugin install. Minted by [`ShellState::install_plugin`], cleared by the same
/// call once it settles; `cancel` is the token every step of the install checks, so a cancel that
/// arrives mid-read stops the install at the next step boundary rather than after it has committed.
pub struct ShellPluginInstall {
    pub plugin_id: String,
    pub phase: ShellPluginInstallPhase,
    pub cancel: CancelToken,
}

/// 📁️ The plugin-modules root every resident program's artifact path sits two levels under —
/// `<modules_root>/<plugin_id>/<file>.wasm` is `load_wasm_plugins`'s own layout convention, already
/// relied on by the renderer's extension activation (`🧊️renderer/🦀️.rs`'s `activate_extensions_of`).
/// Reading it back off a resident entry is what lets the shell install a plugin without the renderer
/// having to hand it a second copy of a path it already owns.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn plugin_modules_root_of(programs: &[ProgramBridgeEntry]) -> Option<std::path::PathBuf> {
    programs.iter().find_map(|entry| entry.wasm_artifact_path().and_then(|path| path.parent()).and_then(|dir| dir.parent()).map(std::path::Path::to_path_buf))
}

/// 🎯 Resolves the descriptor-bound canonical surface id this local selection may request. It is a
/// preference, never an authority: a local WGPU selection verifies no execution-target bytes and so
/// can never mint a `DocumentExecutionTargetLeaseFieldsV1`.
pub(crate) fn document_socket_surface_from_descriptor(plugin_id: &str, package_id: Option<&str>, manifest: &semio_framework::PluginManifest, app: &AppDefinition, window_kind_id: &str) -> Result<String, String> {
    let _package_id = package_id.filter(|value| !value.is_empty()).ok_or_else(|| "document open requires a verified package descriptor".to_string())?;
    if plugin_id != manifest.plugin_id || manifest.version.is_empty() || !manifest.apps.iter().any(|candidate| candidate.id == app.id) || !app.window_kinds.iter().any(|window| window.id == window_kind_id) {
        return Err("document open local plugin selection is not descriptor-bound".into());
    }
    Ok(semio_framework::manifest::surface_app_id(&app.dialect, app.role))
}

/// 🌐️ The three parts a `remote://host:port/space/document` sync-card uri names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct RemoteBackboneUri {
    pub(crate) host_port: String,
    pub(crate) space_id: String,
    pub(crate) document_id: String,
}

/// 🌐️ The wgpu twin of the React shell's `parseRemoteBackboneUri` (`💻️os/🟦️.ts`): everything up to
/// the first slash is the host, up to the second the space, the rest the document; a uri missing a
/// part names no hub document.
pub(crate) fn parse_remote_backbone_uri(uri: &str) -> Option<RemoteBackboneUri> {
    let rest = uri.strip_prefix("remote://")?;
    let first = rest.find('/').filter(|index| *index > 0)?;
    let second = rest[first + 1..].find('/').map(|index| index + first + 1).filter(|index| *index > 0)?;
    Some(RemoteBackboneUri { host_port: rest[..first].to_string(), space_id: rest[first + 1..second].to_string(), document_id: rest[second + 1..].to_string() })
}

/// 🪪️ Admits a hub-selected execution-target lease only for the package this shell actually mounted:
/// the same plugin, the same package, the same component digest, the document's schema and the
/// surface the shell requested. Anything else would lend the document actor a kind identity for
/// code that is not running here.
pub(crate) fn document_execution_target_admitted(
    lease: &semio_framework_os_kernel::os_directory::DocumentExecutionTargetLeaseFieldsV1,
    plugin_id: &str,
    package_id: Option<&str>,
    component_sha256: Option<&str>,
    artifact_schema: &str,
    surface_id: &str,
) -> Result<(), &'static str> {
    if lease.package.plugin_id != plugin_id {
        return Err("document-execution-target.plugin-mismatch");
    }
    if package_id != Some(lease.package.package_id.as_str()) {
        return Err("document-execution-target.package-mismatch");
    }
    if component_sha256 != Some(lease.component.sha256.as_str()) {
        return Err("document-execution-target.component-mismatch");
    }
    if lease.artifact.schema != artifact_schema {
        return Err("document-execution-target.schema-mismatch");
    }
    if lease.surface.surface_id != surface_id {
        return Err("document-execution-target.surface-mismatch");
    }
    Ok(())
}

pub(crate) fn wgpu_document_socket_surface(program: &ProgramBridgeEntry, app: &AppDefinition, window_kind_id: &str) -> Result<String, String> {
    document_socket_surface_from_descriptor(&program.plugin_id, program.package_id.as_deref(), &program.manifest, app, window_kind_id)
}

pub(crate) fn bind_wgpu_document_socket_surface(host: &ArtifactHost, document_id: &str, artifact_schema: &str, bindings: &[PersistenceBinding], program: &ProgramBridgeEntry, app: &AppDefinition, window_kind_id: &str) -> Result<(), String> {
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
    if app.io.artifact_schema != artifact_schema {
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

//#region ⚔️ConflictProjection
/// ⚔️ One OPEN first-class conflict as this shell's chrome needs it — the target-neutral
/// projection of `protocol::Conflict` React reads through `selectOpenConflicts` +
/// `conflictKindLabel`/`conflictMessageText` (`🏛️ShellHost/🟦️.tsx`). Deliberately NOT the wire type:
/// the wire type only exists on the native backend (the JS program bridge has no `exchange` door,
/// exactly as `read_history` already documents), and the panel builder must compile and be
/// unit-testable on both targets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShellConflictRow {
    pub id: String,
    /// ⚔️ `ConflictKind::Quarantined` — React's `ui.conflict.quarantined` vs `ui.conflict.degraded`
    /// label gate, read off the kind's own tag and never parsed from message prose.
    pub quarantined: bool,
    /// 📨️ The WORST message's fault code and text (`conflict.messages[0]`, React's own pick).
    pub code: String,
    pub message: String,
    /// 🆚️ The incoming/degraded side React's selected-row DiffView preview compares with the
    /// currently unavailable local snapshot (an empty `before` string).
    pub preview_after: String,
}
//#endregion ⚔️ConflictProjection

//#region 🔖️CheckInPure
/// 📌️ ticket 26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END §C5 — auto check-in policy
/// constants, byte-identical to the React shell's `AUTO_CHECKIN_IDLE_MS`/`AUTO_CHECKIN_EDIT_THRESHOLD`
/// (`ShellHelpers/🟦️.tsx`).
pub(crate) const AUTO_CHECKIN_IDLE_MS: i64 = 20_000;
pub(crate) const AUTO_CHECKIN_EDIT_THRESHOLD: u32 = 200;

/// 🧾️ ticket §C5 — folds one `HistoryPatch` into the running per-session entry map, the wgpu twin of
/// the React shell's `applyHistoryPatch` reducer (`ShellHost/🟦️.tsx`): `replace=true` (a
/// fresh `ReadHistory` snapshot on session/document mount) clears the map first; otherwise a patch no
/// newer than the tracked cursor is a stale/duplicate reply and is ignored outright. Returns whether
/// the fold actually changed anything, so callers only re-derive the uncommitted count when it could
/// have moved.
///
/// 🔑️ Rows fold under [`semio_framework::kernel::HistoryEntry::key`] — `edit:<editId>`, else `seq:<seq>` — the
/// identity the kernel wire names (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING), so a row re-emitted for the
/// same edit replaces its predecessor instead of doubling it.
pub(crate) fn fold_history_patch(entries: &mut BTreeMap<String, semio_framework::kernel::HistoryEntry>, cursor: &mut u64, patch: &semio_framework::kernel::HistoryPatch, replace: bool) -> bool {
    if !replace && patch.cursor <= *cursor {
        return false;
    }
    if replace {
        entries.clear();
    }
    for entry in &patch.upserts {
        entries.insert(entry.key(), entry.clone());
    }
    *cursor = patch.cursor;
    true
}

/// 🧾️ The folded rows oldest-first — by `seq`, the command-log order every walk over the ledger reads.
pub(crate) fn history_rows_oldest_first(entries: &BTreeMap<String, semio_framework::kernel::HistoryEntry>) -> Vec<&semio_framework::kernel::HistoryEntry> {
    let mut rows: Vec<&semio_framework::kernel::HistoryEntry> = entries.values().collect();
    rows.sort_by_key(|entry| entry.seq);
    rows
}

/// 🧾️ ticket §C5 — uncommitted-since-last-checkpoint count: the SAME "since the last Change" fold the
/// React shell's `uncommittedEditCount` (`ShellHost/🟦️.tsx`) uses — every applied
/// mutation-kind entry counts, reset to 0 the moment a `commitCheckpoint` history-kind entry is seen,
/// walked oldest-first.
pub(crate) fn uncommitted_edit_count(entries: &BTreeMap<String, semio_framework::kernel::HistoryEntry>) -> u32 {
    let mut pending = 0u32;
    for entry in history_rows_oldest_first(entries) {
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
pub(crate) fn can_check_in(role: semio_framework::manifest::AppRole) -> bool {
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
pub(crate) fn auto_checkin_should_fire(uncommitted_count: u32, pending: bool, last_edit_at_ms: Option<i64>, now_ms: i64, idle_ms: i64, threshold: u32) -> bool {
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
pub(crate) fn should_checkpoint_before_detach(role: semio_framework::manifest::AppRole, has_attached_document: bool, uncommitted_count: u32) -> bool {
    can_check_in(role) && has_attached_document && uncommitted_count > 0
}

/// 📌️ ticket §C5 item 6 — the pure decision `observe_invocation_history` uses to fire `TouchArtifact`:
/// a checkpoint id change only counts when THIS shell itself dispatched the checkpoint that produced
/// it (`checkpoint_dispatched`) — a checkpoint id present from the very first snapshot (nothing
/// "landed", the session just mounted with a pre-existing one) or one made by a REMOTE peer must never
/// trigger a redundant `TouchArtifact` of our own.
pub(crate) fn checkpoint_landed(previous_checkpoint_id: Option<&str>, next_checkpoint_id: Option<&str>, checkpoint_dispatched: bool) -> bool {
    checkpoint_dispatched && next_checkpoint_id.is_some() && next_checkpoint_id != previous_checkpoint_id
}
//#endregion 🔖️CheckInPure

//#region 🧪️IdentityDirectoryPresenceTests
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🔬️wgpu-identity-directory-presence/🦀️.rs"]
mod identity_directory_presence_tests;
//#endregion 🧪️IdentityDirectoryPresenceTests

//#region ShellTypes
/// 🪪️ React's `InputOriginV1` values this target can mint — a CONTROL press, and a pointer/camera
/// stream an engine surface derived from raw input (`🏛️ShellHost/🎯️input-ledger/🟦️.ts`).
pub(crate) const SHELL_DISPATCH_ORIGIN_USER: &str = "user";
pub(crate) const SHELL_DISPATCH_ORIGIN_GESTURE: &str = "gesture";

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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpawnedAppEntry {
    pub id: String,
    pub plugin_id: String,
    pub instance_id: u32,
    pub app_id: String,
    pub label: String,
    pub breadcrumb: Vec<String>,
}

/// 📌️ Host-owned panel state, serialized into `ViewModel::panel_json`.
///
/// 🪶️ It deliberately carries NO app roster. The roster is one [`SpaceProgramEntry`] per app of
/// every loaded plugin — it grew with the plugin closure, was written on every panel commit and was
/// never read back ([`Self::build_space_workflows`] recomputes it from `self.plugins` wherever it is
/// actually needed), so it was pure inflation of the one long string a view context may carry
/// (65 536 characters, `🪟️view-context/🧬️schema/🔣️.json`). What remains is bounded by the open
/// spawned instances (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpacePanelState {
    pub active_panel_tab: String,
    pub spawned_apps: Vec<SpawnedAppEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_spawned_id: Option<String>,
}

/** 🖱️ The `surface` half of one right-click's context-menu request, as
 * [`ShellState::resolve_context_menu_surface`] resolved it: which surface owns the click, the
 * camelCase menu/kind vocabulary id React sends for it, and that kind's own per-row hits, selection
 * groups and (text editor only) text context. */
#[derive(Clone, Debug, Default)]
pub struct ContextMenuSurfaceResolution {
    pub surface_id: String,
    pub kind: String,
    pub hits: Vec<ui_wgpu::wgpu::ContextMenuHit>,
    pub selection: Vec<ui_wgpu::wgpu::ContextMenuSelectionGroup>,
    pub text: Option<ui_wgpu::wgpu::ContextMenuTextContext>,
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
    /// 💬️ Why a disabled row cannot run (`ContextMenuItemSpec.reason`, producer-localized): painted muted beside the label
    /// and announced as the focusable disabled row's description.
    pub reason: Option<String>,
    pub separator: bool,
    pub checked: bool,
}

impl ContextMenuItem {
    /// 🏷️ The row's painted text: its label, and beside it the reason a disabled row cannot run.
    pub(crate) fn painted_label(&self) -> String {
        match self.reason.as_deref().filter(|_| self.disabled) {
            Some(reason) => format!("{} \u{b7} {reason}", self.label),
            None => self.label.clone(),
        }
    }

    /// ♿️ Registers the row's hit and its accessible presentation: an enabled row fires its action; a disabled row is still
    /// a focusable `menuitem` (`aria-disabled`, described by its reason) that fires nothing — the `RowAction` contract.
    pub(crate) fn register_hit(&self, row: Rect, input: &mut InputState<ActionDescriptor>) {
        input.register_hit(HitTarget { rect: row, event: self.action.clone().filter(|_| !self.disabled), control_id: Some(self.id.clone()), kind: HitKind::ContextMenu, drag_axis: None, drag_data: None });
        note_chrome_control_name(&self.id, Some(self.label.as_str()));
        note_chrome_control_disabled(&self.id, self.disabled);
        note_chrome_control_description(&self.id, self.reason.as_deref().filter(|_| self.disabled));
    }
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

/** ⌨️ Result of routing a key while the shell context menu is open. */
pub enum ContextMenuKeyOutcome {
    Ignored,
    Consumed,
    Activate(ActionDescriptor),
    CloseMenu,
}

/// ⌨️ The rows arrow keys reach: every row but a separator — a disabled row is focusable, so its reason is heard.
fn context_menu_focusable_indices(items: &[ContextMenuItem]) -> Vec<usize> {
    items.iter().enumerate().filter(|(_, item)| !item.separator).map(|(index, _)| index).collect()
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

pub(crate) fn context_menu_item_at_path<'a>(root: &'a [ContextMenuItem], path: &[usize]) -> Option<&'a ContextMenuItem> {
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

pub(crate) fn context_menu_path_for_item_id(root: &[ContextMenuItem], item_id: &str, prefix: &mut Vec<usize>) -> Option<Vec<usize>> {
    for (index, item) in root.iter().enumerate() {
        if item.separator {
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

pub(crate) fn context_menu_move_active(root: &[ContextMenuItem], path: &[usize], down: bool) -> Vec<usize> {
    let level_prefix = if path.is_empty() { &[][..] } else { &path[..path.len() - 1] };
    let level = context_menu_items_at_level(root, level_prefix);
    let enabled = context_menu_focusable_indices(level);
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

pub(crate) fn context_menu_path_for_ordinal(root: &[ContextMenuItem], path: &[usize], ordinal: usize) -> Option<Vec<usize>> {
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

pub(crate) fn context_menu_open_submenu_path(root: &[ContextMenuItem], path: &[usize]) -> Option<Vec<usize>> {
    let item = context_menu_item_at_path(root, path)?;
    if item.children.is_empty() {
        return None;
    }
    let enabled = context_menu_focusable_indices(&item.children);
    if enabled.is_empty() {
        return Some(path.to_vec());
    }
    let mut next = path.to_vec();
    next.push(enabled[0]);
    Some(next)
}

#[cfg(test)]
pub(crate) fn context_menu_paths_equal(a: &[usize], b: &[usize]) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(left, right)| left == right)
}

#[cfg(test)]
pub(crate) fn context_menu_submenu_open(active: &[usize], row_path: &[usize], is_active: bool, has_children: bool) -> bool {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ShellPaletteKind {
    Search,
    Find,
}

impl ShellPaletteKind {
    pub(crate) fn input_id(self) -> &'static str {
        match self {
            Self::Search => "ui.search.input",
            Self::Find => "ui.find.input",
        }
    }

    pub(crate) fn dialog_id(self) -> &'static str {
        match self {
            Self::Search => "ui.search.dialog",
            Self::Find => "ui.find.dialog",
        }
    }

    pub(crate) fn list_id(self) -> &'static str {
        match self {
            Self::Search => "ui.search.list",
            Self::Find => "ui.find.list",
        }
    }

    pub(crate) fn close_id(self) -> &'static str {
        match self {
            Self::Search => "ui.search.close",
            Self::Find => "ui.find.close",
        }
    }

    pub(crate) fn control_id(self, index: usize) -> String {
        match self {
            Self::Search => format!("ui.search.item.{index}"),
            Self::Find => format!("ui.find.item.{index}"),
        }
    }

    pub(crate) fn from_input_id(id: &str) -> Option<Self> {
        match id {
            "ui.search.input" => Some(Self::Search),
            "ui.find.input" => Some(Self::Find),
            _ => None,
        }
    }

    fn from_dialog_id(id: &str) -> Option<Self> {
        match id {
            "ui.search.dialog" => Some(Self::Search),
            "ui.find.dialog" => Some(Self::Find),
            _ => None,
        }
    }

    pub(crate) fn from_close_id(id: &str) -> Option<Self> {
        match id {
            "ui.search.close" => Some(Self::Search),
            "ui.find.close" => Some(Self::Find),
            _ => None,
        }
    }

    pub(crate) fn owns_control_id(self, id: &str) -> bool {
        id == self.dialog_id() || id == self.input_id() || id == self.list_id() || id == self.close_id() || Self::row_index(id).is_some_and(|(kind, _)| kind == self)
    }

    pub(crate) fn row_index(id: &str) -> Option<(Self, usize)> {
        [(Self::Search, "ui.search.item."), (Self::Find, "ui.find.item.")].into_iter().find_map(|(kind, prefix)| id.strip_prefix(prefix).and_then(|index| index.parse().ok()).map(|index| (kind, index)))
    }
}

#[derive(Clone, Debug)]
pub(crate) enum ShellPalettePlanEntry {
    Input { rect: Rect, query: String, placeholder: String },
    Group { rect: Rect, label: String },
    Row { rect: Rect, index: usize, item_id: String, label: String, description: Option<String>, selected: bool },
    Empty { rect: Rect, label: String },
    Close { rect: Rect, label: String },
}

#[derive(Clone, Debug)]
pub(crate) struct ShellPalettePlan {
    pub(crate) kind: ShellPaletteKind,
    pub(crate) dialog: Rect,
    pub(crate) list: Rect,
    pub(crate) entries: Vec<ShellPalettePlanEntry>,
}

const SHELL_PALETTE_RESULT_LIMIT: usize = 20;
pub(crate) const SHELL_PALETTE_PLAN_ENTRY_CAPACITY: usize = 2 + SHELL_PALETTE_RESULT_LIMIT * 2;
pub(crate) const SHELL_PALETTE_DESKTOP_MAX_WIDTH_PX: f32 = 512.0;
pub(crate) const SHELL_PALETTE_LIST_MAX_UI_SPACING: f32 = 93.75;
pub(crate) const SHELL_PALETTE_TINY_UI_SPACING: f32 = 3.0;
pub(crate) const SHELL_PALETTE_MEDIUM_UI_SPACING: f32 = 7.0;

/// ⌨️ The shell's OWN overlay query fields. Keyboard focus on one of these is the shell typing into
/// its own chrome, NOT the user typing into app content, so the hardcoded shell chords (and the
/// focused-input Enter/Escape commit) must look straight through them — otherwise the chord that
/// opens the quick-search palette disables every route out of it, including its own toggle and
/// Escape (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub(crate) const SHELL_OVERLAY_INPUT_IDS: [&str; 2] = ["ui.search.input", "ui.find.input"];

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
/// 🧵️ One open document's live `framework/sync` actor channel held by the wgpu shell on both
/// targets. Mirrors `os-shell.tsx`'s `openArtifactSessionsRef` entry: the shell owns the `cmd_tx`/event
/// receiver while the sandboxed plugin instance's store pumps through the registered
/// `ChannelBackbone` (see `framework/product/os/core/rs`'s `ArtifactHost` canonical sequence).
pub struct ShellSyncChannel {
    pub document_id: String,
    pub document_key: ArtifactDocumentKey,
    pub actor_uri: String,
    pub instance_id: u32,
    pub plugin_id: String,
    pub binding_generation: u64,
    pub cmd_tx: ArtifactMailboxSender,
    pub events: tokio::sync::broadcast::Receiver<ArtifactEvent>,
    pub connected_at_ms: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ShellSyncOwner {
    pub(crate) plugin_id: String,
    pub(crate) instance_id: u32,
    pub(crate) binding_generation: u64,
    pub(crate) actor_uri: String,
    pub(crate) document_key: String,
}

pub(crate) fn shell_sync_owner_matches(active: &ShellSyncOwner, completion: &ShellSyncOwner) -> bool {
    active == completion
}

pub(crate) fn shell_sync_channel_owner(channel: &ShellSyncChannel) -> ShellSyncOwner {
    ShellSyncOwner { plugin_id: channel.plugin_id.clone(), instance_id: channel.instance_id, binding_generation: channel.binding_generation, actor_uri: channel.actor_uri.clone(), document_key: shell_hub_document_key(&channel.document_key) }
}

pub(crate) fn route_document_backbone_effects(actor_uri: &str, cmd_tx: &ArtifactMailboxSender, effects: Vec<semio_framework::kernel::Effect>) -> Result<Vec<semio_framework::kernel::Effect>, String> {
    let mut remaining = Vec::new();
    for effect in effects {
        match effect {
            semio_framework::kernel::Effect::SendMessage { target: semio_framework::kernel::MessageEndpoint::Backbone { uri }, payload } => {
                if uri != actor_uri {
                    return Err("plugin document-backbone effect targeted a different actor owner".into());
                }
                match store_sync::os_store::backbone::decode_hot_backbone_message_exact(&payload).map_err(|error| error.to_string())? {
                    store_sync::os_store::backbone::BackboneMessage::Mutations { .. } => cmd_tx.send(ArtifactActorMsg::DocumentBackbone { message: payload }).map_err(|_| "document-backbone mailbox refused the canonical mutation message".to_string())?,
                    store_sync::os_store::backbone::BackboneMessage::Ack { .. } => {}
                    store_sync::os_store::backbone::BackboneMessage::Genesis { .. } => return Err("hot document-backbone egress cannot publish genesis".into()),
                    store_sync::os_store::backbone::BackboneMessage::Member { .. } => return Err("a composed member requires its exact member transport lane".into()),
                    store_sync::os_store::backbone::BackboneMessage::Retract { .. } => return Err("a retraction flows from the actor to its store, never back".into()),
                }
            }
            other => remaining.push(other),
        }
    }
    Ok(remaining)
}
//#endregion 🔖️NativeSyncChannel

pub(crate) const SHELL_DRIVER_AXES: &[(&str, &str, &[(&str, &str)])] = &[
    ("labels", "settings.driver.labels", &[("full", "settings.driver.labelsOption.full"), ("icons", "settings.driver.labelsOption.icons")]),
    ("labelTier", "settings.driver.labelTier", &[("beginner", "settings.driver.labelTierOption.beginner"), ("normal", "settings.driver.labelTierOption.normal")]),
    ("drag", "settings.driver.drag", &[("handle", "settings.driver.dragOption.handle"), ("surface", "settings.driver.dragOption.surface")]),
    ("chrome", "settings.driver.chrome", &[("always", "settings.driver.chromeOption.always"), ("hover", "settings.driver.chromeOption.hover")]),
    ("gumball", "settings.driver.gumball", &[("always", "settings.driver.gumballOption.always"), ("hover", "settings.driver.gumballOption.hover")]),
    ("tooltips", "settings.driver.tooltips", &[("full", "settings.driver.tooltipsOption.full"), ("minimal", "settings.driver.tooltipsOption.minimal"), ("none", "settings.driver.tooltipsOption.none")]),
    ("hotkeys", "settings.driver.hotkeys", &[("inline", "settings.driver.hotkeysOption.inline"), ("tooltip", "settings.driver.hotkeysOption.tooltip"), ("none", "settings.driver.hotkeysOption.none")]),
];

pub(crate) fn builtin_ui_driver_document(driver_id: &str) -> OsUiDriver {
    match driver_id {
        "compact" => {
            OsUiDriver { driver_id: "compact".into(), label: "Compact".into(), config: DslValue::object([("labels","icons"),("labelTier","normal"),("drag","surface"),("chrome","hover"),("gumball","hover"),("tooltips","none"),("hotkeys","none")].map(|(key,value)|(key.to_owned(),DslValue::String(value.to_owned())))) }
        }
        _ => OsUiDriver {
            driver_id: "default".into(),
            label: "Default".into(),
            config: DslValue::object([("labels","full"),("labelTier","normal"),("drag","handle"),("chrome","always"),("gumball","always"),("tooltips","full"),("hotkeys","inline")].map(|(key,value)|(key.to_owned(),DslValue::String(value.to_owned())))),
        },
    }
}

pub(crate) fn resolve_ui_driver_document(driver_id: &str, custom: &HashMap<String, OsUiDriver>) -> OsUiDriver {
    custom.get(driver_id).cloned().unwrap_or_else(|| builtin_ui_driver_document(driver_id))
}

pub(crate) fn ui_driver_axis<'a>(driver: &'a OsUiDriver, key: &str) -> Option<&'a str> {
    driver.config.get(key).and_then(DslValue::as_str)
}

pub(crate) fn ui_driver_document_chrome(driver: &OsUiDriver) -> ui_wgpu::wgpu::UiDriverChrome {
    ui_wgpu::wgpu::UiDriverChrome::builtin(&driver.driver_id).with_axes(
        ui_driver_axis(driver, "labels"),
        ui_driver_axis(driver, "labelTier"),
        ui_driver_axis(driver, "drag"),
        ui_driver_axis(driver, "chrome"),
        ui_driver_axis(driver, "gumball"),
        ui_driver_axis(driver, "tooltips"),
        ui_driver_axis(driver, "hotkeys"),
    )
}

/// 🚗️ React's `resolveUiDriver`: custom first, then builtin, then `DEFAULT_UI_DRIVER`, with
/// all seven axes carried into the renderer-owned chrome snapshot.
pub(crate) fn resolve_ui_driver_chrome(driver_id: &str, custom: &HashMap<String, OsUiDriver>) -> ui_wgpu::wgpu::UiDriverChrome {
    ui_driver_document_chrome(&resolve_ui_driver_document(driver_id, custom))
}

pub(crate) fn valid_ui_driver_axis_value(key: &str, value: &str) -> bool {
    SHELL_DRIVER_AXES.iter().find(|(candidate, _, _)| *candidate == key).is_some_and(|(_, _, options)| options.iter().any(|(candidate, _)| *candidate == value))
}

pub(crate) fn custom_ui_driver_id(label: &str) -> Option<String> {
    let mut slug = String::new();
    let mut separator = false;
    for character in label.trim().to_lowercase().chars() {
        if character.is_ascii_lowercase() || character.is_ascii_digit() {
            if separator && !slug.is_empty() {
                slug.push('-');
            }
            slug.push(character);
            separator = false;
        } else if !slug.is_empty() {
            separator = true;
        }
    }
    (!slug.is_empty()).then(|| format!("custom.{slug}"))
}

//#region 🔖️ChromeThreadBoundary
pub(crate) struct RetainedContentFocus {
    pub(crate) node: ui_wgpu::wgpu::NodeId,
    pub(crate) key: ui_wgpu::wgpu::NodeKey,
    pub(crate) kind: RetainedNodeFocusKind,
}

/// 🏗️ Send-capable inputs, scratch state, and outputs consumed by chrome construction.
#[derive(Default)]
pub(crate) struct ShellChromeBuildState {
    pub(crate) find_items: ShellFindItems,
    /// 🎯️ Which retained node holds focus per window, `None` for a window whose content is blurred —
    /// the wgpu twin of React's active-root registry (`🧱️elements/🌈️Surface/🟦️.tsx:186-197`, whose
    /// `focusin` listener stores `resolveSurfaceActiveRoot(event.target)`, not a boolean). This used to
    /// be a bare `bool`, which is why `UiCommand::FocusChanged`'s node id was thrown away on arrival
    /// (ticket 26/09/17 packet W2k).
    pub(crate) content_focus: HashMap<String, Option<RetainedContentFocus>>,
    pub(crate) focused_retained_surface: Option<String>,
    pub(crate) tooltip_titles: HashMap<String, String>,
    /// 🚗️ The resolved `UiDriver` axes this chrome presents under — React's `UiDriverProvider` value
    /// (`🧱️elements/🚗️UiDriver/🟦️.tsx`), re-resolved from `driver_id` + `custom_drivers` every time
    /// preferences load or change. It is what makes `tooltips: "none"` (the shipped `compact`
    /// driver) actually suppress a hover tooltip on this target; before ticket 26/09/17 packet W15a
    /// the whole `UiDriver` vocabulary had zero readers anywhere in the wgpu tree, so every chrome
    /// control fired its tooltip under every driver.
    pub(crate) driver: ui_wgpu::wgpu::UiDriverChrome,
    pub(crate) tooltip_hover: Option<ChromeTooltipHover>,
    pub(crate) dialog_stack: Vec<ChromeDialogRequest>,
    pub(crate) tour_state: Option<ChromeTourState>,
    pub(crate) introduction_host_suppressed: bool,
    pub(crate) previous_pointer_down: bool,
    pub(crate) clicked_this_frame: bool,
    pub(crate) tour_reveal_latch: Option<String>,
    pub(crate) element_rects: HashMap<String, ChromeElementRectEntry>,
    pub(crate) introduction_seen: HashMap<String, bool>,
    pub(crate) introduction_seen_writes: Vec<String>,
    pub(crate) tutorial_dispatch_internal: bool,
    pub(crate) preferences: ChromePrefsState,
    /// 🧯️ The one live non-fatal notice — see `🧯️TransientNoticeAndAgentOverlays`.
    pub(crate) transient_notice: Option<ShellTransientNotice>,
    /// 📣️ The structured guest refusal behind the dispatch-fault string the funnel classifies next — see [`RefusedGuestFault`].
    pub(crate) refused_guest_fault: Option<RefusedGuestFault>,
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) refused_frame_fault: Option<semio_framework_actor::pack::PackError>,
    #[cfg(not(target_arch="wasm32"))]
    pub(crate) refused_kernel_fault:Option<semio_framework_actor::KernelError>,
    #[cfg(not(target_arch="wasm32"))]
    pub(crate) refused_actor_call:Option<semio_framework_plugin_host::original_actor_context::OriginalActorFixedCall>,
    /// 🌉️ MCP agent bridge consumer (presence + parked approvals), packet W1j.
    pub(crate) agent: crate::agent_bridge::AgentBridgeState,
    /// ✅️ The approvals modal's own open/dismiss state, alongside `agent.pending_approvals`.
    pub(crate) agent_approvals: crate::agent_approvals::AgentApprovalsModel,
}

/// 🖥️ UI-thread persistence and platform integration state surrounding chrome construction.
#[derive(Default)]
pub(crate) struct ShellChromePresentState {
    pub(crate) last_persisted_dock_ui: Option<DockUiState>,
    pub(crate) last_persisted_dock_skeleton: Option<Option<DockSkeleton>>,
    pub(crate) preferences_loaded: bool,
    pub(crate) last_synced_preferences: Option<UiPrefsSnapshot>,
    pub(crate) maintenance: ShellChromeMaintenance,
}

pub(crate) const SHELL_CHROME_IO_FIELD_BYTES: usize = 4 * 1024;

#[derive(Default)]
struct ShellChromeMaintenance {
    pub(crate) load_requested: bool,
    pub(crate) load_phase: u8,
    pub(crate) introduction_read: Option<String>,
    pub(crate) introduction_write: Option<String>,
    pub(crate) layout_requested: bool,
    pub(crate) presence_requested: bool,
    pub(crate) persist_requested: bool,
    pub(crate) persist_phase: u8,
}

impl ShellChromeMaintenance {
    pub(crate) fn pending(&self) -> bool {
        self.load_requested || self.introduction_read.is_some() || self.introduction_write.is_some() || self.layout_requested || self.presence_requested || self.persist_requested
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
/// 🧵️ Compile-time proof that chrome construction state can move to a worker.
pub(crate) fn assert_shell_chrome_build_state_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<ShellChromeBuildState>();
}
//#endregion 🔖️ChromeThreadBoundary
//#endregion ShellTypes
