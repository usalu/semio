//! 🔧️ Builder traits: the `HasBase`, `HasChildren`, `HasStackLayout` and `Buildable` traits that every builder implements. See [`super`].

use super::*;

//#region 🔖️Builder
//#region 🔧️Traits
/// 🔧️ The chainable vocabulary every builder in this file carries — identity, style, visual state,
/// actions, and accessibility overrides. Set [`HasBase::id`] explicitly for any node whose position
/// among its siblings can change: the positional fallback key ([`HasChildren::child`]) is what
/// preserves a node's state across a re-present, and it only stays stable while sibling order and
/// count do not.
pub trait HasBase: Sized {
    /// 🔩️ Internal accessor — never call directly; use the chainable methods below.
    #[doc(hidden)]
    fn base_mut(&mut self) -> &mut NodeBase;

    /// 🔑️ Sets the reconciliation key explicitly.
    fn try_id(mut self, id: impl AsRef<str>) -> Result<Self, Self> {
        let Some(id) = crate::UiText::try_from_str(id.as_ref()) else { return Err(self) };
        self.base_mut().id = Some(id);
        Ok(self)
    }

    /// 🚫️ Marks the node non-interactive without removing it from the tree.
    fn disabled(mut self, disabled: bool) -> Self {
        self.base_mut().disabled = disabled;
        self
    }

    /// 🧭️ Sets the node's [`crate::Activity`] lifecycle state.
    fn activity(mut self, activity: crate::Activity) -> Self {
        self.base_mut().activity = activity;
        self
    }

    /// 🎨️ Overrides the node's design-token [`crate::StyleSpec`] wholesale.
    fn style(mut self, style: crate::StyleSpec) -> Self {
        self.base_mut().style = style;
        self
    }

    /// 🎨️ Sets the [`crate::Tone`] color role.
    fn tone(mut self, tone: crate::Tone) -> Self {
        self.base_mut().style.tone = tone;
        self
    }

    /// 🖌️ Sets the [`crate::Variant`] chrome treatment.
    fn variant(mut self, variant: crate::Variant) -> Self {
        self.base_mut().style.variant = variant;
        self
    }

    /// 📏️ Sets the [`crate::SizeToken`] t-shirt size.
    fn size(mut self, size: crate::SizeToken) -> Self {
        self.base_mut().style.size = size;
        self
    }

    /// 🔆️ Sets the [`crate::Emphasis`] visual prominence.
    fn emphasis(mut self, emphasis: crate::Emphasis) -> Self {
        self.base_mut().style.emphasis = emphasis;
        self
    }

    /// 📐️ Sets the [`crate::Density`] spacing.
    fn density(mut self, density: crate::Density) -> Self {
        self.base_mut().style.density = density;
        self
    }

    /// 📋️ Attaches a resolved context menu.
    fn menu(mut self, menu: crate::MenuRef) -> Self {
        self.base_mut().menu = Some(menu);
        self
    }

    /// 🎬️ Binds `trigger` to `action` with no args.
    #[expect(clippy::result_large_err, reason = "A full binding list returns the builder and exact binding without allocating for rejection.")]
    fn try_on(mut self, trigger: crate::Trigger, action: crate::ActionId) -> Result<Self, (Self, crate::ActionBinding)> {
        let binding = crate::ActionBinding { trigger, action, args: None, capability: None };
        match self.base_mut().bindings.try_push(binding) {
            Ok(()) => Ok(self),
            Err(binding) => Err((self, binding)),
        }
    }

    /// 🎬️ Binds `trigger` to `action`, carrying `args` for the action to consume.
    #[expect(clippy::result_large_err, reason = "A refused binding retains the builder and argument owner together for caller-directed retirement.")]
    fn try_on_with(mut self, trigger: crate::Trigger, action: crate::ActionId, args: crate::UiValue) -> Result<Self, (Self, crate::ActionBinding)> {
        let binding = crate::ActionBinding { trigger, action, args: Some(args), capability: None };
        match self.base_mut().bindings.try_push(binding) {
            Ok(()) => Ok(self),
            Err(binding) => Err((self, binding)),
        }
    }

    /// ♿️ Overrides the node's accessible name. [`button`] and [`tree_item`] already derive this from
    /// their visible label; call this only to override that default.
    fn try_label(mut self, label: impl AsRef<str>) -> Result<Self, Self> {
        let Some(label) = crate::UiText::try_from_str(label.as_ref()).map(crate::Label) else { return Err(self) };
        self.base_mut().accessibility.label = Some(label);
        Ok(self)
    }

    /// ♿️ Sets the node's accessible description.
    fn try_describe(mut self, description: impl AsRef<str>) -> Result<Self, Self> {
        let Some(description) = crate::UiText::try_from_str(description.as_ref()).map(crate::Label) else { return Err(self) };
        self.base_mut().accessibility.description = Some(description);
        Ok(self)
    }

    /// 📢️ Makes the node a live region: a reader announces its text whenever it changes, without the
    /// reader ever having to reach the node. This is the ONLY way a background state change — an
    /// evaluation going `Computing → Evaluated`, a progress phase advancing — is heard at all, so a
    /// status surface that never calls this is silent by construction.
    fn live(mut self, live: crate::Liveness) -> Self {
        self.base_mut().accessibility.live = live;
        self
    }

    /// ⌨️ Declares the keyboard shortcut that reaches this node, announced by a reader and mirrored
    /// into `aria-keyshortcuts`. Declaring it here never BINDS it — the binding stays with whoever
    /// owns the keybinding — so the two can be checked against each other.
    fn try_shortcut(mut self, shortcut: impl AsRef<str>) -> Result<Self, Self> {
        let Some(shortcut) = crate::UiText::try_from_str(shortcut.as_ref()) else { return Err(self) };
        self.base_mut().accessibility.shortcut = Some(shortcut);
        Ok(self)
    }
}

/// 👶️ Adds `.child(..)`/`.children(..)` to a builder whose component genuinely nests others.
pub trait HasChildren: HasBase {
    /// 👶️ Appends one already-built or still-buildable child. An omitted child key becomes
    /// `"#{position}"` at the moment it is pushed here — stable only while the number and order of its
    /// siblings do not change, which is why a node whose position can shift (a reorderable list row,
    /// for instance) should carry an explicit [`HasBase::id`] instead.
    #[expect(clippy::result_large_err, reason = "The rejected child and partial builder retain their original ownership without an extra error allocation.")]
    fn try_child(mut self, child: impl Into<BuiltNode>) -> Result<Self, (Self, BuiltNode)> {
        let mut node = child.into();
        if node.key.is_empty() {
            let Some(key) = positional_key(self.base_mut().children.len()) else { return Err((self, node)) };
            node.key = key;
        }
        if let Err(node) = self.base_mut().children.try_push(node) {
            return Err((self, node));
        }
        Ok(self)
    }

    #[expect(clippy::result_large_err, reason = "A rejected child preserves the exact builder, rejected node, and remaining iterator without allocating.")]
    fn try_children<T: Into<BuiltNode>, I: IntoIterator<Item = T>>(mut self, children: I) -> Result<Self, RetainedChildren<Self, I::IntoIter>> {
        let mut remaining = children.into_iter();
        while let Some(child) = remaining.next() {
            match self.try_child(child) {
                Ok(builder) => self = builder,
                Err((builder, rejected)) => return Err(RetainedChildren { builder, rejected, remaining }),
            }
        }
        Ok(self)
    }

    #[cfg(test)]
    fn child(self, child: impl Into<BuiltNode>) -> Self {
        self.try_child(child).unwrap_or_else(|(builder, _)| builder)
    }

    #[cfg(test)]
    fn children<T: Into<BuiltNode>, I: IntoIterator<Item = T>>(self, children: I) -> Self {
        self.try_children(children).unwrap_or_else(|error| error.builder)
    }
}

pub struct RetainedChildren<B, I> {
    pub builder: B,
    pub rejected: BuiltNode,
    pub remaining: I,
}

/// 📏️ Adds [`crate::StackLayout`] tuning to a builder whose default layout is
/// [`crate::LayoutSpec::Stack`] — true of every implementor in this file, so these methods never fall
/// through to a no-op in practice.
pub trait HasStackLayout: HasBase {
    /// ↔️ Sets the gap between children.
    fn gap(mut self, gap: crate::SpaceToken) -> Self {
        if let crate::LayoutSpec::Stack(stack) = &mut self.base_mut().layout {
            stack.gap = gap;
        }
        self
    }

    /// 📐️ Sets the container's own padding.
    fn padding(mut self, padding: crate::EdgeSpace) -> Self {
        if let crate::LayoutSpec::Stack(stack) = &mut self.base_mut().layout {
            stack.padding = padding;
        }
        self
    }

    /// ↕️ Sets cross-axis alignment.
    fn align(mut self, align: crate::Align) -> Self {
        if let crate::LayoutSpec::Stack(stack) = &mut self.base_mut().layout {
            stack.align = align;
        }
        self
    }

    /// ↔️ Sets main-axis distribution.
    fn justify(mut self, justify: crate::Justify) -> Self {
        if let crate::LayoutSpec::Stack(stack) = &mut self.base_mut().layout {
            stack.justify = justify;
        }
        self
    }

    /// 📈️ Lets the stack grow to fill its parent's remaining main-axis space.
    fn grow(mut self, grow: bool) -> Self {
        if let crate::LayoutSpec::Stack(stack) = &mut self.base_mut().layout {
            stack.grow = grow;
        }
        self
    }

    /// 🔁️ Permits children to wrap onto additional lines.
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
/// so this trait — and therefore `.try_build()` — is simply absent from `ImageBuilder<NoAlt>`'s method set
/// at compile time, rather than present but panicking.
pub trait Buildable: Into<BuiltNode> {
    #[expect(clippy::result_large_err, reason = "A failed positional-key assignment returns the already-built node directly for explicit retirement.")]
    fn try_build(self) -> Result<BuiltNode, BuiltNode>
    where
        Self: Sized,
    {
        let mut node: BuiltNode = self.into();
        if node.key.is_empty() {
            let Some(key) = positional_key(0) else { return Err(node) };
            node.key = key;
        }
        Ok(node)
    }
}

impl<T: Into<BuiltNode>> Buildable for T {}
//#endregion 🔧️Traits
//#endregion 🔖️Builder
