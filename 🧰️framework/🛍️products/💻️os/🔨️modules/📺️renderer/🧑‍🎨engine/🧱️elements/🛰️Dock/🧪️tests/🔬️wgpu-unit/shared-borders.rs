#[test]
fn shared_border_segments_match_language_neutral_contract() {
    use shared_borders::{resolve_border_segments, BorderPoint, BorderSegment, BorderState};
    fn segment(value: &serde_json::Value) -> BorderSegment {
        let point = |name: &str| BorderPoint { x: value[name]["x"].as_f64().unwrap(), y: value[name]["y"].as_f64().unwrap() };
        BorderSegment { id: value["id"].as_str().unwrap().into(), owner: value["owner"].as_str().unwrap().into(), from: point("from"), to: point("to"), state: BorderState::parse(value["state"].as_str().unwrap()).unwrap() }
    }
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().find(|path| path.join("🧰️framework/🔨️modules/🖱️ui").is_dir()).unwrap();
    let cases: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(root.join("🧰️framework/🔨️modules/🖱️ui/🔨️modules/📏️border-presentation/cases.json")).unwrap()).unwrap();
    for case in cases.as_array().unwrap() {
        let mut input = case["input"].as_array().unwrap().iter().map(segment).collect::<Vec<_>>();
        let expected = case["expected"].as_array().unwrap();
        for _ in 0..2 {
            let actual = resolve_border_segments(&input).unwrap();
            assert_eq!(actual.len(), expected.len(), "{}", case["name"]);
            for (actual, expected) in actual.iter().zip(expected) {
                assert_eq!(actual.segment, segment(expected));
                assert_eq!(actual.contributors, expected["contributors"].as_array().unwrap().iter().map(|id| id.as_str().unwrap().to_string()).collect::<Vec<_>>());
            }
            input.reverse();
        }
    }
}

#[test]
fn frame_border_is_the_outline_plus_the_chips_inner_edges_all_inward() {
    let frame = WindowFrame::from_measured_top(Rect::new(0.0, 0.0, 200.0, 100.0), 60.0, 40.0, 24.0);
    let edges = window_frame_border_edges(&frame);
    assert_eq!(edges.len(), 4 + 2 + 1 + 1, "outline, two chip baselines, the first chip's right edge and the second chip's left edge");
    assert!(edges.iter().any(|edge| edge.from.y == 0.0 && edge.to.y == 0.0 && edge.from.x == 0.0 && edge.to.x == 200.0), "the top outline is one line across the chip row");
    assert!(edges.iter().any(|edge| edge.from.x == 0.0 && edge.to.x == 60.0 && edge.from.y == 24.0 && edge.to.y == 24.0), "left chip baseline");
    assert!(edges.iter().any(|edge| edge.from.x == 160.0 && edge.to.x == 200.0 && edge.from.y == 24.0), "right chip baseline");
    assert!(!edges.iter().any(|edge| edge.from.x == 60.0 && edge.to.x == 160.0 && edge.from.y == 24.0), "nothing closes the span between the chips");
    let bottom = WindowFrame::new(frame.bounds, WindowFrameEdge::default(), frame.top.clone());
    for shape in [&frame, &bottom] {
        for edge in window_frame_border_edges(shape) {
            let [x, y, w, h] = edge.paint_rect(1.0);
            assert!(shape.contains(x + w * 0.5, y + h * 0.5), "paint must remain inside the frame");
        }
    }
    let plain = WindowFrame::new(Rect::new(10.0, 20.0, 200.0, 100.0), WindowFrameEdge::default(), WindowFrameEdge::default());
    let edges = window_frame_border_edges(&plain);
    assert_eq!(edges.len(), 4);
    for edge in edges {
        let [x, y, w, h] = edge.paint_rect(1.0);
        assert!(x >= 10.0 && y >= 20.0 && x + w <= 210.0 && y + h <= 120.0);
    }
}

#[test]
fn shared_dock_frames_abut_and_resize_hit_stays_centered() {
    let mut dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
    dock.root = DockNode::Row(vec![(stack_with("a"), 1.0), (stack_with("b"), 1.0)]);
    let bounds = Rect::new(0.0, 0.0, 800.0, 600.0);
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let (bodies, frames) = dock.stack_body_rects_with_frames(bounds, &theme, &HashMap::new(), &mut atlas);
    assert_eq!(frames["a"].bounds.x + frames["a"].bounds.w, frames["b"].bounds.x);
    assert_eq!(bodies[1].1.x, bodies[0].1.x + bodies[0].1.w, "content planes abut, no body inset");
    let mut input = InputState::<ActionDescriptor>::default();
    let mut draw = DrawList::default();
    let labels = HashMap::new();
    let icons = IconAtlas::default();
    let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &icons, input: &mut input, theme: &theme, window_labels: &labels, window_icon_ids: &labels, control_names: None };
    dock.register_resize_hits(&mut ctx, bounds);
    let hit = input.staged_hits().iter().find(|hit| matches!(hit.kind, HitKind::DockSplit)).unwrap();
    assert_eq!(hit.rect.w, 20.0);
    assert_eq!(hit.rect.x + hit.rect.w * 0.5, 400.0);
    assert_eq!(dock.split_axis_extent(&Vec::new(), bounds, 0.0), Some(800.0));
}

/// 🖐️ React's `RESIZABLE_HIT_TARGET_MIN_FINE_PX`/`_COARSE_PX` read from the fixture its own Canvas test pins them to: a touch-class dock registers the 28 px splitter, a fine one the 20 px.
#[test]
fn the_splitter_hit_is_react_s_fine_target_and_its_coarse_target_on_a_touch_dock() {
    let fixture: serde_json::Value = serde_json::from_str(repo_text!("🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🧪️tests/dock-cases.json")).expect("shared dock fixture");
    let (fine, coarse) = (fixture["fineHitSize"].as_f64().unwrap() as f32, fixture["coarseHitSize"].as_f64().unwrap() as f32);
    assert_eq!((split_hit_extent(false), split_hit_extent(true)), (fine, coarse));
    let bounds = Rect::new(0.0, 0.0, 800.0, 600.0);
    let theme = Theme::default();
    let width_of = |coarse_pointer: bool| {
        let mut dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
        dock.root = DockNode::Row(vec![(stack_with("a"), 1.0), (stack_with("b"), 1.0)]);
        dock.coarse_pointer = coarse_pointer;
        let mut atlas = FontAtlas::builtin();
        let mut input = InputState::<ActionDescriptor>::default();
        let mut draw = DrawList::default();
        let labels = HashMap::new();
        let icons = IconAtlas::default();
        let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &icons, input: &mut input, theme: &theme, window_labels: &labels, window_icon_ids: &labels, control_names: None };
        dock.register_resize_hits(&mut ctx, bounds);
        input.staged_hits().iter().find(|hit| matches!(hit.kind, HitKind::DockSplit)).unwrap().rect.w
    };
    assert_eq!((width_of(false), width_of(true)), (fine, coarse));
}

#[test]
fn shared_dock_seam_is_one_hover_stroke_next_to_active_window() {
    let mut dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
    dock.root = DockNode::Row(vec![(stack_with("a"), 1.0), (stack_with("b"), 1.0)]);
    dock.active_stack = Some(vec![0]);
    let bounds = Rect::new(0.0, 0.0, 800.0, 600.0);
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let mut input = InputState::<ActionDescriptor>::default();
    input.pointer_x = 450.0;
    input.pointer_y = 300.0;
    let mut draw = DrawList::default();
    let labels = HashMap::new();
    let icons = IconAtlas::default();
    let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &icons, input: &mut input, theme: &theme, window_labels: &labels, window_icon_ids: &labels, control_names: None };
    dock.paint_chrome(&mut ctx, bounds, false);
    let seam = draw.layers.iter().flat_map(|layer| &layer.ui_instances).filter(|instance| {
        instance.rect[2] == theme.stroke_hairline && instance.rect[0] >= 400.0 - theme.stroke_hairline && instance.rect[0] <= 400.0 && instance.rect[1] <= 300.0 && instance.rect[1] + instance.rect[3] > 300.0
    }).collect::<Vec<_>>();
    assert_eq!(seam.len(), 1);
    let color = theme.border_emphasized;
    assert_eq!(seam[0].color, [color.r, color.g, color.b, color.a]);
    assert!(draw.layers.iter().flat_map(|layer| &layer.ui_instances).any(|instance| instance.rect[0] == 0.0 && instance.rect[2] == theme.stroke_hairline && instance.rect[1] + instance.rect[3] > 300.0));
}
