//! 🌿️ Listeners and dispatch tree: the typed `ListenerSet`, the `DispatchNode` and the frame-local `DispatchTree`. See [`super`].

use super::*;

//#region 🔖️DispatchTree
/// 🧩️ One node's typed interaction contract — the "typed listeners" that replace the old `HitKind`
/// enum. `bindings` are the node's own [`ui_contract::ActionBinding`]s (already the contract's closed,
/// product-agnostic `Trigger` set); `surface`/`node`/`node_key`/`revision` are the protocol addressing
/// needed to stamp a real [`UiIntent`] when one fires, captured at the revision this node's snapshot
/// was built from — see [`is_stale`] for why that revision matters. `value` is the node's current
/// declarative value if it is [`DispatchFlags::EDITABLE`] (`None` otherwise), used only to seed an
/// [`EditState`] on focus gain — dispatch never interprets it beyond that.
#[derive(Debug, Default)]
pub struct ListenerSet {
    pub surface: SurfaceId,
    pub node: UiNodeId,
    pub node_key: UiText,
    pub revision: UiRevision,
    pub value: Option<UiValue>,
    pub bindings: Vec<ActionBinding>,
}

impl ListenerSet {
    pub fn credited_clone(&self) -> Option<Self> {
        Some(Self {
            surface: self.surface.clone(),
            node: self.node,
            node_key: self.node_key.clone(),
            revision: self.revision,
            value: match self.value.as_ref() {
                Some(value) => Some(value.credited_clone()?),
                None => None,
            },
            bindings: self.bindings.iter().map(ActionBinding::credited_clone).collect::<Option<Vec<_>>>()?,
        })
    }

    pub fn binding_for(&self, trigger: Trigger) -> Option<&ActionBinding> {
        self.bindings.iter().find(|binding| binding.trigger == trigger)
    }
}

#[cfg(test)]
impl Clone for ListenerSet {
    fn clone(&self) -> Self {
        self.credited_clone().expect("bounded test listener set has clone credits")
    }
}

/// 🧱️ One frame-local dispatch node — see this file's module docstring for why dynamic interaction
/// state (`ACTIVE`/`FOCUSED`/`HOVERED`) is deliberately absent: [`DispatchTree`] is immutable once
/// built, so that state lives on [`Dispatcher`] instead, keyed by [`ElementId`].
pub struct DispatchNode {
    pub parent: Option<FrameNodeId>,
    pub hitbox: Option<HitboxId>,
    pub focus: Option<FocusId>,
    pub element: ElementId,
    pub flags: DispatchFlags,
    pub listeners: ListenerSet,
}

/// 🌳️ The frame-local interpreted dispatch tree — built node-by-node during prepaint (see
/// `element.rs`'s `PrepaintCx::register`/`PrepaintCx::with_children`) and held
/// read-only inside the presented `Rc<FrameSnapshot>` for the lifetime of that generation. `index` is
/// the reverse [`ElementId`] →
/// [`FrameNodeId`] lookup [`Dispatcher`] uses to resolve its own persistent, `ElementId`-keyed state
/// (capture/focus/overlay roots/drag source) against *this* frame's tree.
#[derive(Default)]
pub struct DispatchTree {
    nodes: Vec<DispatchNode>,
    pub(super) children: Vec<Vec<FrameNodeId>>,
    hitboxes: Vec<Hitbox>,
    pub(super) root: Option<FrameNodeId>,
    pub(super) revision: UiRevision,
    pub(super) index: HashMap<ElementId, FrameNodeId>,
}

impl DispatchTree {
    pub fn new(revision: UiRevision) -> Self {
        Self { nodes: Vec::new(), children: Vec::new(), hitboxes: Vec::new(), root: None, revision, index: HashMap::new() }
    }

    /// ➕️ Appends one node. A `hitbox.clips_children`/`hitbox.hit_transparent` bit, if the hitbox is
    /// present, is folded into `flags` automatically so a caller never has to state the same fact
    /// twice. `parent: None` makes this node the tree's root (last such call wins — real callers build
    /// exactly one root).
    pub fn insert(&mut self, parent: Option<FrameNodeId>, element: ElementId, mut flags: DispatchFlags, listeners: ListenerSet, hitbox: Option<Hitbox>) -> FrameNodeId {
        let id = FrameNodeId(self.nodes.len() as u32);
        let hitbox_id = hitbox.map(|hitbox| {
            if hitbox.clips_children {
                flags.insert(DispatchFlags::CLIPS_CHILDREN);
            }
            if hitbox.hit_transparent {
                flags.insert(DispatchFlags::HIT_TRANSPARENT);
            }
            let hitbox_id = HitboxId(self.hitboxes.len() as u32);
            self.hitboxes.push(hitbox);
            hitbox_id
        });
        let focus = flags.contains(DispatchFlags::FOCUSABLE).then_some(FocusId(id));
        self.nodes.push(DispatchNode { parent, hitbox: hitbox_id, focus, element, flags, listeners });
        self.children.push(Vec::new());
        match parent {
            Some(parent) => self.children[parent.0 as usize].push(id),
            None => self.root = Some(id),
        }
        self.index.insert(element, id);
        id
    }

    pub fn node(&self, id: FrameNodeId) -> Option<&DispatchNode> {
        self.nodes.get(id.0 as usize)
    }

    pub fn children(&self, id: FrameNodeId) -> &[FrameNodeId] {
        self.children.get(id.0 as usize).map_or(&[], Vec::as_slice)
    }

    pub fn element_node(&self, element: ElementId) -> Option<FrameNodeId> {
        self.index.get(&element).copied()
    }

    pub fn bounds(&self, hitbox: HitboxId) -> Bounds {
        self.hitboxes[hitbox.0 as usize].bounds
    }

    pub fn root(&self) -> Option<FrameNodeId> {
        self.root
    }

    pub fn revision(&self) -> UiRevision {
        self.revision
    }

    /// 📇️ Every hitbox registered this frame, flat — [`Hitbox`] earning its place as a spatial-index
    /// source (a future quadtree/BVH over pointer-heavy scenes) rather than as tree structure, which
    /// now lives in [`DispatchNode::parent`]/[`Self::children`] instead. Index order matches
    /// [`HitboxId`] (`self.bounds`'s own indexing), never insertion-into-`nodes` order for a node
    /// without a hitbox.
    pub fn hitboxes(&self) -> &[Hitbox] {
        &self.hitboxes
    }
}
//#endregion 🔖️DispatchTree
