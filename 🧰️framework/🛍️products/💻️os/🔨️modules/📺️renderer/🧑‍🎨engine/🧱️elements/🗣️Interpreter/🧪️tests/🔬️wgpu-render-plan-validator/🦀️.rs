
use super::*;
use ui_wgpu::wgpu::{TableScene, UiStackNode, World3dScene, build_table_scene, build_world_3d_scene};

#[test]
fn validate_ui_node_rejects_oversized_json_payload() {
    let limits = RenderPlanLimits { max_json_payload_bytes: 16, ..RenderPlanLimits::default() };
    let node = build_table_scene("table", "controller", TableScene::base("[]", "x".repeat(32)));
    let error = validate_ui_node(&node, &limits).expect_err("oversized payload should be rejected");
    assert!(error.contains("table.rows"));
    assert!(error.contains("32 bytes"));
}

fn empty_stack(children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children, menu: None })
}

#[test]
fn validate_component_scene_rejects_oversized_mesh_count() {
    let limits = RenderPlanLimits { max_mesh_count: 2, ..RenderPlanLimits::default() };
    let meshes_json = serde_json::to_string(&vec![serde_json::json!({"id": "m"}); 3]).unwrap();
    let node = build_world_3d_scene(
        "world",
        "controller",
        World3dScene {
            snapshot: None,
            camera_json: "{}".into(),
            meshes_json,
            instances_json: "[]".into(),
            selection_json: "{}".into(),
            vortices_json: None,
            attractions_json: None,
            target_volumes_json: None,
            references_json: None,
            brush_preview_json: None,
            interaction_json: None,
            engagement_preview_json: None,
            lod_json: None,
            chunking_json: None,
            environment_json: None,
            frame_json: None,
            fit_json: None,
            terrain_json: None,
            points_json: None,
            status_json: None,
            domain_id: None,
            domain_granularity_id: None,
            lanes: Vec::new(),
        },
    );
    let error = validate_ui_node(&node, &limits).expect_err("oversized mesh count should be rejected");
    assert!(error.contains("mesh count 3 exceeds max 2"));
}

#[test]
fn validate_ui_node_rejects_oversized_node_count() {
    let limits = RenderPlanLimits { max_node_count: 3, ..RenderPlanLimits::default() };
    let tree = empty_stack(vec![empty_stack(vec![]), empty_stack(vec![]), empty_stack(vec![])]);
    let error = validate_ui_node(&tree, &limits).expect_err("oversized node count should be rejected");
    assert!(error.contains("node count 4 exceeds max 3"));
}

#[test]
fn validate_ui_node_rejects_oversized_tree_depth() {
    let limits = RenderPlanLimits { max_tree_depth: 2, ..RenderPlanLimits::default() };
    let mut tree = empty_stack(vec![]);
    for _ in 0..4 {
        tree = empty_stack(vec![tree]);
    }
    let error = validate_ui_node(&tree, &limits).expect_err("oversized tree depth should be rejected");
    assert!(error.contains("tree depth"));
    assert!(error.contains("exceeds max 2"));
}

//#region UiImageLoadingTests
const TEST_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="40" height="20"><rect width="40" height="20" fill="red"/></svg>"#;

fn tiny_png_bytes(r: u8, g: u8, b: u8) -> Vec<u8> {
    let img = image::RgbaImage::from_pixel(4, 2, image::Rgba([r, g, b, 255]));
    let mut bytes: Vec<u8> = Vec::new();
    image::DynamicImage::ImageRgba8(img).write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png).expect("encode tiny test png");
    bytes
}

#[test]
fn resolve_ui_image_decodes_inline_svg_data_url_at_natural_aspect_ratio() {
    use base64::Engine;
    let src = format!("data:image/svg+xml;base64,{}", base64::engine::general_purpose::STANDARD.encode(TEST_SVG));
    let (key, size) = resolve_ui_image("svg-image-test-a", &src);
    assert!(key.is_some(), "inline svg data url should decode to a raster key");
    assert_eq!(size, Some((40, 20)), "natural size should come from the svg's own width/height");
}

#[test]
fn resolve_ui_image_decodes_plain_utf8_svg_data_url() {
    let src = format!("data:image/svg+xml,{TEST_SVG}");
    let (key, size) = resolve_ui_image("svg-image-test-plain", &src);
    assert!(key.is_some(), "a non-base64 (plain utf-8) svg data url should also decode");
    assert_eq!(size, Some((40, 20)));
}

#[test]
fn inline_svg_saturation_rejects_before_parse_or_source_copy() {
    let mut reservations = Vec::with_capacity(256);
    for index in 0..256 {
        reservations.push(ui_wgpu::wgpu::PreparedRasterReservation::try_reserve(format!("svg-held-{index}")).expect("fixed process raster slot"));
    }
    INLINE_SVG_PARSE_CALLS.with(|calls| calls.set(0));
    let src = format!("data:image/svg+xml,{TEST_SVG}");
    assert_eq!(resolve_ui_image("svg-saturated", &src), (None, None));
    INLINE_SVG_PARSE_CALLS.with(|calls| assert_eq!(calls.get(), 0, "source parse/copy must follow fixed reservation"));
    for reservation in reservations {
        let mut rejected = reservation.reject("test slot release", Vec::new());
        while !rejected.close_step() {}
    }
    let mut cursor = crate::scenes::PendingRasterUploadCursor::default();
    while !matches!(cursor.step(), crate::scenes::PendingRasterUploadStep::Fault(_)) {}
}

#[test]
fn resolve_ui_image_queues_a_fetch_for_http_url_and_renders_nothing_until_resolved() {
    let id = "http-image-test-a";
    let (key, size) = resolve_ui_image(id, "https://example.invalid/pic.png");
    assert!(key.is_none(), "an unresolved http(s) url should not yet have a raster key");
    assert!(size.is_none());
    let mut pending = crate::take_next_renderer_asset().expect("the http(s) URL reserves one fixed request owner");
    assert_eq!(pending.url(), "https://example.invalid/pic.png");
    assert_eq!(pending.kind(), WorldAssetRequestKind::UiImage { id: WorldAssetMetadataId::try_from_str(id).unwrap() });
    pending.begin_close();
    crate::return_renderer_asset(pending).expect("cancelled UI image request returns to the shared authority");
    while crate::retire_cancelled_renderer_asset_step() {}
}

#[test]
fn apply_ui_image_bytes_decodes_fetched_png_and_resolve_ui_image_then_finds_it() {
    let id = "http-image-test-b";
    let url = "https://example.invalid/tiny.png";
    let _ = resolve_ui_image(id, url);
    let mut pending = crate::take_next_renderer_asset().expect("UI image request owner");
    pending.begin_close();
    crate::return_renderer_asset(pending).expect("test request handback");
    while crate::retire_cancelled_renderer_asset_step() {}
    apply_ui_image_bytes(id, url, &tiny_png_bytes(9, 8, 7));
    let (key, size) = resolve_ui_image(id, url);
    assert!(key.is_some(), "a completed fetch should resolve to a raster key on the next render pass");
    assert_eq!(size, Some((4, 2)));
}

#[test]
fn apply_ui_image_bytes_rasterizes_fetched_svg_content_sniffed_by_extension() {
    let id = "http-image-test-c";
    let url = "https://example.invalid/icon.svg";
    apply_ui_image_bytes(id, url, TEST_SVG.as_bytes());
    let (key, size) = resolve_ui_image(id, url);
    assert!(key.is_some(), "svg content sniffed by the .svg extension should rasterize to a raster key");
    assert_eq!(size, Some((40, 20)));
}

#[test]
fn object_contain_rect_fills_exactly_when_aspect_ratios_match() {
    let bounds = Rect::new(0.0, 0.0, 100.0, 50.0);
    let fit = object_contain_rect(bounds, 40.0, 20.0);
    assert!((fit.w - 100.0).abs() < 0.01);
    assert!((fit.h - 50.0).abs() < 0.01);
}

#[test]
fn object_contain_rect_letterboxes_and_centers_narrower_content() {
    let bounds = Rect::new(0.0, 0.0, 100.0, 50.0);
    let fit = object_contain_rect(bounds, 10.0, 20.0);
    assert!(fit.h <= 50.0 + 0.01);
    assert!(fit.w < 100.0, "narrower-than-bounds content should not stretch to fill the width");
    assert!(fit.x > 0.0, "narrower-than-bounds content should be horizontally centered");
}
//#endregion UiImageLoadingTests
