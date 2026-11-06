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
#[derive(semio_framework_value::NativeReceive,semio_framework_value::RetireOwned)]
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

/// 🌳️ A tree's inherited row and value-column presentation: `Compact` packs rows for measure panes, `Touch` makes every row at least 44 px tall (WCAG 2.5.5) so a finger hits it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
#[derive(semio_framework_value::NativeReceive,semio_framework_value::RetireOwned)]
pub enum TreePresentation {
    #[default]
    Standard,
    Compact,
    Touch,
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
#[derive(semio_framework_value::NativeReceive,semio_framework_value::RetireOwned)]
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
#[derive(semio_framework_value::NativeReceive,semio_framework_value::RetireOwned)]
pub struct TreeWindow {
    pub total: u32,
    pub offset: u32,
    pub row_extent: TreeWindowRowExtent,
}

/// 🔑️ Joins the segments of a windowed container's **window path** — the identity both sides of the [Design notes](README.md#source-notes-70).
pub const TREE_WINDOW_PATH_SEPARATOR: &str = "␟";

/// 📑️ Props for `Component::TreeSection` — a labeled, collapsible grouping of `TreeItem` children.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
#[derive(semio_framework_value::NativeReceive,semio_framework_value::RetireOwned)]
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

/// 🌿️ Props for `Component::TreeItem` — a single row. `items`/`control` are gone: nested items and [Design notes](README.md#source-notes-115).
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
#[derive(semio_framework_value::NativeReceive,semio_framework_value::RetireOwned)]
pub struct TreeItemProps {
    pub label: Label,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub draggable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub drag_data: Option<crate::UiFixedMap<crate::UiText>>,
    /// 👁️ Domain "eye toggle": the row stays visible, dimmed, and clickable (to un-hide). NOT the
    /// same axis as the record's `activity`/`disabled` — a dimmed row is still fully interactive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub dimmed: Option<bool>,
    /// ☑️ The row's choice state when it is one option of a single-choice list: `Some(true)` is the chosen option, `Some(false)` a
    /// choosable one that is not chosen, `None` not an option row. Every renderer exposes it as the row's selected state (React
    /// `aria-selected`, the wgpu ARIA mirror's `selected`) and paints the chosen row selected — never by its icon alone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
    /// 📇️ The materialised slice of this row's logical child list — see [`TreeWindow`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<TreeWindow>,
    /// 🏹️ The interaction granularity this row picks when the tree carries the domain binding: the
    /// row is a pick target of the tree's `interaction_domain`, keyed by its own record key, so it
    /// needs no argument map and no per-row binding of its own.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub granularity: Option<crate::UiText>,
    /// 🧰️ A direct `ContainerRole::Toolbar` child laid out horizontally inside this row. The
    /// document validator requires the target to be one of this record's direct children and every
    /// toolbar child to be a real [`Component::Button`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub inline_toolbar: Option<crate::UiNodeId>,
    /// 🎞️ A direct Surface child placed in a bounded detail band below this row. It is neither an
    /// inline control nor a nested TreeItem, and remains a real independently hosted scene.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<crate::UiNodeId>,
    #[serde(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    #[value(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    pub row_actions: crate::UiFixedList<RowAction>,
    /// 🎯️ The one target `row_actions` and the activation fire on — present exactly when either is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
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
