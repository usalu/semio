//! 🖼️ Unit tests of the UI image loading of scene-canvas2d: inline SVG and data url admission, fetched bytes, object-contain fit and the inline image paint step.

use crate::ui_image_loading::{apply_ui_image_bytes, current_ui_image_key, object_contain_rect, render_ui_image_step, resolve_ui_image, INLINE_SVG_PARSE_CALLS, UI_IMAGE_SIZES};
use infinite_world::world::{WorldAssetMetadataId, WorldAssetRequestKind};
use semio_framework_os_renderer_wgpu_asset_authority::{retire_cancelled_renderer_asset_step, return_renderer_asset, take_next_renderer_asset};
use semio_framework_os_renderer_wgpu_contracts::framework_widget_context;
use semio_framework_os_renderer_wgpu_scene_state::{PendingRasterUploadCursor, PendingRasterUploadStep};
use ui_wgpu::wgpu::{ActionDescriptor, Rect};

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

/// 🧯️ Saturates the raster ledger FROM WHEREVER IT IS rather than demanding all 256 slots: the
/// producer ledger is process-wide, so every neighbouring test that resolved a ui image still holds
/// one. Demanding the full capacity made this law `expect`-panic mid-loop, and the panic then leaked
/// the reservations it had already taken — which is why the whole inline-svg family failed in-suite
/// and passed alone (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY wave 2–6 integration).
#[test]
fn inline_svg_saturation_rejects_before_parse_or_source_copy() {
    let mut reservations = Vec::with_capacity(256);
    for index in 0..256 {
        let Ok(reservation) = ui_wgpu::wgpu::PreparedRasterReservation::try_reserve(format!("svg-held-{index}")) else { break };
        reservations.push(reservation);
    }
    assert!(!reservations.is_empty(), "the fixed raster ledger admitted nothing at all — the saturation law would pass vacuously");
    assert!(ui_wgpu::wgpu::PreparedRasterReservation::try_reserve("svg-held-plus-one".to_string()).is_err(), "the ledger is saturated once it refuses one more");
    INLINE_SVG_PARSE_CALLS.with(|calls| calls.set(0));
    let src = format!("data:image/svg+xml,{TEST_SVG}");
    assert_eq!(resolve_ui_image("svg-saturated", &src), (None, None));
    INLINE_SVG_PARSE_CALLS.with(|calls| assert_eq!(calls.get(), 0, "source parse/copy must follow fixed reservation"));
    for reservation in reservations {
        let mut rejected = reservation.reject("test slot release", Vec::new());
        while !rejected.close_step() {}
    }
    let mut cursor = PendingRasterUploadCursor::default();
    for _ in 0..65_536 {
        match cursor.step() {
            PendingRasterUploadStep::Complete => return,
            PendingRasterUploadStep::Fault(fault) => panic!("back-pressure retires without faulting the frame: {fault}"),
            _ => {}
        }
    }
    panic!("the upload cursor retires the parked back-pressure refusal within its bounded pass");
}

#[test]
fn resolve_ui_image_queues_a_fetch_for_http_url_and_renders_nothing_until_resolved() {
    let id = "http-image-test-a";
    let (key, size) = resolve_ui_image(id, "https://example.invalid/pic.png");
    assert!(key.is_none(), "an unresolved http(s) url should not yet have a raster key");
    assert!(size.is_none());
    let mut pending = take_next_renderer_asset().expect("the http(s) URL reserves one fixed request owner");
    assert_eq!(pending.url(), "https://example.invalid/pic.png");
    assert_eq!(pending.kind(), WorldAssetRequestKind::UiImage { id: WorldAssetMetadataId::try_from_str(id).unwrap() });
    pending.begin_close();
    return_renderer_asset(pending).expect("cancelled UI image request returns to the shared authority");
    while retire_cancelled_renderer_asset_step() {}
}

#[test]
fn apply_ui_image_bytes_decodes_fetched_png_and_resolve_ui_image_then_finds_it() {
    let id = "http-image-test-b";
    let url = "https://example.invalid/tiny.png";
    let _ = resolve_ui_image(id, url);
    let mut pending = take_next_renderer_asset().expect("UI image request owner");
    pending.begin_close();
    return_renderer_asset(pending).expect("test request handback");
    while retire_cancelled_renderer_asset_step() {}
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

/// ⚖️ Law: the PRODUCTION paint path renders an inline `data:` image. `render_ui_image_step`'s
/// phase 2 answered `ScenePaintStep::Fault` for every `src` starting with `data:`, and
/// `resolve_ui_image`/`resolve_ui_image_svg`/`resolve_ui_image_url` were ALL `#[cfg(test)]` — so
/// `Component::Image` with an inline bitmap faulted the whole paint step on browser and native
/// alike, while React's `ImageView` is a plain `<img src>` the browser decodes natively.
#[test]
fn render_ui_image_step_paints_an_inline_data_url_instead_of_faulting() {
    use base64::Engine;
    let src = format!("data:image/png;base64,{}", base64::engine::general_purpose::STANDARD.encode(tiny_png_bytes(3, 2, 1)));
    let image = ui_wgpu::wgpu::UiImageNode { id: "inline-data-image".into(), src, alt: None, presence: ui_wgpu::wgpu::UiPresence::default(), menu: None };
    let bounds = Rect::new(0.0, 0.0, 80.0, 40.0);
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let theme = ui_wgpu::wgpu::Theme::default();
    let mut scroll = std::collections::HashMap::new();
    let mut collapsed = std::collections::HashMap::new();
    let mut selects = std::collections::HashMap::new();
    let mut cursor = ui_wgpu::wgpu::ScenePaintCursor::default();
    {
        let mut ctx = framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
        for _ in 0..4096 {
            match render_ui_image_step(&image, bounds, &mut ctx, &mut cursor) {
                ui_wgpu::wgpu::ScenePaintStep::Pending => {}
                ui_wgpu::wgpu::ScenePaintStep::Complete => break,
                ui_wgpu::wgpu::ScenePaintStep::Fault => panic!("an inline data:image/png source must paint, not fault"),
            }
        }
    }
    let rasters: Vec<String> = draw.layers.iter().flat_map(|layer| layer.raster_instances.iter()).map(|(key, _)| key.clone()).collect();
    assert!(rasters.iter().any(|key| key.contains("inline-data-image")), "the decoded inline bitmap must be drawn as a raster quad keyed by its node id, got {rasters:?}");
    let size = UI_IMAGE_SIZES.with(|cell| cell.borrow().get("inline-data-image").copied());
    assert_eq!(size, Some((4, 2)), "the natural pixel size comes from the decoded bitmap, so object-contain matches React's intrinsic aspect ratio");
}

/// 📻️ An avatar source replacement keeps fallback visible until that exact source owns decoded pixels.
#[test]
fn avatar_image_replacement_cannot_publish_the_previous_sources_pixels() {
    let id = "vfs-avatar-current-source";
    let first = format!("data:image/svg+xml,{TEST_SVG}");
    let current = current_ui_image_key(id, &first).expect("the first avatar source is decoded");
    assert_eq!(current_ui_image_key(id, &first), Some(current.clone()));
    assert_eq!(current_ui_image_key(id, "data:image/svg+xml,invalid"), None);
    assert_eq!(current_ui_image_key(id, ""), None);
    assert_eq!(current_ui_image_key(id, &first), Some(current));
}
