//! 📜️ Renderer-independent UI declarations, window layouts, actions and engagement contracts.

pub mod layout {
    // #region layout
    //! 📐️ Window layouts, panel tab constants, and engagement rails.

    use crate::IconName;
    use semio_framework_value::DslValue;
    use semio_framework_value::{FromValue, ToValue};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use semio_framework_ui_contract::UiFixedList;

    //#region 🔖️Action
    /// 🎬️ One action a control fires; the default (an empty action name) is the unbound marker `fired_action` answers `None`
    /// for.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct ActionDescriptor {
        pub controller_id: String,
        pub action: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub args: Option<DslValue>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct StyleSpec {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub variant: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub size: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub density: Option<String>,
    }
    //#endregion 🔖️Action

    //#region 🔖️Presence
    /// 🧭️ The one shared, mandatory visual state every rendered UI element carries — orthogonal to
    /// `status` and to the `hover`/`selected` flags. `Hidden` short-circuits everything else: a hidden
    /// element is not rendered at all (no layout, no paint, no events) — renderers/reconcile must check
    /// this before doing anything with the rest of an element's `UiPresence`.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub enum UiState {
        Introducing,
        Celebrating,
        Previewed,
        #[default]
        Normal,
        Disabled,
        Hidden,
    }

    /// 🧭️ The activity lifecycle of a UI element, orthogonal to [`UiState`] and composable with it.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub enum UiStatus {
        Waiting,
        Loading,
        #[default]
        Idle,
        Finished,
    }

    // 🚫️async: E1 pure accessor consumed by external-trait impls (serde skip_serializing_if) — see R9
    fn is_default<T: Default + PartialEq>(value: &T) -> bool {
        *value == T::default()
    }

    /// 🧭️ The shared, compile-time-enforced state model every rendered UI element embeds as a
    /// mandatory `presence` field: `state` × `status` × `hover` × `selected`. All combinations are
    /// visually distinguishable except `state == Hidden`, which makes the rest irrelevant — see
    /// [`UiPresence::visible`]. Defaults to fully inert (`Normal`/`Idle`/`false`/`false`) and is omitted
    /// from the wire format entirely at default (see `UiPresence::is_default`).
    /// 👥️ One peer's mark on the element carrying this `UiPresence` — hover/selection dot plus
    /// initials chip (contract-freeze §C7.6 of ticket `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/
    /// SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION`). `label` is the actor id
    /// itself (no display name is carried this far down the stack — see `PeerPresence`'s own doc
    /// comment in the plugin crate); a renderer that has the full roster may substitute a friendlier
    /// name, but every renderer must always carry SOME text alongside color (never color alone).
    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiPeerMark {
        pub actor: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub color: Option<u8>,
        #[serde(default, skip_serializing_if = "is_default")]
        #[value(default, skip_serializing_if = "is_default")]
        pub hovered: bool,
        #[serde(default, skip_serializing_if = "is_default")]
        #[value(default, skip_serializing_if = "is_default")]
        pub selected: bool,
        pub label: String,
    }

    /// 🧭️ The shared, compile-time-enforced state model every rendered UI element embeds as a
    /// mandatory `presence` field: `state` × `status` × `hover` × `selected` × own `color` × peer
    /// `marks`. All combinations are visually distinguishable except `state == Hidden`, which makes
    /// the rest irrelevant — see [`UiPresence::visible`]. Defaults to fully inert
    /// (`Normal`/`Idle`/`false`/`false`/`None`/`[]`) and is omitted from the wire format entirely at
    /// default (see `UiPresence::is_default`). `color`/`peers` (ticket 26/08/17/SHARED-PRESENCE-
    /// SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C7.6) make `UiPresence` `Clone`-only — no
    /// longer `Copy` — since `peers: Vec<UiPeerMark>` owns heap data; `UiNode::presence()`/
    /// `UiControlNode::presence()` therefore return `&UiPresence`, not a by-value copy.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase", default)]
    #[value(rename_all = "camelCase", default)]
    pub struct UiPresence {
        #[serde(skip_serializing_if = "is_default")]
        #[value(skip_serializing_if = "is_default")]
        pub state: UiState,
        #[serde(skip_serializing_if = "is_default")]
        #[value(skip_serializing_if = "is_default")]
        pub status: UiStatus,
        #[serde(skip_serializing_if = "is_default")]
        #[value(skip_serializing_if = "is_default")]
        pub hover: bool,
        #[serde(skip_serializing_if = "is_default")]
        #[value(skip_serializing_if = "is_default")]
        pub selected: bool,
        /// 🎨️ This session's own hub-assigned palette index — stamped onto every `interaction_domain`-
        /// bound tree item by `ui_tree_stamp_presence`, `None` for a folder-only session with no hub.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub color: Option<u8>,
        /// 👥️ Every OTHER peer currently marking this element (hover and/or selection), sorted by
        /// actor.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        pub peers: Vec<UiPeerMark>,
    }

    impl UiPresence {
        // 🚫️async: E1 pure accessor consumed by external-trait impls (serde skip_serializing_if) — see R9
        pub fn is_default(&self) -> bool {
            *self == Self::default()
        }
        /// 🙈️ `false` for `state == Hidden` — callers must not render, lay out, or hit-test the element.
        pub fn visible(&self) -> bool {
            self.state != UiState::Hidden
        }
        pub fn state(state: UiState) -> Self {
            Self { state, ..Self::default() }
        }
        pub fn status(status: UiStatus) -> Self {
            Self { status, ..Self::default() }
        }
        pub fn selected(selected: bool) -> Self {
            Self { selected, ..Self::default() }
        }
        pub fn with_state(self, state: UiState) -> Self {
            Self { state, ..self }
        }
        pub fn with_status(self, status: UiStatus) -> Self {
            Self { status, ..self }
        }
        pub fn with_hover(self, hover: bool) -> Self {
            Self { hover, ..self }
        }
        pub fn with_selected(self, selected: bool) -> Self {
            Self { selected, ..self }
        }
        /// 🙈️ `Hidden` when `hidden`, else `Normal` — the one-line migration for today's `is_hidden`
        /// flags on elements that are genuinely not rendered (not to be confused with a domain "dimmed"
        /// prop, e.g. a tree item's eye-toggle, which must stay visible/clickable).
        pub fn hidden_if(hidden: bool) -> Self {
            Self::state(if hidden { UiState::Hidden } else { UiState::Normal })
        }
        /// 🚫️ `Disabled` when `disabled`, else `Normal` — the one-line migration for today's `disabled` flags.
        pub fn disabled_if(disabled: bool) -> Self {
            Self::state(if disabled { UiState::Disabled } else { UiState::Normal })
        }
        /// 🎉️ `Celebrating` when `celebrating`, else `Normal` — the transient completion emphasis fired e.g.
        /// when an introduction step advances by the user performing the taught action.
        pub fn celebrate_if(celebrating: bool) -> Self {
            Self::state(if celebrating { UiState::Celebrating } else { UiState::Normal })
        }
    }

    impl UiStatus {
        /// 🌀️ The one-line migration for today's independent `loading`/`waiting` flag pairs — `loading`
        /// wins precedence when both are set, matching the prior ad-hoc convention.
        pub fn busy(loading: bool, waiting: bool) -> Self {
            if loading {
                UiStatus::Loading
            } else if waiting {
                UiStatus::Waiting
            } else {
                UiStatus::Idle
            }
        }
    }
    //#endregion 🔖️Presence

    //#region 🔖️ContextMenu
    /// 🖱️ A render-time address for an on-demand context menu, carried by any `UiNode`/scene surface —
    /// bytes only, never items. At right-click time the host resolves the nearest `menu` up the tree and
    /// asks the owning plugin's `context-menu` WIT export to compute rows fresh (see
    /// `ContextMenuRequest`/`ContextMenuResponse`); nothing here is cached across clicks.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct UiMenuRef {
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub args: Option<DslValue>,
    }

    /// 🖱️ One row of a resolved context menu — serde camelCase twin of TS `ContextMenuItemSpec`
    /// (`framework/core/js/index.ts`). Plugins build these with `MenuBuilder`; the host maps them
    /// through `ContextMenuController` (React) / `render_context_menu` (wgpu) unchanged.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct ContextMenuItemSpec {
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub color: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub shortcut: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub disabled: Option<bool>,
        /// 💬️ Why a disabled row cannot run, producer-localized: the row stays focusable (`aria-disabled`) and every host
        /// announces this as its description (`aria-describedby`) — the `RowAction::disabled_because` contract on menu rows.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub reason: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub separator: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub checked: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub destructive: Option<bool>,
        /// 🎯️ An action id, dispatched via the surface's already-scoped `dispatch(action, args)` — NOT
        /// an `ActionDescriptor` (no separate `controllerId`); matches the pre-existing TS
        /// `ContextMenuItemSpec.action` shape (`framework/core/js/index.ts`), which every emitting
        /// plugin already produces this way.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub action: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub args: Option<DslValue>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub hover_action: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub hover_args: Option<DslValue>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub children: Option<Vec<ContextMenuItemSpec>>,
    }

    impl ContextMenuItemSpec {
        /// 🧾️ This row disabled because of `reason` — still focusable, never dispatched, the reason announced as its description.
        pub fn disabled_because(mut self, reason: String) -> Self {
            self.disabled = Some(true);
            self.reason = Some(reason);
            self
        }
    }

    //#region 🗂️ContextMenuOrganizer
    /// 🗂️ Canonical ribbon-parent taxonomy — Rust twin of ui-react's closed `UiRibbonParentCategory`
    /// union (`🧰️framework/🔨️modules/🖱️ui/⚛️react/⚡️implementations/🟦️typescript/🟦️.tsx` ~3317). Id
    /// spelling and order are load-bearing: `organize_context_menu` sorts `menu.group.<category>` rows by
    /// this order (unknown categories sort after, in emit order) and `ribbon_parent_label` covers exactly
    /// these 20 ids.
    pub const RIBBON_PARENT_CATEGORIES: [&str; 20] =
        ["history", "hand", "selection", "lasso", "filter", "open", "save", "transfer", "transform", "create", "view", "actions", "settings", "methods", "mode", "targets", "export", "tools", "utilities", "sync"];

    /// 🗂️ EN/DE label for a `RIBBON_PARENT_CATEGORIES` id — hand-maintained Rust twin of ui-react's
    /// `uiRibbonParentEn`/`uiRibbonParentDe` bundles (same file as above, ~3826/~3797). `None` for any id
    /// outside the closed 20-id taxonomy — callers (`shell_context_menu_item_from_spec`) fall back to the
    /// raw category id in that case.
    pub fn ribbon_parent_label(category: &str, is_de: bool) -> Option<&'static str> {
        Some(match (category, is_de) {
            ("history", false) => "History",
            ("history", true) => "Verlauf",
            ("hand", false) => "Hand",
            ("hand", true) => "Hand",
            ("selection", false) => "Selection",
            ("selection", true) => "Auswahl",
            ("lasso", false) => "Lasso",
            ("lasso", true) => "Lasso",
            ("filter", false) => "Filter",
            ("filter", true) => "Filter",
            ("open", false) => "Open",
            ("open", true) => "Öffnen",
            ("save", false) => "Save",
            ("save", true) => "Speichern",
            ("transfer", false) => "Transfer",
            ("transfer", true) => "Transfer",
            ("transform", false) => "Transform",
            ("transform", true) => "Transformieren",
            ("create", false) => "Create",
            ("create", true) => "Erstellen",
            ("view", false) => "View",
            ("view", true) => "Ansicht",
            ("actions", false) => "Actions",
            ("actions", true) => "Aktionen",
            ("settings", false) => "Settings",
            ("settings", true) => "Einstellungen",
            ("methods", false) => "Methods",
            ("methods", true) => "Methoden",
            ("mode", false) => "Mode",
            ("mode", true) => "Modus",
            ("targets", false) => "Targets",
            ("targets", true) => "Ziele",
            ("export", false) => "Export",
            ("export", true) => "Export",
            ("tools", false) => "Tools",
            ("tools", true) => "Werkzeuge",
            ("utilities", false) => "Utilities",
            ("utilities", true) => "Hilfsmittel",
            ("sync", false) => "Sync",
            ("sync", true) => "Sync",
            _ => return None,
        })
    }

    /// 🗂️ Every context-menu GROUP row id carries this prefix — the one spelling `organize_context_menu`
    /// synthesizes and `shell_context_menu_item_from_spec` resolves against. Twin of ui-react's
    /// `CONTEXT_MENU_GROUP_ID_PREFIX` (`🧰️framework/🔨️modules/🔺️mesh/🟦️.ts`).
    pub const CONTEXT_MENU_GROUP_ID_PREFIX: &str = "menu.group.";
    /// 🗂️ The one group category OUTSIDE [`RIBBON_PARENT_CATEGORIES`]: the overflow bucket
    /// `organize_context_menu` synthesizes when the row budget is exceeded. Twin of ui-react's
    /// `CONTEXT_MENU_OVERFLOW_CATEGORY`.
    pub const CONTEXT_MENU_OVERFLOW_CATEGORY: &str = "more";

    /// 🗂️ EN/DE display label for a `menu.group.<category>` row, which the guest emits with `label: None`
    /// by contract (the host owns the chrome vocabulary). Taxonomy categories resolve through
    /// [`ribbon_parent_label`]; [`CONTEXT_MENU_OVERFLOW_CATEGORY`] — the one id outside the closed 20-id
    /// taxonomy, and therefore the one that used to render with an EMPTY label — resolves here. `None`
    /// for a row that is not a group row, or a group row whose category is unknown. Twin of ui-react's
    /// `contextMenuGroupLabel` (`🛠️ShellHelpers/🟦️.tsx`), which reads `ui.contextMenu.more` for the same id.
    pub fn context_menu_group_label(id: &str, is_de: bool) -> Option<&'static str> {
        let category = id.strip_prefix(CONTEXT_MENU_GROUP_ID_PREFIX)?;
        if category == CONTEXT_MENU_OVERFLOW_CATEGORY {
            return Some(if is_de { "Mehr" } else { "More" });
        }
        ribbon_parent_label(category, is_de)
    }

    const CONTEXT_MENU_ROW_BUDGET: usize = 9;
    const CONTEXT_MENU_PRIMARY_BUDGET: usize = 5;

    // 🚫️async: E1 pure accessor consumed by sync-only std call sites (Option::map fn-value, sort_by_key comparator) — see R9
    fn context_menu_is_bare_separator(item: &ContextMenuItemSpec) -> bool {
        item.separator == Some(true) && item.label.is_none()
    }

    /// 🗂️ D1: a separator carrying a `label` is a non-interactive section header, not a divider.
    // 🚫️async: E1 pure accessor consumed by sync-only std call sites (Option::map fn-value, sort_by_key comparator) — see R9
    fn context_menu_is_header(item: &ContextMenuItemSpec) -> bool {
        item.separator == Some(true) && item.label.is_some()
    }

    // 🚫️async: E1 pure accessor consumed by sync-only std call sites (Option::map fn-value, sort_by_key comparator) — see R9
    fn context_menu_is_group_row(item: &ContextMenuItemSpec) -> bool {
        item.id.starts_with(CONTEXT_MENU_GROUP_ID_PREFIX)
    }

    // 🚫️async: E1 pure accessor consumed by sync-only std call sites (Option::map fn-value, sort_by_key comparator) — see R9
    fn context_menu_group_category(item: &ContextMenuItemSpec) -> &str {
        item.id.strip_prefix(CONTEXT_MENU_GROUP_ID_PREFIX).unwrap_or(item.id.as_str())
    }

    // 🚫️async: E1 pure accessor consumed by sync-only std call sites (Option::map fn-value, sort_by_key comparator) — see R9
    fn context_menu_taxonomy_rank(category: &str) -> usize {
        RIBBON_PARENT_CATEGORIES.iter().position(|known| *known == category).unwrap_or(RIBBON_PARENT_CATEGORIES.len())
    }

    fn context_menu_separator_row(seed: usize) -> ContextMenuItemSpec {
        ContextMenuItemSpec { id: format!("separator-organized-{seed}"), separator: Some(true), ..Default::default() }
    }

    /// 🗂️ Collapses a run of consecutive bare (unlabeled) separators down to one, then drops a bare
    /// separator left at the very start or end (nothing to separate from/to). A labeled separator (header,
    /// see `context_menu_is_header`) is never touched by this — it always survives in place, adjacent bare
    /// separators collapse/drop around it independently.
    fn context_menu_normalize_separators(items: Vec<ContextMenuItemSpec>) -> Vec<ContextMenuItemSpec> {
        let mut out: Vec<ContextMenuItemSpec> = Vec::with_capacity(items.len());
        for item in items {
            if context_menu_is_bare_separator(&item) && out.last().is_some_and(context_menu_is_bare_separator) {
                continue;
            }
            out.push(item);
        }
        if out.first().is_some_and(context_menu_is_bare_separator) {
            out.remove(0);
        }
        while out.last().is_some_and(context_menu_is_bare_separator) {
            out.pop();
        }
        out
    }

    /// 🗂️ Merges rows sharing a `menu.group.<category>` id at the position of the first occurrence,
    /// concatenating and deduping their `children` by id (first occurrence wins).
    fn context_menu_merge_group_rows(items: Vec<ContextMenuItemSpec>) -> Vec<ContextMenuItemSpec> {
        let mut out: Vec<ContextMenuItemSpec> = Vec::with_capacity(items.len());
        let mut group_index: HashMap<String, usize> = HashMap::new();
        for item in items {
            if context_menu_is_group_row(&item) {
                if let Some(&index) = group_index.get(&item.id) {
                    let children = out[index].children.get_or_insert_with(Vec::new);
                    for child in item.children.unwrap_or_default() {
                        if !children.iter().any(|existing| existing.id == child.id) {
                            children.push(child);
                        }
                    }
                } else {
                    group_index.insert(item.id.clone(), out.len());
                    out.push(item);
                }
            } else {
                out.push(item);
            }
        }
        out
    }

    /// 🗂️ ≤9-interactive-row emission (D2 rule): plain leaves/headers in emit order, then group rows in
    /// taxonomy order (unknown categories after, emit order), then — only if any exist — a separator
    /// followed by destructive leaves.
    fn context_menu_emit_within_budget(items: Vec<ContextMenuItemSpec>) -> Vec<ContextMenuItemSpec> {
        let mut leaves_and_headers: Vec<ContextMenuItemSpec> = Vec::new();
        let mut group_rows: Vec<ContextMenuItemSpec> = Vec::new();
        let mut destructive_leaves: Vec<ContextMenuItemSpec> = Vec::new();
        for item in items {
            if context_menu_is_group_row(&item) {
                group_rows.push(item);
            } else if item.destructive == Some(true) {
                destructive_leaves.push(item);
            } else {
                leaves_and_headers.push(item);
            }
        }
        group_rows.sort_by_key(|group| context_menu_taxonomy_rank(context_menu_group_category(group)));
        let mut out = leaves_and_headers;
        out.extend(group_rows);
        if !destructive_leaves.is_empty() {
            out.push(context_menu_separator_row(out.len()));
            out.extend(destructive_leaves);
        }
        out
    }

    /// 🗂️ >9-interactive-row emission (D2 rule): the first 5 plain leaves outside any header section stay
    /// primaries; every header's trailing run of leaves folds into a group keyed by that header's own
    /// (slugified) label; every other plain leaf buckets into `menu.group.<category_of(action) ?? "actions">`;
    /// pre-existing group rows pass through unchanged; groups then sort in taxonomy order and, if the
    /// primaries+groups row count is still over budget, the excess trailing groups fold into one
    /// `menu.group.more`. Destructive leaves are carried separately and appended last, after a separator.
    fn context_menu_emit_over_budget(items: Vec<ContextMenuItemSpec>, category_of: &dyn Fn(&str) -> Option<String>) -> Vec<ContextMenuItemSpec> {
        fn bucket_mut(buckets: &mut Vec<ContextMenuItemSpec>, id: String) -> usize {
            match buckets.iter().position(|bucket| bucket.id == id) {
                Some(index) => index,
                None => {
                    buckets.push(ContextMenuItemSpec { id, label: None, children: Some(Vec::new()), ..Default::default() });
                    buckets.len() - 1
                }
            }
        }

        let mut primaries: Vec<ContextMenuItemSpec> = Vec::new();
        let mut existing_groups: Vec<ContextMenuItemSpec> = Vec::new();
        let mut destructive_leaves: Vec<ContextMenuItemSpec> = Vec::new();
        let mut bucketed_groups: Vec<ContextMenuItemSpec> = Vec::new();
        let mut current_header_key: Option<String> = None;

        for item in items {
            if context_menu_is_header(&item) {
                current_header_key = item.label.clone();
                continue;
            }
            if context_menu_is_group_row(&item) {
                existing_groups.push(item);
                current_header_key = None;
                continue;
            }
            if item.destructive == Some(true) {
                destructive_leaves.push(item);
                continue;
            }
            if let Some(header_label) = &current_header_key {
                let slug = header_label.to_lowercase().split_whitespace().collect::<Vec<_>>().join("-");
                let index = bucket_mut(&mut bucketed_groups, format!("{CONTEXT_MENU_GROUP_ID_PREFIX}{slug}"));
                bucketed_groups[index].children.get_or_insert_with(Vec::new).push(item);
                continue;
            }
            if primaries.len() < CONTEXT_MENU_PRIMARY_BUDGET {
                primaries.push(item);
                continue;
            }
            let category = category_of(item.action.as_deref().unwrap_or(item.id.as_str())).unwrap_or_else(|| "actions".into());
            let index = bucket_mut(&mut bucketed_groups, format!("{CONTEXT_MENU_GROUP_ID_PREFIX}{category}"));
            bucketed_groups[index].children.get_or_insert_with(Vec::new).push(item);
        }

        let mut groups = existing_groups;
        groups.extend(bucketed_groups);
        groups.sort_by_key(|group| context_menu_taxonomy_rank(context_menu_group_category(group)));

        let mut out = primaries;
        out.extend(groups);
        if out.len() > CONTEXT_MENU_ROW_BUDGET {
            let fold_from = CONTEXT_MENU_ROW_BUDGET - 1;
            let overflowing_groups = out.split_off(fold_from);
            let mut folded_children: Vec<ContextMenuItemSpec> = Vec::new();
            for group in overflowing_groups {
                folded_children.extend(group.children.unwrap_or_default());
            }
            out.push(ContextMenuItemSpec { id: format!("{CONTEXT_MENU_GROUP_ID_PREFIX}{CONTEXT_MENU_OVERFLOW_CATEGORY}"), label: None, children: Some(folded_children), ..Default::default() });
        }
        if !destructive_leaves.is_empty() {
            out.push(context_menu_separator_row(out.len()));
            out.extend(destructive_leaves);
        }
        out
    }

    /// 🗂️ Pure organizer enforced at every context-menu funnel (SDK `VcsArtifactApp::context_menu`, shell
    /// builders) — recurses into `children`, normalizes separators (labeled = kept header, bare
    /// leading/trailing/doubled = dropped), merges duplicate `menu.group.<category>` rows (deduping their
    /// children by id), then applies the ≤9-row / >9-row emission policy from D2 of the grouped-context-menu
    /// mechanism design (`context_menu_emit_within_budget`/`context_menu_emit_over_budget`).
    /// `category_of` resolves a leaf's dispatched action id to a `RIBBON_PARENT_CATEGORIES` id (`None`
    /// buckets into `"actions"`) — pass `AppActionRegistry::category_of` at the SDK funnel, or
    /// `ActionDefinition.category` lookups in shell builders.
    pub fn organize_context_menu(items: Vec<ContextMenuItemSpec>, category_of: &dyn Fn(&str) -> Option<String>) -> Vec<ContextMenuItemSpec> {
        let mut recursed: Vec<ContextMenuItemSpec> = Vec::with_capacity(items.len());
        for item in items {
            let children = item.children.map(|children| organize_context_menu(children, category_of));
            recursed.push(ContextMenuItemSpec { children, ..item });
        }
        let items = context_menu_normalize_separators(recursed);
        let items = context_menu_merge_group_rows(items);
        let interactive_count = items.iter().filter(|item| item.separator != Some(true)).count();
        if interactive_count <= CONTEXT_MENU_ROW_BUDGET {
            context_menu_emit_within_budget(items)
        } else {
            context_menu_emit_over_budget(items, category_of)
        }
    }

    /// 🗂️ Declarative input row for `build_shell_context_menu_specs` — Rust twin of the TS `ShellMenuAction`
    /// the shell fallback builder consumes (`buildShellContextMenuItems`, renderer). `kind` is the raw
    /// `ActionKind`/`CommandKind` discriminant string, carried through for host-side styling parity (unused
    /// by the builder itself).
    #[derive(Clone, Debug, PartialEq)]
    pub struct ShellMenuAction {
        pub id: String,
        pub label: String,
        pub icon: Option<String>,
        pub keys: Option<String>,
        pub kind: String,
        pub category: Option<String>,
        pub in_palette: bool,
        pub arg_carrying: bool,
    }

    /// 🗂️ Shell fallback context menu (window-background right-click, "no plugin resolved a menu") builder
    /// — filters to `in_palette` actions, emits one leaf per action (an `arg_carrying` action routes
    /// through the reserved `"shell.openActionPane"` action id with `{"actionId": id}` args so the host can
    /// prompt for arguments before dispatch), appends a fixed `"shell.openPalette"` leaf when
    /// `include_palette`, then runs the whole thing through `organize_context_menu` (D5) exactly like every
    /// plugin-emitted menu.
    pub fn build_shell_context_menu_specs(actions: &[ShellMenuAction], include_palette: bool) -> Vec<ContextMenuItemSpec> {
        let mut items: Vec<ContextMenuItemSpec> = actions
            .iter()
            .filter(|action| action.in_palette)
            .map(|action| ContextMenuItemSpec {
                id: action.id.clone(),
                label: Some(action.label.clone()),
                icon: action.icon.clone(),
                shortcut: action.keys.clone(),
                action: Some(if action.arg_carrying { "shell.openActionPane".into() } else { action.id.clone() }),
                args: action.arg_carrying.then(|| DslValue::Object(vec![("actionId".into(), DslValue::String(action.id.clone()))])),
                ..Default::default()
            })
            .collect();
        if include_palette {
            items.push(ContextMenuItemSpec { id: "shell.openPalette".into(), label: Some("Command Palette".into()), action: Some("shell.openPalette".into()), ..Default::default() });
        }
        let categories: HashMap<String, Option<String>> = actions.iter().map(|action| (action.id.clone(), action.category.clone())).collect();
        organize_context_menu(items, &|id| categories.get(id).cloned().flatten())
    }
    //#endregion 🗂️ContextMenuOrganizer

    /// 🖱️ Payload of the WIT `context-menu` export's request — mirrors `context-menu-request-json`'s
    /// `json` string. `surface` carries scene-target info (`World3D`/`nodeGraph`/`tiledMap`/... hit-test
    /// results); `menu` is the `UiMenuRef` the host resolved from `data-menu-id`/a scene surface
    /// convention id (`"world3d"`, `"nodeGraph"`, `"window"`, `"panel:<tabId>"`, ...).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct ContextMenuHit {
        pub domain: String,
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub label: Option<String>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct ContextMenuSelectionGroup {
        pub domain: String,
        pub ids: Vec<String>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct ContextMenuTextContext {
        pub caret: usize,
        pub has_selection: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub word: Option<String>,
        pub can_rename: bool,
        pub has_completions: bool,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct ContextMenuSurfaceTarget {
        pub surface_id: String,
        pub kind: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        pub hits: Vec<ContextMenuHit>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        pub selection: Vec<ContextMenuSelectionGroup>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub text: Option<ContextMenuTextContext>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct ContextMenuPoint {
        pub x: f64,
        pub y: f64,
    }

    /// 🖱️ The plugin-facing on-demand menu request — deliberately does NOT carry view state (this crate
    /// must never reference `semio_framework`'s `ViewModel`, same boundary as every other type
    /// here). Mirrors `handle_action`/`render`/`tool_measures`, which all take `view_state: &ViewModel`
    /// as a separate `ArtifactApp` method parameter rather than embedding it in the request payload; the
    /// plugin SDK's `plugin_context_menu` free function parses the WIT-level combined JSON (which DOES
    /// carry `viewState`, matching the TS `PluginContextMenuRequest` wire shape) and splits it into this
    /// smaller struct plus a typed `ViewModel` before calling `ArtifactApp::context_menu`.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct ContextMenuRequest {
        pub menu: UiMenuRef,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub surface: Option<ContextMenuSurfaceTarget>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub window_instance_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub point: Option<ContextMenuPoint>,
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct ContextMenuResponse {
        pub items: Vec<ContextMenuItemSpec>,
    }
    //#endregion 🔖️ContextMenu

    //#region 🔖️PanelTabConstants
    pub const FRAMEWORK_PANEL_TAB_ARTIFACT_ID: &str = "framework.panel.artifact";
    pub const FRAMEWORK_PANEL_TAB_CATALOGUE_ID: &str = "framework.panel.catalogue";
    pub const FRAMEWORK_PANEL_TAB_INSPECTION_ID: &str = "framework.panel.inspection";
    pub const FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL: &str = "Artifact";
    pub const FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL: &str = "Catalogue";
    pub const FRAMEWORK_PANEL_TAB_INSPECTION_LABEL: &str = "Inspection";
    pub const FRAMEWORK_PANEL_TAB_ARTIFACT_ICON_ID: &str = "framework.panel.artifact";
    pub const FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID: &str = "framework.panel.catalogue";
    pub const FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID: &str = "framework.panel.inspection";
    pub const FRAMEWORK_PANEL_TAB_PARAMETERS_ID: &str = "framework.panel.parameters";
    pub const FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL: &str = "Parameters";
    pub const FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID: &str = "framework.panel.parameters";
    /// 🕰️ Auto-injected into every app's `panel_tabs` by `AppBuilder::build_definition` — unlike
    /// Document/Catalogue/Inspection/Parameters (per-app content, opt-in), the command-history panel's
    /// content is framework-generic (`HistoryView`), so every app gets it unconditionally.
    pub const FRAMEWORK_PANEL_TAB_HISTORY_ID: &str = "framework.panel.history";
    pub const FRAMEWORK_PANEL_TAB_HISTORY_LABEL: &str = "History";
    pub const FRAMEWORK_PANEL_TAB_HISTORY_ICON_ID: &str = "framework.panel.history";
    /// 🕰️ Reserved `body_key` intercepted first in `VcsArtifactApp::render`, before any app-specific
    /// body-key match — both renderers fetch it like any other panel-tab body.
    pub const FRAMEWORK_HISTORY_BODY_KEY: &str = "framework.body.history";
    /// ⏯️ Auto-injected into the `panel_tabs` of every app that declares a tool run (`ToolRunDefinition` on a tool or
    /// utility) by `AppBuilder::build_definition`: the framework ToolRun panel, one group per run, fetched as the
    /// reserved body `framework.body.toolRun` and revealed by the shell when a run starts.
    pub const FRAMEWORK_PANEL_TAB_TOOL_RUN_ID: &str = "framework.panel.toolRun";
    pub const FRAMEWORK_PANEL_TAB_TOOL_RUN_LABEL: &str = "Tool runs";
    pub const FRAMEWORK_PANEL_TAB_TOOL_RUN_ICON_ID: &str = "framework.panel.toolRun";

    /// 🗣️ Resolves a well-known framework panel-tab id to its native English/German label; unknown ids resolve to None so app-specific panel tabs are left untouched.
    pub fn framework_panel_tab_label(id: &str, is_de: bool) -> Option<&'static str> {
        match (id, is_de) {
            (FRAMEWORK_PANEL_TAB_ARTIFACT_ID, false) => Some(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL),
            (FRAMEWORK_PANEL_TAB_ARTIFACT_ID, true) => Some("Dokument"),
            (FRAMEWORK_PANEL_TAB_CATALOGUE_ID, false) => Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL),
            (FRAMEWORK_PANEL_TAB_CATALOGUE_ID, true) => Some("Katalog"),
            (FRAMEWORK_PANEL_TAB_INSPECTION_ID, false) => Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL),
            (FRAMEWORK_PANEL_TAB_INSPECTION_ID, true) => Some("Inspektion"),
            (FRAMEWORK_PANEL_TAB_PARAMETERS_ID, false) => Some(FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL),
            (FRAMEWORK_PANEL_TAB_PARAMETERS_ID, true) => Some("Parameter"),
            (FRAMEWORK_PANEL_TAB_HISTORY_ID, false) => Some(FRAMEWORK_PANEL_TAB_HISTORY_LABEL),
            (FRAMEWORK_PANEL_TAB_HISTORY_ID, true) => Some("Verlauf"),
            (FRAMEWORK_PANEL_TAB_TOOL_RUN_ID, false) => Some(FRAMEWORK_PANEL_TAB_TOOL_RUN_LABEL),
            (FRAMEWORK_PANEL_TAB_TOOL_RUN_ID, true) => Some("Werkzeugläufe"),
            _ => None,
        }
    }
    //#endregion 🔖️PanelTabConstants

    //#region 🔖️WindowLayout
    // 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
    fn kind_window() -> String {
        "window".into()
    }

    // 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
    fn kind_stack() -> String {
        "stack".into()
    }

    /// 🧭️ Corner of a window stack where a tab chip docks.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub enum WindowStackCorner {
        #[default]
        TopLeft,
        TopRight,
        BottomLeft,
        BottomRight,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct WindowLayoutWindowNode {
        #[serde(default = "kind_window")]
        #[value(default = "kind_window")]
        pub kind: String,
        pub window_kind_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub instance_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub template_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        pub corner: Option<WindowStackCorner>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct WindowLayoutStackNode {
        #[serde(default = "kind_stack")]
        #[value(default = "kind_stack")]
        pub kind: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub size: Option<f64>,
        // ⚠️ `alias = "activeId"` is deliberately NOT mirrored into `#[value(...)]` — the
        // first-party derive (`🌱️value/✨️derive`) has no `alias` attribute (see its module
        // docstring's supported-attribute list) and an unrecognized key is a hard compile error.
        // `ToValue`/`FromValue` round-trips on the canonical `activeWindowKindId` key only; the
        // serde path keeps accepting the legacy `activeId` alias unchanged.
        #[serde(skip_serializing_if = "Option::is_none", alias = "activeId")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub active_window_kind_id: Option<String>,
        pub children: Vec<WindowLayoutWindowNode>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct WindowLayoutAxisNode {
        pub kind: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub size: Option<f64>,
        pub children: Vec<WindowLayoutChild>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum WindowLayoutChild {
        Axis(WindowLayoutAxisNode),
        Stack(WindowLayoutStackNode),
    }

    impl<'de> Deserialize<'de> for WindowLayoutChild {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let value = serde_json::Value::deserialize(deserializer)?;
            match serde_window_layout_node_is_stack(&value).map_err(serde::de::Error::custom)? {
                true => serde_json::from_value(value).map(WindowLayoutChild::Stack).map_err(serde::de::Error::custom),
                false => serde_json::from_value(value).map(WindowLayoutChild::Axis).map_err(serde::de::Error::custom),
            }
        }
    }

    // 🌱️ Hand-written, not derived: `#[serde(untagged)]` has no `#[value(...)]` equivalent (the
    // derive's supported container attributes are `rename_all`/`tag`/`content`/`default`/
    // `deny_unknown_fields`/`transparent`/`bound` only — see `🌱️value/✨️derive/🦀️.rs`'s module
    // docstring). The canonical `kind` field is the discriminator because both shapes may omit
    // every optional field and an order-based trial would then misclassify an empty stack as an axis.
    impl ToValue for WindowLayoutChild {
        fn to_value(&self) -> DslValue {
            match self {
                WindowLayoutChild::Axis(node) => node.to_value(),
                WindowLayoutChild::Stack(node) => node.to_value(),
            }
        }
    }

    impl FromValue for WindowLayoutChild {
        fn from_value(value: DslValue) -> Result<Self, semio_framework_value::ValueError> {
            match window_layout_node_is_stack(&value)? {
                true => WindowLayoutStackNode::from_value(value).map(WindowLayoutChild::Stack),
                false => WindowLayoutAxisNode::from_value(value).map(WindowLayoutChild::Axis),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum WindowLayoutRoot {
        Axis(WindowLayoutAxisNode),
        Stack(WindowLayoutStackNode),
    }

    impl<'de> Deserialize<'de> for WindowLayoutRoot {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let value = serde_json::Value::deserialize(deserializer)?;
            match serde_window_layout_node_is_stack(&value).map_err(serde::de::Error::custom)? {
                true => serde_json::from_value(value).map(WindowLayoutRoot::Stack).map_err(serde::de::Error::custom),
                false => serde_json::from_value(value).map(WindowLayoutRoot::Axis).map_err(serde::de::Error::custom),
            }
        }
    }

    // 🌱️ Hand-written for the same reason as `WindowLayoutChild` above — `#[serde(untagged)]` has
    // no `#[value(...)]` equivalent.
    impl ToValue for WindowLayoutRoot {
        fn to_value(&self) -> DslValue {
            match self {
                WindowLayoutRoot::Axis(node) => node.to_value(),
                WindowLayoutRoot::Stack(node) => node.to_value(),
            }
        }
    }

    impl FromValue for WindowLayoutRoot {
        fn from_value(value: DslValue) -> Result<Self, semio_framework_value::ValueError> {
            match window_layout_node_is_stack(&value)? {
                true => WindowLayoutStackNode::from_value(value).map(WindowLayoutRoot::Stack),
                false => WindowLayoutAxisNode::from_value(value).map(WindowLayoutRoot::Axis),
            }
        }
    }

    fn window_layout_node_is_stack(value: &DslValue) -> Result<bool, semio_framework_value::ValueError> {
        let DslValue::Object(entries) = value else { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected window layout object")) };
        let kind = entries.iter().find(|(key, _)| key == "kind").and_then(|(_, value)| value.as_str()).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing field `kind`"))?;
        window_layout_kind_is_stack(kind).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error))
    }

    fn serde_window_layout_node_is_stack(value: &serde_json::Value) -> Result<bool, String> {
        let kind = value.as_object().and_then(|entries| entries.get("kind")).and_then(serde_json::Value::as_str).ok_or_else(|| "missing field `kind`".to_string())?;
        window_layout_kind_is_stack(kind)
    }

    fn window_layout_kind_is_stack(kind: &str) -> Result<bool, String> {
        match kind {
            "stack" => Ok(true),
            "row" | "column" | "horizontal" | "vertical" => Ok(false),
            _ => Err(format!("unsupported window layout kind `{kind}`")),
        }
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct WindowLayout {
        pub root: WindowLayoutRoot,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct NamedLayout {
        pub id: String,
        pub label: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub icon_id: Option<IconName>,
        pub layout: WindowLayout,
        pub origin: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub group_path: Option<Vec<String>>,
    }

    pub fn create_window_layout(window_kind_id: impl Into<String>, title: Option<String>, instance_id: Option<String>, template_id: Option<String>) -> WindowLayoutWindowNode {
        WindowLayoutWindowNode { kind: kind_window(), window_kind_id: window_kind_id.into(), title, instance_id, template_id, corner: None }
    }

    pub fn create_stack_layout(window_kind_ids: &[String], titles: Option<&[String]>) -> WindowLayout {
        let mut children = Vec::with_capacity(window_kind_ids.len());
        for (index, id) in window_kind_ids.iter().enumerate() {
            children.push(create_window_layout(id.clone(), titles.and_then(|rows| rows.get(index).cloned()), None, None));
        }
        WindowLayout { root: WindowLayoutRoot::Stack(WindowLayoutStackNode { kind: kind_stack(), size: None, active_window_kind_id: None, children }) }
    }

    /// 🪟️ `direction` is an AXIS direction (`row`/`column`). `stack` is not an axis — it is the
    /// tabbed leaf container — so a `stack` request is answered with the single stack root
    /// [`create_stack_layout`] mints. Minting `Axis { kind: "stack" }` instead put a stack node
    /// under a node the whole shell reads AS a stack, and every consumer that walks a stack's
    /// children as window leaves (the React seed resolver
    /// `convertFrameworkLayoutNodeToModeLayout`) then produced windows with no id — which is how
    /// `🎞️animate` booted with zero panes (ticket 26/09/18, slice F1).
    pub fn create_default_layout(window_ids: &[String], direction: &str, sizes: Option<&[f64]>, titles: Option<&[String]>) -> WindowLayout {
        if direction == kind_stack() {
            return create_stack_layout(window_ids, titles);
        }
        let mut children = Vec::with_capacity(window_ids.len());
        for (index, id) in window_ids.iter().enumerate() {
            let window = create_window_layout(id.clone(), titles.and_then(|rows| rows.get(index).cloned()).or_else(|| Some(id.clone())), None, None);
            children.push(WindowLayoutChild::Stack(WindowLayoutStackNode { kind: kind_stack(), size: sizes.and_then(|rows| rows.get(index).copied()), active_window_kind_id: None, children: vec![window] }));
        }
        WindowLayout { root: WindowLayoutRoot::Axis(WindowLayoutAxisNode { kind: direction.into(), size: None, children }) }
    }

    pub fn create_tab_stack_layout(window_ids: &[String], titles: Option<&[String]>) -> WindowLayout {
        create_stack_layout(window_ids, titles)
    }

    /// 🪟️ Builds a balanced fallback layout for an app that declares no `default_layout`: a single
    /// stack when there is one window, otherwise an even row of single-window stacks.
    pub fn even_window_layout(window_ids: &[String]) -> WindowLayout {
        if window_ids.is_empty() {
            return WindowLayout { root: WindowLayoutRoot::Stack(WindowLayoutStackNode { kind: kind_stack(), size: None, active_window_kind_id: None, children: vec![] }) };
        }
        if window_ids.len() == 1 {
            return WindowLayout {
                root: WindowLayoutRoot::Stack(WindowLayoutStackNode { kind: kind_stack(), size: None, active_window_kind_id: Some(window_ids[0].clone()), children: vec![create_window_layout(window_ids[0].clone(), None, None, None)] }),
            };
        }
        let count = window_ids.len() as f64;
        let mut children = Vec::with_capacity(window_ids.len());
        for id in window_ids {
            let window = create_window_layout(id.clone(), None, None, None);
            children.push(WindowLayoutChild::Stack(WindowLayoutStackNode { kind: kind_stack(), size: Some(1.0 / count), active_window_kind_id: Some(id.clone()), children: vec![window] }));
        }
        WindowLayout { root: WindowLayoutRoot::Axis(WindowLayoutAxisNode { kind: "row".into(), size: None, children }) }
    }

    pub fn create_named_layout(id: impl Into<String>, label: impl Into<String>, layout: WindowLayout, origin: impl Into<String>, icon_id: Option<IconName>, group_path: Option<Vec<String>>) -> NamedLayout {
        NamedLayout { id: id.into(), label: label.into(), icon_id, layout, origin: origin.into(), group_path }
    }

    pub fn merge_named_layouts(base: &[NamedLayout], extension: &[NamedLayout]) -> Vec<NamedLayout> {
        let mut merged: HashMap<String, NamedLayout> = HashMap::new();
        for entry in base {
            merged.insert(entry.id.clone(), entry.clone());
        }
        for entry in extension {
            merged.insert(entry.id.clone(), entry.clone());
        }
        merged.into_values().collect()
    }

    /// 🧭️ Collects every `window_kind_id` referenced by a layout tree.
    pub fn collect_window_kind_ids_from_layout(layout: &WindowLayout) -> Vec<String> {
        let mut ids = Vec::new();
        collect_window_kind_ids_from_root(&layout.root, &mut ids);
        ids
    }

    fn collect_window_kind_ids_from_root(root: &WindowLayoutRoot, out: &mut Vec<String>) {
        match root {
            WindowLayoutRoot::Axis(axis) => collect_window_kind_ids_from_children(&axis.children, out),
            WindowLayoutRoot::Stack(stack) => collect_window_kind_ids_from_stack(stack, out),
        }
    }

    fn collect_window_kind_ids_from_children(children: &[WindowLayoutChild], out: &mut Vec<String>) {
        for child in children {
            match child {
                WindowLayoutChild::Axis(axis) => collect_window_kind_ids_from_children(&axis.children, out),
                WindowLayoutChild::Stack(stack) => collect_window_kind_ids_from_stack(stack, out),
            }
        }
    }

    fn collect_window_kind_ids_from_stack(stack: &WindowLayoutStackNode, out: &mut Vec<String>) {
        for window in &stack.children {
            out.push(window.window_kind_id.clone());
        }
    }
    //#endregion 🔖️WindowLayout

    //#region 🔖️WindowMeasure
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct MeasureSelectItem {
        pub id: String,
        pub value: String,
        pub label: String,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
    #[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
    pub enum WindowMeasure {
        Select {
            id: String,
            label: Option<String>,
            value: String,
            items: Vec<MeasureSelectItem>,
            on_change: ActionDescriptor,
        },
        Slider {
            id: String,
            label: Option<String>,
            value: f64,
            min: f64,
            max: f64,
            step: Option<f64>,
            /// 🎚️ Absolute value on the fixed `[min, max]` range that is already preloaded/ready.
            /// Renderers keep `max` stable and draw a highlight from the knob to this extent.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[value(default, skip_serializing_if = "Option::is_none")]
            ready: Option<f64>,
            /// 🌀️ When true, the measure tree leaf shows a loading ring while preload continues.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[value(default, skip_serializing_if = "Option::is_none")]
            loading: Option<bool>,
            /// 🌀️ When true, the measure tree leaf shows a dashed, slower waiting ring; `loading` takes precedence when both are set.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[value(default, skip_serializing_if = "Option::is_none")]
            waiting: Option<bool>,
            /// 🚫️ When true, the slider is inert — used when a parent weight is zero so joint percentages cannot change anything.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[value(default, skip_serializing_if = "Option::is_none")]
            disabled: Option<bool>,
            on_change: ActionDescriptor,
        },
        /// 🔢️ Unbounded numeric entry — the measure-overlay sibling of `UiNumberStepperNode`.
        /// Both bounds are optional: `max: None` means "no ceiling, type any value", which a
        /// [`WindowMeasure::Slider`] can never express because its `min`/`max` are mandatory.
        Number {
            id: String,
            label: Option<String>,
            value: f64,
            /// ⬇️ Inclusive floor; `None` leaves the value unbounded below.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[value(default, skip_serializing_if = "Option::is_none")]
            min: Option<f64>,
            /// ⬆️ Inclusive ceiling; `None` leaves the value unbounded above — renderers must not clamp.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[value(default, skip_serializing_if = "Option::is_none")]
            max: Option<f64>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[value(default, skip_serializing_if = "Option::is_none")]
            step: Option<f64>,
            /// 🎚️ Absolute value already prepared/committed, drawn as a soft extent behind the entry.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[value(default, skip_serializing_if = "Option::is_none")]
            ready: Option<f64>,
            /// 🌀️ When true, the measure tree leaf shows a loading ring while work continues.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[value(default, skip_serializing_if = "Option::is_none")]
            loading: Option<bool>,
            /// 🌀️ When true, the measure tree leaf shows a dashed, slower waiting ring; `loading` takes precedence when both are set.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[value(default, skip_serializing_if = "Option::is_none")]
            waiting: Option<bool>,
            /// 🚫️ When true, the entry is inert.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[value(default, skip_serializing_if = "Option::is_none")]
            disabled: Option<bool>,
            on_change: ActionDescriptor,
        },
        Toggle {
            id: String,
            icon_id: IconName,
            label: Option<String>,
            pressed: bool,
            text: Option<String>,
            on_change: ActionDescriptor,
        },
        Group {
            id: String,
            label: String,
            default_open: Option<bool>,
            /// 🎯️ When `Some(utility_id)`, this group is *utility-scoped chrome*: the shell surfaces it only while
            /// `ViewModel.active_utility_id == utility_id`, and renders it in the dedicated "Utility Options" rail
            /// beside the utility bar — never in the always-on Measures overlay. When absent, the group is a
            /// general measure and stays in the Measures overlay exactly as before. See [`partition_window_measures`].
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            active_utility_id: Option<String>,
            /// 🎚️ Optional header slider — when set with `on_change`, the group row hosts a weight control (e.g. object-kind probability).
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            value: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            min: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            max: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            step: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            ready: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            loading: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            waiting: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            on_change: Option<ActionDescriptor>,
            children: Vec<WindowMeasure>,
        },
    }
    //#endregion 🔖️WindowMeasure

    impl WindowMeasure {
        /// 🌳️ Builds a measure group with default slider/header fields unset.
        pub fn measure_group(id: impl Into<String>, label: impl Into<String>, children: Vec<WindowMeasure>) -> Self {
            Self::Group { id: id.into(), label: label.into(), default_open: None, active_utility_id: None, value: None, min: None, max: None, step: None, ready: None, loading: None, waiting: None, on_change: None, children }
        }
    }

    //#region 🔖️PartitionWindowMeasures
    pub const WINDOW_MEASURE_PARTITION_CAPACITY: usize = 64;

    #[derive(Debug)]
    pub struct WindowMeasurePartition<'a> {
        pub general: UiFixedList<&'a WindowMeasure, WINDOW_MEASURE_PARTITION_CAPACITY>,
        pub utility_options: UiFixedList<&'a WindowMeasure, WINDOW_MEASURE_PARTITION_CAPACITY>,
    }

    /// 🎯️ Splits a window's top-level measures into `(general, utility_options)`.
    ///
    /// A top-level [`WindowMeasure::Group`] tagged with `active_utility_id: Some(id)` is *utility-scoped chrome*:
    /// its **children** land in `utility_options` **only** when `id == active_utility_id`, and the tagged wrapper
    /// is dropped from both buckets otherwise (it is irrelevant to whichever utility — or no utility — is
    /// currently active). The wrapper itself is a routing envelope only — never rendered — so activating a
    /// utility shows its option tree directly (no duplicate utility-name group header). Every untagged group
    /// and every non-group top-level measure stays in `general`, unchanged. Tagging is a top-level concept only.
    pub fn partition_window_measures<'a>(measures: &'a [WindowMeasure], active_utility_id: Option<&str>) -> Result<WindowMeasurePartition<'a>, &'a WindowMeasure> {
        let mut partition = WindowMeasurePartition { general: UiFixedList::default(), utility_options: UiFixedList::default() };
        for measure in measures {
            match measure {
                WindowMeasure::Group { active_utility_id: Some(scoped), children, .. } => {
                    if active_utility_id == Some(scoped.as_str()) {
                        for child in children {
                            partition.utility_options.try_push(child)?;
                        }
                    }
                }
                _ => partition.general.try_push(measure)?,
            }
        }
        Ok(partition)
    }
    //#endregion 🔖️PartitionWindowMeasures

    //#region 🔖️WindowEngagement
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct WindowEngagementOption {
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub icon_id: Option<IconName>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub pressed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub disabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub action: Option<ActionDescriptor>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct WindowEngagementInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub placeholder: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub disabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub on_change: Option<ActionDescriptor>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub on_submit: Option<ActionDescriptor>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub on_repeat_last: Option<ActionDescriptor>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub on_abort: Option<ActionDescriptor>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct WindowEngagementStatus {
        pub id: String,
        pub text: String,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct WindowEngagementPossible {
        pub id: String,
        pub label: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub detail: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub action: Option<ActionDescriptor>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct WindowEngagementRingOption {
        pub id: String,
        pub label: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub disabled: Option<bool>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct WindowEngagementToggleGroupOption {
        pub id: String,
        pub label: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub disabled: Option<bool>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct WindowEngagementSelectItem {
        pub id: String,
        pub value: String,
        pub label: String,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
    #[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
    pub enum WindowEngagementControl {
        Slider { id: Option<String>, label: Option<String>, value: f64, min: f64, max: f64, step: Option<f64>, unit: Option<String>, disabled: Option<bool>, on_change: Option<ActionDescriptor>, on_commit: Option<ActionDescriptor> },
        Stepper { id: Option<String>, label: Option<String>, value: f64, min: Option<f64>, max: Option<f64>, step: Option<f64>, unit: Option<String>, disabled: Option<bool>, on_change: Option<ActionDescriptor>, on_commit: Option<ActionDescriptor> },
        Ring { id: Option<String>, label: Option<String>, value: Option<String>, options: Vec<WindowEngagementRingOption>, disabled: Option<bool>, on_select: Option<ActionDescriptor> },
        ToggleGroup { id: Option<String>, label: Option<String>, value: Option<String>, options: Vec<WindowEngagementToggleGroupOption>, disabled: Option<bool>, on_select: Option<ActionDescriptor> },
        Select { id: Option<String>, label: Option<String>, value: Option<String>, placeholder: Option<String>, items: Vec<WindowEngagementSelectItem>, disabled: Option<bool>, on_change: Option<ActionDescriptor> },
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct WindowEngagement {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub session_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub options: Option<Vec<WindowEngagementOption>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub input: Option<WindowEngagementInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub control: Option<WindowEngagementControl>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub controls: Option<Vec<WindowEngagementControl>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub status: Option<Vec<WindowEngagementStatus>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        pub possible_engagements: Option<Vec<WindowEngagementPossible>>,
    }

    /// 🤝️ Closed replacement for `Option<WindowEngagement>` — makes "this window kind never engages" a
    /// named variant instead of `None`, so absence is an explicit, typed state rather than an implicit gap.
    /// ⚠️ `WindowEngagement` is a wide variant (nested `Vec`/`Option` fields), making `Some` far
    /// larger than `None` — boxing it would be a breaking public-API change (every construction/match
    /// site across ~30 plugins would need `Box::new`/deref updates), out of scope for a mechanical pass.
    #[allow(clippy::large_enum_variant, reason = "boxing is a breaking public API change, out of T1 scope")]
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase", tag = "kind", content = "value")]
    #[value(rename_all = "camelCase", tag = "kind", content = "value")]
    pub enum WindowEngagementSlot {
        #[default]
        None,
        Some(WindowEngagement),
    }

    impl WindowEngagementSlot {
        pub fn as_option(&self) -> Option<&WindowEngagement> {
            match self {
                WindowEngagementSlot::None => None,
                WindowEngagementSlot::Some(engagement) => Some(engagement),
            }
        }
    }

    pub fn default_viewport_engagement() -> WindowEngagement {
        WindowEngagement {
            session_active: Some(true),
            options: None,
            input: None,
            control: None,
            controls: None,
            status: Some(vec![WindowEngagementStatus { id: "framework.viewport.status".into(), text: "Viewport".into() }]),
            possible_engagements: None,
        }
    }

    /// 🎛️ Everything a window kind can expose beyond its rendered body — always present as a shape,
    /// empty collections/`WindowEngagementSlot::None` for windows that don't use a given facet.
    /// Replaces the previously separately-optional `measures`/`engagement` pair on `WindowKindDefinition`.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct WindowOptions {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        pub measures: Vec<WindowMeasure>,
        #[serde(default)]
        #[value(default)]
        pub engagement: WindowEngagementSlot,
    }
    //#endregion 🔖️WindowEngagement

    //#region 🔖️WireFormatGoldenTests
    /** 🧊️ Golden wire-format tests: freeze exact JSON for layout/action/engagement types
    before these move into ui_wgpu, so the move can be proven byte-identical. */
    #[cfg(test)]
    include!("../../🧪️tests/🔬️targets-wgpu-component-layout-layout-wire-format/🦀️.rs");
    //#endregion 🔖️WireFormatGoldenTests

    //#region 🔖️ValueRoundTripTests
    /// 🌱️ `ToValue`/`FromValue` round-trip tests for the keystone types this pass gave the traits
    /// to — `FromValue(ToValue(x)) == x` for each, matching the requirement in ticket
    /// 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
    #[cfg(test)]
    include!("../../🧪️tests/🔬️targets-wgpu-component-layout-value-round-trip/🦀️.rs");
    //#endregion 🔖️ValueRoundTripTests
    // #endregion layout
}

pub mod utilities {
    // #region utilities
    //! 🧰️ Declarative per-mode utility bar utility trees.

    use super::layout::ActionDescriptor;
    use crate::IconName;
    use semio_framework_value::DslValue;
    use semio_framework_value::{FromValue, ToValue};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub enum UtilityCategory {
        Selection,
        Utilities,
        History,
        Sync,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
    #[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
    pub enum UtilityNode {
        Separator {
            id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            order: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            disabled: Option<bool>,
        },
        Button {
            id: String,
            icon_id: IconName,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            label: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            order: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            disabled: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            category: Option<UtilityCategory>,
            on_press: ActionDescriptor,
        },
        Toggle {
            id: String,
            icon_id: IconName,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            label: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            order: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            pressed: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            disabled: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            category: Option<UtilityCategory>,
            on_change: ActionDescriptor,
        },
        Collection {
            id: String,
            icon_id: IconName,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            label: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            order: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            disabled: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[value(skip_serializing_if = "Option::is_none")]
            category: Option<UtilityCategory>,
            children: Vec<UtilityNode>,
        },
    }

    impl UtilityNode {
        pub fn category(&self) -> UtilityCategory {
            match self {
                UtilityNode::Separator { .. } => UtilityCategory::Utilities,
                UtilityNode::Button { category, .. } => category.unwrap_or(UtilityCategory::Utilities),
                UtilityNode::Toggle { category, .. } => category.unwrap_or(UtilityCategory::Utilities),
                UtilityNode::Collection { category, .. } => category.unwrap_or(UtilityCategory::Utilities),
            }
        }

        pub fn with_category(mut self, category: UtilityCategory) -> Self {
            match &mut self {
                UtilityNode::Button { category: slot, .. } | UtilityNode::Toggle { category: slot, .. } | UtilityNode::Collection { category: slot, .. } => *slot = Some(category),
                UtilityNode::Separator { .. } => {}
            }
            self
        }
    }

    pub fn utility_separator(id: impl Into<String>) -> UtilityNode {
        UtilityNode::Separator { id: id.into(), order: None, disabled: None }
    }

    pub fn utility_button(id: impl Into<String>, icon_id: IconName, label: impl Into<String>, on_press: ActionDescriptor) -> UtilityNode {
        let label = label.into();
        UtilityNode::Button { id: id.into(), icon_id, label: Some(label.clone()), text: None, title: Some(label), order: None, disabled: None, category: None, on_press }
    }

    pub fn utility_toggle(id: impl Into<String>, icon_id: IconName, label: impl Into<String>, pressed: bool, on_change: ActionDescriptor) -> UtilityNode {
        let label = label.into();
        UtilityNode::Toggle { id: id.into(), icon_id, label: Some(label.clone()), text: None, title: Some(label), order: None, pressed: Some(pressed), disabled: None, category: None, on_change }
    }

    pub fn utility_collection(id: impl Into<String>, icon_id: IconName, label: impl Into<String>, children: Vec<UtilityNode>) -> UtilityNode {
        let label = label.into();
        UtilityNode::Collection { id: id.into(), icon_id, label: Some(label.clone()), text: None, title: Some(label), order: None, disabled: None, category: None, children }
    }

    //#region 🔖️DeriveUtilityNodes
    /// 🧰️ A resolved utility ready to be laid out into the utility bar. `framework_core` maps its
    /// `UtilityDefinition` onto this before calling `derive_utility_nodes` — `ui_wgpu` can't reference
    /// `framework_core::UtilityDefinition` directly (that crate depends on `ui_wgpu`, not the reverse).
    #[derive(Clone, Debug, PartialEq)]
    pub struct DerivedUtilitySpec {
        pub id: String,
        pub label: String,
        pub icon_id: IconName,
        pub group: Option<String>,
        pub category: Option<UtilityCategory>,
    }

    /// 🧰 Derives the utility bar `UtilityNode` tree from resolved utilities and the host-owned active utility id.
    /// Each utility becomes a `Toggle` whose `pressed` reflects `active_utility_id == Some(id)` and whose
    /// `on_change` dispatches `setActiveUtility { utilityId }` against `controller_id`. Utilities sharing a `group`
    /// collapse into one `Collection` (placed where the group first appears, in utility order); ungrouped
    /// utilities stay flat siblings. A group that ends with exactly one child is hoisted to a top-level toggle —
    /// a lone `group:transform`/`transform` pair must not render as two nested "Transform" rows. This is the
    /// single source of truth for the utility bar — `ArtifactApp::utilities` no longer exists.
    pub fn derive_utility_nodes(controller_id: &str, utilities: &[DerivedUtilitySpec], active_utility_id: Option<&str>) -> Vec<UtilityNode> {
        fn utility_toggle_node(controller_id: &str, utility: &DerivedUtilitySpec, active_utility_id: Option<&str>) -> UtilityNode {
            UtilityNode::Toggle {
                id: utility.id.clone(),
                icon_id: utility.icon_id,
                label: Some(utility.label.clone()),
                text: None,
                title: Some(utility.label.clone()),
                order: None,
                pressed: Some(active_utility_id == Some(utility.id.as_str())),
                disabled: None,
                category: utility.category,
                on_change: ActionDescriptor { controller_id: controller_id.to_string(), action: "setActiveUtility".into(), args: Some(DslValue::Object(vec![("utilityId".into(), DslValue::String(utility.id.clone()))])) },
            }
        }

        let mut nodes: Vec<UtilityNode> = Vec::new();
        let mut group_positions: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for utility in utilities {
            let node = utility_toggle_node(controller_id, utility, active_utility_id);
            match &utility.group {
                None => nodes.push(node),
                Some(group) => {
                    if let Some(&index) = group_positions.get(group) {
                        if let UtilityNode::Collection { children, .. } = &mut nodes[index] {
                            children.push(node);
                        }
                    } else {
                        group_positions.insert(group.clone(), nodes.len());
                        nodes.push(UtilityNode::Collection {
                            id: format!("group:{group}"),
                            icon_id: utility.icon_id,
                            label: Some(group.clone()),
                            text: None,
                            title: Some(group.clone()),
                            order: None,
                            disabled: None,
                            category: utility.category,
                            children: vec![node],
                        });
                    }
                }
            }
        }
        nodes
            .into_iter()
            .map(|node| match node {
                UtilityNode::Collection { mut children, .. } if children.len() == 1 => children.remove(0),
                other => other,
            })
            .collect()
    }
    //#endregion 🔖DeriveUtilityNodes

    //#region 🔖WireFormatGoldenTests
    /** 🧊 Golden wire-format tests: freeze exact JSON for UtilityNode before it moves into ui_wgpu. */
    #[cfg(test)]
    include!("../../🧪️tests/🔬️targets-wgpu-component-utilities-utility-node-wire-format/🦀️.rs");
    //#endregion 🔖WireFormatGoldenTests
    // #endregion utilities
}

pub mod role_chrome {
    // #region role_chrome
    //! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §5: role-aware wgpu
    //! shell chrome primitives, parity with the React shell (lane 1-C) against the SAME frozen
    //! strings/command ids as `📋️contract-freeze.md` §3/§5. Domain-neutral: this crate never depends
    //! on `semio_framework` (see the `wgpu` feature's `Cargo.toml` deps), so `ChromeRole` is a local
    //! wire-compatible mirror of `semio_framework::AppRole`/the TS host's own `AppRole` mirror in
    //! `🎠️kernel/🟦️.ts` — same boundary this file already draws around `PluginCatalog`-style
    //! product data. A concrete `AppRouter`/`ConfigStore` never appears here: callers (the renderer
    //! product) resolve real entries and hand this module only already-resolved data to render.

    use super::layout::{ContextMenuItemSpec, ShellMenuAction};
    use super::utilities::{UtilityCategory, UtilityNode};
    use semio_framework_value::DslValue;
    use semio_framework_value::{FromValue, ToValue};
    use serde::{Deserialize, Serialize};

    //#region 🔖️ChromeRole
    /// 👁️✏️ Wire-compatible with Rust `semio_framework::AppRole`/TS `AppRole` — exactly `"viewer"`/
    /// `"editor"`, contract freeze §1 C1.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
    #[serde(rename_all = "lowercase")]
    #[value(rename_all = "lowercase")]
    pub enum ChromeRole {
        Viewer,
        Editor,
    }

    impl ChromeRole {
        pub fn as_str(self) -> &'static str {
            match self {
                ChromeRole::Viewer => "viewer",
                ChromeRole::Editor => "editor",
            }
        }

        /// 🌱️ Contract freeze §5: boot role from `SEMIO_APP_ROLE`/`VITE_SEMIO_APP_ROLE`, values
        /// `"viewer"`/`"editor"`, default `"editor"` — anything else (unset, empty, unrecognized)
        /// falls back to the frozen default rather than erroring, since a boot-time env var is not a
        /// place to hard-fail a shell.
        pub fn from_boot_env(value: Option<&str>) -> Self {
            match value {
                Some("viewer") => ChromeRole::Viewer,
                _ => ChromeRole::Editor,
            }
        }

        pub fn is_read_only(self) -> bool {
            matches!(self, ChromeRole::Viewer)
        }
    }
    //#endregion 🔖️ChromeRole

    //#region 🔖️FrozenStrings
    /// 🗣️ Window title chip — contract freeze §5: en `"Viewer"`/`"Editor"`, de
    /// `"Betrachter"`/`"Editor"`. Mirrors `layout::ribbon_parent_label`'s own `(value, is_de) ->
    /// &'static str` idiom — these are framework-owned, terminology-invariant strings (no app
    /// terminology axis), same category as that function's own consumers.
    pub fn role_title_chip_text(role: ChromeRole, is_de: bool) -> &'static str {
        match (role, is_de) {
            (ChromeRole::Viewer, false) => "Viewer",
            (ChromeRole::Viewer, true) => "Betrachter",
            (ChromeRole::Editor, false) => "Editor",
            (ChromeRole::Editor, true) => "Editor",
        }
    }

    /// 🗣️ Context-menu/palette entry — contract freeze §5: en `"Open with…"` / de `"Öffnen mit…"`.
    pub fn open_with_label_text(is_de: bool) -> &'static str {
        if is_de {
            "Öffnen mit…"
        } else {
            "Open with…"
        }
    }

    /// 🗣️ "Set as default" toggle — contract freeze §5: en `"Set as default"` / de `"Als Standard
    /// festlegen"`.
    pub fn set_as_default_label_text(is_de: bool) -> &'static str {
        if is_de {
            "Als Standard festlegen"
        } else {
            "Set as default"
        }
    }
    //#endregion 🔖️FrozenStrings

    //#region 🔖️OsCommandIds
    /// 🎮️ Contract freeze §3's frozen OS command ids — the "Set as default" toggle in
    /// `open_with_menu_item` dispatches these.
    pub const OS_OPEN_ARTIFACT_WITH: &str = "os.open-artifact-with";
    pub const OS_SET_DEFAULT_VIEWER: &str = "os.set-default-viewer";
    pub const OS_SET_DEFAULT_EDITOR: &str = "os.set-default-editor";
    pub const OS_CLEAR_DEFAULT_APP: &str = "os.clear-default-app";

    /// 🎮️ Contract freeze §5's frozen palette command ids — dispatched by an "Open with…" leaf.
    pub const PALETTE_OPEN_ARTIFACT_WITH_VIEWER: &str = "open-artifact-with-viewer";
    pub const PALETTE_OPEN_ARTIFACT_WITH_EDITOR: &str = "open-artifact-with-editor";

    fn palette_open_with_action(role: ChromeRole) -> &'static str {
        match role {
            ChromeRole::Viewer => PALETTE_OPEN_ARTIFACT_WITH_VIEWER,
            ChromeRole::Editor => PALETTE_OPEN_ARTIFACT_WITH_EDITOR,
        }
    }

    fn os_set_default_action(role: ChromeRole) -> &'static str {
        match role {
            ChromeRole::Viewer => OS_SET_DEFAULT_VIEWER,
            ChromeRole::Editor => OS_SET_DEFAULT_EDITOR,
        }
    }
    //#endregion 🔖️OsCommandIds

    //#region 🔖️OpenWithMenu
    /// 🗂️ One `AppRouter` entry (contract freeze §3) ready for the "Open with…" menu — the host
    /// resolves the real `AppRouter`/`AppRef`/`OpeningPreferences` state; this crate only renders an
    /// already-resolved list (domain-neutral boundary, see module doc).
    #[derive(Clone, Debug, PartialEq)]
    pub struct OpenWithEntry {
        pub plugin_id: String,
        pub app_id: String,
        pub label: String,
        pub role: ChromeRole,
        pub is_default: bool,
    }

    fn open_with_args(entry: &OpenWithEntry) -> DslValue {
        DslValue::Object(vec![("pluginId".into(), DslValue::String(entry.plugin_id.clone())), ("appId".into(), DslValue::String(entry.app_id.clone()))])
    }

    fn open_with_entry_item(entry: &OpenWithEntry, is_de: bool) -> ContextMenuItemSpec {
        let toggle_action = if entry.is_default { OS_CLEAR_DEFAULT_APP } else { os_set_default_action(entry.role) };
        let toggle = ContextMenuItemSpec {
            id: format!("menu.open-with.{}.{}.set-default", entry.plugin_id, entry.app_id),
            label: Some(set_as_default_label_text(is_de).to_string()),
            checked: Some(entry.is_default),
            action: Some(toggle_action.into()),
            args: Some(open_with_args(entry)),
            ..Default::default()
        };
        ContextMenuItemSpec {
            id: format!("menu.open-with.{}.{}", entry.plugin_id, entry.app_id),
            label: Some(entry.label.clone()),
            action: Some(palette_open_with_action(entry.role).into()),
            args: Some(open_with_args(entry)),
            children: Some(vec![toggle]),
            ..Default::default()
        }
    }

    /// 🖱️ Builds the "Open with…" `ContextMenuItemSpec` submenu row — contract freeze §5: entries
    /// grouped by role, viewer group first then editor (matching `AppRole`'s own declaration order,
    /// contract §1 C1), each headed by a labeled separator (`layout::context_menu_is_header`'s own
    /// convention: a separator carrying a `label` is a non-interactive section header). Every entry
    /// carries a nested "Set as default" toggle child dispatching `OS_SET_DEFAULT_VIEWER`/`_EDITOR`
    /// when turning default ON, `OS_CLEAR_DEFAULT_APP` when turning it OFF — the toggle direction is
    /// resolved here (not left to the host) because there is no single OS command that flips a
    /// boolean; `checked` mirrors `entry.is_default` for a caller to paint a checkmark. Clicking the
    /// entry itself dispatches `PALETTE_OPEN_ARTIFACT_WITH_VIEWER`/`_EDITOR` with `{pluginId,
    /// appId}` args. An empty `entries` list still returns the "Open with…" row with zero children —
    /// the caller decides whether to omit an empty menu.
    pub fn open_with_menu_item(entries: &[OpenWithEntry], is_de: bool) -> ContextMenuItemSpec {
        let mut children: Vec<ContextMenuItemSpec> = Vec::new();
        for role in [ChromeRole::Viewer, ChromeRole::Editor] {
            let group: Vec<&OpenWithEntry> = entries.iter().filter(|entry| entry.role == role).collect();
            if group.is_empty() {
                continue;
            }
            children.push(ContextMenuItemSpec { id: format!("menu.open-with.{}.header", role.as_str()), label: Some(role_title_chip_text(role, is_de).to_string()), separator: Some(true), ..Default::default() });
            for entry in group {
                children.push(open_with_entry_item(entry, is_de));
            }
        }
        ContextMenuItemSpec { id: "menu.open-with".into(), label: Some(open_with_label_text(is_de).to_string()), children: Some(children), ..Default::default() }
    }
    //#endregion 🔖️OpenWithMenu

    //#region 🔖️RoleFiltering
    /// 🚫️ Contract freeze §5: "viewer chrome hides every `Mutation`-kind action/utility" — drops
    /// every `ShellMenuAction` whose raw `kind` discriminant (`ActionKind`/`CommandKind`, see
    /// `ShellMenuAction`'s own doc) is `"Mutation"` for `ChromeRole::Viewer`; a no-op for
    /// `ChromeRole::Editor`.
    pub fn filter_shell_menu_actions_for_role(actions: &[ShellMenuAction], role: ChromeRole) -> Vec<ShellMenuAction> {
        if role == ChromeRole::Editor {
            return actions.to_vec();
        }
        actions.iter().filter(|action| action.kind != "Mutation").cloned().collect()
    }

    fn disable_utility(mut utility: UtilityNode) -> UtilityNode {
        match &mut utility {
            UtilityNode::Separator { disabled, .. } => *disabled = Some(true),
            UtilityNode::Button { disabled, .. } => *disabled = Some(true),
            UtilityNode::Toggle { disabled, .. } => *disabled = Some(true),
            UtilityNode::Collection { disabled, .. } => *disabled = Some(true),
        }
        utility
    }

    /// 🚫️ Contract freeze §5: "...and disables undo/redo" — forces `disabled: Some(true)` on every
    /// `UtilityCategory::History` utility (undo/redo/checkpoint/alternative, `VcsArtifactApp`'s own
    /// history vocabulary) for `ChromeRole::Viewer`; every other utility passes through unchanged.
    /// `UtilityNode` has no `Mutation` category to hide here — that vocabulary lives on
    /// `ShellMenuAction`/context-menu actions, see `filter_shell_menu_actions_for_role`.
    pub fn apply_role_to_utilities(utilities: Vec<UtilityNode>, role: ChromeRole) -> Vec<UtilityNode> {
        if role == ChromeRole::Editor {
            return utilities;
        }
        let mut out = Vec::with_capacity(utilities.len());
        for utility in utilities {
            if utility.category() == UtilityCategory::History {
                out.push(disable_utility(utility));
            } else {
                out.push(utility);
            }
        }
        out
    }
    //#endregion 🔖️RoleFiltering

    #[cfg(test)]
    include!("../../🧪️tests/🔬️targets-wgpu-component-role-chrome/🦀️.rs");
    // #endregion role_chrome
}


pub use semio_framework_icon_contract::IconName;

/// 📐️ The inverse of [`named_space_token`].
pub fn space_token_name(token: semio_framework_ui_contract::SpaceToken) -> &'static str {
    match token {
        semio_framework_ui_contract::SpaceToken::None => "none",
        semio_framework_ui_contract::SpaceToken::Xs => "xs",
        semio_framework_ui_contract::SpaceToken::Sm => "sm",
        semio_framework_ui_contract::SpaceToken::Md => "md",
        semio_framework_ui_contract::SpaceToken::Lg => "lg",
        semio_framework_ui_contract::SpaceToken::Xl => "xl",
        semio_framework_ui_contract::SpaceToken::Xxl => "xxl",
    }
}

