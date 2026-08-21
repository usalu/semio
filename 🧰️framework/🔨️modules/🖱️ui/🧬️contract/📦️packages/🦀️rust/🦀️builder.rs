//! @emoji 🏗️ Ergonomic semantic builders (`ui::stack()`, `ui::button()`) — wasip2-safe, no engine.
//!
//! 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
//! below is plain sync by owner ruling U1.
//!
//! ⚠️ Decision (flagged for the coordinator, detailed in `📓️terra-contract-builder-report.md`):
//! [`BuiltNode`] duplicates the shape the sibling `semio-framework-ui-runtime` crate's own
//! `ComponentTree` needs, because this crate must not depend on that crate (the dependency runs the
//! other way — see `📦️glue.rs`). The runtime converts a `BuiltNode` tree into its own reconciler
//! input; if the two shapes drift, that conversion is the one place to fix, never a second builder.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

//#region 🔖️Builder

//#region 🧱️BuiltNode
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn is_false(value: &bool) -> bool {
    !*value
}

/// 🧱️ The contract-local shape a builder terminates into — everything a [`crate::UiNodeRecord`]
/// carries except `id` (minted by the runtime at reconciliation, never by an author) and with
/// `children` nested inline rather than addressed by [`crate::UiNodeId`], since a freshly authored
/// tree has no ids yet to address by. Every field below serializes away at its default, mirroring the
/// wire-cost guarantee this whole builder family exists to make automatic.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct BuiltNode {
    /// 🔑️ The reconciliation key — an author-set [`HasBase::id`] or a positional `"#N"` fallback. See
    /// [`HasChildren::child`] for why the fallback is only stable while sibling order is unchanged.
    pub key: String,
    pub component: crate::Component,
    #[serde(default, skip_serializing_if = "is_default")]
    pub layout: crate::LayoutSpec,
    #[serde(default, skip_serializing_if = "is_default")]
    pub style: crate::StyleSpec,
    #[serde(default, skip_serializing_if = "is_default")]
    pub activity: crate::Activity,
    #[serde(default, skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub accessibility: crate::AccessibilitySpec,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bindings: Vec<crate::ActionBinding>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub menu: Option<crate::MenuRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<BuiltNode>,
}
//#endregion 🧱️BuiltNode

//#region 🧩️Base
/// 🧰️ Record-level state shared by every builder in this file — reachable only through the chainable
/// methods on [`HasBase`]/[`HasChildren`]/[`HasStackLayout`], never through its own fields, which stay
/// private to this module.
pub struct NodeBase {
    id: Option<String>,
    layout: crate::LayoutSpec,
    style: crate::StyleSpec,
    activity: crate::Activity,
    disabled: bool,
    accessibility: crate::AccessibilitySpec,
    bindings: Vec<crate::ActionBinding>,
    menu: Option<crate::MenuRef>,
    children: Vec<BuiltNode>,
}

impl NodeBase {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn with_layout(layout: crate::LayoutSpec) -> Self {
        Self { id: None, layout, style: crate::StyleSpec::default(), activity: crate::Activity::default(), disabled: false, accessibility: crate::AccessibilitySpec::default(), bindings: Vec::new(), menu: None, children: Vec::new() }
    }

    /// 🍃️ A terminal node's own base — [`crate::LayoutSpec::Leaf`], the crate's own default.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn leaf() -> Self {
        Self::with_layout(crate::LayoutSpec::default())
    }

    /// 📚️ A children-arranging node's own base — [`crate::LayoutSpec::Stack`] along `axis`.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn stack(axis: crate::Axis) -> Self {
        Self::with_layout(crate::LayoutSpec::Stack(crate::StackLayout { axis, ..Default::default() }))
    }
}

/// 🔢️ The fallback key for a node at `index` among its siblings — stable only while sibling count
/// and order stay fixed; see [`HasChildren::child`].
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn positional_key(index: usize) -> String {
    format!("#{index}")
}

/// 🏗️ Assembles `base` and `component` into a [`BuiltNode`], leaving `key` empty when `base.id` was
/// never set — the caller (either [`Buildable::build`] or [`HasChildren::child`]) fills that sentinel
/// with the right default for its position.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn assemble(base: NodeBase, component: crate::Component) -> BuiltNode {
    BuiltNode {
        key: base.id.unwrap_or_default(),
        component,
        layout: base.layout,
        style: base.style,
        activity: base.activity,
        disabled: base.disabled,
        accessibility: base.accessibility,
        bindings: base.bindings,
        menu: base.menu,
        children: base.children,
    }
}
//#endregion 🧩️Base

//#region 🔧️Traits
/// 🔧️ The chainable vocabulary every builder in this file carries — identity, style, visual state,
/// actions, and accessibility overrides. Set [`HasBase::id`] explicitly for any node whose position
/// among its siblings can change: the positional fallback key ([`HasChildren::child`]) is what
/// preserves a node's state across a re-present, and it only stays stable while sibling order and
/// count do not.
pub trait HasBase: Sized {
    /// 🔩️ Internal accessor — never call directly; use the chainable methods below.
    #[doc(hidden)]
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn base_mut(&mut self) -> &mut NodeBase;

    /// 🔑️ Sets the reconciliation key explicitly.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn id(mut self, id: impl Into<String>) -> Self {
        self.base_mut().id = Some(id.into());
        self
    }

    /// 🚫️ Marks the node non-interactive without removing it from the tree.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn disabled(mut self, disabled: bool) -> Self {
        self.base_mut().disabled = disabled;
        self
    }

    /// 🧭️ Sets the node's [`crate::Activity`] lifecycle state.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn activity(mut self, activity: crate::Activity) -> Self {
        self.base_mut().activity = activity;
        self
    }

    /// 🎨️ Overrides the node's design-token [`crate::StyleSpec`] wholesale.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn style(mut self, style: crate::StyleSpec) -> Self {
        self.base_mut().style = style;
        self
    }

    /// 🎨️ Sets the [`crate::Tone`] color role.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn tone(mut self, tone: crate::Tone) -> Self {
        self.base_mut().style.tone = tone;
        self
    }

    /// 🖌️ Sets the [`crate::Variant`] chrome treatment.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn variant(mut self, variant: crate::Variant) -> Self {
        self.base_mut().style.variant = variant;
        self
    }

    /// 📏️ Sets the [`crate::SizeToken`] t-shirt size.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn size(mut self, size: crate::SizeToken) -> Self {
        self.base_mut().style.size = size;
        self
    }

    /// 🔆️ Sets the [`crate::Emphasis`] visual prominence.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn emphasis(mut self, emphasis: crate::Emphasis) -> Self {
        self.base_mut().style.emphasis = emphasis;
        self
    }

    /// 📐️ Sets the [`crate::Density`] spacing.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn density(mut self, density: crate::Density) -> Self {
        self.base_mut().style.density = density;
        self
    }

    /// 📋️ Attaches a resolved context menu.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn menu(mut self, menu: crate::MenuRef) -> Self {
        self.base_mut().menu = Some(menu);
        self
    }

    /// 🎬️ Binds `trigger` to `action` with no args.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn on(mut self, trigger: crate::Trigger, action: crate::ActionId) -> Self {
        self.base_mut().bindings.push(crate::ActionBinding { trigger, action, args: None, capability: None });
        self
    }

    /// 🎬️ Binds `trigger` to `action`, carrying `args` for the action to consume.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn on_with(mut self, trigger: crate::Trigger, action: crate::ActionId, args: crate::UiValue) -> Self {
        self.base_mut().bindings.push(crate::ActionBinding { trigger, action, args: Some(args), capability: None });
        self
    }

    /// ♿️ Overrides the node's accessible name. [`button`] and [`tree_item`] already derive this from
    /// their visible label; call this only to override that default.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn label(mut self, label: impl Into<crate::Label>) -> Self {
        self.base_mut().accessibility.label = Some(label.into());
        self
    }

    /// ♿️ Sets the node's accessible description.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn describe(mut self, description: impl Into<crate::Label>) -> Self {
        self.base_mut().accessibility.description = Some(description.into());
        self
    }
}

/// 👶️ Adds `.child(..)`/`.children(..)` to a builder whose component genuinely nests others.
pub trait HasChildren: HasBase {
    /// 👶️ Appends one already-built or still-buildable child. An omitted child key becomes
    /// `"#{position}"` at the moment it is pushed here — stable only while the number and order of its
    /// siblings do not change, which is why a node whose position can shift (a reorderable list row,
    /// for instance) should carry an explicit [`HasBase::id`] instead.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn child(mut self, child: impl Into<BuiltNode>) -> Self {
        let mut node = child.into();
        if node.key.is_empty() {
            node.key = positional_key(self.base_mut().children.len());
        }
        self.base_mut().children.push(node);
        self
    }

    /// 👶️ Appends every item of `children` in order, each through [`HasChildren::child`].
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn children<T: Into<BuiltNode>, I: IntoIterator<Item = T>>(mut self, children: I) -> Self {
        for child in children {
            self = self.child(child);
        }
        self
    }
}

/// 📏️ Adds [`crate::StackLayout`] tuning to a builder whose default layout is
/// [`crate::LayoutSpec::Stack`] — true of every implementor in this file, so these methods never fall
/// through to a no-op in practice.
pub trait HasStackLayout: HasBase {
    /// ↔️ Sets the gap between children.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn gap(mut self, gap: crate::SpaceToken) -> Self {
        if let crate::LayoutSpec::Stack(stack) = &mut self.base_mut().layout {
            stack.gap = gap;
        }
        self
    }

    /// 📐️ Sets the container's own padding.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn padding(mut self, padding: crate::EdgeSpace) -> Self {
        if let crate::LayoutSpec::Stack(stack) = &mut self.base_mut().layout {
            stack.padding = padding;
        }
        self
    }

    /// ↕️ Sets cross-axis alignment.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn align(mut self, align: crate::Align) -> Self {
        if let crate::LayoutSpec::Stack(stack) = &mut self.base_mut().layout {
            stack.align = align;
        }
        self
    }

    /// ↔️ Sets main-axis distribution.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn justify(mut self, justify: crate::Justify) -> Self {
        if let crate::LayoutSpec::Stack(stack) = &mut self.base_mut().layout {
            stack.justify = justify;
        }
        self
    }

    /// 📈️ Lets the stack grow to fill its parent's remaining main-axis space.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn grow(mut self, grow: bool) -> Self {
        if let crate::LayoutSpec::Stack(stack) = &mut self.base_mut().layout {
            stack.grow = grow;
        }
        self
    }

    /// 🔁️ Permits children to wrap onto additional lines.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn wrap(mut self, wrap: bool) -> Self {
        if let crate::LayoutSpec::Stack(stack) = &mut self.base_mut().layout {
            stack.wrap = wrap;
        }
        self
    }
}

/// 🏗️ Finalizes any builder into a [`BuiltNode`], filling an empty key with the positional default
/// `"#0"`. Blanket-implemented for every `T: Into<BuiltNode>`; [`ImageBuilder<NoAlt>`](ImageBuilder)
/// deliberately does NOT implement `Into<BuiltNode>` (only [`ImageBuilder<HasAlt>`](ImageBuilder) does),
/// so this trait — and therefore `.build()` — is simply absent from `ImageBuilder<NoAlt>`'s method set
/// at compile time, rather than present but panicking.
pub trait Buildable: Into<BuiltNode> {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn build(self) -> BuiltNode
    where
        Self: Sized,
    {
        let mut node: BuiltNode = self.into();
        if node.key.is_empty() {
            node.key = positional_key(0);
        }
        node
    }
}

impl<T: Into<BuiltNode>> Buildable for T {}
//#endregion 🔧️Traits

//#region 📚️Stack
/// 📚️ A one-axis flex-like container — `Component::Container(role: Plain)` over
/// `LayoutSpec::Stack`. Build with [`stack`], [`column`], or [`row`].
pub struct StackBuilder {
    base: NodeBase,
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
    base: NodeBase,
    role: crate::ContainerRole,
    label: crate::Label,
    description: Option<String>,
    required: Option<bool>,
    error: Option<String>,
    default_open: Option<bool>,
    drop_overlay: Option<crate::DropOverlaySpec>,
}

impl ContainerBuilder {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn new(role: crate::ContainerRole, label: crate::Label) -> Self {
        Self { base: NodeBase::stack(crate::Axis::Vertical), role, label, description: None, required: None, error: None, default_open: None, drop_overlay: None }
    }

    /// 📝️ Sets the container's help/description copy.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
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
    pub fn error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
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
pub fn section(label: impl Into<crate::Label>) -> ContainerBuilder {
    ContainerBuilder::new(crate::ContainerRole::Section, label.into())
}

/// 🗂️ A labeled form field — `Component::Container(role: Field)`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn field(label: impl Into<crate::Label>) -> ContainerBuilder {
    ContainerBuilder::new(crate::ContainerRole::Field, label.into())
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
    base: NodeBase,
    value: crate::Label,
    emphasize: Option<bool>,
}

/// 📝️ Display text reading `value`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn text(value: impl Into<crate::Label>) -> TextBuilder {
    TextBuilder { base: NodeBase::leaf(), value: value.into(), emphasize: None }
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
    base: NodeBase,
    label: crate::Label,
    icon: String,
}

/// 🔘️ A button reading `label`. Its accessible name defaults to `label` — override with
/// [`HasBase::label`] if the visible and accessible text should differ.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn button(label: impl Into<crate::Label>) -> ButtonBuilder {
    let label = label.into();
    let mut base = NodeBase::leaf();
    base.accessibility.label = Some(label.clone());
    ButtonBuilder { base, label, icon: String::new() }
}

impl ButtonBuilder {
    /// 🖼️ Sets the icon key.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = icon.into();
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

//#region ⌨️Input
/// ⌨️ A text/number/date/color/file input — `Component::Input`. Build with [`input`].
pub struct InputBuilder {
    base: NodeBase,
    kind: crate::InputKind,
    value: String,
    placeholder: Option<crate::Label>,
    commit: Option<String>,
    min: Option<f64>,
    max: Option<f64>,
    step: Option<f64>,
    accept: Option<String>,
}

/// ⌨️ An input of `kind`, initially empty.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn input(kind: crate::InputKind) -> InputBuilder {
    InputBuilder { base: NodeBase::leaf(), kind, value: String::new(), placeholder: None, commit: None, min: None, max: None, step: None, accept: None }
}

impl InputBuilder {
    /// ✍️ Sets the current value.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    /// 💬️ Sets placeholder copy.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn placeholder(mut self, placeholder: impl Into<crate::Label>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// 🫳️ Sets the commit convention (e.g. `"blur"`).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn commit(mut self, commit: impl Into<String>) -> Self {
        self.commit = Some(commit.into());
        self
    }

    /// ⬇️ Sets the minimum value (`InputKind::Number`).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self
    }

    /// ⬆️ Sets the maximum value (`InputKind::Number`).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    /// 🔢️ Sets the step increment (`InputKind::Number`).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn step(mut self, step: f64) -> Self {
        self.step = Some(step);
        self
    }

    /// 📎️ Sets the accepted file-type filter (`InputKind::File`).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn accept(mut self, accept: impl Into<String>) -> Self {
        self.accept = Some(accept.into());
        self
    }
}

impl HasBase for InputBuilder {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

impl From<InputBuilder> for BuiltNode {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(builder: InputBuilder) -> Self {
        assemble(
            builder.base,
            crate::Component::Input(crate::InputProps { kind: builder.kind, value: builder.value, placeholder: builder.placeholder, commit: builder.commit, min: builder.min, max: builder.max, step: builder.step, accept: builder.accept }),
        )
    }
}
//#endregion ⌨️Input

//#region 🔀️Toggle
/// 🔀️ A binary switch — `Component::Toggle`. Build with [`toggle`].
pub struct ToggleBuilder {
    base: NodeBase,
    on: bool,
    icon: String,
    text: Option<crate::Label>,
}

/// 🔀️ A toggle currently `on` or off.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn toggle(on: bool) -> ToggleBuilder {
    ToggleBuilder { base: NodeBase::leaf(), on, icon: String::new(), text: None }
}

impl ToggleBuilder {
    /// 🖼️ Sets the icon key.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = icon.into();
        self
    }

    /// 📝️ Sets the visible text, and — unless [`HasBase::label`] already set one explicitly — the
    /// accessible name too, for the same "hard to omit" reason [`button`] auto-derives its own.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn text(mut self, text: impl Into<crate::Label>) -> Self {
        let text = text.into();
        self.base.accessibility.label.get_or_insert_with(|| text.clone());
        self.text = Some(text);
        self
    }
}

impl HasBase for ToggleBuilder {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

impl From<ToggleBuilder> for BuiltNode {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(builder: ToggleBuilder) -> Self {
        assemble(builder.base, crate::Component::Toggle(crate::ToggleProps { on: builder.on, icon: builder.icon, text: builder.text }))
    }
}
//#endregion 🔀️Toggle

//#region 🔽️Select
/// 🔽️ A single-choice dropdown — `Component::Select`. Build with [`select`].
pub struct SelectBuilder {
    base: NodeBase,
    value: String,
    items: Vec<crate::SelectItem>,
    placeholder: Option<crate::Label>,
}

/// 🔽️ A select currently holding `value`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn select(value: impl Into<String>) -> SelectBuilder {
    SelectBuilder { base: NodeBase::leaf(), value: value.into(), items: Vec::new(), placeholder: None }
}

impl SelectBuilder {
    /// ➕️ Appends one option.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn item(mut self, value: impl Into<String>, label: impl Into<crate::Label>) -> Self {
        self.items.push(crate::SelectItem { value: value.into(), label: label.into() });
        self
    }

    /// ➕️ Appends every option of `items` in order.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn items(mut self, items: impl IntoIterator<Item = crate::SelectItem>) -> Self {
        self.items.extend(items);
        self
    }

    /// 💬️ Sets placeholder copy shown while no option is selected.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn placeholder(mut self, placeholder: impl Into<crate::Label>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }
}

impl HasBase for SelectBuilder {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

impl From<SelectBuilder> for BuiltNode {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(builder: SelectBuilder) -> Self {
        assemble(builder.base, crate::Component::Select(crate::SelectProps { value: builder.value, items: builder.items, placeholder: builder.placeholder }))
    }
}
//#endregion 🔽️Select

//#region 🎚️Slider
/// 🎚️ A continuous range control — `Component::Slider`. Build with [`slider`].
pub struct SliderBuilder {
    base: NodeBase,
    value: f64,
    min: f64,
    max: f64,
    step: f64,
    unit: Option<String>,
}

/// 🎚️ A slider currently at `value`, defaulting to the `0.0..=1.0` normalized range with a `0.1`
/// step — the common case for an unlabeled proportion.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn slider(value: f64) -> SliderBuilder {
    SliderBuilder { base: NodeBase::leaf(), value, min: 0.0, max: 1.0, step: 0.1, unit: None }
}

impl SliderBuilder {
    /// ⬇️ Sets the minimum value.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn min(mut self, min: f64) -> Self {
        self.min = min;
        self
    }

    /// ⬆️ Sets the maximum value.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }

    /// 🔢️ Sets the step increment.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }

    /// 📏️ Sets the displayed unit suffix.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }
}

impl HasBase for SliderBuilder {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

impl From<SliderBuilder> for BuiltNode {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(builder: SliderBuilder) -> Self {
        assemble(builder.base, crate::Component::Slider(crate::SliderProps { value: builder.value, min: builder.min, max: builder.max, step: builder.step, unit: builder.unit }))
    }
}
//#endregion 🎚️Slider

//#region 🌲️Tree
/// 🌲️ A selection-bound tree's own binding node — `Component::Tree`. Sections and items are its
/// [`HasChildren::child`]ren, not inline fields. Build with [`tree`].
pub struct TreeBuilder {
    base: NodeBase,
    interaction_domain: Option<String>,
}

/// 🌲️ An empty tree, ready for [`tree_section`]/[`tree_item`] children.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn tree() -> TreeBuilder {
    TreeBuilder { base: NodeBase::stack(crate::Axis::Vertical), interaction_domain: None }
}

impl TreeBuilder {
    /// 🕹️ Binds this tree to an app-declared `InteractionDefinition` domain.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn interaction_domain(mut self, domain: impl Into<String>) -> Self {
        self.interaction_domain = Some(domain.into());
        self
    }
}

impl HasBase for TreeBuilder {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}
impl HasChildren for TreeBuilder {}
impl HasStackLayout for TreeBuilder {}

impl From<TreeBuilder> for BuiltNode {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(builder: TreeBuilder) -> Self {
        assemble(builder.base, crate::Component::Tree(crate::TreeProps { interaction_domain: builder.interaction_domain }))
    }
}

/// 🌲️ A labeled, collapsible grouping of tree items — `Component::TreeSection`. Build with
/// [`tree_section`].
pub struct TreeSectionBuilder {
    base: NodeBase,
    label: crate::Label,
    default_open: Option<bool>,
}

/// 🌲️ A tree section reading `label`, ready for [`tree_item`] children.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn tree_section(label: impl Into<crate::Label>) -> TreeSectionBuilder {
    TreeSectionBuilder { base: NodeBase::stack(crate::Axis::Vertical), label: label.into(), default_open: None }
}

impl TreeSectionBuilder {
    /// 🔽️ Sets whether the section starts open.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = Some(default_open);
        self
    }
}

impl HasBase for TreeSectionBuilder {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}
impl HasChildren for TreeSectionBuilder {}
impl HasStackLayout for TreeSectionBuilder {}

impl From<TreeSectionBuilder> for BuiltNode {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(builder: TreeSectionBuilder) -> Self {
        assemble(builder.base, crate::Component::TreeSection(crate::TreeSectionProps { label: Some(builder.label), default_open: builder.default_open }))
    }
}

/// 🌿️ A single tree row — `Component::TreeItem`. Nested items are its [`HasChildren::child`]ren.
/// Build with [`tree_item`].
pub struct TreeItemBuilder {
    base: NodeBase,
    label: crate::Label,
    description: Option<String>,
    icon: Option<String>,
    default_open: Option<bool>,
    draggable: Option<bool>,
    drag_data: Option<HashMap<String, String>>,
    dimmed: Option<bool>,
    row_actions: Vec<crate::RowAction>,
}

/// 🌿️ A tree row reading `label`. Its accessible name defaults to `label`, the same "hard to omit"
/// derivation [`button`] applies.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn tree_item(label: impl Into<crate::Label>) -> TreeItemBuilder {
    let label = label.into();
    let mut base = NodeBase::stack(crate::Axis::Vertical);
    base.accessibility.label = Some(label.clone());
    TreeItemBuilder { base, label, description: None, icon: None, default_open: None, draggable: None, drag_data: None, dimmed: None, row_actions: Vec::new() }
}

impl TreeItemBuilder {
    /// 📝️ Sets secondary description copy.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 🖼️ Sets the icon key.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// 🔽️ Sets whether the row starts open (when it has nested items).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = Some(default_open);
        self
    }

    /// 🖐️ Marks the row draggable.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn draggable(mut self, draggable: bool) -> Self {
        self.draggable = Some(draggable);
        self
    }

    /// 🏷️ Sets the payload a drag of this row carries.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn drag_data(mut self, drag_data: HashMap<String, String>) -> Self {
        self.drag_data = Some(drag_data);
        self
    }

    /// 👁️ Sets the domain "eye toggle" dimmed state — the row stays visible and clickable, just
    /// visually de-emphasized; not the same axis as [`HasBase::disabled`].
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn dimmed(mut self, dimmed: bool) -> Self {
        self.dimmed = Some(dimmed);
        self
    }

    /// 🎬️ Appends one row action.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn row_action(mut self, row_action: crate::RowAction) -> Self {
        self.row_actions.push(row_action);
        self
    }

    /// 🎬️ Appends every row action of `row_actions` in order.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn row_actions(mut self, row_actions: impl IntoIterator<Item = crate::RowAction>) -> Self {
        self.row_actions.extend(row_actions);
        self
    }
}

impl HasBase for TreeItemBuilder {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}
impl HasChildren for TreeItemBuilder {}
impl HasStackLayout for TreeItemBuilder {}

impl From<TreeItemBuilder> for BuiltNode {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
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
                row_actions: builder.row_actions,
            }),
        )
    }
}
//#endregion 🌲️Tree

//#region 🖼️Image
/// 🖼️ Typestate marker: [`ImageBuilder::alt`]/[`ImageBuilder::decorative`] not yet called. An
/// `ImageBuilder<NoAlt>` has no `build()` at all — there is no inherent `build` on this state, and the
/// blanket [`Buildable`] impl only applies to `T: Into<BuiltNode>`, which `ImageBuilder<NoAlt>` is not
/// (only `ImageBuilder<HasAlt>` implements [`From`] into [`BuiltNode`]). Calling `.build()` on this
/// state is therefore a COMPILE error (`E0599: no method named build`), never a runtime panic — see
/// this module's own accessible-UI mandate in [`crate`]'s docs and the compile-fail example below.
pub struct NoAlt;

/// 🖼️ Typestate marker: [`ImageBuilder::alt`] or [`ImageBuilder::decorative`] was called, so the image
/// carries a decided accessible name (either real alt text or an explicit decorative opt-out). Only
/// `ImageBuilder<HasAlt>` implements `Into<BuiltNode>`, which is what makes [`Buildable::build`]
/// available on it and not on `ImageBuilder<NoAlt>`.
pub struct HasAlt;

/// 🖼️ Whether an [`ImageBuilder`] carries real accessible text or has deliberately opted out —
/// `ImageBuilder<HasAlt>` is only ever constructed with this set to `Some`, by [`ImageBuilder::alt`] or
/// [`ImageBuilder::decorative`], so [`ImageBuilder`]'s own `From` impl can treat the `None` case as an
/// internal invariant rather than a user-reachable failure (see [`From<ImageBuilder<HasAlt>>`]).
enum ImageAlt {
    Text(crate::Label),
    Decorative,
}

/// 🖼️ An image — `Component::Image`. Build with [`image`], which returns `ImageBuilder<NoAlt>`; call
/// [`ImageBuilder::alt`] or [`ImageBuilder::decorative`] to obtain an `ImageBuilder<HasAlt>` before
/// [`ImageBuilder::build`]/[`Buildable::build`] is even callable — CLAUDE.md's accessible-UI mandate
/// makes an image's accessible name non-optional, and a **typestate**, not a runtime panic, is the
/// cheapest place to hold that line: a plugin that forgets `.alt(..)`/`.decorative()` fails to compile,
/// it never reaches a running actor to crash.
///
/// ```compile_fail
/// use semio_framework_ui_contract::*;
/// // ImageBuilder<NoAlt> has no `build()` — E0599, no method named `build` found.
/// let _ = image("atlas://logo").build();
/// ```
///
/// ```
/// use semio_framework_ui_contract::*;
/// // .alt(..)/.decorative() moves to ImageBuilder<HasAlt>, which DOES have `build()`.
/// let _ = image("atlas://logo").alt("Company logo").build();
/// let _ = image("atlas://deco").decorative().build();
/// ```
pub struct ImageBuilder<State = NoAlt> {
    base: NodeBase,
    src: String,
    alt: Option<ImageAlt>,
    _state: std::marker::PhantomData<State>,
}

/// 🖼️ An image loaded from `src`, in state [`NoAlt`] — call [`ImageBuilder::alt`] or
/// [`ImageBuilder::decorative`] before `.build()` becomes callable at all.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn image(src: impl Into<String>) -> ImageBuilder<NoAlt> {
    ImageBuilder { base: NodeBase::leaf(), src: src.into(), alt: None, _state: std::marker::PhantomData }
}

impl<State> ImageBuilder<State> {
    /// ♿️ Supplies the accessible alt text, unlocking `.build()` by transitioning to [`HasAlt`].
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn alt(self, alt: impl Into<crate::Label>) -> ImageBuilder<HasAlt> {
        ImageBuilder { base: self.base, src: self.src, alt: Some(ImageAlt::Text(alt.into())), _state: std::marker::PhantomData }
    }

    /// 🙈️ Explicitly opts out: the image is decorative and hidden from the accessibility tree.
    /// Unlocks `.build()` by transitioning to [`HasAlt`], the same as [`ImageBuilder::alt`].
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn decorative(self) -> ImageBuilder<HasAlt> {
        ImageBuilder { base: self.base, src: self.src, alt: Some(ImageAlt::Decorative), _state: std::marker::PhantomData }
    }
}

impl<State> HasBase for ImageBuilder<State> {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

impl From<ImageBuilder<HasAlt>> for BuiltNode {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(builder: ImageBuilder<HasAlt>) -> Self {
        let ImageBuilder { mut base, src, alt, .. } = builder;
        let alt = alt.expect("🚫️ ImageBuilder<HasAlt> is only ever constructed by .alt(..)/.decorative(), both of which set Some — an internal invariant, never a user-reachable panic");
        let alt_label = match alt {
            ImageAlt::Text(label) => {
                base.accessibility.label.get_or_insert_with(|| label.clone());
                Some(label)
            }
            ImageAlt::Decorative => {
                base.accessibility.hidden = true;
                None
            }
        };
        assemble(base, crate::Component::Image(crate::ImageProps { src, alt: alt_label }))
    }
}
//#endregion 🖼️Image

//#region 🗺️Surface
/// 🗺️ An embedded product surface — `Component::Surface`. Build with [`surface`].
pub struct SurfaceBuilder {
    base: NodeBase,
    props: crate::SurfaceProps,
}

/// 🗺️ A surface carrying `props` verbatim — every field of [`crate::SurfaceProps`] is the product's
/// own concern, not something this builder second-guesses.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn surface(props: crate::SurfaceProps) -> SurfaceBuilder {
    SurfaceBuilder { base: NodeBase::leaf(), props }
}

impl HasBase for SurfaceBuilder {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

impl From<SurfaceBuilder> for BuiltNode {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(builder: SurfaceBuilder) -> Self {
        assemble(builder.base, crate::Component::Surface(builder.props))
    }
}
//#endregion 🗺️Surface

//#region 🧩️Extension
/// 🧩️ An external plugin slot — `Component::Extension`. Build with [`extension`].
pub struct ExtensionBuilder {
    base: NodeBase,
    extension: String,
    props: crate::UiValue,
}

/// 🧩️ An extension slot addressed by `name`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn extension(name: impl Into<String>) -> ExtensionBuilder {
    ExtensionBuilder { base: NodeBase::leaf(), extension: name.into(), props: crate::UiValue::Null }
}

impl ExtensionBuilder {
    /// 📦️ Sets the opaque props payload passed to the slot.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn props(mut self, props: crate::UiValue) -> Self {
        self.props = props;
        self
    }
}

impl HasBase for ExtensionBuilder {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

impl From<ExtensionBuilder> for BuiltNode {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(builder: ExtensionBuilder) -> Self {
        assemble(builder.base, crate::Component::Extension(crate::ExtensionProps { extension: builder.extension, props: builder.props }))
    }
}
//#endregion 🧩️Extension

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖️WireCost
    #[test]
    fn button_serializes_to_minimal_json() {
        let node = button("Save").build();
        let json = serde_json::to_value(&node).expect("serialize");
        assert!(json.get("layout").is_none());
        assert!(json.get("style").is_none());
        assert!(json.get("disabled").is_none());
        assert!(json.get("bindings").is_none());
        assert!(json.get("menu").is_none());
        assert!(json.get("children").is_none());
        assert_eq!(json.get("key").and_then(|v| v.as_str()), Some("#0"));
        assert_eq!(json.get("component").and_then(|c| c.get("type")).and_then(|t| t.as_str()), Some("button"));
    }
    //#endregion 🔖️WireCost

    //#region 🔖️NestedShape
    #[test]
    fn nested_column_builds_expected_shape() {
        let node = column().children([text("A"), text("B")]).build();
        assert!(matches!(node.component, crate::Component::Container(crate::ContainerProps { role: crate::ContainerRole::Plain, .. })));
        assert!(matches!(node.layout, crate::LayoutSpec::Stack(crate::StackLayout { axis: crate::Axis::Vertical, .. })));
        assert_eq!(node.children.len(), 2);
        match &node.children[0].component {
            crate::Component::Text(props) => assert_eq!(props.value, crate::Label::from("A")),
            other => panic!("expected text, got {other:?}"),
        }
        match &node.children[1].component {
            crate::Component::Text(props) => assert_eq!(props.value, crate::Label::from("B")),
            other => panic!("expected text, got {other:?}"),
        }
    }

    #[test]
    fn mixed_child_types_nest_through_child() {
        let node = row().child(text("A")).child(button("Go")).build();
        assert_eq!(node.children.len(), 2);
        assert!(matches!(node.children[0].component, crate::Component::Text(_)));
        assert!(matches!(node.children[1].component, crate::Component::Button(_)));
    }
    //#endregion 🔖️NestedShape

    //#region 🔖️Bindings
    #[test]
    fn on_lands_in_bindings() {
        let action = crate::ActionId::v1("app", "save");
        let node = button("Save").on(crate::Trigger::Activate, action.clone()).build();
        assert_eq!(node.bindings.len(), 1);
        assert_eq!(node.bindings[0].trigger, crate::Trigger::Activate);
        assert_eq!(node.bindings[0].action, action);
        assert!(node.bindings[0].args.is_none());
    }

    #[test]
    fn on_with_carries_args() {
        let action = crate::ActionId::v1("app", "setValue");
        let node = input(crate::InputKind::Text).on_with(crate::Trigger::Change, action.clone(), crate::UiValue::Text("hi".into())).build();
        assert_eq!(node.bindings[0].args, Some(crate::UiValue::Text("hi".into())));
    }
    //#endregion 🔖️Bindings

    //#region 🔖️Accessibility
    #[test]
    fn button_auto_derives_accessibility_label_from_visible_label() {
        let node = button("Save").build();
        assert_eq!(node.accessibility.label, Some(crate::Label::from("Save")));
    }

    #[test]
    fn explicit_label_overrides_auto_derived_accessibility_label() {
        let node = button("Save").label("Save the document").build();
        assert_eq!(node.accessibility.label, Some(crate::Label::from("Save the document")));
    }

    /// 🚫️ `image(..)` without `.alt(..)`/`.decorative()` is a COMPILE error now (see the `compile_fail`
    /// doctest on [`ImageBuilder`] itself), not a runtime panic — `ImageBuilder<NoAlt>` has no `build()`
    /// at all, so there is nothing for a `#[test]` here to call. This comment stands in its place so the
    /// next reader finds the negative case instead of assuming it was dropped.
    #[test]
    fn image_builder_no_alt_state_has_no_build_method_verified_by_the_type_doc_compile_fail_test() {
        let _: ImageBuilder<NoAlt> = image("atlas://logo");
    }

    #[test]
    fn image_decorative_hides_from_accessibility_tree_and_omits_alt() {
        let node = image("atlas://deco").decorative().build();
        assert!(node.accessibility.hidden);
        match node.component {
            crate::Component::Image(props) => assert!(props.alt.is_none()),
            other => panic!("expected image, got {other:?}"),
        }
    }

    #[test]
    fn image_alt_populates_component_and_accessibility() {
        let node = image("atlas://logo").alt("Company logo").build();
        assert_eq!(node.accessibility.label, Some(crate::Label::from("Company logo")));
        match node.component {
            crate::Component::Image(props) => assert_eq!(props.alt, Some(crate::Label::from("Company logo"))),
            other => panic!("expected image, got {other:?}"),
        }
    }
    //#endregion 🔖️Accessibility

    //#region 🔖️Keys
    #[test]
    fn positional_keys_are_stable_and_distinct_among_siblings() {
        let build = || column().children([text("A"), text("B"), text("C")]).build();
        let first = build();
        let second = build();
        let keys: Vec<&str> = first.children.iter().map(|child| child.key.as_str()).collect();
        assert_eq!(keys, vec!["#0", "#1", "#2"]);
        let second_keys: Vec<&str> = second.children.iter().map(|child| child.key.as_str()).collect();
        assert_eq!(keys, second_keys);
        let unique: std::collections::HashSet<&str> = keys.into_iter().collect();
        assert_eq!(unique.len(), 3);
    }

    #[test]
    fn explicit_id_overrides_positional_key() {
        let node = column().child(text("A").id("first")).child(text("B")).build();
        assert_eq!(node.children[0].key, "first");
        assert_eq!(node.children[1].key, "#1");
    }
    //#endregion 🔖️Keys
}
//#endregion 🧪️Tests

//#endregion 🔖️Builder
