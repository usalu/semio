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
    fn paint_slot(&mut self, slot: &SceneSlot<'_>, draw: &mut DrawList, atlas: &mut FontAtlas, icons: Option<&IconAtlas>);
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
pub(crate) fn collect_scene_slots<'tree>(tree: &'tree UiTree, root: NodeId) -> Vec<SceneSlot<'tree>> {
    let mut slots = Vec::new();
    collect_scene_slots_node(tree, root, 0.0, 0.0, &mut slots);
    slots
}

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
mod tests {
    use super::*;
    use crate::wgpu::component::ui::{UiComponentSceneNode, UiGroupNode, UiPresence, UiStackNode, UiTextNode};
    use crate::wgpu::flex::LayoutEngine;
    use crate::wgpu::theme::Theme;
    use crate::wgpu::Label;

    fn text(value: &str) -> UiNode {
        UiNode::Text(UiTextNode { value: Label::data(value), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })
    }

    fn scene(surface_id: &str) -> UiNode {
        UiNode::ComponentScene(UiComponentSceneNode {
            surface_id: surface_id.into(),
            controller_id: "ctrl".into(),
            component_kind: SurfaceKind::World3d,
            pane_id: None,
            binding_id: None,
            presence: UiPresence::default(),
            canvas_2d: None,
            world_3d: None,
            node_graph: None,
            text_editor: None,
            table: None,
            paint_2d: None,
            virtual_file_system: None,
            tiled_map: None,
            board2d: None,
            icon_render: None,
            ink_canvas: None,
            graph_timeline: None,
            block_list: None,
            diff_view: None,
            event_feed: None,
            menu: None,
        })
    }

    fn image(id: &str) -> UiNode {
        UiNode::Image(UiImageNode { id: id.into(), src: "https://example.test/x.png".into(), alt: None, presence: UiPresence::default(), menu: None })
    }

    fn stack(children: Vec<UiNode>) -> UiNode {
        UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: Some("none".into()), padding: Some("standard".into()), id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children, menu: None })
    }

    fn group(children: Vec<UiNode>) -> UiNode {
        UiNode::Group(UiGroupNode { id: "group".into(), label: Label::data("Group"), default_open: None, presence: UiPresence::default(), children, menu: None })
    }

    fn layout(node: &UiNode) -> UiTree {
        let mut tree = UiTree::new();
        tree.apply_tree(node);
        let root = tree.root.unwrap();
        let mut engine = LayoutEngine::new();
        let mut atlas = FontAtlas::builtin();
        let theme = Theme::default();
        engine.compute(&mut tree, root, &mut atlas, &theme, 400.0, 400.0);
        tree
    }

    #[test]
    fn collects_a_scene_leaf_with_its_absolute_rect_accounting_for_ancestor_offsets() {
        let tree = layout(&stack(vec![text("above"), scene("surface.one")]));
        let root = tree.root.unwrap();

        let slots = collect_scene_slots(&tree, root);
        assert_eq!(slots.len(), 1);
        let slot = &slots[0];
        assert_eq!(slot.surface(), Some(("surface.one", SurfaceKind::World3d)));
        // The scene leaf is the stack's second child (below the text sibling plus the stack's own
        // top padding) -- a nonzero absolute y proves ancestor offsets were accumulated, not just
        // the leaf's own parent-relative `LayoutBucket` coordinates.
        assert!(slot.rect.y > 0.0, "expected the scene leaf offset below its text sibling, got y={}", slot.rect.y);
        assert!(slot.rect.w > 0.0 && slot.rect.h > 0.0);
    }

    #[test]
    fn finds_no_slots_when_the_tree_has_no_scene_nodes() {
        let tree = layout(&stack(vec![text("only text")]));
        let root = tree.root.unwrap();
        assert!(collect_scene_slots(&tree, root).is_empty());
    }

    #[test]
    fn collects_multiple_scene_leaves_in_document_order() {
        let tree = layout(&stack(vec![scene("surface.a"), scene("surface.b")]));
        let root = tree.root.unwrap();

        let slots = collect_scene_slots(&tree, root);
        let ids: Vec<&str> = slots.iter().filter_map(|slot| slot.surface().map(|(id, _)| id)).collect();
        assert_eq!(ids, vec!["surface.a", "surface.b"]);
    }

    #[test]
    fn collects_an_image_leaf_alongside_a_scene_leaf() {
        let tree = layout(&stack(vec![image("img.one"), scene("surface.one")]));
        let root = tree.root.unwrap();

        let slots = collect_scene_slots(&tree, root);
        assert_eq!(slots.len(), 2);
        assert!(matches!(slots[0].content, SlotContent::Image(node) if node.id == "img.one"));
        assert!(matches!(slots[1].content, SlotContent::Scene(node) if node.surface_id == "surface.one"));
    }

    #[test]
    fn collects_a_scene_leaf_nested_under_a_group_ancestor() {
        // 🌳️ Regression for the shadow-walk gap this bridge replaces: the legacy immediate-mode walk
        // it superseded only recursed into Stack/Section/Field, so a ComponentScene nested under a
        // Group never resolved to real content. `collect_scene_slots_node` recurses into every
        // node's `tree.children` unconditionally, so a Group ancestor is no different from a Stack.
        let tree = layout(&group(vec![text("label"), scene("surface.nested")]));
        let root = tree.root.unwrap();

        let slots = collect_scene_slots(&tree, root);
        assert_eq!(slots.len(), 1);
        assert_eq!(slots[0].surface(), Some(("surface.nested", SurfaceKind::World3d)));
    }
}
// #endregion scene_slots
