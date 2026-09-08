// #region scene_slots
//! 🎬️ Scene-host bridge: after each layout+paint pass the engine collects every `ComponentScene`/
//! `Image` leaf's resolved absolute rect PLUS a borrowed reference to its own stored `UiNode`
//! payload into a `SceneSlot`, and hands each one to a caller-provided `SceneHost`, which owns the
//! actual scene/image rendering (world3d via `infinite_world`, canvas2d, vello surfaces, raster
//! image decode/upload). `ui_wgpu` never links vello/resvg/tiny-skia/an image codec itself — it only
//! orchestrates slot geometry and payload borrowing, matching the plan's dependency-graph invariant
//! that those crates stay in the renderer. Slots borrow directly from the retained `UiTree`'s own
//! arena-stored `UiNode` — never a second parallel structure — so a host reading a slot's payload is
//! reading the exact same data `paint::paint_node` would have painted a placeholder for.

use crate::wgpu::arena::NodeId;
use crate::wgpu::component::ui::{SurfaceKind, UiComponentSceneNode, UiImageNode, UiNode};
use crate::wgpu::draw::{DrawList, IconAtlas};
use crate::wgpu::geometry::Rect;
use crate::wgpu::text::FontAtlas;
use crate::wgpu::tree::UiTree;

/// 🎬️ A `SceneSlot`'s borrowed payload — points directly at the leaf's own `UiNode` variant stored
/// in the retained `UiTree`'s arena, never a clone.
#[derive(Debug, PartialEq)]
pub enum SlotContent<'tree> {
    Scene(&'tree UiComponentSceneNode),
    Image(&'tree UiImageNode),
}

/// 🎬️ One `ComponentScene`/`Image` leaf's resolved absolute rect plus its full borrowed payload,
/// ready to hand to a `SceneHost`.
#[derive(Debug, PartialEq)]
pub struct SceneSlot<'tree> {
    pub node: NodeId,
    pub rect: Rect,
    pub content: SlotContent<'tree>,
}

/// 🎞️ Retained producer-consumer cursor for one scene or image leaf.
#[derive(Default)]
pub struct ScenePaintCursor {
    node: Option<NodeId>,
    phase: u16,
    item: usize,
    page: usize,
    byte: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScenePaintStep {
    Pending,
    Complete,
    Fault,
}

impl ScenePaintCursor {
    pub fn bind(&mut self, node: NodeId) -> Result<bool, ()> {
        match self.node {
            None => {
                self.node = Some(node);
                Ok(false)
            }
            Some(active) if active == node => Ok(true),
            Some(_) => Err(()),
        }
    }

    pub const fn phase(&self) -> u16 {
        self.phase
    }

    pub fn advance_phase(&mut self) -> Result<(), ()> {
        self.phase = self.phase.checked_add(1).ok_or(())?;
        self.item = 0;
        self.page = 0;
        self.byte = 0;
        Ok(())
    }

    pub const fn item(&self) -> usize {
        self.item
    }

    pub fn advance_item(&mut self) -> Result<(), ()> {
        self.item = self.item.checked_add(1).ok_or(())?;
        Ok(())
    }

    pub const fn page(&self) -> usize {
        self.page
    }

    pub fn advance_page(&mut self) -> Result<(), ()> {
        self.page = self.page.checked_add(1).ok_or(())?;
        Ok(())
    }

    pub const fn byte(&self) -> usize {
        self.byte
    }

    pub fn advance_byte(&mut self) -> Result<(), ()> {
        self.byte = self.byte.checked_add(1).ok_or(())?;
        Ok(())
    }

    pub fn finish(&mut self) -> ScenePaintStep {
        self.node = None;
        self.phase = 0;
        self.item = 0;
        self.page = 0;
        self.byte = 0;
        ScenePaintStep::Complete
    }

    pub fn close_step(&mut self) -> bool {
        if self.node.take().is_some() {
            self.phase = 0;
            self.item = 0;
            self.page = 0;
            self.byte = 0;
            return false;
        }
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.node.is_none()
    }
}

impl<'tree> SceneSlot<'tree> {
    /// 🪪️ `(surface_id, SurfaceKind)` when this slot is a `ComponentScene` — `None` for `Image`,
    /// which carries no `SurfaceKind` (it's routed by `SlotContent`'s own variant instead).
    pub fn surface(&self) -> Option<(&'tree str, SurfaceKind)> {
        match self.content {
            SlotContent::Scene(scene) => Some((scene.surface_id.as_str(), scene.component_kind)),
            SlotContent::Image(_) => None,
        }
    }
}

/// 🖇️ External scene/image renderer — the only place vello/world3d/raster-decode-specific code may
/// live; `ui_wgpu` calls into it after layout+paint with resolved slot geometry plus the borrowed
/// node payload, never the reverse. Paint-only this milestone: routing pointer/keyboard events that
/// hit a slot to this same host needs a different mechanism (event routing is keyed by `NodeId`
/// through `events::EventRouter` today, which knows nothing about host-owned sub-surfaces) — that's
/// later, separate work, not this trait's job to anticipate.
pub trait SceneHost {
    /// 🖌️ Pushes this slot's own draw calls into `draw` — the retained window's own `DrawList`, in
    /// that window's local `(0,0)`-origin coordinate space, the same space `slot.rect` is expressed
    /// in (the caller composites/offsets the whole `DrawList` afterward, same as every other
    /// retained-paint call). `atlas`/`icons` are the SAME instances the frame's caller passed into
    /// `Ui::frame`, reborrowed fresh per slot so a host that draws text/icons shares the one real,
    /// GPU-uploaded glyph/icon texture instead of needing (or clobbering) its own.
    fn paint_slot_step(&mut self, slot: &SceneSlot<'_>, cursor: &mut ScenePaintCursor, draw: &mut DrawList, atlas: &mut FontAtlas, icons: Option<&IconAtlas>) -> ScenePaintStep;

    #[cfg(test)]
    fn paint_slot(&mut self, slot: &SceneSlot<'_>, draw: &mut DrawList, atlas: &mut FontAtlas, icons: Option<&IconAtlas>) {
        let mut cursor = ScenePaintCursor::default();
        while matches!(self.paint_slot_step(slot, &mut cursor, draw, atlas, icons), ScenePaintStep::Pending) {}
    }
}

/// 📥️ Walks `tree` from `root`, collecting every `ComponentScene`/`Image` leaf's absolute rect
/// (ancestor offsets accumulated the same way `events::hit_test_node`/`paint::paint_node` do) plus a
/// borrowed reference to its own stored `UiNode` payload. Recurses into every node's own arena
/// children unconditionally — not gated by node kind — so leaves nested under ANY container
/// (`Stack`/`Field`/`Section`/`Group`/`Tree` alike) are found; `tree.children` already reflects
/// `reconcile`'s real parent-child links for every `UiNode` kind, including `Field`'s single child,
/// so there is no special-casing needed here for any one container kind. Always includes every
/// reachable leaf regardless of `DIRTY_PAINT`/`DIRTY_LAYOUT` — scene/image leaves are always-dirty
/// unless the host opts into its own caching, so `ui_wgpu` doesn't try to cache on the host's behalf
/// this milestone.
#[cfg(test)]
pub(crate) fn collect_scene_slots<'tree>(tree: &'tree UiTree, root: NodeId) -> Vec<SceneSlot<'tree>> {
    let mut slots = Vec::new();
    collect_scene_slots_node(tree, root, 0.0, 0.0, &mut slots);
    slots
}

pub(crate) fn scene_slot_for_node<'tree>(tree: &'tree UiTree, id: NodeId, origin_x: f32, origin_y: f32) -> Option<SceneSlot<'tree>> {
    let node = tree.node(id)?;
    let layout = tree.accepted_layout(id)?;
    let rect = Rect::new(origin_x + layout.x, origin_y + layout.y, layout.width, layout.height);
    match &node.spec.0 {
        UiNode::ComponentScene(scene) => Some(SceneSlot { node: id, rect, content: SlotContent::Scene(scene) }),
        UiNode::Image(image) => Some(SceneSlot { node: id, rect, content: SlotContent::Image(image) }),
        _ => None,
    }
}

#[cfg(test)]
fn collect_scene_slots_node<'tree>(tree: &'tree UiTree, id: NodeId, origin_x: f32, origin_y: f32, out: &mut Vec<SceneSlot<'tree>>) {
    let Some(node) = tree.node(id) else { return };
    let Some(layout) = tree.accepted_layout(id) else { return };
    let abs_x = origin_x + layout.x;
    let abs_y = origin_y + layout.y;
    let rect = Rect::new(abs_x, abs_y, layout.width, layout.height);
    match &node.spec.0 {
        UiNode::ComponentScene(scene) => out.push(SceneSlot { node: id, rect, content: SlotContent::Scene(scene) }),
        UiNode::Image(image) => out.push(SceneSlot { node: id, rect, content: SlotContent::Image(image) }),
        _ => {}
    }
    for child in tree.children(id) {
        collect_scene_slots_node(tree, child, abs_x, abs_y, out);
    }
}

#[cfg(test)]
#[path = "../../../../🧪️tests/🔬️targets-wgpu-scene-slots-unit/🦀️.rs"]
mod tests;
// #endregion scene_slots
