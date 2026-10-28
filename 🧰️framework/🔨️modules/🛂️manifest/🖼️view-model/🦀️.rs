//! 👁️ View model handed to guests: view context capacities, public invocation capacity and UI refresh sections.

use semio_framework_tool_run::ToolRunTraceCursor;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::Terminology;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use serde::Deserialize;
use serde::Serialize;
use semio_framework_value_ordered::OrderedMap;

// #region 🛂️Manifest
//#region 🔖️Manifest
// 🚧️ UNBLOCKED (26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS): `locale`/
// `terminology` are `ui_wgpu::wgpu::{Locale, Terminology}`, which now have hand-written
// `ToValue`/`FromValue` in `🖱️ui/🎯️targets/🧊️wgpu/🌐️locale-terminology/🧾️value/🦀️.rs`
// (that crate's own generated Rust projection is do-not-edit, so the impls live in a sibling `#[path]`
// mount instead). Kept additive: `ViewModel` is consumed outside this pass by 🛍️products/💻️os
// (plugin/renderer modules) and ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue
// added alongside, not replacing, Serialize/Deserialize.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned, semio_framework_value::NativeReceive, semio_framework_value::RetainedClone)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ViewModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub active_mode_id: Option<String>,
    /// 🛠️ The host-owned active tool of the active mode (never a document field, never a VCS operation) —
    /// mutually exclusive with `active_utility_id`: activating one clears the other (see the React
    /// shell's `onAction` interceptors).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub active_tool_id: Option<String>,
    /// 🧰️ Host-owned active utility per window **instance** (never a document field, never a VCS operation). The shell
    /// sends the full map on every refresh so plugins can build per-pane scene state; tools stay mode-wide via
    /// `active_tool_id`.
    #[serde(default, skip_serializing_if = "OrderedMap::is_empty")]
    #[value(default, skip_serializing_if = "OrderedMap::is_empty")]
    pub active_utility_by_window_id: OrderedMap<String>,
    /// 🧰️ Per-call overlay: the host-owned active utility for the window targeted by this `render`/`handle_action`
    /// call (`window_id`). On batched `refresh-ui`, the plugin stamps this from
    /// `active_utility_by_window_id` per window entry — never from the focused window alone.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub active_utility_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub active_window_kind_id: Option<String>,
    /// 🧩️ Parent-owned input for one embedded surface; never persisted in the contributor document.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub extension_input_json: Option<String>,
    /// 🎯️ The window instance the user is LOOKING at — the shell's own last-focused pane. Sent on
    /// every call and, unlike [`Self::window_id`], deliberately kept by [`Self::for_panel`] while it
    /// names a live [`Self::window_instances`] entry: an
    /// app-level panel is not rendered FOR a window, but a panel that authors per-window settings
    /// still has to address the pane the user last touched instead of the roster's first entry
    /// (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B12 §5.1 measured a Settings edit landing on the
    /// base window kind, a pane nobody is looking at).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub focused_window_id: Option<String>,
    /// 🗣️ Active UI locale; plugins resolve their own label set from this via `resolve_labels`/
    /// `app_labels!`. Non-optional — the shell always resolves one (see `initUiLocaleSync`/
    /// `detectShellLocale`) before the first `render`, so "nobody set the locale" is unrepresentable.
    pub locale: Locale,
    /// 📌️ Host-owned panel state, opaque to the guest and bounded independently of embedded input.
    ///
    /// Contributions are deliberately NOT a view-state field. They are installed into the guest by
    /// the paged `setContributions` command run the host publisher owns
    /// (`🛠️ShellHelpers/🧩️contributions/🟦️.ts`), which the guest folds into its own registry
    /// ([`crate::parse_contributions`], `🌊️flow/📔️registry/🦀️.rs`). Riding the aggregated closure
    /// inside every refresh's view state put a 248 635-character payload against a 65 536-character
    /// schema bound — no guest ever read it back
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub panel_json: Option<String>,
    /// 📞️ Current host session identity for this call. This is ephemeral OS/session context:
    /// plugins may use it to qualify authored work and presentation, but must not persist a copy in
    /// app or document configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub session_identity: Option<ViewSessionIdentity>,
    /// 🧲️ Active terminology id (`Native` default, or an app-declared alternative term set).
    pub terminology: Terminology,
    /// ⏯️ The tool run trace cursor each window instance's renderer echoes, keyed by window instance id —
    /// host-owned like [`Self::active_utility_by_window_id`]. The guest answers the trace pages after it
    /// inside that window's scene `toolRunTrace` lane (`📋️tool-run-contract.md` §3.2).
    #[serde(default, skip_serializing_if = "OrderedMap::is_empty")]
    #[value(default, skip_serializing_if = "OrderedMap::is_empty")]
    pub tool_run_trace_cursor_by_window_id: OrderedMap<ToolRunTraceCursor>,
    /// 🪟️ Rows the tallest visible panel body fits, the shared first-paint budget a guest spends in
    /// document order over containers the host has not yet seen. Absent means "the host has not
    /// measured a viewport yet" — the guest falls back to its own default, never to unbounded.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub tree_viewport_rows: Option<u32>,
    /// 🌿️ Every tree container the host holds state for, flattened over all panel bodies — the ONE
    /// source of truth for which containers are open and which rows are on screen. A guest reads them
    /// per body and materialises exactly the named windows; it keeps no expansion state of its own,
    /// which is what lets an open container survive a refresh.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub tree_windows: Vec<TreeWindowRequest>,
    /// 🪟️ The window instance a `render`/`handle_action` call targets — programs key all per-window
    /// option state (grid, LOD, selection mode, …) off this, never off `active_window_kind_id`, so that
    /// two window instances of the same kind (e.g. split top/perspective panes) never share options.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    /// 🔓️ The live set of open window instances (base + spawned/split), sent on every refresh/action so
    /// `window_engagements`/`window_measures` can return one entry per instance instead of per kind.
    #[serde(default)]
    #[value(default)]
    pub window_instances: Vec<ViewWindowInstance>,
}

/// 📬️ One tree container's host-known state: whether the user opened or closed it (`None` = the
/// author's own default still stands) and the row window on screen, overscan included. `body_key`
/// names the panel body the container lives in, `node_key` the container's **window path** within it.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned, semio_framework_value::NativeReceive, semio_framework_value::RetainedClone)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TreeWindowRequest {
    pub body_key: String,
    /// 🔑️ A **view-context identifier**: printable (no C0 control or DEL code point) and at most 256
    /// code points, the same law `parseResolvedPluginViewState`/`admitCrossingViewContext` applies to
    /// every other identifier a view context carries (`🛂️manifest/🟦️.ts`). A path that cannot satisfy
    /// it is one the host never sends, and the guest then renders that container as unrequested.
    ///
    /// 🔑️ The container's **window path**, not a bare node key: the node keys of its enclosing
    /// windowed containers, outermost first, then its own key, joined by
    /// `semio_framework_ui_contract::TREE_WINDOW_PATH_SEPARATOR` (U+001F). A top-level section's path
    /// IS its own key, so a flat request is unchanged; a nested one is
    /// `"fem3d-play-artifact.load-cases\u{1f}dead"`. Identity is the path rather than the key so that
    /// two containers reusing one entity id under different parents — a load case and a combination
    /// both called `uls`, one object id in four cad pane sections — keep independent open and window
    /// state, while their node keys stay the raw pick target ids the interaction domain registers.
    pub node_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub open: Option<bool>,
    pub offset: u32,
    pub rows: u32,
}

/// 🔌️ One live window instance, as seen by a plugin: `id` is the instance id (equal to `window_kind_id`
/// for a base, unsplit window), `window_kind_id` is the `AppDefinition.windowKinds` entry it renders.
// 🚧️ UNBLOCKED: was gated on `ViewModel` above, itself gated on `ui_wgpu::wgpu::{Locale,
// Terminology}` gaining `ToValue`/`FromValue` — both now converted (see `ViewModel`'s own comment).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned, semio_framework_value::NativeReceive, semio_framework_value::RetainedClone)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ViewWindowInstance {
    pub id: String,
    pub window_kind_id: String,
}

/// 🪪️ The authenticated OS session identity projected into every guest call while available.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned, semio_framework_value::NativeReceive, semio_framework_value::RetainedClone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ViewSessionIdentity {
    pub user_id: String,
    pub display_name: String,
}

impl ViewModel {
    /// 🌐️ Creates empty host context with the caller's explicit language and terminology.
    pub fn new(locale: Locale, terminology: Terminology) -> Self {
        Self {
            active_mode_id: None,
            active_window_kind_id: None,
            active_utility_id: None,
            active_utility_by_window_id: Default::default(),
            active_tool_id: None,
            panel_json: None,
            session_identity: None,
            extension_input_json: None,
            locale,
            terminology,
            window_id: None,
            focused_window_id: None,
            window_instances: Vec::new(),
            tool_run_trace_cursor_by_window_id: Default::default(),
            tree_windows: Vec::new(),
            tree_viewport_rows: None,
        }
    }

    /// 🎛️ Projects app-level panels without binding their controls to a window.
    ///
    /// 🎯️ [`Self::focused_window_id`] survives — that is the whole point of the field — but only while
    /// it names a pane the host actually carries. A shell publishes the new mode's roster before it
    /// refocuses (the wgpu shell boots straight into `generate` with the edit-mode `procedural-main`
    /// still focused), and a focused pane the roster does not list made
    /// `WindowConfigOwnerRegistry::capture` fault before the panel's body key was ever matched — so
    /// every app panel published NOTHING and reached the user as three
    /// `wgpu-ui.surface-not-published:framework.panel.*` fault cards, while the framework-rendered
    /// History panel (served before that capture) published fine
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). A stale focus is a mode-switch race; a panel
    /// answers it by authoring against no pane, never by publishing nothing.
    pub fn for_panel(&self) -> Self {
        let focused_window_id = self.focused_window_id.clone().filter(|id| self.window_instances.iter().any(|window| &window.id == id));
        Self { window_id: None, active_window_kind_id: None, active_utility_id: None, focused_window_id, ..self.clone() }
    }

    /// 🖥️ Projects host-owned context onto one concrete window without borrowing the focused window's utility.
    pub fn for_window_instance(&self, window_id: &str) -> Option<Self> {
        let window = self.window_instances.iter().find(|window| window.id == window_id)?;
        Some(Self {
            window_id: Some(window.id.clone()),
            active_window_kind_id: Some(window.window_kind_id.clone()),
            active_utility_id: self.active_utility_by_window_id.get(window_id).cloned(),
            ..self.clone()
        })
    }

    /// 🎚️ Resolves utilities through exact window ownership; kind requests need an addressed or unique live instance.
    pub fn window_utility(&self, window_id: Option<&str>) -> Option<&str> {
        let Some(requested) = window_id.or(self.window_id.as_deref()) else { return self.active_utility_id.as_deref().filter(|utility| !utility.is_empty()) };
        let instance = |id: Option<&str>| id.and_then(|id| self.window_instances.iter().find(|window| window.id == id && window.window_kind_id == requested));
        let id = if self.window_instances.iter().any(|window| window.id == requested) {
            requested
        } else if self.window_instances.iter().any(|window| window.window_kind_id == requested) {
            let mut candidates = self.window_instances.iter().filter(|window| window.window_kind_id == requested);
            let first = candidates.next()?;
            let unique = candidates.next().is_none().then_some(first);
            &instance(self.window_id.as_deref()).or_else(|| instance(self.focused_window_id.as_deref())).or(unique)?.id
        } else { requested };
        self.active_utility_by_window_id.get(id).map(String::as_str).or_else(|| (self.window_id.as_deref() == Some(id)).then_some(self.active_utility_id.as_deref()).flatten()).filter(|utility| !utility.is_empty())
    }

    /// 🎬️ Overlays the host-owned mode tool, then binds the view to a window or the panel.
    pub fn for_host_armed_action(&self, host_active_tool_id: Option<&str>, window_id: Option<&str>) -> Option<Self> {
        let armed = Self { active_tool_id: host_active_tool_id.map(str::to_string), ..self.clone() };
        match window_id {
            Some(id) => armed.for_window_instance(id),
            None => Some(armed.for_panel()),
        }
    }
}

//#region 📏️ViewContextCapacity
/// 🪞️ The surface view context's capacities, mirrored from the ONE language-neutral declaration
/// `🪟️view-context/🧬️schema/🔣️.json` — the same file `parseResolvedPluginViewState`
/// (`🛂️manifest/🟦️.ts`) and its Ajv oracle validate against. Pinned from Rust by
/// `🧪️tests/🔬️view-context-capacity/🦀️.rs`, so neither side can drift from the schema.
pub const VIEW_CONTEXT_IDENTIFIER_CHARS: usize = 256;

/// 📏️ `panelJson` capacity, in Unicode characters (schema `maxLength`).
pub const VIEW_CONTEXT_LONG_STRING_CHARS: usize = 65_536;

/// 🧬️ `activeUtilityByWindowId` capacity (schema `maxProperties`).
pub const VIEW_CONTEXT_UTILITY_ENTRIES: usize = 64;

/// 🔲️ `windowInstances` capacity (schema `maxItems`).
pub const VIEW_CONTEXT_WINDOW_INSTANCES: usize = 64;

/// 🕵️ `toolRunTraceCursorByWindowId` capacity (schema `maxProperties`).
pub const VIEW_CONTEXT_TRACE_CURSOR_ENTRIES: usize = 64;

/// 🌲️ `treeWindows` capacity (schema `maxItems`) — one document's worth of containers
/// (`ui_contract::UI_DOCUMENT_NODES`), the widest tree a single refresh can be about.
pub const VIEW_CONTEXT_TREE_WINDOWS: usize = 128;

/// 🔢️ Numeric fields of one `TreeWindowRequest`: `offset` and `rows`. `open` is a boolean and
/// `bodyKey`/`nodeKey` are `Identifier`-shaped.
pub const VIEW_CONTEXT_TREE_WINDOW_NUMERIC_FIELDS: usize = 2;

/// 🔤️ Fields one `TreeWindowRequest` encodes: `bodyKey`, `nodeKey`, `open`, `offset`, `rows`.
pub const VIEW_CONTEXT_TREE_WINDOW_FIELDS: usize = 5;

/// 👣️ Numeric fields of one `ToolRunTraceCursor`: `run`, `generation`, `page`.
pub const VIEW_CONTEXT_TRACE_CURSOR_FIELDS: usize = 3;

/// 📐️ Widest decimal a numeric field reaches in text form: `run` up to 2^53-1, and every `u32`
/// field (`generation`, `page`, `offset`, `rows`, `treeViewportRows`) comfortably inside it.
const VIEW_CONTEXT_TRACE_CURSOR_DIGITS: usize = 16;

/// 🆔️ `Identifier`-typed scalar fields: `activeModeId`, `activeWindowKindId`, `activeUtilityId`,
/// `activeToolId`, `windowId`, `focusedWindowId`.
pub const VIEW_CONTEXT_IDENTIFIER_FIELDS: usize = 6;

/// 🪪️ Identifier-shaped fields nested in `sessionIdentity`: `userId` and `displayName`.
pub const VIEW_CONTEXT_SESSION_IDENTITY_FIELDS: usize = 2;

/// 📥️ Bounded opaque inputs: panel state and one embedded surface's parameters.
pub const VIEW_CONTEXT_LONG_STRING_FIELDS: usize = 2;

/// 🔣️ Worst-case UTF-8 expansion of one schema character — the schema bounds characters, the wire
/// carries bytes.
const VIEW_CONTEXT_BYTES_PER_CHAR: usize = 4;

/// 🎞️ Per-encoded-value framing allowance (tag, length prefix, key) of the pack wire the shell
/// encodes a view context with.
const VIEW_CONTEXT_FRAMING_BYTES_PER_VALUE: usize = 64;

/// 🎒️ Encoded values a maximal view context carries: every scalar field, the two enums, every
/// collection and each of their entries' fields.
const VIEW_CONTEXT_ENCODED_VALUES: usize = VIEW_CONTEXT_IDENTIFIER_FIELDS
    + VIEW_CONTEXT_SESSION_IDENTITY_FIELDS
    + VIEW_CONTEXT_LONG_STRING_FIELDS
    + 2
    + 1
    + VIEW_CONTEXT_UTILITY_ENTRIES * 2
    + 1
    + VIEW_CONTEXT_WINDOW_INSTANCES * 2
    + 1
    + VIEW_CONTEXT_TRACE_CURSOR_ENTRIES * (1 + VIEW_CONTEXT_TRACE_CURSOR_FIELDS)
    + 1
    + VIEW_CONTEXT_TREE_WINDOWS * (1 + VIEW_CONTEXT_TREE_WINDOW_FIELDS)
    + 1;

/// 📡️ Largest wire-encoded surface view context the contract can produce — the admission bound
/// `plugin_mount_surface` holds `Event::SurfaceVisible`'s `view_state` to.
///
/// Derived from the schema above rather than borrowed: this used to reuse
/// `MAX_PUBLIC_ACTION_BODY_BYTES` (256 KiB), the DFF *public action* admission cap, which is
/// unrelated to a view context and smaller than the 64 Ki-character long string the schema
/// alone permits — so a schema-valid context was rejected as
/// `plugin.internal: surface context exceeds its wire bound` and every window body of the affected
/// app was replaced by that fault card (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub const MAX_SURFACE_VIEW_CONTEXT_BYTES: usize = (VIEW_CONTEXT_IDENTIFIER_FIELDS * VIEW_CONTEXT_IDENTIFIER_CHARS
    + VIEW_CONTEXT_SESSION_IDENTITY_FIELDS * VIEW_CONTEXT_IDENTIFIER_CHARS
    + VIEW_CONTEXT_LONG_STRING_FIELDS * VIEW_CONTEXT_LONG_STRING_CHARS
    + VIEW_CONTEXT_UTILITY_ENTRIES * 2 * VIEW_CONTEXT_IDENTIFIER_CHARS
    + VIEW_CONTEXT_WINDOW_INSTANCES * 2 * VIEW_CONTEXT_IDENTIFIER_CHARS
    + VIEW_CONTEXT_TRACE_CURSOR_ENTRIES * (VIEW_CONTEXT_IDENTIFIER_CHARS + VIEW_CONTEXT_TRACE_CURSOR_FIELDS * VIEW_CONTEXT_TRACE_CURSOR_DIGITS)
    + VIEW_CONTEXT_TREE_WINDOWS * (2 * VIEW_CONTEXT_IDENTIFIER_CHARS + VIEW_CONTEXT_TREE_WINDOW_NUMERIC_FIELDS * VIEW_CONTEXT_TRACE_CURSOR_DIGITS + VIEW_CONTEXT_TRACE_CURSOR_DIGITS)
    + VIEW_CONTEXT_TRACE_CURSOR_DIGITS)
    * VIEW_CONTEXT_BYTES_PER_CHAR
    + VIEW_CONTEXT_ENCODED_VALUES * VIEW_CONTEXT_FRAMING_BYTES_PER_VALUE;

/// 🔠️ Largest surface body key the same contract admits — a body key is a window-kind id, a
/// panel-tab leaf id or a reserved `UiRefreshSection` key, all `Identifier`-shaped.
pub const MAX_SURFACE_BODY_KEY_BYTES: usize = VIEW_CONTEXT_IDENTIFIER_CHARS * VIEW_CONTEXT_BYTES_PER_CHAR;

//#endregion 📏️ViewContextCapacity
//#region 📏️PublicInvocationCapacity

/// 🧳️ Largest UTF-8 body one structurally addressed action/command JSON invocation may occupy on
/// the way into a plugin process — `🎛️public-invocation/🧬️schema/🔣️.json`'s `maxBodyBytes`.
pub const PUBLIC_INVOCATION_BODY_BYTES: usize = 262_144;

/// 🔂️ Largest single JSON string one invocation may carry — `maxStringBytes`. Counted as escaped
/// bytes minus their leading backslashes, so `\"` costs one and `A` costs five.
///
/// This is the bound that actually sizes a host→guest push, and NO tool execution contract can
/// widen it: `validate_public_json_envelope` runs before the addressed tool's own
/// `max_raw_wire_bytes` is ever consulted. Any payload larger than this must be paged by its
/// producer (`🏛️ShellHost/🟦️.tsx`'s `publicInvocationStringPages`).
pub const PUBLIC_INVOCATION_STRING_BYTES: usize = 4_096;

/// ⏬️ Deepest object/array nesting one invocation may reach — `maxDepth`.
pub const PUBLIC_INVOCATION_DEPTH: usize = 64;

/// ✖️ Wire bytes one counted string byte may occupy — `escapePairWireFactor`. A two-character
/// escape pair (`\"`, `\\`, `\n`) counts ONE against [`PUBLIC_INVOCATION_STRING_BYTES`] and occupies
/// TWO on the wire; every longer escape (`\u00XX`, six bytes) counts five, a smaller ratio. So a
/// page filled to the string bound occupies at most twice its counted extent, which is what a tool
/// carrying such a page must declare as its `max_raw_wire_bytes`.
pub const PUBLIC_INVOCATION_ESCAPE_PAIR_WIRE_FACTOR: usize = 2;

/// 💲️ What ONE character of a raw string costs against [`PUBLIC_INVOCATION_STRING_BYTES`] once the
/// JSON encoder has written it — the exact accounting `validate_public_json_envelope` performs,
/// which skips a leading `\` and counts every byte after it. Non-ASCII is charged at its `\uXXXX`
/// escape (five per UTF-16 unit), never at its shorter raw UTF-8 form, so a page cut with this
/// function is admitted whether or not the encoder escapes above U+007F.
pub fn public_invocation_char_cost(character: char) -> usize {
    match character {
        '"' | '\\' | '\n' | '\r' | '\t' | '\u{8}' | '\u{c}' => 1,
        character if (character as u32) < 0x20 => 5,
        character if character.is_ascii() => 1,
        character => character.len_utf16() * 5,
    }
}

/// 📄️ Cuts one oversized string into the page run a public invocation can actually carry — each
/// page filled to, and never past, [`PUBLIC_INVOCATION_STRING_BYTES`] as
/// [`public_invocation_char_cost`] measures it, split only on character boundaries.
///
/// The twin of `publicInvocationStringPages` in `🛠️ShellHelpers/🟦️.tsx`; the two must cut the same
/// payload identically, which `🪝️extension-invocation/🟦️.ts` and this module's own capacity law pin.
/// An empty input yields one empty page, so a producer always sends at least one addressed page.
pub fn public_invocation_string_pages(text: &str) -> Vec<String> {
    let mut pages = Vec::new();
    let mut page = String::new();
    let mut cost = 0usize;
    for character in text.chars() {
        let next = public_invocation_char_cost(character);
        if cost + next > PUBLIC_INVOCATION_STRING_BYTES {
            pages.push(std::mem::take(&mut page));
            cost = 0;
        }
        page.push(character);
        cost += next;
    }
    pages.push(page);
    pages
}

//#endregion 📏️PublicInvocationCapacity
//#region 🔖️UiRefreshSection

/// 📑️ The four refresh sections that are NOT authored window/panel bodies. Each is its own retained
/// surface whose reserved body key names the object-safe accessor the plugin runtime calls in place of
/// `PluginApp::render` (`window_engagements`/`window_measures`/`tool_measures`/`app_catalogue`), so
/// measures, tool measures, engagements and the app-static catalogue publish, re-publish and page
/// through exactly the same `Event::SurfaceVisible` → mount → reconcile → `UiPatch` law as a window body.
///
/// 🛍️ `Catalogue` is app-STATIC: it carries the whole registered operator/palette catalogue exactly
/// once per app instance (hash-conditional, so an unchanged catalogue costs one hash compare), instead
/// of riding on every node-graph scene payload where it blew the fixed `UI_FIXED_BYTES` surface
/// admission as soon as real operator sets were installed (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// Mirrored in TypeScript by `UI_REFRESH_SECTIONS` in `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts`.
/// Both sides are pinned against the one language-neutral declaration in
/// `🧪️tests/🔬️ui-refresh-section/🔣️.json`, so neither can drift.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiRefreshSection {
    Engagements,
    Measures,
    Tools,
    Catalogue,
}

/// ⌨️ Response field / host refresh-cache key of each [`UiRefreshSection`], in `UiRefreshSection::ALL` order.
pub const UI_REFRESH_SECTION_KEYS: [&str; 4] = ["engagements", "measures", "tools", "catalogue"];

/// 🪧️ Reserved body key — and retained surface key — of each [`UiRefreshSection`], in
/// `UiRefreshSection::ALL` order. Dotted and `framework.`-prefixed so it can never collide with an
/// app-authored window instance id or panel tab id, which are plain identifiers.
pub const UI_REFRESH_SECTION_BODY_KEYS: [&str; 4] = ["framework.section.engagements", "framework.section.measures", "framework.section.tools", "framework.section.catalogue"];

impl UiRefreshSection {
    pub const ALL: [Self; 4] = [Self::Engagements, Self::Measures, Self::Tools, Self::Catalogue];

    /// 🔐️ See [`UI_REFRESH_SECTION_KEYS`].
    pub fn key(self) -> &'static str {
        UI_REFRESH_SECTION_KEYS[self as usize]
    }

    /// 🗝️ See [`UI_REFRESH_SECTION_BODY_KEYS`].
    pub fn body_key(self) -> &'static str {
        UI_REFRESH_SECTION_BODY_KEYS[self as usize]
    }

    /// 🔎️ Resolves a mounted surface's body key back to the section it renders, `None` for every
    /// app-authored body.
    pub fn from_body_key(body_key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|section| section.body_key() == body_key)
    }
}

//#endregion 🔖️UiRefreshSection
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest

#[cfg(test)]
use semio_framework_value::DslValue;
#[cfg(test)]
use semio_framework_value::Number;
#[cfg(test)]
use semio_framework_value::retained_clone::RetainedCloneGrant;

#[cfg(test)]
#[global_allocator]
static TEST_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

// #region 🛂️Manifest
//#region 🔖️Manifest
#[cfg(test)]
#[path = "../🧪️tests/🔬️window-view-context/🦀️.rs"]
mod window_view_context_tests;

#[cfg(test)]
#[path = "../🧪️tests/🪟️resolved-host-context/🦀️.rs"]
mod resolved_host_context_tests;

//#region 📏️PublicInvocationCapacity
#[cfg(test)]
#[path = "../🧪️tests/🔬️public-invocation-capacity/🦀️.rs"]
mod public_invocation_capacity_tests;

//#endregion 📏️PublicInvocationCapacity
//#region 📏️ViewContextCapacity
#[cfg(test)]
#[path = "../🧪️tests/🔬️view-context-capacity/🦀️.rs"]
mod view_context_capacity_tests;

#[cfg(test)]
#[path = "../🧪️tests/🔢️integer-carriers/🦀️.rs"]
mod view_context_integer_carrier_tests;

//#endregion 📏️ViewContextCapacity
//#region 🔖️UiRefreshSection
#[cfg(test)]
#[path = "../🧪️tests/🔬️ui-refresh-section/🦀️.rs"]
mod ui_refresh_section_tests;
//#endregion 🔖️UiRefreshSection
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest

#[path="🪟️selection/🦀️.rs"]
mod original_window_selection;
pub use original_window_selection::ViewModelWindowSelection;
