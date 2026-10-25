//! 🌲️ Field, section and tree nodes: the field and section nodes, the tree action, window, item, section and tree nodes and the tree presence stamping. See [`super`].

use super::*;

// #region ui
//#region 🔖Primitives
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiFieldNode {
    pub id: String,
    pub label: Label,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub child: Box<UiNode>,
    #[serde(default, skip_serializing_if = "UiPresence::is_default")]
    #[value(default, skip_serializing_if = "UiPresence::is_default")]
    pub presence: UiPresence,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub menu: Option<UiMenuRef>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiSectionNode {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none", alias = "title")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    #[serde(default, skip_serializing_if = "UiPresence::is_default")]
    #[value(default, skip_serializing_if = "UiPresence::is_default")]
    pub presence: UiPresence,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub menu: Option<UiMenuRef>,
    pub children: Vec<UiNode>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum UiTreeActionPlacement {
    #[default]
    Row,
    Menu,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiTreeItemAction {
    pub icon_id: IconName,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    pub action: ActionDescriptor,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<UiTreeActionPlacement>,
    /// 🚫️ Painted and announced disabled; its gesture dispatches nothing (the contract refuses the verb typed).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    #[value(default, skip_serializing_if = "std::ops::Not::not")]
    pub disabled: bool,
    /// 💬️ Why a disabled action cannot run, producer-localized — the contract's `RowAction.reason`, which every renderer
    /// announces as the disabled action's description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<Label>,
}

impl UiTreeItemAction {
    /** 📍️ Row actions paint on the tree header; menu actions belong in the row context menu. */
    pub fn placement(&self) -> UiTreeActionPlacement {
        self.placement.clone().unwrap_or_default()
    }
}

/// 🪟️ The materialised slice of a logically `total`-long tree child list: the node's `items` are
/// the entries `[offset, offset + items.len())` of that list. `total > 0` with no materialised
/// items means expandable-but-not-yet-loaded, never "empty" — a renderer pitches the
/// unmaterialised rows as empty spacer bands so the scrollbar spans the whole document. The
/// legacy-node mirror of `ui_contract::TreeWindow`; the two are bridged in
/// `🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs`. The retained layout pitches the unmaterialised rows as spacers
/// (`layout::tree_window_spacer_px`), `Ui::tree_window_measures` reads every windowed container against its scroll
/// viewport, and the one request rule (`🧱️elements/🌳️Tree/🪟️window/🦀️.rs`, React's `treeWindowServedRequestsV1`) asks
/// back the rows the viewport needs — the same `ViewModel.tree_windows` React's host reports. There is deliberately no
/// `+N` continuation-row fallback.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum UiTreeWindowRowExtent {
    #[default]
    Standard,
    CompactText,
    CompactSmallControl,
    CompactControl,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiTreeWindow {
    pub total: u32,
    pub offset: u32,
    pub row_extent: UiTreeWindowRowExtent,
}

impl UiTreeWindow {
    /// ⬆️ Rows of empty pitch a renderer paints BEFORE the materialised children.
    pub fn leading_rows(&self) -> u32 {
        self.offset.min(self.total)
    }

    /// ⬇️ Rows of empty pitch a renderer paints AFTER `materialised` children.
    pub fn trailing_rows(&self, materialised: usize) -> u32 {
        self.total.saturating_sub(self.leading_rows()).saturating_sub(u32::try_from(materialised).unwrap_or(u32::MAX))
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiTreeItemNode {
    pub id: String,
    pub label: Label,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "icon")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<IconName>,
    #[serde(default, skip_serializing_if = "UiPresence::is_default")]
    #[value(default, skip_serializing_if = "UiPresence::is_default")]
    pub presence: UiPresence,
    #[serde(skip_serializing_if = "Option::is_none", alias = "expanded")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub action: Option<ActionDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<UiTreeItemAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub draggable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub drag_data: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<UiTreeItemNode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub control: Option<UiControlNode>,
    /// 🧱️ The lines this row's content fills in its value column when that content is more than one inline
    /// `control` — a recipe group (colour, vector, reference list), text, progress or several nodes — stacked one
    /// line per leaf and label (`reconcile::tree_content_lines`); the content's own document nodes paint there.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub content_lines: Option<u16>,
    /// 🎛️ The explicitly related horizontal Toolbar and its real Button nodes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub inline_toolbar: Option<UiStackNode>,
    /// 🎞️ One semantic engine Surface placed below the row chrome.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<UiComponentSceneNode>,
    /// 👁️ Domain "eye toggle" flag: the row stays visible, dimmed, and clickable (to un-hide) —
    /// this is NOT `presence.state == Hidden`, which means not rendered at all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub dimmed: Option<bool>,
    /// 🖱️ Row-level context-menu address — most rows share one `menu.id` across a tree with the row
    /// id carried in `args` (e.g. `{"id": row.id}`), rather than minting a unique menu id per row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub menu: Option<UiMenuRef>,
    /// 🍕️ The materialised slice of this row's logical child list — see [`UiTreeWindow`]. A row
    /// with `window.total > 0` is expandable even while `items` is empty or absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<UiTreeWindow>,
    /// 🎯️ The interaction granularity this row picks when the owning tree carries the domain
    /// binding (`UiTreeNode::interaction_domain`), keyed by this row's own `id`. Carried here so a
    /// later wgpu pick path can synthesise the same domain intent React's interpreter already
    /// does; nothing in the wgpu target reads it yet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    /// 🚦️ The semantic tone this row's label and icon paint (`Theme::tone_ink`) — a history row's outcome, a refused
    /// input: one of the four outcome roles, never the only cue of the state it colours.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub tone: Option<ui_contract::Tone>,
}

impl UiTreeItemNode {
    /** 🌳️ Builds a tree item with optional extensions unset. */
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
            content_lines: None,
            inline_toolbar: None,
            detail: None,
            dimmed: None,
            menu: None,
            window: None,
            granularity: None,
            tone: None,
        }
    }

    /// 🗂️ Whether the item discloses rows: materialised `items`, or a tree window over rows not streamed yet
    /// (`window.total > 0` with no children — React's `TreeDataWindow` reads it the same way).
    pub fn has_rows(&self) -> bool {
        self.items.as_deref().is_some_and(|items| !items.is_empty()) || self.window.is_some_and(|window| window.total > 0)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiTreeSectionNode {
    pub id: String,
    /// 🧰️ Horizontal toolbar rendered inside this section's header.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub header_toolbar: Option<UiStackNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    #[serde(default, skip_serializing_if = "UiPresence::is_default")]
    #[value(default, skip_serializing_if = "UiPresence::is_default")]
    pub presence: UiPresence,
    pub items: Vec<UiTreeItemNode>,
    /// 🔪️ The materialised slice of this section's logical child list — see [`UiTreeWindow`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<UiTreeWindow>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiTreeNode {
    #[serde(default, skip_serializing_if = "ui_contract::TreePresentation::is_standard")]
    #[value(default, skip_serializing_if = "ui_contract::TreePresentation::is_standard")]
    pub presentation: ui_contract::TreePresentation,
    pub sections: Vec<UiTreeSectionNode>,
    #[serde(default, skip_serializing_if = "UiPresence::is_default")]
    #[value(default, skip_serializing_if = "UiPresence::is_default")]
    pub presence: UiPresence,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub drop_action: Option<ActionDescriptor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub menu: Option<UiMenuRef>,
    /// 🕹️ Binds this rendered tree to an app-declared `InteractionDefinition` domain — the framework
    /// (not the app) then owns the domain's selection/hover via `interactionSelect`/`interactionHover`,
    /// stamped back onto item `presence` by `ui_tree_stamp_presence`. Replaces the deleted per-app
    /// `selected_ids`/`highlighted_ids`/`selection_change` wire surface.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
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
//#endregion 🔖️Primitives
// #endregion ui
