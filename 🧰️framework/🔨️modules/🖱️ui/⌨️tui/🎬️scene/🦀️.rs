use crate::tui::chrome::ChromeState;
use crate::tui::geometry::{Pos, Rect};
use crate::tui::layout::Constraint;
use crate::tui::theme::{Role, Surface};
use crate::tui::widget::WidgetState;

const LAYOUT_DIRTY: u8 = 1;
const PAINT_DIRTY: u8 = 2;

/// ??? A stable, generation-checked handle to a scene node.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NodeId {
    index: u32,
    generation: u32,
}

/// ??? The payload a node carries.
pub enum NodeContent {
    Box,
    Text(String),
    Widget(WidgetState),
    Chrome(ChromeState),
}

/// ??? A node's visual role (independent of its content).
#[derive(Default, Clone, Copy)]
pub struct Style {
    pub surface: Option<Surface>,
    pub fg: Option<Role>,
    pub attrs: u8,
}

/// ??? One retained scene node.
pub struct Node {
    pub content: NodeContent,
    pub style: Style,
    pub constraint: Constraint,
    pub visible: bool,
    pub(crate) children: Vec<NodeId>,
    pub(crate) parent: Option<NodeId>,
    pub(crate) rect: Rect,
    pub(crate) dirty: u8,
}

impl Node {
    pub fn new(content: NodeContent) -> Self {
        Self { content, style: Style::default(), constraint: Constraint::default(), visible: true, children: Vec::new(), parent: None, rect: Rect::default(), dirty: LAYOUT_DIRTY | PAINT_DIRTY }
    }

    pub fn children(&self) -> &[NodeId] {
        &self.children
    }
}

struct Slot {
    node: Option<Node>,
    generation: u32,
}

/// ??? A generational-arena retained scene tree, renderer-agnostic.
pub struct Scene {
    slots: Vec<Slot>,
    free: Vec<u32>,
    root: NodeId,
}

impl Scene {
    pub fn new() -> Self {
        let mut slots = Vec::new();
        slots.push(Slot { node: Some(Node::new(NodeContent::Box)), generation: 0 });
        Self { slots, free: Vec::new(), root: NodeId { index: 0, generation: 0 } }
    }

    pub fn root(&self) -> NodeId {
        self.root
    }

    pub fn add(&mut self, parent: NodeId, mut node: Node) -> NodeId {
        node.parent = Some(parent);
        let id = if let Some(index) = self.free.pop() {
            let slot = &mut self.slots[index as usize];
            slot.generation += 1;
            let id = NodeId { index, generation: slot.generation };
            slot.node = Some(node);
            id
        } else {
            let index = self.slots.len() as u32;
            self.slots.push(Slot { node: Some(node), generation: 0 });
            NodeId { index, generation: 0 }
        };
        if let Some(p) = self.slots[parent.index as usize].node.as_mut() {
            p.children.push(id);
        }
        self.mark_dirty(parent, LAYOUT_DIRTY | PAINT_DIRTY);
        id
    }

    pub fn remove(&mut self, id: NodeId) {
        let children = self.node(id).children.clone();
        for child in children {
            self.remove(child);
        }
        if let Some(parent) = self.node(id).parent {
            if let Some(p) = self.slots[parent.index as usize].node.as_mut() {
                p.children.retain(|c| *c != id);
            }
            self.mark_dirty(parent, LAYOUT_DIRTY | PAINT_DIRTY);
        }
        self.slots[id.index as usize].node = None;
        self.free.push(id.index);
    }

    /// ?? Moves `id` under `new_parent`, preserving subtree state for layout remounts.
    pub fn reparent(&mut self, id: NodeId, new_parent: NodeId) {
        if !self.valid(id) || !self.valid(new_parent) || id == new_parent {
            return;
        }
        if let Some(old_parent) = self.node(id).parent {
            if let Some(p) = self.slots[old_parent.index as usize].node.as_mut() {
                p.children.retain(|c| *c != id);
            }
            self.mark_dirty(old_parent, LAYOUT_DIRTY | PAINT_DIRTY);
        }
        if let Some(p) = self.slots[new_parent.index as usize].node.as_mut() {
            if !p.children.contains(&id) {
                p.children.push(id);
            }
        }
        self.node_raw_mut(id).parent = Some(new_parent);
        self.mark_dirty(new_parent, LAYOUT_DIRTY | PAINT_DIRTY);
        self.mark_dirty(id, LAYOUT_DIRTY | PAINT_DIRTY);
    }

    fn valid(&self, id: NodeId) -> bool {
        self.slots.get(id.index as usize).is_some_and(|s| s.generation == id.generation && s.node.is_some())
    }

    pub fn node(&self, id: NodeId) -> &Node {
        self.slots[id.index as usize].node.as_ref().expect("stale NodeId")
    }

    pub(crate) fn node_raw_mut(&mut self, id: NodeId) -> &mut Node {
        self.slots[id.index as usize].node.as_mut().expect("stale NodeId")
    }

    pub fn rect(&self, id: NodeId) -> Rect {
        self.node(id).rect
    }

    pub(crate) fn mark_dirty(&mut self, id: NodeId, flags: u8) {
        let mut cursor = Some(id);
        while let Some(cur) = cursor {
            if !self.valid(cur) {
                break;
            }
            let node = self.node_raw_mut(cur);
            if node.dirty & flags == flags {
                break;
            }
            node.dirty |= flags;
            cursor = node.parent;
        }
    }

    pub(crate) fn take_dirty(&mut self, id: NodeId) -> u8 {
        let d = self.node(id).dirty;
        self.node_raw_mut(id).dirty = 0;
        d
    }

    pub fn node_mut(&mut self, id: NodeId) -> NodeMut<'_> {
        NodeMut { scene: self, id }
    }

    /// ??? The deepest visible node whose rect contains `pos`.
    pub fn hit(&self, pos: Pos) -> Option<NodeId> {
        fn walk(scene: &Scene, id: NodeId, pos: Pos) -> Option<NodeId> {
            let node = scene.node(id);
            if !node.visible || !node.rect.contains(pos) {
                return None;
            }
            for &child in node.children.iter().rev() {
                if let Some(hit) = walk(scene, child, pos) {
                    return Some(hit);
                }
            }
            Some(id)
        }
        walk(self, self.root, pos)
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

/// ?? A scoped mutation handle: every setter marks layout/paint dirty up the parent chain.
pub struct NodeMut<'a> {
    scene: &'a mut Scene,
    id: NodeId,
}

impl<'a> NodeMut<'a> {
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.scene.node_raw_mut(self.id).content = NodeContent::Text(text.into());
        self.scene.mark_dirty(self.id, LAYOUT_DIRTY | PAINT_DIRTY);
    }

    pub fn set_constraint(&mut self, constraint: Constraint) {
        self.scene.node_raw_mut(self.id).constraint = constraint;
        self.scene.mark_dirty(self.id, LAYOUT_DIRTY);
    }

    pub fn set_style(&mut self, style: Style) {
        self.scene.node_raw_mut(self.id).style = style;
        self.scene.mark_dirty(self.id, PAINT_DIRTY);
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.scene.node_raw_mut(self.id).visible = visible;
        self.scene.mark_dirty(self.id, LAYOUT_DIRTY | PAINT_DIRTY);
    }

    pub fn widget(&mut self) -> Option<&mut WidgetState> {
        self.scene.mark_dirty(self.id, PAINT_DIRTY);
        match &mut self.scene.node_raw_mut(self.id).content {
            NodeContent::Widget(w) => Some(w),
            _ => None,
        }
    }

    pub fn chrome(&mut self) -> Option<&mut ChromeState> {
        self.scene.mark_dirty(self.id, PAINT_DIRTY);
        match &mut self.scene.node_raw_mut(self.id).content {
            NodeContent::Chrome(c) => Some(c),
            _ => None,
        }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }
}
