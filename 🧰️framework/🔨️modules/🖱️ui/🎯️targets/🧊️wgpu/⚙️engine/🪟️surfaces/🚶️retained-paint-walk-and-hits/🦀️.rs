//! 🚶️ Retained paint walk and hits: the paint credits, the walk and its clip helpers, the paint phase, the frame paint census, the scene hits, the hit registration and the paint frame. See [`super`].

use super::*;

//#region 🔖️UiWindow
pub const RETAINED_PAINT_DEPTH_CREDITS: usize = 64;

pub const UI_RETIRED_COMPONENT_SCENE_CAPACITY: usize = 256;

/// 🎯️ Fixed ceiling on one window's pointer registry — well under `input`'s own
/// `HIT_TARGET_CAPACITY`, so a hostile document can never crowd the chrome out of the shell's
/// registry.
const RETAINED_HIT_REGISTRY_CAPACITY: usize = 4_096;

#[derive(Clone, Copy)]
pub struct RetainedPaintVisit {
    pub node: NodeId,
    pub origin_x: f32,
    pub origin_y: f32,
    pub next_child: Option<NodeId>,
    pub entered: bool,
    /// 🪟️ The OPEN overlay root this visit sits under, inherited down the subtree — `None` for
    /// ordinary in-flow content. The paint phase routes a visit that names one into `DrawList`'s
    /// overlay bucket so a floating surface's content composites above every panel, on top of the
    /// surface chrome the `Overlays` phase drew for it (ticket 26/09/17 packet W15a).
    pub overlay_root: Option<NodeId>,
}

pub struct RetainedPaintWalk {
    pub visits: [Option<RetainedPaintVisit>; RETAINED_PAINT_DEPTH_CREDITS],
    pub len: usize,
}

pub enum RetainedPaintWalkStep {
    Visit(NodeId, f32, f32, Option<NodeId>),
    Scalar,
    Complete,
    DepthFault,
}

impl RetainedPaintWalk {
    pub fn new(tree: &UiTree, root: NodeId) -> Self {
        let root_visit = RetainedPaintVisit { node: root, origin_x: 0.0, origin_y: 0.0, next_child: retained_first_child(tree, root), entered: false, overlay_root: None };
        let mut visits = [None; RETAINED_PAINT_DEPTH_CREDITS];
        visits[0] = Some(root_visit);
        Self { visits, len: 1 }
    }

    /// 🪜️ An OPEN floating overlay's content root ignores its in-flow parent offset and takes
    /// the placement `events::resolve_overlay_placement_side` settled on — the SAME override
    /// `events::hit_test`/`absolute_rect` apply, so what is painted is what is hit
    /// (`UiTree::overlay_origins`). Everything below it accumulates from there as usual.
    pub fn step(&mut self, tree: &UiTree) -> RetainedPaintWalkStep {
        let Some(index) = self.len.checked_sub(1) else { return RetainedPaintWalkStep::Complete };
        let Some(visit) = self.visits[index].as_mut() else { return RetainedPaintWalkStep::DepthFault };
        if !visit.entered {
            visit.entered = true;
            return RetainedPaintWalkStep::Visit(visit.node, visit.origin_x, visit.origin_y, visit.overlay_root);
        }
        if let Some(child) = visit.next_child {
            visit.next_child = tree.node(child).and_then(|node| node.next_sibling);
            if tree.accepted_layout(child).is_none() {
                return RetainedPaintWalkStep::Scalar;
            }
            if self.len == RETAINED_PAINT_DEPTH_CREDITS {
                return RetainedPaintWalkStep::DepthFault;
            }
            let Some(child_origin) = tree.child_walk_origin(visit.node, (visit.origin_x, visit.origin_y)) else { return RetainedPaintWalkStep::DepthFault };
            let overlay_origin = tree.overlay_walk_origin(child);
            let (child_origin_x, child_origin_y) = overlay_origin.unwrap_or(child_origin);
            let child_overlay_root = if overlay_origin.is_some() { Some(child) } else { visit.overlay_root };
            let child_visit = RetainedPaintVisit { node: child, origin_x: child_origin_x, origin_y: child_origin_y, next_child: retained_first_child(tree, child), entered: false, overlay_root: child_overlay_root };
            self.visits[self.len] = Some(child_visit);
            self.len += 1;
            return RetainedPaintWalkStep::Scalar;
        }
        self.visits[index] = None;
        self.len = index;
        RetainedPaintWalkStep::Scalar
    }
}

fn intersect_rect(parent: Option<semio_framework_ui_theme::geometry::Rect>, rect: semio_framework_ui_theme::geometry::Rect) -> Option<semio_framework_ui_theme::geometry::Rect> {
    let Some(parent) = parent else { return Some(rect) };
    let x = parent.x.max(rect.x);
    let y = parent.y.max(rect.y);
    let right = (parent.x + parent.w).min(rect.x + rect.w);
    let bottom = (parent.y + parent.h).min(rect.y + rect.h);
    Some(semio_framework_ui_theme::geometry::Rect::new(x, y, (right - x).max(0.0), (bottom - y).max(0.0)))
}

pub fn offset_rect(rect: semio_framework_ui_theme::geometry::Rect, x: f32, y: f32) -> semio_framework_ui_theme::geometry::Rect {
    semio_framework_ui_theme::geometry::Rect::new(rect.x + x, rect.y + y, rect.w, rect.h)
}

pub fn retained_node_clip(tree: &UiTree, node: NodeId) -> Option<semio_framework_ui_theme::geometry::Rect> {
    if tree.node(node).is_some_and(|node| matches!(&node.spec.0, UiNode::Select(_)) && node.state.open) {
        return None;
    }
    let mut path = [None; RETAINED_PAINT_DEPTH_CREDITS];
    let mut len = 0;
    let mut cursor = Some(node);
    while let Some(id) = cursor {
        if len == RETAINED_PAINT_DEPTH_CREDITS {
            return Some(semio_framework_ui_theme::geometry::Rect::default());
        }
        path[len] = Some(id);
        len += 1;
        cursor = tree.node(id).and_then(|node| node.parent);
    }
    let mut origin = (0.0_f32, 0.0_f32);
    let mut clip = None;
    for index in (0..len).rev() {
        let id = path[index]?;
        if let Some(overlay_origin) = tree.overlay_walk_origin(id) {
            origin = overlay_origin;
            clip = None;
        }
        let layout = tree.accepted_layout(id)?;
        let rect = semio_framework_ui_theme::geometry::Rect::new(origin.0 + layout.x, origin.1 + layout.y, layout.width, layout.height);
        if id == node {
            return clip;
        }
        if tree.node(id).is_some_and(|node| node.flags.contains(NodeFlags::CLIPS_CHILDREN)) {
            clip = intersect_rect(clip, rect);
        }
        origin = tree.child_walk_origin(id, origin)?;
    }
    clip
}

fn retained_first_child(tree: &UiTree, id: NodeId) -> Option<NodeId> {
    tree.disclosure_open(id).unwrap_or(true).then(|| tree.node(id).and_then(|node| node.first_child)).flatten()
}

#[derive(Clone, Copy)]
pub enum RetainedPaintPhase {
    Synchronize,
    /// 🔓️ One open overlay's own chrome per step — the modal backdrop a `Dialog`/`CommandPalette`
    /// sits behind and the glass surface every floating overlay sits on. React portals these and the
    /// browser paints them for free; this target has to, and until ticket 26/09/17 packet W15a it
    /// never did on the per-frame path: `paint_overlay_backdrop`/`paint_overlay_surface` existed but
    /// were reachable only through the `🪟️OverlayApi` façade, which no host called, so an overlay
    /// that was not hand-pumped showed its content with no surface under it at all.
    Overlays,
    Paint,
    Scenes,
    Tooltip,
    /// 📚️ Re-derives this window's pointer registry from the SAME walk, and therefore the same
    /// accumulated origin, the paint phase just drew from — one geometry for what is painted and
    /// what is hit (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    Hits,
    Publish,
    Complete,
    Fault,
}

/// 📊️ What one window's own retained paint contributed to a frame: non-empty draw layers, quad
/// instances, and the glyph subset of those quads.
///
/// The production path is `frame_into_step`, which paints into the CALLER's draw list and never
/// publishes into `UiWindow::draw` — so `Ui::draw_list` (and every probe reading it) reports an empty
/// list no matter how much the window painted. This census is measured as the DELTA the window's own
/// paint frame appended to whatever target it was handed, so it answers the same question for both
/// paint entries (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-document-reconcile-2026-09-12.md`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiFramePaintCensus {
    pub layers: usize,
    pub quads: usize,
    pub glyphs: usize,
    /// 🌍️ The 3d half of the same frame. A `World3d` window's whole output is `ScenePass3d`s —
    /// instanced mesh draws, translucent draws, textured draws and the grid's line draws — and none
    /// of it is a `UiInstance`, so a census counting only quads reported `0` for a surface that was
    /// drawing a solid (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). A layer a scene pass targets now
    /// counts as non-empty for the same reason.
    pub scene_passes: usize,
    pub scene_draws: usize,
    pub scene_instances: usize,
}

impl UiFramePaintCensus {
    pub fn of(draw: &DrawList) -> Self {
        let mut census = Self::default();
        let scene_layers: std::collections::BTreeSet<usize> = draw.scene_passes.iter().map(|pass| pass.layer_index).collect();
        for (index, layer) in draw.layers.iter().enumerate() {
            if !layer.ui_instances.is_empty()
                || !layer.raster_instances.is_empty()
                || !layer.vector_vertices.is_empty()
                || !layer.overlay_ui_instances.is_empty()
                || !layer.overlay_raster_instances.is_empty()
                || !layer.overlay_vector_vertices.is_empty()
                || scene_layers.contains(&index)
            {
                census.layers += 1;
            }
            for instance in layer.ui_instances.iter().chain(layer.overlay_ui_instances.iter()) {
                census.quads += 1;
                if instance.params[2] == semio_framework_ui_draw_contract::KIND_GLYPH || instance.params[2] == semio_framework_ui_draw_contract::KIND_AFFINE_GLYPH {
                    census.glyphs += 1;
                }
            }
        }
        census.scene_passes = draw.scene_passes.len();
        for pass in &draw.scene_passes {
            census.scene_draws += pass.shadow_draws.len() + pass.draws.len() + pass.translucent_draws.len() + pass.material_draws.len() + pass.textured_draws.len() + pass.line_draws.len();
            census.scene_instances += pass.shadow_draws.iter().chain(pass.draws.iter()).chain(pass.translucent_draws.iter()).map(|draw| draw.instances.len()).sum::<usize>()
                + pass.material_draws.iter().map(|draw| draw.instances.len()).sum::<usize>()
                + pass.textured_draws.iter().map(|draw| draw.instances.len()).sum::<usize>();
        }
        census
    }

    pub fn since(self, baseline: Self) -> Self {
        Self {
            layers: self.layers.saturating_sub(baseline.layers),
            quads: self.quads.saturating_sub(baseline.quads),
            glyphs: self.glyphs.saturating_sub(baseline.glyphs),
            scene_passes: self.scene_passes.saturating_sub(baseline.scene_passes),
            scene_draws: self.scene_draws.saturating_sub(baseline.scene_draws),
            scene_instances: self.scene_instances.saturating_sub(baseline.scene_instances),
        }
    }
}

/// 🖱️ One `ComponentScene` leaf resolved under a point by [`Ui::scene_at`] — the surface's own
/// identity plus the ABSOLUTE (window-local) rect it was last laid out at, which is the same rect
/// every per-kind pointer/pick resolver is expressed against.
#[derive(Clone, Debug, PartialEq)]
pub struct UiSceneHit {
    pub node: NodeId,
    pub surface_id: String,
    pub controller_id: String,
    pub kind: semio_framework_ui_declaration::ui::SurfaceKind,
    pub rect: semio_framework_ui_theme::geometry::Rect,
}

/// 🔑️ `node`'s own absolute rect plus its scene identity, when it IS a `ComponentScene` leaf — the
/// ONE geometry answer `UiTree::absolute_rect` gives, so a scene inside an open floating overlay
/// resolves at that overlay's placement exactly as the paint and hit walks put it there.
pub fn scene_slot_for_node_absolute(tree: &UiTree, id: NodeId) -> Option<UiSceneHit> {
    let UiNode::ComponentScene(scene) = &tree.node(id)?.spec.0 else { return None };
    Some(UiSceneHit { node: id, surface_id: scene.surface_id.clone(), controller_id: scene.controller_id.clone(), kind: scene.component_kind, rect: tree.absolute_rect(id)? })
}

/// 📋️ Appends `node`'s registry entry, if it has one, at the ABSOLUTE rect the paint walk just
/// painted it at — `origin` is that walk's own accumulated offset, so the two can never diverge.
fn push_retained_hit(out: &mut Vec<RetainedHitRegistration>, overlay_hits: &mut usize, overlay: bool, mut registration: RetainedHitRegistration) {
    registration.overlay = overlay;
    if overlay {
        out.push(registration);
        *overlay_hits += 1;
    } else {
        out.insert(out.len().saturating_sub(*overlay_hits), registration);
    }
}

fn push_clipped_retained_hit(out: &mut Vec<RetainedHitRegistration>, overlay_hits: &mut usize, overlay: bool, clip: Option<semio_framework_ui_theme::geometry::Rect>, mut registration: RetainedHitRegistration) {
    let Some(rect) = intersect_rect(clip, registration.rect) else { return };
    if rect.w <= 0.0 || rect.h <= 0.0 {
        return;
    }
    registration.rect = rect;
    push_retained_hit(out, overlay_hits, overlay, registration);
}

pub fn register_retained_hit(
    tree: &UiTree,
    theme: &Theme,
    driver_drag: UiDriverDrag,
    reversed: bool,
    inline: ui_contract::FlowInline,
    node: NodeId,
    origin_x: f32,
    origin_y: f32,
    clip: Option<semio_framework_ui_theme::geometry::Rect>,
    overlay: bool,
    overlay_hits: &mut usize,
    out: &mut Vec<RetainedHitRegistration>,
) {
    if out.len() >= RETAINED_HIT_REGISTRY_CAPACITY {
        return;
    }
    let Some(layout) = tree.accepted_layout(node) else { return };
    let authored_rect = semio_framework_ui_theme::geometry::Rect::new(origin_x + layout.x, origin_y + layout.y, layout.width, layout.height);
    let Some(visible_rect) = intersect_rect(clip, authored_rect) else { return };
    if visible_rect.w <= 0.0 || visible_rect.h <= 0.0 {
        return;
    }
    let metrics = semio_framework_ui_wgpu_mounted_layout::retained_tree_row_metrics(tree, node, &TreeRowMetrics::from_theme(theme).with_inline(inline));
    let overlay = overlay || tree.is_open_select_popup_row(node);
    if let Some(registration) = retained_hit_registration(tree, node, authored_rect, &metrics, driver_drag, reversed) {
        push_clipped_retained_hit(out, overlay_hits, overlay, clip, registration);
    }
    if out.len() < RETAINED_HIT_REGISTRY_CAPACITY {
        if let Some(registration) = retained_tree_drag_handle_registration(tree, node, authored_rect, &metrics, driver_drag) {
            push_clipped_retained_hit(out, overlay_hits, overlay, clip, registration);
        }
    }
    if out.len() < RETAINED_HIT_REGISTRY_CAPACITY {
        if let Some(registration) = retained_tree_chevron_registration(tree, node, authored_rect, &metrics, reversed) {
            push_clipped_retained_hit(out, overlay_hits, overlay, clip, registration);
        }
    }
    let Some((select_id, popup)) = tree.node(node).and_then(|node| match &node.spec.0 {
        UiNode::Select(select) => node.state.select_popup.map(|popup| (select.id.as_str(), popup)),
        _ => None,
    }) else {
        return;
    };
    let Some(local) = tree.absolute_rect(node) else { return };
    let popup = popup.translated(authored_rect.x - local.x, authored_rect.y - local.y);
    for (rect, up) in [(popup.up, true), (popup.down, false)] {
        let Some(rect) = rect else { continue };
        if out.len() >= RETAINED_HIT_REGISTRY_CAPACITY {
            return;
        }
        push_retained_hit(
            out,
            overlay_hits,
            true,
            RetainedHitRegistration { node, rect, overlay: true, scene: None, kind: HitKind::DropdownItem, control_id: semio_framework_ui_select_behavior::select_scroll_control_id(select_id, up), action: None, drag_axis: None, drag_data: None },
        );
    }
}

/// 🎞️ How many open overlays one frame paints chrome for. A fixed array, not a `Vec`: the ladder
/// must not grow a per-frame working set, and a surface with more than this many simultaneously open
/// floating overlays is already outside anything React's own shell produces.
pub const UI_FRAME_OVERLAY_CHROME: usize = 8;

pub struct RetainedPaintFrame {
    pub phase: RetainedPaintPhase,
    pub walk: RetainedPaintWalk,
    pub candidate: DrawList,
    pub sync_node: Option<NodeId>,
    pub node_sync: RetainedInteractiveSyncCursor,
    pub paint_node: Option<(NodeId, f32, f32)>,
    /// 🪣️ Whether the node `paint_node` names paints into the overlay bucket — true exactly when it
    /// sits under one of `overlay_chrome`'s roots.
    pub paint_overlay: bool,
    /// 🔂️ The overlays this frame owes chrome for, captured once when the frame opened so every
    /// step of the `Overlays` phase paints against the same placements the layout published.
    pub overlay_chrome: [Option<UiOverlayPlacement>; UI_FRAME_OVERLAY_CHROME],
    /// 🔢️ How far the `Overlays` phase has walked `overlay_chrome`.
    pub overlay_index: usize,
    pub node_paint: RetainedNodePaintCursor,
    pub scene_node: Option<(NodeId, f32, f32)>,
    pub scene_paint: ScenePaintCursor,
    pub tooltip: Option<RetainedTooltipPaint>,
    pub interaction_epoch: u64,
    pub revision: u64,
    pub theme_revision: u64,
    pub viewport_revision: u64,
    pub viewport_origin: [f32; 2],
    pub baseline: UiFramePaintCensus,
    pub hit_candidates: Vec<RetainedHitRegistration>,
    /// 🩺️ Which sub-step drove this frame terminal — a `UiFrameStep::Fault` is otherwise
    /// undiagnosable from outside the engine.
    pub fault_site: Option<&'static str>,
    pub progress: u64,
}

impl RetainedPaintFrame {
    pub fn note_progress(&mut self) {
        self.progress = self.progress.saturating_add(1);
    }
}
//#endregion 🔖️UiWindow
