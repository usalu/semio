//! @emoji 🎭️ The `Present` trait and the keyed `ComponentTree` a presenter builds.
//!
//! `TreeNode` is the contract crate's fixed-page `BuiltNode`; child shape is an admitted arena handle,
//! never inline recursion. The retained producer below visits one field, child, or duplicate-key
//! comparison per opportunity and publishes only after the complete candidate has passed its census.
//!
//! [`PresentCx::read`] is the ONLY way a presenter reads entity state, which is what makes
//! [`crate::DependencyTracker`]'s actual-read tracking automatic rather than declared: a presenter
//! never lists its dependencies, it just reads through `cx` and the right edges fall out.
//!
//! 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
//! below is plain sync by owner ruling U1, which supersedes this program's general async-everything
//! default for exactly this crate.

use std::mem::take;

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
pub use ui_contract::BuiltNode as TreeNode;

pub fn position_key(position: usize) -> Option<ui_contract::UiText> {
    ui_contract::UiText::try_format(format_args!("#{position}"))
}

#[derive(Debug)]
pub struct ComponentTree {
    pub root: TreeNode,
}

impl ComponentTree {
    #[cfg(test)]
    pub fn new(root: TreeNode) -> Self {
        Self { root }
    }
}

pub const COMPONENT_TREE_PRODUCER_DEPTH: usize = 64;
const COMPONENT_TREE_NODE_FIELDS: u8 = 11;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentTreeProducerFault {
    Cancelled,
    Deadline,
    DuplicateSiblingKey,
    Generation { expected: u64, actual: u64 },
    IdentifierBytes,
    NodeCapacity,
    NodeDepth,
    RejectedBacking,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentTreeProducerStep {
    MoreWork,
    Complete,
    Fault(ComponentTreeProducerFault),
}

struct ComponentTreeProducerFrame {
    node: TreeNode,
    source: ui_contract::BuiltChildrenIntoIter,
    admitted: ui_contract::BuiltChildren,
    pending: Option<TreeNode>,
    compare: usize,
    field: u8,
}

impl ComponentTreeProducerFrame {
    fn new(mut node: TreeNode) -> Self {
        let source = take(&mut node.children).into_iter();
        Self { node, source, admitted: ui_contract::BuiltChildren::default(), pending: None, compare: 0, field: 0 }
    }
}

pub struct ComponentTreeProducer {
    generation: u64,
    stack: ui_contract::UiFixedList<ComponentTreeProducerFrame, COMPONENT_TREE_PRODUCER_DEPTH>,
    complete: Option<ComponentTree>,
    overflow: Option<TreeNode>,
    fault: Option<ComponentTreeProducerFault>,
    nodes: usize,
}

impl ComponentTreeProducer {
    pub fn try_new(root: TreeNode, generation: u64) -> Result<Self, TreeNode> {
        if generation == 0 {
            return Err(root);
        }
        let mut stack = ui_contract::UiFixedList::default();
        if let Err(frame) = stack.try_push(ComponentTreeProducerFrame::new(root)) {
            return Err(frame.node);
        }
        Ok(Self { generation, stack, complete: None, overflow: None, fault: None, nodes: 0 })
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn step(&mut self, generation: u64, cancelled: bool, deadline_expired: bool) -> ComponentTreeProducerStep {
        if let Some(fault) = self.fault {
            return ComponentTreeProducerStep::Fault(fault);
        }
        if generation != self.generation {
            let fault = ComponentTreeProducerFault::Generation { expected: self.generation, actual: generation };
            self.fault = Some(fault);
            return ComponentTreeProducerStep::Fault(fault);
        }
        if cancelled {
            self.fault = Some(ComponentTreeProducerFault::Cancelled);
            return ComponentTreeProducerStep::Fault(ComponentTreeProducerFault::Cancelled);
        }
        if deadline_expired {
            self.fault = Some(ComponentTreeProducerFault::Deadline);
            return ComponentTreeProducerStep::Fault(ComponentTreeProducerFault::Deadline);
        }
        let Some(frame) = self.stack.last_mut() else { return ComponentTreeProducerStep::Complete };
        if !frame.node.rejected_children.is_empty() {
            self.fault = Some(ComponentTreeProducerFault::RejectedBacking);
            return ComponentTreeProducerStep::Fault(ComponentTreeProducerFault::RejectedBacking);
        }
        if frame.field < COMPONENT_TREE_NODE_FIELDS {
            if frame.field == 0 && frame.node.key.len() > ui_contract::UI_TEXT_MAX_BYTES {
                self.fault = Some(ComponentTreeProducerFault::IdentifierBytes);
                return ComponentTreeProducerStep::Fault(ComponentTreeProducerFault::IdentifierBytes);
            }
            frame.field += 1;
            return ComponentTreeProducerStep::MoreWork;
        }
        if frame.pending.is_none() {
            if let Some(child) = frame.source.next() {
                frame.pending = Some(child);
                frame.compare = 0;
                return ComponentTreeProducerStep::MoreWork;
            }
            let Some(mut complete) = self.stack.pop() else { return ComponentTreeProducerStep::Complete };
            complete.node.children = complete.admitted;
            self.nodes = match self.nodes.checked_add(1) {
                Some(nodes) if nodes <= ui_contract::UI_BUILT_CHILD_RETIRE_SLOTS => nodes,
                _ => {
                    self.overflow = Some(complete.node);
                    self.fault = Some(ComponentTreeProducerFault::NodeCapacity);
                    return ComponentTreeProducerStep::Fault(ComponentTreeProducerFault::NodeCapacity);
                }
            };
            if let Some(parent) = self.stack.last_mut() {
                if let Err(node) = parent.admitted.try_push(complete.node) {
                    parent.pending = Some(node);
                    self.fault = Some(ComponentTreeProducerFault::NodeCapacity);
                    return ComponentTreeProducerStep::Fault(ComponentTreeProducerFault::NodeCapacity);
                }
                return ComponentTreeProducerStep::MoreWork;
            }
            self.complete = Some(ComponentTree { root: complete.node });
            return ComponentTreeProducerStep::Complete;
        }
        if frame.compare < frame.admitted.len() {
            let duplicate = frame.pending.as_ref().is_some_and(|node| frame.admitted[frame.compare].key.as_str() == node.key.as_str());
            frame.compare += 1;
            if duplicate {
                self.fault = Some(ComponentTreeProducerFault::DuplicateSiblingKey);
                return ComponentTreeProducerStep::Fault(ComponentTreeProducerFault::DuplicateSiblingKey);
            }
            return ComponentTreeProducerStep::MoreWork;
        }
        let Some(child) = frame.pending.take() else {
            self.fault = Some(ComponentTreeProducerFault::RejectedBacking);
            return ComponentTreeProducerStep::Fault(ComponentTreeProducerFault::RejectedBacking);
        };
        if let Err(child_frame) = self.stack.try_push(ComponentTreeProducerFrame::new(child)) {
            let Some(parent) = self.stack.last_mut() else {
                self.fault = Some(ComponentTreeProducerFault::NodeDepth);
                return ComponentTreeProducerStep::Fault(ComponentTreeProducerFault::NodeDepth);
            };
            parent.pending = Some(child_frame.node);
            self.fault = Some(ComponentTreeProducerFault::NodeDepth);
            return ComponentTreeProducerStep::Fault(ComponentTreeProducerFault::NodeDepth);
        }
        ComponentTreeProducerStep::MoreWork
    }

    pub fn has_complete(&self) -> bool {
        self.complete.is_some()
    }

    pub fn take_complete(&mut self) -> Option<ComponentTree> {
        self.complete.take()
    }

    pub fn fault(&self) -> Option<ComponentTreeProducerFault> {
        self.fault
    }

    pub fn close_step(&mut self) -> bool {
        if let Some(tree) = self.complete.take() {
            drop(tree);
            return false;
        }
        if let Some(node) = self.overflow.take() {
            drop(node);
            return false;
        }
        if let Some(frame) = self.stack.pop() {
            drop(frame);
            return false;
        }
        ui_contract::close_built_node_page_one()
    }
}
//#endregion 🔖️ComponentTree

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn leaf(key: &str) -> TreeNode {
        TreeNode::try_new(key, ui_contract::Component::Separator(ui_contract::SeparatorProps {})).ok().expect("bounded fixture key")
    }

    #[test]
    fn mounted_producer_advances_one_opportunity_and_publishes_only_complete_candidate() {
        let root = leaf("root").try_with_children([leaf("a"), leaf("b")]).ok().expect("fixed child pages");
        let mut producer = ComponentTreeProducer::try_new(root, 7).ok().expect("nonzero generation");
        assert!(producer.take_complete().is_none());
        assert_eq!(producer.step(7, false, false), ComponentTreeProducerStep::MoreWork);
        assert!(producer.take_complete().is_none());
        for _ in 0..256 {
            if producer.step(7, false, false) == ComponentTreeProducerStep::Complete {
                break;
            }
        }
        assert!(producer.take_complete().is_some());
    }

    #[test]
    fn duplicate_stale_cancel_and_deadline_fault_before_publication() {
        let duplicate = leaf("root").try_with_children([leaf("same"), leaf("same")]).ok().expect("fixed child pages");
        let mut duplicate = ComponentTreeProducer::try_new(duplicate, 9).ok().expect("producer");
        for _ in 0..256 {
            if matches!(duplicate.step(9, false, false), ComponentTreeProducerStep::Fault(ComponentTreeProducerFault::DuplicateSiblingKey)) {
                break;
            }
        }
        assert_eq!(duplicate.fault(), Some(ComponentTreeProducerFault::DuplicateSiblingKey));
        assert!(duplicate.take_complete().is_none());

        let mut stale = ComponentTreeProducer::try_new(leaf("stale"), 11).ok().expect("producer");
        assert_eq!(stale.step(12, false, false), ComponentTreeProducerStep::Fault(ComponentTreeProducerFault::Generation { expected: 11, actual: 12 }));
        let mut cancelled = ComponentTreeProducer::try_new(leaf("cancel"), 13).ok().expect("producer");
        assert_eq!(cancelled.step(13, true, false), ComponentTreeProducerStep::Fault(ComponentTreeProducerFault::Cancelled));
        let mut deadline = ComponentTreeProducer::try_new(leaf("deadline"), 15).ok().expect("producer");
        assert_eq!(deadline.step(15, false, true), ComponentTreeProducerStep::Fault(ComponentTreeProducerFault::Deadline));
    }

    #[test]
    fn deep_tree_maximum_and_plus_one_preserve_exact_fault_owner_for_incremental_close() {
        let mut maximum = leaf("leaf");
        for depth in 1..COMPONENT_TREE_PRODUCER_DEPTH {
            maximum = leaf(position_key(depth).expect("bounded position").as_str()).try_with_children([maximum]).ok().expect("fixed child page");
        }
        let mut maximum = ComponentTreeProducer::try_new(maximum, 17).ok().expect("producer");
        for _ in 0..4_096 {
            if maximum.step(17, false, false) == ComponentTreeProducerStep::Complete {
                break;
            }
        }
        assert!(maximum.take_complete().is_some());

        let mut plus_one = leaf("leaf");
        for depth in 1..=COMPONENT_TREE_PRODUCER_DEPTH {
            plus_one = leaf(position_key(depth).expect("bounded position").as_str()).try_with_children([plus_one]).ok().expect("fixed child page");
        }
        let mut plus_one = ComponentTreeProducer::try_new(plus_one, 19).ok().expect("producer");
        for _ in 0..4_096 {
            if matches!(plus_one.step(19, false, false), ComponentTreeProducerStep::Fault(ComponentTreeProducerFault::NodeDepth)) {
                break;
            }
        }
        assert_eq!(plus_one.fault(), Some(ComponentTreeProducerFault::NodeDepth));
        let mut opportunities = 0;
        while !plus_one.close_step() {
            opportunities += 1;
            assert!(opportunities < 1_024);
        }
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
