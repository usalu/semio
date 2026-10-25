//! 📌️ Retained hit registration: projects laid-out retained nodes onto the pointer registry at the band paint owns.

use semio_framework_ui_declaration::layout::ActionDescriptor;
use semio_framework_ui_theme::geometry::Rect;
use std::collections::HashMap;
use semio_framework_ui_wgpu_input::{DragAxis, HitKind, HitTarget};

//#region 🎯️RetainedHitRegistry
/// ➡️ One interactive retained node, projected onto this region's flat pointer registry.
///
/// 🩸️ The shell's registry used to hold the chrome and NOTHING of any retained window body: a
/// pointer inside a published `mounted_layout` row rect answered the WINDOW's
/// `HitKind::ScrollRegion`, so no row action dispatched, no wheel reached a scene and no hover
/// resolved (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️wgpu-runtime-mailbox-dispatch-2026-09-13.md` §6). `rect` is minted by the SAME accumulated
/// origin the retained paint walk paints at, so a row can never again be drawn at one rectangle and
/// hit-tested at another.
#[derive(Clone, Debug, PartialEq)]
pub struct RetainedSceneHit {
    pub surface_id: String,
    pub kind: semio_framework_ui_declaration::ui::SurfaceKind,
}

/// 🎬️ Retained pointer geometry and the canonical scene identity that produced it.
#[derive(Clone, Debug, PartialEq)]
pub struct RetainedHitRegistration {
    pub node: semio_framework_ui_arena::NodeId,
    pub rect: Rect,
    pub overlay: bool,
    pub scene: Option<RetainedSceneHit>,
    pub kind: HitKind,
    pub control_id: String,
    pub action: Option<ActionDescriptor>,
    pub drag_axis: Option<DragAxis>,
    pub drag_data: Option<HashMap<String, String>>,
}

impl RetainedHitRegistration {
    /// 🎯️ The registry entry a host pushes into its own `InputState`.
    pub fn to_hit_target(&self) -> HitTarget<ActionDescriptor> {
        HitTarget { rect: self.rect, event: self.action.clone(), control_id: Some(self.control_id.clone()), kind: self.kind, drag_axis: self.drag_axis, drag_data: self.drag_data.clone() }
    }
}

/// 🌳️ Which row of its owning `Tree` a synthesized `Stack` is, re-derived from that tree's own
/// still-intact spec by key — the same discriminator `mounted_layout::tree_row_kind` measures with,
/// so the row a pointer resolves and the row the layout published are one classification.
enum RetainedTreeRow<'a> {
    Section(&'a semio_framework_ui_declaration::ui::UiTreeSectionNode),
    Item(&'a semio_framework_ui_declaration::ui::UiTreeItemNode),
}

fn retained_tree_row<'a>(tree: &'a semio_framework_ui_retained_tree::tree::UiTree, id: semio_framework_ui_arena::NodeId) -> Option<RetainedTreeRow<'a>> {
    let semio_framework_ui_retained_tree::tree::NodeKey::Explicit(key) = &tree.node(id)?.key else { return None };
    let owner = crate::owning_tree_spec(tree, id)?;
    if let Some(section) = owner.sections.iter().find(|section| &section.id == key) {
        return Some(RetainedTreeRow::Section(section));
    }
    owner.sections.iter().find_map(|section| crate::find_tree_item(&section.items, key, 0)).map(RetainedTreeRow::Item)
}

pub fn retained_tree_section_header_band(rect: Rect, header_height: f32, reversed: bool) -> Rect {
    semio_framework_ui_wgpu_layout::layout::tree_section_header_band(rect, header_height, reversed)
}

/// 🖱️ The `HitKind`/control id an engine surface canvas registers under. `World3d` is its own kind;
/// every other bespoke-dispatch surface reuses the `ScrollRegion` + `.pane`/`.map` convention
/// `ShellState::scroll_region_is_scene_surface` already reads, so a wheel over one propagates to the
/// scene instead of scrolling the window that hosts it.
pub fn retained_scene_hit(scene: &semio_framework_ui_declaration::ui::UiComponentSceneNode) -> (HitKind, String) {
    use semio_framework_ui_declaration::ui::SurfaceKind;
    match scene.component_kind {
        SurfaceKind::World3d => (HitKind::World3d, scene.surface_id.clone()),
        SurfaceKind::NodeGraph | SurfaceKind::Board2d => (HitKind::ScrollRegion, format!("{}.pane", scene.surface_id)),
        SurfaceKind::TiledMap => (HitKind::ScrollRegion, format!("{}.map", scene.surface_id)),
        _ => (HitKind::ComponentScene, scene.surface_id.clone()),
    }
}

/// 🖌️ Projects one laid-out retained node onto the registry at the same band paint owns.
pub fn retained_hit_registration(
    tree: &semio_framework_ui_retained_tree::tree::UiTree,
    id: semio_framework_ui_arena::NodeId,
    rect: Rect,
    metrics: &semio_framework_ui_wgpu_layout::layout::TreeRowMetrics,
    driver_drag: semio_framework_ui_wgpu_layout::chrome::UiDriverDrag,
    reversed: bool,
) -> Option<RetainedHitRegistration> {
    use semio_framework_ui_declaration::ui::UiNode;
    let node = tree.node(id)?;
    if !node.spec.0.presence().visible() || rect.w <= 0.0 || rect.h <= 0.0 {
        return None;
    }
    let entry = |kind: HitKind, control_id: String, action: Option<ActionDescriptor>, rect: Rect| Some(RetainedHitRegistration { node: id, rect, overlay: false, scene: None, kind, control_id, action, drag_axis: None, drag_data: None });
    match &node.spec.0 {
        UiNode::Stack(stack) => match retained_tree_row(tree, id) {
            Some(RetainedTreeRow::Section(section)) => {
                let band = retained_tree_section_header_band(rect, semio_framework_ui_wgpu_layout::layout::tree_section_header_height(section, metrics), reversed);
                if band.h <= 0.0 {
                    return None;
                }
                entry(HitKind::TreeItem, format!("section.chevron.{}", section.id), stack.activate.clone(), band)
            }
            Some(RetainedTreeRow::Item(item)) => {
                let band = retained_tree_section_header_band(rect, metrics.row_height, reversed);
                if band.h <= 0.0 {
                    return None;
                }
                let draggable = item.draggable.unwrap_or(false);
                let surface_drag = draggable && driver_drag == semio_framework_ui_wgpu_layout::chrome::UiDriverDrag::Surface;
                Some(RetainedHitRegistration {
                    node: id,
                    rect: band,
                    overlay: false,
                    scene: None,
                    kind: HitKind::TreeItem,
                    control_id: format!("tree.label.{}", item.id),
                    action: stack.activate.clone().or_else(|| item.action.clone()),
                    drag_axis: surface_drag.then_some(DragAxis::Both),
                    drag_data: surface_drag.then(|| item.drag_data.clone().unwrap_or_default()),
                })
            }
            None => {
                let activate = stack.activate.clone()?;
                let control_id = stack.id.clone().filter(|id| !id.is_empty()).unwrap_or_else(|| activate.action.clone());
                entry(HitKind::Button, control_id, Some(activate), rect)
            }
        },
        UiNode::Button(button) => entry(HitKind::Button, button.id.clone().unwrap_or_else(|| button.action.action.clone()), Some(button.action.clone()), rect),
        UiNode::Section(section) if section.label.is_some() => {
            let band = Rect::new(rect.x, rect.y, rect.w, crate::retained_section_title_height(tree, id).min(rect.h));
            (band.h > 0.0).then(|| RetainedHitRegistration { node: id, rect: band, overlay: false, scene: None, kind: HitKind::Toggle, control_id: section.id.clone(), action: None, drag_axis: None, drag_data: None })
        }
        UiNode::Input(input) => entry(HitKind::Input, input.id.clone(), None, rect),
        UiNode::Select(select) => entry(HitKind::Select, select.id.clone(), None, rect),
        UiNode::Toggle(toggle) => entry(HitKind::Toggle, toggle.id.clone(), None, if toggle.appearance == ui_contract::ToggleAppearance::Checkbox { semio_framework_ui_wgpu_layout::layout::tree_checkbox_hit_rect(rect, metrics.inline) } else { rect }),
        UiNode::Slider(slider) => Some(RetainedHitRegistration { node: id, rect, overlay: false, scene: None, kind: HitKind::Slider, control_id: slider.id.clone(), action: None, drag_axis: Some(DragAxis::Horizontal), drag_data: None }),
        UiNode::NumberStepper(stepper) => entry(HitKind::NumberStepper, stepper.id.clone(), None, rect),
        UiNode::Ring(ring) => Some(RetainedHitRegistration { node: id, rect, overlay: false, scene: None, kind: HitKind::Ring, control_id: ring.id.clone(), action: None, drag_axis: Some(DragAxis::Both), drag_data: None }),
        UiNode::IconSelect(select) => entry(HitKind::IconSelect, select.id.clone(), None, rect),
        UiNode::ComponentScene(scene) => {
            let (kind, control_id) = retained_scene_hit(scene);
            let mut registration = entry(kind, control_id, None, rect)?;
            registration.scene = Some(RetainedSceneHit { surface_id: scene.surface_id.clone(), kind: scene.component_kind });
            Some(registration)
        }
        _ => None,
    }
}

pub fn retained_tree_chevron_registration(tree: &semio_framework_ui_retained_tree::tree::UiTree, id: semio_framework_ui_arena::NodeId, rect: Rect, metrics: &semio_framework_ui_wgpu_layout::layout::TreeRowMetrics, reversed: bool) -> Option<RetainedHitRegistration> {
    let RetainedTreeRow::Item(item) = retained_tree_row(tree, id)? else { return None };
    item.items.as_deref().filter(|items| !items.is_empty())?;
    let depth = tree.tree_item_depth(id)?;
    let rect = semio_framework_ui_wgpu_layout::layout::tree_item_chevron_rect(rect, depth, metrics, reversed);
    (rect.w > 0.0 && rect.h > 0.0).then(|| RetainedHitRegistration { node: id, rect, overlay: false, scene: None, kind: HitKind::TreeItem, control_id: format!("tree.chevron.{}", item.id), action: None, drag_axis: None, drag_data: None })
}

pub fn retained_tree_drag_handle_registration(
    tree: &semio_framework_ui_retained_tree::tree::UiTree,
    id: semio_framework_ui_arena::NodeId,
    rect: Rect,
    metrics: &semio_framework_ui_wgpu_layout::layout::TreeRowMetrics,
    driver_drag: semio_framework_ui_wgpu_layout::chrome::UiDriverDrag,
) -> Option<RetainedHitRegistration> {
    use semio_framework_ui_declaration::ui::UiNode;
    use semio_framework_ui_wgpu_layout::layout::{tree_drag_handle_rect, tree_drag_role, TreeDragRole};
    if driver_drag != semio_framework_ui_wgpu_layout::chrome::UiDriverDrag::Handle || rect.w <= 0.0 || rect.h <= 0.0 {
        return None;
    }
    let node = tree.node(id)?;
    if !node.spec.0.presence().visible() {
        return None;
    }
    let UiNode::Stack(_) = &node.spec.0 else { return None };
    let RetainedTreeRow::Item(item) = retained_tree_row(tree, id)? else { return None };
    let role = tree_drag_role(item)?;
    let relative = tree_drag_handle_rect(rect.w, metrics);
    let handle = Rect::new(rect.x + relative.x, rect.y + relative.y, relative.w.min(rect.w), relative.h.min(metrics.row_height.min(rect.h)));
    if handle.w <= 0.0 || handle.h <= 0.0 {
        return None;
    }
    Some(RetainedHitRegistration {
        node: id,
        rect: handle,
        overlay: false,
        scene: None,
        kind: HitKind::TreeDragHandle,
        control_id: format!("tree.drag.{}.{}", if role == TreeDragRole::Transfer { "transfer" } else { "sort" }, item.id),
        action: None,
        drag_axis: Some(DragAxis::Both),
        drag_data: Some(item.drag_data.clone().unwrap_or_default()),
    })
}

//#endregion 🎯️RetainedHitRegistry
