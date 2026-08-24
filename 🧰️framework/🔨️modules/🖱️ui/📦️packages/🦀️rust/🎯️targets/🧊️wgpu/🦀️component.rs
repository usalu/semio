//! 🧩 `component` engine module — extracted from wgpu `📦️glue.rs` (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE).

pub mod layout {
    // #region layout
    //! 📐️ Window layouts, panel tab constants, and engagement rails.

    use crate::wgpu::IconName;
    use dsl::DslValue;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use ui_contract::UiFixedList;

    //#region 🔖️Action
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ActionDescriptor {
        pub controller_id: String,
        pub action: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub args: Option<DslValue>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct StyleSpec {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub variant: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub size: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub density: Option<String>,
    }
    //#endregion 🔖️Action

    //#region 🔖️Presence
    /// 🧭️ The one shared, mandatory visual state every rendered UI element carries — orthogonal to
    /// `status` and to the `hover`/`selected` flags. `Hidden` short-circuits everything else: a hidden
    /// element is not rendered at all (no layout, no paint, no events) — renderers/reconcile must check
    /// this before doing anything with the rest of an element's `UiPresence`.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
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
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
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
    /// initials chip (contract-freeze §C7.6 of ticket `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/
    /// SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION`). `label` is the actor id
    /// itself (no display name is carried this far down the stack — see `PeerPresence`'s own doc
    /// comment in the plugin crate); a renderer that has the full roster may substitute a friendlier
    /// name, but every renderer must always carry SOME text alongside color (never color alone).
    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiPeerMark {
        pub actor: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub color: Option<u8>,
        #[serde(default, skip_serializing_if = "is_default")]
        pub hovered: bool,
        #[serde(default, skip_serializing_if = "is_default")]
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
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase", default)]
    pub struct UiPresence {
        #[serde(skip_serializing_if = "is_default")]
        pub state: UiState,
        #[serde(skip_serializing_if = "is_default")]
        pub status: UiStatus,
        #[serde(skip_serializing_if = "is_default")]
        pub hover: bool,
        #[serde(skip_serializing_if = "is_default")]
        pub selected: bool,
        /// 🎨️ This session's own hub-assigned palette index — stamped onto every `interaction_domain`-
        /// bound tree item by `ui_tree_stamp_presence`, `None` for a folder-only session with no hub.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub color: Option<u8>,
        /// 👥️ Every OTHER peer currently marking this element (hover and/or selection), sorted by
        /// actor.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiMenuRef {
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub args: Option<DslValue>,
    }

    /// 🖱️ One row of a resolved context menu — serde camelCase twin of TS `ContextMenuItemSpec`
    /// (`framework/core/js/index.ts`). Plugins build these with `MenuBuilder`; the host maps them
    /// through `ContextMenuController` (React) / `render_context_menu` (wgpu) unchanged.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ContextMenuItemSpec {
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub color: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub shortcut: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub disabled: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub separator: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub checked: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub destructive: Option<bool>,
        /// 🎯️ An action id, dispatched via the surface's already-scoped `dispatch(action, args)` — NOT
        /// an `ActionDescriptor` (no separate `controllerId`); matches the pre-existing TS
        /// `ContextMenuItemSpec.action` shape (`framework/core/js/index.ts`), which every emitting
        /// plugin already produces this way.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub action: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub args: Option<DslValue>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub hover_action: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub hover_args: Option<DslValue>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub children: Option<Vec<ContextMenuItemSpec>>,
    }

    //#region 🗂️ContextMenuOrganizer
    /// 🗂️ Canonical ribbon-parent taxonomy — Rust twin of ui-react's closed `UiRibbonParentCategory`
    /// union (`🧰️framework/🔨️modules/🖱️ui/⚛️react/⚡️implementations/🟦️typescript/📦️index.tsx` ~3317). Id
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
        item.id.starts_with("menu.group.")
    }

    // 🚫️async: E1 pure accessor consumed by sync-only std call sites (Option::map fn-value, sort_by_key comparator) — see R9
    fn context_menu_group_category(item: &ContextMenuItemSpec) -> &str {
        item.id.strip_prefix("menu.group.").unwrap_or(item.id.as_str())
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
                let index = bucket_mut(&mut bucketed_groups, format!("menu.group.{slug}"));
                bucketed_groups[index].children.get_or_insert_with(Vec::new).push(item);
                continue;
            }
            if primaries.len() < CONTEXT_MENU_PRIMARY_BUDGET {
                primaries.push(item);
                continue;
            }
            let category = category_of(item.action.as_deref().unwrap_or(item.id.as_str())).unwrap_or_else(|| "actions".into());
            let index = bucket_mut(&mut bucketed_groups, format!("menu.group.{category}"));
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
            out.push(ContextMenuItemSpec { id: "menu.group.more".into(), label: None, children: Some(folded_children), ..Default::default() });
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
        if interactive_count <= CONTEXT_MENU_ROW_BUDGET { context_menu_emit_within_budget(items) } else { context_menu_emit_over_budget(items, category_of) }
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
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ContextMenuHit {
        pub domain: String,
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub label: Option<String>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ContextMenuSelectionGroup {
        pub domain: String,
        pub ids: Vec<String>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ContextMenuTextContext {
        pub caret: usize,
        pub has_selection: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub word: Option<String>,
        pub can_rename: bool,
        pub has_completions: bool,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ContextMenuSurfaceTarget {
        pub surface_id: String,
        pub kind: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub hits: Vec<ContextMenuHit>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub selection: Vec<ContextMenuSelectionGroup>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub text: Option<ContextMenuTextContext>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
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
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ContextMenuRequest {
        pub menu: UiMenuRef,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub surface: Option<ContextMenuSurfaceTarget>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub window_instance_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub point: Option<ContextMenuPoint>,
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ContextMenuResponse {
        pub items: Vec<ContextMenuItemSpec>,
    }
    //#endregion 🔖️ContextMenu

    //#region 🔖️PanelTabConstants
    pub const FRAMEWORK_PANEL_TAB_ARTIFACT_ID: &str = "framework.panel.artifact";
    pub const FRAMEWORK_PANEL_TAB_CATALOGUE_ID: &str = "framework.panel.catalogue";
    pub const FRAMEWORK_PANEL_TAB_INSPECTION_ID: &str = "framework.panel.inspection";
    pub const FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL: &str = "Document";
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
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum WindowStackCorner {
        #[default]
        TopLeft,
        TopRight,
        BottomLeft,
        BottomRight,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WindowLayoutWindowNode {
        #[serde(default = "kind_window")]
        pub kind: String,
        pub window_kind_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub instance_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub template_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub corner: Option<WindowStackCorner>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WindowLayoutStackNode {
        #[serde(default = "kind_stack")]
        pub kind: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub size: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none", alias = "activeId")]
        pub active_window_kind_id: Option<String>,
        pub children: Vec<WindowLayoutWindowNode>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WindowLayoutAxisNode {
        pub kind: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub size: Option<f64>,
        pub children: Vec<WindowLayoutChild>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum WindowLayoutChild {
        Axis(WindowLayoutAxisNode),
        Stack(WindowLayoutStackNode),
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum WindowLayoutRoot {
        Axis(WindowLayoutAxisNode),
        Stack(WindowLayoutStackNode),
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WindowLayout {
        pub root: WindowLayoutRoot,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NamedLayout {
        pub id: String,
        pub label: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub icon_id: Option<IconName>,
        pub layout: WindowLayout,
        pub origin: String,
        #[serde(skip_serializing_if = "Option::is_none")]
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

    pub fn create_default_layout(window_ids: &[String], direction: &str, sizes: Option<&[f64]>, titles: Option<&[String]>) -> WindowLayout {
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
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct MeasureSelectItem {
        pub id: String,
        pub value: String,
        pub label: String,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
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
            ready: Option<f64>,
            /// 🌀️ When true, the measure tree leaf shows a loading ring while preload continues.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            loading: Option<bool>,
            /// 🌀️ When true, the measure tree leaf shows a dashed, slower waiting ring; `loading` takes precedence when both are set.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            waiting: Option<bool>,
            /// 🚫️ When true, the slider is inert — used when a parent weight is zero so joint percentages cannot change anything.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            disabled: Option<bool>,
            /// 🪣️ When set, this is a reveal-group id: the host must NOT dispatch `onChange` on every drag
            /// value — only on gesture commit (pointer-up) — and while dragging must locally cut off
            /// instances tagged with this reveal group's id instead. See `WorldInstancesLayer`'s reveal
            /// cutoff store and `revealCutoffs` in `World3dScene.interaction_json`.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            reveal: Option<String>,
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
            active_utility_id: Option<String>,
            /// 🎚️ Optional header slider — when set with `on_change`, the group row hosts a weight control (e.g. object-kind probability).
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            min: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            max: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            step: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            ready: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            loading: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            waiting: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
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

    /// @emoji 🎯️ Splits a window's top-level measures into `(general, utility_options)`.
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
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WindowEngagementOption {
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub icon_id: Option<IconName>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pressed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub disabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub action: Option<ActionDescriptor>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WindowEngagementInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub placeholder: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub disabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub on_change: Option<ActionDescriptor>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub on_submit: Option<ActionDescriptor>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub on_repeat_last: Option<ActionDescriptor>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub on_abort: Option<ActionDescriptor>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WindowEngagementStatus {
        pub id: String,
        pub text: String,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WindowEngagementPossible {
        pub id: String,
        pub label: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub detail: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub action: Option<ActionDescriptor>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WindowEngagementRingOption {
        pub id: String,
        pub label: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub disabled: Option<bool>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WindowEngagementToggleGroupOption {
        pub id: String,
        pub label: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub disabled: Option<bool>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WindowEngagementSelectItem {
        pub id: String,
        pub value: String,
        pub label: String,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
    pub enum WindowEngagementControl {
        Slider { id: Option<String>, label: Option<String>, value: f64, min: f64, max: f64, step: Option<f64>, unit: Option<String>, disabled: Option<bool>, on_change: Option<ActionDescriptor>, on_commit: Option<ActionDescriptor> },
        Stepper { id: Option<String>, label: Option<String>, value: f64, min: Option<f64>, max: Option<f64>, step: Option<f64>, unit: Option<String>, disabled: Option<bool>, on_change: Option<ActionDescriptor>, on_commit: Option<ActionDescriptor> },
        Ring { id: Option<String>, label: Option<String>, value: Option<String>, options: Vec<WindowEngagementRingOption>, disabled: Option<bool>, on_select: Option<ActionDescriptor> },
        ToggleGroup { id: Option<String>, label: Option<String>, value: Option<String>, options: Vec<WindowEngagementToggleGroupOption>, disabled: Option<bool>, on_select: Option<ActionDescriptor> },
        Select { id: Option<String>, label: Option<String>, value: Option<String>, placeholder: Option<String>, items: Vec<WindowEngagementSelectItem>, disabled: Option<bool>, on_change: Option<ActionDescriptor> },
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WindowEngagement {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub session_active: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub options: Option<Vec<WindowEngagementOption>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub input: Option<WindowEngagementInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub control: Option<WindowEngagementControl>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub controls: Option<Vec<WindowEngagementControl>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<Vec<WindowEngagementStatus>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub possible_engagements: Option<Vec<WindowEngagementPossible>>,
    }

    /// 🤝️ Closed replacement for `Option<WindowEngagement>` — makes "this window kind never engages" a
    /// named variant instead of `None`, so absence is an explicit, typed state rather than an implicit gap.
    /// ⚠️ `WindowEngagement` is a wide variant (nested `Vec`/`Option` fields), making `Some` far
    /// larger than `None` — boxing it would be a breaking public-API change (every construction/match
    /// site across ~30 plugins would need `Box::new`/deref updates), out of scope for a mechanical pass.
    #[allow(clippy::large_enum_variant, reason = "boxing is a breaking public API change, out of T1 scope")]
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase", tag = "kind", content = "value")]
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
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct WindowOptions {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub measures: Vec<WindowMeasure>,
        #[serde(default)]
        pub engagement: WindowEngagementSlot,
    }
    //#endregion 🔖️WindowEngagement

    //#region 🔖️WireFormatGoldenTests
    /** 🧊️ Golden wire-format tests: freeze exact JSON for layout/action/engagement types
    before these move into ui_wgpu, so the move can be proven byte-identical. */
    #[cfg(test)]
    mod layout_wire_format_tests {
        use super::*;

        const GOLDEN_ACTION_DESCRIPTOR_JSON: &str = "[{\"controllerId\":\"ctrl\",\"action\":\"doThing\",\"args\":42.0},{\"controllerId\":\"ctrl\",\"action\":\"doOther\"},{\"variant\":\"primary\",\"size\":\"md\"}]";

        #[semio_framework_async_macros::async_test]
        async fn action_descriptor_and_style_spec_serialize_to_golden_json() {
            let values = (
                ActionDescriptor { controller_id: "ctrl".into(), action: "doThing".into(), args: Some(DslValue::Number(42.0)) },
                ActionDescriptor { controller_id: "ctrl".into(), action: "doOther".into(), args: None },
                StyleSpec { variant: Some("primary".into()), size: Some("md".into()), density: None },
            );
            let json = serde_json::to_string(&values).unwrap();
            assert_eq!(json, GOLDEN_ACTION_DESCRIPTOR_JSON);
        }

        const GOLDEN_WINDOW_LAYOUT_JSON: &str = "{\"root\":{\"kind\":\"horizontal\",\"children\":[{\"kind\":\"stack\",\"size\":0.5,\"activeWindowKindId\":\"main\",\"children\":[{\"kind\":\"window\",\"windowKindId\":\"main\",\"title\":\"Main\"}]},{\"kind\":\"vertical\",\"size\":0.5,\"children\":[]}]}}";

        #[semio_framework_async_macros::async_test]
        async fn window_layout_serializes_to_golden_json() {
            let layout = WindowLayout {
                root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
                    kind: "horizontal".into(),
                    size: None,
                    children: vec![
                        WindowLayoutChild::Stack(WindowLayoutStackNode {
                            kind: "stack".into(),
                            size: Some(0.5),
                            active_window_kind_id: Some("main".into()),
                            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: "main".into(), title: Some("Main".into()), instance_id: None, template_id: None, corner: None }],
                        }),
                        WindowLayoutChild::Axis(WindowLayoutAxisNode { kind: "vertical".into(), size: Some(0.5), children: vec![] }),
                    ],
                }),
            };
            let json = serde_json::to_string(&layout).unwrap();
            assert_eq!(json, GOLDEN_WINDOW_LAYOUT_JSON);
            let roundtripped: WindowLayout = serde_json::from_str(&json).unwrap();
            assert_eq!(roundtripped, layout);
        }

        const GOLDEN_WINDOW_MEASURE_JSON: &str = "[{\"kind\":\"select\",\"id\":\"m1\",\"label\":\"Mode\",\"value\":\"a\",\"items\":[{\"id\":\"a\",\"value\":\"a\",\"label\":\"A\"}],\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"measureSelect\"}},{\"kind\":\"slider\",\"id\":\"m2\",\"label\":null,\"value\":1.0,\"min\":0.0,\"max\":2.0,\"step\":0.5,\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"measureSlider\"}},{\"kind\":\"toggle\",\"id\":\"m3\",\"iconId\":\"layout-grid\",\"label\":null,\"pressed\":true,\"text\":null,\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"measureToggle\"}},{\"kind\":\"group\",\"id\":\"m4\",\"label\":\"Group\",\"defaultOpen\":true,\"children\":[]}]";

        #[semio_framework_async_macros::async_test]
        async fn window_measure_serializes_to_golden_json() {
            let measures = vec![
                WindowMeasure::Select {
                    id: "m1".into(),
                    label: Some("Mode".into()),
                    value: "a".into(),
                    items: vec![MeasureSelectItem { id: "a".into(), value: "a".into(), label: "A".into() }],
                    on_change: ActionDescriptor { controller_id: "ctrl".into(), action: "measureSelect".into(), args: None },
                },
                WindowMeasure::Slider {
                    id: "m2".into(),
                    label: None,
                    value: 1.0,
                    min: 0.0,
                    max: 2.0,
                    step: Some(0.5),
                    ready: None,
                    loading: None,
                    waiting: None,
                    disabled: None,
                    reveal: None,
                    on_change: ActionDescriptor { controller_id: "ctrl".into(), action: "measureSlider".into(), args: None },
                },
                WindowMeasure::Toggle { id: "m3".into(), icon_id: IconName::LayoutGrid, label: None, pressed: true, text: None, on_change: ActionDescriptor { controller_id: "ctrl".into(), action: "measureToggle".into(), args: None } },
                WindowMeasure::Group {
                    id: "m4".into(),
                    label: "Group".into(),
                    default_open: Some(true),
                    active_utility_id: None,
                    value: None,
                    min: None,
                    max: None,
                    step: None,
                    ready: None,
                    loading: None,
                    waiting: None,
                    on_change: None,
                    children: vec![],
                },
            ];
            let json = serde_json::to_string(&measures).unwrap();
            assert_eq!(json, GOLDEN_WINDOW_MEASURE_JSON);
            let roundtripped: Vec<WindowMeasure> = serde_json::from_str(&json).unwrap();
            assert_eq!(roundtripped, measures);
        }

        fn utility_scoped_group(id: &str, utility: Option<&str>, children: Vec<WindowMeasure>) -> WindowMeasure {
            WindowMeasure::Group {
                id: id.into(),
                label: id.to_uppercase(),
                default_open: None,
                active_utility_id: utility.map(str::to_string),
                children,
                value: None,
                min: None,
                max: None,
                step: None,
                ready: None,
                loading: None,
                waiting: None,
                on_change: None,
            }
        }

        fn measure_toggle(id: &str) -> WindowMeasure {
            WindowMeasure::Toggle { id: id.into(), icon_id: IconName::PanelLeft, label: Some(id.into()), pressed: true, text: None, on_change: ActionDescriptor { controller_id: "c".into(), action: "t".into(), args: None } }
        }

        #[semio_framework_async_macros::async_test]
        async fn partition_window_measures_unwraps_matching_utility_group_children_into_utility_options() {
            let measures = vec![utility_scoped_group("brush-params", Some("brush"), vec![measure_toggle("size")])];
            let partition = partition_window_measures(&measures, Some("brush")).expect("fixed partition");
            assert!(partition.general.is_empty());
            assert_eq!(partition.utility_options.len(), 1);
            assert!(matches!(partition.utility_options.get(0).copied(), Some(WindowMeasure::Toggle { id, .. }) if id == "size"), "tagged wrapper is routing-only — children render flat");
        }

        #[semio_framework_async_macros::async_test]
        async fn partition_window_measures_drops_non_matching_utility_group_from_both_buckets() {
            let measures = vec![utility_scoped_group("brush-params", Some("brush"), vec![measure_toggle("size")])];
            let other = partition_window_measures(&measures, Some("fill")).expect("fixed partition");
            assert!(other.general.is_empty() && other.utility_options.is_empty(), "wrong active utility drops the group entirely");
            let none = partition_window_measures(&measures, None).expect("fixed partition");
            assert!(none.general.is_empty() && none.utility_options.is_empty(), "no active utility drops the group entirely");
        }

        #[semio_framework_async_macros::async_test]
        async fn partition_window_measures_keeps_untagged_group_and_non_group_in_general() {
            let measures = vec![
                utility_scoped_group("grid", None, vec![]),
                WindowMeasure::Slider {
                    id: "zoom".into(),
                    label: None,
                    value: 1.0,
                    min: 0.0,
                    max: 2.0,
                    step: None,
                    ready: None,
                    loading: None,
                    waiting: None,
                    disabled: None,
                    reveal: None,
                    on_change: ActionDescriptor { controller_id: "c".into(), action: "z".into(), args: None },
                },
            ];
            let partition = partition_window_measures(&measures, Some("brush")).expect("fixed partition");
            assert_eq!(partition.general.len(), 2, "untagged group and slider both stay general");
            assert!(partition.utility_options.is_empty());
        }

        #[semio_framework_async_macros::async_test]
        async fn partition_window_measures_empty_input_roundtrips_to_empty() {
            let partition = partition_window_measures(&[], Some("brush")).expect("fixed partition");
            assert!(partition.general.is_empty() && partition.utility_options.is_empty());
        }

        #[semio_framework_async_macros::async_test]
        async fn partition_window_measures_max_plus_one_returns_exact_borrowed_owner() {
            let measures = (0..=WINDOW_MEASURE_PARTITION_CAPACITY).map(|index| measure_toggle(&format!("measure-{index}"))).collect::<Vec<_>>();
            let rejected = partition_window_measures(&measures, None).expect_err("maximum plus one must refuse");
            assert!(std::ptr::eq(rejected, &measures[WINDOW_MEASURE_PARTITION_CAPACITY]));
        }

        const GOLDEN_WINDOW_ENGAGEMENT_JSON: &str = "{\"sessionActive\":true,\"options\":[{\"id\":\"opt1\",\"label\":\"Option\",\"pressed\":false}],\"input\":{\"id\":\"in1\",\"value\":\"v\"},\"control\":{\"kind\":\"slider\",\"id\":\"sl1\",\"label\":null,\"value\":1.0,\"min\":0.0,\"max\":2.0,\"step\":null,\"unit\":null,\"disabled\":null,\"onChange\":null,\"onCommit\":null},\"status\":[{\"id\":\"st1\",\"text\":\"Ready\"}],\"possibleEngagements\":[{\"id\":\"pe1\",\"label\":\"Possible\"}]}";

        #[semio_framework_async_macros::async_test]
        async fn window_engagement_serializes_to_golden_json() {
            let engagement = WindowEngagement {
                session_active: Some(true),
                options: Some(vec![WindowEngagementOption { id: "opt1".into(), label: Some("Option".into()), icon_id: None, pressed: Some(false), disabled: None, action: None }]),
                input: Some(WindowEngagementInput { id: Some("in1".into()), value: Some("v".into()), placeholder: None, disabled: None, on_change: None, on_submit: None, on_repeat_last: None, on_abort: None }),
                control: Some(WindowEngagementControl::Slider { id: Some("sl1".into()), label: None, value: 1.0, min: 0.0, max: 2.0, step: None, unit: None, disabled: None, on_change: None, on_commit: None }),
                controls: None,
                status: Some(vec![WindowEngagementStatus { id: "st1".into(), text: "Ready".into() }]),
                possible_engagements: Some(vec![WindowEngagementPossible { id: "pe1".into(), label: "Possible".into(), detail: None, action: None }]),
            };
            let json = serde_json::to_string(&engagement).unwrap();
            assert_eq!(json, GOLDEN_WINDOW_ENGAGEMENT_JSON);
            let roundtripped: WindowEngagement = serde_json::from_str(&json).unwrap();
            assert_eq!(roundtripped, engagement);
        }
    }
    //#endregion 🔖️WireFormatGoldenTests
    // #endregion layout
}

pub mod utilities {
    // #region utilities
    //! 🧰️ Declarative per-mode utility bar utility trees.

    use super::layout::ActionDescriptor;
    use crate::wgpu::IconName;
    use dsl::DslValue;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum UtilityCategory {
        Selection,
        Utilities,
        History,
        Sync,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
    pub enum UtilityNode {
        Separator {
            id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            order: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            disabled: Option<bool>,
        },
        Button {
            id: String,
            icon_id: IconName,
            #[serde(skip_serializing_if = "Option::is_none")]
            label: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            order: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            disabled: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            category: Option<UtilityCategory>,
            on_press: ActionDescriptor,
        },
        Toggle {
            id: String,
            icon_id: IconName,
            #[serde(skip_serializing_if = "Option::is_none")]
            label: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            order: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pressed: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            disabled: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            category: Option<UtilityCategory>,
            on_change: ActionDescriptor,
        },
        Collection {
            id: String,
            icon_id: IconName,
            #[serde(skip_serializing_if = "Option::is_none")]
            label: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            order: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            disabled: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
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
    /// @emoji 🧰️ A resolved utility ready to be laid out into the utility bar. `framework_core` maps its
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

    /// @emoji 🧰 Derives the utility bar `UtilityNode` tree from resolved utilities and the host-owned active utility id.
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
    mod utility_node_wire_format_tests {
        use super::super::layout::ActionDescriptor;
        use super::*;

        const GOLDEN_UTILITY_NODE_JSON: &str = "[{\"kind\":\"separator\",\"id\":\"sep1\",\"order\":1},{\"kind\":\"button\",\"id\":\"btn1\",\"iconId\":\"wrench\",\"label\":\"Utility\",\"title\":\"Utility\",\"category\":\"history\",\"onPress\":{\"controllerId\":\"ctrl\",\"action\":\"runUtility\"}},{\"kind\":\"toggle\",\"id\":\"tog1\",\"iconId\":\"panel-left\",\"label\":\"Toggle\",\"title\":\"Toggle\",\"pressed\":true,\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"toggleUtility\"}},{\"kind\":\"collection\",\"id\":\"col1\",\"iconId\":\"folder\",\"label\":\"Group\",\"title\":\"Group\",\"children\":[{\"kind\":\"separator\",\"id\":\"sep2\"}]}]";

        #[semio_framework_async_macros::async_test]
        async fn utility_node_serializes_to_golden_json() {
            let nodes = vec![
                UtilityNode::Separator { id: "sep1".into(), order: Some(1), disabled: None },
                utility_button("btn1", IconName::Wrench, "Utility", ActionDescriptor { controller_id: "ctrl".into(), action: "runUtility".into(), args: None }).with_category(UtilityCategory::History),
                utility_toggle("tog1", IconName::PanelLeft, "Toggle", true, ActionDescriptor { controller_id: "ctrl".into(), action: "toggleUtility".into(), args: None }),
                utility_collection("col1", IconName::Folder, "Group", vec![utility_separator("sep2")]),
            ];
            let json = serde_json::to_string(&nodes).unwrap();
            assert_eq!(json, GOLDEN_UTILITY_NODE_JSON);
            let roundtripped: Vec<UtilityNode> = serde_json::from_str(&json).unwrap();
            assert_eq!(roundtripped, nodes);
        }

        fn spec(id: &str, group: Option<&str>) -> DerivedUtilitySpec {
            DerivedUtilitySpec { id: id.into(), label: id.to_uppercase(), icon_id: IconName::CircleDot, group: group.map(str::to_string), category: None }
        }

        #[semio_framework_async_macros::async_test]
        async fn derive_utility_nodes_marks_the_active_utility_pressed() {
            let nodes = derive_utility_nodes("ctrl", &[spec("select", None), spec("brush", None)], Some("brush"));
            assert_eq!(nodes.len(), 2);
            match &nodes[0] {
                UtilityNode::Toggle { id, pressed, on_change, .. } => {
                    assert_eq!(id, "select");
                    assert_eq!(*pressed, Some(false));
                    assert_eq!(on_change.action, "setActiveUtility");
                    assert_eq!(on_change.args, Some(DslValue::Object(vec![("utilityId".into(), DslValue::String("select".into()))])));
                }
                other => panic!("expected toggle, got {other:?}"),
            }
            match &nodes[1] {
                UtilityNode::Toggle { id, pressed, .. } => {
                    assert_eq!(id, "brush");
                    assert_eq!(*pressed, Some(true));
                }
                other => panic!("expected toggle, got {other:?}"),
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn derive_utility_nodes_groups_shared_group_into_one_collection() {
            let nodes = derive_utility_nodes("ctrl", &[spec("select", None), spec("line", Some("shapes")), spec("rect", Some("shapes"))], None);
            assert_eq!(nodes.len(), 2, "one ungrouped toggle + one shapes collection");
            assert!(matches!(&nodes[0], UtilityNode::Toggle { id, .. } if id == "select"));
            match &nodes[1] {
                UtilityNode::Collection { id, children, .. } => {
                    assert_eq!(id, "group:shapes");
                    assert_eq!(children.len(), 2);
                    assert!(matches!(&children[0], UtilityNode::Toggle { id, .. } if id == "line"));
                    assert!(matches!(&children[1], UtilityNode::Toggle { id, .. } if id == "rect"));
                }
                other => panic!("expected collection, got {other:?}"),
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn derive_utility_nodes_hoists_single_child_groups() {
            let nodes = derive_utility_nodes("ctrl", &[spec("transform", Some("transform")), spec("brush", None)], Some("transform"));
            assert_eq!(nodes.len(), 2, "lone group child is hoisted — no nested Transform/Transform pair");
            match &nodes[0] {
                UtilityNode::Toggle { id, pressed, .. } => {
                    assert_eq!(id, "transform");
                    assert_eq!(*pressed, Some(true));
                }
                other => panic!("expected hoisted toggle, got {other:?}"),
            }
            assert!(matches!(&nodes[1], UtilityNode::Toggle { id, .. } if id == "brush"));
        }
    }
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
    //! `🎠️kernel/🟦️component.ts` — same boundary this file already draws around `PluginCatalog`-style
    //! product data. A concrete `AppRouter`/`ConfigStore` never appears here: callers (the renderer
    //! product) resolve real entries and hand this module only already-resolved data to render.

    use super::layout::{ContextMenuItemSpec, ShellMenuAction};
    use super::utilities::{UtilityCategory, UtilityNode};
    use dsl::DslValue;
    use serde::{Deserialize, Serialize};

    //#region 🔖️ChromeRole
    /// 👁️✏️ Wire-compatible with Rust `semio_framework::AppRole`/TS `AppRole` — exactly `"viewer"`/
    /// `"editor"`, contract freeze §1 C1.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
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
        if is_de { "Öffnen mit…" } else { "Open with…" }
    }

    /// 🗣️ "Set as default" toggle — contract freeze §5: en `"Set as default"` / de `"Als Standard
    /// festlegen"`.
    pub fn set_as_default_label_text(is_de: bool) -> &'static str {
        if is_de { "Als Standard festlegen" } else { "Set as default" }
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
    mod role_chrome_tests {
        use super::super::layout::ActionDescriptor;
        use super::super::utilities::{utility_button, utility_toggle};
        use super::*;
        use crate::wgpu::IconName;

        #[semio_framework_async_macros::async_test]
        async fn from_boot_env_accepts_viewer_and_falls_back_to_editor() {
            assert_eq!(ChromeRole::from_boot_env(Some("viewer")), ChromeRole::Viewer);
            assert_eq!(ChromeRole::from_boot_env(Some("editor")), ChromeRole::Editor);
            assert_eq!(ChromeRole::from_boot_env(Some("bogus")), ChromeRole::Editor);
            assert_eq!(ChromeRole::from_boot_env(Some("")), ChromeRole::Editor);
            assert_eq!(ChromeRole::from_boot_env(None), ChromeRole::Editor);
        }

        #[semio_framework_async_macros::async_test]
        async fn title_chip_text_covers_both_roles_in_both_locales() {
            assert_eq!(role_title_chip_text(ChromeRole::Viewer, false), "Viewer");
            assert_eq!(role_title_chip_text(ChromeRole::Viewer, true), "Betrachter");
            assert_eq!(role_title_chip_text(ChromeRole::Editor, false), "Editor");
            assert_eq!(role_title_chip_text(ChromeRole::Editor, true), "Editor");
        }

        fn entry(plugin_id: &str, app_id: &str, role: ChromeRole, is_default: bool) -> OpenWithEntry {
            OpenWithEntry { plugin_id: plugin_id.into(), app_id: app_id.into(), label: app_id.into(), role, is_default }
        }

        #[semio_framework_async_macros::async_test]
        async fn open_with_menu_item_groups_by_role_viewer_first_then_editor() {
            let entries = vec![entry("norm", "s.cad.cad@1/*#editor", ChromeRole::Editor, false), entry("cad", "s.cad.cad@1/*#viewer", ChromeRole::Viewer, true)];
            let menu = open_with_menu_item(&entries, false);
            assert_eq!(menu.id, "menu.open-with");
            assert_eq!(menu.label.as_deref(), Some("Open with…"));
            let children = menu.children.expect("submenu children");
            assert_eq!(children.len(), 4, "viewer header + viewer entry + editor header + editor entry");
            assert_eq!(children[0].label.as_deref(), Some("Viewer"));
            assert_eq!(children[0].separator, Some(true));
            assert_eq!(children[1].id, "menu.open-with.cad.s.cad.cad@1/*#viewer");
            assert_eq!(children[2].label.as_deref(), Some("Editor"));
            assert_eq!(children[3].id, "menu.open-with.norm.s.cad.cad@1/*#editor");
        }

        #[semio_framework_async_macros::async_test]
        async fn open_with_menu_item_toggle_sets_when_not_default_and_clears_when_default() {
            let entries = vec![entry("cad", "editor", ChromeRole::Editor, false), entry("norm", "editor-alt", ChromeRole::Editor, true)];
            let menu = open_with_menu_item(&entries, false);
            let editor_entries: Vec<_> = menu.children.unwrap().into_iter().filter(|item| item.separator != Some(true)).collect();
            let not_default_toggle = editor_entries[0].children.as_ref().unwrap()[0].clone();
            assert_eq!(not_default_toggle.action.as_deref(), Some(OS_SET_DEFAULT_EDITOR));
            assert_eq!(not_default_toggle.checked, Some(false));
            let already_default_toggle = editor_entries[1].children.as_ref().unwrap()[0].clone();
            assert_eq!(already_default_toggle.action.as_deref(), Some(OS_CLEAR_DEFAULT_APP));
            assert_eq!(already_default_toggle.checked, Some(true));
        }

        #[semio_framework_async_macros::async_test]
        async fn open_with_menu_item_localizes_headers_and_label_to_german() {
            let entries = vec![entry("cad", "viewer", ChromeRole::Viewer, false)];
            let menu = open_with_menu_item(&entries, true);
            assert_eq!(menu.label.as_deref(), Some("Öffnen mit…"));
            assert_eq!(menu.children.unwrap()[1].children.as_ref().unwrap()[0].label.as_deref(), Some("Als Standard festlegen"));
        }

        fn shell_action(id: &str, kind: &str) -> ShellMenuAction {
            ShellMenuAction { id: id.into(), label: id.into(), icon: None, keys: None, kind: kind.into(), category: None, in_palette: true, arg_carrying: false }
        }

        #[semio_framework_async_macros::async_test]
        async fn filter_shell_menu_actions_drops_mutation_kind_for_viewer_only() {
            let actions = vec![shell_action("shell.rename", "Mutation"), shell_action("shell.zoomIn", "View")];
            let viewer = filter_shell_menu_actions_for_role(&actions, ChromeRole::Viewer);
            assert_eq!(viewer.iter().map(|action| action.id.as_str()).collect::<Vec<_>>(), vec!["shell.zoomIn"]);
            let editor = filter_shell_menu_actions_for_role(&actions, ChromeRole::Editor);
            assert_eq!(editor.len(), 2, "editor chrome keeps every action");
        }

        #[semio_framework_async_macros::async_test]
        async fn apply_role_to_utilities_disables_history_only_for_viewer() {
            let press = ActionDescriptor { controller_id: "history".into(), action: "undo".into(), args: None };
            let toggle = ActionDescriptor { controller_id: "select".into(), action: "toggleSelect".into(), args: None };
            let utilities = vec![utility_button("undo", IconName::RotateCcw, "Undo", press).with_category(UtilityCategory::History), utility_toggle("select", IconName::MousePointer, "Select", false, toggle)];
            let viewer = apply_role_to_utilities(utilities.clone(), ChromeRole::Viewer);
            assert_eq!(viewer[0].category(), UtilityCategory::History);
            assert!(matches!(&viewer[0], UtilityNode::Button { disabled: Some(true), .. }), "undo must be disabled for a viewer");
            assert!(matches!(&viewer[1], UtilityNode::Toggle { disabled: None, .. }), "non-history utilities are untouched");
            let editor = apply_role_to_utilities(utilities, ChromeRole::Editor);
            assert!(matches!(&editor[0], UtilityNode::Button { disabled: None, .. }), "editor chrome never disables history utilities");
        }
    }
    // #endregion role_chrome
}

pub mod ui {
    // #region ui
    //! 🧩 Declarative UI graph types shared by kernel, plugins, and renderers.

    use crate::wgpu::IconName;
    use crate::wgpu::Label;
    use dsl::DslValue;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    //#region 🔖Action
    pub use super::layout::{ActionDescriptor, StyleSpec, UiPeerMark, UiPresence, UiState, UiStatus};
    pub use super::layout::{ContextMenuHit, ContextMenuItemSpec, ContextMenuPoint, ContextMenuRequest, ContextMenuResponse, ContextMenuSelectionGroup, ContextMenuSurfaceTarget, ContextMenuTextContext, UiMenuRef};
    pub use super::layout::{RIBBON_PARENT_CATEGORIES, ShellMenuAction, build_shell_context_menu_specs, organize_context_menu, ribbon_parent_label};
    //#endregion 🔖Action

    //#region 🔖Primitives
    // 🚧 NOT typegen-derived: `children: Vec<UiNode>` makes this recursive through `UiNode`, which isn't
    // itself typegen-derived yet (blocked on the `ComponentScene` scene family — see 🔖️Manifest in
    // framework/core/rs/lib.rs). Hand-mirrored in framework/core/js/index.ts until that lands.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiStackNode {
        pub direction: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub gap: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub padding: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub activate: Option<ActionDescriptor>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub drop_action: Option<ActionDescriptor>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub drop_overlay: Option<UiDropOverlaySpec>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
        pub children: Vec<UiNode>,
    }

    /// 📥️ Hover-state copy for a `UiStackNode`'s `drop_overlay`: shown while a drag is over the stack, ahead of `drop_action` firing on release.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiDropOverlaySpec {
        pub title: Label,
        pub hint: Label,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub accept: Option<String>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiTextNode {
        pub value: Label,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub emphasize: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data_attributes: Option<HashMap<String, String>>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiButtonNode {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        pub icon_id: IconName,
        pub label: Label,
        pub action: ActionDescriptor,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub style: Option<StyleSpec>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiSeparatorNode {
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiImageNode {
        pub id: String,
        pub src: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub alt: Option<Label>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiInputNode {
        pub id: String,
        pub input_kind: String,
        pub value: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub placeholder: Option<Label>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub commit: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub step: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub accept: Option<String>,
        pub on_change: ActionDescriptor,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiSelectItem {
        pub value: String,
        pub label: Label,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiSelectNode {
        pub id: String,
        pub value: String,
        pub items: Vec<UiSelectItem>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub placeholder: Option<Label>,
        pub on_change: ActionDescriptor,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiToggleNode {
        pub id: String,
        pub icon_id: IconName,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub text: Option<Label>,
        pub on_change: ActionDescriptor,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    /** @emoji 🌿️ A nestable labeled container of `UiNode` children — the declarative-tree mechanism for
     * subtrees like `Origin > X/Y/Z`: `ui_declarative_child_to_tree_item` expands a `Group` into a
     * `UiTreeItemNode` whose `items` are its recursively-converted children, so depth composes to any
     * level (`Plane > Origin > X/Y/Z`). Unlike `UiSectionNode` (top-level tree sections only, see
     * `assertNoNestedTreeSections` on the TS side), a `Group` may itself appear as another `Group`'s or
     * `UiFieldNode`'s child. */
    // 🚧️ NOT typegen-derived: `children: Vec<UiNode>` is recursive through `UiNode` (see `UiStackNode`'s
    // doc comment on this same gap).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiGroupNode {
        pub id: String,
        pub label: Label,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub default_open: Option<bool>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
        pub children: Vec<UiNode>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiKeyValueEntry {
        pub label: Label,
        pub value: String,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiKeyValueNode {
        pub entries: Vec<UiKeyValueEntry>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiSliderNode {
        pub id: String,
        pub value: f64,
        pub min: f64,
        pub max: f64,
        pub step: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
        pub on_change: ActionDescriptor,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiNumberStepperNode {
        pub id: String,
        pub value: f64,
        pub step: f64,
        pub uniform: bool,
        pub on_absolute: ActionDescriptor,
        pub on_delta: ActionDescriptor,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiRingNode {
        pub id: String,
        pub orb_id: String,
        pub t: f64,
        pub on_change: ActionDescriptor,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiIconSelectNode {
        pub id: String,
        pub value: String,
        pub uniform: bool,
        pub classifier_kind: String,
        pub on_change: ActionDescriptor,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "type", rename_all = "camelCase")]
    pub enum UiControlNode {
        Input(UiInputNode),
        Select(UiSelectNode),
        Toggle(UiToggleNode),
        Button(UiButtonNode),
        KeyValue(UiKeyValueNode),
        Slider(UiSliderNode),
        NumberStepper(UiNumberStepperNode),
        Ring(UiRingNode),
        IconSelect(UiIconSelectNode),
    }

    impl UiControlNode {
        /// 🧭️ Exhaustive accessor — a new control variant fails to compile here until wired.
        /// `&UiPresence` (ticket 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-
        /// CREATION C7.6): `UiPresence` gained `peers: Vec<UiPeerMark>` and lost `Copy`.
        pub fn presence(&self) -> &UiPresence {
            match self {
                UiControlNode::Input(n) => &n.presence,
                UiControlNode::Select(n) => &n.presence,
                UiControlNode::Toggle(n) => &n.presence,
                UiControlNode::Button(n) => &n.presence,
                UiControlNode::KeyValue(n) => &n.presence,
                UiControlNode::Slider(n) => &n.presence,
                UiControlNode::NumberStepper(n) => &n.presence,
                UiControlNode::Ring(n) => &n.presence,
                UiControlNode::IconSelect(n) => &n.presence,
            }
        }
        pub fn presence_mut(&mut self) -> &mut UiPresence {
            match self {
                UiControlNode::Input(n) => &mut n.presence,
                UiControlNode::Select(n) => &mut n.presence,
                UiControlNode::Toggle(n) => &mut n.presence,
                UiControlNode::Button(n) => &mut n.presence,
                UiControlNode::KeyValue(n) => &mut n.presence,
                UiControlNode::Slider(n) => &mut n.presence,
                UiControlNode::NumberStepper(n) => &mut n.presence,
                UiControlNode::Ring(n) => &mut n.presence,
                UiControlNode::IconSelect(n) => &mut n.presence,
            }
        }
    }

    // 🚧️ NOT typegen-derived: `child: Box<UiNode>` is recursive through `UiNode` (see `UiStackNode`'s
    // doc comment on this same gap).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiFieldNode {
        pub id: String,
        pub label: Label,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub required: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub error: Option<String>,
        pub child: Box<UiNode>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    // 🚧️ NOT typegen-derived: `children: Vec<UiNode>` is recursive through `UiNode` (see `UiStackNode`'s
    // doc comment on this same gap).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiSectionNode {
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", alias = "title")]
        pub label: Option<Label>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub default_open: Option<bool>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
        pub children: Vec<UiNode>,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub enum UiTreeActionPlacement {
        #[default]
        Row,
        Menu,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiTreeItemAction {
        pub icon_id: IconName,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub label: Option<Label>,
        pub action: ActionDescriptor,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub placement: Option<UiTreeActionPlacement>,
    }

    impl UiTreeItemAction {
        /** @emoji 📍️ Row actions paint on the tree header; menu actions belong in the row context menu. */
        pub fn placement(&self) -> UiTreeActionPlacement {
            self.placement.clone().unwrap_or_default()
        }
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiTreeItemNode {
        pub id: String,
        pub label: Label,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", alias = "icon")]
        pub icon_id: Option<IconName>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(skip_serializing_if = "Option::is_none", alias = "expanded")]
        pub default_open: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub action: Option<ActionDescriptor>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub actions: Option<Vec<UiTreeItemAction>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub draggable: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub drag_data: Option<HashMap<String, String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub items: Option<Vec<UiTreeItemNode>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub control: Option<UiControlNode>,
        /// 👁️ Domain "eye toggle" flag: the row stays visible, dimmed, and clickable (to un-hide) —
        /// this is NOT `presence.state == Hidden`, which means not rendered at all.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub dimmed: Option<bool>,
        /// 🖱️ Row-level context-menu address — most rows share one `menu.id` across a tree with the row
        /// id carried in `args` (e.g. `{"id": row.id}`), rather than minting a unique menu id per row.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    impl UiTreeItemNode {
        /** @emoji 🌳️ Builds a tree item with optional extensions unset. */
        pub fn base(id: impl Into<String>, label: impl Into<Label>) -> Self {
            Self {
                id: id.into(),
                label: label.into(),
                description: None,
                icon_id: None,
                presence: UiPresence::default(),
                default_open: None,
                action: None,
                actions: None,
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                dimmed: None,
                menu: None,
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiTreeSectionNode {
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub label: Option<Label>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub default_open: Option<bool>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        pub items: Vec<UiTreeItemNode>,
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiTreeNode {
        pub sections: Vec<UiTreeSectionNode>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub drop_action: Option<ActionDescriptor>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
        /// 🕹️ Binds this rendered tree to an app-declared `InteractionDefinition` domain — the framework
        /// (not the app) then owns the domain's selection/hover via `interactionSelect`/`interactionHover`,
        /// stamped back onto item `presence` by `ui_tree_stamp_presence`. Replaces the deleted per-app
        /// `selected_ids`/`highlighted_ids`/`selection_change` wire surface.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub interaction_domain: Option<String>,
    }

    /// 🖌️ Stamps `selected`/`previewed`/`color`/`peers` per-item presence across every item in every
    /// section of a tree — the framework-side counterpart of a `UiTreeNode.interaction_domain`
    /// binding. `previewed` wins visually over a plain `selected` item only insofar as both are
    /// representable simultaneously (an item can be selected AND previewed). `own_color` (ticket
    /// 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C7.6) is stamped onto
    /// every item unconditionally — it names the color THIS session renders its own hover/selection
    /// as, whether or not this particular item is currently marked; `peer_marks_for` resolves an
    /// item's own peer roster by id (called once per item, not pre-collected, since the caller's
    /// `InteractionView::peers_selecting`/`peers_hovering` are themselves per-id lookups).
    pub fn ui_tree_stamp_presence(sections: &mut [UiTreeSectionNode], selected: &std::collections::HashSet<String>, previewed: &std::collections::HashSet<String>, own_color: Option<u8>, peer_marks_for: &dyn Fn(&str) -> Vec<UiPeerMark>) {
        fn stamp_items(items: &mut [UiTreeItemNode], selected: &std::collections::HashSet<String>, previewed: &std::collections::HashSet<String>, own_color: Option<u8>, peer_marks_for: &dyn Fn(&str) -> Vec<UiPeerMark>) {
            for item in items {
                item.presence.selected = selected.contains(&item.id);
                if previewed.contains(&item.id) {
                    item.presence.state = UiState::Previewed;
                }
                item.presence.color = own_color;
                item.presence.peers = peer_marks_for(&item.id);
                if let Some(children) = &mut item.items {
                    stamp_items(children, selected, previewed, own_color, peer_marks_for);
                }
            }
        }
        for section in sections {
            stamp_items(&mut section.items, selected, previewed, own_color, peer_marks_for);
        }
    }

    // 🚧️ NOT typegen-derived: `fields: Vec<UiNode>` is recursive through `UiNode` (see `UiStackNode`'s
    // doc comment on this same gap).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiInspectorFieldGroup {
        pub id: String,
        pub label: Label,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub default_open: Option<bool>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        pub fields: Vec<UiNode>,
    }

    pub const UI_INSPECTOR_MIXED_PLACEHOLDER: &str = "Mixed";
    //#endregion 🔖️Primitives

    //#region 🔖️InspectorHelpers
    pub fn ui_inspector_all_equal<T: PartialEq>(values: &[T]) -> bool {
        if values.len() <= 1 {
            return true;
        }
        values.windows(2).all(|pair| pair[0] == pair[1])
    }

    pub struct UiInspectorMixedText {
        pub value: String,
        pub placeholder: Option<String>,
    }

    pub fn ui_inspector_mixed_text(values: &[String]) -> UiInspectorMixedText {
        let uniform = ui_inspector_all_equal(values);
        UiInspectorMixedText { value: if uniform { values.first().cloned().unwrap_or_default() } else { String::new() }, placeholder: if uniform { None } else { Some(UI_INSPECTOR_MIXED_PLACEHOLDER.into()) } }
    }

    pub struct UiInspectorMixedNumber {
        pub value: f64,
        pub uniform: bool,
    }

    pub fn ui_inspector_mixed_number(values: &[f64]) -> UiInspectorMixedNumber {
        let uniform = ui_inspector_all_equal(values);
        UiInspectorMixedNumber { value: if uniform { *values.first().unwrap_or(&0.0) } else { f64::NAN }, uniform }
    }

    pub fn ui_inspector_mixed_select(values: &[String]) -> UiInspectorMixedText {
        ui_inspector_mixed_text(values)
    }

    pub struct UiInspectorMixedToggle {
        pub pressed: bool,
        pub uniform: bool,
    }

    pub fn ui_inspector_mixed_toggle(values: &[bool]) -> UiInspectorMixedToggle {
        let uniform = ui_inspector_all_equal(values);
        UiInspectorMixedToggle { pressed: uniform && values.first().copied().unwrap_or(false), uniform }
    }

    pub fn ui_inspector_mixed_slider(values: &[f64]) -> UiInspectorMixedNumber {
        ui_inspector_mixed_number(values)
    }

    pub fn ui_inspector_readonly_field(id: impl Into<String>, label: impl Into<Label>, value: impl Into<String>) -> UiNode {
        let id = id.into();
        UiNode::Field(UiFieldNode {
            menu: None,
            id: id.clone(),
            label: label.into(),
            child: Box::new(UiNode::Input(UiInputNode {
                menu: None,
                id,
                input_kind: "text".into(),
                value: value.into(),
                placeholder: None,
                commit: None,
                on_change: ActionDescriptor { controller_id: String::new(), action: String::new(), args: None },
                min: None,
                max: None,
                step: None,
                accept: None,
                presence: UiPresence::default(),
            })),
            description: None,
            required: None,
            error: None,
            presence: UiPresence::default(),
        })
    }

    /** @emoji 🔢️ Builds an editable number-stepper field row, computing the mixed/uniform display from
     * `values` via {@link ui_inspector_mixed_number}. `action` is cloned into both `onAbsolute` (typed
     * entry, dispatched with `{value}` merged into `args`) and `onDelta` (nudge buttons, `{delta}`) —
     * callers' patch handlers branch on whichever key the dispatched action actually carries. */
    pub fn ui_inspector_stepper_field(id: impl Into<String>, label: impl Into<Label>, values: &[f64], step: f64, action: ActionDescriptor) -> UiNode {
        let id = id.into();
        let mixed = ui_inspector_mixed_number(values);
        UiNode::Field(UiFieldNode {
            menu: None,
            id: id.clone(),
            label: label.into(),
            child: Box::new(UiNode::NumberStepper(UiNumberStepperNode { menu: None, id, value: mixed.value, step, uniform: mixed.uniform, on_absolute: action.clone(), on_delta: action, presence: UiPresence::default() })),
            description: None,
            required: None,
            error: None,
            presence: UiPresence::default(),
        })
    }

    /** @emoji 🔘️ Builds an editable boolean toggle field row, computing the mixed/uniform display from
     * `values` via {@link ui_inspector_mixed_toggle}. */
    pub fn ui_inspector_toggle_field(id: impl Into<String>, label: impl Into<Label>, icon_id: impl Into<IconName>, values: &[bool], action: ActionDescriptor) -> UiNode {
        let id = id.into();
        let mixed = ui_inspector_mixed_toggle(values);
        UiNode::Field(UiFieldNode {
            menu: None,
            id: id.clone(),
            label: label.into(),
            child: Box::new(UiNode::Toggle(UiToggleNode { menu: None, id, icon_id: icon_id.into(), text: None, on_change: action, presence: UiPresence::selected(mixed.pressed) })),
            description: None,
            required: None,
            error: None,
            presence: UiPresence::default(),
        })
    }

    /** @emoji 📐️ Builds a nested `Origin`-style group: a parent tree item labeled `label` containing
     * three {@link ui_inspector_stepper_field} children (`X`/`Y`/`Z`), each computing its own per-axis
     * mixed state independently — a multi-selection that agrees on X but not Y shows only Y as "Mixed".
     * `axis_action(axis)` builds the per-axis `ActionDescriptor`; callers typically merge
     * `{"field": "<id>.x"}` etc. into its `args` so the patch handler can dot-path into the right
     * component with `value` (absolute) or `delta` (relative, offset-preserving across multi-select). */
    pub fn ui_inspector_vec3_group(id: impl Into<String>, label: impl Into<Label>, values: &[[f64; 3]], step: f64, axis_action: impl Fn(&str) -> ActionDescriptor) -> UiNode {
        let id = id.into();
        let xs: Vec<f64> = values.iter().map(|v| v[0]).collect();
        let ys: Vec<f64> = values.iter().map(|v| v[1]).collect();
        let zs: Vec<f64> = values.iter().map(|v| v[2]).collect();
        UiNode::Group(UiGroupNode {
            menu: None,
            id: id.clone(),
            label: label.into(),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: vec![
                // 🔤️ Axis symbols (X/Y/Z) are mathematical notation, not translatable UI chrome.
                ui_inspector_stepper_field(format!("{id}.x"), Label::data("X"), &xs, step, axis_action("x")),
                ui_inspector_stepper_field(format!("{id}.y"), Label::data("Y"), &ys, step, axis_action("y")),
                ui_inspector_stepper_field(format!("{id}.z"), Label::data("Z"), &zs, step, axis_action("z")),
            ],
        })
    }

    pub fn ui_inspector_groups_to_tree(groups: &[UiInspectorFieldGroup]) -> UiNode {
        let sections: Vec<UiSectionNode> = groups
            .iter()
            .filter(|group| !group.fields.is_empty())
            .map(|group| UiSectionNode { menu: None, id: group.id.clone(), label: Some(group.label.clone()), default_open: Some(group.default_open.unwrap_or(true)), presence: UiPresence::default(), children: group.fields.clone() })
            .collect();
        ui_declarative_sections_to_tree(&sections)
    }

    pub fn ui_declarative_sections_to_tree(sections: &[UiSectionNode]) -> UiNode {
        let mut tree_sections: Vec<UiTreeSectionNode> = Vec::with_capacity(sections.len());
        for section in sections {
            let mut items = Vec::with_capacity(section.children.len());
            for (index, child) in section.children.iter().enumerate() {
                items.push(ui_declarative_child_to_tree_item(child, format!("{}.{}", section.id, index)));
            }
            tree_sections.push(UiTreeSectionNode { id: section.id.clone(), label: section.label.clone(), default_open: Some(section.default_open.unwrap_or(true)), presence: section.presence.clone(), items });
        }
        UiNode::Tree(if tree_sections.is_empty() {
            UiTreeNode {
                menu: None,
                sections: vec![UiTreeSectionNode {
                    id: "empty".into(),
                    label: None,
                    default_open: None,
                    presence: UiPresence::default(),
                    items: vec![UiTreeItemNode {
                        id: "empty".into(),
                        label: Label::data("—"),
                        description: None,
                        icon_id: None,
                        presence: UiPresence::default(),
                        default_open: None,
                        action: None,
                        actions: None,
                        draggable: None,
                        drag_data: None,
                        items: None,
                        control: None,
                        dimmed: None,
                        menu: None,
                    }],
                }],
                presence: UiPresence::default(),
                interaction_domain: None,
                drop_action: None,
            }
        } else {
            UiTreeNode { menu: None, sections: tree_sections, presence: UiPresence::default(), drop_action: None, interaction_domain: None }
        })
    }

    fn ui_declarative_child_to_tree_item(node: &UiNode, fallback_id: String) -> UiTreeItemNode {
        match node {
            UiNode::Text(text) => UiTreeItemNode {
                menu: None,
                id: format!("{}.text", fallback_id),
                label: text.value.clone(),
                description: None,
                icon_id: None,
                presence: UiPresence::default(),
                default_open: None,
                action: None,
                actions: None,
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                dimmed: None,
            },
            UiNode::Field(field) => {
                let description = if let UiNode::Input(input) = field.child.as_ref() { input.placeholder.clone().map(Label::into_string).or_else(|| if input.value.is_empty() { None } else { Some(input.value.clone()) }) } else { None };
                UiTreeItemNode {
                    menu: None,
                    id: field.id.clone(),
                    label: field.label.clone(),
                    description,
                    icon_id: None,
                    presence: UiPresence::default(),
                    default_open: None,
                    action: None,
                    actions: None,
                    draggable: None,
                    drag_data: None,
                    items: None,
                    control: ui_node_to_control(&field.child),
                    dimmed: None,
                }
            }
            UiNode::Button(button) => UiTreeItemNode {
                menu: None,
                id: button.id.clone().unwrap_or(fallback_id),
                label: button.label.clone(),
                description: None,
                icon_id: None,
                presence: UiPresence::default(),
                default_open: None,
                action: None,
                actions: None,
                draggable: None,
                drag_data: None,
                items: None,
                control: Some(UiControlNode::Button(button.clone())),
                dimmed: None,
            },
            UiNode::Input(input) => tree_control_item(input.id.clone(), UiControlNode::Input(input.clone())),
            UiNode::Select(select) => tree_control_item(select.id.clone(), UiControlNode::Select(select.clone())),
            UiNode::Toggle(toggle) => tree_control_item(toggle.id.clone(), UiControlNode::Toggle(toggle.clone())),
            UiNode::Group(group) => {
                let mut items = Vec::with_capacity(group.children.len());
                for (index, child) in group.children.iter().enumerate() {
                    items.push(ui_declarative_child_to_tree_item(child, format!("{}.{}", group.id, index)));
                }
                UiTreeItemNode {
                    menu: None,
                    id: group.id.clone(),
                    label: group.label.clone(),
                    description: None,
                    icon_id: None,
                    presence: UiPresence::default(),
                    default_open: group.default_open,
                    action: None,
                    actions: None,
                    draggable: None,
                    drag_data: None,
                    items: Some(items),
                    control: None,
                    dimmed: None,
                }
            }
            UiNode::KeyValue(key_value) => tree_control_item(fallback_id, UiControlNode::KeyValue(key_value.clone())),
            UiNode::Slider(slider) => tree_control_item(slider.id.clone(), UiControlNode::Slider(slider.clone())),
            UiNode::NumberStepper(stepper) => tree_control_item(stepper.id.clone(), UiControlNode::NumberStepper(stepper.clone())),
            UiNode::Ring(ring) => tree_control_item(ring.id.clone(), UiControlNode::Ring(ring.clone())),
            UiNode::IconSelect(icon_select) => tree_control_item(icon_select.id.clone(), UiControlNode::IconSelect(icon_select.clone())),
            UiNode::Separator(_) => UiTreeItemNode {
                menu: None,
                id: format!("{}.sep", fallback_id),
                label: Label::data("—"),
                description: None,
                icon_id: None,
                presence: UiPresence::default(),
                default_open: None,
                action: None,
                actions: None,
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                dimmed: None,
            },
            other => UiTreeItemNode {
                menu: None,
                id: fallback_id,
                label: Label::data(format!("{other:?}")),
                description: None,
                icon_id: None,
                presence: UiPresence::default(),
                default_open: None,
                action: None,
                actions: None,
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                dimmed: None,
            },
        }
    }

    fn tree_control_item(id: String, control: UiControlNode) -> UiTreeItemNode {
        UiTreeItemNode {
            menu: None,
            id,
            label: Label::data(String::new()),
            description: None,
            icon_id: None,
            presence: UiPresence::default(),
            default_open: None,
            action: None,
            actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: Some(control),
            dimmed: None,
        }
    }
    //#endregion 🔖️InspectorHelpers

    //#region 🔖️ComponentScenes
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum SurfaceKind {
        #[serde(rename = "canvas-2d")]
        Canvas2d,
        #[serde(rename = "world-3d")]
        World3d,
        #[serde(rename = "node-graph")]
        NodeGraph,
        #[serde(rename = "text-editor")]
        TextEditor,
        #[serde(rename = "table")]
        Table,
        #[serde(rename = "paint-2d")]
        Paint2d,
        #[serde(rename = "virtualFileSystem")]
        VirtualFileSystem,
        #[serde(rename = "tiled-map")]
        TiledMap,
        #[serde(rename = "board-2d")]
        Board2d,
        #[serde(rename = "icon-render")]
        IconRender,
        #[serde(rename = "ink-canvas")]
        InkCanvas,
        #[serde(rename = "graph-timeline")]
        GraphTimeline,
        #[serde(rename = "block-list")]
        BlockList,
        #[serde(rename = "diff-view")]
        DiffView,
        #[serde(rename = "event-feed")]
        EventFeed,
    }

    impl SurfaceKind {
        pub fn as_str(self) -> &'static str {
            match self {
                Self::Canvas2d => "canvas-2d",
                Self::World3d => "world-3d",
                Self::NodeGraph => "node-graph",
                Self::TextEditor => "text-editor",
                Self::Table => "table",
                Self::Paint2d => "paint-2d",
                Self::VirtualFileSystem => "virtualFileSystem",
                Self::TiledMap => "tiled-map",
                Self::Board2d => "board-2d",
                Self::IconRender => "icon-render",
                Self::InkCanvas => "ink-canvas",
                Self::GraphTimeline => "graph-timeline",
                Self::BlockList => "block-list",
                Self::DiffView => "diff-view",
                Self::EventFeed => "event-feed",
            }
        }

        pub fn is_viewport(self) -> bool {
            matches!(self, Self::World3d | Self::NodeGraph | Self::Canvas2d | Self::Board2d | Self::InkCanvas)
        }
    }

    // 🎬️ The 15 product scene structs (`Canvas2dScene`, `World3dScene`, `NodeGraphScene` + its
    // nested port/node/edge/viewport/operator records, `TextEditorScene`, `TableScene`,
    // `Paint2dScene`, `IconRenderScene`, `VirtualFileSystemScene`, `TiledMapScene`, `Board2dScene`,
    // `InkCanvasScene`, `GraphTimelineScene`, `DiffViewScene`, `EventFeedScene`, `BlockListScene`)
    // relocated to `semio-framework-ui-scene` — ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-
    // RUNTIME packet `scene-surface`. Re-exported here (not a compatibility shim: this crate
    // genuinely still builds `UiNode`s from them, see `build_table_scene` etc. below) so every
    // existing `ui_wgpu::wgpu::TableScene`/`World3dScene`/... reference keeps compiling unchanged.
    // `TableScene::drop_action_json` and `NodeGraphOperatorChannelRecord::default_json` are now
    // opaque JSON strings, not live `ActionDescriptor`/`serde_json::Value` — see the scene crate's
    // own `🦀️scenes.rs` header for why (that crate is wasm-safe and depends on nothing beyond
    // `ui_contract`/`serde`, so it cannot carry either type).
    pub use ui_scene::{
        BlockListScene, Board2dScene, Canvas2dRejectedSnapshotPage, Canvas2dScene, Canvas2dSnapshotDescriptor, Canvas2dSnapshotFault, Canvas2dSnapshotLease, Canvas2dSnapshotPage, Canvas2dSnapshotWriteToken, DiffViewScene, EventFeedScene,
        GraphTimelineScene, IconRenderScene, InkCanvasScene, NodeGraphEdgeRecord, NodeGraphFindItem, NodeGraphHover, NodeGraphNodeRecord, NodeGraphOperatorChannelRecord, NodeGraphOperatorRecord, NodeGraphOperatorVariadicRecord, NodeGraphPortRecord,
        NodeGraphScene, NodeGraphViewport, Paint2dScene, SceneDoc, TableScene, TextEditorScene, TiledMapScene, VirtualFileSystemScene, World3dRejectedSnapshotPage, World3dScene, World3dSnapshotDescriptor, World3dSnapshotFault, World3dSnapshotItem,
        World3dSnapshotLease, World3dSnapshotPage, World3dSnapshotPageKind, World3dSnapshotSpan, World3dSnapshotWriteToken, canvas2d_snapshot_abort_write, canvas2d_snapshot_abort_write_step, canvas2d_snapshot_admit_page, canvas2d_snapshot_begin,
        canvas2d_snapshot_begin_close, canvas2d_snapshot_close_step, canvas2d_snapshot_seal, canvas2d_snapshot_terminal_is_empty, canvas2d_snapshot_with_page, canvas2d_snapshot_write_terminal_is_empty, decode as decode_surface_doc,
        encode as encode_surface_doc, world3d_snapshot_abort_write, world3d_snapshot_abort_write_step, world3d_snapshot_admit_page, world3d_snapshot_begin, world3d_snapshot_begin_close, world3d_snapshot_close_step, world3d_snapshot_seal,
        world3d_snapshot_terminal_is_empty, world3d_snapshot_with_page, world3d_snapshot_write_terminal_is_empty,
    };

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WorldMeshLodEntry {
        pub lod: f64,
        pub url: String,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WorldLodRecord {
        #[serde(default = "default_true")]
        pub automatic: bool,
        #[serde(default = "default_manual_lod")]
        pub manual: f64,
        #[serde(default = "default_distance_reference")]
        pub distance_reference: f64,
        #[serde(default)]
        pub depth_variable: bool,
        #[serde(default = "default_grid_factor")]
        pub grid_factor: f64,
        #[serde(default)]
        pub grid_snap_enabled: bool,
        #[serde(default = "default_true")]
        pub show_grid: bool,
        #[serde(default)]
        pub grid_datum: Option<[f64; 3]>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WorldChunkingRecord {
        pub chunk_size: f64,
        pub max_distance: f64,
    }

    // 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
    fn default_manual_lod() -> f64 {
        100.0
    }

    // 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
    fn default_distance_reference() -> f64 {
        100.0
    }

    // 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
    fn default_grid_factor() -> f64 {
        10.0
    }

    // 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
    fn default_true() -> bool {
        true
    }

    pub fn world3d_default_lod_json() -> String {
        serde_json::json!({
            "automatic": true,
            "manual": 100.0,
            "distanceReference": 100.0,
            "depthVariable": false,
            "gridFactor": 10.0,
            "gridSnapEnabled": false,
            "showGrid": true,
            "gridDatum": [0.0, 0.0, 0.0],
        })
        .to_string()
    }

    pub fn world3d_chunking_json(chunk_size: f64, max_distance: f64) -> String {
        serde_json::json!({
            "chunkSize": chunk_size,
            "maxDistance": max_distance,
        })
        .to_string()
    }

    // 🎬️ `world3d_default_selection_json`/`world3d_default_meshes_json` moved with `World3dScene`
    // itself into `semio-framework-ui-scene` (they back its `#[serde(default = ...)]` fields) — no
    // longer defined here; nothing else in this file called them directly.

    pub fn world3d_camera_json(position: [f64; 3], target: [f64; 3], fov: f64) -> String {
        serde_json::json!({
            "position": position,
            "target": target,
            "up": [0.0, 0.0, 1.0],
            "fov": fov,
        })
        .to_string()
    }

    // 🎬️ `NodeGraphRecords` (`NodeGraphPortRecord`/`NodeGraphNodeRecord`/`NodeGraphEdgeRecord`/
    // `NodeGraphViewport`/`NodeGraphFindItem`/`NodeGraphHover`/`NodeGraphOperator*Record`) and
    // `NodeGraphScene` itself moved to `semio-framework-ui-scene` together (see the re-export above)
    // — the latter's fields are typed directly against the former, so they moved as one unit.

    // 🎬️ `TextEditorScene` and `TableScene` (+ its `base()`) moved to `semio-framework-ui-scene`
    // (see the re-export above). `TableScene.drop_action` is `drop_action_json: Option<String>` on
    // the moved type — it named `ActionDescriptor` (this crate's own type, defined just below via
    // `UiTreeItemAction`'s sibling), which the wasm-safe scene crate cannot depend on.
    //
    // `TableCell`/`table_row_json` stay here unmoved: they build the `rows_json` STRING that flows
    // into `TableScene.rows_json`, so `TableScene` never had a typed dependency on them.

    //#region 🔖️TableCells
    /// 🧾️ A typed table cell value: plain text/number, or an interactive stepper/button group.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "kind", rename_all = "camelCase")]
    pub enum TableCell {
        Text { value: String },
        Number { value: f64 },
        Stepper { value: f64, min: f64, max: f64, step: f64, action: ActionDescriptor },
        Buttons { buttons: Vec<UiTreeItemAction> },
    }

    /// 🧾️ Builds one `rows_json` record: an id, an optional drag payload, and typed/plain cells keyed by column id.
    pub fn table_row_json(id: impl Into<String>, drag_payload: Option<&serde_json::Value>, cells: &[(&str, TableCell)]) -> serde_json::Value {
        let mut row = serde_json::Map::new();
        row.insert("id".into(), serde_json::Value::String(id.into()));
        if let Some(payload) = drag_payload {
            row.insert("_drag".into(), payload.clone());
        }
        for (column_id, cell) in cells {
            let value = serde_json::to_value(cell).unwrap_or(serde_json::Value::Null);
            row.insert((*column_id).to_string(), value);
        }
        serde_json::Value::Object(row)
    }
    //#endregion 🔖️TableCells

    /* @emoji 🖼️ Paint-2d scene: WASM `RasterSession` sync channels for the composite/navigator windows, see framework/surface/paint/rs/lib.rs. */
    // 🎬️ `Paint2dScene`, `IconRenderScene`, `VirtualFileSystemScene`, `TiledMapScene` (+ its default
    // fns and `base()`), `Board2dScene` (+ its default fns and `base()`), `InkCanvasScene` (+ its
    // default fn and `base()`), `GraphTimelineScene`, `DiffViewScene`, and `EventFeedScene` all moved
    // to `semio-framework-ui-scene` (see the re-export above).

    /** @emoji 🧩️ A palette entry for a block kind insertable into a [`BlockListScene`], contributed
     * either by the host app's own built-ins or by a `"playbook.blockKind"` topic contribution. */
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct BlockPaletteEntry {
        pub block_kind: String,
        pub label: String,
        pub icon_id: IconName,
    }

    // 🎬️ `BlockListScene` itself moved to `semio-framework-ui-scene` (see the re-export above);
    // `BlockPaletteEntry` above stays — `palette_json` is an opaque string on the moved type.

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiExternalSlotNode {
        pub plugin_id: String,
        pub app_id: String,
        pub body_key: String,
        pub params_json: String,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UiComponentSceneNode {
        pub surface_id: String,
        pub controller_id: String,
        pub component_kind: SurfaceKind,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pane_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub binding_id: Option<String>,
        #[serde(default, skip_serializing_if = "UiPresence::is_default")]
        pub presence: UiPresence,
        /// 🖱️ Optional override of the implicit per-`component_kind` convention id (`"world3d"`,
        /// `"nodeGraph"`, `"tiledMap"`, ...) the host uses when resolving which surface answers a
        /// right-click — set only when a plugin needs a menu id other than the surface-kind default.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub menu: Option<UiMenuRef>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub canvas_2d: Option<Canvas2dScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub world_3d: Option<World3dScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub node_graph: Option<NodeGraphScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub text_editor: Option<TextEditorScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub table: Option<TableScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub paint_2d: Option<Paint2dScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub virtual_file_system: Option<VirtualFileSystemScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tiled_map: Option<TiledMapScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub board2d: Option<Board2dScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub icon_render: Option<IconRenderScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ink_canvas: Option<InkCanvasScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub graph_timeline: Option<GraphTimelineScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub block_list: Option<BlockListScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub diff_view: Option<DiffViewScene>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub event_feed: Option<EventFeedScene>,
    }
    //#endregion 🔖️ComponentScenes

    //#region 🔖️UiNode
    /// ⚠️ `ComponentScene`'s payload dwarfs the other variants (nested scene JSON blobs) — boxing it
    /// would be a breaking public-API change (every construction/match site across ~30 plugins would
    /// need `Box::new`/deref updates), out of scope for a mechanical pass.
    #[allow(clippy::large_enum_variant, reason = "boxing is a breaking public API change, out of T1 scope")]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "type", rename_all = "camelCase")]
    pub enum UiNode {
        Stack(UiStackNode),
        Text(UiTextNode),
        Button(UiButtonNode),
        Separator(UiSeparatorNode),
        Input(UiInputNode),
        Select(UiSelectNode),
        Toggle(UiToggleNode),
        KeyValue(UiKeyValueNode),
        Slider(UiSliderNode),
        NumberStepper(UiNumberStepperNode),
        Ring(UiRingNode),
        IconSelect(UiIconSelectNode),
        Field(UiFieldNode),
        Section(UiSectionNode),
        Group(UiGroupNode),
        Tree(UiTreeNode),
        Image(UiImageNode),
        ComponentScene(UiComponentSceneNode),
        ExternalSlot(UiExternalSlotNode),
    }

    impl UiNode {
        /// 🧭️ Exhaustive presence accessor — adding a `UiNode` variant fails to compile here (and in
        /// `presence_mut` and `paint_node`'s match) until the new element's `presence` field is wired up.
        pub fn presence(&self) -> &UiPresence {
            match self {
                UiNode::Stack(n) => &n.presence,
                UiNode::Text(n) => &n.presence,
                UiNode::Button(n) => &n.presence,
                UiNode::Separator(n) => &n.presence,
                UiNode::Input(n) => &n.presence,
                UiNode::Select(n) => &n.presence,
                UiNode::Toggle(n) => &n.presence,
                UiNode::KeyValue(n) => &n.presence,
                UiNode::Slider(n) => &n.presence,
                UiNode::NumberStepper(n) => &n.presence,
                UiNode::Ring(n) => &n.presence,
                UiNode::IconSelect(n) => &n.presence,
                UiNode::Field(n) => &n.presence,
                UiNode::Section(n) => &n.presence,
                UiNode::Group(n) => &n.presence,
                UiNode::Tree(n) => &n.presence,
                UiNode::Image(n) => &n.presence,
                UiNode::ComponentScene(n) => &n.presence,
                UiNode::ExternalSlot(n) => &n.presence,
            }
        }
        pub fn presence_mut(&mut self) -> &mut UiPresence {
            match self {
                UiNode::Stack(n) => &mut n.presence,
                UiNode::Text(n) => &mut n.presence,
                UiNode::Button(n) => &mut n.presence,
                UiNode::Separator(n) => &mut n.presence,
                UiNode::Input(n) => &mut n.presence,
                UiNode::Select(n) => &mut n.presence,
                UiNode::Toggle(n) => &mut n.presence,
                UiNode::KeyValue(n) => &mut n.presence,
                UiNode::Slider(n) => &mut n.presence,
                UiNode::NumberStepper(n) => &mut n.presence,
                UiNode::Ring(n) => &mut n.presence,
                UiNode::IconSelect(n) => &mut n.presence,
                UiNode::Field(n) => &mut n.presence,
                UiNode::Section(n) => &mut n.presence,
                UiNode::Group(n) => &mut n.presence,
                UiNode::Tree(n) => &mut n.presence,
                UiNode::Image(n) => &mut n.presence,
                UiNode::ComponentScene(n) => &mut n.presence,
                UiNode::ExternalSlot(n) => &mut n.presence,
            }
        }
        /// 🖱️ Exhaustive context-menu-ref accessor — adding a `UiNode` variant fails to compile here
        /// (and in `menu_mut`) until the new element's `menu` field is wired up. `None` means the
        /// element bubbles right-clicks to its nearest menu-bearing ancestor.
        pub fn menu(&self) -> Option<&UiMenuRef> {
            match self {
                UiNode::Stack(n) => n.menu.as_ref(),
                UiNode::Text(n) => n.menu.as_ref(),
                UiNode::Button(n) => n.menu.as_ref(),
                UiNode::Separator(n) => n.menu.as_ref(),
                UiNode::Input(n) => n.menu.as_ref(),
                UiNode::Select(n) => n.menu.as_ref(),
                UiNode::Toggle(n) => n.menu.as_ref(),
                UiNode::KeyValue(n) => n.menu.as_ref(),
                UiNode::Slider(n) => n.menu.as_ref(),
                UiNode::NumberStepper(n) => n.menu.as_ref(),
                UiNode::Ring(n) => n.menu.as_ref(),
                UiNode::IconSelect(n) => n.menu.as_ref(),
                UiNode::Field(n) => n.menu.as_ref(),
                UiNode::Section(n) => n.menu.as_ref(),
                UiNode::Group(n) => n.menu.as_ref(),
                UiNode::Tree(n) => n.menu.as_ref(),
                UiNode::Image(n) => n.menu.as_ref(),
                UiNode::ComponentScene(n) => n.menu.as_ref(),
                UiNode::ExternalSlot(n) => n.menu.as_ref(),
            }
        }
        pub fn menu_mut(&mut self) -> &mut Option<UiMenuRef> {
            match self {
                UiNode::Stack(n) => &mut n.menu,
                UiNode::Text(n) => &mut n.menu,
                UiNode::Button(n) => &mut n.menu,
                UiNode::Separator(n) => &mut n.menu,
                UiNode::Input(n) => &mut n.menu,
                UiNode::Select(n) => &mut n.menu,
                UiNode::Toggle(n) => &mut n.menu,
                UiNode::KeyValue(n) => &mut n.menu,
                UiNode::Slider(n) => &mut n.menu,
                UiNode::NumberStepper(n) => &mut n.menu,
                UiNode::Ring(n) => &mut n.menu,
                UiNode::IconSelect(n) => &mut n.menu,
                UiNode::Field(n) => &mut n.menu,
                UiNode::Section(n) => &mut n.menu,
                UiNode::Group(n) => &mut n.menu,
                UiNode::Tree(n) => &mut n.menu,
                UiNode::Image(n) => &mut n.menu,
                UiNode::ComponentScene(n) => &mut n.menu,
                UiNode::ExternalSlot(n) => &mut n.menu,
            }
        }
    }

    // 🎬️ `NodeGraphScene::base` and `TextEditorScene::base` moved into `semio-framework-ui-scene`
    // itself (inherent impls on a foreign type are an orphan-rule error, not just a style choice —
    // see R9/E116). `json_view`/`code_input` stay here as free functions: both embed a `ui_wgpu`-only
    // `ActionDescriptor` into `settings_json` via `serde_json`, so they legitimately belong on this
    // side of the boundary the same way `TableCell`/`table_row_json` do.

    /** @emoji 📖️ Builds a read-only JSON viewer text-editor scene: a pretty-printed JSON buffer,
     * `"json"` language, and `settingsJson` set to `{"readOnly":true}`. */
    pub fn text_editor_json_view(json_pretty: String) -> TextEditorScene {
        let mut scene = TextEditorScene::base(json_pretty, Some("json".into()), None);
        scene.settings_json = Some(serde_json::json!({ "readOnly": true }).to_string());
        scene
    }

    /** @emoji ⌨️ Builds an editable code-input text-editor scene wired to a host settings-change
     * action: `settingsJson` carries `{"readOnly":false,"onEditSettings":<ActionDescriptor>}`, fired
     * by the renderer when the user edits editor settings (font size, tab width, ...) via its own
     * chrome. */
    pub fn text_editor_code_input(buffer: String, language: &str, on_edit_settings: &ActionDescriptor) -> TextEditorScene {
        let mut scene = TextEditorScene::base(buffer, Some(language.into()), None);
        scene.settings_json = Some(serde_json::json!({ "readOnly": false, "onEditSettings": on_edit_settings }).to_string());
        scene
    }

    //#region 🔖️SceneActions
    /** @emoji 🎮️ Renderer-to-plugin action names for node-graph surfaces. */
    pub mod node_graph_actions {
        pub const SELECT: &str = "nodeGraphSelect";
        pub const HOVER: &str = "nodeGraphHover";
        pub const EDIT: &str = "nodeGraphEdit";
        pub const VIEWPORT: &str = "nodeGraphViewport";
        pub const SPOTLIGHT_COMMIT: &str = "spotlightCommit";
    }

    /** @emoji ✍️ Renderer-to-plugin action names for text-editor surfaces. */
    pub mod text_editor_actions {
        pub const EDIT: &str = "textEdit";
        pub const SELECT: &str = "textSelect";
        pub const HOVER: &str = "textHover";
        pub const REQUEST_COMPLETIONS: &str = "requestCompletions";
        pub const COMMIT_RENAME: &str = "commitRename";
        pub const FORMAT_DOCUMENT: &str = "formatDocument";
    }

    /** @emoji 🧩️ Renderer-to-plugin action names for 2D board surfaces. */
    pub mod board2d_actions {
        pub const APPLY_BOARD_EVENTS: &str = "applyBoardEvents";
    }

    /** @emoji 🖊️ Renderer-to-plugin action names for ink canvas surfaces. */
    pub mod ink_canvas_actions {
        pub const APPLY_EVENTS: &str = "inkApplyEvents";
    }

    /** @emoji 🗺️ Renderer-to-plugin action names for tiled map surfaces. */
    pub mod tiled_map_actions {
        pub const SET_CAMERA: &str = "setCamera";
        pub const SET_FEATURE_SELECTION: &str = "setFeatureSelection";
        pub const SET_HOVER: &str = "setHover";
        pub const SET_SELECTION_METHOD: &str = "setSelectionMethod";
        pub const SET_SELECTION_MODE: &str = "setSelectionMode";
        pub const CLEAR_SELECTION: &str = "clearSelection";
        pub const SELECT_ALL: &str = "selectAll";
        pub const DESELECT: &str = "deselect";
        pub const FOCUS_FEATURE: &str = "focusFeature";
        pub const OPEN_SOURCE: &str = "openSource";
        pub const SET_LAYER_STROKE_SCALE: &str = "setLayerStrokeScale";
        pub const FIT_WORLD: &str = "fitWorld";
    }
    //#endregion 🔖️SceneActions

    pub fn ui_stack_vertical(children: Vec<UiNode>) -> UiNode {
        UiNode::Stack(UiStackNode { menu: None, direction: "vertical".into(), gap: Some("standard".into()), padding: None, id: None, presence: UiPresence::default(), activate: None, children, drop_action: None, drop_overlay: None })
    }

    /** @emoji 🖼️ Builds an image node rendering a source URL or path. */
    pub fn ui_image(id: impl Into<String>, src: impl Into<String>, alt: Option<Label>) -> UiNode {
        UiNode::Image(UiImageNode { menu: None, id: id.into(), src: src.into(), alt, presence: UiPresence::default() })
    }

    /** @emoji 🎛️ Extracts the control payload of a {@link UiNode} when it is a control variant. */
    pub fn ui_node_to_control(node: &UiNode) -> Option<UiControlNode> {
        match node {
            UiNode::Input(input) => Some(UiControlNode::Input(input.clone())),
            UiNode::Select(select) => Some(UiControlNode::Select(select.clone())),
            UiNode::Toggle(toggle) => Some(UiControlNode::Toggle(toggle.clone())),
            UiNode::Button(button) => Some(UiControlNode::Button(button.clone())),
            UiNode::KeyValue(key_value) => Some(UiControlNode::KeyValue(key_value.clone())),
            UiNode::Slider(slider) => Some(UiControlNode::Slider(slider.clone())),
            UiNode::NumberStepper(stepper) => Some(UiControlNode::NumberStepper(stepper.clone())),
            UiNode::Ring(ring) => Some(UiControlNode::Ring(ring.clone())),
            UiNode::IconSelect(icon) => Some(UiControlNode::IconSelect(icon.clone())),
            _ => None,
        }
    }

    /** @emoji 🎛️ Wraps a {@link UiControlNode} back into its matching {@link UiNode} control variant (inverse of {@link ui_node_to_control}). */
    pub fn ui_control_to_node(control: UiControlNode) -> UiNode {
        match control {
            UiControlNode::Input(input) => UiNode::Input(input),
            UiControlNode::Select(select) => UiNode::Select(select),
            UiControlNode::Toggle(toggle) => UiNode::Toggle(toggle),
            UiControlNode::Button(button) => UiNode::Button(button),
            UiControlNode::KeyValue(key_value) => UiNode::KeyValue(key_value),
            UiControlNode::Slider(slider) => UiNode::Slider(slider),
            UiControlNode::NumberStepper(stepper) => UiNode::NumberStepper(stepper),
            UiControlNode::Ring(ring) => UiNode::Ring(ring),
            UiControlNode::IconSelect(icon) => UiNode::IconSelect(icon),
        }
    }

    impl Default for UiNode {
        // 🚫️async: E1 impl of external trait (Default) — inlines `ui_stack_vertical`'s pure body rather
        // than asyncifying that widely-async-consumed builder just for this one sync call site — see R9
        fn default() -> Self {
            UiNode::Stack(UiStackNode { menu: None, direction: "vertical".into(), gap: Some("standard".into()), padding: None, id: None, presence: UiPresence::default(), activate: None, children: vec![], drop_action: None, drop_overlay: None })
        }
    }

    pub fn ui_text(value: impl Into<Label>) -> UiNode {
        UiNode::Text(UiTextNode { menu: None, value: value.into(), emphasize: None, data_attributes: None, presence: UiPresence::default() })
    }

    /** @emoji 🔌️ Renders a contributing program body inline at this tree position. */
    pub fn ui_external_slot(plugin_id: impl Into<String>, app_id: impl Into<String>, body_key: impl Into<String>, params_json: impl Into<String>) -> UiNode {
        UiNode::ExternalSlot(UiExternalSlotNode { menu: None, plugin_id: plugin_id.into(), app_id: app_id.into(), body_key: body_key.into(), params_json: params_json.into(), presence: UiPresence::default() })
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per scene-kind payload; grouping into a struct is a T2 restructure, out of scope")]
    fn component_scene(
        surface_id: impl Into<String>,
        controller_id: impl Into<String>,
        component_kind: SurfaceKind,
        pane_id: Option<String>,
        binding_id: Option<String>,
        canvas_2d: Option<Canvas2dScene>,
        world_3d: Option<World3dScene>,
        node_graph: Option<NodeGraphScene>,
        text_editor: Option<TextEditorScene>,
        table: Option<TableScene>,
        paint_2d: Option<Paint2dScene>,
        virtual_file_system: Option<VirtualFileSystemScene>,
        tiled_map: Option<TiledMapScene>,
        board2d: Option<Board2dScene>,
    ) -> UiNode {
        UiNode::ComponentScene(UiComponentSceneNode {
            menu: None,
            surface_id: surface_id.into(),
            controller_id: controller_id.into(),
            component_kind,
            pane_id,
            binding_id,
            presence: UiPresence::default(),
            canvas_2d,
            world_3d,
            node_graph,
            text_editor,
            table,
            paint_2d,
            virtual_file_system,
            tiled_map,
            board2d,
            icon_render: None,
            ink_canvas: None,
            graph_timeline: None,
            block_list: None,
            diff_view: None,
            event_feed: None,
        })
    }

    pub fn build_canvas_2d_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: Canvas2dScene) -> UiNode {
        component_scene(surface_id, controller_id, SurfaceKind::Canvas2d, None, None, Some(scene), None, None, None, None, None, None, None, None)
    }

    pub fn build_world_3d_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: World3dScene) -> UiNode {
        component_scene(surface_id, controller_id, SurfaceKind::World3d, None, None, None, Some(scene), None, None, None, None, None, None, None)
    }

    pub fn build_node_graph_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: NodeGraphScene) -> UiNode {
        component_scene(surface_id, controller_id, SurfaceKind::NodeGraph, None, None, None, None, Some(scene), None, None, None, None, None, None)
    }

    pub fn build_text_editor_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: TextEditorScene) -> UiNode {
        component_scene(surface_id, controller_id, SurfaceKind::TextEditor, None, None, None, None, None, Some(scene), None, None, None, None, None)
    }

    //#region 🔖️TextIdentifierOccurrences
    /// 🔎️ Expands an offset in `text` to the bounds of the identifier (`[A-Za-z0-9_]+`) it falls in, if any.
    pub fn text_identifier_bounds_at(text: &str, offset: usize) -> Option<(usize, usize)> {
        let bytes = text.as_bytes();
        let is_ident = |byte: u8| (byte as char).is_ascii_alphanumeric() || byte == b'_';
        let mut index = offset.min(bytes.len());
        while index > 0 && is_ident(bytes[index - 1]) {
            index -= 1;
        }
        let start = index;
        while index < bytes.len() && is_ident(bytes[index]) {
            index += 1;
        }
        if start == index { None } else { Some((start, index)) }
    }

    /// 🔎️ JSON `{selection, hover}` occurrence ranges for the identifier under `cursor`, for editor cross-highlighting.
    pub fn text_identifier_occurrences_json(text: &str, cursor: usize) -> Option<String> {
        let (start, end) = text_identifier_bounds_at(text, cursor)?;
        let needle = &text[start..end];
        if needle.is_empty() {
            return None;
        }
        let mut ranges = Vec::new();
        let mut scan = 0usize;
        while let Some(found) = text[scan..].find(needle) {
            let at = scan + found;
            let next_end = at + needle.len();
            if text_identifier_bounds_at(text, at) == Some((at, next_end)) {
                ranges.push(serde_json::json!({ "start": at, "end": next_end }));
            }
            scan = at + needle.len();
        }
        let ranges_json = serde_json::to_string(&ranges).unwrap_or_else(|_| "[]".into());
        Some(serde_json::json!({ "selection": ranges_json, "hover": ranges_json }).to_string())
    }
    //#endregion 🔖️TextIdentifierOccurrences

    pub fn build_table_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: TableScene) -> UiNode {
        component_scene(surface_id, controller_id, SurfaceKind::Table, None, None, None, None, None, None, Some(scene), None, None, None, None)
    }

    pub fn build_paint_2d_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: Paint2dScene) -> UiNode {
        component_scene(surface_id, controller_id, SurfaceKind::Paint2d, None, None, None, None, None, None, None, Some(scene), None, None, None)
    }

    pub fn build_virtual_file_system_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: VirtualFileSystemScene, pane_id: Option<String>, binding_id: Option<String>) -> UiNode {
        component_scene(surface_id, controller_id, SurfaceKind::VirtualFileSystem, pane_id, binding_id, None, None, None, None, None, None, Some(scene), None, None)
    }

    pub fn build_tiled_map_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: TiledMapScene) -> UiNode {
        component_scene(surface_id, controller_id, SurfaceKind::TiledMap, None, None, None, None, None, None, None, None, None, Some(scene), None)
    }

    pub fn build_board2d_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: Board2dScene) -> UiNode {
        component_scene(surface_id, controller_id, SurfaceKind::Board2d, None, None, None, None, None, None, None, None, None, None, Some(scene))
    }

    pub fn build_icon_render_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: IconRenderScene) -> UiNode {
        let UiNode::ComponentScene(node) = component_scene(surface_id, controller_id, SurfaceKind::IconRender, None, None, None, None, None, None, None, None, None, None, None) else { unreachable!() };
        UiNode::ComponentScene(UiComponentSceneNode { icon_render: Some(scene), ..node })
    }

    pub fn build_ink_canvas_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: InkCanvasScene) -> UiNode {
        let UiNode::ComponentScene(node) = component_scene(surface_id, controller_id, SurfaceKind::InkCanvas, None, None, None, None, None, None, None, None, None, None, None) else { unreachable!() };
        UiNode::ComponentScene(UiComponentSceneNode { ink_canvas: Some(scene), ..node })
    }

    pub fn build_graph_timeline_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: GraphTimelineScene) -> UiNode {
        let UiNode::ComponentScene(node) = component_scene(surface_id, controller_id, SurfaceKind::GraphTimeline, None, None, None, None, None, None, None, None, None, None, None) else { unreachable!() };
        UiNode::ComponentScene(UiComponentSceneNode { graph_timeline: Some(scene), ..node })
    }

    pub fn build_block_list_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: BlockListScene) -> UiNode {
        let UiNode::ComponentScene(node) = component_scene(surface_id, controller_id, SurfaceKind::BlockList, None, None, None, None, None, None, None, None, None, None, None) else { unreachable!() };
        UiNode::ComponentScene(UiComponentSceneNode { block_list: Some(scene), ..node })
    }

    pub fn build_diff_view_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: DiffViewScene) -> UiNode {
        let UiNode::ComponentScene(node) = component_scene(surface_id, controller_id, SurfaceKind::DiffView, None, None, None, None, None, None, None, None, None, None, None) else { unreachable!() };
        UiNode::ComponentScene(UiComponentSceneNode { diff_view: Some(scene), ..node })
    }

    pub fn build_event_feed_scene(surface_id: impl Into<String>, controller_id: impl Into<String>, scene: EventFeedScene) -> UiNode {
        let UiNode::ComponentScene(node) = component_scene(surface_id, controller_id, SurfaceKind::EventFeed, None, None, None, None, None, None, None, None, None, None, None) else { unreachable!() };
        UiNode::ComponentScene(UiComponentSceneNode { event_feed: Some(scene), ..node })
    }

    //#region 🔖️StatusBuilders
    /** @emoji 🗂️ Builds an empty-state placeholder: a centered title, optional description text, and an
     * optional call-to-action button. */
    pub fn ui_empty_state(id: &str, title: Label, description: Option<Label>, action: Option<UiButtonNode>) -> UiNode {
        let mut children = vec![UiNode::Text(UiTextNode { menu: None, value: title, emphasize: Some(true), data_attributes: None, presence: UiPresence::default() })];
        if let Some(description) = description {
            children.push(ui_text(description));
        }
        if let Some(action) = action {
            children.push(UiNode::Button(action));
        }
        UiNode::Stack(UiStackNode {
            menu: None,
            direction: "vertical".into(),
            gap: Some("standard".into()),
            padding: Some("standard".into()),
            id: Some(id.into()),
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            children,
        })
    }

    /** @emoji ⚠️ Builds an error-state placeholder: an emphasized message and an optional retry button. */
    pub fn ui_error_state(id: &str, message: Label, retry: Option<ActionDescriptor>) -> UiNode {
        let mut children = vec![UiNode::Text(UiTextNode { menu: None, value: message, emphasize: Some(true), data_attributes: None, presence: UiPresence::default() })];
        if let Some(retry) = retry {
            children.push(UiNode::Button(UiButtonNode {
                menu: None,
                id: Some(format!("{id}.retry")),
                icon_id: IconName::RotateCw,
                // 🚧️ Framework-level fallback copy (not app content); not yet routed through app_labels!
                // since this SDK-level builder predates the two-axis macro — flagged for a follow-up pass.
                label: Label::data("Retry"),
                action: retry,
                style: None,
                presence: UiPresence::default(),
            }));
        }
        UiNode::Stack(UiStackNode {
            menu: None,
            direction: "vertical".into(),
            gap: Some("standard".into()),
            padding: Some("standard".into()),
            id: Some(id.into()),
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            children,
        })
    }

    /** @emoji 🩺️ Builds a plugin-recovery panel: bilingual (en/de) crash copy plus three fixed actions —
     * restart the app (`recovery.restartApp`), disable the offending program (`recovery.disablePlugin`), or
     * open diagnostics (`recovery.showDiagnostics`). `quarantined` swaps in the host-auto-disabled copy. */
    pub fn ui_recovery_panel(plugin_id: &str, quarantined: bool, is_de: bool) -> UiNode {
        let title = if is_de { "Plugin-Wiederherstellung" } else { "Plugin Recovery" };
        let message = match (quarantined, is_de) {
            (true, true) => "Dieses Plugin wurde nach wiederholten Abstürzen unter Quarantäne gestellt.",
            (true, false) => "This program was quarantined after repeated crashes.",
            (false, true) => "Dieses Plugin ist abgestürzt.",
            (false, false) => "This program crashed.",
        };
        let restart_label = if is_de { "App neu starten" } else { "Restart App" };
        let disable_label = if is_de { "Plugin deaktivieren" } else { "Disable Plugin" };
        let diagnostics_label = if is_de { "Diagnose anzeigen" } else { "Show Diagnostics" };
        let args = Some(DslValue::Object(vec![("pluginId".into(), DslValue::String(plugin_id.to_string()))]));
        UiNode::Stack(UiStackNode {
            menu: None,
            direction: "vertical".into(),
            gap: Some("standard".into()),
            padding: Some("standard".into()),
            id: Some("recovery.panel".into()),
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            children: vec![
                UiNode::Text(UiTextNode { menu: None, value: Label::data(title), emphasize: Some(true), data_attributes: None, presence: UiPresence::default() }),
                ui_text(Label::data(message)),
                UiNode::Button(UiButtonNode {
                    menu: None,
                    id: Some("recovery.restartApp".into()),
                    icon_id: IconName::RotateCcw,
                    label: Label::data(restart_label),
                    action: ActionDescriptor { controller_id: "recovery".into(), action: "recovery.restartApp".into(), args: args.clone() },
                    style: None,
                    presence: UiPresence::default(),
                }),
                UiNode::Button(UiButtonNode {
                    menu: None,
                    id: Some("recovery.disablePlugin".into()),
                    icon_id: IconName::Link2Off,
                    label: Label::data(disable_label),
                    action: ActionDescriptor { controller_id: "recovery".into(), action: "recovery.disablePlugin".into(), args: args.clone() },
                    style: None,
                    presence: UiPresence::default(),
                }),
                UiNode::Button(UiButtonNode {
                    menu: None,
                    id: Some("recovery.showDiagnostics".into()),
                    icon_id: IconName::Info,
                    label: Label::data(diagnostics_label),
                    action: ActionDescriptor { controller_id: "recovery".into(), action: "recovery.showDiagnostics".into(), args },
                    style: None,
                    presence: UiPresence::default(),
                }),
            ],
        })
    }

    /** @emoji 📥️ Builds a drop-zone `Stack` for importing files: `drop_overlay` supplies the hover-state
     * title/hint/accept copy, `drop_action` fires once the drop completes. */
    pub fn ui_import_drop_zone(id: &str, title: Label, hint: Label, accept: Option<&str>, drop_action: ActionDescriptor) -> UiNode {
        UiNode::Stack(UiStackNode {
            menu: None,
            direction: "vertical".into(),
            gap: Some("standard".into()),
            padding: Some("standard".into()),
            id: Some(id.into()),
            presence: UiPresence::default(),
            activate: None,
            drop_action: Some(drop_action),
            drop_overlay: Some(UiDropOverlaySpec { title: title.clone(), hint: hint.clone(), accept: accept.map(Into::into) }),
            children: vec![ui_text(title), ui_text(hint)],
        })
    }
    //#endregion 🔖️StatusBuilders
    //#endregion 🔖️UiNode

    //#region 🔖️WireFormatGoldenTests
    /** 🧊️ Golden wire-format tests: freeze exact JSON for every UiNode/scene/SurfaceKind
    before these types move into ui_wgpu, so the move can be proven byte-identical. */
    #[cfg(test)]
    mod ui_node_wire_format_tests {
        use super::*;

        fn act(action: &str) -> ActionDescriptor {
            ActionDescriptor { controller_id: "ctrl".into(), action: action.into(), args: None }
        }

        fn sample_tree() -> UiNode {
            UiNode::Stack(UiStackNode {
                menu: None,
                direction: "vertical".into(),
                gap: Some("md".into()),
                padding: None,
                id: Some("root".into()),
                presence: UiPresence::default(),
                activate: None,
                drop_action: None,
                drop_overlay: None,
                children: vec![
                    UiNode::Text(UiTextNode { menu: None, value: Label::data("Hello"), emphasize: Some(true), data_attributes: None, presence: UiPresence::default() }),
                    UiNode::Button(UiButtonNode { menu: None, id: Some("btn1".into()), icon_id: IconName::Save, label: Label::data("Save"), action: act("save"), style: None, presence: UiPresence::default() }),
                    UiNode::Separator(UiSeparatorNode { menu: None, presence: UiPresence::default() }),
                    UiNode::Input(UiInputNode {
                        menu: None,
                        id: "inp1".into(),
                        input_kind: "text".into(),
                        value: "abc".into(),
                        placeholder: Some(Label::data("type...")),
                        commit: None,
                        min: None,
                        max: None,
                        step: None,
                        accept: None,
                        on_change: act("setValue"),
                        presence: UiPresence::default(),
                    }),
                    UiNode::Select(UiSelectNode {
                        menu: None,
                        id: "sel1".into(),
                        value: "a".into(),
                        items: vec![UiSelectItem { value: "a".into(), label: Label::data("A") }, UiSelectItem { value: "b".into(), label: Label::data("B") }],
                        placeholder: None,
                        on_change: act("selectChange"),
                        presence: UiPresence::default(),
                    }),
                    UiNode::Toggle(UiToggleNode { menu: None, id: "tog1".into(), icon_id: IconName::AlignLeft, text: None, on_change: act("toggle"), presence: UiPresence::selected(true) }),
                    UiNode::Group(UiGroupNode {
                        menu: None,
                        id: "grp1".into(),
                        label: Label::data("Group"),
                        default_open: Some(true),
                        presence: UiPresence::default(),
                        children: vec![UiNode::Text(UiTextNode { menu: None, value: Label::data("child"), emphasize: None, data_attributes: None, presence: UiPresence::default() })],
                    }),
                    UiNode::KeyValue(UiKeyValueNode { menu: None, entries: vec![UiKeyValueEntry { label: Label::data("K"), value: "V".into() }], presence: UiPresence::default() }),
                    UiNode::Slider(UiSliderNode { menu: None, id: "sl1".into(), value: 0.5, min: 0.0, max: 1.0, step: 0.1, unit: Some("%".into()), on_change: act("sliderChange"), presence: UiPresence::default() }),
                    UiNode::NumberStepper(UiNumberStepperNode { menu: None, id: "num1".into(), value: 2.0, step: 1.0, uniform: true, on_absolute: act("setAbs"), on_delta: act("setDelta"), presence: UiPresence::default() }),
                    UiNode::Ring(UiRingNode { menu: None, id: "ring1".into(), orb_id: "orb1".into(), t: 0.25, presence: UiPresence::default(), on_change: act("ringChange") }),
                    UiNode::IconSelect(UiIconSelectNode { menu: None, id: "icn1".into(), value: "star".into(), uniform: true, classifier_kind: "icon".into(), on_change: act("iconChange"), presence: UiPresence::default() }),
                    UiNode::Field(UiFieldNode {
                        menu: None,
                        id: "field1".into(),
                        label: Label::data("Field"),
                        description: Some("desc".into()),
                        required: Some(true),
                        error: None,
                        child: Box::new(UiNode::Text(UiTextNode { menu: None, value: Label::data("child"), emphasize: None, data_attributes: None, presence: UiPresence::default() })),
                        presence: UiPresence::default(),
                    }),
                    UiNode::Section(UiSectionNode { menu: None, id: "sec1".into(), label: Some(Label::data("Section")), default_open: Some(true), presence: UiPresence::default(), children: vec![] }),
                    UiNode::Tree(UiTreeNode {
                        menu: None,
                        sections: vec![UiTreeSectionNode {
                            id: "treesec1".into(),
                            label: Some(Label::data("Items")),
                            default_open: Some(true),
                            presence: UiPresence::default(),
                            items: vec![{
                                let mut item = UiTreeItemNode::base("item1", Label::data("Item 1"));
                                item.presence.selected = true;
                                item
                            }],
                        }],
                        presence: UiPresence::default(),
                        interaction_domain: None,
                        drop_action: None,
                    }),
                    UiNode::Image(UiImageNode { menu: None, id: "img1".into(), src: "icon.png".into(), alt: Some(Label::data("alt text")), presence: UiPresence::default() }),
                    UiNode::ComponentScene(UiComponentSceneNode {
                        menu: None,
                        surface_id: "surf1".into(),
                        controller_id: "ctrl".into(),
                        component_kind: SurfaceKind::World3d,
                        pane_id: None,
                        binding_id: None,
                        presence: UiPresence::default(),
                        canvas_2d: None,
                        world_3d: Some(World3dScene {
                            snapshot: None,
                            camera_json: "{}".into(),
                            meshes_json: "[]".into(),
                            instances_json: "[]".into(),
                            selection_json: "{}".into(),
                            vortices_json: None,
                            attractions_json: None,
                            target_volumes_json: None,
                            references_json: None,
                            brush_preview_json: None,
                            interaction_json: None,
                            engagement_preview_json: None,
                            lod_json: None,
                            chunking_json: None,
                            environment_json: None,
                            frame_json: None,
                            fit_json: None,
                            terrain_json: None,
                            points_json: None,
                            status_json: None,
                            domain_id: None,
                            domain_granularity_id: None,
                        }),
                        node_graph: None,
                        text_editor: None,
                        table: None,
                        paint_2d: None,
                        virtual_file_system: None,
                        tiled_map: None,
                        board2d: None,
                        icon_render: None,
                        ink_canvas: None,
                        graph_timeline: None,
                        block_list: None,
                        diff_view: None,
                        event_feed: None,
                    }),
                    UiNode::ExternalSlot(UiExternalSlotNode { menu: None, plugin_id: "plugin1".into(), app_id: "app1".into(), body_key: "body1".into(), params_json: "{}".into(), presence: UiPresence::default() }),
                ],
            })
        }

        const GOLDEN_UI_NODE_TREE_JSON: &str = "{\"type\":\"stack\",\"direction\":\"vertical\",\"gap\":\"md\",\"id\":\"root\",\"children\":[{\"type\":\"text\",\"value\":\"Hello\",\"emphasize\":true},{\"type\":\"button\",\"id\":\"btn1\",\"iconId\":\"save\",\"label\":\"Save\",\"action\":{\"controllerId\":\"ctrl\",\"action\":\"save\"}},{\"type\":\"separator\"},{\"type\":\"input\",\"id\":\"inp1\",\"inputKind\":\"text\",\"value\":\"abc\",\"placeholder\":\"type...\",\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"setValue\"}},{\"type\":\"select\",\"id\":\"sel1\",\"value\":\"a\",\"items\":[{\"value\":\"a\",\"label\":\"A\"},{\"value\":\"b\",\"label\":\"B\"}],\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"selectChange\"}},{\"type\":\"toggle\",\"id\":\"tog1\",\"iconId\":\"align-left\",\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"toggle\"},\"presence\":{\"selected\":true}},{\"type\":\"group\",\"id\":\"grp1\",\"label\":\"Group\",\"defaultOpen\":true,\"children\":[{\"type\":\"text\",\"value\":\"child\"}]},{\"type\":\"keyValue\",\"entries\":[{\"label\":\"K\",\"value\":\"V\"}]},{\"type\":\"slider\",\"id\":\"sl1\",\"value\":0.5,\"min\":0.0,\"max\":1.0,\"step\":0.1,\"unit\":\"%\",\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"sliderChange\"}},{\"type\":\"numberStepper\",\"id\":\"num1\",\"value\":2.0,\"step\":1.0,\"uniform\":true,\"onAbsolute\":{\"controllerId\":\"ctrl\",\"action\":\"setAbs\"},\"onDelta\":{\"controllerId\":\"ctrl\",\"action\":\"setDelta\"}},{\"type\":\"ring\",\"id\":\"ring1\",\"orbId\":\"orb1\",\"t\":0.25,\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"ringChange\"}},{\"type\":\"iconSelect\",\"id\":\"icn1\",\"value\":\"star\",\"uniform\":true,\"classifierKind\":\"icon\",\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"iconChange\"}},{\"type\":\"field\",\"id\":\"field1\",\"label\":\"Field\",\"description\":\"desc\",\"required\":true,\"child\":{\"type\":\"text\",\"value\":\"child\"}},{\"type\":\"section\",\"id\":\"sec1\",\"label\":\"Section\",\"defaultOpen\":true,\"children\":[]},{\"type\":\"tree\",\"sections\":[{\"id\":\"treesec1\",\"label\":\"Items\",\"defaultOpen\":true,\"items\":[{\"id\":\"item1\",\"label\":\"Item 1\",\"presence\":{\"selected\":true}}]}]},{\"type\":\"image\",\"id\":\"img1\",\"src\":\"icon.png\",\"alt\":\"alt text\"},{\"type\":\"componentScene\",\"surfaceId\":\"surf1\",\"controllerId\":\"ctrl\",\"componentKind\":\"world-3d\",\"world3d\":{\"cameraJson\":\"{}\",\"meshesJson\":\"[]\",\"instancesJson\":\"[]\",\"selectionJson\":\"{}\"}},{\"type\":\"externalSlot\",\"pluginId\":\"plugin1\",\"appId\":\"app1\",\"bodyKey\":\"body1\",\"paramsJson\":\"{}\"}]}";

        #[semio_framework_async_macros::async_test]
        async fn ui_node_tree_serializes_to_golden_json() {
            let node = sample_tree();
            let json = serde_json::to_string(&node).unwrap();
            assert_eq!(json, GOLDEN_UI_NODE_TREE_JSON, "UiNode wire format drifted \u{2014} lock this in before moving the type into ui_wgpu");
            let roundtripped: UiNode = serde_json::from_str(&json).unwrap();
            assert_eq!(roundtripped, node);
        }

        /// 🌀️ `presence.status` follows the same skip-if-default convention as `presence.selected`: the whole `presence` key is absent when fully default, and round-trips when set.
        #[semio_framework_async_macros::async_test]
        async fn ui_tree_item_loading_status_skips_when_default_and_roundtrips_when_set() {
            let idle = UiTreeItemNode::base("idle", Label::data("Idle"));
            assert!(!serde_json::to_string(&idle).unwrap().contains("presence"));

            let mut loading = UiTreeItemNode::base("loading1", Label::data("Loading"));
            loading.presence.status = UiStatus::Loading;
            let json = serde_json::to_string(&loading).unwrap();
            assert!(json.contains("\"presence\":{\"status\":\"loading\"}"));
            let roundtripped: UiTreeItemNode = serde_json::from_str(&json).unwrap();
            assert_eq!(roundtripped, loading);
        }

        /// 🌀️ `waiting` follows the same skip-if-default convention as `loading`: absent when unset, round-trips when set.
        #[semio_framework_async_macros::async_test]
        async fn ui_tree_item_waiting_status_skips_when_default_and_roundtrips_when_set() {
            let idle = UiTreeItemNode::base("idle", Label::data("Idle"));
            assert!(!serde_json::to_string(&idle).unwrap().contains("presence"));

            let mut waiting = UiTreeItemNode::base("waiting1", Label::data("Waiting"));
            waiting.presence.status = UiStatus::Waiting;
            let json = serde_json::to_string(&waiting).unwrap();
            assert!(json.contains("\"presence\":{\"status\":\"waiting\"}"));
            let roundtripped: UiTreeItemNode = serde_json::from_str(&json).unwrap();
            assert_eq!(roundtripped, waiting);
        }

        /// 🚫️ `presence.state == Hidden` short-circuits everything else — round-trips like any other state.
        #[semio_framework_async_macros::async_test]
        async fn ui_tree_item_hidden_state_roundtrips() {
            let mut hidden = UiTreeItemNode::base("hidden1", Label::data("Hidden"));
            hidden.presence.state = UiState::Hidden;
            assert!(!hidden.presence.visible());
            let json = serde_json::to_string(&hidden).unwrap();
            assert!(json.contains("\"presence\":{\"state\":\"hidden\"}"));
            let roundtripped: UiTreeItemNode = serde_json::from_str(&json).unwrap();
            assert_eq!(roundtripped, hidden);
        }

        /// 🎉️ `presence.state == Celebrating` serializes to `"celebrating"` — guards the TS/Rust `UiState`
        /// mirror staying byte-for-byte (see `ui/styling/js/index.ts`'s `UI_STATES`).
        #[semio_framework_async_macros::async_test]
        async fn ui_tree_item_celebrating_state_roundtrips() {
            let mut celebrating = UiTreeItemNode::base("celebrating1", Label::data("Celebrating"));
            celebrating.presence.state = UiState::Celebrating;
            assert!(celebrating.presence.visible());
            let json = serde_json::to_string(&celebrating).unwrap();
            assert!(json.contains("\"presence\":{\"state\":\"celebrating\"}"));
            let roundtripped: UiTreeItemNode = serde_json::from_str(&json).unwrap();
            assert_eq!(roundtripped, celebrating);
        }

        /// ✨ Every `UiNode` variant's `presence` field actually serializes when set — an exhaustiveness
        /// belt-and-braces check so a future variant can't silently drop its shared state on the wire.
        #[semio_framework_async_macros::async_test]
        async fn every_ui_node_variant_serializes_a_non_default_presence() {
            fn assert_presence_serializes(mut node: UiNode, label: &str) {
                *node.presence_mut() = UiPresence::selected(true);
                let json = serde_json::to_string(&node).unwrap();
                assert!(json.contains("\"presence\""), "{label} did not serialize a non-default presence: {json}");
            }
            assert_presence_serializes(
                UiNode::Stack(UiStackNode { menu: None, direction: "vertical".into(), gap: None, padding: None, id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children: vec![] }),
                "Stack",
            );
            assert_presence_serializes(UiNode::Text(UiTextNode { menu: None, value: Label::data("x"), emphasize: None, data_attributes: None, presence: UiPresence::default() }), "Text");
            assert_presence_serializes(UiNode::Button(UiButtonNode { menu: None, id: None, icon_id: IconName::CircleDot, label: Label::data("l"), action: act("a"), style: None, presence: UiPresence::default() }), "Button");
            assert_presence_serializes(UiNode::Separator(UiSeparatorNode { menu: None, presence: UiPresence::default() }), "Separator");
            assert_presence_serializes(
                UiNode::Input(UiInputNode {
                    menu: None,
                    id: "i".into(),
                    input_kind: "text".into(),
                    value: "v".into(),
                    placeholder: None,
                    commit: None,
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    on_change: act("a"),
                    presence: UiPresence::default(),
                }),
                "Input",
            );
            assert_presence_serializes(UiNode::Select(UiSelectNode { menu: None, id: "i".into(), value: "v".into(), items: vec![], placeholder: None, on_change: act("a"), presence: UiPresence::default() }), "Select");
            assert_presence_serializes(UiNode::Toggle(UiToggleNode { menu: None, id: "i".into(), icon_id: IconName::CircleDot, text: None, on_change: act("a"), presence: UiPresence::default() }), "Toggle");
            assert_presence_serializes(UiNode::KeyValue(UiKeyValueNode { menu: None, entries: vec![], presence: UiPresence::default() }), "KeyValue");
            assert_presence_serializes(UiNode::Slider(UiSliderNode { menu: None, id: "i".into(), value: 0.0, min: 0.0, max: 1.0, step: 0.1, unit: None, on_change: act("a"), presence: UiPresence::default() }), "Slider");
            assert_presence_serializes(UiNode::NumberStepper(UiNumberStepperNode { menu: None, id: "i".into(), value: 0.0, step: 1.0, uniform: true, on_absolute: act("a"), on_delta: act("a"), presence: UiPresence::default() }), "NumberStepper");
            assert_presence_serializes(UiNode::Ring(UiRingNode { menu: None, id: "i".into(), orb_id: "o".into(), t: 0.0, on_change: act("a"), presence: UiPresence::default() }), "Ring");
            assert_presence_serializes(UiNode::IconSelect(UiIconSelectNode { menu: None, id: "i".into(), value: "v".into(), uniform: true, classifier_kind: "icon".into(), on_change: act("a"), presence: UiPresence::default() }), "IconSelect");
            assert_presence_serializes(
                UiNode::Field(UiFieldNode {
                    menu: None,
                    id: "i".into(),
                    label: Label::data("l"),
                    description: None,
                    required: None,
                    error: None,
                    child: Box::new(UiNode::Text(UiTextNode { menu: None, value: Label::data("x"), emphasize: None, data_attributes: None, presence: UiPresence::default() })),
                    presence: UiPresence::default(),
                }),
                "Field",
            );
            assert_presence_serializes(UiNode::Section(UiSectionNode { menu: None, id: "i".into(), label: None, default_open: None, presence: UiPresence::default(), children: vec![] }), "Section");
            assert_presence_serializes(UiNode::Group(UiGroupNode { menu: None, id: "i".into(), label: Label::data("l"), default_open: None, presence: UiPresence::default(), children: vec![] }), "Group");
            assert_presence_serializes(UiNode::Tree(UiTreeNode { menu: None, sections: vec![], presence: UiPresence::default(), drop_action: None, interaction_domain: None }), "Tree");
            assert_presence_serializes(UiNode::Image(UiImageNode { menu: None, id: "i".into(), src: "s".into(), alt: None, presence: UiPresence::default() }), "Image");
            assert_presence_serializes(UiNode::ExternalSlot(UiExternalSlotNode { menu: None, plugin_id: "p".into(), app_id: "a".into(), body_key: "b".into(), params_json: "{}".into(), presence: UiPresence::default() }), "ExternalSlot");
            assert_presence_serializes(
                UiNode::ComponentScene(UiComponentSceneNode {
                    menu: None,
                    surface_id: "s".into(),
                    controller_id: "c".into(),
                    component_kind: SurfaceKind::Canvas2d,
                    pane_id: None,
                    binding_id: None,
                    presence: UiPresence::default(),
                    canvas_2d: None,
                    world_3d: None,
                    node_graph: None,
                    text_editor: None,
                    table: None,
                    paint_2d: None,
                    virtual_file_system: None,
                    tiled_map: None,
                    board2d: None,
                    icon_render: None,
                    ink_canvas: None,
                    graph_timeline: None,
                    block_list: None,
                    diff_view: None,
                    event_feed: None,
                }),
                "ComponentScene",
            );
        }

        /// ☁ `points_json` follows the same `Option<String>` skip-if-none convention as `terrain_json`:
        /// absent when unset, round-trips (camelCase `pointsJson`) when set.
        #[semio_framework_async_macros::async_test]
        async fn world_3d_scene_points_json_skips_when_none_and_roundtrips_when_set() {
            let bare = World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into());
            assert!(!serde_json::to_string(&bare).unwrap().contains("pointsJson"));

            let mut with_points = bare;
            with_points.points_json = Some(r#"[{"id":"cloud-1","positionsB64":"AACAPwAAAEAAAEBA","colorsB64":"/wAA","size":2.0,"sizeAttenuation":true}]"#.into());
            let json = serde_json::to_string(&with_points).unwrap();
            assert!(json.contains("\"pointsJson\":"));
            let roundtripped: World3dScene = serde_json::from_str(&json).unwrap();
            assert_eq!(roundtripped, with_points);
        }

        const GOLDEN_SURFACE_KIND_JSON: &str =
            "[\"canvas-2d\",\"world-3d\",\"node-graph\",\"text-editor\",\"table\",\"paint-2d\",\"virtualFileSystem\",\"tiled-map\",\"board-2d\",\"icon-render\",\"ink-canvas\",\"graph-timeline\",\"diff-view\",\"event-feed\"]";

        #[semio_framework_async_macros::async_test]
        async fn surface_kind_serializes_to_golden_json() {
            let kinds = vec![
                SurfaceKind::Canvas2d,
                SurfaceKind::World3d,
                SurfaceKind::NodeGraph,
                SurfaceKind::TextEditor,
                SurfaceKind::Table,
                SurfaceKind::Paint2d,
                SurfaceKind::VirtualFileSystem,
                SurfaceKind::TiledMap,
                SurfaceKind::Board2d,
                SurfaceKind::IconRender,
                SurfaceKind::InkCanvas,
                SurfaceKind::GraphTimeline,
                SurfaceKind::DiffView,
                SurfaceKind::EventFeed,
            ];
            let json = serde_json::to_string(&kinds).unwrap();
            assert_eq!(json, GOLDEN_SURFACE_KIND_JSON);
            let roundtripped: Vec<SurfaceKind> = serde_json::from_str(&json).unwrap();
            assert_eq!(roundtripped, kinds);
        }

        const GOLDEN_SCENES_JSON: &str = "[{\"cameraX\":1.0,\"cameraY\":2.0,\"zoom\":1.5,\"layersJson\":\"[]\"},{\"columnsJson\":\"[]\",\"rowsJson\":\"[]\"},{\"documentSyncJson\":\"{}\",\"assetsJson\":\"[]\",\"cameraJson\":\"{}\",\"selectionJson\":\"[]\",\"hoveredId\":\"h1\",\"activeUtility\":\"brush\",\"brushSize\":4.0,\"brushOpacity\":1.0,\"viewMode\":\"composite\"},{\"requestJson\":\"{}\"},{\"schemaJson\":\"{}\",\"rowsJson\":\"[]\",\"emptyMessage\":\"Empty\",\"dragDropEnabled\":true},{\"mapFixtureJson\":\"{}\",\"cameraJson\":\"{}\",\"renderMode\":\"combined\",\"vectorStyle\":\"colored\",\"lodMode\":\"automatic\",\"tileUrlTemplate\":\"/osm/{z}/{x}/{y}.png\",\"vectorTileUrlTemplate\":\"/vt/{z}/{x}/{y}.pbf\",\"layerVisibilityJson\":\"{}\",\"layerStrokeScaleJson\":\"{}\",\"selectionJson\":\"{}\",\"hoverJson\":\"null\",\"selectionMethod\":\"rectangle\",\"selectionMode\":\"default\"},{\"fixtureJson\":\"{}\",\"cameraJson\":\"{}\",\"glyphCatalogsJson\":\"{}\",\"selectionJson\":\"[]\",\"interactive\":true,\"selectionMethod\":\"rectangle\",\"gridSnapEnabled\":false,\"gridFactor\":1.0,\"suggestionOffset\":0.0,\"brushWeightsJson\":\"{}\",\"placementCompatibilityJson\":\"[]\",\"lodMode\":\"automatic\"},{\"documentJson\":\"{}\",\"selectionJson\":\"[]\",\"activeUtility\":\"select\",\"viewMode\":\"edit\",\"interactive\":true},{\"columnsJson\":\"[]\"},{\"nodes\":[],\"edges\":[],\"viewport\":{\"x\":0.0,\"y\":0.0,\"zoom\":1.0}},{\"buffer\":\"buf\",\"language\":\"rust\"},{\"stepsJson\":\"[]\",\"paletteJson\":\"[]\"}]";

        #[semio_framework_async_macros::async_test]
        async fn scene_records_serialize_to_golden_json() {
            let scenes = (
                Canvas2dScene { camera_x: 1.0, camera_y: 2.0, zoom: 1.5, layers_json: "[]".into(), snapshot: None },
                TableScene::base("[]", "[]"),
                Paint2dScene {
                    document_sync_json: "{}".into(),
                    assets_json: "[]".into(),
                    camera_json: "{}".into(),
                    selection_json: "[]".into(),
                    hovered_id: Some("h1".into()),
                    active_utility: "brush".into(),
                    brush_size: 4.0,
                    brush_opacity: 1.0,
                    view_mode: "composite".into(),
                    composite_viewport_json: None,
                },
                IconRenderScene { request_json: "{}".into(), footer: None, frame_json: None },
                VirtualFileSystemScene { schema_json: "{}".into(), rows_json: "[]".into(), selected_row_ids_json: None, hovered_row_id: None, empty_message: Some("Empty".into()), drag_drop_enabled: Some(true) },
                TiledMapScene::base("{}".into(), "{}".into()),
                Board2dScene::base("{}".into(), "{}".into(), true),
                InkCanvasScene::base("{}".into(), "select".into(), "edit".into(), true),
                GraphTimelineScene { columns_json: "[]".into() },
                NodeGraphScene::base(vec![], vec![], NodeGraphViewport { x: 0.0, y: 0.0, zoom: 1.0 }),
                TextEditorScene::base("buf".into(), Some("rust".into()), None),
                BlockListScene { steps_json: "[]".into(), palette_json: "[]".into(), selected_id: None, dragging_id: None, domain_id: None },
            );
            let json = serde_json::to_string(&scenes).unwrap();
            assert_eq!(json, GOLDEN_SCENES_JSON);
            let roundtripped: (Canvas2dScene, TableScene, Paint2dScene, IconRenderScene, VirtualFileSystemScene, TiledMapScene, Board2dScene, InkCanvasScene, GraphTimelineScene, NodeGraphScene, TextEditorScene, BlockListScene) =
                serde_json::from_str(&json).unwrap();
            assert_eq!(roundtripped, scenes);
        }

        /// 🆚 `DiffViewScene`/`EventFeedScene` golden coverage lives in its own pair-tuple (rather than
        /// joining `scene_records_serialize_to_golden_json`'s tuple above) because std only implements
        /// `Debug`/`PartialEq` for tuples up to 12 elements, and that tuple is already at the cap.
        const GOLDEN_DIFF_VIEW_EVENT_FEED_SCENES_JSON: &str = "[{\"before\":\"a\",\"after\":\"b\",\"language\":\"rust\",\"mode\":\"unified\"},{\"entriesJson\":\"[]\",\"follow\":true,\"activateAction\":\"openEvent\"}]";

        #[semio_framework_async_macros::async_test]
        async fn diff_view_and_event_feed_scenes_serialize_to_golden_json() {
            let scenes = (
                DiffViewScene { before: "a".into(), after: "b".into(), language: Some("rust".into()), mode: Some("unified".into()), domain_id: None },
                EventFeedScene { entries_json: "[]".into(), follow: Some(true), activate_action: Some("openEvent".into()), domain_id: None },
            );
            let json = serde_json::to_string(&scenes).unwrap();
            assert_eq!(json, GOLDEN_DIFF_VIEW_EVENT_FEED_SCENES_JSON);
            let roundtripped: (DiffViewScene, EventFeedScene) = serde_json::from_str(&json).unwrap();
            assert_eq!(roundtripped, scenes);
        }

        /// 🖱️ `UiMenuRef`/`ContextMenuItemSpec` camelCase wire shape — in particular `hover_args` must
        /// serialize as `hoverArgs` (the exact field-rename pitfall documented on `UiDirtyScope`).
        #[semio_framework_async_macros::async_test]
        async fn ui_menu_ref_and_context_menu_item_spec_roundtrip_camel_case() {
            let menu_ref = UiMenuRef { id: "row".into(), args: Some(DslValue::Object(vec![("id".into(), DslValue::String("row-1".into()))])) };
            let json = serde_json::to_string(&menu_ref).unwrap();
            assert_eq!(json, r#"{"id":"row","args":{"id":"row-1"}}"#);
            assert_eq!(serde_json::from_str::<UiMenuRef>(&json).unwrap(), menu_ref);

            let item = ContextMenuItemSpec {
                id: "delete".into(),
                label: Some("Delete".into()),
                icon: Some("trash".into()),
                color: None,
                shortcut: Some("Del".into()),
                disabled: Some(false),
                separator: None,
                checked: None,
                destructive: Some(true),
                action: Some("deleteSelection".into()),
                args: None,
                hover_action: Some("previewDelete".into()),
                hover_args: Some(DslValue::Object(vec![("x".into(), DslValue::Number(1.0)), ("y".into(), DslValue::Number(2.0))])),
                children: None,
            };
            let json = serde_json::to_string(&item).unwrap();
            assert!(json.contains("\"hoverAction\""), "hover_action must serialize as hoverAction: {json}");
            assert!(json.contains("\"hoverArgs\":{\"x\":1.0,\"y\":2.0}"), "hover_args must serialize as hoverArgs: {json}");
            assert!(!json.contains("\"color\""), "None fields must be omitted: {json}");
            let roundtripped: ContextMenuItemSpec = serde_json::from_str(&json).unwrap();
            assert_eq!(roundtripped, item);
        }

        /// 🖱️ Every `UiNode` variant's `menu` ref actually serializes when set, and is omitted by default
        /// — the same exhaustiveness belt-and-braces check as `every_ui_node_variant_serializes_a_non_default_presence`.
        #[semio_framework_async_macros::async_test]
        async fn every_ui_node_variant_serializes_a_set_menu_ref() {
            fn assert_menu_serializes(mut node: UiNode, label: &str) {
                assert!(!serde_json::to_string(&node).unwrap().contains("\"menu\""), "{label} must omit a default menu ref");
                *node.menu_mut() = Some(UiMenuRef { id: "m".into(), args: None });
                let json = serde_json::to_string(&node).unwrap();
                assert!(json.contains("\"menu\":{\"id\":\"m\"}"), "{label} did not serialize a set menu ref: {json}");
                assert_eq!(node.menu(), Some(&UiMenuRef { id: "m".into(), args: None }));
            }
            assert_menu_serializes(
                UiNode::Stack(UiStackNode { menu: None, direction: "vertical".into(), gap: None, padding: None, id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children: vec![] }),
                "Stack",
            );
            assert_menu_serializes(UiNode::Text(UiTextNode { menu: None, value: Label::data("x"), emphasize: None, data_attributes: None, presence: UiPresence::default() }), "Text");
            assert_menu_serializes(UiNode::Button(UiButtonNode { menu: None, id: None, icon_id: IconName::CircleDot, label: Label::data("l"), action: act("a"), style: None, presence: UiPresence::default() }), "Button");
            assert_menu_serializes(UiNode::Separator(UiSeparatorNode { menu: None, presence: UiPresence::default() }), "Separator");
            assert_menu_serializes(UiNode::Image(UiImageNode { menu: None, id: "i".into(), src: "s".into(), alt: None, presence: UiPresence::default() }), "Image");
            assert_menu_serializes(UiNode::Tree(UiTreeNode { menu: None, sections: vec![], presence: UiPresence::default(), drop_action: None, interaction_domain: None }), "Tree");
        }

        //#region 🗂️OrganizeContextMenuTests
        fn menu_leaf(id: &str) -> ContextMenuItemSpec {
            ContextMenuItemSpec { id: id.into(), label: Some(id.into()), action: Some(id.into()), ..Default::default() }
        }

        fn menu_destructive(id: &str) -> ContextMenuItemSpec {
            ContextMenuItemSpec { destructive: Some(true), ..menu_leaf(id) }
        }

        fn menu_group(category: &str, children: Vec<ContextMenuItemSpec>) -> ContextMenuItemSpec {
            ContextMenuItemSpec { id: format!("menu.group.{category}"), label: None, children: Some(children), ..Default::default() }
        }

        fn menu_header(label: &str) -> ContextMenuItemSpec {
            ContextMenuItemSpec { id: format!("header-{label}"), label: Some(label.into()), separator: Some(true), ..Default::default() }
        }

        fn menu_separator(id: &str) -> ContextMenuItemSpec {
            ContextMenuItemSpec { id: id.into(), separator: Some(true), ..Default::default() }
        }

        // 🚫️async: E1 pure accessor consumed by sync-only std call sites (&dyn Fn value) — see R9
        fn no_category(_id: &str) -> Option<String> {
            None
        }

        #[semio_framework_async_macros::async_test]
        async fn organize_context_menu_emits_as_is_within_budget() {
            let items = vec![menu_leaf("a"), menu_leaf("b"), menu_group("view", vec![menu_leaf("c")])];
            let organized = organize_context_menu(items.clone(), &no_category);
            assert_eq!(organized, items, "within budget with leaves already before groups, nothing is reordered: {organized:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn organize_context_menu_puts_destructive_leaves_last_after_a_separator() {
            let items = vec![menu_destructive("delete"), menu_leaf("a"), menu_leaf("b")];
            let organized = organize_context_menu(items, &no_category);
            assert_eq!(organized.len(), 4, "a separator is inserted before the destructive tail: {organized:?}");
            assert_eq!(organized[0].id, "a");
            assert_eq!(organized[1].id, "b");
            assert_eq!(organized[2].separator, Some(true));
            assert_eq!(organized[2].label, None, "the inserted separator is bare, not a header");
            assert_eq!(organized[3].id, "delete");
            assert_eq!(organized[3].destructive, Some(true));
        }

        #[semio_framework_async_macros::async_test]
        async fn organize_context_menu_merges_same_id_groups_and_dedupes_children_by_id() {
            let items = vec![menu_group("view", vec![menu_leaf("zoomIn"), menu_leaf("zoomOut")]), menu_leaf("a"), menu_group("view", vec![menu_leaf("zoomOut"), menu_leaf("resetZoom")])];
            let organized = organize_context_menu(items, &no_category);
            assert_eq!(organized.iter().filter(|item| item.id == "menu.group.view").count(), 1, "only one merged row remains: {organized:?}");
            let view_group = organized.iter().find(|item| item.id == "menu.group.view").expect("merged view group present");
            let child_ids: Vec<&str> = view_group.children.as_ref().unwrap().iter().map(|child| child.id.as_str()).collect();
            assert_eq!(child_ids, vec!["zoomIn", "zoomOut", "resetZoom"], "children concat in first-seen order, deduped by id: {child_ids:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn organize_context_menu_collapses_doubled_bare_separators_and_drops_leading_trailing_ones() {
            let items = vec![menu_separator("lead-bare"), menu_leaf("a"), menu_separator("dup-1"), menu_separator("dup-2"), menu_leaf("b"), menu_separator("trail-bare")];
            let organized = organize_context_menu(items, &no_category);
            assert_eq!(organized.len(), 3, "leading/trailing bare separators drop, the doubled run collapses to one: {organized:?}");
            assert_eq!(organized[0].id, "a");
            assert_eq!(organized[1].separator, Some(true));
            assert_eq!(organized[1].label, None, "the surviving separator is bare, not a header");
            assert_eq!(organized[2].id, "b");
        }

        #[semio_framework_async_macros::async_test]
        async fn organize_context_menu_keeps_a_labeled_separator_as_a_non_interactive_header() {
            let items = vec![menu_leaf("a"), menu_header("Recent"), menu_leaf("b")];
            let organized = organize_context_menu(items.clone(), &no_category);
            assert_eq!(organized, items, "a header is preserved in place, untouched by budget/ordering: {organized:?}");
            assert_eq!(organized[1].label.as_deref(), Some("Recent"));
            assert_eq!(organized[1].separator, Some(true));
        }

        #[semio_framework_async_macros::async_test]
        async fn organize_context_menu_sorts_group_rows_in_taxonomy_order_unknown_last() {
            let items = vec![menu_group("mystery", vec![menu_leaf("x")]), menu_group("export", vec![menu_leaf("y")]), menu_group("view", vec![menu_leaf("z")])];
            let organized = organize_context_menu(items, &no_category);
            let ids: Vec<&str> = organized.iter().map(|item| item.id.as_str()).collect();
            assert_eq!(ids, vec!["menu.group.view", "menu.group.export", "menu.group.mystery"], "view < export < unknown category: {ids:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn organize_context_menu_folds_overflow_groups_into_menu_group_more() {
            let mut items: Vec<ContextMenuItemSpec> = Vec::with_capacity(5);
            for index in 0..5 {
                items.push(menu_leaf(&format!("primary{index}")));
            }
            for category in ["hand", "selection", "lasso", "filter", "open", "save", "transfer", "transform"] {
                items.push(menu_group(category, vec![menu_leaf(&format!("{category}-child"))]));
            }
            assert!(items.len() > 9, "fixture must exceed the row budget to exercise the >9 path");
            let organized = organize_context_menu(items, &no_category);
            assert_eq!(organized.len(), 9, "primaries + groups clamp to the 9-row budget: {organized:?}");
            assert_eq!(organized.last().unwrap().id, "menu.group.more");
            assert!(!organized.last().unwrap().children.as_ref().unwrap().is_empty(), "the folded group carries the overflowing groups' children");
        }

        #[semio_framework_async_macros::async_test]
        async fn organize_context_menu_buckets_overflow_leaves_by_category_of() {
            let mut items: Vec<ContextMenuItemSpec> = Vec::with_capacity(5);
            for index in 0..5 {
                items.push(menu_leaf(&format!("primary{index}")));
            }
            for index in 0..6 {
                items.push(menu_leaf(&format!("overflow{index}")));
            }
            let categorize = |id: &str| if id.starts_with("overflow") { Some("view".to_string()) } else { None };
            let organized = organize_context_menu(items, &categorize);
            assert_eq!(organized.len(), 6, "5 primaries + 1 view group: {organized:?}");
            assert_eq!(organized[5].id, "menu.group.view");
            assert_eq!(organized[5].children.as_ref().unwrap().len(), 6);
        }

        #[semio_framework_async_macros::async_test]
        async fn ribbon_parent_label_covers_exactly_the_twenty_taxonomy_ids_and_rejects_unknown() {
            assert_eq!(RIBBON_PARENT_CATEGORIES.len(), 20);
            for category in RIBBON_PARENT_CATEGORIES {
                assert!(ribbon_parent_label(category, false).is_some(), "missing EN label for {category:?}");
                assert!(ribbon_parent_label(category, true).is_some(), "missing DE label for {category:?}");
            }
            assert_eq!(ribbon_parent_label("not-a-category", false), None);
        }

        #[semio_framework_async_macros::async_test]
        async fn build_shell_context_menu_specs_shapes_arg_carrying_actions_and_appends_the_palette() {
            let actions = vec![
                ShellMenuAction { id: "shell.rename".into(), label: "Rename".into(), icon: None, keys: None, kind: "Mutation".into(), category: None, in_palette: true, arg_carrying: true },
                ShellMenuAction { id: "shell.hidden".into(), label: "Hidden".into(), icon: None, keys: None, kind: "Mutation".into(), category: None, in_palette: false, arg_carrying: false },
            ];
            let specs = build_shell_context_menu_specs(&actions, true);
            assert_eq!(specs.len(), 2, "the non-palette action is filtered out, the palette leaf is appended: {specs:?}");
            assert_eq!(specs[0].id, "shell.rename");
            assert_eq!(specs[0].action.as_deref(), Some("shell.openActionPane"), "arg-carrying actions route through the reserved action");
            assert_eq!(specs[0].args, Some(DslValue::Object(vec![("actionId".into(), DslValue::String("shell.rename".into()))])));
            assert_eq!(specs[1].id, "shell.openPalette");
            assert_eq!(specs[1].action.as_deref(), Some("shell.openPalette"));
        }
        //#endregion 🗂️OrganizeContextMenuTests
    }
    //#endregion 🔖️WireFormatGoldenTests
    // #endregion ui
}
