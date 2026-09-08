
use super::*;
fn tab(id: &str) -> DockStackTab {
    DockStackTab::new(id)
}

fn tabs(ids: &[&str]) -> Vec<DockStackTab> {
    ids.iter().map(|id| DockStackTab::new(*id)).collect()
}

fn stack_tabs(ids: &[&str], active: &str) -> DockNode {
    DockNode::Stack { windows: tabs(ids), active: active.into() }
}

use crate::shell::ShellState;
use semio_framework::{AppDefinition, AppRole, ArtifactDialect, ModeDefinition, PanelGroup, PanelTabDefinition, PanelTabKind, WindowKindDefinition};
use ui_wgpu::wgpu::LocalizedLabel;
use ui_wgpu::wgpu::{WindowOptions, create_default_layout};

fn sample_app(window_ids: &[&str], layout: Option<WindowLayout>) -> AppDefinition {
    AppDefinition {
        id: "test".into(),
        role: AppRole::Editor,
        dialect: ArtifactDialect { artifact_kind: "s.test.dock".into(), standard: "1".into(), subset: "*".into() },
        label: LocalizedLabel::data("Test"),
        breadcrumb: vec!["semio".into(), "test".into()],
        icon_id: None,
        controller_id: "test".into(),
        modes: semio_framework::Modes::one(ModeDefinition { id: "default".into(), label: LocalizedLabel::data("Default"), icon_id: "pencil".into(), tools: vec![], layout_id: None, commands: vec![] }),
        default_mode_id: "default".into(),
        window_kinds: semio_framework::WindowKinds::try_from(
            window_ids
                .iter()
                .map(|id| WindowKindDefinition {
                    id: (*id).into(),
                    label: LocalizedLabel::data(*id),
                    body_key: format!("{id}.body"),
                    surface_kind: ui_wgpu::wgpu::SurfaceKind::Canvas2d,
                    icon_id: "app-window".into(),
                    options: WindowOptions::default(),
                    actions: vec![],
                    interactions: vec![],
                    utilities: vec![],
                    params_schema: None,
                    artifact_snapshot_schema: None,
                    input_event_schema: None,
                    output_schema: None,
                    capabilities: vec![],
                })
                .collect::<Vec<_>>(),
        )
        .expect("sample_app tests always pass at least one window id"),
        panel_tabs: vec![PanelTabDefinition { kind: PanelTabKind::App("tab".into()), label: LocalizedLabel::data("Tab"), group: PanelGroup::Workbench, body_key: Some("tab.body".into()), children: vec![] }],
        keybindings: vec![],
        interactions: vec![],
        utilities: vec![],
        tools: vec![],
        commands: vec![],
        named_layouts: vec![],
        default_layout: layout,
        terminologies: vec![],
        terminology_breadcrumbs: HashMap::new(),
        introduction: None,
        tutorials: Vec::new(),
        dialogs: Vec::new(),
        media_inputs: Vec::new(),
        media_outputs: Vec::new(),
        artifact_kinds: Vec::new(),
        config: semio_framework_async::block_on(semio_framework::ConfigSpec::empty()),
        command_grammar: semio_framework_async::block_on(semio_framework::CommandGrammar::empty()),
        io: semio_framework::AppIo::default(),
    }
}

#[test]
fn split_axis_extent_uses_row_width_not_canvas_max() {
    let mut dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
    dock.root = DockNode::Column(vec![(DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]), 0.5), (stack_with("c"), 0.5)]);
    let canvas = Rect::new(0.0, 0.0, 1000.0, 800.0);
    let row_extent = dock.split_axis_extent(&vec![0], canvas).unwrap();
    assert!((row_extent - 1000.0).abs() < 0.1);
    let col_extent = dock.split_axis_extent(&vec![], canvas);
    assert!((col_extent.unwrap() - 800.0).abs() < 0.1);
    let nested_extent = dock.split_axis_extent(&vec![0], canvas).unwrap();
    assert!((nested_extent - 1000.0).abs() < 0.1);
}

#[test]
fn panel_scroll_region_blocks_scene_wheel() {
    let panel_scroll = HitTarget { rect: Rect::new(0.0, 0.0, 200.0, 400.0), event: None, control_id: Some("panel.left.lowpoly".into()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None };
    assert!(!ShellState::wheel_propagates_to_scene_surface(Some(&panel_scroll)));
    let world = HitTarget { rect: Rect::new(0.0, 0.0, 800.0, 600.0), event: None, control_id: Some("world-surface".into()), kind: HitKind::World3d, drag_axis: None, drag_data: None };
    assert!(ShellState::wheel_propagates_to_scene_surface(Some(&world)));
    let graph_pane = HitTarget { rect: Rect::new(0.0, 0.0, 800.0, 600.0), event: None, control_id: Some("graph-surface.pane".into()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None };
    assert!(ShellState::wheel_propagates_to_scene_surface(Some(&graph_pane)));
}

#[test]
fn row_layout_stack_content_rects_match_per_window() {
    let layout = create_default_layout(&["flow".into(), "preview".into()], "row", Some(&[68.0, 32.0]), Some(&["Flow".into(), "Preview".into()]));
    let app = sample_app(&["flow", "preview"], Some(layout));
    let dock = DockState::from_app(&app, Some("flow"));
    let canvas = Rect::new(0.0, 0.0, 1200.0, 800.0);
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let labels = HashMap::from([("flow".into(), "Flow".into()), ("preview".into(), "Preview".into())]);
    let placements = dock.stack_body_rects(canvas, &theme, &labels, &mut atlas);
    assert_eq!(placements.len(), 2);
    let flow_rect = placements.iter().find(|(_, _, id)| id == "flow").map(|(_, rect, _)| *rect);
    let preview_rect = placements.iter().find(|(_, _, id)| id == "preview").map(|(_, rect, _)| *rect);
    let flow_rect = flow_rect.expect("flow body rect");
    let preview_rect = preview_rect.expect("preview body rect");
    assert!(flow_rect.w > preview_rect.w);
    assert!((flow_rect.x + flow_rect.w - preview_rect.x).abs() < 1.0);
    assert!(flow_rect.h > 0.0 && preview_rect.h > 0.0);
}

#[test]
fn dock_tab_content_width_reserves_icon_slot() {
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let label = "Main";
    let with_icon = dock_tab_content_width(&mut atlas, &theme, label);
    let text_only = atlas.measure_text(label, theme.font_size_small).0 + theme.padding_standard * 2.0;
    assert!(with_icon > text_only);
}

//#region SilhouetteContentTests

#[test]
fn window_silhouette_clip_union_excludes_cap_gap() {
    let silhouette = WindowSilhouette::from_measured_top(Rect::new(10.0, 20.0, 300.0, 200.0), 80.0, 60.0, 32.0);
    assert_eq!(silhouette.content_clip_rects(), vec![Rect::new(10.0, 52.0, 300.0, 168.0), Rect::new(10.0, 20.0, 80.0, 32.0), Rect::new(250.0, 20.0, 60.0, 32.0)]);
    assert_eq!(silhouette.content_bounds(), Rect::new(10.0, 20.0, 300.0, 200.0));
    assert_eq!(silhouette.safe_body_rect(), Rect::new(10.0, 52.0, 300.0, 168.0));
    assert!(!silhouette.content_clip_rects().iter().any(|rect| rect.contains(150.0, 30.0)));
}

#[test]
fn window_silhouette_v1_matches_typescript_fixture_for_merging_bottom_and_containment() {
    let normalized = WindowSilhouette::new(
        Rect::new(0.0, 0.0, 200.0, 100.0),
        WindowSilhouetteEdge::new(24.0, vec![WindowSilhouetteSpan::new(160.0, 220.0), WindowSilhouetteSpan::new(60.25, 90.0), WindowSilhouetteSpan::new(0.0, 60.0), WindowSilhouetteSpan::new(90.25, 120.0)]),
        WindowSilhouetteEdge::new(16.0, vec![WindowSilhouetteSpan::new(80.0, 120.0), WindowSilhouetteSpan::new(0.0, 40.0)]),
    );
    assert_eq!(normalized.top.spans, vec![WindowSilhouetteSpan::new(0.0, 120.0), WindowSilhouetteSpan::new(160.0, 200.0)]);
    let silhouette = WindowSilhouette::new(
        Rect::new(0.0, 0.0, 200.0, 100.0),
        WindowSilhouetteEdge::new(24.0, vec![WindowSilhouetteSpan::new(160.0, 200.0), WindowSilhouetteSpan::new(0.0, 60.0)]),
        WindowSilhouetteEdge::new(16.0, vec![WindowSilhouetteSpan::new(80.0, 120.0), WindowSilhouetteSpan::new(0.0, 40.0)]),
    );
    assert_eq!(silhouette.glass_regions(), vec![Rect::new(0.0, 0.0, 60.0, 24.0), Rect::new(160.0, 0.0, 40.0, 24.0), Rect::new(0.0, 84.0, 40.0, 16.0), Rect::new(80.0, 84.0, 40.0, 16.0)]);
    assert_eq!(silhouette.safe_clearances(), (24.0, 16.0));
    assert!(silhouette.contains(20.0, 12.0));
    assert!(!silhouette.contains(140.0, 12.0));
    assert!(silhouette.contains(100.0, 50.0));
    assert!(!silhouette.contains(60.0, 92.0));
    assert!(silhouette.contains(100.0, 92.0));
}

#[test]
fn dock_stack_glass_and_hits_exist_only_on_owned_chips() {
    let mut dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
    dock.root = stack_tabs(&["a", "b"], "a");
    let bounds = Rect::new(0.0, 0.0, 600.0, 400.0);
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut draw = DrawList::default();
    let labels = HashMap::from([("a".into(), "A".into()), ("b".into(), "B".into())]);
    let icon_ids = HashMap::new();
    let layout = layout_stack_cap(&tabs(&["a", "b"]), &labels, &icon_ids, &mut atlas, &theme, bounds);
    let gap_point = (bounds.x + bounds.w * 0.5, bounds.y + theme.control_height * 0.5);
    let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &icons, input: &mut input, theme: &theme, window_labels: &labels, window_icon_ids: &icon_ids };
    dock.paint_chrome(&mut ctx, bounds, false);
    assert_eq!(draw.glass_regions.len(), 1, "one top-left corner group chip for both tabs");
    assert!(!draw.glass_regions.iter().any(|region| Rect::new(region.rect[0], region.rect[1], region.rect[2], region.rect[3]).contains(gap_point.0, gap_point.1)));
    assert!(input.hit_targets.iter().any(|hit| hit.control_id.as_deref() == Some("dock.tab..a")));
    assert!(input.hit_targets.iter().any(|hit| hit.control_id.as_deref() == Some("dock.tab..a.close")));
    assert!(input.hit_targets.iter().any(|hit| hit.control_id.as_deref() == Some("dock.tab..a.focus")));
    assert!(!input.hit_targets.iter().any(|hit| hit.control_id.as_deref().is_some_and(|id| id.starts_with("dock.focus.") || id.starts_with("dock.close."))));
    assert!(!input.hit_targets.iter().any(|hit| hit.control_id.as_deref().is_some_and(|id| id.starts_with("dock.stack."))));
    let _ = layout;
}

#[test]
fn apply_drop_tab_moves_window_to_target_corner() {
    let mut dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
    dock.root = DockNode::Row(vec![(stack_tabs(&["a", "x"], "a"), 0.5), (stack_tabs(&["b"], "b"), 0.5)]);
    assert!(dock.remove_window("a"));
    let payload = tab_payload("a", vec![0], 0);
    let zone = DockDropZone::Tab { stack_path: vec![1], corner: WindowStackCorner::BottomRight, index: 0 };
    assert!(dock.apply_drop(&payload, &zone));
    let tabs = dock.stack_tabs_at_path(&vec![1]).expect("target stack");
    assert!(tabs.iter().any(|tab| tab.window_id == "a" && tab.corner == WindowStackCorner::BottomRight));
    assert_eq!(dock.active_window_id.as_deref(), Some("a"));
}

#[test]
fn dock_stack_content_fills_full_bounds_through_one_silhouette_clip() {
    let mut dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
    dock.root = stack_tabs(&["a", "b"], "a");
    let bounds = Rect::new(10.0, 20.0, 600.0, 400.0);
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut draw = DrawList::default();
    let labels = HashMap::from([("a".into(), "A".into()), ("b".into(), "B".into())]);
    let icon_ids = HashMap::new();
    let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &icons, input: &mut input, theme: &theme, window_labels: &labels, window_icon_ids: &icon_ids };
    dock.paint_chrome(&mut ctx, bounds, true);
    let fill = draw.layers.iter().find(|layer| layer.ui_instances.iter().any(|instance| instance.rect == [bounds.x, bounds.y, bounds.w, bounds.h])).expect("full silhouette content fill");
    assert_eq!(fill.clip.as_ref().map(|clip| clip.scissors.len()), Some(3));
    assert!(!fill.clip.as_ref().is_some_and(|clip| clip.scissors.iter().any(|rect| rect.x <= 300 && 300 < rect.x + rect.w && rect.y <= 30 && 30 < rect.y + rect.h)));
}

//#endregion SilhouetteContentTests

#[test]
fn resize_hits_win_over_later_scroll_region() {
    let mut dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
    dock.root = even_layout(&["a".into(), "b".into()]);
    let canvas = Rect::new(0.0, 0.0, 400.0, 300.0);
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut draw = DrawList::default();
    let labels = HashMap::from([("a".into(), "A".into()), ("b".into(), "B".into())]);
    input.register_hit(HitTarget { rect: canvas, event: None, control_id: Some("content.scroll".into()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &IconAtlas::default(), input: &mut input, theme: &theme, window_labels: &labels, window_icon_ids: &HashMap::new() };
    dock.register_resize_hits(&mut ctx, canvas);
    let hit = input.hit_at(200.0, 150.0).expect("split hit");
    assert_eq!(hit.kind, HitKind::DockSplit);
    assert_eq!(hit.drag_axis, Some(DragAxis::Horizontal));
    assert!(hit.rect.w >= 20.0);
}

fn stack_with(id: &str) -> DockNode {
    DockNode::Stack { windows: vec![tab(id)], active: id.into() }
}

//#region DragDropAndLayoutDiffTests

fn tab_payload(window_id: &str, source_path: DockPath, tab_index: usize) -> DockDragPayload {
    DockDragPayload { kind: DockDragKind::Tab, window_id: window_id.into(), source_path, tab_index, ghost_label: window_id.into() }
}

fn stack_payload(window_id: &str, source_path: DockPath, tab_index: usize) -> DockDragPayload {
    DockDragPayload { kind: DockDragKind::Stack, window_id: window_id.into(), source_path, tab_index, ghost_label: window_id.into() }
}

/// 🎯️ Regression pin for the double-removal bug: `apply_drop` used to call `remove_window` again
/// on a window the caller (`ShellState::handle_pointer_move`) had *already* removed at drag
/// promotion, so `remove_window` always failed and every cross-stack tab drop silently no-opped.
#[test]
fn apply_drop_tab_moves_window_across_stacks() {
    let mut dock = DockState::default();
    // 🪟️ Three stacks so removing `a` (the sole occupant of stack `[0]`) prunes that slot without
    // also emptying the drop target — `b` shifts from `[1]` down to `[0]`, and `c` (the actual
    // cross-stack drop target) shifts from `[2]` to `[1]`.
    dock.root = DockNode::Row(vec![(stack_with("a"), 0.34), (stack_with("b"), 0.33), (stack_with("c"), 0.33)]);
    // 🎬️ Mirrors `ShellState::handle_pointer_move`'s eager removal at drag-promotion time. In the
    // real runtime `compute_dock_drop_zone` re-derives `stack_path` from the *current* (already
    // shifted) tree on every subsequent pointer move, so `zone` below targets `c`'s post-removal
    // path `[1]`, exactly as a live drag would have resolved it — not `c`'s stale pre-removal `[2]`.
    assert!(dock.remove_window("a"));
    assert_eq!(node_at(&dock.root, &vec![1]), Some(&stack_with("c")), "c shifted to [1] once a's slot was pruned");
    let payload = tab_payload("a", vec![0], 0);
    let zone = DockDropZone::Tab { stack_path: vec![1], corner: WindowStackCorner::TopLeft, index: 0 };
    assert!(dock.apply_drop(&payload, &zone), "cross-stack tab drop must actually land");
    assert_eq!(node_at(&dock.root, &vec![1]), Some(&stack_tabs(&["a", "c"], "a")));
    assert!(find_stack_path(&dock.root, "b", &mut vec![]).is_some(), "b (untouched by this drop) still resolvable");
    assert_eq!(dock.active_window_id.as_deref(), Some("a"));
}

#[test]
fn apply_drop_tab_reinserts_into_originating_stack_at_new_index() {
    let mut dock = DockState::default();
    dock.root = DockNode::Row(vec![(stack_tabs(&["a", "b", "c"], "a"), 1.0)]);
    assert!(dock.remove_window("a"));
    let payload = tab_payload("a", vec![0], 0);
    let zone = DockDropZone::Tab { stack_path: vec![0], corner: WindowStackCorner::TopLeft, index: 2 };
    assert!(dock.apply_drop(&payload, &zone));
    assert_eq!(node_at(&dock.root, &vec![0]), Some(&stack_tabs(&["b", "c", "a"], "a")));
}

#[test]
fn apply_drop_tab_split_targets_the_post_removal_stack() {
    let mut dock = DockState::default();
    dock.root = DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]);
    assert!(dock.remove_window("a"));
    // 🪟️ Removing the sole occupant of stack `a` prunes it — `b` now sits at path `[0]`.
    let payload = tab_payload("a", vec![0], 0);
    let zone = DockDropZone::Split { stack_path: vec![0], side: DockSide::Right };
    assert!(dock.apply_drop(&payload, &zone));
    assert!(find_stack_path(&dock.root, "a", &mut vec![]).is_some());
    assert!(find_stack_path(&dock.root, "b", &mut vec![]).is_some());
    assert_eq!(dock.active_window_id.as_deref(), Some("a"));
}

#[test]
fn apply_drop_tab_root_split_builds_axis_pair() {
    let mut dock = DockState::default();
    dock.root = stack_tabs(&["a", "b"], "a");
    assert!(dock.remove_window("a"));
    let payload = tab_payload("a", vec![], 0);
    let zone = DockDropZone::RootSplit { side: DockSide::Left };
    assert!(dock.apply_drop(&payload, &zone));
    assert_eq!(dock.root, DockNode::Row(vec![(stack_with("a"), 0.5), (stack_tabs(&["b"], "b"), 0.5)]));
}

#[test]
fn apply_drop_stack_moves_whole_group_preserving_order_and_target_key() {
    let mut dock = DockState::default();
    dock.root = DockNode::Row(vec![(stack_tabs(&["a", "b", "c"], "b"), 0.5), (stack_with("d"), 0.5)]);
    // 🎬️ A stack drag pre-removes only its active window, same as a tab drag.
    assert!(dock.remove_window("b"));
    let payload = stack_payload("b", vec![0], 1);
    let zone = DockDropZone::Tab { stack_path: vec![1], corner: WindowStackCorner::TopLeft, index: 0 };
    assert!(dock.apply_drop(&payload, &zone), "whole-stack tab-join must land");
    // 🔑️ `a`/`c` (the siblings left behind by the eager single-window removal) travel with `b`,
    // reconstructed in their original order around it, and land next to `d` by key — not by the
    // pre-extraction `stack_path`, which the extraction itself would have shifted.
    let target = find_stack_path(&dock.root, "d", &mut vec![]).expect("d still resolvable by key");
    assert_eq!(node_at(&dock.root, &target), Some(&stack_tabs(&["a", "b", "c", "d"], "b")));
    assert!(find_stack_path(&dock.root, "a", &mut vec![]).is_some());
    assert_eq!(dock.active_window_id.as_deref(), Some("b"));
}

#[test]
fn apply_drop_stack_split_reanchors_target_after_extraction_shifts_paths() {
    let mut dock = DockState::default();
    // 🔑️ Source stack `[a, x]` keeps two windows, so the eager active-window removal at promotion
    // (`remove_window("a")`) does *not* collapse it — `b` stays at `[1]` right up until
    // `extract_stack_group` later pulls `x` out too, which *does* empty-and-prune slot `[0]`,
    // shifting `b` from `[1]` down to `[0]`. The `stack_path: [1]` captured in `zone` (from
    // hit-testing *before* this drag even started) is therefore stale by the time the drop lands.
    dock.root = DockNode::Row(vec![(stack_tabs(&["a", "x"], "a"), 0.5), (stack_with("b"), 0.5)]);
    assert!(dock.remove_window("a"));
    assert_eq!(node_at(&dock.root, &vec![1]), Some(&stack_with("b")), "b starts at [1], pre-shift");
    let payload = stack_payload("a", vec![0], 0);
    let zone = DockDropZone::Split { stack_path: vec![1], side: DockSide::Bottom };
    assert!(dock.apply_drop(&payload, &zone), "split must land even though [1] goes stale mid-drop");
    // `b` shifted to `[0]` once `x` was extracted and slot `[0]` collapsed — proof the naive stale
    // path would have missed (or misdirected onto) the wrong node.
    assert_eq!(node_at(&dock.root, &vec![1]), None, "the pre-extraction path is no longer valid at all");
    let b_path = find_stack_path(&dock.root, "b", &mut vec![]).expect("b still resolvable by key");
    assert_eq!(node_at(&dock.root, &b_path), Some(&stack_with("b")), "b itself must be untouched by the split");
    let a_path = find_stack_path(&dock.root, "a", &mut vec![]).expect("a resolvable by key");
    assert_eq!(node_at(&dock.root, &a_path), Some(&stack_tabs(&["a", "x"], "a")), "a's whole group (a + the sibling x it dragged along) landed together");
}

#[test]
fn apply_drop_stack_same_source_is_noop() {
    let mut dock = DockState::default();
    dock.root = DockNode::Row(vec![(stack_tabs(&["a", "b"], "a"), 0.5), (stack_with("c"), 0.5)]);
    assert!(dock.remove_window("a"));
    let before = dock.root.clone();
    let payload = stack_payload("a", vec![0], 0);
    let zone = DockDropZone::Tab { stack_path: vec![0], corner: WindowStackCorner::TopLeft, index: 0 };
    assert!(!dock.apply_drop(&payload, &zone), "dropping a stack back onto itself is a no-operation");
    assert_eq!(dock.root, before);
}

#[test]
fn compute_dock_drop_zone_prefers_tab_bar_over_body_over_root() {
    let tab_bars = vec![(vec![0], WindowStackCorner::TopLeft, Rect::new(0.0, 0.0, 200.0, 24.0), vec![80.0, 80.0])];
    let bodies = vec![(vec![0], Rect::new(0.0, 24.0, 200.0, 200.0), "a".to_string())];
    let canvas = Rect::new(0.0, 0.0, 200.0, 224.0);
    // Inside the tab bar rect → `Tab` zone wins even though it's also inside the body's column span.
    assert_eq!(compute_dock_drop_zone(10.0, 10.0, &tab_bars, &bodies, canvas), Some(DockDropZone::Tab { stack_path: vec![0], corner: WindowStackCorner::TopLeft, index: 0 }));
    // Inside the body but below the tab bar → `Split` zone.
    assert!(matches!(compute_dock_drop_zone(100.0, 100.0, &tab_bars, &bodies, canvas), Some(DockDropZone::Split { .. })));
    // Outside every registered stack but inside the canvas → `RootSplit`.
    assert!(matches!(compute_dock_drop_zone(190.0, 300.0, &tab_bars, &bodies, canvas), None));
    let wide_canvas = Rect::new(0.0, 0.0, 400.0, 400.0);
    assert!(matches!(compute_dock_drop_zone(390.0, 390.0, &tab_bars, &bodies, wide_canvas), Some(DockDropZone::RootSplit { .. })));
}

#[test]
fn resolve_split_side_uses_dominant_axis_from_center() {
    // Wide-and-short body: a small vertical offset from center stays dominated by the x-axis.
    assert_eq!(resolve_split_side(10.0, 60.0, 400.0, 120.0), DockSide::Left);
    assert_eq!(resolve_split_side(390.0, 60.0, 400.0, 120.0), DockSide::Right);
    // Tall-and-narrow body: a small horizontal offset stays dominated by the y-axis.
    assert_eq!(resolve_split_side(60.0, 10.0, 120.0, 400.0), DockSide::Top);
    assert_eq!(resolve_split_side(60.0, 390.0, 120.0, 400.0), DockSide::Bottom);
}

#[test]
fn apply_layout_diff_keeps_current_tab_focused_over_stale_persisted_active() {
    let mut dock = DockState::default();
    dock.root = stack_tabs(&["a", "b"], "b");
    dock.active_window_id = Some("b".into());
    // 🗄️ A persisted `WindowLayout` snapshot whose `active_window_kind_id` predates the user's
    // later in-session tab switch to `b` — a naive `self.dock.root = dock_from_window_layout(...)`
    // teardown would silently revert focus back to `a`.
    let stale_layout = WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: Some("a".into()),
            children: vec![
                WindowLayoutWindowNode { kind: "window".into(), window_kind_id: "a".into(), title: None, instance_id: None, template_id: None, corner: None },
                WindowLayoutWindowNode { kind: "window".into(), window_kind_id: "b".into(), title: None, instance_id: None, template_id: None, corner: None },
            ],
        }),
    };
    dock.apply_layout_diff(&stale_layout);
    assert_eq!(dock.root, stack_tabs(&["a", "b"], "b"), "same membership, reordered-or-not — the user's current tab stays focused");
}

#[test]
fn apply_layout_diff_reanchors_active_and_maximized_stack_by_key() {
    let mut dock = DockState::default();
    dock.root = DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]);
    dock.active_window_id = Some("b".into());
    dock.active_stack = Some(vec![1]);
    dock.maximized_stack = Some(vec![1]);
    dock.split_resize_origin = vec![0.5, 0.5];
    // 🔑️ The incoming layout reverses the two stacks' order — `b`'s *positional* path moves from
    // `[1]` to `[0]`. A stale-path reuse would now silently misdirect `active_stack`/
    // `maximized_stack` at `a` instead of following `b` by key.
    let reversed = WindowLayout {
        root: WindowLayoutRoot::Axis(ui_wgpu::wgpu::WindowLayoutAxisNode {
            kind: "row".into(),
            size: None,
            children: vec![
                WindowLayoutChild::Stack(WindowLayoutStackNode {
                    kind: "stack".into(),
                    size: Some(0.5),
                    active_window_kind_id: Some("b".into()),
                    children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: "b".into(), title: None, instance_id: None, template_id: None, corner: None }],
                }),
                WindowLayoutChild::Stack(WindowLayoutStackNode {
                    kind: "stack".into(),
                    size: Some(0.5),
                    active_window_kind_id: Some("a".into()),
                    children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: "a".into(), title: None, instance_id: None, template_id: None, corner: None }],
                }),
            ],
        }),
    };
    dock.apply_layout_diff(&reversed);
    assert_eq!(dock.active_window_id.as_deref(), Some("b"));
    assert_eq!(dock.active_stack, Some(vec![0]));
    assert_eq!(dock.maximized_stack, Some(vec![0]));
    assert!(dock.split_resize_origin.is_empty(), "an in-flight resize gesture's stale indices must not survive a layout swap");
}

#[test]
fn apply_layout_diff_clears_maximized_stack_when_its_window_is_gone() {
    let mut dock = DockState::default();
    dock.root = DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]);
    dock.active_window_id = Some("a".into());
    dock.maximized_stack = Some(vec![0]);
    let without_a = WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: Some("b".into()),
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: "b".into(), title: None, instance_id: None, template_id: None, corner: None }],
        }),
    };
    dock.apply_layout_diff(&without_a);
    assert_eq!(dock.maximized_stack, None);
    assert_eq!(dock.active_window_id.as_deref(), Some("b"));
}

#[test]
fn diff_dock_node_reuses_unchanged_subtree_and_adopts_new_shape_where_changed() {
    let old = DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]);
    // Identical row → the whole node is byte-for-byte the same value (full reuse).
    let unchanged = diff_dock_node(&old, DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]));
    assert_eq!(unchanged, old);
    // A structural kind change (Stack -> Row) at index 1 has no shared identity — adopt `next` as-is.
    let next = DockNode::Row(vec![(stack_with("a"), 0.5), (DockNode::Row(vec![(stack_with("b"), 1.0)]), 0.5)]);
    let diffed = diff_dock_node(&old, next.clone());
    assert_eq!(diffed, next);
}

//#endregion DragDropAndLayoutDiffTests

#[test]
fn even_layout_single_window() {
    let node = even_layout(&["main".into()]);
    assert!(matches!(node, DockNode::Stack { .. }));
    let dock = DockState::from_app(&sample_app(&["main"], None), None);
    assert_eq!(dock.active_window_id.as_deref(), Some("main"));
}

#[test]
fn even_layout_multiple_windows() {
    let node = even_layout(&["a".into(), "b".into(), "c".into()]);
    assert!(matches!(node, DockNode::Row(_)));
    if let DockNode::Row(children) = node {
        assert_eq!(children.len(), 3);
        for (child, size) in children {
            assert!(matches!(child, DockNode::Stack { .. }));
            assert!((size - 1.0 / 3.0).abs() < 0.001);
        }
    }
}

#[test]
fn parses_default_layout_row() {
    let layout = create_default_layout(&["a".into(), "b".into()], "row", None, None);
    let app = sample_app(&["a", "b"], Some(layout));
    let dock = DockState::from_app(&app, None);
    assert!(matches!(dock.root, DockNode::Row(_)));
}

#[test]
fn close_window_collapses_stack_tabs() {
    let mut dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
    dock.root = stack_tabs(&["a", "b"], "a");
    let path = vec![];
    dock.close_active_in_stack(&path);
    if let DockNode::Stack { windows, active } = &dock.root {
        assert_eq!(dock_tab_ids(windows), vec!["b".to_string()]);
        assert_eq!(active, "b");
    } else {
        panic!("expected stack");
    }
}

#[test]
fn reorder_tab_within_stack() {
    let mut dock = DockState::default();
    dock.root = stack_tabs(&["a", "b", "c"], "a");
    assert!(dock.reorder_tab(&vec![], 0, 2));
    if let DockNode::Stack { windows, .. } = &dock.root {
        assert_eq!(dock_tab_ids(windows), vec!["b".to_string(), "c".to_string(), "a".to_string()]);
    } else {
        panic!("expected stack");
    }
}

#[test]
fn maximized_stack_uses_full_canvas_bounds() {
    let mut dock = DockState::from_app(&sample_app(&["a", "b", "c"], None), Some("a"));
    dock.root = even_layout(&["a".into(), "b".into(), "c".into()]);
    dock.toggle_maximize(&vec![1]);
    let canvas = Rect::new(0.0, 0.0, 900.0, 600.0);
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let bodies = dock.stack_body_rects(canvas, &theme, &HashMap::new(), &mut atlas);
    assert_eq!(bodies.len(), 1);
    let (_, body, _) = &bodies[0];
    assert!((body.w - canvas.w).abs() < 1.0);
    assert!((body.h - (canvas.h - theme.control_height)).abs() < 2.0);
}

#[test]
fn split_drop_preview_covers_half_panel() {
    let body = Rect::new(10.0, 20.0, 400.0, 300.0);
    let left = split_drop_preview_in_body(body, DockSide::Left);
    assert_eq!(left.x, 10.0);
    assert_eq!(left.w, 200.0);
    assert_eq!(left.h, 300.0);
    let right = split_drop_preview_in_body(body, DockSide::Right);
    assert_eq!(right.x, 210.0);
    assert_eq!(right.w, 200.0);
}

#[test]
fn map_marquee_mode_matches_ui_react() {
    use crate::engine_canvas::map_marquee_mode;
    assert_eq!(map_marquee_mode(false, false), "default");
    assert_eq!(map_marquee_mode(true, false), "additive");
    assert_eq!(map_marquee_mode(false, true), "subtractive");
    assert_eq!(map_marquee_mode(true, true), "invertive");
}

//#region WindowActionsAndUtilitiesTests
use semio_framework::{ActionArgDef, ActionDefinition, ActionKind, UtilityDefinition, UtilityRef};
use ui_wgpu::wgpu::{KeyAction, PointerModifiers};

fn mods(meta: bool, ctrl: bool, shift: bool, alt: bool) -> PointerModifiers {
    PointerModifiers { meta, ctrl, shift, alt }
}

/// 🧰️ Builds a two-window app: window `main` scopes `utility.a`, window `aux` scopes nothing; `utility.b`
/// is an orphan (no window references it). Actions: `zeroArg` (no args) + `withArgs` (required text +
/// defaulted toggle) scoped to `main`.
fn actions_utilities_app() -> AppDefinition {
    let mut app = sample_app(&["main", "aux"], None);
    app.controller_id = "ctrl".into();
    app.utilities = vec![UtilityDefinition::new("utility.a", LocalizedLabel::data("Utility A"), "circle"), UtilityDefinition { allows_actions_while_active: true, ..UtilityDefinition::new("utility.b", LocalizedLabel::data("Utility B"), "square") }];
    let actions = vec![
        ActionDefinition::bounded_catalog("zeroArg", LocalizedLabel::data("Zero Arg"), ActionKind::View),
        ActionDefinition {
            args: vec![ActionArgDef::text("name", LocalizedLabel::data("Name")).required(), ActionArgDef { default: Some(semio_framework::to_dsl_value(&true).expect("toggle default")), ..ActionArgDef::toggle("flag", LocalizedLabel::data("Flag")) }],
            keys: Some("mod+e".into()),
            ..ActionDefinition::bounded_catalog("withArgs", LocalizedLabel::data("With Args"), ActionKind::View)
        },
    ];
    // Scope utility.a + both actions to `main`; leave utility.b an orphan referenced by no window.
    for kind in app.window_kinds.iter_mut() {
        if kind.id == "main" {
            kind.utilities = vec![UtilityRef::new("utility.a")];
            kind.actions = actions.clone();
        }
    }
    app
}

fn shell() -> ShellState {
    ShellState::new(vec![], "test".into())
}

#[test]
fn resolve_window_utilities_scopes_explicit_and_orphans() {
    let app = actions_utilities_app();
    let main = app.window_kinds.iter().find(|k| k.id == "main").unwrap();
    let aux = app.window_kinds.iter().find(|k| k.id == "aux").unwrap();
    let main_ids: Vec<&str> = crate::shell::resolve_window_utilities(&app, main).iter().map(|t| t.id.as_str()).collect();
    let aux_ids: Vec<&str> = crate::shell::resolve_window_utilities(&app, aux).iter().map(|t| t.id.as_str()).collect();
    // `main` gets its explicit utility.a first, then the orphan utility.b; `aux` only sees the orphan.
    assert_eq!(main_ids, vec!["utility.a", "utility.b"]);
    assert_eq!(aux_ids, vec!["utility.b"]);
}

#[test]
fn key_event_matches_chord_respects_modifiers() {
    use crate::shell::key_event_matches_chord;
    let z = KeyAction::Char("z".into());
    assert!(key_event_matches_chord(&z, &mods(true, false, false, false), "mod+z"));
    assert!(key_event_matches_chord(&z, &mods(false, true, false, false), "mod+z"));
    // shift held but not declared → no match; declared shift required.
    assert!(!key_event_matches_chord(&z, &mods(true, false, true, false), "mod+z"));
    assert!(key_event_matches_chord(&z, &mods(true, false, true, false), "mod+shift+z"));
    // plain key must not fire while the accelerator is held.
    let k = KeyAction::Char("k".into());
    assert!(key_event_matches_chord(&k, &mods(false, false, false, false), "k"));
    assert!(!key_event_matches_chord(&k, &mods(true, false, false, false), "k"));
    assert!(key_event_matches_chord(&KeyAction::Escape, &mods(false, false, false, false), "escape"));
}

#[test]
fn required_arg_gates_execution_and_merges_defaults() {
    let app = actions_utilities_app();
    let defs = &app.window_kinds.iter().find(|kind| kind.id == "main").unwrap().actions.iter().find(|action| action.id == "withArgs").unwrap().args;
    // Nothing staged → required `name` missing → no executable args (P2 gate).
    assert!(ShellState::resolved_execute_args(defs, &serde_json::Map::new()).is_none());
    // Stage the required arg → executes, merging the defaulted `flag`.
    let mut staged = serde_json::Map::new();
    staged.insert("name".into(), serde_json::json!("hello"));
    let merged = ShellState::resolved_execute_args(defs, &staged).expect("executable");
    assert_eq!(merged.get("name"), Some(&serde_json::json!("hello")));
    assert_eq!(merged.get("flag"), Some(&serde_json::json!(true)));
}

#[test]
fn staging_stage_and_reset_roundtrip() {
    let mut shell = shell();
    shell.stage_arg("main", "withArgs", "name", serde_json::json!("x"));
    shell.stage_arg("main", "withArgs", "flag", serde_json::json!(false));
    let staged = shell.staged_map_for("main", "withArgs");
    assert_eq!(staged.get("name"), Some(&serde_json::json!("x")));
    assert_eq!(staged.get("flag"), Some(&serde_json::json!(false)));
    shell.reset_staged_args("main", "withArgs");
    assert!(shell.staged_map_for("main", "withArgs").is_empty());
}

#[test]
fn utility_activation_toggles_and_switches() {
    let mut shell = shell();
    shell.apply_set_active_utility("main", "utility.a");
    assert_eq!(shell.active_utility_for_window("main"), Some("utility.a"));
    // Re-selecting the active utility deactivates it (the same update a re-click / Escape performs).
    shell.apply_set_active_utility("main", "utility.a");
    assert_eq!(shell.active_utility_for_window("main"), None);
    // Switching to a different utility activates it.
    shell.apply_set_active_utility("main", "utility.a");
    shell.apply_set_active_utility("main", "utility.b");
    assert_eq!(shell.active_utility_for_window("main"), Some("utility.b"));
}

#[test]
fn active_utility_gates_actions_unless_allowed() {
    let app = actions_utilities_app();
    let mut shell = shell();
    // No active utility → actions enabled.
    assert!(shell.actions_enabled_for_window(&app, "main"));
    // utility.a defaults to `allows_actions_while_active = false` → actions gated.
    shell.apply_set_active_utility("main", "utility.a");
    assert!(!shell.actions_enabled_for_window(&app, "main"));
    // utility.b sets the flag true → actions stay enabled.
    shell.apply_set_active_utility("main", "utility.a");
    shell.apply_set_active_utility("main", "utility.b");
    assert!(shell.actions_enabled_for_window(&app, "main"));
}

#[test]
fn action_host_window_id_finds_scoping_window() {
    let app = actions_utilities_app();
    assert_eq!(crate::shell::action_host_window_id(&app, "withArgs").as_deref(), Some("main"));
}

/// 🎯️ The Utility Options rail (`render_utility_options_rail`) resolves its content through
/// `partition_window_measures`: a tagged group surfaces only for its matching active utility, and is
/// absent from BOTH buckets otherwise — untagged groups always stay in the general Measures rail.
#[test]
fn utility_options_partition_gates_tagged_group_by_active_utility() {
    use ui_wgpu::wgpu::{ActionDescriptor, WindowMeasure, partition_window_measures};
    let measures = vec![
        WindowMeasure::Group {
            id: "brush-params".into(),
            label: "Brush".into(),
            default_open: Some(true),
            active_utility_id: Some("utility.a".into()),
            value: None,
            min: None,
            max: None,
            step: None,
            ready: None,
            loading: None,
            waiting: None,
            on_change: None,
            children: vec![WindowMeasure::Toggle {
                id: "brush-size".into(),
                icon_id: "paintbrush".into(),
                label: Some("Size".into()),
                pressed: false,
                text: None,
                on_change: ActionDescriptor { controller_id: "test".into(), action: "noOperation".into(), args: None },
            }],
        },
        WindowMeasure::Group { id: "grid".into(), label: "Grid".into(), default_open: Some(true), active_utility_id: None, value: None, min: None, max: None, step: None, ready: None, loading: None, waiting: None, on_change: None, children: vec![] },
    ];
    let ui_wgpu::wgpu::component::layout::WindowMeasurePartition { general, utility_options } = partition_window_measures(&measures, Some("utility.a")).expect("bounded fixture measure partition");
    assert_eq!(utility_options.len(), 1, "matching utility surfaces the tagged group in utility options");
    assert!(matches!(utility_options.get(0).copied(), Some(WindowMeasure::Toggle { id, .. }) if id == "brush-size"));
    assert_eq!(general.len(), 1, "untagged group stays in the general measures rail");
    assert!(matches!(general.get(0).copied(), Some(WindowMeasure::Group { id, .. }) if id == "grid"));
    let ui_wgpu::wgpu::component::layout::WindowMeasurePartition { general: general_other, utility_options: utility_options_other } = partition_window_measures(&measures, Some("utility.b")).expect("bounded fixture measure partition");
    assert!(utility_options_other.is_empty(), "wrong active utility drops the tagged group");
    assert_eq!(general_other.len(), 1, "untagged group unaffected by active utility");
    let ui_wgpu::wgpu::component::layout::WindowMeasurePartition { general: general_none, utility_options: utility_options_none } = partition_window_measures(&measures, None).expect("bounded fixture measure partition");
    assert!(utility_options_none.is_empty(), "no active utility drops the tagged group");
    assert_eq!(general_none.len(), 1);
}
//#endregion WindowActionsAndUtilitiesTests
