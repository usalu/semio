//! 🧪️ Raster preview uses the same SVG layers, dimensions, and independent pixel probes as export.

use super::{preview_extent, rasterize};
use crate::icon_render::icon_export::{IconSvgExport, svg::tests::{asset, fixture, request}};
use semio_framework_async::CancelToken;

#[test]
fn rgba_preview_matches_export_samples_and_preserves_cancel() {
    let fixture = fixture();
    let mut export = IconSvgExport::new(&request(&fixture), asset(0x5356_4750, &fixture)).expect("SVG export");
    let mut steps = 0;
    while !export.advance().expect("SVG step") {
        steps += 1;
        assert!(steps < 256);
    }
    let contents = export.take_svg().unwrap();
    while !export.close_step() {}

    let cancel = CancelToken::root_now();
    let budget = fixture["previewRaster"]["byteBudget"].as_u64().unwrap() as usize;
    let image = rasterize(&contents, &cancel, budget).expect("preview raster");
    assert_eq!((image.width, image.height), (96, 64));
    for probe in fixture["expected"]["probes"].as_array().unwrap() {
        let x = probe["position"][0].as_u64().unwrap() as usize;
        let y = probe["position"][1].as_u64().unwrap() as usize;
        let offset = 4 * (y * image.width as usize + x);
        let expected = probe["rgba"].as_array().unwrap().iter().map(|channel| channel.as_u64().unwrap() as u8);
        for (actual, expected) in image.pixels[offset..offset + 4].iter().zip(expected) {
            assert!(actual.abs_diff(expected) <= 1);
        }
    }
    cancel.cancel_now();
    assert!(rasterize(&contents, &cancel, budget).is_err());
}

#[test]
fn preview_raster_cap_keeps_the_svg_aspect_ratio_and_straight_alpha() {
    let cancel = CancelToken::root_now();
    let fixture = fixture();
    let raster = &fixture["previewRaster"];
    let budget = raster["byteBudget"].as_u64().unwrap() as usize;
    let fill = raster["fill"].as_str().unwrap();
    let opacity = raster["opacity"].as_f64().unwrap();
    for case in raster["cases"].as_array().unwrap() {
        let width = case["sourceSize"][0].as_u64().unwrap();
        let height = case["sourceSize"][1].as_u64().unwrap();
        let svg = format!(r#"<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}"><rect width="{width}" height="{height}" fill="{fill}" fill-opacity="{opacity}"/></svg>"#);
        let image = rasterize(svg.as_bytes(), &cancel, budget).expect("preview raster");
        assert_eq!(u64::from(image.width), case["expectedSize"][0].as_u64().unwrap());
        assert_eq!(u64::from(image.height), case["expectedSize"][1].as_u64().unwrap());
        assert!(image.pixels.len() <= budget);
        for (actual, expected) in image.pixels[..4].iter().zip(raster["rgba"].as_array().unwrap()) {
            assert!(actual.abs_diff(expected.as_u64().unwrap() as u8) <= 1);
        }
    }
}

#[test]
fn a_preview_requires_one_complete_pixel() {
    assert_eq!(preview_extent(16384.0, 1.0, 3), None);
    assert_eq!(preview_extent(16384.0, 1.0, 4), Some((1, 1)));
    assert_eq!(preview_extent(1.0, 16384.0, 4), Some((1, 1)));
}
