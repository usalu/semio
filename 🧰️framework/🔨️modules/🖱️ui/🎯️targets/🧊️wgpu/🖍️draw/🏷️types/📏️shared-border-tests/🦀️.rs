//! 🧪 Semantic border recording preserves paint positions and scope.

use super::*;
use semio_ui_theme::geometry::Rect;

#[test]
fn records_edges_in_the_active_lane() {
    let mut draw = DrawList::default();
    draw.push_solid([0.0, 0.0, 1.0, 1.0], Rgba::TRANSPARENT);
    draw.push_shared_border_edges(Rect::new(0.0, 0.0, 10.0, 20.0), 1.0, Rgba::TRANSPARENT, SharedBorderState::Hover, [true, false, true, false]);
    assert_eq!(draw.shared_borders.len(), 2);
    assert_eq!(draw.shared_borders[0].slot, 1);
    assert_eq!(draw.shared_borders[1].slot, 2);
    assert_eq!(draw.shared_borders[0].segment.state, SharedBorderState::Hover);
}

#[test]
fn shared_edge_uses_hover_color_once_inside_the_winner() {
    let mut draw = DrawList::default();
    let normal = Rgba { r: 0.2, g: 0.3, b: 0.4, a: 1.0 };
    let hover = Rgba { r: 0.9, g: 0.8, b: 0.7, a: 1.0 };
    draw.push_shared_border(Rect::new(0.0, 0.0, 10.0, 10.0), 1.0, hover, SharedBorderState::Hover);
    draw.push_solid([10.0, 0.0, 10.0, 10.0], normal);
    draw.push_shared_border(Rect::new(10.0, 0.0, 10.0, 10.0), 1.0, normal, SharedBorderState::Normal);
    while !draw.finalize_shared_borders_step().unwrap() {}
    let lane: Vec<_> = draw.layers.iter().flat_map(|layer| &layer.ui_instances).filter(|item| item.rect[2] > 0.0 && item.rect[3] > 0.0).collect();
    let shared: Vec<_> = lane.iter().enumerate().filter(|(_, item)| item.rect == [9.0, 0.0, 1.0, 10.0]).collect();
    assert_eq!(shared.len(), 1);
    assert_eq!(shared[0].1.color, [hover.r, hover.g, hover.b, hover.a]);
}

#[test]
fn partial_shared_edge_splits_and_overlay_remains_separate() {
    let mut draw = DrawList::default();
    let normal = Rgba { r: 0.2, g: 0.3, b: 0.4, a: 1.0 };
    let hover = Rgba { r: 0.9, g: 0.8, b: 0.7, a: 1.0 };
    draw.push_shared_border_edges(Rect::new(0.0, 0.0, 10.0, 20.0), 1.0, normal, SharedBorderState::Normal, [false, true, false, false]);
    draw.push_shared_border_edges(Rect::new(10.0, 5.0, 10.0, 10.0), 1.0, hover, SharedBorderState::Hover, [false, false, false, true]);
    draw.begin_overlay_route();
    draw.push_shared_border_edges(Rect::new(10.0, 5.0, 10.0, 10.0), 1.0, normal, SharedBorderState::Normal, [false, false, false, true]);
    draw.end_overlay_route();
    while !draw.finalize_shared_borders_step().unwrap() {}
    let lane: Vec<_> = draw.layers.iter().flat_map(|layer| &layer.ui_instances).filter(|item| item.rect[2] > 0.0 && item.rect[3] > 0.0).collect();
    assert_eq!(lane.len(), 3);
    assert!(lane.iter().any(|item| item.rect == [9.0, 0.0, 1.0, 5.0] && item.color == [normal.r, normal.g, normal.b, normal.a]));
    assert!(lane.iter().any(|item| item.rect == [10.0, 5.0, 1.0, 10.0] && item.color == [hover.r, hover.g, hover.b, hover.a]));
    assert!(lane.iter().any(|item| item.rect == [9.0, 15.0, 1.0, 5.0] && item.color == [normal.r, normal.g, normal.b, normal.a]));
    assert_eq!(draw.layers.iter().flat_map(|layer| &layer.overlay_ui_instances).filter(|item| item.rect[2] > 0.0 && item.rect[3] > 0.0).count(), 1);
}

#[test]
fn winning_effect_retains_surface_phase_and_segment_clip() {
    let mut draw = DrawList::default();
    let color = Rgba { r: 0.2, g: 0.3, b: 0.4, a: 1.0 };
    draw.push_shared_border_effect(Rect::new(0.0, 0.0, 10.0, 10.0), color, 0.0, 1.0, SharedBorderState::Loading);
    while !draw.finalize_shared_borders_step().unwrap() {}
    let lane: Vec<_> = draw.layers.iter().flat_map(|layer| &layer.ui_instances).filter(|item| item.rect[2] > 0.0 && item.rect[3] > 0.0).collect();
    assert_eq!(lane.len(), 4);
    assert!(lane.iter().all(|item| item.rect == [0.0, 0.0, 10.0, 10.0] && item.params[2] == KIND_LOADING_BORDER && item.params[3] == 1.0));
    assert!(lane.iter().any(|item| item.uv_rect == [0.0, 0.0, 10.0, 1.0]));
}

#[test]
fn cancellation_retires_resolution_backing_at_every_stage() {
    let mut draw = DrawList::default();
    draw.push_shared_border(Rect::new(0.0, 0.0, 10.0, 10.0), 1.0, Rgba::TRANSPARENT, SharedBorderState::Normal);
    draw.push_shared_border(Rect::new(10.0, 0.0, 10.0, 10.0), 1.0, Rgba::TRANSPARENT, SharedBorderState::Hover);
    loop {
        let mut cancelled = draw.clone();
        while !cancelled.retire_step() {}
        assert!(cancelled.retirement_is_empty());
        if draw.finalize_shared_borders_step().unwrap() { break; }
    }
}

#[test]
fn adjacent_layers_share_ownership_in_the_same_scope() {
    let mut draw = DrawList::default();
    let normal = Rgba { r: 0.1, g: 0.2, b: 0.3, a: 1.0 };
    let hover = Rgba { r: 0.8, g: 0.7, b: 0.6, a: 1.0 };
    draw.push_shared_border_edges(Rect::new(0.0, 0.0, 10.0, 10.0), 1.0, hover, SharedBorderState::Hover, [false, true, false, false]);
    draw.layers.push(DrawLayer::default());
    draw.push_shared_border_edges(Rect::new(10.0, 0.0, 10.0, 10.0), 1.0, normal, SharedBorderState::Normal, [false, false, false, true]);
    while !draw.finalize_shared_borders_step().unwrap() {}
    let lane: Vec<_> = draw.layers.iter().flat_map(|layer| &layer.ui_instances).filter(|item| item.rect[2] > 0.0 && item.rect[3] > 0.0).collect();
    assert_eq!(lane.len(), 1);
    assert_eq!(lane[0].color, [hover.r, hover.g, hover.b, hover.a]);
}

#[test]
fn adjacent_scissors_share_the_visible_inset_edge() {
    let mut draw = DrawList::default();
    let color = Rgba { r: 0.2, g: 0.4, b: 0.6, a: 1.0 };
    let normal = Rgba { r: 0.7, g: 0.1, b: 0.2, a: 1.0 };
    draw.layers[0].scissor = Some(ScissorRect { x: 0, y: 0, w: 10, h: 10 });
    draw.push_shared_border_edges(Rect::new(0.0, 0.0, 10.0, 10.0), 1.0, color, SharedBorderState::Hover, [false, true, false, false]);
    draw.layers.push(DrawLayer { scissor: Some(ScissorRect { x: 10, y: 0, w: 10, h: 10 }), ..DrawLayer::default() });
    draw.push_shared_border_edges(Rect::new(10.0, 0.0, 10.0, 10.0), 1.0, normal, SharedBorderState::Normal, [false, false, false, true]);
    while !draw.finalize_shared_borders_step().unwrap() {}
    let painted: Vec<_> = draw.layers.iter().filter(|layer| layer.ui_instances.iter().any(|item| item.rect[2] > 0.0 && item.rect[3] > 0.0)).collect();
    assert_eq!(painted.len(), 1);
    let visible: Vec<_> = painted[0].ui_instances.iter().filter(|item| item.rect[2] > 0.0 && item.rect[3] > 0.0).collect();
    assert_eq!(visible.len(), 1);
    assert_eq!(visible[0].color, [color.r, color.g, color.b, color.a]);
    assert_eq!(painted[0].scissor, Some(ScissorRect { x: 0, y: 0, w: 10, h: 10 }));
}

#[test]
fn scene_and_glass_references_survive_late_border_layers() {
    let mut draw = DrawList::default();
    let hover = Rgba { r: 0.8, g: 0.7, b: 0.6, a: 1.0 };
    draw.push_shared_border_edges(Rect::new(0.0, 0.0, 10.0, 10.0), 1.0, hover, SharedBorderState::Hover, [false, true, false, false]);
    draw.layers.push(DrawLayer::default());
    draw.push_shared_border_edges(Rect::new(10.0, 0.0, 10.0, 10.0), 1.0, Rgba::TRANSPARENT, SharedBorderState::Normal, [false, false, false, true]);
    draw.push_scene_pass(semio_ui_scene_pass::ScenePass3d::default());
    let original_scene = draw.scene_passes[0].layer_index;
    let original_ui_watermark = draw.scene_passes[0].ui_watermark;
    let original_vector_watermark = draw.scene_passes[0].vector_watermark;
    let glass_layer = draw.layers.len() - 1;
    draw.glass_regions.push(GlassRegion { layer_index: glass_layer, rect: [0.0, 0.0, 20.0, 10.0], radius: 0.0, tint: Rgba::TRANSPARENT, alpha: 0.0, blur_px: 0.0, saturate: 1.0 });
    draw.layers[glass_layer].foreground_of = Some(0);
    while !draw.finalize_shared_borders_step().unwrap() {}
    let scene = &draw.scene_passes[0];
    assert!(scene.layer_index >= original_scene && scene.layer_index < draw.layers.len());
    assert_eq!(scene.ui_watermark, original_ui_watermark);
    assert_eq!(scene.vector_watermark, original_vector_watermark);
    assert!(scene.ui_watermark <= draw.layers[scene.layer_index].ui_instances.len());
    assert!(scene.vector_watermark <= draw.layers[scene.layer_index].vector_vertices.len());
    let glass = draw.glass_regions[0];
    assert!(glass.layer_index < draw.layers.len());
    assert_eq!(draw.layers[glass.layer_index].foreground_of, Some(0));
    let painted: Vec<_> = draw.layers.iter().flat_map(|layer| &layer.ui_instances).filter(|item| item.color == [hover.r, hover.g, hover.b, hover.a]).collect();
    assert_eq!(painted.len(), 1);
    let painted_layer = draw.layers.iter().position(|layer| layer.ui_instances.iter().any(|item| item.color == [hover.r, hover.g, hover.b, hover.a] && item.rect[2] > 0.0 && item.rect[3] > 0.0)).unwrap();
    assert!(painted_layer > scene.layer_index);
}

#[test]
fn shared_border_capacity_tracks_incremental_resolution_and_retirement() {
    let mut draw = DrawList::default();
    draw.push_shared_border(Rect::new(0.0, 0.0, 10.0, 10.0), 1.0, Rgba::TRANSPARENT, SharedBorderState::Normal);
    draw.push_shared_border(Rect::new(10.0, 0.0, 10.0, 10.0), 1.0, Rgba::TRANSPARENT, SharedBorderState::Hover);
    assert_eq!(draw.shared_border_retained_capacity_bytes(), 0);
    let mut seen = Vec::new();
    let mut complete = false;
    for _ in 0..100_000 {
        complete = draw.finalize_shared_borders_step().unwrap();
        seen.push(draw.shared_border_retained_capacity_bytes());
        if complete { break; }
    }
    assert!(complete);
    assert!(seen.iter().any(|bytes| *bytes > 0));
    assert!(seen.windows(2).any(|pair| pair[0] != pair[1]));
    let retained = *seen.last().unwrap();
    assert_eq!(draw.shared_border_retained_capacity_bytes(), retained);
    let snapshot = |draw: &DrawList| draw.layers.iter().map(|layer| (layer.scissor, layer.foreground_of, layer.ui_instances.iter().map(|item| (item.rect, item.color, item.params, item.uv_rect)).collect::<Vec<_>>(), layer.overlay_ui_instances.iter().map(|item| (item.rect, item.color, item.params, item.uv_rect)).collect::<Vec<_>>())).collect::<Vec<_>>();
    let before = snapshot(&draw);
    assert!(draw.finalize_shared_borders_step().unwrap());
    assert_eq!(snapshot(&draw), before);
    assert_eq!(draw.shared_border_retained_capacity_bytes(), retained);
    for _ in 0..100_000 { if draw.retire_step() { break; } }
    assert!(draw.retirement_is_empty());
    assert_eq!(draw.shared_border_retained_capacity_bytes(), 0);
}

#[test]
fn scene_pass_separates_coincident_border_ownership() {
    let mut draw = DrawList::default();
    let first = Rgba { r: 0.8, g: 0.1, b: 0.2, a: 1.0 };
    let second = Rgba { r: 0.1, g: 0.2, b: 0.8, a: 1.0 };
    draw.push_shared_border_edges(Rect::new(0.0, 0.0, 10.0, 10.0), 1.0, first, SharedBorderState::Hover, [false, true, false, false]);
    draw.push_scene_pass(semio_ui_scene_pass::ScenePass3d::default());
    draw.push_shared_border_edges(Rect::new(10.0, 0.0, 10.0, 10.0), 1.0, second, SharedBorderState::Normal, [false, false, false, true]);
    while !draw.finalize_shared_borders_step().unwrap() {}
    let scene = draw.scene_passes[0].layer_index;
    let find = |color: Rgba| draw.layers.iter().position(|layer| layer.ui_instances.iter().any(|item| item.color == [color.r, color.g, color.b, color.a] && item.rect[2] > 0.0 && item.rect[3] > 0.0)).unwrap();
    let first_layer = find(first);
    let second_layer = find(second);
    assert!(first_layer > scene);
    assert!(second_layer > first_layer);
}

#[test]
fn retirement_waits_for_quoted_depth() {
    use semio_framework_job::{InteractiveJobCloseStep, RetainedCloneGrant};
    let mut draw = DrawList::default();
    draw.push_shared_border(Rect::new(0.0, 0.0, 10.0, 10.0), 1.0, Rgba::TRANSPARENT, SharedBorderState::Normal);
    let depth = draw.next_retirement_depth_demand().unwrap();
    assert!(depth > 0);
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: depth - 1 };
    assert!(matches!(draw.retire_granted(grant).unwrap(), InteractiveJobCloseStep::Pending { progress } if progress.copied_items == 0));
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: depth };
    assert!(matches!(draw.retire_granted(grant).unwrap(), InteractiveJobCloseStep::Pending { progress } if progress.copied_items == 1));
}

#[test]
fn active_finalizer_clones_report_and_release_scratch() {
    let mut draw = DrawList::default();
    draw.push_shared_border(Rect::new(0.0, 0.0, 10.0, 10.0), 1.0, Rgba::TRANSPARENT, SharedBorderState::Normal);
    draw.push_shared_border(Rect::new(10.0, 0.0, 10.0, 10.0), 1.0, Rgba::TRANSPARENT, SharedBorderState::Hover);
    let mut saw_scratch = false;
    let mut complete = false;
    for _ in 0..100_000 {
        let mut cloned = draw.clone();
        saw_scratch |= cloned.shared_border_retained_capacity_bytes() > 0;
        let mut retired = false;
        for _ in 0..100_000 {
            let before = cloned.shared_border_retained_capacity_bytes();
            let demand = cloned.next_retirement_release_byte_demand().unwrap();
            retired = cloned.retire_step();
            let after = cloned.shared_border_retained_capacity_bytes();
            if before > after { assert!(before - after <= demand); }
            if retired { break; }
        }
        assert!(retired && cloned.retirement_is_empty());
        assert_eq!(cloned.shared_border_retained_capacity_bytes(), 0);
        complete = draw.finalize_shared_borders_step().unwrap();
        if complete { break; }
    }
    assert!(complete && saw_scratch);
}

#[test]
fn finalizer_receipts_fit_preflight_for_clips_growth_and_insertion() {
    let mut draw = DrawList::default();
    draw.layers[0].scissor = Some(ScissorRect { x: 0, y: 0, w: 20, h: 20 });
    draw.layers[0].clip = Some(ClipRegion { scissors: vec![ScissorRect { x: 0, y: 0, w: 10, h: 10 }, ScissorRect { x: 0, y: 10, w: 10, h: 10 }].into() });
    draw.push_shared_border(Rect::new(0.0, 0.0, 10.0, 20.0), 1.0, Rgba::TRANSPARENT, SharedBorderState::Active);
    draw.layers.push(DrawLayer::default());
    draw.push_shared_border(Rect::new(10.0, 5.0, 10.0, 10.0), 1.0, Rgba::TRANSPARENT, SharedBorderState::Hover);
    let mut complete = false;
    let mut copied = false;
    let mut allocated = false;
    let mut released = false;
    for step in 0..100_000 {
        let (items, capacity) = draw.next_shared_border_finalization_allocation_demand().unwrap();
        let copy = draw.next_shared_border_finalization_copy_byte_demand().unwrap();
        let release = draw.next_shared_border_finalization_release_byte_demand().unwrap();
        let (result, progress) = draw.finalize_shared_borders_step_with_progress();
        assert!(progress.copied_items <= items, "step {step}: item preflight");
        assert!(progress.copied_bytes <= copy, "step {step}: copy preflight: {} > {copy}", progress.copied_bytes);
        assert!(progress.retained_capacity_bytes <= capacity, "step {step}: capacity preflight: {} > {capacity}", progress.retained_capacity_bytes);
        assert!(progress.released_bytes <= release, "step {step}: release preflight: {} > {release}", progress.released_bytes);
        copied |= progress.copied_bytes > 0;
        allocated |= progress.retained_capacity_bytes > 0;
        released |= progress.released_bytes > 0;
        complete = result.unwrap();
        if complete { break; }
    }
    assert!(complete && copied && allocated && released);
}

#[test]
fn retained_record_backing_is_preadmitted_across_new_layers() {
    let mut draw = DrawList::default();
    draw.begin_retained_output(16, 64 * 1024).unwrap();
    let admitted = draw.shared_borders.allocated_bytes();
    draw.push_shared_border_edges(Rect::new(0.0, 0.0, 10.0, 10.0), 1.0, Rgba::TRANSPARENT, SharedBorderState::Normal, [false, true, false, false]);
    draw.push_scissor(Rect::new(0.0, 0.0, 20.0, 20.0));
    draw.push_shared_border_edges(Rect::new(10.0, 0.0, 10.0, 10.0), 1.0, Rgba::TRANSPARENT, SharedBorderState::Hover, [false, false, false, true]);
    draw.pop_scissor();
    assert_eq!(draw.shared_borders.len(), 2);
    assert_ne!(draw.shared_borders[0].layer, draw.shared_borders[1].layer);
    assert_eq!(draw.shared_borders.allocated_bytes(), admitted);
    draw.finish_retained_output().unwrap();
}
