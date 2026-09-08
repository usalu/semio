
use super::*;

#[test]
fn parse_minimal_document() {
    let json = r#"{"schema":"raster.document","id":"t","camera":{"x":0,"y":0,"zoom":1},"layers":[]}"#;
    let doc = parse_document(json).expect("parse");
    assert!(doc.layers.is_empty());
}

#[test]
fn parse_play_fixtures() {
    // 🩹️ Was `include_str!` of raster's example fixture; raster migrated that fixture to a
    // handcrafted DSL (`store::ArtifactDsl`), which this JSON-only surface parser doesn't read.
    // Inlined an equivalent layered document so this test still exercises multi-layer parsing.
    let json = r#"{"schema":"raster.document","id":"semio","camera":{"x":0,"y":0,"zoom":1},"layers":[
            {"kind":"adjustment","id":"a","name":"Bright","visible":true,"opacity":1,"blendMode":"normal","transform":{"x":0,"y":0,"scaleX":1,"scaleY":1,"rotation":0},"adjustmentKind":"brightnessContrast"}
        ]}"#;
    let doc = parse_document(json).expect("parse semio fixture");
    assert!(!doc.layers.is_empty(), "semio should have layers");
}

#[test]
fn parse_adjustment_without_params() {
    let json = r#"{"schema":"raster.document","id":"t","layers":[
            {"kind":"adjustment","id":"a","name":"Bright","visible":true,"opacity":1,"blendMode":"normal","transform":{"x":0,"y":0,"scaleX":1,"scaleY":1,"rotation":0},"adjustmentKind":"brightnessContrast"}
        ]}"#;
    let doc = parse_document(json).expect("adjustment params must default");
    assert_eq!(doc.layers.len(), 1);
}

#[test]
fn group_child_world_transform_uses_parent_once() {
    let json = r#"{"schema":"raster.document","id":"t","layers":[
            {"kind":"group","id":"g","name":"G","visible":true,"opacity":1,"blendMode":"normal","transform":{"x":100,"y":0,"scaleX":1,"scaleY":1,"rotation":0},"children":[
                {"kind":"pixel","id":"p","name":"P","visible":true,"opacity":1,"blendMode":"normal","transform":{"x":0,"y":0,"scaleX":1,"scaleY":1,"rotation":0},"width":50,"height":50}
            ]}
        ]}"#;
    let mut host = RasterHost::new();
    host.set_size(400, 400, 1.0);
    host.set_camera(0.0, 0.0, 1.0);
    host.sync_document_json(json).expect("sync");
    let hits: Vec<PickTargetJson> = serde_json::from_str(&host.pick_targets_at_screen_json(300.0, 200.0)).expect("json");
    assert_eq!(hits.first().map(|h| h.id.as_str()), Some("p"), "group translate(100) + camera center should place child at screen x≈300");
}

fn two_pixel_layer_host() -> RasterHost {
    let json = r#"{"schema":"raster.document","id":"t","camera":{"x":0,"y":0,"zoom":1},"layers":[
            {"kind":"pixel","id":"back","name":"Back","visible":true,"opacity":1,"blendMode":"normal","transform":{"x":0,"y":0,"scaleX":1,"scaleY":1,"rotation":0},"width":100,"height":100},
            {"kind":"pixel","id":"front","name":"Front","visible":true,"opacity":1,"blendMode":"normal","transform":{"x":10,"y":0,"scaleX":1,"scaleY":1,"rotation":0},"width":100,"height":100}
        ]}"#;
    let mut host = RasterHost::new();
    host.set_size(400, 400, 1.0);
    host.sync_document_json(json).expect("sync");
    host
}

#[test]
fn pick_targets_topmost_first() {
    let host = two_pixel_layer_host();
    let hits: Vec<PickTargetJson> = serde_json::from_str(&host.pick_targets_at_screen_json(200.0, 200.0)).expect("json");
    assert_eq!(hits.first().map(|h| h.id.as_str()), Some("front"), "later document layer is topmost");
    assert!(hits.iter().any(|h| h.id == "back"), "overlapping back layer still hit");
}

#[test]
fn pick_targets_empty_when_missed() {
    let host = two_pixel_layer_host();
    let hits: Vec<PickTargetJson> = serde_json::from_str(&host.pick_targets_at_screen_json(0.0, 0.0)).expect("json");
    assert!(hits.is_empty());
}

#[test]
fn marquee_hits_containment_vs_crossing() {
    let host = two_pixel_layer_host();
    let full_marquee = r#"{"points":[{"x":0,"y":0},{"x":400,"y":400}],"crossing":false}"#;
    let full_hits: Vec<String> = serde_json::from_str(&host.marquee_hits_json(full_marquee).expect("marquee")).expect("json");
    assert_eq!(full_hits.len(), 2, "marquee covering both layers should fully contain both");

    let partial_marquee = r#"{"points":[{"x":195,"y":150},{"x":260,"y":250}],"crossing":false}"#;
    let partial_hits: Vec<String> = serde_json::from_str(&host.marquee_hits_json(partial_marquee).expect("marquee")).expect("json");
    assert!(partial_hits.is_empty(), "small marquee should not fully contain any layer");

    let crossing_hits: Vec<String> = serde_json::from_str(&host.marquee_hits_json(r#"{"points":[{"x":195,"y":150},{"x":260,"y":250}],"crossing":true}"#).expect("marquee")).expect("json");
    assert!(!crossing_hits.is_empty(), "same rect with crossing=true should hit intersecting layers");
}

#[test]
fn navigator_fit_camera_centers_content() {
    let host = two_pixel_layer_host();
    let camera_json = host.navigator_fit_camera_json(300.0, 300.0);
    let camera: CameraJsonIn = serde_json::from_str(&camera_json).expect("camera json");
    assert!(camera.zoom > 0.0);
    assert!(camera.x.is_finite() && camera.y.is_finite());
}

#[test]
fn navigator_viewport_overlay_tracks_composite_camera() {
    let mut host = two_pixel_layer_host();
    let fit_json = host.navigator_fit_camera_json(300.0, 300.0);
    let fit: CameraJsonIn = serde_json::from_str(&fit_json).expect("camera json");
    host.set_camera(fit.x, fit.y, fit.zoom);
    let overlay_json = host.navigator_viewport_overlay_json(r#"{"x":0,"y":0,"zoom":1}"#, r#"{"width":400,"height":400}"#).expect("overlay");
    let overlay: serde_json::Value = serde_json::from_str(&overlay_json).expect("overlay json");
    assert!(overlay["width"].as_f64().unwrap() > 0.0);
    assert!(overlay["height"].as_f64().unwrap() > 0.0);
}

// #region 📄️ Document parsing errors
#[test]
fn parse_document_rejects_invalid_json() {
    let err = parse_document("not json").err().unwrap();
    assert!(matches!(err, FrameworkSurfacePaintError::Json(_)));
}

#[test]
fn parse_document_rejects_unsupported_schema() {
    let json = r#"{"schema":"vector.document","id":"t","layers":[]}"#;
    let err = parse_document(json).err().unwrap();
    match &err {
        FrameworkSurfacePaintError::UnsupportedSchema(s) => assert_eq!(s, "vector.document"),
        _ => panic!("expected UnsupportedSchema"),
    }
    assert!(err.to_string().contains("unsupported schema"));
}

#[test]
fn parse_document_pixel_defaults_when_fields_absent() {
    let json = r#"{"schema":"raster.document","id":"t","layers":[
            {"kind":"pixel","id":"p","name":"P","transform":{}}
        ]}"#;
    let doc = parse_document(json).expect("parse");
    match &doc.layers[0] {
        LayerNode::Pixel { visible, opacity, blend, width, height, image_key, mask, .. } => {
            assert!(*visible);
            assert_eq!(*opacity, 1.0);
            assert!(matches!(blend, BlendMode::Normal));
            assert_eq!(*width, 512);
            assert_eq!(*height, 512);
            assert!(image_key.is_none());
            assert!(mask.is_none());
        }
        _ => panic!("expected Pixel node"),
    }
}

#[test]
fn parse_document_group_with_mask_and_clip_to_below() {
    let json = r#"{"schema":"raster.document","id":"t","layers":[
            {"kind":"group","id":"g","name":"G","opacity":0.5,"blendMode":"multiply","transform":{},"clipToBelow":true,
             "mask":{"enabled":true,"linked":false,"invert":true,"width":64,"height":32},
             "children":[]}
        ]}"#;
    let doc = parse_document(json).expect("parse");
    match &doc.layers[0] {
        LayerNode::Group { opacity, blend, mask, children, .. } => {
            assert_eq!(*opacity, 0.5);
            assert!(matches!(blend, BlendMode::Multiply));
            let mask = mask.as_ref().expect("mask present");
            assert!(mask.enabled);
            assert!(mask.invert);
            assert_eq!(mask.width, 64);
            assert_eq!(mask.height, 32);
            assert!(children.is_empty());
        }
        _ => panic!("expected Group node"),
    }
}

#[test]
fn parse_document_opacity_out_of_range_is_clamped() {
    let json = r#"{"schema":"raster.document","id":"t","layers":[
            {"kind":"pixel","id":"p","name":"P","opacity":5.0,"transform":{}}
        ]}"#;
    let doc = parse_document(json).expect("parse");
    match &doc.layers[0] {
        LayerNode::Pixel { opacity, .. } => assert_eq!(*opacity, 1.0),
        _ => panic!("expected Pixel node"),
    }
}
// #endregion 📄️ Document parsing errors

// #region 🎨️ Blend mode mapping
#[test]
fn blend_from_str_maps_known_modes() {
    assert!(matches!(blend_from_str("multiply"), BlendMode::Multiply));
    assert!(matches!(blend_from_str("screen"), BlendMode::Screen));
    assert!(matches!(blend_from_str("overlay"), BlendMode::Overlay));
    assert!(matches!(blend_from_str("darken"), BlendMode::Darken));
    assert!(matches!(blend_from_str("lighten"), BlendMode::Lighten));
    assert!(matches!(blend_from_str("colorDodge"), BlendMode::ColorDodge));
    assert!(matches!(blend_from_str("colorBurn"), BlendMode::ColorBurn));
    assert!(matches!(blend_from_str("hardLight"), BlendMode::HardLight));
    assert!(matches!(blend_from_str("softLight"), BlendMode::SoftLight));
    assert!(matches!(blend_from_str("difference"), BlendMode::Difference));
    assert!(matches!(blend_from_str("exclusion"), BlendMode::Exclusion));
    assert!(matches!(blend_from_str("hue"), BlendMode::Hue));
    assert!(matches!(blend_from_str("saturation"), BlendMode::Saturation));
    assert!(matches!(blend_from_str("color"), BlendMode::Color));
    assert!(matches!(blend_from_str("luminosity"), BlendMode::Luminosity));
}

#[test]
fn blend_from_str_falls_back_to_normal_for_unknown() {
    assert!(matches!(blend_from_str("bogus"), BlendMode::Normal));
    assert!(matches!(blend_from_str(""), BlendMode::Normal));
}
// #endregion 🎨️ Blend mode mapping

// #region 🖼️ Pixel helpers
#[test]
fn checkerboard_rgba_has_correct_size_and_opaque_alpha() {
    let rgba = checkerboard_rgba(32, 16, 200, 40);
    assert_eq!(rgba.len(), 32 * 16 * 4);
    assert!(rgba.chunks_exact(4).all(|px| px[3] == 255));
}

#[test]
fn checkerboard_rgba_alternates_cells() {
    let rgba = checkerboard_rgba(32, 32, 200, 40);
    let px = |x: u32, y: u32| rgba[((y * 32 + x) * 4) as usize];
    assert_eq!(px(0, 0), 200, "cell (0,0) is light");
    assert_eq!(px(20, 0), 40, "cell (1,0) is dark");
    assert_eq!(px(0, 20), 40, "cell (0,1) is dark");
    assert_eq!(px(20, 20), 200, "cell (1,1) is light again");
}

#[test]
fn apply_brightness_contrast_shifts_brightness() {
    let mut rgba = vec![128u8, 128, 128, 255];
    apply_brightness_contrast(&mut rgba, 0.2, 0.0);
    assert_eq!(rgba[0], 179);
    assert_eq!(rgba[1], 179);
    assert_eq!(rgba[2], 179);
    assert_eq!(rgba[3], 255, "alpha channel untouched");
}

#[test]
fn apply_brightness_contrast_clamps_extremes() {
    let mut bright = vec![0u8, 0, 0, 255];
    apply_brightness_contrast(&mut bright, 2.0, 0.0);
    assert_eq!(bright[0], 255);

    let mut dark = vec![255u8, 255, 255, 255];
    apply_brightness_contrast(&mut dark, -2.0, 0.0);
    assert_eq!(dark[0], 0);
}

#[test]
fn apply_blur_box_zero_radius_is_noop() {
    let mut rgba = vec![10u8, 20, 30, 255, 200, 100, 50, 255];
    let original = rgba.clone();
    apply_blur_box(&mut rgba, 2, 1, 0);
    assert_eq!(rgba, original);
}

#[test]
fn apply_blur_box_preserves_uniform_image() {
    let width = 8u32;
    let height = 8u32;
    let mut rgba = vec![0u8; (width * height * 4) as usize];
    for px in rgba.chunks_exact_mut(4) {
        px.copy_from_slice(&[100, 150, 200, 255]);
    }
    apply_blur_box(&mut rgba, width, height, 20);
    assert!(rgba.chunks_exact(4).all(|px| px == [100, 150, 200, 255]));
}

#[test]
fn apply_blur_box_smooths_a_sharp_edge() {
    let width = 6u32;
    let height = 1u32;
    let mut rgba = vec![0u8; (width * height * 4) as usize];
    for x in 0..width {
        let v = if x < width / 2 { 0u8 } else { 255u8 };
        let idx = (x * 4) as usize;
        rgba[idx..idx + 4].copy_from_slice(&[v, v, v, 255]);
    }
    apply_blur_box(&mut rgba, width, height, 1);
    let mid = ((width / 2) * 4) as usize;
    assert!(rgba[mid] > 0 && rgba[mid] < 255, "boundary pixel should be averaged, got {}", rgba[mid]);
}
// #endregion 🖼️ Pixel helpers

// #region 🖱️ RasterHost lifecycle
#[test]
fn raster_host_new_has_sane_defaults() {
    let host = RasterHost::new();
    assert_eq!(host.active_utility, "selectMarquee");
    assert_eq!(host.brush_size, 24.0);
    assert_eq!(host.brush_opacity, 1.0);
    assert!(host.selected_ids.is_empty());
    assert!(host.show_selection_chrome);
}

#[test]
fn set_size_clamps_minimums() {
    let mut host = RasterHost::new();
    host.set_size(0, 0, 0.1);
    assert_eq!(host.viewport.width, 1);
    assert_eq!(host.viewport.height, 1);
    assert_eq!(host.viewport.dpr, 1.0);
}

#[test]
fn set_camera_clamps_zoom_bounds() {
    let mut host = RasterHost::new();
    host.set_camera(10.0, 20.0, 1_000_000.0);
    assert_eq!(host.camera.x, 10.0);
    assert_eq!(host.camera.y, 20.0);
    assert!(host.camera.zoom < 1_000_000.0 && host.camera.zoom > 0.0);

    host.set_camera(0.0, 0.0, -5.0);
    assert!(host.camera.zoom > 0.0, "negative zoom must clamp positive");
}

#[test]
fn wheel_screen_changes_zoom_toward_cursor() {
    let mut host = RasterHost::new();
    host.set_size(400, 400, 1.0);
    let before = host.camera.zoom;
    host.wheel_screen(200.0, 200.0, -1.0);
    assert!(host.camera.zoom > before, "negative delta_y should zoom in");
}

#[test]
fn set_canvas_theme_from_json_updates_checkerboard_for_dark_clear() {
    let mut host = RasterHost::new();
    host.set_canvas_theme_from_json(r#"{"rasterClear":[0,0,0,255]}"#).expect("theme");
    assert_eq!((host.checkerboard_light_cell, host.checkerboard_dark_cell), (64, 48));
}

#[test]
fn set_canvas_theme_from_json_rejects_invalid_json() {
    let mut host = RasterHost::new();
    let err = host.set_canvas_theme_from_json("not json").unwrap_err();
    assert!(matches!(err, FrameworkSurfacePaintError::Json(_)));
}

#[test]
fn set_show_selection_chrome_toggles_flag() {
    let mut host = RasterHost::new();
    host.set_show_selection_chrome(false);
    assert!(!host.show_selection_chrome);
}
// #endregion 🖱️ RasterHost lifecycle

// #region ✋️ Pointer / paint interaction
#[test]
fn pointer_down_button1_pans_on_move() {
    let mut host = RasterHost::new();
    host.set_size(400, 400, 1.0);
    host.pointer_down_screen(100.0, 100.0, 1);
    assert!(host.panning);
    let cam_before = (host.camera.x, host.camera.y);
    host.pointer_move_screen(110.0, 130.0);
    assert_ne!((host.camera.x, host.camera.y), cam_before, "pan should move camera");
    host.pointer_up_screen(0.0, 0.0);
    assert!(!host.panning);
    assert!(host.pan_last.is_none());
}

#[test]
fn pointer_down_paint_utility_paints_immediately() {
    let mut host = RasterHost::new();
    host.set_size(400, 400, 1.0);
    host.set_active_utility("paintBrush");
    host.sync_interaction(&["back".to_string()], None);
    host.pointer_down_screen(400.0, 400.0, 0);
    assert!(host.painting);
    let key = RasterHost::layer_pixel_buffer_key("back");
    let buf = host.buffers.paint.get(&key).expect("buffer created");
    let painted = buf.chunks_exact(4).any(|px| px[0] == 40 && px[1] == 120 && px[2] == 220);
    assert!(painted, "brush color should appear in buffer");
}

#[test]
fn pointer_move_while_painting_strokes_between_points() {
    let mut host = RasterHost::new();
    host.set_size(400, 400, 1.0);
    host.set_active_utility("paintBrush");
    host.sync_interaction(&["back".to_string()], None);
    host.pointer_down_screen(350.0, 400.0, 0);
    host.pointer_move_screen(450.0, 400.0);
    let key = RasterHost::layer_pixel_buffer_key("back");
    let buf = host.buffers.paint.get(&key).expect("buffer created");
    let painted_count = buf.chunks_exact(4).filter(|px| px[0] == 40 && px[2] == 220).count();
    assert!(painted_count > 20, "stroke across two points should paint more than a single dab, got {painted_count}");
}

#[test]
fn paint_eraser_reduces_alpha_instead_of_coloring() {
    let mut host = RasterHost::new();
    host.set_size(400, 400, 1.0);
    host.set_active_utility("paintEraser");
    host.set_brush_opacity(1.0);
    host.sync_interaction(&["back".to_string()], None);
    host.pointer_down_screen(400.0, 400.0, 0);
    let key = RasterHost::layer_pixel_buffer_key("back");
    let buf = host.buffers.paint.get(&key).expect("buffer created");
    let center_idx = ((200usize * 512) + 200) * 4;
    assert_eq!(buf[center_idx + 3], 0, "fully-opaque erase should zero alpha");
}
// #endregion ✋️ Pointer / paint interaction

// #region 📤️ Image uploads
fn png_bytes(width: u32, height: u32) -> Vec<u8> {
    let img = image::RgbaImage::from_pixel(width, height, image::Rgba([10, 20, 30, 255]));
    let mut cursor = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgba8(img).write_to(&mut cursor, image::ImageFormat::Png).expect("encode png");
    cursor.into_inner()
}

#[test]
fn upload_layer_image_decodes_valid_png() {
    let mut host = RasterHost::new();
    let bytes = png_bytes(4, 4);
    host.upload_layer_image("layer1", &bytes).expect("decode");
    let key = RasterHost::layer_pixel_buffer_key("layer1");
    let buf = host.buffers.paint.get(&key).expect("buffer stored");
    assert_eq!(buf.len(), 4 * 4 * 4);
    assert_eq!(&buf[0..4], &[10, 20, 30, 255]);
    assert!(host.images.get(&key).is_some());
}

#[test]
fn upload_layer_image_rejects_invalid_bytes() {
    let mut host = RasterHost::new();
    let err = host.upload_layer_image("layer1", b"not an image").unwrap_err();
    assert!(matches!(err, FrameworkSurfacePaintError::Image(_)));
}

#[test]
fn upload_raster_image_key_stores_under_given_key() {
    let mut host = RasterHost::new();
    let bytes = png_bytes(2, 2);
    host.upload_raster_image_key("custom:key", &bytes).expect("decode");
    assert!(host.buffers.paint.contains_key("custom:key"));
    assert!(host.images.get("custom:key").is_some());
}
// #endregion 📤️ Image uploads

// #region ⚙️ Settings
#[test]
fn set_brush_opacity_clamps_range() {
    let mut host = RasterHost::new();
    host.set_brush_opacity(5.0);
    assert_eq!(host.brush_opacity, 1.0);
    host.set_brush_opacity(-1.0);
    assert_eq!(host.brush_opacity, 0.0);
}

#[test]
fn sync_interaction_updates_hovered_and_selected_state() {
    let mut host = RasterHost::new();
    host.sync_interaction(&[], Some("x"));
    assert_eq!(host.hovered_id.as_deref(), Some("x"));
    assert!(host.selected_ids.is_empty());
    host.sync_interaction(&["p".to_string()], None);
    assert!(host.hovered_id.is_none());
    assert_eq!(host.selected_ids, vec!["p".to_string()]);
}

#[test]
fn camera_json_reflects_current_state() {
    let mut host = RasterHost::new();
    host.set_camera(3.0, 4.0, 2.0);
    let json: serde_json::Value = serde_json::from_str(&host.camera_json()).expect("json");
    assert_eq!(json["x"], 3.0);
    assert_eq!(json["y"], 4.0);
    assert_eq!(json["zoom"], 2.0);
}
// #endregion ⚙️ Settings

// #region 🎬️ Scene building
#[test]
fn build_vector_scene_empty_document_is_empty() {
    let mut host = RasterHost::new();
    host.set_size(400, 400, 1.0);
    assert!(host.build_vector_scene().is_empty());
}

#[test]
fn build_vector_scene_skips_invisible_layers() {
    let json = r#"{"schema":"raster.document","id":"t","layers":[
            {"kind":"pixel","id":"p","name":"P","visible":false,"transform":{},"width":50,"height":50}
        ]}"#;
    let mut host = RasterHost::new();
    host.set_size(400, 400, 1.0);
    host.sync_document_json(json).expect("sync");
    assert!(host.build_vector_scene().is_empty(), "invisible layer should not draw");
}

#[test]
fn build_vector_scene_draws_visible_pixel_layer() {
    let json = r#"{"schema":"raster.document","id":"t","layers":[
            {"kind":"pixel","id":"p","name":"P","transform":{},"width":50,"height":50}
        ]}"#;
    let mut host = RasterHost::new();
    host.set_size(400, 400, 1.0);
    host.sync_document_json(json).expect("sync");
    assert!(!host.build_vector_scene().is_empty());
}

#[test]
fn build_vector_scene_adds_stroke_for_selected_layer_when_chrome_enabled() {
    let json = r#"{"schema":"raster.document","id":"t","layers":[
            {"kind":"pixel","id":"p","name":"P","transform":{},"width":50,"height":50}
        ]}"#;
    let mut host = RasterHost::new();
    host.set_size(400, 400, 1.0);
    host.sync_document_json(json).expect("sync");
    let base_count = host.build_vector_scene().path_count();

    host.sync_interaction(&["p".to_string()], None);
    let selected_count = host.build_vector_scene().path_count();
    assert!(selected_count > base_count, "selection chrome should add an extra stroke path");

    host.set_show_selection_chrome(false);
    let hidden_count = host.build_vector_scene().path_count();
    assert_eq!(hidden_count, base_count, "disabling chrome should drop the stroke again");
}

#[test]
fn build_layer_scene_isolates_single_pixel_layer() {
    let json = r#"{"schema":"raster.document","id":"t","layers":[
            {"kind":"pixel","id":"back","name":"Back","transform":{},"width":50,"height":50},
            {"kind":"pixel","id":"front","name":"Front","transform":{},"width":50,"height":50}
        ]}"#;
    let mut host = RasterHost::new();
    host.set_size(400, 400, 1.0);
    host.sync_document_json(json).expect("sync");
    let full = host.build_vector_scene().path_count();
    let isolated = host.build_layer_scene("front").path_count();
    assert!(isolated > 0 && isolated < full, "isolated single-layer scene should draw less than the full composite");
}

#[test]
fn build_layer_scene_group_isolation_recurses_into_children() {
    let json = r#"{"schema":"raster.document","id":"t","layers":[
            {"kind":"group","id":"g","name":"G","transform":{},"children":[
                {"kind":"pixel","id":"child","name":"C","transform":{},"width":50,"height":50}
            ]}
        ]}"#;
    let mut host = RasterHost::new();
    host.set_size(400, 400, 1.0);
    host.sync_document_json(json).expect("sync");
    assert!(!host.build_layer_scene("child").is_empty(), "isolating a group id should still recurse to draw its children");
}

#[test]
fn build_mask_scene_returns_nonempty_scene() {
    let mut host = RasterHost::new();
    host.set_size(400, 400, 1.0);
    assert!(!host.build_mask_scene("any").is_empty());
}

#[test]
fn build_render_scene_matches_vector_scene_at_unit_dpr() {
    let json = r#"{"schema":"raster.document","id":"t","layers":[
            {"kind":"pixel","id":"p","name":"P","transform":{},"width":50,"height":50}
        ]}"#;
    let mut host = RasterHost::new();
    host.set_size(400, 400, 1.0);
    host.sync_document_json(json).expect("sync");
    assert!(!host.build_render_scene().is_empty());
}

#[test]
fn build_render_scene_scales_for_device_pixel_ratio() {
    let json = r#"{"schema":"raster.document","id":"t","layers":[
            {"kind":"pixel","id":"p","name":"P","transform":{},"width":50,"height":50}
        ]}"#;
    let mut host = RasterHost::new();
    host.set_size(400, 400, 2.0);
    host.sync_document_json(json).expect("sync");
    assert!(!host.build_render_scene().is_empty());
}

#[test]
fn append_layer_node_draws_enabled_mask() {
    let json = r#"{"schema":"raster.document","id":"t","layers":[
            {"kind":"pixel","id":"p","name":"P","transform":{},"width":50,"height":50,
             "mask":{"enabled":true,"invert":true,"width":50,"height":50}}
        ]}"#;
    let mut host = RasterHost::new();
    host.set_size(400, 400, 1.0);
    host.sync_document_json(json).expect("sync");
    let masked = host.build_vector_scene().path_count();

    let unmasked_json = r#"{"schema":"raster.document","id":"t","layers":[
            {"kind":"pixel","id":"p","name":"P","transform":{},"width":50,"height":50}
        ]}"#;
    let mut host2 = RasterHost::new();
    host2.set_size(400, 400, 1.0);
    host2.sync_document_json(unmasked_json).expect("sync");
    let unmasked = host2.build_vector_scene().path_count();

    assert!(masked > unmasked, "enabled mask should draw an extra image");
}

#[test]
fn append_layer_node_adjustment_layer_is_transparent_to_scene() {
    let json = r#"{"schema":"raster.document","id":"t","layers":[
            {"kind":"adjustment","id":"a","name":"A","transform":{},"adjustmentKind":"brightnessContrast",
             "params":{"brightness":0.3,"contrast":0.1}}
        ]}"#;
    let mut host = RasterHost::new();
    host.set_size(400, 400, 1.0);
    host.sync_document_json(json).expect("sync");
    assert!(host.build_vector_scene().is_empty(), "adjustment layers draw no scene geometry themselves");
}
// #endregion 🎬️ Scene building

// #region 📐️ ScreenRect
#[test]
fn screen_rect_contains_and_intersects() {
    let outer = ScreenRect { x: 0.0, y: 0.0, width: 100.0, height: 100.0 };
    let inner = ScreenRect { x: 10.0, y: 10.0, width: 20.0, height: 20.0 };
    let overlapping = ScreenRect { x: 90.0, y: 90.0, width: 50.0, height: 50.0 };
    let disjoint = ScreenRect { x: 200.0, y: 200.0, width: 10.0, height: 10.0 };

    assert!(outer.contains(&inner));
    assert!(!outer.contains(&overlapping));
    assert!(outer.intersects(&overlapping));
    assert!(!outer.intersects(&disjoint));
    assert!(outer.contains_point(0.0, 0.0));
    assert!(outer.contains_point(100.0, 100.0));
    assert!(!outer.contains_point(100.1, 50.0));
}

#[test]
fn screen_rect_union_grows_bounding_box() {
    let a = ScreenRect { x: 0.0, y: 0.0, width: 10.0, height: 10.0 };
    let b = ScreenRect { x: 5.0, y: -5.0, width: 10.0, height: 10.0 };
    let merged = ScreenRect::union(Some(a), b);
    assert_eq!(merged.x, 0.0);
    assert_eq!(merged.y, -5.0);
    assert_eq!(merged.width, 15.0);
    assert_eq!(merged.height, 15.0);

    let first = ScreenRect::union(None, a);
    assert_eq!(first.width, 10.0);
}
// #endregion 📐️ ScreenRect

// #region 🎯️ Picking edge cases
#[test]
fn marquee_hits_json_requires_at_least_two_points() {
    let host = two_pixel_layer_host();
    let hits: Vec<String> = serde_json::from_str(&host.marquee_hits_json(r#"{"points":[{"x":0,"y":0}],"crossing":false}"#).expect("marquee")).expect("json");
    assert!(hits.is_empty());
}

#[test]
fn marquee_hits_json_rejects_invalid_json() {
    let host = two_pixel_layer_host();
    let err = host.marquee_hits_json("not json").unwrap_err();
    assert!(matches!(err, FrameworkSurfacePaintError::Json(_)));
}

#[test]
fn navigator_viewport_overlay_rejects_invalid_camera_json() {
    let host = two_pixel_layer_host();
    let err = host.navigator_viewport_overlay_json("not json", r#"{"width":400,"height":400}"#).unwrap_err();
    assert!(matches!(err, FrameworkSurfacePaintError::Json(_)));
}

#[test]
fn navigator_fit_camera_json_falls_back_when_document_is_empty() {
    let mut host = RasterHost::new();
    host.set_camera(7.0, 8.0, 1.5);
    let json = host.navigator_fit_camera_json(300.0, 300.0);
    let camera: CameraJsonIn = serde_json::from_str(&json).expect("camera json");
    assert_eq!(camera.x, 7.0);
    assert_eq!(camera.y, 8.0);
    assert_eq!(camera.zoom, 1.5);
}
// #endregion 🎯️ Picking edge cases
