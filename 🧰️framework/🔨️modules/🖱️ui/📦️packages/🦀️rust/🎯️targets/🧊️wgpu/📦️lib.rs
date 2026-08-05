//! 🖱️ Declarative UI components (default) and retained-mode wgpu engine (feature "engine").

#[path = "../../../../../🖼️assets/⚡️implementations/🟦️typescript/🔣️icons/🤖️generated/🦀️icon_name.rs"]
mod icon_name_gen;

pub use icon_name_gen::IconName;

//#region 🔖️UiAxes
#[path = "🤖️generated.rs"]
mod ui_axes_gen;

pub use ui_axes_gen::{Locale, Terminology};
//#endregion 🔖️UiAxes

//#region 🔖️Label
// 🎗️ Replaces raw `String` labels on `UiNode` and the app manifest — a `Label` is only constructible
// from `app_labels!`-produced `LabelText` or explicit runtime data (`Label::data`), so a hardcoded
// literal assigned to a label field (`label: "LOD".into()`) does not compile. See ticket
// 26/08/03/COMPILE-TIME-CHECKED-UI-LABELS-ACROSS-LOCALE-TERMINOLOGY-AND-BRAND.
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// 🎗️ A display-ready UI string. No `From<&str>`/`From<String>` on purpose.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub struct Label(String);

impl Label {
    /// 📊️ Genuine runtime data (file names, counts, user content) rendered as a label. Passing a
    /// string literal here is a gate violation (see the Rust twin of `uiDataLabel`'s TS lint).
    pub fn data(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl From<LabelText> for Label {
    fn from(text: LabelText) -> Self {
        Self(text.0.to_string())
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::borrow::Borrow<str> for Label {
    fn borrow(&self) -> &str {
        &self.0
    }
}

/// 🧵️ A localized static template, produced only by `app_labels!` (never construct directly — the
/// hidden constructor is the macro's, not a public API; committed source calling it is a gate
/// violation, mirroring the TS `__from_app_labels` ban).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LabelText(&'static str);

impl LabelText {
    #[doc(hidden)]
    pub const fn __from_app_labels(text: &'static str) -> Self {
        Self(text)
    }

    pub fn as_str(self) -> &'static str {
        self.0
    }

    /// 🧵️ Named-placeholder runtime fill, e.g. `labels.selected_count.fill(&[("count", &n.to_string())])`
    /// — substitution, not `format!`, so word order never has to match across locales.
    pub fn fill(self, args: &[(&str, &str)]) -> Label {
        let mut out = self.0.to_string();
        for (name, value) in args {
            out = out.replace(&format!("{{{name}}}"), value);
        }
        Label(out)
    }
}

impl From<LabelText> for String {
    fn from(text: LabelText) -> Self {
        text.0.to_string()
    }
}

/// 🗺️ Full locale×terminology matrix for a manifest label, resolved shell-side per active axes —
/// the multilingual replacement for `AppLabelsOverlay`'s stringly-typed per-id maps. TS mirror is
/// the hand-generated `LocalizedLabel` type in `framework/⚡️implementations/🟦️typescript/🤖️generated/🟦️ui-axes.ts`
/// (not ts-rs-derived — the wire shape below is manually kept in sync with that type).
#[derive(Clone, Debug, PartialEq)]
pub struct LocalizedLabel {
    cells: [[Cow<'static, str>; Locale::COUNT]; Terminology::COUNT],
}

impl LocalizedLabel {
    /// 🗺️ Builds the full matrix from a resolver called once per (terminology, locale) cell.
    pub fn from_fn(mut resolve: impl FnMut(Terminology, Locale) -> String) -> Self {
        let cells = std::array::from_fn(|ti| {
            let terminology = Terminology::ALL[ti];
            std::array::from_fn(|li| Cow::Owned(resolve(terminology, Locale::ALL[li])))
        });
        Self { cells }
    }

    /// 📊️ Locale-invariant runtime data (fixture names, proper nouns) broadcast to every cell.
    pub fn data(value: impl Into<String>) -> Self {
        let value = value.into();
        Self::from_fn(|_, _| value.clone())
    }

    /// 🌐️ Terminology-invariant framework-owned text (same copy regardless of terminology, real
    /// per-locale translation) — for the framework's own built-in manifest text (history actions,
    /// panel tabs, …), which has no app-declared terminology axis. The exhaustive match on `Locale`
    /// (no catch-all) means adding a locale breaks every call site here until translated.
    pub fn native(en: &str, de: &str) -> Self {
        Self::from_fn(|_terminology, locale| {
            match locale {
                Locale::En => en,
                Locale::De => de,
            }
            .to_string()
        })
    }

    pub fn resolve(&self, terminology: Terminology, locale: Locale) -> &str {
        &self.cells[terminology.index()][locale.index()]
    }
}

impl Serialize for LocalizedLabel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut outer = serializer.serialize_map(Some(Terminology::COUNT))?;
        for terminology in Terminology::ALL {
            let inner: std::collections::BTreeMap<&str, &str> = Locale::ALL.iter().map(|&locale| (locale.as_str(), self.resolve(terminology, locale))).collect();
            outer.serialize_entry(terminology.as_str(), &inner)?;
        }
        outer.end()
    }
}

impl<'de> Deserialize<'de> for LocalizedLabel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw: std::collections::HashMap<String, std::collections::HashMap<String, String>> = Deserialize::deserialize(deserializer)?;
        Ok(Self::from_fn(|terminology, locale| raw.get(terminology.as_str()).and_then(|m| m.get(locale.as_str())).cloned().unwrap_or_default()))
    }
}

/// 🗣️ Two-axis label set; implement via `semio_framework_plugin::app_labels!` only — the macro emits
/// an exhaustive `match (terminology, locale)` with no catch-all, so a `Locale`/`Terminology` variant
/// added to the generated axes fails every implementor's build until covered.
pub trait AppLabels: Sized + 'static {
    fn labels(locale: Locale, terminology: Terminology) -> &'static Self;
}
//#endregion 🔖️Label

// #region component
// 🧩️ Declarative UI component model (declarative `UiNode` tree, scene records, `SurfaceKind`, `WindowLayout`/`WindowEngagement`/`WindowMeasure`, `UtilityNode`) — moved verbatim from framework/core/rs/lib.rs; JSON wire format is byte-identical to the pre-move version (see the inline `*_wire_format_tests` mods). Ungated (default features) so wasm32-wasip2 program builds stay dependency-clean; must never reference `semio_framework_core`.
pub mod component {
    pub mod layout {
        // #region layout
        //! 📐️ Window layouts, panel tab constants, and engagement rails.

        use crate::IconName;
        use dsl::DslValue;
        use serde::{Deserialize, Serialize};
        use std::collections::HashMap;

        //#region 🔖️Action
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct ActionDescriptor {
            pub controller_id: String,
            pub action: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional, type = "unknown"))]
            pub args: Option<DslValue>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
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
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
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
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub enum UiStatus {
            Waiting,
            Loading,
            #[default]
            Idle,
            Finished,
        }

        fn is_default<T: Default + PartialEq>(value: &T) -> bool {
            *value == T::default()
        }

        /// 🧭️ The shared, compile-time-enforced state model every rendered UI element embeds as a
        /// mandatory `presence` field: `state` × `status` × `hover` × `selected`. All combinations are
        /// visually distinguishable except `state == Hidden`, which makes the rest irrelevant — see
        /// [`UiPresence::visible`]. Defaults to fully inert (`Normal`/`Idle`/`false`/`false`) and is omitted
        /// from the wire format entirely at default (see `UiPresence::is_default`).
        #[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
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
        }

        impl UiPresence {
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
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct UiMenuRef {
            pub id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional, type = "unknown"))]
            pub args: Option<DslValue>,
        }

        /// 🖱️ One row of a resolved context menu — serde camelCase twin of TS `ContextMenuItemSpec`
        /// (`framework/core/js/index.ts`). Plugins build these with `MenuBuilder`; the host maps them
        /// through `ContextMenuController` (React) / `render_context_menu` (wgpu) unchanged.
        #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct ContextMenuItemSpec {
            pub id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub label: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub icon: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub color: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub shortcut: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub disabled: Option<bool>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub separator: Option<bool>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub checked: Option<bool>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub destructive: Option<bool>,
            /// 🎯️ An action id, dispatched via the surface's already-scoped `dispatch(action, args)` — NOT
            /// an `ActionDescriptor` (no separate `controllerId`); matches the pre-existing TS
            /// `ContextMenuItemSpec.action` shape (`framework/core/js/index.ts`), which every emitting
            /// plugin already produces this way.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub action: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional, type = "unknown"))]
            pub args: Option<DslValue>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub hover_action: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional, type = "unknown"))]
            pub hover_args: Option<DslValue>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
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

        fn context_menu_is_bare_separator(item: &ContextMenuItemSpec) -> bool {
            item.separator == Some(true) && item.label.is_none()
        }

        /// 🗂️ D1: a separator carrying a `label` is a non-interactive section header, not a divider.
        fn context_menu_is_header(item: &ContextMenuItemSpec) -> bool {
            item.separator == Some(true) && item.label.is_some()
        }

        fn context_menu_is_group_row(item: &ContextMenuItemSpec) -> bool {
            item.id.starts_with("menu.group.")
        }

        fn context_menu_group_category(item: &ContextMenuItemSpec) -> &str {
            item.id.strip_prefix("menu.group.").unwrap_or(item.id.as_str())
        }

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
                if context_menu_is_bare_separator(&item) && out.last().map(context_menu_is_bare_separator).unwrap_or(false) {
                    continue;
                }
                out.push(item);
            }
            if out.first().map(context_menu_is_bare_separator).unwrap_or(false) {
                out.remove(0);
            }
            while out.last().map(context_menu_is_bare_separator).unwrap_or(false) {
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

        /// 🗂️ Pure organizer enforced at every context-menu funnel (SDK `VcsDocumentApp::context_menu`, shell
        /// builders) — recurses into `children`, normalizes separators (labeled = kept header, bare
        /// leading/trailing/doubled = dropped), merges duplicate `menu.group.<category>` rows (deduping their
        /// children by id), then applies the ≤9-row / >9-row emission policy from D2 of the grouped-context-menu
        /// mechanism design (`context_menu_emit_within_budget`/`context_menu_emit_over_budget`).
        /// `category_of` resolves a leaf's dispatched action id to a `RIBBON_PARENT_CATEGORIES` id (`None`
        /// buckets into `"actions"`) — pass `AppActionRegistry::category_of` at the SDK funnel, or
        /// `ActionDefinition.category` lookups in shell builders.
        pub fn organize_context_menu(items: Vec<ContextMenuItemSpec>, category_of: &dyn Fn(&str) -> Option<String>) -> Vec<ContextMenuItemSpec> {
            let items: Vec<ContextMenuItemSpec> = items
                .into_iter()
                .map(|item| {
                    let children = item.children.map(|children| organize_context_menu(children, category_of));
                    ContextMenuItemSpec { children, ..item }
                })
                .collect();
            let items = context_menu_merge_group_rows(context_menu_normalize_separators(items));
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
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct ContextMenuHit {
            pub domain: String,
            pub id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub label: Option<String>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct ContextMenuSelectionGroup {
            pub domain: String,
            pub ids: Vec<String>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct ContextMenuTextContext {
            pub caret: usize,
            pub has_selection: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub word: Option<String>,
            pub can_rename: bool,
            pub has_completions: bool,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct ContextMenuSurfaceTarget {
            pub surface_id: String,
            pub kind: String,
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub hits: Vec<ContextMenuHit>,
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub selection: Vec<ContextMenuSelectionGroup>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub text: Option<ContextMenuTextContext>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct ContextMenuPoint {
            pub x: f64,
            pub y: f64,
        }

        /// 🖱️ The plugin-facing on-demand menu request — deliberately does NOT carry view state (this crate
        /// must never reference `semio_framework_core`'s `ViewState`, same boundary as every other type
        /// here). Mirrors `handle_action`/`render`/`tool_measures`, which all take `view_state: &ViewState`
        /// as a separate `DocumentApp` method parameter rather than embedding it in the request payload; the
        /// plugin SDK's `plugin_context_menu` free function parses the WIT-level combined JSON (which DOES
        /// carry `viewState`, matching the TS `PluginContextMenuRequest` wire shape) and splits it into this
        /// smaller struct plus a typed `ViewState` before calling `DocumentApp::context_menu`.
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct ContextMenuRequest {
            pub menu: UiMenuRef,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub surface: Option<ContextMenuSurfaceTarget>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub window_instance_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub point: Option<ContextMenuPoint>,
        }

        #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct ContextMenuResponse {
            pub items: Vec<ContextMenuItemSpec>,
        }
        //#endregion 🔖️ContextMenu

        //#region 🔖️PanelTabConstants
        pub const FRAMEWORK_PANEL_TAB_DOCUMENT_ID: &str = "framework.panel.document";
        pub const FRAMEWORK_PANEL_TAB_CATALOGUE_ID: &str = "framework.panel.catalogue";
        pub const FRAMEWORK_PANEL_TAB_INSPECTION_ID: &str = "framework.panel.inspection";
        pub const FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL: &str = "Document";
        pub const FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL: &str = "Catalogue";
        pub const FRAMEWORK_PANEL_TAB_INSPECTION_LABEL: &str = "Inspection";
        pub const FRAMEWORK_PANEL_TAB_DOCUMENT_ICON_ID: &str = "framework.panel.document";
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
        /// 🕰️ Reserved `body_key` intercepted first in `VcsDocumentApp::render`, before any app-specific
        /// body-key match — both renderers fetch it like any other panel-tab body.
        pub const FRAMEWORK_HISTORY_BODY_KEY: &str = "framework.body.history";

        /// 🗣️ Resolves a well-known framework panel-tab id to its native English/German label; unknown ids resolve to None so app-specific panel tabs are left untouched.
        pub fn framework_panel_tab_label(id: &str, is_de: bool) -> Option<&'static str> {
            match (id, is_de) {
                (FRAMEWORK_PANEL_TAB_DOCUMENT_ID, false) => Some(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL),
                (FRAMEWORK_PANEL_TAB_DOCUMENT_ID, true) => Some("Dokument"),
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
        fn kind_window() -> String {
            "window".into()
        }

        fn kind_stack() -> String {
            "stack".into()
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct WindowLayoutWindowNode {
            #[serde(default = "kind_window")]
            pub kind: String,
            pub window_kind_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub instance_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub template_id: Option<String>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct WindowLayoutStackNode {
            #[serde(default = "kind_stack")]
            pub kind: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub size: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none", alias = "activeId")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub active_window_kind_id: Option<String>,
            pub children: Vec<WindowLayoutWindowNode>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct WindowLayoutAxisNode {
            pub kind: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub size: Option<f64>,
            pub children: Vec<WindowLayoutChild>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(untagged)]
        pub enum WindowLayoutChild {
            Axis(WindowLayoutAxisNode),
            Stack(WindowLayoutStackNode),
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(untagged)]
        pub enum WindowLayoutRoot {
            Axis(WindowLayoutAxisNode),
            Stack(WindowLayoutStackNode),
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct WindowLayout {
            pub root: WindowLayoutRoot,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct NamedLayout {
            pub id: String,
            pub label: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub icon_id: Option<IconName>,
            pub layout: WindowLayout,
            pub origin: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub group_path: Option<Vec<String>>,
        }

        pub fn create_window_layout(window_kind_id: impl Into<String>, title: Option<String>, instance_id: Option<String>, template_id: Option<String>) -> WindowLayoutWindowNode {
            WindowLayoutWindowNode { kind: kind_window(), window_kind_id: window_kind_id.into(), title, instance_id, template_id }
        }

        pub fn create_stack_layout(window_kind_ids: &[String], titles: Option<&[String]>) -> WindowLayout {
            WindowLayout {
                root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
                    kind: kind_stack(),
                    size: None,
                    active_window_kind_id: None,
                    children: window_kind_ids.iter().enumerate().map(|(index, id)| create_window_layout(id.clone(), titles.and_then(|rows| rows.get(index).cloned()), None, None)).collect(),
                }),
            }
        }

        pub fn create_default_layout(window_ids: &[String], direction: &str, sizes: Option<&[f64]>, titles: Option<&[String]>) -> WindowLayout {
            WindowLayout {
                root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
                    kind: direction.into(),
                    size: None,
                    children: window_ids
                        .iter()
                        .enumerate()
                        .map(|(index, id)| {
                            WindowLayoutChild::Stack(WindowLayoutStackNode {
                                kind: kind_stack(),
                                size: sizes.and_then(|rows| rows.get(index).copied()),
                                active_window_kind_id: None,
                                children: vec![create_window_layout(id.clone(), titles.and_then(|rows| rows.get(index).cloned()).or_else(|| Some(id.clone())), None, None)],
                            })
                        })
                        .collect(),
                }),
            }
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
            WindowLayout {
                root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
                    kind: "row".into(),
                    size: None,
                    children: window_ids
                        .iter()
                        .map(|id| WindowLayoutChild::Stack(WindowLayoutStackNode { kind: kind_stack(), size: Some(1.0 / count), active_window_kind_id: Some(id.clone()), children: vec![create_window_layout(id.clone(), None, None, None)] }))
                        .collect(),
                }),
            }
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
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct MeasureSelectItem {
            pub id: String,
            pub value: String,
            pub label: String,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
        pub enum WindowMeasure {
            Select {
                id: String,
                #[cfg_attr(feature = "typegen", ts(optional))]
                label: Option<String>,
                value: String,
                items: Vec<MeasureSelectItem>,
                #[cfg_attr(feature = "typegen", ts(rename = "onChange"))]
                on_change: ActionDescriptor,
            },
            Slider {
                id: String,
                #[cfg_attr(feature = "typegen", ts(optional))]
                label: Option<String>,
                value: f64,
                min: f64,
                max: f64,
                #[cfg_attr(feature = "typegen", ts(optional))]
                step: Option<f64>,
                /// 🎚️ Absolute value on the fixed `[min, max]` range that is already preloaded/ready.
                /// Renderers keep `max` stable and draw a highlight from the knob to this extent.
                #[serde(default, skip_serializing_if = "Option::is_none")]
                #[cfg_attr(feature = "typegen", ts(optional))]
                ready: Option<f64>,
                /// 🌀️ When true, the measure tree leaf shows a loading ring while preload continues.
                #[serde(default, skip_serializing_if = "Option::is_none")]
                #[cfg_attr(feature = "typegen", ts(optional))]
                loading: Option<bool>,
                /// 🌀️ When true, the measure tree leaf shows a dashed, slower waiting ring; `loading` takes precedence when both are set.
                #[serde(default, skip_serializing_if = "Option::is_none")]
                #[cfg_attr(feature = "typegen", ts(optional))]
                waiting: Option<bool>,
                /// 🚫️ When true, the slider is inert — used when a parent weight is zero so joint percentages cannot change anything.
                #[serde(default, skip_serializing_if = "Option::is_none")]
                #[cfg_attr(feature = "typegen", ts(optional))]
                disabled: Option<bool>,
                /// 🪣️ When set, this is a reveal-group id: the host must NOT dispatch `onChange` on every drag
                /// value — only on gesture commit (pointer-up) — and while dragging must locally cut off
                /// instances tagged with this reveal group's id instead. See `WorldInstancesLayer`'s reveal
                /// cutoff store and `revealCutoffs` in `World3dScene.interaction_json`.
                #[serde(default, skip_serializing_if = "Option::is_none")]
                #[cfg_attr(feature = "typegen", ts(optional))]
                reveal: Option<String>,
                #[cfg_attr(feature = "typegen", ts(rename = "onChange"))]
                on_change: ActionDescriptor,
            },
            Toggle {
                id: String,
                #[cfg_attr(feature = "typegen", ts(rename = "iconId"))]
                icon_id: IconName,
                #[cfg_attr(feature = "typegen", ts(optional))]
                label: Option<String>,
                pressed: bool,
                #[cfg_attr(feature = "typegen", ts(optional))]
                text: Option<String>,
                #[cfg_attr(feature = "typegen", ts(rename = "onChange"))]
                on_change: ActionDescriptor,
            },
            Group {
                id: String,
                label: String,
                #[cfg_attr(feature = "typegen", ts(optional, rename = "defaultOpen"))]
                default_open: Option<bool>,
                /// 🎯️ When `Some(utility_id)`, this group is *utility-scoped chrome*: the shell surfaces it only while
                /// `ViewState.active_utility_id == utility_id`, and renders it in the dedicated "Utility Options" rail
                /// beside the utility bar — never in the always-on Measures overlay. When absent, the group is a
                /// general measure and stays in the Measures overlay exactly as before. See [`partition_window_measures`].
                #[serde(skip_serializing_if = "Option::is_none")]
                #[cfg_attr(feature = "typegen", ts(optional, rename = "activeUtilityId"))]
                active_utility_id: Option<String>,
                /// 🎚️ Optional header slider — when set with `on_change`, the group row hosts a weight control (e.g. object-kind probability).
                #[serde(skip_serializing_if = "Option::is_none")]
                #[cfg_attr(feature = "typegen", ts(optional))]
                value: Option<f64>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[cfg_attr(feature = "typegen", ts(optional))]
                min: Option<f64>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[cfg_attr(feature = "typegen", ts(optional))]
                max: Option<f64>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[cfg_attr(feature = "typegen", ts(optional))]
                step: Option<f64>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[cfg_attr(feature = "typegen", ts(optional))]
                ready: Option<f64>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[cfg_attr(feature = "typegen", ts(optional))]
                loading: Option<bool>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[cfg_attr(feature = "typegen", ts(optional))]
                waiting: Option<bool>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[cfg_attr(feature = "typegen", ts(optional, rename = "onChange"))]
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
        /// @emoji 🎯️ Splits a window's top-level measures into `(general, utility_options)`.
        ///
        /// A top-level [`WindowMeasure::Group`] tagged with `active_utility_id: Some(id)` is *utility-scoped chrome*:
        /// its **children** land in `utility_options` **only** when `id == active_utility_id`, and the tagged wrapper
        /// is dropped from both buckets otherwise (it is irrelevant to whichever utility — or no utility — is
        /// currently active). The wrapper itself is a routing envelope only — never rendered — so activating a
        /// utility shows its option tree directly (no duplicate utility-name group header). Every untagged group
        /// and every non-group top-level measure stays in `general`, unchanged. Tagging is a top-level concept only.
        pub fn partition_window_measures(measures: &[WindowMeasure], active_utility_id: Option<&str>) -> (Vec<WindowMeasure>, Vec<WindowMeasure>) {
            let mut general = Vec::new();
            let mut utility_options = Vec::new();
            for measure in measures {
                match measure {
                    WindowMeasure::Group { active_utility_id: Some(scoped), children, .. } => {
                        if active_utility_id == Some(scoped.as_str()) {
                            utility_options.extend(children.iter().cloned());
                        }
                    }
                    _ => general.push(measure.clone()),
                }
            }
            (general, utility_options)
        }
        //#endregion 🔖️PartitionWindowMeasures

        //#region 🔖️WindowEngagement
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct WindowEngagementOption {
            pub id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub label: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub icon_id: Option<IconName>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub pressed: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub disabled: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub action: Option<ActionDescriptor>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct WindowEngagementInput {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub value: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub placeholder: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub disabled: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub on_change: Option<ActionDescriptor>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub on_submit: Option<ActionDescriptor>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub on_repeat_last: Option<ActionDescriptor>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub on_abort: Option<ActionDescriptor>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct WindowEngagementStatus {
            pub id: String,
            pub text: String,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct WindowEngagementPossible {
            pub id: String,
            pub label: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub detail: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub action: Option<ActionDescriptor>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct WindowEngagementRingOption {
            pub id: String,
            pub label: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub disabled: Option<bool>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct WindowEngagementToggleGroupOption {
            pub id: String,
            pub label: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub disabled: Option<bool>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct WindowEngagementSelectItem {
            pub id: String,
            pub value: String,
            pub label: String,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
        pub enum WindowEngagementControl {
            Slider {
                #[cfg_attr(feature = "typegen", ts(optional))]
                id: Option<String>,
                #[cfg_attr(feature = "typegen", ts(optional))]
                label: Option<String>,
                value: f64,
                min: f64,
                max: f64,
                #[cfg_attr(feature = "typegen", ts(optional))]
                step: Option<f64>,
                #[cfg_attr(feature = "typegen", ts(optional))]
                unit: Option<String>,
                #[cfg_attr(feature = "typegen", ts(optional))]
                disabled: Option<bool>,
                #[cfg_attr(feature = "typegen", ts(optional, rename = "onChange"))]
                on_change: Option<ActionDescriptor>,
                #[cfg_attr(feature = "typegen", ts(optional, rename = "onCommit"))]
                on_commit: Option<ActionDescriptor>,
            },
            Stepper {
                #[cfg_attr(feature = "typegen", ts(optional))]
                id: Option<String>,
                #[cfg_attr(feature = "typegen", ts(optional))]
                label: Option<String>,
                value: f64,
                #[cfg_attr(feature = "typegen", ts(optional))]
                min: Option<f64>,
                #[cfg_attr(feature = "typegen", ts(optional))]
                max: Option<f64>,
                #[cfg_attr(feature = "typegen", ts(optional))]
                step: Option<f64>,
                #[cfg_attr(feature = "typegen", ts(optional))]
                unit: Option<String>,
                #[cfg_attr(feature = "typegen", ts(optional))]
                disabled: Option<bool>,
                #[cfg_attr(feature = "typegen", ts(optional, rename = "onChange"))]
                on_change: Option<ActionDescriptor>,
                #[cfg_attr(feature = "typegen", ts(optional, rename = "onCommit"))]
                on_commit: Option<ActionDescriptor>,
            },
            Ring {
                #[cfg_attr(feature = "typegen", ts(optional))]
                id: Option<String>,
                #[cfg_attr(feature = "typegen", ts(optional))]
                label: Option<String>,
                #[cfg_attr(feature = "typegen", ts(optional))]
                value: Option<String>,
                options: Vec<WindowEngagementRingOption>,
                #[cfg_attr(feature = "typegen", ts(optional))]
                disabled: Option<bool>,
                #[cfg_attr(feature = "typegen", ts(optional, rename = "onSelect"))]
                on_select: Option<ActionDescriptor>,
            },
            ToggleGroup {
                #[cfg_attr(feature = "typegen", ts(optional))]
                id: Option<String>,
                #[cfg_attr(feature = "typegen", ts(optional))]
                label: Option<String>,
                #[cfg_attr(feature = "typegen", ts(optional))]
                value: Option<String>,
                options: Vec<WindowEngagementToggleGroupOption>,
                #[cfg_attr(feature = "typegen", ts(optional))]
                disabled: Option<bool>,
                #[cfg_attr(feature = "typegen", ts(optional, rename = "onSelect"))]
                on_select: Option<ActionDescriptor>,
            },
            Select {
                #[cfg_attr(feature = "typegen", ts(optional))]
                id: Option<String>,
                #[cfg_attr(feature = "typegen", ts(optional))]
                label: Option<String>,
                #[cfg_attr(feature = "typegen", ts(optional))]
                value: Option<String>,
                #[cfg_attr(feature = "typegen", ts(optional))]
                placeholder: Option<String>,
                items: Vec<WindowEngagementSelectItem>,
                #[cfg_attr(feature = "typegen", ts(optional))]
                disabled: Option<bool>,
                #[cfg_attr(feature = "typegen", ts(optional, rename = "onChange"))]
                on_change: Option<ActionDescriptor>,
            },
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct WindowEngagement {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub session_active: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub options: Option<Vec<WindowEngagementOption>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub input: Option<WindowEngagementInput>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub control: Option<WindowEngagementControl>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub controls: Option<Vec<WindowEngagementControl>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub status: Option<Vec<WindowEngagementStatus>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub possible_engagements: Option<Vec<WindowEngagementPossible>>,
        }

        /// 🤝️ Closed replacement for `Option<WindowEngagement>` — makes "this window kind never engages" a
        /// named variant instead of `None`, so absence is an explicit, typed state rather than an implicit gap.
        /// ⚠️ `WindowEngagement` is a wide variant (nested `Vec`/`Option` fields), making `Some` far
        /// larger than `None` — boxing it would be a breaking public-API change (every construction/match
        /// site across ~30 plugins would need `Box::new`/deref updates), out of scope for a mechanical pass.
        #[allow(clippy::large_enum_variant, reason = "boxing is a breaking public API change, out of T1 scope")]
        #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
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
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
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

            const GOLDEN_ACTION_DESCRIPTOR_JSON: &str = "[{\"controllerId\":\"ctrl\",\"action\":\"doThing\",\"args\":42},{\"controllerId\":\"ctrl\",\"action\":\"doOther\"},{\"variant\":\"primary\",\"size\":\"md\"}]";

            #[test]
            fn action_descriptor_and_style_spec_serialize_to_golden_json() {
                let values = (
                    ActionDescriptor { controller_id: "ctrl".into(), action: "doThing".into(), args: Some(DslValue::Number(42.0)) },
                    ActionDescriptor { controller_id: "ctrl".into(), action: "doOther".into(), args: None },
                    StyleSpec { variant: Some("primary".into()), size: Some("md".into()), density: None },
                );
                let json = serde_json::to_string(&values).unwrap();
                assert_eq!(json, GOLDEN_ACTION_DESCRIPTOR_JSON);
            }

            const GOLDEN_WINDOW_LAYOUT_JSON: &str = "{\"root\":{\"kind\":\"horizontal\",\"children\":[{\"kind\":\"stack\",\"size\":0.5,\"activeWindowKindId\":\"main\",\"children\":[{\"kind\":\"window\",\"windowKindId\":\"main\",\"title\":\"Main\"}]},{\"kind\":\"vertical\",\"size\":0.5,\"children\":[]}]}}";

            #[test]
            fn window_layout_serializes_to_golden_json() {
                let layout = WindowLayout {
                    root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
                        kind: "horizontal".into(),
                        size: None,
                        children: vec![
                            WindowLayoutChild::Stack(WindowLayoutStackNode {
                                kind: "stack".into(),
                                size: Some(0.5),
                                active_window_kind_id: Some("main".into()),
                                children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: "main".into(), title: Some("Main".into()), instance_id: None, template_id: None }],
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

            #[test]
            fn window_measure_serializes_to_golden_json() {
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

            #[test]
            fn partition_window_measures_unwraps_matching_utility_group_children_into_utility_options() {
                let measures = vec![utility_scoped_group("brush-params", Some("brush"), vec![measure_toggle("size")])];
                let (general, utility_options) = partition_window_measures(&measures, Some("brush"));
                assert!(general.is_empty());
                assert_eq!(utility_options.len(), 1);
                assert!(matches!(&utility_options[0], WindowMeasure::Toggle { id, .. } if id == "size"), "tagged wrapper is routing-only — children render flat");
            }

            #[test]
            fn partition_window_measures_drops_non_matching_utility_group_from_both_buckets() {
                let measures = vec![utility_scoped_group("brush-params", Some("brush"), vec![measure_toggle("size")])];
                let (general_other, utility_options_other) = partition_window_measures(&measures, Some("fill"));
                assert!(general_other.is_empty() && utility_options_other.is_empty(), "wrong active utility drops the group entirely");
                let (general_none, utility_options_none) = partition_window_measures(&measures, None);
                assert!(general_none.is_empty() && utility_options_none.is_empty(), "no active utility drops the group entirely");
            }

            #[test]
            fn partition_window_measures_keeps_untagged_group_and_non_group_in_general() {
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
                let (general, utility_options) = partition_window_measures(&measures, Some("brush"));
                assert_eq!(general.len(), 2, "untagged group and slider both stay general");
                assert!(utility_options.is_empty());
            }

            #[test]
            fn partition_window_measures_empty_input_roundtrips_to_empty() {
                let (general, utility_options) = partition_window_measures(&[], Some("brush"));
                assert!(general.is_empty() && utility_options.is_empty());
            }

            const GOLDEN_WINDOW_ENGAGEMENT_JSON: &str = "{\"sessionActive\":true,\"options\":[{\"id\":\"opt1\",\"label\":\"Option\",\"pressed\":false}],\"input\":{\"id\":\"in1\",\"value\":\"v\"},\"control\":{\"kind\":\"slider\",\"id\":\"sl1\",\"label\":null,\"value\":1.0,\"min\":0.0,\"max\":2.0,\"step\":null,\"unit\":null,\"disabled\":null,\"onChange\":null,\"onCommit\":null},\"status\":[{\"id\":\"st1\",\"text\":\"Ready\"}],\"possibleEngagements\":[{\"id\":\"pe1\",\"label\":\"Possible\"}]}";

            #[test]
            fn window_engagement_serializes_to_golden_json() {
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
        use crate::IconName;
        use dsl::DslValue;
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
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
        /// single source of truth for the utility bar — `DocumentApp::utilities` no longer exists.
        pub fn derive_utility_nodes(controller_id: &str, utilities: &[DerivedUtilitySpec], active_utility_id: Option<&str>) -> Vec<UtilityNode> {
            fn utility_toggle_node(controller_id: &str, utility: &DerivedUtilitySpec, active_utility_id: Option<&str>) -> UtilityNode {
                UtilityNode::Toggle {
                    id: utility.id.clone(),
                    icon_id: utility.icon_id.clone(),
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
                                icon_id: utility.icon_id.clone(),
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

            #[test]
            fn utility_node_serializes_to_golden_json() {
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

            #[test]
            fn derive_utility_nodes_marks_the_active_utility_pressed() {
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

            #[test]
            fn derive_utility_nodes_groups_shared_group_into_one_collection() {
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

            #[test]
            fn derive_utility_nodes_hoists_single_child_groups() {
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

    pub mod ui {
        // #region ui
        //! 🧩 Declarative UI graph types shared by kernel, plugins, and renderers.

        use crate::IconName;
        use crate::Label;
        use dsl::DslValue;
        use serde::{Deserialize, Serialize};
        use std::collections::HashMap;

        //#region 🔖Action
        pub use super::layout::{build_shell_context_menu_specs, organize_context_menu, ribbon_parent_label, ShellMenuAction, RIBBON_PARENT_CATEGORIES};
        pub use super::layout::{ActionDescriptor, StyleSpec, UiPresence, UiState, UiStatus};
        pub use super::layout::{ContextMenuHit, ContextMenuItemSpec, ContextMenuPoint, ContextMenuRequest, ContextMenuResponse, ContextMenuSelectionGroup, ContextMenuSurfaceTarget, ContextMenuTextContext, UiMenuRef};
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
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct UiDropOverlaySpec {
            pub title: Label,
            pub hint: Label,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub accept: Option<String>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct UiTextNode {
            pub value: Label,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub emphasize: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub data_attributes: Option<HashMap<String, String>>,
            #[serde(default, skip_serializing_if = "UiPresence::is_default")]
            #[cfg_attr(feature = "typegen", ts(as = "Option<UiPresence>", optional))]
            pub presence: UiPresence,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub menu: Option<UiMenuRef>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct UiButtonNode {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub id: Option<String>,
            pub icon_id: IconName,
            pub label: Label,
            pub action: ActionDescriptor,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub style: Option<StyleSpec>,
            #[serde(default, skip_serializing_if = "UiPresence::is_default")]
            #[cfg_attr(feature = "typegen", ts(as = "Option<UiPresence>", optional))]
            pub presence: UiPresence,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub menu: Option<UiMenuRef>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct UiSeparatorNode {
            #[serde(default, skip_serializing_if = "UiPresence::is_default")]
            #[cfg_attr(feature = "typegen", ts(as = "Option<UiPresence>", optional))]
            pub presence: UiPresence,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub menu: Option<UiMenuRef>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct UiImageNode {
            pub id: String,
            pub src: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub alt: Option<Label>,
            #[serde(default, skip_serializing_if = "UiPresence::is_default")]
            #[cfg_attr(feature = "typegen", ts(as = "Option<UiPresence>", optional))]
            pub presence: UiPresence,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub menu: Option<UiMenuRef>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct UiInputNode {
            pub id: String,
            pub input_kind: String,
            pub value: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub placeholder: Option<Label>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub commit: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub min: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub max: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub step: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub accept: Option<String>,
            pub on_change: ActionDescriptor,
            #[serde(default, skip_serializing_if = "UiPresence::is_default")]
            #[cfg_attr(feature = "typegen", ts(as = "Option<UiPresence>", optional))]
            pub presence: UiPresence,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub menu: Option<UiMenuRef>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct UiSelectItem {
            pub value: String,
            pub label: Label,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct UiSelectNode {
            pub id: String,
            pub value: String,
            pub items: Vec<UiSelectItem>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub placeholder: Option<Label>,
            pub on_change: ActionDescriptor,
            #[serde(default, skip_serializing_if = "UiPresence::is_default")]
            #[cfg_attr(feature = "typegen", ts(as = "Option<UiPresence>", optional))]
            pub presence: UiPresence,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub menu: Option<UiMenuRef>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct UiToggleNode {
            pub id: String,
            pub icon_id: IconName,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub text: Option<Label>,
            pub on_change: ActionDescriptor,
            #[serde(default, skip_serializing_if = "UiPresence::is_default")]
            #[cfg_attr(feature = "typegen", ts(as = "Option<UiPresence>", optional))]
            pub presence: UiPresence,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
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
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct UiKeyValueEntry {
            pub label: Label,
            pub value: String,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct UiKeyValueNode {
            pub entries: Vec<UiKeyValueEntry>,
            #[serde(default, skip_serializing_if = "UiPresence::is_default")]
            #[cfg_attr(feature = "typegen", ts(as = "Option<UiPresence>", optional))]
            pub presence: UiPresence,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub menu: Option<UiMenuRef>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct UiSliderNode {
            pub id: String,
            pub value: f64,
            pub min: f64,
            pub max: f64,
            pub step: f64,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub unit: Option<String>,
            pub on_change: ActionDescriptor,
            #[serde(default, skip_serializing_if = "UiPresence::is_default")]
            #[cfg_attr(feature = "typegen", ts(as = "Option<UiPresence>", optional))]
            pub presence: UiPresence,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub menu: Option<UiMenuRef>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct UiNumberStepperNode {
            pub id: String,
            pub value: f64,
            pub step: f64,
            pub uniform: bool,
            pub on_absolute: ActionDescriptor,
            pub on_delta: ActionDescriptor,
            #[serde(default, skip_serializing_if = "UiPresence::is_default")]
            #[cfg_attr(feature = "typegen", ts(as = "Option<UiPresence>", optional))]
            pub presence: UiPresence,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub menu: Option<UiMenuRef>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct UiRingNode {
            pub id: String,
            pub orb_id: String,
            pub t: f64,
            pub on_change: ActionDescriptor,
            #[serde(default, skip_serializing_if = "UiPresence::is_default")]
            #[cfg_attr(feature = "typegen", ts(as = "Option<UiPresence>", optional))]
            pub presence: UiPresence,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub menu: Option<UiMenuRef>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct UiIconSelectNode {
            pub id: String,
            pub value: String,
            pub uniform: bool,
            pub classifier_kind: String,
            pub on_change: ActionDescriptor,
            #[serde(default, skip_serializing_if = "UiPresence::is_default")]
            #[cfg_attr(feature = "typegen", ts(as = "Option<UiPresence>", optional))]
            pub presence: UiPresence,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub menu: Option<UiMenuRef>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
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
            pub fn presence(&self) -> UiPresence {
                match self {
                    UiControlNode::Input(n) => n.presence,
                    UiControlNode::Select(n) => n.presence,
                    UiControlNode::Toggle(n) => n.presence,
                    UiControlNode::Button(n) => n.presence,
                    UiControlNode::KeyValue(n) => n.presence,
                    UiControlNode::Slider(n) => n.presence,
                    UiControlNode::NumberStepper(n) => n.presence,
                    UiControlNode::Ring(n) => n.presence,
                    UiControlNode::IconSelect(n) => n.presence,
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
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub enum UiTreeActionPlacement {
            #[default]
            Row,
            Menu,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct UiTreeItemAction {
            pub icon_id: IconName,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub label: Option<Label>,
            pub action: ActionDescriptor,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub placement: Option<UiTreeActionPlacement>,
        }

        impl UiTreeItemAction {
            /** @emoji 📍️ Row actions paint on the tree header; menu actions belong in the row context menu. */
            pub fn placement(&self) -> UiTreeActionPlacement {
                self.placement.clone().unwrap_or_default()
            }
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct UiTreeItemNode {
            pub id: String,
            pub label: Label,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none", alias = "icon")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub icon_id: Option<IconName>,
            #[serde(default, skip_serializing_if = "UiPresence::is_default")]
            #[cfg_attr(feature = "typegen", ts(as = "Option<UiPresence>", optional))]
            pub presence: UiPresence,
            #[serde(skip_serializing_if = "Option::is_none", alias = "expanded")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub default_open: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub action: Option<ActionDescriptor>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub hover_action: Option<ActionDescriptor>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub unhover_action: Option<ActionDescriptor>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub actions: Option<Vec<UiTreeItemAction>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub draggable: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub drag_data: Option<HashMap<String, String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub items: Option<Vec<UiTreeItemNode>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub control: Option<UiControlNode>,
            /// 👁️ Domain "eye toggle" flag: the row stays visible, dimmed, and clickable (to un-hide) —
            /// this is NOT `presence.state == Hidden`, which means not rendered at all.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub dimmed: Option<bool>,
            /// 🖱️ Row-level context-menu address — most rows share one `menu.id` across a tree with the row
            /// id carried in `args` (e.g. `{"id": row.id}`), rather than minting a unique menu id per row.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
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
                    hover_action: None,
                    unhover_action: None,
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
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct UiTreeSectionNode {
            pub id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub label: Option<Label>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub default_open: Option<bool>,
            #[serde(default, skip_serializing_if = "UiPresence::is_default")]
            #[cfg_attr(feature = "typegen", ts(as = "Option<UiPresence>", optional))]
            pub presence: UiPresence,
            pub items: Vec<UiTreeItemNode>,
        }

        #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct UiTreeNode {
            pub sections: Vec<UiTreeSectionNode>,
            #[serde(default, skip_serializing_if = "UiPresence::is_default")]
            #[cfg_attr(feature = "typegen", ts(as = "Option<UiPresence>", optional))]
            pub presence: UiPresence,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub selected_ids: Option<Vec<String>>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub highlighted_ids: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub selection_change: Option<ActionDescriptor>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub drop_action: Option<ActionDescriptor>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
            pub menu: Option<UiMenuRef>,
        }

        /// 🖌️ Stamps `selected`/`previewed` per-item presence across every item in every section of a
        /// tree, replacing the old id-list (`selected_ids`/`highlighted_ids`) approach — the one-line
        /// migration for plugins that held id sets. `previewed` wins visually over a plain `selected` item
        /// only insofar as both are representable simultaneously (an item can be selected AND previewed).
        pub fn ui_tree_stamp_presence(sections: &mut [UiTreeSectionNode], selected: &std::collections::HashSet<String>, previewed: &std::collections::HashSet<String>) {
            fn stamp_items(items: &mut [UiTreeItemNode], selected: &std::collections::HashSet<String>, previewed: &std::collections::HashSet<String>) {
                for item in items {
                    item.presence.selected = selected.contains(&item.id);
                    if previewed.contains(&item.id) {
                        item.presence.state = UiState::Previewed;
                    }
                    if let Some(children) = &mut item.items {
                        stamp_items(children, selected, previewed);
                    }
                }
            }
            for section in sections {
                stamp_items(&mut section.items, selected, previewed);
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
            let tree_sections: Vec<UiTreeSectionNode> = sections
                .iter()
                .map(|section| UiTreeSectionNode {
                    id: section.id.clone(),
                    label: section.label.clone(),
                    default_open: Some(section.default_open.unwrap_or(true)),
                    presence: section.presence,
                    items: section.children.iter().enumerate().map(|(index, child)| ui_declarative_child_to_tree_item(child, format!("{}.{}", section.id, index))).collect(),
                })
                .collect();
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
                            hover_action: None,
                            unhover_action: None,
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
                    selected_ids: None,
                    highlighted_ids: None,
                    selection_change: None,
                    drop_action: None,
                }
            } else {
                UiTreeNode { menu: None, sections: tree_sections, presence: UiPresence::default(), selected_ids: None, highlighted_ids: None, selection_change: None, drop_action: None }
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
                    hover_action: None,
                    unhover_action: None,
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
                        hover_action: None,
                        unhover_action: None,
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
                    hover_action: None,
                    unhover_action: None,
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
                UiNode::Group(group) => UiTreeItemNode {
                    menu: None,
                    id: group.id.clone(),
                    label: group.label.clone(),
                    description: None,
                    icon_id: None,
                    presence: UiPresence::default(),
                    default_open: group.default_open,
                    action: None,
                    hover_action: None,
                    unhover_action: None,
                    actions: None,
                    draggable: None,
                    drag_data: None,
                    items: Some(group.children.iter().enumerate().map(|(index, child)| ui_declarative_child_to_tree_item(child, format!("{}.{}", group.id, index))).collect()),
                    control: None,
                    dimmed: None,
                },
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
                    hover_action: None,
                    unhover_action: None,
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
                    hover_action: None,
                    unhover_action: None,
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
                hover_action: None,
                unhover_action: None,
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
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
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

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Canvas2dScene {
            pub camera_x: f64,
            pub camera_y: f64,
            pub zoom: f64,
            pub layers_json: String,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct World3dScene {
            pub camera_json: String,
            #[serde(default = "world3d_default_meshes_json")]
            pub meshes_json: String,
            pub instances_json: String,
            #[serde(default = "world3d_default_selection_json")]
            pub selection_json: String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub vortices_json: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub attractions_json: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub target_volumes_json: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub references_json: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub brush_preview_json: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub interaction_json: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub engagement_preview_json: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub lod_json: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub chunking_json: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub environment_json: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub frame_json: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub fit_json: Option<String>,
            /// 🌐️⛰️ GIS 3D terrain style/source descriptor (`{tileUrlTemplate, projectOriginLon, projectOriginLat, exaggeration, colorRamp, minZoom, maxZoom}`), consumed by `WorldTerrainLayer`.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub terrain_json: Option<String>,
            /// ☁️ Point-cloud rendering layers, distinct from `meshes_json`'s per-point-mesh path — cheap for
            /// 10^5-10^6 points. An array of `{id, positionsB64, colorsB64?, size, sizeAttenuation}` where
            /// `positionsB64` is base64 of little-endian f32 xyz interleaved and `colorsB64` (optional) is
            /// base64 of u8 rgb interleaved, one per point. Consumed by `WorldPointCloudLayer`.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub points_json: Option<String>,
            /// ⏳️ Off-main-thread compute status (`{"computing": true, "label": "…"}`) shown as an overlay
            /// while a `flowEvalTick` chain is still resolving the meshes this scene renders — the meshes
            /// themselves stay the last-known-good (stale) cache until the chain completes.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub status_json: Option<String>,
        }

        impl World3dScene {
            /** @emoji 🌐️ Builds a world-3d scene with optional extensions unset. */
            pub fn base(camera_json: String, meshes_json: String, instances_json: String, selection_json: String) -> Self {
                Self {
                    camera_json,
                    meshes_json,
                    instances_json,
                    selection_json,
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
                }
            }
        }

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

        fn default_manual_lod() -> f64 {
            100.0
        }

        fn default_distance_reference() -> f64 {
            100.0
        }

        fn default_grid_factor() -> f64 {
            10.0
        }

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

        pub fn world3d_default_selection_json() -> String {
            r#"{"method":"rectangle","mode":"replace","ids":[],"hoveredId":null}"#.into()
        }

        pub fn world3d_default_meshes_json() -> String {
            "[]".into()
        }

        pub fn world3d_camera_json(position: [f64; 3], target: [f64; 3], fov: f64) -> String {
            serde_json::json!({
                "position": position,
                "target": target,
                "up": [0.0, 0.0, 1.0],
                "fov": fov,
            })
            .to_string()
        }

        //#region 🔖️NodeGraphRecords
        /// 🔌️ One port on a node-graph node: identity + display label. Direction is implied by whether the
        /// record lives in the owning node's `inputs` or `outputs` list, not carried as a field. `code`/
        /// `abbreviation`/`fullName`/`artifactKind` (wire key still `resourceKind` — the rename to
        /// `artifactKind` is W4/`OsWorkflowNodeGraphPayload` scope, not this ticket's) are set only for
        /// OS-workflow app-instance nodes; see `framework/surface/node-graph`'s `GraphPortRecord`, which this mirrors.
        #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct NodeGraphPortRecord {
            pub id: String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub label: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub code: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub abbreviation: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub full_name: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceKind")]
            pub artifact_kind: Option<String>,
        }

        /// 🕸️ One node-graph node: identity, label, layout rect, typed input/output ports.
        #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct NodeGraphNodeRecord {
            pub id: String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub label: Option<String>,
            pub x: f64,
            pub y: f64,
            pub width: f64,
            pub height: f64,
            #[serde(default)]
            pub inputs: Vec<NodeGraphPortRecord>,
            #[serde(default)]
            pub outputs: Vec<NodeGraphPortRecord>,
            /// 🪐️ Set only for OS-workflow app-instance nodes (the space canvas's node-graph rides a richer
            /// node shape than the generic plugin producers) — see `framework/surface/node-graph`'s
            /// `GraphNodeRecord`, which this mirrors, and `NodeGraphError`-free `DagNodeKind::AppInstance` wiring.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub instance_id: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub plugin_id: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub app_id: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub icon: Option<String>,
        }

        /// 🕸️ One node-graph edge between two node/port endpoints.
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct NodeGraphEdgeRecord {
            pub id: String,
            pub source_node_id: String,
            pub source_port_id: String,
            pub target_node_id: String,
            pub target_port_id: String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub label: Option<String>,
        }

        /// 📷️ Node-graph camera: pan position + zoom factor.
        #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct NodeGraphViewport {
            #[serde(default)]
            pub x: f64,
            #[serde(default)]
            pub y: f64,
            #[serde(default = "default_true_zoom")]
            pub zoom: f64,
        }

        fn default_true_zoom() -> f64 {
            1.0
        }

        /// 🔎️ One spotlight/find result row for a node-graph surface.
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct NodeGraphFindItem {
            pub id: String,
            pub label: String,
            pub category: String,
        }

        /// 🖱️ Hovered node id, if any.
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct NodeGraphHover {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub node_id: Option<String>,
        }

        /// ➕️ Variadic input/output slot on an operator catalogue entry (mirrors neural engine's `VariadicSpec` wire shape).
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct NodeGraphOperatorVariadicRecord {
            pub slot_key: String,
            pub min: usize,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub max: Option<usize>,
        }

        /// 🔌️ Declared operator channel (input or output), mirrors neural engine's `ChannelSpec` wire shape —
        /// `cardinality` rides as its already-serialized symbol string (`"!"`/`"?"`/`"*"`/`"+"`/digits).
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct NodeGraphOperatorChannelRecord {
            pub code: String,
            pub abbreviation: String,
            pub name: String,
            pub full_name: String,
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub operators: Vec<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub default: Option<serde_json::Value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub label: Option<String>,
            #[serde(default)]
            pub cardinality: String,
        }

        /// 🧠️ One operator catalogue entry offered to a flow-backed node-graph's spotlight/palette, mirrors
        /// neural engine's `OperatorInfo` wire shape (kept as a local mirror: `ui_wgpu` sits below the neural
        /// engine crate in the dependency graph, so it cannot import that type directly).
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct NodeGraphOperatorRecord {
            pub id: String,
            pub extension: String,
            pub name: String,
            pub abbreviation: String,
            pub icon: String,
            pub summary: String,
            #[serde(default)]
            pub inputs: Vec<NodeGraphOperatorChannelRecord>,
            #[serde(default)]
            pub outputs: Vec<NodeGraphOperatorChannelRecord>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub variadic_input: Option<NodeGraphOperatorVariadicRecord>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub variadic_output: Option<NodeGraphOperatorVariadicRecord>,
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub group: Vec<String>,
        }
        //#endregion 🔖️NodeGraphRecords

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct NodeGraphScene {
            #[serde(default)]
            pub nodes: Vec<NodeGraphNodeRecord>,
            #[serde(default)]
            pub edges: Vec<NodeGraphEdgeRecord>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub viewport: Option<NodeGraphViewport>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub editable: Option<bool>,
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub operators: Vec<NodeGraphOperatorRecord>,
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub find_items: Vec<NodeGraphFindItem>,
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub selection: Vec<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub hover: Option<NodeGraphHover>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub preview_off_json: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub lod_json: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub catalogue_json: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub controls_json: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub clusters_json: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub computing_json: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub status_json: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub capabilities_json: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub fixture_json: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub presence_peers_json: Option<String>,
            /// 🧵️ Channel-structured eval outputs from an off-main-thread `flowEvalTick` chain, applied via
            /// `FlowSession::applyEvalOutputsJson` — lets a view-only `FlowHost` (e.g. a renderer's canvas
            /// session) pick up results without ever calling `evaluate` itself.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub eval_json: Option<String>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct TextEditorScene {
            pub buffer: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub language: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub selection_json: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub tokens_json: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub diagnostics_json: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub completions_json: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub overlays_json: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub occurrences_json: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub placeholders_json: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub extra_carets_json: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub selectable_spans_json: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub settings_json: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub camera_json: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub hover_json: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub newline_gates_json: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub rename_json: Option<String>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct TableScene {
            pub columns_json: String,
            pub rows_json: String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub selection_json: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub row_drag_mime: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub drop_action: Option<ActionDescriptor>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub sort_json: Option<String>,
        }

        impl TableScene {
            /** @emoji 📋️ Builds a table scene with optional extensions (selection/drag/sort) unset. */
            pub fn base(columns_json: impl Into<String>, rows_json: impl Into<String>) -> Self {
                Self { columns_json: columns_json.into(), rows_json: rows_json.into(), selection_json: None, row_drag_mime: None, drop_action: None, sort_json: None }
            }
        }

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

        /** @emoji 🖼️ Paint-2d scene: WASM `RasterSession` sync channels for the composite/navigator windows, see framework/surface/paint/rs/lib.rs. */
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Paint2dScene {
            pub document_sync_json: String,
            pub assets_json: String,
            pub camera_json: String,
            pub selection_json: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub hovered_id: Option<String>,
            pub active_utility: String,
            pub brush_size: f64,
            pub brush_opacity: f64,
            pub view_mode: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub composite_viewport_json: Option<String>,
        }

        /** @emoji 🖼️ Icon-render scene: client-side render request for a shot preview, see https://threejs.org/docs/#examples/en/renderers/SVGRenderer. */
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct IconRenderScene {
            pub request_json: String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub footer: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub frame_json: Option<String>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct VirtualFileSystemScene {
            pub schema_json: String,
            pub rows_json: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub selected_row_ids_json: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub hovered_row_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub empty_message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub drag_drop_enabled: Option<bool>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct TiledMapScene {
            pub map_fixture_json: String,
            pub camera_json: String,
            #[serde(default = "tiled_map_default_render_mode")]
            pub render_mode: String,
            #[serde(default = "tiled_map_default_vector_style")]
            pub vector_style: String,
            #[serde(default = "tiled_map_default_lod_mode")]
            pub lod_mode: String,
            #[serde(default = "tiled_map_default_tile_url_template")]
            pub tile_url_template: String,
            #[serde(default = "tiled_map_default_vector_tile_url_template")]
            pub vector_tile_url_template: String,
            #[serde(default = "tiled_map_default_layer_visibility_json")]
            pub layer_visibility_json: String,
            #[serde(default = "tiled_map_default_layer_stroke_scale_json")]
            pub layer_stroke_scale_json: String,
            #[serde(default = "tiled_map_default_selection_json")]
            pub selection_json: String,
            #[serde(default = "tiled_map_default_hover_json")]
            pub hover_json: String,
            #[serde(default = "tiled_map_default_selection_method")]
            pub selection_method: String,
            #[serde(default = "tiled_map_default_selection_mode")]
            pub selection_mode: String,
        }

        pub fn tiled_map_default_render_mode() -> String {
            "combined".into()
        }

        pub fn tiled_map_default_vector_style() -> String {
            "colored".into()
        }

        pub fn tiled_map_default_lod_mode() -> String {
            "automatic".into()
        }

        pub fn tiled_map_default_tile_url_template() -> String {
            "/osm/{z}/{x}/{y}.png".into()
        }

        pub fn tiled_map_default_vector_tile_url_template() -> String {
            "/vt/{z}/{x}/{y}.pbf".into()
        }

        /** 🗺️ Empty layer-visibility gate map: the owning plugin's engine defaults every layer id it recognizes to visible, so the framework need not enumerate app-specific layer ids. */
        pub fn tiled_map_default_layer_visibility_json() -> String {
            "{}".into()
        }

        /** 🗺️ Empty layer-stroke-scale multiplier map: the owning plugin's engine defaults every layer id it recognizes to a 1.0 multiplier, so the framework need not enumerate app-specific layer ids. */
        pub fn tiled_map_default_layer_stroke_scale_json() -> String {
            "{}".into()
        }

        /** 🗺️ Empty selection: the owning plugin's engine treats a missing selection key as "none selected", so the framework need not encode app-specific feature categories. */
        pub fn tiled_map_default_selection_json() -> String {
            "{}".into()
        }

        pub fn tiled_map_default_hover_json() -> String {
            "null".into()
        }

        pub fn tiled_map_default_selection_method() -> String {
            "rectangle".into()
        }

        pub fn tiled_map_default_selection_mode() -> String {
            "default".into()
        }

        impl TiledMapScene {
            /** @emoji 🗺️ Builds a tiled map scene with optional extensions unset. */
            pub fn base(map_fixture_json: String, camera_json: String) -> Self {
                Self {
                    map_fixture_json,
                    camera_json,
                    render_mode: tiled_map_default_render_mode(),
                    vector_style: tiled_map_default_vector_style(),
                    lod_mode: tiled_map_default_lod_mode(),
                    tile_url_template: tiled_map_default_tile_url_template(),
                    vector_tile_url_template: tiled_map_default_vector_tile_url_template(),
                    layer_visibility_json: tiled_map_default_layer_visibility_json(),
                    layer_stroke_scale_json: tiled_map_default_layer_stroke_scale_json(),
                    selection_json: tiled_map_default_selection_json(),
                    hover_json: tiled_map_default_hover_json(),
                    selection_method: tiled_map_default_selection_method(),
                    selection_mode: tiled_map_default_selection_mode(),
                }
            }
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Board2dScene {
            pub fixture_json: String,
            pub camera_json: String,
            #[serde(default = "board2d_default_glyph_catalogs_json")]
            pub glyph_catalogs_json: String,
            #[serde(default = "board2d_default_selection_json")]
            pub selection_json: String,
            #[serde(default)]
            pub interactive: bool,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub hovered_id: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub active_utility: Option<String>,
            #[serde(default = "board2d_default_selection_method")]
            pub selection_method: String,
            #[serde(default)]
            pub grid_snap_enabled: bool,
            #[serde(default = "board2d_default_grid_factor")]
            pub grid_factor: f64,
            #[serde(default)]
            pub suggestion_offset: f64,
            #[serde(default = "board2d_default_brush_weights_json")]
            pub brush_weights_json: String,
            #[serde(default = "board2d_default_placement_compatibility_json")]
            pub placement_compatibility_json: String,
            #[serde(default = "board2d_default_lod_mode")]
            pub lod_mode: String,
        }

        pub fn board2d_default_glyph_catalogs_json() -> String {
            "{}".into()
        }

        pub fn board2d_default_selection_json() -> String {
            "[]".into()
        }

        pub fn board2d_default_selection_method() -> String {
            "rectangle".into()
        }

        pub fn board2d_default_grid_factor() -> f64 {
            1.0
        }

        pub fn board2d_default_brush_weights_json() -> String {
            "{}".into()
        }

        pub fn board2d_default_placement_compatibility_json() -> String {
            "[]".into()
        }

        pub fn board2d_default_lod_mode() -> String {
            "automatic".into()
        }

        impl Board2dScene {
            /** @emoji 🧩️ Builds a 2D board scene with optional extensions unset. */
            pub fn base(fixture_json: String, camera_json: String, interactive: bool) -> Self {
                Self {
                    fixture_json,
                    camera_json,
                    glyph_catalogs_json: board2d_default_glyph_catalogs_json(),
                    selection_json: board2d_default_selection_json(),
                    interactive,
                    hovered_id: None,
                    active_utility: None,
                    selection_method: board2d_default_selection_method(),
                    grid_snap_enabled: false,
                    grid_factor: board2d_default_grid_factor(),
                    suggestion_offset: 0.0,
                    brush_weights_json: board2d_default_brush_weights_json(),
                    placement_compatibility_json: board2d_default_placement_compatibility_json(),
                    lod_mode: board2d_default_lod_mode(),
                }
            }
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct InkCanvasScene {
            pub document_json: String,
            #[serde(default = "ink_canvas_default_selection_json")]
            pub selection_json: String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub hovered_id: Option<String>,
            pub active_utility: String,
            pub view_mode: String,
            #[serde(default)]
            pub interactive: bool,
        }

        pub fn ink_canvas_default_selection_json() -> String {
            "[]".into()
        }

        impl InkCanvasScene {
            /** @emoji 🖊️ Builds an ink canvas scene with the default empty selection. */
            pub fn base(document_json: String, active_utility: String, view_mode: String, interactive: bool) -> Self {
                Self { document_json, selection_json: ink_canvas_default_selection_json(), hovered_id: None, active_utility, view_mode, interactive }
            }
        }

        /** @emoji 🗄️ A checkpoint ancestor-graph history view. `columns_json` is a `HistoryColumn[]` array
         * (see `store::HistoryColumn`), newest checkpoint first. */
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct GraphTimelineScene {
            pub columns_json: String,
        }

        /** @emoji 🆚️ A before/after text comparison. `mode` picks the renderer's layout (`"unified"` inline
         * hunks or `"split"` side-by-side panes); `language` is an optional syntax-highlighting hint. */
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct DiffViewScene {
            pub before: String,
            pub after: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub language: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub mode: Option<String>,
        }

        /** @emoji 📰️ A chronological feed of host-authored events. `entries_json` is a
         * `{id, timestampMs, iconId, title, detail?, tone?}[]` array; `activate_action` (if set) is the
         * action name fired with the clicked entry's `id` when an entry is activated. */
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct EventFeedScene {
            pub entries_json: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub follow: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub activate_action: Option<String>,
        }

        /** @emoji 🧩️ A palette entry for a block kind insertable into a [`BlockListScene`], contributed
         * either by the host app's own built-ins or by a `Contribution::PlaybookBlockKind` module. */
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct BlockPaletteEntry {
            pub block_kind: String,
            pub label: String,
            pub icon_id: IconName,
        }

        /** @emoji 🧩️ A strict, ordered list of steps/blocks for the Blockly-like list editor. `steps_json`
         * is a `PlaybookStep[]` array (see `playbook::PlaybookStep`), `palette_json` is a
         * `BlockPaletteEntry[]` array of the block kinds available to insert. */
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct BlockListScene {
            pub steps_json: String,
            pub palette_json: String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub selected_id: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub dragging_id: Option<String>,
        }

        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
        #[serde(rename_all = "camelCase")]
        pub struct UiExternalSlotNode {
            pub plugin_id: String,
            pub app_id: String,
            pub body_key: String,
            pub params_json: String,
            #[serde(default, skip_serializing_if = "UiPresence::is_default")]
            #[cfg_attr(feature = "typegen", ts(as = "Option<UiPresence>", optional))]
            pub presence: UiPresence,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[cfg_attr(feature = "typegen", ts(optional))]
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
            pub fn presence(&self) -> UiPresence {
                match self {
                    UiNode::Stack(n) => n.presence,
                    UiNode::Text(n) => n.presence,
                    UiNode::Button(n) => n.presence,
                    UiNode::Separator(n) => n.presence,
                    UiNode::Input(n) => n.presence,
                    UiNode::Select(n) => n.presence,
                    UiNode::Toggle(n) => n.presence,
                    UiNode::KeyValue(n) => n.presence,
                    UiNode::Slider(n) => n.presence,
                    UiNode::NumberStepper(n) => n.presence,
                    UiNode::Ring(n) => n.presence,
                    UiNode::IconSelect(n) => n.presence,
                    UiNode::Field(n) => n.presence,
                    UiNode::Section(n) => n.presence,
                    UiNode::Group(n) => n.presence,
                    UiNode::Tree(n) => n.presence,
                    UiNode::Image(n) => n.presence,
                    UiNode::ComponentScene(n) => n.presence,
                    UiNode::ExternalSlot(n) => n.presence,
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

        impl NodeGraphScene {
            /** @emoji 🕸️ Builds a node-graph scene with optional extensions unset. */
            pub fn base(nodes: Vec<NodeGraphNodeRecord>, edges: Vec<NodeGraphEdgeRecord>, viewport: NodeGraphViewport) -> Self {
                Self {
                    nodes,
                    edges,
                    viewport: Some(viewport),
                    editable: None,
                    operators: Vec::new(),
                    find_items: Vec::new(),
                    selection: Vec::new(),
                    hover: None,
                    preview_off_json: None,
                    lod_json: None,
                    catalogue_json: None,
                    controls_json: None,
                    clusters_json: None,
                    computing_json: None,
                    capabilities_json: None,
                    fixture_json: None,
                    presence_peers_json: None,
                    eval_json: None,
                    status_json: None,
                }
            }
        }

        impl TextEditorScene {
            /** @emoji ✍️ Builds a text-editor scene with optional extensions unset. */
            pub fn base(buffer: String, language: Option<String>, selection_json: Option<String>) -> Self {
                Self {
                    buffer,
                    language,
                    selection_json,
                    tokens_json: None,
                    diagnostics_json: None,
                    completions_json: None,
                    overlays_json: None,
                    occurrences_json: None,
                    placeholders_json: None,
                    extra_carets_json: None,
                    selectable_spans_json: None,
                    settings_json: None,
                    camera_json: None,
                    hover_json: None,
                    newline_gates_json: None,
                    rename_json: None,
                }
            }

            /** @emoji 📖️ Builds a read-only JSON viewer scene: a pretty-printed JSON buffer, `"json"`
             * language, and `settingsJson` set to `{"readOnly":true}`. */
            pub fn json_view(json_pretty: String) -> Self {
                let mut scene = Self::base(json_pretty, Some("json".into()), None);
                scene.settings_json = Some(serde_json::json!({ "readOnly": true }).to_string());
                scene
            }

            /** @emoji ⌨️ Builds an editable code-input scene wired to a host settings-change action:
             * `settingsJson` carries `{"readOnly":false,"onEditSettings":<ActionDescriptor>}`, fired by the
             * renderer when the user edits editor settings (font size, tab width, ...) via its own chrome. */
            pub fn code_input(buffer: String, language: &str, on_edit_settings: &ActionDescriptor) -> Self {
                let mut scene = Self::base(buffer, Some(language.into()), None);
                scene.settings_json = Some(serde_json::json!({ "readOnly": false, "onEditSettings": on_edit_settings }).to_string());
                scene
            }
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
            fn default() -> Self {
                ui_stack_vertical(vec![])
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
            if start == index {
                None
            } else {
                Some((start, index))
            }
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
                            selected_ids: None,
                            highlighted_ids: None,
                            selection_change: None,
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

            #[test]
            fn ui_node_tree_serializes_to_golden_json() {
                let node = sample_tree();
                let json = serde_json::to_string(&node).unwrap();
                assert_eq!(json, GOLDEN_UI_NODE_TREE_JSON, "UiNode wire format drifted \u{2014} lock this in before moving the type into ui_wgpu");
                let roundtripped: UiNode = serde_json::from_str(&json).unwrap();
                assert_eq!(roundtripped, node);
            }

            /// 🌀️ `presence.status` follows the same skip-if-default convention as `presence.selected`: the whole `presence` key is absent when fully default, and round-trips when set.
            #[test]
            fn ui_tree_item_loading_status_skips_when_default_and_roundtrips_when_set() {
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
            #[test]
            fn ui_tree_item_waiting_status_skips_when_default_and_roundtrips_when_set() {
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
            #[test]
            fn ui_tree_item_hidden_state_roundtrips() {
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
            #[test]
            fn ui_tree_item_celebrating_state_roundtrips() {
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
            #[test]
            fn every_ui_node_variant_serializes_a_non_default_presence() {
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
                assert_presence_serializes(UiNode::Tree(UiTreeNode { menu: None, sections: vec![], presence: UiPresence::default(), selected_ids: None, highlighted_ids: None, selection_change: None, drop_action: None }), "Tree");
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
            #[test]
            fn world_3d_scene_points_json_skips_when_none_and_roundtrips_when_set() {
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

            #[test]
            fn surface_kind_serializes_to_golden_json() {
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

            const GOLDEN_SCENES_JSON: &str = "[{\"cameraX\":1.0,\"cameraY\":2.0,\"zoom\":1.5,\"layersJson\":\"[]\"},{\"columnsJson\":\"[]\",\"rowsJson\":\"[]\"},{\"documentSyncJson\":\"{}\",\"assetsJson\":\"[]\",\"cameraJson\":\"{}\",\"selectionJson\":\"[]\",\"hoveredId\":\"h1\",\"activeUtility\":\"brush\",\"brushSize\":4.0,\"brushOpacity\":1.0,\"viewMode\":\"composite\"},{\"requestJson\":\"{}\"},{\"schemaJson\":\"{}\",\"rowsJson\":\"[]\",\"emptyMessage\":\"Empty\",\"dragDropEnabled\":true},{\"mapFixtureJson\":\"{}\",\"cameraJson\":\"{}\",\"renderMode\":\"combined\",\"vectorStyle\":\"colored\",\"lodMode\":\"automatic\",\"tileUrlTemplate\":\"/osm/{z}/{x}/{y}.png\",\"vectorTileUrlTemplate\":\"/vt/{z}/{x}/{y}.pbf\",\"layerVisibilityJson\":\"{}\",\"layerStrokeScaleJson\":\"{}\",\"selectionJson\":\"{}\",\"hoverJson\":\"null\",\"selectionMethod\":\"rectangle\",\"selectionMode\":\"default\"},{\"fixtureJson\":\"{}\",\"cameraJson\":\"{}\",\"glyphCatalogsJson\":\"{}\",\"selectionJson\":\"[]\",\"interactive\":true,\"selectionMethod\":\"rectangle\",\"gridSnapEnabled\":false,\"gridFactor\":1.0,\"suggestionOffset\":0.0,\"brushWeightsJson\":\"{}\",\"placementCompatibilityJson\":\"[]\",\"lodMode\":\"automatic\"},{\"documentJson\":\"{}\",\"selectionJson\":\"[]\",\"activeUtility\":\"select\",\"viewMode\":\"edit\",\"interactive\":true},{\"columnsJson\":\"[]\"},{\"nodesJson\":\"[]\",\"edgesJson\":\"[]\",\"viewportJson\":\"{}\"},{\"buffer\":\"buf\",\"language\":\"rust\"},{\"stepsJson\":\"[]\",\"paletteJson\":\"[]\"}]";

            #[test]
            fn scene_records_serialize_to_golden_json() {
                let scenes = (
                    Canvas2dScene { camera_x: 1.0, camera_y: 2.0, zoom: 1.5, layers_json: "[]".into() },
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
                    NodeGraphScene::base("[]".into(), "[]".into(), "{}".into()),
                    TextEditorScene::base("buf".into(), Some("rust".into()), None),
                    BlockListScene { steps_json: "[]".into(), palette_json: "[]".into(), selected_id: None, dragging_id: None },
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

            #[test]
            fn diff_view_and_event_feed_scenes_serialize_to_golden_json() {
                let scenes =
                    (DiffViewScene { before: "a".into(), after: "b".into(), language: Some("rust".into()), mode: Some("unified".into()) }, EventFeedScene { entries_json: "[]".into(), follow: Some(true), activate_action: Some("openEvent".into()) });
                let json = serde_json::to_string(&scenes).unwrap();
                assert_eq!(json, GOLDEN_DIFF_VIEW_EVENT_FEED_SCENES_JSON);
                let roundtripped: (DiffViewScene, EventFeedScene) = serde_json::from_str(&json).unwrap();
                assert_eq!(roundtripped, scenes);
            }

            /// 🖱️ `UiMenuRef`/`ContextMenuItemSpec` camelCase wire shape — in particular `hover_args` must
            /// serialize as `hoverArgs` (the exact field-rename pitfall documented on `UiDirtyScope`).
            #[test]
            fn ui_menu_ref_and_context_menu_item_spec_roundtrip_camel_case() {
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
            #[test]
            fn every_ui_node_variant_serializes_a_set_menu_ref() {
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
                assert_menu_serializes(UiNode::Text(UiTextNode { menu: None, value: "x".into(), emphasize: None, data_attributes: None, presence: UiPresence::default() }), "Text");
                assert_menu_serializes(UiNode::Button(UiButtonNode { menu: None, id: None, icon_id: IconName::CircleDot, label: "l".into(), action: act("a"), style: None, presence: UiPresence::default() }), "Button");
                assert_menu_serializes(UiNode::Separator(UiSeparatorNode { menu: None, presence: UiPresence::default() }), "Separator");
                assert_menu_serializes(UiNode::Image(UiImageNode { menu: None, id: "i".into(), src: "s".into(), alt: None, presence: UiPresence::default() }), "Image");
                assert_menu_serializes(UiNode::Tree(UiTreeNode { menu: None, sections: vec![], presence: UiPresence::default(), selected_ids: None, highlighted_ids: None, selection_change: None, drop_action: None }), "Tree");
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

            fn no_category(_id: &str) -> Option<String> {
                None
            }

            #[test]
            fn organize_context_menu_emits_as_is_within_budget() {
                let items = vec![menu_leaf("a"), menu_leaf("b"), menu_group("view", vec![menu_leaf("c")])];
                let organized = organize_context_menu(items.clone(), &no_category);
                assert_eq!(organized, items, "within budget with leaves already before groups, nothing is reordered: {organized:?}");
            }

            #[test]
            fn organize_context_menu_puts_destructive_leaves_last_after_a_separator() {
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

            #[test]
            fn organize_context_menu_merges_same_id_groups_and_dedupes_children_by_id() {
                let items = vec![menu_group("view", vec![menu_leaf("zoomIn"), menu_leaf("zoomOut")]), menu_leaf("a"), menu_group("view", vec![menu_leaf("zoomOut"), menu_leaf("resetZoom")])];
                let organized = organize_context_menu(items, &no_category);
                assert_eq!(organized.iter().filter(|item| item.id == "menu.group.view").count(), 1, "only one merged row remains: {organized:?}");
                let view_group = organized.iter().find(|item| item.id == "menu.group.view").expect("merged view group present");
                let child_ids: Vec<&str> = view_group.children.as_ref().unwrap().iter().map(|child| child.id.as_str()).collect();
                assert_eq!(child_ids, vec!["zoomIn", "zoomOut", "resetZoom"], "children concat in first-seen order, deduped by id: {child_ids:?}");
            }

            #[test]
            fn organize_context_menu_collapses_doubled_bare_separators_and_drops_leading_trailing_ones() {
                let items = vec![menu_separator("lead-bare"), menu_leaf("a"), menu_separator("dup-1"), menu_separator("dup-2"), menu_leaf("b"), menu_separator("trail-bare")];
                let organized = organize_context_menu(items, &no_category);
                assert_eq!(organized.len(), 3, "leading/trailing bare separators drop, the doubled run collapses to one: {organized:?}");
                assert_eq!(organized[0].id, "a");
                assert_eq!(organized[1].separator, Some(true));
                assert_eq!(organized[1].label, None, "the surviving separator is bare, not a header");
                assert_eq!(organized[2].id, "b");
            }

            #[test]
            fn organize_context_menu_keeps_a_labeled_separator_as_a_non_interactive_header() {
                let items = vec![menu_leaf("a"), menu_header("Recent"), menu_leaf("b")];
                let organized = organize_context_menu(items.clone(), &no_category);
                assert_eq!(organized, items, "a header is preserved in place, untouched by budget/ordering: {organized:?}");
                assert_eq!(organized[1].label.as_deref(), Some("Recent"));
                assert_eq!(organized[1].separator, Some(true));
            }

            #[test]
            fn organize_context_menu_sorts_group_rows_in_taxonomy_order_unknown_last() {
                let items = vec![menu_group("mystery", vec![menu_leaf("x")]), menu_group("export", vec![menu_leaf("y")]), menu_group("view", vec![menu_leaf("z")])];
                let organized = organize_context_menu(items, &no_category);
                let ids: Vec<&str> = organized.iter().map(|item| item.id.as_str()).collect();
                assert_eq!(ids, vec!["menu.group.view", "menu.group.export", "menu.group.mystery"], "view < export < unknown category: {ids:?}");
            }

            #[test]
            fn organize_context_menu_folds_overflow_groups_into_menu_group_more() {
                let mut items: Vec<ContextMenuItemSpec> = (0..5).map(|index| menu_leaf(&format!("primary{index}"))).collect();
                for category in ["hand", "selection", "lasso", "filter", "open", "save", "transfer", "transform"] {
                    items.push(menu_group(category, vec![menu_leaf(&format!("{category}-child"))]));
                }
                assert!(items.len() > 9, "fixture must exceed the row budget to exercise the >9 path");
                let organized = organize_context_menu(items, &no_category);
                assert_eq!(organized.len(), 9, "primaries + groups clamp to the 9-row budget: {organized:?}");
                assert_eq!(organized.last().unwrap().id, "menu.group.more");
                assert!(!organized.last().unwrap().children.as_ref().unwrap().is_empty(), "the folded group carries the overflowing groups' children");
            }

            #[test]
            fn organize_context_menu_buckets_overflow_leaves_by_category_of() {
                let mut items: Vec<ContextMenuItemSpec> = (0..5).map(|index| menu_leaf(&format!("primary{index}"))).collect();
                for index in 0..6 {
                    items.push(menu_leaf(&format!("overflow{index}")));
                }
                let categorize = |id: &str| if id.starts_with("overflow") { Some("view".to_string()) } else { None };
                let organized = organize_context_menu(items, &categorize);
                assert_eq!(organized.len(), 6, "5 primaries + 1 view group: {organized:?}");
                assert_eq!(organized[5].id, "menu.group.view");
                assert_eq!(organized[5].children.as_ref().unwrap().len(), 6);
            }

            #[test]
            fn ribbon_parent_label_covers_exactly_the_twenty_taxonomy_ids_and_rejects_unknown() {
                assert_eq!(RIBBON_PARENT_CATEGORIES.len(), 20);
                for category in RIBBON_PARENT_CATEGORIES {
                    assert!(ribbon_parent_label(category, false).is_some(), "missing EN label for {category:?}");
                    assert!(ribbon_parent_label(category, true).is_some(), "missing DE label for {category:?}");
                }
                assert_eq!(ribbon_parent_label("not-a-category", false), None);
            }

            #[test]
            fn build_shell_context_menu_specs_shapes_arg_carrying_actions_and_appends_the_palette() {
                let actions = vec![
                    ShellMenuAction { id: "shell.rename".into(), label: "Rename".into(), icon: None, keys: None, kind: "Operation".into(), category: None, in_palette: true, arg_carrying: true },
                    ShellMenuAction { id: "shell.hidden".into(), label: "Hidden".into(), icon: None, keys: None, kind: "Operation".into(), category: None, in_palette: false, arg_carrying: false },
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
}
// #endregion component

#[cfg(feature = "engine")]
pub mod arena {
    // #region arena
    //! 🕳️ Hand-rolled generational arena for retained-mode tree nodes. No third-party slotmap dep: a
    //! wrapper around an external crate's handle type would hide nothing and add a dependency for a
    //! ~120-line data structure (repo rule: don't wrap external types without adding value).

    /// 🪪️ Opaque handle into an `Arena`: a slot index plus a generation counter. A stale `NodeId`
    /// (same index, old generation) never aliases a value inserted into a recycled slot.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct NodeId {
        index: u32,
        generation: u32,
    }

    enum Slot<T> {
        Occupied { generation: u32, value: T },
        Free { generation: u32, next_free: Option<u32> },
    }

    /// 🌳️ Generational-index arena: O(1) insert/remove/get, freed slots recycled via an intrusive free
    /// list threaded through `Slot::Free`.
    pub struct Arena<T> {
        slots: Vec<Slot<T>>,
        free_head: Option<u32>,
    }

    impl<T> Default for Arena<T> {
        fn default() -> Self {
            Self { slots: Vec::new(), free_head: None }
        }
    }

    impl<T> Arena<T> {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn insert(&mut self, value: T) -> NodeId {
            match self.free_head {
                Some(index) => {
                    let (generation, next_free) = match &self.slots[index as usize] {
                        Slot::Free { generation, next_free } => (*generation, *next_free),
                        Slot::Occupied { .. } => unreachable!("free list points at an occupied slot"),
                    };
                    self.free_head = next_free;
                    self.slots[index as usize] = Slot::Occupied { generation, value };
                    NodeId { index, generation }
                }
                None => {
                    let index = self.slots.len() as u32;
                    self.slots.push(Slot::Occupied { generation: 0, value });
                    NodeId { index, generation: 0 }
                }
            }
        }

        pub fn remove(&mut self, id: NodeId) -> Option<T> {
            let slot = self.slots.get_mut(id.index as usize)?;
            match slot {
                Slot::Occupied { generation, .. } if *generation == id.generation => {
                    let next_free = self.free_head;
                    let freed_generation = generation.wrapping_add(1);
                    let previous = std::mem::replace(slot, Slot::Free { generation: freed_generation, next_free });
                    self.free_head = Some(id.index);
                    match previous {
                        Slot::Occupied { value, .. } => Some(value),
                        Slot::Free { .. } => unreachable!(),
                    }
                }
                _ => None,
            }
        }

        pub fn get(&self, id: NodeId) -> Option<&T> {
            match self.slots.get(id.index as usize)? {
                Slot::Occupied { generation, value } if *generation == id.generation => Some(value),
                _ => None,
            }
        }

        pub fn get_mut(&mut self, id: NodeId) -> Option<&mut T> {
            match self.slots.get_mut(id.index as usize)? {
                Slot::Occupied { generation, value } if *generation == id.generation => Some(value),
                _ => None,
            }
        }

        pub fn contains(&self, id: NodeId) -> bool {
            self.get(id).is_some()
        }

        /// 🚶️ Iterates every live `(NodeId, &T)` pair; freed slots are skipped.
        pub fn iter(&self) -> impl Iterator<Item = (NodeId, &T)> {
            self.slots.iter().enumerate().filter_map(|(index, slot)| match slot {
                Slot::Occupied { generation, value } => Some((NodeId { index: index as u32, generation: *generation }, value)),
                Slot::Free { .. } => None,
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn insert_and_get_round_trip() {
            let mut arena = Arena::new();
            let id = arena.insert(42);
            assert_eq!(arena.get(id), Some(&42));
        }

        #[test]
        fn remove_invalidates_the_old_node_id() {
            let mut arena = Arena::new();
            let id = arena.insert(1);
            assert_eq!(arena.remove(id), Some(1));
            assert_eq!(arena.get(id), None);
            assert_eq!(arena.remove(id), None);
        }

        #[test]
        fn reused_slot_bumps_generation_so_old_id_does_not_alias_new_value() {
            let mut arena = Arena::new();
            let a = arena.insert(1);
            arena.remove(a);
            let b = arena.insert(2);
            assert_eq!(b.index, a.index);
            assert_ne!(b.generation, a.generation);
            assert_eq!(arena.get(a), None);
            assert_eq!(arena.get(b), Some(&2));
        }

        #[test]
        fn iterates_over_live_slots_only() {
            let mut arena = Arena::new();
            let a = arena.insert(10);
            let b = arena.insert(20);
            arena.remove(a);
            let remaining: Vec<i32> = arena.iter().map(|(_, value)| *value).collect();
            assert_eq!(remaining, vec![20]);
            assert!(arena.contains(b));
            assert!(!arena.contains(a));
        }
    }
    // #endregion arena
}

#[cfg(feature = "engine")]
pub mod tree {
    // #region tree
    //! 🌲️ Retained scene-graph: one `UiTree` per window, holding `Node`s in a generational `Arena`
    //! with parent/first-child/last-child/sibling links and dirty-flag propagation. The engine facade
    //! (a later milestone) holds `HashMap<window_id, UiTree>`.

    use crate::arena::{Arena, NodeId};
    use crate::component::ui::UiNode;

    /// 🔑️ Stable child identity for keyed reconciliation: the source `UiNode`'s explicit `id` field
    /// when it has one, else a `(variant, ordinal)` positional fallback.
    #[derive(Clone, PartialEq, Eq, Hash, Debug)]
    pub enum NodeKey {
        Explicit(String),
        Positional(u32, u32),
    }

    /// 🚩️ Per-node dirty/interaction bits. Hand-rolled over a `u16` (no `bitflags` dep) to keep the
    /// crate dependency-free for this ~10-flag set.
    #[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
    pub struct NodeFlags(u16);

    impl NodeFlags {
        pub const HOVERED: NodeFlags = NodeFlags(1 << 0);
        pub const ACTIVE: NodeFlags = NodeFlags(1 << 1);
        pub const FOCUSED: NodeFlags = NodeFlags(1 << 2);
        pub const DIRTY_LAYOUT: NodeFlags = NodeFlags(1 << 3);
        pub const DIRTY_PAINT: NodeFlags = NodeFlags(1 << 4);
        pub const SUBTREE_DIRTY: NodeFlags = NodeFlags(1 << 5);
        pub const OVERLAY: NodeFlags = NodeFlags(1 << 6);
        pub const CLIPS_CHILDREN: NodeFlags = NodeFlags(1 << 7);
        pub const HIT_TRANSPARENT: NodeFlags = NodeFlags(1 << 8);
        pub const HAS_POPUP: NodeFlags = NodeFlags(1 << 9);
        /// 🫳️ M5 `events`: this node is a drag source (has, or can have, a registered `DragPayload`).
        /// Purely advisory for paint (grab-cursor affordance)/cursor derivation — `events` itself tracks
        /// draggability via its own `EventRouter::set_drag_payload` registry, not this flag.
        pub const DRAG_SOURCE: NodeFlags = NodeFlags(1 << 10);
        /// 🎯️ M5 `events`: this node accepts drops (paired with `EventRouter::set_drop_accept` for the
        /// finer per-widget predicate). `events::nearest_accepting_drop_target` walks the bubble chain
        /// looking for this flag.
        pub const DROP_TARGET: NodeFlags = NodeFlags(1 << 11);
        /// 🖱️ M5 `events`: this node owns a scrollable viewport (`WidgetState::scroll_offset`).
        /// `events::nearest_scrollable_ancestor` walks the bubble chain from a wheel event's hit target
        /// looking for this flag.
        pub const SCROLLABLE: NodeFlags = NodeFlags(1 << 12);

        pub const fn empty() -> Self {
            NodeFlags(0)
        }

        pub fn set(&mut self, flag: NodeFlags, on: bool) {
            if on {
                self.0 |= flag.0;
            } else {
                self.0 &= !flag.0;
            }
        }

        pub fn contains(&self, flag: NodeFlags) -> bool {
            self.0 & flag.0 == flag.0
        }
    }

    /// 🧩️ Retained per-node widget spec. For M2 a thin clone of the last-applied `UiNode` (used as the
    /// reconcile diff baseline); refined into per-variant retained fields in M4.
    #[derive(Clone, Debug, PartialEq)]
    pub struct WidgetSpec(pub UiNode);

    /// ✍️ A focused editable text widget's live buffer (`events`' M5 key-routing writes here). Byte
    /// offsets throughout (`caret`/`anchor`), not char indices: Rust string slicing/`replace_range` are
    /// natively byte-indexed, and `events::{prev_char_boundary, next_char_boundary}` step these
    /// safely across multi-byte UTF-8 without an O(n) char-counting pass on every keystroke. Selection
    /// is `anchor..caret` in either order (mirrors the DOM `Selection` model: `anchor` is where the
    /// selection started, `caret`/`focus` is the live end that arrow keys move).
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct EditState {
        pub text: String,
        pub caret: usize,
        pub anchor: usize,
        /// 🈶️ IME preedit text, `Some` only mid-composition (`events::UiEvent::Ime`). Modeled so the
        /// shape is ready to receive real OS IME events; actually wiring winit's `Ime`/a hidden DOM
        /// input to this is a later `host`-region concern, out of `events`' scope.
        pub composition: Option<String>,
        pub scroll_x: f32,
    }

    /// 🎛️ Interactive per-node state that survives `reconcile::apply_tree` untouched (only `spec` is
    /// ever overwritten by reconciliation — see `reconcile::diff_and_update`), which is exactly the
    /// "focused buffer wins over a fresh incoming `value`" guarantee M5 `events` needs: as long as
    /// `edit` stays `Some`, an `apply_tree` call re-diffing this node's declarative `value` never
    /// touches it. `events::FocusState::set_focus` seeds `edit` from the widget's declarative value on
    /// focus and clears it on blur, so external state governs again once editing ends.
    #[derive(Clone, Debug, Default)]
    pub struct WidgetState {
        pub edit: Option<EditState>,
        /// 🖱️ M5 `events` scroll routing's live offset for a `NodeFlags::SCROLLABLE` node.
        pub scroll_offset: (f32, f32),
        /// 🔽️ M5/W2 wiring: whether a `Select`'s synthesized popup (`reconcile::children_of`'s `Select`
        /// arm always builds the item rows unconditionally, per that module's own doc comment) is
        /// currently shown. Toggled by `events::EventRouter::dispatch`'s `Select`-click handling (via
        /// `open_overlay`/`close_overlay`, see `EventRouter::toggle_select_popup`/`finish_close`), read
        /// by `paint::paint_select` to decide whether to paint the popup at all.
        pub open: bool,
    }

    /// 📐️ Resolved rect from the last taffy layout pass, in the node's **parent-relative** coordinate
    /// space (taffy's own `Layout::location`/`Layout::size` semantics — no extra transform needed when
    /// consuming it; a later paint milestone accumulates ancestor offsets while walking the tree, same
    /// as it already walks parent/child links for painting). `cached_text_measure` mirrors the last
    /// `(text, wrap width bucket)` this node was measured at, so `flex::LayoutEngine` can skip
    /// re-shaping an unchanged text node against an unchanged constraint.
    /// 📏️ `(text, wrap width bucket)` key paired with its measured `(width, height)` — see
    /// `LayoutBucket::cached_text_measure`.
    pub type TextMeasureCache = Option<((String, Option<u32>), (f32, f32))>;

    #[derive(Clone, Debug, Default)]
    pub struct LayoutBucket {
        pub x: f32,
        pub y: f32,
        pub width: f32,
        pub height: f32,
        pub cached_text_measure: TextMeasureCache,
    }

    /// 🎨️ M4 decision: stays an empty marker. Every `paint::paint_*` function recomputes its `DrawList`
    /// entries fresh from `spec`+`layout`+`theme` on each visit instead of caching tessellation output —
    /// the composite widgets that would benefit most from caching (Select's open menu, Tree's rows)
    /// aren't retained children yet (that's a later reconcile milestone), so there's nothing stable to
    /// key a cache on without duplicating that future work; recomputing a handful of quads/glyph-runs
    /// per dirty node per frame is cheap relative to the tessellation this replaces. Revisit once
    /// composite expansion lands and glyph-run caching becomes worth the bookkeeping.
    #[derive(Clone, Debug, Default)]
    pub struct PaintBucket;

    /// 🍃️ One retained tree node: tree links, identity, spec/state/layout/paint buckets, dirty flags.
    pub struct Node {
        pub parent: Option<NodeId>,
        pub first_child: Option<NodeId>,
        pub last_child: Option<NodeId>,
        pub prev_sibling: Option<NodeId>,
        pub next_sibling: Option<NodeId>,
        pub key: NodeKey,
        pub spec: WidgetSpec,
        pub state: WidgetState,
        pub layout: LayoutBucket,
        pub paint: PaintBucket,
        pub flags: NodeFlags,
    }

    impl Node {
        pub fn new(key: NodeKey, spec: WidgetSpec) -> Self {
            Self { parent: None, first_child: None, last_child: None, prev_sibling: None, next_sibling: None, key, spec, state: WidgetState::default(), layout: LayoutBucket::default(), paint: PaintBucket, flags: NodeFlags::empty() }
        }
    }

    /// 🌲️ One window's retained scene-graph: a generational arena of `Node`s plus its root.
    #[derive(Default)]
    pub struct UiTree {
        arena: Arena<Node>,
        pub root: Option<NodeId>,
    }

    impl UiTree {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn node(&self, id: NodeId) -> Option<&Node> {
            self.arena.get(id)
        }

        pub fn node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
            self.arena.get_mut(id)
        }

        pub fn contains(&self, id: NodeId) -> bool {
            self.arena.contains(id)
        }

        /// 🔗️ Inserts `node` as the last child of `parent` (or as a root if `parent` is `None` and no
        /// root exists yet), threading the sibling links.
        pub fn insert_child(&mut self, parent: Option<NodeId>, mut node: Node) -> NodeId {
            node.parent = parent;
            let id = self.arena.insert(node);
            match parent {
                Some(parent_id) => {
                    let prev_last = self.arena.get(parent_id).and_then(|p| p.last_child);
                    if let Some(prev_last_id) = prev_last {
                        if let Some(prev_last_node) = self.arena.get_mut(prev_last_id) {
                            prev_last_node.next_sibling = Some(id);
                        }
                    }
                    if let Some(child) = self.arena.get_mut(id) {
                        child.prev_sibling = prev_last;
                    }
                    if let Some(parent_node) = self.arena.get_mut(parent_id) {
                        if parent_node.first_child.is_none() {
                            parent_node.first_child = Some(id);
                        }
                        parent_node.last_child = Some(id);
                    }
                }
                None => {
                    if self.root.is_none() {
                        self.root = Some(id);
                    }
                }
            }
            id
        }

        /// 🧹️ Detaches `id` from its parent/siblings and recursively removes its subtree, freeing every
        /// arena slot involved.
        pub fn remove(&mut self, id: NodeId) {
            let Some(node) = self.arena.get(id) else { return };
            let (parent, prev_sibling, next_sibling) = (node.parent, node.prev_sibling, node.next_sibling);
            let children: Vec<NodeId> = self.children(id).collect();
            for child in children {
                self.remove(child);
            }
            match prev_sibling {
                Some(prev_id) => {
                    if let Some(prev) = self.arena.get_mut(prev_id) {
                        prev.next_sibling = next_sibling;
                    }
                }
                None => {
                    if let Some(parent_id) = parent {
                        if let Some(parent_node) = self.arena.get_mut(parent_id) {
                            parent_node.first_child = next_sibling;
                        }
                    }
                }
            }
            match next_sibling {
                Some(next_id) => {
                    if let Some(next) = self.arena.get_mut(next_id) {
                        next.prev_sibling = prev_sibling;
                    }
                }
                None => {
                    if let Some(parent_id) = parent {
                        if let Some(parent_node) = self.arena.get_mut(parent_id) {
                            parent_node.last_child = prev_sibling;
                        }
                    }
                }
            }
            if self.root == Some(id) {
                self.root = None;
            }
            self.arena.remove(id);
        }

        /// 🚨️ Sets `flags` on `id` (setting `DIRTY_LAYOUT` implies `DIRTY_PAINT`, since layout changes
        /// always require a repaint), then bubbles `SUBTREE_DIRTY` up the parent chain, stopping at the
        /// first ancestor that already carries it — every ancestor above it is necessarily already
        /// marked too, so walking further is wasted work.
        pub fn mark_dirty(&mut self, id: NodeId, flags: NodeFlags) {
            let mut flags = flags;
            if flags.contains(NodeFlags::DIRTY_LAYOUT) {
                flags.set(NodeFlags::DIRTY_PAINT, true);
            }
            let parent = match self.arena.get_mut(id) {
                Some(node) => {
                    node.flags.set(flags, true);
                    node.parent
                }
                None => return,
            };
            let mut cursor = parent;
            while let Some(ancestor_id) = cursor {
                let Some(ancestor) = self.arena.get_mut(ancestor_id) else { break };
                if ancestor.flags.contains(NodeFlags::SUBTREE_DIRTY) {
                    break;
                }
                ancestor.flags.set(NodeFlags::SUBTREE_DIRTY, true);
                cursor = ancestor.parent;
            }
        }

        /// 🚶️ Iterates the direct children of `id` in tree order via the first-child/next-sibling links.
        pub fn children(&self, id: NodeId) -> impl Iterator<Item = NodeId> + '_ {
            let mut next = self.arena.get(id).and_then(|n| n.first_child);
            std::iter::from_fn(move || {
                let current = next?;
                next = self.arena.get(current).and_then(|n| n.next_sibling);
                Some(current)
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::component::ui::{UiNode, UiPresence, UiTextNode};

        fn text(value: &str) -> UiNode {
            UiNode::Text(UiTextNode { value: value.into(), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })
        }

        fn leaf(discriminant: u32, ordinal: u32, value: &str) -> Node {
            Node::new(NodeKey::Positional(discriminant, ordinal), WidgetSpec(text(value)))
        }

        #[test]
        fn insert_and_iterate_children_in_order() {
            let mut tree = UiTree::new();
            let root = tree.insert_child(None, leaf(0, 0, "root"));
            let a = tree.insert_child(Some(root), leaf(1, 0, "a"));
            let b = tree.insert_child(Some(root), leaf(1, 1, "b"));
            let grandchild = tree.insert_child(Some(b), leaf(1, 0, "c"));

            let children: Vec<NodeId> = tree.children(root).collect();
            assert_eq!(children, vec![a, b]);
            let grandchildren: Vec<NodeId> = tree.children(b).collect();
            assert_eq!(grandchildren, vec![grandchild]);
            assert_eq!(tree.children(grandchild).count(), 0);
        }

        #[test]
        fn mark_dirty_sets_layout_and_paint_and_bubbles_subtree_dirty_to_root() {
            let mut tree = UiTree::new();
            let root = tree.insert_child(None, leaf(0, 0, "root"));
            let mid = tree.insert_child(Some(root), leaf(1, 0, "mid"));
            let grandchild = tree.insert_child(Some(mid), leaf(1, 0, "leaf"));

            tree.mark_dirty(grandchild, NodeFlags::DIRTY_LAYOUT);

            let grandchild_flags = tree.node(grandchild).unwrap().flags;
            assert!(grandchild_flags.contains(NodeFlags::DIRTY_LAYOUT));
            assert!(grandchild_flags.contains(NodeFlags::DIRTY_PAINT));
            assert!(tree.node(mid).unwrap().flags.contains(NodeFlags::SUBTREE_DIRTY));
            assert!(tree.node(root).unwrap().flags.contains(NodeFlags::SUBTREE_DIRTY));
        }

        #[test]
        fn mark_dirty_stops_bubbling_once_it_hits_an_already_dirty_ancestor() {
            let mut tree = UiTree::new();
            let root = tree.insert_child(None, leaf(0, 0, "root"));
            let mid = tree.insert_child(Some(root), leaf(1, 0, "mid"));
            let leaf_a = tree.insert_child(Some(mid), leaf(1, 0, "a"));
            let leaf_b = tree.insert_child(Some(mid), leaf(1, 1, "b"));

            tree.mark_dirty(leaf_a, NodeFlags::DIRTY_PAINT);
            assert!(tree.node(mid).unwrap().flags.contains(NodeFlags::SUBTREE_DIRTY));
            assert!(tree.node(root).unwrap().flags.contains(NodeFlags::SUBTREE_DIRTY));

            // mid and root already carry SUBTREE_DIRTY; this second call must still end up correct
            // (leaf_b itself dirtied, ancestors still dirtied) even though it stops bubbling at `mid`.
            tree.mark_dirty(leaf_b, NodeFlags::DIRTY_PAINT);
            assert!(tree.node(leaf_b).unwrap().flags.contains(NodeFlags::DIRTY_PAINT));
            assert!(!tree.node(leaf_a).unwrap().flags.contains(NodeFlags::SUBTREE_DIRTY));
            assert!(tree.node(mid).unwrap().flags.contains(NodeFlags::SUBTREE_DIRTY));
            assert!(tree.node(root).unwrap().flags.contains(NodeFlags::SUBTREE_DIRTY));
        }

        #[test]
        fn remove_detaches_node_and_frees_its_children_slots() {
            let mut tree = UiTree::new();
            let root = tree.insert_child(None, leaf(0, 0, "root"));
            let mid = tree.insert_child(Some(root), leaf(1, 0, "mid"));
            let grandchild = tree.insert_child(Some(mid), leaf(1, 0, "leaf"));

            tree.remove(mid);

            assert!(!tree.contains(mid));
            assert!(!tree.contains(grandchild));
            assert!(tree.contains(root));
            assert_eq!(tree.children(root).count(), 0);
        }
    }
    // #endregion tree
}

#[cfg(feature = "engine")]
pub mod reconcile {
    // #region reconcile
    //! 🔁️ Keyed single-pass reconciliation: applies an incoming declarative `UiNode` tree to a retained
    //! `UiTree`, matching children by key, diffing matched nodes, and marking the minimal dirty flags.
    //! `Stack`/`Section`/`Field` recurse into their own literal `UiNode` children; `Select`/`Tree` recurse
    //! into *synthesized* retained children (see `🔖️CompositeExpansion` below) built from their
    //! non-`UiNode` payload (`items`/`sections`) since there is no dedicated `UiNode` variant for "one
    //! Select option row" or "one Tree row" to reuse verbatim — the remaining 14 variants have no nested
    //! `UiNode`/composite payload at all, so a diffed leaf is already their complete, correct treatment.
    //! KNOWN GAP (wiring request, not fixable from this region alone — see `tree::WidgetState`'s own doc
    //! comment): `Select`'s synthesized option rows are always built, unconditionally, regardless of
    //! open/closed — `tree::WidgetState` is currently a zero-field marker with nowhere to record "is this
    //! Select open", so this region can't gate the *rows' existence* on it. `NodeFlags::HAS_POPUP` is set
    //! on the `Select` node itself (whenever it has ≥1 item) so a later events/paint milestone can find
    //! the always-ready rows once `WidgetState` grows an `open`-like field to gate *showing*/hit-testing
    //! them — no further reconcile-side change should be needed at that point.

    use crate::Label;
    use crate::UiTreeActionPlacement;
    use dsl::DslValue;
    use std::borrow::Cow;
    use std::collections::{HashMap, HashSet};

    use crate::arena::NodeId;
    use crate::component::layout::ActionDescriptor;
    use crate::component::ui::{ui_control_to_node, UiButtonNode, UiNode, UiPresence, UiSelectItem, UiSelectNode, UiStackNode, UiTreeItemAction, UiTreeItemNode, UiTreeNode, UiTreeSectionNode};
    use crate::tree::{Node, NodeFlags, NodeKey, UiTree, WidgetSpec};
    use crate::IconName;

    fn variant_discriminant(node: &UiNode) -> u32 {
        match node {
            UiNode::Stack(_) => 0,
            UiNode::Text(_) => 1,
            UiNode::Button(_) => 2,
            UiNode::Separator(_) => 3,
            UiNode::Input(_) => 4,
            UiNode::Select(_) => 5,
            UiNode::Toggle(_) => 6,
            UiNode::KeyValue(_) => 8,
            UiNode::Slider(_) => 9,
            UiNode::NumberStepper(_) => 10,
            UiNode::Ring(_) => 11,
            UiNode::IconSelect(_) => 12,
            UiNode::Field(_) => 13,
            UiNode::Section(_) => 14,
            UiNode::Tree(_) => 15,
            UiNode::Image(_) => 16,
            UiNode::ComponentScene(_) => 17,
            UiNode::ExternalSlot(_) => 18,
            UiNode::Group(_) => 19,
        }
    }

    fn explicit_id(node: &UiNode) -> Option<&str> {
        match node {
            UiNode::Stack(n) => n.id.as_deref(),
            UiNode::Button(n) => n.id.as_deref(),
            UiNode::Input(n) => Some(n.id.as_str()),
            UiNode::Select(n) => Some(n.id.as_str()),
            UiNode::Toggle(n) => Some(n.id.as_str()),
            UiNode::Slider(n) => Some(n.id.as_str()),
            UiNode::NumberStepper(n) => Some(n.id.as_str()),
            UiNode::Ring(n) => Some(n.id.as_str()),
            UiNode::IconSelect(n) => Some(n.id.as_str()),
            UiNode::Field(n) => Some(n.id.as_str()),
            UiNode::Section(n) => Some(n.id.as_str()),
            UiNode::Group(n) => Some(n.id.as_str()),
            UiNode::Image(n) => Some(n.id.as_str()),
            UiNode::ComponentScene(n) => Some(n.surface_id.as_str()),
            UiNode::ExternalSlot(n) => Some(n.body_key.as_str()),
            UiNode::Text(_) | UiNode::Separator(_) | UiNode::KeyValue(_) | UiNode::Tree(_) => None,
        }
    }

    fn node_key(node: &UiNode, ordinal: u32) -> NodeKey {
        match explicit_id(node) {
            Some(id) if !id.is_empty() => NodeKey::Explicit(id.to_string()),
            _ => NodeKey::Positional(variant_discriminant(node), ordinal),
        }
    }

    /// 🌿️ The keyed-diffable children of `node`: `Stack`/`Section`'s own `children`, `Field`'s single
    /// `child`, borrowed straight from `node` (no allocation); `Select`/`Tree`'s *synthesized* rows (see
    /// `🔖️CompositeExpansion`), freshly built each call since they're derived from non-`UiNode` payload.
    /// Everything else has no nested `UiNode` payload to recurse into. `presence.state == Hidden`
    /// children are dropped here — hidden means not rendered at all, so they get no retained node, no
    /// layout, no paint, no hit-test; this is the one choke point every caller goes through.
    fn children_of(node: &UiNode) -> Vec<Cow<'_, UiNode>> {
        let children = match node {
            UiNode::Stack(n) => n.children.iter().map(Cow::Borrowed).collect(),
            UiNode::Section(n) => n.children.iter().map(Cow::Borrowed).collect(),
            UiNode::Group(n) => n.children.iter().map(Cow::Borrowed).collect(),
            UiNode::Field(n) => vec![Cow::Borrowed(n.child.as_ref())],
            UiNode::Select(select) => select.items.iter().map(|item| Cow::Owned(select_item_row(select, item))).collect(),
            UiNode::Tree(tree_node) => tree_node.sections.iter().map(|section| Cow::Owned(tree_section_row(tree_node, section))).collect(),
            _ => Vec::new(),
        };
        children.into_iter().filter(|child: &Cow<'_, UiNode>| child.presence().visible()).collect()
    }

    //#region 🔖️CompositeExpansion
    /// 🔽️ Synthesizes one retained `Button` row per `Select` item, keyed by the item's own `value` (via
    /// `explicit_id`'s `UiNode::Button` arm) — `UiSelectItem.value` is already Select's stable per-option
    /// identity (it's what `UiSelectNode.value` itself holds to name the current choice), so reusing it as
    /// the row's key needs no extra bookkeeping. See this module's doc comment for the open/closed
    /// `WidgetState` wiring request this groundwork is waiting on.
    fn select_item_row(select: &UiSelectNode, item: &UiSelectItem) -> UiNode {
        UiNode::Button(UiButtonNode { id: Some(item.value.clone()), icon_id: IconName::CircleDot, label: item.label.clone(), action: with_item_value_arg(&select.on_change, &item.value), style: None, presence: UiPresence::default(), menu: None })
    }

    /// 🏷️ Clones `action`, merging a `"value"` key into its JSON `args` object (creating one if absent)
    /// so a click on one synthesized `Select` row is distinguishable from any other row once a later
    /// events milestone dispatches it — `on_change.clone()` alone would fire an identical, valueless
    /// action for every row.
    fn with_item_value_arg(action: &ActionDescriptor, value: &str) -> ActionDescriptor {
        let mut merged = action.clone();
        let mut entries = match merged.args.take() {
            Some(DslValue::Object(map)) => map,
            _ => Vec::new(),
        };
        entries.push(("value".to_string(), DslValue::String(value.to_string())));
        merged.args = Some(DslValue::Object(entries));
        merged
    }

    /// 🌳️ Synthesizes one retained `Stack` row per `Tree` section, keyed by `section.id`, wrapping its
    /// `items` (recursively expanded by `tree_item_row`) as retained children.
    fn tree_section_row(tree_node: &UiTreeNode, section: &UiTreeSectionNode) -> UiNode {
        UiNode::Stack(UiStackNode {
            direction: "vertical".into(),
            gap: None,
            padding: None,
            id: Some(section.id.clone()),
            presence: section.presence,
            activate: None,
            drop_action: tree_node.drop_action.clone(),
            drop_overlay: None,
            children: section.items.iter().map(|item| tree_item_row(tree_node, item)).collect(),
            menu: None,
        })
    }

    /// 🌳️ Synthesizes one retained `Stack` row per `Tree` item, keyed by `item.id`. Carries the item's own
    /// `presence` (already the single source of truth for selected/previewed/status — no more union with
    /// a tree-level id list) and `activate` (the row's click `action`) as a `UiStackNode`'s own fields,
    /// plus its embedded `control` (via `ui_control_to_node`), trailing `actions` (via
    /// `tree_item_action_row`), and nested `items` (recursively) as retained children.
    /// `hover_action`/`unhover_action`/`draggable`/`drag_data` have no matching `UiStackNode` field to
    /// carry them structurally — a later events/interaction milestone re-derives those straight from this
    /// row's key (`item.id`) against the parent `Tree` node's still-fully-intact `spec.0` (reconcile never
    /// drops fields, only clones them into `WidgetSpec`).
    fn tree_item_row(tree_node: &UiTreeNode, item: &UiTreeItemNode) -> UiNode {
        let mut children: Vec<UiNode> = Vec::new();
        if let Some(control) = &item.control {
            children.push(ui_control_to_node(control.clone()));
        }
        for action in item.actions.iter().flatten() {
            if action.placement() == UiTreeActionPlacement::Menu {
                continue;
            }
            children.push(tree_item_action_row(action));
        }
        for nested in item.items.iter().flatten() {
            children.push(tree_item_row(tree_node, nested));
        }
        UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: Some(item.id.clone()), presence: item.presence, activate: item.action.clone(), drop_action: None, drop_overlay: None, children, menu: item.menu.clone() })
    }

    /// 🌳️ Synthesizes one retained `Button` row per `UiTreeItemAction` (a `Tree` item's trailing/
    /// row-placement action buttons). No stable id exists on `UiTreeItemAction` itself (unlike items/
    /// sections), so this leaves `UiButtonNode.id` unset — `node_key`'s positional fallback (keyed by the
    /// action's ordinal within its parent row's `actions` list) is already stable across re-renders for a
    /// fixed action set, matching every other id-less synthesized/leaf child in this module.
    fn tree_item_action_row(action: &UiTreeItemAction) -> UiNode {
        UiNode::Button(UiButtonNode { id: None, icon_id: action.icon_id.clone(), label: action.label.clone().unwrap_or_else(|| Label::data("")), action: action.action.clone(), style: None, presence: UiPresence::default(), menu: None })
    }
    //#endregion 🔖️CompositeExpansion

    /// ⚖️ Whether the two nodes' *own* scalar fields (excluding nested `UiNode` children, which are
    /// reconciled and dirtied independently) are equal.
    fn own_fields_equal(previous: &UiNode, next: &UiNode) -> bool {
        match (previous, next) {
            (UiNode::Stack(p), UiNode::Stack(n)) => {
                p.direction == n.direction && p.gap == n.gap && p.padding == n.padding && p.id == n.id && p.presence == n.presence && p.activate == n.activate && p.drop_action == n.drop_action && p.children.len() == n.children.len()
            }
            (UiNode::Section(p), UiNode::Section(n)) => p.id == n.id && p.label == n.label && p.default_open == n.default_open && p.presence == n.presence && p.children.len() == n.children.len(),
            (UiNode::Field(p), UiNode::Field(n)) => p.id == n.id && p.label == n.label && p.description == n.description && p.required == n.required && p.error == n.error && p.presence == n.presence,
            _ => previous == next,
        }
    }

    /// 📐️ Whether the field(s) that differ between `previous` and `next` affect measurement/layout (as
    /// opposed to paint-only state like `selected`/`status`/`disabled`). `presence.visible()` flipping
    /// (i.e. `state` crossing into/out of `Hidden`) always counts — a hidden element occupies no layout
    /// space at all, so becoming hidden/unhidden must re-run layout for its parent, unlike every other
    /// `presence` change (selected/status/hover/previewed/disabled), which is paint-only.
    fn layout_affecting_change(previous: &UiNode, next: &UiNode) -> bool {
        if previous.presence().visible() != next.presence().visible() {
            return true;
        }
        match (previous, next) {
            (UiNode::Stack(p), UiNode::Stack(n)) => p.direction != n.direction || p.gap != n.gap || p.padding != n.padding || p.children.len() != n.children.len(),
            (UiNode::Text(p), UiNode::Text(n)) => p.value != n.value,
            (UiNode::Field(p), UiNode::Field(n)) => p.label != n.label || p.description != n.description,
            (UiNode::Section(p), UiNode::Section(n)) => p.label != n.label || p.children.len() != n.children.len(),
            _ => false,
        }
    }

    impl UiTree {
        /// 🔁️ Applies an incoming declarative `UiNode` tree to this retained tree: keyed single-pass
        /// child matching, minimal-dirty-flag diffing of matched nodes, insertion of unmatched incoming
        /// children, removal of unmatched existing children. Re-applying an identical tree sets zero
        /// dirty flags anywhere in the tree.
        pub fn apply_tree(&mut self, incoming: &UiNode) {
            let key = node_key(incoming, 0);
            match self.root {
                Some(root_id) if self.node(root_id).map(|n| &n.key) == Some(&key) => {
                    self.diff_and_update(root_id, incoming);
                    self.reconcile_children(root_id, incoming);
                }
                Some(root_id) => {
                    self.remove(root_id);
                    self.root = None;
                    self.insert_new_root(key, incoming);
                }
                None => self.insert_new_root(key, incoming),
            }
        }

        fn insert_new_root(&mut self, key: NodeKey, incoming: &UiNode) {
            let id = self.insert_child(None, Node::new(key, WidgetSpec(incoming.clone())));
            self.mark_dirty(id, NodeFlags::DIRTY_LAYOUT);
            self.root = Some(id);
            self.reconcile_children(id, incoming);
        }

        fn diff_and_update(&mut self, id: NodeId, incoming: &UiNode) {
            let (needs_layout, needs_paint) = match self.node(id) {
                Some(node) if own_fields_equal(&node.spec.0, incoming) => (false, false),
                Some(node) if layout_affecting_change(&node.spec.0, incoming) => (true, true),
                Some(_) => (false, true),
                None => return,
            };
            if let Some(node) = self.node_mut(id) {
                node.spec = WidgetSpec(incoming.clone());
            }
            if needs_layout {
                self.mark_dirty(id, NodeFlags::DIRTY_LAYOUT);
            } else if needs_paint {
                self.mark_dirty(id, NodeFlags::DIRTY_PAINT);
            }
        }

        /// 🚩️ Keeps structural `NodeFlags` that reflect `incoming`'s own shape (not its diff status) in
        /// sync — currently just `HAS_POPUP` on a `Select` with ≥1 item, so a later events/paint milestone
        /// can find "this Select has synthesized option rows ready under it" (see this module's own doc
        /// comment for the `WidgetState` open/closed wiring request that gates actually showing them)
        /// without re-deriving it from `spec.0` itself. Deliberately bypasses `mark_dirty` — direct flag
        /// mutation, no `SUBTREE_DIRTY` bubbling — since this is bookkeeping metadata, not a repaint signal.
        fn sync_composite_flags(&mut self, id: NodeId, incoming: &UiNode) {
            if let UiNode::Select(select) = incoming {
                if let Some(node) = self.node_mut(id) {
                    node.flags.set(NodeFlags::HAS_POPUP, !select.items.is_empty());
                }
            }
        }

        fn reconcile_children(&mut self, parent: NodeId, incoming: &UiNode) {
            self.sync_composite_flags(parent, incoming);
            let incoming_children = children_of(incoming);
            let existing_children: Vec<NodeId> = self.children(parent).collect();

            let mut existing_by_key: HashMap<NodeKey, NodeId> = HashMap::with_capacity(existing_children.len());
            for child_id in &existing_children {
                if let Some(node) = self.node(*child_id) {
                    existing_by_key.insert(node.key.clone(), *child_id);
                }
            }

            let mut used_keys: HashSet<NodeKey> = HashSet::with_capacity(incoming_children.len());
            let mut matched_ids: HashSet<NodeId> = HashSet::with_capacity(incoming_children.len());
            for (ordinal, child) in incoming_children.iter().enumerate() {
                let key = node_key(child, ordinal as u32);
                let matched_id = match existing_by_key.get(&key) {
                    Some(existing_id) if !used_keys.contains(&key) => {
                        used_keys.insert(key);
                        self.diff_and_update(*existing_id, child);
                        self.reconcile_children(*existing_id, child);
                        *existing_id
                    }
                    _ => {
                        let id = self.insert_child(Some(parent), Node::new(key, WidgetSpec(child.clone().into_owned())));
                        self.mark_dirty(id, NodeFlags::DIRTY_LAYOUT);
                        self.reconcile_children(id, child);
                        id
                    }
                };
                matched_ids.insert(matched_id);
            }

            for existing_id in existing_children {
                if !matched_ids.contains(&existing_id) {
                    self.remove(existing_id);
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::component::layout::ActionDescriptor;
        use crate::component::ui::{ui_tree_stamp_presence, UiButtonNode, UiControlNode, UiPresence, UiStackNode, UiTextNode, UiToggleNode};
        use crate::tree::NodeFlags;

        fn action() -> ActionDescriptor {
            ActionDescriptor { controller_id: "ctrl".into(), action: "go".into(), args: None }
        }

        fn text(value: &str) -> UiNode {
            UiNode::Text(UiTextNode { value: value.into(), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })
        }

        fn button(id: &str, label: &str) -> UiNode {
            UiNode::Button(UiButtonNode { id: Some(id.into()), icon_id: IconName::CircleDot, label: label.into(), action: action(), style: None, presence: UiPresence::default(), menu: None })
        }

        fn stack(id: &str, children: Vec<UiNode>) -> UiNode {
            UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, id: Some(id.into()), presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children, menu: None })
        }

        fn clear_dirty(tree: &mut UiTree, id: NodeId) {
            if let Some(node) = tree.node_mut(id) {
                node.flags.set(NodeFlags::DIRTY_LAYOUT, false);
                node.flags.set(NodeFlags::DIRTY_PAINT, false);
                node.flags.set(NodeFlags::SUBTREE_DIRTY, false);
            }
            let children: Vec<NodeId> = tree.children(id).collect();
            for child in children {
                clear_dirty(tree, child);
            }
        }

        fn any_dirty(tree: &UiTree, id: NodeId) -> bool {
            let node = tree.node(id).unwrap();
            let dirty = node.flags.contains(NodeFlags::DIRTY_LAYOUT) || node.flags.contains(NodeFlags::DIRTY_PAINT) || node.flags.contains(NodeFlags::SUBTREE_DIRTY);
            dirty || tree.children(id).any(|child| any_dirty(tree, child))
        }

        #[test]
        fn reapplying_an_identical_tree_sets_zero_dirty_flags() {
            let mut tree = UiTree::new();
            let ui = stack("root", vec![text("hello"), button("btn", "Go")]);
            tree.apply_tree(&ui);
            let root = tree.root.unwrap();
            // fresh insert marks everything dirty; that's expected and not under test here.
            clear_dirty(&mut tree, root);

            tree.apply_tree(&ui);

            assert!(!any_dirty(&tree, root));
        }

        #[test]
        fn text_value_change_dirties_that_node_and_ancestors_but_not_siblings() {
            let mut tree = UiTree::new();
            tree.apply_tree(&stack("root", vec![text("hello"), text("world")]));
            let root = tree.root.unwrap();
            clear_dirty(&mut tree, root);

            tree.apply_tree(&stack("root", vec![text("changed"), text("world")]));

            let children: Vec<NodeId> = tree.children(root).collect();
            let first = tree.node(children[0]).unwrap();
            assert!(first.flags.contains(NodeFlags::DIRTY_LAYOUT));
            assert!(first.flags.contains(NodeFlags::DIRTY_PAINT));
            let second = tree.node(children[1]).unwrap();
            assert!(!second.flags.contains(NodeFlags::DIRTY_LAYOUT));
            assert!(!second.flags.contains(NodeFlags::DIRTY_PAINT));
            assert!(tree.node(root).unwrap().flags.contains(NodeFlags::SUBTREE_DIRTY));
        }

        #[test]
        fn adding_a_child_inserts_exactly_one_new_dirty_node_and_leaves_siblings_untouched() {
            let mut tree = UiTree::new();
            tree.apply_tree(&stack("root", vec![text("hello")]));
            let root = tree.root.unwrap();
            clear_dirty(&mut tree, root);

            tree.apply_tree(&stack("root", vec![text("hello"), text("new")]));

            let children: Vec<NodeId> = tree.children(root).collect();
            assert_eq!(children.len(), 2);
            let first = tree.node(children[0]).unwrap();
            assert!(!first.flags.contains(NodeFlags::DIRTY_LAYOUT));
            assert!(!first.flags.contains(NodeFlags::DIRTY_PAINT));
            let second = tree.node(children[1]).unwrap();
            assert!(second.flags.contains(NodeFlags::DIRTY_LAYOUT));
        }

        #[test]
        fn removing_a_child_frees_its_arena_slot() {
            let mut tree = UiTree::new();
            tree.apply_tree(&stack("root", vec![text("hello"), text("bye")]));
            let root = tree.root.unwrap();
            let children_before: Vec<NodeId> = tree.children(root).collect();
            let removed_id = children_before[1];

            tree.apply_tree(&stack("root", vec![text("hello")]));

            assert!(!tree.contains(removed_id));
            assert_eq!(tree.children(root).count(), 1);
        }

        //#region 🔖️CompositeExpansionTests
        fn select(id: &str, value: &str, items: Vec<(&str, &str)>) -> UiNode {
            UiNode::Select(UiSelectNode {
                id: id.into(),
                value: value.into(),
                items: items.into_iter().map(|(value, label)| UiSelectItem { value: value.into(), label: label.into() }).collect(),
                placeholder: None,
                on_change: action(),
                presence: UiPresence::default(),
                menu: None,
            })
        }

        fn tree_item(id: &str, label: &str) -> UiTreeItemNode {
            UiTreeItemNode {
                id: id.into(),
                label: label.into(),
                description: None,
                icon_id: None,
                presence: UiPresence::default(),
                default_open: None,
                action: None,
                hover_action: None,
                unhover_action: None,
                actions: None,
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                dimmed: None,
                menu: None,
            }
        }

        fn tree_ui(mut sections: Vec<UiTreeSectionNode>, selected_ids: Option<Vec<String>>) -> UiNode {
            if let Some(ids) = selected_ids {
                let selected: HashSet<String> = ids.into_iter().collect();
                ui_tree_stamp_presence(&mut sections, &selected, &HashSet::new());
            }
            UiNode::Tree(UiTreeNode { sections, presence: UiPresence::default(), selected_ids: None, highlighted_ids: None, selection_change: None, drop_action: None, menu: None })
        }

        #[test]
        fn select_expands_items_into_keyed_button_rows_carrying_the_chosen_value_and_flags_has_popup() {
            let mut tree = UiTree::new();
            tree.apply_tree(&select("sel", "a", vec![("a", "Alpha"), ("b", "Beta")]));
            let root = tree.root.unwrap();

            assert!(tree.node(root).unwrap().flags.contains(NodeFlags::HAS_POPUP));
            let children: Vec<NodeId> = tree.children(root).collect();
            assert_eq!(children.len(), 2);
            let first = tree.node(children[0]).unwrap();
            assert_eq!(first.key, NodeKey::Explicit("a".into()));
            match &first.spec.0 {
                UiNode::Button(button) => {
                    assert_eq!(button.label, "Alpha");
                    assert_eq!(button.action.args, Some(DslValue::Object(vec![("value".into(), DslValue::String("a".into()))])));
                }
                other => panic!("expected a synthesized Button row, got {other:?}"),
            }
        }

        #[test]
        fn select_removing_an_item_removes_its_row_and_clears_has_popup_once_empty() {
            let mut tree = UiTree::new();
            tree.apply_tree(&select("sel", "a", vec![("a", "Alpha"), ("b", "Beta")]));
            let root = tree.root.unwrap();
            let children_before: Vec<NodeId> = tree.children(root).collect();
            let removed = children_before[1];

            tree.apply_tree(&select("sel", "a", vec![("a", "Alpha")]));
            assert!(!tree.contains(removed));
            assert_eq!(tree.children(root).count(), 1);

            tree.apply_tree(&select("sel", "a", vec![]));
            assert_eq!(tree.children(root).count(), 0);
            assert!(!tree.node(root).unwrap().flags.contains(NodeFlags::HAS_POPUP));
        }

        #[test]
        fn tree_expands_sections_and_nested_items_into_keyed_stack_rows() {
            let mut tree = UiTree::new();
            let nested = UiTreeItemNode { items: Some(vec![tree_item("child", "Child")]), menu: None, ..tree_item("parent", "Parent") };
            let ui = tree_ui(vec![UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![nested] }], Some(vec!["parent".into()]));
            tree.apply_tree(&ui);
            let root = tree.root.unwrap();

            let sections: Vec<NodeId> = tree.children(root).collect();
            assert_eq!(sections.len(), 1);
            assert_eq!(tree.node(sections[0]).unwrap().key, NodeKey::Explicit("s1".into()));

            let items: Vec<NodeId> = tree.children(sections[0]).collect();
            assert_eq!(items.len(), 1);
            let parent_node = tree.node(items[0]).unwrap();
            assert_eq!(parent_node.key, NodeKey::Explicit("parent".into()));
            match &parent_node.spec.0 {
                UiNode::Stack(stack) => assert!(stack.presence.selected, "item.presence.selected unset but its id was stamped selected"),
                other => panic!("expected a synthesized Stack row, got {other:?}"),
            }

            let grandchildren: Vec<NodeId> = tree.children(items[0]).collect();
            assert_eq!(grandchildren.len(), 1);
            assert_eq!(tree.node(grandchildren[0]).unwrap().key, NodeKey::Explicit("child".into()));
        }

        #[test]
        fn tree_item_control_and_trailing_actions_become_retained_children_too() {
            let mut tree = UiTree::new();
            let item = UiTreeItemNode {
                control: Some(UiControlNode::Toggle(UiToggleNode { id: "tog".into(), icon_id: IconName::CircleDot, text: None, on_change: action(), presence: UiPresence::selected(true), menu: None })),
                actions: Some(vec![UiTreeItemAction { icon_id: IconName::Trash2, label: Some("Delete".into()), action: action(), placement: Some(UiTreeActionPlacement::Menu) }]),
                ..tree_item("leaf", "Leaf")
            };
            let ui = tree_ui(vec![UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] }], None);
            tree.apply_tree(&ui);
            let root = tree.root.unwrap();
            let section = tree.children(root).next().unwrap();
            let row = tree.children(section).next().unwrap();

            let row_children: Vec<NodeId> = tree.children(row).collect();
            assert_eq!(row_children.len(), 1, "menu-placement actions are not retained row children; only the embedded control remains");
            assert!(matches!(tree.node(row_children[0]).unwrap().spec.0, UiNode::Toggle(_)), "control comes first");
        }

        #[test]
        fn reapplying_an_identical_select_or_tree_sets_zero_dirty_flags() {
            let mut tree = UiTree::new();
            let select_ui = select("sel", "a", vec![("a", "Alpha"), ("b", "Beta")]);
            tree.apply_tree(&select_ui);
            let root = tree.root.unwrap();
            clear_dirty(&mut tree, root);
            tree.apply_tree(&select_ui);
            assert!(!any_dirty(&tree, root), "re-applying an identical Select must not dirty its synthesized rows");

            let mut tree = UiTree::new();
            let tree_ui_value = tree_ui(vec![UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![tree_item("a", "A")] }], None);
            tree.apply_tree(&tree_ui_value);
            let root = tree.root.unwrap();
            clear_dirty(&mut tree, root);
            tree.apply_tree(&tree_ui_value);
            assert!(!any_dirty(&tree, root), "re-applying an identical Tree must not dirty its synthesized rows");
        }
        //#endregion 🔖️CompositeExpansionTests
    }
    // #endregion reconcile
}

#[cfg(feature = "engine")]
pub mod chrome {
    // #region chrome
    //! 🎛️ Bordered chrome primitives shared by widgets and shell renderers.

    use crate::draw::DrawList;
    use crate::draw::IconAtlas;
    use crate::geometry::Rect;
    use crate::text::FontAtlas;
    use crate::theme::{Rgba, Theme};

    pub const ICON_TINY: f32 = 14.0;

    pub const TRANSPARENT: Rgba = Rgba::new(0.0, 0.0, 0.0, 0.0);

    pub fn push_chrome_group_border(draw: &mut DrawList, rect: Rect, theme: &Theme) {
        let hair = theme.stroke_hairline;
        push_chrome_border(draw, rect, hair, theme.border_normal, true, true, true, true);
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per border edge/style flag; grouping into a struct is a T2 restructure, out of scope")]
    pub fn push_chrome_border(draw: &mut DrawList, rect: Rect, stroke: f32, color: Rgba, top: bool, right: bool, bottom: bool, left: bool) {
        if top {
            draw.push_solid([rect.x, rect.y, rect.w, stroke], color);
        }
        if bottom {
            draw.push_solid([rect.x, rect.y + rect.h - stroke, rect.w, stroke], color);
        }
        if left {
            draw.push_solid([rect.x, rect.y, stroke, rect.h], color);
        }
        if right {
            draw.push_solid([rect.x + rect.w - stroke, rect.y, stroke, rect.h], color);
        }
    }

    pub fn push_window_cap_border(draw: &mut DrawList, rect: Rect, stroke: f32, color: Rgba) {
        push_chrome_border(draw, rect, stroke, color, true, true, false, true);
    }

    pub fn push_control_border(draw: &mut DrawList, rect: Rect, theme: &Theme, border: Rgba, bg: Rgba) {
        if bg.a > 0.0 {
            draw.push_solid([rect.x, rect.y, rect.w, rect.h], bg);
        }
        let hair = theme.stroke_hairline;
        draw.push_solid([rect.x, rect.y, rect.w, hair], border);
        draw.push_solid([rect.x, rect.y + rect.h - hair, rect.w, hair], border);
        draw.push_solid([rect.x, rect.y, hair, rect.h], border);
        draw.push_solid([rect.x + rect.w - hair, rect.y, hair, rect.h], border);
    }

    pub fn push_icon(draw: &mut DrawList, icons: &IconAtlas, icon_id: &str, x: f32, y: f32, size: f32, color: Rgba) {
        if let Some(uv) = icons.icon_uv(icon_id) {
            draw.push_textured([x, y, size, size], uv, color);
        }
    }

    pub fn measure_action_item(atlas: &mut FontAtlas, theme: &Theme, icon: bool, label: Option<&str>) -> f32 {
        let icon_w = if icon { ICON_TINY + theme.gap_standard } else { 0.0 };
        let text_w = label.map_or(0.0, |value| atlas.measure_text(value, theme.font_size_small).0);
        theme.padding_standard * 2.0 + icon_w + text_w
    }

    pub fn chrome_item_bg(theme: &Theme, active: bool, hovered: bool) -> Rgba {
        if active {
            if hovered {
                theme.accent_hover
            } else {
                theme.selected
            }
        } else if hovered {
            theme.button_hover
        } else {
            TRANSPARENT
        }
    }

    pub fn chrome_item_text(theme: &Theme, active: bool, hovered: bool) -> Rgba {
        if active {
            theme.active_foreground
        } else if hovered {
            theme.border_emphasized
        } else {
            theme.text_element
        }
    }

    pub fn item_bg(theme: &Theme, pressed: bool, hovered: bool) -> Rgba {
        chrome_item_bg(theme, pressed, hovered)
    }

    pub fn item_text(theme: &Theme, pressed: bool, hovered: bool) -> Rgba {
        chrome_item_text(theme, pressed, hovered)
    }
    // #endregion chrome
}

#[cfg(feature = "engine")]
pub mod cursor {
    // #region cursor
    //! 🖱️ Theme-aware Semio cursor URLs for wgpu canvas hover parity with React.

    use crate::arena::NodeId;
    use crate::component::ui::UiNode;
    use crate::events::CaptureKind;
    use crate::input::{DragAxis, HitKind, HitTarget};
    use crate::tree::{NodeFlags, UiTree};

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SemioCursor {
        Default,
        Pointer,
        Selectable,
        Foldable,
        Grab,
        Grabbing,
        Text,
        EwResize,
        NsResize,
        NwseResize,
        NeswResize,
        Move,
        Crosshair,
        NotAllowed,
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub struct CursorDragState {
        pub tree_drag: bool,
        pub dock_drag: bool,
        pub pointer_drag_active: bool,
        pub pointer_drag_axis: Option<DragAxis>,
        pub pointer_drag_kind: Option<HitKind>,
    }

    pub fn resolve_semio_cursor<E>(hit: Option<&HitTarget<E>>, drag: CursorDragState) -> SemioCursor {
        if drag.tree_drag || drag.dock_drag {
            return SemioCursor::Grabbing;
        }
        if drag.pointer_drag_active {
            return cursor_for_active_drag(drag.pointer_drag_kind, drag.pointer_drag_axis);
        }
        let Some(hit) = hit else {
            return SemioCursor::Default;
        };
        if let Some(id) = hit.control_id.as_deref() {
            if id.contains(".chevron.") || id.starts_with("section.chevron.") {
                return SemioCursor::Foldable;
            }
        }
        if matches!(hit.kind, HitKind::PanelResize) {
            return SemioCursor::EwResize;
        }
        if hit.kind == HitKind::DockJoinCorner {
            return SemioCursor::Move;
        }
        if hit.kind == HitKind::DockSplit {
            return hit.drag_axis.map_or(SemioCursor::Default, axis_cursor);
        }
        if hit.kind == HitKind::ScrollRegion {
            if let Some(axis) = hit.drag_axis {
                return axis_cursor(axis);
            }
        }
        match hit.kind {
            HitKind::Input => SemioCursor::Text,
            HitKind::Select => SemioCursor::Foldable,
            HitKind::Slider => SemioCursor::Grab,
            HitKind::Window => SemioCursor::Grab,
            HitKind::TreeItem => {
                if hit.drag_data.is_some() {
                    SemioCursor::Grab
                } else {
                    SemioCursor::Selectable
                }
            }
            HitKind::TreeDropTarget => SemioCursor::Move,
            HitKind::World3d => SemioCursor::Default,
            HitKind::Button | HitKind::Toggle | HitKind::PanelTab | HitKind::NavbarItem | HitKind::ContextMenu | HitKind::DropdownItem => SemioCursor::Selectable,
            HitKind::ScrollRegion | HitKind::PanelResize | HitKind::DockSplit | HitKind::DockJoinCorner => SemioCursor::Default,
            HitKind::Generic => SemioCursor::Selectable,
        }
    }

    fn cursor_for_active_drag(kind: Option<HitKind>, axis: Option<DragAxis>) -> SemioCursor {
        match kind {
            Some(HitKind::Slider) => SemioCursor::Grabbing,
            Some(HitKind::PanelResize) => SemioCursor::EwResize,
            Some(HitKind::DockSplit) => axis.map_or(SemioCursor::Default, axis_cursor),
            Some(HitKind::DockJoinCorner) => SemioCursor::Move,
            Some(HitKind::ScrollRegion) => axis.map_or(SemioCursor::Default, axis_cursor),
            _ => axis.map_or(SemioCursor::Grabbing, axis_cursor),
        }
    }

    fn axis_cursor(axis: DragAxis) -> SemioCursor {
        match axis {
            DragAxis::Horizontal => SemioCursor::EwResize,
            DragAxis::Vertical => SemioCursor::NsResize,
            DragAxis::Both => SemioCursor::NwseResize,
            DragAxis::Ring => SemioCursor::Crosshair,
        }
    }

    /// 🖱️ M5's retained-tree counterpart to `resolve_semio_cursor`: derives a cursor from the
    /// `events::EventRouter`'s own hovered/capture `NodeId`s (via its `hovered()`/`capture()` accessors)
    /// rather than the immediate-mode `input::HitTarget`. Mostly wiring existing pieces together — an
    /// active `CaptureKind` wins outright (dragging/scrolling a thumb never re-derives from whatever's
    /// merely hovered underneath), otherwise it falls back to the hovered node's own `NodeFlags`/
    /// `UiNode` variant.
    pub fn resolve_semio_cursor_from_tree(tree: &UiTree, hovered: Option<NodeId>, capture: Option<(NodeId, CaptureKind)>) -> SemioCursor {
        if let Some((_, kind)) = capture {
            match kind {
                CaptureKind::Drag => return SemioCursor::Grabbing,
                CaptureKind::ScrollThumb(axis) => {
                    return match axis {
                        crate::events::ScrollAxis::Horizontal => SemioCursor::EwResize,
                        crate::events::ScrollAxis::Vertical => SemioCursor::NsResize,
                    };
                }
                CaptureKind::Press => {}
            }
        }
        let Some(target) = capture.map(|(id, _)| id).or(hovered) else {
            return SemioCursor::Default;
        };
        let Some(node) = tree.node(target) else {
            return SemioCursor::Default;
        };
        if node.flags.contains(NodeFlags::DRAG_SOURCE) {
            return SemioCursor::Grab;
        }
        match &node.spec.0 {
            UiNode::Input(_) => SemioCursor::Text,
            UiNode::Select(_) | UiNode::IconSelect(_) => SemioCursor::Foldable,
            UiNode::Slider(_) | UiNode::NumberStepper(_) | UiNode::Ring(_) => SemioCursor::Grab,
            UiNode::Button(_) | UiNode::Toggle(_) => SemioCursor::Selectable,
            _ if node.flags.contains(NodeFlags::DROP_TARGET) => SemioCursor::Default,
            _ => SemioCursor::Default,
        }
    }

    pub fn semio_cursor_css(cursor: SemioCursor, dark: bool) -> &'static str {
        match (cursor, dark) {
            (SemioCursor::Default, false) => "url(/asset/cursor/🔣️cursor.svg) 0 0, default",
            (SemioCursor::Default, true) => "url(/asset/cursor/🔣️cursor_dark.svg) 0 0, default",
            (SemioCursor::Pointer, false) => "url(/asset/cursor/🔣️cursor_pointer.svg) 0 0, pointer",
            (SemioCursor::Pointer, true) => "url(/asset/cursor/🔣️cursor_pointer_dark_inkscape.svg) 0 0, pointer",
            (SemioCursor::Selectable, false) => "url(/asset/cursor/🔣️cursor_selectable.svg) 0 0, pointer",
            (SemioCursor::Selectable, true) => "url(/asset/cursor/🔣️cursor_selectable_dark.svg) 0 0, pointer",
            (SemioCursor::Foldable, false) => "url(/asset/cursor/🔣️cursor_foldable.svg) 0 0, pointer",
            (SemioCursor::Foldable, true) => "url(/asset/cursor/🔣️cursor_foldable_dark.svg) 0 0, pointer",
            (SemioCursor::Grab, false) => "url(/asset/cursor/🔣️cursor_grab.svg) 0 0, grab",
            (SemioCursor::Grab, true) => "url(/asset/cursor/🔣️cursor_grab_dark.svg) 0 0, grab",
            (SemioCursor::Grabbing, _) => "url(/asset/cursor/🔣️cursor_grabbing.svg) 0 0, grabbing",
            (SemioCursor::Text, _) => "text",
            (SemioCursor::EwResize, false) => "url(/asset/cursor/🔣️cursor_ew-resize.svg) 16 2, ew-resize",
            (SemioCursor::EwResize, true) => "url(/asset/cursor/🔣️cursor_ew-resize_dark.svg) 16 2, ew-resize",
            (SemioCursor::NsResize, false) => "url(/asset/cursor/🔣️cursor_ns-resize.svg) 2 16, ns-resize",
            (SemioCursor::NsResize, true) => "url(/asset/cursor/🔣️cursor_ns-resize_dark.svg) 2 16, ns-resize",
            (SemioCursor::NwseResize, false) => "url(/asset/cursor/🔣️cursor_nwse-resize.svg) 16 16, nwse-resize",
            (SemioCursor::NwseResize, true) => "url(/asset/cursor/🔣️cursor_nwse-resize_dark.svg) 16 16, nwse-resize",
            (SemioCursor::NeswResize, false) => "url(/asset/cursor/🔣️cursor_nesw-resize_dark.svg) 16 16, nesw-resize",
            (SemioCursor::NeswResize, true) => "url(/asset/cursor/🔣️cursor_nesw-resize_dark.svg) 16 16, nesw-resize",
            (SemioCursor::Move, false) => "url(/asset/cursor/🔣️cursor_move_inkscape.svg) 16 16, move",
            (SemioCursor::Move, true) => "url(/asset/cursor/🔣️cursor_move_dark.svg) 16 16, move",
            (SemioCursor::Crosshair, false) => "url(/asset/cursor/🔣️cursor_crosshair.svg) 16 16, crosshair",
            (SemioCursor::Crosshair, true) => "url(/asset/cursor/🔣️cursor_crosshair_dark.svg) 16 16, crosshair",
            (SemioCursor::NotAllowed, false) => "url(/asset/cursor/🔣️cursor_not-allowed.svg) 0 0, not-allowed",
            (SemioCursor::NotAllowed, true) => "url(/asset/cursor/🔣️cursor_not-allowed_dark.svg) 0 0, not-allowed",
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn apply_canvas_cursor(canvas: &web_sys::HtmlCanvasElement, cursor: SemioCursor, dark: bool, last: &mut Option<(SemioCursor, bool)>) {
        use wasm_bindgen::JsCast;
        let key = (cursor, dark);
        if last.as_ref() == Some(&key) {
            return;
        }
        *last = Some(key);
        let css = semio_cursor_css(cursor, dark);
        if let Some(element) = canvas.dyn_ref::<web_sys::HtmlElement>() {
            let _ = element.style().set_property("cursor", css);
        }
    }

    pub fn apply_window_cursor(window: &winit::window::Window, cursor: SemioCursor, dark: bool, last: &mut Option<(SemioCursor, bool)>) {
        let key = (cursor, dark);
        if last.as_ref() == Some(&key) {
            return;
        }
        *last = Some(key);
        let _ = dark;
        window.set_cursor(winit_cursor_icon(cursor));
    }

    fn winit_cursor_icon(cursor: SemioCursor) -> winit::window::CursorIcon {
        use winit::window::CursorIcon;
        match cursor {
            SemioCursor::Default => CursorIcon::Default,
            SemioCursor::Pointer | SemioCursor::Selectable | SemioCursor::Foldable => CursorIcon::Pointer,
            SemioCursor::Grab => CursorIcon::Grab,
            SemioCursor::Grabbing => CursorIcon::Grabbing,
            SemioCursor::Text => CursorIcon::Text,
            SemioCursor::EwResize => CursorIcon::EwResize,
            SemioCursor::NsResize => CursorIcon::NsResize,
            SemioCursor::NwseResize => CursorIcon::NwseResize,
            SemioCursor::NeswResize => CursorIcon::NeswResize,
            SemioCursor::Move => CursorIcon::Move,
            SemioCursor::Crosshair => CursorIcon::Crosshair,
            SemioCursor::NotAllowed => CursorIcon::NotAllowed,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::component::ui::UiPresence;
        use crate::geometry::Rect;
        use std::collections::HashMap;

        fn hit(kind: HitKind, axis: Option<DragAxis>) -> HitTarget<()> {
            HitTarget { rect: Rect::new(0.0, 0.0, 10.0, 10.0), event: None, control_id: None, kind, drag_axis: axis, drag_data: None }
        }

        #[test]
        fn dock_split_horizontal_uses_ew_cursor() {
            let mut target = hit(HitKind::DockSplit, Some(DragAxis::Horizontal));
            target.control_id = Some("dock.split.0.0".into());
            let cursor = resolve_semio_cursor(Some(&target), CursorDragState::default());
            assert_eq!(cursor, SemioCursor::EwResize);
        }

        #[test]
        fn dock_join_corner_uses_move_cursor() {
            let target = hit(HitKind::DockJoinCorner, Some(DragAxis::Both));
            let cursor = resolve_semio_cursor(Some(&target), CursorDragState::default());
            assert_eq!(cursor, SemioCursor::Move);
        }

        #[test]
        fn dock_tab_uses_grab_cursor() {
            let cursor = resolve_semio_cursor(Some(&hit(HitKind::Window, None)), CursorDragState::default());
            assert_eq!(cursor, SemioCursor::Grab);
        }

        #[test]
        fn panel_resize_uses_ew_cursor() {
            let cursor = resolve_semio_cursor(Some(&hit(HitKind::PanelResize, Some(DragAxis::Horizontal))), CursorDragState::default());
            assert_eq!(cursor, SemioCursor::EwResize);
        }

        #[test]
        fn active_slider_drag_uses_grabbing() {
            let cursor = resolve_semio_cursor::<()>(None, CursorDragState { pointer_drag_active: true, pointer_drag_axis: Some(DragAxis::Horizontal), pointer_drag_kind: Some(HitKind::Slider), ..CursorDragState::default() });
            assert_eq!(cursor, SemioCursor::Grabbing);
        }

        #[test]
        fn tree_draggable_label_uses_grab() {
            let mut target = hit(HitKind::TreeItem, Some(DragAxis::Both));
            target.drag_data = Some(HashMap::from([("id".into(), "x".into())]));
            let cursor = resolve_semio_cursor(Some(&target), CursorDragState::default());
            assert_eq!(cursor, SemioCursor::Grab);
        }

        #[test]
        fn dark_theme_cursor_urls_use_dark_assets() {
            assert!(semio_cursor_css(SemioCursor::Default, true).contains("🔣️cursor_dark.svg"));
            assert!(semio_cursor_css(SemioCursor::Selectable, false).contains("🔣️cursor_selectable.svg"));
        }

        //#region 🔖️RetainedTreeCursorTests
        use crate::component::layout::ActionDescriptor;
        use crate::component::ui::{UiInputNode, UiStackNode, UiTextNode};
        use crate::events::ScrollAxis;
        use crate::tree::{Node, NodeKey, WidgetSpec};

        fn leaf(node: UiNode) -> (UiTree, NodeId) {
            let mut tree = UiTree::new();
            let id = tree.insert_child(None, Node::new(NodeKey::Positional(0, 0), WidgetSpec(node)));
            (tree, id)
        }

        #[test]
        fn hovering_an_input_uses_the_text_cursor() {
            let (tree, id) = leaf(UiNode::Input(UiInputNode {
                id: "name".into(),
                input_kind: "text".into(),
                value: String::new(),
                placeholder: None,
                commit: None,
                min: None,
                max: None,
                step: None,
                accept: None,
                on_change: ActionDescriptor { controller_id: "c".into(), action: "a".into(), args: None },
                presence: UiPresence::default(),
                menu: None,
            }));
            assert_eq!(resolve_semio_cursor_from_tree(&tree, Some(id), None), SemioCursor::Text);
        }

        #[test]
        fn hovering_a_drag_source_uses_the_grab_cursor() {
            let (mut tree, id) =
                leaf(UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children: Vec::new(), menu: None }));
            tree.node_mut(id).unwrap().flags.set(NodeFlags::DRAG_SOURCE, true);
            assert_eq!(resolve_semio_cursor_from_tree(&tree, Some(id), None), SemioCursor::Grab);
        }

        #[test]
        fn an_active_drag_capture_overrides_whatever_is_merely_hovered() {
            let (tree, dragged) = leaf(UiNode::Text(UiTextNode { value: "x".into(), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None }));
            let cursor = resolve_semio_cursor_from_tree(&tree, None, Some((dragged, CaptureKind::Drag)));
            assert_eq!(cursor, SemioCursor::Grabbing);
        }

        #[test]
        fn a_vertical_scroll_thumb_capture_uses_the_ns_resize_cursor() {
            let (tree, scrollable) = leaf(UiNode::Text(UiTextNode { value: "x".into(), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None }));
            let cursor = resolve_semio_cursor_from_tree(&tree, None, Some((scrollable, CaptureKind::ScrollThumb(ScrollAxis::Vertical))));
            assert_eq!(cursor, SemioCursor::NsResize);
        }
        //#endregion 🔖️RetainedTreeCursorTests
    }
    // #endregion cursor
}

#[cfg(feature = "engine")]
pub mod draw {
    // #region draw
    //! 🖌️ Draw list and GPU pipeline for UI quads, vector geometry, and 3D scene passes.

    use crate::shaders::{BLUR_DOWNSAMPLE_SHADER, GLASS_SHADER, SCENE_BLIT_SHADER, UI_SHADER, VECTOR_SHADER, WORLD3D_LINES_SHADER, WORLD3D_SHADER};
    use crate::theme::{GlassStyle, Level, Rgba, Theme};
    use bytemuck::{Pod, Zeroable};
    use kernel_3d_scene::ScenePass3d;
    use wgpu::util::DeviceExt;

    pub const KIND_SOLID: f32 = 3.0;
    pub const KIND_ROUNDED: f32 = 1.0;
    pub const KIND_GLYPH: f32 = 2.0;
    pub const KIND_TEXTURED: f32 = 4.0;
    pub const KIND_RASTER: f32 = 5.0;
    /// 🌀️ Clockwise spinning + pulsing loading ring (see `UiInstance::loading_border` and `UI_SHADER`'s `kind == 6` branch).
    pub const KIND_LOADING_BORDER: f32 = 6.0;
    /// 🌀️ Dashed, slow-spinning + gently pulsing waiting ring (see `UiInstance::waiting_border` and `UI_SHADER`'s `kind == 7` branch).
    pub const KIND_WAITING_BORDER: f32 = 7.0;
    /// ✅️ Solid, static at-bounds ring for `UiStatus::Finished` (see `UiInstance::finished_border` and `UI_SHADER`'s `kind == 8` branch) — no motion, distinguishing it from loading/waiting.
    pub const KIND_FINISHED_BORDER: f32 = 8.0;
    /// 💫️ Raised-cosine breathing pulse ring for `UiState::Introducing` (see `UiInstance::introducing_border` and `UI_SHADER`'s `kind == 9` branch) — the single shared implementation of the introduction-tour pulse, driven by `globals._pad.x`.
    pub const KIND_INTRODUCING_BORDER: f32 = 9.0;
    pub const SCENE_MIP_LEVELS: u32 = 5;

    #[repr(C)]
    #[derive(Clone, Copy, Debug, Pod, Zeroable)]
    struct BlurGlobals {
        src_mip: f32,
        _pad: [f32; 7],
    }

    #[repr(C)]
    #[derive(Clone, Copy, Debug, Pod, Zeroable)]
    pub struct GlassInstance {
        pub rect: [f32; 4],
        pub tint: [f32; 4],
        pub params: [f32; 4],
    }

    #[derive(Clone, Copy, Debug)]
    pub struct GlassRegion {
        pub rect: [f32; 4],
        pub radius: f32,
        pub tint: Rgba,
        pub alpha: f32,
        pub blur_px: f32,
        pub saturate: f32,
    }

    pub struct SceneColorTarget {
        texture: wgpu::Texture,
        blur_scratch: wgpu::Texture,
        blur_scratch_mip_views: Vec<wgpu::TextureView>,
        sample_view: wgpu::TextureView,
        mip_views: Vec<wgpu::TextureView>,
        sampler: wgpu::Sampler,
        width: u32,
        height: u32,
    }

    impl SceneColorTarget {
        pub fn ensure(device: &wgpu::Device, target: &mut Option<Self>, width: u32, height: u32, format: wgpu::TextureFormat) {
            let width = width.max(1);
            let height = height.max(1);
            if let Some(existing) = target {
                if existing.width == width && existing.height == height {
                    return;
                }
            }
            let texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("scene_color"),
                size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
                mip_level_count: SCENE_MIP_LEVELS,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_SRC,
                view_formats: &[format],
            });
            let blur_scratch = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("scene_blur_scratch"),
                size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
                mip_level_count: SCENE_MIP_LEVELS,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[format],
            });
            let blur_scratch_mip_views = (0..SCENE_MIP_LEVELS)
                .map(|level| {
                    blur_scratch.create_view(&wgpu::TextureViewDescriptor {
                        label: Some(&format!("scene_blur_scratch_mip_{level}")),
                        format: Some(format),
                        dimension: Some(wgpu::TextureViewDimension::D2),
                        base_mip_level: level,
                        mip_level_count: Some(1),
                        ..Default::default()
                    })
                })
                .collect();
            let sample_view = texture.create_view(&wgpu::TextureViewDescriptor {
                label: Some("scene_color_sample"),
                format: Some(format),
                dimension: Some(wgpu::TextureViewDimension::D2),
                base_mip_level: 0,
                mip_level_count: Some(SCENE_MIP_LEVELS),
                ..Default::default()
            });
            let mip_views = (0..SCENE_MIP_LEVELS)
                .map(|level| {
                    texture.create_view(&wgpu::TextureViewDescriptor {
                        label: Some(&format!("scene_color_mip_{level}")),
                        format: Some(format),
                        dimension: Some(wgpu::TextureViewDimension::D2),
                        base_mip_level: level,
                        mip_level_count: Some(1),
                        ..Default::default()
                    })
                })
                .collect();
            let sampler =
                device.create_sampler(&wgpu::SamplerDescriptor { label: Some("scene_color_sampler"), mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Linear, mipmap_filter: wgpu::FilterMode::Linear, ..Default::default() });
            *target = Some(Self { texture, blur_scratch, blur_scratch_mip_views, sample_view, mip_views, sampler, width, height });
        }

        pub fn mip_view(&self, level: u32) -> &wgpu::TextureView {
            &self.mip_views[level as usize]
        }

        pub fn sample_view(&self) -> &wgpu::TextureView {
            &self.sample_view
        }

        pub fn sampler(&self) -> &wgpu::Sampler {
            &self.sampler
        }

        pub fn blur_scratch_mip_view(&self, level: u32) -> &wgpu::TextureView {
            &self.blur_scratch_mip_views[level as usize]
        }

        fn mip_extent(&self, level: u32) -> wgpu::Extent3d {
            wgpu::Extent3d { width: (self.width >> level).max(1), height: (self.height >> level).max(1), depth_or_array_layers: 1 }
        }

        pub fn copy_mip_to_blur_scratch(&self, encoder: &mut wgpu::CommandEncoder, src_mip: u32) {
            let extent = self.mip_extent(src_mip);
            encoder.copy_texture_to_texture(
                wgpu::TexelCopyTextureInfo { texture: &self.texture, mip_level: src_mip, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
                wgpu::TexelCopyTextureInfo { texture: &self.blur_scratch, mip_level: src_mip, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
                extent,
            );
        }
    }

    #[repr(C)]
    #[derive(Clone, Copy, Debug, Pod, Zeroable)]
    pub struct UiGlobals {
        pub screen_size: [f32; 2],
        pub _pad: [f32; 2],
    }

    #[repr(C)]
    #[derive(Clone, Copy, Debug, Pod, Zeroable)]
    pub struct UiInstance {
        pub rect: [f32; 4],
        pub color: [f32; 4],
        pub params: [f32; 4],
        pub uv_rect: [f32; 4],
    }

    impl UiInstance {
        pub fn solid(rect: [f32; 4], color: Rgba) -> Self {
            Self { rect, color: [color.r, color.g, color.b, color.a], params: [0.0, 0.0, KIND_SOLID, 0.0], uv_rect: [0.0, 0.0, 1.0, 1.0] }
        }

        pub fn rounded(rect: [f32; 4], color: Rgba, radius: f32, border: f32, border_color: Rgba) -> Self {
            Self { rect, color: [color.r, color.g, color.b, color.a], params: [radius, border, KIND_ROUNDED, border_color.a], uv_rect: [0.0, 0.0, 1.0, 1.0] }
        }

        /// 🌀️ Clockwise spinning + pulsing loading ring in `color`; the sweep and pulse phase come from `globals._pad.x` (elapsed seconds) in `UI_SHADER`.
        pub fn loading_border(rect: [f32; 4], color: Rgba, radius: f32, border: f32) -> Self {
            Self { rect, color: [color.r, color.g, color.b, color.a], params: [radius, border, KIND_LOADING_BORDER, 0.0], uv_rect: [0.0, 0.0, 1.0, 1.0] }
        }

        /// 🌀️ Dashed, slow-spinning + gently pulsing waiting ring in `color`; the sweep and pulse phase come from `globals._pad.x` (elapsed seconds) in `UI_SHADER`.
        pub fn waiting_border(rect: [f32; 4], color: Rgba, radius: f32, border: f32) -> Self {
            Self { rect, color: [color.r, color.g, color.b, color.a], params: [radius, border, KIND_WAITING_BORDER, 0.0], uv_rect: [0.0, 0.0, 1.0, 1.0] }
        }

        /// ✅️ Solid, static at-bounds ring for `UiStatus::Finished` in `color` — no animation.
        pub fn finished_border(rect: [f32; 4], color: Rgba, radius: f32, border: f32) -> Self {
            Self { rect, color: [color.r, color.g, color.b, color.a], params: [radius, border, KIND_FINISHED_BORDER, 0.0], uv_rect: [0.0, 0.0, 1.0, 1.0] }
        }

        /// 💫️ Raised-cosine breathing pulse ring for `UiState::Introducing` in `color`; phase comes from `globals._pad.x` in `UI_SHADER`.
        pub fn introducing_border(rect: [f32; 4], color: Rgba, radius: f32, border: f32) -> Self {
            Self { rect, color: [color.r, color.g, color.b, color.a], params: [radius, border, KIND_INTRODUCING_BORDER, 0.0], uv_rect: [0.0, 0.0, 1.0, 1.0] }
        }

        pub fn glyph(rect: [f32; 4], color: Rgba, uv_rect: [f32; 4]) -> Self {
            Self { rect, color: [color.r, color.g, color.b, color.a], params: [0.0, 0.0, KIND_GLYPH, 0.0], uv_rect }
        }

        pub fn textured(rect: [f32; 4], uv_rect: [f32; 4], color: Rgba) -> Self {
            Self { rect, color: [color.r, color.g, color.b, color.a], params: [0.0, 0.0, KIND_TEXTURED, 0.0], uv_rect }
        }

        pub fn raster(rect: [f32; 4], uv_rect: [f32; 4], alpha: f32) -> Self {
            Self { rect, color: [1.0, 1.0, 1.0, alpha], params: [0.0, 0.0, KIND_RASTER, 0.0], uv_rect }
        }
    }

    #[repr(C)]
    #[derive(Clone, Copy, Debug, Pod, Zeroable)]
    pub struct VectorVertex {
        pub position: [f32; 2],
        pub color: [f32; 4],
    }

    #[derive(Clone, Copy, Debug)]
    pub struct ScissorRect {
        pub x: u32,
        pub y: u32,
        pub w: u32,
        pub h: u32,
    }

    impl ScissorRect {
        pub fn from_rect(rect: crate::geometry::Rect, _screen_h: f32) -> Self {
            let x = rect.x.max(0.0) as u32;
            let y = rect.y.max(0.0) as u32;
            let w = rect.w.max(0.0) as u32;
            let h = rect.h.max(0.0) as u32;
            Self { x, y, w, h }
        }

        pub fn intersect(&self, other: &Self) -> Self {
            let x0 = self.x.max(other.x);
            let y0 = self.y.max(other.y);
            let x1 = (self.x + self.w).min(other.x + other.w);
            let y1 = (self.y + self.h).min(other.y + other.h);
            Self { x: x0, y: y0, w: x1.saturating_sub(x0), h: y1.saturating_sub(y0) }
        }
    }

    #[derive(Default)]
    pub struct DrawLayer {
        pub scissor: Option<ScissorRect>,
        pub foreground_of: Option<usize>,
        pub ui_instances: Vec<UiInstance>,
        pub raster_instances: Vec<(String, UiInstance)>,
        pub vector_vertices: Vec<VectorVertex>,
        pub overlay_ui_instances: Vec<UiInstance>,
        pub overlay_vector_vertices: Vec<VectorVertex>,
    }

    pub struct DrawList {
        pub scene_passes: Vec<ScenePass3d>,
        pub layers: Vec<DrawLayer>,
        pub glass_regions: Vec<GlassRegion>,
        scissor_stack: Vec<ScissorRect>,
        glass_content_stack: Vec<usize>,
        screen_h: f32,
    }

    impl Default for DrawList {
        fn default() -> Self {
            let mut list = Self { scene_passes: Vec::new(), layers: Vec::new(), glass_regions: Vec::new(), scissor_stack: Vec::new(), glass_content_stack: Vec::new(), screen_h: 720.0 };
            list.layers.push(DrawLayer::default());
            list
        }
    }

    impl DrawList {
        pub fn set_screen_height(&mut self, height: f32) {
            self.screen_h = height;
        }

        fn active_foreground_of(&self) -> Option<usize> {
            self.glass_content_stack.last().copied()
        }

        fn active_layer(&mut self) -> &mut DrawLayer {
            if self.layers.is_empty() {
                self.layers.push(DrawLayer::default());
            }
            self.layers.last_mut().expect("layer")
        }

        pub fn clear(&mut self) {
            self.scene_passes.clear();
            self.layers.clear();
            self.layers.push(DrawLayer::default());
            self.glass_regions.clear();
            self.scissor_stack.clear();
            self.glass_content_stack.clear();
        }

        pub fn push_scissor(&mut self, rect: crate::geometry::Rect) {
            let mut scissor = ScissorRect::from_rect(rect, self.screen_h);
            if let Some(parent) = self.scissor_stack.last() {
                scissor = parent.intersect(&scissor);
            }
            self.scissor_stack.push(scissor);
            self.layers.push(DrawLayer { scissor: Some(scissor), foreground_of: self.active_foreground_of(), ..DrawLayer::default() });
        }

        pub fn pop_scissor(&mut self) {
            self.scissor_stack.pop();
            let parent = self.scissor_stack.last().copied();
            self.layers.push(DrawLayer { scissor: parent, foreground_of: self.active_foreground_of(), ..DrawLayer::default() });
        }

        pub fn push_scene_pass(&mut self, mut pass: ScenePass3d) {
            if self.layers.is_empty() {
                self.layers.push(DrawLayer::default());
            }
            let layer_index = self.layers.len() - 1;
            let layer = &self.layers[layer_index];
            pass.layer_index = layer_index;
            pass.ui_watermark = layer.ui_instances.len();
            pass.vector_watermark = layer.vector_vertices.len();
            self.scene_passes.push(pass);
        }

        pub fn push_solid(&mut self, rect: [f32; 4], color: Rgba) {
            self.active_layer().ui_instances.push(UiInstance::solid(rect, color));
        }

        pub fn push_rounded(&mut self, rect: [f32; 4], color: Rgba, radius: f32) {
            self.active_layer().ui_instances.push(UiInstance::rounded(rect, color, radius, 0.0, color));
        }

        /// 🌀️ Clockwise spinning + pulsing loading ring around `rect`, in `color` (gray `theme.border_normal` at rest, `theme.selected` when the node is selected/active).
        pub fn push_loading_border(&mut self, rect: [f32; 4], color: Rgba, radius: f32, stroke: f32) {
            self.active_layer().ui_instances.push(UiInstance::loading_border(rect, color, radius, stroke));
        }

        /// 🌀️ Dashed, slow-spinning + gently pulsing waiting ring around `rect`, in `color` (gray `theme.border_normal` at rest, `theme.selected` when the node is selected/active).
        pub fn push_waiting_border(&mut self, rect: [f32; 4], color: Rgba, radius: f32, stroke: f32) {
            self.active_layer().ui_instances.push(UiInstance::waiting_border(rect, color, radius, stroke));
        }

        /// ✅️ Solid, static at-bounds ring around `rect`, in `color` — `UiStatus::Finished`.
        pub fn push_finished_border(&mut self, rect: [f32; 4], color: Rgba, radius: f32, stroke: f32) {
            self.active_layer().ui_instances.push(UiInstance::finished_border(rect, color, radius, stroke));
        }

        /// 💫️ Raised-cosine breathing pulse ring around `rect`, in `color` — `UiState::Introducing`.
        pub fn push_introducing_border(&mut self, rect: [f32; 4], color: Rgba, radius: f32, stroke: f32) {
            self.active_layer().ui_instances.push(UiInstance::introducing_border(rect, color, radius, stroke));
        }

        /// 🧊️ Pushes a glass region rendered with an already-resolved `style` — callers derive `style`
        /// from `Theme::glass(level)` themselves (see
        /// `.🦑️repo/🎫️tickets/26/07/27/UNIFIED-6-LEVEL-UI-SURFACE-SYSTEM/contract.txt`) rather than this method
        /// picking a per-tier lookup.
        pub fn push_glass(&mut self, rect: [f32; 4], radius: f32, style: GlassStyle) -> usize {
            let index = self.glass_regions.len();
            self.glass_regions.push(GlassRegion { rect, radius, tint: style.tint, alpha: style.alpha, blur_px: style.blur_px, saturate: style.saturate });
            index
        }

        pub fn begin_glass_content(&mut self, region: usize) {
            self.glass_content_stack.push(region);
            self.layers.push(DrawLayer { scissor: None, foreground_of: Some(region), ..DrawLayer::default() });
        }

        pub fn end_glass_content(&mut self) {
            self.glass_content_stack.pop();
            self.layers.push(DrawLayer { scissor: self.scissor_stack.last().copied(), foreground_of: self.active_foreground_of(), ..DrawLayer::default() });
        }

        pub fn push_glyph(&mut self, rect: [f32; 4], color: Rgba, uv_rect: [f32; 4]) {
            self.active_layer().ui_instances.push(UiInstance::glyph(rect, color, uv_rect));
        }

        pub fn push_glyph_overlay(&mut self, rect: [f32; 4], color: Rgba, uv_rect: [f32; 4]) {
            self.active_layer().overlay_ui_instances.push(UiInstance::glyph(rect, color, uv_rect));
        }

        pub fn push_solid_overlay(&mut self, rect: [f32; 4], color: Rgba) {
            self.active_layer().overlay_ui_instances.push(UiInstance::solid(rect, color));
        }

        pub fn push_textured(&mut self, rect: [f32; 4], uv_rect: [f32; 4], color: Rgba) {
            self.active_layer().ui_instances.push(UiInstance::textured(rect, uv_rect, color));
        }

        pub fn push_raster_quad(&mut self, key: &str, rect: [f32; 4], uv_rect: [f32; 4], alpha: f32) {
            self.active_layer().raster_instances.push((key.to_string(), UiInstance::raster(rect, uv_rect, alpha)));
        }

        pub fn push_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: Rgba, width: f32) {
            let dx = x1 - x0;
            let dy = y1 - y0;
            let len = (dx * dx + dy * dy).sqrt().max(0.001);
            let nx = -dy / len * width * 0.5;
            let ny = dx / len * width * 0.5;
            let c = [color.r, color.g, color.b, color.a];
            let layer = self.active_layer();
            layer.vector_vertices.extend_from_slice(&[
                VectorVertex { position: [x0 + nx, y0 + ny], color: c },
                VectorVertex { position: [x1 + nx, y1 + ny], color: c },
                VectorVertex { position: [x0 - nx, y0 - ny], color: c },
                VectorVertex { position: [x1 + nx, y1 + ny], color: c },
                VectorVertex { position: [x1 - nx, y1 - ny], color: c },
                VectorVertex { position: [x0 - nx, y0 - ny], color: c },
            ]);
        }

        pub fn push_line_overlay(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: Rgba, width: f32) {
            let dx = x1 - x0;
            let dy = y1 - y0;
            let len = (dx * dx + dy * dy).sqrt().max(0.001);
            let nx = -dy / len * width * 0.5;
            let ny = dx / len * width * 0.5;
            let c = [color.r, color.g, color.b, color.a];
            let layer = self.active_layer();
            layer.overlay_vector_vertices.extend_from_slice(&[
                VectorVertex { position: [x0 + nx, y0 + ny], color: c },
                VectorVertex { position: [x1 + nx, y1 + ny], color: c },
                VectorVertex { position: [x0 - nx, y0 - ny], color: c },
                VectorVertex { position: [x1 + nx, y1 + ny], color: c },
                VectorVertex { position: [x1 - nx, y1 - ny], color: c },
                VectorVertex { position: [x0 - nx, y0 - ny], color: c },
            ]);
        }

        pub fn push_triangle_fan(&mut self, points: &[[f32; 2]], color: Rgba) {
            if points.len() < 3 {
                return;
            }
            let c = [color.r, color.g, color.b, color.a];
            let layer = self.active_layer();
            for tri in 1..points.len() - 1 {
                layer.vector_vertices.push(VectorVertex { position: points[0], color: c });
                layer.vector_vertices.push(VectorVertex { position: points[tri], color: c });
                layer.vector_vertices.push(VectorVertex { position: points[tri + 1], color: c });
            }
        }

        pub fn push_triangle_fan_overlay(&mut self, points: &[[f32; 2]], color: Rgba) {
            if points.len() < 3 {
                return;
            }
            let c = [color.r, color.g, color.b, color.a];
            let layer = self.active_layer();
            for tri in 1..points.len() - 1 {
                layer.overlay_vector_vertices.push(VectorVertex { position: points[0], color: c });
                layer.overlay_vector_vertices.push(VectorVertex { position: points[tri], color: c });
                layer.overlay_vector_vertices.push(VectorVertex { position: points[tri + 1], color: c });
            }
        }

        #[allow(clippy::too_many_arguments, reason = "one arg per line endpoint/style attribute; grouping into a struct is a T2 restructure, out of scope")]
        pub fn push_dashed_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: Rgba, width: f32, dash: f32, gap: f32) {
            for (sx0, sy0, sx1, sy1) in dashed_line_segments(x0, y0, x1, y1, dash, gap) {
                self.push_line(sx0, sy0, sx1, sy1, color, width);
            }
        }

        #[allow(clippy::too_many_arguments, reason = "one arg per line endpoint/style attribute; grouping into a struct is a T2 restructure, out of scope")]
        pub fn push_dashed_line_overlay(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: Rgba, width: f32, dash: f32, gap: f32) {
            for (sx0, sy0, sx1, sy1) in dashed_line_segments(x0, y0, x1, y1, dash, gap) {
                self.push_line_overlay(sx0, sy0, sx1, sy1, color, width);
            }
        }
    }

    pub const SELECTION_MARQUEE_STROKE_WIDTH: f32 = 1.5;
    pub const SELECTION_MARQUEE_FILL_ALPHA: f32 = 0.12;
    pub const SELECTION_MARQUEE_DASH_LEN: f32 = 5.0;
    pub const SELECTION_MARQUEE_DASH_GAP: f32 = 4.0;

    pub fn selection_marquee_stroke(theme: &Theme) -> Rgba {
        theme.selected
    }

    pub fn selection_marquee_fill(theme: &Theme) -> Rgba {
        theme.selected.with_alpha(SELECTION_MARQUEE_FILL_ALPHA)
    }

    fn dashed_line_segments(x0: f32, y0: f32, x1: f32, y1: f32, dash: f32, gap: f32) -> Vec<(f32, f32, f32, f32)> {
        let dx = x1 - x0;
        let dy = y1 - y0;
        let len = (dx * dx + dy * dy).sqrt().max(0.001);
        let ux = dx / len;
        let uy = dy / len;
        let mut traveled = 0.0f32;
        let mut drawing = true;
        let mut segments = Vec::new();
        while traveled < len {
            let segment = if drawing { dash } else { gap };
            let next = (traveled + segment).min(len);
            if drawing {
                segments.push((x0 + ux * traveled, y0 + uy * traveled, x0 + ux * next, y0 + uy * next));
            }
            traveled = next;
            drawing = !drawing;
        }
        segments
    }

    #[cfg(test)]
    mod selection_marquee_tests {
        use super::*;
        use crate::theme::Theme;

        #[test]
        fn dashed_line_segments_emit_dashes_along_segment() {
            let segments = dashed_line_segments(0.0, 0.0, 20.0, 0.0, 5.0, 4.0);
            assert!(!segments.is_empty());
            let span: f32 = segments.iter().map(|(x0, _, x1, _)| x1 - x0).sum();
            assert!(span > 0.0 && span <= 20.0);
        }

        #[test]
        fn selection_marquee_colors_use_active_token_only() {
            let theme = Theme::default();
            assert_eq!(selection_marquee_stroke(&theme), theme.selected);
            assert_eq!(selection_marquee_fill(&theme).a, SELECTION_MARQUEE_FILL_ALPHA);
        }
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per line endpoint/style attribute; grouping into a struct is a T2 restructure, out of scope")]
    fn push_marquee_segment(draw: &mut DrawList, overlay: bool, x0: f32, y0: f32, x1: f32, y1: f32, stroke: Rgba, dashed: bool) {
        if dashed {
            if overlay {
                draw.push_dashed_line_overlay(x0, y0, x1, y1, stroke, SELECTION_MARQUEE_STROKE_WIDTH, SELECTION_MARQUEE_DASH_LEN, SELECTION_MARQUEE_DASH_GAP);
            } else {
                draw.push_dashed_line(x0, y0, x1, y1, stroke, SELECTION_MARQUEE_STROKE_WIDTH, SELECTION_MARQUEE_DASH_LEN, SELECTION_MARQUEE_DASH_GAP);
            }
        } else if overlay {
            draw.push_line_overlay(x0, y0, x1, y1, stroke, SELECTION_MARQUEE_STROKE_WIDTH);
        } else {
            draw.push_line(x0, y0, x1, y1, stroke, SELECTION_MARQUEE_STROKE_WIDTH);
        }
    }

    pub fn paint_selection_marquee(draw: &mut DrawList, theme: &Theme, crossing: bool, lasso: bool, points: &[[f32; 2]], overlay: bool) {
        if points.len() < 2 {
            return;
        }
        let stroke = selection_marquee_stroke(theme);
        let fill = selection_marquee_fill(theme);
        let dashed = crossing;
        if lasso {
            if points.len() >= 3 {
                if overlay {
                    draw.push_triangle_fan_overlay(points, fill);
                } else {
                    draw.push_triangle_fan(points, fill);
                }
            }
            for window in points.windows(2) {
                push_marquee_segment(draw, overlay, window[0][0], window[0][1], window[1][0], window[1][1], stroke, dashed);
            }
            let first = points[0];
            let last = points[points.len() - 1];
            push_marquee_segment(draw, overlay, last[0], last[1], first[0], first[1], stroke, dashed);
            return;
        }
        let start = points[0];
        let end = points[points.len() - 1];
        let rx = start[0].min(end[0]);
        let ry = start[1].min(end[1]);
        let rw = (end[0] - start[0]).abs();
        let rh = (end[1] - start[1]).abs();
        if overlay {
            draw.push_solid_overlay([rx, ry, rw, rh], fill);
        } else {
            draw.push_solid([rx, ry, rw, rh], fill);
        }
        push_marquee_segment(draw, overlay, start[0], start[1], end[0], start[1], stroke, dashed);
        push_marquee_segment(draw, overlay, end[0], start[1], end[0], end[1], stroke, dashed);
        push_marquee_segment(draw, overlay, end[0], end[1], start[0], end[1], stroke, dashed);
        push_marquee_segment(draw, overlay, start[0], end[1], start[0], start[1], stroke, dashed);
    }

    pub fn ear_clip_polygon(points: &[[f32; 2]]) -> Vec<[f32; 2]> {
        if points.len() < 3 {
            return Vec::new();
        }
        let mut indices: Vec<usize> = (0..points.len()).collect();
        let mut triangles = Vec::new();
        let mut guard = 0usize;
        while indices.len() > 3 && guard < points.len() * points.len() {
            guard += 1;
            let mut ear_found = false;
            for i in 0..indices.len() {
                let prev = indices[(i + indices.len() - 1) % indices.len()];
                let curr = indices[i];
                let next = indices[(i + 1) % indices.len()];
                let a = points[prev];
                let b = points[curr];
                let c = points[next];
                let cross = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
                if cross <= 0.0 {
                    continue;
                }
                let mut contains = false;
                for &idx in &indices {
                    if idx == prev || idx == curr || idx == next {
                        continue;
                    }
                    let p = points[idx];
                    if point_in_triangle(p, a, b, c) {
                        contains = true;
                        break;
                    }
                }
                if contains {
                    continue;
                }
                triangles.push(a);
                triangles.push(b);
                triangles.push(c);
                indices.remove(i);
                ear_found = true;
                break;
            }
            if !ear_found {
                break;
            }
        }
        if indices.len() == 3 {
            triangles.push(points[indices[0]]);
            triangles.push(points[indices[1]]);
            triangles.push(points[indices[2]]);
        }
        triangles
    }

    fn point_in_triangle(p: [f32; 2], a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> bool {
        let d1 = sign(p, a, b);
        let d2 = sign(p, b, c);
        let d3 = sign(p, c, a);
        let has_neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
        let has_pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
        !(has_neg && has_pos)
    }

    #[repr(C)]
    #[derive(Clone, Copy, Debug, Pod, Zeroable)]
    pub struct World3dVertex {
        pub position: [f32; 3],
        pub normal: [f32; 3],
    }

    #[repr(C)]
    #[derive(Clone, Copy, Debug, Pod, Zeroable)]
    pub struct World3dGlobals {
        pub view_proj: [f32; 16],
        pub light_dir: [f32; 4],
    }

    #[repr(C)]
    #[derive(Clone, Copy, Debug, Pod, Zeroable)]
    pub struct World3dGpuInstance {
        pub model0: [f32; 4],
        pub model1: [f32; 4],
        pub model2: [f32; 4],
        pub model3: [f32; 4],
        pub color: [f32; 4],
        pub flags: [f32; 4],
    }

    impl World3dGpuInstance {
        pub fn from_instance(model: [f32; 16], color: [f32; 4], selected: bool, hovered: bool) -> Self {
            Self {
                model0: [model[0], model[1], model[2], model[3]],
                model1: [model[4], model[5], model[6], model[7]],
                model2: [model[8], model[9], model[10], model[11]],
                model3: [model[12], model[13], model[14], model[15]],
                color,
                flags: [if selected { 1.0 } else { 0.0 }, if hovered { 1.0 } else { 0.0 }, 0.0, 0.0],
            }
        }
    }

    pub struct GpuMeshBuffers {
        pub vertex_buffer: wgpu::Buffer,
        pub index_buffer: wgpu::Buffer,
        pub index_count: u32,
    }

    #[derive(Default)]
    pub struct MeshGpuStore {
        meshes: std::collections::HashMap<String, GpuMeshBuffers>,
    }

    pub fn mesh_content_version(positions: &[f32], normals: &[f32], indices: &[u32]) -> u64 {
        let mut hash = 0xcbf29ce484222325u64;
        for value in positions.iter().chain(normals.iter()) {
            hash ^= value.to_bits() as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        for value in indices {
            hash ^= *value as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }

    impl MeshGpuStore {
        pub fn get(&self, key: &str) -> Option<&GpuMeshBuffers> {
            self.meshes.get(key)
        }

        pub fn lookup_key(mesh_key: &str, version: u64) -> String {
            format!("{mesh_key}:{version}")
        }

        pub fn get_versioned(&self, mesh_key: &str, version: u64) -> Option<&GpuMeshBuffers> {
            self.get(&Self::lookup_key(mesh_key, version))
        }

        pub fn ensure_mesh(&mut self, device: &wgpu::Device, key: &str, version: u64, positions: &[f32], normals: &[f32], indices: &[u32]) {
            let store_key = format!("{key}:{version}");
            if self.meshes.contains_key(&store_key) {
                return;
            }
            let prefix = format!("{key}:");
            self.meshes.retain(|existing, _| !existing.starts_with(&prefix) || existing == &store_key);
            let mut vertices = Vec::with_capacity(positions.len() / 3);
            for index in 0..positions.len() / 3 {
                vertices.push(World3dVertex {
                    position: [positions[index * 3], positions[index * 3 + 1], positions[index * 3 + 2]],
                    normal: [normals.get(index * 3).copied().unwrap_or(0.0), normals.get(index * 3 + 1).copied().unwrap_or(1.0), normals.get(index * 3 + 2).copied().unwrap_or(0.0)],
                });
            }
            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("world3d_vertices"), contents: bytemuck::cast_slice(&vertices), usage: wgpu::BufferUsages::VERTEX });
            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("world3d_indices"), contents: bytemuck::cast_slice(indices), usage: wgpu::BufferUsages::INDEX });
            self.meshes.insert(store_key, GpuMeshBuffers { vertex_buffer, index_buffer, index_count: indices.len() as u32 });
        }

        pub fn evict_mesh(&mut self, key: &str) {
            let prefix = format!("{key}:");
            self.meshes.retain(|existing, _| !existing.starts_with(&prefix));
        }
    }

    pub const WORLD_GLOBALS_SLOT_SIZE: u64 = 256;

    #[derive(Default)]
    pub struct GrowBuffer {
        buffer: Option<wgpu::Buffer>,
        capacity: usize,
    }

    impl GrowBuffer {
        pub fn slice(&self) -> Option<wgpu::BufferSlice<'_>> {
            self.buffer.as_ref().map(|buffer| buffer.slice(..))
        }

        pub fn upload<T: Pod>(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, data: &[T], usage: wgpu::BufferUsages, label: &str) -> Option<wgpu::BufferSlice<'_>> {
            if data.is_empty() {
                return None;
            }
            let bytes = bytemuck::cast_slice(data);
            let required = bytes.len();
            if self.capacity < required {
                self.capacity = required.next_power_of_two().max(256);
                self.buffer = Some(device.create_buffer(&wgpu::BufferDescriptor { label: Some(label), size: self.capacity as u64, usage, mapped_at_creation: false }));
            }
            let buffer = self.buffer.as_ref()?;
            queue.write_buffer(buffer, 0, bytes);
            Some(buffer.slice(..))
        }
    }

    #[derive(Default)]
    pub struct FrameBuffers {
        pub world_instances: GrowBuffer,
        pub world_lines: GrowBuffer,
        pub ui_instances: GrowBuffer,
        pub vector_vertices: GrowBuffer,
        pub glass_instances: GrowBuffer,
    }

    #[repr(C)]
    #[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
    struct WorldLineGpuVertex {
        position: [f32; 3],
        color: [f32; 4],
    }

    struct WorldDrawRange {
        mesh_key: String,
        mesh_version: u64,
        instance_offset: u32,
        instance_count: u32,
    }

    struct PreparedWorldPass {
        globals: World3dGlobals,
        viewport: [f32; 4],
        draws: Vec<WorldDrawRange>,
        translucent_draws: Vec<WorldDrawRange>,
        line_start: u32,
        line_count: u32,
    }

    struct WorldGlobalsRing {
        buffer: wgpu::Buffer,
        bind_group: wgpu::BindGroup,
        slot_stride: u32,
        capacity_slots: u32,
    }

    impl WorldGlobalsRing {
        fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, initial_slots: u32) -> Self {
            let slot_stride = WORLD_GLOBALS_SLOT_SIZE as u32;
            let capacity_slots = initial_slots.max(1);
            let buffer =
                device.create_buffer(&wgpu::BufferDescriptor { label: Some("world3d_globals_ring"), size: slot_stride as u64 * capacity_slots as u64, usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("world3d_bind_group"),
                layout,
                entries: &[wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding { buffer: &buffer, offset: 0, size: std::num::NonZeroU64::new(size_of::<World3dGlobals>() as u64) }) }],
            });
            Self { buffer, bind_group, slot_stride, capacity_slots }
        }

        fn ensure_slots(&mut self, device: &wgpu::Device, layout: &wgpu::BindGroupLayout, slots: u32) {
            if slots <= self.capacity_slots {
                return;
            }
            self.capacity_slots = slots.next_power_of_two().max(4);
            self.buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("world3d_globals_ring"),
                size: self.slot_stride as u64 * self.capacity_slots as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            self.bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("world3d_bind_group"),
                layout,
                entries: &[wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding { buffer: &self.buffer, offset: 0, size: std::num::NonZeroU64::new(size_of::<World3dGlobals>() as u64) }) }],
            });
        }

        fn write_passes(&self, queue: &wgpu::Queue, passes: &[World3dGlobals]) {
            for (index, globals) in passes.iter().enumerate() {
                let offset = (index as u64) * self.slot_stride as u64;
                queue.write_buffer(&self.buffer, offset, bytemuck::bytes_of(globals));
            }
        }

        fn offset_for_slot(&self, slot: u32) -> u32 {
            slot * self.slot_stride
        }
    }

    fn sign(p1: [f32; 2], p2: [f32; 2], p3: [f32; 2]) -> f32 {
        (p1[0] - p3[0]) * (p2[1] - p3[1]) - (p2[0] - p3[0]) * (p1[1] - p3[1])
    }

    pub const ICON_ATLAS_TEXTURE_SIZE: u32 = 2048;

    pub struct IconAtlas {
        pub width: u32,
        pub height: u32,
        pub pixels: Vec<u8>,
        entries: std::collections::HashMap<String, [f32; 4]>,
    }

    impl Default for IconAtlas {
        fn default() -> Self {
            Self { width: 1, height: 1, pixels: vec![0, 0, 0, 0], entries: std::collections::HashMap::new() }
        }
    }

    impl IconAtlas {
        pub fn from_packed(width: u32, height: u32, pixels: Vec<u8>, entries: Vec<(String, [f32; 4])>) -> Self {
            Self { width, height, pixels, entries: entries.into_iter().collect() }
        }

        pub fn icon_uv(&self, icon_id: &str) -> Option<[f32; 4]> {
            self.entries.get(icon_id).copied()
        }
    }

    pub struct RasterTexture {
        pub texture: wgpu::Texture,
        pub bind_group: wgpu::BindGroup,
        pub width: u32,
        pub height: u32,
    }

    pub struct RasterTextureStore {
        textures: std::collections::HashMap<String, RasterTexture>,
        layout: wgpu::BindGroupLayout,
        sampler: wgpu::Sampler,
    }

    impl RasterTextureStore {
        pub fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> Self {
            let sampler = device.create_sampler(&wgpu::SamplerDescriptor { label: Some("raster_sampler"), mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Linear, ..Default::default() });
            Self { textures: std::collections::HashMap::new(), layout: layout.clone(), sampler }
        }

        #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
        pub fn ensure_raster(
            &mut self,
            device: &wgpu::Device,
            queue: &wgpu::Queue,
            globals_buffer: &wgpu::Buffer,
            glyph_view: &wgpu::TextureView,
            glyph_sampler: &wgpu::Sampler,
            _icon_view: &wgpu::TextureView,
            _icon_sampler: &wgpu::Sampler,
            key: &str,
            pixels: &[u8],
            width: u32,
            height: u32,
        ) {
            if let Some(existing) = self.textures.get(key) {
                queue.write_texture(
                    wgpu::TexelCopyTextureInfo { texture: &existing.texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
                    pixels,
                    wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(width * 4), rows_per_image: Some(height) },
                    wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
                );
                return;
            }
            let texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("raster_texture"),
                size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            queue.write_texture(
                wgpu::TexelCopyTextureInfo { texture: &texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
                pixels,
                wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(width * 4), rows_per_image: Some(height) },
                wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            );
            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("raster_texture_bind_group"),
                layout: &self.layout,
                entries: &[
                    wgpu::BindGroupEntry { binding: 0, resource: globals_buffer.as_entire_binding() },
                    wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(glyph_view) },
                    wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(glyph_sampler) },
                    wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&view) },
                    wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::Sampler(&self.sampler) },
                ],
            });
            self.textures.insert(key.to_string(), RasterTexture { texture, bind_group, width, height });
        }

        pub fn get(&self, key: &str) -> Option<&RasterTexture> {
            self.textures.get(key)
        }

        #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
        pub fn replace_gpu_bind_group(
            &mut self,
            device: &wgpu::Device,
            globals_buffer: &wgpu::Buffer,
            glyph_view: &wgpu::TextureView,
            glyph_sampler: &wgpu::Sampler,
            key: &str,
            raster_view: &wgpu::TextureView,
            texture: wgpu::Texture,
            width: u32,
            height: u32,
        ) {
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("raster_bind_group"),
                layout: &self.layout,
                entries: &[
                    wgpu::BindGroupEntry { binding: 0, resource: globals_buffer.as_entire_binding() },
                    wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(glyph_view) },
                    wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(glyph_sampler) },
                    wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(raster_view) },
                    wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::Sampler(&self.sampler) },
                ],
            });
            self.textures.insert(key.to_string(), RasterTexture { texture, bind_group, width, height });
        }
    }

    pub(crate) struct UiPipelines {
        ui_pipeline: wgpu::RenderPipeline,
        vector_pipeline: wgpu::RenderPipeline,
        world_pipeline: wgpu::RenderPipeline,
        world_pipeline_translucent: wgpu::RenderPipeline,
        world_line_pipeline: wgpu::RenderPipeline,
        blur_downsample_pipeline: wgpu::RenderPipeline,
        scene_blit_pipeline: wgpu::RenderPipeline,
        glass_pipeline: wgpu::RenderPipeline,
        quad_vertex_buffer: wgpu::Buffer,
        globals_buffer: wgpu::Buffer,
        blur_globals_buffer: wgpu::Buffer,
        world_globals_ring: WorldGlobalsRing,
        world_bind_group_layout: wgpu::BindGroupLayout,
        blur_bind_group_layout: wgpu::BindGroupLayout,
        scene_bind_group_layout: wgpu::BindGroupLayout,
        glyph_texture: wgpu::Texture,
        glyph_sampler: wgpu::Sampler,
        icon_texture: wgpu::Texture,
        icon_sampler: wgpu::Sampler,
        glyph_bind_group: wgpu::BindGroup,
        bind_group_layout: wgpu::BindGroupLayout,
    }

    struct LayerBatch {
        layer_index: usize,
        scissor: Option<ScissorRect>,
        ui_start: u32,
        ui_count: u32,
        vec_start: u32,
        vec_count: u32,
    }

    enum LayerBatchFilter {
        Backdrop,
        Foreground,
    }

    impl Copy for LayerBatchFilter {}

    impl Clone for LayerBatchFilter {
        fn clone(&self) -> Self {
            *self
        }
    }

    fn layer_matches_filter(layer: &DrawLayer, filter: LayerBatchFilter) -> bool {
        match filter {
            LayerBatchFilter::Backdrop => layer.foreground_of.is_none(),
            LayerBatchFilter::Foreground => layer.foreground_of.is_some(),
        }
    }

    fn build_layer_batches(draw: &DrawList, filter: LayerBatchFilter) -> (Vec<UiInstance>, Vec<VectorVertex>, Vec<LayerBatch>) {
        let mut all_ui = Vec::new();
        let mut all_vec = Vec::new();
        let mut batches = Vec::new();
        let scene_layers: std::collections::HashSet<usize> = draw.scene_passes.iter().filter(|pass| layer_matches_filter(&draw.layers[pass.layer_index], filter)).map(|pass| pass.layer_index).collect();
        for (layer_index, layer) in draw.layers.iter().enumerate() {
            if !layer_matches_filter(layer, filter) {
                continue;
            }
            if layer.ui_instances.is_empty() && layer.vector_vertices.is_empty() && !scene_layers.contains(&layer_index) {
                continue;
            }
            let ui_start = all_ui.len() as u32;
            all_ui.extend_from_slice(&layer.ui_instances);
            let vec_start = all_vec.len() as u32;
            all_vec.extend_from_slice(&layer.vector_vertices);
            batches.push(LayerBatch { layer_index, scissor: layer.scissor, ui_start, ui_count: layer.ui_instances.len() as u32, vec_start, vec_count: layer.vector_vertices.len() as u32 });
        }
        (all_ui, all_vec, batches)
    }

    fn build_overlay_layer_batches(draw: &DrawList, filter: LayerBatchFilter) -> (Vec<UiInstance>, Vec<VectorVertex>, Vec<LayerBatch>) {
        let mut all_ui = Vec::new();
        let mut all_vec = Vec::new();
        let mut batches = Vec::new();
        for (layer_index, layer) in draw.layers.iter().enumerate() {
            if !layer_matches_filter(layer, filter) {
                continue;
            }
            if layer.overlay_ui_instances.is_empty() && layer.overlay_vector_vertices.is_empty() {
                continue;
            }
            let ui_start = all_ui.len() as u32;
            all_ui.extend_from_slice(&layer.overlay_ui_instances);
            let vec_start = all_vec.len() as u32;
            all_vec.extend_from_slice(&layer.overlay_vector_vertices);
            batches.push(LayerBatch { layer_index, scissor: layer.scissor, ui_start, ui_count: layer.overlay_ui_instances.len() as u32, vec_start, vec_count: layer.overlay_vector_vertices.len() as u32 });
        }
        (all_ui, all_vec, batches)
    }

    fn set_pass_scissor(pass: &mut wgpu::RenderPass<'_>, scissor: Option<ScissorRect>, width: f32, height: f32) {
        if let Some(scissor) = scissor {
            pass.set_scissor_rect(scissor.x, scissor.y, scissor.w, scissor.h);
        } else {
            pass.set_scissor_rect(0, 0, width as u32, height as u32);
        }
    }

    impl UiPipelines {
        pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
            let globals_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("ui_globals_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry { binding: 4, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
                ],
            });

            let glyph_bind_group_layout = globals_bind_group_layout.clone();
            let _ = glyph_bind_group_layout;

            let ui_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("ui_shader"), source: wgpu::ShaderSource::Wgsl(UI_SHADER.into()) });
            let vector_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("vector_shader"), source: wgpu::ShaderSource::Wgsl(VECTOR_SHADER.into()) });
            let world_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("world3d_shader"), source: wgpu::ShaderSource::Wgsl(WORLD3D_SHADER.into()) });
            let world_lines_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("world3d_lines_shader"), source: wgpu::ShaderSource::Wgsl(WORLD3D_LINES_SHADER.into()) });

            let depth_state =
                Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth24Plus, depth_write_enabled: true, depth_compare: wgpu::CompareFunction::Less, stencil: wgpu::StencilState::default(), bias: wgpu::DepthBiasState::default() });
            let overlay_depth_state =
                Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth24Plus, depth_write_enabled: false, depth_compare: wgpu::CompareFunction::Always, stencil: wgpu::StencilState::default(), bias: wgpu::DepthBiasState::default() });

            let quad_vertices: &[f32] = &[0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0];
            let quad_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("ui_quad_vertices"), contents: bytemuck::cast_slice(quad_vertices), usage: wgpu::BufferUsages::VERTEX });

            let globals_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("ui_globals"),
                contents: bytemuck::bytes_of(&UiGlobals { screen_size: [1.0, 1.0], _pad: [0.0, 0.0] }),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

            let glyph_texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("glyph_atlas"),
                size: wgpu::Extent3d { width: ICON_ATLAS_TEXTURE_SIZE, height: ICON_ATLAS_TEXTURE_SIZE, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::R8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            let glyph_view = glyph_texture.create_view(&wgpu::TextureViewDescriptor::default());
            let glyph_sampler = device.create_sampler(&wgpu::SamplerDescriptor { label: Some("glyph_sampler"), mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Linear, ..Default::default() });
            let icon_texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("icon_atlas"),
                size: wgpu::Extent3d { width: ICON_ATLAS_TEXTURE_SIZE, height: ICON_ATLAS_TEXTURE_SIZE, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            let icon_view = icon_texture.create_view(&wgpu::TextureViewDescriptor::default());
            let icon_sampler = device.create_sampler(&wgpu::SamplerDescriptor { label: Some("icon_sampler"), mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Linear, ..Default::default() });
            let glyph_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("ui_bind_group"),
                layout: &globals_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry { binding: 0, resource: globals_buffer.as_entire_binding() },
                    wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&glyph_view) },
                    wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&glyph_sampler) },
                    wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&icon_view) },
                    wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::Sampler(&icon_sampler) },
                ],
            });
            let ui_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("ui_pipeline_layout"), bind_group_layouts: &[&globals_bind_group_layout], push_constant_ranges: &[] });
            let ui_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("ui_pipeline"),
                layout: Some(&ui_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &ui_shader,
                    entry_point: Some("vs_main"),
                    buffers: &[
                        wgpu::VertexBufferLayout { array_stride: 8, step_mode: wgpu::VertexStepMode::Vertex, attributes: &[wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x2 }] },
                        wgpu::VertexBufferLayout {
                            array_stride: size_of::<UiInstance>() as wgpu::BufferAddress,
                            step_mode: wgpu::VertexStepMode::Instance,
                            attributes: &[
                                wgpu::VertexAttribute { offset: 0, shader_location: 1, format: wgpu::VertexFormat::Float32x4 },
                                wgpu::VertexAttribute { offset: 16, shader_location: 2, format: wgpu::VertexFormat::Float32x4 },
                                wgpu::VertexAttribute { offset: 32, shader_location: 3, format: wgpu::VertexFormat::Float32x4 },
                                wgpu::VertexAttribute { offset: 48, shader_location: 4, format: wgpu::VertexFormat::Float32x4 },
                            ],
                        },
                    ],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &ui_shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState { format, blend: Some(wgpu::BlendState::ALPHA_BLENDING), write_mask: wgpu::ColorWrites::ALL })],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: overlay_depth_state.clone(),
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            });

            let vector_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("vector_pipeline_layout"), bind_group_layouts: &[&globals_bind_group_layout], push_constant_ranges: &[] });
            let vector_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("vector_pipeline"),
                layout: Some(&vector_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &vector_shader,
                    entry_point: Some("vs_main"),
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: size_of::<VectorVertex>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x2 }, wgpu::VertexAttribute { offset: 8, shader_location: 1, format: wgpu::VertexFormat::Float32x4 }],
                    }],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &vector_shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState { format, blend: Some(wgpu::BlendState::ALPHA_BLENDING), write_mask: wgpu::ColorWrites::ALL })],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: overlay_depth_state,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            });

            let world_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("world3d_globals_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: true, min_binding_size: std::num::NonZeroU64::new(size_of::<World3dGlobals>() as u64) },
                    count: None,
                }],
            });

            let world_globals_ring = WorldGlobalsRing::new(device, &world_bind_group_layout, 8);

            let world_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("world3d_pipeline_layout"), bind_group_layouts: &[&world_bind_group_layout], push_constant_ranges: &[] });
            let world_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("world3d_pipeline"),
                layout: Some(&world_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &world_shader,
                    entry_point: Some("vs_main"),
                    buffers: &[
                        wgpu::VertexBufferLayout {
                            array_stride: size_of::<World3dVertex>() as wgpu::BufferAddress,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &[wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x3 }, wgpu::VertexAttribute { offset: 12, shader_location: 1, format: wgpu::VertexFormat::Float32x3 }],
                        },
                        wgpu::VertexBufferLayout {
                            array_stride: size_of::<World3dGpuInstance>() as wgpu::BufferAddress,
                            step_mode: wgpu::VertexStepMode::Instance,
                            attributes: &[
                                wgpu::VertexAttribute { offset: 0, shader_location: 3, format: wgpu::VertexFormat::Float32x4 },
                                wgpu::VertexAttribute { offset: 16, shader_location: 4, format: wgpu::VertexFormat::Float32x4 },
                                wgpu::VertexAttribute { offset: 32, shader_location: 5, format: wgpu::VertexFormat::Float32x4 },
                                wgpu::VertexAttribute { offset: 48, shader_location: 6, format: wgpu::VertexFormat::Float32x4 },
                                wgpu::VertexAttribute { offset: 64, shader_location: 7, format: wgpu::VertexFormat::Float32x4 },
                                wgpu::VertexAttribute { offset: 80, shader_location: 8, format: wgpu::VertexFormat::Float32x4 },
                            ],
                        },
                    ],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &world_shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState { format, blend: Some(wgpu::BlendState::REPLACE), write_mask: wgpu::ColorWrites::ALL })],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState { cull_mode: None, ..Default::default() },
                depth_stencil: depth_state,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            });
            let translucent_depth_state = Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState { constant: -2, slope_scale: -1.0, clamp: 0.0 },
            });
            let world_line_depth_state =
                Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth24Plus, depth_write_enabled: false, depth_compare: wgpu::CompareFunction::LessEqual, stencil: wgpu::StencilState::default(), bias: wgpu::DepthBiasState::default() });
            let world_pipeline_translucent = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("world3d_pipeline_translucent"),
                layout: Some(&world_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &world_shader,
                    entry_point: Some("vs_main"),
                    buffers: &[
                        wgpu::VertexBufferLayout {
                            array_stride: size_of::<World3dVertex>() as wgpu::BufferAddress,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &[wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x3 }, wgpu::VertexAttribute { offset: 12, shader_location: 1, format: wgpu::VertexFormat::Float32x3 }],
                        },
                        wgpu::VertexBufferLayout {
                            array_stride: size_of::<World3dGpuInstance>() as wgpu::BufferAddress,
                            step_mode: wgpu::VertexStepMode::Instance,
                            attributes: &[
                                wgpu::VertexAttribute { offset: 0, shader_location: 3, format: wgpu::VertexFormat::Float32x4 },
                                wgpu::VertexAttribute { offset: 16, shader_location: 4, format: wgpu::VertexFormat::Float32x4 },
                                wgpu::VertexAttribute { offset: 32, shader_location: 5, format: wgpu::VertexFormat::Float32x4 },
                                wgpu::VertexAttribute { offset: 48, shader_location: 6, format: wgpu::VertexFormat::Float32x4 },
                                wgpu::VertexAttribute { offset: 64, shader_location: 7, format: wgpu::VertexFormat::Float32x4 },
                                wgpu::VertexAttribute { offset: 80, shader_location: 8, format: wgpu::VertexFormat::Float32x4 },
                            ],
                        },
                    ],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &world_shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState { format, blend: Some(wgpu::BlendState::ALPHA_BLENDING), write_mask: wgpu::ColorWrites::ALL })],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState { cull_mode: Some(wgpu::Face::Back), ..Default::default() },
                depth_stencil: translucent_depth_state,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            });
            let world_line_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("world3d_line_pipeline"),
                layout: Some(&world_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &world_lines_shader,
                    entry_point: Some("vs_main"),
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: size_of::<WorldLineGpuVertex>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x3 }, wgpu::VertexAttribute { offset: 12, shader_location: 1, format: wgpu::VertexFormat::Float32x4 }],
                    }],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &world_lines_shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState { format, blend: Some(wgpu::BlendState::ALPHA_BLENDING), write_mask: wgpu::ColorWrites::ALL })],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState { topology: wgpu::PrimitiveTopology::LineList, ..Default::default() },
                depth_stencil: world_line_depth_state,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            });

            let blur_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("blur_downsample_shader"), source: wgpu::ShaderSource::Wgsl(BLUR_DOWNSAMPLE_SHADER.into()) });
            let scene_blit_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("scene_blit_shader"), source: wgpu::ShaderSource::Wgsl(SCENE_BLIT_SHADER.into()) });
            let glass_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("glass_shader"), source: wgpu::ShaderSource::Wgsl(GLASS_SHADER.into()) });

            let blur_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("blur_downsample_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: std::num::NonZeroU64::new(size_of::<BlurGlobals>() as u64) },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
                ],
            });

            let scene_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("scene_sample_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
                ],
            });

            let blur_globals_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("blur_globals"),
                contents: bytemuck::bytes_of(&BlurGlobals { src_mip: 0.0, _pad: [0.0; 7] }),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

            let blur_downsample_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("blur_downsample_pipeline_layout"), bind_group_layouts: &[&blur_bind_group_layout], push_constant_ranges: &[] });
            let blur_downsample_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("blur_downsample_pipeline"),
                layout: Some(&blur_downsample_pipeline_layout),
                vertex: wgpu::VertexState { module: &blur_shader, entry_point: Some("vs_main"), buffers: &[], compilation_options: Default::default() },
                fragment: Some(wgpu::FragmentState {
                    module: &blur_shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState { format, blend: Some(wgpu::BlendState::REPLACE), write_mask: wgpu::ColorWrites::ALL })],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            });

            let scene_blit_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("scene_blit_pipeline_layout"), bind_group_layouts: &[&scene_bind_group_layout], push_constant_ranges: &[] });
            let scene_blit_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("scene_blit_pipeline"),
                layout: Some(&scene_blit_pipeline_layout),
                vertex: wgpu::VertexState { module: &scene_blit_shader, entry_point: Some("vs_main"), buffers: &[], compilation_options: Default::default() },
                fragment: Some(wgpu::FragmentState {
                    module: &scene_blit_shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState { format, blend: Some(wgpu::BlendState::REPLACE), write_mask: wgpu::ColorWrites::ALL })],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            });

            let glass_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("glass_pipeline_layout"), bind_group_layouts: &[&globals_bind_group_layout, &scene_bind_group_layout], push_constant_ranges: &[] });
            let glass_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("glass_pipeline"),
                layout: Some(&glass_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &glass_shader,
                    entry_point: Some("vs_main"),
                    buffers: &[
                        wgpu::VertexBufferLayout { array_stride: 8, step_mode: wgpu::VertexStepMode::Vertex, attributes: &[wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x2 }] },
                        wgpu::VertexBufferLayout {
                            array_stride: size_of::<GlassInstance>() as wgpu::BufferAddress,
                            step_mode: wgpu::VertexStepMode::Instance,
                            attributes: &[
                                wgpu::VertexAttribute { offset: 0, shader_location: 1, format: wgpu::VertexFormat::Float32x4 },
                                wgpu::VertexAttribute { offset: 16, shader_location: 2, format: wgpu::VertexFormat::Float32x4 },
                                wgpu::VertexAttribute { offset: 32, shader_location: 3, format: wgpu::VertexFormat::Float32x4 },
                            ],
                        },
                    ],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &glass_shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState { format, blend: Some(wgpu::BlendState::ALPHA_BLENDING), write_mask: wgpu::ColorWrites::ALL })],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            });

            let _ = queue;
            Self {
                ui_pipeline,
                vector_pipeline,
                world_pipeline,
                world_pipeline_translucent,
                world_line_pipeline,
                blur_downsample_pipeline,
                scene_blit_pipeline,
                glass_pipeline,
                quad_vertex_buffer,
                globals_buffer,
                blur_globals_buffer,
                world_globals_ring,
                world_bind_group_layout,
                blur_bind_group_layout,
                scene_bind_group_layout,
                glyph_texture,
                glyph_sampler,
                icon_texture,
                icon_sampler,
                glyph_bind_group,
                bind_group_layout: globals_bind_group_layout,
            }
        }

        pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
            &self.bind_group_layout
        }

        pub fn globals_buffer(&self) -> &wgpu::Buffer {
            &self.globals_buffer
        }

        pub fn glyph_view(&self) -> wgpu::TextureView {
            self.glyph_texture.create_view(&wgpu::TextureViewDescriptor::default())
        }

        pub fn glyph_sampler(&self) -> &wgpu::Sampler {
            &self.glyph_sampler
        }

        pub fn icon_view(&self) -> wgpu::TextureView {
            self.icon_texture.create_view(&wgpu::TextureViewDescriptor::default())
        }

        pub fn icon_sampler(&self) -> &wgpu::Sampler {
            &self.icon_sampler
        }

        pub fn depth_format(&self) -> wgpu::TextureFormat {
            wgpu::TextureFormat::Depth24Plus
        }

        fn prepare_world_passes(draw: &DrawList, filter: LayerBatchFilter) -> (Vec<PreparedWorldPass>, Vec<World3dGpuInstance>, Vec<WorldLineGpuVertex>, Vec<Option<usize>>) {
            let mut prepared = Vec::new();
            let mut all_instances = Vec::new();
            let mut all_lines = Vec::new();
            let mut pass_index_map = vec![None; draw.scene_passes.len()];
            for (source_index, scene) in draw.scene_passes.iter().enumerate() {
                if !layer_matches_filter(&draw.layers[scene.layer_index], filter) {
                    continue;
                }
                let mut pass_draws = Vec::new();
                for draw_call in &scene.draws {
                    if draw_call.instances.is_empty() {
                        continue;
                    }
                    let instance_offset = all_instances.len() as u32;
                    let instance_count = draw_call.instances.len() as u32;
                    for instance in &draw_call.instances {
                        all_instances.push(World3dGpuInstance::from_instance(instance.model.to_cols_array(), instance.color, instance.selected, instance.hovered));
                    }
                    pass_draws.push(WorldDrawRange { mesh_key: draw_call.mesh_key.clone(), mesh_version: draw_call.mesh_version, instance_offset, instance_count });
                }
                let mut translucent_draws = Vec::new();
                for draw_call in &scene.translucent_draws {
                    if draw_call.instances.is_empty() {
                        continue;
                    }
                    let instance_offset = all_instances.len() as u32;
                    let instance_count = draw_call.instances.len() as u32;
                    for instance in &draw_call.instances {
                        all_instances.push(World3dGpuInstance::from_instance(instance.model.to_cols_array(), instance.color, instance.selected, instance.hovered));
                    }
                    translucent_draws.push(WorldDrawRange { mesh_key: draw_call.mesh_key.clone(), mesh_version: draw_call.mesh_version, instance_offset, instance_count });
                }
                let line_start = all_lines.len() as u32;
                for line_draw in &scene.line_draws {
                    for vertex in &line_draw.vertices {
                        all_lines.push(WorldLineGpuVertex { position: vertex.position, color: vertex.color });
                    }
                }
                let line_count = all_lines.len() as u32 - line_start;
                pass_index_map[source_index] = Some(prepared.len());
                prepared.push(PreparedWorldPass {
                    globals: World3dGlobals { view_proj: scene.view_proj, light_dir: [scene.light_dir[0], scene.light_dir[1], scene.light_dir[2], 0.0] },
                    viewport: scene.viewport,
                    draws: pass_draws,
                    translucent_draws,
                    line_start,
                    line_count,
                });
            }
            (prepared, all_instances, all_lines, pass_index_map)
        }

        fn upload_world_passes(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, draw: &DrawList, frame_buffers: &mut FrameBuffers, filter: LayerBatchFilter) -> Option<(Vec<PreparedWorldPass>, Vec<Option<usize>>)> {
            if draw.scene_passes.is_empty() {
                return None;
            }
            let (prepared, all_instances, all_lines, pass_index_map) = Self::prepare_world_passes(draw, filter);
            if prepared.is_empty() {
                return None;
            }
            if all_instances.is_empty() && all_lines.is_empty() {
                return Some((prepared, pass_index_map));
            }
            self.world_globals_ring.ensure_slots(device, &self.world_bind_group_layout, prepared.len() as u32);
            let globals: Vec<World3dGlobals> = prepared.iter().map(|pass| pass.globals).collect();
            self.world_globals_ring.write_passes(queue, &globals);
            if !all_instances.is_empty() {
                frame_buffers.world_instances.upload(device, queue, &all_instances, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "world3d_instances");
            }
            if !all_lines.is_empty() {
                frame_buffers.world_lines.upload(device, queue, &all_lines, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "world3d_lines");
            }
            Some((prepared, pass_index_map))
        }

        #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
        fn draw_world_pass_at<'a>(
            &'a self,
            pass: &mut wgpu::RenderPass<'a>,
            mesh_store: &MeshGpuStore,
            prepared: &PreparedWorldPass,
            slot: u32,
            instance_buffer: wgpu::BufferSlice<'a>,
            line_buffer: Option<wgpu::BufferSlice<'a>>,
            screen_w: f32,
            screen_h: f32,
        ) {
            let instance_stride = size_of::<World3dGpuInstance>() as u64;
            pass.set_pipeline(&self.world_pipeline);
            let viewport = prepared.viewport;
            pass.set_viewport(viewport[0], viewport[1], viewport[2], viewport[3], 0.0, 1.0);
            pass.set_scissor_rect(viewport[0] as u32, viewport[1] as u32, viewport[2] as u32, viewport[3] as u32);
            pass.set_bind_group(0, &self.world_globals_ring.bind_group, &[self.world_globals_ring.offset_for_slot(slot)]);
            for draw_call in &prepared.draws {
                Self::draw_world_range(pass, mesh_store, draw_call, instance_buffer, instance_stride);
            }
            if prepared.line_count > 0 {
                if let Some(line_buffer) = line_buffer {
                    pass.set_pipeline(&self.world_line_pipeline);
                    pass.set_bind_group(0, &self.world_globals_ring.bind_group, &[self.world_globals_ring.offset_for_slot(slot)]);
                    let line_stride = size_of::<WorldLineGpuVertex>() as u64;
                    let byte_offset = prepared.line_start as u64 * line_stride;
                    pass.set_vertex_buffer(0, line_buffer.slice(byte_offset..byte_offset + prepared.line_count as u64 * line_stride));
                    pass.draw(0..prepared.line_count, 0..1);
                }
            }
            if !prepared.translucent_draws.is_empty() {
                pass.set_pipeline(&self.world_pipeline_translucent);
                pass.set_bind_group(0, &self.world_globals_ring.bind_group, &[self.world_globals_ring.offset_for_slot(slot)]);
                for draw_call in &prepared.translucent_draws {
                    Self::draw_world_range(pass, mesh_store, draw_call, instance_buffer, instance_stride);
                }
            }
            pass.set_viewport(0.0, 0.0, screen_w, screen_h, 0.0, 1.0);
            pass.set_scissor_rect(0, 0, screen_w as u32, screen_h as u32);
            pass.set_pipeline(&self.ui_pipeline);
            pass.set_bind_group(0, &self.glyph_bind_group, &[]);
        }

        fn draw_world_range<'a>(pass: &mut wgpu::RenderPass<'a>, mesh_store: &MeshGpuStore, draw_call: &WorldDrawRange, instance_buffer: wgpu::BufferSlice<'a>, instance_stride: u64) {
            let store_key = MeshGpuStore::lookup_key(&draw_call.mesh_key, draw_call.mesh_version);
            let Some(mesh) = mesh_store.get(&store_key) else {
                return;
            };
            let byte_offset = draw_call.instance_offset as u64 * instance_stride;
            pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            pass.set_vertex_buffer(1, instance_buffer.slice(byte_offset..byte_offset + draw_call.instance_count as u64 * instance_stride));
            pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..mesh.index_count, 0, 0..draw_call.instance_count);
        }

        fn draw_ui_instances<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>, instance_buffer: &wgpu::BufferSlice<'a>, start: u32, count: u32) {
            if count == 0 {
                return;
            }
            pass.set_pipeline(&self.ui_pipeline);
            pass.set_bind_group(0, &self.glyph_bind_group, &[]);
            pass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));
            pass.set_vertex_buffer(1, *instance_buffer);
            pass.draw(0..6, start..start + count);
        }

        #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
        fn draw_raster_layers(
            &self,
            pass: &mut wgpu::RenderPass<'_>,
            raster_store: &RasterTextureStore,
            draw: &DrawList,
            frame_buffers: &mut FrameBuffers,
            device: &wgpu::Device,
            queue: &wgpu::Queue,
            width: f32,
            height: f32,
            filter: LayerBatchFilter,
        ) {
            for layer in &draw.layers {
                if !layer_matches_filter(layer, filter) {
                    continue;
                }
                if layer.raster_instances.is_empty() {
                    continue;
                }
                if let Some(scissor) = layer.scissor {
                    set_pass_scissor(pass, Some(scissor), width, height);
                } else {
                    pass.set_scissor_rect(0, 0, width as u32, height as u32);
                }
                let mut batch_key: Option<String> = None;
                let mut batch_instances: Vec<UiInstance> = Vec::new();
                let mut flush = |key: &str, instances: &[UiInstance]| {
                    if instances.is_empty() {
                        return;
                    }
                    let Some(rt) = raster_store.get(key) else {
                        return;
                    };
                    pass.set_pipeline(&self.ui_pipeline);
                    pass.set_bind_group(0, &rt.bind_group, &[]);
                    let Some(buffer) = frame_buffers.ui_instances.upload(device, queue, instances, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "raster_instances") else {
                        return;
                    };
                    pass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));
                    pass.set_vertex_buffer(1, buffer);
                    pass.draw(0..6, 0..instances.len() as u32);
                };
                for (key, instance) in &layer.raster_instances {
                    if batch_key.as_deref() != Some(key.as_str()) {
                        if let Some(ref prior) = batch_key {
                            flush(prior, &batch_instances);
                        }
                        batch_key = Some(key.clone());
                        batch_instances.clear();
                    }
                    batch_instances.push(*instance);
                }
                if let Some(ref key) = batch_key {
                    flush(key, &batch_instances);
                }
            }
            pass.set_scissor_rect(0, 0, width as u32, height as u32);
        }

        fn draw_vector_vertices<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>, vector_buffer: &wgpu::BufferSlice<'a>, start: u32, count: u32) {
            if count == 0 {
                return;
            }
            pass.set_pipeline(&self.vector_pipeline);
            pass.set_bind_group(0, &self.glyph_bind_group, &[]);
            pass.set_vertex_buffer(0, *vector_buffer);
            pass.draw(start..start + count, 0..1);
        }

        #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
        fn render_interleaved_layers<'a>(
            &'a self,
            pass: &mut wgpu::RenderPass<'a>,
            draw: &DrawList,
            batches: &[LayerBatch],
            ui_buffer: Option<&wgpu::BufferSlice<'a>>,
            vector_buffer: Option<&wgpu::BufferSlice<'a>>,
            world_prepared: Option<&[PreparedWorldPass]>,
            pass_index_map: &[Option<usize>],
            instance_buffer: Option<wgpu::BufferSlice<'a>>,
            line_buffer: Option<wgpu::BufferSlice<'a>>,
            mesh_store: &MeshGpuStore,
            width: f32,
            height: f32,
            depth_enabled: bool,
        ) {
            for batch in batches {
                set_pass_scissor(pass, batch.scissor, width, height);
                let mut layer_passes: Vec<(usize, usize, usize)> = draw.scene_passes.iter().enumerate().filter(|(_, scene)| scene.layer_index == batch.layer_index).map(|(index, scene)| (index, scene.ui_watermark, scene.vector_watermark)).collect();
                layer_passes.sort_by_key(|(_, ui, vec)| (*ui, *vec));
                if layer_passes.is_empty() {
                    if let Some(instance_buffer) = ui_buffer {
                        self.draw_ui_instances(pass, instance_buffer, batch.ui_start, batch.ui_count);
                    }
                    if let Some(vector_buffer) = vector_buffer {
                        self.draw_vector_vertices(pass, vector_buffer, batch.vec_start, batch.vec_count);
                    }
                    continue;
                }
                let mut ui_local = 0u32;
                let mut vec_local = 0u32;
                for (pass_index, ui_mark, vec_mark) in layer_passes {
                    let ui_mark = ui_mark as u32;
                    let vec_mark = vec_mark as u32;
                    if ui_mark > ui_local {
                        if let Some(instance_buffer) = ui_buffer {
                            self.draw_ui_instances(pass, instance_buffer, batch.ui_start + ui_local, ui_mark - ui_local);
                        }
                        ui_local = ui_mark;
                    }
                    if vec_mark > vec_local {
                        if let Some(vector_buffer) = vector_buffer {
                            self.draw_vector_vertices(pass, vector_buffer, batch.vec_start + vec_local, vec_mark - vec_local);
                        }
                        vec_local = vec_mark;
                    }
                    if depth_enabled {
                        if let (Some(prepared), Some(instance_buffer)) = (world_prepared, instance_buffer.as_ref()) {
                            if let Some(prepared_slot) = pass_index_map.get(pass_index).and_then(|slot| *slot) {
                                if let Some(scene) = prepared.get(prepared_slot) {
                                    self.draw_world_pass_at(pass, mesh_store, scene, prepared_slot as u32, *instance_buffer, line_buffer, width, height);
                                }
                            }
                        }
                    }
                }
                if ui_local < batch.ui_count {
                    if let Some(instance_buffer) = ui_buffer {
                        self.draw_ui_instances(pass, instance_buffer, batch.ui_start + ui_local, batch.ui_count - ui_local);
                    }
                }
                if vec_local < batch.vec_count {
                    if let Some(vector_buffer) = vector_buffer {
                        self.draw_vector_vertices(pass, vector_buffer, batch.vec_start + vec_local, batch.vec_count - vec_local);
                    }
                }
            }
            pass.set_scissor_rect(0, 0, width as u32, height as u32);
        }

        pub fn update_globals(&self, queue: &wgpu::Queue, width: f32, height: f32, time_seconds: f32) {
            queue.write_buffer(&self.globals_buffer, 0, bytemuck::bytes_of(&UiGlobals { screen_size: [width, height], _pad: [time_seconds, 0.0] }));
        }

        pub fn upload_glyph_atlas(&self, queue: &wgpu::Queue, pixels: &[u8], width: u32, height: u32) {
            queue.write_texture(
                wgpu::TexelCopyTextureInfo { texture: &self.glyph_texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
                pixels,
                wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(width), rows_per_image: Some(height) },
                wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            );
        }

        pub fn upload_icon_atlas(&self, queue: &wgpu::Queue, pixels: &[u8], width: u32, height: u32) {
            queue.write_texture(
                wgpu::TexelCopyTextureInfo { texture: &self.icon_texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
                pixels,
                wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(width * 4), rows_per_image: Some(height) },
                wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            );
        }

        #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
        pub fn render_scene_content<'a>(
            &'a mut self,
            device: &wgpu::Device,
            queue: &wgpu::Queue,
            encoder: &mut wgpu::CommandEncoder,
            scene: &'a SceneColorTarget,
            depth_view: Option<&'a wgpu::TextureView>,
            draw: &DrawList,
            mesh_store: &MeshGpuStore,
            raster_store: &RasterTextureStore,
            frame_buffers: &mut FrameBuffers,
            width: f32,
            height: f32,
            time_seconds: f32,
        ) {
            self.update_globals(queue, width, height, time_seconds);
            let scene_view = scene.mip_view(0);
            let world_upload = if depth_view.is_some() { self.upload_world_passes(device, queue, draw, frame_buffers, LayerBatchFilter::Backdrop) } else { None };
            let (prepared_holder, pass_index_map) = match world_upload {
                Some((prepared, map)) => (Some(prepared), map),
                None => (None, vec![None; draw.scene_passes.len()]),
            };
            let world_prepared = prepared_holder.as_deref();
            let (all_ui, all_vec, batches) = build_layer_batches(draw, LayerBatchFilter::Backdrop);
            let ui_buffer = if all_ui.is_empty() { None } else { frame_buffers.ui_instances.upload(device, queue, &all_ui, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "ui_instances") };
            let vector_buffer = if all_vec.is_empty() { None } else { frame_buffers.vector_vertices.upload(device, queue, &all_vec, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "vector_vertices") };
            let instance_buffer = frame_buffers.world_instances.slice();
            let line_buffer = frame_buffers.world_lines.slice();
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ui_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: scene_view,
                    resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.05, g: 0.05, b: 0.06, a: 1.0 }), store: wgpu::StoreOp::Store },
                    depth_slice: None,
                })],
                depth_stencil_attachment: depth_view.map(|depth| wgpu::RenderPassDepthStencilAttachment { view: depth, depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }), stencil_ops: None }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            self.render_interleaved_layers(&mut pass, draw, &batches, ui_buffer.as_ref(), vector_buffer.as_ref(), world_prepared, &pass_index_map, instance_buffer, line_buffer, mesh_store, width, height, depth_view.is_some());
            drop(pass);
            if draw.layers.iter().any(|layer| layer_matches_filter(layer, LayerBatchFilter::Backdrop) && !layer.raster_instances.is_empty()) {
                let mut raster_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("ui_raster_pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: scene_view, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }, depth_slice: None })],
                    depth_stencil_attachment: depth_view.map(|depth| wgpu::RenderPassDepthStencilAttachment { view: depth, depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }), stencil_ops: None }),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                self.draw_raster_layers(&mut raster_pass, raster_store, draw, frame_buffers, device, queue, width, height, LayerBatchFilter::Backdrop);
            }
            let (overlay_ui, overlay_vec, overlay_batches) = build_overlay_layer_batches(draw, LayerBatchFilter::Backdrop);
            if !overlay_ui.is_empty() || !overlay_vec.is_empty() {
                let overlay_ui_buffer = if overlay_ui.is_empty() { None } else { frame_buffers.ui_instances.upload(device, queue, &overlay_ui, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "overlay_ui_instances") };
                let overlay_vector_buffer = if overlay_vec.is_empty() { None } else { frame_buffers.vector_vertices.upload(device, queue, &overlay_vec, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "overlay_vector_vertices") };
                let mut overlay_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("ui_overlay_pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: scene_view, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }, depth_slice: None })],
                    depth_stencil_attachment: depth_view.map(|depth| wgpu::RenderPassDepthStencilAttachment { view: depth, depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }), stencil_ops: None }),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                self.render_interleaved_layers(&mut overlay_pass, draw, &overlay_batches, overlay_ui_buffer.as_ref(), overlay_vector_buffer.as_ref(), None, &[], None, None, mesh_store, width, height, depth_view.is_some());
            }
        }

        fn has_glass_foreground(draw: &DrawList) -> bool {
            let layer_content = draw.layers.iter().any(|layer| layer.foreground_of.is_some() && (!layer.ui_instances.is_empty() || !layer.vector_vertices.is_empty() || !layer.raster_instances.is_empty()));
            let scene_content = draw.scene_passes.iter().any(|pass| layer_matches_filter(&draw.layers[pass.layer_index], LayerBatchFilter::Foreground));
            layer_content || scene_content
        }

        #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
        fn render_glass_foreground<'a>(
            &'a mut self,
            device: &wgpu::Device,
            queue: &wgpu::Queue,
            encoder: &mut wgpu::CommandEncoder,
            view: &'a wgpu::TextureView,
            draw: &DrawList,
            depth_view: Option<&'a wgpu::TextureView>,
            mesh_store: &MeshGpuStore,
            raster_store: &RasterTextureStore,
            frame_buffers: &mut FrameBuffers,
            width: f32,
            height: f32,
        ) {
            if !Self::has_glass_foreground(draw) {
                return;
            }
            let world_upload = if depth_view.is_some() { self.upload_world_passes(device, queue, draw, frame_buffers, LayerBatchFilter::Foreground) } else { None };
            let (prepared_holder, pass_index_map) = match world_upload {
                Some((prepared, map)) => (Some(prepared), map),
                None => (None, vec![None; draw.scene_passes.len()]),
            };
            let world_prepared = prepared_holder.as_deref();
            let (all_ui, all_vec, batches) = build_layer_batches(draw, LayerBatchFilter::Foreground);
            if all_ui.is_empty() && all_vec.is_empty() && batches.is_empty() && world_prepared.is_none() {
                return;
            }
            let ui_buffer = if all_ui.is_empty() { None } else { frame_buffers.ui_instances.upload(device, queue, &all_ui, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "glass_foreground_ui_instances") };
            let vector_buffer = if all_vec.is_empty() { None } else { frame_buffers.vector_vertices.upload(device, queue, &all_vec, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "glass_foreground_vector_vertices") };
            let instance_buffer = frame_buffers.world_instances.slice();
            let line_buffer = frame_buffers.world_lines.slice();
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("glass_foreground_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }, depth_slice: None })],
                depth_stencil_attachment: depth_view.map(|depth| wgpu::RenderPassDepthStencilAttachment { view: depth, depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }), stencil_ops: None }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            self.render_interleaved_layers(&mut pass, draw, &batches, ui_buffer.as_ref(), vector_buffer.as_ref(), world_prepared, &pass_index_map, instance_buffer, line_buffer, mesh_store, width, height, depth_view.is_some());
            drop(pass);
            if draw.layers.iter().any(|layer| layer_matches_filter(layer, LayerBatchFilter::Foreground) && !layer.raster_instances.is_empty()) {
                let mut raster_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("glass_foreground_raster_pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment { view, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }, depth_slice: None })],
                    depth_stencil_attachment: depth_view.map(|depth| wgpu::RenderPassDepthStencilAttachment { view: depth, depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }), stencil_ops: None }),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                self.draw_raster_layers(&mut raster_pass, raster_store, draw, frame_buffers, device, queue, width, height, LayerBatchFilter::Foreground);
            }
            let (overlay_ui, overlay_vec, overlay_batches) = build_overlay_layer_batches(draw, LayerBatchFilter::Foreground);
            if !overlay_ui.is_empty() || !overlay_vec.is_empty() {
                let overlay_ui_buffer = if overlay_ui.is_empty() { None } else { frame_buffers.ui_instances.upload(device, queue, &overlay_ui, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "glass_foreground_overlay_ui_instances") };
                let overlay_vector_buffer =
                    if overlay_vec.is_empty() { None } else { frame_buffers.vector_vertices.upload(device, queue, &overlay_vec, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "glass_foreground_overlay_vector_vertices") };
                let mut overlay_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("glass_foreground_overlay_pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment { view, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }, depth_slice: None })],
                    depth_stencil_attachment: depth_view.map(|depth| wgpu::RenderPassDepthStencilAttachment { view: depth, depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }), stencil_ops: None }),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                self.render_interleaved_layers(&mut overlay_pass, draw, &overlay_batches, overlay_ui_buffer.as_ref(), overlay_vector_buffer.as_ref(), None, &[], None, None, mesh_store, width, height, depth_view.is_some());
            }
        }

        #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
        pub fn composite_to_swapchain<'a>(
            &'a mut self,
            device: &wgpu::Device,
            queue: &wgpu::Queue,
            encoder: &mut wgpu::CommandEncoder,
            view: &'a wgpu::TextureView,
            scene: &'a SceneColorTarget,
            depth_view: Option<&'a wgpu::TextureView>,
            draw: &DrawList,
            overlay: Option<&DrawList>,
            mesh_store: &MeshGpuStore,
            raster_store: &RasterTextureStore,
            frame_buffers: &mut FrameBuffers,
            width: f32,
            height: f32,
        ) {
            self.run_blur_chain(device, queue, scene);
            self.blit_scene_to_swapchain(device, encoder, view, scene);
            let max_mip = SCENE_MIP_LEVELS - 1;
            self.composite_glass_regions(device, queue, encoder, view, scene, frame_buffers, &draw.glass_regions, max_mip, width, height);
            self.render_glass_foreground(device, queue, encoder, view, draw, depth_view, mesh_store, raster_store, frame_buffers, width, height);
            if let Some(overlay) = overlay {
                if !overlay.glass_regions.is_empty() {
                    self.composite_glass_regions(device, queue, encoder, view, scene, frame_buffers, &overlay.glass_regions, max_mip, width, height);
                }
                self.render_glass_foreground(device, queue, encoder, view, overlay, depth_view, mesh_store, raster_store, frame_buffers, width, height);
                let mut overlay_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("ui_overlay_pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment { view, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }, depth_slice: None })],
                    depth_stencil_attachment: depth_view.map(|depth| wgpu::RenderPassDepthStencilAttachment { view: depth, depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }), stencil_ops: None }),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                self.render_overlay(device, queue, &mut overlay_pass, overlay, frame_buffers, width, height);
            }
        }

        #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
        #[allow(dead_code, reason = "top-level UiPipelines render entrypoint; not yet called internally, likely wired externally by framework/renderer/wgpu")]
        pub fn render<'a>(
            &'a mut self,
            device: &wgpu::Device,
            queue: &wgpu::Queue,
            encoder: &mut wgpu::CommandEncoder,
            view: &'a wgpu::TextureView,
            scene: &'a SceneColorTarget,
            depth_view: Option<&'a wgpu::TextureView>,
            draw: &DrawList,
            overlay: Option<&DrawList>,
            mesh_store: &MeshGpuStore,
            raster_store: &RasterTextureStore,
            frame_buffers: &mut FrameBuffers,
            width: f32,
            height: f32,
            time_seconds: f32,
        ) {
            self.render_scene_content(device, queue, encoder, scene, depth_view, draw, mesh_store, raster_store, frame_buffers, width, height, time_seconds);
            self.composite_to_swapchain(device, queue, encoder, view, scene, depth_view, draw, overlay, mesh_store, raster_store, frame_buffers, width, height);
        }

        fn run_blur_chain(&self, device: &wgpu::Device, queue: &wgpu::Queue, scene: &SceneColorTarget) {
            for mip in 1..SCENE_MIP_LEVELS {
                let src_mip = mip - 1;
                queue.write_buffer(&self.blur_globals_buffer, 0, bytemuck::bytes_of(&BlurGlobals { src_mip: 0.0, _pad: [0.0; 7] }));
                let blur_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("blur_downsample_bind_group"),
                    layout: &self.blur_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry { binding: 0, resource: self.blur_globals_buffer.as_entire_binding() },
                        wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(scene.blur_scratch_mip_view(src_mip)) },
                        wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(scene.sampler()) },
                    ],
                });
                let mut copy_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("blur_copy_encoder") });
                scene.copy_mip_to_blur_scratch(&mut copy_encoder, src_mip);
                queue.submit(Some(copy_encoder.finish()));
                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("blur_downsample_encoder") });
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("blur_downsample_pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: scene.mip_view(mip),
                        resolve_target: None,
                        ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                pass.set_pipeline(&self.blur_downsample_pipeline);
                pass.set_bind_group(0, &blur_bind_group, &[]);
                pass.draw(0..6, 0..1);
                drop(pass);
                queue.submit(Some(encoder.finish()));
            }
        }

        fn blit_scene_to_swapchain(&self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView, scene: &SceneColorTarget) {
            let scene_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("scene_blit_bind_group"),
                layout: &self.scene_bind_group_layout,
                entries: &[wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(scene.sample_view()) }, wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(scene.sampler()) }],
            });
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("scene_blit_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.05, g: 0.05, b: 0.06, a: 1.0 }), store: wgpu::StoreOp::Store },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(&self.scene_blit_pipeline);
            pass.set_bind_group(0, &scene_bind_group, &[]);
            pass.draw(0..6, 0..1);
        }

        #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
        fn composite_glass_regions(
            &self,
            device: &wgpu::Device,
            queue: &wgpu::Queue,
            encoder: &mut wgpu::CommandEncoder,
            view: &wgpu::TextureView,
            scene: &SceneColorTarget,
            frame_buffers: &mut FrameBuffers,
            regions: &[GlassRegion],
            max_mip: u32,
            width: f32,
            height: f32,
        ) {
            if regions.is_empty() {
                return;
            }
            let instances: Vec<GlassInstance> = regions
                .iter()
                .map(|region| GlassInstance { rect: region.rect, tint: [region.tint.r, region.tint.g, region.tint.b, region.tint.a], params: [region.radius, region.alpha, Theme::glass_mip_level(region.blur_px, max_mip), region.saturate] })
                .collect();
            let glass_buffer = frame_buffers.glass_instances.upload(device, queue, &instances, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "glass_instances");
            let Some(glass_buffer) = glass_buffer else {
                return;
            };
            let scene_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("glass_scene_bind_group"),
                layout: &self.scene_bind_group_layout,
                entries: &[wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(scene.sample_view()) }, wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(scene.sampler()) }],
            });
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("glass_composite_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }, depth_slice: None })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(&self.glass_pipeline);
            pass.set_bind_group(0, &self.glyph_bind_group, &[]);
            pass.set_bind_group(1, &scene_bind_group, &[]);
            pass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));
            pass.set_vertex_buffer(1, glass_buffer.slice(..));
            pass.draw(0..6, 0..instances.len() as u32);
            let _ = (width, height);
        }

        #[allow(clippy::too_many_arguments, reason = "one arg per GPU resource/dimension; grouping into a struct is a T2 restructure, out of scope")]
        pub fn render_overlay<'a>(&'a self, device: &wgpu::Device, queue: &wgpu::Queue, pass: &mut wgpu::RenderPass<'a>, overlay: &DrawList, frame_buffers: &mut FrameBuffers, width: f32, height: f32) {
            pass.set_pipeline(&self.ui_pipeline);
            pass.set_bind_group(0, &self.glyph_bind_group, &[]);

            let (all_ui, all_vec, batches) = build_layer_batches(overlay, LayerBatchFilter::Backdrop);
            let ui_buffer = if all_ui.is_empty() { None } else { frame_buffers.ui_instances.upload(device, queue, &all_ui, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "overlay_ui_instances") };
            let vector_buffer = if all_vec.is_empty() { None } else { frame_buffers.vector_vertices.upload(device, queue, &all_vec, wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, "overlay_vector_vertices") };

            for batch in &batches {
                set_pass_scissor(pass, batch.scissor, width, height);
                if batch.ui_count > 0 {
                    if let Some(instance_buffer) = &ui_buffer {
                        pass.set_pipeline(&self.ui_pipeline);
                        pass.set_bind_group(0, &self.glyph_bind_group, &[]);
                        pass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));
                        pass.set_vertex_buffer(1, *instance_buffer);
                        pass.draw(0..6, batch.ui_start..batch.ui_start + batch.ui_count);
                    }
                }
                if batch.vec_count > 0 {
                    if let Some(vector_buffer) = &vector_buffer {
                        pass.set_pipeline(&self.vector_pipeline);
                        pass.set_vertex_buffer(0, *vector_buffer);
                        pass.draw(batch.vec_start..batch.vec_start + batch.vec_count, 0..1);
                    }
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{ear_clip_polygon, mesh_content_version, DrawList, ScissorRect, WORLD_GLOBALS_SLOT_SIZE};
        use crate::geometry::Rect;
        use crate::theme::Rgba;
        use kernel_3d_scene::ScenePass3d;

        #[test]
        fn scissor_intersects_child() {
            let a = ScissorRect { x: 0, y: 0, w: 100, h: 100 };
            let b = ScissorRect { x: 50, y: 50, w: 100, h: 100 };
            let c = a.intersect(&b);
            assert_eq!(c.w, 50);
            assert_eq!(c.h, 50);
        }

        #[test]
        fn scissor_from_rect_uses_top_left_origin() {
            let scissor = ScissorRect::from_rect(Rect::new(10.0, 20.0, 80.0, 60.0), 720.0);
            assert_eq!(scissor.x, 10);
            assert_eq!(scissor.y, 20);
            assert_eq!(scissor.w, 80);
            assert_eq!(scissor.h, 60);
        }

        #[test]
        fn draw_list_push_scissor_splits_layers() {
            let mut draw = DrawList::default();
            draw.set_screen_height(200.0);
            draw.push_solid([0.0, 0.0, 200.0, 200.0], Rgba::new(1.0, 0.0, 0.0, 1.0));
            draw.push_scissor(Rect::new(10.0, 10.0, 80.0, 80.0));
            draw.push_solid([10.0, 10.0, 80.0, 80.0], Rgba::new(0.0, 1.0, 0.0, 1.0));
            draw.pop_scissor();
            assert!(draw.layers.len() >= 3);
        }

        #[test]
        fn ear_clip_produces_triangles() {
            let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
            let tris = ear_clip_polygon(&square);
            assert!(tris.len() >= 3);
        }

        #[test]
        fn world_globals_slot_size_is_aligned() {
            const { assert!(WORLD_GLOBALS_SLOT_SIZE >= 80) };
            assert_eq!(WORLD_GLOBALS_SLOT_SIZE % 256, 0);
        }

        #[test]
        fn scene_pass_records_layer_watermarks() {
            let mut draw = DrawList::default();
            draw.push_solid([0.0, 0.0, 10.0, 10.0], Rgba::new(1.0, 0.0, 0.0, 1.0));
            draw.push_solid([1.0, 1.0, 8.0, 8.0], Rgba::new(0.0, 1.0, 0.0, 1.0));
            draw.push_scene_pass(ScenePass3d { viewport: [0.0, 0.0, 100.0, 100.0], view_proj: [0.0; 16], light_dir: [0.0, 0.0, 1.0], ..Default::default() });
            draw.push_line(0.0, 0.0, 1.0, 1.0, Rgba::new(0.0, 0.0, 1.0, 1.0), 1.0);
            let pass = &draw.scene_passes[0];
            assert_eq!(pass.layer_index, 0);
            assert_eq!(pass.ui_watermark, 2);
            assert_eq!(pass.vector_watermark, 0);
            assert_eq!(draw.layers[0].ui_instances.len(), 2);
            assert_eq!(draw.layers[0].vector_vertices.len(), 6);
        }

        #[test]
        fn mesh_instances_without_lines_are_valid_world_pass() {
            use kernel_3d_scene::{Instance3d, SceneDraw3d, ScenePass3d};

            let pass = ScenePass3d {
                viewport: [0.0, 0.0, 320.0, 240.0],
                view_proj: [0.0; 16],
                light_dir: [0.4, 0.6, 0.8],
                draws: vec![SceneDraw3d {
                    mesh_key: "box".into(),
                    mesh_version: 1,
                    instances: vec![Instance3d { id: "preview".into(), model: Instance3d::model_from_trs([0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0]), color: [0.7, 0.7, 0.75, 1.0], selected: false, hovered: false }],
                }],
                ..Default::default()
            };
            assert!(!pass.draws[0].instances.is_empty());
            assert!(pass.line_draws.is_empty());
        }

        #[test]
        fn mesh_content_version_changes_with_indices() {
            let v0 = mesh_content_version(&[0.0, 0.0, 0.0], &[0.0, 1.0, 0.0], &[0, 1, 2]);
            let v1 = mesh_content_version(&[0.0, 0.0, 0.0], &[0.0, 1.0, 0.0], &[0, 2, 1]);
            assert_ne!(v0, v1);
        }

        #[test]
        fn overlay_layers_collected_separately_from_backdrop_ui() {
            use super::{build_layer_batches, build_overlay_layer_batches, LayerBatchFilter};
            let mut draw = DrawList::default();
            draw.push_solid([0.0, 0.0, 100.0, 100.0], Rgba::new(0.1, 0.1, 0.1, 1.0));
            draw.push_glyph_overlay([10.0, 10.0, 20.0, 12.0], Rgba::new(1.0, 1.0, 1.0, 1.0), [0.0, 0.0, 0.1, 0.1]);
            draw.push_line_overlay(0.0, 0.0, 50.0, 50.0, Rgba::new(1.0, 0.0, 0.0, 1.0), 1.0);
            let (backdrop_ui, _, _) = build_layer_batches(&draw, LayerBatchFilter::Backdrop);
            let (overlay_ui, overlay_vec, overlay_batches) = build_overlay_layer_batches(&draw, LayerBatchFilter::Backdrop);
            assert_eq!(backdrop_ui.len(), 1);
            assert_eq!(overlay_ui.len(), 1);
            assert_eq!(overlay_vec.len(), 6);
            assert_eq!(overlay_batches.len(), 1);
            assert_eq!(draw.layers[overlay_batches[0].layer_index].overlay_ui_instances.len(), 1);
        }

        #[test]
        fn glass_content_layers_tagged_with_foreground_of() {
            use super::{Level, Theme};
            let theme = Theme::default();
            let mut draw = DrawList::default();
            draw.push_solid([0.0, 0.0, 100.0, 100.0], Rgba::new(0.2, 0.2, 0.2, 1.0));
            let glass = draw.push_glass([10.0, 10.0, 80.0, 80.0], 8.0, theme.glass(Level::Panel));
            assert_eq!(glass, 0);
            draw.begin_glass_content(glass);
            draw.push_solid([10.0, 10.0, 80.0, 80.0], Rgba::new(1.0, 0.0, 0.0, 1.0));
            draw.end_glass_content();
            let backdrop = draw.layers.iter().filter(|layer| layer.foreground_of.is_none()).count();
            let foreground = draw.layers.iter().filter(|layer| layer.foreground_of == Some(glass)).count();
            assert_eq!(backdrop, 2);
            assert_eq!(foreground, 1);
            assert_eq!(draw.layers[1].ui_instances.len(), 1);
        }

        #[test]
        fn glass_foreground_layers_excluded_from_backdrop_batches() {
            use super::{build_layer_batches, LayerBatchFilter, Level, Theme};
            let theme = Theme::default();
            let mut draw = DrawList::default();
            draw.push_solid([0.0, 0.0, 200.0, 200.0], Rgba::new(0.1, 0.1, 0.1, 1.0));
            let glass = draw.push_glass([20.0, 20.0, 160.0, 160.0], 8.0, theme.glass(Level::Panel));
            draw.begin_glass_content(glass);
            draw.push_solid([20.0, 20.0, 160.0, 160.0], Rgba::new(1.0, 0.0, 0.0, 1.0));
            draw.end_glass_content();
            let (backdrop_ui, _, backdrop_batches) = build_layer_batches(&draw, LayerBatchFilter::Backdrop);
            let (foreground_ui, _, foreground_batches) = build_layer_batches(&draw, LayerBatchFilter::Foreground);
            assert_eq!(backdrop_ui.len(), 1);
            assert_eq!(foreground_ui.len(), 1);
            assert_eq!(backdrop_batches.len(), 1);
            assert_eq!(foreground_batches.len(), 1);
            assert!(draw.layers[backdrop_batches[0].layer_index].foreground_of.is_none());
            assert_eq!(draw.layers[foreground_batches[0].layer_index].foreground_of, Some(glass));
        }

        #[test]
        fn glass_scissor_inherits_foreground_tag() {
            use super::{Level, Theme};
            let theme = Theme::default();
            let mut draw = DrawList::default();
            let glass = draw.push_glass([0.0, 0.0, 100.0, 100.0], 8.0, theme.glass(Level::Panel));
            draw.begin_glass_content(glass);
            draw.push_scissor(Rect::new(10.0, 10.0, 80.0, 80.0));
            draw.push_solid([10.0, 10.0, 80.0, 80.0], Rgba::new(0.0, 1.0, 0.0, 1.0));
            draw.pop_scissor();
            draw.end_glass_content();
            let scissor_layer = draw.layers.iter().find(|layer| layer.scissor.is_some()).expect("scissor layer");
            assert_eq!(scissor_layer.foreground_of, Some(glass));
        }

        /// 🪜️ `Theme::glass` must be formula-derived off `Level::index` (never a per-tier lookup
        /// table): alpha/blur both monotone across all 6 levels. There is deliberately no separate
        /// "chrome" variant — a level's attached chrome (title caps, ribbons, tab bars, rails) always
        /// renders the exact same `glass(level)` as its body, so one level never shows two appearances.
        #[test]
        fn glass_alpha_and_blur_are_formula_derived_per_level() {
            use super::{Level, Theme};
            let theme = Theme::default();
            let ordered = [Level::Base, Level::Window, Level::Pane, Level::Panel, Level::Dialog, Level::Menu];
            for (k, level) in ordered.iter().enumerate() {
                assert_eq!(level.index(), k);
                let style = theme.glass(*level);
                assert!((style.alpha - (1.0 - k as f32 * ui_styling::levels::GLASS_ALPHA_STEP as f32)).abs() < 1e-6);
                assert!((style.blur_px - k as f32 * ui_styling::levels::GLASS_BLUR_STEP_PX as f32).abs() < 1e-6);
                assert_eq!(style.tint, theme.level_bg[k]);
            }
            assert!(theme.glass(Level::Base).alpha > theme.glass(Level::Menu).alpha);
            assert!(theme.glass(Level::Base).blur_px < theme.glass(Level::Menu).blur_px);
            assert_eq!(theme.surface(Level::Panel), theme.level_bg[Level::Panel.index()]);
        }
    }
    // #endregion draw
}

pub mod geometry {
    // #region geometry
    //! 📐️ Axis-aligned rectangles for layout and hit testing.

    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct Rect {
        pub x: f32,
        pub y: f32,
        pub w: f32,
        pub h: f32,
    }

    impl Rect {
        pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
            Self { x, y, w, h }
        }

        pub fn contains(&self, px: f32, py: f32) -> bool {
            px >= self.x && py >= self.y && px < self.x + self.w && py < self.y + self.h
        }

        pub fn inset(&self, amount: f32) -> Self {
            Self { x: self.x + amount, y: self.y + amount, w: (self.w - amount * 2.0).max(0.0), h: (self.h - amount * 2.0).max(0.0) }
        }
    }
    // #endregion geometry
}

/// 🗺️ Reusable minimap-navigator layout math — panel/viewport placement, content-fit checks, and
/// screen<->world mapping for a bottom-right pannable-camera minimap widget (wgpu parity with the dag
/// board's `MinimapWidget`). Relocated (as pure geometry, not the paint call) from
/// `♾️infinite/🎲️board/directed/🕸️dag`'s private `impl` methods — see
/// `.🦑️repo/🎫️tickets/26/08/05/FRAMEWORK-BUILDER-PASSTHROUGHS-APP-COMMANDS-MACRO-WIDGET-EXTRACTION`.
///
/// Deliberately NOT nested inside `widgets` (that module is `#[cfg(feature = "engine")]`, pulling in
/// wgpu/winit/parley/kernel_3d_scene): this math has zero rendering-backend dependency, so it lives at
/// the crate's lightweight (default-feature) tier instead, letting a vello/canvas-based consumer like the
/// dag board depend on `ui_wgpu` WITHOUT the heavyweight `engine` feature. The dag board's own
/// `paint_minimap_widget` (the actual `vello::Scene` fill/stroke calls, keyed off DAG-specific node types)
/// stays where it is — that part is genuinely backend- and app-specific, not portable geometry.
pub mod minimap {
    // #region minimap
    /// 🗺️ Computed screen-space layout for one minimap frame.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct MinimapLayout {
        pub panel: (f64, f64, f64, f64),
        pub world_min_x: f64,
        pub world_min_y: f64,
        pub scale: f64,
        pub map_origin_x: f64,
        pub map_origin_y: f64,
        pub viewport: (f64, f64, f64, f64),
    }

    /// 🗺️ Axis-aligned content bounds in world space (already padded by the caller).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct MinimapContentBounds {
        pub min_x: f64,
        pub min_y: f64,
        pub max_x: f64,
        pub max_y: f64,
    }

    /// 🗺️ True when a `viewport_w`x`viewport_h` viewport at `camera_zoom` centered on
    /// `(camera_x, camera_y)` already shows the whole of `content` (within `tolerance_px` screen
    /// pixels) — the minimap should hide itself in this case.
    pub fn content_fully_visible(content: &MinimapContentBounds, viewport_w: u32, viewport_h: u32, camera_x: f64, camera_y: f64, camera_zoom: f64, tolerance_px: f64) -> bool {
        let zoom = camera_zoom.max(1e-9);
        let half_w = viewport_w as f64 / (2.0 * zoom);
        let half_h = viewport_h as f64 / (2.0 * zoom);
        let tol = tolerance_px / zoom;
        camera_x - half_w <= content.min_x + tol && camera_x + half_w >= content.max_x - tol && camera_y - half_h <= content.min_y + tol && camera_y + half_h >= content.max_y - tol
    }

    /// 🗺️ Bottom-right inset panel of `panel_w`x`panel_h` with `margin` from the viewport edge, its
    /// content scaled to fit `content_fit_ratio` (clamped `0.5..=0.98`) of the panel, plus the camera's
    /// current view rect mapped into minimap-local coordinates.
    #[allow(clippy::too_many_arguments, reason = "mirrors the dag board's own MinimapWidgetLayout inputs 1:1 — a struct would just move the same arity into a constructor")]
    pub fn layout(content: &MinimapContentBounds, viewport_w: u32, viewport_h: u32, camera_x: f64, camera_y: f64, camera_zoom: f64, panel_w: f64, panel_h: f64, margin: f64, content_fit_ratio: f64) -> MinimapLayout {
        let ratio = content_fit_ratio.clamp(0.5, 0.98);
        let panel_x0 = viewport_w as f64 - margin - panel_w;
        let panel_y0 = viewport_h as f64 - margin - panel_h;
        let panel_x1 = panel_x0 + panel_w;
        let panel_y1 = panel_y0 + panel_h;
        let inset_x = panel_w * (1.0 - ratio) * 0.5;
        let inset_y = panel_h * (1.0 - ratio) * 0.5;
        let inner = (panel_x0 + inset_x, panel_y0 + inset_y, panel_x1 - inset_x, panel_y1 - inset_y);
        let inner_w = inner.2 - inner.0;
        let inner_h = inner.3 - inner.1;
        let cw = (content.max_x - content.min_x).max(1e-6);
        let ch = (content.max_y - content.min_y).max(1e-6);
        let scale = (inner_w / cw).min(inner_h / ch);
        let graph_w = cw * scale;
        let graph_h = ch * scale;
        let offset_x = inner.0 + (inner_w - graph_w) * 0.5;
        let offset_y = inner.1 + (inner_h - graph_h) * 0.5;
        let zoom = camera_zoom.max(1e-9);
        let half_w = viewport_w as f64 / (2.0 * zoom);
        let half_h = viewport_h as f64 / (2.0 * zoom);
        let view_min_x = camera_x - half_w;
        let view_min_y = camera_y - half_h;
        let view_max_x = camera_x + half_w;
        let view_max_y = camera_y + half_h;
        let to_mini = |wx: f64, wy: f64| (offset_x + (wx - content.min_x) * scale, offset_y + (wy - content.min_y) * scale);
        let (vx0, vy0) = to_mini(view_min_x, view_min_y);
        let (vx1, vy1) = to_mini(view_max_x, view_max_y);
        let viewport = (vx0.min(vx1), vy0.min(vy1), vx0.max(vx1), vy1.max(vy1));
        MinimapLayout { panel: (panel_x0, panel_y0, panel_x1, panel_y1), world_min_x: content.min_x, world_min_y: content.min_y, scale, map_origin_x: offset_x, map_origin_y: offset_y, viewport }
    }

    /// 🗺️ Inverse of `layout`'s world -> minimap mapping — a minimap-local screen point `(sx, sy)` back
    /// to world coordinates, given the `world_min_x`/`world_min_y`/`scale`/`map_origin_x`/`map_origin_y`
    /// a prior `layout()` call returned.
    pub fn screen_to_world(map_origin_x: f64, map_origin_y: f64, world_min_x: f64, world_min_y: f64, scale: f64, sx: f64, sy: f64) -> (f64, f64) {
        (world_min_x + (sx - map_origin_x) / scale, world_min_y + (sy - map_origin_y) / scale)
    }

    /// 🗺️ Point-in-axis-aligned-rect test — shared by panel/viewport hit-testing.
    pub fn point_in_rect((x0, y0, x1, y1): (f64, f64, f64, f64), sx: f64, sy: f64) -> bool {
        sx >= x0 && sx <= x1 && sy >= y0 && sy <= y1
    }
    // #endregion minimap
}

#[cfg(feature = "engine")]
pub mod gpu {
    // #region gpu
    //! 🖥️ WebGPU device, surface, and frame loop.

    use crate::draw::{DrawList, FrameBuffers, MeshGpuStore, RasterTextureStore, SceneColorTarget, UiPipelines};
    use crate::text::FontAtlas;
    use wgpu::Surface;

    pub struct GpuContext {
        device: wgpu::Device,
        queue: wgpu::Queue,
        surface: Surface<'static>,
        config: wgpu::SurfaceConfiguration,
        color_target_format: wgpu::TextureFormat,
        pipelines: UiPipelines,
        frame_buffers: FrameBuffers,
        depth_texture: Option<wgpu::Texture>,
        depth_view: Option<wgpu::TextureView>,
        mesh_store: MeshGpuStore,
        raster_store: RasterTextureStore,
        scene_color: Option<SceneColorTarget>,
        width: u32,
        height: u32,
        dpr: f32,
    }

    impl GpuContext {
        pub async fn from_window(window: std::sync::Arc<winit::window::Window>) -> Result<Self, String> {
            let dpr = window.scale_factor() as f32;
            let size = window.inner_size();
            let css_width = size.width as f32 / dpr;
            let css_height = size.height as f32 / dpr;
            let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor { backends: if cfg!(target_arch = "wasm32") { wgpu::Backends::BROWSER_WEBGPU } else { wgpu::Backends::PRIMARY }, ..Default::default() });
            let surface = instance.create_surface(wgpu::SurfaceTarget::Window(Box::new(window))).map_err(|err| format!("surface: {err:?}"))?;
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, compatible_surface: Some(&surface), force_fallback_adapter: false })
                .await
                .map_err(|err| format!("adapter: {err:?}"))?;
            let (device, queue) = adapter
                .request_device(&wgpu::DeviceDescriptor {
                    label: Some("ui_wgpu"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default().using_resolution(adapter.limits()),
                    memory_hints: Default::default(),
                    trace: wgpu::Trace::Off,
                    experimental_features: Default::default(),
                })
                .await
                .map_err(|err| format!("device: {err:?}"))?;
            let caps = surface.get_capabilities(&adapter);
            let surface_format = caps.formats.iter().copied().find(|f| !f.is_srgb()).unwrap_or(caps.formats[0]);
            let color_target_format = if surface_format.is_srgb() { surface_format } else { surface_format.add_srgb_suffix() };
            let width = (css_width * dpr).max(1.0) as u32;
            let height = (css_height * dpr).max(1.0) as u32;
            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface_format,
                width,
                height,
                present_mode: wgpu::PresentMode::AutoVsync,
                alpha_mode: caps.alpha_modes[0],
                view_formats: vec![color_target_format],
                desired_maximum_frame_latency: 2,
            };
            surface.configure(&device, &config);
            let pipelines = UiPipelines::new(&device, &queue, color_target_format);
            let raster_store = RasterTextureStore::new(&device, pipelines.bind_group_layout());
            let mut gpu = Self {
                device,
                queue,
                surface,
                config,
                color_target_format,
                pipelines,
                frame_buffers: FrameBuffers::default(),
                depth_texture: None,
                depth_view: None,
                mesh_store: MeshGpuStore::default(),
                raster_store,
                scene_color: None,
                width,
                height,
                dpr,
            };
            gpu.ensure_depth();
            Ok(gpu)
        }

        fn ensure_depth(&mut self) {
            let depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("ui_depth"),
                size: wgpu::Extent3d { width: self.width.max(1), height: self.height.max(1), depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: self.pipelines.depth_format(),
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            });
            let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
            self.depth_texture = Some(depth_texture);
            self.depth_view = Some(depth_view);
        }

        pub fn resize(&mut self, css_width: f32, css_height: f32, dpr: f32) {
            self.dpr = dpr;
            let width = (css_width * dpr).max(1.0) as u32;
            let height = (css_height * dpr).max(1.0) as u32;
            if width == self.width && height == self.height {
                return;
            }
            self.width = width;
            self.height = height;
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.scene_color = None;
            self.ensure_depth();
        }

        fn ensure_scene_color(&mut self) {
            SceneColorTarget::ensure(&self.device, &mut self.scene_color, self.width, self.height, self.color_target_format);
        }

        pub fn mesh_store_mut(&mut self) -> &mut MeshGpuStore {
            &mut self.mesh_store
        }

        pub fn ensure_mesh(&mut self, key: &str, version: u64, positions: &[f32], normals: &[f32], indices: &[u32]) {
            self.mesh_store.ensure_mesh(&self.device, key, version, positions, normals, indices);
        }

        pub fn evict_mesh(&mut self, key: &str) {
            self.mesh_store.evict_mesh(key);
        }

        pub fn render_frame(&mut self, draw: &DrawList, overlay: Option<&DrawList>, time_seconds: f32) -> Result<(), String> {
            self.ensure_scene_color();
            let scene = self.scene_color.as_ref().expect("scene_color");
            let frame = self.surface.get_current_texture().map_err(|err| format!("frame: {err:?}"))?;
            let view = frame.texture.create_view(&wgpu::TextureViewDescriptor { format: Some(self.color_target_format), ..Default::default() });
            let depth_view = self.depth_view.as_ref();
            let mut scene_encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("ui_wgpu_scene") });
            self.pipelines.render_scene_content(&self.device, &self.queue, &mut scene_encoder, scene, depth_view, draw, &self.mesh_store, &self.raster_store, &mut self.frame_buffers, self.width as f32, self.height as f32, time_seconds);
            self.queue.submit(Some(scene_encoder.finish()));
            let mut composite_encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("ui_wgpu_composite") });
            self.pipelines.composite_to_swapchain(&self.device, &self.queue, &mut composite_encoder, &view, scene, depth_view, draw, overlay, &self.mesh_store, &self.raster_store, &mut self.frame_buffers, self.width as f32, self.height as f32);
            self.queue.submit(Some(composite_encoder.finish()));
            frame.present();
            Ok(())
        }

        pub fn upload_font_atlas(&self, atlas: &FontAtlas) {
            self.pipelines.upload_glyph_atlas(&self.queue, &atlas.pixels, atlas.width, atlas.height);
        }

        pub fn upload_icon_atlas(&self, atlas: &crate::draw::IconAtlas) {
            self.pipelines.upload_icon_atlas(&self.queue, &atlas.pixels, atlas.width, atlas.height);
        }

        pub fn ensure_raster_texture(&mut self, key: &str, pixels: &[u8], width: u32, height: u32) {
            self.raster_store.ensure_raster(
                &self.device,
                &self.queue,
                self.pipelines.globals_buffer(),
                &self.pipelines.glyph_view(),
                self.pipelines.glyph_sampler(),
                &self.pipelines.icon_view(),
                self.pipelines.icon_sampler(),
                key,
                pixels,
                width,
                height,
            );
        }

        pub fn ensure_world_plane_texture(&mut self, key: &str, pixels: &[u8], width: u32, height: u32) {
            self.ensure_raster_texture(key, pixels, width, height);
        }

        pub fn device(&self) -> &wgpu::Device {
            &self.device
        }

        pub fn queue(&self) -> &wgpu::Queue {
            &self.queue
        }

        pub fn dpr(&self) -> f32 {
            self.dpr
        }

        pub fn register_engine_texture(&mut self, key: &str, texture: wgpu::Texture, view: &wgpu::TextureView, width: u32, height: u32) {
            self.raster_store.replace_gpu_bind_group(&self.device, self.pipelines.globals_buffer(), &self.pipelines.glyph_view(), self.pipelines.glyph_sampler(), key, view, texture, width, height);
        }

        pub fn width(&self) -> u32 {
            self.width
        }

        pub fn height(&self) -> u32 {
            self.height
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn schedule_frame(window: &winit::window::Window, callback: impl FnMut() + 'static) {
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;

        let mut callback = callback;
        let closure = Closure::wrap(Box::new(move || {
            callback();
        }) as Box<dyn FnMut()>);
        web_sys::window().and_then(|w| w.request_animation_frame(closure.as_ref().unchecked_ref()).ok());
        closure.forget();
        let _ = window;
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn schedule_frame(window: &winit::window::Window, _callback: impl FnMut() + 'static) {
        window.request_redraw();
    }
    // #endregion gpu
}

#[cfg(feature = "engine")]
pub mod input {
    // #region input
    //! 🖱️ Pointer and keyboard input state for hit testing.

    use crate::geometry::Rect;
    use std::rc::Rc;

    use std::collections::HashMap;

    #[derive(Clone, Debug)]
    pub struct HitTarget<E> {
        pub rect: Rect,
        pub event: Option<E>,
        pub control_id: Option<String>,
        pub kind: HitKind,
        pub drag_axis: Option<DragAxis>,
        pub drag_data: Option<HashMap<String, String>>,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DragAxis {
        Horizontal,
        Vertical,
        Both,
        Ring,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TreeDropPosition {
        Before,
        After,
        Inside,
    }

    #[derive(Clone, Debug)]
    pub struct TreeDragState {
        pub source_id: String,
        pub drag_data: HashMap<String, String>,
        pub x: f32,
        pub y: f32,
        pub drop_target_id: Option<String>,
        pub drop_position: TreeDropPosition,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HitKind {
        Button,
        Toggle,
        Input,
        Select,
        Slider,
        TreeItem,
        TreeDropTarget,
        PanelTab,
        NavbarItem,
        Window,
        World3d,
        PanelResize,
        DockSplit,
        DockJoinCorner,
        ScrollRegion,
        ContextMenu,
        DropdownItem,
        Generic,
    }

    #[derive(Clone, Debug, Default)]
    pub struct PointerModifiers {
        pub shift: bool,
        pub ctrl: bool,
        pub alt: bool,
        pub meta: bool,
    }

    impl PointerModifiers {
        pub fn ctrl_or_meta(&self) -> bool {
            self.ctrl || self.meta
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct DragState {
        pub active: bool,
        pub button: i16,
        pub start_x: f32,
        pub start_y: f32,
        pub current_x: f32,
        pub current_y: f32,
        pub target_id: Option<String>,
        pub axis: Option<DragAxis>,
        pub kind: Option<HitKind>,
        pub points: Vec<[f32; 2]>,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum KeyAction {
        Char(String),
        Backspace,
        Delete,
        Enter,
        Escape,
        ArrowLeft,
        ArrowRight,
        ArrowUp,
        ArrowDown,
        Tab,
        Space(bool),
    }

    pub struct InputState<E> {
        pub pointer_x: f32,
        pub pointer_y: f32,
        pub pointer_down: bool,
        pub pointer_button: i16,
        pub wheel_delta: f32,
        pub modifiers: PointerModifiers,
        pub drag: DragState,
        pub hovered_id: Option<String>,
        pub focused_id: Option<String>,
        pub text_buffer: String,
        pub cursor_pos: usize,
        pub hit_targets: Vec<HitTarget<E>>,
        pub pending_events: Vec<E>,
        pub pending_keys: Vec<KeyAction>,
        pub right_click_pos: Option<(f32, f32)>,
    }

    impl<E> Default for InputState<E> {
        fn default() -> Self {
            Self {
                pointer_x: 0.0,
                pointer_y: 0.0,
                pointer_down: false,
                pointer_button: 0,
                wheel_delta: 0.0,
                modifiers: PointerModifiers::default(),
                drag: DragState::default(),
                hovered_id: None,
                focused_id: None,
                text_buffer: String::new(),
                cursor_pos: 0,
                hit_targets: Vec::new(),
                pending_events: Vec::new(),
                pending_keys: Vec::new(),
                right_click_pos: None,
            }
        }
    }

    impl<E: Clone> InputState<E> {
        pub fn clear_frame(&mut self) {
            self.hit_targets.clear();
            self.wheel_delta = 0.0;
            self.right_click_pos = None;
        }

        pub fn register_hit(&mut self, target: HitTarget<E>) {
            self.hit_targets.push(target);
        }

        pub fn hit_at(&self, x: f32, y: f32) -> Option<&HitTarget<E>> {
            self.hit_targets.iter().rev().find(|target| target.rect.contains(x, y))
        }

        pub fn update_hover(&mut self, x: f32, y: f32) {
            self.pointer_x = x;
            self.pointer_y = y;
            self.hovered_id = self.hit_at(x, y).and_then(|hit| hit.control_id.clone());
        }

        pub fn begin_drag(&mut self, x: f32, y: f32, button: i16, target_id: Option<String>, axis: Option<DragAxis>, kind: Option<HitKind>) {
            self.drag = DragState { active: true, button, start_x: x, start_y: y, current_x: x, current_y: y, target_id, axis, kind, points: vec![[x, y]] };
        }

        pub fn update_drag(&mut self, x: f32, y: f32) {
            if self.drag.active {
                self.drag.current_x = x;
                self.drag.current_y = y;
                self.drag.points.push([x, y]);
            }
        }

        pub fn end_drag(&mut self) -> DragState {
            let drag = self.drag.clone();
            self.drag = DragState::default();
            drag
        }

        pub fn drain_events(&mut self) -> Vec<E> {
            std::mem::take(&mut self.pending_events)
        }

        pub fn drain_keys(&mut self) -> Vec<KeyAction> {
            std::mem::take(&mut self.pending_keys)
        }

        pub fn queue_event(&mut self, event: E) {
            self.pending_events.push(event);
        }

        pub fn queue_key(&mut self, action: KeyAction) {
            self.pending_keys.push(action);
        }

        pub fn focus_input(&mut self, id: &str, value: &str) {
            self.focused_id = Some(id.to_string());
            self.text_buffer = value.to_string();
            self.cursor_pos = value.len();
        }

        pub fn blur_input(&mut self) {
            self.focused_id = None;
            self.text_buffer.clear();
            self.cursor_pos = 0;
        }

        pub fn insert_char(&mut self, ch: char) {
            if self.cursor_pos <= self.text_buffer.len() {
                self.text_buffer.insert(self.cursor_pos, ch);
                self.cursor_pos += 1;
            }
        }

        pub fn backspace(&mut self) {
            if self.cursor_pos > 0 {
                self.cursor_pos -= 1;
                self.text_buffer.remove(self.cursor_pos);
            }
        }

        pub fn delete_forward(&mut self) {
            if self.cursor_pos < self.text_buffer.len() {
                self.text_buffer.remove(self.cursor_pos);
            }
        }

        pub fn move_cursor(&mut self, delta: i32) {
            let len = self.text_buffer.len() as i32;
            self.cursor_pos = ((self.cursor_pos as i32) + delta).clamp(0, len) as usize;
        }
    }

    #[derive(Clone)]
    pub struct PointerCallbacks {
        pub on_move: Rc<dyn Fn(f32, f32, bool, i16, PointerModifiers)>,
        pub on_button: Rc<dyn Fn(f32, f32, bool, i16, PointerModifiers)>,
        pub on_wheel: Rc<dyn Fn(f32, f32, f32, PointerModifiers)>,
        pub on_key: Rc<dyn Fn(KeyAction, PointerModifiers)>,
        pub on_context_menu: Rc<dyn Fn(f32, f32)>,
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn hit_at_prefers_content_registered_after_scroll_region() {
            let mut input = InputState::<()>::default();
            let scroll = Rect::new(0.0, 0.0, 200.0, 200.0);
            let row = Rect::new(0.0, 24.0, 200.0, 24.0);
            input.register_hit(HitTarget { rect: scroll, event: None, control_id: Some("scroll".into()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
            input.register_hit(HitTarget { rect: row, event: None, control_id: Some("tree.label.item-1".into()), kind: HitKind::TreeItem, drag_axis: None, drag_data: None });
            let hit = input.hit_at(10.0, 36.0).expect("row point should hit");
            assert_eq!(hit.control_id.as_deref(), Some("tree.label.item-1"));
            assert_eq!(hit.kind, HitKind::TreeItem);
        }
    }
    // #endregion input
}

#[cfg(feature = "engine")]
pub mod layout {
    // #region layout
    //! 🧮️ Flex stack layout for widget trees.

    use crate::geometry::Rect;
    use crate::theme::Theme;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Direction {
        Vertical,
        Horizontal,
    }

    pub fn gap_for_token(theme: &Theme, token: Option<&str>) -> f32 {
        match token {
            Some("tight") => 4.0,
            Some("loose") => 12.0,
            Some("none") | Some("0") => 0.0,
            _ => theme.gap_standard,
        }
    }

    pub fn padding_for_token(theme: &Theme, token: Option<&str>) -> f32 {
        match token {
            Some("none") | Some("0") => 0.0,
            Some("tight") => 6.0,
            Some("loose") => 16.0,
            _ => theme.padding_standard,
        }
    }

    pub fn layout_vertical(bounds: Rect, gap: f32, padding: f32, child_heights: &[f32]) -> Vec<Rect> {
        let inner = bounds.inset(padding);
        let total_gap = gap * (child_heights.len().saturating_sub(1) as f32);
        let total_children: f32 = child_heights.iter().sum();
        let mut y = inner.y;
        let mut rects = Vec::with_capacity(child_heights.len());
        let available = (inner.h - total_gap - total_children).max(0.0);
        let extra_per_child = if child_heights.is_empty() { 0.0 } else { available / child_heights.len() as f32 };
        for &height in child_heights {
            let h = height + extra_per_child;
            rects.push(Rect::new(inner.x, y, inner.w, h));
            y += h + gap;
        }
        rects
    }

    pub fn layout_horizontal(bounds: Rect, gap: f32, padding: f32, child_widths: &[f32]) -> Vec<Rect> {
        let inner = bounds.inset(padding);
        let total_gap = gap * (child_widths.len().saturating_sub(1) as f32);
        let total_children: f32 = child_widths.iter().sum();
        let mut x = inner.x;
        let mut rects = Vec::with_capacity(child_widths.len());
        let available = (inner.w - total_gap - total_children).max(0.0);
        let extra_per_child = if child_widths.is_empty() { 0.0 } else { available / child_widths.len() as f32 };
        for &width in child_widths {
            let w = width + extra_per_child;
            rects.push(Rect::new(x, inner.y, w, inner.h));
            x += w + gap;
        }
        rects
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn vertical_layout_distributes_children() {
            let theme = Theme::default();
            let bounds = Rect::new(0.0, 0.0, 100.0, 100.0);
            let rects = layout_vertical(bounds, 4.0, 8.0, &[20.0, 30.0]);
            assert_eq!(rects.len(), 2);
            assert!(rects[0].h > 20.0);
            assert!(rects[1].y > rects[0].y);
            let _ = theme;
        }
    }
    // #endregion layout
}

#[cfg(feature = "engine")]
pub mod flex {
    // #region flex
    //! 📏️ Taffy-backed flex layout for the retained tree (`tree`/`reconcile`). Taffy's own types
    //! (`taffy::TaffyTree`, `taffy::NodeId`, `taffy::Style`, …) are fully wrapped by `LayoutEngine` and
    //! never appear in any item visible outside this crate — `LayoutEngine` itself is `pub(crate)`
    //! (narrowest visibility that compiles) since only the retained `engine` façade needs it.
    //! Style mapping reuses `layout::gap_for_token`/`layout::padding_for_token` (the old immediate-mode
    //! `layout` region stays in place — `widgets`/`chrome` still call its `layout_vertical`/
    //! `layout_horizontal` directly, so it isn't deleted this milestone). Pixel-parity requirements:
    //! every child of a `Stack` gets `flex_grow: 1.0` so leftover main-axis space distributes equally
    //! among siblings, matching the old hand-rolled stack layout's `extra_per_child` behaviour; a
    //! `Field`'s sole synthetic child (`reconcile::children_of`) gets the same treatment so it fills the
    //! label-adjusted remainder `widgets::render_widget`'s `WidgetNode::Field` branch carves out (see
    //! `apply_field_metrics`) — a `Section`'s synthetic children deliberately do *not* grow (see
    //! `apply_section_metrics`), since `WidgetNode::Section` stacks them at their own intrinsic size with
    //! no `extra_per_child`-style redistribution, unlike a `Stack`'s or `Field`'s.

    use std::collections::HashMap;

    use taffy::prelude::*;

    use crate::arena::NodeId;
    use crate::component::ui::UiNode;
    use crate::layout::{gap_for_token, padding_for_token};
    use crate::text::FontAtlas;
    use crate::theme::Theme;
    use crate::tree::{NodeFlags, UiTree};

    /// 🖋️ Default text size (px) used for intrinsic measurement during layout, ahead of the per-node
    /// resolved style a later paint milestone introduces.
    const DEFAULT_TEXT_SIZE_PX: f32 = 14.0;

    /// 🍃️ Per-taffy-leaf context: which retained nodes need a measure callback (only `Text`) and which
    /// don't (everything else measures as zero-size content, matching the pre-taffy immediate-mode
    /// widgets that size themselves from fixed control-height/theme metrics rather than intrinsic text).
    enum LeafContext {
        None,
        Text(String),
    }

    /// 🖇️ Reads intrinsic content size for taffy's leaf-measurement callback. Implemented for
    /// `text::FontAtlas` so taffy can ask fontdue for wrap-aware text metrics without ui_wgpu's flex
    /// module depending on fontdue directly.
    pub(crate) trait TextMeasure {
        fn measure(&mut self, text: &str, max_width: Option<f32>) -> (f32, f32);
    }

    impl TextMeasure for FontAtlas {
        fn measure(&mut self, text: &str, max_width: Option<f32>) -> (f32, f32) {
            match max_width {
                Some(width) if width > 0.0 => self.measure_text_wrapped(text, width, DEFAULT_TEXT_SIZE_PX),
                _ => self.measure_text(text, DEFAULT_TEXT_SIZE_PX),
            }
        }
    }

    fn quantize_width(width: Option<f32>) -> Option<u32> {
        width.map(|w| w.round().max(0.0) as u32)
    }

    /// 📦️ Maps a retained node's `WidgetSpec` to a taffy `Style`. `Stack` becomes a real flex container
    /// (direction/gap/padding from its own fields); `Field`/`Section` become a column flex container too
    /// (their reconciled synthetic child(ren) need real flexbox participation to match `widgets`' hand-
    /// rolled geometry — see `apply_field_metrics`/`apply_section_metrics`), zeroed here since both are
    /// theme-dependent; every other variant is a content leaf (auto-sized, measured via `LeafContext`
    /// where applicable). `flex_grow` is layered on top by the caller for children of a `Stack`/`Field`,
    /// not set here, since it depends on the *parent's* kind.
    fn style_for(node: &UiNode) -> Style {
        match node {
            UiNode::Stack(stack) => {
                let vertical = stack.direction != "horizontal";
                Style {
                    display: Display::Flex,
                    flex_direction: if vertical { FlexDirection::Column } else { FlexDirection::Row },
                    gap: Size { width: length(0.0_f32), height: length(0.0_f32) },
                    padding: Rect { left: length(0.0_f32), right: length(0.0_f32), top: length(0.0_f32), bottom: length(0.0_f32) },
                    size: Size { width: auto(), height: auto() },
                    ..Default::default()
                }
            }
            UiNode::Field(_) | UiNode::Section(_) => Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                gap: Size { width: length(0.0_f32), height: length(0.0_f32) },
                padding: Rect { left: length(0.0_f32), right: length(0.0_f32), top: length(0.0_f32), bottom: length(0.0_f32) },
                size: Size { width: auto(), height: auto() },
                ..Default::default()
            },
            _ => Style { size: Size { width: auto(), height: auto() }, ..Default::default() },
        }
    }

    /// 🎚️ Applies `theme`-resolved gap/padding onto a freshly built `Stack` style (kept separate from
    /// `style_for` so the latter stays theme-independent and trivially testable).
    fn apply_stack_metrics(style: &mut Style, stack: &crate::component::ui::UiStackNode, theme: &Theme) {
        let gap = gap_for_token(theme, stack.gap.as_deref());
        let padding = padding_for_token(theme, stack.padding.as_deref());
        style.gap = Size { width: length(gap), height: length(gap) };
        style.padding = Rect { left: length(padding), right: length(padding), top: length(padding), bottom: length(padding) };
    }

    /// 🎚️ `widgets::render_widget`'s `WidgetNode::Field` branch: `Rect::new(bounds.x, bounds.y + label_h
    /// + gap, bounds.w, bounds.h - label_h - gap)`, where `label_h = theme.font_size_small` and
    /// `gap = gap_for_token(theme, Some("standard"))`. Reserving that same top padding on `Field`'s own
    /// taffy container, combined with `style_with_grow` granting its sole child `flex_grow: 1.0`, resolves
    /// that child to the identical rect taffy-side (default `align_items: Stretch` already matches the
    /// full `bounds.w`, since `Field`'s container has no left/right padding).
    fn apply_field_metrics(style: &mut Style, theme: &Theme) {
        let label_h = theme.font_size_small;
        let gap = gap_for_token(theme, Some("standard"));
        style.padding.top = length(label_h + gap);
    }

    /// 🔖️ Mirrors `widgets`'/`paint`'s own private `PANEL_HEADER` constant: the header-row height
    /// `WidgetNode::Section`'s branch reserves for its content unconditionally (`y = bounds.y +
    /// PANEL_HEADER`, even when `label` is `None` — only the header's chevron+text *paint* is gated on
    /// `label.is_some()`, not this offset).
    const SECTION_HEADER_HEIGHT: f32 = 24.0;

    /// 🎚️ `WidgetNode::Section`'s branch stacks its children with a plain `y += h + ctx.theme.gap_standard`
    /// loop — each kept at its own intrinsic size, never `layout_vertical`'s `extra_per_child` leftover
    /// redistribution (unlike a `Stack`'s or `Field`'s child; see `apply_field_metrics`). Reserving the
    /// header offset as top padding and `theme.gap_standard` as the inter-row gap reproduces that
    /// positioning without granting `flex_grow` — `style_with_grow`'s `flex_grow_child` gate deliberately
    /// stays `Stack`/`Field`-only.
    fn apply_section_metrics(style: &mut Style, theme: &Theme) {
        style.padding.top = length(SECTION_HEADER_HEIGHT);
        style.gap = Size { width: length(0.0_f32), height: length(theme.gap_standard) };
    }

    fn leaf_context(node: &UiNode) -> LeafContext {
        match node {
            UiNode::Text(text) => LeafContext::Text(text.value.clone().into_string()),
            _ => LeafContext::None,
        }
    }

    /// 🧮️ Owns a taffy flexbox tree mirroring one retained `UiTree` and the `NodeId -> taffy::NodeId`
    /// mapping between them. Used only by the `engine` façade (a later milestone); never exposed
    /// outside the crate.
    pub(crate) struct LayoutEngine {
        taffy: TaffyTree<LeafContext>,
        mapping: HashMap<NodeId, taffy::NodeId>,
    }

    impl Default for LayoutEngine {
        fn default() -> Self {
            Self { taffy: TaffyTree::new(), mapping: HashMap::new() }
        }
    }

    impl LayoutEngine {
        pub(crate) fn new() -> Self {
            Self::default()
        }

        /// 🔁️ Runs a taffy layout pass over `tree` rooted at `root` if (and only if) anything in the
        /// tree carries `DIRTY_LAYOUT`/`SUBTREE_DIRTY` (checking the root alone suffices — `mark_dirty`
        /// always bubbles `SUBTREE_DIRTY` to the root, so an all-clean tree is a single flag read, no
        /// walk). Returns whether a layout pass actually ran.
        pub(crate) fn compute(&mut self, tree: &mut UiTree, root: NodeId, atlas: &mut FontAtlas, theme: &Theme, available_width: f32, available_height: f32) -> bool {
            let Some(root_node) = tree.node(root) else { return false };
            let needs_layout = root_node.flags.contains(NodeFlags::DIRTY_LAYOUT) || root_node.flags.contains(NodeFlags::SUBTREE_DIRTY);
            if !needs_layout {
                return false;
            }

            self.prune_removed(tree);
            let root_taffy = self.sync(tree, theme, root, false);

            let mut root_style = self.taffy.style(root_taffy).cloned().unwrap_or_default();
            root_style.size = Size { width: length(available_width), height: length(available_height) };
            let _ = self.taffy.set_style(root_taffy, root_style);

            let available = Size { width: AvailableSpace::Definite(available_width), height: AvailableSpace::Definite(available_height) };
            let _ = self.taffy.compute_layout_with_measure(root_taffy, available, |known_dimensions, available_space, _node_id, node_context, _style| {
                if let (Some(width), Some(height)) = (known_dimensions.width, known_dimensions.height) {
                    return Size { width, height };
                }
                match node_context {
                    Some(LeafContext::Text(text)) => {
                        let max_width = known_dimensions.width.or_else(|| available_space.width.into_option());
                        let (measured_w, measured_h) = atlas.measure(text, max_width);
                        Size { width: known_dimensions.width.unwrap_or(measured_w), height: known_dimensions.height.unwrap_or(measured_h) }
                    }
                    _ => Size::ZERO,
                }
            });

            self.write_back(tree, atlas, root);
            true
        }

        /// 🧹️ Drops taffy-side nodes whose retained counterpart no longer exists (removed by
        /// `reconcile`), so the mapping doesn't grow unbounded across the tree's lifetime.
        fn prune_removed(&mut self, tree: &UiTree) {
            let stale: Vec<NodeId> = self.mapping.keys().copied().filter(|id| !tree.contains(*id)).collect();
            for id in stale {
                if let Some(taffy_id) = self.mapping.remove(&id) {
                    let _ = self.taffy.remove(taffy_id);
                }
            }
        }

        /// 🌲️ Depth-first: ensures every retained node reachable from `id` has a taffy counterpart,
        /// refreshing style/children only for nodes that are new or carry `DIRTY_LAYOUT` (everything
        /// else keeps its existing taffy node untouched, letting taffy's own layout cache skip
        /// recomputation for genuinely unchanged subtrees). `flex_grow_child` is true when `id`'s parent
        /// is a `Stack` or `Field` — the two kinds whose child(ren) should grow to fill leftover space
        /// (a `Section`'s children deliberately don't; see `apply_section_metrics`).
        fn sync(&mut self, tree: &UiTree, theme: &Theme, id: NodeId, flex_grow_child: bool) -> taffy::NodeId {
            let node = tree.node(id).expect("sync called with a live NodeId");
            let grows_children = matches!(node.spec.0, UiNode::Stack(_) | UiNode::Field(_));
            let dirty = node.flags.contains(NodeFlags::DIRTY_LAYOUT);
            let existing = self.mapping.get(&id).copied();

            let children: Vec<NodeId> = tree.children(id).collect();
            let child_taffy_ids: Vec<taffy::NodeId> = children.iter().map(|&child_id| self.sync(tree, theme, child_id, grows_children)).collect();

            let taffy_id = match existing {
                Some(taffy_id) if !dirty => taffy_id,
                Some(taffy_id) => {
                    let style = self.style_with_grow(&node.spec.0, theme, flex_grow_child);
                    let _ = self.taffy.set_style(taffy_id, style);
                    taffy_id
                }
                None => {
                    let style = self.style_with_grow(&node.spec.0, theme, flex_grow_child);
                    self.taffy.new_leaf_with_context(style, leaf_context(&node.spec.0)).expect("taffy leaf insert")
                }
            };
            if existing.is_none() || dirty {
                let _ = self.taffy.set_children(taffy_id, &child_taffy_ids);
            }
            self.mapping.insert(id, taffy_id);
            taffy_id
        }

        fn style_with_grow(&self, node: &UiNode, theme: &Theme, flex_grow_child: bool) -> Style {
            let mut style = style_for(node);
            match node {
                UiNode::Stack(stack) => apply_stack_metrics(&mut style, stack, theme),
                UiNode::Field(_) => apply_field_metrics(&mut style, theme),
                UiNode::Section(_) => apply_section_metrics(&mut style, theme),
                _ => {}
            }
            if flex_grow_child {
                style.flex_grow = 1.0;
            }
            style
        }

        /// 📝️ Copies taffy's resolved `location`/`size` into each node's `LayoutBucket` (parent-relative,
        /// same space taffy itself uses — see `tree::LayoutBucket`'s doc comment) and clears
        /// `DIRTY_LAYOUT`. Text nodes also get their `cached_text_measure` refreshed at the node's final
        /// resolved width, so a following unchanged-constraint measurement is a cache hit.
        fn write_back(&mut self, tree: &mut UiTree, atlas: &mut FontAtlas, id: NodeId) {
            if let Some(&taffy_id) = self.mapping.get(&id) {
                if let Ok(layout) = self.taffy.layout(taffy_id) {
                    let (x, y, width, height) = (layout.location.x, layout.location.y, layout.size.width, layout.size.height);
                    let text_value = match tree.node(id).map(|n| &n.spec.0) {
                        Some(UiNode::Text(text)) => Some(text.value.clone().into_string()),
                        _ => None,
                    };
                    if let Some(node) = tree.node_mut(id) {
                        node.layout.x = x;
                        node.layout.y = y;
                        node.layout.width = width;
                        node.layout.height = height;
                        node.flags.set(NodeFlags::DIRTY_LAYOUT, false);
                        // write_back always walks the whole subtree from `root`, so by the time it
                        // finishes every descendant is up to date and SUBTREE_DIRTY can clear too —
                        // otherwise it would never clear and `compute`'s early-out would never fire.
                        node.flags.set(NodeFlags::SUBTREE_DIRTY, false);
                    }
                    if let Some(value) = text_value {
                        let key = (value.clone(), quantize_width(Some(width)));
                        let already_cached = tree.node(id).and_then(|n| n.layout.cached_text_measure.as_ref().map(|(k, _)| k.clone())) == Some(key.clone());
                        if !already_cached {
                            let measured = atlas.measure(&value, Some(width));
                            if let Some(node) = tree.node_mut(id) {
                                node.layout.cached_text_measure = Some((key, measured));
                            }
                        }
                    }
                }
            }
            let children: Vec<NodeId> = tree.children(id).collect();
            for child in children {
                self.write_back(tree, atlas, child);
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::component::ui::{UiFieldNode, UiPresence, UiSectionNode, UiStackNode, UiTextNode};

        fn text(value: &str) -> UiNode {
            UiNode::Text(UiTextNode { value: value.into(), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })
        }

        fn stack(direction: &str, children: Vec<UiNode>) -> UiNode {
            UiNode::Stack(UiStackNode { direction: direction.into(), gap: Some("none".into()), padding: Some("none".into()), id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children, menu: None })
        }

        #[test]
        fn vertical_stack_lays_children_top_to_bottom_with_correct_y_offsets() {
            let mut tree = UiTree::new();
            tree.apply_tree(&stack("vertical", vec![text("hello"), text("a longer line of text")]));
            let root = tree.root.unwrap();
            let mut engine = LayoutEngine::new();
            let mut atlas = FontAtlas::builtin();
            let theme = Theme::default();

            let ran = engine.compute(&mut tree, root, &mut atlas, &theme, 400.0, 400.0);
            assert!(ran);

            let children: Vec<NodeId> = tree.children(root).collect();
            assert_eq!(children.len(), 2);
            let first = &tree.node(children[0]).unwrap().layout;
            let second = &tree.node(children[1]).unwrap().layout;
            assert_eq!(first.y, 0.0);
            assert!(second.y >= first.y + first.height);
        }

        #[test]
        fn horizontal_stack_distributes_equal_leftover_width_across_children() {
            let mut tree = UiTree::new();
            let children = vec![
                UiNode::Separator(crate::component::ui::UiSeparatorNode { presence: UiPresence::default(), menu: None }),
                UiNode::Separator(crate::component::ui::UiSeparatorNode { presence: UiPresence::default(), menu: None }),
                UiNode::Separator(crate::component::ui::UiSeparatorNode { presence: UiPresence::default(), menu: None }),
            ];
            tree.apply_tree(&stack("horizontal", children));
            let root = tree.root.unwrap();
            let mut engine = LayoutEngine::new();
            let mut atlas = FontAtlas::builtin();
            let theme = Theme::default();

            engine.compute(&mut tree, root, &mut atlas, &theme, 300.0, 100.0);

            let widths: Vec<f32> = tree.children(root).map(|id| tree.node(id).unwrap().layout.width).collect();
            assert_eq!(widths.len(), 3);
            for w in &widths {
                assert!((*w - 100.0).abs() < 0.5, "expected equal-thirds width, got {w}");
            }
        }

        #[test]
        fn recomputing_with_nothing_dirty_is_a_no_operation() {
            let mut tree = UiTree::new();
            tree.apply_tree(&stack("vertical", vec![text("hello")]));
            let root = tree.root.unwrap();
            let mut engine = LayoutEngine::new();
            let mut atlas = FontAtlas::builtin();
            let theme = Theme::default();

            assert!(engine.compute(&mut tree, root, &mut atlas, &theme, 200.0, 200.0));
            let first_pass = tree.node(root).unwrap().layout.clone();

            let ran_again = engine.compute(&mut tree, root, &mut atlas, &theme, 200.0, 200.0);
            assert!(!ran_again, "DIRTY_LAYOUT/SUBTREE_DIRTY are cleared after a pass, so a second call must early-out");
            let second_pass = tree.node(root).unwrap().layout.clone();
            assert_eq!((first_pass.x, first_pass.y, first_pass.width, first_pass.height), (second_pass.x, second_pass.y, second_pass.width, second_pass.height));
        }

        #[test]
        fn text_measurement_is_cached_per_unchanged_width() {
            let mut atlas = FontAtlas::builtin();
            let first = atlas.measure("hello world", Some(120.0));
            let second = atlas.measure("hello world", Some(120.0));
            assert_eq!(first, second);

            let mut tree = UiTree::new();
            tree.apply_tree(&stack("vertical", vec![text("cache me")]));
            let root = tree.root.unwrap();
            let mut engine = LayoutEngine::new();
            let theme = Theme::default();
            engine.compute(&mut tree, root, &mut atlas, &theme, 200.0, 200.0);

            let child = tree.children(root).next().unwrap();
            let cached = tree.node(child).unwrap().layout.cached_text_measure.clone();
            assert!(cached.is_some(), "text node should have a cached measurement after layout");
        }

        //#region 🔖️FieldSectionGrowSemantics
        #[test]
        fn field_child_grows_to_fill_the_label_adjusted_remainder() {
            let mut tree = UiTree::new();
            let field = UiNode::Field(UiFieldNode { id: "f".into(), label: "Label".into(), description: None, required: None, error: None, child: Box::new(text("child")), presence: UiPresence::default(), menu: None });
            tree.apply_tree(&field);
            let root = tree.root.unwrap();
            let mut engine = LayoutEngine::new();
            let mut atlas = FontAtlas::builtin();
            let theme = Theme::default();

            engine.compute(&mut tree, root, &mut atlas, &theme, 200.0, 100.0);

            let child = tree.children(root).next().unwrap();
            let layout = &tree.node(child).unwrap().layout;
            let label_h = theme.font_size_small;
            let gap = gap_for_token(&theme, Some("standard"));
            assert!((layout.y - (label_h + gap)).abs() < 0.5, "child should start below the label, got y={}", layout.y);
            assert!((layout.height - (100.0 - label_h - gap)).abs() < 0.5, "child should fill the label-adjusted remainder, got height={}", layout.height);
            assert!((layout.width - 200.0).abs() < 0.5, "child should stretch to the field's full width, got width={}", layout.width);
        }

        #[test]
        fn section_children_stack_below_the_header_at_their_own_intrinsic_height_with_gap() {
            let mut tree = UiTree::new();
            let section = UiNode::Section(UiSectionNode { id: "s".into(), label: Some("Section".into()), default_open: Some(true), presence: UiPresence::default(), children: vec![text("a"), text("a longer line of text")], menu: None });
            tree.apply_tree(&section);
            let root = tree.root.unwrap();
            let mut engine = LayoutEngine::new();
            let mut atlas = FontAtlas::builtin();
            let theme = Theme::default();

            engine.compute(&mut tree, root, &mut atlas, &theme, 200.0, 200.0);

            let children: Vec<NodeId> = tree.children(root).collect();
            assert_eq!(children.len(), 2);
            let first = tree.node(children[0]).unwrap().layout.clone();
            let second = tree.node(children[1]).unwrap().layout.clone();
            assert!((first.y - SECTION_HEADER_HEIGHT).abs() < 0.5, "first child should start right below the header offset, got y={}", first.y);
            assert!((second.y - (first.y + first.height + theme.gap_standard)).abs() < 0.5, "second child should sit one gap below the first child's own intrinsic height, got y={} first.height={}", second.y, first.height);
            assert!((first.width - 200.0).abs() < 0.5, "children should stretch to the section's full width, got width={}", first.width);
        }
        //#endregion 🔖️FieldSectionGrowSemantics
    }
    // #endregion flex
}

#[cfg(feature = "engine")]
pub mod shaders {
    // #region shaders
    //! 🧊️ WGSL shader sources for the raw wgpu UI renderer.

    pub const UI_SHADER: &str = r#"
struct Globals {
    screen_size: vec2<f32>,
    _pad: vec2<f32>,
}

@group(0) @binding(0) var<uniform> globals: Globals;
@group(0) @binding(1) var glyph_atlas: texture_2d<f32>;
@group(0) @binding(2) var glyph_sampler: sampler;
@group(0) @binding(3) var icon_atlas: texture_2d<f32>;
@group(0) @binding(4) var icon_sampler: sampler;

struct VertexInput {
    @location(0) corner: vec2<f32>,
}

struct InstanceInput {
    @location(1) rect: vec4<f32>,
    @location(2) color: vec4<f32>,
    @location(3) params: vec4<f32>,
    @location(4) uv_rect: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) local: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) params: vec4<f32>,
    @location(4) uv: vec2<f32>,
}

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;
    let pos = instance.rect.xy + vertex.corner * instance.rect.zw;
    let ndc = (pos / globals.screen_size) * 2.0 - vec2<f32>(1.0, 1.0);
    out.clip_position = vec4<f32>(ndc.x, -ndc.y, 0.0, 1.0);
    out.local = vertex.corner * instance.rect.zw;
    out.size = instance.rect.zw;
    out.color = instance.color;
    out.params = instance.params;
    let uv_min = instance.uv_rect.xy;
    let uv_max = instance.uv_rect.zw;
    out.uv = mix(uv_min, uv_max, vertex.corner);
    return out;
}

fn sdf_rounded_rect(p: vec2<f32>, half_size: vec2<f32>, radius: f32) -> f32 {
    let q = abs(p) - half_size + vec2<f32>(radius);
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let kind = i32(in.params.z + 0.5);
    let glyph = textureSample(glyph_atlas, glyph_sampler, in.uv);
    let icon = textureSample(icon_atlas, icon_sampler, in.uv);
    if (kind == 1) {
        let half = in.size * 0.5;
        let p = in.local - half;
        let radius = in.params.x;
        let border = in.params.y;
        let dist = sdf_rounded_rect(p, half, radius);
        let fill_alpha = 1.0 - smoothstep(-1.0, 0.0, dist);
        let border_alpha = 1.0 - smoothstep(border - 1.0, border, abs(dist));
        let alpha = max(fill_alpha * in.color.a, border_alpha * in.params.w);
        return vec4<f32>(in.color.rgb, alpha);
    }
    if (kind == 6) {
        let half = in.size * 0.5;
        let p = in.local - half;
        let radius = in.params.x;
        let border = in.params.y;
        let dist = sdf_rounded_rect(p, half, radius);
        let border_alpha = 1.0 - smoothstep(border - 1.0, border, abs(dist));
        let two_pi = 6.28318530718;
        let duration = 1.6;
        let phase = globals._pad.x / duration;
        var theta = atan2(p.x, -p.y);
        theta = theta - floor(theta / two_pi) * two_pi;
        var spin = phase * two_pi;
        spin = spin - floor(spin / two_pi) * two_pi;
        var sweep = theta - spin;
        sweep = sweep - floor(sweep / two_pi) * two_pi;
        let comet_alpha = sweep / two_pi;
        let ring_alpha = max(comet_alpha, 0.2);
        let pulse = 0.775 - 0.225 * cos(two_pi * phase);
        let alpha = border_alpha * ring_alpha * pulse * in.color.a;
        return vec4<f32>(in.color.rgb, alpha);
    }
    if (kind == 7) {
        let half = in.size * 0.5;
        let p = in.local - half;
        let radius = in.params.x;
        let border = in.params.y;
        let dist = sdf_rounded_rect(p, half, radius);
        let border_alpha = 1.0 - smoothstep(border - 1.0, border, abs(dist));
        let two_pi = 6.28318530718;
        let duration = 3.2;
        let phase = globals._pad.x / duration;
        var theta = atan2(p.x, -p.y);
        theta = theta - floor(theta / two_pi) * two_pi;
        var spin = phase * two_pi;
        spin = spin - floor(spin / two_pi) * two_pi;
        var sweep = theta - spin;
        sweep = sweep - floor(sweep / two_pi) * two_pi;
        let dash = step(fract(sweep / two_pi * 12.0), 0.6);
        let ring_alpha = max(dash, 0.2);
        let pulse = 0.85 - 0.15 * cos(two_pi * phase);
        let alpha = border_alpha * ring_alpha * pulse * in.color.a;
        return vec4<f32>(in.color.rgb, alpha);
    }
    if (kind == 8) {
        let half = in.size * 0.5;
        let p = in.local - half;
        let radius = in.params.x;
        let border = in.params.y;
        let dist = sdf_rounded_rect(p, half, radius);
        let border_alpha = 1.0 - smoothstep(border - 1.0, border, abs(dist));
        let alpha = border_alpha * in.color.a;
        return vec4<f32>(in.color.rgb, alpha);
    }
    if (kind == 9) {
        let half = in.size * 0.5;
        let p = in.local - half;
        let radius = in.params.x;
        let border = in.params.y;
        let dist = sdf_rounded_rect(p, half, radius);
        let border_alpha = 1.0 - smoothstep(border - 1.0, border, abs(dist));
        let two_pi = 6.28318530718;
        let duration = 1.6;
        let phase = globals._pad.x / duration;
        let pulse = 0.5 - 0.5 * cos(two_pi * phase);
        let alpha = border_alpha * pulse * in.color.a;
        return vec4<f32>(in.color.rgb, alpha);
    }
    if (kind == 2) {
        return vec4<f32>(in.color.rgb, glyph.r * in.color.a);
    }
    if (kind == 4 || kind == 5) {
        return vec4<f32>(icon.rgb * in.color.rgb, icon.a * in.color.a);
    }
    if (kind == 3) {
        return in.color;
    }
    return in.color;
}
"#;

    pub const VECTOR_SHADER: &str = r#"
struct Globals {
    screen_size: vec2<f32>,
    _pad: vec2<f32>,
}

@group(0) @binding(0) var<uniform> globals: Globals;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let ndc = (vertex.position / globals.screen_size) * 2.0 - vec2<f32>(1.0, 1.0);
    out.clip_position = vec4<f32>(ndc.x, -ndc.y, 0.0, 1.0);
    out.color = vertex.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
"#;

    pub const WORLD3D_SHADER: &str = r#"
struct Globals {
    view_proj: mat4x4<f32>,
    light_dir: vec4<f32>,
}

@group(0) @binding(0) var<uniform> globals: Globals;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

struct InstanceInput {
    @location(3) model0: vec4<f32>,
    @location(4) model1: vec4<f32>,
    @location(5) model2: vec4<f32>,
    @location(6) model3: vec4<f32>,
    @location(7) color: vec4<f32>,
    @location(8) flags: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) flags: vec4<f32>,
}

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;
    let model = mat4x4<f32>(instance.model0, instance.model1, instance.model2, instance.model3);
    let world_pos = model * vec4<f32>(vertex.position, 1.0);
    out.clip_position = globals.view_proj * world_pos;
    let normal_matrix = mat3x3<f32>(
        model[0].xyz,
        model[1].xyz,
        model[2].xyz
    );
    out.normal = normalize(normal_matrix * vertex.normal);
    out.color = instance.color;
    out.flags = instance.flags;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let n = normalize(in.normal);
    let diffuse = max(dot(n, normalize(globals.light_dir.xyz)), 0.28);
    var color = in.color.rgb * diffuse;
    if (in.flags.x > 0.5) {
        color = mix(color, vec3<f32>(0.35, 0.75, 1.0), 0.65);
    }
    if (in.flags.y > 0.5) {
        color = mix(color, vec3<f32>(1.0, 0.85, 0.35), 0.55);
    }
    return vec4<f32>(color, in.color.a);
}
"#;

    pub const WORLD3D_LINES_SHADER: &str = r#"
struct Globals {
    view_proj: mat4x4<f32>,
    light_dir: vec4<f32>,
}

@group(0) @binding(0) var<uniform> globals: Globals;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = globals.view_proj * vec4<f32>(vertex.position, 1.0);
    out.color = vertex.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
"#;

    pub const WORLD3D_TEXTURED_SHADER: &str = r#"
struct Globals {
    view_proj: mat4x4<f32>,
    light_dir: vec4<f32>,
}

@group(0) @binding(0) var<uniform> globals: Globals;
@group(1) @binding(0) var tex: texture_2d<f32>;
@group(1) @binding(1) var tex_sampler: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
}

struct InstanceInput {
    @location(3) model0: vec4<f32>,
    @location(4) model1: vec4<f32>,
    @location(5) model2: vec4<f32>,
    @location(6) model3: vec4<f32>,
    @location(7) tint: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) tint: vec4<f32>,
}

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;
    let model = mat4x4<f32>(instance.model0, instance.model1, instance.model2, instance.model3);
    let world_pos = model * vec4<f32>(vertex.position, 1.0);
    out.clip_position = globals.view_proj * world_pos;
    out.uv = vertex.uv;
    out.tint = instance.tint;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let sampled = textureSample(tex, tex_sampler, in.uv);
    return vec4<f32>(sampled.rgb * in.tint.rgb, sampled.a * in.tint.a);
}
"#;

    pub const BLUR_DOWNSAMPLE_SHADER: &str = r#"
struct BlurGlobals {
    src_mip: f32,
    _pad: vec3<f32>,
}

@group(0) @binding(0) var<uniform> blur_globals: BlurGlobals;
@group(0) @binding(1) var src_tex: texture_2d<f32>;
@group(0) @binding(2) var src_samp: sampler;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0)
    );
    var uvs = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 1.0), vec2<f32>(1.0, 1.0), vec2<f32>(0.0, 0.0),
        vec2<f32>(0.0, 0.0), vec2<f32>(1.0, 1.0), vec2<f32>(1.0, 0.0)
    );
    var out: VertexOutput;
    let pos = positions[vid];
    out.clip_position = vec4<f32>(pos, 0.0, 1.0);
    out.uv = uvs[vid];
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let mip = u32(blur_globals.src_mip);
    let dim = vec2<f32>(textureDimensions(src_tex, mip));
    let texel = vec2<f32>(1.0) / dim;
    let uv = in.uv;
    let src_mip = blur_globals.src_mip;
    var c = textureSampleLevel(src_tex, src_samp, uv, src_mip) * 4.0;
    c += textureSampleLevel(src_tex, src_samp, uv + vec2<f32>(-texel.x, 0.0), src_mip);
    c += textureSampleLevel(src_tex, src_samp, uv + vec2<f32>(texel.x, 0.0), src_mip);
    c += textureSampleLevel(src_tex, src_samp, uv + vec2<f32>(0.0, -texel.y), src_mip);
    c += textureSampleLevel(src_tex, src_samp, uv + vec2<f32>(0.0, texel.y), src_mip);
    return c / 8.0;
}
"#;

    pub const SCENE_BLIT_SHADER: &str = r#"
@group(0) @binding(0) var scene_tex: texture_2d<f32>;
@group(0) @binding(1) var scene_samp: sampler;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0)
    );
    var uvs = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 1.0), vec2<f32>(1.0, 1.0), vec2<f32>(0.0, 0.0),
        vec2<f32>(0.0, 0.0), vec2<f32>(1.0, 1.0), vec2<f32>(1.0, 0.0)
    );
    var out: VertexOutput;
    let pos = positions[vid];
    out.clip_position = vec4<f32>(pos, 0.0, 1.0);
    out.uv = uvs[vid];
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSampleLevel(scene_tex, scene_samp, in.uv, 0.0);
}
"#;

    pub const GLASS_SHADER: &str = r#"
struct Globals {
    screen_size: vec2<f32>,
    _pad: vec2<f32>,
}

@group(0) @binding(0) var<uniform> globals: Globals;
@group(1) @binding(0) var scene_tex: texture_2d<f32>;
@group(1) @binding(1) var scene_samp: sampler;

struct VertexInput {
    @location(0) corner: vec2<f32>,
}

struct GlassInstanceInput {
    @location(1) rect: vec4<f32>,
    @location(2) tint: vec4<f32>,
    @location(3) params: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) local: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) tint: vec4<f32>,
    @location(3) params: vec4<f32>,
    @location(4) scene_uv: vec2<f32>,
}

@vertex
fn vs_main(vertex: VertexInput, instance: GlassInstanceInput) -> VertexOutput {
    var out: VertexOutput;
    let pos = instance.rect.xy + vertex.corner * instance.rect.zw;
    let ndc = (pos / globals.screen_size) * 2.0 - vec2<f32>(1.0, 1.0);
    out.clip_position = vec4<f32>(ndc.x, -ndc.y, 0.0, 1.0);
    out.local = vertex.corner * instance.rect.zw;
    out.size = instance.rect.zw;
    out.tint = instance.tint;
    out.params = instance.params;
    out.scene_uv = pos / globals.screen_size;
    return out;
}

fn sdf_rounded_rect(p: vec2<f32>, half_size: vec2<f32>, radius: f32) -> f32 {
    let q = abs(p) - half_size + vec2<f32>(radius);
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let half = in.size * 0.5;
    let p = in.local - half;
    let radius = in.params.x;
    let dist = sdf_rounded_rect(p, half, radius);
    let fill_alpha = 1.0 - smoothstep(-1.0, 0.0, dist);
    if (fill_alpha <= 0.001) {
        discard;
    }
    let mip = in.params.z;
    let saturate = in.params.w;
    let tint_alpha = in.params.y;
    let blurred = textureSampleLevel(scene_tex, scene_samp, in.scene_uv, mip);
    let luma = dot(blurred.rgb, vec3<f32>(0.2126, 0.7152, 0.0722));
    let saturated = mix(vec3<f32>(luma), blurred.rgb, saturate);
    let rgb = mix(saturated, in.tint.rgb, tint_alpha);
    return vec4<f32>(rgb, fill_alpha);
}
"#;
    // #endregion shaders
}

#[cfg(feature = "engine")]
pub mod text {
    // #region text
    //! 🖋️ Glyph atlas — `parley` (shaping/font-fallback resolution) + `fontique` (font collection
    //! and generic/emoji fallback registry) + `swash` (rasterization), packed into two atlas pages
    //! (alpha-only `pixels` for regular glyphs, RGBA `color_pixels` for COLR/bitmap color-emoji
    //! glyphs). A built-in 8×16 ASCII bitmap mode is kept as the deterministic, dependency-free
    //! fallback used by `FontAtlas::builtin()` (relied on by many call sites across the crate for
    //! fast/fixed-metric test setup) and by any single codepoint no registered font can shape at all.
    //! See `.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w1b-text-stack.md` for the full
    //! architecture writeup, including the deliberate measurement/paint-consistency tradeoff that
    //! keeps `measure_text`/`ensure_glyph` per-codepoint rather than switching to whole-string
    //! `parley::Layout` metrics.

    use std::borrow::Cow;
    use std::collections::HashMap;
    use std::sync::Arc;

    use parley::fontique::{Blob, Collection, CollectionOptions, FamilyId, FontInfoOverride, GenericFamily, SourceCache};
    use parley::{FontContext, FontStack, LayoutContext, PositionedLayoutItem, StyleProperty};
    use swash::scale::image::Content as SwashContent;
    use swash::scale::{Render, ScaleContext, Source, StrikeWith};
    use swash::zeno::Format as SwashFormat;
    use swash::FontRef as SwashFontRef;

    pub struct GlyphEntry {
        pub atlas_x: u32,
        pub atlas_y: u32,
        pub width: u32,
        pub height: u32,
        pub advance: f32,
        pub bearing_x: f32,
        pub bearing_y: f32,
        /// 🎨️ True when this glyph lives in the RGBA `color_pixels` page (COLR/bitmap color emoji)
        /// rather than the alpha-only `pixels` page. Paint call sites in `widgets`/`paint` (outside
        /// this region) still only sample the alpha page — see the report's wiring-request section.
        pub is_color: bool,
    }

    /// 🔤️ Fixed family names every registered font is forced under via `FontInfoOverride`, so
    /// multi-file families (Noto Emoji's 12 codepoint-range buckets) merge into one fontique family
    /// regardless of what each file's own `name` table declares.
    const FAMILY_SANS: &str = "Anta";
    const FAMILY_SERIF: &str = "Kelly Slab";
    const FAMILY_MONO: &str = "Share Tech Mono";
    const FAMILY_EMOJI: &str = "Noto Emoji";

    static ANTA_LATIN: &[u8] = include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🔤️fonts/🔤️anta/🔤️latin.ttf");
    static KELLY_SLAB_LATIN: &[u8] = include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🔤️fonts/🔤️kelly-slab/🔤️latin.ttf");
    static SHARE_TECH_MONO_LATIN: &[u8] = include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🔤️fonts/🔤️share-tech-mono/🔤️latin.ttf");
    static NOTO_EMOJI_BUCKETS: [&[u8]; 12] = [
        include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🔤️fonts/😀️noto-emoji/🔤️0-400.ttf"),
        include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🔤️fonts/😀️noto-emoji/🔤️1-400.ttf"),
        include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🔤️fonts/😀️noto-emoji/🔤️2-400.ttf"),
        include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🔤️fonts/😀️noto-emoji/🔤️3-400.ttf"),
        include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🔤️fonts/😀️noto-emoji/🔤️4-400.ttf"),
        include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🔤️fonts/😀️noto-emoji/🔤️5-400.ttf"),
        include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🔤️fonts/😀️noto-emoji/🔤️6-400.ttf"),
        include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🔤️fonts/😀️noto-emoji/🔤️7-400.ttf"),
        include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🔤️fonts/😀️noto-emoji/🔤️8-400.ttf"),
        include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🔤️fonts/😀️noto-emoji/🔤️9-400.ttf"),
        include_bytes!("../../../../🖼️assets/⚡️implementations/🟦️typescript/🔤️fonts/😀️noto-emoji/🔤️10-400.ttf"),
        include_bytes!("../../../../🖼️assets/⚡️implementations/🟦️typescript/🔤️fonts/😀️noto-emoji/🔤️11-400.ttf"),
    ];

    const BITMAP_GLYPH_W: u32 = 8;
    const BITMAP_GLYPH_H: u32 = 16;

    static BITMAP_FONT: [[u8; 8]; 95] = [
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        [0x18, 0x3C, 0x3C, 0x18, 0x18, 0x00, 0x18, 0x00],
        [0x36, 0x36, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00],
        [0x36, 0x36, 0x7F, 0x36, 0x7F, 0x36, 0x36, 0x00],
        [0x0C, 0x3E, 0x60, 0x3C, 0x06, 0x7C, 0x18, 0x00],
        [0x00, 0x63, 0x66, 0x0C, 0x18, 0x33, 0x63, 0x00],
        [0x1C, 0x36, 0x1C, 0x6E, 0x3B, 0x33, 0x6E, 0x00],
        [0x06, 0x06, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00],
        [0x18, 0x30, 0x60, 0x60, 0x60, 0x30, 0x18, 0x00],
        [0x06, 0x0C, 0x18, 0x18, 0x18, 0x0C, 0x06, 0x00],
        [0x00, 0x66, 0x3C, 0xFF, 0x3C, 0x66, 0x00, 0x00],
        [0x00, 0x18, 0x18, 0x7E, 0x18, 0x18, 0x00, 0x00],
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x30],
        [0x00, 0x00, 0x00, 0x7E, 0x00, 0x00, 0x00, 0x00],
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00],
        [0x06, 0x0C, 0x18, 0x30, 0x60, 0xC0, 0x80, 0x00],
        [0x3C, 0x66, 0x6E, 0x76, 0x66, 0x66, 0x3C, 0x00],
        [0x18, 0x38, 0x18, 0x18, 0x18, 0x18, 0x7E, 0x00],
        [0x3C, 0x66, 0x06, 0x1C, 0x30, 0x60, 0x7E, 0x00],
        [0x3C, 0x66, 0x06, 0x1C, 0x06, 0x66, 0x3C, 0x00],
        [0x0C, 0x1C, 0x3C, 0x6C, 0x7E, 0x0C, 0x0C, 0x00],
        [0x7E, 0x60, 0x7C, 0x06, 0x06, 0x66, 0x3C, 0x00],
        [0x1C, 0x30, 0x60, 0x7C, 0x66, 0x66, 0x3C, 0x00],
        [0x7E, 0x06, 0x0C, 0x18, 0x30, 0x30, 0x30, 0x00],
        [0x3C, 0x66, 0x66, 0x3C, 0x66, 0x66, 0x3C, 0x00],
        [0x3C, 0x66, 0x66, 0x3E, 0x06, 0x0C, 0x38, 0x00],
        [0x00, 0x18, 0x18, 0x00, 0x00, 0x18, 0x18, 0x00],
        [0x00, 0x18, 0x18, 0x00, 0x00, 0x18, 0x18, 0x30],
        [0x0C, 0x18, 0x30, 0x60, 0x30, 0x18, 0x0C, 0x00],
        [0x00, 0x00, 0x7E, 0x00, 0x7E, 0x00, 0x00, 0x00],
        [0x30, 0x18, 0x0C, 0x06, 0x0C, 0x18, 0x30, 0x00],
        [0x3C, 0x66, 0x06, 0x0C, 0x18, 0x00, 0x18, 0x00],
        [0x3C, 0x66, 0x6E, 0x6A, 0x6E, 0x60, 0x3C, 0x00],
        [0x3C, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x00],
        [0x7C, 0x66, 0x66, 0x7C, 0x66, 0x66, 0x7C, 0x00],
        [0x3C, 0x66, 0x60, 0x60, 0x60, 0x66, 0x3C, 0x00],
        [0x78, 0x6C, 0x66, 0x66, 0x66, 0x6C, 0x78, 0x00],
        [0x7E, 0x60, 0x60, 0x7C, 0x60, 0x60, 0x7E, 0x00],
        [0x7E, 0x60, 0x60, 0x7C, 0x60, 0x60, 0x60, 0x00],
        [0x3C, 0x66, 0x60, 0x6E, 0x66, 0x66, 0x3C, 0x00],
        [0x66, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x00],
        [0x3C, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00],
        [0x1E, 0x0C, 0x0C, 0x0C, 0x0C, 0x6C, 0x38, 0x00],
        [0x66, 0x6C, 0x78, 0x70, 0x78, 0x6C, 0x66, 0x00],
        [0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x7E, 0x00],
        [0x63, 0x77, 0x7F, 0x6B, 0x63, 0x63, 0x63, 0x00],
        [0x66, 0x76, 0x7E, 0x7E, 0x6E, 0x66, 0x66, 0x00],
        [0x3C, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00],
        [0x7C, 0x66, 0x66, 0x7C, 0x60, 0x60, 0x60, 0x00],
        [0x3C, 0x66, 0x66, 0x66, 0x6E, 0x6C, 0x3A, 0x00],
        [0x7C, 0x66, 0x66, 0x7C, 0x6C, 0x66, 0x66, 0x00],
        [0x3C, 0x66, 0x60, 0x3C, 0x06, 0x66, 0x3C, 0x00],
        [0x7E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00],
        [0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00],
        [0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x18, 0x00],
        [0x63, 0x63, 0x63, 0x6B, 0x7F, 0x77, 0x63, 0x00],
        [0x66, 0x66, 0x3C, 0x18, 0x3C, 0x66, 0x66, 0x00],
        [0x66, 0x66, 0x66, 0x3C, 0x18, 0x18, 0x18, 0x00],
        [0x7E, 0x06, 0x0C, 0x18, 0x30, 0x60, 0x7E, 0x00],
        [0x3C, 0x30, 0x30, 0x30, 0x30, 0x30, 0x3C, 0x00],
        [0xC0, 0x60, 0x30, 0x18, 0x0C, 0x06, 0x02, 0x00],
        [0x3C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x3C, 0x00],
        [0x10, 0x38, 0x6C, 0x00, 0x00, 0x00, 0x00, 0x00],
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF],
        [0x30, 0x18, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00],
        [0x00, 0x00, 0x3C, 0x06, 0x3E, 0x66, 0x3E, 0x00],
        [0x60, 0x60, 0x7C, 0x66, 0x66, 0x66, 0x7C, 0x00],
        [0x00, 0x00, 0x3C, 0x66, 0x60, 0x66, 0x3C, 0x00],
        [0x06, 0x06, 0x3E, 0x66, 0x66, 0x66, 0x3E, 0x00],
        [0x00, 0x00, 0x3C, 0x66, 0x7E, 0x60, 0x3C, 0x00],
        [0x1C, 0x30, 0x7C, 0x30, 0x30, 0x30, 0x30, 0x00],
        [0x00, 0x00, 0x3E, 0x66, 0x66, 0x3E, 0x06, 0x3C],
        [0x60, 0x60, 0x7C, 0x66, 0x66, 0x66, 0x66, 0x00],
        [0x18, 0x00, 0x38, 0x18, 0x18, 0x18, 0x3C, 0x00],
        [0x0C, 0x00, 0x1C, 0x0C, 0x0C, 0x6C, 0x6C, 0x38],
        [0x60, 0x60, 0x66, 0x6C, 0x78, 0x6C, 0x66, 0x00],
        [0x38, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00],
        [0x00, 0x00, 0x36, 0x7F, 0x6B, 0x6B, 0x63, 0x00],
        [0x00, 0x00, 0x7C, 0x66, 0x66, 0x66, 0x66, 0x00],
        [0x00, 0x00, 0x3C, 0x66, 0x66, 0x66, 0x3C, 0x00],
        [0x00, 0x00, 0x7C, 0x66, 0x66, 0x7C, 0x60, 0x60],
        [0x00, 0x00, 0x3E, 0x66, 0x66, 0x3E, 0x06, 0x06],
        [0x00, 0x00, 0x7C, 0x66, 0x60, 0x60, 0x60, 0x00],
        [0x00, 0x00, 0x3E, 0x60, 0x3C, 0x06, 0x7C, 0x00],
        [0x30, 0x30, 0x7C, 0x30, 0x30, 0x30, 0x1C, 0x00],
        [0x00, 0x00, 0x66, 0x66, 0x66, 0x66, 0x3E, 0x00],
        [0x00, 0x00, 0x66, 0x66, 0x66, 0x3C, 0x18, 0x00],
        [0x00, 0x00, 0x63, 0x6B, 0x6B, 0x7F, 0x36, 0x00],
        [0x00, 0x00, 0x66, 0x3C, 0x18, 0x3C, 0x66, 0x00],
        [0x00, 0x00, 0x66, 0x66, 0x66, 0x3E, 0x06, 0x3C],
        [0x00, 0x00, 0x7E, 0x0C, 0x18, 0x30, 0x7E, 0x00],
        [0x0E, 0x18, 0x18, 0x70, 0x18, 0x18, 0x0E, 0x00],
        [0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00],
        [0x70, 0x18, 0x18, 0x0E, 0x18, 0x18, 0x70, 0x00],
        [0x31, 0x6B, 0x46, 0x00, 0x00, 0x00, 0x00, 0x00],
    ];

    /// 🧩️ How a `FontAtlas` resolves and rasterizes glyphs. `Bitmap` is the deterministic,
    /// dependency-free 8×16 ASCII fallback used by `FontAtlas::builtin()`; `Shaped` runs the full
    /// parley/fontique/swash pipeline against a registered `fontique::Collection`.
    enum AtlasMode {
        Bitmap,
        Shaped,
    }

    /// 📦️ One rasterized glyph ready to be packed into an atlas page, produced by either
    /// `rasterize_bitmap_glyph` or `rasterize_shaped_glyph`.
    struct RasterizedGlyph {
        bitmap: Vec<u8>,
        width: u32,
        height: u32,
        bearing_x: f32,
        bearing_y: f32,
        advance: f32,
        is_color: bool,
    }

    /// 🧭️ The font and glyph a single codepoint resolved to after running it through parley's
    /// shaping/itemization (which performs the family + generic-emoji-fallback font selection).
    struct ResolvedGlyph {
        glyph_id: swash::GlyphId,
        advance: f32,
        font_data: Blob<u8>,
        font_index: u32,
    }

    pub struct FontAtlas {
        pub width: u32,
        pub height: u32,
        pub pixels: Vec<u8>,
        /// 🌈️ RGBA emoji atlas page (2048×2048×4 in `Shaped` mode, empty in `Bitmap` mode since that
        /// mode never produces color glyphs).
        pub color_width: u32,
        pub color_height: u32,
        pub color_pixels: Vec<u8>,
        mode: AtlasMode,
        font_cx: FontContext,
        layout_cx: LayoutContext<[u8; 4]>,
        scale_cx: ScaleContext,
        glyphs: HashMap<(char, u32), GlyphEntry>,
        cursor_x: u32,
        cursor_y: u32,
        row_height: u32,
        color_cursor_x: u32,
        color_cursor_y: u32,
        color_row_height: u32,
        dirty: bool,
        color_dirty: bool,
    }

    /// 🗂️ A `fontique::Collection` with no system-font scanning (deterministic, self-contained —
    /// every family this atlas ever shapes with is one of the four `FAMILY_*` names registered from
    /// `include_bytes!`-embedded assets).
    fn empty_font_context() -> FontContext {
        FontContext { collection: Collection::new(CollectionOptions { shared: false, system_fonts: false }), source_cache: SourceCache::default() }
    }

    /// 📥️ Registers `bytes` into `collection` under a forced `family` name (ignoring whatever family
    /// name the font file's own `name` table declares), so multi-file families like Noto Emoji's 12
    /// codepoint-range buckets always merge into one fontique family.
    fn register_family(collection: &mut Collection, bytes: &[u8], family: &'static str) -> Option<FamilyId> {
        let over = FontInfoOverride { family_name: Some(family), ..Default::default() };
        collection.register_fonts(Blob::new(Arc::new(bytes.to_vec())), Some(over)).into_iter().next().map(|(id, _)| id)
    }

    impl FontAtlas {
        pub fn builtin() -> Self {
            Self {
                width: 2048,
                height: 2048,
                pixels: vec![0; 2048 * 2048],
                color_width: 0,
                color_height: 0,
                color_pixels: Vec::new(),
                mode: AtlasMode::Bitmap,
                font_cx: empty_font_context(),
                layout_cx: LayoutContext::new(),
                scale_cx: ScaleContext::new(),
                glyphs: HashMap::new(),
                cursor_x: 1,
                cursor_y: 1,
                row_height: 0,
                color_cursor_x: 1,
                color_cursor_y: 1,
                color_row_height: 0,
                dirty: false,
                color_dirty: false,
            }
        }

        pub fn take_dirty(&mut self) -> bool {
            let dirty = self.dirty;
            self.dirty = false;
            dirty
        }

        /// 🌈️ Same contract as `take_dirty` for the RGBA emoji atlas page (`color_pixels`). Wiring
        /// this into an actual GPU upload is a `gpu`-region call site left as a wiring request — see
        /// `GpuContext::upload_font_atlas`/`upload_emoji_atlas` in the report.
        pub fn take_color_dirty(&mut self) -> bool {
            let dirty = self.color_dirty;
            self.color_dirty = false;
            dirty
        }

        /// 🏗️ Builds a fully self-contained `Shaped`-mode atlas: registers the embedded Anta, Kelly
        /// Slab and Share Tech Mono families plus all 12 Noto Emoji fallback buckets, then wires
        /// `GenericFamily::Emoji`/`SansSerif`/`Serif`/`Monospace` to them. `primary_override`, when it
        /// parses as a real font, replaces the embedded Anta bytes as the `FAMILY_SANS` source (this
        /// is how `from_bytes` keeps honoring whatever bytes the host fetched) — falling back to the
        /// embedded copy keeps this constructor infallible even for garbage/empty-ish input.
        fn shaped(primary_override: Option<&[u8]>) -> Self {
            let mut collection = Collection::new(CollectionOptions { shared: false, system_fonts: false });
            let sans_bytes: &[u8] = match primary_override {
                Some(bytes) if SwashFontRef::from_index(bytes, 0).is_some() => bytes,
                _ => ANTA_LATIN,
            };
            let sans_id = register_family(&mut collection, sans_bytes, FAMILY_SANS).or_else(|| register_family(&mut collection, ANTA_LATIN, FAMILY_SANS)).expect("embedded Anta font asset must register");
            let serif_id = register_family(&mut collection, KELLY_SLAB_LATIN, FAMILY_SERIF).expect("embedded Kelly Slab font asset must register");
            let mono_id = register_family(&mut collection, SHARE_TECH_MONO_LATIN, FAMILY_MONO).expect("embedded Share Tech Mono font asset must register");
            let mut emoji_id: Option<FamilyId> = None;
            for bucket in NOTO_EMOJI_BUCKETS {
                if let Some(id) = register_family(&mut collection, bucket, FAMILY_EMOJI) {
                    emoji_id.get_or_insert(id);
                }
            }
            let emoji_id = emoji_id.expect("embedded Noto Emoji font assets must register");
            collection.set_generic_families(GenericFamily::Emoji, std::iter::once(emoji_id));
            collection.set_generic_families(GenericFamily::SansSerif, std::iter::once(sans_id));
            collection.set_generic_families(GenericFamily::Serif, std::iter::once(serif_id));
            collection.set_generic_families(GenericFamily::Monospace, std::iter::once(mono_id));
            Self {
                width: 2048,
                height: 2048,
                pixels: vec![0; 2048 * 2048],
                color_width: 2048,
                color_height: 2048,
                color_pixels: vec![0; 2048 * 2048 * 4],
                mode: AtlasMode::Shaped,
                font_cx: FontContext { collection, source_cache: SourceCache::default() },
                layout_cx: LayoutContext::new(),
                scale_cx: ScaleContext::new(),
                glyphs: HashMap::new(),
                cursor_x: 1,
                cursor_y: 1,
                row_height: 0,
                color_cursor_x: 1,
                color_cursor_y: 1,
                color_row_height: 0,
                dirty: false,
                color_dirty: false,
            }
        }

        /// 🔡️ `bytes` empty ⇒ deterministic `builtin()` bitmap mode (unchanged contract). Any
        /// non-empty input — including bytes that fail to parse as a font — now resolves into full
        /// `Shaped` mode (registering the embedded Anta/Kelly Slab/Share Tech Mono/Noto Emoji
        /// families regardless), which is a strict improvement over the old fontdue-only pipeline's
        /// "garbage bytes ⇒ crude ASCII boxes" behavior.
        pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
            if bytes.is_empty() {
                return Ok(Self::builtin());
            }
            Ok(Self::shaped(Some(bytes)))
        }

        /// 🔑️ Quantizes a float px size to the glyph-cache's integer key component, so float jitter
        /// (e.g. 15.999999 vs 16.0) doesn't fragment the cache into near-duplicate entries.
        fn quantize_size(size_px: f32) -> u32 {
            size_px.round().max(1.0) as u32
        }

        /// 🔍️ Fetches (rasterizing on first use) the glyph for `ch` at `size_px`, keyed by
        /// `(char, size_px)` so the same character rasterized at two different sizes never returns the
        /// wrong bitmap (the pre-fix bug: a `char`-only key meant later sizes reused the first size's
        /// rasterization, blurring text at any size other than whichever was cached first).
        pub fn ensure_glyph(&mut self, ch: char, size_px: f32) -> &GlyphEntry {
            let key = (ch, Self::quantize_size(size_px));
            if !self.glyphs.contains_key(&key) {
                self.rasterize_glyph(key);
            }
            self.glyphs.get(&key).expect("glyph inserted")
        }

        fn rasterize_glyph(&mut self, key: (char, u32)) {
            let (ch, size_px) = key;
            let glyph = match self.mode {
                AtlasMode::Bitmap => self.rasterize_bitmap_glyph(ch),
                AtlasMode::Shaped => self.rasterize_shaped_glyph(ch, size_px as f32),
            };
            self.pack_glyph(key, glyph);
        }

        /// 🧵️ Resolves `ch` to a font + glyph id by running a single-codepoint `parley::Layout`. This
        /// is what performs family resolution and (via parley's built-in emoji-cluster detection)
        /// automatic fallback into the registered `GenericFamily::Emoji` family. Returns `None` when
        /// no registered font (including the emoji fallback) could shape the codepoint at all.
        fn shape_single_char(&mut self, ch: char, size_px: f32) -> Option<ResolvedGlyph> {
            let text = ch.to_string();
            let mut builder = self.layout_cx.ranged_builder(&mut self.font_cx, &text, 1.0, true);
            builder.push_default(StyleProperty::FontStack(FontStack::Source(Cow::Borrowed(FAMILY_SANS))));
            builder.push_default(StyleProperty::FontSize(size_px));
            let mut layout: parley::Layout<[u8; 4]> = builder.build(&text);
            layout.break_all_lines(None);
            for line in layout.lines() {
                for item in line.items() {
                    let PositionedLayoutItem::GlyphRun(run) = item else { continue };
                    let Some(glyph) = run.glyphs().next() else { continue };
                    let font = run.run().font();
                    return Some(ResolvedGlyph { glyph_id: glyph.id, advance: glyph.advance, font_data: font.data.clone(), font_index: font.index });
                }
            }
            None
        }

        /// 🖌️ Rasterizes a resolved glyph via swash, preferring color bitmap/outline sources (COLR,
        /// embedded color bitmaps — what carries Noto Emoji's color) over the plain scalable outline,
        /// so any glyph the resolved font can render in color comes back as `Content::Color` RGBA and
        /// everything else comes back as an 8-bit alpha mask.
        fn render_resolved(&mut self, resolved: &ResolvedGlyph, size_px: f32) -> Option<RasterizedGlyph> {
            let data = resolved.font_data.data();
            let font_ref = SwashFontRef::from_index(data, resolved.font_index as usize)?;
            let mut scaler = self.scale_cx.builder(font_ref).size(size_px).hint(true).build();
            let image = Render::new(&[Source::ColorBitmap(StrikeWith::BestFit), Source::ColorOutline(0), Source::Outline]).format(SwashFormat::Alpha).render(&mut scaler, resolved.glyph_id)?;
            let is_color = matches!(image.content, SwashContent::Color);
            Some(RasterizedGlyph {
                bitmap: image.data,
                width: image.placement.width,
                height: image.placement.height,
                bearing_x: image.placement.left as f32,
                bearing_y: (image.placement.top - image.placement.height as i32) as f32,
                advance: resolved.advance,
                is_color,
            })
        }

        /// 🪶️ `shape_single_char` + `render_resolved`, falling back to the plain ASCII bitmap glyph
        /// (see `rasterize_bitmap_glyph`) when no registered font — including the emoji fallback —
        /// can shape `ch` at all (e.g. scripts none of Anta/Kelly Slab/Share Tech Mono/Noto Emoji
        /// cover, such as CJK or Arabic; a pre-existing limitation this atlas doesn't newly regress).
        fn rasterize_shaped_glyph(&mut self, ch: char, size_px: f32) -> RasterizedGlyph {
            let Some(resolved) = self.shape_single_char(ch, size_px) else {
                return self.rasterize_bitmap_glyph(ch);
            };
            if let Some(glyph) = self.render_resolved(&resolved, size_px) {
                return glyph;
            }
            RasterizedGlyph { bitmap: Vec::new(), width: 0, height: 0, bearing_x: 0.0, bearing_y: 0.0, advance: resolved.advance, is_color: false }
        }

        fn rasterize_bitmap_glyph(&self, ch: char) -> RasterizedGlyph {
            let index = ch as u32;
            let glyph_index = if (32..127).contains(&index) { (index - 32) as usize } else { 0 };
            let pattern = &BITMAP_FONT[glyph_index.min(BITMAP_FONT.len() - 1)];
            let mut bitmap = vec![0u8; (BITMAP_GLYPH_W * BITMAP_GLYPH_H) as usize];
            for (row, row_bits) in pattern.iter().enumerate() {
                for col in 0..BITMAP_GLYPH_W {
                    if (row_bits >> (7 - col)) & 1 == 1 {
                        bitmap[row * BITMAP_GLYPH_W as usize + col as usize] = 255;
                    }
                }
            }
            RasterizedGlyph { bitmap, width: BITMAP_GLYPH_W, height: BITMAP_GLYPH_H, bearing_x: 0.0, bearing_y: 0.0, advance: BITMAP_GLYPH_W as f32 + 2.0, is_color: false }
        }

        /// 📐️ Bin-packs one rasterized glyph into the alpha (`pixels`) or color (`color_pixels`)
        /// atlas page, per `RasterizedGlyph::is_color`, and records the resulting `GlyphEntry`.
        fn pack_glyph(&mut self, key: (char, u32), glyph: RasterizedGlyph) {
            let RasterizedGlyph { bitmap, width, height, bearing_x, bearing_y, advance, is_color } = glyph;
            let (atlas_x, atlas_y) = if is_color {
                if self.color_cursor_x + width + 2 >= self.color_width {
                    self.color_cursor_x = 1;
                    self.color_cursor_y += self.color_row_height + 2;
                    self.color_row_height = 0;
                }
                let x = self.color_cursor_x;
                let y = self.color_cursor_y;
                for row in 0..height {
                    let dst = (((y + row) * self.color_width + x) * 4) as usize;
                    let src = (row * width * 4) as usize;
                    if !bitmap.is_empty() && width > 0 {
                        self.color_pixels[dst..dst + (width * 4) as usize].copy_from_slice(&bitmap[src..src + (width * 4) as usize]);
                    }
                }
                self.color_cursor_x += width + 2;
                self.color_row_height = self.color_row_height.max(height);
                self.color_dirty = true;
                (x, y)
            } else {
                if self.cursor_x + width + 2 >= self.width {
                    self.cursor_x = 1;
                    self.cursor_y += self.row_height + 2;
                    self.row_height = 0;
                }
                let x = self.cursor_x;
                let y = self.cursor_y;
                for row in 0..height {
                    let dst = ((y + row) * self.width + x) as usize;
                    let src = (row * width) as usize;
                    if !bitmap.is_empty() && width > 0 {
                        self.pixels[dst..dst + width as usize].copy_from_slice(&bitmap[src..src + width as usize]);
                    }
                }
                self.cursor_x += width + 2;
                self.row_height = self.row_height.max(height);
                self.dirty = true;
                (x, y)
            };
            self.glyphs.insert(key, GlyphEntry { atlas_x, atlas_y, width, height, advance, bearing_x, bearing_y, is_color });
        }

        pub fn measure_text(&mut self, text: &str, size: f32) -> (f32, f32) {
            let mut width = 0.0f32;
            let mut max_height = 0.0f32;
            for ch in text.chars() {
                let glyph = self.ensure_glyph(ch, size);
                width += glyph.advance;
                max_height = max_height.max(glyph.height as f32 + glyph.bearing_y);
            }
            (width, max_height.max(size))
        }

        pub fn measure_text_wrapped(&mut self, text: &str, max_width: f32, size: f32) -> (f32, f32) {
            let mut lines = Vec::new();
            let mut current = String::new();
            for word in text.split_whitespace() {
                let trial = if current.is_empty() { word.to_string() } else { format!("{current} {word}") };
                let (w, _) = self.measure_text(&trial, size);
                if w > max_width && !current.is_empty() {
                    lines.push(current);
                    current = word.to_string();
                } else {
                    current = trial;
                }
            }
            if !current.is_empty() {
                lines.push(current);
            }
            let line_h = size * 1.35;
            let height = lines.len().max(1) as f32 * line_h;
            let width = lines.iter().map(|line| self.measure_text(line, size).0).fold(0.0f32, f32::max).min(max_width);
            (width, height)
        }
    }

    pub async fn fetch_font_bytes(url: &str) -> Result<Vec<u8>, String> {
        #[cfg(target_arch = "wasm32")]
        {
            use js_sys::Uint8Array;
            use wasm_bindgen::JsCast;
            use wasm_bindgen_futures::JsFuture;
            use web_sys::{Request, RequestInit, RequestMode, Response};

            let opts = RequestInit::new();
            opts.set_method("GET");
            opts.set_mode(RequestMode::Cors);
            let request = Request::new_with_str_and_init(url, &opts).map_err(|_| "request failed")?;
            let window = web_sys::window().ok_or("no window")?;
            let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.map_err(|_| "fetch failed")?;
            let resp: Response = resp_value.dyn_into().map_err(|_| "response cast failed")?;
            if !resp.ok() {
                return Ok(Vec::new());
            }
            let buffer = JsFuture::from(resp.array_buffer().map_err(|_| "array_buffer failed")?).await.map_err(|_| "buffer failed")?;
            let array = Uint8Array::new(&buffer);
            let mut bytes = vec![0u8; array.length() as usize];
            array.copy_to(&mut bytes);
            Ok(bytes)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = url;
            Ok(Vec::new())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::FontAtlas;

        #[test]
        fn from_bytes_falls_back_to_embedded_fonts_for_unparseable_input() {
            assert!(FontAtlas::from_bytes(&[]).is_ok());
            let woff2 = b"wOF2\x00\x01\x00\x00";
            let mut atlas = FontAtlas::from_bytes(woff2).expect("unparseable bytes still register embedded fonts");
            let glyph = atlas.ensure_glyph('A', 16.0);
            assert!(glyph.width > 0);
            assert!(!glyph.is_color, "'A' must rasterize as a regular alpha glyph, not a color one");
        }

        #[test]
        fn same_char_at_different_sizes_does_not_collide_in_the_glyph_cache() {
            let mut atlas = FontAtlas::builtin();
            atlas.ensure_glyph('A', 16.0);
            assert_eq!(atlas.glyphs.len(), 1);
            atlas.ensure_glyph('A', 32.0);
            assert_eq!(atlas.glyphs.len(), 2, "a second size for the same char must add a new cache entry, not collide");
            atlas.ensure_glyph('A', 16.0);
            assert_eq!(atlas.glyphs.len(), 2, "re-requesting an already-cached (char, size) must not insert again");
        }

        #[test]
        fn shaped_mode_resolves_real_font_metrics_that_differ_from_the_bitmap_fallback() {
            let mut atlas = FontAtlas::from_bytes(super::ANTA_LATIN).expect("embedded Anta bytes must load");
            let glyph = atlas.ensure_glyph('W', 24.0);
            assert!(glyph.width > 0 && glyph.height > 0);
            assert!(!glyph.is_color);
        }

        /// 🔤️ The bundled `ui/asset/font/noto-emoji/*.ttf` buckets are the monochrome "Noto Emoji"
        /// family (`glyf` outlines only — verified no `COLR`/`CPAL`/`CBDT`/`CBLC`/`sbix` table is
        /// present), not "Noto Color Emoji", so real emoji codepoints correctly resolve through the
        /// `GenericFamily::Emoji` fallback and rasterize successfully, but land on the alpha page
        /// like any other outline glyph. `packing_a_synthetic_color_glyph_lands_on_the_rgba_page_and_marks_it_dirty`
        /// below exercises the RGBA color-page path directly, since these assets never trigger it.
        #[test]
        fn emoji_codepoints_resolve_through_the_noto_emoji_fallback_family() {
            let mut atlas = FontAtlas::from_bytes(super::ANTA_LATIN).expect("embedded Anta bytes must load");
            let glyph = atlas.ensure_glyph('😀', 32.0);
            assert!(glyph.width > 0 && glyph.height > 0, "emoji glyph must produce a non-empty raster");
            assert!(!glyph.is_color, "the bundled Noto Emoji assets are monochrome outline-only");
            assert!(atlas.take_dirty());
        }

        #[test]
        fn packing_a_synthetic_color_glyph_lands_on_the_rgba_page_and_marks_it_dirty() {
            let mut atlas = FontAtlas::from_bytes(super::ANTA_LATIN).expect("embedded Anta bytes must load");
            assert!(!atlas.take_color_dirty());
            atlas.pack_glyph(('🔥', 32), super::RasterizedGlyph { bitmap: vec![255u8; 4 * 4 * 4], width: 4, height: 4, bearing_x: 0.0, bearing_y: 0.0, advance: 32.0, is_color: true });
            let glyph = atlas.ensure_glyph('🔥', 32.0);
            assert!(glyph.is_color);
            assert_eq!((glyph.width, glyph.height), (4, 4));
            assert!(atlas.take_color_dirty(), "packing a color glyph must mark the color page dirty");
            assert!(!atlas.take_color_dirty(), "take_color_dirty must reset after being read");
        }
    }
    // #endregion text
}

pub mod theme {
    // #region theme
    //! 🎨️ Theme colors and metrics for wgpu UI rendering.

    use crate::geometry::Rect;
    use ui_styling::appearance::AppearanceName;
    use ui_styling::{
        levels,
        metrics::{chrome as chrome_metrics, dom, typography},
        radii, strokes, ChromePalette, CHROME_DARK, CHROME_LIGHT,
    };

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Rgba {
        pub r: f32,
        pub g: f32,
        pub b: f32,
        pub a: f32,
    }

    impl Rgba {
        pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
            Self { r, g, b, a }
        }

        pub fn from_srgb8(r: u8, g: u8, b: u8, a: u8) -> Self {
            let [lr, lg, lb, la] = ui_styling::color::rgba8_to_linear(r, g, b, a);
            Self::new(lr, lg, lb, la)
        }

        fn from_chrome(c: &[f32; 4]) -> Self {
            Self::new(c[0], c[1], c[2], c[3])
        }

        pub fn with_alpha(self, a: f32) -> Self {
            Self::new(self.r, self.g, self.b, a)
        }
    }

    //#region 🔖️Level
    /// 🪜️ The unified 6-level UI surface axis (base..menu, both z-order and glass/shade formula input)
    /// — see `ui/styling/🔣️tokens.json`'s `levels` block and `.🦑️repo/🎫️tickets/26/07/27/UNIFIED-6-LEVEL-UI-SURFACE-SYSTEM/contract.txt`.
    /// Replaces the old unlinked level-name axis (canvas/window/panel/overlay/temporary) plus a
    /// separate glass-tier axis (panel/ribbon/menu/windowOptions) with one formula-derived enum.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Level {
        Base,
        Window,
        Pane,
        Panel,
        Dialog,
        Menu,
    }

    impl Level {
        /// 🔢️ Ordinal step `k` (0..=5) every formula-derived value (`Theme::surface`/`glass`)
        /// is computed from — mirrors `ui/styling/rs/🤖️generated.rs`'s `levels::NAMES` ordering.
        pub const fn index(self) -> usize {
            match self {
                Level::Base => 0,
                Level::Window => 1,
                Level::Pane => 2,
                Level::Panel => 3,
                Level::Dialog => 4,
                Level::Menu => 5,
            }
        }
    }
    //#endregion 🔖️Level

    #[derive(Clone, Copy, Debug)]
    pub struct GlassStyle {
        pub tint: Rgba,
        pub alpha: f32,
        pub blur_px: f32,
        pub saturate: f32,
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Theme {
        pub background: Rgba,
        pub panel: Rgba,
        pub panel_border: Rgba,
        pub navbar: Rgba,
        pub text: Rgba,
        pub text_muted: Rgba,
        pub accent: Rgba,
        pub accent_hover: Rgba,
        pub active_foreground: Rgba,
        pub button: Rgba,
        pub button_hover: Rgba,
        pub input_bg: Rgba,
        pub separator: Rgba,
        pub selected: Rgba,
        pub canvas_clear: Rgba,
        pub temporary: Rgba,
        pub gap_standard: f32,
        pub padding_standard: f32,
        pub navbar_height: f32,
        pub panel_header_height: f32,
        pub control_height: f32,
        pub control_height_small: f32,
        pub glass_saturate: f32,
        pub font_size_body: f32,
        pub font_size_small: f32,
        pub font_size_emphasized: f32,
        pub footer_height: f32,
        pub panel_inset: f32,
        pub panel_min_width: f32,
        pub panel_max_width: f32,
        pub window_measures_default_width: f32,
        pub window_engagement_max_width: f32,
        pub overlay_shadow: Rgba,
        pub focus_ring: Rgba,
        pub row_hover: Rgba,
        pub border_radius: f32,
        pub border_normal: Rgba,
        pub border_emphasized: Rgba,
        pub text_element: Rgba,
        pub stroke_hairline: f32,
        pub checker_light: Rgba,
        pub checker_dark: Rgba,
        pub diagram_stroke: Rgba,
        pub diagram_seam: Rgba,
        pub diagram_accent: Rgba,
        pub diagram_accent_fill: Rgba,
        pub error: Rgba,
        /// 🪜️ Plain per-level fill, indexed by `Level::index` — `ui-surface`'s wgpu counterpart, backing
        /// `Theme::surface`/`glass`. Populated from the generated `levelBase..levelMenu`
        /// chrome paints (see `from_chrome` below).
        pub level_bg: [Rgba; 6],
    }

    impl Default for Theme {
        fn default() -> Self {
            Self::dark()
        }
    }

    fn chrome_px(ui_spacing_mult: f64) -> f32 {
        (chrome_metrics::UI_SPACING_COMPACT_PX * ui_spacing_mult) as f32
    }

    fn panel_width(ui_spacing_mult: f64) -> f32 {
        (chrome_metrics::UI_SPACING_COMPACT_PX * ui_spacing_mult) as f32
    }

    fn from_chrome(chrome: &ChromePalette) -> Theme {
        Theme {
            background: Rgba::from_chrome(&chrome.base),
            panel: Rgba::from_chrome(&chrome.level_panel),
            panel_border: Rgba::from_chrome(&chrome.border_normal),
            navbar: Rgba::from_chrome(&chrome.level_window),
            text: Rgba::from_chrome(&chrome.foreground),
            text_muted: Rgba::from_chrome(&chrome.muted_foreground),
            accent: Rgba::from_chrome(&chrome.accent),
            accent_hover: Rgba::from_chrome(&chrome.active_hover),
            active_foreground: Rgba::from_chrome(&chrome.active_foreground),
            button: Rgba::from_chrome(&chrome.level_window),
            button_hover: Rgba::from_chrome(&chrome.hover_interactive_fill),
            input_bg: Rgba::from_chrome(&chrome.base),
            separator: Rgba::from_chrome(&chrome.border_normal),
            selected: Rgba::from_chrome(&chrome.active_base),
            canvas_clear: Rgba::from_chrome(&chrome.base),
            temporary: Rgba::from_chrome(&chrome.level_menu),
            gap_standard: chrome_px(chrome_metrics::GAP_STANDARD_UI_SPACING),
            padding_standard: chrome_px(chrome_metrics::PADDING_STANDARD_UI_SPACING),
            navbar_height: chrome_px(chrome_metrics::NAVBAR_HEIGHT_UI_SPACING),
            panel_header_height: chrome_px(chrome_metrics::PANEL_HEADER_HEIGHT_UI_SPACING),
            control_height: chrome_px(chrome_metrics::CONTROL_HEIGHT_UI_SPACING),
            control_height_small: chrome_px(5.0),
            glass_saturate: levels::GLASS_SATURATE as f32,
            font_size_body: typography::TEXT_SM_PX as f32,
            font_size_small: typography::TEXT_XS_PX as f32,
            font_size_emphasized: typography::TEXT_BASE_PX as f32,
            footer_height: chrome_px(chrome_metrics::FOOTER_HEIGHT_UI_SPACING),
            panel_inset: chrome_px(chrome_metrics::PANEL_INSET_UI_SPACING),
            panel_min_width: panel_width(dom::LAYOUT_PANEL_MIN_UI_SPACING),
            panel_max_width: panel_width(dom::LAYOUT_PANEL_MAX_UI_SPACING),
            window_measures_default_width: chrome_px(dom::LAYOUT_PANEL_RAIL_UI_SPACING),
            window_engagement_max_width: chrome_px(dom::LAYOUT_ENGAGEMENT_MAX_UI_SPACING),
            overlay_shadow: Rgba::new(0.0, 0.0, 0.0, 0.0),
            focus_ring: Rgba::from_chrome(&chrome.accent).with_alpha(0.6),
            row_hover: Rgba::from_chrome(&chrome.hover_interactive_fill),
            border_radius: radii::CHROME as f32,
            border_normal: Rgba::from_chrome(&chrome.border_normal),
            border_emphasized: Rgba::from_chrome(&chrome.border_emphasized),
            text_element: Rgba::from_chrome(&chrome.border_element),
            stroke_hairline: strokes::CHROME_BORDER_HAIRLINE as f32,
            checker_light: Rgba::new(0.85, 0.85, 0.85, 1.0),
            checker_dark: Rgba::new(0.72, 0.72, 0.72, 1.0),
            diagram_stroke: Rgba::new(0.2, 0.55, 0.95, 0.95),
            diagram_seam: Rgba::new(0.95, 0.45, 0.2, 0.95),
            diagram_accent: Rgba::new(0.25, 0.45, 0.65, 0.9),
            diagram_accent_fill: Rgba::new(0.25, 0.35, 0.55, 0.8),
            error: Rgba::new(0.95, 0.35, 0.35, 1.0),
            level_bg: [
                Rgba::from_chrome(&chrome.level_base),
                Rgba::from_chrome(&chrome.level_window),
                Rgba::from_chrome(&chrome.level_pane),
                Rgba::from_chrome(&chrome.level_panel),
                Rgba::from_chrome(&chrome.level_dialog),
                Rgba::from_chrome(&chrome.level_menu),
            ],
        }
    }

    impl Theme {
        pub fn light() -> Self {
            from_chrome(&CHROME_LIGHT)
        }

        pub fn dark() -> Self {
            from_chrome(&CHROME_DARK)
        }

        pub fn for_name(name: AppearanceName) -> Self {
            match name {
                AppearanceName::Light => Self::light(),
                AppearanceName::Dark => Self::dark(),
            }
        }

        //#region 🔖️LevelSurfaces
        /// 🪜️ Plain per-level fill (no blur/alpha) — `ui-surface`'s wgpu counterpart.
        pub fn surface(&self, level: Level) -> Rgba {
            self.level_bg[level.index()]
        }

        /// 🧊️ Formula-derived glass style for `level` — `ui-glass`'s wgpu counterpart. Alpha steps down
        /// and blur steps up per level index (`ui/styling/🔣️tokens.json`'s `levels` block:
        /// `alpha(k) = 1 - k * glassAlphaStep`, `blur(k) = k * glassBlurStepPx`), read from
        /// `ui_styling::levels` constants — never a per-tier lookup table.
        pub fn glass(&self, level: Level) -> GlassStyle {
            let k = level.index() as f32;
            GlassStyle { tint: self.level_bg[level.index()], alpha: 1.0 - k * levels::GLASS_ALPHA_STEP as f32, blur_px: k * levels::GLASS_BLUR_STEP_PX as f32, saturate: self.glass_saturate }
        }

        //#endregion 🔖️LevelSurfaces

        pub fn glass_mip_level(blur_px: f32, max_mip: u32) -> f32 {
            (blur_px / 4.0).log2().max(0.0).min(max_mip as f32)
        }
    }

    pub type ThemedRect = Rect;

    // #endregion theme
}

#[cfg(feature = "engine")]
pub mod paint {
    // #region paint
    //! 🖌️ Retained paint pass. Per-`UiNode`-variant drawing logic mechanically ported from the
    //! immediate-mode `widgets::render_*` functions (see that region's doc comment for why it still
    //! exists), reading resolved geometry from `tree::LayoutBucket` (accumulating parent-relative
    //! offsets while walking, since taffy's `Layout::location` is parent-relative — see that struct's
    //! doc comment) instead of the old `bounds: Rect` argument an immediate-mode caller threaded down.
    //! Interaction-derived visuals (hover/focus/active/selected) read live `NodeFlags`/`WidgetState`,
    //! written each frame by `events::EventRouter` (M5, landed) — no longer default/empty by the time
    //! `paint_tree` runs, as an earlier revision of this comment used to caveat. `WidgetState`-backed
    //! composites have since gained real paint support too: an open `Select`'s popup expands live
    //! (`paint_select`'s `open`/`retained` params, wired by the W2 pass — see
    //! `.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w2-ui-wgpu-integration.md`), and a focused
    //! `Input`'s caret/selection-highlight render straight from its live `EditState` (`paint_input`,
    //! W2 widget-visuals pass). `Tree`'s live scroll offset (`WidgetState::scroll_offset`) remains the
    //! one rest-state-only exception — no scrollable-viewport paint exists yet, out of every pass to
    //! date's scope.

    use crate::arena::NodeId;
    use crate::chrome::{chrome_item_bg, item_bg, item_text, push_chrome_border, push_control_border, push_icon, ICON_TINY};
    use crate::component::ui::{
        UiButtonNode, UiComponentSceneNode, UiControlNode, UiExternalSlotNode, UiFieldNode, UiGroupNode, UiIconSelectNode, UiImageNode, UiInputNode, UiKeyValueNode, UiNode, UiNumberStepperNode, UiPresence, UiRingNode, UiSectionNode, UiSelectItem,
        UiSelectNode, UiSliderNode, UiStackNode, UiState, UiStatus, UiTextNode, UiToggleNode, UiTreeItemNode, UiTreeNode, UI_INSPECTOR_MIXED_PLACEHOLDER,
    };
    use crate::draw::{DrawList, IconAtlas};
    use crate::geometry::Rect;
    use crate::text::FontAtlas;
    use crate::theme::{Level, Rgba, Theme};
    use crate::tree::{EditState, NodeFlags, NodeKey, UiTree};
    use crate::widgets::{draw_text_on, wrap_text};
    use crate::IconName;
    use crate::Label;
    use crate::UiTreeActionPlacement;

    const PANEL_HEADER: f32 = 24.0;
    const TREE_ROW_HEIGHT: f32 = 24.0;
    const TREE_INDENT_PER_LEVEL: f32 = 10.0;
    const TREE_TOGGLE_WIDTH: f32 = 14.0;
    const TREE_ICON_SIZE: f32 = 14.0;

    /// 🖼️ Top-level entry point: unconditionally walks and (re)paints every node reachable from `root`,
    /// clearing `DIRTY_PAINT` as it visits (mirroring `flex::LayoutEngine::write_back`'s clear-as-you-go
    /// pattern) but never touching `DIRTY_LAYOUT`/`SUBTREE_DIRTY` — clearing those is `flex`'s job and
    /// `flex::LayoutEngine::compute` already runs (and clears them) before paint each frame, per the
    /// intended pipeline. Deliberately has **no internal early-out**: `DrawList` only supports a full
    /// clear-and-rebuild (no API to remove/replace a single dirty subtree's prior draw calls while
    /// leaving clean siblings' draw calls in place), so a genuinely incremental repaint isn't safe to
    /// build yet. Whether to call `paint_tree` at all this frame — i.e. "was anything dirty" — is a
    /// decision a later milestone's `engine` facade owns (it already knows from driving `flex::compute`
    /// and `reconcile::apply_tree`), not something `paint_tree` decides for itself.
    /// 🎬️ `has_scene_host` gates the `ComponentScene`/`Image` leaf arms below (see `paint_node`'s own
    /// match): when the caller's `engine::Ui::frame` has a real `scene_slots::SceneHost` for this tick,
    /// those leaves paint NOTHING here — the host paints the real content into the same rect right after
    /// this call, in `Ui::frame`'s `collect_scene_slots` loop — instead of this pass drawing placeholder
    /// chrome that the host would then have to paint over. With no host (`false`), behavior is unchanged
    /// from before this parameter existed: `paint_component_scene`/`paint_image`'s own placeholder chrome.
    pub(crate) fn paint_tree(tree: &mut UiTree, root: NodeId, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, has_scene_host: bool, draw: &mut DrawList) {
        sync_interactive_state(tree, root, theme);
        paint_node(tree, root, 0.0, 0.0, theme, atlas, icons, has_scene_host, draw);
        clear_dirty_paint(tree, root);
    }

    fn clear_dirty_paint(tree: &mut UiTree, id: NodeId) {
        if let Some(node) = tree.node_mut(id) {
            node.flags.set(NodeFlags::DIRTY_PAINT, false);
        }
        let children: Vec<NodeId> = tree.children(id).collect();
        for child in children {
            clear_dirty_paint(tree, child);
        }
    }

    //#region 🔖️InteractiveStateSync
    // 🔗️ W2 wiring: a paint-owned pre-pass, mutable (unlike `paint_node`'s own read-only walk below),
    // run once per `paint_tree` call before painting anything — writes derived state `flex`/`reconcile`
    // have no way to produce for composite widgets they don't fully own the interactive geometry of:
    //  - an open `Select`'s synthesized item-row `Button`s (`reconcile::children_of`'s `Select` arm)
    //    get real per-row `LayoutBucket` rects here (`flex::style_for`'s fallback leaf style gives every
    //    one of them a zero-size rect — neither `Select` nor its rows are a flex container `flex` grants
    //    space to), computed with the exact geometry `paint_select` itself paints the popup at (see
    //    `select_popup_row_rect`), so `events::hit_test` can actually find and click them.
    //  - a `Stack`'s `NodeFlags::DROP_TARGET` bit is kept in sync with its own `drop_action` field
    //    (`events::nearest_accepting_drop_target` walks the bubble chain for this flag).
    //  - a `Tree`'s synthesized per-row `Stack`s (`reconcile::children_of`'s `Tree` arm) get real
    //    per-row rects too (same zero-size root cause as `Select`'s rows), computed with the exact
    //    row-height/indent math `paint_tree_item` paints rows at, and their `NodeFlags::DRAG_SOURCE` bit
    //    is kept in sync with the *original* `UiTreeItemNode`'s `draggable` field (`reconcile` never
    //    drops fields, only clones them into `WidgetSpec` — see that module's own doc comment).

    /// 🔎️ Finds `parent`'s retained child keyed `key` — `reconcile`'s synthesized `Select`/`Tree` rows
    /// are keyed by stable identity (`item.value`/`section.id`/`item.id`, see `reconcile::children_of`),
    /// so this is how this pass re-associates a declarative row (`UiSelectItem`/`UiTreeItemNode`) with
    /// its already-existing retained `NodeId`, robust to reconcile's insertion-order quirks (a re-used
    /// matched child physically keeps its old sibling-list position — see that module's own doc comment
    /// on why key lookup, not positional indexing, is the safe way to do this).
    fn find_child_by_key(tree: &UiTree, parent: NodeId, key: &NodeKey) -> Option<NodeId> {
        tree.children(parent).find(|&child| tree.node(child).map(|n| &n.key) == Some(key))
    }

    fn sync_interactive_state(tree: &mut UiTree, id: NodeId, theme: &Theme) {
        let select_open: Option<(Vec<UiSelectItem>, f32, f32)> = tree.node(id).and_then(|node| match &node.spec.0 {
            UiNode::Select(select) if node.state.open => Some((select.items.clone(), node.layout.width, node.layout.height)),
            _ => None,
        });
        if let Some((items, select_w, select_h)) = select_open {
            sync_select_popup_rows(tree, id, &items, select_w, select_h, theme);
        }

        let stack_drop_target: Option<bool> = tree.node(id).and_then(|node| match &node.spec.0 {
            UiNode::Stack(stack) => Some(stack.drop_action.is_some()),
            _ => None,
        });
        if let Some(accepts_drop) = stack_drop_target {
            if let Some(node) = tree.node_mut(id) {
                node.flags.set(NodeFlags::DROP_TARGET, accepts_drop);
            }
        }

        if tree.node(id).is_some_and(|node| matches!(node.spec.0, UiNode::Tree(_))) {
            sync_tree_row_layout(tree, id);
        }

        let children: Vec<NodeId> = tree.children(id).collect();
        for child in children {
            sync_interactive_state(tree, child, theme);
        }
    }

    /// 📐️ One popup row's `(x, y, w, h)` **relative to the `Select`'s own top-left** — shared by
    /// `sync_select_popup_rows` (writes it into the row's retained `LayoutBucket`) and `paint_select`
    /// (paints it), so the two can never drift apart. Mirrors `widgets::render_select_menu`'s literal
    /// geometry: the popup sits `select_h + 2.0` below the trigger, each row inset `2.0`,
    /// `theme.control_height` tall.
    fn select_popup_row_rect(select_w: f32, select_h: f32, index: usize, theme: &Theme) -> Rect {
        let item_h = theme.control_height;
        let menu_y = select_h + 2.0;
        Rect::new(2.0, menu_y + 2.0 + index as f32 * item_h, (select_w - 4.0).max(0.0), item_h)
    }

    fn sync_select_popup_rows(tree: &mut UiTree, select_id: NodeId, items: &[UiSelectItem], select_w: f32, select_h: f32, theme: &Theme) {
        for (index, item) in items.iter().enumerate() {
            let Some(row_id) = find_child_by_key(tree, select_id, &NodeKey::Explicit(item.value.clone())) else { continue };
            let rect = select_popup_row_rect(select_w, select_h, index, theme);
            if let Some(node) = tree.node_mut(row_id) {
                node.layout.x = rect.x;
                node.layout.y = rect.y;
                node.layout.width = rect.w;
                node.layout.height = rect.h;
            }
        }
    }

    /// 🌳️ Gives each of a `Tree`'s synthesized per-section `Stack`s (`reconcile::children_of`'s `Tree`
    /// arm, keyed by `section.id`) real `LayoutBucket` geometry, cumulative down the tree exactly like
    /// `paint_tree_widget`'s own procedural walk (header height, then each item's row height including
    /// any expanded nested rows).
    fn sync_tree_row_layout(tree: &mut UiTree, tree_id: NodeId) {
        let Some(tree_node) = tree.node(tree_id).and_then(|node| match &node.spec.0 {
            UiNode::Tree(tree_node) => Some(tree_node.clone()),
            _ => None,
        }) else {
            return;
        };
        let width = tree.node(tree_id).map_or(0.0, |node| node.layout.width);
        let mut section_y = 0.0;
        for section in &tree_node.sections {
            let Some(section_id) = find_child_by_key(tree, tree_id, &NodeKey::Explicit(section.id.clone())) else { continue };
            let header_offset = if section.label.is_some() { PANEL_HEADER } else { 0.0 };
            let mut item_y = header_offset;
            for item in &section.items {
                item_y += sync_tree_item_layout(tree, section_id, item, item_y, width);
            }
            if let Some(node) = tree.node_mut(section_id) {
                node.layout.x = 0.0;
                node.layout.y = section_y;
                node.layout.width = width;
                node.layout.height = item_y;
            }
            section_y += item_y;
        }
    }

    /// 🌳️ Recursive per-item counterpart of `sync_tree_row_layout`, one level down — writes `item`'s own
    /// retained row `Stack` geometry (found by `item.id`, `reconcile::tree_item_row`'s key) at
    /// `y_offset` relative to `parent` (its retained parent row/section), then recurses into any
    /// expanded nested `items` relative to *this* row, mirroring `paint_tree_item`'s identical
    /// recursion. Also keeps `NodeFlags::DRAG_SOURCE` synced with `item.draggable` (see
    /// `events::is_plain_stack_container`/`set_drag_payload` for the two consumers of that bit). Returns
    /// the total height (own row + any expanded nested rows) consumed, for the caller's own cursor.
    fn sync_tree_item_layout(tree: &mut UiTree, parent: NodeId, item: &UiTreeItemNode, y_offset: f32, width: f32) -> f32 {
        if !item.presence.visible() {
            return 0.0;
        }
        let Some(item_id) = find_child_by_key(tree, parent, &NodeKey::Explicit(item.id.clone())) else {
            return TREE_ROW_HEIGHT;
        };
        let expandable = item.items.as_ref().is_some_and(|items| !items.is_empty());
        let expanded = expandable && item.default_open.unwrap_or(false);
        let mut nested_height = 0.0;
        if expanded {
            for nested in item.items.as_ref().unwrap() {
                nested_height += sync_tree_item_layout(tree, item_id, nested, TREE_ROW_HEIGHT + nested_height, width);
            }
        }
        let total_height = TREE_ROW_HEIGHT + nested_height;
        if let Some(node) = tree.node_mut(item_id) {
            node.layout.x = 0.0;
            node.layout.y = y_offset;
            node.layout.width = width;
            node.layout.height = total_height;
            node.flags.set(NodeFlags::DRAG_SOURCE, item.draggable.unwrap_or(false));
        }
        total_height
    }
    //#endregion 🔖️InteractiveStateSync

    /// 🎯️ Per-variant paint dispatcher for one retained node, given `(origin_x, origin_y)` — the
    /// absolute position of *this node's parent's* content-box origin (so `origin + node.layout.{x,y}`
    /// is this node's own absolute top-left, matching taffy's parent-relative `Layout::location`).
    #[allow(clippy::too_many_arguments, reason = "one arg per paint context resource; grouping into a struct is a T2 restructure, out of scope")]
    /// 🧭️ The one shared presence overlay every `UiNode` variant gets for free, drawn centrally by
    /// `paint_node` after that variant's own paint: `previewed`/`disabled` fills underneath nothing extra
    /// (disabled reads as a scrim so it composes over whatever the variant already drew), a `status` ring
    /// (loading spin / waiting dash / finished solid — mutually exclusive, `idle` draws nothing), an
    /// outset accent ring for `selected`, and a breathing pulse ring for `introducing`. `hover` has no
    /// dedicated draw call here — it's folded into `flags` before dispatch (see `paint_node`) so every
    /// variant's own hover-aware fill (already reading `NodeFlags::HOVERED`) picks it up for free.
    fn presence_overlay(draw: &mut DrawList, bounds: Rect, theme: &Theme, presence: UiPresence) {
        if presence.state == UiState::Disabled {
            draw.push_solid([bounds.x, bounds.y, bounds.w, bounds.h], theme.panel.with_alpha(0.35));
        }
        let ring_color = if presence.selected { theme.selected } else { theme.border_normal };
        match presence.status {
            UiStatus::Loading => paint_loading_border(draw, bounds, ring_color, theme),
            UiStatus::Waiting => paint_waiting_border(draw, bounds, ring_color, theme),
            UiStatus::Finished => draw.push_finished_border([bounds.x, bounds.y, bounds.w, bounds.h], ring_color, theme.border_radius, theme.stroke_hairline),
            UiStatus::Idle => {}
        }
        if presence.selected {
            let ring = Rect::new(bounds.x - 1.0, bounds.y - 1.0, bounds.w + 2.0, bounds.h + 2.0);
            push_chrome_border(draw, ring, theme.stroke_hairline, theme.accent, true, true, true, true);
        } else if presence.state == UiState::Previewed {
            // 🔍️ Inset (not outset, unlike `selected`'s ring) hairline so the two stay distinguishable
            // when composed — a previewed-and-selected element still reads as selected via the outset ring.
            push_chrome_border(draw, bounds, theme.stroke_hairline, theme.accent, true, true, true, true);
        }
        if presence.state == UiState::Introducing {
            draw.push_introducing_border([bounds.x, bounds.y, bounds.w, bounds.h], theme.accent, theme.border_radius, theme.stroke_hairline);
        }
        // 🎉️ `Celebrating` reuses the introducing breathing-pulse ring — `Theme` has no primary/secondary/
        // tertiary triad to cycle through, so `theme.accent` is the honest static reduction of the CSS
        // spinning tri-color ring for this shader-less renderer; a true conic tri-color ring is out of scope.
        if presence.state == UiState::Celebrating {
            draw.push_introducing_border([bounds.x, bounds.y, bounds.w, bounds.h], theme.accent, theme.border_radius, theme.stroke_hairline);
        }
    }

    pub(crate) fn paint_node(tree: &UiTree, id: NodeId, origin_x: f32, origin_y: f32, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, has_scene_host: bool, draw: &mut DrawList) {
        let Some(node) = tree.node(id) else { return };
        let presence = node.spec.0.presence();
        if !presence.visible() {
            return;
        }
        let abs_x = origin_x + node.layout.x;
        let abs_y = origin_y + node.layout.y;
        let bounds = Rect::new(abs_x, abs_y, node.layout.width, node.layout.height);
        if matches!(presence.status, UiStatus::Loading | UiStatus::Waiting) {
            draw.push_solid([bounds.x + theme.padding_standard, bounds.y + theme.padding_standard, (bounds.w - theme.padding_standard * 2.0).max(0.0), (bounds.h - theme.padding_standard * 2.0).max(0.0)], theme.button_hover);
            presence_overlay(draw, bounds, theme, presence);
            return;
        }
        // 🖱️ Authored `presence.hover` (default false) composes with live pointer hover: every variant's
        // own paint already reads `NodeFlags::HOVERED` for its hover-aware fill, so folding the authored
        // flag in here — suppressed while disabled, matching `events::EventRouter`'s own suppression —
        // makes it effective everywhere for free, with no per-variant paint changes.
        let mut flags = node.flags;
        if presence.state != UiState::Disabled {
            flags.set(NodeFlags::HOVERED, flags.contains(NodeFlags::HOVERED) || presence.hover);
        }
        match &node.spec.0 {
            UiNode::Stack(stack) => {
                paint_stack_frame(stack, bounds, flags, theme, draw);
                paint_stack(tree, id, abs_x, abs_y, theme, atlas, icons, has_scene_host, draw);
            }
            UiNode::Text(text) => paint_text(text, bounds, theme, atlas, draw),
            UiNode::Separator(_) => paint_separator(bounds, theme, draw),
            UiNode::Button(button) => paint_button(button, bounds, flags, theme, atlas, icons, draw),
            UiNode::Input(input) => paint_input(input, node.state.edit.as_ref(), bounds, flags, theme, atlas, draw),
            UiNode::Select(select) => paint_select(select, bounds, flags, node.state.open, Some((tree, id)), theme, atlas, icons, draw),
            UiNode::Toggle(toggle) => paint_toggle(toggle, bounds, flags, theme, atlas, icons, draw),
            UiNode::KeyValue(kv) => paint_key_value(kv, bounds, theme, atlas, draw),
            UiNode::Slider(slider) => paint_slider(slider, bounds, theme, atlas, draw),
            UiNode::NumberStepper(stepper) => paint_number_stepper(stepper, bounds, flags, theme, atlas, draw),
            UiNode::Ring(ring) => paint_ring(ring, bounds, theme, draw),
            UiNode::IconSelect(select) => paint_icon_select(select, bounds, flags, theme, atlas, icons, draw),
            UiNode::Field(field) => {
                paint_field(field, bounds, theme, atlas, draw);
                paint_stack(tree, id, abs_x, abs_y, theme, atlas, icons, has_scene_host, draw);
            }
            UiNode::Section(section) => {
                paint_section(section, bounds, theme, atlas, icons, draw);
                paint_stack(tree, id, abs_x, abs_y, theme, atlas, icons, has_scene_host, draw);
            }
            UiNode::Group(group) => {
                paint_group(group, bounds, theme, atlas, icons, draw);
                paint_stack(tree, id, abs_x, abs_y, theme, atlas, icons, has_scene_host, draw);
            }
            UiNode::Tree(tree_node) => paint_tree_widget(tree_node, bounds, theme, atlas, icons, draw),
            // 🎬️ With a `SceneHost` registered this tick, leave these two rects untouched here —
            // `engine::Ui::frame`'s `collect_scene_slots` loop paints the real content right after this
            // pass returns. With no host, fall back to the unchanged placeholder chrome.
            UiNode::Image(image) => {
                if !has_scene_host {
                    paint_image(image, bounds, theme, atlas, draw);
                }
            }
            UiNode::ComponentScene(scene) => {
                if !has_scene_host {
                    paint_component_scene(scene, bounds, theme, draw);
                }
            }
            UiNode::ExternalSlot(slot) => paint_external_slot(slot, bounds, theme, atlas, draw),
        }
        presence_overlay(draw, bounds, theme, presence);
    }

    /// 🌀️ Shared "this node is loading" affordance for every `UiNode` kind that carries a
    /// `loading: Option<bool>` flag (`Button`, `Stack`, `Section`, `Tree`, `TreeItem`). Delegates to
    /// `draw::DrawList::push_loading_border`, which already renders a real time-varying (spinning +
    /// pulsing) ring via `UI_SHADER`'s `kind == 6` branch fed by `render_frame`'s `time_seconds`
    /// uniform (see `UiInstance::loading_border`'s doc comment) — despite older planning docs assuming
    /// no animation-clock scaffolding exists anywhere in this crate, `draw`/`shaders` already wired one
    /// in at the GPU layer; this helper just standardizes the radius/stroke args every `paint` call site
    /// passes into that existing primitive, leaving only `color` (which varies with e.g. selected state)
    /// to the caller.
    fn paint_loading_border(draw: &mut DrawList, bounds: Rect, color: Rgba, theme: &Theme) {
        draw.push_loading_border([bounds.x, bounds.y, bounds.w, bounds.h], color, theme.border_radius, theme.stroke_hairline);
    }

    /// 🌀️ Shared "this node is waiting" affordance mirroring `paint_loading_border`: dashed, slower ring
    /// via `draw::DrawList::push_waiting_border` (`UI_SHADER`'s `kind == 7` branch). Callers dispatch
    /// `loading` before `waiting` so the more active state wins when both flags are set.
    fn paint_waiting_border(draw: &mut DrawList, bounds: Rect, color: Rgba, theme: &Theme) {
        draw.push_waiting_border([bounds.x, bounds.y, bounds.w, bounds.h], color, theme.border_radius, theme.stroke_hairline);
    }

    /// 🎴️ A `Stack`'s `activate`/`selected` visual affordances, ported from
    /// `framework/renderer/react/ui-interpreter.tsx`'s `case "stack"` (`widgets::WidgetNode::Stack` has
    /// neither field to port from — see this region's own doc comment on why `widgets` is an incomplete
    /// reference for fixtures like this one): `activate` (React's `"border bg-panel cursor-pointer
    /// rounded-md"`) paints a filled `theme.panel` background (brighter, `theme.button_hover`, while
    /// `events::EventRouter`'s hover-chain has flagged it `NodeFlags::HOVERED` — see
    /// `events::is_plain_stack_container`'s matching hit-test exception for why an activatable Stack can
    /// be hovered/clicked at all) with a normal border; `selected` (`"ring-primary border-primary
    /// ring-1"`) paints an accent-colored border plus a slightly outset accent ring, approximating the
    /// DOM's separate `ring`+`border` layers with this crate's single stroke-rect primitive.
    /// `dropAction`'s accept-a-drop affordance has no dedicated visual in the React reference either
    /// (`onDragOver`/`onDrop` are behavioral only) — its only paint-visible effect is keeping
    /// `NodeFlags::DROP_TARGET` in sync (`sync_interactive_state`, above), consumed by
    /// `events`/cursor-derivation, not drawn here.
    fn paint_stack_frame(stack: &UiStackNode, bounds: Rect, flags: NodeFlags, theme: &Theme, draw: &mut DrawList) {
        let activatable = stack.activate.is_some();
        if !activatable {
            return;
        }
        let hovered = flags.contains(NodeFlags::HOVERED);
        let bg = if hovered { theme.button_hover } else { theme.panel };
        push_control_border(draw, bounds, theme, theme.border_normal, bg);
    }

    /// 🧱️ `Stack`'s own paint (beyond `paint_node`'s separate `paint_stack_frame` call for its
    /// `activate`/`selected` affordance) is a no-operation — it's pure layout; this just recurses into its
    /// retained children, each offset by this node's absolute top-left. Also reused by `Field`/`Section`,
    /// whose single/`children` nested `UiNode`s reconcile already expands into retained children (see
    /// `reconcile::children_of`) — `paint_stack_frame` doesn't apply to either (neither carries
    /// `activate`/`selected`).
    #[allow(clippy::too_many_arguments, reason = "one arg per paint context resource; grouping into a struct is a T2 restructure, out of scope")]
    fn paint_stack(tree: &UiTree, id: NodeId, abs_x: f32, abs_y: f32, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, has_scene_host: bool, draw: &mut DrawList) {
        let children: Vec<NodeId> = tree.children(id).collect();
        for child in children {
            paint_node(tree, child, abs_x, abs_y, theme, atlas, icons, has_scene_host, draw);
        }
    }

    fn paint_text(node: &UiTextNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, draw: &mut DrawList) {
        let emphasize = node.emphasize.unwrap_or(false);
        let size = if emphasize { theme.font_size_emphasized } else { theme.font_size_body };
        let color = if emphasize { theme.text } else { theme.text_muted };
        let lines = wrap_text(atlas, node.value.as_str(), bounds.w.max(1.0), size);
        let line_h = size * 1.35;
        for (index, line) in lines.iter().enumerate() {
            draw_text_on(draw, atlas, line, bounds.x, bounds.y + line_h * index as f32 + size, size, color);
        }
    }

    fn paint_separator(bounds: Rect, theme: &Theme, draw: &mut DrawList) {
        let y = bounds.y + bounds.h * 0.5;
        draw.push_line(bounds.x, y, bounds.x + bounds.w, y, theme.separator, 1.0);
    }

    fn paint_button(node: &UiButtonNode, bounds: Rect, flags: NodeFlags, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList) {
        // 🚫️ `disabled:opacity-50` is the shared dimming convention this codebase's React reference
        // (`ui/js/react/index.tsx`'s form controls) uses for every disabled interactive control; ported
        // here via `Rgba::with_alpha` since `paint` has no CSS to lean on. A disabled control also can't
        // be hovered — `widgets::render_button` has no `disabled` concept at all (see this region's own
        // doc comment on why `widgets` is an incomplete reference for this specific fixture), so this is
        // an independent, `UiButtonNode.disabled`-driven fix rather than a widgets port.
        let disabled = node.presence.state == UiState::Disabled;
        let hovered = !disabled && flags.contains(NodeFlags::HOVERED);
        // 🎯️ `formControlFocusBorderClass`'s `focus-visible:border-accent` (`ui/js/react/index.tsx`,
        // applied to every form-control primitive including `Button`) — `widgets::render_button` never
        // implemented a focus ring either (only `render_input` did), so this is another independent
        // React-sourced fix, mirroring `paint_input`'s own established border-swap convention.
        let focused = !disabled && flags.contains(NodeFlags::FOCUSED);
        let dim = |color: Rgba| if disabled { color.with_alpha(color.a * 0.5) } else { color };
        let bg = dim(item_bg(theme, false, hovered));
        let border = if focused { theme.border_emphasized } else { theme.border_normal };
        push_control_border(draw, bounds, theme, dim(border), bg);
        let mut text_x = bounds.x + theme.padding_standard;
        let icon_key = if node.icon_id == IconName::CircleDot { node.label.as_str() } else { node.icon_id.as_str() };
        if let Some(icons) = icons {
            if icons.icon_uv(icon_key).is_some() {
                push_icon(draw, icons, icon_key, text_x, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, dim(item_text(theme, false, hovered)));
                text_x += ICON_TINY + theme.gap_standard;
            }
        }
        draw_text_on(draw, atlas, node.label.as_str(), text_x, bounds.y + (bounds.h + theme.font_size_body) * 0.5 - 2.0, theme.font_size_body, dim(item_text(theme, false, hovered)));
    }

    /// ↔ Local mirror of `events::selection_bounds` — `anchor..caret` as `(start, end)` regardless of
    /// which is smaller (see `tree::EditState`'s own doc comment). Duplicated rather than imported
    /// across the `paint`/`events` module boundary for a one-line pure function; keep the two in sync
    /// if `EditState`'s selection convention ever changes.
    fn edit_selection_bounds(anchor: usize, caret: usize) -> (usize, usize) {
        (anchor.min(caret), anchor.max(caret))
    }

    /// ✍️ `edit` is `node.state.edit` (see `tree::WidgetState`'s doc comment: `Some` only while this
    /// `Input` is focused and has a live typing buffer). While present, the live `EditState::text`
    /// (with any in-progress IME `composition` spliced in at the caret for preview) wins over the
    /// declarative `node.value` — the same "focused buffer governs" contract `events::FocusState`
    /// already establishes — since caret/selection coordinates are only meaningful against the exact
    /// string they were computed from. Neither `widgets::render_input` nor React's native `<input>`
    /// (whose caret/selection are rendered by the browser itself, not by application code — there is no
    /// CSS/JSX to port for their exact geometry) has anything to port from, so caret/selection styling
    /// (`theme.accent`) is this pass's own independent choice, kept consistent with `paint_input`'s own
    /// pre-existing `border_emphasized`-on-focus convention.
    fn paint_input(node: &UiInputNode, edit: Option<&EditState>, bounds: Rect, flags: NodeFlags, theme: &Theme, atlas: &mut FontAtlas, draw: &mut DrawList) {
        let focused = flags.contains(NodeFlags::FOCUSED);
        let border = if focused { theme.border_emphasized } else { theme.border_normal };
        push_control_border(draw, bounds, theme, border, theme.input_bg);
        let text_x = bounds.x + 8.0;
        let text_baseline_y = bounds.y + (bounds.h + theme.font_size_body) * 0.5 - 2.0;
        if let Some(edit) = focused.then_some(edit).flatten() {
            let (start, end) = edit_selection_bounds(edit.anchor, edit.caret);
            if start != end {
                let (x0, _) = atlas.measure_text(&edit.text[..start], theme.font_size_body);
                let (x1, _) = atlas.measure_text(&edit.text[..end], theme.font_size_body);
                let sel_h = theme.font_size_body * 1.2;
                let sel_y = bounds.y + (bounds.h - sel_h) * 0.5;
                draw.push_solid([text_x + x0, sel_y, (x1 - x0).max(1.0), sel_h], theme.accent.with_alpha(0.3));
            }
            let mut display = edit.text.clone();
            if let Some(composition) = &edit.composition {
                display.insert_str(edit.caret, composition);
            }
            draw_text_on(draw, atlas, &display, text_x, text_baseline_y, theme.font_size_body, theme.text);
            let (caret_x, _) = atlas.measure_text(&edit.text[..edit.caret], theme.font_size_body);
            let caret_h = theme.font_size_body * 1.2;
            let caret_y = bounds.y + (bounds.h - caret_h) * 0.5;
            draw.push_solid([text_x + caret_x, caret_y, 1.0, caret_h], theme.accent);
            return;
        }
        let (display, muted): (&str, bool) = if node.value.is_empty() { (node.placeholder.as_ref().map(Label::as_str).unwrap_or(""), true) } else { (node.value.as_str(), false) };
        draw_text_on(draw, atlas, display, text_x, text_baseline_y, theme.font_size_body, if muted { theme.text_muted } else { theme.text });
    }

    /// 🔽️ `retained` is `Some((tree, id))` for a real top-level `Select` node (able to read its
    /// synthesized item rows' live `NodeFlags::HOVERED` for the popup's row-hover highlight) and `None`
    /// for an inline `Select` painted via `paint_control` (a `TreeItem`'s embedded control — no per-
    /// control `NodeId` exists for that yet, same caveat `paint_control`'s own doc comment already
    /// makes, so it always paints closed regardless of `open`). W2 wiring: `open` (from
    /// `tree::WidgetState::open`, toggled by `events::EventRouter::toggle_select_popup`) now has a real
    /// data source, closing the gap this function's own doc comment used to describe — when `true`, the
    /// popup paints below the trigger with the exact geometry `select_popup_row_rect` also writes into
    /// the rows' `LayoutBucket` (see `sync_select_popup_rows`), so clicking a row actually hit-tests.
    #[allow(clippy::too_many_arguments, reason = "one arg per paint context resource; grouping into a struct is a T2 restructure, out of scope")]
    fn paint_select(node: &UiSelectNode, bounds: Rect, flags: NodeFlags, open: bool, retained: Option<(&UiTree, NodeId)>, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList) {
        let hovered = flags.contains(NodeFlags::HOVERED);
        // 🎯️ `SelectTrigger`'s own `formControlFocusBorderClass` (`ui/js/react/index.tsx`) swaps its
        // border to `border-accent` on `focus-visible` — mirrored via the same border-swap convention
        // `paint_input`/`paint_button` already use, since `widgets::render_select` never implemented one.
        let focused = flags.contains(NodeFlags::FOCUSED);
        let bg = if hovered { theme.button_hover } else { theme.input_bg };
        let border = if focused { theme.border_emphasized } else { theme.border_normal };
        push_control_border(draw, bounds, theme, border, bg);
        let label = node.items.iter().find(|item| item.value == node.value).map_or_else(|| node.placeholder.as_ref().map(Label::as_str).unwrap_or("Select…"), |item| item.label.as_str());
        draw_text_on(draw, atlas, label, bounds.x + theme.padding_standard, bounds.y + (bounds.h + theme.font_size_body) * 0.5 - 2.0, theme.font_size_body, theme.text);
        if let Some(icons) = icons {
            push_icon(draw, icons, "chevron-down", bounds.x + bounds.w - theme.padding_standard - ICON_TINY, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, theme.text_element);
        }
        if !open {
            return;
        }
        let row_children: Vec<NodeId> = retained.map(|(tree, id)| tree.children(id).collect()).unwrap_or_default();
        let item_h = theme.control_height;
        let menu_h = node.items.len() as f32 * item_h + 4.0;
        let menu = Rect::new(bounds.x, bounds.y + bounds.h + 2.0, bounds.w, menu_h);
        draw.push_glass([menu.x, menu.y, menu.w, menu.h], theme.border_radius, theme.glass(Level::Menu));
        for (index, item) in node.items.iter().enumerate() {
            let relative = select_popup_row_rect(bounds.w, bounds.h, index, theme);
            let row = Rect::new(bounds.x + relative.x, bounds.y + relative.y, relative.w, relative.h);
            let row_hovered = retained.zip(row_children.get(index)).is_some_and(|((tree, _), &row_id)| tree.node(row_id).is_some_and(|n| n.flags.contains(NodeFlags::HOVERED)));
            if row_hovered || item.value == node.value {
                draw.push_rounded([row.x, row.y, row.w, row.h], theme.row_hover, theme.border_radius);
            }
            draw_text_on(draw, atlas, item.label.as_str(), row.x + 8.0, row.y + 18.0, theme.font_size_body, theme.text);
        }
    }

    fn paint_toggle(node: &UiToggleNode, bounds: Rect, flags: NodeFlags, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList) {
        let pressed = node.presence.selected;
        let hovered = flags.contains(NodeFlags::HOVERED);
        // 🎯️ Same `formControlFocusBorderClass` border-swap as `paint_button`/`paint_select` — the icon-
        // button variant `Toggle` renders through (`ui/js/react/index.tsx`) carries it too.
        let focused = flags.contains(NodeFlags::FOCUSED);
        let bg = item_bg(theme, pressed, hovered);
        let border = if focused { theme.border_emphasized } else { theme.border_normal };
        push_control_border(draw, bounds, theme, border, bg);
        let mut content_x = bounds.x + theme.padding_standard;
        if let Some(icons) = icons {
            if icons.icon_uv(node.icon_id.as_str()).is_some() {
                push_icon(draw, icons, node.icon_id.as_str(), content_x, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, item_text(theme, pressed, hovered));
                content_x += ICON_TINY + theme.gap_standard;
            }
        }
        if let Some(text) = &node.text {
            draw_text_on(draw, atlas, text.as_str(), content_x, bounds.y + (bounds.h + theme.font_size_body) * 0.5 - 2.0, theme.font_size_body, item_text(theme, pressed, hovered));
        }
    }

    fn paint_key_value(node: &UiKeyValueNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, draw: &mut DrawList) {
        let label_w = node.entries.iter().map(|entry| atlas.measure_text(entry.label.as_str(), theme.font_size_small).0).fold(0.0f32, f32::max);
        let value_x = bounds.x + label_w + theme.gap_standard * 2.0;
        let row_h = theme.control_height;
        for (index, entry) in node.entries.iter().enumerate() {
            let y = bounds.y + index as f32 * row_h;
            draw_text_on(draw, atlas, entry.label.as_str(), bounds.x, y + (row_h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text_muted);
            draw_text_on(draw, atlas, &entry.value, value_x, y + (row_h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text);
        }
    }

    fn paint_slider(node: &UiSliderNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, draw: &mut DrawList) {
        let track_y = bounds.y + bounds.h * 0.5;
        draw.push_rounded([bounds.x, track_y - 2.0, bounds.w, 4.0], theme.separator, 2.0);
        let range = (node.max - node.min).max(f64::EPSILON);
        let t = ((node.value - node.min) / range).clamp(0.0, 1.0);
        let knob_x = bounds.x + bounds.w * t as f32;
        draw.push_rounded([knob_x - 6.0, track_y - 6.0, 12.0, 12.0], theme.accent, 6.0);
        // 📏️ `ui-interpreter.tsx`'s `case "slider"` is the ground truth for the unit-label readout
        // (`WidgetNode::Slider` has no `unit` field at all, so there's nothing to port from `widgets`
        // here either): `{control.value} {control.unit}`, muted small text, trailing the track. React
        // lays it out as a sibling flex item outside the slider's own box; `paint` has no extra layout
        // space to claim (that's `flex`'s call, out of scope here), so this right-aligns inside the
        // slider's own bounds as the closest in-bounds approximation.
        if let Some(unit) = &node.unit {
            let text = format!("{} {unit}", node.value);
            let (w, _) = atlas.measure_text(&text, theme.font_size_small);
            draw_text_on(draw, atlas, &text, bounds.x + bounds.w - w, track_y + theme.font_size_small * 0.5 - 2.0, theme.font_size_small, theme.text_muted);
        }
    }

    fn paint_number_stepper(node: &UiNumberStepperNode, bounds: Rect, flags: NodeFlags, theme: &Theme, atlas: &mut FontAtlas, draw: &mut DrawList) {
        let seg = bounds.w / 3.0;
        let minus = Rect::new(bounds.x, bounds.y, seg, bounds.h);
        let center = Rect::new(bounds.x + seg, bounds.y, seg, bounds.h);
        let plus = Rect::new(bounds.x + seg * 2.0, bounds.y, seg, bounds.h);
        let hair = theme.stroke_hairline;
        // 🖱️ `Stepper`'s minus/plus `<Button variant="outline">`s (`ui/js/react/index.tsx`) each carry
        // their own `hover:bg-muted`/`focus-visible:bg-muted`/`formControlFocusBorderClass`; this retained
        // model has no per-segment `NodeId` (the whole stepper is one hit-testable node — see this
        // function's caller, `paint_control`'s doc comment, for the same one-`NodeId`-per-composite
        // caveat), so the closest in-model approximation tints the shared outer bg/border for hover/focus,
        // which the nested center-segment border below then repaints back to `input_bg`/`border_normal`
        // (the center "value" segment isn't a button — it never carries React's own hover/focus fill).
        let hovered = flags.contains(NodeFlags::HOVERED);
        let focused = flags.contains(NodeFlags::FOCUSED);
        let outer_bg = if hovered { theme.button_hover } else { theme.input_bg };
        let outer_border = if focused { theme.border_emphasized } else { theme.border_normal };
        push_control_border(draw, bounds, theme, outer_border, outer_bg);
        draw.push_solid([bounds.x + seg, bounds.y, hair, bounds.h], theme.border_normal);
        draw.push_solid([bounds.x + seg * 2.0, bounds.y, hair, bounds.h], theme.border_normal);
        // 🔲️ `widgets::render_number_stepper` renders the center value segment through a full
        // `render_input` call, which nests its own `push_control_border` box around the value —
        // `golden_number_stepper_known_gap`'s doc comment measured this as the exact 14-vs-19-instance
        // divergence (the missing nested border box). Ported verbatim here to close that gap.
        push_control_border(draw, center, theme, theme.border_normal, theme.input_bg);
        draw_text_on(draw, atlas, "−", minus.x + seg * 0.5 - 4.0, minus.y + 18.0, theme.font_size_body, theme.text);
        // 🔀️ `uniform: false` means the selection's values disagree (`ui-interpreter.tsx`'s
        // `case "numberStepper"`: `value: control.uniform ? control.value : undefined, mixed: !control.uniform`
        // fed into `<Stepper mixed>`, which shows `mixedLabel` — `UI_INSPECTOR_MIXED_PLACEHOLDER`'s Rust
        // side of that same string) instead of a formatted number. `widgets::render_number_stepper`
        // ignores `uniform` entirely (both branches of its `if uniform {..} else {..}` format the same
        // way — a `widgets`-side gap this doesn't port from, since there's nothing correct to port).
        let (text, text_color) = if node.uniform { (format!("{:.3}", node.value), theme.text) } else { (UI_INSPECTOR_MIXED_PLACEHOLDER.to_string(), theme.text_muted) };
        draw_text_on(draw, atlas, &text, center.x + 8.0, center.y + (center.h + theme.font_size_body) * 0.5 - 2.0, theme.font_size_body, text_color);
        draw_text_on(draw, atlas, "+", plus.x + seg * 0.5 - 4.0, plus.y + 18.0, theme.font_size_body, theme.text);
    }

    fn paint_ring(node: &UiRingNode, bounds: Rect, theme: &Theme, draw: &mut DrawList) {
        let cx = bounds.x + bounds.w * 0.5;
        let cy = bounds.y + bounds.h * 0.5;
        let radius = bounds.w.min(bounds.h) * 0.4;
        let segments = 48usize;
        let mut points = Vec::with_capacity(segments + 1);
        for i in 0..=segments {
            let angle = std::f32::consts::TAU * i as f32 / segments as f32;
            points.push([cx + angle.cos() * radius, cy + angle.sin() * radius]);
        }
        for window in points.windows(2) {
            draw.push_line(window[0][0], window[0][1], window[1][0], window[1][1], theme.separator, 2.0);
        }
        let disabled = node.presence.state == UiState::Disabled;
        let knob_angle = std::f32::consts::TAU * node.t as f32;
        let kx = cx + knob_angle.cos() * radius;
        let ky = cy + knob_angle.sin() * radius;
        let accent = if disabled { theme.text_muted } else { theme.accent };
        draw.push_rounded([kx - 6.0, ky - 6.0, 12.0, 12.0], accent, 6.0);
    }

    fn paint_icon_select(node: &UiIconSelectNode, bounds: Rect, flags: NodeFlags, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList) {
        let hovered = flags.contains(NodeFlags::HOVERED);
        // 🎯️ Same border-swap-on-focus convention as `paint_button`/`paint_select`/`paint_toggle` — the
        // real `IconSelector` (`ui/js/react/index.tsx`) nests a `Select` for its mode picker, which
        // inherits `formControlFocusBorderClass` the same way.
        let focused = flags.contains(NodeFlags::FOCUSED);
        let border = if focused { theme.border_emphasized } else { theme.border_normal };
        push_control_border(draw, bounds, theme, border, chrome_item_bg(theme, false, hovered));
        let content_x = bounds.x + theme.padding_standard;
        let has_icon = icons.and_then(|icons| icons.icon_uv(&node.value)).is_some();
        if let (true, Some(icons)) = (has_icon, icons) {
            push_icon(draw, icons, &node.value, content_x, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, theme.text_element);
        } else {
            draw_text_on(draw, atlas, &node.value, content_x, bounds.y + (bounds.h + theme.font_size_body) * 0.5 - 2.0, theme.font_size_body, theme.text);
        }
    }

    /// 📝️ A `Field`'s label (+ required marker) and description/error text; its `child` control is a
    /// retained child painted separately by `paint_stack`. Layout intent ported from `ui/js/react/index.tsx`'s
    /// `Field` component (`widgets::render_widget`'s `WidgetNode::Field` arm only draws the bare label —
    /// no description/required/error at all — so those three are an independent port from the React
    /// reference, not from `widgets`): label (+ `*` required marker in `theme.error`) on the first line,
    /// description muted-small below it, error (in `theme.error`) below that. `reconcile`/`flex` don't
    /// yet reserve the child control's layout slot below this text (see `golden_field_known_gap`'s doc
    /// comment — a documented `flex` gap, out of scope here), so these lines are positioned relative to
    /// `bounds.y` only; they'll land correctly once that flex gap is fixed.
    fn paint_field(node: &UiFieldNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, draw: &mut DrawList) {
        let label_size = theme.font_size_small;
        draw_text_on(draw, atlas, node.label.as_str(), bounds.x, bounds.y + label_size, label_size, theme.text_muted);
        let mut y = bounds.y + label_size;
        if node.required.unwrap_or(false) {
            let (label_w, _) = atlas.measure_text(node.label.as_str(), label_size);
            draw_text_on(draw, atlas, "*", bounds.x + label_w + 2.0, y, label_size, theme.error);
        }
        if let Some(description) = &node.description {
            y += label_size + theme.gap_standard * 0.5;
            draw_text_on(draw, atlas, description, bounds.x, y, label_size, theme.text_muted);
        }
        if let Some(error) = &node.error {
            y += label_size + theme.gap_standard * 0.5;
            draw_text_on(draw, atlas, error, bounds.x, y, label_size, theme.error);
        }
    }

    /// 📂️ A `Section`'s header chevron+label; its `children` are retained children painted separately by
    /// `paint_stack`. Collapsed state still reads `default_open` directly — no `WidgetState`-backed
    /// toggle persistence exists for `Section` yet (unlike `Select`'s popup open/closed state and
    /// `Input`'s live edit buffer, both wired by now — see `WidgetState`'s own doc comment).
    fn paint_section(node: &UiSectionNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList) {
        let Some(label) = &node.label else { return };
        let collapsed = !node.default_open.unwrap_or(true);
        let chevron = if collapsed { "chevron-right" } else { "chevron-down" };
        if let Some(icons) = icons {
            push_icon(draw, icons, chevron, bounds.x, bounds.y + (PANEL_HEADER - ICON_TINY) * 0.5, ICON_TINY, theme.text_element);
        }
        draw_text_on(draw, atlas, label.as_str(), bounds.x + TREE_TOGGLE_WIDTH + theme.gap_standard, bounds.y + (PANEL_HEADER + theme.font_size_body) * 0.5 - 2.0, theme.font_size_body, theme.text);
    }

    /** @emoji 🌿️ Same header chrome as {@link paint_section} (chevron + label), for a `Group`'s always-
     * present `label` — used when a nested subtree (e.g. `Origin`) is painted directly in the native
     * retained tree rather than pre-expanded into `UiTreeItemNode.items`. */
    fn paint_group(node: &UiGroupNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList) {
        let collapsed = !node.default_open.unwrap_or(true);
        let chevron = if collapsed { "chevron-right" } else { "chevron-down" };
        if let Some(icons) = icons {
            push_icon(draw, icons, chevron, bounds.x, bounds.y + (PANEL_HEADER - ICON_TINY) * 0.5, ICON_TINY, theme.text_element);
        }
        draw_text_on(draw, atlas, node.label.as_str(), bounds.x + TREE_TOGGLE_WIDTH + theme.gap_standard, bounds.y + (PANEL_HEADER + theme.font_size_body) * 0.5 - 2.0, theme.font_size_body, theme.text);
    }

    fn paint_tree_widget(node: &UiTreeNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList) {
        draw.push_scissor(bounds);
        let mut y = bounds.y;
        for section in &node.sections {
            if let Some(label) = &section.label {
                // 🗂️ `widgets::render_tree_section_header` draws a folder icon before the label and
                // dims the label to `text_muted` only while collapsed (`text_element` otherwise) —
                // ported here; previously this always used `text_muted` regardless of collapsed state.
                let collapsed = !section.default_open.unwrap_or(true);
                let text_color = if collapsed { theme.text_muted } else { theme.text_element };
                let label_x = bounds.x + TREE_TOGGLE_WIDTH + theme.gap_standard;
                if let Some(icons) = icons {
                    push_icon(draw, icons, "folder", label_x, y + (PANEL_HEADER - TREE_ICON_SIZE) * 0.5, TREE_ICON_SIZE, text_color);
                }
                draw_text_on(draw, atlas, label.as_str(), label_x + TREE_ICON_SIZE + theme.gap_standard, y + (PANEL_HEADER + theme.font_size_small) * 0.5 - 2.0, theme.font_size_small, text_color);
                y += PANEL_HEADER;
            }
            for item in &section.items {
                y = paint_tree_item(item, bounds.x, bounds.w, y, 1, node, theme, atlas, icons, draw, &[]);
            }
        }
        draw.pop_scissor();
        // 🧭️ Status/selected/introducing rings for the whole `Tree` are drawn once, centrally, by
        // `paint_node`'s shared `presence_overlay` — not duplicated here.
    }

    /// 🌳️ Recursive row painter for one `Tree` item (and, if expanded, its nested `items`). Ports every
    /// piece of `widgets::render_tree_item`'s visual structure that depends only on static retained data
    /// (ancestor guide lines, selected/highlighted text color, description text, always-visible actions,
    /// an inline `control`) — anything that depends on *live* hover/drag/focus state (row hover fill,
    /// hover-revealed actions, hover-highlighted action icons, drag guides) stays out of scope: there is
    /// no per-tree-row `NodeId`/`NodeFlags` yet (`reconcile::children_of` doesn't expand `Tree` into
    /// retained item children — see `paint_select`'s neighboring doc comment for the same root cause), so
    /// there is nowhere to read a live per-row hover/drag flag from until that reconcile expansion lands.
    #[allow(clippy::too_many_arguments, reason = "one arg per paint context resource; grouping into a struct is a T2 restructure, out of scope")]
    fn paint_tree_item(item: &UiTreeItemNode, x: f32, width: f32, y: f32, depth: u32, tree_node: &UiTreeNode, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList, is_last_at_level: &[bool]) -> f32 {
        if !item.presence.visible() {
            return y;
        }
        let row = Rect::new(x, y, width, TREE_ROW_HEIGHT);
        let selected = item.presence.selected;
        let previewed = item.presence.state == UiState::Previewed;
        let dimmed = item.dimmed.unwrap_or(false) || item.presence.state == UiState::Disabled;
        if selected {
            draw.push_rounded([row.x, row.y, row.w, row.h], theme.selected, theme.border_radius);
        } else if previewed {
            draw.push_rounded([row.x, row.y, row.w, row.h], theme.row_hover, theme.border_radius);
        }
        let ring_color = if selected { theme.selected } else { theme.border_normal };
        match item.presence.status {
            UiStatus::Loading => paint_loading_border(draw, row, ring_color, theme),
            UiStatus::Waiting => paint_waiting_border(draw, row, ring_color, theme),
            UiStatus::Finished => draw.push_finished_border([row.x, row.y, row.w, row.h], ring_color, theme.border_radius, theme.stroke_hairline),
            UiStatus::Idle => {}
        }
        if item.presence.state == UiState::Introducing || item.presence.state == UiState::Celebrating {
            draw.push_introducing_border([row.x, row.y, row.w, row.h], theme.accent, theme.border_radius, theme.stroke_hairline);
        }
        paint_tree_guides(draw, x, row.y, row.h, depth, is_last_at_level, theme);
        let indent = x + (depth - 1) as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH;
        let expandable = item.items.as_ref().is_some_and(|items| !items.is_empty());
        if expandable {
            if let Some(icons) = icons {
                let chevron = if item.default_open.unwrap_or(false) { "chevron-down" } else { "chevron-right" };
                push_icon(draw, icons, chevron, indent - TREE_TOGGLE_WIDTH, row.y + (TREE_ROW_HEIGHT - ICON_TINY) * 0.5, ICON_TINY, theme.text_element);
            }
        }
        // 🎨️ `widgets::render_tree_item`'s `text_color`: selected/previewed rows use `active_foreground`
        // for both icon tint and label (previously this always used `text_element`/`theme.text`);
        // `dimmed` (the eye-toggle "hidden in scene" domain flag, or `presence.state == Disabled`) halves
        // its alpha without skipping the row — it stays visible and clickable to un-hide/re-enable.
        let text_color = if selected || previewed { theme.active_foreground } else { theme.text_element };
        let text_color = if dimmed { text_color.with_alpha(text_color.a * 0.5) } else { text_color };
        if let (Some(icons), Some(icon_id)) = (icons, item.icon_id) {
            push_icon(draw, icons, icon_id.as_str(), indent, row.y + (TREE_ROW_HEIGHT - TREE_ICON_SIZE) * 0.5, TREE_ICON_SIZE, text_color);
        }
        let label_x = indent + if item.icon_id.is_some() { TREE_ICON_SIZE + theme.gap_standard } else { 0.0 };
        draw_text_on(draw, atlas, item.label.as_str(), label_x, row.y + (TREE_ROW_HEIGHT + theme.font_size_body) * 0.5 - 2.0, theme.font_size_body, text_color);
        if let Some(description) = &item.description {
            let (label_w, _) = atlas.measure_text(item.label.as_str(), theme.font_size_body);
            draw_text_on(draw, atlas, description, label_x + label_w + theme.gap_standard, row.y + (TREE_ROW_HEIGHT + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text_muted);
        }
        let mut actions_x = row.x + row.w - theme.gap_standard;
        if let Some(icons) = icons {
            for action in item.actions.iter().flatten().rev() {
                if action.placement() == UiTreeActionPlacement::Menu {
                    continue;
                }
                actions_x -= TREE_ICON_SIZE + theme.padding_standard;
                push_icon(draw, icons, action.icon_id.as_str(), actions_x, row.y + (TREE_ROW_HEIGHT - TREE_ICON_SIZE) * 0.5, TREE_ICON_SIZE, theme.text_element);
            }
        }
        // 🎛️ An inline per-row control (e.g. a small toggle/select embedded in a tree row), static data
        // already present on `UiTreeItemNode` that the old paint pass never rendered at all.
        if let Some(control) = &item.control {
            let control_w = 120.0;
            let control_rect = Rect::new(row.x + row.w - control_w - theme.gap_standard, row.y + (row.h - theme.control_height) * 0.5, control_w, theme.control_height);
            paint_control(control, control_rect, theme, atlas, icons, draw);
        }
        let mut next_y = y + TREE_ROW_HEIGHT;
        if expandable && item.default_open.unwrap_or(false) {
            for (index, child) in item.items.as_ref().unwrap().iter().enumerate() {
                let mut child_is_last = is_last_at_level.to_vec();
                child_is_last.push(index + 1 == item.items.as_ref().unwrap().len());
                next_y = paint_tree_item(child, x, width, next_y, depth + 1, tree_node, theme, atlas, icons, draw, &child_is_last);
            }
        }
        next_y
    }

    /// 📏️ Ancestor connector lines for one tree row, ported from `widgets::tree_draw_guides` — adjusted
    /// for `paint_tree_item`'s `depth` starting at `1` for top-level items (`widgets`' `render_tree_item`
    /// starts its own `depth` at `0`), so every `widgets_depth` reference there is this function's
    /// `depth - 1`.
    fn paint_tree_guides(draw: &mut DrawList, row_x: f32, row_y: f32, row_h: f32, depth: u32, is_last_at_level: &[bool], theme: &Theme) {
        let hair = theme.stroke_hairline.max(1.0);
        let guide_color = theme.border_normal;
        for level in 0..depth.saturating_sub(1) {
            if is_last_at_level.get(level as usize).copied().unwrap_or(false) {
                continue;
            }
            let x = row_x + level as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH * 0.5;
            draw.push_solid([x, row_y, hair, row_h], guide_color);
        }
        if depth > 1 {
            let x = row_x + (depth - 2) as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH * 0.5;
            let mid_y = row_y + row_h * 0.5;
            draw.push_solid([x, row_y, hair, mid_y - row_y], guide_color);
            draw.push_solid([x, mid_y, TREE_INDENT_PER_LEVEL * 0.5, hair], guide_color);
        }
    }

    /// 🎛️ Adapter from a `TreeItem`'s inline `UiControlNode` payload (a narrower enum than `UiNode` —
    /// see `component::ui::UiControlNode`'s own doc comment) to the matching `paint_*` function; mirrors
    /// `paint_node`'s `UiNode` dispatch table one level down. No per-control `NodeId` exists for an inline
    /// tree-row control yet, so it always paints at rest (`NodeFlags::empty()`) — same interactive-state
    /// caveat as the rest of this function's caller.
    fn paint_control(control: &UiControlNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, draw: &mut DrawList) {
        let flags = NodeFlags::empty();
        match control {
            UiControlNode::Button(node) => paint_button(node, bounds, flags, theme, atlas, icons, draw),
            UiControlNode::Input(node) => paint_input(node, None, bounds, flags, theme, atlas, draw),
            UiControlNode::Select(node) => paint_select(node, bounds, flags, false, None, theme, atlas, icons, draw),
            UiControlNode::Toggle(node) => paint_toggle(node, bounds, flags, theme, atlas, icons, draw),
            UiControlNode::KeyValue(node) => paint_key_value(node, bounds, theme, atlas, draw),
            UiControlNode::Slider(node) => paint_slider(node, bounds, theme, atlas, draw),
            UiControlNode::NumberStepper(node) => paint_number_stepper(node, bounds, flags, theme, atlas, draw),
            UiControlNode::Ring(node) => paint_ring(node, bounds, theme, draw),
            UiControlNode::IconSelect(node) => paint_icon_select(node, bounds, flags, theme, atlas, icons, draw),
        }
    }

    /// 🖼️ `paint_node`'s caller (`paint_tree`) only reaches this when `has_scene_host` is `false` this
    /// tick — a real `scene_slots::SceneHost` paints the actual image content instead (see `paint_node`'s
    /// `UiNode::Image` arm). No host-side texture-upload queue exists in `ui_wgpu` itself even so (that
    /// lives in the renderer's `program_bridge`/`engine_canvas`, outside this crate's scope); paints a
    /// raster quad keyed by `src` on the chance a caller-owned `RasterTextureStore` already has that key
    /// uploaded, falling back to `alt` text when there's nothing to show yet.
    fn paint_image(node: &UiImageNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, draw: &mut DrawList) {
        if node.src.is_empty() {
            if let Some(alt) = &node.alt {
                draw_text_on(draw, atlas, alt.as_str(), bounds.x + 4.0, bounds.y + 16.0, theme.font_size_small, theme.text_muted);
            }
            return;
        }
        draw.push_raster_quad(&node.src, [bounds.x, bounds.y, bounds.w, bounds.h], [0.0, 0.0, 1.0, 1.0], 1.0);
    }

    /// 🎬️ `paint_node`'s caller only reaches this when `has_scene_host` is `false` this tick — with a
    /// real `scene_slots::SceneHost` registered, `engine::Ui::frame`'s `collect_scene_slots` loop paints
    /// the actual scene surface (canvas2d/world3d/node-graph/…) into this same rect right after this
    /// pass returns (see `paint_node`'s `UiNode::ComponentScene` arm), so this placeholder chrome is
    /// purely the no-host fallback — "there's something visible in that rect" rather than nothing.
    fn paint_component_scene(node: &UiComponentSceneNode, bounds: Rect, theme: &Theme, draw: &mut DrawList) {
        let _ = &node.surface_id;
        push_control_border(draw, bounds, theme, theme.border_normal, theme.panel);
    }

    /// 🧩️ Same placeholder-chrome treatment as `paint_component_scene`: the plugin body itself is a host
    /// concern (`program_bridge`), out of scope here; label the slot with its `body_key` for now.
    fn paint_external_slot(node: &UiExternalSlotNode, bounds: Rect, theme: &Theme, atlas: &mut FontAtlas, draw: &mut DrawList) {
        push_control_border(draw, bounds, theme, theme.border_normal, theme.panel);
        draw_text_on(draw, atlas, &node.body_key, bounds.x + theme.padding_standard, bounds.y + (bounds.h + theme.font_size_small) * 0.5 - 2.0, theme.font_size_small, theme.text_muted);
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::component::layout::ActionDescriptor;
        use crate::component::ui::{UiFieldNode, UiNumberStepperNode, UiSectionNode, UiSeparatorNode, UiSliderNode, UiStackNode, UiTreeItemAction, UiTreeSectionNode};
        use crate::draw::{KIND_GLYPH, KIND_LOADING_BORDER, KIND_SOLID, KIND_WAITING_BORDER};
        use crate::flex::LayoutEngine;
        use crate::tree::EditState;

        fn action() -> ActionDescriptor {
            ActionDescriptor { controller_id: "ctrl".into(), action: "go".into(), args: None }
        }

        fn text(value: &str) -> UiNode {
            UiNode::Text(UiTextNode { value: value.into(), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })
        }

        fn stack(children: Vec<UiNode>) -> UiNode {
            UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: Some("none".into()), padding: Some("none".into()), id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children, menu: None })
        }

        fn loading_button(id: &str) -> UiNode {
            UiNode::Button(UiButtonNode {
                id: Some(id.into()),
                icon_id: IconName::CircleDot,
                label: id.into(),
                action: ActionDescriptor { controller_id: "ctrl".into(), action: "go".into(), args: None },
                style: None,
                presence: UiPresence::status(UiStatus::Loading),
                menu: None,
            })
        }

        fn waiting_button(id: &str) -> UiNode {
            UiNode::Button(UiButtonNode {
                id: Some(id.into()),
                icon_id: IconName::CircleDot,
                label: id.into(),
                action: ActionDescriptor { controller_id: "ctrl".into(), action: "go".into(), args: None },
                style: None,
                presence: UiPresence::status(UiStatus::Waiting),
                menu: None,
            })
        }

        fn setup(ui: &UiNode) -> (UiTree, NodeId, Theme, FontAtlas) {
            let mut tree = UiTree::new();
            tree.apply_tree(ui);
            let root = tree.root.unwrap();
            let theme = Theme::default();
            let mut atlas = FontAtlas::builtin();
            let mut engine = LayoutEngine::new();
            engine.compute(&mut tree, root, &mut atlas, &theme, 400.0, 400.0);
            (tree, root, theme, atlas)
        }

        #[test]
        fn painting_a_text_node_emits_glyph_instances() {
            let (mut tree, root, theme, mut atlas) = setup(&text("hi"));
            let mut draw = DrawList::default();

            paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

            let total_instances: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
            assert!(total_instances > 0, "text node should emit at least one glyph instance");
        }

        #[test]
        fn painting_a_stack_recurses_into_every_child() {
            let ui = stack(vec![text("a"), UiNode::Separator(UiSeparatorNode { presence: UiPresence::default(), menu: None }), text("b")]);
            let (mut tree, root, theme, mut atlas) = setup(&ui);
            let mut draw = DrawList::default();

            paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

            let total_instances: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
            let total_vectors: usize = draw.layers.iter().map(|layer| layer.vector_vertices.len()).sum();
            assert!(total_instances > 0, "text children should have emitted glyphs");
            assert!(total_vectors > 0, "separator child should have emitted a line");
        }

        #[test]
        fn paint_tree_clears_dirty_paint_but_leaves_layout_dirt_flags_untouched() {
            let (mut tree, root, theme, mut atlas) = setup(&text("hi"));
            assert!(tree.node(root).unwrap().flags.contains(NodeFlags::DIRTY_PAINT));
            let mut draw = DrawList::default();

            paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

            let after_first = tree.node(root).unwrap().flags;
            assert!(!after_first.contains(NodeFlags::DIRTY_PAINT));
            assert!(!after_first.contains(NodeFlags::DIRTY_LAYOUT), "paint must not touch DIRTY_LAYOUT, that's flex's job");
            assert!(!after_first.contains(NodeFlags::SUBTREE_DIRTY), "flex::compute already cleared SUBTREE_DIRTY before paint ran");

            // Second call must be a no-operation w.r.t. these flags — repeat of the M3 SUBTREE_DIRTY bug class
            // (calling twice shouldn't set or double-clear something it shouldn't).
            paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);
            let after_second = tree.node(root).unwrap().flags;
            assert_eq!(after_first, after_second);
        }

        #[test]
        fn painting_a_loading_button_emits_a_loading_border_instance() {
            let (mut tree, root, theme, mut atlas) = setup(&loading_button("save"));
            let mut draw = DrawList::default();

            paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

            let has_loading_border = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.params[2] - KIND_LOADING_BORDER).abs() < 0.01);
            assert!(has_loading_border, "loading button should emit a KIND_LOADING_BORDER instance");
        }

        #[test]
        fn painting_a_waiting_button_emits_a_waiting_border_instance() {
            let (mut tree, root, theme, mut atlas) = setup(&waiting_button("save"));
            let mut draw = DrawList::default();

            paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

            let has_waiting_border = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.params[2] - KIND_WAITING_BORDER).abs() < 0.01);
            assert!(has_waiting_border, "waiting button should emit a KIND_WAITING_BORDER instance");
        }

        // 🚫️ `painting_a_loading_and_waiting_button_prefers_the_loading_border` deleted: `status` is now a
        // single `UiStatus` enum, so "loading and waiting both set" is unrepresentable — which is the point.

        //#region 🔖️FidelityFixes
        // 🩹️ One test per fidelity gap this pass closed (see `.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w1c-paint-parity.md`),
        // additive to the pre-existing tests above.

        fn button(id: &str, disabled: bool) -> UiNode {
            UiNode::Button(UiButtonNode { id: Some(id.into()), icon_id: IconName::CircleDot, label: id.into(), action: action(), style: None, presence: UiPresence::disabled_if(disabled), menu: None })
        }

        #[test]
        fn painting_a_disabled_button_dims_its_border_alpha() {
            let (mut tree, root, theme, mut atlas) = setup(&button("btn", true));
            let mut draw = DrawList::default();

            paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

            let dimmed = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.color[3] - theme.border_normal.a * 0.5).abs() < 0.01);
            assert!(dimmed, "a disabled button should paint its border at half alpha");
        }

        fn loading_section(id: &str) -> UiNode {
            UiNode::Section(UiSectionNode { id: id.into(), label: Some("Sec".into()), default_open: Some(true), presence: UiPresence::status(UiStatus::Loading), children: vec![text("child")], menu: None })
        }

        #[test]
        fn painting_a_loading_section_emits_a_loading_border_instance() {
            let (mut tree, root, theme, mut atlas) = setup(&loading_section("sec"));
            let mut draw = DrawList::default();

            paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

            let has_loading_border = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.params[2] - KIND_LOADING_BORDER).abs() < 0.01);
            assert!(has_loading_border, "a loading section should emit a KIND_LOADING_BORDER instance");
        }

        fn waiting_section(id: &str) -> UiNode {
            UiNode::Section(UiSectionNode { id: id.into(), label: Some("Sec".into()), default_open: Some(true), presence: UiPresence::status(UiStatus::Waiting), children: vec![text("child")], menu: None })
        }

        #[test]
        fn painting_a_waiting_section_emits_a_waiting_border_instance() {
            let (mut tree, root, theme, mut atlas) = setup(&waiting_section("sec"));
            let mut draw = DrawList::default();

            paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

            let has_waiting_border = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.params[2] - KIND_WAITING_BORDER).abs() < 0.01);
            assert!(has_waiting_border, "a waiting section should emit a KIND_WAITING_BORDER instance");
        }

        fn loading_stack(children: Vec<UiNode>) -> UiNode {
            UiNode::Stack(UiStackNode {
                direction: "vertical".into(),
                gap: Some("none".into()),
                padding: Some("none".into()),
                id: None,
                presence: UiPresence::status(UiStatus::Loading),
                activate: None,
                drop_action: None,
                drop_overlay: None,
                children,
                menu: None,
            })
        }

        #[test]
        fn painting_a_loading_stack_emits_a_loading_border_instance() {
            let (mut tree, root, theme, mut atlas) = setup(&loading_stack(vec![text("a")]));
            let mut draw = DrawList::default();

            paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

            let has_loading_border = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.params[2] - KIND_LOADING_BORDER).abs() < 0.01);
            assert!(has_loading_border, "a loading stack should emit a KIND_LOADING_BORDER instance");
        }

        fn waiting_stack(children: Vec<UiNode>) -> UiNode {
            UiNode::Stack(UiStackNode {
                direction: "vertical".into(),
                gap: Some("none".into()),
                padding: Some("none".into()),
                id: None,
                presence: UiPresence::status(UiStatus::Waiting),
                activate: None,
                drop_action: None,
                drop_overlay: None,
                children,
                menu: None,
            })
        }

        #[test]
        fn painting_a_waiting_stack_emits_a_waiting_border_instance() {
            let (mut tree, root, theme, mut atlas) = setup(&waiting_stack(vec![text("a")]));
            let mut draw = DrawList::default();

            paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

            let has_waiting_border = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.params[2] - KIND_WAITING_BORDER).abs() < 0.01);
            assert!(has_waiting_border, "a waiting stack should emit a KIND_WAITING_BORDER instance");
        }

        fn loading_tree() -> UiNode {
            UiNode::Tree(UiTreeNode {
                sections: vec![UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![UiTreeItemNode::base("i1", "Item")] }],
                presence: UiPresence::status(UiStatus::Loading),
                selected_ids: None,
                highlighted_ids: None,
                selection_change: None,
                drop_action: None,
                menu: None,
            })
        }

        fn waiting_tree() -> UiNode {
            UiNode::Tree(UiTreeNode {
                sections: vec![UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![UiTreeItemNode::base("i1", "Item")] }],
                presence: UiPresence::status(UiStatus::Waiting),
                selected_ids: None,
                highlighted_ids: None,
                selection_change: None,
                drop_action: None,
                menu: None,
            })
        }

        #[test]
        fn painting_a_loading_tree_emits_a_loading_border_instance() {
            let (mut tree, root, theme, mut atlas) = setup(&loading_tree());
            let mut draw = DrawList::default();

            paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

            let has_loading_border = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.params[2] - KIND_LOADING_BORDER).abs() < 0.01);
            assert!(has_loading_border, "a loading tree should emit a KIND_LOADING_BORDER instance");
        }

        #[test]
        fn painting_a_waiting_tree_emits_a_waiting_border_instance() {
            let (mut tree, root, theme, mut atlas) = setup(&waiting_tree());
            let mut draw = DrawList::default();

            paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

            let has_waiting_border = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| (instance.params[2] - KIND_WAITING_BORDER).abs() < 0.01);
            assert!(has_waiting_border, "a waiting tree should emit a KIND_WAITING_BORDER instance");
        }

        fn stepper(id: &str, value: f64, uniform: bool) -> UiNode {
            UiNode::NumberStepper(UiNumberStepperNode { id: id.into(), value, step: 1.0, uniform, on_absolute: action(), on_delta: action(), presence: UiPresence::default(), menu: None })
        }

        #[test]
        fn painting_a_uniform_number_stepper_nests_a_border_around_its_center_value() {
            let (mut tree, root, theme, mut atlas) = setup(&stepper("ns", 2.0, true));
            let mut draw = DrawList::default();

            paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

            let total: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
            // Outer control border (bg + 4 edges = 5) + 2 divider lines + nested center-value border
            // (bg + 4 edges = 5) + minus/"2.000"/plus glyphs (1 + 5 + 1 = 7) = 19 — the exact instance
            // count `golden_number_stepper_known_gap`'s doc comment measured `widgets::render_number_stepper`
            // emitting (vs this region's pre-fix 14), now matched by porting the nested border.
            assert_eq!(total, 19, "uniform NumberStepper should now nest a border around its center value, matching widgets' 19-instance output");
        }

        #[test]
        fn painting_a_mixed_number_stepper_shows_the_mixed_placeholder_in_muted_color() {
            let (mut tree, root, theme, mut atlas) = setup(&stepper("ns", 2.0, false));
            let mut draw = DrawList::default();

            paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

            let has_muted_glyph = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| {
                (instance.params[2] - KIND_GLYPH).abs() < 0.01 && (instance.color[0] - theme.text_muted.r).abs() < 0.001 && (instance.color[1] - theme.text_muted.g).abs() < 0.001 && (instance.color[2] - theme.text_muted.b).abs() < 0.001
            });
            assert!(has_muted_glyph, "a non-uniform (mixed) NumberStepper should paint its center value's glyphs in theme.text_muted (the 'Mixed' placeholder)");
        }

        fn slider(id: &str, unit: Option<&str>) -> UiNode {
            UiNode::Slider(UiSliderNode { id: id.into(), value: 0.5, min: 0.0, max: 1.0, step: 0.01, unit: unit.map(String::from), on_change: action(), presence: UiPresence::default(), menu: None })
        }

        #[test]
        fn painting_a_slider_with_a_unit_emits_extra_glyphs_for_the_readout() {
            let (mut plain_tree, plain_root, theme, mut plain_atlas) = setup(&slider("sl", None));
            let mut plain_draw = DrawList::default();
            paint_tree(&mut plain_tree, plain_root, &theme, &mut plain_atlas, None, false, &mut plain_draw);

            let (mut unit_tree, unit_root, theme2, mut unit_atlas) = setup(&slider("sl", Some("mm")));
            let mut unit_draw = DrawList::default();
            paint_tree(&mut unit_tree, unit_root, &theme2, &mut unit_atlas, None, false, &mut unit_draw);

            let plain_total: usize = plain_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
            let unit_total: usize = unit_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
            assert!(unit_total > plain_total, "a slider with a unit should paint extra glyphs for its value+unit readout");
        }

        fn field(description: Option<&str>, required: bool, error: Option<&str>) -> UiNode {
            UiNode::Field(UiFieldNode {
                id: "f".into(),
                label: "Label".into(),
                description: description.map(String::from),
                required: Some(required),
                error: error.map(String::from),
                child: Box::new(text("child")),
                presence: UiPresence::default(),
                menu: None,
            })
        }

        #[test]
        fn painting_a_field_with_description_required_and_error_emits_extra_glyphs() {
            let (mut bare_tree, bare_root, theme, mut bare_atlas) = setup(&field(None, false, None));
            let mut bare_draw = DrawList::default();
            paint_tree(&mut bare_tree, bare_root, &theme, &mut bare_atlas, None, false, &mut bare_draw);

            let (mut rich_tree, rich_root, theme2, mut rich_atlas) = setup(&field(Some("desc"), true, Some("bad")));
            let mut rich_draw = DrawList::default();
            paint_tree(&mut rich_tree, rich_root, &theme2, &mut rich_atlas, None, false, &mut rich_draw);

            let bare_total: usize = bare_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
            let rich_total: usize = rich_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
            assert!(rich_total > bare_total, "description/required-marker/error should each add glyph instances beyond the bare label");
        }

        fn tree_with_item_description() -> UiNode {
            let mut item = UiTreeItemNode::base("i1", "Item One");
            item.description = Some("desc".into());
            item.actions = Some(vec![UiTreeItemAction { icon_id: IconName::Sparkles, label: None, action: action(), placement: None }]);
            UiNode::Tree(UiTreeNode {
                sections: vec![UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] }],
                presence: UiPresence::default(),
                selected_ids: None,
                highlighted_ids: None,
                selection_change: None,
                drop_action: None,
                menu: None,
            })
        }

        fn tree_with_bare_item() -> UiNode {
            UiNode::Tree(UiTreeNode {
                sections: vec![UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![UiTreeItemNode::base("i1", "Item One")] }],
                presence: UiPresence::default(),
                selected_ids: None,
                highlighted_ids: None,
                selection_change: None,
                drop_action: None,
                menu: None,
            })
        }

        #[test]
        fn painting_a_tree_item_with_description_emits_more_than_a_bare_item() {
            let (mut bare_tree, bare_root, theme, mut bare_atlas) = setup(&tree_with_bare_item());
            let mut bare_draw = DrawList::default();
            paint_tree(&mut bare_tree, bare_root, &theme, &mut bare_atlas, None, false, &mut bare_draw);

            let (mut rich_tree, rich_root, theme2, mut rich_atlas) = setup(&tree_with_item_description());
            let mut rich_draw = DrawList::default();
            paint_tree(&mut rich_tree, rich_root, &theme2, &mut rich_atlas, None, false, &mut rich_draw);

            let bare_total: usize = bare_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
            let rich_total: usize = rich_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
            assert!(rich_total > bare_total, "a tree item's description text should paint extra glyphs beyond a bare item (icons are None here, so its always-visible action doesn't add its own icon instance in this fixture)");
        }
        //#endregion 🔖️FidelityFixes

        //#region 🔖️W2InteractivityFixes
        // 🔽️🎴️🌳️ Tests for `.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY`'s W2 pass: Select popup painting
        // (`paint_select` + `sync_select_popup_rows`), Stack `activate`/`selected`/`drop_action`
        // (`paint_stack_frame` + `sync_interactive_state`'s `DROP_TARGET` sync), and Tree row real layout
        // + `DRAG_SOURCE` sync (`sync_tree_row_layout`/`sync_tree_item_layout`).

        fn select(id: &str, value: &str) -> UiNode {
            UiNode::Select(UiSelectNode {
                id: id.into(),
                value: value.into(),
                items: vec![UiSelectItem { value: "a".into(), label: "Alpha".into() }, UiSelectItem { value: "b".into(), label: "Beta".into() }],
                placeholder: None,
                on_change: action(),
                presence: UiPresence::default(),
                menu: None,
            })
        }

        #[test]
        fn painting_an_open_select_popup_emits_more_instances_than_a_closed_one_and_highlights_the_value() {
            let (mut tree, root, theme, mut atlas) = setup(&select("sel", "b"));
            let mut closed_draw = DrawList::default();
            paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut closed_draw);
            let closed_total: usize = closed_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();

            tree.node_mut(root).unwrap().state.open = true;
            tree.mark_dirty(root, NodeFlags::DIRTY_PAINT);
            let mut open_draw = DrawList::default();
            paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut open_draw);
            let open_total: usize = open_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();

            assert!(open_total > closed_total, "an open Select should paint its popup rows in addition to the closed trigger");
            // "Beta" (value "b") is the current value — its row should paint a `row_hover`-colored
            // highlight rect (a KIND_ROUNDED instance in exactly `theme.row_hover`).
            let has_selected_highlight = open_draw
                .layers
                .iter()
                .flat_map(|layer| layer.ui_instances.iter())
                .any(|instance| (instance.color[0] - theme.row_hover.r).abs() < 0.001 && (instance.color[1] - theme.row_hover.g).abs() < 0.001 && (instance.color[2] - theme.row_hover.b).abs() < 0.001);
            assert!(has_selected_highlight, "the popup row matching the Select's current value should paint a row_hover highlight");
        }

        #[test]
        fn opening_a_selects_popup_gives_its_synthesized_item_rows_real_hit_testable_layout() {
            let (mut tree, root, theme, mut atlas) = setup(&select("sel", "a"));
            tree.node_mut(root).unwrap().state.open = true;
            tree.mark_dirty(root, NodeFlags::DIRTY_PAINT);
            let mut draw = DrawList::default();

            paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

            let row_a = find_child_by_key(&tree, root, &NodeKey::Explicit("a".into())).expect("reconcile should have synthesized a retained row for item \"a\"");
            let row_b = find_child_by_key(&tree, root, &NodeKey::Explicit("b".into())).expect("reconcile should have synthesized a retained row for item \"b\"");
            let bucket_a = &tree.node(row_a).unwrap().layout;
            let bucket_b = &tree.node(row_b).unwrap().layout;
            assert!(bucket_a.width > 0.0 && bucket_a.height > 0.0, "an open Select's row should get real (non-zero) layout so events::hit_test can find it");
            assert!(bucket_b.y > bucket_a.y, "row \"b\" should be laid out below row \"a\"");
        }

        fn drop_stack(drop_action: Option<ActionDescriptor>) -> UiNode {
            UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: Some("dz".into()), presence: UiPresence::default(), activate: None, drop_action, drop_overlay: None, children: vec![text("child")], menu: None })
        }

        #[test]
        fn a_stacks_drop_target_flag_tracks_its_drop_action() {
            let (mut tree, root, theme, mut atlas) = setup(&drop_stack(Some(action())));
            let mut draw = DrawList::default();
            paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);
            assert!(tree.node(root).unwrap().flags.contains(NodeFlags::DROP_TARGET), "a Stack with a drop_action should be flagged NodeFlags::DROP_TARGET");

            let (mut plain_tree, plain_root, plain_theme, mut plain_atlas) = setup(&drop_stack(None));
            let mut plain_draw = DrawList::default();
            paint_tree(&mut plain_tree, plain_root, &plain_theme, &mut plain_atlas, None, false, &mut plain_draw);
            assert!(!plain_tree.node(plain_root).unwrap().flags.contains(NodeFlags::DROP_TARGET), "a Stack without a drop_action must not be flagged DROP_TARGET");
        }

        fn activatable_stack(selected: bool) -> UiNode {
            UiNode::Stack(UiStackNode {
                direction: "vertical".into(),
                gap: None,
                padding: None,
                id: Some("card".into()),
                presence: UiPresence::selected(selected),
                activate: Some(action()),
                drop_action: None,
                drop_overlay: None,
                children: vec![text("child")],
                menu: None,
            })
        }

        #[test]
        fn an_activatable_stack_paints_a_frame_and_a_selected_one_paints_an_extra_ring() {
            let (mut bare_tree, bare_root, theme, mut bare_atlas) = setup(&stack(vec![text("child")]));
            let mut bare_draw = DrawList::default();
            paint_tree(&mut bare_tree, bare_root, &theme, &mut bare_atlas, None, false, &mut bare_draw);
            let bare_total: usize = bare_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();

            let (mut card_tree, card_root, theme2, mut card_atlas) = setup(&activatable_stack(false));
            let mut card_draw = DrawList::default();
            paint_tree(&mut card_tree, card_root, &theme2, &mut card_atlas, None, false, &mut card_draw);
            let card_total: usize = card_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
            assert!(card_total > bare_total, "an activatable Stack should paint a bg+border frame a bare Stack doesn't");

            let (mut selected_tree, selected_root, theme3, mut selected_atlas) = setup(&activatable_stack(true));
            let mut selected_draw = DrawList::default();
            paint_tree(&mut selected_tree, selected_root, &theme3, &mut selected_atlas, None, false, &mut selected_draw);
            let selected_total: usize = selected_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
            assert!(selected_total > card_total, "a selected activatable Stack should paint an extra ring border beyond the plain activate frame");
        }

        fn tree_with_draggable_item() -> UiNode {
            let mut item = UiTreeItemNode::base("i1", "Item One");
            item.draggable = Some(true);
            UiNode::Tree(UiTreeNode {
                sections: vec![UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] }],
                presence: UiPresence::default(),
                selected_ids: None,
                highlighted_ids: None,
                selection_change: None,
                drop_action: None,
                menu: None,
            })
        }

        #[test]
        fn a_trees_draggable_item_gets_real_row_layout_and_the_drag_source_flag() {
            let (mut tree, root, theme, mut atlas) = setup(&tree_with_draggable_item());
            let mut draw = DrawList::default();

            paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

            let section = find_child_by_key(&tree, root, &NodeKey::Explicit("s1".into())).expect("reconcile should have synthesized a retained row for section \"s1\"");
            let row = find_child_by_key(&tree, section, &NodeKey::Explicit("i1".into())).expect("reconcile should have synthesized a retained row for item \"i1\"");
            let bucket = &tree.node(row).unwrap().layout;
            assert!(bucket.width > 0.0 && bucket.height > 0.0, "a Tree row should get real (non-zero) layout so events::hit_test can find it");
            assert!(tree.node(row).unwrap().flags.contains(NodeFlags::DRAG_SOURCE), "a draggable Tree item's row should be flagged NodeFlags::DRAG_SOURCE");
        }
        //#endregion 🔖️W2InteractivityFixes

        //#region 🔖️W2WidgetVisuals
        // 🖱️✍️🎯️ Tests for `.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY`'s W2 widget-visuals pass: a
        // focused `Input`'s caret/selection-highlight (`paint_input`, sourced from `tree::EditState`),
        // and the `formControlFocusBorderClass`-matching focus ring ported onto every remaining
        // focusable control kind (`Button`/`Select`/`Toggle`/`NumberStepper`/`IconSelect`, plus a
        // `NumberStepper` hover tint) that only `paint_input` had before this pass.
        fn input(id: &str, value: &str) -> UiNode {
            UiNode::Input(UiInputNode {
                id: id.into(),
                input_kind: "text".into(),
                value: value.into(),
                placeholder: None,
                commit: None,
                min: None,
                max: None,
                step: None,
                accept: None,
                on_change: action(),
                presence: UiPresence::default(),
                menu: None,
            })
        }

        fn focus(tree: &mut UiTree, id: NodeId) {
            tree.node_mut(id).unwrap().flags.set(NodeFlags::FOCUSED, true);
            tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
        }

        fn has_solid_instance_colored(draw: &DrawList, color: Rgba) -> bool {
            draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| {
                (instance.params[2] - KIND_SOLID).abs() < 0.01 && (instance.color[0] - color.r).abs() < 0.001 && (instance.color[1] - color.g).abs() < 0.001 && (instance.color[2] - color.b).abs() < 0.001 && (instance.color[3] - color.a).abs() < 0.001
            })
        }

        #[test]
        fn painting_an_unfocused_input_emits_no_caret_or_selection() {
            let (mut tree, root, theme, mut atlas) = setup(&input("in", "hello"));
            tree.node_mut(root).unwrap().state.edit = Some(EditState { text: "hello".into(), caret: 5, anchor: 0, composition: None, scroll_x: 0.0 });
            let mut draw = DrawList::default();

            paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

            assert!(!has_solid_instance_colored(&draw, theme.accent), "an unfocused Input must not paint a caret even if a stale EditState lingers on it");
            assert!(!has_solid_instance_colored(&draw, theme.accent.with_alpha(0.3)), "an unfocused Input must not paint a selection highlight");
        }

        #[test]
        fn painting_a_focused_input_with_a_collapsed_selection_emits_a_caret_line_but_no_highlight() {
            let (mut tree, root, theme, mut atlas) = setup(&input("in", "hello"));
            focus(&mut tree, root);
            tree.node_mut(root).unwrap().state.edit = Some(EditState { text: "hello".into(), caret: 5, anchor: 5, composition: None, scroll_x: 0.0 });
            let mut draw = DrawList::default();

            paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

            assert!(has_solid_instance_colored(&draw, theme.accent), "a focused Input should paint its caret as a theme.accent solid instance");
            assert!(!has_solid_instance_colored(&draw, theme.accent.with_alpha(0.3)), "a collapsed selection (anchor == caret) must not paint a highlight rect");
        }

        #[test]
        fn painting_a_focused_input_with_a_real_selection_emits_a_translucent_highlight() {
            let (mut tree, root, theme, mut atlas) = setup(&input("in", "hello world"));
            focus(&mut tree, root);
            tree.node_mut(root).unwrap().state.edit = Some(EditState { text: "hello world".into(), caret: 5, anchor: 0, composition: None, scroll_x: 0.0 });
            let mut draw = DrawList::default();

            paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

            assert!(has_solid_instance_colored(&draw, theme.accent.with_alpha(0.3)), "a real anchor..caret selection should paint a theme.accent-at-0.3-alpha highlight rect");
        }

        #[test]
        fn painting_a_focused_input_shows_the_live_edit_buffer_text_not_the_stale_declarative_value() {
            let (mut tree, root, theme, mut atlas) = setup(&input("in", "old"));
            focus(&mut tree, root);
            tree.node_mut(root).unwrap().state.edit = Some(EditState { text: "a much longer buffer".into(), caret: 20, anchor: 20, composition: None, scroll_x: 0.0 });
            let mut draw = DrawList::default();

            let (mut stale_tree, stale_root, stale_theme, mut stale_atlas) = setup(&input("in", "old"));
            let mut stale_draw = DrawList::default();
            paint_tree(&mut stale_tree, stale_root, &stale_theme, &mut stale_atlas, None, false, &mut stale_draw);
            let stale_total: usize = stale_draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();

            paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);
            let focused_total: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();

            assert!(focused_total > stale_total, "a focused Input with a live EditState should paint its live (longer) buffer text, not the stale shorter declarative value");
        }

        fn toggle(id: &str) -> UiNode {
            UiNode::Toggle(UiToggleNode { id: id.into(), icon_id: IconName::CircleDot, text: Some("Toggle".into()), on_change: action(), presence: UiPresence::default(), menu: None })
        }

        fn icon_select(id: &str) -> UiNode {
            UiNode::IconSelect(UiIconSelectNode { id: id.into(), value: "star".into(), uniform: true, classifier_kind: "generic".into(), on_change: action(), presence: UiPresence::default(), menu: None })
        }

        /// 🎯️ Shared assertion for the border-swap-on-focus fix: an otherwise-identical pair of trees,
        /// one with `NodeFlags::FOCUSED` set on the root, should differ in at least one border instance's
        /// color (`theme.border_emphasized` replacing `theme.border_normal`) — mirrors
        /// `formControlFocusBorderClass`'s `focus-visible:border-accent` (`ui/js/react/index.tsx`).
        fn assert_focus_swaps_border_color(make: impl Fn() -> UiNode, label: &str) {
            let (mut unfocused_tree, unfocused_root, theme, mut unfocused_atlas) = setup(&make());
            let mut unfocused_draw = DrawList::default();
            paint_tree(&mut unfocused_tree, unfocused_root, &theme, &mut unfocused_atlas, None, false, &mut unfocused_draw);

            let (mut focused_tree, focused_root, focused_theme, mut focused_atlas) = setup(&make());
            focus(&mut focused_tree, focused_root);
            let mut focused_draw = DrawList::default();
            paint_tree(&mut focused_tree, focused_root, &focused_theme, &mut focused_atlas, None, false, &mut focused_draw);

            assert!(!has_solid_instance_colored(&unfocused_draw, theme.border_emphasized), "{label}: an unfocused control must not paint its border_emphasized color");
            assert!(has_solid_instance_colored(&focused_draw, theme.border_emphasized), "{label}: a focused control should swap its border to theme.border_emphasized");
        }

        #[test]
        fn painting_a_focused_button_swaps_its_border_to_border_emphasized() {
            assert_focus_swaps_border_color(|| button("btn", false), "Button");
        }

        #[test]
        fn painting_a_focused_select_swaps_its_border_to_border_emphasized() {
            assert_focus_swaps_border_color(|| select("sel", "a"), "Select");
        }

        #[test]
        fn painting_a_focused_toggle_swaps_its_border_to_border_emphasized() {
            assert_focus_swaps_border_color(|| toggle("tog"), "Toggle");
        }

        #[test]
        fn painting_a_focused_number_stepper_swaps_its_outer_border_to_border_emphasized() {
            assert_focus_swaps_border_color(|| stepper("ns", 2.0, true), "NumberStepper");
        }

        #[test]
        fn painting_a_focused_icon_select_swaps_its_border_to_border_emphasized() {
            assert_focus_swaps_border_color(|| icon_select("ic"), "IconSelect");
        }

        #[test]
        fn painting_a_hovered_number_stepper_tints_its_outer_background() {
            let (mut tree, root, theme, mut atlas) = setup(&stepper("ns", 2.0, true));
            tree.node_mut(root).unwrap().flags.set(NodeFlags::HOVERED, true);
            tree.mark_dirty(root, NodeFlags::DIRTY_PAINT);
            let mut draw = DrawList::default();

            paint_tree(&mut tree, root, &theme, &mut atlas, None, false, &mut draw);

            assert!(has_solid_instance_colored(&draw, theme.button_hover), "a hovered NumberStepper should tint its shared minus/plus background to theme.button_hover");
        }
        //#endregion 🔖️W2WidgetVisuals
    }
    // #endregion paint
}

#[cfg(feature = "engine")]
pub mod events {
    // #region events
    //! 🎯️ Retained-mode input routing (`UiEvent` in, `UiCommand` out): reverse-paint-order hit testing
    //! with clip pruning and overlay priority, pointer capture, Tab-order focus, and parent-chain
    //! bubbling. Conceptually replaces the old immediate-mode `input` region's per-frame
    //! `hit_targets`/`DragState` bookkeeping, but that region stays fully in place — `widgets`/`chrome`
    //! and, transitively, `framework/renderer/wgpu` and `infinite_world` still consume it directly, and
    //! the cutover to this module is later-phase renderer-thinning work (see the plan). `events` is
    //! purely additive: it depends on `tree`/`component`/`geometry` only, never on `input`.

    use std::collections::HashMap;

    use crate::arena::NodeId;
    use crate::component::layout::ActionDescriptor;
    use crate::component::ui::{SurfaceKind, UiNode, UiTreeItemNode, UiTreeSectionNode};
    use crate::geometry::Rect;
    use crate::tree::{EditState, Node, NodeFlags, NodeKey, UiTree};
    use crate::IconName;

    //#region 🔖️UiEvent
    /// 🖱️ Mouse button identity for `UiEvent::{PointerDown,PointerUp}`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PointerButton {
        Primary,
        Secondary,
        Middle,
    }

    /// ⌨️ Modifier keys held during a keyboard event. A minimal fresh type rather than reusing
    /// `input::PointerModifiers`, so this module stays decoupled from the region it conceptually
    /// replaces (see module doc comment).
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct EventModifiers {
        pub shift: bool,
        pub ctrl: bool,
        pub alt: bool,
        pub meta: bool,
    }

    /// 📥️ Input events the host feeds into `EventRouter::dispatch`.
    #[derive(Clone, Debug, PartialEq)]
    pub enum UiEvent {
        PointerDown {
            x: f32,
            y: f32,
            button: PointerButton,
        },
        PointerUp {
            x: f32,
            y: f32,
            button: PointerButton,
        },
        PointerMove {
            x: f32,
            y: f32,
        },
        Scroll {
            x: f32,
            y: f32,
            delta_x: f32,
            delta_y: f32,
        },
        KeyDown {
            key: String,
            modifiers: EventModifiers,
        },
        KeyUp {
            key: String,
            modifiers: EventModifiers,
        },
        TextInput {
            text: String,
        },
        /// 📋️ Host-delivered clipboard text in response to a `UiCommand::ClipboardPasteRequested` (the
        /// actual OS clipboard read is a `host`-region/renderer-integration concern; this is just the
        /// inbound half of the round trip). Routed identically to `TextInput`: inserted at the focused
        /// `EditState`'s caret, replacing any selection.
        Paste {
            text: String,
        },
        /// 🈶️ IME composition lifecycle for the focused editable node. Shapes `EditState::composition`
        /// is ready to receive; actually wiring winit's `Ime` events (native) or a hidden DOM input
        /// (web) to *produce* these is later `host`-region work, out of scope here.
        Ime(ImeEvent),
    }

    /// 🈶️ One IME composition step — see `UiEvent::Ime`.
    #[derive(Clone, Debug, PartialEq)]
    pub enum ImeEvent {
        Start,
        /// `cursor` is the IME's own preedit-relative cursor, informational only (not routed into
        /// `EditState::caret` — the composition is still uncommitted).
        Update {
            text: String,
            cursor: usize,
        },
        /// 🈶️ Finalizes the composition: clears `EditState::composition` and inserts `text` at the caret
        /// exactly like `TextInput`.
        Commit {
            text: String,
        },
        Cancel,
    }
    //#endregion 🔖️UiEvent

    //#region 🔖️HitTest
    /// 🎯️ Reverse-paint-order hit test from `root`: `paint::paint_stack` walks first_child→last_child
    /// (parent background first, then children in that order, each drawn over the last), so the
    /// topmost node at any point is the *last*-painted one — this walk visits children last-first to
    /// match. Overlay-flagged (`NodeFlags::OVERLAY`) children are tested before normal siblings at
    /// every level, so a popup always wins over base content underneath it. `CLIPS_CHILDREN` prunes
    /// early: a point outside that node's own bounds skips testing its children entirely, even if a
    /// child's own (unclipped) rect would nominally contain the point. `HIT_TRANSPARENT` nodes are
    /// skipped for the match itself (their children are still tested — pass-through). Returns the
    /// deepest/topmost matching node.
    pub(crate) fn hit_test(tree: &UiTree, root: NodeId, x: f32, y: f32) -> Option<NodeId> {
        hit_test_node(tree, root, 0.0, 0.0, x, y)
    }

    fn hit_test_node(tree: &UiTree, id: NodeId, origin_x: f32, origin_y: f32, x: f32, y: f32) -> Option<NodeId> {
        let node = tree.node(id)?;
        let abs_x = origin_x + node.layout.x;
        let abs_y = origin_y + node.layout.y;
        let inside = Rect::new(abs_x, abs_y, node.layout.width, node.layout.height).contains(x, y);
        if node.flags.contains(NodeFlags::CLIPS_CHILDREN) && !inside {
            return None;
        }
        let mut overlays: Vec<NodeId> = Vec::new();
        let mut normal: Vec<NodeId> = Vec::new();
        for child in tree.children(id) {
            match tree.node(child) {
                Some(child_node) if child_node.flags.contains(NodeFlags::OVERLAY) => overlays.push(child),
                _ => normal.push(child),
            }
        }
        for child in overlays.into_iter().rev().chain(normal.into_iter().rev()) {
            if let Some(hit) = hit_test_node(tree, child, abs_x, abs_y, x, y) {
                return Some(hit);
            }
        }
        // A bare `Stack` is a layout-only container with no interaction semantics of its own — it
        // must never be the hit result itself, only a pass-through to its children (same intent as
        // `HIT_TRANSPARENT`, just implicit for this variant instead of flag-driven) — *unless* W2
        // wiring (`is_plain_stack_container`) finds it actually carries `activate`/`drop_action`, is a
        // registered drag source, or (for `Tree`'s synthesized per-row `Stack`s, see
        // `reconcile::children_of`'s `Tree` arm) its original `UiTreeItemNode` spec has a
        // `hover_action`/`unhover_action` — any of those make it a real interaction target.
        let is_plain_container = is_plain_stack_container(tree, id, node);
        if inside && !node.flags.contains(NodeFlags::HIT_TRANSPARENT) && !is_plain_container {
            Some(id)
        } else {
            None
        }
    }

    /// 🎯️🌳️ W2 wiring: a `Stack` (`node.spec.0`) stops being a plain pass-through container the moment
    /// it carries `activate`/`drop_action` of its own, or is a registered `NodeFlags::DRAG_SOURCE`
    /// (`paint::sync_interactive_state` keeps that flag synced with `Tree` rows' `draggable` field, and
    /// `dispatch`'s `PointerDown` handling can only ever register a drag payload on a node that's
    /// actually reachable as a hit-test target in the first place — see `find_tree_item_spec`'s own
    /// caller in `dispatch`). A `Tree`'s synthesized per-row `Stack` (`reconcile::children_of`'s `Tree`
    /// arm, keyed by `item.id`) has no room for `hover_action`/`unhover_action` on its own retained
    /// shape either — those are re-derived here, by key, straight from the row's *original*
    /// `UiTreeItemNode` spec (`find_tree_item_spec`), so a row with only a hover affordance (no
    /// `action`/`draggable`) still becomes hit-testable.
    fn is_plain_stack_container(tree: &UiTree, id: NodeId, node: &Node) -> bool {
        let UiNode::Stack(stack) = &node.spec.0 else { return false };
        if stack.activate.is_some() || stack.drop_action.is_some() || node.flags.contains(NodeFlags::DRAG_SOURCE) {
            return false;
        }
        if stack.id.is_some() {
            if let Some(item) = find_tree_item_spec(tree, id) {
                if item.hover_action.is_some() || item.unhover_action.is_some() {
                    return false;
                }
            }
        }
        true
    }

    //#region 🔖️TreeItemLookup
    /// 🌳️ Re-derives a `Tree` row's *original* `UiTreeItemNode` spec — `hover_action`/`unhover_action`/
    /// `draggable`/`drag_data`, fields `UiStackNode` (the row's synthesized retained shape, see
    /// `reconcile::children_of`'s `Tree` arm) has no room for at all — by walking up from `row` to the
    /// nearest ancestor `UiNode::Tree` and searching its still-fully-intact spec (`reconcile` never
    /// drops fields, only clones them into `WidgetSpec` — see that module's own doc comment) for the
    /// item whose `id` matches this row's own stable key (`NodeKey::Explicit(item.id)`, exactly what
    /// `reconcile::tree_item_row` keys the row with). `None` for anything that isn't a keyed descendant
    /// of a `Tree` (ordinary `Stack`s, a `Tree`'s section rows, which are keyed by `section.id` instead).
    fn find_tree_item_spec(tree: &UiTree, row: NodeId) -> Option<&UiTreeItemNode> {
        let NodeKey::Explicit(row_id) = &tree.node(row)?.key else { return None };
        let mut ancestor = tree.node(row)?.parent;
        while let Some(candidate) = ancestor {
            let candidate_node = tree.node(candidate)?;
            if let UiNode::Tree(tree_node) = &candidate_node.spec.0 {
                return find_item_in_sections(&tree_node.sections, row_id);
            }
            ancestor = candidate_node.parent;
        }
        None
    }

    fn find_item_in_sections<'a>(sections: &'a [UiTreeSectionNode], id: &str) -> Option<&'a UiTreeItemNode> {
        sections.iter().find_map(|section| find_item_in_items(&section.items, id))
    }

    fn find_item_in_items<'a>(items: &'a [UiTreeItemNode], id: &str) -> Option<&'a UiTreeItemNode> {
        for item in items {
            if item.id == id {
                return Some(item);
            }
            if let Some(nested) = &item.items {
                if let Some(found) = find_item_in_items(nested, id) {
                    return Some(found);
                }
            }
        }
        None
    }
    //#endregion 🔖️TreeItemLookup

    /// 📐️ A node's absolute (window-space) origin: `LayoutBucket`'s own doc comment fixes `x`/`y` as
    /// **parent-relative**, so this walks the parent chain to `root` (whose own origin is `(0.0, 0.0)`)
    /// summing offsets. Used by the overlay placement/dismissal machinery, which needs a node's real
    /// on-screen bounds rather than its parent-relative layout rect.
    fn node_abs_origin(tree: &UiTree, id: NodeId) -> (f32, f32) {
        match tree.node(id) {
            Some(node) => {
                let (parent_x, parent_y) = match node.parent {
                    Some(parent) => node_abs_origin(tree, parent),
                    None => (0.0, 0.0),
                };
                (parent_x + node.layout.x, parent_y + node.layout.y)
            }
            None => (0.0, 0.0),
        }
    }

    /// 📐️ `node_abs_origin` plus the node's own size, as a `Rect` — `None` if `id` isn't in `tree`.
    pub(crate) fn node_abs_rect(tree: &UiTree, id: NodeId) -> Option<Rect> {
        let node = tree.node(id)?;
        let (x, y) = node_abs_origin(tree, id);
        Some(Rect::new(x, y, node.layout.width, node.layout.height))
    }

    /// 🎯️ `hit_test`, but `subtree_root` need not be the window's true tree root (an overlay root
    /// virtually never is — it's some descendant node). `hit_test` walks from its `root` argument
    /// treating that node's own `layout.x`/`layout.y` as relative to origin `(0.0, 0.0)`, which is only
    /// correct window-absolute-coordinate behavior when `root` itself has no parent; this instead
    /// resolves `subtree_root`'s *parent's* absolute origin (`(0.0, 0.0)` if it has none) and translates
    /// `(x, y)` into that frame first, so overlay dismissal/hover-out checks against a non-root overlay
    /// subtree stay correct regardless of how deep it's nested.
    pub(crate) fn hit_test_subtree(tree: &UiTree, subtree_root: NodeId, x: f32, y: f32) -> Option<NodeId> {
        let (parent_x, parent_y) = match tree.node(subtree_root).and_then(|node| node.parent) {
            Some(parent) => node_abs_origin(tree, parent),
            None => (0.0, 0.0),
        };
        hit_test(tree, subtree_root, x - parent_x, y - parent_y)
    }
    //#endregion 🔖️HitTest

    //#region 🔖️Capture
    /// ↕️ Which axis a `CaptureKind::ScrollThumb` drag maps pointer delta onto.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ScrollAxis {
        Horizontal,
        Vertical,
    }

    /// 🫳️ What kind of interaction currently holds pointer capture. A coarser-grained, retained-mode
    /// replacement for the old `input::DragState`/`TreeDragState` pair. `Drag` is a generic
    /// `DragSession` (see 🔖️DragDrop below) promoted from `Press` once pointer movement past a small
    /// threshold is observed on a node with a registered `DragPayload`. `ScrollThumb` is a scrollbar
    /// thumb (painted by `paint`, registered via `EventRouter::register_scroll_thumb`) dragging its
    /// owning `NodeFlags::SCROLLABLE` node's `WidgetState::scroll_offset` along one axis.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CaptureKind {
        Press,
        Drag,
        ScrollThumb(ScrollAxis),
    }

    /// 🔒️ Once a node captures, subsequent pointer-move/up events route directly to it regardless of
    /// what's actually under the pointer, until released on `PointerUp` (or explicit `release`).
    #[derive(Clone, Copy, Debug, Default)]
    struct CaptureState {
        target: Option<(NodeId, CaptureKind)>,
    }

    impl CaptureState {
        fn release(&mut self) -> Option<(NodeId, CaptureKind)> {
            self.target.take()
        }
    }
    //#endregion 🔖️Capture

    //#region 🔖️Focus
    /// 🎯️ Which `UiNode` variants participate in Tab-order focus cycling.
    fn is_focusable(node: &UiNode) -> bool {
        matches!(node, UiNode::Input(_) | UiNode::Button(_) | UiNode::Select(_) | UiNode::Toggle(_) | UiNode::Slider(_) | UiNode::NumberStepper(_) | UiNode::Ring(_) | UiNode::IconSelect(_))
    }

    fn collect_focusable(tree: &UiTree, id: NodeId, out: &mut Vec<NodeId>) {
        if let Some(node) = tree.node(id) {
            if is_focusable(&node.spec.0) {
                out.push(id);
            }
        }
        for child in tree.children(id) {
            collect_focusable(tree, child, out);
        }
    }

    /// 🔦️ Currently-focused node plus a lazily-rebuilt document-order Tab cycle over focusable nodes.
    struct FocusState {
        focused: Option<NodeId>,
        tab_order: Vec<NodeId>,
    }

    impl FocusState {
        fn new() -> Self {
            Self { focused: None, tab_order: Vec::new() }
        }

        /// 🎯️ Sets/clears focus, flipping `NodeFlags::FOCUSED` on the old and new targets and marking
        /// both `DIRTY_PAINT` (a focus ring likely needs repainting) via `UiTree::mark_dirty`. A no-operation
        /// (no flag churn) when `node` already matches the current focus. Also owns `EditState`'s
        /// lifecycle: blurring a node clears its `WidgetState::edit` (the buffer relinquishes control,
        /// so the node's declarative `value` governs again on the next `apply_tree`); focusing a
        /// `UiNode::Input` for the first time seeds `edit` from that declarative `value` with the caret
        /// at the end — see `tree::WidgetState`'s own doc comment for why reconcile never clobbers this.
        fn set_focus(&mut self, tree: &mut UiTree, node: Option<NodeId>) {
            if self.focused == node {
                return;
            }
            if let Some(previous) = self.focused {
                if let Some(previous_node) = tree.node_mut(previous) {
                    previous_node.flags.set(NodeFlags::FOCUSED, false);
                    previous_node.state.edit = None;
                }
                tree.mark_dirty(previous, NodeFlags::DIRTY_PAINT);
            }
            if let Some(next) = node {
                if let Some(next_node) = tree.node_mut(next) {
                    next_node.flags.set(NodeFlags::FOCUSED, true);
                    if next_node.state.edit.is_none() {
                        if let UiNode::Input(input) = &next_node.spec.0 {
                            let caret = input.value.len();
                            next_node.state.edit = Some(EditState { text: input.value.clone(), caret, anchor: caret, composition: None, scroll_x: 0.0 });
                        }
                    }
                }
                tree.mark_dirty(next, NodeFlags::DIRTY_PAINT);
            }
            self.focused = node;
        }

        fn clear_focus(&mut self, tree: &mut UiTree) {
            self.set_focus(tree, None);
        }

        fn rebuild_tab_order(&mut self, tree: &UiTree, root: NodeId) {
            self.tab_order.clear();
            collect_focusable(tree, root, &mut self.tab_order);
        }

        fn focus_next(&mut self, tree: &mut UiTree, root: NodeId) {
            self.rebuild_tab_order(tree, root);
            if self.tab_order.is_empty() {
                self.set_focus(tree, None);
                return;
            }
            let next_index = match self.focused.and_then(|id| self.tab_order.iter().position(|&candidate| candidate == id)) {
                Some(index) => (index + 1) % self.tab_order.len(),
                None => 0,
            };
            self.set_focus(tree, Some(self.tab_order[next_index]));
        }

        fn focus_prev(&mut self, tree: &mut UiTree, root: NodeId) {
            self.rebuild_tab_order(tree, root);
            if self.tab_order.is_empty() {
                self.set_focus(tree, None);
                return;
            }
            let previous_index = match self.focused.and_then(|id| self.tab_order.iter().position(|&candidate| candidate == id)) {
                Some(index) => (index + self.tab_order.len() - 1) % self.tab_order.len(),
                None => self.tab_order.len() - 1,
            };
            self.set_focus(tree, Some(self.tab_order[previous_index]));
        }
    }
    //#endregion 🔖️Focus

    //#region 🔖️Bubble
    /// 🫧️ Walks from `from` up through `parent` links (including `from` itself), calling `handler(id)`
    /// for each ancestor until it returns `true` ("handled, stop bubbling") or the root is reached.
    pub(crate) fn bubble<F: FnMut(NodeId) -> bool>(tree: &UiTree, from: NodeId, mut handler: F) {
        let mut cursor = Some(from);
        while let Some(id) = cursor {
            if handler(id) {
                return;
            }
            cursor = tree.node(id).and_then(|node| node.parent);
        }
    }

    /// 🌳️ Whether `id` is `ancestor` itself or a descendant of it, walking the parent chain.
    fn is_descendant(tree: &UiTree, id: NodeId, ancestor: NodeId) -> bool {
        let mut found = false;
        bubble(tree, id, |current| {
            if current == ancestor {
                found = true;
                true
            } else {
                false
            }
        });
        found
    }
    //#endregion 🔖️Bubble

    //#region 🔖️Overlay
    // 🪟️ One first-class overlay mechanism serving Select popups, context menus, tooltips, dialogs, and
    // a command palette — not five bespoke implementations. `NodeFlags::OVERLAY` already gives a
    // flagged child hit-test priority over its normal siblings (see 🔖️HitTest above); `EventRouter`
    // layers open/close/anchor/placement/dismissal/focus-trap bookkeeping on top of that one existing
    // primitive. Building the popup CONTENTS (a `Select`'s item list, a context menu's entries, …) is
    // explicitly not this module's job — a caller (future `reconcile`/`paint`/`host` wiring) reconciles
    // that subtree in and hands this module its root `NodeId` plus a `kind`/`anchor`; from there this
    // module owns the subtree's lifecycle.

    /// 🏷️ Which of the five overlay use-cases is open — drives the default placement rule and
    /// dismissal policy (`OverlayKind::default_placement`/`dismiss_policy`).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum OverlayKind {
        SelectPopup,
        ContextMenu,
        Tooltip,
        Dialog,
        CommandPalette,
    }

    /// ⚓️ What an overlay is positioned relative to: an existing node (a `Select`'s trigger, a hovered
    /// row) or a raw point (where a context menu was right-clicked, where the pointer was when a
    /// tooltip's hover-delay fired).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum OverlayAnchor {
        Node(NodeId),
        Point { x: f32, y: f32 },
    }

    /// 📐️ How an overlay's resolved position is computed from its anchor — see
    /// `resolve_overlay_placement`.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum OverlayPlacement {
        /// 👇️ `SelectPopup`/`ContextMenu`: directly below the anchor, flipped above it if that would
        /// overflow the viewport's bottom edge.
        BelowAnchorWithFlip,
        /// 🖱️ `Tooltip`: offset from the anchor point.
        AtPointer { offset_x: f32, offset_y: f32 },
        /// 🎯️ `Dialog`/`CommandPalette`: viewport-centered.
        Centered,
    }

    /// 🚪️ How an open overlay can be dismissed.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct DismissPolicy {
        /// 👆️ A `PointerDown` landing outside the overlay's subtree closes it and swallows the press
        /// (doesn't fall through to whatever's underneath) — standard popup click-outside semantics.
        pub outside_press_swallow: bool,
        /// ⎋️ `Escape` closes this overlay if it's the topmost open one.
        pub escape_closes: bool,
        /// ⏱️ Tooltip-specific: close this many seconds after the pointer leaves both the anchor and the
        /// overlay's own bounds. **Not actually debounced yet** — this crate has no animation-clock
        /// scaffolding anywhere (`engine::Ui::needs_frame`'s own doc comment makes the same admission for
        /// animations generally), so `maybe_dismiss_tooltip_on_hover_out` closes immediately on hover-out
        /// today; this field records the *intended* delay for whenever a clock exists to debounce it.
        pub hover_out_delay_seconds: Option<f32>,
    }

    impl OverlayKind {
        pub fn default_placement(self) -> OverlayPlacement {
            match self {
                OverlayKind::SelectPopup | OverlayKind::ContextMenu => OverlayPlacement::BelowAnchorWithFlip,
                OverlayKind::Tooltip => OverlayPlacement::AtPointer { offset_x: 12.0, offset_y: 16.0 },
                OverlayKind::Dialog | OverlayKind::CommandPalette => OverlayPlacement::Centered,
            }
        }

        pub fn dismiss_policy(self) -> DismissPolicy {
            match self {
                OverlayKind::Tooltip => DismissPolicy { outside_press_swallow: false, escape_closes: true, hover_out_delay_seconds: Some(0.4) },
                _ => DismissPolicy { outside_press_swallow: true, escape_closes: true, hover_out_delay_seconds: None },
            }
        }
    }

    /// 🪟️ One currently-open overlay's lifecycle state.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct OpenOverlay {
        /// 🌳️ The overlay's content subtree root — `EventRouter::open_overlay` flags this
        /// `NodeFlags::OVERLAY` for hit-test priority and clears it again on close.
        pub root: NodeId,
        pub kind: OverlayKind,
        pub anchor: OverlayAnchor,
        pub placement: OverlayPlacement,
        pub dismiss: DismissPolicy,
        /// 🔒️ `Dialog`/`CommandPalette`: while `true`, Tab-order cycling is bounded to this overlay's
        /// subtree (see `EventRouter::dispatch`'s `Tab` handling).
        pub focus_trap: bool,
    }

    /// 🥞️ Open overlays in z-order (last = topmost = painted last = hit-tested first, matching
    /// `NodeFlags::OVERLAY`'s own priority rule). Only one `EventRouter` field, but a `Vec` rather than a
    /// single slot because a context menu can itself spawn a submenu, or a Select popup can open above a
    /// Dialog — nesting is a real case this mechanism must support, not just a single global popup.
    #[derive(Default)]
    pub(crate) struct OverlayStack {
        open: Vec<OpenOverlay>,
    }

    impl OverlayStack {
        fn new() -> Self {
            Self::default()
        }

        fn open(&mut self, overlay: OpenOverlay) {
            self.open.push(overlay);
        }

        fn topmost(&self) -> Option<&OpenOverlay> {
            self.open.last()
        }

        fn close_root(&mut self, root: NodeId) -> Option<OpenOverlay> {
            let position = self.open.iter().position(|overlay| overlay.root == root)?;
            Some(self.open.remove(position))
        }

        fn close_topmost(&mut self) -> Option<OpenOverlay> {
            self.open.pop()
        }

        /// 🔒️ The root of the topmost `focus_trap` overlay, if any — `Escape`/outside-press only ever
        /// close the *topmost* overlay, but a focus trap set by a lower (still-open) trapping overlay
        /// stays in effect once a higher non-trapping overlay (e.g. a `Tooltip`) is on top of it, so this
        /// searches from the top down rather than just checking `topmost()`.
        fn topmost_focus_trap_root(&self) -> Option<NodeId> {
            self.open.iter().rev().find(|overlay| overlay.focus_trap).map(|overlay| overlay.root)
        }
    }

    /// 📐️ Resolves an overlay's top-left origin from its anchor, `kind`'s `placement` rule, the
    /// overlay's own measured `content_size` (post-layout — paint/flex, not this module, own measuring
    /// it), and the window's `viewport` size. Pure geometry: callers (a future `paint`/`flex` wiring)
    /// still own actually writing the result into the overlay root's layout — `events` only decides
    /// *where*, per the module doc comment's "the content subtree itself is whatever the caller
    /// reconciled in" scoping.
    pub fn resolve_overlay_placement(tree: &UiTree, anchor: OverlayAnchor, content_size: (f32, f32), viewport: (f32, f32), placement: OverlayPlacement) -> (f32, f32) {
        let anchor_rect = match anchor {
            OverlayAnchor::Node(id) => node_abs_rect(tree, id).unwrap_or(Rect::new(0.0, 0.0, 0.0, 0.0)),
            OverlayAnchor::Point { x, y } => Rect::new(x, y, 0.0, 0.0),
        };
        let (content_w, content_h) = content_size;
        let (viewport_w, viewport_h) = viewport;
        match placement {
            OverlayPlacement::BelowAnchorWithFlip => {
                let below_y = anchor_rect.y + anchor_rect.h;
                let fits_below = below_y + content_h <= viewport_h;
                let y = if fits_below { below_y } else { (anchor_rect.y - content_h).max(0.0) };
                let x = anchor_rect.x.clamp(0.0, (viewport_w - content_w).max(0.0));
                (x, y)
            }
            OverlayPlacement::AtPointer { offset_x, offset_y } => {
                let x = (anchor_rect.x + offset_x).clamp(0.0, (viewport_w - content_w).max(0.0));
                let y = (anchor_rect.y + offset_y).clamp(0.0, (viewport_h - content_h).max(0.0));
                (x, y)
            }
            OverlayPlacement::Centered => (((viewport_w - content_w) / 2.0).max(0.0), ((viewport_h - content_h) / 2.0).max(0.0)),
        }
    }
    //#endregion 🔖️Overlay

    //#region 🔖️DragDrop
    // 🫳️ Generic drag-and-drop session lifecycle: start-drag (promoted from a `Press` capture once
    // pointer movement clears `DRAG_PROMOTE_THRESHOLD_SQ`), update-position/evaluate-drop-target
    // (`EventRouter::update_drag`, called from `PointerMove`), commit-or-cancel (`PointerUp`). Building
    // the specific CONSUMERS (tree reorder, dock retiling, …) is out of scope — this is wire-format
    // parity plumbing for whatever consumes `UiCommand::DropCommitted`.

    /// 🏷️ Drag payload: MIME-style keys, JSON-encoded string values — exactly the shape
    /// `framework/renderer/react/ui-interpreter.tsx`'s `handleDrop` reads off `DataTransfer` (`data:
    /// Record<string, string>`, matched by `application/x-semio-*` key prefix) and the shape
    /// `UiTreeItemNode::drag_data` already carries. Reusing this shape (rather than a bespoke Rust enum)
    /// means a later workstream wiring this into the same program action contracts needs zero translation.
    pub type DragPayload = HashMap<String, String>;

    /// 👻️ Minimal drag-ghost shape — the actual visual is `paint`'s job (another region/agent); this is
    /// just enough for a caller to render *something* under the pointer.
    #[derive(Clone, Debug, PartialEq)]
    pub struct DragGhost {
        pub label: String,
        pub offset_x: f32,
        pub offset_y: f32,
    }

    /// 🫳️ One active drag, from promotion out of a `Press` capture (`EventRouter::maybe_promote_to_drag`)
    /// through to `PointerUp`'s commit/cancel.
    #[derive(Clone, Debug, PartialEq)]
    pub struct DragSession {
        pub source: NodeId,
        pub payload: DragPayload,
        pub ghost: Option<DragGhost>,
        pub pointer_x: f32,
        pub pointer_y: f32,
        /// 🎯️ The nearest `NodeFlags::DROP_TARGET` ancestor of whatever's under the pointer right now
        /// that also passes its registered accept predicate (`EventRouter::set_drop_accept`), if any —
        /// recomputed every `PointerMove` by `EventRouter::update_drag`.
        pub drop_target: Option<NodeId>,
    }

    /// 📏️ Squared pixel distance a `Press` capture on a `DragPayload`-registered node must travel before
    /// `EventRouter::maybe_promote_to_drag` promotes it to a real `DragSession` — a small dead-zone so an
    /// ordinary click on a draggable node doesn't spuriously start (and then immediately cancel) a drag.
    const DRAG_PROMOTE_THRESHOLD_SQ: f32 = 16.0;
    //#endregion 🔖️DragDrop

    //#region 🔖️Scroll
    // 🖱️ Wheel events route to the nearest scrollable ancestor of the node under the pointer, walking
    // the bubble chain for `NodeFlags::SCROLLABLE` exactly like `nearest_accepting_drop_target` walks it
    // for `NodeFlags::DROP_TARGET`. Thumb-drag capture (`CaptureKind::ScrollThumb`) is a separate path:
    // `paint` paints the actual thumb node wherever it likes in the tree and registers it once via
    // `EventRouter::register_scroll_thumb`, decoupling thumb geometry from scrollable-content geometry.

    /// 🖱️ Walks `from`'s bubble chain (inclusive) for the nearest `NodeFlags::SCROLLABLE` node.
    fn nearest_scrollable_ancestor(tree: &UiTree, from: NodeId) -> Option<NodeId> {
        let mut found = None;
        bubble(tree, from, |id| {
            if tree.node(id).is_some_and(|node| node.flags.contains(NodeFlags::SCROLLABLE)) {
                found = Some(id);
                true
            } else {
                false
            }
        });
        found
    }
    //#endregion 🔖️Scroll

    //#region 🔖️EditRouting
    // ✍️ Key routing for a focused editable node's `tree::EditState`. Byte-offset caret/anchor
    // throughout (see `EditState`'s own doc comment for why); `prev_char_boundary`/`next_char_boundary`
    // step one `char` at a time without re-deriving a full `char_indices` pass per keystroke.

    fn prev_char_boundary(text: &str, index: usize) -> usize {
        if index == 0 {
            return 0;
        }
        let mut candidate = index - 1;
        while candidate > 0 && !text.is_char_boundary(candidate) {
            candidate -= 1;
        }
        candidate
    }

    fn next_char_boundary(text: &str, index: usize) -> usize {
        if index >= text.len() {
            return text.len();
        }
        let mut candidate = index + 1;
        while candidate < text.len() && !text.is_char_boundary(candidate) {
            candidate += 1;
        }
        candidate
    }

    /// ↔ Selection bounds as `(start, end)` regardless of which of `anchor`/`caret` is smaller —
    /// `EditState`'s own doc comment documents the selection as `anchor..caret` in either order.
    fn selection_bounds(anchor: usize, caret: usize) -> (usize, usize) {
        (anchor.min(caret), anchor.max(caret))
    }

    /// ✍️ Replaces the current selection (or inserts at the caret if there isn't one) with `text`,
    /// collapsing caret and anchor to just past the inserted text. Shared by `TextInput`, `Paste`, and
    /// `Ime::Commit` routing — insertion semantics are identical for all three.
    fn insert_at_caret(edit: &mut EditState, text: &str) {
        let (start, end) = selection_bounds(edit.anchor, edit.caret);
        edit.text.replace_range(start..end, text);
        let caret = start + text.len();
        edit.caret = caret;
        edit.anchor = caret;
    }
    //#endregion 🔖️EditRouting

    //#region 🔖️UiCommand
    /// 📤️ What the engine emits for the host to act on, drained once per tick.
    #[derive(Clone, Debug, PartialEq)]
    pub enum UiCommand {
        /// 🧩️ A widget's declarative `ActionDescriptor` fired (currently: a `Button` clicked while it
        /// still had the node the pointer went down on under the pointer at release).
        App { window_id: String, action: ActionDescriptor },
        /// 🔦️ Focus moved (or cleared) as a result of routing an event.
        FocusChanged { window_id: String, node: Option<NodeId> },
        /// 🪟️ An overlay closed — either explicitly (`EventRouter::close_overlay`) or via dismissal
        /// (outside-press swallow, `Escape`, tooltip hover-out).
        OverlayClosed { window_id: String, root: NodeId, kind: OverlayKind },
        /// 🫳️ A `DragSession` released over an accepting drop target.
        DropCommitted { window_id: String, source: NodeId, target: NodeId, payload: DragPayload },
        /// 🫳️ A `DragSession` released with no accepting drop target under the pointer.
        DropCancelled { window_id: String, source: NodeId },
        /// 📋️ `Ctrl`/`Cmd`+`C` over a text selection — host copies `text` to the OS clipboard.
        ClipboardCopy { window_id: String, text: String },
        /// 📋️ `Ctrl`/`Cmd`+`X` over a text selection — `text` is already removed from the `EditState`
        /// buffer; host copies it to the OS clipboard.
        ClipboardCut { window_id: String, text: String },
        /// 📋️ `Ctrl`/`Cmd`+`V`: host must read the OS clipboard and feed the result back as
        /// `UiEvent::Paste` (the OS clipboard read itself is a `host`-region concern, not `events`').
        ClipboardPasteRequested { window_id: String },
        /// 🎬️ A real `PointerDown`/`PointerUp`/`PointerMove`/`Scroll` `event` that hit-tested to a
        /// `ComponentScene` leaf — the host looks up `node`'s live `UiComponentSceneNode` (same
        /// `window_id`+`node` the retained tree's `scene_slots` region reads) and routes `event` into
        /// that `kind`'s own per-`SurfaceKind` input handler, instead of sampling an aggregate
        /// `InputState` once per render frame the way `framework/renderer/wgpu`'s `RenderEntry` region
        /// used to. `surface_id`/`kind`/`rect` are carried directly (resolved once here, at dispatch
        /// time, from the same ancestor-offset accumulation `scene_slots::collect_scene_slots`/
        /// `hit_test_node` use) so a host doesn't need its own tree walk just to decide whether this
        /// surface already gets real OS-event-driven input through its own bespoke host (`world-3d`/
        /// `node-graph`/`tiled-map`/`board-2d`) before paying for the `node` lookup.
        Scene { window_id: String, node: NodeId, surface_id: String, kind: SurfaceKind, rect: Rect, event: UiEvent },
    }

    /// 🧭️ Owns capture + focus + overlay + drag + scroll-thumb state for one window's retained tree and
    /// turns `UiEvent`s into `NodeFlags`/`WidgetState` updates plus a minimal, correct (not speculative)
    /// set of `UiCommand`s. Per-widget-variant semantics beyond generic routing (e.g. actually committing
    /// an edited `Input`'s value via its `on_change` `ActionDescriptor`) are a documented gap for a later
    /// milestone, same as this struct's own precedent (`Button` was the only concretely-wired variant
    /// before M5).
    /// 🎯️ A `set_drop_accept` predicate — see `EventRouter::drop_accept`.
    type DropAcceptPredicate = Box<dyn Fn(&DragPayload) -> bool>;

    pub(crate) struct EventRouter {
        window_id: String,
        capture: CaptureState,
        focus: FocusState,
        hovered: Option<NodeId>,
        /// 🫧️ Every node currently in the hover bubble chain (leaf-to-root from `hovered`), so an
        /// ancestor container (e.g. a `Stack`-based tree-item row, which `hit_test` never itself returns
        /// as the match — see 🔖️HitTest's `is_plain_container`) still observes `NodeFlags::HOVERED` for
        /// `paint`'s hover-reveal (React's `placement` / driver.chrome reveal) to key off of.
        hover_chain: Vec<NodeId>,
        /// 👇️ Pointer position at the start of the current `Press` capture, for `maybe_promote_to_drag`'s
        /// movement-threshold check.
        press_origin: Option<(f32, f32)>,
        overlays: OverlayStack,
        drag: Option<DragSession>,
        /// 🫳️ Per-node `DragPayload` a `Press` capture on that node may promote into, set via
        /// `set_drag_payload`.
        drag_payloads: HashMap<NodeId, DragPayload>,
        /// 🎯️ Per-node accept predicate refining plain `NodeFlags::DROP_TARGET` membership, set via
        /// `set_drop_accept`. Absent from this map but flagged `DROP_TARGET` still accepts everything.
        drop_accept: HashMap<NodeId, DropAcceptPredicate>,
        /// 🖱️ Scrollbar-thumb node id → (its owning `NodeFlags::SCROLLABLE` node, drag axis), set via
        /// `register_scroll_thumb`.
        scroll_thumbs: HashMap<NodeId, (NodeId, ScrollAxis)>,
        /// 🖱️ `(pointer_x, pointer_y, scroll_offset_x, scroll_offset_y)` captured at the start of a
        /// `ScrollThumb` drag, for `update_scroll_thumb`'s delta-computation baseline.
        thumb_start: Option<(f32, f32, f32, f32)>,
    }

    impl EventRouter {
        pub(crate) fn new(window_id: impl Into<String>) -> Self {
            Self {
                window_id: window_id.into(),
                capture: CaptureState::default(),
                focus: FocusState::new(),
                hovered: None,
                hover_chain: Vec::new(),
                press_origin: None,
                overlays: OverlayStack::new(),
                drag: None,
                drag_payloads: HashMap::new(),
                drop_accept: HashMap::new(),
                scroll_thumbs: HashMap::new(),
                thumb_start: None,
            }
        }

        fn resolve_target(&self, tree: &UiTree, root: NodeId, x: f32, y: f32) -> Option<NodeId> {
            match self.capture.target {
                Some((id, _)) => Some(id),
                None => hit_test(tree, root, x, y),
            }
        }

        /// 👆️ Flips `NodeFlags::HOVERED` off every node in the old hover bubble chain that isn't in the
        /// new one, and on for every new node that wasn't in the old one — see `hover_chain`'s own doc
        /// comment for why the whole chain (not just the leaf) carries the flag. W2 wiring: also fires a
        /// `Tree` row's `hover_action`/`unhover_action` (`find_tree_item_spec`) on entering/leaving —
        /// `UiStackNode` (a row's synthesized retained shape) has no field for either, so this is the same
        /// re-derivation `is_plain_stack_container`'s tree-row exception uses, fired via `UiCommand::App`
        /// exactly like a `Button`'s click (see `dispatch`'s `PointerUp` handling for that precedent).
        fn update_hover(&mut self, tree: &mut UiTree, target: Option<NodeId>) -> Vec<UiCommand> {
            let mut commands = Vec::new();
            if self.hovered == target {
                return commands;
            }
            let mut new_chain = Vec::new();
            if let Some(leaf) = target {
                bubble(tree, leaf, |id| {
                    new_chain.push(id);
                    false
                });
            }
            for &previous in &self.hover_chain {
                if !new_chain.contains(&previous) {
                    if let Some(node) = tree.node_mut(previous) {
                        node.flags.set(NodeFlags::HOVERED, false);
                    }
                    tree.mark_dirty(previous, NodeFlags::DIRTY_PAINT);
                    if let Some(action) = find_tree_item_spec(tree, previous).and_then(|item| item.unhover_action.clone()) {
                        commands.push(UiCommand::App { window_id: self.window_id.clone(), action });
                    }
                }
            }
            for &next in &new_chain {
                if !self.hover_chain.contains(&next) {
                    if let Some(node) = tree.node_mut(next) {
                        node.flags.set(NodeFlags::HOVERED, true);
                    }
                    tree.mark_dirty(next, NodeFlags::DIRTY_PAINT);
                    if let Some(action) = find_tree_item_spec(tree, next).and_then(|item| item.hover_action.clone()) {
                        commands.push(UiCommand::App { window_id: self.window_id.clone(), action });
                    }
                }
            }
            self.hover_chain = new_chain;
            self.hovered = target;
            commands
        }

        //#region 🔖️OverlayApi
        /// 🪟️ Opens an overlay: flags `root` `NodeFlags::OVERLAY` (hit-test priority — see 🔖️HitTest) and
        /// pushes it onto the z-ordered stack with `kind`'s default placement/dismissal policy.
        /// `Dialog`/`CommandPalette` become focus-trap scopes automatically.
        pub(crate) fn open_overlay(&mut self, tree: &mut UiTree, root: NodeId, kind: OverlayKind, anchor: OverlayAnchor) {
            if let Some(node) = tree.node_mut(root) {
                node.flags.set(NodeFlags::OVERLAY, true);
            }
            tree.mark_dirty(root, NodeFlags::DIRTY_PAINT);
            let focus_trap = matches!(kind, OverlayKind::Dialog | OverlayKind::CommandPalette);
            self.overlays.open(OpenOverlay { root, kind, anchor, placement: kind.default_placement(), dismiss: kind.dismiss_policy(), focus_trap });
        }

        pub(crate) fn close_overlay(&mut self, tree: &mut UiTree, root: NodeId) -> Vec<UiCommand> {
            match self.overlays.close_root(root) {
                Some(overlay) => self.finish_close(tree, overlay),
                None => Vec::new(),
            }
        }

        pub(crate) fn close_topmost_overlay(&mut self, tree: &mut UiTree) -> Vec<UiCommand> {
            match self.overlays.close_topmost() {
                Some(overlay) => self.finish_close(tree, overlay),
                None => Vec::new(),
            }
        }

        #[allow(dead_code, reason = "overlay-stack accessor, not yet called; likely wired by a later events-integration milestone")]
        pub(crate) fn topmost_overlay(&self) -> Option<&OpenOverlay> {
            self.overlays.topmost()
        }

        /// 🔽️ W2 wiring: the consumer-side effect of a `Select` click — flips `tree::WidgetState::open`
        /// via `open_overlay`/`close_overlay` (root *and* anchor are the `Select` node itself: its own
        /// synthesized item rows, see `reconcile::children_of`'s `Select` arm, are already its retained
        /// children, and marking the `Select` node `NodeFlags::OVERLAY` gives the whole popup hit-test
        /// priority over its own later-painted siblings). All dismissal paths (outside-press, `Escape`,
        /// an explicit `close_overlay`, or picking an item — see `dispatch`'s `PointerUp` handling) funnel
        /// through `finish_close`, which clears `open` back to `false` uniformly.
        pub(crate) fn toggle_select_popup(&mut self, tree: &mut UiTree, select_id: NodeId) -> Vec<UiCommand> {
            let already_open = tree.node(select_id).is_some_and(|node| node.state.open);
            if already_open {
                self.close_overlay(tree, select_id)
            } else {
                self.open_overlay(tree, select_id, OverlayKind::SelectPopup, OverlayAnchor::Node(select_id));
                if let Some(node) = tree.node_mut(select_id) {
                    node.state.open = true;
                }
                Vec::new()
            }
        }

        /// 🧹️ Clears `NodeFlags::OVERLAY`, and clears focus too if it was inside the closed overlay's
        /// subtree (dangling focus into a now-hidden subtree would otherwise route key events nowhere
        /// useful). `SelectPopup`'s `tree::WidgetState::open` is the popup's own show/hide bit
        /// (`paint::paint_select` reads it) — cleared here too, so every dismissal path (see
        /// `toggle_select_popup`'s doc comment) stays in sync with the overlay lifecycle uniformly.
        fn finish_close(&mut self, tree: &mut UiTree, overlay: OpenOverlay) -> Vec<UiCommand> {
            if let Some(node) = tree.node_mut(overlay.root) {
                node.flags.set(NodeFlags::OVERLAY, false);
                if overlay.kind == OverlayKind::SelectPopup {
                    node.state.open = false;
                }
            }
            tree.mark_dirty(overlay.root, NodeFlags::DIRTY_PAINT);
            let mut out = vec![UiCommand::OverlayClosed { window_id: self.window_id.clone(), root: overlay.root, kind: overlay.kind }];
            if let Some(focused) = self.focus.focused {
                if is_descendant(tree, focused, overlay.root) {
                    self.focus.clear_focus(tree);
                    out.push(UiCommand::FocusChanged { window_id: self.window_id.clone(), node: None });
                }
            }
            out
        }

        /// 👆️ If the topmost overlay dismisses on outside-press and `(x, y)` lands outside its subtree,
        /// closes it and returns the resulting commands — the caller must swallow the press (not route it
        /// any further) when this returns `Some`.
        fn dismiss_topmost_if_outside_press(&mut self, tree: &mut UiTree, x: f32, y: f32) -> Option<Vec<UiCommand>> {
            let top = self.overlays.topmost()?;
            if !top.dismiss.outside_press_swallow {
                return None;
            }
            let overlay_root = top.root;
            if hit_test_subtree(tree, overlay_root, x, y).is_some() {
                return None;
            }
            Some(self.close_topmost_overlay(tree))
        }

        /// 🖱️ `Tooltip`-only: closes the topmost overlay once the pointer leaves both its anchor and its
        /// own bounds. See `DismissPolicy::hover_out_delay_seconds` for why this is immediate, not
        /// debounced.
        fn maybe_dismiss_tooltip_on_hover_out(&mut self, tree: &mut UiTree, x: f32, y: f32) -> Vec<UiCommand> {
            let Some(top) = self.overlays.topmost() else { return Vec::new() };
            if top.kind != OverlayKind::Tooltip {
                return Vec::new();
            }
            let overlay_root = top.root;
            let anchor = top.anchor;
            let inside_overlay = node_abs_rect(tree, overlay_root).is_some_and(|rect| rect.contains(x, y));
            let inside_anchor = match anchor {
                OverlayAnchor::Node(id) => node_abs_rect(tree, id).is_some_and(|rect| rect.contains(x, y)),
                OverlayAnchor::Point { .. } => false,
            };
            if inside_overlay || inside_anchor {
                return Vec::new();
            }
            self.close_topmost_overlay(tree)
        }
        //#endregion 🔖️OverlayApi

        //#region 🔖️DragDropApi
        pub(crate) fn set_drag_payload(&mut self, node: NodeId, payload: DragPayload) {
            self.drag_payloads.insert(node, payload);
        }

        #[allow(dead_code, reason = "drag-drop registry accessor, not yet called; likely wired by a later events-integration milestone")]
        pub(crate) fn clear_drag_payload(&mut self, node: NodeId) {
            self.drag_payloads.remove(&node);
        }

        #[allow(dead_code, reason = "drag-drop registry accessor, not yet called; likely wired by a later events-integration milestone")]
        pub(crate) fn set_drop_accept(&mut self, node: NodeId, predicate: impl Fn(&DragPayload) -> bool + 'static) {
            self.drop_accept.insert(node, Box::new(predicate));
        }

        #[allow(dead_code, reason = "drag-drop registry accessor, not yet called; likely wired by a later events-integration milestone")]
        pub(crate) fn drag_session(&self) -> Option<&DragSession> {
            self.drag.as_ref()
        }

        /// 🫳️ Promotes a `Press` capture on a `drag_payloads`-registered node to `CaptureKind::Drag` once
        /// the pointer has moved past `DRAG_PROMOTE_THRESHOLD_SQ` from `press_origin`.
        fn maybe_promote_to_drag(&mut self, x: f32, y: f32) {
            let Some((id, CaptureKind::Press)) = self.capture.target else { return };
            let Some(payload) = self.drag_payloads.get(&id).cloned() else { return };
            let Some((origin_x, origin_y)) = self.press_origin else { return };
            if (x - origin_x).powi(2) + (y - origin_y).powi(2) < DRAG_PROMOTE_THRESHOLD_SQ {
                return;
            }
            self.capture.target = Some((id, CaptureKind::Drag));
            self.drag = Some(DragSession { source: id, payload, ghost: None, pointer_x: x, pointer_y: y, drop_target: None });
        }

        /// 🫳️ Live-updates the active `DragSession`'s pointer position and re-evaluates the drop target
        /// under it.
        fn update_drag(&mut self, tree: &UiTree, root: NodeId, x: f32, y: f32) {
            if let Some(drag) = self.drag.as_mut() {
                drag.pointer_x = x;
                drag.pointer_y = y;
            }
            let target = hit_test(tree, root, x, y).and_then(|hit| self.nearest_accepting_drop_target(tree, hit));
            if let Some(drag) = self.drag.as_mut() {
                drag.drop_target = target;
            }
        }

        /// 🎯️ Walks `from`'s bubble chain for the nearest `NodeFlags::DROP_TARGET` node whose
        /// `drop_accept` predicate (if any) accepts the active `DragSession`'s payload.
        fn nearest_accepting_drop_target(&self, tree: &UiTree, from: NodeId) -> Option<NodeId> {
            let mut found = None;
            bubble(tree, from, |id| {
                if !tree.node(id).is_some_and(|node| node.flags.contains(NodeFlags::DROP_TARGET)) {
                    return false;
                }
                let accepts = match self.drop_accept.get(&id) {
                    Some(predicate) => self.drag.as_ref().is_some_and(|drag| predicate(&drag.payload)),
                    None => true,
                };
                if accepts {
                    found = Some(id);
                    true
                } else {
                    false
                }
            });
            found
        }
        //#endregion 🔖️DragDropApi

        //#region 🔖️ScrollApi
        #[allow(dead_code, reason = "scroll-thumb registry accessor, not yet called; likely wired by a later events-integration milestone")]
        pub(crate) fn register_scroll_thumb(&mut self, thumb: NodeId, scrollable: NodeId, axis: ScrollAxis) {
            self.scroll_thumbs.insert(thumb, (scrollable, axis));
        }

        fn route_scroll(&mut self, tree: &mut UiTree, root: NodeId, x: f32, y: f32, delta_x: f32, delta_y: f32) {
            let Some(hit) = hit_test(tree, root, x, y) else { return };
            let Some(scrollable) = nearest_scrollable_ancestor(tree, hit) else { return };
            if let Some(node) = tree.node_mut(scrollable) {
                let (offset_x, offset_y) = node.state.scroll_offset;
                node.state.scroll_offset = ((offset_x + delta_x).max(0.0), (offset_y + delta_y).max(0.0));
            }
            tree.mark_dirty(scrollable, NodeFlags::DIRTY_PAINT);
        }

        fn update_scroll_thumb(&mut self, tree: &mut UiTree, scrollable: NodeId, axis: ScrollAxis, x: f32, y: f32) {
            let Some((origin_x, origin_y, start_x, start_y)) = self.thumb_start else { return };
            let (delta_x, delta_y) = (x - origin_x, y - origin_y);
            let Some(node) = tree.node_mut(scrollable) else { return };
            node.state.scroll_offset = match axis {
                ScrollAxis::Horizontal => ((start_x + delta_x).max(0.0), start_y),
                ScrollAxis::Vertical => (start_x, (start_y + delta_y).max(0.0)),
            };
            tree.mark_dirty(scrollable, NodeFlags::DIRTY_PAINT);
        }
        //#endregion 🔖️ScrollApi

        //#region 🔖️EditApi
        fn route_text_insert(&mut self, tree: &mut UiTree, text: &str) {
            let Some(id) = self.focus.focused else { return };
            let Some(node) = tree.node_mut(id) else { return };
            let Some(edit) = node.state.edit.as_mut() else { return };
            insert_at_caret(edit, text);
            tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
        }

        fn route_ime(&mut self, tree: &mut UiTree, event: &ImeEvent) {
            let Some(id) = self.focus.focused else { return };
            let Some(node) = tree.node_mut(id) else { return };
            let Some(edit) = node.state.edit.as_mut() else { return };
            match event {
                ImeEvent::Start => edit.composition = Some(String::new()),
                ImeEvent::Update { text, .. } => edit.composition = Some(text.clone()),
                ImeEvent::Commit { text } => {
                    edit.composition = None;
                    insert_at_caret(edit, text);
                }
                ImeEvent::Cancel => edit.composition = None,
            }
            tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
        }

        /// ⌨️ Caret motion (with `Shift` extending the selection), `Home`/`End`, `Backspace`/`Delete`,
        /// and clipboard shortcuts for the focused node's `EditState`. A no-operation if nothing is focused or
        /// the focused node has no `EditState` (isn't a `UiNode::Input`, or hasn't been focused since
        /// `FocusState::set_focus` seeded one).
        fn route_edit_key(&mut self, tree: &mut UiTree, key: &str, modifiers: EventModifiers) -> Vec<UiCommand> {
            let mut out = Vec::new();
            let Some(id) = self.focus.focused else { return out };
            let Some(node) = tree.node_mut(id) else { return out };
            let Some(edit) = node.state.edit.as_mut() else { return out };
            let has_selection = edit.anchor != edit.caret;
            match key {
                "ArrowLeft" => {
                    edit.caret = if has_selection && !modifiers.shift { selection_bounds(edit.anchor, edit.caret).0 } else { prev_char_boundary(&edit.text, edit.caret) };
                    if !modifiers.shift {
                        edit.anchor = edit.caret;
                    }
                }
                "ArrowRight" => {
                    edit.caret = if has_selection && !modifiers.shift { selection_bounds(edit.anchor, edit.caret).1 } else { next_char_boundary(&edit.text, edit.caret) };
                    if !modifiers.shift {
                        edit.anchor = edit.caret;
                    }
                }
                "Home" => {
                    edit.caret = 0;
                    if !modifiers.shift {
                        edit.anchor = 0;
                    }
                }
                "End" => {
                    edit.caret = edit.text.len();
                    if !modifiers.shift {
                        edit.anchor = edit.text.len();
                    }
                }
                "Backspace" => {
                    if has_selection {
                        let (start, end) = selection_bounds(edit.anchor, edit.caret);
                        edit.text.replace_range(start..end, "");
                        edit.caret = start;
                        edit.anchor = start;
                    } else if edit.caret > 0 {
                        let start = prev_char_boundary(&edit.text, edit.caret);
                        edit.text.replace_range(start..edit.caret, "");
                        edit.caret = start;
                        edit.anchor = start;
                    }
                }
                "Delete" => {
                    if has_selection {
                        let (start, end) = selection_bounds(edit.anchor, edit.caret);
                        edit.text.replace_range(start..end, "");
                        edit.caret = start;
                        edit.anchor = start;
                    } else if edit.caret < edit.text.len() {
                        let end = next_char_boundary(&edit.text, edit.caret);
                        edit.text.replace_range(edit.caret..end, "");
                    }
                }
                "c" | "C" if modifiers.ctrl || modifiers.meta => {
                    if has_selection {
                        let (start, end) = selection_bounds(edit.anchor, edit.caret);
                        out.push(UiCommand::ClipboardCopy { window_id: self.window_id.clone(), text: edit.text[start..end].to_string() });
                    }
                    return out;
                }
                "x" | "X" if modifiers.ctrl || modifiers.meta => {
                    if has_selection {
                        let (start, end) = selection_bounds(edit.anchor, edit.caret);
                        out.push(UiCommand::ClipboardCut { window_id: self.window_id.clone(), text: edit.text[start..end].to_string() });
                        edit.text.replace_range(start..end, "");
                        edit.caret = start;
                        edit.anchor = start;
                    } else {
                        return out;
                    }
                }
                "v" | "V" if modifiers.ctrl || modifiers.meta => {
                    out.push(UiCommand::ClipboardPasteRequested { window_id: self.window_id.clone() });
                    return out;
                }
                _ => return out,
            }
            tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
            out
        }
        //#endregion 🔖️EditApi

        //#region 🔖️CursorApi
        #[allow(dead_code, reason = "cursor-state accessor, not yet called; likely wired by a later events-integration milestone")]
        pub(crate) fn hovered(&self) -> Option<NodeId> {
            self.hovered
        }

        #[allow(dead_code, reason = "cursor-state accessor, not yet called; likely wired by a later events-integration milestone")]
        pub(crate) fn capture(&self) -> Option<(NodeId, CaptureKind)> {
            self.capture.target
        }

        /// 🎯️ Read-only: whether this window's retained content currently holds keyboard focus — see
        /// `engine::Ui::window_has_focus` (its only caller), added for the `w2-input-wiring` host-side
        /// focus arbitration (content vs. chrome routing, `.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY`).
        pub(crate) fn is_focused(&self) -> bool {
            self.focus.focused.is_some()
        }
        //#endregion 🔖️CursorApi

        /// 🧹️ Drops registry entries (`drag_payloads`/`drop_accept`/`scroll_thumbs`) keyed by a `NodeId`
        /// `reconcile` has since removed from `tree` — generation-tagged `NodeId`s (see `arena`'s own doc
        /// comment) make stale entries harmless to *use* (they simply never match a live node again), but
        /// this keeps the maps from growing unboundedly across a long session's worth of churn.
        fn prune_dead_registrations(&mut self, tree: &UiTree) {
            self.drag_payloads.retain(|id, _| tree.contains(*id));
            self.drop_accept.retain(|id, _| tree.contains(*id));
            self.scroll_thumbs.retain(|thumb, (scrollable, _)| tree.contains(*thumb) && tree.contains(*scrollable));
        }

        /// 🚦️ Resolves the event's target (capture target if captured, else `hit_test`), updates
        /// interaction flags, and returns any `UiCommand`s the event produced.
        pub(crate) fn dispatch(&mut self, tree: &mut UiTree, root: NodeId, event: &UiEvent) -> Vec<UiCommand> {
            self.prune_dead_registrations(tree);
            let mut commands = Vec::new();
            match event {
                UiEvent::PointerMove { x, y } => {
                    self.maybe_promote_to_drag(*x, *y);
                    match self.capture.target {
                        Some((_, CaptureKind::Drag)) => self.update_drag(tree, root, *x, *y),
                        Some((scrollable, CaptureKind::ScrollThumb(axis))) => self.update_scroll_thumb(tree, scrollable, axis, *x, *y),
                        _ => {}
                    }
                    let target = self.resolve_target(tree, root, *x, *y);
                    if let Some(id) = target {
                        if let Some(cmd) = self.scene_command(tree, id, event) {
                            commands.push(cmd);
                        }
                    }
                    commands.extend(self.update_hover(tree, target));
                    commands.extend(self.maybe_dismiss_tooltip_on_hover_out(tree, *x, *y));
                }
                UiEvent::PointerDown { x, y, .. } => {
                    if let Some(dismissed) = self.dismiss_topmost_if_outside_press(tree, *x, *y) {
                        return dismissed;
                    }
                    self.press_origin = Some((*x, *y));
                    let target = hit_test(tree, root, *x, *y);
                    commands.extend(self.update_hover(tree, target));
                    if let Some(id) = target {
                        if let Some(cmd) = self.scene_command(tree, id, event) {
                            commands.push(cmd);
                        }
                        if let Some(&(scrollable, axis)) = self.scroll_thumbs.get(&id) {
                            let offset = tree.node(scrollable).map(|node| node.state.scroll_offset).unwrap_or_default();
                            self.capture.target = Some((scrollable, CaptureKind::ScrollThumb(axis)));
                            self.thumb_start = Some((*x, *y, offset.0, offset.1));
                        } else {
                            if let Some(node) = tree.node_mut(id) {
                                node.flags.set(NodeFlags::ACTIVE, true);
                            }
                            tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
                            self.capture.target = Some((id, CaptureKind::Press));
                            let focusable = tree.node(id).is_some_and(|node| is_focusable(&node.spec.0));
                            if focusable {
                                self.focus.set_focus(tree, Some(id));
                                commands.push(UiCommand::FocusChanged { window_id: self.window_id.clone(), node: Some(id) });
                            }
                            // 🫳️ W2 wiring: a `Tree` row's `draggable`/`drag_data` (re-derived by key —
                            // see `find_tree_item_spec`) registers this press as a promotable drag
                            // source, exactly like a widget spec would call `set_drag_payload` itself if
                            // `UiStackNode` had room for the field (it doesn't — see that fn's own doc).
                            if let Some(item) = find_tree_item_spec(tree, id) {
                                if item.draggable.unwrap_or(false) {
                                    self.set_drag_payload(id, item.drag_data.clone().unwrap_or_default());
                                }
                            }
                        }
                    } else {
                        self.focus.clear_focus(tree);
                        commands.push(UiCommand::FocusChanged { window_id: self.window_id.clone(), node: None });
                    }
                }
                UiEvent::PointerUp { x, y, .. } => {
                    if let Some((active_id, kind)) = self.capture.release() {
                        match kind {
                            CaptureKind::Press => {
                                if let Some(node) = tree.node_mut(active_id) {
                                    node.flags.set(NodeFlags::ACTIVE, false);
                                }
                                tree.mark_dirty(active_id, NodeFlags::DIRTY_PAINT);
                                if hit_test(tree, root, *x, *y) == Some(active_id) {
                                    // 🔽️🎴️ W2 wiring: `Select` toggles its popup (`toggle_select_popup`);
                                    // a `Button` (this covers `Select`'s own synthesized item rows too —
                                    // see `reconcile::children_of`'s `Select` arm — since they're plain
                                    // `UiNode::Button`s) fires its action, additionally closing an open
                                    // `SelectPopup` if this button *is* one of that popup's rows (picking
                                    // an item closes the popup, per `toggle_select_popup`'s doc comment);
                                    // a `Stack` with `activate` set fires that action (see
                                    // `paint::paint_stack_frame`'s matching visual for the same field).
                                    let is_select = tree.node(active_id).is_some_and(|node| matches!(node.spec.0, UiNode::Select(_)));
                                    if is_select {
                                        commands.extend(self.toggle_select_popup(tree, active_id));
                                    } else {
                                        let fired = tree.node(active_id).and_then(|node| match &node.spec.0 {
                                            UiNode::Button(button) => Some((button.action.clone(), node.parent)),
                                            UiNode::Stack(stack) => stack.activate.clone().map(|action| (action, None)),
                                            _ => None,
                                        });
                                        if let Some((action, parent)) = fired {
                                            commands.push(UiCommand::App { window_id: self.window_id.clone(), action });
                                            if let Some(parent) = parent {
                                                let picked_from_open_select = self.overlays.topmost().is_some_and(|overlay| overlay.kind == OverlayKind::SelectPopup && overlay.root == parent);
                                                if picked_from_open_select {
                                                    commands.extend(self.close_topmost_overlay(tree));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            CaptureKind::Drag => {
                                if let Some(node) = tree.node_mut(active_id) {
                                    node.flags.set(NodeFlags::ACTIVE, false);
                                }
                                tree.mark_dirty(active_id, NodeFlags::DIRTY_PAINT);
                                if let Some(drag) = self.drag.take() {
                                    commands.push(match drag.drop_target {
                                        Some(target) => UiCommand::DropCommitted { window_id: self.window_id.clone(), source: drag.source, target, payload: drag.payload },
                                        None => UiCommand::DropCancelled { window_id: self.window_id.clone(), source: drag.source },
                                    });
                                }
                            }
                            CaptureKind::ScrollThumb(_) => {
                                self.thumb_start = None;
                            }
                        }
                    }
                    let target = self.resolve_target(tree, root, *x, *y);
                    if let Some(id) = target {
                        if let Some(cmd) = self.scene_command(tree, id, event) {
                            commands.push(cmd);
                        }
                    }
                    commands.extend(self.update_hover(tree, target));
                }
                UiEvent::KeyDown { key, modifiers } => {
                    if key == "Escape" {
                        commands.extend(self.close_topmost_overlay(tree));
                    } else if key == "Tab" {
                        let scope = self.overlays.topmost_focus_trap_root().unwrap_or(root);
                        if modifiers.shift {
                            self.focus.focus_prev(tree, scope);
                        } else {
                            self.focus.focus_next(tree, scope);
                        }
                        commands.push(UiCommand::FocusChanged { window_id: self.window_id.clone(), node: self.focus.focused });
                    } else {
                        commands.extend(self.route_edit_key(tree, key, *modifiers));
                    }
                }
                UiEvent::KeyUp { .. } => {}
                UiEvent::TextInput { text } => self.route_text_insert(tree, text),
                UiEvent::Paste { text } => self.route_text_insert(tree, text),
                UiEvent::Ime(ime_event) => self.route_ime(tree, ime_event),
                UiEvent::Scroll { x, y, delta_x, delta_y } => {
                    if let Some(id) = hit_test(tree, root, *x, *y) {
                        if let Some(cmd) = self.scene_command(tree, id, event) {
                            commands.push(cmd);
                        }
                    }
                    self.route_scroll(tree, root, *x, *y, *delta_x, *delta_y);
                }
            }
            commands
        }

        /// 🎬️ If `id` is a `ComponentScene` leaf, resolves its `SurfaceKind`/absolute rect (the same
        /// ancestor-offset accumulation `scene_slots::collect_scene_slots`/`hit_test_node`/
        /// `paint::paint_node` each do independently — not reusing `collect_scene_slots` itself, since
        /// that walks the WHOLE tree per call and this runs once per real input event) and builds the
        /// `UiCommand::Scene` the host should route into that surface's per-`SurfaceKind` input handler.
        fn scene_command(&self, tree: &UiTree, id: NodeId, event: &UiEvent) -> Option<UiCommand> {
            let node = tree.node(id)?;
            let UiNode::ComponentScene(scene) = &node.spec.0 else { return None };
            let mut x = node.layout.x;
            let mut y = node.layout.y;
            let mut current = node.parent;
            while let Some(parent_id) = current {
                let parent = tree.node(parent_id)?;
                x += parent.layout.x;
                y += parent.layout.y;
                current = parent.parent;
            }
            Some(UiCommand::Scene { window_id: self.window_id.clone(), node: id, surface_id: scene.surface_id.clone(), kind: scene.component_kind, rect: Rect::new(x, y, node.layout.width, node.layout.height), event: event.clone() })
        }
    }
    //#endregion 🔖️UiCommand

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::component::ui::{UiButtonNode, UiComponentSceneNode, UiInputNode, UiPresence, UiSelectItem, UiSelectNode, UiSeparatorNode, UiStackNode, UiTextNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode};
        use crate::tree::{Node, NodeKey, WidgetSpec};

        fn action() -> ActionDescriptor {
            ActionDescriptor { controller_id: "ctrl".into(), action: "go".into(), args: None }
        }

        fn select_ui(id: &str, value: &str) -> UiNode {
            UiNode::Select(UiSelectNode {
                id: id.into(),
                value: value.into(),
                items: vec![UiSelectItem { value: "a".into(), label: "A".into() }, UiSelectItem { value: "b".into(), label: "B".into() }],
                placeholder: None,
                on_change: action(),
                presence: UiPresence::default(),
                menu: None,
            })
        }

        fn tree_ui(sections: Vec<UiTreeSectionNode>) -> UiNode {
            UiNode::Tree(UiTreeNode { sections, presence: UiPresence::default(), selected_ids: None, highlighted_ids: None, selection_change: None, drop_action: None, menu: None })
        }

        /// 🌳️ Manually inserts a `Tree` row `Stack` (mirroring `reconcile::tree_item_row`'s synthesized
        /// shape/key exactly) as a retained child of `tree_id` — these tests build the retained tree by
        /// hand (like every other test in this module, via `leaf`), so `reconcile` never actually runs;
        /// this stand-in keeps the row's key (`NodeKey::Explicit(item.id)`) and geometry consistent with
        /// what `paint::sync_tree_row_layout` would have written.
        fn insert_tree_row(tree: &mut UiTree, tree_id: NodeId, item_id: &str, rect: (f32, f32, f32, f32)) -> NodeId {
            let spec =
                UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: Some(item_id.into()), presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children: Vec::new(), menu: None });
            let id = tree.insert_child(Some(tree_id), Node::new(NodeKey::Explicit(item_id.into()), WidgetSpec(spec)));
            let bucket = tree.node_mut(id).unwrap();
            bucket.layout.x = rect.0;
            bucket.layout.y = rect.1;
            bucket.layout.width = rect.2;
            bucket.layout.height = rect.3;
            id
        }

        fn input_ui(id: &str, value: &str) -> UiNode {
            UiNode::Input(UiInputNode {
                id: id.into(),
                input_kind: "text".into(),
                value: value.into(),
                placeholder: None,
                commit: None,
                min: None,
                max: None,
                step: None,
                accept: None,
                on_change: action(),
                presence: UiPresence::default(),
                menu: None,
            })
        }

        fn stack_ui() -> UiNode {
            UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children: Vec::new(), menu: None })
        }

        fn text_ui(value: &str) -> UiNode {
            UiNode::Text(UiTextNode { value: value.into(), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })
        }

        fn separator_ui() -> UiNode {
            UiNode::Separator(UiSeparatorNode { presence: UiPresence::default(), menu: None })
        }

        fn button_ui(id: &str) -> UiNode {
            UiNode::Button(UiButtonNode { id: Some(id.into()), icon_id: IconName::CircleDot, label: id.into(), action: action(), style: None, presence: UiPresence::default(), menu: None })
        }

        fn leaf(tree: &mut UiTree, parent: Option<NodeId>, ordinal: u32, node: UiNode, rect: (f32, f32, f32, f32)) -> NodeId {
            let id = tree.insert_child(parent, Node::new(NodeKey::Positional(ordinal, ordinal), WidgetSpec(node)));
            let bucket = tree.node_mut(id).unwrap();
            bucket.layout.x = rect.0;
            bucket.layout.y = rect.1;
            bucket.layout.width = rect.2;
            bucket.layout.height = rect.3;
            id
        }

        fn set_flag(tree: &mut UiTree, id: NodeId, flag: NodeFlags) {
            tree.node_mut(id).unwrap().flags.set(flag, true);
        }

        #[test]
        fn hit_test_finds_the_topmost_of_two_non_overlapping_siblings() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            let left = leaf(&mut tree, Some(root), 1, text_ui("left"), (0.0, 0.0, 100.0, 100.0));
            let right = leaf(&mut tree, Some(root), 2, text_ui("right"), (100.0, 0.0, 100.0, 100.0));

            assert_eq!(hit_test(&tree, root, 50.0, 50.0), Some(left));
            assert_eq!(hit_test(&tree, root, 150.0, 50.0), Some(right));
        }

        #[test]
        fn hit_test_respects_clips_children_pruning() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            let clipper = leaf(&mut tree, Some(root), 1, stack_ui(), (0.0, 0.0, 50.0, 50.0));
            set_flag(&mut tree, clipper, NodeFlags::CLIPS_CHILDREN);
            // child's own rect extends far outside the clipper's 50x50 bounds.
            let overflowing_child = leaf(&mut tree, Some(clipper), 1, text_ui("overflow"), (0.0, 0.0, 500.0, 500.0));

            assert_eq!(hit_test(&tree, root, 400.0, 400.0), None, "point outside the clipper must not match the overflowing child");
            assert_eq!(hit_test(&tree, root, 10.0, 10.0), Some(overflowing_child), "inside the clip bounds the child still matches");
        }

        #[test]
        fn hit_test_skips_hit_transparent_node_but_still_matches_its_children() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            let overlay_glass = leaf(&mut tree, Some(root), 1, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            set_flag(&mut tree, overlay_glass, NodeFlags::HIT_TRANSPARENT);
            let child = leaf(&mut tree, Some(overlay_glass), 1, text_ui("under-glass"), (10.0, 10.0, 50.0, 50.0));

            assert_eq!(hit_test(&tree, root, 30.0, 30.0), Some(child));
            assert_eq!(hit_test(&tree, root, 150.0, 150.0), None, "hit-transparent node itself must never match outside its children");
        }

        #[test]
        fn capture_routes_move_and_up_to_the_captured_node_regardless_of_pointer_position() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            let a = leaf(&mut tree, Some(root), 1, separator_ui(), (0.0, 0.0, 100.0, 100.0));
            let _b = leaf(&mut tree, Some(root), 2, separator_ui(), (100.0, 0.0, 100.0, 100.0));
            let mut router = EventRouter::new("main");

            router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 50.0, y: 50.0, button: PointerButton::Primary });
            assert_eq!(router.capture.target.map(|(id, _)| id), Some(a));

            // pointer moved far outside `a`'s bounds and into `b`'s — capture must still target `a`.
            router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 150.0, y: 50.0 });
            assert_eq!(router.hovered, Some(a));

            router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 150.0, y: 50.0, button: PointerButton::Primary });
            assert_eq!(router.capture.target, None, "capture releases on PointerUp");
        }

        #[test]
        fn focus_next_and_prev_cycle_only_through_focusable_nodes() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 300.0, 100.0));
            leaf(&mut tree, Some(root), 1, text_ui("not focusable"), (0.0, 0.0, 50.0, 20.0));
            let button_a = leaf(&mut tree, Some(root), 2, button_ui("a"), (50.0, 0.0, 50.0, 20.0));
            leaf(&mut tree, Some(root), 3, separator_ui(), (100.0, 0.0, 50.0, 20.0));
            let button_b = leaf(&mut tree, Some(root), 4, button_ui("b"), (150.0, 0.0, 50.0, 20.0));
            let mut focus = FocusState::new();

            focus.focus_next(&mut tree, root);
            assert_eq!(focus.focused, Some(button_a));
            focus.focus_next(&mut tree, root);
            assert_eq!(focus.focused, Some(button_b));
            focus.focus_next(&mut tree, root);
            assert_eq!(focus.focused, Some(button_a), "cycles back to the first focusable node");

            focus.focus_prev(&mut tree, root);
            assert_eq!(focus.focused, Some(button_b), "wraps to the last focusable node going backwards");
        }

        #[test]
        fn set_focus_flips_the_focused_flag_in_both_directions() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 100.0, 100.0));
            let a = leaf(&mut tree, Some(root), 1, button_ui("a"), (0.0, 0.0, 50.0, 20.0));
            let b = leaf(&mut tree, Some(root), 2, button_ui("b"), (0.0, 20.0, 50.0, 20.0));
            let mut focus = FocusState::new();

            focus.set_focus(&mut tree, Some(a));
            assert!(tree.node(a).unwrap().flags.contains(NodeFlags::FOCUSED));
            assert!(!tree.node(b).unwrap().flags.contains(NodeFlags::FOCUSED));

            focus.set_focus(&mut tree, Some(b));
            assert!(!tree.node(a).unwrap().flags.contains(NodeFlags::FOCUSED), "moving focus away must clear the old node's flag");
            assert!(tree.node(b).unwrap().flags.contains(NodeFlags::FOCUSED));

            focus.clear_focus(&mut tree);
            assert!(!tree.node(b).unwrap().flags.contains(NodeFlags::FOCUSED), "clearing focus must clear the flag, not just the router's own field");
        }

        #[test]
        fn clicking_a_button_emits_its_action_descriptor_as_a_ui_command() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 100.0, 100.0));
            let button = leaf(&mut tree, Some(root), 1, button_ui("go"), (0.0, 0.0, 100.0, 40.0));
            let mut router = EventRouter::new("main");

            router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
            let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary });

            let expected = action();
            assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::App { window_id, action } if window_id == "main" && *action == expected)));
            let _ = button;
        }

        #[test]
        fn releasing_off_the_captured_button_does_not_fire_its_action() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 100.0, 100.0));
            leaf(&mut tree, Some(root), 1, button_ui("go"), (0.0, 0.0, 40.0, 40.0));
            let mut router = EventRouter::new("main");

            router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
            let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 90.0, y: 90.0, button: PointerButton::Primary });

            assert!(commands.iter().all(|cmd| !matches!(cmd, UiCommand::App { .. })), "release outside the pressed button must not fire its action");
        }

        #[test]
        fn bubble_stops_when_a_handler_returns_true() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 100.0, 100.0));
            let mid = leaf(&mut tree, Some(root), 1, stack_ui(), (0.0, 0.0, 100.0, 100.0));
            let leaf_node = leaf(&mut tree, Some(mid), 1, text_ui("leaf"), (0.0, 0.0, 20.0, 20.0));

            let mut visited = Vec::new();
            bubble(&tree, leaf_node, |id| {
                visited.push(id);
                id == mid
            });

            assert_eq!(visited, vec![leaf_node, mid], "bubbling must stop at `mid` and never reach `root`");
        }

        //#region 🔖️OverlayTests
        #[test]
        fn overlay_open_flags_the_node_and_close_clears_it_and_emits_overlay_closed() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            let popup = leaf(&mut tree, Some(root), 1, stack_ui(), (10.0, 10.0, 50.0, 50.0));
            let mut router = EventRouter::new("main");

            router.open_overlay(&mut tree, popup, OverlayKind::SelectPopup, OverlayAnchor::Node(root));
            assert!(tree.node(popup).unwrap().flags.contains(NodeFlags::OVERLAY));
            assert_eq!(router.topmost_overlay().map(|overlay| overlay.kind), Some(OverlayKind::SelectPopup));

            let commands = router.close_topmost_overlay(&mut tree);
            assert!(!tree.node(popup).unwrap().flags.contains(NodeFlags::OVERLAY));
            assert!(router.topmost_overlay().is_none());
            assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::OverlayClosed { root, kind, .. } if *root == popup && *kind == OverlayKind::SelectPopup)));
        }

        #[test]
        fn pointer_down_outside_a_dismissable_overlay_closes_it_and_swallows_the_press() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            leaf(&mut tree, Some(root), 1, button_ui("underneath"), (0.0, 0.0, 200.0, 200.0));
            let popup = leaf(&mut tree, Some(root), 2, stack_ui(), (10.0, 10.0, 50.0, 50.0));
            leaf(&mut tree, Some(popup), 1, text_ui("item"), (0.0, 0.0, 50.0, 50.0));
            let mut router = EventRouter::new("main");
            router.open_overlay(&mut tree, popup, OverlayKind::SelectPopup, OverlayAnchor::Node(root));

            let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 150.0, y: 150.0, button: PointerButton::Primary });

            assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::OverlayClosed { .. })), "outside press must close the overlay");
            assert!(router.topmost_overlay().is_none());
            assert_eq!(router.capture(), None, "the outside press must be swallowed, not routed to whatever's underneath");
        }

        #[test]
        fn pointer_down_inside_a_dismissable_overlay_does_not_close_it() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            let popup = leaf(&mut tree, Some(root), 1, stack_ui(), (10.0, 10.0, 50.0, 50.0));
            leaf(&mut tree, Some(popup), 1, text_ui("item"), (0.0, 0.0, 50.0, 50.0));
            let mut router = EventRouter::new("main");
            router.open_overlay(&mut tree, popup, OverlayKind::SelectPopup, OverlayAnchor::Node(root));

            let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 20.0, y: 20.0, button: PointerButton::Primary });

            assert!(commands.iter().all(|cmd| !matches!(cmd, UiCommand::OverlayClosed { .. })), "a press inside the overlay must not dismiss it");
            assert!(router.topmost_overlay().is_some());
        }

        #[test]
        fn escape_closes_only_the_topmost_of_two_open_overlays() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            let menu = leaf(&mut tree, Some(root), 1, stack_ui(), (0.0, 0.0, 50.0, 50.0));
            let submenu = leaf(&mut tree, Some(root), 2, stack_ui(), (60.0, 0.0, 50.0, 50.0));
            let mut router = EventRouter::new("main");
            router.open_overlay(&mut tree, menu, OverlayKind::ContextMenu, OverlayAnchor::Node(root));
            router.open_overlay(&mut tree, submenu, OverlayKind::ContextMenu, OverlayAnchor::Node(menu));

            let commands = router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Escape".into(), modifiers: EventModifiers::default() });

            assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::OverlayClosed { root, .. } if *root == submenu)));
            assert_eq!(router.topmost_overlay().map(|overlay| overlay.root), Some(menu), "only the topmost overlay closes on Escape");
        }

        #[test]
        fn tab_focus_is_trapped_inside_an_open_dialog_overlay() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 300.0, 300.0));
            leaf(&mut tree, Some(root), 1, button_ui("a"), (0.0, 0.0, 50.0, 20.0));
            leaf(&mut tree, Some(root), 2, button_ui("b"), (50.0, 0.0, 50.0, 20.0));
            let dialog = leaf(&mut tree, Some(root), 3, stack_ui(), (100.0, 100.0, 100.0, 100.0));
            let button_c = leaf(&mut tree, Some(dialog), 1, button_ui("c"), (0.0, 0.0, 50.0, 20.0));
            let button_d = leaf(&mut tree, Some(dialog), 2, button_ui("d"), (50.0, 0.0, 50.0, 20.0));
            let mut router = EventRouter::new("main");
            router.open_overlay(&mut tree, dialog, OverlayKind::Dialog, OverlayAnchor::Point { x: 0.0, y: 0.0 });

            let tab = || UiEvent::KeyDown { key: "Tab".into(), modifiers: EventModifiers::default() };
            router.dispatch(&mut tree, root, &tab());
            assert_eq!(router.focus.focused, Some(button_c));
            router.dispatch(&mut tree, root, &tab());
            assert_eq!(router.focus.focused, Some(button_d));
            router.dispatch(&mut tree, root, &tab());
            assert_eq!(router.focus.focused, Some(button_c), "focus-trapped Tab cycling must never reach button_a/button_b outside the dialog");
        }
        //#endregion 🔖️OverlayTests

        //#region 🔖️DragDropTests
        #[test]
        fn drag_session_promotes_after_threshold_and_commits_on_an_accepting_drop_target() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            let source = leaf(&mut tree, Some(root), 1, text_ui("drag-me"), (0.0, 0.0, 20.0, 20.0));
            set_flag(&mut tree, source, NodeFlags::DRAG_SOURCE);
            let target = leaf(&mut tree, Some(root), 2, stack_ui(), (100.0, 100.0, 50.0, 50.0));
            set_flag(&mut tree, target, NodeFlags::DROP_TARGET);
            leaf(&mut tree, Some(target), 1, text_ui("drop-here"), (0.0, 0.0, 50.0, 50.0));
            let mut router = EventRouter::new("main");
            let mut payload = DragPayload::new();
            payload.insert("application/x-semio-catalogue-item".into(), "{\"id\":\"abc\"}".into());
            router.set_drag_payload(source, payload.clone());

            router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 5.0, y: 5.0, button: PointerButton::Primary });
            assert_eq!(router.capture(), Some((source, CaptureKind::Press)), "a plain press must not immediately start a drag");

            // Small move under the promotion threshold: still just a Press.
            router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 6.0, y: 6.0 });
            assert_eq!(router.capture(), Some((source, CaptureKind::Press)));

            // Move past the threshold and over the drop target: promotes to Drag and finds the target.
            router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 120.0, y: 120.0 });
            assert_eq!(router.capture(), Some((source, CaptureKind::Drag)));
            assert_eq!(router.drag_session().and_then(|drag| drag.drop_target), Some(target));

            let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 120.0, y: 120.0, button: PointerButton::Primary });
            assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::DropCommitted { source: s, target: t, payload: p, .. } if *s == source && *t == target && *p == payload)));
            assert_eq!(router.capture(), None);
            assert!(router.drag_session().is_none());
        }

        #[test]
        fn drag_session_cancels_when_released_over_no_accepting_drop_target() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0_f32, 200.0));
            let source = leaf(&mut tree, Some(root), 1, text_ui("drag-me"), (0.0, 0.0, 20.0, 20.0));
            let mut router = EventRouter::new("main");
            router.set_drag_payload(source, DragPayload::new());

            router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 5.0, y: 5.0, button: PointerButton::Primary });
            router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 190.0, y: 190.0 });
            assert_eq!(router.capture(), Some((source, CaptureKind::Drag)));

            let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 190.0, y: 190.0, button: PointerButton::Primary });
            assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::DropCancelled { source: s, .. } if *s == source)));
        }

        #[test]
        fn a_drop_targets_accept_predicate_can_reject_the_active_payload() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            let source = leaf(&mut tree, Some(root), 1, text_ui("drag-me"), (0.0, 0.0, 20.0, 20.0));
            let target = leaf(&mut tree, Some(root), 2, stack_ui(), (100.0, 100.0, 50.0, 50.0));
            set_flag(&mut tree, target, NodeFlags::DROP_TARGET);
            leaf(&mut tree, Some(target), 1, text_ui("drop-here"), (0.0, 0.0, 50.0, 50.0));
            let mut router = EventRouter::new("main");
            router.set_drag_payload(source, DragPayload::from([("application/x-semio-tree-section-reorder".to_string(), "x".to_string())]));
            router.set_drop_accept(target, |payload| payload.contains_key("application/x-semio-catalogue-item"));

            router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 5.0, y: 5.0, button: PointerButton::Primary });
            router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 120.0, y: 120.0 });

            assert_eq!(router.drag_session().and_then(|drag| drag.drop_target), None, "the predicate must reject this payload's mime key");
        }
        //#endregion 🔖️DragDropTests

        //#region 🔖️ScrollTests
        #[test]
        fn scroll_routes_to_the_nearest_scrollable_ancestor_and_clamps_at_zero() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            set_flag(&mut tree, root, NodeFlags::SCROLLABLE);
            leaf(&mut tree, Some(root), 1, text_ui("content"), (10.0, 10.0, 20.0, 20.0));
            let mut router = EventRouter::new("main");

            router.dispatch(&mut tree, root, &UiEvent::Scroll { x: 15.0, y: 15.0, delta_x: 0.0, delta_y: 30.0 });
            assert_eq!(tree.node(root).unwrap().state.scroll_offset, (0.0, 30.0));

            router.dispatch(&mut tree, root, &UiEvent::Scroll { x: 15.0, y: 15.0, delta_x: 0.0, delta_y: -100.0 });
            assert_eq!(tree.node(root).unwrap().state.scroll_offset, (0.0, 0.0), "scroll offset must clamp at zero, not go negative");
        }

        #[test]
        fn scroll_thumb_capture_drags_the_scrollable_offset_along_its_registered_axis() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            set_flag(&mut tree, root, NodeFlags::SCROLLABLE);
            // A bare `Stack` is a `hit_test`-transparent pass-through container (see 🔖️HitTest's
            // `is_plain_container`) — the thumb needs to be a real leaf to be hit-testable itself.
            let thumb = leaf(&mut tree, Some(root), 1, separator_ui(), (190.0, 0.0, 10.0, 40.0));
            let mut router = EventRouter::new("main");
            router.register_scroll_thumb(thumb, root, ScrollAxis::Vertical);

            router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 195.0, y: 5.0, button: PointerButton::Primary });
            assert_eq!(router.capture(), Some((root, CaptureKind::ScrollThumb(ScrollAxis::Vertical))));

            router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 195.0, y: 25.0 });
            assert_eq!(tree.node(root).unwrap().state.scroll_offset, (0.0, 20.0));

            router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 195.0, y: 25.0, button: PointerButton::Primary });
            assert_eq!(router.capture(), None);
        }
        //#endregion 🔖️ScrollTests

        //#region 🔖️EditStateTests
        #[test]
        fn focusing_an_input_seeds_edit_state_from_its_value_and_blur_clears_it() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            let input = leaf(&mut tree, Some(root), 1, input_ui("name", "hello"), (0.0, 0.0, 100.0, 20.0));
            let mut router = EventRouter::new("main");

            router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
            assert_eq!(tree.node(input).unwrap().state.edit, Some(EditState { text: "hello".into(), caret: 5, anchor: 5, composition: None, scroll_x: 0.0 }));

            // clicking empty space blurs.
            router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 190.0, y: 190.0, button: PointerButton::Primary });
            assert_eq!(tree.node(input).unwrap().state.edit, None, "blur must relinquish the buffer so the declarative value governs again");
        }

        #[test]
        fn arrow_keys_move_the_caret_and_backspace_deletes_the_previous_char() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            leaf(&mut tree, Some(root), 1, input_ui("name", "abc"), (0.0, 0.0, 100.0, 20.0));
            let mut router = EventRouter::new("main");
            router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
            let input = router.focus.focused.unwrap();

            router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "ArrowLeft".into(), modifiers: EventModifiers::default() });
            assert_eq!(tree.node(input).unwrap().state.edit.as_ref().unwrap().caret, 2);

            router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "ArrowLeft".into(), modifiers: EventModifiers { shift: true, ..Default::default() } });
            let edit = tree.node(input).unwrap().state.edit.clone().unwrap();
            assert_eq!((edit.anchor, edit.caret), (2, 1), "shift+arrow extends the selection instead of collapsing it");

            router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Backspace".into(), modifiers: EventModifiers::default() });
            let edit = tree.node(input).unwrap().state.edit.clone().unwrap();
            assert_eq!(edit.text, "ac", "backspace over a selection deletes the selected range");
            assert_eq!((edit.anchor, edit.caret), (1, 1));
        }

        #[test]
        fn character_insertion_replaces_the_selection() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            leaf(&mut tree, Some(root), 1, input_ui("name", "abc"), (0.0, 0.0, 100.0, 20.0));
            let mut router = EventRouter::new("main");
            router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
            let input = router.focus.focused.unwrap();

            router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Home".into(), modifiers: EventModifiers::default() });
            router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "End".into(), modifiers: EventModifiers { shift: true, ..Default::default() } });
            router.dispatch(&mut tree, root, &UiEvent::TextInput { text: "xyz".into() });

            let edit = tree.node(input).unwrap().state.edit.clone().unwrap();
            assert_eq!(edit.text, "xyz");
            assert_eq!((edit.anchor, edit.caret), (3, 3));
        }

        #[test]
        fn copy_over_a_selection_emits_a_clipboard_command_without_mutating_the_buffer() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            leaf(&mut tree, Some(root), 1, input_ui("name", "hello"), (0.0, 0.0, 100.0, 20.0));
            let mut router = EventRouter::new("main");
            router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
            let input = router.focus.focused.unwrap();

            router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Home".into(), modifiers: EventModifiers::default() });
            router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "End".into(), modifiers: EventModifiers { shift: true, ..Default::default() } });
            let commands = router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "c".into(), modifiers: EventModifiers { ctrl: true, ..Default::default() } });

            assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::ClipboardCopy { text, .. } if text == "hello")));
            assert_eq!(tree.node(input).unwrap().state.edit.as_ref().unwrap().text, "hello", "copy must not mutate the buffer");
        }

        #[test]
        fn ime_commit_inserts_the_composed_text_and_clears_composition() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            leaf(&mut tree, Some(root), 1, input_ui("name", ""), (0.0, 0.0, 100.0, 20.0));
            let mut router = EventRouter::new("main");
            router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
            let input = router.focus.focused.unwrap();

            router.dispatch(&mut tree, root, &UiEvent::Ime(ImeEvent::Start));
            router.dispatch(&mut tree, root, &UiEvent::Ime(ImeEvent::Update { text: "ねこ".into(), cursor: 2 }));
            assert_eq!(tree.node(input).unwrap().state.edit.as_ref().unwrap().composition.as_deref(), Some("ねこ"));

            router.dispatch(&mut tree, root, &UiEvent::Ime(ImeEvent::Commit { text: "ねこ".into() }));
            let edit = tree.node(input).unwrap().state.edit.clone().unwrap();
            assert_eq!(edit.text, "ねこ");
            assert_eq!(edit.composition, None);
        }
        //#endregion 🔖️EditStateTests

        //#region 🔖️HoverRevealTests
        #[test]
        fn hovering_a_leaf_marks_its_whole_ancestor_chain_hovered_and_clearing_hover_clears_it_all() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 100.0, 100.0));
            let row = leaf(&mut tree, Some(root), 1, stack_ui(), (0.0, 0.0, 100.0, 100.0));
            let label = leaf(&mut tree, Some(row), 1, text_ui("item"), (0.0, 0.0, 50.0, 20.0));
            let mut router = EventRouter::new("main");

            router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 10.0, y: 10.0 });
            assert!(tree.node(label).unwrap().flags.contains(NodeFlags::HOVERED));
            assert!(tree.node(row).unwrap().flags.contains(NodeFlags::HOVERED), "an ancestor Stack row must observe hover too, for paint's reveal-on-hover");
            assert!(tree.node(root).unwrap().flags.contains(NodeFlags::HOVERED));

            router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 500.0, y: 500.0 });
            assert!(!tree.node(label).unwrap().flags.contains(NodeFlags::HOVERED));
            assert!(!tree.node(row).unwrap().flags.contains(NodeFlags::HOVERED));
            assert!(!tree.node(root).unwrap().flags.contains(NodeFlags::HOVERED));
        }
        //#endregion 🔖️HoverRevealTests

        //#region 🔖️W2InteractivityTests
        // 🔽️🎴️🌳️ Tests for the wiring closed out per `.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY`'s W2
        // pass: `Select` popup open/close (`toggle_select_popup`/`finish_close`), `Stack`
        // `activate`/`drop_action` (`is_plain_stack_container`'s hit-test exception), and `Tree` row
        // `hover_action`/`unhover_action`/`draggable` (`find_tree_item_spec`).

        #[test]
        fn clicking_a_select_opens_its_popup_and_clicking_again_closes_it() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            let select = leaf(&mut tree, Some(root), 1, select_ui("sel", "a"), (0.0, 0.0, 100.0, 30.0));
            let mut router = EventRouter::new("main");

            router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
            router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary });
            assert!(tree.node(select).unwrap().state.open, "clicking a closed select should open its popup");
            assert!(tree.node(select).unwrap().flags.contains(NodeFlags::OVERLAY), "an open select's popup subtree should win hit-test priority over its siblings");

            router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
            router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary });
            assert!(!tree.node(select).unwrap().state.open, "clicking an open select's trigger again should close its popup");
            assert!(!tree.node(select).unwrap().flags.contains(NodeFlags::OVERLAY));
        }

        #[test]
        fn a_press_outside_an_open_selects_popup_closes_it_and_swallows_the_press() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            let select = leaf(&mut tree, Some(root), 1, select_ui("sel", "a"), (0.0, 0.0, 100.0, 30.0));
            let mut router = EventRouter::new("main");
            router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
            router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary });
            assert!(tree.node(select).unwrap().state.open);

            let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 190.0, y: 190.0, button: PointerButton::Primary });

            assert!(!tree.node(select).unwrap().state.open, "a press well outside the select and its popup should close it");
            assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::OverlayClosed { kind: OverlayKind::SelectPopup, .. })));
        }

        #[test]
        fn picking_a_selects_item_row_fires_its_action_and_closes_the_popup() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            let select = leaf(&mut tree, Some(root), 1, select_ui("sel", "a"), (0.0, 0.0, 100.0, 30.0));
            let row_b = leaf(&mut tree, Some(select), 1, button_ui("b"), (0.0, 32.0, 100.0, 24.0));
            let mut router = EventRouter::new("main");
            router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
            router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary });
            assert!(tree.node(select).unwrap().state.open);

            router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 40.0, button: PointerButton::Primary });
            let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 40.0, button: PointerButton::Primary });

            let expected = action();
            assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::App { action, .. } if *action == expected)), "picking a row should fire its (merged) action");
            assert!(!tree.node(select).unwrap().state.open, "picking an item should close the popup, per toggle_select_popup's dismissal-paths doc comment");
            let _ = row_b;
        }

        fn activatable_stack_ui(action: ActionDescriptor) -> UiNode {
            UiNode::Stack(UiStackNode {
                direction: "vertical".into(),
                gap: None,
                padding: None,
                id: Some("card".into()),
                presence: UiPresence::default(),
                activate: Some(action),
                drop_action: None,
                drop_overlay: None,
                children: Vec::new(),
                menu: None,
            })
        }

        #[test]
        fn clicking_an_activatable_stack_fires_its_activate_action() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            let _card = leaf(&mut tree, Some(root), 1, activatable_stack_ui(action()), (0.0, 0.0, 100.0, 40.0));
            let mut router = EventRouter::new("main");

            router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
            let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary });

            let expected = action();
            assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::App { action, .. } if *action == expected)), "clicking an activatable Stack should fire its `activate` action");
        }

        #[test]
        fn a_bare_stack_without_activate_or_drop_action_stays_a_hit_test_pass_through() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            let _plain = leaf(&mut tree, Some(root), 1, stack_ui(), (0.0, 0.0, 100.0, 40.0));

            assert_eq!(hit_test(&tree, root, 10.0, 10.0), None, "a bare Stack (no activate/drop_action/drag_source/tree-row hover affordance) must remain a hit-test pass-through");
        }

        #[test]
        fn hovering_a_tree_row_fires_its_hover_action_and_leaving_fires_unhover_action() {
            let mut item = UiTreeItemNode::base("row1", "Row One");
            item.hover_action = Some(action());
            let mut leave_action = action();
            leave_action.action = "leave".into();
            item.unhover_action = Some(leave_action.clone());
            let section = UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] };

            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            let tree_id = leaf(&mut tree, Some(root), 1, tree_ui(vec![section]), (0.0, 0.0, 200.0, 200.0));
            insert_tree_row(&mut tree, tree_id, "row1", (0.0, 0.0, 200.0, 24.0));
            let mut router = EventRouter::new("main");

            let entered = router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 10.0, y: 10.0 });
            let expected_enter = action();
            assert!(entered.iter().any(|cmd| matches!(cmd, UiCommand::App { action, .. } if *action == expected_enter)), "moving onto a hover-only tree row should fire its hover_action even though it has no activate/draggable of its own");

            let left = router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 190.0, y: 190.0 });
            assert!(left.iter().any(|cmd| matches!(cmd, UiCommand::App { action, .. } if *action == leave_action)), "moving off the row should fire its unhover_action");
        }

        #[test]
        fn pressing_a_draggable_tree_row_then_moving_past_threshold_promotes_it_to_a_drag_session() {
            let mut item = UiTreeItemNode::base("row1", "Row One");
            item.draggable = Some(true);
            let payload = DragPayload::from([("application/x-semio-tree-section-reorder".to_string(), "{}".to_string())]);
            item.drag_data = Some(payload.clone());
            let section = UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] };

            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            let tree_id = leaf(&mut tree, Some(root), 1, tree_ui(vec![section]), (0.0, 0.0, 200.0, 200.0));
            let row_id = insert_tree_row(&mut tree, tree_id, "row1", (0.0, 0.0, 200.0, 24.0));
            // `paint::sync_tree_row_layout` is what would normally flip this on (mirroring `item.draggable`)
            // — these tests build the retained tree by hand (no `paint_tree` call), so it's set directly.
            tree.node_mut(row_id).unwrap().flags.set(NodeFlags::DRAG_SOURCE, true);
            let mut router = EventRouter::new("main");

            router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
            assert_eq!(router.capture(), Some((row_id, CaptureKind::Press)), "the row must be a real hit-test target once DRAG_SOURCE-flagged");

            router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 30.0, y: 10.0 });
            let drag = router.drag_session().expect("moving past the promote threshold should start a DragSession for a draggable row");
            assert_eq!(drag.source, row_id);
            assert_eq!(drag.payload, payload);
        }
        //#endregion 🔖️W2InteractivityTests

        //#region 🔖️W4SceneCommandTests
        /// 🎬️ A minimal `ComponentScene` leaf — every optional per-`SurfaceKind` payload left `None`,
        /// mirroring `scene_slots::tests::scene`'s own fixture (this module can't reuse that one directly:
        /// it's private to the `scene_slots` submodule).
        fn component_scene_ui(surface_id: &str, kind: SurfaceKind) -> UiNode {
            UiNode::ComponentScene(UiComponentSceneNode {
                surface_id: surface_id.into(),
                controller_id: "ctrl".into(),
                component_kind: kind,
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
                menu: None,
            })
        }

        #[test]
        fn pointer_down_on_a_component_scene_leaf_emits_a_scene_command() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            let scene_id = leaf(&mut tree, Some(root), 1, component_scene_ui("s1", SurfaceKind::Canvas2d), (10.0, 10.0, 100.0, 80.0));
            let mut router = EventRouter::new("main");

            let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 20.0, y: 20.0, button: PointerButton::Secondary });

            let scene_cmd = commands.iter().find_map(|cmd| match cmd {
                UiCommand::Scene { window_id, node, surface_id, kind, rect, event } => Some((window_id, node, surface_id, kind, rect, event)),
                _ => None,
            });
            let (window_id, node, surface_id, kind, rect, event) = scene_cmd.expect("pointer-down over a ComponentScene leaf should emit UiCommand::Scene");
            assert_eq!(window_id, "main");
            assert_eq!(*node, scene_id);
            assert_eq!(surface_id, "s1");
            assert_eq!(*kind, SurfaceKind::Canvas2d);
            assert_eq!(*rect, Rect::new(10.0, 10.0, 100.0, 80.0), "rect should be the leaf's own absolute layout rect");
            assert_eq!(*event, UiEvent::PointerDown { x: 20.0, y: 20.0, button: PointerButton::Secondary }, "the real event should be carried through verbatim, including its button");
        }

        #[test]
        fn pointer_down_outside_any_component_scene_leaf_emits_no_scene_command() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            leaf(&mut tree, Some(root), 1, component_scene_ui("s1", SurfaceKind::Canvas2d), (10.0, 10.0, 100.0, 80.0));
            let mut router = EventRouter::new("main");

            let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 150.0, y: 150.0, button: PointerButton::Primary });

            assert!(!commands.iter().any(|cmd| matches!(cmd, UiCommand::Scene { .. })), "a press outside the scene's own rect should not emit UiCommand::Scene");
        }

        #[test]
        fn pointer_down_on_a_plain_button_emits_no_scene_command() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            leaf(&mut tree, Some(root), 1, button_ui("b1"), (0.0, 0.0, 50.0, 20.0));
            let mut router = EventRouter::new("main");

            let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });

            assert!(!commands.iter().any(|cmd| matches!(cmd, UiCommand::Scene { .. })), "a plain widget leaf should never emit UiCommand::Scene");
        }

        #[test]
        fn pointer_move_over_a_component_scene_leaf_emits_a_scene_command() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            leaf(&mut tree, Some(root), 1, component_scene_ui("s1", SurfaceKind::InkCanvas), (0.0, 0.0, 200.0, 200.0));
            let mut router = EventRouter::new("main");

            let commands = router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 50.0, y: 50.0 });

            assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::Scene { kind: SurfaceKind::InkCanvas, .. })), "moving over a ComponentScene leaf should emit UiCommand::Scene too, not just PointerDown/Up");
        }

        #[test]
        fn scroll_over_a_component_scene_leaf_emits_a_scene_command_and_still_routes_container_scroll() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
            leaf(&mut tree, Some(root), 1, component_scene_ui("s1", SurfaceKind::Table), (0.0, 0.0, 200.0, 200.0));
            let mut router = EventRouter::new("main");

            let commands = router.dispatch(&mut tree, root, &UiEvent::Scroll { x: 50.0, y: 50.0, delta_x: 0.0, delta_y: 12.0 });

            assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::Scene { kind: SurfaceKind::Table, event: UiEvent::Scroll { .. }, .. })), "wheel input over a ComponentScene leaf should emit UiCommand::Scene carrying the Scroll event");
        }

        #[test]
        fn a_component_scene_nested_under_a_container_resolves_its_absolute_rect() {
            let mut tree = UiTree::new();
            let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 300.0, 300.0));
            let container = leaf(&mut tree, Some(root), 1, stack_ui(), (20.0, 30.0, 250.0, 250.0));
            let scene_id = leaf(&mut tree, Some(container), 2, component_scene_ui("s1", SurfaceKind::Paint2d), (5.0, 5.0, 100.0, 100.0));
            let mut router = EventRouter::new("main");

            // Absolute position is (20+5, 30+5) = (25, 35); a point inside that rect must hit-test to the scene.
            let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 40.0, y: 50.0, button: PointerButton::Primary });

            let rect = commands.iter().find_map(|cmd| match cmd {
                UiCommand::Scene { node, rect, .. } if *node == scene_id => Some(*rect),
                _ => None,
            });
            assert_eq!(rect, Some(Rect::new(25.0, 35.0, 100.0, 100.0)), "a nested scene's rect should accumulate every ancestor's own layout offset");
        }
        //#endregion 🔖️W4SceneCommandTests
    }
    // #endregion events
}

#[cfg(feature = "engine")]
pub mod scene_slots {
    // #region scene_slots
    //! 🎬️ Scene-host bridge: after each layout+paint pass the engine collects every `ComponentScene`/
    //! `Image` leaf's resolved absolute rect PLUS a borrowed reference to its own stored `UiNode`
    //! payload into a `SceneSlot`, and hands each one to a caller-provided `SceneHost`, which owns the
    //! actual scene/image rendering (world3d via `infinite_world`, canvas2d, vello surfaces, raster
    //! image decode/upload). `ui_wgpu` never links vello/resvg/tiny-skia/an image codec itself — it only
    //! orchestrates slot geometry and payload borrowing, matching the plan's dependency-graph invariant
    //! that those crates stay in the renderer. Slots borrow directly from the retained `UiTree`'s own
    //! arena-stored `UiNode` — never a second parallel structure — so a host reading a slot's payload is
    //! reading the exact same data `paint::paint_node` would have painted a placeholder for.

    use crate::arena::NodeId;
    use crate::component::ui::{SurfaceKind, UiComponentSceneNode, UiImageNode, UiNode};
    use crate::draw::{DrawList, IconAtlas};
    use crate::geometry::Rect;
    use crate::text::FontAtlas;
    use crate::tree::UiTree;

    /// 🎬️ A `SceneSlot`'s borrowed payload — points directly at the leaf's own `UiNode` variant stored
    /// in the retained `UiTree`'s arena, never a clone.
    #[derive(Debug, PartialEq)]
    pub enum SlotContent<'tree> {
        Scene(&'tree UiComponentSceneNode),
        Image(&'tree UiImageNode),
    }

    /// 🎬️ One `ComponentScene`/`Image` leaf's resolved absolute rect plus its full borrowed payload,
    /// ready to hand to a `SceneHost`.
    #[derive(Debug, PartialEq)]
    pub struct SceneSlot<'tree> {
        pub node: NodeId,
        pub rect: Rect,
        pub content: SlotContent<'tree>,
    }

    impl<'tree> SceneSlot<'tree> {
        /// 🪪️ `(surface_id, SurfaceKind)` when this slot is a `ComponentScene` — `None` for `Image`,
        /// which carries no `SurfaceKind` (it's routed by `SlotContent`'s own variant instead).
        pub fn surface(&self) -> Option<(&'tree str, SurfaceKind)> {
            match self.content {
                SlotContent::Scene(scene) => Some((scene.surface_id.as_str(), scene.component_kind)),
                SlotContent::Image(_) => None,
            }
        }
    }

    /// 🖇️ External scene/image renderer — the only place vello/world3d/raster-decode-specific code may
    /// live; `ui_wgpu` calls into it after layout+paint with resolved slot geometry plus the borrowed
    /// node payload, never the reverse. Paint-only this milestone: routing pointer/keyboard events that
    /// hit a slot to this same host needs a different mechanism (event routing is keyed by `NodeId`
    /// through `events::EventRouter` today, which knows nothing about host-owned sub-surfaces) — that's
    /// later, separate work, not this trait's job to anticipate.
    pub trait SceneHost {
        /// 🖌️ Pushes this slot's own draw calls into `draw` — the retained window's own `DrawList`, in
        /// that window's local `(0,0)`-origin coordinate space, the same space `slot.rect` is expressed
        /// in (the caller composites/offsets the whole `DrawList` afterward, same as every other
        /// retained-paint call). `atlas`/`icons` are the SAME instances the frame's caller passed into
        /// `Ui::frame`, reborrowed fresh per slot so a host that draws text/icons shares the one real,
        /// GPU-uploaded glyph/icon texture instead of needing (or clobbering) its own.
        fn paint_slot(&mut self, slot: &SceneSlot<'_>, draw: &mut DrawList, atlas: &mut FontAtlas, icons: Option<&IconAtlas>);
    }

    /// 📥️ Walks `tree` from `root`, collecting every `ComponentScene`/`Image` leaf's absolute rect
    /// (ancestor offsets accumulated the same way `events::hit_test_node`/`paint::paint_node` do) plus a
    /// borrowed reference to its own stored `UiNode` payload. Recurses into every node's own arena
    /// children unconditionally — not gated by node kind — so leaves nested under ANY container
    /// (`Stack`/`Field`/`Section`/`Group`/`Tree` alike) are found; `tree.children` already reflects
    /// `reconcile`'s real parent-child links for every `UiNode` kind, including `Field`'s single child,
    /// so there is no special-casing needed here for any one container kind. Always includes every
    /// reachable leaf regardless of `DIRTY_PAINT`/`DIRTY_LAYOUT` — scene/image leaves are always-dirty
    /// unless the host opts into its own caching, so `ui_wgpu` doesn't try to cache on the host's behalf
    /// this milestone.
    pub(crate) fn collect_scene_slots<'tree>(tree: &'tree UiTree, root: NodeId) -> Vec<SceneSlot<'tree>> {
        let mut slots = Vec::new();
        collect_scene_slots_node(tree, root, 0.0, 0.0, &mut slots);
        slots
    }

    fn collect_scene_slots_node<'tree>(tree: &'tree UiTree, id: NodeId, origin_x: f32, origin_y: f32, out: &mut Vec<SceneSlot<'tree>>) {
        let Some(node) = tree.node(id) else { return };
        let abs_x = origin_x + node.layout.x;
        let abs_y = origin_y + node.layout.y;
        let rect = Rect::new(abs_x, abs_y, node.layout.width, node.layout.height);
        match &node.spec.0 {
            UiNode::ComponentScene(scene) => out.push(SceneSlot { node: id, rect, content: SlotContent::Scene(scene) }),
            UiNode::Image(image) => out.push(SceneSlot { node: id, rect, content: SlotContent::Image(image) }),
            _ => {}
        }
        for child in tree.children(id) {
            collect_scene_slots_node(tree, child, abs_x, abs_y, out);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::component::ui::{UiComponentSceneNode, UiGroupNode, UiPresence, UiStackNode, UiTextNode};
        use crate::flex::LayoutEngine;
        use crate::theme::Theme;

        fn text(value: &str) -> UiNode {
            UiNode::Text(UiTextNode { value: value.into(), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })
        }

        fn scene(surface_id: &str) -> UiNode {
            UiNode::ComponentScene(UiComponentSceneNode {
                surface_id: surface_id.into(),
                controller_id: "ctrl".into(),
                component_kind: SurfaceKind::World3d,
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
                menu: None,
            })
        }

        fn image(id: &str) -> UiNode {
            UiNode::Image(UiImageNode { id: id.into(), src: "https://example.test/x.png".into(), alt: None, presence: UiPresence::default(), menu: None })
        }

        fn stack(children: Vec<UiNode>) -> UiNode {
            UiNode::Stack(UiStackNode {
                direction: "vertical".into(),
                gap: Some("none".into()),
                padding: Some("standard".into()),
                id: None,
                presence: UiPresence::default(),
                activate: None,
                drop_action: None,
                drop_overlay: None,
                children,
                menu: None,
            })
        }

        fn group(children: Vec<UiNode>) -> UiNode {
            UiNode::Group(UiGroupNode { id: "group".into(), label: "Group".into(), default_open: None, presence: UiPresence::default(), children, menu: None })
        }

        fn layout(node: &UiNode) -> UiTree {
            let mut tree = UiTree::new();
            tree.apply_tree(node);
            let root = tree.root.unwrap();
            let mut engine = LayoutEngine::new();
            let mut atlas = FontAtlas::builtin();
            let theme = Theme::default();
            engine.compute(&mut tree, root, &mut atlas, &theme, 400.0, 400.0);
            tree
        }

        #[test]
        fn collects_a_scene_leaf_with_its_absolute_rect_accounting_for_ancestor_offsets() {
            let tree = layout(&stack(vec![text("above"), scene("surface.one")]));
            let root = tree.root.unwrap();

            let slots = collect_scene_slots(&tree, root);
            assert_eq!(slots.len(), 1);
            let slot = &slots[0];
            assert_eq!(slot.surface(), Some(("surface.one", SurfaceKind::World3d)));
            // The scene leaf is the stack's second child (below the text sibling plus the stack's own
            // top padding) -- a nonzero absolute y proves ancestor offsets were accumulated, not just
            // the leaf's own parent-relative `LayoutBucket` coordinates.
            assert!(slot.rect.y > 0.0, "expected the scene leaf offset below its text sibling, got y={}", slot.rect.y);
            assert!(slot.rect.w > 0.0 && slot.rect.h > 0.0);
        }

        #[test]
        fn finds_no_slots_when_the_tree_has_no_scene_nodes() {
            let tree = layout(&stack(vec![text("only text")]));
            let root = tree.root.unwrap();
            assert!(collect_scene_slots(&tree, root).is_empty());
        }

        #[test]
        fn collects_multiple_scene_leaves_in_document_order() {
            let tree = layout(&stack(vec![scene("surface.a"), scene("surface.b")]));
            let root = tree.root.unwrap();

            let slots = collect_scene_slots(&tree, root);
            let ids: Vec<&str> = slots.iter().filter_map(|slot| slot.surface().map(|(id, _)| id)).collect();
            assert_eq!(ids, vec!["surface.a", "surface.b"]);
        }

        #[test]
        fn collects_an_image_leaf_alongside_a_scene_leaf() {
            let tree = layout(&stack(vec![image("img.one"), scene("surface.one")]));
            let root = tree.root.unwrap();

            let slots = collect_scene_slots(&tree, root);
            assert_eq!(slots.len(), 2);
            assert!(matches!(slots[0].content, SlotContent::Image(node) if node.id == "img.one"));
            assert!(matches!(slots[1].content, SlotContent::Scene(node) if node.surface_id == "surface.one"));
        }

        #[test]
        fn collects_a_scene_leaf_nested_under_a_group_ancestor() {
            // 🌳️ Regression for the shadow-walk gap this bridge replaces: the legacy immediate-mode walk
            // it superseded only recursed into Stack/Section/Field, so a ComponentScene nested under a
            // Group never resolved to real content. `collect_scene_slots_node` recurses into every
            // node's `tree.children` unconditionally, so a Group ancestor is no different from a Stack.
            let tree = layout(&group(vec![text("label"), scene("surface.nested")]));
            let root = tree.root.unwrap();

            let slots = collect_scene_slots(&tree, root);
            assert_eq!(slots.len(), 1);
            assert_eq!(slots[0].surface(), Some(("surface.nested", SurfaceKind::World3d)));
        }
    }
    // #endregion scene_slots
}

#[cfg(feature = "engine")]
pub mod shell {
    // #region shell
    //! 🪟️ Retained representation of dock/split/tab/window-cap chrome, built from the declarative
    //! `WindowLayout` vocabulary (not `UiNode` — window-shell chrome isn't expressed as app-declarative
    //! `UiNode`s). `Shell` owns its own `UiTree` so a later `engine` facade milestone can run the same
    //! layout/paint/events passes over shell chrome that it runs over app content trees.
    //! `set_window_layout` does a full teardown+rebuild each call rather than keyed diffing (window
    //! layouts change far less often per-frame than widget content, so a full rebuild is a reasonable v1
    //! — incremental shell-tree diffing is a documented gap for a later milestone). Drag-to-reorder/
    //! drop-zone computation is stubbed this milestone (see `dispatch`'s doc comment): only hit-testing
    //! plus click-to-activate-tab is fully implemented.

    use crate::arena::NodeId;
    use crate::component::layout::{ActionDescriptor, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode};
    use crate::component::ui::{UiButtonNode, UiNode, UiPresence, UiStackNode};
    use crate::events::{hit_test, UiEvent};
    use crate::tree::{Node, NodeFlags, NodeKey, UiTree, WidgetSpec};
    use crate::IconName;
    use crate::Label;

    const SHELL_AXIS: u32 = 200;
    const SHELL_STACK: u32 = 201;

    /// 📤️ What `Shell::dispatch` surfaces to the host: chrome-level interactions that aren't app
    /// `ActionDescriptor`s (those still flow through `events::UiCommand::App`).
    #[derive(Clone, Debug, PartialEq)]
    pub enum ShellEvent {
        /// 🫳️ A window-cap/tab-header press started a potential drag. `Shell::dispatch` never emits this
        /// yet — see its doc comment for what's stubbed vs. implemented this milestone.
        PanelDragStarted { pane_id: String },
        /// 📥️ A dragged pane was released over a drop zone. `Shell::dispatch` never emits this yet (no
        /// drop-target geometry is computed this milestone); kept in the enum so the host-facing API
        /// shape is settled ahead of the drag-and-drop implementation landing in a later milestone.
        PanelDropped { pane_id: String, target: String },
        /// 🖱️ A tab/window-cap was clicked (pointer down and up over the same window leaf).
        TabActivated { window_id: String },
    }

    /// 🪟️ Retained dock/split/tab/window-cap/navbar chrome, driven by declarative `WindowLayout`.
    pub struct Shell {
        tree: UiTree,
        layout: Option<WindowLayout>,
        navbar: Vec<String>,
        pressed: Option<NodeId>,
        window_kind_icons: std::collections::HashMap<String, IconName>,
    }

    impl Shell {
        /// 🌱️ An empty shell: no layout applied yet, no navbar items.
        pub fn new() -> Self {
            Self { tree: UiTree::new(), layout: None, navbar: Vec::new(), pressed: None, window_kind_icons: std::collections::HashMap::new() }
        }

        /// 🪟️ Maps window kind ids to Lucide icon ids for tab-cap painting in `set_window_layout`.
        pub fn set_window_kind_icons(&mut self, icons: std::collections::HashMap<String, IconName>) {
            self.window_kind_icons = icons;
        }

        /// 🔁️ Rebuilds the shell's retained tree from `layout` (full teardown+rebuild, see module doc).
        /// Axis nodes become row/column `Stack` containers; stack (tab-group) nodes become `Stack`
        /// containers marked `CLIPS_CHILDREN` (a tab group clips its content to its own bounds); each
        /// window leaf becomes a `Button`-shaped hit target keyed by its `instance_id` (falling back to
        /// `window_kind_id`) — a plain `Stack` can never itself be a hit target
        /// (`events::hit_test`'s bare-`Stack`-is-pass-through-only rule), so window caps deliberately use
        /// a non-`Stack` variant instead.
        pub fn set_window_layout(&mut self, layout: WindowLayout) {
            let mut tree = UiTree::new();
            let root_id = tree.insert_child(None, Node::new(NodeKey::Explicit("shell.root".into()), WidgetSpec(root_stack())));
            tree.mark_dirty(root_id, NodeFlags::DIRTY_LAYOUT);
            build_root(&mut tree, root_id, &layout.root, &self.window_kind_icons);
            self.tree = tree;
            self.layout = Some(layout);
            self.pressed = None;
        }

        /// 🧭️ Minimal stub: stores whatever navbar-relevant labels the host provides. A full navbar data
        /// model and pixel-perfect chrome painting are deferred to a later milestone — getting the tree
        /// integration point right matters more than the visual right now.
        pub fn set_navbar(&mut self, items: Vec<String>) {
            self.navbar = items;
        }

        /// 📖️ The declarative layout last applied via `set_window_layout`, if any.
        pub fn window_layout(&self) -> Option<&WindowLayout> {
            self.layout.as_ref()
        }

        /// 🌳️ Read access to the shell's retained tree, for a later `engine` facade to layout/paint/route.
        pub fn tree(&self) -> &UiTree {
            &self.tree
        }

        /// 🌳️ Mutable access to the shell's retained tree.
        pub fn tree_mut(&mut self) -> &mut UiTree {
            &mut self.tree
        }

        /// 🧭️ The stub navbar item labels currently set via `set_navbar`.
        pub fn navbar(&self) -> &[String] {
            &self.navbar
        }

        /// 🕹️ Hit-tests `event` against the shell's own retained tree and surfaces `ShellEvent`s. Fully
        /// implemented: `PointerDown` over a window-cap captures it; a matching `PointerUp` over the
        /// *same* window-cap emits `TabActivated`. Stubbed: no `PanelDragStarted`/`PanelDropped` are
        /// emitted yet — drag-to-reorder needs drop-zone geometry this milestone doesn't compute
        /// (documented gap, deferred to Phase 4's shell carve-over).
        pub fn dispatch(&mut self, event: &UiEvent) -> Vec<ShellEvent> {
            let mut out = Vec::new();
            let Some(root) = self.tree.root else { return out };
            match event {
                UiEvent::PointerDown { x, y, .. } => {
                    self.pressed = hit_test(&self.tree, root, *x, *y);
                }
                UiEvent::PointerUp { x, y, .. } => {
                    let released = hit_test(&self.tree, root, *x, *y);
                    if let (Some(pressed_id), Some(released_id)) = (self.pressed.take(), released) {
                        if pressed_id == released_id {
                            if let Some(NodeKey::Explicit(window_id)) = self.tree.node(released_id).map(|node| node.key.clone()) {
                                out.push(ShellEvent::TabActivated { window_id });
                            }
                        }
                    }
                }
                _ => {}
            }
            out
        }
    }

    impl Default for Shell {
        fn default() -> Self {
            Self::new()
        }
    }

    fn root_stack() -> UiNode {
        UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, id: Some("shell.root".into()), presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children: Vec::new(), menu: None })
    }

    fn build_root(tree: &mut UiTree, parent: NodeId, root: &WindowLayoutRoot, window_kind_icons: &std::collections::HashMap<String, IconName>) {
        match root {
            WindowLayoutRoot::Axis(axis) => build_axis(tree, parent, axis, 0, window_kind_icons),
            WindowLayoutRoot::Stack(stack) => build_stack(tree, parent, stack, 0, window_kind_icons),
        }
    }

    fn build_axis(tree: &mut UiTree, parent: NodeId, axis: &WindowLayoutAxisNode, ordinal: u32, window_kind_icons: &std::collections::HashMap<String, IconName>) {
        let spec =
            UiNode::Stack(UiStackNode { direction: axis.kind.clone(), gap: Some("none".into()), padding: None, id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children: Vec::new(), menu: None });
        let id = tree.insert_child(Some(parent), Node::new(NodeKey::Positional(SHELL_AXIS, ordinal), WidgetSpec(spec)));
        tree.mark_dirty(id, NodeFlags::DIRTY_LAYOUT);
        for (index, child) in axis.children.iter().enumerate() {
            match child {
                WindowLayoutChild::Axis(nested) => build_axis(tree, id, nested, index as u32, window_kind_icons),
                WindowLayoutChild::Stack(nested) => build_stack(tree, id, nested, index as u32, window_kind_icons),
            }
        }
    }

    /// 🗂️ A tab group: a `Stack` container marked `CLIPS_CHILDREN` (its content clips to its own bounds)
    /// whose children are the window-cap `Button` leaves built by `build_window`.
    fn build_stack(tree: &mut UiTree, parent: NodeId, stack: &WindowLayoutStackNode, ordinal: u32, window_kind_icons: &std::collections::HashMap<String, IconName>) {
        let spec = UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children: Vec::new(), menu: None });
        let id = tree.insert_child(Some(parent), Node::new(NodeKey::Positional(SHELL_STACK, ordinal), WidgetSpec(spec)));
        if let Some(node) = tree.node_mut(id) {
            node.flags.set(NodeFlags::CLIPS_CHILDREN, true);
        }
        tree.mark_dirty(id, NodeFlags::DIRTY_LAYOUT);
        for (index, window) in stack.children.iter().enumerate() {
            build_window(tree, id, window, index as u32, window_kind_icons);
        }
    }

    /// 🪟️ One window-cap/tab-header hit target, keyed by `instance_id` (falling back to
    /// `window_kind_id`). Modeled as a `Button` rather than a `Stack` specifically so `events::hit_test`
    /// treats it as a matchable leaf, not a pass-through container (see `set_window_layout`'s doc
    /// comment).
    fn build_window(tree: &mut UiTree, parent: NodeId, window: &WindowLayoutWindowNode, ordinal: u32, window_kind_icons: &std::collections::HashMap<String, IconName>) {
        let _ = ordinal;
        let window_id = window.instance_id.clone().unwrap_or_else(|| window.window_kind_id.clone());
        let label = window.title.clone().unwrap_or_else(|| window.window_kind_id.clone());
        let icon_id = window_kind_icons.get(&window.window_kind_id).copied().unwrap_or(IconName::AppWindow);
        let spec = UiNode::Button(UiButtonNode {
            id: Some(window_id.clone()),
            icon_id,
            label: Label::data(label),
            action: ActionDescriptor { controller_id: "shell.window".into(), action: "activate".into(), args: None },
            style: None,
            presence: UiPresence::default(),
            menu: None,
        });
        let id = tree.insert_child(Some(parent), Node::new(NodeKey::Explicit(window_id), WidgetSpec(spec)));
        tree.mark_dirty(id, NodeFlags::DIRTY_LAYOUT);
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::flex::LayoutEngine;
        use crate::text::FontAtlas;
        use crate::theme::Theme;

        fn single_window_layout(window_kind_id: &str) -> WindowLayout {
            crate::even_window_layout(&[window_kind_id.to_string()])
        }

        fn run_layout(shell: &mut Shell) {
            let root = shell.tree().root.expect("set_window_layout must produce a root");
            let mut engine = LayoutEngine::new();
            let mut atlas = FontAtlas::builtin();
            let theme = Theme::default();
            engine.compute(shell.tree_mut(), root, &mut atlas, &theme, 400.0, 400.0);
        }

        fn count_nodes(tree: &UiTree, id: NodeId) -> usize {
            1 + tree.children(id).map(|child| count_nodes(tree, child)).sum::<usize>()
        }

        #[test]
        fn set_window_layout_with_one_window_produces_the_expected_retained_tree_shape() {
            let mut shell = Shell::new();
            shell.set_window_layout(single_window_layout("app.viewport"));

            let root = shell.tree().root.expect("expected a root node");
            // shell.root -> tab-group Stack -> one Button window leaf = 3 nodes.
            assert_eq!(count_nodes(shell.tree(), root), 3);
            let tab_group = shell.tree().children(root).next().expect("expected a tab-group child");
            assert!(shell.tree().node(tab_group).unwrap().flags.contains(NodeFlags::CLIPS_CHILDREN));
            let window_leaf = shell.tree().children(tab_group).next().expect("expected a window leaf");
            assert!(matches!(shell.tree().node(window_leaf).unwrap().spec.0, UiNode::Button(_)));
        }

        #[test]
        fn set_window_layout_called_twice_with_the_same_layout_is_idempotent_and_does_not_panic() {
            let mut shell = Shell::new();
            shell.set_window_layout(single_window_layout("app.viewport"));
            let first_count = count_nodes(shell.tree(), shell.tree().root.unwrap());

            shell.set_window_layout(single_window_layout("app.viewport"));
            let second_count = count_nodes(shell.tree(), shell.tree().root.unwrap());

            assert_eq!(first_count, second_count);
            assert_eq!(shell.window_layout(), Some(&single_window_layout("app.viewport")));
        }

        #[test]
        fn pointer_down_and_up_on_the_same_window_cap_activates_its_tab() {
            let mut shell = Shell::new();
            shell.set_window_layout(single_window_layout("app.viewport"));
            run_layout(&mut shell);

            let down = shell.dispatch(&UiEvent::PointerDown { x: 10.0, y: 10.0, button: crate::events::PointerButton::Primary });
            assert!(down.is_empty(), "press alone must not activate a tab");

            let up = shell.dispatch(&UiEvent::PointerUp { x: 10.0, y: 10.0, button: crate::events::PointerButton::Primary });
            assert_eq!(up, vec![ShellEvent::TabActivated { window_id: "app.viewport".into() }]);
        }

        #[test]
        fn pointer_down_then_up_outside_the_pressed_window_cap_does_not_activate_a_tab() {
            let mut shell = Shell::new();
            shell.set_window_layout(single_window_layout("app.viewport"));
            run_layout(&mut shell);

            shell.dispatch(&UiEvent::PointerDown { x: 10.0, y: 10.0, button: crate::events::PointerButton::Primary });
            let up = shell.dispatch(&UiEvent::PointerUp { x: -50.0, y: -50.0, button: crate::events::PointerButton::Primary });
            assert!(up.is_empty(), "releasing outside every hit target must not activate a tab");
        }
    }
    // #endregion shell
}

#[cfg(feature = "engine")]
pub mod engine {
    // #region engine
    //! 🧵️ The retained-mode `Ui` façade: the missing keystone tying `arena`/`tree`/`reconcile`/`flex`/
    //! `paint`/`events`/`scene_slots`/`shell` into one usable pipeline — each of those regions was built
    //! and individually tested to its own milestone but nothing ever assembled them together, and nothing
    //! in `framework/renderer/wgpu` calls into any of it (see `.🦑️repo/🎫️tickets/26/07/11/RETAINED-MODE-UI-CRATE`'s
    //! plan for the historical intent). This module is purely additive: the immediate-mode `widgets`
    //! path stays the only pipeline actually driving pixels until a later workstream proves this façade
    //! out (via the golden `tests` module below) and cuts over.

    use std::collections::HashMap;

    use crate::component::layout::WindowLayout;
    use crate::component::ui::UiNode;
    use crate::draw::{DrawList, IconAtlas};
    use crate::events::{EventRouter, UiCommand, UiEvent};
    use crate::flex::LayoutEngine;
    use crate::paint::paint_tree;
    use crate::scene_slots::{collect_scene_slots, SceneHost};
    use crate::shell::{Shell, ShellEvent};
    use crate::text::FontAtlas;
    use crate::theme::Theme;
    use crate::tree::{NodeFlags, UiTree};
    use crate::IconName;

    //#region 🔖️UiWindow
    /// 🪟️ One window's retained pipeline state: its `UiTree` (`reconcile`'s diff target), the taffy
    /// `LayoutEngine` that lays it out (`flex`), the `EventRouter` owning its capture/focus/hover state
    /// (`events`), and the `DrawList` `paint::paint_tree` last painted into. Mirrors `tree`'s own doc
    /// comment ("the engine facade... holds `HashMap<window_id, UiTree>`") by keying the *whole*
    /// per-window pipeline the same way, not just the tree.
    struct UiWindow {
        tree: UiTree,
        layout: LayoutEngine,
        router: EventRouter,
        draw: DrawList,
        viewport: (f32, f32),
    }

    impl UiWindow {
        fn new(window_id: &str) -> Self {
            Self { tree: UiTree::new(), layout: LayoutEngine::new(), router: EventRouter::new(window_id), draw: DrawList::default(), viewport: (0.0, 0.0) }
        }

        /// 🚨️ Whether this window's root (and thus, transitively, anything below it per
        /// `UiTree::mark_dirty`'s bubbling) still needs a layout or paint pass.
        fn is_dirty(&self) -> bool {
            self.tree.root.and_then(|root| self.tree.node(root)).is_some_and(|node| node.flags.contains(NodeFlags::DIRTY_LAYOUT) || node.flags.contains(NodeFlags::DIRTY_PAINT) || node.flags.contains(NodeFlags::SUBTREE_DIRTY))
        }
    }
    //#endregion 🔖️UiWindow

    //#region 🔖️Ui
    /// 🧵️ Assembles the individually-milestoned retained modules into the one façade a host drives per
    /// tick: `apply_tree` runs `reconcile`, `frame` runs `flex` (dirty-gated) then `paint` then hands
    /// `scene_slots` to an optional `SceneHost`, `dispatch_event` runs `events::EventRouter`, and
    /// `needs_frame` reads the same dirty flags `frame` itself gates on. One `UiWindow` per window id
    /// (app-content trees); window-chrome (dock/split/tab) is the separate `Shell` this façade also owns,
    /// since `shell`'s own doc comment models it as independent of any single window's content tree.
    /// Never submits to the GPU itself — `frame` returns a `&DrawList` for the caller to hand to the
    /// existing `gpu::GpuContext::render_frame`, exactly like the immediate-mode `widgets` path's callers
    /// already do; wiring that hand-off into a real host event loop is later, renderer-thinning work.
    pub struct Ui {
        windows: HashMap<String, UiWindow>,
        shell: Shell,
        theme: Theme,
        pending_commands: Vec<UiCommand>,
    }

    impl Ui {
        pub fn new() -> Self {
            Self { windows: HashMap::new(), shell: Shell::new(), theme: Theme::default(), pending_commands: Vec::new() }
        }

        pub fn set_theme(&mut self, theme: Theme) {
            self.theme = theme;
        }

        fn window_mut(&mut self, window_id: &str) -> &mut UiWindow {
            self.windows.entry(window_id.to_string()).or_insert_with(|| UiWindow::new(window_id))
        }

        /// 📐️ Stores the viewport a later `frame` call lays out against for `window_id`, creating that
        /// window's retained state on first use.
        pub fn set_viewport(&mut self, window_id: &str, width: f32, height: f32) {
            self.window_mut(window_id).viewport = (width, height);
        }

        /// 🔁️ Runs `UiTree::apply_tree` (`reconcile`) to diff `ui_node` into `window_id`'s retained tree,
        /// creating that window's tree/layout-engine/event-router on first use.
        pub fn apply_tree(&mut self, window_id: &str, ui_node: &UiNode) {
            self.window_mut(window_id).tree.apply_tree(ui_node);
        }

        pub fn set_window_kind_icons(&mut self, icons: std::collections::HashMap<String, IconName>) {
            self.shell.set_window_kind_icons(icons);
        }

        /// 🪟️ Rebuilds the shared `Shell`'s retained dock/split/tab chrome from a declarative
        /// `WindowLayout` (independent of any window's `apply_tree`d content — see `shell`'s doc comment).
        pub fn set_window_layout(&mut self, layout: WindowLayout) {
            self.shell.set_window_layout(layout);
        }

        /// 🧭️ Forwards to `Shell::set_navbar` (stub — see that method's doc comment).
        pub fn set_navbar(&mut self, items: Vec<String>) {
            self.shell.set_navbar(items);
        }

        pub fn shell(&self) -> &Shell {
            &self.shell
        }

        /// 🚦️ True when any window's retained tree still carries `DIRTY_LAYOUT`/`DIRTY_PAINT`/
        /// `SUBTREE_DIRTY` on its root. No animation-clock scaffolding exists anywhere in this crate yet
        /// (nothing under `arena`/`tree`/`reconcile`/`flex`/`paint`/`events`/`scene_slots`/`shell`
        /// schedules a future wake), so this is purely dirty-flag-driven; wiring a real animation deadline
        /// is separate follow-up work, not this façade's job to invent.
        pub fn needs_frame(&self) -> bool {
            self.windows.values().any(UiWindow::is_dirty)
        }

        /// 🖼️ The dirty-gated per-tick pipeline for `window_id`: `flex::LayoutEngine::compute` (itself a
        /// no-operation unless the root carries `DIRTY_LAYOUT`/`SUBTREE_DIRTY`) followed — only if that or the
        /// root's own `DIRTY_PAINT` fired — by `paint::paint_tree`, then handing every
        /// `scene_slots::collect_scene_slots` leaf to `scene_host`, when the caller passed one this tick.
        /// Returns `None` if `window_id` has no tree yet (`apply_tree` never called). A dirty window
        /// always repaints its whole tree — `paint::paint_tree`'s own doc comment: `DrawList` only
        /// supports a full clear-and-rebuild, no incremental dirty-subtree replacement yet.
        ///
        /// 🖋️ `atlas`/`icons` are the CALLER's own `FontAtlas`/`IconAtlas` — `Ui` never owns either (see
        /// this region's top-of-file doc comment): the host must pass the SAME instances it already
        /// `GpuContext::upload_font_atlas`/`upload_icon_atlas`s every frame, exactly like `flex::LayoutEngine::
        /// compute`/`paint::paint_tree` already receive them as parameters rather than fields. This lets
        /// retained-mode content share glyph/icon UVs with the rest of the host's chrome instead of
        /// clobbering (or never populating) a second, independent GPU texture.
        ///
        /// 🎬️ `scene_host` is a PER-FRAME parameter, not a stored field (there used to be a stored
        /// `Option<Box<dyn SceneHost>>` — removed): a caller-owned host typically needs to borrow the
        /// same per-frame state this call site already has in scope (a `GpuContext`, per-surface state
        /// maps, …), which a `Box<dyn SceneHost>` stored on `Ui` itself could never hold, exactly like
        /// `atlas`/`icons` above are parameters rather than fields for the same reason. `paint_tree`
        /// already knows (via `scene_host.is_some()`) whether to paint its own placeholder chrome for
        /// `ComponentScene`/`Image` leaves this tick or leave that rect for the host to fill in below —
        /// see `paint`'s own doc comment on that gate.
        pub fn frame(&mut self, window_id: &str, viewport_width: f32, viewport_height: f32, atlas: &mut FontAtlas, icons: Option<&IconAtlas>, mut scene_host: Option<&mut dyn SceneHost>) -> Option<&DrawList> {
            let window = self.windows.get_mut(window_id)?;
            let root = window.tree.root?;
            window.viewport = (viewport_width, viewport_height);
            let dirty = window.tree.node(root).is_some_and(|node| node.flags.contains(NodeFlags::DIRTY_LAYOUT) || node.flags.contains(NodeFlags::DIRTY_PAINT) || node.flags.contains(NodeFlags::SUBTREE_DIRTY));
            if !dirty {
                return Some(&window.draw);
            }
            window.layout.compute(&mut window.tree, root, atlas, &self.theme, viewport_width, viewport_height);
            window.draw.clear();
            paint_tree(&mut window.tree, root, &self.theme, atlas, icons, scene_host.is_some(), &mut window.draw);
            if let Some(host) = scene_host.as_deref_mut() {
                for slot in collect_scene_slots(&window.tree, root) {
                    host.paint_slot(&slot, &mut window.draw, atlas, icons);
                }
            }
            Some(&window.draw)
        }

        /// 📤️ Direct access to `window_id`'s last-painted `DrawList` without re-running the pipeline.
        pub fn draw_list(&self, window_id: &str) -> Option<&DrawList> {
            self.windows.get(window_id).map(|window| &window.draw)
        }

        /// 🕹️ Routes `event` through `window_id`'s `events::EventRouter` (hit-test, capture, focus, hover
        /// updates), returning the `UiCommand`s it produced and also queuing them for a later
        /// `drain_commands` call — callers may use either.
        #[allow(clippy::needless_pass_by_value, reason = "changing to &UiEvent is a breaking public API change across ~30 downstream plugins, out of T1 scope")]
        pub fn dispatch_event(&mut self, window_id: &str, event: UiEvent) -> Vec<UiCommand> {
            let Some(window) = self.windows.get_mut(window_id) else { return Vec::new() };
            let Some(root) = window.tree.root else { return Vec::new() };
            let commands = window.router.dispatch(&mut window.tree, root, &event);
            self.pending_commands.extend(commands.iter().cloned());
            commands
        }

        /// 🪟️ Routes `event` through the shared `Shell`'s own hit-testing, surfacing chrome-level
        /// `ShellEvent`s (tab activation today; drag/drop is `Shell::dispatch`'s own documented gap).
        pub fn dispatch_shell_event(&mut self, event: &UiEvent) -> Vec<ShellEvent> {
            self.shell.dispatch(event)
        }

        /// 📥️ Drains every `UiCommand` queued by `dispatch_event` calls since the last drain.
        pub fn drain_commands(&mut self) -> Vec<UiCommand> {
            std::mem::take(&mut self.pending_commands)
        }
    }

    impl Default for Ui {
        fn default() -> Self {
            Self::new()
        }
    }
    //#endregion 🔖️Ui

    //#region 🔬️Introspection
    /// 🔬️ Read-only accessors for the wgpu↔React parity structural-dump harness (see
    /// `.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY` and `framework/renderer/wgpu`'s own
    /// `🔬️Introspection` region, which is the actual JSON-building caller): exposes just enough of
    /// `Ui`'s private `windows`/`theme` state for a caller to walk a window's retained `UiTree` (via
    /// `UiTree::node`/`UiTree::children`, both already public) and know which theme it last painted
    /// with. Purely additive and read-only — no new engine behavior, nothing here is called from
    /// `apply_tree`/`frame`/`dispatch_event`'s own pipeline.
    impl Ui {
        /// 🪟️ Every window id this façade currently tracks retained state for (`HashMap` iteration
        /// order — not insertion order; a caller needing a deterministic pick must sort/filter itself).
        pub fn window_ids(&self) -> impl Iterator<Item = &str> {
            self.windows.keys().map(String::as_str)
        }

        /// 📐️ `window_id`'s last `set_viewport`/`frame` viewport, if that window has any retained state.
        pub fn viewport(&self, window_id: &str) -> Option<(f32, f32)> {
            self.windows.get(window_id).map(|window| window.viewport)
        }

        /// 🌲️ Read-only access to `window_id`'s retained tree (root + `Node` arena) for a caller to walk.
        pub fn tree(&self, window_id: &str) -> Option<&UiTree> {
            self.windows.get(window_id).map(|window| &window.tree)
        }

        /// 🎨️ The theme this façade last painted every window with (`Theme` is `Copy`).
        pub fn theme(&self) -> Theme {
            self.theme
        }

        /// 🎯️ Whether `window_id`'s retained content currently has a focused node — `false` if that
        /// window has no retained state at all. Lets a host (`w2-input-wiring`,
        /// `.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w2-input-wiring.md`) decide whether real
        /// keyboard/IME events belong to this window's content (route via `dispatch_event`) or should
        /// fall back to chrome-level shortcuts. Forwards to `EventRouter::is_focused`, itself added this
        /// same pass — both purely additive reads, no change to `dispatch_event`'s own focus logic.
        pub fn window_has_focus(&self, window_id: &str) -> bool {
            self.windows.get(window_id).is_some_and(|window| window.router.is_focused())
        }
    }
    //#endregion 🔬️Introspection

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::component::layout::ActionDescriptor;
        use crate::component::ui::{
            ui_node_to_control, SurfaceKind, UiButtonNode, UiComponentSceneNode, UiControlNode, UiExternalSlotNode, UiFieldNode, UiGroupNode, UiIconSelectNode, UiImageNode, UiInputNode, UiKeyValueEntry, UiKeyValueNode, UiNumberStepperNode,
            UiPresence, UiRingNode, UiSectionNode, UiSelectItem, UiSelectNode, UiSeparatorNode, UiSliderNode, UiStackNode, UiState, UiTextNode, UiToggleNode, UiTreeItemAction, UiTreeItemNode, UiTreeNode, UiTreeSectionNode,
        };
        use crate::events::PointerButton;
        use crate::geometry::Rect;
        use crate::input::InputState;
        use crate::scene_slots::SceneSlot;
        use crate::widgets::{
            draw_text_on, draw_text_overlay_on, measure_widget, render_scroll_region, render_widget, wrap_text, ControlNode, InputMeta, KeyValueEntry, RingMeta, SelectItem, SliderMeta, StepperMeta, TreeItem, TreeItemAction, TreeSection,
            WidgetContext, WidgetInteractionMaps, WidgetNode,
        };
        use std::collections::HashMap as StdHashMap;

        //#region 🔖️FacadeTests
        fn stack_ui(children: Vec<UiNode>) -> UiNode {
            UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: Some("root".into()), presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children, menu: None })
        }

        fn action() -> ActionDescriptor {
            ActionDescriptor { controller_id: "ctrl".into(), action: "go".into(), args: None }
        }

        fn button_ui(id: &str, label: &str) -> UiNode {
            UiNode::Button(UiButtonNode { id: Some(id.into()), icon_id: IconName::CircleDot, label: label.into(), action: action(), style: None, presence: UiPresence::default(), menu: None })
        }

        #[test]
        fn apply_tree_then_frame_produces_a_non_empty_draw_list() {
            let mut ui = Ui::new();
            let mut atlas = FontAtlas::builtin();
            ui.apply_tree("main", &stack_ui(vec![UiNode::Text(UiTextNode { value: "hi".into(), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })]));

            assert!(ui.needs_frame(), "a freshly applied tree must report needing a frame");
            let draw = ui.frame("main", 400.0, 400.0, &mut atlas, None, None).expect("frame must produce a draw list once a tree was applied");
            let total: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
            assert!(total > 0, "expected the text node to emit at least one glyph instance");
        }

        #[test]
        fn frame_before_any_apply_tree_returns_none() {
            let mut ui = Ui::new();
            let mut atlas = FontAtlas::builtin();
            assert!(ui.frame("nonexistent", 400.0, 400.0, &mut atlas, None, None).is_none());
        }

        #[test]
        fn needs_frame_is_false_once_a_stable_tree_has_been_framed() {
            let mut ui = Ui::new();
            let mut atlas = FontAtlas::builtin();
            let ui_node = stack_ui(vec![UiNode::Text(UiTextNode { value: "hi".into(), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })]);
            ui.apply_tree("main", &ui_node);
            ui.frame("main", 400.0, 400.0, &mut atlas, None, None);
            assert!(!ui.needs_frame(), "nothing changed since the last frame, so no frame should be needed");

            ui.apply_tree("main", &ui_node);
            assert!(!ui.needs_frame(), "re-applying an identical tree must set zero dirty flags (reconcile's own golden rule)");
        }

        #[test]
        fn dispatch_event_emits_a_button_click_command_and_it_is_also_drainable() {
            let mut ui = Ui::new();
            let mut atlas = FontAtlas::builtin();
            ui.apply_tree("main", &stack_ui(vec![button_ui("go", "Go")]));
            ui.frame("main", 400.0, 400.0, &mut atlas, None, None);

            ui.dispatch_event("main", UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
            let commands = ui.dispatch_event("main", UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary });

            assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::App { action: fired_action, .. } if *fired_action == action())));
            let drained = ui.drain_commands();
            assert!(!drained.is_empty(), "commands dispatched should also be queryable via drain_commands");
            assert!(ui.drain_commands().is_empty(), "a second drain with nothing new dispatched must be empty");
        }

        #[test]
        fn set_window_layout_wires_into_the_facades_shell() {
            let mut ui = Ui::new();
            ui.set_window_layout(crate::even_window_layout(&["app.viewport".to_string()]));
            assert!(ui.shell().window_layout().is_some());
        }
        //#endregion 🔖️FacadeTests

        //#region 🔖️SceneHostTests
        fn component_scene_ui(surface_id: &str) -> UiNode {
            UiNode::ComponentScene(UiComponentSceneNode {
                surface_id: surface_id.into(),
                controller_id: "ctrl".into(),
                component_kind: SurfaceKind::World3d,
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
                menu: None,
            })
        }

        /// 🎬️ A bare-bones `SceneHost` recording every call it receives, so tests can assert `Ui::frame`
        /// actually reaches the host (once per slot, with the right payload) instead of just trusting the
        /// wiring compiles. Paints a single filled rect per slot so a hosted frame's `DrawList` is
        /// distinguishable from an unpainted one.
        struct RecordingSceneHost {
            paint_calls: usize,
            last_surface_id: Option<String>,
        }

        impl SceneHost for RecordingSceneHost {
            fn paint_slot(&mut self, slot: &SceneSlot<'_>, draw: &mut DrawList, _atlas: &mut FontAtlas, _icons: Option<&IconAtlas>) {
                self.paint_calls += 1;
                self.last_surface_id = slot.surface().map(|(surface_id, _)| surface_id.to_string());
                draw.push_rounded([slot.rect.x, slot.rect.y, slot.rect.w, slot.rect.h], Theme::default().accent, 0.0);
            }
        }

        #[test]
        fn frame_with_no_scene_host_falls_back_to_the_placeholder_chrome() {
            let mut ui = Ui::new();
            let mut atlas = FontAtlas::builtin();
            ui.apply_tree("w", &stack_ui(vec![component_scene_ui("surface.no-host")]));
            let draw = ui.frame("w", 400.0, 400.0, &mut atlas, None, None).expect("frame must produce a draw list");
            let instances: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
            assert!(instances > 0, "with no scene host registered, paint_component_scene's placeholder chrome should still paint");
        }

        #[test]
        fn frame_with_a_scene_host_routes_the_component_scene_leaf_through_it() {
            let mut ui = Ui::new();
            let mut atlas = FontAtlas::builtin();
            ui.apply_tree("w", &stack_ui(vec![component_scene_ui("surface.host-test")]));

            let mut host = RecordingSceneHost { paint_calls: 0, last_surface_id: None };
            let draw = ui.frame("w", 400.0, 400.0, &mut atlas, None, Some(&mut host)).expect("frame must produce a draw list even with a scene host registered");
            let instances: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();

            assert_eq!(host.paint_calls, 1, "the host should be invoked exactly once for the single ComponentScene leaf");
            assert_eq!(host.last_surface_id.as_deref(), Some("surface.host-test"));
            assert!(instances > 0, "the host's own draw call should still land in the frame's DrawList");
        }

        #[test]
        fn frame_with_a_scene_host_still_paints_ancestor_chrome_around_the_hosted_slot() {
            // 🌳️ Nests the ComponentScene under a Group (not just a bare Stack) — regression for the
            // shadow-walk gap this bridge replaces: `collect_scene_slots` must still find it, and the
            // Group's own header/frame chrome (unrelated to the scene leaf) must still paint normally.
            let mut ui = Ui::new();
            let mut atlas = FontAtlas::builtin();
            let group_node = UiNode::Group(UiGroupNode {
                id: "group".into(),
                label: "Group".into(),
                default_open: None,
                presence: UiPresence::default(),
                children: vec![UiNode::Text(UiTextNode { value: "label".into(), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None }), component_scene_ui("surface.nested")],
                menu: None,
            });
            ui.apply_tree("w", &group_node);

            let mut host = RecordingSceneHost { paint_calls: 0, last_surface_id: None };
            ui.frame("w", 400.0, 400.0, &mut atlas, None, Some(&mut host)).expect("frame must produce a draw list");

            assert_eq!(host.paint_calls, 1);
            assert_eq!(host.last_surface_id.as_deref(), Some("surface.nested"));
        }
        //#endregion 🔖️SceneHostTests

        //#region 🔖️GoldenHarness
        /// 🏆️ Acceptance gate for this workstream: for a curated fixture of every `UiNode` variant, runs
        /// the retained façade (`apply_tree` + `frame`) and the immediate-mode path (`render_widget` over
        /// a hand-converted `WidgetNode`) and asserts they emit structurally equivalent `DrawList`s
        /// (same instance/vector/raster counts — not bit-identical geometry, per this ticket's brief).
        /// `to_widget_node`/`control_to_widget`/`tree_*_to_widget` below mirror
        /// `framework/renderer/wgpu/rs/lib.rs`'s private `ui_node_to_widget` conversion; they're
        /// duplicated here (test-only) rather than shared because that crate depends on `ui_wgpu`, never
        /// the reverse — keeping the two in sync is this harness's job.
        fn to_widget_node(node: &UiNode) -> WidgetNode<ActionDescriptor> {
            match node {
                UiNode::Stack(stack) => WidgetNode::Stack { direction: stack.direction.clone(), gap: stack.gap.clone(), padding: stack.padding.clone(), children: stack.children.iter().map(to_widget_node).collect() },
                UiNode::Text(text) => WidgetNode::Text { value: text.value.clone(), emphasize: text.emphasize.unwrap_or(false) },
                UiNode::Separator(_) => WidgetNode::Separator,
                UiNode::Button(button) => WidgetNode::Button { id: button.id.clone(), icon_id: Some(button.icon_id.clone()), label: button.label.clone(), event: Some(button.action.clone()) },
                UiNode::Input(input) => {
                    WidgetNode::Input { id: input.id.clone(), input_kind: input.input_kind.clone(), value: input.value.clone(), placeholder: input.placeholder.clone(), commit: input.commit.clone(), on_change: Some(input.on_change.clone()) }
                }
                UiNode::Select(select) => WidgetNode::Select {
                    id: select.id.clone(),
                    value: select.value.clone(),
                    items: select.items.iter().map(|item| SelectItem { value: item.value.clone(), label: item.label.clone() }).collect(),
                    placeholder: select.placeholder.clone(),
                    on_change: Some(select.on_change.clone()),
                },
                UiNode::Toggle(toggle) => WidgetNode::Toggle { id: toggle.id.clone(), icon_id: toggle.icon_id.clone(), pressed: toggle.presence.selected, text: toggle.text.clone(), on_change: Some(toggle.on_change.clone()) },
                UiNode::KeyValue(kv) => WidgetNode::KeyValue { entries: kv.entries.iter().map(|entry| KeyValueEntry { label: entry.label.clone(), value: entry.value.clone() }).collect() },
                UiNode::Slider(slider) => WidgetNode::Slider { id: slider.id.clone(), value: slider.value, min: slider.min, max: slider.max, step: slider.step, ready: None, disabled: false, on_change: Some(slider.on_change.clone()) },
                UiNode::NumberStepper(stepper) => {
                    WidgetNode::NumberStepper { id: stepper.id.clone(), value: stepper.value, step: stepper.step, uniform: stepper.uniform, on_absolute: Some(stepper.on_absolute.clone()), on_delta: Some(stepper.on_delta.clone()) }
                }
                UiNode::Ring(ring) => WidgetNode::Ring { id: ring.id.clone(), t: ring.t, disabled: ring.presence.state == UiState::Disabled, on_change: Some(ring.on_change.clone()) },
                UiNode::IconSelect(select) => WidgetNode::IconSelect { id: select.id.clone(), value: select.value.clone(), uniform: select.uniform, classifier_kind: select.classifier_kind.clone(), on_change: Some(select.on_change.clone()) },
                UiNode::Field(field) => match ui_node_to_control(&field.child) {
                    Some(control) => WidgetNode::Field { id: field.id.clone(), label: field.label.clone(), child: control_to_widget(&control) },
                    None => WidgetNode::Section { id: field.id.clone(), label: Some(field.label.clone()), default_open: true, children: vec![to_widget_node(&field.child)] },
                },
                UiNode::Section(section) => WidgetNode::Section { id: section.id.clone(), label: section.label.clone(), default_open: section.default_open.unwrap_or(true), children: section.children.iter().map(to_widget_node).collect() },
                UiNode::Group(group) => WidgetNode::Section { id: group.id.clone(), label: Some(group.label.clone()), default_open: group.default_open.unwrap_or(true), children: group.children.iter().map(to_widget_node).collect() },
                UiNode::Tree(tree) => WidgetNode::Tree {
                    // 🧭️ Per-item `selected`/`highlighted` (see `tree_item_to_widget`) already carry the
                    // full signal from `item.presence` — the tree-level id lists are gone, not re-derived.
                    sections: tree.sections.iter().map(tree_section_to_widget).collect(),
                    selected_ids: Vec::new(),
                    highlighted_ids: Vec::new(),
                    selection_change: tree.selection_change.clone(),
                },
                // KNOWN GAP: `WidgetNode<E>` (the immediate-mode `widgets` region's tree type) has no
                // Image/ComponentScene/ExternalSlot variant at all — the renderer's own
                // `ui_node_to_widget` collapses all three to an empty placeholder `Text` node, which
                // isn't a like-for-like rendering of the same node. There is no immediate-mode output to
                // compare the retained `paint::paint_image`/`paint_component_scene`/`paint_external_slot`
                // against; see the golden tests below for these three, which verify the retained side
                // alone produces sane output and skip the two-pipeline equivalence assertion.
                UiNode::Image(_) | UiNode::ComponentScene(_) | UiNode::ExternalSlot(_) => WidgetNode::Text { value: String::new(), emphasize: false },
            }
        }

        fn control_to_widget(control: &UiControlNode) -> ControlNode<ActionDescriptor> {
            match control {
                UiControlNode::Button(n) => ControlNode::Button { id: n.id.clone(), icon_id: Some(n.icon_id.clone()), label: n.label.clone(), event: Some(n.action.clone()) },
                UiControlNode::Input(n) => ControlNode::Input { id: n.id.clone(), input_kind: n.input_kind.clone(), value: n.value.clone(), placeholder: n.placeholder.clone(), commit: n.commit.clone(), on_change: Some(n.on_change.clone()) },
                UiControlNode::Select(n) => ControlNode::Select {
                    id: n.id.clone(),
                    value: n.value.clone(),
                    items: n.items.iter().map(|item| SelectItem { value: item.value.clone(), label: item.label.clone() }).collect(),
                    placeholder: n.placeholder.clone(),
                    on_change: Some(n.on_change.clone()),
                },
                UiControlNode::Toggle(n) => ControlNode::Toggle { id: n.id.clone(), icon_id: n.icon_id.clone(), pressed: n.presence.selected, text: n.text.clone(), on_change: Some(n.on_change.clone()) },
                UiControlNode::KeyValue(n) => ControlNode::KeyValue { entries: n.entries.iter().map(|entry| KeyValueEntry { label: entry.label.clone(), value: entry.value.clone() }).collect() },
                UiControlNode::Slider(n) => ControlNode::Slider { id: n.id.clone(), value: n.value, min: n.min, max: n.max, step: n.step, ready: None, disabled: false, on_change: Some(n.on_change.clone()) },
                UiControlNode::NumberStepper(n) => ControlNode::NumberStepper { id: n.id.clone(), value: n.value, step: n.step, uniform: n.uniform, on_absolute: Some(n.on_absolute.clone()), on_delta: Some(n.on_delta.clone()) },
                UiControlNode::Ring(n) => ControlNode::Ring { id: n.id.clone(), t: n.t, disabled: n.presence.state == UiState::Disabled, on_change: Some(n.on_change.clone()) },
                UiControlNode::IconSelect(n) => ControlNode::IconSelect { id: n.id.clone(), value: n.value.clone(), uniform: n.uniform, classifier_kind: n.classifier_kind.clone(), on_change: Some(n.on_change.clone()) },
            }
        }

        /// 🎛️ Same per-variant field mapping as `control_to_widget`, but into a `WidgetNode` instead of a
        /// `ControlNode` — needed for `TreeItem::control: Option<Box<WidgetNode<E>>>`, which (unlike
        /// `Field`'s `child: ControlNode<E>`) embeds a full widget, not a bare control payload.
        fn control_to_widget_node(control: &UiControlNode) -> WidgetNode<ActionDescriptor> {
            match control {
                UiControlNode::Button(n) => WidgetNode::Button { id: n.id.clone(), icon_id: Some(n.icon_id.clone()), label: n.label.clone(), event: Some(n.action.clone()) },
                UiControlNode::Input(n) => WidgetNode::Input { id: n.id.clone(), input_kind: n.input_kind.clone(), value: n.value.clone(), placeholder: n.placeholder.clone(), commit: n.commit.clone(), on_change: Some(n.on_change.clone()) },
                UiControlNode::Select(n) => WidgetNode::Select {
                    id: n.id.clone(),
                    value: n.value.clone(),
                    items: n.items.iter().map(|item| SelectItem { value: item.value.clone(), label: item.label.clone() }).collect(),
                    placeholder: n.placeholder.clone(),
                    on_change: Some(n.on_change.clone()),
                },
                UiControlNode::Toggle(n) => WidgetNode::Toggle { id: n.id.clone(), icon_id: n.icon_id.clone(), pressed: n.presence.selected, text: n.text.clone(), on_change: Some(n.on_change.clone()) },
                UiControlNode::KeyValue(n) => WidgetNode::KeyValue { entries: n.entries.iter().map(|entry| KeyValueEntry { label: entry.label.clone(), value: entry.value.clone() }).collect() },
                UiControlNode::Slider(n) => WidgetNode::Slider { id: n.id.clone(), value: n.value, min: n.min, max: n.max, step: n.step, ready: None, disabled: false, on_change: Some(n.on_change.clone()) },
                UiControlNode::NumberStepper(n) => WidgetNode::NumberStepper { id: n.id.clone(), value: n.value, step: n.step, uniform: n.uniform, on_absolute: Some(n.on_absolute.clone()), on_delta: Some(n.on_delta.clone()) },
                UiControlNode::Ring(n) => WidgetNode::Ring { id: n.id.clone(), t: n.t, disabled: n.presence.state == UiState::Disabled, on_change: Some(n.on_change.clone()) },
                UiControlNode::IconSelect(n) => WidgetNode::IconSelect { id: n.id.clone(), value: n.value.clone(), uniform: n.uniform, classifier_kind: n.classifier_kind.clone(), on_change: Some(n.on_change.clone()) },
            }
        }

        fn tree_action_to_widget(action: &UiTreeItemAction) -> TreeItemAction<ActionDescriptor> {
            TreeItemAction { icon_id: action.icon_id.clone(), label: action.label.clone(), event: action.action.clone(), placement: action.placement() }
        }

        fn tree_item_to_widget(item: &UiTreeItemNode) -> TreeItem<ActionDescriptor> {
            TreeItem {
                id: item.id.clone(),
                label: item.label.clone(),
                description: item.description.clone(),
                icon_id: item.icon_id.clone(),
                selected: item.presence.selected,
                highlighted: item.presence.state == UiState::Previewed,
                default_open: item.default_open.unwrap_or(false),
                dimmed: item.dimmed.unwrap_or(false),
                event: item.action.clone(),
                hover_event: item.hover_action.clone(),
                unhover_event: item.unhover_action.clone(),
                actions: item.actions.as_ref().map(|actions| actions.iter().map(tree_action_to_widget).collect()).unwrap_or_default(),
                draggable: item.draggable.unwrap_or(false),
                drag_data: item.drag_data.clone().unwrap_or_default(),
                control: item.control.as_ref().map(|control| Box::new(control_to_widget_node(control))),
                children: item.items.as_ref().map(|items| items.iter().map(tree_item_to_widget).collect()).unwrap_or_default(),
            }
        }

        fn tree_section_to_widget(section: &UiTreeSectionNode) -> TreeSection<ActionDescriptor> {
            TreeSection { id: section.id.clone(), label: section.label.clone(), default_open: section.default_open.unwrap_or(true), items: section.items.iter().map(tree_item_to_widget).collect() }
        }

        /// 📊️ Total (ui_instances incl. overlay, vector_vertices incl. overlay, raster_instances) across
        /// every layer of a `DrawList` — the "structurally equivalent" signal this harness compares,
        /// deliberately coarser than exact geometry per this ticket's tolerance allowance.
        fn stats(draw: &DrawList) -> (usize, usize, usize) {
            let instances = draw.layers.iter().map(|layer| layer.ui_instances.len() + layer.overlay_ui_instances.len()).sum();
            let vectors = draw.layers.iter().map(|layer| layer.vector_vertices.len() + layer.overlay_vector_vertices.len()).sum();
            let raster = draw.layers.iter().map(|layer| layer.raster_instances.len()).sum();
            (instances, vectors, raster)
        }

        fn retained_stats(node: &UiNode) -> (usize, usize, usize) {
            let mut ui = Ui::new();
            let mut atlas = FontAtlas::builtin();
            ui.apply_tree("golden", node);
            let draw = ui.frame("golden", 400.0, 400.0, &mut atlas, None, None).expect("apply_tree then frame must produce a draw list");
            stats(draw)
        }

        fn immediate_stats(node: &UiNode, bounds: Rect) -> (usize, usize, usize) {
            let widget = to_widget_node(node);
            let mut draw = DrawList::default();
            let mut atlas = FontAtlas::builtin();
            let theme = Theme::default();
            let mut input = InputState::<ActionDescriptor>::default();
            let mut scroll_offsets: StdHashMap<String, f32> = StdHashMap::new();
            let mut collapsed_sections: StdHashMap<String, bool> = StdHashMap::new();
            let mut open_selects: StdHashMap<String, bool> = StdHashMap::new();
            let mut ctx = WidgetContext {
                draw: &mut draw,
                overlay: None,
                atlas: &mut atlas,
                icons: None,
                input: &mut input,
                theme: &theme,
                scroll_offsets: &mut scroll_offsets,
                collapsed_sections: &mut collapsed_sections,
                open_selects: &mut open_selects,
                interaction_maps: None,
                pick_clip: None,
            };
            render_widget(&widget, bounds, &mut ctx);
            stats(&draw)
        }

        const VIEWPORT: Rect = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };

        /// 🧱️ Wraps a leaf `UiNode` as the sole child of a gap-less/padding-less vertical `Stack`: on the
        /// retained side, `flex::LayoutEngine` always forces the *root* to the full viewport size
        /// (`compute`'s `root_style.size` override) and gives a `Stack`'s only child `flex_grow: 1.0`, so
        /// the child's resolved `LayoutBucket` is exactly the full viewport. On the immediate side,
        /// `layout::layout_vertical`/`layout_horizontal`'s `extra_per_child` gives a lone child the same
        /// full bounds. Wrapping every leaf fixture this way guarantees both pipelines paint it at
        /// identical bounds, which is what makes an exact instance/vector-count comparison meaningful
        /// instead of an artifact of divergent layout math.
        fn leaf(child: UiNode) -> UiNode {
            UiNode::Stack(UiStackNode {
                direction: "vertical".into(),
                gap: Some("none".into()),
                padding: Some("none".into()),
                id: None,
                presence: UiPresence::default(),
                activate: None,
                drop_action: None,
                drop_overlay: None,
                children: vec![child],
                menu: None,
            })
        }

        fn assert_equivalent(kind: &str, node: &UiNode) {
            let retained = retained_stats(node);
            let immediate = immediate_stats(node, VIEWPORT);
            assert_eq!(retained, immediate, "{kind}: retained (instances, vectors, raster) {retained:?} != immediate {immediate:?}");
        }

        // `action()` is shared with 🔖️FacadeTests above — both sub-regions live in the same `mod tests`.

        #[test]
        fn golden_stack() {
            let node = UiNode::Stack(UiStackNode {
                direction: "vertical".into(),
                gap: Some("none".into()),
                padding: Some("none".into()),
                id: None,
                presence: UiPresence::default(),
                activate: None,
                drop_action: None,
                drop_overlay: None,
                children: vec![
                    UiNode::Text(UiTextNode { value: "hello".into(), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None }),
                    UiNode::Separator(UiSeparatorNode { presence: UiPresence::default(), menu: None }),
                ],
                menu: None,
            });
            assert_equivalent("Stack", &node);
        }

        #[test]
        fn golden_text() {
            assert_equivalent("Text", &leaf(UiNode::Text(UiTextNode { value: "hello world".into(), emphasize: Some(true), data_attributes: None, presence: UiPresence::default(), menu: None })));
        }

        #[test]
        fn golden_button() {
            assert_equivalent("Button", &leaf(UiNode::Button(UiButtonNode { id: Some("btn".into()), icon_id: IconName::CircleDot, label: "Go".into(), action: action(), style: None, presence: UiPresence::default(), menu: None })));
        }

        #[test]
        fn golden_separator() {
            assert_equivalent("Separator", &leaf(UiNode::Separator(UiSeparatorNode { presence: UiPresence::default(), menu: None })));
        }

        #[test]
        fn golden_input() {
            assert_equivalent(
                "Input",
                &leaf(UiNode::Input(UiInputNode {
                    id: "in".into(),
                    input_kind: "text".into(),
                    value: "abc".into(),
                    placeholder: None,
                    commit: None,
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    on_change: action(),
                    presence: UiPresence::default(),
                    menu: None,
                })),
            );
        }

        #[test]
        fn golden_select() {
            assert_equivalent(
                "Select",
                &leaf(UiNode::Select(UiSelectNode {
                    id: "sel".into(),
                    value: "a".into(),
                    items: vec![UiSelectItem { value: "a".into(), label: "Alpha".into() }, UiSelectItem { value: "b".into(), label: "Beta".into() }],
                    placeholder: None,
                    on_change: action(),
                    presence: UiPresence::default(),
                    menu: None,
                })),
            );
        }

        #[test]
        fn golden_toggle() {
            // 🚫️ `presence.selected` is intentionally NOT exercised here: the shared `presence_overlay`
            // now draws an outset accent ring for ANY selected element (see
            // `selected_presence_draws_an_outset_ring_on_any_element`, below) — a deliberate new
            // capability `widgets::render_toggle` (the immediate-mode reference this harness compares
            // against) never had, so a selected fixture would fail this equivalence check for the wrong
            // reason. This test stays scoped to the base (unselected) toggle's fill/label parity.
            assert_equivalent("Toggle", &leaf(UiNode::Toggle(UiToggleNode { id: "tog".into(), icon_id: IconName::CircleDot, text: Some("On".into()), on_change: action(), presence: UiPresence::default(), menu: None })));
        }

        /// ✨️ `presence.selected` draws its outset accent ring universally — proven here on `Toggle`, a
        /// non-`Stack` variant, since `selected` used to be a `UiStackNode`-only field. Instance count
        /// grows vs. the unselected fixture (the extra `push_chrome_border` edges), confirming the ring
        /// is now a shared channel every element gets for free from `presence_overlay`.
        #[test]
        fn selected_presence_draws_an_outset_ring_on_any_element() {
            let unselected = UiNode::Toggle(UiToggleNode { id: "tog".into(), icon_id: IconName::CircleDot, text: Some("On".into()), on_change: action(), presence: UiPresence::default(), menu: None });
            let selected = UiNode::Toggle(UiToggleNode { id: "tog".into(), icon_id: IconName::CircleDot, text: Some("On".into()), on_change: action(), presence: UiPresence::selected(true), menu: None });
            let (unselected_instances, _, _) = retained_stats(&leaf(unselected));
            let (selected_instances, _, _) = retained_stats(&leaf(selected));
            assert!(selected_instances > unselected_instances, "a selected element should paint more instances than an unselected one (the outset accent ring)");
        }

        #[test]
        fn golden_key_value() {
            assert_equivalent("KeyValue", &leaf(UiNode::KeyValue(UiKeyValueNode { entries: vec![UiKeyValueEntry { label: "Name".into(), value: "Semio".into() }], presence: UiPresence::default(), menu: None })));
        }

        #[test]
        fn golden_slider() {
            assert_equivalent("Slider", &leaf(UiNode::Slider(UiSliderNode { id: "sl".into(), value: 0.5, min: 0.0, max: 1.0, step: 0.01, unit: None, on_change: action(), presence: UiPresence::default(), menu: None })));
        }

        /// KNOWN GAP: `widgets::render_number_stepper` renders its center value segment via a full
        /// `render_input` call (which itself calls `push_control_border` — a background fill plus 4
        /// border-edge quads, 5 instances), giving the center value its own nested input-style border box.
        /// `paint::paint_number_stepper` instead just `draw_text_on`s the formatted value directly with no
        /// surrounding border. Confirmed by running this fixture: retained emits 14 instances (one
        /// `push_control_border` for the whole control + 2 divider lines + 3 text runs), immediate emits
        /// 19 (the same 14 plus the center value's own nested 5-instance border box) — a real, reproducible
        /// paint-logic difference, not a fixture/harness artifact. This is real follow-up work for `paint`
        /// (either add the nested border to `paint_number_stepper`, or confirm the immediate path's nested
        /// border is unintentional and should be dropped there instead — a product decision outside this
        /// façade's scope), not something to paper over here.
        #[test]
        fn golden_number_stepper_known_gap() {
            let (instances, _, _) = retained_stats(&leaf(UiNode::NumberStepper(UiNumberStepperNode { id: "ns".into(), value: 2.0, step: 1.0, uniform: false, on_absolute: action(), on_delta: action(), presence: UiPresence::default(), menu: None })));
            assert!(instances > 0, "NumberStepper should paint its minus/value/plus segments");
        }

        /// 🔒️ Added by `w1c-paint-parity` (see `.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w1c-paint-parity.md`):
        /// `paint::paint_number_stepper` now ports `widgets::render_number_stepper`'s nested
        /// center-value border box (the exact gap `golden_number_stepper_known_gap`'s doc comment
        /// above documents), closing the 14-vs-19-instance divergence for the `uniform: true` case.
        /// Left `golden_number_stepper_known_gap` itself untouched (still valid, still a `uniform: false`
        /// fixture) and added this as a new, additive `assert_equivalent` case for `uniform: true`
        /// instead, per this workstream's "don't modify existing tests" rule.
        #[test]
        fn golden_number_stepper() {
            assert_equivalent("NumberStepper", &leaf(UiNode::NumberStepper(UiNumberStepperNode { id: "ns".into(), value: 2.0, step: 1.0, uniform: true, on_absolute: action(), on_delta: action(), presence: UiPresence::default(), menu: None })));
        }

        #[test]
        fn golden_ring() {
            assert_equivalent("Ring", &leaf(UiNode::Ring(UiRingNode { id: "ring".into(), orb_id: "orb".into(), t: 0.25, on_change: action(), presence: UiPresence::default(), menu: None })));
        }

        #[test]
        fn golden_icon_select() {
            assert_equivalent(
                "IconSelect",
                &leaf(UiNode::IconSelect(UiIconSelectNode { id: "ic".into(), value: IconName::Sparkles.to_string(), uniform: false, classifier_kind: "kind".into(), on_change: action(), presence: UiPresence::default(), menu: None })),
            );
        }

        #[test]
        fn golden_tree() {
            let item = |id: &str, label: &str| UiTreeItemNode {
                id: id.into(),
                label: label.into(),
                description: None,
                icon_id: None,
                presence: UiPresence::default(),
                default_open: None,
                action: None,
                hover_action: None,
                unhover_action: None,
                actions: None,
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                dimmed: None,
                menu: None,
            };
            let node = UiNode::Tree(UiTreeNode {
                sections: vec![UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item("i1", "Item One"), item("i2", "Item Two")] }],
                presence: UiPresence::default(),
                selected_ids: None,
                highlighted_ids: None,
                selection_change: None,
                drop_action: None,
                menu: None,
            });
            assert_equivalent("Tree", &node);
        }

        /// KNOWN GAP: `reconcile` only expands `Field`/`Section` into a real retained child for their
        /// `child`/`children` payload (per `reconcile`'s own module doc comment — M2 recurses into
        /// `Stack`/`Section`/`Field` only), but `flex::LayoutEngine` only grants `flex_grow: 1.0` to a
        /// `Stack`'s children (see `style_with_grow`'s `flex_grow_child` param, gated on
        /// `matches!(node.spec.0, UiNode::Stack(_))`). A `Field`/`Section`'s synthetic retained child is
        /// therefore laid out at its own intrinsic content size instead of filling the label-adjusted
        /// remainder the way `widgets::render_widget`'s hand-rolled `Field`/`Section` branches
        /// (`Rect::new(bounds.x, bounds.y + label_h + gap, bounds.w, bounds.h - label_h - gap)` for
        /// `Field`, per-child accumulated `y` for `Section`) explicitly carve out. The two pipelines'
        /// geometry — and therefore instance counts for size-dependent content like wrapped `Text` — can
        /// genuinely diverge here. This is real follow-up work for `flex`, not something this façade can
        /// paper over; these two tests verify the retained side alone produces sane, non-empty output.
        #[test]
        fn golden_field_known_gap() {
            let node = UiNode::Field(UiFieldNode {
                id: "f".into(),
                label: "Label".into(),
                description: None,
                required: None,
                error: None,
                child: Box::new(UiNode::Input(UiInputNode {
                    id: "in".into(),
                    input_kind: "text".into(),
                    value: "abc".into(),
                    placeholder: None,
                    commit: None,
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    on_change: action(),
                    presence: UiPresence::default(),
                    menu: None,
                })),
                presence: UiPresence::default(),
                menu: None,
            });
            let (instances, _, _) = retained_stats(&node);
            assert!(instances > 0, "Field should paint its label plus its child control");
        }

        #[test]
        fn golden_section_known_gap() {
            let node = UiNode::Section(UiSectionNode {
                id: "sec".into(),
                label: Some("Section".into()),
                default_open: Some(true),
                presence: UiPresence::default(),
                children: vec![UiNode::Text(UiTextNode { value: "child".into(), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })],
                menu: None,
            });
            let (instances, _, _) = retained_stats(&node);
            assert!(instances > 0, "Section should paint its header label plus its children");
        }

        /// KNOWN GAP: see `to_widget_node`'s own `UiNode::Image | UiNode::ComponentScene | UiNode::ExternalSlot`
        /// match arm doc comment — `WidgetNode<E>` has no variant for any of these three, so there is no
        /// immediate-mode equivalent to compare against at all. `paint::paint_image`/
        /// `paint_component_scene`/`paint_external_slot` are themselves documented placeholders (no host
        /// texture-upload queue / scene-host / plugin-body wiring exists in `ui_wgpu` yet either) — these
        /// tests only verify the retained side produces the placeholder chrome its own doc comments
        /// promise, not equivalence with anything immediate-mode.
        #[test]
        fn golden_image_known_gap() {
            let node = UiNode::Image(UiImageNode { id: "img".into(), src: String::new(), alt: Some("alt text".into()), presence: UiPresence::default(), menu: None });
            let (instances, _, _) = retained_stats(&node);
            assert!(instances > 0, "an empty-src Image should still paint its alt text");
        }

        #[test]
        fn golden_component_scene_known_gap() {
            let node = UiNode::ComponentScene(UiComponentSceneNode {
                surface_id: "surf".into(),
                controller_id: "ctrl".into(),
                component_kind: SurfaceKind::World3d,
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
                menu: None,
            });
            let (instances, _, _) = retained_stats(&node);
            assert!(instances > 0, "ComponentScene should paint its placeholder border chrome");
        }

        #[test]
        fn golden_external_slot_known_gap() {
            let node = UiNode::ExternalSlot(UiExternalSlotNode { plugin_id: "plug".into(), app_id: "app".into(), body_key: "body".into(), params_json: "{}".into(), presence: UiPresence::default(), menu: None });
            let (instances, _, _) = retained_stats(&node);
            assert!(instances > 0, "ExternalSlot should paint its placeholder chrome plus its body_key label");
        }
        //#endregion 🔖️GoldenHarness

        //#region 🔬️IntrospectionTests
        #[test]
        fn window_ids_viewport_tree_and_theme_expose_private_window_state() {
            let mut ui = Ui::new();
            assert_eq!(ui.window_ids().count(), 0);
            assert_eq!(ui.viewport("win"), None);
            assert!(ui.tree("win").is_none());

            let node = UiNode::Text(UiTextNode { value: "hi".into(), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None });
            ui.apply_tree("win", &node);
            ui.set_viewport("win", 800.0, 600.0);

            let ids: Vec<&str> = ui.window_ids().collect();
            assert_eq!(ids, vec!["win"]);
            assert_eq!(ui.viewport("win"), Some((800.0, 600.0)));
            let tree = ui.tree("win").expect("tree exists after apply_tree");
            assert!(tree.root.is_some());
            assert_eq!(ui.theme().text.a, Theme::default().text.a);
        }
        //#endregion 🔬️IntrospectionTests

        //#region 🧩️WidgetsInternalsTests
        /// 🧰️ Owns every piece `widgets::WidgetContext<'_, ActionDescriptor>` borrows, so each test can
        /// build one without fighting lifetimes; `ctx()` re-borrows fresh each call (a `WidgetContext`
        /// isn't `Clone`/reusable once passed to `render_widget`, which can mutate through it).
        struct WidgetHarness {
            draw: DrawList,
            atlas: FontAtlas,
            theme: Theme,
            input: InputState<ActionDescriptor>,
            scroll_offsets: StdHashMap<String, f32>,
            collapsed_sections: StdHashMap<String, bool>,
            open_selects: StdHashMap<String, bool>,
            maps: WidgetInteractionMaps<ActionDescriptor>,
        }

        impl WidgetHarness {
            fn new() -> Self {
                Self {
                    draw: DrawList::default(),
                    atlas: FontAtlas::builtin(),
                    theme: Theme::default(),
                    input: InputState::default(),
                    scroll_offsets: StdHashMap::new(),
                    collapsed_sections: StdHashMap::new(),
                    open_selects: StdHashMap::new(),
                    maps: WidgetInteractionMaps::default(),
                }
            }

            fn ctx(&mut self) -> WidgetContext<'_, ActionDescriptor> {
                WidgetContext {
                    draw: &mut self.draw,
                    overlay: None,
                    atlas: &mut self.atlas,
                    icons: None,
                    input: &mut self.input,
                    theme: &self.theme,
                    scroll_offsets: &mut self.scroll_offsets,
                    collapsed_sections: &mut self.collapsed_sections,
                    open_selects: &mut self.open_selects,
                    interaction_maps: Some(&mut self.maps),
                    pick_clip: None,
                }
            }
        }

        #[test]
        fn wrap_text_wraps_long_text_across_multiple_lines() {
            let mut atlas = FontAtlas::builtin();
            let long = "word ".repeat(40);
            let lines = wrap_text(&mut atlas, &long, 100.0, 16.0);
            assert!(lines.len() > 1, "text far wider than max_width must wrap into multiple lines");
            for line in &lines {
                assert!(!line.is_empty());
            }
        }

        #[test]
        fn wrap_text_of_empty_string_yields_one_empty_line() {
            let mut atlas = FontAtlas::builtin();
            let lines = wrap_text(&mut atlas, "", 200.0, 16.0);
            assert_eq!(lines, vec![String::new()], "an empty input must still produce a single (empty) line, never zero lines");
        }

        #[test]
        fn measure_widget_stack_vertical_sums_child_heights_and_maxes_width() {
            let mut atlas = FontAtlas::builtin();
            let theme = Theme::default();
            let node = WidgetNode::<ActionDescriptor>::Stack { direction: "vertical".into(), gap: Some("none".into()), padding: Some("none".into()), children: vec![WidgetNode::Separator, WidgetNode::Separator] };
            let (_, h) = measure_widget(&mut atlas, &theme, &node);
            let (_, single_h) = measure_widget(&mut atlas, &theme, &WidgetNode::<ActionDescriptor>::Separator);
            assert!((h - single_h * 2.0).abs() < 0.001, "two stacked separators with no gap/padding must measure to exactly twice one separator's height, got {h} vs {single_h}");
        }

        #[test]
        fn measure_widget_stack_horizontal_sums_child_widths() {
            let mut atlas = FontAtlas::builtin();
            let theme = Theme::default();
            let button = || WidgetNode::<ActionDescriptor>::Button { id: Some("b".into()), icon_id: None, label: "Go".into(), event: None };
            let node = WidgetNode::Stack { direction: "horizontal".into(), gap: Some("none".into()), padding: Some("none".into()), children: vec![button(), button()] };
            let (w, _) = measure_widget(&mut atlas, &theme, &node);
            assert!((w - theme.control_height * 2.0).abs() < 0.001, "two gap-less horizontal buttons must measure to exactly twice one control's width");
        }

        #[test]
        fn measure_widget_separator_uses_theme_control_height_floor() {
            let mut atlas = FontAtlas::builtin();
            let theme = Theme::default();
            let (w, h) = measure_widget(&mut atlas, &theme, &WidgetNode::<ActionDescriptor>::Separator);
            assert_eq!(w, theme.control_height.max(1.0));
            assert_eq!(h, 1.0 + theme.gap_standard);
        }

        #[test]
        fn measure_widget_key_value_grows_with_entry_count() {
            let mut atlas = FontAtlas::builtin();
            let theme = Theme::default();
            let one = WidgetNode::<ActionDescriptor>::KeyValue { entries: vec![KeyValueEntry { label: "A".into(), value: "1".into() }] };
            let two = WidgetNode::<ActionDescriptor>::KeyValue { entries: vec![KeyValueEntry { label: "A".into(), value: "1".into() }, KeyValueEntry { label: "B".into(), value: "2".into() }] };
            let (_, h1) = measure_widget(&mut atlas, &theme, &one);
            let (_, h2) = measure_widget(&mut atlas, &theme, &two);
            assert!((h2 - h1 * 2.0).abs() < 0.001, "KeyValue height must scale linearly with entry count");
        }

        #[test]
        fn measure_widget_ring_is_fixed_size() {
            let mut atlas = FontAtlas::builtin();
            let theme = Theme::default();
            let (w, h) = measure_widget(&mut atlas, &theme, &WidgetNode::<ActionDescriptor>::Ring { id: "r".into(), t: 0.5, disabled: false, on_change: None });
            assert_eq!((w, h), (80.0, 80.0));
        }

        #[test]
        fn measure_widget_field_combines_label_and_child_height() {
            let mut atlas = FontAtlas::builtin();
            let theme = Theme::default();
            let node = WidgetNode::<ActionDescriptor>::Field { id: "f".into(), label: "Label".into(), child: ControlNode::Slider { id: "s".into(), value: 0.5, min: 0.0, max: 1.0, step: 0.1, ready: None, disabled: false, on_change: None } };
            let (_, h) = measure_widget(&mut atlas, &theme, &node);
            assert!(h > theme.control_height, "a Field's total height must be its label plus its child control, so it must exceed the control's own height alone");
        }

        #[test]
        fn measure_widget_section_sums_header_and_children_plus_gap() {
            let mut atlas = FontAtlas::builtin();
            let theme = Theme::default();
            let empty = WidgetNode::<ActionDescriptor>::Section { id: "s".into(), label: None, default_open: true, children: vec![] };
            let with_child = WidgetNode::<ActionDescriptor>::Section { id: "s".into(), label: None, default_open: true, children: vec![WidgetNode::Separator] };
            let (_, empty_h) = measure_widget(&mut atlas, &theme, &empty);
            let (_, child_h) = measure_widget(&mut atlas, &theme, &with_child);
            assert!(child_h > empty_h, "adding a child must grow a Section's measured height beyond its bare header height");
        }

        #[test]
        fn measure_widget_tree_skips_dimmed_items_in_height() {
            let mut atlas = FontAtlas::builtin();
            let theme = Theme::default();
            let item = |id: &str, dimmed: bool| TreeItem {
                id: id.into(),
                label: id.into(),
                description: None,
                icon_id: None,
                selected: false,
                highlighted: false,
                default_open: false,
                dimmed,
                event: None,
                hover_event: None,
                unhover_event: None,
                actions: vec![],
                draggable: false,
                drag_data: StdHashMap::new(),
                control: None,
                children: vec![],
            };
            let visible =
                WidgetNode::<ActionDescriptor>::Tree { sections: vec![TreeSection { id: "sec".into(), label: None, default_open: true, items: vec![item("a", false)] }], selected_ids: vec![], highlighted_ids: vec![], selection_change: None };
            let dimmed = WidgetNode::<ActionDescriptor>::Tree { sections: vec![TreeSection { id: "sec".into(), label: None, default_open: true, items: vec![item("a", true)] }], selected_ids: vec![], highlighted_ids: vec![], selection_change: None };
            let (_, visible_h) = measure_widget(&mut atlas, &theme, &visible);
            let (_, dimmed_h) = measure_widget(&mut atlas, &theme, &dimmed);
            assert!(dimmed_h < visible_h, "a dimmed tree item must contribute zero height, so the dimmed tree must measure shorter than the visible one");
        }

        #[test]
        fn widget_interaction_maps_clear_frame_empties_every_map() {
            let mut maps = WidgetInteractionMaps::<ActionDescriptor>::default();
            maps.input_metas.insert("i".into(), InputMeta { on_change: action(), commit: None, value: "v".into() });
            maps.select_metas.insert("s".into(), action());
            maps.toggle_metas.insert("t".into(), (true, action()));
            maps.slider_metas.insert("sl".into(), SliderMeta { on_change: action(), min: 0.0, max: 1.0, step: 0.1, value: 0.5, bounds_x: 0.0, bounds_w: 10.0 });
            maps.stepper_metas.insert("st".into(), StepperMeta { on_absolute: action(), on_delta: action(), step: 1.0, value: 1.0 });
            maps.ring_metas.insert("r".into(), RingMeta { on_change: action(), disabled: false, center_x: 0.0, center_y: 0.0, radius: 10.0 });
            maps.slider_live_values.insert("sl".into(), 0.5);
            maps.ring_live_values.insert("r".into(), 0.5);
            maps.tree_hover_commands.insert("h".into(), action());
            maps.tree_unhover_commands.insert("u".into(), action());
            maps.tree_selection_change = Some(action());

            maps.clear_frame();

            assert!(maps.input_metas.is_empty());
            assert!(maps.select_metas.is_empty());
            assert!(maps.toggle_metas.is_empty());
            assert!(maps.slider_metas.is_empty());
            assert!(maps.stepper_metas.is_empty());
            assert!(maps.ring_metas.is_empty());
            assert!(maps.slider_live_values.is_empty());
            assert!(maps.ring_live_values.is_empty());
            assert!(maps.tree_hover_commands.is_empty());
            assert!(maps.tree_unhover_commands.is_empty());
            assert!(maps.tree_selection_change.is_none());
        }

        #[test]
        fn render_widget_input_registers_interaction_meta_when_maps_present() {
            let mut h = WidgetHarness::new();
            let node = WidgetNode::Input { id: "in".into(), input_kind: "text".into(), value: "hello".into(), placeholder: None, commit: Some("blur".into()), on_change: Some(action()) };
            render_widget(&node, VIEWPORT, &mut h.ctx());
            let meta = h.maps.input_metas.get("in").expect("register_input_meta must populate the map when interaction_maps is Some and on_change is Some");
            assert_eq!(meta.value, "hello");
            assert_eq!(meta.commit.as_deref(), Some("blur"));
        }

        #[test]
        fn render_widget_input_with_no_on_change_does_not_register_meta() {
            let mut h = WidgetHarness::new();
            let node = WidgetNode::Input { id: "in".into(), input_kind: "text".into(), value: "hello".into(), placeholder: None, commit: None, on_change: None };
            render_widget(&node, VIEWPORT, &mut h.ctx());
            assert!(h.maps.input_metas.is_empty(), "no on_change means nothing should be wired for the host to fire");
        }

        #[test]
        fn render_widget_select_and_toggle_register_interaction_metas() {
            let mut h = WidgetHarness::new();
            let select = WidgetNode::Select { id: "sel".into(), value: "a".into(), items: vec![SelectItem { value: "a".into(), label: "Alpha".into() }], placeholder: None, on_change: Some(action()) };
            render_widget(&select, VIEWPORT, &mut h.ctx());
            assert!(h.maps.select_metas.contains_key("sel"));

            let toggle = WidgetNode::Toggle { id: "tog".into(), icon_id: IconName::CircleDot, pressed: true, text: Some("On".into()), on_change: Some(action()) };
            render_widget(&toggle, VIEWPORT, &mut h.ctx());
            let (pressed, _) = h.maps.toggle_metas.get("tog").expect("toggle meta must be registered");
            assert!(*pressed);
        }

        #[test]
        fn render_widget_slider_registers_meta_and_live_value_unless_disabled() {
            let mut h = WidgetHarness::new();
            let enabled = WidgetNode::Slider { id: "sl".into(), value: 0.5, min: 0.0, max: 1.0, step: 0.01, ready: None, disabled: false, on_change: Some(action()) };
            render_widget(&enabled, VIEWPORT, &mut h.ctx());
            assert!(h.maps.slider_metas.contains_key("sl"));
            assert!(h.maps.slider_live_values.contains_key("sl"));

            let mut h2 = WidgetHarness::new();
            let disabled = WidgetNode::Slider { id: "sl".into(), value: 0.5, min: 0.0, max: 1.0, step: 0.01, ready: None, disabled: true, on_change: Some(action()) };
            render_widget(&disabled, VIEWPORT, &mut h2.ctx());
            assert!(h2.maps.slider_metas.is_empty(), "a disabled slider must not register interaction metadata");
            assert!(h2.maps.slider_live_values.is_empty());
        }

        #[test]
        fn render_widget_number_stepper_registers_stepper_meta() {
            let mut h = WidgetHarness::new();
            let node = WidgetNode::NumberStepper { id: "ns".into(), value: 3.0, step: 1.0, uniform: false, on_absolute: Some(action()), on_delta: Some(action()) };
            render_widget(&node, VIEWPORT, &mut h.ctx());
            let meta = h.maps.stepper_metas.get("ns").expect("stepper meta must be registered when both on_absolute and on_delta are Some");
            assert_eq!(meta.value, 3.0);
            assert!(h.maps.input_metas.contains_key("ns.input"), "the stepper's embedded value segment renders through render_input and must also register an input meta");
        }

        #[test]
        fn render_widget_ring_registers_meta_and_live_value() {
            let mut h = WidgetHarness::new();
            let node = WidgetNode::Ring { id: "r".into(), t: 0.25, disabled: false, on_change: Some(action()) };
            render_widget(&node, VIEWPORT, &mut h.ctx());
            assert!(h.maps.ring_metas.contains_key("r"));
            assert_eq!(h.maps.ring_live_values.get("r"), Some(&0.25));
        }

        #[test]
        fn render_widget_field_draws_label_and_delegates_to_control() {
            let mut h = WidgetHarness::new();
            let node = WidgetNode::Field { id: "f".into(), label: "Name".into(), child: ControlNode::Input { id: "in".into(), input_kind: "text".into(), value: "x".into(), placeholder: None, commit: None, on_change: Some(action()) } };
            render_widget(&node, VIEWPORT, &mut h.ctx());
            assert!(h.maps.input_metas.contains_key("in"), "Field must render its child control (an Input here), which registers its own interaction meta");
            let total: usize = h.draw.layers.iter().map(|l| l.ui_instances.len()).sum();
            assert!(total > 0, "Field must paint its label plus its child control");
        }

        #[test]
        fn render_widget_section_toggles_collapsed_state_from_default_open() {
            let child = || WidgetNode::<ActionDescriptor>::Text { value: "child text".into(), emphasize: false };
            let mut h = WidgetHarness::new();
            let closed = WidgetNode::<ActionDescriptor>::Section { id: "sec".into(), label: Some("Sec".into()), default_open: false, children: vec![child()] };
            render_widget(&closed, VIEWPORT, &mut h.ctx());
            assert_eq!(h.collapsed_sections.get("section.sec"), Some(&true), "a Section with default_open: false must seed its collapsed_sections entry as collapsed");

            let mut h2 = WidgetHarness::new();
            let open = WidgetNode::<ActionDescriptor>::Section { id: "sec".into(), label: Some("Sec".into()), default_open: true, children: vec![child()] };
            render_widget(&open, VIEWPORT, &mut h2.ctx());
            assert_eq!(h2.collapsed_sections.get("section.sec"), Some(&false));
            let closed_instances: usize = h.draw.layers.iter().map(|l| l.ui_instances.len()).sum();
            let open_instances: usize = h2.draw.layers.iter().map(|l| l.ui_instances.len()).sum();
            assert!(open_instances > closed_instances, "an open section must also paint its (visible) child's glyphs, a collapsed one must not");
        }

        #[test]
        fn render_widget_tree_populates_hover_and_unhover_commands() {
            let mut h = WidgetHarness::new();
            let item = TreeItem {
                id: "i1".into(),
                label: "Item".into(),
                description: None,
                icon_id: None,
                selected: false,
                highlighted: false,
                default_open: false,
                dimmed: false,
                event: None,
                hover_event: Some(action()),
                unhover_event: Some(action()),
                actions: vec![],
                draggable: false,
                drag_data: StdHashMap::new(),
                control: None,
                children: vec![],
            };
            let node = WidgetNode::<ActionDescriptor>::Tree {
                sections: vec![TreeSection { id: "s".into(), label: Some("Section".into()), default_open: true, items: vec![item] }],
                selected_ids: vec![],
                highlighted_ids: vec![],
                selection_change: Some(action()),
            };
            render_widget(&node, VIEWPORT, &mut h.ctx());
            assert!(h.maps.tree_hover_commands.contains_key("i1"));
            assert!(h.maps.tree_unhover_commands.contains_key("i1"));
            assert_eq!(h.maps.tree_selection_change, Some(action()));
        }

        #[test]
        fn render_widget_tree_row_actions_register_hits_without_hover() {
            let mut h = WidgetHarness::new();
            let item = TreeItem {
                id: "i1".into(),
                label: "Item".into(),
                description: None,
                icon_id: None,
                selected: false,
                highlighted: false,
                default_open: false,
                dimmed: false,
                event: None,
                hover_event: None,
                unhover_event: None,
                actions: vec![TreeItemAction { icon_id: IconName::CircleDot, label: Some("Del".into()), event: action(), placement: UiTreeActionPlacement::Row }],
                draggable: false,
                drag_data: StdHashMap::new(),
                control: None,
                children: vec![],
            };
            let node = WidgetNode::<ActionDescriptor>::Tree { sections: vec![TreeSection { id: "s".into(), label: None, default_open: true, items: vec![item] }], selected_ids: vec![], highlighted_ids: vec![], selection_change: None };
            render_widget(&node, VIEWPORT, &mut h.ctx());
            let action_hits = h.input.hit_targets.iter().filter(|t| t.control_id.as_deref() == Some("tree.action.i1.0")).count();
            assert_eq!(action_hits, 1, "row-placement actions must register a hit target even when the row is unhovered");
        }

        #[test]
        fn render_widget_tree_menu_placement_skips_row_action_hits() {
            let mut h = WidgetHarness::new();
            let item = TreeItem {
                id: "i1".into(),
                label: "Item".into(),
                description: None,
                icon_id: None,
                selected: false,
                highlighted: false,
                default_open: false,
                dimmed: false,
                event: None,
                hover_event: None,
                unhover_event: None,
                actions: vec![TreeItemAction { icon_id: IconName::CircleDot, label: Some("Del".into()), event: action(), placement: UiTreeActionPlacement::Menu }],
                draggable: false,
                drag_data: StdHashMap::new(),
                control: None,
                children: vec![],
            };
            let node = WidgetNode::<ActionDescriptor>::Tree { sections: vec![TreeSection { id: "s".into(), label: None, default_open: true, items: vec![item] }], selected_ids: vec![], highlighted_ids: vec![], selection_change: None };
            render_widget(&node, VIEWPORT, &mut h.ctx());
            let action_hits = h.input.hit_targets.iter().filter(|t| t.control_id.as_deref() == Some("tree.action.i1.0")).count();
            assert_eq!(action_hits, 0, "menu-placement actions must not register row hit targets");
        }

        #[test]
        fn render_widget_tree_marks_selected_and_highlighted_ids_via_ids_list() {
            let mut h = WidgetHarness::new();
            let item = TreeItem {
                id: "i1".into(),
                label: "Item".into(),
                description: None,
                icon_id: None,
                selected: false,
                highlighted: false,
                default_open: false,
                dimmed: false,
                event: Some(action()),
                hover_event: None,
                unhover_event: None,
                actions: vec![],
                draggable: false,
                drag_data: StdHashMap::new(),
                control: None,
                children: vec![],
            };
            let node = WidgetNode::<ActionDescriptor>::Tree { sections: vec![TreeSection { id: "s".into(), label: None, default_open: true, items: vec![item] }], selected_ids: vec!["i1".into()], highlighted_ids: vec![], selection_change: None };
            render_widget(&node, VIEWPORT, &mut h.ctx());
            let hit = h.input.hit_targets.iter().find(|t| t.control_id.as_deref() == Some("tree.label.i1")).expect("tree item label must register a hit target");
            assert_eq!(hit.event, Some(action()));
        }

        #[test]
        fn render_scroll_region_clamps_stale_offset_to_new_max_scroll() {
            let mut h = WidgetHarness::new();
            h.scroll_offsets.insert("scroll".into(), 500.0);
            let bounds = Rect::new(0.0, 0.0, 100.0, 100.0);
            {
                let mut ctx = h.ctx();
                render_scroll_region("scroll", bounds, 150.0, &mut ctx, |_content, _ctx| {});
            }
            assert_eq!(h.scroll_offsets.get("scroll"), Some(&50.0), "offset must clamp to max_scroll (content_height - bounds.h) even if a stale value was larger");
        }

        #[test]
        fn render_scroll_region_registers_a_scroll_region_hit_target() {
            let mut h = WidgetHarness::new();
            let bounds = Rect::new(0.0, 0.0, 100.0, 100.0);
            {
                let mut ctx = h.ctx();
                render_scroll_region("myscroll", bounds, 400.0, &mut ctx, |_content, _ctx| {});
            }
            assert!(h.input.hit_targets.iter().any(|t| t.control_id.as_deref() == Some("myscroll")));
        }

        #[test]
        fn draw_text_on_emits_one_glyph_instance_per_character() {
            let mut draw = DrawList::default();
            let mut atlas = FontAtlas::builtin();
            draw_text_on(&mut draw, &mut atlas, "abc", 0.0, 0.0, 16.0, Theme::default().text);
            let total: usize = draw.layers.iter().map(|l| l.ui_instances.len()).sum();
            assert_eq!(total, 3);
        }

        #[test]
        fn draw_text_overlay_on_writes_to_the_overlay_channel_not_the_main_one() {
            let mut draw = DrawList::default();
            let mut atlas = FontAtlas::builtin();
            draw_text_overlay_on(&mut draw, &mut atlas, "hi", 0.0, 0.0, 16.0, Theme::default().text);
            let main: usize = draw.layers.iter().map(|l| l.ui_instances.len()).sum();
            let overlay: usize = draw.layers.iter().map(|l| l.overlay_ui_instances.len()).sum();
            assert_eq!(main, 0, "overlay glyphs must not land in the main ui_instances channel");
            assert_eq!(overlay, 2, "one overlay glyph instance per character");
        }
        //#endregion 🧩️WidgetsInternalsTests
    }
    // #endregion engine
}

#[cfg(feature = "engine")]
pub mod widgets {
    // #region widgets
    //! 🧩️ Generic widget tree — layout, measurement, and drawing.

    use crate::chrome::{chrome_item_bg, item_bg, item_text, push_control_border, push_icon, ICON_TINY};
    use crate::draw::{DrawList, IconAtlas};
    use crate::geometry::Rect;
    use crate::input::{DragAxis, HitKind, HitTarget, InputState};
    use crate::layout::{gap_for_token, layout_horizontal, layout_vertical, padding_for_token};
    use crate::text::FontAtlas;
    use crate::theme::{Level, Rgba, Theme};
    use crate::IconName;
    use crate::UiTreeActionPlacement;
    use std::collections::HashMap;

    #[derive(Clone, Debug)]
    pub struct InputMeta<E> {
        pub on_change: E,
        pub commit: Option<String>,
        pub value: String,
    }

    #[derive(Clone, Debug)]
    pub struct SliderMeta<E> {
        pub on_change: E,
        pub min: f64,
        pub max: f64,
        pub step: f64,
        pub value: f64,
        pub bounds_x: f32,
        pub bounds_w: f32,
    }

    #[derive(Clone, Debug)]
    pub struct StepperMeta<E> {
        pub on_absolute: E,
        pub on_delta: E,
        pub step: f64,
        pub value: f64,
    }

    #[derive(Clone, Debug)]
    pub struct RingMeta<E> {
        pub on_change: E,
        pub disabled: bool,
        pub center_x: f32,
        pub center_y: f32,
        pub radius: f32,
    }

    pub struct WidgetInteractionMaps<E> {
        pub input_metas: HashMap<String, InputMeta<E>>,
        pub select_metas: HashMap<String, E>,
        pub toggle_metas: HashMap<String, (bool, E)>,
        pub slider_metas: HashMap<String, SliderMeta<E>>,
        pub stepper_metas: HashMap<String, StepperMeta<E>>,
        pub ring_metas: HashMap<String, RingMeta<E>>,
        pub slider_live_values: HashMap<String, f64>,
        pub ring_live_values: HashMap<String, f64>,
        pub tree_hover_commands: HashMap<String, E>,
        pub tree_unhover_commands: HashMap<String, E>,
        pub tree_selection_change: Option<E>,
    }

    impl<E> Default for WidgetInteractionMaps<E> {
        fn default() -> Self {
            Self {
                input_metas: HashMap::new(),
                select_metas: HashMap::new(),
                toggle_metas: HashMap::new(),
                slider_metas: HashMap::new(),
                stepper_metas: HashMap::new(),
                ring_metas: HashMap::new(),
                slider_live_values: HashMap::new(),
                ring_live_values: HashMap::new(),
                tree_hover_commands: HashMap::new(),
                tree_unhover_commands: HashMap::new(),
                tree_selection_change: None,
            }
        }
    }

    impl<E> WidgetInteractionMaps<E> {
        pub fn clear_frame(&mut self) {
            self.input_metas.clear();
            self.select_metas.clear();
            self.toggle_metas.clear();
            self.slider_metas.clear();
            self.stepper_metas.clear();
            self.ring_metas.clear();
            self.slider_live_values.clear();
            self.ring_live_values.clear();
            self.tree_hover_commands.clear();
            self.tree_unhover_commands.clear();
            self.tree_selection_change = None;
        }
    }

    pub struct WidgetContext<'a, E> {
        pub draw: &'a mut DrawList,
        pub overlay: Option<&'a mut DrawList>,
        pub atlas: &'a mut FontAtlas,
        pub icons: Option<&'a IconAtlas>,
        pub input: &'a mut InputState<E>,
        pub theme: &'a Theme,
        pub scroll_offsets: &'a mut HashMap<String, f32>,
        pub collapsed_sections: &'a mut HashMap<String, bool>,
        pub open_selects: &'a mut HashMap<String, bool>,
        pub interaction_maps: Option<&'a mut WidgetInteractionMaps<E>>,
        pub pick_clip: Option<Rect>,
    }

    #[derive(Clone, Debug)]
    pub struct SelectItem {
        pub value: String,
        pub label: String,
    }

    #[derive(Clone, Debug)]
    pub struct KeyValueEntry {
        pub label: String,
        pub value: String,
    }

    #[derive(Clone, Debug)]
    pub struct TreeItemAction<E> {
        pub icon_id: IconName,
        pub label: Option<String>,
        pub event: E,
        pub placement: UiTreeActionPlacement,
    }

    #[derive(Clone, Debug)]
    pub struct TreeItem<E> {
        pub id: String,
        pub label: String,
        pub description: Option<String>,
        pub icon_id: Option<IconName>,
        pub selected: bool,
        pub highlighted: bool,
        pub default_open: bool,
        pub dimmed: bool,
        pub event: Option<E>,
        pub hover_event: Option<E>,
        pub unhover_event: Option<E>,
        pub actions: Vec<TreeItemAction<E>>,
        pub draggable: bool,
        pub drag_data: HashMap<String, String>,
        pub control: Option<Box<WidgetNode<E>>>,
        pub children: Vec<TreeItem<E>>,
    }

    #[derive(Clone, Debug)]
    pub struct TreeSection<E> {
        pub id: String,
        pub label: Option<String>,
        pub default_open: bool,
        pub items: Vec<TreeItem<E>>,
    }

    #[derive(Clone, Debug)]
    pub enum ControlNode<E> {
        Button { id: Option<String>, icon_id: Option<IconName>, label: String, event: Option<E> },
        Input { id: String, input_kind: String, value: String, placeholder: Option<String>, commit: Option<String>, on_change: Option<E> },
        Select { id: String, value: String, items: Vec<SelectItem>, placeholder: Option<String>, on_change: Option<E> },
        Toggle { id: String, icon_id: IconName, pressed: bool, text: Option<String>, on_change: Option<E> },
        KeyValue { entries: Vec<KeyValueEntry> },
        Slider { id: String, value: f64, min: f64, max: f64, step: f64, ready: Option<f64>, disabled: bool, on_change: Option<E> },
        NumberStepper { id: String, value: f64, step: f64, uniform: bool, on_absolute: Option<E>, on_delta: Option<E> },
        Ring { id: String, t: f64, disabled: bool, on_change: Option<E> },
        IconSelect { id: String, value: String, uniform: bool, classifier_kind: String, on_change: Option<E> },
    }

    #[derive(Clone, Debug)]
    pub enum WidgetNode<E> {
        Stack { direction: String, gap: Option<String>, padding: Option<String>, children: Vec<WidgetNode<E>> },
        Text { value: String, emphasize: bool },
        Separator,
        Button { id: Option<String>, icon_id: Option<IconName>, label: String, event: Option<E> },
        Input { id: String, input_kind: String, value: String, placeholder: Option<String>, commit: Option<String>, on_change: Option<E> },
        Select { id: String, value: String, items: Vec<SelectItem>, placeholder: Option<String>, on_change: Option<E> },
        Toggle { id: String, icon_id: IconName, pressed: bool, text: Option<String>, on_change: Option<E> },
        KeyValue { entries: Vec<KeyValueEntry> },
        Slider { id: String, value: f64, min: f64, max: f64, step: f64, ready: Option<f64>, disabled: bool, on_change: Option<E> },
        NumberStepper { id: String, value: f64, step: f64, uniform: bool, on_absolute: Option<E>, on_delta: Option<E> },
        Ring { id: String, t: f64, disabled: bool, on_change: Option<E> },
        IconSelect { id: String, value: String, uniform: bool, classifier_kind: String, on_change: Option<E> },
        Field { id: String, label: String, child: ControlNode<E> },
        Section { id: String, label: Option<String>, default_open: bool, children: Vec<WidgetNode<E>> },
        Tree { sections: Vec<TreeSection<E>>, selected_ids: Vec<String>, highlighted_ids: Vec<String>, selection_change: Option<E> },
    }

    const PANEL_HEADER: f32 = 24.0;
    const TREE_ROW_HEIGHT: f32 = 24.0;
    const TREE_INDENT_PER_LEVEL: f32 = 10.0;
    const TREE_TOGGLE_WIDTH: f32 = 14.0;
    const TREE_ICON_SIZE: f32 = 14.0;
    const TREE_SECTION_GAP: f32 = 8.0;

    pub fn measure_widget<E>(atlas: &mut FontAtlas, theme: &Theme, node: &WidgetNode<E>) -> (f32, f32) {
        match node {
            WidgetNode::Stack { direction, gap, padding, children } => {
                let gap = gap_for_token(theme, gap.as_deref());
                let padding = padding_for_token(theme, padding.as_deref()) * 2.0;
                let vertical = direction != "horizontal";
                let mut total_main = 0.0f32;
                let mut max_cross = 0.0f32;
                for (index, child) in children.iter().enumerate() {
                    let (w, h) = measure_widget(atlas, theme, child);
                    if vertical {
                        total_main += h;
                        max_cross = max_cross.max(w);
                        if index + 1 < children.len() {
                            total_main += gap;
                        }
                    } else {
                        total_main += w;
                        max_cross = max_cross.max(h);
                        if index + 1 < children.len() {
                            total_main += gap;
                        }
                    }
                }
                if vertical {
                    (max_cross + padding, total_main + padding)
                } else {
                    (total_main + padding, max_cross + padding)
                }
            }
            WidgetNode::Text { value, emphasize } => {
                let size = if *emphasize { theme.font_size_emphasized } else { theme.font_size_body };
                let (w, _) = atlas.measure_text(value, size);
                let lines = wrap_text(atlas, value, w.max(120.0), size);
                (w.max(120.0), lines.len() as f32 * size * 1.35)
            }
            WidgetNode::Separator => (theme.control_height.max(1.0), 1.0 + theme.gap_standard),
            WidgetNode::Button { .. } | WidgetNode::Input { .. } | WidgetNode::Select { .. } | WidgetNode::Toggle { .. } | WidgetNode::Slider { .. } | WidgetNode::NumberStepper { .. } | WidgetNode::IconSelect { .. } => {
                (theme.control_height, theme.control_height)
            }
            WidgetNode::KeyValue { entries } => {
                let label_w = entries.iter().map(|e| atlas.measure_text(&e.label, theme.font_size_small).0).fold(0.0f32, f32::max);
                (label_w + theme.gap_standard * 2.0 + 80.0, entries.len() as f32 * theme.control_height)
            }
            WidgetNode::Ring { .. } => (80.0, 80.0),
            WidgetNode::Field { label, child, .. } => {
                let label_h = theme.font_size_small;
                let gap = gap_for_token(theme, Some("standard"));
                let (cw, ch) = measure_control(atlas, theme, child);
                (cw.max(atlas.measure_text(label, theme.font_size_small).0), label_h + gap + ch)
            }
            WidgetNode::Section { children, label, .. } => {
                let mut height = PANEL_HEADER;
                let mut max_w = 0.0f32;
                if label.is_some() {
                    max_w = max_w.max(160.0);
                }
                for child in children {
                    let (w, h) = measure_widget(atlas, theme, child);
                    max_w = max_w.max(w);
                    height += h + theme.gap_standard;
                }
                (max_w.max(120.0), height)
            }
            WidgetNode::Tree { sections, .. } => (measure_tree_sections_width(sections, atlas, theme), measure_tree_sections(sections)),
        }
    }

    fn measure_control<E>(atlas: &mut FontAtlas, theme: &Theme, control: &ControlNode<E>) -> (f32, f32) {
        match control {
            ControlNode::Button { .. } | ControlNode::Input { .. } | ControlNode::Select { .. } | ControlNode::Toggle { .. } | ControlNode::Slider { .. } | ControlNode::NumberStepper { .. } | ControlNode::IconSelect { .. } => {
                (theme.control_height, theme.control_height)
            }
            ControlNode::KeyValue { entries } => {
                let label_w = entries.iter().map(|e| atlas.measure_text(&e.label, theme.font_size_small).0).fold(0.0f32, f32::max);
                (label_w + theme.gap_standard * 2.0 + 80.0, entries.len() as f32 * theme.control_height)
            }
            ControlNode::Ring { .. } => (80.0, 80.0),
        }
    }

    pub fn render_widget<E: Clone>(node: &WidgetNode<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
        match node {
            WidgetNode::Stack { direction, gap, padding, children } => {
                let gap = gap_for_token(ctx.theme, gap.as_deref());
                let padding = padding_for_token(ctx.theme, padding.as_deref());
                let vertical = direction != "horizontal";
                let sizes: Vec<f32> = children
                    .iter()
                    .map(|child| {
                        let (w, h) = measure_widget(ctx.atlas, ctx.theme, child);
                        if vertical {
                            h
                        } else {
                            w
                        }
                    })
                    .collect();
                let rects = if vertical { layout_vertical(bounds, gap, padding, &sizes) } else { layout_horizontal(bounds, gap, padding, &sizes) };
                for (child, rect) in children.iter().zip(rects.iter()) {
                    render_widget(child, *rect, ctx);
                }
            }
            WidgetNode::Text { value, emphasize } => {
                let size = if *emphasize { ctx.theme.font_size_emphasized } else { ctx.theme.font_size_body };
                let color = if *emphasize { ctx.theme.text } else { ctx.theme.text_muted };
                draw_text_wrapped(ctx, value, bounds.x, bounds.y, bounds.w.max(1.0), size, color);
            }
            WidgetNode::Separator => {
                let y = bounds.y + bounds.h * 0.5;
                ctx.draw.push_line(bounds.x, y, bounds.x + bounds.w, y, ctx.theme.separator, 1.0);
            }
            WidgetNode::Button { id, icon_id, label, event } => {
                render_button(id.as_ref(), *icon_id, label, event.clone(), bounds, ctx);
            }
            WidgetNode::Input { id, value, placeholder, commit, on_change, .. } => {
                register_input_meta(ctx, id, value, commit.clone(), on_change.clone());
                render_input(id, value, placeholder.as_deref(), bounds, ctx);
            }
            WidgetNode::Select { id, value, items, placeholder, on_change } => {
                register_select_meta(ctx, id, on_change.clone());
                render_select(id, value, items, placeholder.as_deref(), bounds, ctx);
            }
            WidgetNode::Toggle { id, icon_id, pressed, text, on_change } => {
                register_toggle_meta(ctx, id, *pressed, on_change.clone());
                render_toggle(id, *icon_id, *pressed, text.as_deref(), bounds, ctx);
            }
            WidgetNode::KeyValue { entries } => render_key_value(entries, bounds, ctx),
            WidgetNode::Slider { id, value, min, max, step, ready, disabled, on_change } => {
                render_slider(id, *value, *min, *max, *step, *ready, *disabled, on_change.clone(), bounds, ctx);
            }
            WidgetNode::NumberStepper { id, value, step, uniform, on_absolute, on_delta } => {
                render_number_stepper(id, *value, *step, *uniform, on_absolute.clone(), on_delta.clone(), bounds, ctx);
            }
            WidgetNode::Ring { id, t, disabled, on_change } => {
                render_ring(id, *t, *disabled, on_change.clone(), bounds, ctx);
            }
            WidgetNode::IconSelect { id, value, uniform, classifier_kind, on_change } => {
                render_icon_select(id, value, *uniform, classifier_kind, on_change.clone(), bounds, ctx);
            }
            WidgetNode::Field { label, child, .. } => {
                let label_h = ctx.theme.font_size_small;
                let gap = gap_for_token(ctx.theme, Some("standard"));
                draw_text(ctx, label, bounds.x, bounds.y + label_h, ctx.theme.font_size_small, ctx.theme.text_muted);
                let child_bounds = Rect::new(bounds.x, bounds.y + label_h + gap, bounds.w, bounds.h - label_h - gap);
                render_control(child, child_bounds, ctx);
            }
            WidgetNode::Section { label, children, id, default_open } => {
                let section_key = format!("section.{id}");
                if !ctx.collapsed_sections.contains_key(&section_key) {
                    ctx.collapsed_sections.insert(section_key.clone(), !default_open);
                }
                let collapsed = tree_row_collapsed(ctx.collapsed_sections, &section_key, *default_open);
                if label.is_some() {
                    let header = Rect::new(bounds.x, bounds.y, bounds.w, PANEL_HEADER);
                    let chevron_rect = Rect::new(bounds.x, bounds.y, TREE_TOGGLE_WIDTH, PANEL_HEADER);
                    let chevron = if collapsed { "chevron-right" } else { "chevron-down" };
                    tree_draw_chevron(ctx, chevron, chevron_rect);
                    if let Some(label) = label {
                        draw_text(ctx, label, bounds.x + TREE_TOGGLE_WIDTH + ctx.theme.gap_standard, bounds.y + (PANEL_HEADER + ctx.theme.font_size_body) * 0.5 - 2.0, ctx.theme.font_size_body, ctx.theme.text);
                    }
                    ctx.input.register_hit(HitTarget { rect: header, event: None, control_id: Some(format!("section.chevron.{id}")), kind: HitKind::Generic, drag_axis: None, drag_data: None });
                }
                if !collapsed {
                    let mut y = bounds.y + PANEL_HEADER;
                    for child in children {
                        let (_, h) = measure_widget(ctx.atlas, ctx.theme, child);
                        let child_bounds = Rect::new(bounds.x, y, bounds.w, h);
                        render_widget(child, child_bounds, ctx);
                        y += h + ctx.theme.gap_standard;
                    }
                }
            }
            WidgetNode::Tree { sections, selected_ids, highlighted_ids, selection_change } => {
                if let Some(maps) = ctx.interaction_maps.as_deref_mut() {
                    maps.tree_selection_change = selection_change.clone();
                }
                let scroll_id = format!("tree:{:.0}:{:.0}", bounds.x, bounds.y);
                let content_h = measure_tree_sections_state(sections, ctx.collapsed_sections);
                render_scroll_region(&scroll_id, bounds, content_h.max(bounds.h), ctx, |content, ctx| {
                    render_tree(sections, selected_ids, highlighted_ids, content, ctx);
                });
            }
        }
    }

    fn render_control<E: Clone>(control: &ControlNode<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
        match control {
            ControlNode::Button { id, icon_id, label, event } => {
                render_button(id.as_ref(), *icon_id, label, event.clone(), bounds, ctx);
            }
            ControlNode::Input { id, value, placeholder, commit, on_change, .. } => {
                register_input_meta(ctx, id, value, commit.clone(), on_change.clone());
                render_input(id, value, placeholder.as_deref(), bounds, ctx);
            }
            ControlNode::Select { id, value, items, placeholder, on_change } => {
                register_select_meta(ctx, id, on_change.clone());
                render_select(id, value, items, placeholder.as_deref(), bounds, ctx);
            }
            ControlNode::Toggle { id, icon_id, pressed, text, on_change } => {
                register_toggle_meta(ctx, id, *pressed, on_change.clone());
                render_toggle(id, *icon_id, *pressed, text.as_deref(), bounds, ctx);
            }
            ControlNode::KeyValue { entries } => render_key_value(entries, bounds, ctx),
            ControlNode::Slider { id, value, min, max, step, ready, disabled, on_change } => {
                render_slider(id, *value, *min, *max, *step, *ready, *disabled, on_change.clone(), bounds, ctx);
            }
            ControlNode::NumberStepper { id, value, step, uniform, on_absolute, on_delta } => {
                render_number_stepper(id, *value, *step, *uniform, on_absolute.clone(), on_delta.clone(), bounds, ctx);
            }
            ControlNode::Ring { id, t, disabled, on_change } => render_ring(id, *t, *disabled, on_change.clone(), bounds, ctx),
            ControlNode::IconSelect { id, value, uniform, classifier_kind, on_change } => {
                render_icon_select(id, value, *uniform, classifier_kind, on_change.clone(), bounds, ctx);
            }
        }
    }

    fn register_input_meta<E: Clone>(ctx: &mut WidgetContext<'_, E>, id: &str, value: &str, commit: Option<String>, on_change: Option<E>) {
        if let (Some(maps), Some(on_change)) = (ctx.interaction_maps.as_deref_mut(), on_change) {
            maps.input_metas.insert(id.to_string(), InputMeta { on_change, commit, value: value.to_string() });
        }
    }

    fn register_select_meta<E: Clone>(ctx: &mut WidgetContext<'_, E>, id: &str, on_change: Option<E>) {
        if let (Some(maps), Some(on_change)) = (ctx.interaction_maps.as_deref_mut(), on_change) {
            maps.select_metas.insert(id.to_string(), on_change);
        }
    }

    fn register_toggle_meta<E: Clone>(ctx: &mut WidgetContext<'_, E>, id: &str, pressed: bool, on_change: Option<E>) {
        if let (Some(maps), Some(on_change)) = (ctx.interaction_maps.as_deref_mut(), on_change) {
            maps.toggle_metas.insert(id.to_string(), (pressed, on_change));
        }
    }

    fn render_button<E: Clone>(id: Option<&String>, icon_id: Option<IconName>, label: &str, event: Option<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
        let control_id = id.cloned().or_else(|| Some(label.to_string()));
        let hovered = ctx.input.hovered_id == control_id;
        let bg = item_bg(ctx.theme, false, hovered);
        push_control_border(ctx.draw, bounds, ctx.theme, ctx.theme.border_normal, bg);
        let mut text_x = bounds.x + ctx.theme.padding_standard;
        let icon_key = icon_id.filter(|id| *id != IconName::CircleDot).map(IconName::as_str).unwrap_or(label);
        if let Some(icons) = ctx.icons {
            if icons.icon_uv(icon_key).is_some() {
                push_icon(ctx.draw, icons, icon_key, text_x, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, item_text(ctx.theme, false, hovered));
                text_x += ICON_TINY + ctx.theme.gap_standard;
            }
        }
        draw_text(ctx, label, text_x, bounds.y + (bounds.h + ctx.theme.font_size_body) * 0.5 - 2.0, ctx.theme.font_size_body, item_text(ctx.theme, false, hovered));
        ctx.input.register_hit(HitTarget { rect: bounds, event, control_id, kind: HitKind::Button, drag_axis: None, drag_data: None });
    }

    fn render_input<E: Clone>(id: &str, value: &str, placeholder: Option<&str>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
        let focused = ctx.input.focused_id.as_deref() == Some(id);
        let border = if focused { ctx.theme.border_emphasized } else { ctx.theme.border_normal };
        push_control_border(ctx.draw, bounds, ctx.theme, border, ctx.theme.input_bg);
        let (display, muted) = if focused {
            (ctx.input.text_buffer.clone(), false)
        } else if value.is_empty() {
            (placeholder.unwrap_or("").to_string(), true)
        } else {
            (value.to_string(), false)
        };
        draw_text(ctx, &display, bounds.x + 8.0, bounds.y + (bounds.h + ctx.theme.font_size_body) * 0.5 - 2.0, ctx.theme.font_size_body, if muted { ctx.theme.text_muted } else { ctx.theme.text });
        if focused {
            let cursor_x = bounds.x + 8.0 + measure_text_width(ctx, &display[..ctx.input.cursor_pos.min(display.len())], ctx.theme.font_size_body);
            ctx.draw.push_solid([cursor_x, bounds.y + 6.0, 1.0, bounds.h - 12.0], ctx.theme.text);
        }
        ctx.input.register_hit(HitTarget { rect: bounds, event: None, control_id: Some(id.to_string()), kind: HitKind::Input, drag_axis: None, drag_data: None });
    }

    #[path = "../../../../🧱️elements/Select/🧊️component.rs"]
    mod select;
    pub(crate) use select::{render_select, render_select_menu};

    fn render_toggle<E: Clone>(id: &str, icon_id: IconName, pressed: bool, text: Option<&str>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
        let hovered = ctx.input.hovered_id.as_deref() == Some(id);
        let bg = item_bg(ctx.theme, pressed, hovered);
        push_control_border(ctx.draw, bounds, ctx.theme, ctx.theme.border_normal, bg);
        let mut content_x = bounds.x + ctx.theme.padding_standard;
        if let Some(icons) = ctx.icons {
            if icons.icon_uv(icon_id.as_str()).is_some() {
                push_icon(ctx.draw, icons, icon_id.as_str(), content_x, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, item_text(ctx.theme, pressed, hovered));
                content_x += ICON_TINY + ctx.theme.gap_standard;
            }
        }
        if let Some(text) = text {
            draw_text(ctx, text, content_x, bounds.y + (bounds.h + ctx.theme.font_size_body) * 0.5 - 2.0, ctx.theme.font_size_body, item_text(ctx.theme, pressed, hovered));
        }
        ctx.input.register_hit(HitTarget { rect: bounds, event: None, control_id: Some(id.to_string()), kind: HitKind::Toggle, drag_axis: None, drag_data: None });
    }

    fn render_key_value<E>(entries: &[KeyValueEntry], bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
        let label_w = entries.iter().map(|e| measure_text_width(ctx, &e.label, ctx.theme.font_size_small)).fold(0.0f32, f32::max);
        let value_x = bounds.x + label_w + ctx.theme.gap_standard * 2.0;
        let row_h = ctx.theme.control_height;
        for (index, entry) in entries.iter().enumerate() {
            let y = bounds.y + index as f32 * row_h;
            draw_text(ctx, &entry.label, bounds.x, y + (row_h + ctx.theme.font_size_small) * 0.5 - 1.0, ctx.theme.font_size_small, ctx.theme.text_muted);
            draw_text(ctx, &entry.value, value_x, y + (row_h + ctx.theme.font_size_small) * 0.5 - 1.0, ctx.theme.font_size_small, ctx.theme.text);
        }
    }

    fn quantize_step(value: f64, step: f64, min: f64) -> f64 {
        if step <= 0.0 {
            return value;
        }
        min + ((value - min) / step).round() * step
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per widget/render-context field; grouping into a struct is a T2 restructure, out of scope")]
    fn render_slider<E: Clone>(id: &str, value: f64, min: f64, max: f64, step: f64, ready: Option<f64>, disabled: bool, on_change: Option<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
        let track_y = bounds.y + bounds.h * 0.5;
        let dim = |color: Rgba| if disabled { color.with_alpha(color.a * 0.5) } else { color };
        ctx.draw.push_rounded([bounds.x, track_y - 2.0, bounds.w, 4.0], dim(ctx.theme.separator), 2.0);
        let range = (max - min).max(f64::EPSILON);
        let mut t = ((value - min) / range).clamp(0.0, 1.0);
        if !disabled && ctx.input.drag.active && ctx.input.drag.target_id.as_deref() == Some(id) {
            let dx = ctx.input.drag.current_x - ctx.input.drag.start_x;
            t = (t as f32 + dx / bounds.w.max(1.0)).clamp(0.0, 1.0) as f64;
        }
        let selectable_max = ready.map(|extent| extent.clamp(min, max)).unwrap_or(max);
        let live = quantize_step(min + t * range, step, min).clamp(min, selectable_max);
        if !disabled {
            if let Some(maps) = ctx.interaction_maps.as_deref_mut() {
                if let Some(on_change) = on_change {
                    maps.slider_metas.insert(id.to_string(), SliderMeta { on_change, min, max: selectable_max, step, value, bounds_x: bounds.x, bounds_w: bounds.w });
                }
                maps.slider_live_values.insert(id.to_string(), live);
            }
        }
        let value_t = ((live - min) / range).clamp(0.0, 1.0) as f32;
        if let Some(ready_extent) = ready {
            let ready_t = ((ready_extent.clamp(min, max) - min) / range).clamp(0.0, 1.0) as f32;
            if ready_t > value_t {
                let ready_x = bounds.x + bounds.w * value_t;
                let ready_w = bounds.w * (ready_t - value_t);
                ctx.draw.push_rounded([ready_x, track_y - 2.0, ready_w, 4.0], dim(Rgba::new(0.03433981, 0.63759687, 0.52099557, 1.0)), 2.0);
            }
        }
        let knob_x = bounds.x + bounds.w * value_t;
        ctx.draw.push_rounded([knob_x - 6.0, track_y - 6.0, 12.0, 12.0], dim(ctx.theme.accent), 6.0);
        if !disabled {
            ctx.input.register_hit(HitTarget { rect: bounds, event: None, control_id: Some(id.to_string()), kind: HitKind::Slider, drag_axis: Some(DragAxis::Horizontal), drag_data: None });
        }
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per widget/render-context field; grouping into a struct is a T2 restructure, out of scope")]
    fn render_number_stepper<E: Clone>(id: &str, value: f64, step: f64, _uniform: bool, on_absolute: Option<E>, on_delta: Option<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
        let seg = bounds.w / 3.0;
        let minus = Rect::new(bounds.x, bounds.y, seg, bounds.h);
        let center = Rect::new(bounds.x + seg, bounds.y, seg, bounds.h);
        let plus = Rect::new(bounds.x + seg * 2.0, bounds.y, seg, bounds.h);
        let hair = ctx.theme.stroke_hairline;
        push_control_border(ctx.draw, bounds, ctx.theme, ctx.theme.border_normal, ctx.theme.input_bg);
        ctx.draw.push_solid([bounds.x + seg, bounds.y, hair, bounds.h], ctx.theme.border_normal);
        ctx.draw.push_solid([bounds.x + seg * 2.0, bounds.y, hair, bounds.h], ctx.theme.border_normal);
        let minus_hovered = ctx.input.hovered_id.as_deref() == Some(&format!("{id}.minus"));
        let plus_hovered = ctx.input.hovered_id.as_deref() == Some(&format!("{id}.plus"));
        if minus_hovered {
            ctx.draw.push_solid([minus.x, minus.y, minus.w, minus.h], ctx.theme.button_hover);
        }
        if plus_hovered {
            ctx.draw.push_solid([plus.x, plus.y, plus.w, plus.h], ctx.theme.button_hover);
        }
        draw_text(ctx, "−", minus.x + seg * 0.5 - 4.0, minus.y + 18.0, ctx.theme.font_size_body, ctx.theme.text);
        let text = format!("{value:.3}");
        let input_id = format!("{id}.input");
        register_input_meta(ctx, &input_id, &text, None, on_absolute.clone());
        render_input(&input_id, &text, None, center, ctx);
        draw_text(ctx, "+", plus.x + seg * 0.5 - 4.0, plus.y + 18.0, ctx.theme.font_size_body, ctx.theme.text);
        if let (Some(maps), Some(on_absolute), Some(on_delta)) = (ctx.interaction_maps.as_deref_mut(), on_absolute, on_delta) {
            maps.stepper_metas.insert(id.to_string(), StepperMeta { on_absolute, on_delta, step, value });
        }
        ctx.input.register_hit(HitTarget { rect: minus, event: None, control_id: Some(format!("{id}.minus")), kind: HitKind::Generic, drag_axis: None, drag_data: None });
        ctx.input.register_hit(HitTarget { rect: plus, event: None, control_id: Some(format!("{id}.plus")), kind: HitKind::Generic, drag_axis: None, drag_data: None });
    }

    fn render_ring<E: Clone>(id: &str, t: f64, disabled: bool, on_change: Option<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
        let cx = bounds.x + bounds.w * 0.5;
        let cy = bounds.y + bounds.h * 0.5;
        let radius = bounds.w.min(bounds.h) * 0.4;
        let segments = 48usize;
        let mut points = Vec::with_capacity(segments + 1);
        for i in 0..=segments {
            let angle = std::f32::consts::TAU * i as f32 / segments as f32;
            points.push([cx + angle.cos() * radius, cy + angle.sin() * radius]);
        }
        for window in points.windows(2) {
            ctx.draw.push_line(window[0][0], window[0][1], window[1][0], window[1][1], ctx.theme.separator, 2.0);
        }
        let mut knob_t = t;
        if !disabled && ctx.input.drag.active && ctx.input.drag.target_id.as_deref() == Some(id) {
            let dx = ctx.input.drag.current_x - cx;
            let dy = ctx.input.drag.current_y - cy;
            knob_t = (dy.atan2(dx) as f64 / std::f64::consts::TAU).rem_euclid(1.0);
        }
        if let (Some(maps), Some(on_change)) = (ctx.interaction_maps.as_deref_mut(), on_change) {
            maps.ring_metas.insert(id.to_string(), RingMeta { on_change, disabled, center_x: cx, center_y: cy, radius });
            maps.ring_live_values.insert(id.to_string(), knob_t);
        }
        let knob_angle = std::f32::consts::TAU * knob_t as f32;
        let kx = cx + knob_angle.cos() * radius;
        let ky = cy + knob_angle.sin() * radius;
        let accent = if disabled { ctx.theme.text_muted } else { ctx.theme.accent };
        ctx.draw.push_rounded([kx - 6.0, ky - 6.0, 12.0, 12.0], accent, 6.0);
        if !disabled {
            ctx.input.register_hit(HitTarget { rect: bounds, event: None, control_id: Some(id.to_string()), kind: HitKind::Slider, drag_axis: Some(DragAxis::Ring), drag_data: None });
        }
    }

    fn render_icon_select<E: Clone>(id: &str, value: &str, _uniform: bool, _classifier_kind: &str, on_change: Option<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
        push_control_border(ctx.draw, bounds, ctx.theme, ctx.theme.border_normal, chrome_item_bg(ctx.theme, false, ctx.input.hovered_id.as_deref() == Some(id)));
        let content_x = bounds.x + ctx.theme.padding_standard;
        if let Some(icons) = ctx.icons {
            if icons.icon_uv(value).is_some() {
                push_icon(ctx.draw, icons, value, content_x, bounds.y + (bounds.h - ICON_TINY) * 0.5, ICON_TINY, ctx.theme.text_element);
            } else {
                draw_text(ctx, value, content_x, bounds.y + (bounds.h + ctx.theme.font_size_body) * 0.5 - 2.0, ctx.theme.font_size_body, ctx.theme.text);
            }
        } else {
            draw_text(ctx, value, content_x, bounds.y + (bounds.h + ctx.theme.font_size_body) * 0.5 - 2.0, ctx.theme.font_size_body, ctx.theme.text);
        }
        ctx.input.register_hit(HitTarget { rect: bounds, event: on_change, control_id: Some(id.to_string()), kind: HitKind::Generic, drag_axis: None, drag_data: None });
    }

    fn measure_tree_sections_width<E>(sections: &[TreeSection<E>], atlas: &mut FontAtlas, theme: &Theme) -> f32 {
        let collapsed = HashMap::new();
        measure_tree_sections_width_state(sections, atlas, theme, &collapsed, 0)
    }

    fn measure_tree_sections_width_state<E>(sections: &[TreeSection<E>], atlas: &mut FontAtlas, theme: &Theme, collapsed: &HashMap<String, bool>, depth: u32) -> f32 {
        let mut max_w = 0.0f32;
        for section in sections {
            let section_key = format!("section.{}", section.id);
            let section_collapsed = collapsed.get(&section_key).copied().unwrap_or(!section.default_open);
            if let Some(label) = &section.label {
                let w = atlas.measure_text(label, theme.font_size_small).0 + tree_gutter_width(0) + TREE_ICON_SIZE + theme.gap_standard * 2.0;
                max_w = max_w.max(w);
            }
            if !section_collapsed {
                for item in &section.items {
                    max_w = max_w.max(measure_tree_item_width(item, atlas, theme, collapsed, depth));
                }
            }
        }
        max_w.max(120.0)
    }

    fn measure_tree_item_width<E>(item: &TreeItem<E>, atlas: &mut FontAtlas, theme: &Theme, collapsed: &HashMap<String, bool>, depth: u32) -> f32 {
        if item.dimmed {
            return 0.0;
        }
        let mut w = tree_gutter_width(depth) + TREE_ICON_SIZE + theme.gap_standard + atlas.measure_text(&item.label, theme.font_size_body).0 + theme.gap_standard;
        if let Some(description) = &item.description {
            w += atlas.measure_text(description, theme.font_size_small).0 + theme.gap_standard;
        }
        for action in &item.actions {
            w += TREE_ICON_SIZE + theme.padding_standard;
            if let Some(label) = &action.label {
                w += atlas.measure_text(label, theme.font_size_small).0 + theme.gap_standard;
            }
        }
        if item.control.is_some() {
            w += 120.0 + theme.gap_standard;
        }
        let key = format!("tree.{}", item.id);
        let item_collapsed = collapsed.get(&key).copied().unwrap_or(!item.default_open);
        if !item_collapsed {
            for child in &item.children {
                w = w.max(measure_tree_item_width(child, atlas, theme, collapsed, depth + 1));
            }
        }
        w
    }

    fn measure_tree_sections<E>(sections: &[TreeSection<E>]) -> f32 {
        let collapsed = HashMap::new();
        measure_tree_sections_state(sections, &collapsed)
    }

    fn measure_tree_sections_state<E>(sections: &[TreeSection<E>], collapsed: &HashMap<String, bool>) -> f32 {
        let mut height = 0.0;
        for section in sections {
            height += TREE_ROW_HEIGHT;
            let section_key = format!("section.{}", section.id);
            let section_collapsed = collapsed.get(&section_key).copied().unwrap_or(!section.default_open);
            if !section_collapsed {
                for item in &section.items {
                    height += measure_tree_item_height(item, collapsed);
                }
                height += TREE_SECTION_GAP;
            }
        }
        height
    }

    fn measure_tree_item_height<E>(item: &TreeItem<E>, collapsed: &HashMap<String, bool>) -> f32 {
        if item.dimmed {
            return 0.0;
        }
        let mut height = TREE_ROW_HEIGHT;
        let key = format!("tree.{}", item.id);
        let item_collapsed = collapsed.get(&key).copied().unwrap_or(!item.default_open);
        if !item_collapsed {
            for child in &item.children {
                height += measure_tree_item_height(child, collapsed);
            }
        }
        height
    }

    fn tree_gutter_width(depth: u32) -> f32 {
        depth as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH
    }

    fn tree_icon_id<E>(item: &TreeItem<E>, expandable: bool) -> Option<&str> {
        item.icon_id.map(IconName::as_str).or(if expandable { Some("folder") } else { None })
    }

    fn tree_row_collapsed(collapsed: &HashMap<String, bool>, key: &str, default_open: bool) -> bool {
        collapsed.get(key).copied().unwrap_or(!default_open)
    }

    fn render_tree<E: Clone>(sections: &[TreeSection<E>], selected_ids: &[String], highlighted_ids: &[String], bounds: Rect, ctx: &mut WidgetContext<'_, E>) {
        let mut y = bounds.y;
        for section in sections {
            let section_key = format!("section.{}", section.id);
            if !ctx.collapsed_sections.contains_key(&section_key) {
                ctx.collapsed_sections.insert(section_key.clone(), !section.default_open);
            }
            let section_collapsed = tree_row_collapsed(ctx.collapsed_sections, &section_key, section.default_open);
            render_tree_section_header(section, bounds, y, section_collapsed, ctx);
            y += TREE_ROW_HEIGHT;
            if !section_collapsed {
                for item in &section.items {
                    y += render_tree_item(item, Rect::new(bounds.x, y, bounds.w, TREE_ROW_HEIGHT), ctx, 0, selected_ids, highlighted_ids, &[]);
                }
                y += TREE_SECTION_GAP;
            }
        }
    }

    fn render_tree_section_header<E: Clone>(section: &TreeSection<E>, bounds: Rect, y: f32, collapsed: bool, ctx: &mut WidgetContext<'_, E>) {
        let row = Rect::new(bounds.x, y, bounds.w, TREE_ROW_HEIGHT);
        let gutter_w = TREE_TOGGLE_WIDTH;
        let gutter = Rect::new(row.x, row.y, gutter_w, row.h);
        let content = Rect::new(row.x + gutter_w, row.y, row.w - gutter_w, row.h);
        let chevron = if collapsed { "chevron-right" } else { "chevron-down" };
        tree_draw_chevron(ctx, chevron, gutter);
        ctx.input.register_hit(HitTarget { rect: gutter, event: None, control_id: Some(format!("section.chevron.{}", section.id)), kind: HitKind::TreeItem, drag_axis: None, drag_data: None });
        if let Some(label) = &section.label {
            let text_color = if collapsed { ctx.theme.text_muted } else { ctx.theme.text_element };
            let label_x = content.x + ctx.theme.gap_standard;
            if let Some(uv) = ctx.icons.and_then(|icons| icons.icon_uv("folder")) {
                draw_icon(ctx, uv, label_x, content.y + (content.h - TREE_ICON_SIZE) * 0.5, TREE_ICON_SIZE, text_color);
            }
            draw_text(ctx, label, label_x + TREE_ICON_SIZE + ctx.theme.gap_standard, content.y + (content.h + ctx.theme.font_size_small) * 0.5 - 1.0, ctx.theme.font_size_small, text_color);
        }
    }

    fn render_tree_item<E: Clone>(item: &TreeItem<E>, bounds: Rect, ctx: &mut WidgetContext<'_, E>, depth: u32, selected_ids: &[String], highlighted_ids: &[String], is_last_at_level: &[bool]) -> f32 {
        if item.dimmed {
            return 0.0;
        }
        let key = format!("tree.{}", item.id);
        if !ctx.collapsed_sections.contains_key(&key) {
            ctx.collapsed_sections.insert(key.clone(), !item.default_open);
        }
        let collapsed = tree_row_collapsed(ctx.collapsed_sections, &key, item.default_open);
        let expandable = !item.children.is_empty();
        let gutter_w = tree_gutter_width(depth);
        let row = Rect::new(bounds.x, bounds.y, bounds.w, TREE_ROW_HEIGHT);
        let gutter = Rect::new(row.x, row.y, gutter_w, row.h);
        let content = Rect::new(row.x + gutter_w, row.y, row.w - gutter_w, row.h);
        let hovered = ctx.input.hovered_id.as_deref().is_some_and(|id| id.strip_prefix("tree.label.").is_some_and(|v| v == item.id));
        let selected = item.selected || selected_ids.iter().any(|id| id == &item.id);
        let highlighted = item.highlighted || highlighted_ids.iter().any(|id| id == &item.id);
        tree_draw_guides(ctx, gutter, depth, is_last_at_level);
        if expandable {
            let chevron = if collapsed { "chevron-right" } else { "chevron-down" };
            let chevron_rect = Rect::new(gutter.x + depth as f32 * TREE_INDENT_PER_LEVEL, gutter.y, TREE_TOGGLE_WIDTH, gutter.h);
            tree_draw_chevron(ctx, chevron, chevron_rect);
            ctx.input.register_hit(HitTarget { rect: chevron_rect, event: None, control_id: Some(format!("tree.chevron.{}", item.id)), kind: HitKind::TreeItem, drag_axis: None, drag_data: None });
        }
        if selected {
            ctx.draw.push_rounded([content.x, content.y, content.w, content.h], ctx.theme.selected, ctx.theme.border_radius);
        } else if highlighted || hovered {
            ctx.draw.push_rounded([content.x, content.y, content.w, content.h], ctx.theme.row_hover, ctx.theme.border_radius);
        }
        let mut label_x = content.x + ctx.theme.gap_standard;
        let icon_id = tree_icon_id(item, expandable);
        let text_color = if selected || highlighted {
            ctx.theme.active_foreground
        } else if hovered {
            ctx.theme.border_emphasized
        } else if item.dimmed {
            ctx.theme.text_muted
        } else {
            ctx.theme.text_element
        };
        if let Some(icon_id) = icon_id {
            if let Some(uv) = ctx.icons.and_then(|icons| icons.icon_uv(icon_id)) {
                draw_icon(ctx, uv, label_x, content.y + (content.h - TREE_ICON_SIZE) * 0.5, TREE_ICON_SIZE, text_color);
                label_x += TREE_ICON_SIZE + ctx.theme.gap_standard;
            }
        }
        draw_text(ctx, &item.label, label_x, content.y + (content.h + ctx.theme.font_size_body) * 0.5 - 2.0, ctx.theme.font_size_body, text_color);
        if let Some(description) = &item.description {
            let label_w = measure_text_width(ctx, &item.label, ctx.theme.font_size_body);
            draw_text(ctx, description, label_x + label_w + ctx.theme.gap_standard, content.y + (content.h + ctx.theme.font_size_small) * 0.5 - 1.0, ctx.theme.font_size_small, ctx.theme.text_muted);
        }
        let mut actions_x = content.x + content.w - ctx.theme.gap_standard;
        for (index, action) in item.actions.iter().enumerate().rev() {
            if action.placement == UiTreeActionPlacement::Menu {
                continue;
            }
            let label_w = action.label.as_ref().map_or(0.0, |label| measure_text_width(ctx, label, ctx.theme.font_size_small) + ctx.theme.gap_standard);
            let action_w = TREE_ICON_SIZE + ctx.theme.padding_standard + label_w;
            actions_x -= action_w;
            let action_rect = Rect::new(actions_x, content.y + (content.h - TREE_ICON_SIZE) * 0.5 - 2.0, action_w, TREE_ICON_SIZE + 4.0);
            if let Some(uv) = ctx.icons.and_then(|icons| icons.icon_uv(action.icon_id.as_str())) {
                let action_color = if hovered { ctx.theme.border_emphasized } else { ctx.theme.text_element };
                draw_icon(ctx, uv, action_rect.x + 2.0, action_rect.y + 2.0, TREE_ICON_SIZE, action_color);
            }
            if hovered {
                if let Some(label) = &action.label {
                    draw_text(ctx, label, action_rect.x + TREE_ICON_SIZE + 4.0, action_rect.y + (TREE_ICON_SIZE + ctx.theme.font_size_small) * 0.5, ctx.theme.font_size_small, ctx.theme.text_muted);
                }
            }
            ctx.input.register_hit(HitTarget { rect: action_rect, event: Some(action.event.clone()), control_id: Some(format!("tree.action.{}.{}", item.id, index)), kind: HitKind::Button, drag_axis: None, drag_data: None });
        }
        if let Some(hover) = &item.hover_event {
            if let Some(maps) = ctx.interaction_maps.as_deref_mut() {
                maps.tree_hover_commands.insert(item.id.clone(), hover.clone());
            }
        }
        if let Some(unhover) = &item.unhover_event {
            if let Some(maps) = ctx.interaction_maps.as_deref_mut() {
                maps.tree_unhover_commands.insert(item.id.clone(), unhover.clone());
            }
        }
        if let Some(control) = &item.control {
            let control_w = 120.0;
            let control_rect = Rect::new(content.x + content.w - control_w - ctx.theme.gap_standard, content.y + (content.h - ctx.theme.control_height) * 0.5, control_w, ctx.theme.control_height);
            render_widget(control, control_rect, ctx);
        }
        let label_rect = Rect::new(label_x, content.y, content.x + content.w - label_x - ctx.theme.gap_standard, content.h);
        ctx.input.register_hit(HitTarget {
            rect: label_rect,
            event: item.event.clone(),
            control_id: Some(format!("tree.label.{}", item.id)),
            kind: HitKind::TreeItem,
            drag_axis: if item.draggable { Some(DragAxis::Both) } else { None },
            drag_data: if item.draggable && !item.drag_data.is_empty() { Some(item.drag_data.clone()) } else { None },
        });
        let mut height = TREE_ROW_HEIGHT;
        if !collapsed {
            for (index, child) in item.children.iter().enumerate() {
                let mut child_is_last = is_last_at_level.to_vec();
                child_is_last.push(index + 1 == item.children.len());
                let child_bounds = Rect::new(bounds.x, bounds.y + height, bounds.w, TREE_ROW_HEIGHT);
                height += render_tree_item(child, child_bounds, ctx, depth + 1, selected_ids, highlighted_ids, &child_is_last);
            }
        }
        height
    }

    fn tree_draw_chevron<E>(ctx: &mut WidgetContext<'_, E>, icon_id: &str, rect: Rect) {
        if let Some(uv) = ctx.icons.and_then(|icons| icons.icon_uv(icon_id)) {
            draw_icon(ctx, uv, rect.x + (rect.w - TREE_ICON_SIZE) * 0.5, rect.y + (rect.h - TREE_ICON_SIZE) * 0.5, TREE_ICON_SIZE, ctx.theme.text_muted);
        }
    }

    fn tree_draw_guides<E>(ctx: &mut WidgetContext<'_, E>, gutter: Rect, depth: u32, is_last_at_level: &[bool]) {
        let hair = ctx.theme.stroke_hairline.max(1.0);
        let guide_color = ctx.theme.border_normal;
        for level in 0..depth {
            if is_last_at_level.get(level as usize).copied().unwrap_or(false) {
                continue;
            }
            let x = gutter.x + level as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH * 0.5;
            ctx.draw.push_solid([x, gutter.y, hair, gutter.h], guide_color);
        }
        if depth > 0 {
            let x = gutter.x + (depth - 1) as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH * 0.5;
            let mid_y = gutter.y + gutter.h * 0.5;
            ctx.draw.push_solid([x, gutter.y, hair, mid_y - gutter.y], guide_color);
            ctx.draw.push_solid([x, mid_y, TREE_INDENT_PER_LEVEL * 0.5, hair], guide_color);
        }
    }

    pub fn render_scroll_region<E: Clone, F: FnOnce(Rect, &mut WidgetContext<'_, E>)>(scroll_id: &str, bounds: Rect, content_height: f32, ctx: &mut WidgetContext<'_, E>, render_content: F) {
        let max_scroll = (content_height - bounds.h).max(0.0);
        let offset = ctx.scroll_offsets.entry(scroll_id.to_string()).or_insert(0.0);
        *offset = offset.clamp(0.0, max_scroll);
        let scroll = *offset;
        ctx.input.register_hit(HitTarget { rect: bounds, event: None, control_id: Some(scroll_id.to_string()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
        ctx.draw.push_scissor(bounds);
        let content_bounds = Rect::new(bounds.x, bounds.y - scroll, bounds.w, content_height);
        render_content(content_bounds, ctx);
        ctx.draw.pop_scissor();
    }

    pub fn draw_icon<E>(ctx: &mut WidgetContext<'_, E>, uv: [f32; 4], x: f32, y: f32, size: f32, color: Rgba) {
        ctx.draw.push_textured([x, y, size, size], uv, color);
    }

    fn measure_text_width<E>(ctx: &mut WidgetContext<'_, E>, text: &str, size: f32) -> f32 {
        let (w, _) = ctx.atlas.measure_text(text, size);
        w
    }

    pub fn draw_text_wrapped<E>(ctx: &mut WidgetContext<'_, E>, text: &str, x: f32, y: f32, max_width: f32, size: f32, color: Rgba) -> f32 {
        let lines = wrap_text(ctx.atlas, text, max_width, size);
        let line_h = size * 1.35;
        for (index, line) in lines.iter().enumerate() {
            draw_text(ctx, line, x, y + line_h * index as f32 + size, size, color);
        }
        lines.len() as f32 * line_h
    }

    pub fn wrap_text(atlas: &mut FontAtlas, text: &str, max_width: f32, size: f32) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current = String::new();
        for word in text.split_whitespace() {
            let trial = if current.is_empty() { word.to_string() } else { format!("{current} {word}") };
            let (w, _) = atlas.measure_text(&trial, size);
            if w > max_width && !current.is_empty() {
                lines.push(current);
                current = word.to_string();
            } else {
                current = trial;
            }
        }
        if !current.is_empty() {
            lines.push(current);
        }
        if lines.is_empty() {
            lines.push(String::new());
        }
        lines
    }

    pub fn draw_text_on(draw: &mut DrawList, atlas: &mut FontAtlas, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
        let atlas_w = atlas.width as f32;
        let atlas_h = atlas.height as f32;
        let mut cursor_x = x;
        for ch in text.chars() {
            let glyph = atlas.ensure_glyph(ch, size);
            let gw = glyph.width as f32;
            let gh = glyph.height as f32;
            let gx = cursor_x + glyph.bearing_x;
            let gy = y - gh - glyph.bearing_y;
            let uv_rect = [glyph.atlas_x as f32 / atlas_w, glyph.atlas_y as f32 / atlas_h, (glyph.atlas_x + glyph.width) as f32 / atlas_w, (glyph.atlas_y + glyph.height) as f32 / atlas_h];
            draw.push_glyph([gx, gy, gw.max(1.0), gh.max(1.0)], color, uv_rect);
            cursor_x += glyph.advance;
        }
    }

    pub fn draw_text_overlay_on(draw: &mut DrawList, atlas: &mut FontAtlas, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
        let atlas_w = atlas.width as f32;
        let atlas_h = atlas.height as f32;
        let mut cursor_x = x;
        for ch in text.chars() {
            let glyph = atlas.ensure_glyph(ch, size);
            let gw = glyph.width as f32;
            let gh = glyph.height as f32;
            let gx = cursor_x + glyph.bearing_x;
            let gy = y - gh - glyph.bearing_y;
            let uv_rect = [glyph.atlas_x as f32 / atlas_w, glyph.atlas_y as f32 / atlas_h, (glyph.atlas_x + glyph.width) as f32 / atlas_w, (glyph.atlas_y + glyph.height) as f32 / atlas_h];
            draw.push_glyph_overlay([gx, gy, gw.max(1.0), gh.max(1.0)], color, uv_rect);
            cursor_x += glyph.advance;
        }
    }

    pub fn draw_text<E>(ctx: &mut WidgetContext<'_, E>, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
        let atlas_w = ctx.atlas.width as f32;
        let atlas_h = ctx.atlas.height as f32;
        let mut cursor_x = x;
        for ch in text.chars() {
            let glyph = ctx.atlas.ensure_glyph(ch, size);
            let gw = glyph.width as f32;
            let gh = glyph.height as f32;
            let gx = cursor_x + glyph.bearing_x;
            let gy = y - gh - glyph.bearing_y;
            let uv_rect = [glyph.atlas_x as f32 / atlas_w, glyph.atlas_y as f32 / atlas_h, (glyph.atlas_x + glyph.width) as f32 / atlas_w, (glyph.atlas_y + glyph.height) as f32 / atlas_h];
            ctx.draw.push_glyph([gx, gy, gw.max(1.0), gh.max(1.0)], color, uv_rect);
            cursor_x += glyph.advance;
        }
    }

    pub fn draw_text_overlay<E>(ctx: &mut WidgetContext<'_, E>, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
        draw_text_overlay_on(ctx.draw, ctx.atlas, text, x, y, size, color);
    }

    //#region 🔖️Gizmo
    /** 🧭️ Screen-space XYZ orientation gizmo (wgpu parity with React `WorldOrbitViewGizmo`) — placement,
    hit-testing, and paint. Relocated verbatim from `♾️infinite/🌍️world` (see
    `.🦑️repo/🎫️tickets/26/08/05/FRAMEWORK-BUILDER-PASSTHROUGHS-APP-COMMANDS-MACRO-WIDGET-EXTRACTION`) so any
    plugin's world-3d window can reuse it, not only `♾️infinite`'s own. `World3dState`-specific hover-state
    plumbing (`update_world_orbit_view_gizmo_hover`, which owns `&mut World3dState`) stays in `♾️infinite/🌍️world`
    — app-specific config plumbing, not paint logic — and now calls through to `orbit_view_gizmo_placement`/
    `orbit_view_gizmo_tips`/`orbit_view_gizmo_hit_test` here. */
    pub mod gizmo {
        use crate::widgets::WidgetContext;
        use crate::{Camera3d, Rect, Rgba, Vec3};

        /// 🧭️ Permanent X/Y/Z paints — primary / secondary / tertiary (semio tokens), not muted chrome.
        pub fn spatial_axis_rgba(axis: u8, alpha: f32) -> Rgba {
            match axis {
                0 => Rgba::new(1.0, 0.204, 0.310, alpha),   // primary #ff344f
                1 => Rgba::new(0.204, 0.820, 0.749, alpha), // secondary #34d1bf
                _ => Rgba::new(0.980, 0.584, 0.0, alpha),   // tertiary #fa9500
            }
        }

        /// 🧭️ Mirrors `resolveSceneGizmoViewportPlacement` — bottom-right corner inset matching pane `--spacing-single` chrome.
        pub fn orbit_view_gizmo_placement(viewport: Rect) -> (f32, f32) {
            let chrome_inset = 4.0_f32;
            let gizmo_half_extent = 28.0_f32;
            let preferred = chrome_inset + gizmo_half_extent;
            let max_fit = (viewport.w.min(viewport.h) / 3.0).floor().max(22.0);
            let margin = preferred.min(max_fit);
            (margin, margin)
        }

        /// 🧭️ Screen-space tip used for orbit-view gizmo hover hit-testing and paint.
        pub struct OrbitViewGizmoTip {
            pub screen_x: f32,
            pub screen_y: f32,
            pub depth: f32,
            pub pick_radius: f32,
            pub color: Rgba,
            pub is_corner: bool,
            pub prominent: bool,
        }

        pub fn orbit_view_gizmo_tips(camera: &Camera3d, viewport: Rect) -> Vec<OrbitViewGizmoTip> {
            let (margin_x, margin_y) = orbit_view_gizmo_placement(viewport);
            let origin_x = viewport.x + viewport.w - margin_x;
            let origin_y = viewport.y + viewport.h - margin_y;
            let axis_len = (viewport.w.min(viewport.h) * 0.04).clamp(14.0, 24.0);
            let forward = camera.position.sub(camera.target);
            let forward_len = forward.length();
            if forward_len < 1e-5 {
                return Vec::new();
            }
            let forward = forward.scale(1.0 / forward_len);
            let right = forward.cross(camera.up);
            let right_len = right.length();
            if right_len < 1e-5 {
                return Vec::new();
            }
            let right = right.scale(1.0 / right_len);
            let up = right.cross(forward).normalize();
            let neutral = Rgba::new(0.62, 0.62, 0.66, 0.9);
            let axes = [
                (Vec3::new(1.0, 0.0, 0.0), spatial_axis_rgba(0, 1.0), true),
                (Vec3::new(-1.0, 0.0, 0.0), spatial_axis_rgba(0, 0.75), false),
                (Vec3::new(0.0, 1.0, 0.0), spatial_axis_rgba(1, 1.0), true),
                (Vec3::new(0.0, -1.0, 0.0), spatial_axis_rgba(1, 0.75), false),
                (Vec3::new(0.0, 0.0, 1.0), spatial_axis_rgba(2, 1.0), true),
                (Vec3::new(0.0, 0.0, -1.0), spatial_axis_rgba(2, 0.75), false),
            ];
            let corners = [
                (Vec3::new(0.72, 0.72, 0.72), true),
                (Vec3::new(-0.72, 0.72, 0.72), true),
                (Vec3::new(0.72, -0.72, 0.72), true),
                (Vec3::new(-0.72, -0.72, 0.72), true),
                (Vec3::new(0.72, 0.72, -0.72), false),
                (Vec3::new(-0.72, 0.72, -0.72), false),
                (Vec3::new(0.72, -0.72, -0.72), false),
                (Vec3::new(-0.72, -0.72, -0.72), false),
            ];
            let mut tips: Vec<OrbitViewGizmoTip> = axes
                .into_iter()
                .map(|(axis, color, prominent)| {
                    let sx = axis.dot(right);
                    let sy = -axis.dot(up);
                    let depth = axis.dot(forward);
                    let tip_x = origin_x + sx * axis_len;
                    let tip_y = origin_y + sy * axis_len;
                    let pick_radius = if prominent { 10.0 } else { 7.0 };
                    OrbitViewGizmoTip { screen_x: tip_x, screen_y: tip_y, depth, pick_radius, color, is_corner: false, prominent }
                })
                .chain(corners.into_iter().map(|(axis, prominent)| {
                    let sx = axis.dot(right);
                    let sy = -axis.dot(up);
                    let depth = axis.dot(forward);
                    let tip_x = origin_x + sx * axis_len;
                    let tip_y = origin_y + sy * axis_len;
                    let pick_radius = if prominent { 10.0 } else { 7.0 };
                    OrbitViewGizmoTip { screen_x: tip_x, screen_y: tip_y, depth, pick_radius, color: neutral, is_corner: true, prominent }
                }))
                .collect();
            tips.push(OrbitViewGizmoTip { screen_x: origin_x, screen_y: origin_y, depth: 0.0, pick_radius: 9.0, color: neutral, is_corner: false, prominent: true });
            tips
        }

        pub fn orbit_view_gizmo_hit_test(x: f32, y: f32, tips: &[OrbitViewGizmoTip]) -> Option<usize> {
            tips.iter()
                .enumerate()
                .filter_map(|(index, tip)| {
                    let distance = ((x - tip.screen_x).powi(2) + (y - tip.screen_y).powi(2)).sqrt();
                    if distance <= tip.pick_radius + 3.0 {
                        Some((index, distance))
                    } else {
                        None
                    }
                })
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(index, _)| index)
        }

        /// 🧭️ Screen-space XYZ orientation gizmo in the lower-right of every world-3d window (wgpu parity with React `WorldOrbitViewGizmo`).
        pub fn paint_orbit_view_gizmo<E>(ctx: &mut WidgetContext<'_, E>, camera: &Camera3d, viewport: Rect, hovered_tip: Option<usize>) {
            let (margin_x, margin_y) = orbit_view_gizmo_placement(viewport);
            let origin_x = viewport.x + viewport.w - margin_x;
            let origin_y = viewport.y + viewport.h - margin_y;
            let tips = orbit_view_gizmo_tips(camera, viewport);
            let mut ordered: Vec<(f32, usize)> = tips.iter().enumerate().map(|(index, tip)| (tip.depth, index)).collect();
            ordered.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            let has_hover = hovered_tip.is_some();
            for (_, index) in ordered {
                let tip = &tips[index];
                let hovered = hovered_tip == Some(index);
                let depth_fade = if tip.depth > 0.05 { 0.45 } else { 1.0 };
                let hover_fade = if has_hover && !hovered { 0.42 } else { 1.0 };
                let alpha = (tip.color.a * depth_fade * hover_fade).min(1.0);
                let stroke = Rgba::new(tip.color.r, tip.color.g, tip.color.b, if hovered { tip.color.a.min(1.0) } else { alpha });
                ctx.draw.push_line_overlay(origin_x, origin_y, tip.screen_x, tip.screen_y, stroke, if tip.is_corner { 1.5 } else { 2.0 });
                let r = if tip.prominent {
                    if hovered {
                        3.6
                    } else {
                        3.0
                    }
                } else if hovered {
                    2.4
                } else {
                    2.0
                };
                ctx.draw.push_solid_overlay([tip.screen_x - r, tip.screen_y - r, tip.screen_x + r, tip.screen_y + r], stroke);
            }
        }
    }
    //#endregion 🔖️Gizmo
    // #endregion widgets
}

#[cfg(feature = "engine")]
pub mod host {
    // #region host
    //! 🪟️ winit window event bridge into pointer callbacks.

    use crate::input::{KeyAction, PointerCallbacks, PointerModifiers};
    use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent};
    use winit::keyboard::{Key, NamedKey};

    pub fn pointer_coords(_window: &winit::window::Window, position: winit::dpi::PhysicalPosition<f64>) -> (f32, f32) {
        (position.x as f32, position.y as f32)
    }

    pub fn modifiers_from_winit(modifiers: winit::keyboard::ModifiersState) -> PointerModifiers {
        PointerModifiers { shift: modifiers.shift_key(), ctrl: modifiers.control_key(), alt: modifiers.alt_key(), meta: modifiers.super_key() }
    }

    #[derive(Default)]
    pub struct WindowInputState {
        pub pointer_x: f32,
        pub pointer_y: f32,
        pub pointer_down: bool,
        pub pointer_button: i16,
        pub modifiers: PointerModifiers,
    }

    pub fn dispatch_window_event(window: &winit::window::Window, event: &WindowEvent, input: &mut WindowInputState, callbacks: &PointerCallbacks) -> bool {
        match event {
            WindowEvent::ModifiersChanged(modifiers) => {
                input.modifiers = modifiers_from_winit(modifiers.state());
                true
            }
            WindowEvent::CursorMoved { position, .. } => {
                let (x, y) = pointer_coords(window, *position);
                input.pointer_x = x;
                input.pointer_y = y;
                (callbacks.on_move)(x, y, input.pointer_down, input.pointer_button, input.modifiers.clone());
                true
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let down = *state == ElementState::Pressed;
                let btn = mouse_button_to_i16(*button);
                if down {
                    input.pointer_down = true;
                    input.pointer_button = btn;
                } else if input.pointer_down {
                    input.pointer_down = false;
                }
                (callbacks.on_button)(input.pointer_x, input.pointer_y, down, btn, input.modifiers.clone());
                true
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let delta_y = match delta {
                    MouseScrollDelta::LineDelta(_, y) => *y * 40.0,
                    MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
                };
                (callbacks.on_wheel)(delta_y, input.pointer_x, input.pointer_y, input.modifiers.clone());
                true
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let Key::Named(NamedKey::Space) = &event.logical_key {
                    (callbacks.on_key)(KeyAction::Space(event.state == ElementState::Pressed), input.modifiers.clone());
                    return true;
                }
                if event.state != ElementState::Pressed {
                    return true;
                }
                let action = key_action_from_event(event);
                if let Some(action) = action {
                    (callbacks.on_key)(action, input.modifiers.clone());
                }
                true
            }
            _ => false,
        }
    }

    fn mouse_button_to_i16(button: MouseButton) -> i16 {
        match button {
            MouseButton::Left => 0,
            MouseButton::Right => 2,
            MouseButton::Middle => 1,
            MouseButton::Back => 3,
            MouseButton::Forward => 4,
            MouseButton::Other(id) => id as i16,
        }
    }

    fn key_action_from_event(event: &KeyEvent) -> Option<KeyAction> {
        match &event.logical_key {
            Key::Named(NamedKey::Backspace) => Some(KeyAction::Backspace),
            Key::Named(NamedKey::Delete) => Some(KeyAction::Delete),
            Key::Named(NamedKey::Enter) => Some(KeyAction::Enter),
            Key::Named(NamedKey::Escape) => Some(KeyAction::Escape),
            Key::Named(NamedKey::ArrowLeft) => Some(KeyAction::ArrowLeft),
            Key::Named(NamedKey::ArrowRight) => Some(KeyAction::ArrowRight),
            Key::Named(NamedKey::ArrowUp) => Some(KeyAction::ArrowUp),
            Key::Named(NamedKey::ArrowDown) => Some(KeyAction::ArrowDown),
            Key::Named(NamedKey::Tab) => Some(KeyAction::Tab),
            Key::Character(ch) if ch.chars().count() == 1 => Some(KeyAction::Char(ch.to_string())),
            _ => None,
        }
    }

    //#region 🔖️ClipboardHost
    /** 📋️ OS clipboard write for `events::UiCommand::ClipboardCopy`/`ClipboardCut` — a caller (e.g.
     * `framework/renderer/wgpu`'s `interpreter::apply_ui_commands`) hands over the already-computed
     * copied/cut `text` and this fn is the ONLY thing in either engine that touches a real clipboard
     * backend, matching this crate's "wrap external libraries behind an interface, never leak the
     * library's own types past it" convention. Native wraps `arboard::Clipboard::set_text` (silently
     * no-ops without a display/clipboard, e.g. headless CI — `Clipboard::new()`'s `Err` is swallowed
     * rather than propagated, since there is no sensible way for a UI copy gesture to surface a clipboard
     * backend failure back through this call chain). Wasm fires the async Clipboard API's `writeText`
     * without awaiting it: the underlying `Promise` already starts executing the instant it's created, so
     * not awaiting it just means this fn doesn't itself learn whether the write ultimately succeeded —
     * exactly like a browser's own Ctrl+C, which never blocks the UI thread on the OS clipboard settling. */
    #[cfg(not(target_arch = "wasm32"))]
    pub fn clipboard_write_text(text: &str) {
        if let Ok(mut clipboard) = arboard::Clipboard::new() {
            let _ = clipboard.set_text(text.to_string());
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn clipboard_write_text(text: &str) {
        if let Some(window) = web_sys::window() {
            let _ = window.navigator().clipboard().write_text(text);
        }
    }

    /** 📋️ Blocking OS clipboard read for `events::UiCommand::ClipboardPasteRequested` — native only:
     * `arboard::Clipboard::get_text` is itself synchronous, so a caller can read the OS clipboard and
     * feed the result straight back into `engine::Ui::dispatch_event` as a `events::UiEvent::Paste`
     * within the very same call. `None` on any failure (no clipboard backend, or the clipboard doesn't
     * currently hold text) — a caller treats that identically to "user pasted nothing". */
    #[cfg(not(target_arch = "wasm32"))]
    pub fn clipboard_read_text() -> Option<String> {
        arboard::Clipboard::new().ok()?.get_text().ok()
    }

    /** 📋️ The wasm mirror of `clipboard_read_text` above — `async` because the browser's Clipboard API
     * is Promise-based with no synchronous escape hatch; a caller drives this from a
     * `wasm_bindgen_futures::spawn_local` task (see `report-w3-clipboard-dnd.md`), since the OS
     * clipboard permission prompt/read can't resolve within one synchronous per-frame call. */
    #[cfg(target_arch = "wasm32")]
    pub async fn clipboard_read_text() -> Option<String> {
        let promise = web_sys::window()?.navigator().clipboard().read_text();
        wasm_bindgen_futures::JsFuture::from(promise).await.ok()?.as_string()
    }
    //#endregion 🔖️ClipboardHost
    // #endregion host
}

// #region re-exports
// 🧩️ Always available: declarative component types + engine-agnostic primitives (default features).
pub use component::layout::{
    build_shell_context_menu_specs, collect_window_kind_ids_from_layout, create_default_layout, create_named_layout, create_stack_layout, create_tab_stack_layout, create_window_layout, default_viewport_engagement, even_window_layout,
    framework_panel_tab_label, merge_named_layouts, organize_context_menu, partition_window_measures, ribbon_parent_label, ActionDescriptor, MeasureSelectItem, NamedLayout, ShellMenuAction, StyleSpec, WindowEngagement, WindowEngagementControl,
    WindowEngagementInput, WindowEngagementOption, WindowEngagementPossible, WindowEngagementSlot, WindowEngagementStatus, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode,
    WindowMeasure, WindowOptions, FRAMEWORK_HISTORY_BODY_KEY, FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ICON_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_HISTORY_ICON_ID, FRAMEWORK_PANEL_TAB_HISTORY_ID, FRAMEWORK_PANEL_TAB_HISTORY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID, FRAMEWORK_PANEL_TAB_PARAMETERS_ID, FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL, RIBBON_PARENT_CATEGORIES,
};
pub use component::ui::*;
pub use component::utilities::{utility_button, utility_collection, utility_separator, utility_toggle, UtilityCategory, UtilityNode};
pub use geometry::Rect;
pub use theme::{GlassStyle, Level, Rgba, Theme};

// 🖥️ Retained-mode engine surface (feature = "engine" only).
#[cfg(feature = "engine")]
pub use arena::{Arena, NodeId};
#[cfg(all(feature = "engine", target_arch = "wasm32"))]
pub use cursor::apply_canvas_cursor;
#[cfg(feature = "engine")]
pub use cursor::{apply_window_cursor, resolve_semio_cursor, CursorDragState, SemioCursor};
#[cfg(feature = "engine")]
pub use draw::{ear_clip_polygon, mesh_content_version, paint_selection_marquee, DrawList, IconAtlas, MeshGpuStore, RasterTextureStore};
#[cfg(feature = "engine")]
pub use tree::{EditState, LayoutBucket, Node, NodeFlags, NodeKey, PaintBucket, UiTree, WidgetSpec, WidgetState};
// 🪟️🫳️🖱️ W2 wiring: `w1d-events-overlay`'s overlay/drag-drop/scroll types, previously reachable only
// via `ui_wgpu::events::*` (the module itself is `pub`, just not curated into this flattened surface)
// — `EventRouter` itself stays `pub(crate)` (an `engine::Ui` implementation detail; drive it via
// `Ui::dispatch_event`), but the data these `UiCommand`s/the host's own drag-ghost rendering need are
// now part of the crate's curated public API like every other `events` type already was.
#[cfg(feature = "engine")]
pub use events::{resolve_overlay_placement, CaptureKind, DismissPolicy, DragGhost, DragPayload, DragSession, EventModifiers, ImeEvent, OpenOverlay, OverlayAnchor, OverlayKind, OverlayPlacement, PointerButton, ScrollAxis, UiCommand, UiEvent};
#[cfg(feature = "engine")]
pub use scene_slots::{SceneHost, SceneSlot, SlotContent};
#[cfg(feature = "engine")]
pub use shell::{Shell, ShellEvent};
// 🧵️ W2 wiring: the retained-mode façade itself (`engine::Ui` — `apply_tree`/`frame`/
// `dispatch_event`/`needs_frame`/`drain_commands`) was never re-exported at all before this pass;
// this is the actual public entry point a host drives per tick, per `report-w0-engine-facade.md`'s
// own closing wiring request.
#[cfg(feature = "engine")]
pub use chrome::{chrome_item_bg, chrome_item_text, item_bg, item_text, measure_action_item, push_chrome_border, push_chrome_group_border, push_control_border, push_icon, push_window_cap_border, ICON_TINY};
#[cfg(feature = "engine")]
pub use engine::Ui;
#[cfg(feature = "engine")]
pub use gpu::schedule_frame;
#[cfg(feature = "engine")]
pub use gpu::GpuContext;
#[cfg(feature = "engine")]
pub use host::{clipboard_read_text, clipboard_write_text, dispatch_window_event, modifiers_from_winit, pointer_coords, WindowInputState};
#[cfg(feature = "engine")]
pub use input::{DragAxis, DragState, HitKind, HitTarget, InputState, KeyAction, PointerCallbacks, PointerModifiers, TreeDragState, TreeDropPosition};
#[cfg(feature = "engine")]
pub use kernel_3d_scene::{
    aabb_intersects_frustum, axis_rotate_angle, frustum_planes, gumball_axis_drag_plane_normal, gumball_extent, gumball_eye, gumball_project_ray_onto_axis, marquee_is_crossing_from_path, point_in_polygon, project_point, quat_from_basis,
    ray_aabb_slab, ray_pick_instance, ray_plane_point, ray_segment_distance, rect_contains, rotate_vector, screen_select_instances, transform_aabb, vec3_from_f64, Camera3d, Instance3d, LineDraw3d, LineVertex3d, Mat4, Mesh3d, OrbitController,
    SceneDraw3d, ScenePass3d, TexturedDraw3d, TexturedInstance3d, Vec3,
};
#[cfg(feature = "engine")]
pub use layout::{gap_for_token, layout_horizontal, layout_vertical, padding_for_token};
#[cfg(feature = "engine")]
pub use text::{fetch_font_bytes, FontAtlas};
#[cfg(feature = "engine")]
pub use widgets::{
    draw_icon, draw_text, draw_text_overlay, draw_text_wrapped, measure_widget, render_scroll_region, render_widget, wrap_text, ControlNode, InputMeta, KeyValueEntry, RingMeta, SelectItem, SliderMeta, StepperMeta, TreeItem, TreeItemAction,
    TreeSection, WidgetContext, WidgetInteractionMaps, WidgetNode,
};
// #endregion re-exports
