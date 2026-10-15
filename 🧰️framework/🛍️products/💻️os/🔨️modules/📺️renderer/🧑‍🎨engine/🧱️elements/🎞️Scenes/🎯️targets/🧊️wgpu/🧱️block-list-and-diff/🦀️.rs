//! 🧱️ Block list and diff view scenes.

use super::*;

//#region BlockList
use ui_wgpu::wgpu::{BlockListSelectionTarget,BlockListBlock,BlockListStep,BlockListPaletteEntry};

/** 🧩️ What one laid-out block-list target paints as. Keeping the geometry (`BlockListTarget`)
 * and the appearance apart is what lets the paint and the pointer path share ONE layout pass: the
 * paint walks the plan forwards, the hit test walks it backwards so a card's own buttons win over
 * the card band they sit in. */
enum BlockListPaint {
    AddStep,
    StepCard { title: String, description: Option<String>, selected: bool, leading_inset: f32, surface_grip: bool },
    BlockRow { label: String, kind: String, selected: bool, leading_inset: f32, surface_grip: bool },
    IconButton { icon: &'static str, target_label: String },
    DragHandle { icon: &'static str },
    PaletteEntry { icon_id: String, label: String, leading_inset: f32 },
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum BlockListRole {
    Action,
    Step { step_id: String, index: usize },
    StepHandle { step_id: String, index: usize },
    Block { step_id: String, block_id: String, index: usize },
    BlockHandle { step_id: String, block_id: String, index: usize },
    Palette { kind: String },
    PaletteHandle { kind: String },
}

/** 🧩️ One laid-out `SurfaceKind::BlockList` target: where it sits, the control id it is drawn
 * and hovered under, and the action a press on it dispatches (`None` for a disabled reorder button —
 * React renders those disabled, so the press must fall through to nothing, not to the card). */
struct BlockListTarget {
    pub(crate) rect: Rect,
    pub(crate) control_id: String,
    pub(crate) action: Option<ActionDescriptor>,
    pub(crate) paint: BlockListPaint,
    pub(crate) role: BlockListRole,
}

/// 🧩️ A block list's whole laid-out surface: the two chrome bands, the scrolling step body, and the
/// flat target list split into "before the body" / "inside the body" / "palette rail".
struct BlockListPlan {
    header: Rect,
    pub(crate) body: Rect,
    palette_rect: Rect,
    palette_target_step_id: Option<String>,
    targets: Vec<BlockListTarget>,
    body_range: std::ops::Range<usize>,
}

/// 📐️ Lays out one block list. The action payloads are the ones `🧩️BlockListHost/🟦️.tsx` dispatches
/// verbatim (`addStep`/`removeStep`/`moveStep`/`addBlock`/`removeBlock`/`moveBlock`); reordering is
/// driven by explicit move-up/move-down buttons rather than the React host's dnd-kit drag, because
/// this renderer has no cross-frame list-drag primitive.
fn block_list_action(scene: &UiComponentSceneNode, action: &str, args: semio_framework::DslValue) -> ActionDescriptor {
    ActionDescriptor { controller_id: scene.controller_id.clone(), action: action.to_string(), args: Some(args) }
}

fn block_list_selection_action(scene: &UiComponentSceneNode, target: Option<&BlockListSelectionTarget>) -> Option<ActionDescriptor> {
    let domain = scene.block_list.as_ref()?.domain_id.as_deref().filter(|id| !id.is_empty())?;
    let target = target.filter(|target| !target.id.is_empty() && !target.granularity.is_empty())?;
    Some(block_list_action(scene, "interactionSelect", semio_framework::dsl_value!({ "domainId": domain, "targets": json!([target]).to_string(), "merge": "replace", "method": "pick" })))
}

fn block_list_handle_rect(row: Rect, theme: &Theme) -> Rect {
    let size = theme.control_height_small.min(row.h).min(row.w.max(0.0));
    Rect::new(row.x + theme.padding_standard, row.y + (theme.control_height.min(row.h) - size) * 0.5, size.min((row.w - theme.padding_standard).max(0.0)), size)
}

fn block_list_palette_target_step_id<'a>(steps: &'a [BlockListStep], selected_id: Option<&str>) -> Option<&'a str> {
    if let Some(selected_id) = selected_id {
        if let Some(step) = steps.iter().find(|step| step.id == selected_id) {
            return Some(step.id.as_str());
        }
        if let Some(step) = steps.iter().find(|step| step.blocks.iter().any(|block| block.id == selected_id)) {
            return Some(step.id.as_str());
        }
    }
    steps.first().map(|step| step.id.as_str())
}

fn block_list_plan(scene: &UiComponentSceneNode, bounds: Rect, theme: &Theme, driver_drag: UiDriverDrag) -> BlockListPlan {
    let pad = theme.padding_standard;
    let row_h = theme.control_height;
    let btn_w = theme.control_height_small;
    let palette_w = (bounds.w * 0.22).clamp(theme.control_height * 5.0, theme.control_height * 8.0);
    let main = Rect::new(bounds.x, bounds.y, (bounds.w - palette_w).max(0.0), bounds.h);
    let palette_rect = Rect::new(main.x + main.w, bounds.y, palette_w, bounds.h);
    let header = Rect::new(main.x, main.y, main.w, row_h);
    let body = Rect::new(main.x, main.y + row_h, main.w, (main.h - row_h).max(0.0));
    let steps = scene.block_list.as_ref().map(|list| list.steps.as_slice()).unwrap_or_default();
    let palette = scene.block_list.as_ref().map(|list| list.palette.as_slice()).unwrap_or_default();
    let selected_id = scene.block_list.as_ref().and_then(|list| list.selected_id.clone());
    let selected_id = selected_id.as_deref();
    let palette_target_step_id = block_list_palette_target_step_id(&steps, selected_id).map(str::to_owned);

    let mut targets = Vec::new();
    targets.push(BlockListTarget {
        rect: Rect::new(header.x + header.w - theme.control_height * 4.0 - pad, header.y + pad, theme.control_height * 4.0, (row_h - pad * 2.0).max(0.0)),
        control_id: format!("{}.addStep", scene.host_id),
        action: Some(block_list_action(scene, "addStep", semio_framework::dsl_value!({}))),
        paint: BlockListPaint::AddStep,
        role: BlockListRole::Action,
    });
    let body_start = targets.len();

    let scroll = scroll_offset(&scene.host_id, "blockList");
    let mut y = body.y - scroll;
    for (step_index, step) in steps.iter().enumerate() {
        let block_count = step.blocks.len();
        let description_h = step.description.as_ref().map_or(0.0, |_| theme.font_size_small + pad);
        let step_h = row_h + description_h + block_count as f32 * row_h + pad * 2.0;
        let step_rect = Rect::new(body.x + pad, y, (body.w - pad * 2.0).max(0.0), step_h);
        let handle_rect = block_list_handle_rect(step_rect, theme);
        let leading_inset = handle_rect.w + theme.gap_standard;
        targets.push(BlockListTarget {
            rect: step_rect,
            control_id: format!("{}.step.{}", scene.host_id, step.id),
            action: block_list_selection_action(scene, step.target.as_ref()),
            paint: BlockListPaint::StepCard { title: step.title.clone(), description: step.description.clone(), selected: selected_id == Some(step.id.as_str()), leading_inset, surface_grip: driver_drag == UiDriverDrag::Surface },
            role: BlockListRole::Step { step_id: step.id.clone(), index: step_index },
        });
        let btn_y = step_rect.y + (row_h - theme.control_height_small) * 0.5;
        targets.push(BlockListTarget {
            rect: Rect::new(step_rect.x + step_rect.w - pad - btn_w, btn_y, btn_w, theme.control_height_small),
            control_id: format!("{}.step.{}.remove", scene.host_id, step.id),
            action: Some(block_list_action(scene, "removeStep", semio_framework::dsl_value!({ "stepId": step.id }))),
            paint: BlockListPaint::IconButton { icon: "trash-2", target_label: step.title.clone() },
            role: BlockListRole::Action,
        });
        if driver_drag == UiDriverDrag::Handle {
            targets.push(BlockListTarget {
                rect: handle_rect,
                control_id: format!("{}.step.{}.drag", scene.host_id, step.id),
                action: None,
                paint: BlockListPaint::DragHandle { icon: "grip-vertical" },
                role: BlockListRole::StepHandle { step_id: step.id.clone(), index: step_index },
            });
        }
        let mut inner_y = step_rect.y + row_h + description_h;
        for (block_index, block) in step.blocks.iter().enumerate() {
            let block_rect = Rect::new(step_rect.x + pad, inner_y, (step_rect.w - pad * 2.0).max(0.0), row_h);
            let block_handle_rect = block_list_handle_rect(block_rect, theme);
            let block_leading_inset = block_handle_rect.w + theme.gap_standard;
            targets.push(BlockListTarget {
                rect: block_rect,
                control_id: format!("{}.block.{}", scene.host_id, block.id),
                action: block_list_selection_action(scene, block.target.as_ref()),
                paint: BlockListPaint::BlockRow { label: block.label.clone(), kind: block.kind.clone(), selected: selected_id == Some(block.id.as_str()), leading_inset: block_leading_inset, surface_grip: driver_drag == UiDriverDrag::Surface },
                role: BlockListRole::Block { step_id: step.id.clone(), block_id: block.id.clone(), index: block_index },
            });
            let block_btn_y = block_rect.y + (row_h - theme.control_height_small) * 0.5;
            targets.push(BlockListTarget {
                rect: Rect::new(block_rect.x + block_rect.w - pad - btn_w, block_btn_y, btn_w, theme.control_height_small),
                control_id: format!("{}.block.{}.remove", scene.host_id, block.id),
                action: Some(block_list_action(scene, "removeBlock", semio_framework::dsl_value!({ "stepId": step.id, "blockId": block.id }))),
                paint: BlockListPaint::IconButton { icon: "trash-2", target_label: block.label.clone() },
                role: BlockListRole::Action,
            });
            if driver_drag == UiDriverDrag::Handle {
                targets.push(BlockListTarget {
                    rect: block_handle_rect,
                    control_id: format!("{}.block.{}.drag", scene.host_id, block.id),
                    action: None,
                    paint: BlockListPaint::DragHandle { icon: "grip-vertical" },
                    role: BlockListRole::BlockHandle { step_id: step.id.clone(), block_id: block.id.clone(), index: block_index },
                });
            }
            inner_y += row_h;
        }
        y += step_h + theme.gap_standard;
    }
    let body_end = targets.len();

    let mut py = palette_rect.y + pad;
    for entry in &palette {
        let palette_row = Rect::new(palette_rect.x + pad, py, (palette_rect.w - pad * 2.0).max(0.0), row_h);
        let palette_handle_rect = block_list_handle_rect(palette_row, theme);
        targets.push(BlockListTarget {
            rect: palette_row,
            control_id: format!("{}.palette.{}", scene.host_id, entry.block_kind),
            action: palette_target_step_id.as_ref().map(|step_id| block_list_action(scene, "addBlock", semio_framework::dsl_value!({ "stepId": step_id, "kind": entry.block_kind }))),
            paint: BlockListPaint::PaletteEntry {
                icon_id: entry.icon_id.clone(),
                label: entry.label.clone(),
                leading_inset: if driver_drag == UiDriverDrag::Handle && palette_target_step_id.is_some() { palette_handle_rect.w + theme.gap_standard } else { 0.0 },
            },
            role: BlockListRole::Palette { kind: entry.block_kind.clone() },
        });
        if driver_drag == UiDriverDrag::Handle && palette_target_step_id.is_some() {
            targets.push(BlockListTarget {
                rect: palette_handle_rect,
                control_id: format!("{}.palette.{}.drag", scene.host_id, entry.block_kind),
                action: None,
                paint: BlockListPaint::DragHandle { icon: "move" },
                role: BlockListRole::PaletteHandle { kind: entry.block_kind.clone() },
            });
        }
        py += row_h + theme.gap_standard;
    }
    BlockListPlan { header, body, palette_rect, palette_target_step_id, targets, body_range: body_start..body_end }
}

fn block_list_source_rect(plan: &BlockListPlan, role: &BlockListRole) -> Option<Rect> {
    match role {
        BlockListRole::StepHandle { step_id, index } => plan.targets.iter().find_map(|target| match &target.role {
            BlockListRole::Step { step_id: candidate, index: candidate_index } if candidate == step_id && candidate_index == index => Some(target.rect),
            _ => None,
        }),
        BlockListRole::BlockHandle { step_id, block_id, index } => plan.targets.iter().find_map(|target| match &target.role {
            BlockListRole::Block { step_id: candidate_step, block_id: candidate_block, index: candidate_index } if candidate_step == step_id && candidate_block == block_id && candidate_index == index => Some(target.rect),
            _ => None,
        }),
        BlockListRole::PaletteHandle { kind } => plan.targets.iter().find_map(|target| match &target.role {
            BlockListRole::Palette { kind: candidate } if candidate == kind => Some(target.rect),
            _ => None,
        }),
        BlockListRole::Step { .. } | BlockListRole::Block { .. } | BlockListRole::Palette { .. } => plan.targets.iter().find(|target| &target.role == role).map(|target| target.rect),
        BlockListRole::Action => None,
    }
}

pub(crate) fn block_list_transfer_start(scene: &UiComponentSceneNode, bounds: Rect, x: f32, y: f32, theme: &Theme, driver_drag: UiDriverDrag) -> Option<SceneListTransferStart> {
    scene.block_list.as_ref()?;
    let plan = block_list_plan(scene, bounds, theme, driver_drag);
    let inside_body = plan.body.contains(x, y);
    let target = plan.targets.iter().enumerate().rev().find_map(|(index, target)| {
        if plan.body_range.contains(&index) && !inside_body {
            return None;
        }
        target.rect.contains(x, y).then_some(target)
    })?;
    let source = match (&target.role, driver_drag) {
        (BlockListRole::StepHandle { step_id, index }, UiDriverDrag::Handle) | (BlockListRole::Step { step_id, index }, UiDriverDrag::Surface) => SceneListTransferSource::BlockListStep { step_id: step_id.clone(), index: *index },
        (BlockListRole::BlockHandle { step_id, block_id, index }, UiDriverDrag::Handle) | (BlockListRole::Block { step_id, block_id, index }, UiDriverDrag::Surface) => {
            SceneListTransferSource::BlockListBlock { step_id: step_id.clone(), block_id: block_id.clone(), index: *index }
        }
        (BlockListRole::PaletteHandle { kind }, UiDriverDrag::Handle) | (BlockListRole::Palette { kind }, UiDriverDrag::Surface) if plan.palette_target_step_id.is_some() => {
            SceneListTransferSource::BlockListPalette { kind: kind.clone(), mime: BLOCK_LIST_PALETTE_DRAG_MIME.to_string(), payload: kind.clone() }
        }
        _ => return None,
    };
    Some(SceneListTransferStart { source, rect: block_list_source_rect(&plan, &target.role)? })
}

fn block_list_steps(scene: &UiComponentSceneNode) -> Option<&[BlockListStep]> {
    Some(&scene.block_list.as_ref()?.steps)
}

fn block_list_source_is_current(scene: &UiComponentSceneNode, source: &SceneListTransferSource) -> bool {
    let Some(steps) = block_list_steps(scene) else { return false };
    match source {
        SceneListTransferSource::BlockListStep { step_id, index } => steps.get(*index).is_some_and(|step| &step.id == step_id),
        SceneListTransferSource::BlockListBlock { step_id, block_id, index } => steps.iter().find(|step| &step.id == step_id).and_then(|step| step.blocks.get(*index)).is_some_and(|block| &block.id == block_id),
        SceneListTransferSource::BlockListPalette { kind, mime, payload } => {
            mime == BLOCK_LIST_PALETTE_DRAG_MIME
                && payload == kind
                && block_list_palette_target_step_id(&steps, scene.block_list.as_ref().and_then(|list| list.selected_id.as_deref())).is_some()
                && scene.block_list.as_ref().is_some_and(|list| list.palette.iter().any(|entry| &entry.block_kind == kind))
        }
        SceneListTransferSource::TableRow { .. } => false,
    }
}

fn block_list_closest_target<'a>(plan: &'a BlockListPlan, center_x: f32, center_y: f32, accepts: impl Fn(&BlockListRole) -> bool) -> Option<&'a BlockListRole> {
    plan.targets
        .iter()
        .filter(|target| accepts(&target.role))
        .min_by(|left, right| {
            let distance = |rect: Rect| {
                let dx = rect.x + rect.w * 0.5 - center_x;
                let dy = rect.y + rect.h * 0.5 - center_y;
                dx * dx + dy * dy
            };
            distance(left.rect).total_cmp(&distance(right.rect))
        })
        .map(|target| &target.role)
}

pub(crate) fn block_list_transfer_drop_action(scene: &UiComponentSceneNode, bounds: Rect, x: f32, y: f32, window_id: &str, theme: &Theme, session: &SceneListTransferSession) -> Option<ActionDescriptor> {
    if session.source_window_id != window_id || session.source_host_id != scene.host_id || !block_list_source_is_current(scene, &session.source) {
        return None;
    }
    let plan = block_list_plan(scene, bounds, theme, session.driver_drag);
    let translated_center_x = session.source_rect.x + session.source_rect.w * 0.5 + (x - session.start_x);
    let translated_center_y = session.source_rect.y + session.source_rect.h * 0.5 + (y - session.start_y);
    match &session.source {
        SceneListTransferSource::BlockListStep { step_id, index: source_index } => {
            let BlockListRole::Step { index, .. } = block_list_closest_target(&plan, translated_center_x, translated_center_y, |role| matches!(role, BlockListRole::Step { .. }))? else { return None };
            (*index != *source_index).then(|| block_list_action(scene, "moveStep", semio_framework::dsl_value!({ "stepId": step_id, "index": index })))
        }
        SceneListTransferSource::BlockListBlock { step_id, block_id, index: source_index } => {
            let BlockListRole::Block { index, .. } = block_list_closest_target(&plan, translated_center_x, translated_center_y, |role| matches!(role, BlockListRole::Block { step_id: candidate, .. } if candidate == step_id))? else { return None };
            (*index != *source_index).then(|| block_list_action(scene, "moveBlock", semio_framework::dsl_value!({ "blockId": block_id, "fromStepId": step_id, "toStepId": step_id, "index": index })))
        }
        SceneListTransferSource::BlockListPalette { kind, mime, payload } if mime == BLOCK_LIST_PALETTE_DRAG_MIME && payload == kind => {
            let step_id = plan.targets.iter().rev().find_map(|target| match &target.role {
                BlockListRole::Step { step_id, .. } if target.rect.contains(x, y) && plan.body.contains(x, y) => Some(step_id.clone()),
                _ => None,
            })?;
            Some(block_list_action(scene, "addBlock", semio_framework::dsl_value!({ "stepId": step_id, "kind": kind })))
        }
        SceneListTransferSource::BlockListPalette { .. } => None,
        SceneListTransferSource::TableRow { .. } => None,
    }
}

/// 🖌️ Draws one planned block-list target.
fn paint_block_list_target(ctx: &mut FrameworkWidgetContext<'_>, target: &BlockListTarget, labels: BlockListChromeLabels) {
    let theme = ctx.theme;
    let pad = theme.padding_standard;
    let row_h = theme.control_height;
    let rect = target.rect;
    match &target.paint {
        BlockListPaint::AddStep => {
            render_widget(&WidgetNode::Button { id: Some(target.control_id.clone()), icon_id: Some("plus".into()), label: labels.add_step.into(), event: target.action.clone() }, rect, ctx);
        }
        BlockListPaint::StepCard { title, description, selected, leading_inset, surface_grip } => {
            ctx.draw.push_rounded([rect.x, rect.y, rect.w, rect.h], if *selected { theme.selected } else { theme.button }, theme.border_radius);
            draw_ink_rect_outline(ctx.draw, rect.x, rect.y, rect.w, rect.h, theme.border_normal, theme.stroke_hairline);
            if *surface_grip {
                if let Some(icons) = ctx.icons {
                    let handle = block_list_handle_rect(rect, theme);
                    let size = handle.w.min(handle.h);
                    push_icon(ctx.draw, icons, "grip-vertical", handle.x, handle.y, size, theme.text_muted);
                }
            }
            draw_text(ctx, title, rect.x + pad + leading_inset, rect.y + row_h * 0.65, theme.font_size_body, if *selected { theme.active_foreground } else { theme.text });
            if let Some(description) = description {
                draw_text(ctx, description, rect.x + pad, rect.y + row_h + theme.font_size_small, theme.font_size_small, theme.text_muted);
            }
        }
        BlockListPaint::BlockRow { label, kind, selected, leading_inset, surface_grip } => {
            if *selected {
                ctx.draw.push_rounded([rect.x, rect.y, rect.w, rect.h], theme.selected, theme.border_radius.min(4.0));
            }
            if *surface_grip {
                if let Some(icons) = ctx.icons {
                    let handle = block_list_handle_rect(rect, theme);
                    let size = handle.w.min(handle.h);
                    push_icon(ctx.draw, icons, "grip-vertical", handle.x, handle.y, size, theme.text_muted);
                }
            }
            draw_text(ctx, label, rect.x + pad + leading_inset, rect.y + row_h * 0.65, theme.font_size_small, if *selected { theme.active_foreground } else { theme.text });
            draw_text(ctx, kind, rect.x + rect.w * 0.5, rect.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
        }
        BlockListPaint::IconButton { icon, .. } => {
            render_widget(&WidgetNode::Button { id: Some(target.control_id.clone()), icon_id: Some((*icon).into()), label: String::new(), event: target.action.clone() }, rect, ctx);
        }
        BlockListPaint::DragHandle { icon } => {
            if let Some(icons) = ctx.icons {
                let size = rect.w.min(rect.h);
                push_icon(ctx.draw, icons, icon, rect.x + (rect.w - size) * 0.5, rect.y + (rect.h - size) * 0.5, size, theme.text_muted);
            }
        }
        BlockListPaint::PaletteEntry { icon_id, label, leading_inset } => {
            ctx.draw.push_rounded([rect.x, rect.y, rect.w, rect.h], theme.button, theme.border_radius);
            draw_ink_rect_outline(ctx.draw, rect.x, rect.y, rect.w, rect.h, theme.border_normal, theme.stroke_hairline);
            let icon_size = theme.control_height_small.min(rect.h);
            let icon_x = rect.x + pad + leading_inset;
            if let (Some(icons), Some(icon)) = (ctx.icons, IconName::from_str(icon_id)) {
                push_icon(ctx.draw, icons, icon.as_str(), icon_x, rect.y + (rect.h - icon_size) * 0.5, icon_size, theme.text_muted);
            }
            draw_text(ctx, label, icon_x + icon_size + theme.gap_standard, rect.y + row_h * 0.65, theme.font_size_small, theme.text);
        }
    }
}

fn block_list_accessibility_controls(plan: &BlockListPlan, bounds: Rect, labels: BlockListChromeLabels) -> Vec<BlockListAccessibilityControl> {
    plan.targets
        .iter()
        .enumerate()
        .filter_map(|(index, target)| {
            let clip = if plan.body_range.contains(&index) { plan.body } else { bounds };
            let visible = target.rect.x < clip.x + clip.w && target.rect.x + target.rect.w > clip.x && target.rect.y < clip.y + clip.h && target.rect.y + target.rect.h > clip.y;
            let action = target.action.clone().filter(|_| visible)?;
            let label = match &target.paint {
                BlockListPaint::AddStep => labels.add_step.to_string(),
                BlockListPaint::IconButton { target_label, .. } => format!("{}: {target_label}", labels.delete),
                BlockListPaint::PaletteEntry { label, .. } => label.clone(),
                BlockListPaint::StepCard { title, .. } => title.clone(),
                BlockListPaint::BlockRow { label, .. } => label.clone(),
                BlockListPaint::DragHandle { .. } => return None,
            };
            Some(BlockListAccessibilityControl { key: target.control_id.clone(), label, rect: target.rect, action })
        })
        .collect()
}

/// 🧩️ Renders the strict-list Blockly-like block-list builder (`SurfaceKind::BlockList`): steps
/// stacked vertically (each with its ordered blocks) plus a palette rail for inserting new blocks,
/// mirroring `🧩️BlockListHost/🟦️.tsx`'s layout and action verbs.
pub(crate) fn render_block_list(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, driver_drag: UiDriverDrag, labels: BlockListChromeLabels) {
    let theme = ctx.theme;
    if scene.block_list.is_none() {
        stage_block_list_accessibility_controls(&scene.host_id, Vec::new());
        return render_placeholder("block-list", bounds, ctx);
    }
    let plan = block_list_plan(scene, bounds, theme, driver_drag);
    let pad = theme.padding_standard;
    let row_h = theme.control_height;
    stage_block_list_accessibility_controls(&scene.host_id, block_list_accessibility_controls(&plan, bounds, labels));

    draw_text(ctx, labels.steps, plan.header.x + pad, plan.header.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
    for target in &plan.targets[..plan.body_range.start] {
        paint_block_list_target(ctx, target, labels);
    }

    ctx.input.register_hit(HitTarget { rect: plan.body, event: None, control_id: Some(scroll_key(&scene.host_id, "blockList")), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    ctx.draw.push_scissor(plan.body);
    reserve_list_rows(ctx, list_visible_row_capacity(plan.body, row_h));
    for target in &plan.targets[plan.body_range.clone()] {
        if target.rect.y + target.rect.h < plan.body.y || target.rect.y > plan.body.y + plan.body.h {
            continue;
        }
        paint_block_list_target(ctx, target, labels);
    }
    ctx.draw.pop_scissor();

    ctx.draw.push_line(plan.palette_rect.x, plan.palette_rect.y, plan.palette_rect.x, plan.palette_rect.y + plan.palette_rect.h, theme.separator, theme.stroke_hairline);
    for target in &plan.targets[plan.body_range.end..] {
        paint_block_list_target(ctx, target, labels);
    }
}

/// 🎯️ Resolves a pointer point inside a `SurfaceKind::BlockList` against the SAME plan the paint drew
/// from — scanned back to front so a card's reorder/remove buttons answer before the card itself.
pub(crate) fn block_list_hit(scene: &UiComponentSceneNode, bounds: Rect, x: f32, y: f32, theme: &Theme, driver_drag: UiDriverDrag) -> Option<SceneListHit> {
    scene.block_list.as_ref()?;
    let plan = block_list_plan(scene, bounds, theme, driver_drag);
    let inside_body = plan.body.contains(x, y);
    for (index, target) in plan.targets.iter().enumerate().rev() {
        if plan.body_range.contains(&index) && !inside_body {
            continue;
        }
        if target.rect.contains(x, y) {
            return Some(SceneListHit::row(target.control_id.clone(), target.action.clone()));
        }
    }
    None
}

fn block_list_accessibility_action_is_current(scene: &UiComponentSceneNode, action: &ActionDescriptor) -> bool {
    let Some(list) = scene.block_list.as_ref() else { return false };
    if action.controller_id != scene.controller_id {
        return false;
    }
    let steps = &list.steps;
    let palette = &list.palette;
    let arg = |key: &str| action.args.as_ref().and_then(|args| args.get(key)).and_then(semio_framework::DslValue::as_str);
    match action.action.as_str() {
        "interactionSelect" => steps.iter().any(|step| {
            block_list_selection_action(scene, step.target.as_ref()).as_ref() == Some(action)
                || step.blocks.iter().any(|block| block_list_selection_action(scene, block.target.as_ref()).as_ref() == Some(action))
        }),
        "addStep" => true,
        "removeStep" => arg("stepId").is_some_and(|step_id| steps.iter().any(|step| step.id == step_id)),
        "removeBlock" => match (arg("stepId"), arg("blockId")) {
            (Some(step_id), Some(block_id)) => steps.iter().any(|step| step.id == step_id && step.blocks.iter().any(|block| block.id == block_id)),
            _ => false,
        },
        "addBlock" => match (arg("stepId"), arg("kind")) {
            (Some(step_id), Some(kind)) => {
                block_list_palette_target_step_id(&steps, list.selected_id.as_deref()) == Some(step_id) && palette.iter().any(|entry| entry.block_kind == kind)
            }
            _ => false,
        },
        _ => false,
    }
}

pub(crate) fn block_list_accessibility_activate(scene: &UiComponentSceneNode, key: &str, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Option<Result<(), ui_wgpu::wgpu::BoundedActionFault>> {
    if scene.component_kind != SurfaceKind::BlockList {
        return None;
    }
    let control = accepted_block_list_accessibility_controls(&scene.host_id).into_iter().find(|control| control.key == key)?;
    block_list_accessibility_action_is_current(scene, &control.action).then(|| write_scene_action(input, &control.action))
}

//#endregion BlockList

//#region BlockListTests
#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-block-list/🦀️.rs"]
mod block_list_tests;
//#endregion BlockListTests

//#region DiffView
#[derive(Clone, Copy, PartialEq, Debug)]
enum DiffLineOperation {
    Equal,
    Removed,
    Added,
}

#[derive(Clone, Copy, Debug)]
struct DiffLine<'a> {
    operation: DiffLineOperation,
    pub(crate) text: &'a str,
    /// 🔢️ 1-based line number in `before`, present on `Equal` and `Removed` lines — React's
    /// `beforeNo` (`🔺️DiffViewHost/🟦️.tsx:20`, assigned at `:40`/`:44`/`:52`).
    before_no: Option<u32>,
    /// 🔢️ 1-based line number in `after`, present on `Equal` and `Added` lines — React's `afterNo`.
    after_no: Option<u32>,
}

/// 🧮️ Above this many `before.len() * after.len()` DP cells, [`diff_lines`] skips the LCS table and
/// falls back to a positional compare so a single huge [`SurfaceKind::DiffView`] payload can't blow
/// up per-frame recompute cost (this crate re-derives the diff every render pass, mirroring how
/// `render_graph_timeline` re-parses `columns_json` every frame rather than caching it).
const DIFF_LCS_CELL_BUDGET: usize = 200_000;

/// 🔢️ A 0-based source index as the 1-based gutter number React prints (`i + 1`).
fn diff_line_no(index: usize) -> Option<u32> {
    u32::try_from(index + 1).ok()
}

/// 📏️ One gutter column's width. React gives each number span `w-10` — Tailwind's 2.5rem, 40 logical
/// px at the default root font size (`🔺️DiffViewHost/🟦️.tsx:106-107`, `:123`).
const DIFF_GUTTER_COLUMN_W: f32 = 40.0;
const DIFF_MARKER_COLUMN_REM: f32 = 0.75;

#[derive(Clone, Copy, Debug)]
struct DiffViewMetrics {
    pub(crate) inner: Rect,
    pub(crate) row_height: f32,
    unified_before_right_x: f32,
    unified_after_right_x: f32,
    unified_marker_x: f32,
    unified_text_x: f32,
    unified_text_width: f32,
    split_pane_width: f32,
    split_divider_x: f32,
    split_divider_width: f32,
    split_right_x: f32,
    split_text_width: f32,
}

fn diff_view_metrics(bounds: Rect, theme: &Theme) -> DiffViewMetrics {
    let (pad, gap) = (theme.padding_standard, theme.gap_standard);
    let inner = Rect::new(bounds.x + pad, bounds.y + pad, (bounds.w - pad * 2.0).max(0.0), (bounds.h - pad * 2.0).max(0.0));
    let row_content_x = inner.x + pad;
    let unified_before_right_x = row_content_x + DIFF_GUTTER_COLUMN_W;
    let unified_after_right_x = unified_before_right_x + gap + DIFF_GUTTER_COLUMN_W;
    let unified_marker_x = unified_after_right_x + gap;
    let unified_text_x = unified_marker_x + theme.root_rem_pixels * DIFF_MARKER_COLUMN_REM + gap;
    let split_divider_width = theme.stroke_hairline.max(1.0);
    let split_pane_width = ((inner.w - gap * 2.0 - split_divider_width) * 0.5).max(1.0);
    let split_divider_x = inner.x + split_pane_width + gap;
    let split_right_x = split_divider_x + split_divider_width + gap;
    DiffViewMetrics {
        inner,
        row_height: ui_wgpu::wgpu::text::line_height(theme.font_size_small),
        unified_before_right_x,
        unified_after_right_x,
        unified_marker_x,
        unified_text_x,
        unified_text_width: (inner.x + inner.w - pad - unified_text_x).max(1.0),
        split_pane_width,
        split_divider_x,
        split_divider_width,
        split_right_x,
        split_text_width: (split_pane_width - pad * 2.0 - DIFF_GUTTER_COLUMN_W - gap).max(1.0),
    }
}

/// 🔢️ Draws one right-aligned, muted gutter number inside the column whose RIGHT edge is `right_x`,
/// exactly React's `text-right tabular-nums text-muted-foreground` span. A `None` number prints
/// nothing at all (React renders `""`), which is what a removed line's `afterNo` is.
fn draw_diff_gutter_number(ctx: &mut FrameworkWidgetContext<'_>, number: Option<u32>, right_x: f32, baseline_y: f32, size: f32, color: Rgba) {
    let Some(number) = number else {
        return;
    };
    let text = number.to_string();
    let width = ctx.atlas.measure_text_face(TextFace::Mono, &text, size).0;
    draw_text_face(ctx, TextFace::Mono, &text, right_x - width, baseline_y, size, color);
}

fn diff_lines<'a>(before: &[&'a str], after: &[&'a str]) -> Vec<DiffLine<'a>> {
    let (n, m) = (before.len(), after.len());
    if n.saturating_mul(m) > DIFF_LCS_CELL_BUDGET {
        let mut out = Vec::with_capacity(n + m);
        for i in 0..n.max(m) {
            let (before_no, after_no) = (u32::try_from(i + 1).ok(), u32::try_from(i + 1).ok());
            match (before.get(i).copied(), after.get(i).copied()) {
                (Some(b), Some(a)) if b == a => out.push(DiffLine { operation: DiffLineOperation::Equal, text: b, before_no, after_no }),
                (Some(b), Some(a)) => {
                    out.push(DiffLine { operation: DiffLineOperation::Removed, text: b, before_no, after_no: None });
                    out.push(DiffLine { operation: DiffLineOperation::Added, text: a, before_no: None, after_no });
                }
                (Some(b), None) => out.push(DiffLine { operation: DiffLineOperation::Removed, text: b, before_no, after_no: None }),
                (None, Some(a)) => out.push(DiffLine { operation: DiffLineOperation::Added, text: a, before_no: None, after_no }),
                (None, None) => {}
            }
        }
        return out;
    }
    let mut table = vec![vec![0u32; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            table[i][j] = if before[i] == after[j] { table[i + 1][j + 1] + 1 } else { table[i + 1][j].max(table[i][j + 1]) };
        }
    }
    let mut out = Vec::with_capacity(n + m);
    let (mut i, mut j) = (0, 0);
    while i < n && j < m {
        if before[i] == after[j] {
            out.push(DiffLine { operation: DiffLineOperation::Equal, text: before[i], before_no: diff_line_no(i), after_no: diff_line_no(j) });
            i += 1;
            j += 1;
        } else if table[i + 1][j] >= table[i][j + 1] {
            out.push(DiffLine { operation: DiffLineOperation::Removed, text: before[i], before_no: diff_line_no(i), after_no: None });
            i += 1;
        } else {
            out.push(DiffLine { operation: DiffLineOperation::Added, text: after[j], before_no: None, after_no: diff_line_no(j) });
            j += 1;
        }
    }
    while i < n {
        out.push(DiffLine { operation: DiffLineOperation::Removed, text: before[i], before_no: diff_line_no(i), after_no: None });
        i += 1;
    }
    while j < m {
        out.push(DiffLine { operation: DiffLineOperation::Added, text: after[j], before_no: None, after_no: diff_line_no(j) });
        j += 1;
    }
    out
}
/// 🪞️ Pairs consecutive remove/add runs into aligned rows for the split-pane layout; equal lines
/// mirror onto both sides. Port of `buildSplitRows` in `🔺️DiffViewHost/🟦️.tsx` — without it a
/// changed line occupied two rows (a left-only one, then a right-only one) instead of one aligned pair.
fn split_diff_rows<'a>(operations: &[DiffLine<'a>]) -> Vec<(Option<DiffLine<'a>>, Option<DiffLine<'a>>)> {
    let mut rows = Vec::with_capacity(operations.len());
    let mut index = 0;
    while index < operations.len() {
        let line = operations[index];
        if line.operation == DiffLineOperation::Equal {
            rows.push((Some(line), Some(line)));
            index += 1;
            continue;
        }
        let mut removed = Vec::new();
        while index < operations.len() && operations[index].operation == DiffLineOperation::Removed {
            removed.push(operations[index]);
            index += 1;
        }
        let mut added = Vec::new();
        while index < operations.len() && operations[index].operation == DiffLineOperation::Added {
            added.push(operations[index]);
            index += 1;
        }
        for pair in 0..removed.len().max(added.len()) {
            rows.push((removed.get(pair).copied(), added.get(pair).copied()));
        }
    }
    rows
}

/// 🩹️ Renders `SurfaceKind::DiffView` with React's inset, Share Tech Mono face, 16 px line
/// boxes, semantic added/removed ink, fixed number gutters, and preserved-space wrapping.
pub(crate) fn render_diff_view(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>) {
    let theme = ctx.theme;
    let Some(diff) = &scene.diff_view else {
        return render_placeholder("diff-view", bounds, ctx);
    };
    let before_lines: Vec<&str> = diff.before.split('\n').collect();
    let after_lines: Vec<&str> = diff.after.split('\n').collect();
    let operations = diff_lines(&before_lines, &after_lines);
    let pad = theme.padding_standard;
    let gap = theme.gap_standard;
    let size = theme.font_size_small;
    let metrics = diff_view_metrics(bounds, theme);
    let (inner, row_h) = (metrics.inner, metrics.row_height);
    let split = diff.mode.as_deref() == Some("split");

    let scroll = scroll_offset(&scene.host_id, "diff");
    ctx.input.register_hit(HitTarget { rect: bounds, event: None, control_id: Some(scroll_key(&scene.host_id, "diff")), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    ctx.draw.push_scissor(bounds);
    reserve_list_rows(ctx, list_visible_row_capacity(inner, row_h));
    if operations.is_empty() {
        draw_text_face(ctx, TextFace::Mono, "—", inner.x + pad, inner.y + size, size, theme.text_muted);
        ctx.draw.pop_scissor();
        return;
    }

    if split {
        let mut plans = Vec::new();
        let mut content_h = 0.0f32;
        for (left, right) in split_diff_rows(&operations) {
            let left_lines = left.map_or_else(|| vec![0..0], |line| ctx.atlas.pre_wrap_lines(TextFace::Mono, line.text, metrics.split_text_width, size));
            let right_lines = right.map_or_else(|| vec![0..0], |line| ctx.atlas.pre_wrap_lines(TextFace::Mono, line.text, metrics.split_text_width, size));
            let height = left_lines.len().max(right_lines.len()).max(1) as f32 * row_h;
            plans.push((content_h, height, left, left_lines, right, right_lines));
            content_h += height;
        }
        ctx.draw.push_line(metrics.split_divider_x + metrics.split_divider_width * 0.5, inner.y - scroll, metrics.split_divider_x + metrics.split_divider_width * 0.5, inner.y + content_h - scroll, theme.separator, metrics.split_divider_width);
        for (offset, height, left, left_lines, right, right_lines) in plans {
            let y = inner.y + offset - scroll;
            if y + height < inner.y || y > inner.y + inner.h {
                continue;
            }
            let baseline = y + size;
            draw_diff_gutter_number(ctx, left.and_then(|line| line.before_no), inner.x + pad + DIFF_GUTTER_COLUMN_W, baseline, size, theme.text_muted);
            draw_diff_gutter_number(ctx, right.and_then(|line| line.after_no), metrics.split_right_x + pad + DIFF_GUTTER_COLUMN_W, baseline, size, theme.text_muted);
            if let Some(line) = left {
                let color = if line.operation == DiffLineOperation::Removed { theme.error } else { theme.text };
                for (index, range) in left_lines.into_iter().enumerate() {
                    draw_text_face(ctx, TextFace::Mono, &line.text[range], inner.x + pad + DIFF_GUTTER_COLUMN_W + gap, baseline + index as f32 * row_h, size, color);
                }
            }
            if let Some(line) = right {
                let color = if line.operation == DiffLineOperation::Added { theme.diff_added } else { theme.text };
                for (index, range) in right_lines.into_iter().enumerate() {
                    draw_text_face(ctx, TextFace::Mono, &line.text[range], metrics.split_right_x + pad + DIFF_GUTTER_COLUMN_W + gap, baseline + index as f32 * row_h, size, color);
                }
            }
        }
        ctx.draw.pop_scissor();
        return;
    }

    let mut plans = Vec::new();
    let mut content_h = 0.0f32;
    for line in operations {
        let lines = ctx.atlas.pre_wrap_lines(TextFace::Mono, line.text, metrics.unified_text_width, size);
        let height = lines.len().max(1) as f32 * row_h;
        plans.push((content_h, height, line, lines));
        content_h += height;
    }
    for (offset, height, line, lines) in plans {
        let y = inner.y + offset - scroll;
        if y + height < inner.y || y > inner.y + inner.h {
            continue;
        }
        let baseline = y + size;
        draw_diff_gutter_number(ctx, line.before_no, metrics.unified_before_right_x, baseline, size, theme.text_muted);
        draw_diff_gutter_number(ctx, line.after_no, metrics.unified_after_right_x, baseline, size, theme.text_muted);
        let (marker, color) = match line.operation {
            DiffLineOperation::Added => ('+', theme.diff_added),
            DiffLineOperation::Removed => ('-', theme.error),
            DiffLineOperation::Equal => (' ', theme.text),
        };
        draw_text_face(ctx, TextFace::Mono, &marker.to_string(), metrics.unified_marker_x, baseline, size, color);
        for (index, range) in lines.into_iter().enumerate() {
            draw_text_face(ctx, TextFace::Mono, &line.text[range], metrics.unified_text_x, baseline + index as f32 * row_h, size, color);
        }
    }
    ctx.draw.pop_scissor();
}

//#endregion DiffView

//#region DiffViewTests
#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-diff-view/🦀️.rs"]
mod diff_view_tests;
//#endregion DiffViewTests
