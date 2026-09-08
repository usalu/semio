
use super::*;

#[semio_framework_async_macros::async_test]
async fn raster_image_layer_and_asset_builds_a_pixel_layer_and_matching_asset() {
    let (asset_id, asset, layer) = raster_image_layer_and_asset("aGVsbG8=");
    assert_eq!(asset.data, b"hello".to_vec());
    let RasterLayerNode::Pixel { image_key, .. } = &layer else { panic!("expected pixel layer") };
    assert_eq!(image_key.as_deref(), Some(asset_id.as_str()));
}

/// 🧪️ Builds a real one-pixel-layer document whose asset child carries genuinely PNG-encoded
/// content (through the same `mint_raster_asset_child` funnel every mutation uses), so the
/// composite tests below exercise the real materialization path and never a fabricated handle.
fn document_with_solid_layer(red: u8, green: u8, blue: u8, alpha: u8, width: u32, height: u32) -> RasterSnapshot {
    let pixel_count = width as usize * height as usize;
    let mut rgba8 = Vec::with_capacity(pixel_count * 4);
    for _ in 0..pixel_count {
        rgba8.extend_from_slice(&[red, green, blue, alpha]);
    }
    let image = SemioImageSnapshot { schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(), width, height, colorspace: SemioColorspace::Rgba, bit_depth: 8, frames: vec![SemioImageFrame { delay_ms: 0, rgba8 }], icc: None, metadata: Vec::new() };
    raster_document_from_semio_image(&image, "fixture", "Fixture").expect("fixture document")
}

#[semio_framework_async_macros::async_test]
async fn composite_flattens_a_pixel_layer_back_to_its_own_canvas() {
    let document = document_with_solid_layer(10, 20, 30, 255, 4, 2);
    let composite = raster_composite_image(&document).expect("composite");
    assert_eq!((composite.width, composite.height), (4, 2));
    let frame = composite.frames.first().expect("one frame");
    assert_eq!(frame.rgba8.len(), 4 * 2 * 4);
    assert_eq!(&frame.rgba8[..4], &[10, 20, 30, 255]);
}

#[semio_framework_async_macros::async_test]
async fn composite_refuses_a_visible_adjustment_layer_with_a_reason() {
    let mut document = document_with_solid_layer(1, 2, 3, 255, 2, 2);
    document.layers.push(crate::schema::create_layer_of_kind("adjustment"));
    let error = raster_composite_image(&document).expect_err("adjustment layers must refuse");
    assert!(error.contains("adjustment layer"), "{error}");
}

#[semio_framework_async_macros::async_test]
async fn composite_refuses_an_unknown_blend_mode_with_a_reason() {
    let mut document = document_with_solid_layer(1, 2, 3, 255, 2, 2);
    if let Some(RasterLayerNode::Pixel { blend_mode, .. }) = document.layers.first_mut() {
        *blend_mode = "colorDodge".into();
    }
    let error = raster_composite_image(&document).expect_err("unknown blend modes must refuse");
    assert!(error.contains("unsupported blend mode"), "{error}");
}

#[semio_framework_async_macros::async_test]
async fn composite_refuses_a_document_with_nothing_to_flatten() {
    let error = raster_composite_image(&crate::schema::empty_raster_snapshot()).expect_err("an empty document has no composite");
    assert!(error.contains("nothing to flatten"), "{error}");
}

/// 🧪️ The real end-to-end pixel hop this packet exists for: composite → stdio's own
/// `semio/image` → `bmp` serializer → stdio's own `encode_bmp`, then all the way back. A BMP v3
/// file starts with `BM`, and the round trip must recover the same RGB (alpha is the format's
/// own documented loss).
#[semio_framework_async_macros::async_test]
async fn bmp_export_writes_real_bytes_that_import_reads_back() {
    let document = document_with_solid_layer(200, 100, 50, 255, 3, 2);
    let bytes = crate::io::export::serializers::artifacts::bmp::v_v3::any::serialize_bytes(&document).expect("bmp export");
    assert_eq!(&bytes[..2], b"BM", "real BITMAPFILEHEADER magic, not DSL text");
    let reimported = crate::io::import::deserializers::artifacts::bmp::v_v3::any::deserialize_bytes(&bytes).expect("bmp import");
    let composite = raster_composite_image(&reimported).expect("composite of the reimported document");
    assert_eq!((composite.width, composite.height), (3, 2));
    assert_eq!(&composite.frames[0].rgba8[..3], &[200, 100, 50]);
}

/// 🧪️ A PNG export must carry the 8-byte PNG signature — the single sharpest proof that no leaf
/// is printing this artifact's own DSL text under a foreign extension any more.
#[semio_framework_async_macros::async_test]
async fn png_export_writes_a_real_png_signature() {
    let document = document_with_solid_layer(0, 128, 255, 255, 2, 2);
    let bytes = crate::io::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&document).expect("png export");
    assert_eq!(&bytes[..8], &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]);
}

/// 🧫️ The one cross-language oracle both implementations of the bmp hop assert. The bun twin
/// (`🚪️io/🧪️tests/🟦️.ts`) writes and reads the SAME `bmpHex` from its own hand-written BMP v3
/// codec, so a drift between this plugin's Rust path (composite → stdio `SemioImageToBmp` →
/// stdio `encode_bmp`) and an independent second implementation fails in BOTH languages instead
/// of going unnoticed.
const BMP_PARITY_FIXTURES: &[&str] = &[include_str!("../🧫️fixtures/🪟️solid-3x2.json"), include_str!("../🧫️fixtures/🌈️gradient-5x3.json")];

fn parity_fixture(text: &str) -> (u32, u32, Vec<u8>, String) {
    use semio_s_artifact_stdio_json::schema::snapshot::{JsonValue, parse_json_text};
    let JsonValue::Object { members } = parse_json_text(text).expect("parity fixture is valid json") else { panic!("parity fixture root must be an object") };
    let member = |key: &str| members.iter().find(|entry| entry.key == key).map(|entry| entry.value.clone()).unwrap_or_else(|| panic!("parity fixture has no {key:?} member"));
    let number = |value: &JsonValue| match value {
        JsonValue::Number { lexeme } => lexeme.parse::<u32>().expect("parity fixture numbers are integers"),
        other => panic!("parity fixture expected a number, got {other:?}"),
    };
    let width = number(&member("width"));
    let height = number(&member("height"));
    let JsonValue::Array { items } = member("rgba8") else { panic!("parity fixture rgba8 must be an array") };
    let rgba8 = items.iter().map(|item| number(item) as u8).collect();
    let JsonValue::String { value: bmp_hex } = member("bmpHex") else { panic!("parity fixture bmpHex must be a string") };
    (width, height, rgba8, bmp_hex)
}

fn hex_of(bytes: &[u8]) -> String {
    bytes.iter().fold(String::with_capacity(bytes.len() * 2), |mut text, byte| {
        text.push_str(&format!("{byte:02x}"));
        text
    })
}

fn bytes_of(hex: &str) -> Vec<u8> {
    (0..hex.len() / 2).map(|index| u8::from_str_radix(&hex[index * 2..index * 2 + 2], 16).expect("parity fixture hex")).collect()
}

fn parity_document(width: u32, height: u32, rgba8: Vec<u8>) -> RasterSnapshot {
    let image = SemioImageSnapshot { schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(), width, height, colorspace: SemioColorspace::Rgba, bit_depth: 8, frames: vec![SemioImageFrame { delay_ms: 0, rgba8 }], icc: None, metadata: Vec::new() };
    raster_document_from_semio_image(&image, "parity", "Parity").expect("parity document")
}

/// 🧪️ The Rust bmp EXPORT must produce the exact bytes the TypeScript twin produces.
#[semio_framework_async_macros::async_test]
async fn bmp_export_matches_the_typescript_parity_fixture() {
    for text in BMP_PARITY_FIXTURES {
        let (width, height, rgba8, bmp_hex) = parity_fixture(text);
        let document = parity_document(width, height, rgba8);
        let bytes = crate::io::export::serializers::artifacts::bmp::v_v3::any::serialize_bytes(&document).expect("bmp export");
        assert_eq!(hex_of(&bytes), bmp_hex, "the Rust bmp writer drifted from the TypeScript twin");
    }
}

/// 🧪️ The Rust bmp IMPORT must recover the exact canvas the TypeScript twin recovers.
#[semio_framework_async_macros::async_test]
async fn bmp_import_matches_the_typescript_parity_fixture() {
    for text in BMP_PARITY_FIXTURES {
        let (width, height, rgba8, bmp_hex) = parity_fixture(text);
        let document = crate::io::import::deserializers::artifacts::bmp::v_v3::any::deserialize_bytes(&bytes_of(&bmp_hex)).expect("bmp import");
        let composite = raster_composite_image(&document).expect("composite of the imported document");
        assert_eq!((composite.width, composite.height), (width, height));
        assert_eq!(composite.frames[0].rgba8, rgba8, "the Rust bmp reader drifted from the TypeScript twin");
    }
}

/// 🧪️ Every hop this subset declines is declined with a SENTENCE, never with silently wrong
/// bytes and never with a bare "not implemented".
#[semio_framework_async_macros::async_test]
async fn declined_hops_refuse_with_a_reason() {
    let document = document_with_solid_layer(1, 2, 3, 255, 2, 2);
    let pdf_export = crate::io::export::serializers::artifacts::pdf::v1_4::any::serialize_bytes(&document).expect_err("pdf export is declined");
    assert!(pdf_export.contains("pdf export not supported for a raster document:"), "{pdf_export}");
    let pdf_import = crate::io::import::deserializers::artifacts::pdf::v1_4::any::deserialize_bytes(b"%PDF-1.4\n").expect_err("pdf import is declined");
    assert!(pdf_import.contains("pdf import not supported for a raster document:"), "{pdf_import}");
    let dwg_export = crate::io::export::serializers::artifacts::dwg::v_ac1018::any::serialize_bytes(&document).expect_err("dwg export is declined");
    assert!(dwg_export.contains("dwg export not supported for a raster document:"), "{dwg_export}");
}

/// 🧪️ The two advertised-kind lists must name only formats a leaf really encodes/decodes —
/// `negotiate_wire_format` picks workflow wires straight out of them.
#[semio_framework_async_macros::async_test]
async fn advertised_stdio_kinds_exclude_every_declined_hop() {
    assert!(!export_stdio_kinds().contains(&"stdio.pdf"), "pdf export is declined");
    assert!(!export_stdio_kinds().contains(&"stdio.dwg"), "dwg export is declined");
    assert!(!import_stdio_kinds().contains(&"stdio.pdf"), "pdf import is declined");
    assert!(import_stdio_kinds().contains(&"stdio.dwg"), "dwg import is real");
    for kind in ["stdio.bmp", "stdio.gif", "stdio.jpg", "stdio.json", "stdio.png", "stdio.svg", "stdio.tiff"] {
        assert!(export_stdio_kinds().contains(&kind), "{kind} export is real");
        assert!(import_stdio_kinds().contains(&kind), "{kind} import is real");
    }
    assert_eq!(crate::artifact_kind().export_stdio_kinds, export_stdio_kinds().to_vec());
    assert_eq!(crate::artifact_kind().import_stdio_kinds, import_stdio_kinds().to_vec());
}
