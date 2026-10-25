//! 🌴️ Tree builders: the tree, tree section and tree item builders. See [`super`].

use super::*;

//#region 🔖️Builder
//#region 🌲️Tree
/// 🌲️ A selection-bound tree's own binding node — `Component::Tree`. Sections and items are its
/// [`HasChildren::child`]ren, not inline fields. Build with [`tree`].
pub struct TreeBuilder {
    pub(super) base: NodeBase,
    presentation: crate::TreePresentation,
    interaction_domain: Option<crate::UiText>,
}

/// 🌱️ An empty tree, ready for [`tree_section`]/[`tree_item`] children.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn tree() -> TreeBuilder {
    TreeBuilder { base: NodeBase::stack(crate::Axis::Vertical), presentation: crate::TreePresentation::Standard, interaction_domain: None }
}

impl TreeBuilder {
    /// 🌳️ Selects the inherited row and value-column presentation.
    pub fn presentation(mut self, presentation: crate::TreePresentation) -> Self {
        self.presentation = presentation;
        self
    }

    /// 🕹️ Binds this tree to an app-declared `InteractionDefinition` domain.
    pub fn interaction_domain(mut self, domain: crate::UiText) -> Self {
        self.interaction_domain = Some(domain);
        self
    }
}

impl HasBase for TreeBuilder {
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

impl HasChildren for TreeBuilder {}

impl HasStackLayout for TreeBuilder {}

impl From<TreeBuilder> for BuiltNode {
    fn from(builder: TreeBuilder) -> Self {
        assemble(builder.base, crate::Component::Tree(crate::TreeProps { presentation: builder.presentation, interaction_domain: builder.interaction_domain }))
    }
}

/// 📑️ A labeled, collapsible grouping of tree items — `Component::TreeSection`. Build with
/// [`tree_section`].
pub struct TreeSectionBuilder {
    pub(super) base: NodeBase,
    pub(super) label: crate::Label,
    pub(super) default_open: Option<bool>,
    pub(super) window: Option<crate::TreeWindow>,
    header_toolbar: Option<crate::UiNodeId>,
}

/// 📗️ A tree section reading `label`, ready for [`tree_item`] children.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn tree_section(label: crate::Label) -> TreeSectionBuilder {
    TreeSectionBuilder { base: NodeBase::stack(crate::Axis::Vertical), label, default_open: None, window: None, header_toolbar: None }
}

impl TreeSectionBuilder {
    /// 🔽️ Sets whether the section starts open.
    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = Some(default_open);
        self
    }

    /// 🪟️ Declares which slice of a longer logical child list the built children actually are — see
    /// [`crate::TreeWindow`].
    pub fn window(mut self, window: crate::TreeWindow) -> Self {
        self.window = Some(window);
        self
    }

    /// 🎛️ Relates a direct horizontal Toolbar child to the section header.
    pub fn header_toolbar(mut self, header_toolbar: crate::UiNodeId) -> Self {
        self.header_toolbar = Some(header_toolbar);
        self
    }
}

impl HasBase for TreeSectionBuilder {
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

impl HasChildren for TreeSectionBuilder {}

impl HasStackLayout for TreeSectionBuilder {}

impl From<TreeSectionBuilder> for BuiltNode {
    fn from(builder: TreeSectionBuilder) -> Self {
        assemble(builder.base, crate::Component::TreeSection(crate::TreeSectionProps { label: Some(builder.label), default_open: builder.default_open, window: builder.window, header_toolbar: builder.header_toolbar }))
    }
}

/// 🌿️ A single tree row — `Component::TreeItem`. Nested items are its [`HasChildren::child`]ren.
/// Build with [`tree_item`].
pub struct TreeItemBuilder {
    pub(super) base: NodeBase,
    pub(super) label: crate::Label,
    pub(super) description: Option<crate::UiText>,
    pub(super) icon: Option<crate::UiText>,
    pub(super) default_open: Option<bool>,
    draggable: Option<bool>,
    drag_data: Option<crate::UiFixedMap<crate::UiText>>,
    dimmed: Option<bool>,
    selected: Option<bool>,
    pub(super) window: Option<crate::TreeWindow>,
    granularity: Option<crate::UiText>,
    pub(super) row_actions: crate::UiFixedList<crate::RowAction>,
    pub(super) target: Option<crate::RowTarget>,
}

/// 🍃️ A tree row reading `label`. Its accessible name defaults to `label`, the same "hard to omit"
/// derivation [`button`] applies.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn tree_item(label: crate::Label) -> TreeItemBuilder {
    let mut base = NodeBase::stack(crate::Axis::Vertical);
    base.accessibility.label = Some(label.clone());
    TreeItemBuilder { base, label, description: None, icon: None, default_open: None, draggable: None, drag_data: None, dimmed: None, selected: None, window: None, granularity: None, row_actions: crate::UiFixedList::default(), target: None }
}

impl TreeItemBuilder {
    /// 📝️ Sets secondary description copy.
    pub fn description(mut self, description: crate::UiText) -> Self {
        self.description = Some(description);
        self
    }

    /// 🖼️ Sets the icon key.
    pub fn icon(mut self, icon: crate::UiText) -> Self {
        self.icon = Some(icon);
        self
    }

    /// 📂️ Sets whether the row starts open (when it has nested items).
    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = Some(default_open);
        self
    }

    /// 🖐️ Marks the row draggable.
    pub fn draggable(mut self, draggable: bool) -> Self {
        self.draggable = Some(draggable);
        self
    }

    /// 🏷️ Sets the payload a drag of this row carries.
    pub fn drag_data(mut self, drag_data: crate::UiFixedMap<crate::UiText>) -> Self {
        self.drag_data = Some(drag_data);
        self
    }

    /// 👁️ Sets the domain "eye toggle" dimmed state — the row stays visible and clickable, just
    /// visually de-emphasized; not the same axis as [`HasBase::disabled`].
    pub fn dimmed(mut self, dimmed: bool) -> Self {
        self.dimmed = Some(dimmed);
        self
    }

    /// ☑️ Marks this row as one option of a single-choice list, chosen or not ([`crate::TreeItemProps::selected`]).
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = Some(selected);
        self
    }

    /// 🍕️ Declares which slice of a longer logical child list the built children actually are — see
    /// [`crate::TreeWindow`].
    pub fn window(mut self, window: crate::TreeWindow) -> Self {
        self.window = Some(window);
        self
    }

    /// 🏹️ Makes the row a pick target of the tree's `interaction_domain` at this granularity, keyed
    /// by the row's own record key — no per-row binding and no argument map.
    pub fn granularity(mut self, granularity: crate::UiText) -> Self {
        self.granularity = Some(granularity);
        self
    }

    /// 🎬️ Appends one row action — a verb on the row's [`crate::RowTarget`].
    #[expect(clippy::result_large_err, reason = "A full row-action list returns its builder and original action owner for caller-directed retirement.")]
    pub fn try_row_action(mut self, row_action: crate::RowAction) -> Result<Self, (Self, crate::RowAction)> {
        match self.row_actions.try_push(row_action) {
            Ok(()) => Ok(self),
            Err(row_action) => Err((self, row_action)),
        }
    }

    /// 🎯️ Sets the ONE target every row action and the activation of this row fire on.
    pub fn target(mut self, target: crate::RowTarget) -> Self {
        self.target = Some(target);
        self
    }
}

impl HasBase for TreeItemBuilder {
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

impl HasChildren for TreeItemBuilder {}

impl HasStackLayout for TreeItemBuilder {}

impl From<TreeItemBuilder> for BuiltNode {
    fn from(builder: TreeItemBuilder) -> Self {
        assemble(
            builder.base,
            crate::Component::TreeItem(crate::TreeItemProps {
                label: builder.label,
                description: builder.description,
                icon: builder.icon,
                default_open: builder.default_open,
                draggable: builder.draggable,
                drag_data: builder.drag_data,
                dimmed: builder.dimmed,
                selected: builder.selected,
                window: builder.window,
                granularity: builder.granularity,
                inline_toolbar: None,
                detail: None,
                row_actions: builder.row_actions,
                target: builder.target,
            }),
        )
    }
}
//#endregion 🌲️Tree
//#endregion 🔖️Builder
