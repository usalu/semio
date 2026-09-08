
use super::*;

#[test]
fn rect_exports_svg() {
    let mut store = DrawingStore::new();
    let rect = store.rect(0.0, 0.0, 10.0, 20.0).expect("rect");
    let svg = store.export_svg_sync(&rect).expect("svg");
    assert!(svg.contains("<svg"));
    assert!(svg.contains("10"));
}

#[test]
fn rect_exports_pdf() {
    let mut store = DrawingStore::new();
    let rect = store.rect(0.0, 0.0, 10.0, 20.0).expect("rect");
    let pdf = store.export_pdf_sync(&rect).expect("pdf");
    assert!(pdf.starts_with(b"%PDF"));
}

#[test]
fn group_flattens_children() {
    let mut store = DrawingStore::new();
    let a = store.rect(0.0, 0.0, 5.0, 5.0).unwrap();
    let b = store.circle(10.0, 10.0, 3.0).unwrap();
    let group = store.group(&[a, b]).unwrap();
    let scene = store.flatten_scene_sync(&group).unwrap();
    assert_eq!(scene.nodes.len(), 2);
}

// #region Geometry primitives export
#[test]
fn ellipse_exports_svg_with_cubic_curves() {
    let mut store = DrawingStore::new();
    let ellipse = store.ellipse(5.0, 5.0, 4.0, 2.0).unwrap();
    let svg = store.export_svg_sync(&ellipse).expect("svg");
    assert!(svg.contains("C "));
    assert!(svg.contains("Z"));
}

#[test]
fn line_exports_svg_move_and_line() {
    let mut store = DrawingStore::new();
    let line = store.line(0.0, 0.0, 10.0, 10.0).unwrap();
    let svg = store.export_svg_sync(&line).expect("svg");
    assert!(svg.contains("M 0 0"));
    assert!(svg.contains("L 10 10"));
}

#[test]
fn polygon_exports_closed_svg_path() {
    let mut store = DrawingStore::new();
    let polygon = store.polygon(&[[0.0, 0.0], [4.0, 0.0], [2.0, 4.0]]).unwrap();
    let svg = store.export_svg_sync(&polygon).expect("svg");
    assert!(svg.contains("Z"));
}

#[test]
fn polyline_path_exports_open_path_without_close() {
    let mut store = DrawingStore::new();
    let polyline = store.polyline_path(&[[0.0, 0.0], [4.0, 0.0], [2.0, 4.0]]).unwrap();
    let svg = store.export_svg_sync(&polyline).expect("svg");
    assert!(!svg.contains("Z"));
}

#[test]
fn polygon_errors_on_too_few_points() {
    let mut store = DrawingStore::new();
    let err = store.polygon(&[[0.0, 0.0], [1.0, 1.0]]).unwrap_err();
    assert!(matches!(err, semio_framework_2d::DrawingError::InvalidInput(_)));
}

#[test]
fn polyline_path_errors_on_too_few_points() {
    let mut store = DrawingStore::new();
    let err = store.polyline_path(&[[0.0, 0.0]]).unwrap_err();
    assert!(matches!(err, semio_framework_2d::DrawingError::InvalidInput(_)));
}
// #endregion Geometry primitives export

// #region Style
#[test]
fn set_fill_solid_renders_opaque_hex_color() {
    let mut store = DrawingStore::new();
    let rect = store.rect(0.0, 0.0, 5.0, 5.0).unwrap();
    let filled = store.set_fill(&rect, FillStyle::Solid { color: [1.0, 0.0, 0.0, 1.0] }).unwrap();
    let svg = store.export_svg_sync(&filled).expect("svg");
    assert!(svg.contains(r##"fill="#ff0000""##));
}

#[test]
fn set_fill_with_alpha_renders_rgba() {
    let mut store = DrawingStore::new();
    let rect = store.rect(0.0, 0.0, 5.0, 5.0).unwrap();
    let filled = store.set_fill(&rect, FillStyle::Solid { color: [0.0, 1.0, 0.0, 0.5] }).unwrap();
    let svg = store.export_svg_sync(&filled).expect("svg");
    assert!(svg.contains("rgba(0,255,0,0.500)"));
}

#[test]
fn linear_gradient_fill_renders_gradient_defs() {
    let mut store = DrawingStore::new();
    let rect = store.rect(0.0, 0.0, 5.0, 5.0).unwrap();
    let stops = vec![GradientStop { offset: 0.0, color: [1.0, 1.0, 1.0, 1.0] }, GradientStop { offset: 1.0, color: [0.0, 0.0, 0.0, 1.0] }];
    let filled = store.linear_gradient_fill(&rect, 0.0, 0.0, 5.0, 5.0, &stops).unwrap();
    let svg = store.export_svg_sync(&filled).expect("svg");
    assert!(svg.contains("<linearGradient"));
    assert!(svg.contains("fill=\"url(#lg"));
}

#[test]
fn set_fill_radial_gradient_renders_defs() {
    let mut store = DrawingStore::new();
    let rect = store.rect(0.0, 0.0, 5.0, 5.0).unwrap();
    let stops = vec![GradientStop { offset: 0.0, color: [1.0, 0.0, 0.0, 1.0] }];
    let fill = FillStyle::RadialGradient { cx: 2.5, cy: 2.5, r: 2.0, stops };
    let filled = store.set_fill(&rect, fill).unwrap();
    let svg = store.export_svg_sync(&filled).expect("svg");
    assert!(svg.contains("<radialGradient"));
    assert!(svg.contains("fill=\"url(#rg"));
}

#[test]
fn set_stroke_renders_stroke_attributes() {
    let mut store = DrawingStore::new();
    let rect = store.rect(0.0, 0.0, 5.0, 5.0).unwrap();
    let stroke = StrokeStyle { color: [0.0, 0.0, 1.0, 1.0], width: 2.0, cap: LineCap::Round, join: LineJoin::Round, dash: vec![] };
    let stroked = store.set_stroke(&rect, stroke).unwrap();
    let svg = store.export_svg_sync(&stroked).expect("svg");
    assert!(svg.contains(r##"stroke="#0000ff""##));
    assert!(svg.contains(r#"stroke-width="2""#));
}
// #endregion Style

// #region Transforms
#[test]
fn translate_moves_exported_geometry() {
    let mut store = DrawingStore::new();
    let rect = store.rect(0.0, 0.0, 5.0, 5.0).unwrap();
    let moved = store.translate(&rect, 10.0, 20.0).unwrap();
    let scene = store.flatten_scene_sync(&moved).unwrap();
    assert_eq!(scene.nodes[0].transform.transform_point([0.0, 0.0]), [10.0, 20.0]);
}

#[test]
fn rotate_and_scale_compose_into_transform() {
    let mut store = DrawingStore::new();
    let rect = store.rect(0.0, 0.0, 5.0, 5.0).unwrap();
    let rotated = store.rotate(&rect, std::f64::consts::FRAC_PI_2).unwrap();
    let scaled = store.scale(&rotated, 2.0, 2.0).unwrap();
    let scene = store.flatten_scene_sync(&scaled).unwrap();
    let [x, y] = scene.nodes[0].transform.transform_point([1.0, 0.0]);
    assert!((x - 0.0).abs() < 1e-9);
    assert!((y - 2.0).abs() < 1e-9);
}
// #endregion Transforms

// #region Group and clip
#[test]
fn group_errors_on_empty_children() {
    let mut store = DrawingStore::new();
    let err = store.group(&[]).unwrap_err();
    assert!(matches!(err, semio_framework_2d::DrawingError::InvalidInput(_)));
}

#[test]
fn apply_clip_stores_clip_segments_on_flatten() {
    let mut store = DrawingStore::new();
    let rect = store.rect(0.0, 0.0, 5.0, 5.0).unwrap();
    let circle = store.circle(2.0, 2.0, 1.0).unwrap();
    let clipped = store.apply_clip(&rect, &circle).unwrap();
    let scene = store.flatten_scene_sync(&clipped).unwrap();
    assert!(scene.nodes[0].clip.as_ref().is_some_and(|segments| !segments.is_empty()));
}
// #endregion Group and clip

// #region Text
#[test]
fn text_with_fill_renders_colored_text_element() {
    let mut store = DrawingStore::new();
    let text = store.text(1.0, 2.0, "hi", 12.0).unwrap();
    let colored = store.set_fill(&text, FillStyle::Solid { color: [0.0, 0.0, 1.0, 1.0] }).unwrap();
    let svg = store.export_svg_sync(&colored).expect("svg");
    assert!(svg.contains(r##"<text x="1" y="2" font-size="12" fill="#0000ff">hi</text>"##));
}

#[test]
fn text_without_fill_defaults_to_black() {
    let mut store = DrawingStore::new();
    let text = store.text(0.0, 0.0, "plain", 10.0).unwrap();
    let svg = store.export_svg_sync(&text).expect("svg");
    assert!(svg.contains(r#"fill="black">plain"#));
}

#[test]
fn text_with_gradient_fill_falls_back_to_black_in_svg() {
    let mut store = DrawingStore::new();
    let text = store.text(0.0, 0.0, "grad", 10.0).unwrap();
    let stops = vec![GradientStop { offset: 0.0, color: [1.0, 1.0, 1.0, 1.0] }];
    let gradient = store.linear_gradient_fill(&text, 0.0, 0.0, 1.0, 1.0, &stops).unwrap();
    let svg = store.export_svg_sync(&gradient).expect("svg");
    assert!(svg.contains(r#"fill="black">grad"#));
}
// #endregion Text

// #region Boolean operations via kernel trait
#[test]
fn bool_union_via_kernel_trait() {
    let mut store = DrawingStore::new();
    let a = store.rect_path(0.0, 0.0, 10.0, 10.0).unwrap();
    let b = store.rect_path(5.0, 5.0, 10.0, 10.0).unwrap();
    let merged = store.bool_union(&a, &b).unwrap();
    assert_eq!(store.kind(&merged).unwrap(), DrawingKind::Path);
}

#[test]
fn bool_difference_via_kernel_trait() {
    let mut store = DrawingStore::new();
    let a = store.rect_path(0.0, 0.0, 10.0, 10.0).unwrap();
    let b = store.rect_path(5.0, 5.0, 10.0, 10.0).unwrap();
    let diff = store.bool_difference(&a, &b).unwrap();
    let scene = store.flatten_scene_sync(&diff).unwrap();
    assert!(!scene.nodes.is_empty());
}

#[test]
fn bool_intersection_via_kernel_trait() {
    let mut store = DrawingStore::new();
    let a = store.rect_path(0.0, 0.0, 10.0, 10.0).unwrap();
    let b = store.rect_path(5.0, 5.0, 10.0, 10.0).unwrap();
    let intersection = store.bool_intersection(&a, &b).unwrap();
    let scene = store.flatten_scene_sync(&intersection).unwrap();
    assert!(!scene.nodes.is_empty());
}

#[test]
fn bool_xor_via_kernel_trait() {
    let mut store = DrawingStore::new();
    let a = store.rect_path(0.0, 0.0, 10.0, 10.0).unwrap();
    let b = store.rect_path(5.0, 5.0, 10.0, 10.0).unwrap();
    let xor = store.bool_xor(&a, &b).unwrap();
    let scene = store.flatten_scene_sync(&xor).unwrap();
    assert!(!scene.nodes.is_empty());
}

#[test]
fn bool_op_many_single_handle_is_content_addressed() {
    let mut store = DrawingStore::new();
    let rect = store.rect(0.0, 0.0, 5.0, 5.0).unwrap();
    let forked = store.bool_op_many("union", std::slice::from_ref(&rect)).unwrap();
    assert_eq!(forked.as_str(), rect.as_str());
    assert_eq!(store.kind(&forked).unwrap(), DrawingKind::Rect);
}

#[test]
fn bool_op_many_merges_multiple_handles() {
    let mut store = DrawingStore::new();
    let a = store.rect_path(0.0, 0.0, 10.0, 10.0).unwrap();
    let b = store.rect_path(5.0, 0.0, 10.0, 10.0).unwrap();
    let c = store.rect_path(0.0, 5.0, 10.0, 10.0).unwrap();
    let merged = store.bool_op_many("union", &[a, b, c]).unwrap();
    assert_eq!(store.kind(&merged).unwrap(), DrawingKind::Path);
}

#[test]
fn bool_op_many_errors_on_empty_handles() {
    let mut store = DrawingStore::new();
    let err = store.bool_op_many("union", &[]).unwrap_err();
    assert!(matches!(err, semio_framework_2d::DrawingError::InvalidInput(_)));
}

#[test]
fn boolean_segments_trait_delegates_to_booleans_module() {
    let store = DrawingStore::new();
    let a = rect_segments(0.0, 0.0, 10.0, 10.0);
    let b = rect_segments(5.0, 5.0, 10.0, 10.0);
    let merged = store.boolean_segments(&a, &b, "union").expect("union");
    assert!(!merged.is_empty());
}
// #endregion Boolean operations via kernel trait

// #region Trace via kernel trait
#[test]
fn trace_bitmap_trait_delegates_to_trace_module() {
    let mut store = DrawingStore::new();
    let width = 6_u32;
    let height = 6_u32;
    let mut mask = vec![0_u8; (width * height) as usize];
    for y in 1..5 {
        for x in 1..5 {
            mask[(y * width + x) as usize] = 255;
        }
    }
    let traced = store.trace_bitmap(width, height, &mask, 0.5, 0.5).unwrap();
    assert_eq!(store.kind(&traced).unwrap(), DrawingKind::Path);
}
// #endregion Trace via kernel trait

// #region Registry lifecycle
#[test]
fn registry_len_tracks_inserted_handles() {
    let mut store = DrawingStore::new();
    assert_eq!(store.registry_len(), 0);
    let rect = store.rect(0.0, 0.0, 5.0, 5.0).unwrap();
    assert_eq!(store.registry_len(), 1);
    store.set_fill(&rect, FillStyle::Solid { color: [1.0, 1.0, 1.0, 1.0] }).unwrap();
    assert_eq!(store.registry_len(), 2);
}

#[test]
fn dispose_sync_removes_handle_from_registry() {
    let mut store = DrawingStore::new();
    let rect = store.rect(0.0, 0.0, 5.0, 5.0).unwrap();
    store.dispose_sync(&rect);
    assert_eq!(store.registry_len(), 0);
    let err = store.kind(&rect).unwrap_err();
    assert!(matches!(err, semio_framework_2d::DrawingError::MissingHandle(_)));
}

#[test]
fn retain_sync_keeps_only_live_handles() {
    let mut store = DrawingStore::new();
    let a = store.rect(0.0, 0.0, 5.0, 5.0).unwrap();
    let b = store.circle(1.0, 1.0, 1.0).unwrap();
    let live: HashSet<String> = [a.as_str().to_string()].into_iter().collect();
    store.retain_sync(&live);
    assert!(store.kind(&a).is_ok());
    assert!(store.kind(&b).is_err());
}

#[test]
fn missing_handle_errors_on_set_fill_and_translate() {
    let mut store = DrawingStore::new();
    let bogus = DrawingHandle("not-valid-hex".to_string());
    let fill_err = store.set_fill(&bogus, FillStyle::Solid { color: [0.0, 0.0, 0.0, 1.0] }).unwrap_err();
    assert!(matches!(fill_err, semio_framework_2d::DrawingError::MissingHandle(_)));
    let translate_err = store.translate(&bogus, 1.0, 1.0).unwrap_err();
    assert!(matches!(translate_err, semio_framework_2d::DrawingError::MissingHandle(_)));
}

#[test]
fn flatten_scene_errors_on_missing_handle() {
    let store = DrawingStore::new();
    let bogus = DrawingHandle("not-valid-hex".to_string());
    let err = store.flatten_scene(&bogus).unwrap_err();
    assert!(matches!(err, semio_framework_2d::DrawingError::MissingHandle(_)));
}
// #endregion Registry lifecycle

// #region Scene bounds
#[test]
fn scene_bounds_grows_to_fit_text_and_shapes() {
    let mut store = DrawingStore::new();
    let rect = store.rect(600.0, 0.0, 10.0, 10.0).unwrap();
    let text = store.text(0.0, 700.0, "wide label", 20.0).unwrap();
    let group = store.group(&[rect, text]).unwrap();
    let scene = store.flatten_scene_sync(&group).unwrap();
    assert!(scene.width >= 610.0);
    assert!(scene.height >= 720.0);
}
// #endregion Scene bounds

// #region Engine derive
#[test]
fn drawing_engine_id_matches_os_contract() {
    assert_eq!(DrawingEngine::ENGINE_ID, "s.2d.drawing");
}

#[test]
fn derive_twice_same_node_is_same_handle() {
    let mut store = DrawingStore::new();
    let first = store.rect(1.0, 2.0, 3.0, 4.0).unwrap();
    let second = store.rect(1.0, 2.0, 3.0, 4.0).unwrap();
    assert_eq!(first.as_str(), second.as_str());
    assert_eq!(store.registry_len(), 2);
}

#[test]
fn handles_are_hex_engine_keys() {
    let mut store = DrawingStore::new();
    let rect = store.rect(0.0, 0.0, 1.0, 1.0).unwrap();
    assert_eq!(rect.as_str().len(), 64);
    assert!(rect.as_str().chars().all(|ch| ch.is_ascii_hexdigit()));
}
// #endregion Engine derive

/// 📐️ Migrated from `🧰️framework/🔨️modules/◻️2d/⚙️engine/🦀️.rs` alongside `Affine2D`
/// itself (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS).
#[test]
fn affine_multiplies_identity() {
    let point = [3.0, 4.0];
    let moved = Affine2D::translate(1.0, 2.0).multiply(Affine2D::identity()).transform_point(point);
    assert_eq!(moved, [4.0, 6.0]);
}
