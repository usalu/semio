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
use ui_wgpu::wgpu::{create_default_layout, WindowOptions};

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
        actions: vec![],
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
fn concrete_window_instances_round_trip_without_kind_collapse() {
    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ExpectedWindow {
        id: String,
        window_kind_id: String,
    }
    #[derive(serde::Deserialize)]
    struct Fixture {
        layout: WindowLayout,
        expected: Vec<ExpectedWindow>,
    }
    let fixture: Fixture = serde_json::from_str(include_str!("../../🧫️fixtures/🪟️concrete-window-instances.json")).expect("concrete window fixture");
    let mut dock = DockState::from_app(&sample_app(&["canvas"], Some(fixture.layout)), Some("canvas-copy"));
    let actual = dock.window_instances();
    assert_eq!(actual, fixture.expected.iter().map(|window| (window.id.clone(), window.window_kind_id.clone())).collect::<Vec<_>>());
    assert_eq!(dock.active_window_id.as_deref(), Some("canvas-copy"));
    let DockNode::Stack { windows, .. } = &dock.root else { panic!("fixture root must be a stack") };
    let labels = HashMap::from([("canvas".to_string(), "Canvas".to_string())]);
    let mut atlas = FontAtlas::builtin();
    let chrome = layout_stack_cap(windows, &labels, &HashMap::new(), &mut atlas, &Theme::default(), Rect::new(0.0, 0.0, 640.0, 480.0), 0);
    assert_eq!(chrome.groups[0].tabs.iter().map(|tab| tab.label.as_str()).collect::<Vec<_>>(), vec!["Canvas", "Canvas"]);
    let payload = DockDragPayload { kind: DockDragKind::Tab, window_id: "canvas-copy".into(), window_kind_id: "canvas".into(), template_id: None, source_path: Vec::new(), tab_index: 1, ghost_label: "Canvas copy".into() };
    assert!(dock.apply_drop(&payload, &DockDropZone::RootSplit { side: DockSide::Right }));
    assert_eq!(dock.window_kind_id("canvas-copy"), Some("canvas"));
    let persisted = serde_json::to_value(dock.to_window_layout()).expect("persisted layout");
    assert!(persisted.to_string().contains("canvas-copy"));
    println!("[DEBUG] native dock retained two concrete instances of one window kind across chrome, render and drag persistence");
}

#[test]
fn split_axis_extent_uses_row_width_not_canvas_max() {
    let mut dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
    dock.root = DockNode::Column(vec![(DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]), 0.5), (stack_with("c"), 0.5)]);
    let canvas = Rect::new(0.0, 0.0, 1000.0, 800.0);
    let separator = Theme::default().gap_standard;
    let row_extent = dock.split_axis_extent(&vec![0], canvas, separator).unwrap();
    assert!((row_extent - (1000.0 - separator)).abs() < 0.1);
    let col_extent = dock.split_axis_extent(&vec![], canvas, separator);
    assert!((col_extent.unwrap() - (800.0 - separator)).abs() < 0.1);
    let nested_extent = dock.split_axis_extent(&vec![0], canvas, separator).unwrap();
    assert!((nested_extent - (1000.0 - separator)).abs() < 0.1);
}

#[test]
fn unregistered_world_and_pane_identifiers_cannot_claim_scene_wheel() {
    let shell = ShellState::new(Vec::new(), String::new());
    let theme = Theme::default();
    for (kind, id) in [(HitKind::ScrollRegion, "panel.left.lowpoly"), (HitKind::World3d, "world-surface"), (HitKind::ScrollRegion, "graph-surface.pane"), (HitKind::ScrollRegion, "map-surface.map")] {
        let mut input = InputState::<ActionDescriptor>::default();
        input.register_hit(HitTarget { rect: Rect::new(0.0, 0.0, 800.0, 600.0), event: None, control_id: Some(id.into()), kind, drag_axis: None, drag_data: None });
        input.publish_hits();
        assert!(!shell.wheel_reaches_scene_surface(400.0, 300.0, &input, &theme), "{id} has no published scene provenance");
    }
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
    assert!((preview_rect.x - (flow_rect.x + flow_rect.w) - theme.gap_standard - theme.padding_standard * 2.0).abs() < 0.01, "📐️ one separator plus both adjacent body insets separate scene content");
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
    let layout = layout_stack_cap(&tabs(&["a", "b"]), &labels, &icon_ids, &mut atlas, &theme, bounds, 0);
    let gap_point = (bounds.x + bounds.w * 0.5, bounds.y + theme.control_height * 0.5);
    let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &icons, input: &mut input, theme: &theme, window_labels: &labels, window_icon_ids: &icon_ids, control_names: None };
    dock.paint_chrome(&mut ctx, bounds, false);
    assert_eq!(draw.glass_regions.len(), 1, "one top-left corner group chip for both tabs");
    assert!(!draw.glass_regions.iter().any(|region| Rect::new(region.rect[0], region.rect[1], region.rect[2], region.rect[3]).contains(gap_point.0, gap_point.1)));
    assert!(input.staged_hits().iter().any(|hit| hit.control_id.as_deref() == Some("dock.tab..a")));
    assert!(input.staged_hits().iter().any(|hit| hit.control_id.as_deref() == Some("dock.tab..a.close")));
    assert!(input.staged_hits().iter().any(|hit| hit.control_id.as_deref() == Some("dock.tab..a.focus")));
    assert!(!input.staged_hits().iter().any(|hit| hit.control_id.as_deref().is_some_and(|id| id.starts_with("dock.focus.") || id.starts_with("dock.close."))));
    assert!(!input.staged_hits().iter().any(|hit| hit.control_id.as_deref().is_some_and(|id| id.starts_with("dock.stack."))));
    let _ = layout;
}

#[test]
fn dock_cap_depth_is_control_plus_padding_with_inset_actions_and_active_fill() {
    let mut dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
    dock.root = stack_tabs(&["a", "b"], "a");
    dock.active_stack = Some(Vec::new());
    let bounds = Rect::new(10.0, 20.0, 600.0, 400.0);
    let mut theme = Theme::default();
    theme.control_height = 24.0;
    theme.padding_standard = 8.0;
    theme.navbar_height = 52.0;
    let cap_depth = 40.0;
    assert_eq!(dock_cap_depth(&theme), cap_depth);
    assert_ne!(dock_cap_depth(&theme), theme.navbar_height);
    assert_eq!(stack_tab_bar_rect(bounds, &theme).h, cap_depth);

    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut draw = DrawList::default();
    let labels = HashMap::from([("a".into(), "A".into()), ("b".into(), "B".into())]);
    let icon_ids = HashMap::new();
    let action_count = dock_tab_actions(dock.show_maximize(), false).len();
    let layout = layout_stack_cap(&tabs(&["a", "b"]), &labels, &icon_ids, &mut atlas, &theme, bounds, action_count);
    assert!(layout.groups.iter().flat_map(|group| &group.tabs).all(|tab| tab.rect.h == cap_depth));
    let silhouette = stack_window_silhouette(bounds, &theme, &layout);
    assert_eq!(silhouette.safe_body_rect().y, bounds.y + cap_depth);
    let active_rect = layout.groups.iter().flat_map(|group| &group.tabs).find(|tab| tab.window_id == "a").expect("active tab").rect;

    let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &icons, input: &mut input, theme: &theme, window_labels: &labels, window_icon_ids: &icon_ids, control_names: None };
    dock.paint_chrome(&mut ctx, bounds, false);
    let selected = [theme.selected.r, theme.selected.g, theme.selected.b, theme.selected.a];
    assert!(
        draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| instance.rect == [active_rect.x, active_rect.y, active_rect.w, active_rect.h] && instance.color == selected),
        "globally active selected tab owns the full cap fill"
    );

    let select = input.staged_hits().iter().find(|hit| hit.control_id.as_deref() == Some("dock.tab..a")).expect("select hit");
    assert_eq!((select.rect.y, select.rect.h), (bounds.y, cap_depth));
    for suffix in ["focus", "close", "drag"] {
        let id = format!("dock.tab..a.{suffix}");
        let action = input.staged_hits().iter().find(|hit| hit.control_id.as_deref() == Some(id.as_str())).unwrap_or_else(|| panic!("missing {id}"));
        assert_eq!((action.rect.y, action.rect.h), (bounds.y + theme.padding_standard, theme.control_height));
    }
}

#[test]
fn apply_drop_tab_moves_window_to_target_corner() {
    let mut dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
    dock.root = DockNode::Row(vec![(stack_tabs(&["a", "x"], "a"), 0.5), (stack_tabs(&["b"], "b"), 0.5)]);
    let payload = tab_payload("a", vec![0], 0);
    let zone = DockDropZone::Tab { stack_path: vec![1], corner: WindowStackCorner::BottomRight, index: 0 };
    assert!(dock.apply_drop(&payload, &zone));
    let tabs = dock.stack_tabs_at_path(&vec![1]).expect("target stack");
    assert!(tabs.iter().any(|tab| tab.window_id == "a" && tab.corner == WindowStackCorner::BottomRight));
    assert_eq!(dock.active_window_id.as_deref(), Some("a"));
}

/// 🪟️ ONE merged top span, not one per tab: React's tab bar is `flex items-stretch justify-start`
/// with no gap utility (`🎨️Canvas/🟦️.tsx:1108`), so adjacent chips form a single silhouette edge
/// and the content clip is body + that one span.
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
    let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &icons, input: &mut input, theme: &theme, window_labels: &labels, window_icon_ids: &icon_ids, control_names: None };
    dock.paint_chrome(&mut ctx, bounds, true);
    let fill = draw
        .layers
        .iter()
        .find(|layer| layer.clip.is_some() && layer.ui_instances.iter().any(|instance| instance.rect == [bounds.x, bounds.y, bounds.w, bounds.h]))
        .expect("full silhouette content fill");
    assert_eq!(fill.clip.as_ref().map(|clip| clip.scissors.len()), Some(2));
    assert!(!fill.clip.as_ref().is_some_and(|clip| clip.scissors.iter().any(|rect| rect.x <= 300 && 300 < rect.x + rect.w && rect.y <= 30 && 30 < rect.y + rect.h)));
}

//#endregion SilhouetteContentTests

/// 🏁️ `register_hit` only STAGES; `hit_at` resolves the PUBLISHED registry, so the frame build has
/// to be promoted before the pointer authority can see either target.
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
    let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &IconAtlas::default(), input: &mut input, theme: &theme, window_labels: &labels, window_icon_ids: &HashMap::new(), control_names: None };
    dock.register_resize_hits(&mut ctx, canvas);
    input.publish_hits();
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
    DockDragPayload { kind: DockDragKind::Tab, window_id: window_id.into(), window_kind_id: window_id.into(), template_id: None, source_path, tab_index, ghost_label: window_id.into() }
}

fn stack_payload(window_id: &str, source_path: DockPath, tab_index: usize) -> DockDragPayload {
    DockDragPayload { kind: DockDragKind::Stack, window_id: window_id.into(), window_kind_id: window_id.into(), template_id: None, source_path, tab_index, ghost_label: window_id.into() }
}

/// 🎯️ A cross-stack tab drop lands where the DERIVED tree said it would: the drag leaves the
/// committed tree alone and paints `render_view`'s docked-out derivation, so the `stack_path` the
/// pointer resolved there is the very path `apply_drop`'s own re-derivation addresses.
///
/// 🩸️ The lane used to remove the window from the committed tree at drag promotion and then insert
/// into the mutated tree; a drop that refused left the user's layout mutilated, and `apply_drop`'s
/// own (second) removal always failed, which silently no-opped every cross-stack drop.
///
/// 🪟️ Three stacks so lifting `a` (the sole occupant of stack `[0]`) prunes that slot without
/// also emptying the drop target — `b` shifts from `[1]` down to `[0]`, and `c` (the actual
/// cross-stack drop target) shifts from `[2]` to `[1]`.
#[test]
fn apply_drop_tab_moves_window_across_stacks() {
    let mut dock = DockState::default();
    dock.root = DockNode::Row(vec![(stack_with("a"), 0.34), (stack_with("b"), 0.33), (stack_with("c"), 0.33)]);
    let payload = tab_payload("a", vec![0], 0);
    let floating = dock.render_view(Some(&payload));
    assert_eq!(node_at(&floating.root, &vec![1]), Some(&stack_with("c")), "c shifted to [1] in the DERIVED tree the drag paints");
    assert_eq!(node_at(&dock.root, &vec![0]), Some(&stack_with("a")), "the committed tree still holds a — a drag is not an edit");
    let zone = DockDropZone::Tab { stack_path: vec![1], corner: WindowStackCorner::TopLeft, index: 0 };
    assert!(dock.apply_drop(&payload, &zone), "cross-stack tab drop must actually land");
    assert_eq!(node_at(&dock.root, &vec![1]), Some(&stack_tabs(&["a", "c"], "a")));
    assert!(find_stack_path(&dock.root, "b", &mut vec![]).is_some(), "b (untouched by this drop) still resolvable");
    assert_eq!(dock.active_window_id.as_deref(), Some("a"));
}

/// 🎯️ An abandoned drag costs nothing: the committed tree is byte-identical before and after, which
/// is what retires the `WindowLayout` snapshot the old eager-removal lane had to keep.
///
/// 🪟️ Lifting `a` leaves a one-child axis, which React's `collapseLayout` HOISTS — so the derived
/// tree the pointer hit-tests is the bare stack at the ROOT path, and that is the path the drop
/// carries. (The old drag lane collapsed on a prune-only rule and left the axis standing at `[0]`.)
#[test]
fn an_abandoned_tab_drag_leaves_the_committed_tree_untouched() {
    let mut dock = DockState::default();
    dock.root = DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]);
    let before = dock.root.clone();
    let payload = tab_payload("a", vec![0], 0);
    let floating = dock.render_view(Some(&payload));
    assert_eq!(floating.root, stack_with("b"), "the derivation hoists b out of the one-child axis");
    assert_eq!(dock.root, before);
    assert!(!dock.apply_drop(&payload, &DockDropZone::Tab { stack_path: vec![7], corner: WindowStackCorner::TopLeft, index: 0 }), "a zone that resolves to nothing refuses");
    assert_eq!(dock.root, before, "a refused drop is not an edit either");
}

#[test]
fn apply_drop_tab_reinserts_into_originating_stack_at_new_index() {
    let mut dock = DockState::default();
    dock.root = DockNode::Row(vec![(stack_tabs(&["a", "b", "c"], "a"), 1.0)]);
    let payload = tab_payload("a", vec![0], 0);
    let floating = dock.render_view(Some(&payload));
    assert_eq!(floating.root, stack_tabs(&["b", "c"], "b"));
    let zone = DockDropZone::Tab { stack_path: vec![], corner: WindowStackCorner::TopLeft, index: 2 };
    assert!(dock.apply_drop(&payload, &zone));
    assert_eq!(dock.root, stack_tabs(&["b", "c", "a"], "a"));
}

/// 🪟️ Lifting the sole occupant of stack `a` prunes its slot and hoists `b` to the ROOT of the
/// derived tree — so that is where the split lands, and the axis is rebuilt from scratch.
#[test]
fn apply_drop_tab_split_targets_the_post_removal_stack() {
    let mut dock = DockState::default();
    dock.root = DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]);
    let payload = tab_payload("a", vec![0], 0);
    let zone = DockDropZone::Split { stack_path: vec![], side: DockSide::Right };
    assert!(dock.apply_drop(&payload, &zone));
    assert_eq!(dock.root, DockNode::Row(vec![(stack_with("b"), 0.5), (stack_with("a"), 0.5)]));
    assert_eq!(dock.active_window_id.as_deref(), Some("a"));
}

#[test]
fn apply_drop_tab_root_split_builds_axis_pair() {
    let mut dock = DockState::default();
    dock.root = stack_tabs(&["a", "b"], "a");
    let payload = tab_payload("a", vec![], 0);
    let zone = DockDropZone::RootSplit { side: DockSide::Left };
    assert!(dock.apply_drop(&payload, &zone));
    assert_eq!(dock.root, DockNode::Row(vec![(stack_with("a"), 0.5), (stack_tabs(&["b"], "b"), 0.5)]));
}

/// 🪟️ A whole-stack drag lifts the stack NODE, tab order and all — `extractStackFromLayout`, not
/// a window-by-window removal that had to be stitched back together around `tab_index`.
#[test]
fn apply_drop_stack_moves_whole_group_preserving_order_and_target_key() {
    let mut dock = DockState::default();
    dock.root = DockNode::Row(vec![(stack_tabs(&["a", "b", "c"], "b"), 0.5), (stack_with("d"), 0.5)]);
    let payload = stack_payload("b", vec![0], 1);
    let floating = dock.render_view(Some(&payload));
    assert_eq!(floating.root, stack_with("d"), "the one remaining sibling hoists to the root");
    let zone = DockDropZone::Tab { stack_path: vec![], corner: WindowStackCorner::TopLeft, index: 0 };
    assert!(dock.apply_drop(&payload, &zone), "whole-stack tab-join must land");
    assert_eq!(dock.root, stack_tabs(&["a", "b", "c", "d"], "b"), "the group keeps its order and joins d's corner");
    assert_eq!(dock.active_window_id.as_deref(), Some("b"));
}

/// 🎯️ A whole-stack split needs no key re-anchoring at all under the committed-tree model: the zone
/// the pointer resolved against the docked-out tree IS a path into the tree `apply_drop` re-derives.
///
/// 🩸️ The old lane removed the drag's active window at promotion and extracted the REST of the source
/// stack inside `apply_drop`, so ancestors collapsed *between* hit-testing and committing; it had to
/// remember the target stack's active window and look its path up again afterwards.
#[test]
fn apply_drop_stack_split_lands_on_the_derived_path_without_reanchoring() {
    let mut dock = DockState::default();
    dock.root = DockNode::Row(vec![(stack_tabs(&["a", "x"], "a"), 0.5), (stack_with("b"), 0.5)]);
    let payload = stack_payload("a", vec![0], 0);
    let floating = dock.render_view(Some(&payload));
    assert_eq!(floating.root, stack_with("b"), "lifting the whole stack hoists b to the root");
    let zone = DockDropZone::Split { stack_path: vec![], side: DockSide::Bottom };
    assert!(dock.apply_drop(&payload, &zone), "the derived root path is exactly where the drop lands");
    assert_eq!(dock.root, DockNode::Column(vec![(stack_with("b"), 0.5), (stack_tabs(&["a", "x"], "a"), 0.5)]), "a travelled with its sibling x, as one stack node");
    assert_eq!(dock.active_window_id.as_deref(), Some("a"));
}

#[test]
fn apply_drop_stack_same_source_is_noop() {
    let mut dock = DockState::default();
    dock.root = DockNode::Row(vec![(stack_tabs(&["a", "b"], "a"), 0.5), (stack_with("c"), 0.5)]);
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
    assert!((body.w - (canvas.w - theme.padding_standard * 2.0)).abs() < 1.0);
    assert!((body.h - (canvas.h - dock_cap_depth(&theme))).abs() < 2.0);
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
    assert_eq!(map_marquee_mode(false, false), "replace");
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
    use ui_wgpu::wgpu::{partition_window_measures, ActionDescriptor, WindowMeasure};
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

//#region WindowSystemReactParityTests
// 🪟️ Ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1i — every assertion below names the React
// line it pins. Reference: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx` (`ModeDockTabBar`
// 974-1102, `showMaximize` 1158/1800, `removeWindowFromLayout` 307-318, `collapseLayout` 225-236,
// `applyAxisResizeDelta` 593-611) and `🔨️modules/🎛️chrome-control-presentation/🟦️.ts:35`.

fn dock_with(root: DockNode, active: &str) -> DockState {
    let mut dock = DockState::default();
    dock.root = root;
    dock.sync_active_window(active);
    dock
}

fn painted_tab_control_ids(dock: &DockState, labels: &HashMap<String, String>) -> Vec<String> {
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut draw = DrawList::default();
    let icon_ids = HashMap::new();
    let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &icons, input: &mut input, theme: &theme, window_labels: labels, window_icon_ids: &icon_ids, control_names: None };
    dock.paint_chrome(&mut ctx, Rect::new(0.0, 0.0, 600.0, 400.0), false);
    input.staged_hits().iter().filter_map(|hit| hit.control_id.clone()).filter(|id| id.starts_with("dock.tab.")).collect()
}

/// 📑️ React renders Focus/Unfocus only when `!mobile && canMaximize`, then Close unconditionally —
/// `🎨️Canvas/🟦️.tsx:1072-1100`. There is no third ("new window") action anywhere in that tab bar.
#[test]
fn dock_tab_actions_match_react_mode_dock_tab_bar() {
    assert_eq!(dock_tab_actions(true, false), vec![("focus", "maximize-2"), ("close", "x"), ("drag", "grip-vertical")]);
    assert_eq!(dock_tab_actions(true, true), vec![("focus", "minimize-2"), ("close", "x"), ("drag", "grip-vertical")], "a maximized stack shows Unfocus");
    assert_eq!(dock_tab_actions(false, false), vec![("close", "x"), ("drag", "grip-vertical")], "single window or mobile keeps Close plus the grip");
    assert!(!dock_tab_actions(true, false).iter().any(|(action, _)| *action == "new"), "the dead new-window chip is gone");
}

/// 📑️ `canMaximize = modeCollectWindowIds(layout).length > 1` (`🎨️Canvas/🟦️.tsx:1800`) and
/// `showMaximize = !mobile && canMaximize` (`:1158`).
#[test]
fn show_maximize_follows_window_count_and_mobile() {
    let single = dock_with(stack_tabs(&["a"], "a"), "a");
    assert!(!single.can_maximize());
    assert!(!single.show_maximize());
    let mut many = dock_with(DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]), "a");
    assert!(many.can_maximize());
    assert!(many.show_maximize());
    many.mobile = true;
    assert!(many.can_maximize(), "the canvas still holds two windows");
    assert!(!many.show_maximize(), "mobile hides Focus/Unfocus — a mobile window always fills the canvas");
}

/// 📑️ Exactly the React action set reaches the input layer: `.close` always, `.focus` only when the
/// canvas can maximize, and never a `.new` control id (no shell dispatch arm ever answered one).
#[test]
fn dock_tab_hits_register_only_reacts_two_actions() {
    let labels = HashMap::from([("a".to_string(), "A".to_string()), ("b".to_string(), "B".to_string())]);
    let two = dock_with(stack_tabs(&["a", "b"], "a"), "a");
    let ids = painted_tab_control_ids(&two, &labels);
    assert!(ids.iter().any(|id| id == "dock.tab..a.close"));
    assert!(ids.iter().any(|id| id == "dock.tab..a.focus"));
    assert!(ids.iter().any(|id| id == "dock.tab..a.drag"), "React's DragHandle is the ONLY drag origin, so it must be a target");
    assert!(!ids.iter().any(|id| id.ends_with(".new")), "no dead new-window control id is ever registered");
    let one = dock_with(stack_tabs(&["a"], "a"), "a");
    let ids = painted_tab_control_ids(&one, &labels);
    assert!(ids.iter().any(|id| id == "dock.tab..a.close"), "close stays reachable for a lone window");
    assert!(ids.iter().any(|id| id == "dock.tab..a.drag"), "so does the grip");
    assert!(!ids.iter().any(|id| id.ends_with(".focus")), "React hides Focus when the canvas holds one window");
}

/// 📑️ ⚖️ LAW: a tab publishes its SELECT target before its action chips, and its chips left to right
/// (`focus`, `close`, `drag`) — React's own DOM order (`🎨️Canvas/🟦️.tsx:1034-1100`: the
/// `<button role="tab">` label, then `mode-dock-tab-focus`, `mode-dock-tab-close`, then the
/// `DragHandle`).
///
/// 🩸️ The chips were published FIRST, so the first `dock.tab.…` row of a stack was its destructive
/// `close` chip. The parity probe's `window-reopen` step resolves "the dock tab" by scanning the
/// published rows in order: on React it landed on the `mode-dock-tabbar` container and did nothing,
/// here it landed on `dock.tab..puzzle3d-main-perspective.close` and closed the LAST world pane, after
/// which every window-owned chord was a hinted no-op against an empty dock
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `🗑️generated/w12c-parity-run-19/steps.json` step 18).
#[test]
fn a_tabs_select_target_is_published_before_its_destructive_chips() {
    let labels = HashMap::from([("a".to_string(), "A".to_string()), ("b".to_string(), "B".to_string())]);
    let two = dock_with(stack_tabs(&["a", "b"], "a"), "a");
    let ids = painted_tab_control_ids(&two, &labels);
    let first = ids.first().map(String::as_str);
    assert_eq!(first, Some("dock.tab..a"), "📑️ the first dock tab row a stack publishes is a SELECT target, never a close chip: {ids:?}");
    for window_id in ["a", "b"] {
        let position = |suffix: &str| ids.iter().position(|id| *id == format!("dock.tab..{window_id}{suffix}")).unwrap_or_else(|| panic!("📑️ {window_id}{suffix} is registered: {ids:?}"));
        let select = position("");
        assert!(select < position(".focus"), "📑️ {window_id}: select precedes Focus");
        assert!(position(".focus") < position(".close"), "📑️ {window_id}: Focus precedes Close, left to right");
        assert!(position(".close") < position(".drag"), "📑️ {window_id}: Close precedes the grip");
    }
    assert!(!ids.iter().take_while(|id| !id.ends_with("..a")).any(|id| id.ends_with(".close")), "📑️ no close chip is published before the tab it belongs to");
}

/// 🈳️ ⚖️ LAW: a stack holding NO tabs still answers a DROP body — React's `WindowChrome` keeps
/// rendering for an empty stack, which is how a dragged window is dropped back into an emptied dock —
/// but it owns no window and no silhouette, because React's `activeDescriptor` is `undefined` there
/// and its `stackBody` renders nothing (`🎨️Canvas/🟦️.tsx:1176`/`:1212`).
///
/// 🩸️ The silhouette was keyed by the stack's `active` tab unconditionally, so closing the last window
/// minted a `""`-keyed window: the engine-surface census reported `windows=["", "tool.fill", …]`, the
/// chrome census published a `window:` surface, and the body registered a full-bounds
/// `HitKind::ScrollRegion` with an EMPTY control id over the whole canvas
/// (`📓️w12c-chords-and-camera-live.md` §4.3).
#[test]
fn an_emptied_stack_keeps_its_drop_body_and_owns_no_window() {
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let labels = HashMap::from([("a".to_string(), "A".to_string())]);
    let mut dock = dock_with(stack_tabs(&["a"], "a"), "a");
    assert!(dock.close_window("a"), "🪟️ the last window closes");
    let (bodies, silhouettes) = dock.stack_body_rects_with_silhouettes(Rect::new(0.0, 0.0, 600.0, 400.0), &theme, &labels, &mut atlas);
    assert_eq!(bodies.len(), 1, "🈳️ the emptied root stack is still a drop body");
    assert_eq!(bodies[0].2, "", "🈳️ …and names no window");
    assert!(silhouettes.is_empty(), "🈳️ an empty stack owns no silhouette — React draws no window there");
    assert!(dock.collect_window_ids().is_empty(), "🈳️ and the dock holds no window id at all");
}

/// 📑️ `modeDockTabClassName` caps a tab at `max-w-[12rem]` and truncates its label
/// (`🎛️chrome-control-presentation/🟦️.ts:35`) — a long title must never widen the tab bar.
#[test]
fn dock_tab_label_truncates_at_reacts_twelve_rem_cap() {
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let long = "Generation Preview Viewport With A Very Long Window Title";
    let (display, width) = dock_tab_chip(&mut atlas, &theme, long, 2);
    assert!((width - MODE_DOCK_TAB_MAX_WIDTH_PX).abs() < 0.001, "a long tab is pinned to 12rem, got {width}");
    assert!(display.ends_with('…'), "truncation appends an ellipsis, got {display:?}");
    assert!(display.len() < long.len());
    let short = "Main";
    let (display_short, width_short) = dock_tab_chip(&mut atlas, &theme, short, 2);
    assert_eq!(display_short, short, "a short label is never truncated");
    assert!(width_short < MODE_DOCK_TAB_MAX_WIDTH_PX);
    assert_eq!(truncate_label_to_width(&mut atlas, short, theme.font_size_small, 10_000.0), short);
}

/// 🪟️ Closing the LAST tab of a stack collapses the split it lived in — React's
/// `collapseLayout(removeWindowFromLayout(prev, id))` (`🎨️Canvas/🟦️.tsx:1475-1485`). This used to be
/// a silent no-operation (`if windows.len() <= 1 { return false }`).
#[test]
fn closing_the_last_tab_of_a_stack_collapses_the_layout() {
    let mut dock = dock_with(DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]), "a");
    assert!(dock.close_window_in_stack(&vec![0], "a"), "React always allows close");
    assert_eq!(dock.root, stack_with("b"), "the single surviving child is hoisted into the root");
    assert_eq!(dock.active_window_id.as_deref(), Some("b"));
    assert_eq!(dock.active_stack, Some(vec![]));
}

/// 🪟️ React re-focuses `children[0]` of the stack, and the canvas re-focuses `remaining[0]`
/// (`🎨️Canvas/🟦️.tsx:313, 1481`) — not the tab to the left of the one that closed.
#[test]
fn closing_the_active_tab_focuses_the_first_remaining_window() {
    let mut dock = dock_with(stack_tabs(&["a", "b", "c"], "b"), "b");
    assert!(dock.close_window_in_stack(&vec![], "b"));
    assert_eq!(dock.root, stack_tabs(&["a", "c"], "a"));
    assert_eq!(dock.active_window_id.as_deref(), Some("a"));
}

/// 🪟️ Closing the only window leaves React's empty root stack and no active window.
#[test]
fn closing_the_only_window_empties_the_dock() {
    let mut dock = dock_with(stack_tabs(&["a"], "a"), "a");
    assert!(dock.close_window_in_stack(&vec![], "a"));
    assert_eq!(dock.root, DockNode::Stack { windows: vec![], active: String::new() });
    assert_eq!(dock.active_window_id, None);
    assert_eq!(dock.active_stack, None);
    assert!(!dock.close_window("a"), "a window that is not in the layout cannot be closed");
}

/// 🪟️ `collapseLayout` hoists a single-child axis and keeps `only.size ?? node.size`
/// (`🎨️Canvas/🟦️.tsx:225-236`) — the hoisted child carries the ratio it held among its own siblings.
#[test]
fn collapse_hoists_a_single_child_axis_keeping_the_childs_ratio() {
    let inner = DockNode::Row(vec![(stack_with("a"), 0.25), (stack_with("b"), 0.75)]);
    let mut dock = dock_with(DockNode::Column(vec![(inner, 0.4), (stack_with("c"), 0.6)]), "c");
    assert!(dock.close_window_in_stack(&vec![0, 0], "a"));
    assert_eq!(dock.root, DockNode::Column(vec![(stack_with("b"), 0.75), (stack_with("c"), 0.6)]), "the emptied row is hoisted away and b keeps its own 0.75");
    assert_eq!(dock.active_window_id.as_deref(), Some("c"), "closing a non-active window leaves focus alone");
}

/// 🔲️ React drops a stale maximized path the moment the canvas falls back to one window
/// (`🎨️Canvas/🟦️.tsx:1802-1805`), and never offers maximize for a lone window at all.
#[test]
fn maximize_is_inert_and_self_clearing_for_a_single_window_canvas() {
    let mut single = dock_with(stack_tabs(&["a"], "a"), "a");
    single.toggle_maximize(&vec![]);
    assert_eq!(single.maximized_stack, None, "a lone window already fills the canvas");
    let mut two = dock_with(DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]), "a");
    two.toggle_maximize(&vec![1]);
    assert_eq!(two.maximized_stack, Some(vec![1]));
    assert!(two.close_window_in_stack(&vec![0], "a"));
    assert_eq!(two.maximized_stack, None, "the canvas is down to one window, so the maximized path is dropped");
}

/// ↔️ `applyAxisResizeDelta` conserves the dragged PAIR's total and floors each side at `minPct = 8`
/// (`🎨️Canvas/🟦️.tsx:593-611`).
#[test]
fn split_resize_conserves_the_pair_total_and_floors_at_eight_percent() {
    let mut dock = dock_with(DockNode::Row(vec![(stack_with("a"), 0.25), (stack_with("b"), 0.25), (stack_with("c"), 0.5)]), "a");
    dock.begin_split_drag(&vec![]);
    dock.apply_split_drag(&vec![], 0, 100.0, 1000.0);
    let DockNode::Row(children) = &dock.root else { panic!("row") };
    assert!((children[0].1 - 0.35).abs() < 1e-5);
    assert!((children[1].1 - 0.15).abs() < 1e-5);
    assert!((children[2].1 - 0.5).abs() < 1e-5, "the untouched sibling keeps its share");
    dock.begin_split_drag(&vec![]);
    dock.apply_split_drag(&vec![], 0, 10_000.0, 1000.0);
    let DockNode::Row(children) = &dock.root else { panic!("row") };
    assert!((children[0].1 + children[1].1 - 0.5).abs() < 1e-5, "the pair total survives the clamp");
    assert!((children[1].1 - SPLIT_MIN_FRACTION).abs() < 1e-5, "the squeezed side floors at React's 8 %");
    assert!((children[2].1 - 0.5).abs() < 1e-5, "and the clamp never restretches the rest of the axis");
}

/// 📏️ Authored window layouts retain React's percentage weights, so a physical pointer delta must
/// be converted into that axis's captured weight scale before the split is solved.
#[test]
fn split_resize_moves_the_separator_by_the_pointer_delta_on_react_percentage_weights() {
    let repo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../../../../../../..");
    let fixture_path = repo.join("🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/📐️dock-axis-geometry/🔣️.json");
    let fixture: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&fixture_path).unwrap_or_else(|error| panic!("read {}: {error}", fixture_path.display()))).expect("dock axis fixture");
    let oracle = &fixture["resizeOracle"];
    let axis_extent = oracle["axisExtentPixels"].as_f64().expect("axis extent") as f32;
    let delta = oracle["deltaPixels"].as_f64().expect("delta pixels") as f32;
    let before: Vec<f32> = oracle["beforeWeights"].as_array().expect("before weights").iter().map(|value| value.as_f64().expect("weight") as f32).collect();
    let after: Vec<f32> = oracle["afterWeights"].as_array().expect("after weights").iter().map(|value| value.as_f64().expect("weight") as f32).collect();
    let mut dock = dock_with(DockNode::Row(before.iter().enumerate().map(|(index, weight)| (stack_with(&format!("window-{index}")), *weight)).collect()), "window-0");
    let canvas = Rect::new(0.0, 0.0, axis_extent, 200.0);
    let before_frame = dock.stack_frame_rects_with_separator(canvas, 0.0)[0].1;
    let before_separator = before_frame.x + before_frame.w;
    let origin = dock.begin_split_drag(&vec![]);
    dock.apply_split_drag_with_origin(&vec![], 0, delta, axis_extent, &origin);
    let after_frame = dock.stack_frame_rects_with_separator(canvas, 0.0)[0].1;
    let after_separator = after_frame.x + after_frame.w;
    let DockNode::Row(children) = &dock.root else { panic!("row") };
    for ((_, actual), expected) in children.iter().zip(after) {
        assert!((*actual - expected).abs() < 0.0001, "weight {actual} != {expected}");
    }
    assert!((after_separator - before_separator - oracle["expectedSeparatorDeltaPixels"].as_f64().expect("separator delta") as f32).abs() < 0.001);
}

/// ↔️ Resize gutters: React's `Resizable` separator is a thin visual line with a fat grab zone; wgpu
/// pins the visual at 6 px and the hit at 20 px, centred on the seam, plus 10 px join-corner squares
/// mirroring `modeJoinCornerSpecsForSeparator` (`🎨️Canvas/🟦️.tsx:515-580`).
///
/// 🎯️ `hit_at` resolves the last COMPLETE frame's registry; a walk that has only just staged its
/// targets reads them back through `staged_hits` (`🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs:331-340`).
#[test]
fn split_resize_gutter_hit_is_twenty_pixels_centred_on_the_seam() {
    let dock = dock_with(even_layout(&["a".into(), "b".into()]), "a");
    let canvas = Rect::new(0.0, 0.0, 400.0, 300.0);
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut draw = DrawList::default();
    let labels = HashMap::new();
    let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &IconAtlas::default(), input: &mut input, theme: &theme, window_labels: &labels, window_icon_ids: &HashMap::new(), control_names: None };
    dock.register_resize_hits(&mut ctx, canvas);
    let hit = input.staged_hits().iter().find(|target| target.kind == HitKind::DockSplit).expect("split hit on the seam");
    assert_eq!(hit.control_id.as_deref(), Some("dock.split..0"));
    assert!((hit.rect.w - 20.0).abs() < 0.001, "20 px grab zone");
    assert!((hit.rect.x + hit.rect.w * 0.5 - 200.0).abs() < 0.001, "centred on the seam");
    assert!(hit.rect.contains(200.0, 150.0), "the seam itself is inside the grab zone");
    assert!(!hit.rect.contains(188.0, 150.0), "12 px off the seam is outside it");
}

/// 🎯️ A drag-to-split always previews and commits an exact 50 % split, on both the stack and the
/// root path — React's `splitWithWindow`/`splitRootWithWindow` build a two-child axis with no ratio
/// of their own (`🎨️Canvas/🟦️.tsx:403-421`), and the preview covers exactly half the body (`:778-796`).
#[test]
fn drag_to_split_commits_an_even_two_child_axis() {
    let mut dock = dock_with(stack_tabs(&["a", "b"], "a"), "a");
    assert!(dock.apply_drop(&tab_payload("a", vec![], 0), &DockDropZone::Split { stack_path: vec![], side: DockSide::Bottom }));
    assert_eq!(dock.root, DockNode::Column(vec![(stack_with("b"), 0.5), (stack_with("a"), 0.5)]));
    let mut dock = dock_with(stack_tabs(&["a", "b"], "a"), "a");
    assert!(dock.apply_drop(&tab_payload("a", vec![], 0), &DockDropZone::RootSplit { side: DockSide::Left }));
    assert_eq!(dock.root, DockNode::Row(vec![(stack_with("a"), 0.5), (stack_tabs(&["b"], "b"), 0.5)]));
}

/// 🎯️ A drag-to-merge lands the tab in the target stack's own corner group and focuses it — the
/// corner-local index maps onto the flat child list exactly like `flatIndexForCornerInsert`
/// (`🎨️Canvas/🟦️.tsx:335-347`).
#[test]
fn drag_to_merge_joins_the_target_corner_group_and_focuses_the_tab() {
    let mut dock = dock_with(DockNode::Row(vec![(stack_tabs(&["a", "x"], "a"), 0.5), (stack_with("b"), 0.5)]), "a");
    assert!(dock.apply_drop(&tab_payload("a", vec![0], 0), &DockDropZone::Tab { stack_path: vec![1], corner: WindowStackCorner::TopLeft, index: 0 }));
    let tabs = dock.stack_tabs_at_path(&vec![1]).expect("target stack");
    assert_eq!(dock_tab_ids(&tabs), vec!["a".to_string(), "b".to_string()]);
    assert_eq!(dock.active_window_id.as_deref(), Some("a"));
    assert_eq!(dock.active_stack, Some(vec![1]));
}

/// 🪟️ Tab-bar geometry stays in step with what `render_stack` paints: the per-corner drop widths a
/// drag hit-tests against are the SAME capped chip widths, so a drop index can never point between
/// two painted tabs.
#[test]
fn corner_tab_bar_widths_match_the_painted_chip_widths() {
    let dock = dock_with(stack_tabs(&["a", "b"], "a"), "a");
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let labels = HashMap::from([("a".to_string(), "A Window Whose Title Is Far Too Long To Fit Twelve Rem Of Tab".to_string()), ("b".to_string(), "B".to_string())]);
    let bars = dock.stack_corner_tab_bar_rects(Rect::new(0.0, 0.0, 600.0, 400.0), &theme, &mut atlas, &labels);
    let (_, _, _, widths) = bars.iter().find(|(_, corner, _, _)| *corner == WindowStackCorner::TopLeft).expect("top-left bar");
    let expected: Vec<f32> = ["a", "b"].iter().map(|id| dock_tab_chip(&mut atlas, &theme, labels.get(*id).unwrap(), dock.tab_action_count()).1).collect();
    assert_eq!(widths.len(), 2);
    for (got, want) in widths.iter().zip(expected.iter()) {
        assert!((got - want).abs() < 0.001, "{got} vs {want}");
    }
    assert!((widths[0] - MODE_DOCK_TAB_MAX_WIDTH_PX).abs() < 0.001, "the long title is capped, not grown");
}

// 🪟️ Packet W2d — the window-system remainder (`📓️w1i-window-dock-semantics.md` §4 G1-G8):
// `mobileFlatStack` (`🎨️Canvas/🟦️.tsx:1884-1901`), the committed-tree drag lane (`modeDockOutLayout`
// `:841-847`, `applyModeDrop` `:816-838`), click-to-deactivate (`:1455-1472`), the drag grip
// (`:1101`) and the silhouette focus border (`⚛️react/🟦️.tsx:7494-7509`).

/// 📱️ Below the breakpoint React renders ONE flat tab stack for the whole mode, through the same
/// `ModeDockStack` chrome as desktop — no split tree, no maximize, every window a tab
/// (`🎨️Canvas/🟦️.tsx:1884-1901`).
#[test]
fn mobile_collapses_every_window_into_one_flat_tab_stack() {
    let mut dock = dock_with(DockNode::Row(vec![(stack_tabs(&["a", "x"], "a"), 0.5), (DockNode::Column(vec![(stack_with("b"), 0.5), (stack_with("c"), 0.5)]), 0.5)]), "b");
    dock.toggle_maximize(&vec![0]);
    let desktop = dock.render_view(None);
    assert_eq!(desktop.root, dock.root, "above the breakpoint the render tree IS the committed tree");
    dock.mobile = true;
    let view = dock.render_view(None);
    assert_eq!(view.root, stack_tabs(&["a", "x", "b", "c"], "b"), "layout order, active window kept, one stack");
    assert_eq!(view.maximized_stack, None, "a mobile window already fills the canvas");
    assert_eq!(view.active_stack, Some(Vec::new()), "the flat stack IS the root");
    assert_eq!(dock.root, DockNode::Row(vec![(stack_tabs(&["a", "x"], "a"), 0.5), (DockNode::Column(vec![(stack_with("b"), 0.5), (stack_with("c"), 0.5)]), 0.5)]), "the committed tree is never flattened");
    let bodies = view.stack_body_rects(Rect::new(0.0, 0.0, 600.0, 900.0), &Theme::default(), &HashMap::new(), &mut FontAtlas::builtin());
    assert_eq!(bodies.len(), 1, "one pane, not four");
}

/// 📱️ A flat-stack tab is painted at the ROOT path, so focus has to travel by window ID — React's
/// `activateWindow(windowId)` (`🎨️Canvas/🟦️.tsx:1446-1452`), never by the path the chip sits at.
#[test]
fn activate_window_focuses_by_id_across_the_committed_tree() {
    let mut dock = dock_with(DockNode::Row(vec![(stack_with("a"), 0.5), (stack_tabs(&["b", "c"], "b"), 0.5)]), "a");
    assert!(dock.activate_window("c"));
    assert_eq!(dock.active_window_id.as_deref(), Some("c"));
    assert_eq!(dock.active_stack, Some(vec![1]), "the id resolved to its own stack, not to the root the chip was painted at");
    assert_eq!(dock.stack_tabs_at_path(&vec![1]).map(|tabs| dock_tab_ids(&tabs)), Some(vec!["b".to_string(), "c".to_string()]));
    assert!(!dock.activate_window("nothing-here"));
}

/// 🌫️ Pressing the canvas background or a gutter clears the active window and keeps the layout —
/// React's `deactivateActiveWindow` (`🎨️Canvas/🟦️.tsx:1455-1472`).
#[test]
fn deactivate_clears_focus_without_touching_the_layout() {
    let mut dock = dock_with(DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]), "a");
    let before = dock.root.clone();
    assert!(dock.deactivate_active_window());
    assert_eq!(dock.active_window_id, None);
    assert_eq!(dock.active_stack, None);
    assert_eq!(dock.root, before);
    assert!(!dock.deactivate_active_window(), "an already-unfocused mode has nothing to clear");
}

/// 🪟️ The drag lane now collapses on React's `collapseLayout`: an emptied slot is pruned AND a
/// single-child axis is hoisted, keeping the child's own ratio (`🎨️Canvas/🟦️.tsx:225-236`).
///
/// 🩸️ It used to run a prune-only rule, so the drag lane and the close lane disagreed about the tree
/// a removal leaves behind — the docked-out preview kept a one-child axis React had already hoisted,
/// and every drop-zone path inside it was one segment deeper than React's.
#[test]
fn the_drag_lane_hoists_a_single_child_axis_like_the_close_lane() {
    let mut dock = dock_with(DockNode::Row(vec![(DockNode::Column(vec![(stack_with("a"), 0.25), (stack_with("b"), 0.75)]), 0.4), (stack_with("c"), 0.6)]), "a");
    let payload = tab_payload("a", vec![0, 0], 0);
    let floating = dock.render_view(Some(&payload));
    assert_eq!(floating.root, DockNode::Row(vec![(stack_with("b"), 0.75), (stack_with("c"), 0.6)]), "b hoists into the column's slot with ITS OWN 0.75 ratio");
    let mut closed = dock.clone();
    assert!(closed.close_window("a"));
    assert_eq!(closed.root, floating.root, "one collapse rule for the drag lane and the close lane");
}

/// 🪟️ The active stack's outline is painted, not commented out: a hairline along the whole
/// silhouette in `--active-base` when the stack holds the active window, `--border-normal-color`
/// otherwise (`⚛️react/🟦️.tsx:7494-7509`).
#[test]
fn the_active_stack_paints_a_silhouette_focus_border() {
    let theme = Theme::default();
    let strokes = |dock: &DockState| {
        let mut atlas = FontAtlas::builtin();
        let icons = IconAtlas::default();
        let mut input = InputState::<ActionDescriptor>::default();
        let mut draw = DrawList::default();
        let labels = HashMap::from([("a".to_string(), "A".to_string()), ("b".to_string(), "B".to_string())]);
        let icon_ids = HashMap::new();
        let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &icons, input: &mut input, theme: &theme, window_labels: &labels, window_icon_ids: &icon_ids, control_names: None };
        dock.paint_chrome(&mut ctx, Rect::new(0.0, 0.0, 600.0, 400.0), false);
        draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| instance.rect[2] <= theme.stroke_hairline || instance.rect[3] <= theme.stroke_hairline).count()
    };
    let focused = dock_with(DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]), "a");
    assert!(strokes(&focused) > 0, "both stacks are outlined — the active one just wears a different colour");
    let mut unfocused = focused.clone();
    assert!(unfocused.deactivate_active_window());
    assert_eq!(strokes(&unfocused), strokes(&focused), "the outline is the window frame, not a focus-only decoration");
}

/// 🪶️ Closing a window is ONE lane: the layout collapses by React's rule, focus moves to the first
/// remaining window, the layout is persisted, and whatever the window hosted is torn down —
/// `closeWindow` + `onWindowClose` (`🎨️Canvas/🟦️.tsx:1475-1485`, `🏛️ShellHost/🟦️.tsx:10533-10555`).
#[test]
fn closing_a_window_collapses_persists_and_refocuses_in_one_lane() {
    let mut shell = shell();
    shell.dock.root = DockNode::Row(vec![(stack_with("plug-1"), 0.5), (stack_tabs(&["main", "side"], "main"), 0.5)]);
    shell.dock.sync_active_window("plug-1");
    shell.active_window_id = Some("plug-1".into());
    assert!(shell.close_dock_window("plug-1"), "close is never a no-operation");
    assert_eq!(shell.dock.root, stack_tabs(&["main", "side"], "main"), "the emptied slot is pruned and its sibling hoisted");
    assert_eq!(shell.active_window_id.as_deref(), Some("main"), "React refocuses children[0]");
    assert!(shell.layout_override.is_some(), "every close persists the collapsed layout");
    assert!(!shell.close_dock_window("plug-1"), "a window that is already gone refuses");
}

/// 🪶️ The spawned-app teardown React's `onWindowClose` performs: the closed window's entry leaves the
/// panel and `activeSpawnedId` re-points at whatever is left, so the plugin instance behind it can be
/// destroyed instead of running forever behind a window that no longer exists.
#[test]
fn closing_a_spawned_window_takes_its_panel_entry_and_repoints_the_active_one() {
    let entry = |id: &str, instance: u32| crate::shell::SpawnedAppEntry { id: id.into(), plugin_id: "plug".into(), instance_id: instance, app_id: "app".into(), label: "Plug".into(), breadcrumb: vec!["plug".into()] };
    let mut panel = crate::shell::SpacePanelState { active_panel_tab: "workbench".into(), spawned_apps: vec![entry("plug-1", 7), entry("plug-2", 8)], active_spawned_id: Some("plug-1".into()) };
    let closed = ShellState::take_spawned_entry(&mut panel, "plug-1").expect("the closed window owned a spawned app");
    assert_eq!(closed.instance_id, 7, "the instance id the guest must be told to destroy");
    assert_eq!(panel.spawned_apps.iter().map(|entry| entry.id.as_str()).collect::<Vec<_>>(), vec!["plug-2"]);
    assert_eq!(panel.active_spawned_id.as_deref(), Some("plug-2"));
    assert!(ShellState::take_spawned_entry(&mut panel, "main").is_none(), "a plain window hosts nothing to tear down");
    let last = ShellState::take_spawned_entry(&mut panel, "plug-2").expect("last spawned entry");
    assert_eq!(last.instance_id, 8);
    assert_eq!(panel.active_spawned_id, None);
}
//#endregion WindowSystemReactParityTests

/// 📥️ Tab insert midpoints are measured on the painted (gapless) chip run — React hit-tests real tab
/// rects (`computeModeDropZone`, `🎨️Canvas/🟦️.tsx:797`), so a phantom gap must not shift the index.
#[test]
fn tab_insert_index_follows_the_painted_gapless_chip_run() {
    let bar = Rect::new(0.0, 0.0, 200.0, 24.0);
    let widths = vec![80.0, 80.0];
    assert_eq!(compute_tab_insert_index(10.0, bar, &widths, 0.0), 0);
    assert_eq!(compute_tab_insert_index(45.0, bar, &widths, 0.0), 1);
    assert_eq!(compute_tab_insert_index(125.0, bar, &widths, 0.0), 2, "past the last tab's midpoint the drop appends");
    let tab_bars = vec![(vec![0], WindowStackCorner::TopLeft, bar, widths)];
    let bodies = vec![(vec![0], Rect::new(0.0, 24.0, 200.0, 200.0), "a".to_string())];
    assert_eq!(compute_dock_drop_zone(125.0, 10.0, &tab_bars, &bodies, Rect::new(0.0, 0.0, 200.0, 224.0)), Some(DockDropZone::Tab { stack_path: vec![0], corner: WindowStackCorner::TopLeft, index: 2 }));
}

/// 📐️ The shared React geometry fixture is the dock's physical axis law: themed separators consume
/// extent before weights, nested axes use the same solver, bodies add horizontal WindowChrome
/// padding, and resize targets stay centred on the reserved separator while clipped to the canvas.
#[test]
fn themed_axis_geometry_matches_the_shared_react_fixture() {
    let repo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../../../../../../..");
    let fixture_path = repo.join("🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/📐️dock-axis-geometry/🔣️.json");
    let fixture: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&fixture_path).unwrap_or_else(|error| panic!("read {}: {error}", fixture_path.display()))).expect("dock axis fixture");
    let token = fixture["oracleSample"]["tokenPixels"].as_f64().expect("token pixels") as f32;
    let width = fixture["viewport"]["width"].as_f64().expect("viewport width") as f32;
    let height = fixture["viewport"]["height"].as_f64().expect("viewport height") as f32;
    let canvas = Rect::new(token, token, width - token * 2.0, height - token * 2.0);
    let stack = |id: &str| DockNode::Stack { windows: vec![DockStackTab::new(id)], active: id.to_string() };
    let dock = DockState {
        root: DockNode::Row(vec![(stack("left"), 35.0), (DockNode::Column(vec![(stack("right-top"), 70.0), (stack("right-bottom"), 30.0)]), 65.0)]),
        active_window_id: Some("left".into()),
        active_stack: Some(vec![0]),
        ..DockState::default()
    };
    let frames = dock.stack_frame_rects_with_separator(canvas, token);
    assert_eq!(frames.len(), 3);
    for (_, rect, id) in &frames {
        let expected = &fixture["oracleSample"]["stacks"][id];
        for (actual, key) in [(rect.x, "x"), (rect.y, "y"), (rect.w, "width"), (rect.h, "height")] {
            let wanted = expected[key].as_f64().unwrap_or_else(|| panic!("{id}.{key}")) as f32;
            assert!((actual - wanted).abs() < 0.001, "📐️ {id}.{key}: {actual} != {wanted}");
        }
    }
    assert!((dock.split_axis_extent(&Vec::new(), canvas, token).expect("root row") - (canvas.w - token)).abs() < 0.001);
    assert!((dock.split_axis_extent(&vec![1], canvas, token).expect("nested column") - (canvas.h - token)).abs() < 0.001);

    let theme = Theme::light();
    assert!((theme.gap_standard - token).abs() < 0.001, "📐️ the neutral sample is the default live spacing token");
    let mut atlas = FontAtlas::builtin();
    let labels = HashMap::new();
    let bodies = dock.stack_body_rects(canvas, &theme, &labels, &mut atlas);
    for (_, body, id) in &bodies {
        let frame = frames.iter().find(|(_, _, frame_id)| frame_id == id).expect("matching frame").1;
        assert!((body.x - frame.x - token).abs() < 0.001, "📐️ {id} begins after one horizontal body inset");
        assert!((body.w - (frame.w - token * 2.0)).abs() < 0.001, "📐️ {id} removes both horizontal body insets");
    }

    let mut draw = DrawList::default();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let icon_ids = HashMap::new();
    let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &icons, input: &mut input, theme: &theme, window_labels: &labels, window_icon_ids: &icon_ids, control_names: None };
    dock.register_resize_hits(&mut ctx, canvas);
    let split_hits = input.staged_hits().iter().filter(|hit| hit.kind == HitKind::DockSplit).collect::<Vec<_>>();
    assert_eq!(split_hits.len(), 2, "📐️ root and nested axes each publish one resize target");
    for hit in split_hits {
        assert!(
            hit.rect.x >= canvas.x && hit.rect.y >= canvas.y && hit.rect.x + hit.rect.w <= canvas.x + canvas.w + 0.001 && hit.rect.y + hit.rect.h <= canvas.y + canvas.h + 0.001,
            "📐️ physical hit rect is clipped to its solved canvas: {:?}",
            hit.rect
        );
    }

    let single = DockState { root: stack("single"), ..DockState::default() };
    assert_eq!(single.stack_frame_rects_with_separator(canvas, token)[0].1, canvas, "📐️ a single stack reserves no separator");
}
