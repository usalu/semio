//! 📚️ Stack, container, text and button builders: the stack, column and row builders, the section and field container builder and the text and button builders. See [`super`].

use super::*;

//#region 🔖️Builder
//#region 📚️Stack
/// 📚️ A one-axis flex-like container — `Component::Container(role: Plain)` over
/// `LayoutSpec::Stack`. Build with [`stack`], [`column`], or [`row`].
pub struct StackBuilder {
    pub(super) base: NodeBase,
}

/// 📚️ A stack laid out along `axis`. [`column`]/[`row`] are the common-case shorthands.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn stack(axis: crate::Axis) -> StackBuilder {
    StackBuilder { base: NodeBase::stack(axis) }
}

/// 📚️ A vertical [`stack`].
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn column() -> StackBuilder {
    stack(crate::Axis::Vertical)
}

/// 📚️ A horizontal [`stack`].
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn row() -> StackBuilder {
    stack(crate::Axis::Horizontal)
}

impl HasBase for StackBuilder {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

impl HasChildren for StackBuilder {}

impl HasStackLayout for StackBuilder {}

impl From<StackBuilder> for BuiltNode {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(builder: StackBuilder) -> Self {
        assemble(builder.base, crate::Component::Container(crate::ContainerProps { role: crate::ContainerRole::Plain, label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }))
    }
}
//#endregion 📚️Stack

//#region 🗂️Container
/// 🗂️ A labeled, chrome-carrying container — `Component::Container(role: Section | Field)`. Build
/// with [`section`] or [`field`].
pub struct ContainerBuilder {
    pub(super) base: NodeBase,
    pub(super) role: crate::ContainerRole,
    pub(super) label: crate::Label,
    pub(super) description: Option<crate::UiText>,
    pub(super) required: Option<bool>,
    pub(super) error: Option<crate::UiText>,
    pub(super) default_open: Option<bool>,
    pub(super) drop_overlay: Option<crate::DropOverlaySpec>,
}

impl ContainerBuilder {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(super) fn new(role: crate::ContainerRole, label: crate::Label) -> Self {
        Self { base: NodeBase::stack(crate::Axis::Vertical), role, label, description: None, required: None, error: None, default_open: None, drop_overlay: None }
    }

    /// 📝️ Sets the container's help/description copy.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn description(mut self, description: crate::UiText) -> Self {
        self.description = Some(description);
        self
    }

    /// ❗️ Marks the container required (e.g. a form field).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }

    /// ⚠️ Sets a validation error message.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn error(mut self, error: crate::UiText) -> Self {
        self.error = Some(error);
        self
    }

    /// 🔽️ Sets whether a collapsible section starts open.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = Some(default_open);
        self
    }

    /// 📥️ Sets the hover-state copy shown while a drag is over this container.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn drop_overlay(mut self, drop_overlay: crate::DropOverlaySpec) -> Self {
        self.drop_overlay = Some(drop_overlay);
        self
    }
}

/// 🗂️ A labeled, collapsible section — `Component::Container(role: Section)`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn section(label: crate::Label) -> ContainerBuilder {
    ContainerBuilder::new(crate::ContainerRole::Section, label)
}

/// 🗂️ A labeled form field — `Component::Container(role: Field)`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn field(label: crate::Label) -> ContainerBuilder {
    ContainerBuilder::new(crate::ContainerRole::Field, label)
}

impl HasBase for ContainerBuilder {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

impl HasChildren for ContainerBuilder {}

impl HasStackLayout for ContainerBuilder {}

impl From<ContainerBuilder> for BuiltNode {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(builder: ContainerBuilder) -> Self {
        assemble(
            builder.base,
            crate::Component::Container(crate::ContainerProps {
                role: builder.role,
                label: Some(builder.label),
                description: builder.description,
                required: builder.required,
                error: builder.error,
                default_open: builder.default_open,
                drop_overlay: builder.drop_overlay,
            }),
        )
    }
}
//#endregion 🗂️Container

//#region 📝️Text
/// 📝️ Plain display text — `Component::Text`. Build with [`text`].
pub struct TextBuilder {
    pub(super) base: NodeBase,
    pub(super) value: crate::Label,
    pub(super) emphasize: Option<bool>,
}

/// 📝️ Display text reading `value`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn text(value: crate::Label) -> TextBuilder {
    TextBuilder { base: NodeBase::leaf(), value, emphasize: None }
}

impl TextBuilder {
    /// 🔆️ Marks the text visually emphasized.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn emphasize(mut self, emphasize: bool) -> Self {
        self.emphasize = Some(emphasize);
        self
    }
}

impl HasBase for TextBuilder {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

impl From<TextBuilder> for BuiltNode {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(builder: TextBuilder) -> Self {
        assemble(builder.base, crate::Component::Text(crate::TextProps { value: builder.value, emphasize: builder.emphasize, data_attributes: None }))
    }
}
//#endregion 📝️Text

//#region 🔘️Button
/// 🔘️ A clickable button — `Component::Button`. Build with [`button`].
pub struct ButtonBuilder {
    pub(super) base: NodeBase,
    pub(super) label: crate::Label,
    pub(super) icon: crate::UiText,
}

/// 🔘️ A button reading `label`. Its accessible name defaults to `label` — override with
/// [`HasBase::label`] if the visible and accessible text should differ.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn button(label: crate::Label) -> ButtonBuilder {
    let mut base = NodeBase::leaf();
    base.accessibility.label = Some(label.clone());
    ButtonBuilder { base, label, icon: crate::UiText::default() }
}

impl ButtonBuilder {
    /// 🖼️ Sets the icon key.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn icon(mut self, icon: crate::UiText) -> Self {
        self.icon = icon;
        self
    }
}

impl HasBase for ButtonBuilder {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

impl From<ButtonBuilder> for BuiltNode {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(builder: ButtonBuilder) -> Self {
        assemble(builder.base, crate::Component::Button(crate::ButtonProps { icon: builder.icon, label: builder.label }))
    }
}
//#endregion 🔘️Button
//#endregion 🔖️Builder
