//! 🖼️ View model and package side of the manifest: `ViewModel`, view-context and public-invocation capacities, `UiRefreshSection`,
//! the package descriptor, agent contributions and the media vocabulary.
//!
//! Split from the manifest root by section C of `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/SEAM-MAP-PLUGIN-MANIFEST-HUB.md`.
use super::*;

// #region 🛂️Manifest
//#region 🔖️Manifest
// 🚧️ UNBLOCKED (26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS): `locale`/
// `terminology` are `ui_wgpu::wgpu::{Locale, Terminology}`, which now have hand-written
// `ToValue`/`FromValue` in `🖱️ui/🎯️targets/🧊️wgpu/🌐️locale-terminology/🧾️value/🦀️.rs`
// (that crate's own generated Rust projection is do-not-edit, so the impls live in a sibling `#[path]`
// mount instead). Kept additive: `ViewModel` is consumed outside this pass by 🛍️products/💻️os
// (plugin/renderer modules) and ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue
// added alongside, not replacing, Serialize/Deserialize.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
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
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    #[value(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub active_utility_by_window_id: std::collections::HashMap<String, String>,
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
    /// 🪪️ Current host session identity for this call. This is ephemeral OS/session context:
    /// plugins may use it to qualify authored work and presentation, but must not persist a copy in
    /// app or document configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub session_identity: Option<ViewSessionIdentity>,
    /// 🗣️ Active terminology id (`Native` default, or an app-declared alternative term set).
    pub terminology: Terminology,
    /// ⏯️ The tool run trace cursor each window instance's renderer echoes, keyed by window instance id —
    /// host-owned like [`Self::active_utility_by_window_id`]. The guest answers the trace pages after it
    /// inside that window's scene `toolRunTrace` lane (`📋️tool-run-contract.md` §3.2).
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    #[value(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub tool_run_trace_cursor_by_window_id: std::collections::HashMap<String, ToolRunTraceCursor>,
    /// 🪟️ Rows the tallest visible panel body fits, the shared first-paint budget a guest spends in
    /// document order over containers the host has not yet seen. Absent means "the host has not
    /// measured a viewport yet" — the guest falls back to its own default, never to unbounded.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub tree_viewport_rows: Option<u32>,    /// 🪟️ Every tree container the host holds state for, flattened over all panel bodies — the ONE
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
    /// 🪟️ The live set of open window instances (base + spawned/split), sent on every refresh/action so
    /// `window_engagements`/`window_measures` can return one entry per instance instead of per kind.
    #[serde(default)]
    #[value(default)]
    pub window_instances: Vec<ViewWindowInstance>,
}

/// 🪟️ One tree container's host-known state: whether the user opened or closed it (`None` = the
/// author's own default still stands) and the row window on screen, overscan included. `body_key`
/// names the panel body the container lives in, `node_key` the container's **window path** within it.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
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

/// 🪟️ One live window instance, as seen by a plugin: `id` is the instance id (equal to `window_kind_id`
/// for a base, unsplit window), `window_kind_id` is the `AppDefinition.windowKinds` entry it renders.
// 🚧️ UNBLOCKED: was gated on `ViewModel` above, itself gated on `ui_wgpu::wgpu::{Locale,
// Terminology}` gaining `ToValue`/`FromValue` — both now converted (see `ViewModel`'s own comment).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ViewWindowInstance {
    pub id: String,
    pub window_kind_id: String,
}

/// 🪪️ The authenticated OS session identity projected into every guest call while available.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
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

    /// 📌️ Projects app-level panels without binding their controls to a window.
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

    /// 🎯️ Projects host-owned context onto one concrete window without borrowing the focused window's utility.
    pub fn for_window_instance(&self, window_id: &str) -> Option<Self> {
        let window = self.window_instances.iter().find(|window| window.id == window_id)?;
        Some(Self {
            window_id: Some(window.id.clone()),
            active_window_kind_id: Some(window.window_kind_id.clone()),
            active_utility_id: self.active_utility_by_window_id.get(window_id).cloned(),
            ..self.clone()
        })
    }

    /// 🛠️ Overlays the host-owned mode tool, then binds the view to a window or the panel.
    pub fn for_host_armed_action(&self, host_active_tool_id: Option<&str>, window_id: Option<&str>) -> Option<Self> {
        let armed = Self { active_tool_id: host_active_tool_id.map(str::to_string), ..self.clone() };
        match window_id {
            Some(id) => armed.for_window_instance(id),
            None => Some(armed.for_panel()),
        }
    }
}

#[cfg(test)]
#[path = "../🧪️tests/🔬️window-view-context/🦀️.rs"]
mod window_view_context_tests;

//#region 📏️ViewContextCapacity
/// 📏️ The surface view context's capacities, mirrored from the ONE language-neutral declaration
/// `🪟️view-context/🧬️schema/🔣️.json` — the same file `parseResolvedPluginViewState`
/// (`🛂️manifest/🟦️.ts`) and its Ajv oracle validate against. Pinned from Rust by
/// `🧪️tests/🔬️view-context-capacity/🦀️.rs`, so neither side can drift from the schema.
pub const VIEW_CONTEXT_IDENTIFIER_CHARS: usize = 256;
/// 📏️ `panelJson` capacity, in Unicode characters (schema `maxLength`).
pub const VIEW_CONTEXT_LONG_STRING_CHARS: usize = 65_536;
/// 📏️ `activeUtilityByWindowId` capacity (schema `maxProperties`).
pub const VIEW_CONTEXT_UTILITY_ENTRIES: usize = 64;
/// 📏️ `windowInstances` capacity (schema `maxItems`).
pub const VIEW_CONTEXT_WINDOW_INSTANCES: usize = 64;
/// 📏️ `toolRunTraceCursorByWindowId` capacity (schema `maxProperties`).
pub const VIEW_CONTEXT_TRACE_CURSOR_ENTRIES: usize = 64;
/// 📏️ `treeWindows` capacity (schema `maxItems`) — one document's worth of containers
/// (`ui_contract::UI_DOCUMENT_NODES`), the widest tree a single refresh can be about.
pub const VIEW_CONTEXT_TREE_WINDOWS: usize = 128;
/// 🔢️ Numeric fields of one `TreeWindowRequest`: `offset` and `rows`. `open` is a boolean and
/// `bodyKey`/`nodeKey` are `Identifier`-shaped.
pub const VIEW_CONTEXT_TREE_WINDOW_NUMERIC_FIELDS: usize = 2;
/// 🔤️ Fields one `TreeWindowRequest` encodes: `bodyKey`, `nodeKey`, `open`, `offset`, `rows`.
pub const VIEW_CONTEXT_TREE_WINDOW_FIELDS: usize = 5;
/// 🔢️ Numeric fields of one `ToolRunTraceCursor`: `run`, `generation`, `page`.
pub const VIEW_CONTEXT_TRACE_CURSOR_FIELDS: usize = 3;
/// 📐️ Widest decimal a numeric field reaches in text form: `run` up to 2^53-1, and every `u32`
/// field (`generation`, `page`, `offset`, `rows`, `treeViewportRows`) comfortably inside it.
const VIEW_CONTEXT_TRACE_CURSOR_DIGITS: usize = 16;
/// 🔢️ `Identifier`-typed scalar fields: `activeModeId`, `activeWindowKindId`, `activeUtilityId`,
/// `activeToolId`, `windowId`, `focusedWindowId`.
pub const VIEW_CONTEXT_IDENTIFIER_FIELDS: usize = 6;
/// 🪪️ Identifier-shaped fields nested in `sessionIdentity`: `userId` and `displayName`.
pub const VIEW_CONTEXT_SESSION_IDENTITY_FIELDS: usize = 2;
/// 🔢️ Bounded opaque inputs: panel state and one embedded surface's parameters.
pub const VIEW_CONTEXT_LONG_STRING_FIELDS: usize = 2;
/// 📐️ Worst-case UTF-8 expansion of one schema character — the schema bounds characters, the wire
/// carries bytes.
const VIEW_CONTEXT_BYTES_PER_CHAR: usize = 4;
/// 📐️ Per-encoded-value framing allowance (tag, length prefix, key) of the pack wire the shell
/// encodes a view context with.
const VIEW_CONTEXT_FRAMING_BYTES_PER_VALUE: usize = 64;
/// 🔢️ Encoded values a maximal view context carries: every scalar field, the two enums, every
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

/// 📏️ Largest wire-encoded surface view context the contract can produce — the admission bound
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

/// 📏️ Largest surface body key the same contract admits — a body key is a window-kind id, a
/// panel-tab leaf id or a reserved `UiRefreshSection` key, all `Identifier`-shaped.
pub const MAX_SURFACE_BODY_KEY_BYTES: usize = VIEW_CONTEXT_IDENTIFIER_CHARS * VIEW_CONTEXT_BYTES_PER_CHAR;

#[cfg(test)]
#[path = "../🧪️tests/🔬️view-context-capacity/🦀️.rs"]
mod view_context_capacity_tests;

#[cfg(test)]
#[path = "../🧪️tests/🔢️integer-carriers/🦀️.rs"]
mod view_context_integer_carrier_tests;
//#endregion 📏️ViewContextCapacity

//#region 📏️PublicInvocationCapacity
/// 📏️ Largest UTF-8 body one structurally addressed action/command JSON invocation may occupy on
/// the way into a plugin process — `🎛️public-invocation/🧬️schema/🔣️.json`'s `maxBodyBytes`.
pub const PUBLIC_INVOCATION_BODY_BYTES: usize = 262_144;
/// 📏️ Largest single JSON string one invocation may carry — `maxStringBytes`. Counted as escaped
/// bytes minus their leading backslashes, so `\"` costs one and `A` costs five.
///
/// This is the bound that actually sizes a host→guest push, and NO tool execution contract can
/// widen it: `validate_public_json_envelope` runs before the addressed tool's own
/// `max_raw_wire_bytes` is ever consulted. Any payload larger than this must be paged by its
/// producer (`🏛️ShellHost/🟦️.tsx`'s `publicInvocationStringPages`).
pub const PUBLIC_INVOCATION_STRING_BYTES: usize = 4_096;
/// 📏️ Deepest object/array nesting one invocation may reach — `maxDepth`.
pub const PUBLIC_INVOCATION_DEPTH: usize = 64;
/// 📐️ Wire bytes one counted string byte may occupy — `escapePairWireFactor`. A two-character
/// escape pair (`\"`, `\\`, `\n`) counts ONE against [`PUBLIC_INVOCATION_STRING_BYTES`] and occupies
/// TWO on the wire; every longer escape (`\u00XX`, six bytes) counts five, a smaller ratio. So a
/// page filled to the string bound occupies at most twice its counted extent, which is what a tool
/// carrying such a page must declare as its `max_raw_wire_bytes`.
pub const PUBLIC_INVOCATION_ESCAPE_PAIR_WIRE_FACTOR: usize = 2;

/// 📐️ What ONE character of a raw string costs against [`PUBLIC_INVOCATION_STRING_BYTES`] once the
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
/// payload identically, which `🔬️engine-contract/🟦️.ts` and this module's own capacity law pin.
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

#[cfg(test)]
#[path = "../🧪️tests/🔬️public-invocation-capacity/🦀️.rs"]
mod public_invocation_capacity_tests;
//#endregion 📏️PublicInvocationCapacity

//#region 🔖️UiRefreshSection
/// 🧩️ The four refresh sections that are NOT authored window/panel bodies. Each is its own retained
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

/// 🔑️ Response field / host refresh-cache key of each [`UiRefreshSection`], in `UiRefreshSection::ALL` order.
pub const UI_REFRESH_SECTION_KEYS: [&str; 4] = ["engagements", "measures", "tools", "catalogue"];

/// 🪧️ Reserved body key — and retained surface key — of each [`UiRefreshSection`], in
/// `UiRefreshSection::ALL` order. Dotted and `framework.`-prefixed so it can never collide with an
/// app-authored window instance id or panel tab id, which are plain identifiers.
pub const UI_REFRESH_SECTION_BODY_KEYS: [&str; 4] = ["framework.section.engagements", "framework.section.measures", "framework.section.tools", "framework.section.catalogue"];

impl UiRefreshSection {
    pub const ALL: [Self; 4] = [Self::Engagements, Self::Measures, Self::Tools, Self::Catalogue];

    /// 🔑️ See [`UI_REFRESH_SECTION_KEYS`].
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn key(self) -> &'static str {
        UI_REFRESH_SECTION_KEYS[self as usize]
    }

    /// 🪧️ See [`UI_REFRESH_SECTION_BODY_KEYS`].
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn body_key(self) -> &'static str {
        UI_REFRESH_SECTION_BODY_KEYS[self as usize]
    }

    /// 🔎️ Resolves a mounted surface's body key back to the section it renders, `None` for every
    /// app-authored body.
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn from_body_key(body_key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|section| section.body_key() == body_key)
    }
}

#[cfg(test)]
#[path = "../🧪️tests/🔬️ui-refresh-section/🦀️.rs"]
mod ui_refresh_section_tests;
//#endregion 🔖️UiRefreshSection

//#region 🔖️PackageDescriptor

/// 🚦 How an extension actor runs relative to its host plugin — `📓️design-abi.md` §5. Default
/// `Isolated`: a same-process sandboxed actor, no publisher trust assumed. `Linked` additionally
/// requires the same publisher as the host plugin (enforced at link time, feature-gated to avoid
/// the `semio-framework-os-flow` ↔ extension-crate cycle); `Exclusive` gets a dedicated actor
/// (e.g. flow/brep tessellation); `Cold` runs as a bounded job, not a resident actor.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum ExecutionMode {
    Declarative,
    Linked,
    #[default]
    Isolated,
    Exclusive,
    Cold,
}

/// 🧩️ One extension point a host plugin publishes — replaces the Cargo `consumes` tag
/// (`📓️design-abi.md` §5). `allowed_modes` gates `Linked` (same publisher required);
/// `capability_allowance`/`quota_ceiling` bound what any extension attaching here can ever hold,
/// regardless of what it requests — "a host can never delegate more than it holds".
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExtensionPointDeclaration {
    pub id: String,
    pub publisher_scope: String,
    pub allowed_modes: Vec<ExecutionMode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub capability_allowance: Vec<kernel::CapabilityId>,
    #[serde(default)]
    #[value(default)]
    pub quota_ceiling: kernel::QuotaSchema,
    pub payload_schema: kernel::SchemaId,
    pub activation: kernel::ActivationEvent,
}

/// 📦️ One asset bundled with a package and preloaded into `kernel::Event::InstanceOpen.assets` —
/// `📓️design-abi.md` §2's `read-asset` replacement.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct AssetDeclaration {
    pub name: String,
    pub media_type: MediaType,
    pub size_bytes: u64,
    pub sha256: String,
}

/// 🗂️ One free-form descriptor-only contribution row, keyed by `id` with an opaque JSON
/// `payload` — the residual placeholder shape for the two `ContributionSet` categories (`menus`,
/// `themes`) that still have no real declared-contribution precedent anywhere in the codebase
/// (E1-describe surveyed every `[package.metadata.semio]` `contributes`/`consumes` tag and every
/// manifest-adjacent type: no plugin declares a menu or theme as its own manifest concept today —
/// context menus are derived at runtime from `ActionSemantics`/category metadata, and there is no
/// declared theme/palette contribution anywhere under `🖱️ui/🎨️styling`). Additive: nothing
/// constructs one yet, and a future typed model can replace either category without a wire break.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct DescriptorEntry {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<DslValue>,
}

/// 🗂️ One file-format kind an app declares it can import and/or export — the typed shape for
/// `ContributionSet.file_types` (`📓️design-abi.md` §3), grounded in `AppIo.export_formats`/
/// `import_formats` (currently flat `Vec<String>` scaffolding on that type) paired with the app's
/// own `artifact_media_type`, flattened to one row per format kind across every app the package
/// declares.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct FileTypeContribution {
    pub format_kind: String,
    pub media_type: MediaType,
    pub imports: bool,
    pub exports: bool,
}

/// 🚪️ Which side of an `IoEntryDescriptor` route this row is — owned mirror of `io::IoDirection`
/// (`🚪️io/🦀️.rs`), the same "owned wire twin of a native type living in a sibling
/// framework module" idiom `ContributedMutationMetadata`/`ContributedInferenceMetadata` already
/// use for the os-kernel protocol crate.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum IoEntryDirection {
    Import,
    Export,
}

/// 🚪️ One registered IO dialect route — the typed shape for `ContributionSet.io_entries`
/// (`📓️design-abi.md` §2/§3's absorbed `io-dialects` routing table), an owned mirror of
/// `io::IoKey`'s `(owner, counterpart, direction)` identity built from the already-in-scope
/// `ArtifactDialect` (`🚪️io/🧬️schema/🦀️component.rs`) instead of `IoKey`'s seven flat fields —
/// `IoKey` itself isn't `owned schema exporter`-derived and this crate must not add that derive to a module it
/// doesn't own.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct IoEntryDescriptor {
    pub owner: ArtifactDialect,
    pub counterpart: ArtifactDialect,
    pub direction: IoEntryDirection,
}

/// 🎹️ One registered composer/serializer/deserializer route — the typed shape for
/// `ContributionSet.composer_entries`, an owned mirror of `io::ComposerEntry`'s `(writes, reads)`
/// identity (its third field, the `compose` fn pointer, is runtime-only and has no wire form —
/// a descriptor is build-time, non-executable data).
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ComposerEntryDescriptor {
    pub writes: ArtifactDialect,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub reads: Vec<ArtifactDialect>,
}

/// 🗂️ Everything a package contributes, gathered for static (`describe()`-time) emission —
/// `📓️design-abi.md` §3. `commands`/`topic_contributions`/`artifact_contributions` reuse this
/// crate's existing typed models; `panels` reuses `PanelTabDefinition` (already the typed shape
/// `AppDefinition.panel_tabs` declares, flattened across every app); `inference_services`/
/// `mutation_services` reuse `ContributedInferenceMetadata`/`ContributedMutationMetadata` (a
/// package's OWN registered services on artifact kinds it owns, as opposed to
/// `artifact_contributions`' services contributed onto a DEPENDENCY's kind — same wire shape
/// either way, `contributor == owner` and `depends_on` empty for a self-owned row);
/// `file_types`/`io_entries`/`composer_entries` are new types grounded in `AppIo`/`io::IoKey`/
/// `io::ComposerEntry` — see each type's own doc. `menus`/`themes` stay `DescriptorEntry` — see
/// its doc for why.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ContributionSet {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub commands: Vec<CommandDefinition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub menus: Vec<DescriptorEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub file_types: Vec<FileTypeContribution>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub panels: Vec<PanelTabDefinition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub themes: Vec<DescriptorEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub topic_contributions: Vec<TopicContribution>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub artifact_contributions: Vec<ArtifactContributionDescriptor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub inference_services: Vec<ContributedInferenceMetadata>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub mutation_services: Vec<ContributedMutationMetadata>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub io_entries: Vec<IoEntryDescriptor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub composer_entries: Vec<ComposerEntryDescriptor>,
}

/// 🚨️ The `plugin_id` a failed assembly mints instead of a real one. `plugin_manifest()`
/// (`🔌️plugin/🦀️.rs`) returns this stub whenever `PLUGIN_ASSEMBLY_ERROR` is set, carrying
/// the real error text in `label`. It looks like a descriptor, parses as JSON, and would feed the
/// generated registry catalog with fabricated contributions — so the emitter refuses to write one
/// (`🖨️describe/📦️packages/🦀️rust/🦀️.rs`). Lives here, beside [`PackageDescriptor`], because it
/// is the one crate BOTH the guest SDK that mints it and the host emitter that rejects it depend
/// on; a duplicated string literal in either would drift silently.
pub const ASSEMBLY_FAILED_PLUGIN_ID: &str = "assembly-failed";

/// 📡️ Exact guest-described application-channel ABI required by the compiled component.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExecutionProtocol {
    pub app_channel_version: u32,
}

/// 📦️ The static, build-time-emitted description of a plugin or extension package —
/// `📓️design-abi.md` §3's `describe()` output (`🛂️.descriptor.semio`/`🔣️.json`).
/// Nothing constructs or reads one yet in this packet: additive contract only (packet
/// A2-abi-sdk's builder wiring and E1-describe's emitter/registry `check` gate consume it next).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PackageDescriptor {
    pub descriptor_version: u32,
    pub package_id: String,
    pub role: PackageRole,
    pub manifest: PluginManifest,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub activation_events: Vec<kernel::ActivationEvent>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub capability_requests: Vec<kernel::CapabilityRequest>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extension_points: Vec<ExtensionPointDeclaration>,
    pub execution: ExecutionMode,
    pub execution_protocol: ExecutionProtocol,
    #[serde(default)]
    #[value(default)]
    pub quotas: kernel::QuotaSchema,
    #[serde(default)]
    #[value(default)]
    pub contributions: ContributionSet,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub assets: Vec<AssetDeclaration>,
}

/// 🔤️ Descriptor value with every object ordered by UTF-8 key bytes at every depth.
/// Shared by guest descriptions, emitted packs and exact freshness projections.
#[derive(Clone, Debug, PartialEq)]
pub struct CanonicalDescriptorValue(DslValue);

impl CanonicalDescriptorValue {
    /// 🔃️ Orders the members of every object by key bytes; members of equal keys keep their authored order.
    pub fn new(value: DslValue) -> Self {
        Self(canonical_members(value))
    }

    /// 👁️ The canonical value itself.
    pub fn value(&self) -> &DslValue {
        &self.0
    }
}

/// 🪜️ [`CanonicalDescriptorValue::new`] through every array and object of `value`.
fn canonical_members(value: DslValue) -> DslValue {
    use semio_framework_value::DslValue;
    match value {
        DslValue::Array(items) => DslValue::Array(items.into_iter().map(canonical_members).collect()),
        DslValue::Object(entries) => {
            let mut entries: Vec<(String, DslValue)> = entries.into_iter().map(|(key, entry)| (key, canonical_members(entry))).collect();
            entries.sort_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
            DslValue::Object(entries)
        }
        scalar => scalar,
    }
}


#[cfg(test)]
#[path = "../🧪️tests/🔬️package-descriptor-value-codec/🦀️.rs"]
mod package_descriptor_value_codec_tests;
//#endregion 🔖️PackageDescriptor

//#region 🔖️AgentContributions
// 🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P8-agent-spi, `📋️master.md`
// §3.1: what a package OFFERS to agents — distinct from `capability_requests` on
// `PackageDescriptor` above (what a package NEEDS host permission for). Overloading one for the
// other is exactly the mistake this design forbids: `capability_requests: Vec<kernel::
// CapabilityRequest>` gates HOST PRIVILEGE (`artifacts.write`, `fs:*`, …) this package asks the
// broker for; `AgentContributions` below is a curated ADVERTISEMENT of which of this package's own
// already-declared capabilities (actions/commands, already fully described via `ActionSemantics`/
// `CapabilityPolicy` — P3, `🔖️ActionSemantics` above) an agent may discover and invoke at all, and
// which of those are further promoted to a first-class MCP tool (`📋️master.md` §3.1's `tools/list`
// "promoted" set).
//
// Attachment point, and why it is a LEASE rather than a field added directly here: the obvious
// home is a new `PackageDescriptor.agent: Option<AgentContributions>` field. `PackageDescriptor`'s
// only known construction sites (`describe_plugin()`/`describe_extension()`,
// `🔌️plugin/🛂️describe/🦀️.rs`) live in `semio-framework-plugin` — a crate this packet
// does not own and the peer ticket's W3 is about to freeze — and both build the value as a full
// explicit struct literal (verified by reading the file: no `..Default::default()` anywhere, and
// `PackageDescriptor` has no `Default` impl — `role: PackageRole` has no default variant — so a
// `Default` impl could not rescue an untouched call site either way). Adding the field HERE alone
// would therefore break `cargo check -p semio-framework-plugin --lib` the moment this region
// lands, in a live shared tree, before any reviewer applies a companion lease — exactly the
// destabilisation this packet must not cause. So the field itself, and its counterpart on the
// `PluginDescriptorExtras` side-channel (`🔌️plugin/🦀️.rs`, E2's own established pattern
// for precisely this "avoid cascading through construction sites I don't own" problem) and on
// `ExtensionManifest`, ship together as ONE atomic lease bundle — see `📓️terra-P8-report.md` §2
// for the full reasoning and `📓️lease-P8-agent-descriptor.md` for the exact diffs.
/// 🤖️ What a package OFFERS to agents — see the region header above for the critical
/// `capability_requests` vs `AgentContributions` distinction. `capabilities` are fully-qualified
/// capability ids (the same grammar `🌉️mcp/🗂️catalog` compiles — `<plugin_id>.<app_id>.
/// <action_id>` / `….cmd.<id>` / `….mode.<mode_id>.<id>`, `📋️master.md` §3.1); `promoted` is the
/// subset exposed as a first-class MCP tool (`tools/list`) rather than only reachable via
/// `capabilities.search`/`capabilities.describe`. Both empty by default — an absent
/// `AgentContributions` (the `Option` on `PackageDescriptor` stays `None`) means "not yet
/// agent-enabled", never "agent-enabled with zero capabilities" (an empty-but-`Some` value).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct AgentContributions {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub promoted: Vec<String>,
}

impl AgentContributions {
    /// 🧪️ `promoted ⊆ capabilities` — the one structural invariant every producer
    /// (`describe_plugin()`/`describe_extension()`) and consumer (`📇️registry:check`) must hold.
    /// Pure and dependency-free so both the Rust builder side and the registry's own TypeScript
    /// check (which has no way to call back into this crate) can each verify it independently.
    pub async fn promoted_is_subset_of_capabilities(&self) -> bool {
        self.promoted.iter().all(|id| self.capabilities.contains(id))
    }
}

#[cfg(test)]
#[path = "../🧪️tests/🔬️agent-contributions/🦀️.rs"]
mod agent_contributions_tests;
//#endregion 🔖️AgentContributions

//#region 🔖️MediaVocabulary
// 🔀️ Relocated verbatim from 🔺️mesh/🦀️.rs (ticket 26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT
// wave 4a) — manifest-vocabulary types, not codec material; mesh keeps MeshData/Primitives/
// generic obj-glb-stl codecs and the DWG bit-codec, but the legacy media-format enum itself was retired
// in ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6 — every
// format-kind field below is now a plain string kind id.
//#region ArtifactKind

/// 🗂️ An app-declared OS resource kind (e.g. a 3D mesh format, a raster format) — the manifest-level
/// counterpart to `AppBuilder::artifact_kind(...)` (`framework/plugin/rs`), letting `framework/product/os/core`
/// build its artifact catalog from `AppDefinition.artifact_kinds` at plugin registration time instead of
/// hardcoding a per-app match on kind-id strings. Carries the manifest-level media-kind fields
/// (`media_type`/`schema`/`export_formats`/`import_formats`) directly
/// so one spec carries both the OS-catalog presentation shape and the `MediaType` a wire actually negotiates
/// — see `crate::media_types_compatible`. `OsArtifactDescriptor` (`framework/product/os/core`) threads
/// `media_type` through so registry lookups return it alongside the rest of the descriptor.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ArtifactKindSpec {
    pub id: String,
    /// 🗣️ The kind's own localized name — what a picker shows for it, never its editor app's label.
    pub label: LocalizedLabel,
    pub source_format: String,
    pub component_kind: String,
    pub dimension: String,
    pub media_capability: OsMediaCapability,
    pub media_type: MediaType,
    pub schema: String,
    /// 🗄️ Export target format kind ids (string, the legacy format enum was retired — ticket 26/08/11/
    /// SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6).
    pub export_formats: Vec<String>,
    pub import_formats: Vec<String>,
    /// 🗄️ Stdio export target kind ids (e.g. `stdio.json`) — additive peer of `export_formats`.
    pub export_stdio_kinds: Vec<String>,
    /// 🗄️ Stdio import source kind ids — additive peer of `import_formats`.
    pub import_stdio_kinds: Vec<String>,
}

/// 🏠️ One artifact kind a package hosts — the kind id, one document schema of its owner's codecs, and the OWNER: the plugin
/// that owns the kind (declares it and its codec), a declared dependency of the host (see `PluginManifest::hosted_artifact_kinds`).
/// The owner is explicit, never read off the kind id — an embedded surface's kind (`3d.cad`) names no plugin.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct HostedArtifactKind {
    pub id: String,
    pub schema: String,
    pub owner: String,
}
//#endregion ArtifactKind

//#region MediaType

/// 🧬️ A port or wire's declared media type — the pair a producer offers or a consumer accepts.
// 🚧️ BLOCKED: see `MediaClass` above — same cross-crate serde dependency.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct MediaType {
    pub class: MediaClass,
    pub form: MediaForm,
}

semio_framework_value::artifact_retire_leaf!(MediaType);


/// 🔌️ How a `MediaType` is actually encoded once it crosses a process boundary — binary payloads
/// carry a format kind id string (the legacy format enum was retired — ticket 26/08/11/
/// SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6), structured payloads carry
/// a schema id instead (see `ArtifactKindSpec::schema`).
// 🚧️ BLOCKED: see `MediaClass` above — `🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️.rs`'s
// `WorkflowMediaPort`/`MediaContract` (owned by another agent this pass) still embed this by value
// inside plain `#[derive(Serialize, Deserialize)]` types.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase", tag = "kind")]
#[value(rename_all = "camelCase", tag = "kind")]
pub enum MediaWireFormat {
    Binary { format_kind: String },
    Document { schema: String },
    Intrinsic { schema: String },
}

/// 🔀️ Which side of a wire a `MediaPortSpec` sits on.
// 🚧️ BLOCKED: see `MediaWireFormat` above — same cross-crate serde dependency (`WorkflowMediaPort`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum MediaPortDirection {
    In,
    Out,
}

/// 🔢️ Whether a `MediaPortSpec` accepts/produces exactly one media value or a stream/collection of them — e.g. a mesh-array input that fans in from several upstream producers.
// 🚧️ BLOCKED: see `MediaWireFormat` above — same cross-crate serde dependency (`WorkflowMediaPort`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum PortMultiplicity {
    One,
    Many,
}

/// 🔌️ A single port an app exposes on the workflow — `kind_id` optionally pins it to one `ArtifactKindSpec.id` when the port is more specific than its `media_type` alone conveys.
// 🚧️ BLOCKED: see `MediaWireFormat` above — `WorkflowMediaPort.spec` embeds this by value inside a
// plain `#[derive(Serialize, Deserialize)]` type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct MediaPortSpec {
    pub id: String,
    pub label: String,
    pub direction: MediaPortDirection,
    pub media_type: MediaType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub kind_id: Option<String>,
    pub required: bool,
    pub multiplicity: PortMultiplicity,
}

/// ⚖️ Result of checking whether a producer's `MediaType` can feed a consumer's accepted `MediaType`: exact match, a known lossy-but-allowed conversion, or outright rejection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaCompat {
    Direct,
    Convert { from: MediaForm, to: MediaForm },
    Reject,
}

/// 🔀️ One-way `MediaForm` conversions the workflow is allowed to insert implicitly (e.g. a B-Rep producer feeding a mesh-only consumer). `media_types_compatible` looks up `(produced, accepted)` directly, so add the reverse pair too if a conversion should also hold the other way.
const MEDIA_FORM_CONVERSIONS: &[(MediaForm, MediaForm)] = &[(MediaForm::Brep, MediaForm::Mesh), (MediaForm::Vector, MediaForm::Raster), (MediaForm::Design, MediaForm::Kit), (MediaForm::Type, MediaForm::Kit)];

/// ⚖️ The single source of truth for wire compatibility: classes must match exactly, `MediaForm::Any` on the accepting side takes anything within the class, equal forms are always direct, and everything else falls through to the explicit `MEDIA_FORM_CONVERSIONS` table.
pub async fn media_types_compatible(produced: &MediaType, accepted: &MediaType) -> MediaCompat {
    if produced.class != accepted.class {
        return MediaCompat::Reject;
    }
    if matches!(accepted.form, MediaForm::Any) || produced.form == accepted.form {
        return MediaCompat::Direct;
    }
    for (from, to) in MEDIA_FORM_CONVERSIONS {
        if *from == produced.form && *to == accepted.form {
            return MediaCompat::Convert { from: *from, to: *to };
        }
    }
    MediaCompat::Reject
}
//#endregion MediaType

//#region 🔖️AppIo
/// 🧷️ The non-format fields of `ArtifactKindSpec` (see `ArtifactKind` region above) that describe how
/// a resource presents in the OS catalog — split out so `AppIo` can carry its own `export_formats`/
/// `import_formats` lists without duplicating `ArtifactKindSpec`'s full shape (which stays alive
/// unchanged for now; later waves retire it onto `AppIo`).
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ArtifactPresentation {
    pub id: String,
    pub name: String,
    pub dimension: String,
    pub component_kind: String,
}

/// 🔌️ An app's full media I/O surface — the artifact schema/type every app carries implicitly (see
/// `artifact_in_port`/`artifact_out_port`) plus whatever additional workflow ports, catalog
/// export/import formats, and OS presentation it declares itself. Scaffolding for the typed manifest
/// surface (`AppDefinition.io`); apps don't populate this yet — later waves migrate `media_inputs`/
/// `media_outputs`/`artifact_kinds` onto it.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct AppIo {
    pub artifact_schema: String,
    pub artifact_media_type: MediaType,
    /// 🔌️ App-specific ports only — the implicit artifact ports are auto-injected by `all_ports`.
    pub ports: Vec<MediaPortSpec>,
    pub export_formats: Vec<String>,
    pub import_formats: Vec<String>,
    pub artifact: ArtifactPresentation,
}

impl AppIo {
    /// 🔌️ The implicit `"artifact:in"` port every app accepts, keyed by `self.artifact_media_type`.
    pub async fn artifact_in_port(&self) -> MediaPortSpec {
        MediaPortSpec { id: "artifact:in".into(), label: "Artifact".into(), direction: MediaPortDirection::In, media_type: self.artifact_media_type, kind_id: None, required: true, multiplicity: PortMultiplicity::One }
    }

    /// 🔌️ The implicit `"artifact:out"` port every app produces — see `artifact_in_port`.
    pub async fn artifact_out_port(&self) -> MediaPortSpec {
        MediaPortSpec { id: "artifact:out".into(), label: "Artifact".into(), direction: MediaPortDirection::Out, media_type: self.artifact_media_type, kind_id: None, required: true, multiplicity: PortMultiplicity::One }
    }

    /// 🔌️ The full port list, in stable order: the implicit artifact ports first, followed by every app-specific port declared in `self.ports`.
    pub async fn all_ports(&self) -> Vec<MediaPortSpec> {
        let mut ports = vec![self.artifact_in_port().await, self.artifact_out_port().await];
        ports.extend(self.ports.clone());
        ports
    }

    /// 🏗️ Builds an `AppIo` from just its implicit artifact surface, with no extra ports/formats declared yet — chain `.with_ports(...)` to add app-specific ports.
    pub async fn from_artifact(schema: impl Into<String>, media_type: MediaType, artifact: ArtifactPresentation) -> Self {
        Self { artifact_schema: schema.into(), artifact_media_type: media_type, ports: Vec::new(), export_formats: Vec::new(), import_formats: Vec::new(), artifact }
    }

    /// 🔌️ Attaches app-specific ports (beyond the implicit artifact ports) to this `AppIo`.
    pub async fn with_ports(mut self, ports: Vec<MediaPortSpec>) -> Self {
        self.ports = ports;
        self
    }
}

impl Default for AppIo {
    fn default() -> Self {
        Self {
            artifact_schema: String::new(),
            artifact_media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            ports: Vec::new(),
            export_formats: Vec::new(),
            import_formats: Vec::new(),
            artifact: ArtifactPresentation { id: String::new(), name: String::new(), dimension: String::new(), component_kind: String::new() },
        }
    }
}
//#endregion 🔖️AppIo

//#region 🔖️ConfigSpec
/// 🧮️ An app's full typed configuration record — the manifest-level declaration
/// `AppDefinition.config` carries: one [`ActionArgDef`] per field (its `id` is the config key), so a config field,
/// an action argument and a mutation input share one input vocabulary and one renderer.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, Default)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ConfigSpec {
    #[serde(default)]
    #[value(default)]
    pub fields: Vec<ActionArgDef>,
}

impl ConfigSpec {
    pub async fn empty() -> Self {
        Self::default()
    }
}
//#endregion 🔖️ConfigSpec

//#region 🔖️CommandGrammar
/// 🎛️ One keyword-dispatched command variant (e.g. `move x=1 y=2`) and its field grammar — one
/// [`ActionArgDef`] per field, `required` where the grammar demands the field.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CommandVariantSpec {
    pub keyword: String,
    pub fields: Vec<ActionArgDef>,
}

/// 🎛️ An app's full typed binary command grammar — the manifest-level declaration
/// `AppDefinition.command_grammar` carries. Empty until per-app waves populate it.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, Default)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CommandGrammar {
    #[serde(default)]
    #[value(default)]
    pub variants: Vec<CommandVariantSpec>,
}

impl CommandGrammar {
    pub async fn empty() -> Self {
        Self::default()
    }
}
//#endregion 🔖️CommandGrammar

//#region Media
/// 🎞️ The value that actually flows over a workflow wire, produced by `ArtifactApp::export_media` and consumed by `ArtifactApp::import_media`. Kept separate from the `MediaType` lattice above (which only negotiates *compatibility*, never carries a value) so headless runners and the UI share one payload shape.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Media {
    pub media_type: MediaType,
    pub payload: MediaPayload,
}

/// 📦️ Intrinsic payloads own complete typed values; explicit JSON and addressed binary payloads retain their declared media forms.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", tag = "kind")]
pub enum MediaPayload {
    Structured { schema: String, json: String },
    Binary { format_kind: String, blob_hash: String },
    Intrinsic { schema: String, value: DslValue },
}
#[path="../🎞️media/🦀️.rs"]mod intrinsic_media;

/// 🔑️ A cheap identity for one port's current output, independent of serializing the full payload — the unit the `SpaceRunner` compares to decide whether a downstream node actually needs to see a new value.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MediaFingerprint(pub String);

/// 🌉️ Hand-written, not derived: `#[derive(ToValue, FromValue)]` doesn't support tuple structs —
/// wire-shaped as the bare inner string, matching serde's own default newtype-struct behavior.
impl ToValue for MediaFingerprint {
    fn to_value(&self) -> DslValue {
        self.0.to_value()
    }
}
impl FromValue for MediaFingerprint {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        String::from_value(value).map(MediaFingerprint)
    }
}

impl MediaFingerprint {
    /// 🔑️ Canonical fingerprint of a `Media` value: structured payloads hash their JSON text, binary payloads reuse their existing content hash directly (no re-hashing bytes already addressed by the blob store).
    pub fn of(media: &Media) -> Self {
        match &media.payload {
            MediaPayload::Structured { schema, json } => MediaFingerprint(semio_framework_hash::hash_parts(&[schema.as_str(), json.as_str()])),
            MediaPayload::Binary { blob_hash, .. } => MediaFingerprint(blob_hash.clone()),
            MediaPayload::Intrinsic { schema, value } => MediaFingerprint(intrinsic_media::fingerprint(schema,value)),
        }
    }
}

/// 🚧️ Failure exporting, importing, or fingerprinting media on a declared port.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub enum MediaError {
    UnknownPort(String),
    Incompatible { port: String, produced: MediaType, accepted: MediaType },
    Payload(String, String),
    NotImplemented,
}

impl std::fmt::Display for MediaError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownPort(port) => write!(formatter, "unknown media port `{port}`"),
            Self::Incompatible { port, produced, accepted } => {
                write!(formatter, "port `{port}` produced {produced:?} but the wire accepts {accepted:?}")
            }
            Self::Payload(port, message) => write!(formatter, "media payload error on port `{port}`: {message}"),
            Self::NotImplemented => formatter.write_str("media ports are not implemented for this app"),
        }
    }
}

impl std::error::Error for MediaError {}

/// 🔀️ A registered one-way conversion the workflow may insert on a wire when `media_types_compatible` reports `MediaCompat::Convert`. Kept behind a trait (never a bare closure) so converters can be enumerated, tested, and swapped without touching the runner.
pub trait MediaConverter: Send + Sync {
    fn source_form(&self) -> impl std::future::Future<Output = MediaForm> + Send;
    fn to_form(&self) -> impl std::future::Future<Output = MediaForm> + Send;
    fn convert(&self, media: &Media) -> impl std::future::Future<Output = Result<Media, MediaError>> + Send;
}
//#endregion Media
//#endregion 🔖️MediaVocabulary
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
