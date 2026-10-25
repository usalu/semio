//! 🌴️ Tree props: the tree props, presentation, window and row extent, the section props and the tree item props with their credited targets. See [`super`].

use super::*;

//#region 🎨️Props
/// 🌲️ Props for `Component::Tree` — the tree's own binding, nothing else. Sections and items are no
/// longer inline (`sections: Vec<UiTreeSectionNode>`); they are ordinary child nodes
/// (`Component::TreeSection` / `Component::TreeItem`) reached through the record's `children`.
/// `drop_action` moved to the record's `bindings` (`Trigger::Drop`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct TreeProps {
    #[serde(default, skip_serializing_if = "TreePresentation::is_standard")]
    #[value(default, skip_serializing_if = "TreePresentation::is_standard")]
    pub presentation: TreePresentation,
    /// 🕹️ Binds this tree to an app-declared `InteractionDefinition` domain — selection/hover for
    /// bound items is owned by the framework's presence channel, not by per-item props.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub interaction_domain: Option<crate::UiText>,
}

/// 🌳️ A tree's inherited row and value-column presentation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub enum TreePresentation {
    #[default]
    Standard,
    Compact,
}

impl TreePresentation {
    pub fn is_standard(&self) -> bool {
        matches!(self, Self::Standard)
    }
}

/// 🪟️ Closed geometry token for one unmaterialised row in a virtual Tree window.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub enum TreeWindowRowExtent {
    #[default]
    Standard,
    CompactText,
    CompactSmallControl,
    CompactControl,
}

/// 🍕️ The materialised slice of a logically `total`-long child list: the record's `children` are the
/// entries `[offset, offset + children.len())` of that list. `total > 0` with no materialised
/// children means expandable-but-not-yet-loaded, never "empty"; a renderer pitches the unmaterialised
/// rows as spacers so the scrollbar spans the whole document instead of the loaded window.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct TreeWindow {
    pub total: u32,
    pub offset: u32,
    pub row_extent: TreeWindowRowExtent,
}

/// 🔑️ Joins the segments of a windowed container's **window path** — the identity both sides of the
/// streaming loop address a container by (`ViewModel::tree_windows[].node_key`).
///
/// 🔑️ A container's path is the node keys of its enclosing windowed containers, outermost first, then
/// its own key. A top-level section's path is therefore just its own key, so every flat request stays
/// exactly what it was; a load case's loads nested under `fem3d-play-artifact.load-cases` are
/// `"fem3d-play-artifact.load-cases\u{1f}dead"`. This is what makes container identity collision-free
/// BY CONSTRUCTION while node keys and pick target ids stay untouched: a document whose load case and
/// whose combination are both called `uls` gives them different paths, because their parents differ.
/// Two containers can still collide only as true siblings under one parent — which the UI document
/// itself already refuses (`DuplicateSiblingKey`).
///
/// 🔑️ The separator is U+241F SYMBOL FOR UNIT SEPARATOR — the PRINTABLE glyph, not the C0 control
/// U+001F it depicts. A path crosses the process boundary inside `ViewModel::tree_windows`, and the
/// view-context admission both sides run
/// (`admitCrossingViewContext`/`parseResolvedPluginViewState`, `🛂️manifest/🟦️.ts`) refuses ANY
/// identifier carrying a C0 or DEL code point or exceeding 256 code points — a control separator made
/// every refresh, action and pick carrying a nested path throw `view context: invalid identifier` the
/// moment one nested container existed. A path is therefore a view-context identifier like any other:
/// printable, and at most 256 code points. A container whose own key already contains this glyph is
/// refused at assembly (`ui.tree-window.separator-in-key`) so a path can never be ambiguous.
pub const TREE_WINDOW_PATH_SEPARATOR: &str = "␟";

/// 📑️ Props for `Component::TreeSection` — a labeled, collapsible grouping of `TreeItem` children.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct TreeSectionProps {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    /// 🎛️ The section's direct horizontal Toolbar child whose Button children render in the header.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub header_toolbar: Option<crate::UiNodeId>,
    /// 🔪️ The materialised slice of this section's logical child list — see [`TreeWindow`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<TreeWindow>,
}

/// 🌿️ Props for `Component::TreeItem` — a single row. `items`/`control` are gone: nested items and
/// the old inline `control: Option<UiControlNode>` are now ordinary children on the record (the
/// `UiControlNode` enum does not get ported — every one of its old variants is already a
/// [`Component`] variant in its own right, so a control-as-child-node needs no separate wrapper
/// type). [`TreeItemProps::inline_toolbar`] identifies the one direct horizontal Toolbar child whose
/// real Button children remain independently focusable and actionable inside the row, while
/// [`TreeItemProps::detail`] identifies one direct Surface child placed below the row. The row's
/// primary click action (old `action: Option<ActionDescriptor>`) is [`RowTarget::activation`], a verb on
/// the row's target — never a record `Trigger::Activate` binding.
// 🌱️ No `ToValue`/`FromValue` here: `target: Option<RowTarget>` embeds `UiValue` — see [`RowTarget`].
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeItemProps {
    pub label: Label,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draggable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_data: Option<crate::UiFixedMap<crate::UiText>>,
    /// 👁️ Domain "eye toggle": the row stays visible, dimmed, and clickable (to un-hide). NOT the
    /// same axis as the record's `activity`/`disabled` — a dimmed row is still fully interactive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimmed: Option<bool>,
    /// ☑️ The row's choice state when it is one option of a single-choice list: `Some(true)` is the chosen option, `Some(false)` a
    /// choosable one that is not chosen, `None` not an option row. Every renderer exposes it as the row's selected state (React
    /// `aria-selected`, the wgpu ARIA mirror's `selected`) and paints the chosen row selected — never by its icon alone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
    /// 📇️ The materialised slice of this row's logical child list — see [`TreeWindow`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<TreeWindow>,
    /// 🏹️ The interaction granularity this row picks when the tree carries the domain binding: the
    /// row is a pick target of the tree's `interaction_domain`, keyed by its own record key, so it
    /// needs no argument map and no per-row binding of its own.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granularity: Option<crate::UiText>,
    /// 🧰️ A direct `ContainerRole::Toolbar` child laid out horizontally inside this row. The
    /// document validator requires the target to be one of this record's direct children and every
    /// toolbar child to be a real [`Component::Button`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline_toolbar: Option<crate::UiNodeId>,
    /// 🎞️ A direct Surface child placed in a bounded detail band below this row. It is neither an
    /// inline control nor a nested TreeItem, and remains a real independently hosted scene.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<crate::UiNodeId>,
    #[serde(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    pub row_actions: crate::UiFixedList<RowAction>,
    /// 🎯️ The one target `row_actions` and the activation fire on — present exactly when either is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<RowTarget>,
}

impl TreeItemProps {
    pub(super) fn credited_clone(&self) -> Option<Self> {
        Some(Self {
            label: self.label.clone(),
            description: self.description.clone(),
            icon: self.icon.clone(),
            default_open: self.default_open,
            draggable: self.draggable,
            drag_data: self.drag_data.clone(),
            dimmed: self.dimmed,
            selected: self.selected,
            window: self.window,
            granularity: self.granularity.clone(),
            inline_toolbar: self.inline_toolbar,
            detail: self.detail,
            row_actions: self.row_actions.clone(),
            target: credited_target(&self.target)?,
        })
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(super) fn credited_target(target: &Option<RowTarget>) -> Option<Option<RowTarget>> {
    match target {
        Some(target) => Some(Some(target.credited_clone()?)),
        None => Some(None),
    }
}
//#endregion 🎨️Props
