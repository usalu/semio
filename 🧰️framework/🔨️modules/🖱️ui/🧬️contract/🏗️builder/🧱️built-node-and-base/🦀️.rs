//! 🧱️ Built node and base: the serde default predicates, the `BuiltNode` with its accessors, the shared `NodeBase` and the positional key and node assembly. See [`super`].

use super::*;

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

/// 🧬️ The contract-local shape a builder terminates into — everything a [`crate::UiNodeRecord`]
/// carries except `id` (minted by the runtime at reconciliation, never by an author) and with
/// `children` nested inline rather than addressed by [`crate::UiNodeId`], since a freshly authored
/// tree has no ids yet to address by. Every field below serializes away at its default, mirroring the
/// wire-cost guarantee this whole builder family exists to make automatic.
// 🌱️ No `ToValue`/`FromValue` here (ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-
// ARTIFACTS): `component: crate::Component` (and `bindings`/`menu`) embed `UiValue`, the deliberate
// DslValue-free exception — see its docstring in `🎬️action.rs`.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuiltNode {
    /// 🔑️ The reconciliation key — an author-set [`HasBase::id`] or a positional `"#N"` fallback. See
    /// [`HasChildren::child`] for why the fallback is only stable while sibling order is unchanged.
    pub key: crate::UiText,
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
    #[serde(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    pub bindings: crate::UiNodeBindings,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub menu: Option<crate::MenuRef>,
    #[serde(default, skip_serializing_if = "BuiltChildren::is_empty")]
    pub children: BuiltChildren,
    #[serde(skip)]
    pub rejected_children: BuiltChildren,
}

impl BuiltNode {
    #[expect(clippy::result_large_err, reason = "An oversized key returns its component unchanged without introducing an allocation on the error path.")]
    pub fn try_new(key: impl AsRef<str>, component: crate::Component) -> Result<Self, crate::Component> {
        let Some(key) = crate::UiText::try_from_str(key.as_ref()) else { return Err(component) };
        Ok(Self {
            key,
            component,
            layout: crate::LayoutSpec::default(),
            style: crate::StyleSpec::default(),
            activity: crate::Activity::default(),
            disabled: false,
            accessibility: crate::AccessibilitySpec::default(),
            bindings: crate::UiNodeBindings::default(),
            menu: None,
            children: BuiltChildren::default(),
            rejected_children: BuiltChildren::default(),
        })
    }

    #[expect(clippy::result_large_err, reason = "A rejected positional identity returns the original component without a new heap owner.")]
    pub fn try_at(position: usize, component: crate::Component) -> Result<Self, crate::Component> {
        let Some(key) = positional_key(position) else { return Err(component) };
        Self::try_new(key, component)
    }

    /// 🧾️ A second owner of this whole subtree, credited the same way [`crate::Component::credited_clone`]
    /// credits an embedded [`crate::UiValue`] — the retained read a memoizing author needs in order to
    /// hand a cached tree back out. NOT a `Clone` impl: [`BuiltChildren`] owns a retirement reservation
    /// that must be reserved per owner, so a derived `Clone` would publish one slot twice.
    pub fn credited_clone(&self) -> Option<Self> {
        Some(Self {
            key: self.key.clone(),
            component: self.component.credited_clone()?,
            layout: self.layout.clone(),
            style: self.style,
            activity: self.activity,
            disabled: self.disabled,
            accessibility: self.accessibility.clone(),
            bindings: credited_bindings(&self.bindings)?,
            menu: match self.menu.as_ref() {
                Some(menu) => Some(menu.credited_clone()?),
                None => None,
            },
            children: self.children.credited_clone()?,
            rejected_children: self.rejected_children.credited_clone()?,
        })
    }

    pub fn empty_separator() -> Self {
        Self {
            key: crate::UiText::default(),
            component: crate::Component::Separator(crate::SeparatorProps {}),
            layout: crate::LayoutSpec::default(),
            style: crate::StyleSpec::default(),
            activity: crate::Activity::default(),
            disabled: false,
            accessibility: crate::AccessibilitySpec::default(),
            bindings: crate::UiNodeBindings::default(),
            menu: None,
            children: BuiltChildren::default(),
            rejected_children: BuiltChildren::default(),
        }
    }

    #[expect(clippy::result_large_err, reason = "Child admission hands back the partial parent and exact rejected child for explicit retirement.")]
    pub fn try_with_children(mut self, children: impl IntoIterator<Item = BuiltNode>) -> Result<Self, (Self, BuiltNode)> {
        for mut child in children {
            if child.key.is_empty() {
                let Some(key) = positional_key(self.children.len()) else { return Err((self, child)) };
                child.key = key;
            }
            if let Err(child) = self.children.try_push(child) {
                return Err((self, child));
            }
        }
        Ok(self)
    }

    #[cfg(test)]
    pub fn with_children(self, children: impl IntoIterator<Item = BuiltNode>) -> Self {
        self.try_with_children(children).unwrap_or_else(|(node, _)| node)
    }
}
//#endregion 🧱️BuiltNode

//#region 🧩️Base
/// 🧰️ Record-level state shared by every builder in this file — reachable only through the chainable
/// methods on [`HasBase`]/[`HasChildren`]/[`HasStackLayout`], never through its own fields, which stay
/// private to this module.
pub struct NodeBase {
    pub(super) id: Option<crate::UiText>,
    pub(super) layout: crate::LayoutSpec,
    pub(super) style: crate::StyleSpec,
    pub(super) activity: crate::Activity,
    pub(super) disabled: bool,
    pub(super) accessibility: crate::AccessibilitySpec,
    pub(super) bindings: crate::UiNodeBindings,
    pub(super) menu: Option<crate::MenuRef>,
    pub(super) children: BuiltChildren,
    pub(super) rejected_children: BuiltChildren,
}

impl NodeBase {
    fn with_layout(layout: crate::LayoutSpec) -> Self {
        Self {
            id: None,
            layout,
            style: crate::StyleSpec::default(),
            activity: crate::Activity::default(),
            disabled: false,
            accessibility: crate::AccessibilitySpec::default(),
            bindings: crate::UiNodeBindings::default(),
            menu: None,
            children: BuiltChildren::default(),
            rejected_children: BuiltChildren::default(),
        }
    }

    /// 🍃️ A terminal node's own base — [`crate::LayoutSpec::Leaf`], the crate's own default.
    pub(super) fn leaf() -> Self {
        Self::with_layout(crate::LayoutSpec::default())
    }

    /// 📚️ A children-arranging node's own base — [`crate::LayoutSpec::Stack`] along `axis`.
    pub(super) fn stack(axis: crate::Axis) -> Self {
        Self::with_layout(crate::LayoutSpec::Stack(crate::StackLayout { axis, ..Default::default() }))
    }
}

/// 🔢️ The fallback key for a node at `index` among its siblings — stable only while sibling count
/// and order stay fixed; see [`HasChildren::child`].
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(super) fn positional_key(index: usize) -> Option<crate::UiText> {
    crate::UiText::try_format(format_args!("#{index}"))
}

/// 🏗️ Assembles `base` and `component` into a [`BuiltNode`], leaving `key` empty when `base.id` was
/// never set — the caller (either [`Buildable::build`] or [`HasChildren::child`]) fills that sentinel
/// with the right default for its position.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(super) fn assemble(base: NodeBase, component: crate::Component) -> BuiltNode {
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
        rejected_children: base.rejected_children,
    }
}
//#endregion 🧩️Base
//#endregion 🔖️Builder
