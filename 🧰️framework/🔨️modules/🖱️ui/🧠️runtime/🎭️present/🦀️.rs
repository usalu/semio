//! 🎭️ The `Present` trait and the keyed `ComponentTree` a presenter builds.
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

#[path = "📦️resources/🦀️.rs"]
mod native_resources;
pub use native_resources::*;

#[derive(Debug)]
pub struct ComponentTree {
    pub root: TreeNode,
    pub native_resources: Option<UiNativeResources>,
}

impl ComponentTree {
    pub fn new(root: TreeNode) -> Self {
        Self { root, native_resources: None }
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

#[derive(Clone, Copy)]
struct ComponentTreeProducerFrame { child: usize, compare: usize, field: u8 }
impl ComponentTreeProducerFrame { const EMPTY: Self = Self { child: 0, compare: 0, field: 0 }; }

pub struct ComponentTreeProducer {
    generation: u64,
    source: Option<ComponentTree>,
    stack: [ComponentTreeProducerFrame; COMPONENT_TREE_PRODUCER_DEPTH],
    depth: usize,
    complete: bool,
    fault: Option<ComponentTreeProducerFault>,
    nodes: usize,
    closing: Option<ui_contract::OriginalBuiltTreeRetirement>,
}

impl ComponentTreeProducer {
    /// 🎟️ The native shell is the only census backing; all original child pages remain unchanged.
    pub const fn required_capacity_bytes() -> usize { std::mem::size_of::<Self>() }

    #[expect(clippy::result_large_err, reason = "Rejected admission returns the unchanged original tree without allocation.")]
    pub fn try_new(tree: ComponentTree, generation: u64) -> Result<Self, ComponentTree> {
        if generation == 0 { return Err(tree); }
        Ok(Self { generation, source: Some(tree), stack: [ComponentTreeProducerFrame::EMPTY; COMPONENT_TREE_PRODUCER_DEPTH], depth: 1, complete: false, fault: None, nodes: 0, closing: None })
    }

    pub fn close_native_resources_on_original(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<bool>, semio_framework_job::OriginalTurnFailure> {
        if let Some(resources) = self.source.as_mut().and_then(|tree|tree.native_resources.as_mut()) {
            if resources.close_step_on_original(cx)? != Some(true) { return Ok(None); }
            self.source.as_mut().unwrap().native_resources = None;
            return Ok(Some(false));
        }
        Ok(Some(true))
    }

    pub fn generation(&self) -> u64 { self.generation }
    pub fn original_source(&self) -> Option<&ComponentTree> { self.source.as_ref() }

    fn current_node(&self) -> Option<&TreeNode> {
        let mut node = &self.source.as_ref()?.root;
        for frame in self.stack.get(..self.depth.checked_sub(1)?)? { node = node.children.get(frame.child)?; }
        Some(node)
    }

    fn refuse(&mut self, fault: ComponentTreeProducerFault) -> ComponentTreeProducerStep { self.fault = Some(fault); ComponentTreeProducerStep::Fault(fault) }

    /// 🧭️ One bounded original index path and field/comparison advances without copying or replacing a child.
    pub fn step_on_original(&mut self, generation: u64, cancelled: bool, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ComponentTreeProducerStep, semio_framework_job::OriginalTurnFailure> {
        let work = self.depth.max(1) as u64;
        let grant = cx.retained_grant();
        if cx.should_yield() || cx.fuel_remaining()<work || grant.maximum_items==0 || grant.maximum_depth<self.depth.max(1) { return Ok(ComponentTreeProducerStep::MoreWork); }
        let outcome = self.step(generation,cancelled||cx.is_cancelled(),cx.deadline_exceeded());
        cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;
        cx.consume_fuel(work);
        Ok(outcome)
    }

    pub fn step(&mut self, generation: u64, cancelled: bool, deadline_expired: bool) -> ComponentTreeProducerStep {
        if let Some(fault) = self.fault { return ComponentTreeProducerStep::Fault(fault); }
        if generation != self.generation { return self.refuse(ComponentTreeProducerFault::Generation { expected: self.generation, actual: generation }); }
        if cancelled { return self.refuse(ComponentTreeProducerFault::Cancelled); }
        if deadline_expired { return self.refuse(ComponentTreeProducerFault::Deadline); }
        if self.complete { return ComponentTreeProducerStep::Complete; }
        let Some(index) = self.depth.checked_sub(1) else { return ComponentTreeProducerStep::Complete; };
        let frame = self.stack[index];
        let Some(node) = self.current_node() else { return self.refuse(ComponentTreeProducerFault::RejectedBacking); };
        if !node.rejected_children.is_empty() { return self.refuse(ComponentTreeProducerFault::RejectedBacking); }
        if frame.field < COMPONENT_TREE_NODE_FIELDS {
            if frame.field == 0 && node.key.len() > ui_contract::UI_TEXT_MAX_BYTES { return self.refuse(ComponentTreeProducerFault::IdentifierBytes); }
            self.stack[index].field += 1;
            return ComponentTreeProducerStep::MoreWork;
        }
        if frame.child == node.children.len() {
            self.nodes = match self.nodes.checked_add(1) { Some(nodes) if nodes<=ui_contract::UI_BUILT_CHILD_RETIRE_SLOTS => nodes, _ => return self.refuse(ComponentTreeProducerFault::NodeCapacity) };
            self.depth -= 1;
            if self.depth == 0 { self.complete = true; return ComponentTreeProducerStep::Complete; }
            self.stack[self.depth-1].child += 1;
            self.stack[self.depth-1].compare = 0;
            return ComponentTreeProducerStep::MoreWork;
        }
        if frame.compare < frame.child {
            let duplicate = node.children.get(frame.compare).zip(node.children.get(frame.child)).is_some_and(|(left,right)|left.key.as_str()==right.key.as_str());
            self.stack[index].compare += 1;
            if duplicate { return self.refuse(ComponentTreeProducerFault::DuplicateSiblingKey); }
            return ComponentTreeProducerStep::MoreWork;
        }
        if self.depth == COMPONENT_TREE_PRODUCER_DEPTH { return self.refuse(ComponentTreeProducerFault::NodeDepth); }
        self.stack[self.depth] = ComponentTreeProducerFrame::EMPTY;
        self.depth += 1;
        ComponentTreeProducerStep::MoreWork
    }

    pub fn has_complete(&self) -> bool { self.complete && self.source.is_some() }
    pub fn take_complete(&mut self) -> Option<ComponentTree> { if self.complete { self.source.take() } else { None } }
    pub fn fault(&self) -> Option<ComponentTreeProducerFault> { self.fault }

    /// 🔑️ The duplicate keys remain borrowed from the unchanged original candidate.
    pub fn duplicate_sibling(&self) -> Option<(&str, &str)> {
        if !matches!(self.fault,Some(ComponentTreeProducerFault::DuplicateSiblingKey)) { return None; }
        let node = self.current_node()?;
        Some((node.key.as_str(),node.children.get(self.stack[self.depth.checked_sub(1)?].child)?.key.as_str()))
    }

    /// ♻️ Native resources and the exact original tree close before the separately retained shell releases.
    pub fn close_step_on_original(&mut self,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_framework_job::OriginalTurnFailure> {
        if self.close_native_resources_on_original(cx)? != Some(true) { return Ok(false); }
        if let Some(owner) = self.closing.as_mut() {
            let demand = owner.next_demand(cx.retained_grant().maximum_copy_bytes)?;
            semio_framework_job::close_original_retained_step(cx,demand,|grant|owner.step(grant))?;
            if owner.terminal_is_empty() { self.closing = None; }
            return Ok(false);
        }
        if self.source.is_some() {
            let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0 { return Ok(false); }
            cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);
            self.closing=Some(ui_contract::OriginalBuiltTreeRetirement::new(self.source.take().unwrap().root));self.depth=0;
            return Ok(false);
        }
        Ok(true)
    }

    pub fn terminal_is_empty(&self) -> bool { self.source.is_none() && self.closing.is_none() }

    pub fn close_step_with_grant(&mut self,items:usize) -> bool { for _ in 0..items { if self.close_step() { return true; } } false }

    pub fn close_step(&mut self) -> bool {
        if self.closing.is_some() || self.source.as_ref().is_some_and(|tree|tree.native_resources.is_some()) { return false; }
        if self.source.take().is_some() { self.depth=0;return false; }
        ui_contract::close_built_node_page_one()
    }
}
//#endregion 🔖️ComponentTree

//#region 🧪️Tests
#[cfg(test)]
#[path = "../🧪️tests/🔬️present-unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
