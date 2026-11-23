//! 🔆️ Canvas DCT image previews against an independent PDF renderer.

use super::*;

#[test]
fn coded_image_preview_matches_mupdf() {
    use crate::standards::v1_7::subsets::base::io::import::decode_pdf;
    use semio_framework_io_base64::base64_standard_decode;

    let fixture: Value = serde_json::from_str(include_str!("../🧪️coded-image.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let input = base64_standard_decode(case["pdfBase64"].as_str().unwrap()).unwrap();
        let snapshot = decode_pdf(&input).unwrap_or_else(|error| panic!("{name}: {error}"));
        let scene: Value = serde_json::from_str(&canvas_layers(&snapshot, None)).unwrap();
        let layer = scene.as_array().unwrap().iter().find(|layer| layer["kind"] == "image")
            .unwrap_or_else(|| panic!("{name}: no image layer"));
        let src = layer["image"]["src"].as_str().unwrap().strip_prefix("data:image/png;base64,").unwrap();
        let raster = semio_framework_pixels::decode_png(&base64_standard_decode(src).unwrap()).unwrap();
        let dimensions: [u32; 2] = serde_json::from_value(case["dimensions"].clone()).unwrap();
        assert_eq!([raster.width, raster.height], dimensions, "{name}");
        for sample in case["samples"].as_array().unwrap() {
            let [x, y]: [u32; 2] = serde_json::from_value(sample["at"].clone()).unwrap();
            let offset = ((y * raster.width + x) * 4) as usize;
            let rgba: [u8; 4] = raster.pixels[offset..offset + 4].try_into().unwrap();
            let expected: [u8; 4] = serde_json::from_value(sample["rgba"].clone()).unwrap();
            let tolerance = case["tolerance"].as_u64().unwrap() as u8;
            assert!(rgba.iter().zip(expected).all(|(actual, expected)| actual.abs_diff(expected) <= tolerance),
                "{name} at [{x}, {y}]: {rgba:?} vs {expected:?}");
        }
        render_window(&snapshot, semio_ui_locale::Locale::En).expect("coded image scene admission");
    }
}
