//! 📜️ Renderer-independent UI declarations, window layouts, actions and engagement contracts.

#[path = "🧭️layout-contract/🦀️.rs"]
pub mod layout;

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
    /// 🛠️ A resolved utility ready to be laid out into the utility bar. `framework_core` maps its
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

    /// 🧬️ Derives the utility bar `UtilityNode` tree from resolved utilities and the host-owned active utility id.
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
    //! `🎠️kernel/🧭️app-router/🟦️.ts` — same boundary this file already draws around `PluginCatalog`-style
    //! product data. A concrete `AppRouter`/`ConfigStore` never appears here: callers (the renderer
    //! product) resolve real entries and hand this module only already-resolved data to render.

    use super::layout::{ContextMenuItemSpec, ShellMenuAction};
    use super::utilities::{UtilityCategory, UtilityNode};
    use semio_framework_value::DslValue;
    use semio_framework_value::{FromValue, ToValue};
    use serde::{Deserialize, Serialize};

    //#region 🔖️ChromeRole
    /// 🎭️✏️ Wire-compatible with Rust `semio_framework::AppRole`/TS `AppRole` — exactly `"viewer"`/
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

    /// 🔓️ Context-menu/palette entry — contract freeze §5: en `"Open with…"` / de `"Öffnen mit…"`.
    pub fn open_with_label_text(is_de: bool) -> &'static str {
        if is_de {
            "Öffnen mit…"
        } else {
            "Open with…"
        }
    }

    /// ⭐️ "Set as default" toggle — contract freeze §5: en `"Set as default"` / de `"Als Standard
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

    /// ❄️ Contract freeze §5's frozen palette command ids — dispatched by an "Open with…" leaf.
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

    /// 🔒️ Contract freeze §5: "...and disables undo/redo" — forces `disabled: Some(true)` on every
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
