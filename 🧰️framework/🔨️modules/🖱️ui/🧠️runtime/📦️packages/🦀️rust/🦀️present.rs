//! @emoji 🎭️ The `Present` trait and the keyed `ComponentTree` a presenter builds.
//!
//! `TreeNode` is recursive and that is deliberate — unlike `🧬️contract`'s flat, id-keyed
//! `UiNodeRecord` table, this tree is builder-side and internal to this crate, and it never crosses
//! the wire. A presenter is free to re-run from scratch on every present, so this crate never invents
//! an id: sibling `key`s are the only identity a `TreeNode` carries, and it is packet
//! `runtime-reconcile`'s job to diff two `ComponentTree`s and assign `ui_contract::UiNodeId`s to the
//! result, never this file's.
//!
//! [`PresentCx::read`] is the ONLY way a presenter reads entity state, which is what makes
//! [`crate::DependencyTracker`]'s actual-read tracking automatic rather than declared: a presenter
//! never lists its dependencies, it just reads through `cx` and the right edges fall out.
//!
//! 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
//! below is plain sync by owner ruling U1, which supersedes this program's general async-everything
//! default for exactly this crate.

use std::collections::HashSet;

//#region 🔖️Present
/// 🎭️ Something that presents a [`ComponentTree`] by reading entity state through `cx`. Consumed
/// generically (`fn drive<P: Present>(p: &P, ...)`), never as `dyn Present` — ruling U3 bans `dyn` on
/// a first-party trait, and a coordinator wanting heterogeneous presenters in one collection needs an
/// enum-dispatch or fn-pointer-vtable seam, not a trait object here.
pub trait Present: 'static {
    /// 🖼️ Builds this present's [`ComponentTree`] for the current frame, recording every entity it
    /// actually reads via `cx`. Free to re-run in full on every present — nothing here is expected to
    /// diff against the previous tree; that is `runtime-reconcile`'s job downstream.
    fn present(&self, cx: &mut PresentCx<'_>) -> ComponentTree;
}

/// 🪶️ The stateless variant: any capture-light `Fn(&mut PresentCx<'_>) -> ComponentTree` — a plain fn
/// item or a closure — satisfies [`Present`] directly, with no wrapper struct needed. This is the
/// common case for a screen that owns no persistent fields beyond the entities it reads each present.
impl<F> Present for F
where
    F: Fn(&mut PresentCx<'_>) -> ComponentTree + 'static,
{
    fn present(&self, cx: &mut PresentCx<'_>) -> ComponentTree {
        self(cx)
    }
}

/// 🔭️ The context a [`Present::present`] reads through. Reads recorded here land on whichever
/// [`crate::DependencyTracker`] scope is innermost at the time — opened and closed around the
/// `present()` call by whichever caller drives it (`runtime-reconcile`/`runtime-transact`), never by
/// this file, so a present nested inside another present's own `begin`/`finish` pair attributes its
/// reads to its own surface automatically.
///
/// Wraps `&EntityStore`, not `runtime-entity`'s `Context<'a, T>` — that type is the per-lease
/// mutation-effect handle `EntityStore::update` hands to a closure and deliberately has no store
/// read access at all (so a lease can never be read around); presenting is a read-only traversal
/// across many entities of many types outside any lease, which is exactly `EntityStore::read`'s job.
pub struct PresentCx<'a> {
    tracker: &'a mut crate::DependencyTracker,
    store: &'a crate::EntityStore,
}

impl<'a> PresentCx<'a> {
    /// 🏗️ Wraps the tracker whose innermost open scope should receive this present's reads, plus the
    /// entity store those reads actually resolve against. The caller is expected to have already
    /// called `tracker.begin(surface)` before constructing this and to call `tracker.finish(surface)`
    /// immediately after `present()` returns.
    pub fn new(tracker: &'a mut crate::DependencyTracker, store: &'a crate::EntityStore) -> Self {
        Self { tracker, store }
    }

    /// 👁️ The ONLY way a presenter reads state — this is what makes actual-read tracking automatic:
    /// every call records `entity`'s id against the innermost open present scope, then resolves the
    /// actual value through the store. A read performed anywhere else (an event handler calling
    /// `crate::Entity::read` directly, say) never reaches a tracker at all and so can never become a
    /// frame dependency.
    pub fn read<T: 'static>(&mut self, entity: &crate::Entity<T>) -> &'a T {
        self.tracker.record_read(entity.id());
        entity.read(self.store)
    }
}
//#endregion 🔖️Present

//#region 🔖️ComponentTree
/// 🌳️ One node of the builder-side presentation tree a [`Present::present`] returns. Recursive by
/// design (see the module doc); children are addressed positionally through `children`, never through
/// an id, since no id exists yet at this stage.
#[derive(Clone, Debug, PartialEq)]
pub struct TreeNode {
    pub key: String,
    pub component: ui_contract::Component,
    pub layout: ui_contract::LayoutSpec,
    pub style: ui_contract::StyleSpec,
    pub activity: ui_contract::Activity,
    pub disabled: bool,
    pub accessibility: ui_contract::AccessibilitySpec,
    pub bindings: Vec<ui_contract::ActionBinding>,
    pub menu: Option<ui_contract::MenuRef>,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    /// 🌱️ A node named `key` rendering `component`, every other field at its structural default —
    /// the common case a presenter then customizes field-by-field via ordinary struct-update syntax.
    pub fn new(key: impl Into<String>, component: ui_contract::Component) -> Self {
        Self {
            key: key.into(),
            component,
            layout: ui_contract::LayoutSpec::default(),
            style: ui_contract::StyleSpec::default(),
            activity: ui_contract::Activity::default(),
            disabled: false,
            accessibility: ui_contract::AccessibilitySpec::default(),
            bindings: Vec::new(),
            menu: None,
            children: Vec::new(),
        }
    }

    /// 🔢️ Like [`Self::new`], but with the key defaulted from `position` via [`position_key`] — the
    /// ergonomic path for `Vec::iter().enumerate().map(...)` over children with no natural identity
    /// of their own.
    pub fn at(position: usize, component: ui_contract::Component) -> Self {
        Self::new(position_key(position), component)
    }

    /// 👶️ Attaches `children`, asserting their keys are unique among themselves — a repeated sibling
    /// key is a real authoring bug that otherwise only shows up later as mysterious state loss during
    /// reconciliation, so this fails loudly here instead.
    pub fn with_children(mut self, children: impl IntoIterator<Item = TreeNode>) -> Self {
        self.children = children.into_iter().collect();
        assert_unique_sibling_keys(&self.children);
        self
    }
}

/// 🔑️ The default sibling key for a child at `position` when its author supplies none — stable as
/// long as sibling order itself is stable, the same assumption any index-keyed list already makes.
pub fn position_key(position: usize) -> String {
    format!("#{position}")
}

/// 🚨️ Panics naming the first duplicate sibling key found in `children`. A repeated key is an
/// authoring bug, not a runtime condition to route around — silently keeping the first occurrence
/// would only defer the failure to a much harder-to-diagnose spot downstream in reconciliation.
pub fn assert_unique_sibling_keys(children: &[TreeNode]) {
    let mut seen = HashSet::with_capacity(children.len());
    for child in children {
        assert!(seen.insert(child.key.as_str()), "duplicate sibling key {:?}", child.key);
    }
}

/// 🌳️ The complete presentation tree one [`Present::present`] call produced.
#[derive(Clone, Debug, PartialEq)]
pub struct ComponentTree {
    pub root: TreeNode,
}

impl ComponentTree {
    /// 🏗️ Wraps `root` as a complete tree, asserting every level's sibling keys are unique via an
    /// explicit stack — no recursive call, matching the non-recursive traversal style
    /// `🧬️contract::UiSnapshotState::iter_subtree` already uses for the same reason. Catches a
    /// duplicate key anywhere in the tree, not only among `root`'s immediate children.
    pub fn new(root: TreeNode) -> Self {
        let mut stack = vec![&root];
        while let Some(node) = stack.pop() {
            assert_unique_sibling_keys(&node.children);
            stack.extend(node.children.iter());
        }
        Self { root }
    }
}
//#endregion 🔖️ComponentTree

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn leaf(key: &str) -> TreeNode {
        TreeNode::new(key, ui_contract::Component::Separator(ui_contract::SeparatorProps {}))
    }

    #[test]
    fn duplicate_sibling_keys_via_with_children_panic() {
        let result = std::panic::catch_unwind(|| leaf("root").with_children([leaf("a"), leaf("a")]));
        assert!(result.is_err());
    }

    #[test]
    fn duplicate_sibling_keys_nested_in_the_tree_are_detected() {
        let grandchildren = [leaf("x"), leaf("x")];
        let child = leaf("child");
        let root = leaf("root").with_children([child]);
        let result = std::panic::catch_unwind(|| {
            let mut root = root.clone();
            root.children[0].children = grandchildren.to_vec();
            ComponentTree::new(root)
        });
        assert!(result.is_err());
    }

    #[test]
    fn position_derived_keys_are_stable_and_ergonomic() {
        let children: Vec<TreeNode> = (0..3).map(|i| TreeNode::at(i, ui_contract::Component::Separator(ui_contract::SeparatorProps {}))).collect();
        assert_eq!(children[0].key, "#0");
        assert_eq!(children[1].key, "#1");
        assert_eq!(children[2].key, "#2");
        let tree = ComponentTree::new(leaf("root").with_children(children));
        assert_eq!(tree.root.children.len(), 3);
    }

    #[test]
    fn component_tree_three_levels_builds_and_compares() {
        let grandchild = TreeNode::new("grandchild", ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label::from("leaf"), emphasize: None, data_attributes: None }));
        let child =
            TreeNode::new("child", ui_contract::Component::Container(ui_contract::ContainerProps { role: ui_contract::ContainerRole::Group, label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }))
                .with_children([grandchild.clone()]);
        let root = TreeNode::new("root", ui_contract::Component::Container(ui_contract::ContainerProps { role: ui_contract::ContainerRole::Plain, label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }))
            .with_children([child.clone()]);

        let tree = ComponentTree::new(root.clone());
        assert_eq!(tree.root, root);
        assert_eq!(tree.root.children.len(), 1);
        assert_eq!(tree.root.children[0], child);
        assert_eq!(tree.root.children[0].children[0], grandchild);
        assert_eq!(tree.root.children[0].children[0].children.len(), 0);

        let same_shape = ComponentTree::new(root);
        assert_eq!(tree, same_shape);
    }

    fn _accepts_any_present<P: Present>(_p: &P) {}

    #[test]
    fn a_stateless_fn_item_satisfies_present_generically() {
        fn screen(_cx: &mut PresentCx<'_>) -> ComponentTree {
            ComponentTree::new(leaf("root"))
        }
        _accepts_any_present(&screen);
    }
}
//#endregion 🧪️Tests
