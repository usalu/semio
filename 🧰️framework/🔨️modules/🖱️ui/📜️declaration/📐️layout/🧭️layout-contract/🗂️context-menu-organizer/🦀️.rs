//! 🧹️ Context menu organizer: the menu reference, the context menu item spec, the ribbon parent categories and the group, separator and budget organizing with the shell menu actions. See [`super`].

use super::*;

// #region layout
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

/// 📕️ One row of a resolved context menu — serde camelCase twin of TS `ContextMenuItemSpec`
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
/// 🎀️ Canonical ribbon-parent taxonomy — Rust twin of ui-react's closed `UiRibbonParentCategory`
/// union (`🧰️framework/🔨️modules/🖱️ui/⚛️react/⚡️implementations/🟦️typescript/🟦️.tsx` ~3317). Id
/// spelling and order are load-bearing: `organize_context_menu` sorts `menu.group.<category>` rows by
/// this order (unknown categories sort after, in emit order) and `ribbon_parent_label` covers exactly
/// these 20 ids.
pub const RIBBON_PARENT_CATEGORIES: [&str; 20] =
    ["history", "hand", "selection", "lasso", "filter", "open", "save", "transfer", "transform", "create", "view", "actions", "settings", "methods", "mode", "targets", "export", "tools", "utilities", "sync"];

/// 🔖️ EN/DE label for a `RIBBON_PARENT_CATEGORIES` id — hand-maintained Rust twin of ui-react's
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

/// 👥️ Every context-menu GROUP row id carries this prefix — the one spelling `organize_context_menu`
/// synthesizes and `shell_context_menu_item_from_spec` resolves against. Twin of ui-react's
/// `CONTEXT_MENU_GROUP_ID_PREFIX` (`🧰️framework/🔨️modules/🔺️mesh/🟦️.ts`).
pub const CONTEXT_MENU_GROUP_ID_PREFIX: &str = "menu.group.";

/// 🗂️ The one group category OUTSIDE [`RIBBON_PARENT_CATEGORIES`]: the overflow bucket
/// `organize_context_menu` synthesizes when the row budget is exceeded. Twin of ui-react's
/// `CONTEXT_MENU_OVERFLOW_CATEGORY`.
pub const CONTEXT_MENU_OVERFLOW_CATEGORY: &str = "more";

/// 🏷️ EN/DE display label for a `menu.group.<category>` row, which the guest emits with `label: None`
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

/// 📌️ D1: a separator carrying a `label` is a non-interactive section header, not a divider.
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

/// ➖️ Collapses a run of consecutive bare (unlabeled) separators down to one, then drops a bare
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

/// 🔀️ Merges rows sharing a `menu.group.<category>` id at the position of the first occurrence,
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

/// 💰️ ≤9-interactive-row emission (D2 rule): plain leaves/headers in emit order, then group rows in
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

/// 💸️ >9-interactive-row emission (D2 rule): the first 5 plain leaves outside any header section stay
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

/// 🗃️ Pure organizer enforced at every context-menu funnel (SDK `VcsArtifactApp::context_menu`, shell
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

/// 🎬️ Declarative input row for `build_shell_context_menu_specs` — Rust twin of the TS `ShellMenuAction`
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

/// 🔨️ Shell fallback context menu (window-background right-click, "no plugin resolved a menu") builder
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
//#endregion 🔖️ContextMenu
// #endregion layout
