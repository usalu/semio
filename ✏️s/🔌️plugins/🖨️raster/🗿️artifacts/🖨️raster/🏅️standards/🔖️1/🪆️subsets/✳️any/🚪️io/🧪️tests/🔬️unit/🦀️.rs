use super::*;

#[semio_framework_async_macros::async_test]
async fn raster_image_layer_and_asset_builds_a_pixel_layer_and_matching_asset() {
    let (asset_id, asset, layer) = raster_image_layer_and_asset("aGVsbG8=");
    assert_eq!(asset.data, b"hello".to_vec());
    let RasterLayerNode::Pixel { image_key, .. } = &layer else { panic!("expected pixel layer") };
    assert_eq!(image_key.as_deref(), Some(asset_id.as_str()));
}

/// 🧹️ Every fixture document below owns a populated asset pool (one `semio/image` child minted
/// through the real png funnel), so it must reach the artifact's own retirement seam instead of
/// `RasterOwnedMap`'s fail-closed `Drop`.
fn retire(document: RasterSnapshot) {
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);
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
async fn composite_applies_persisted_mask_assets_and_linked_transforms() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧬️schema/🧫️fixtures/🎭️mask/🔣️.json")).unwrap();
    let pixels:Vec<u8>=fixture["maskImage"]["pixels"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect();
    let image=SemioImageSnapshot {schema:STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(),width:3,height:1,colorspace:SemioColorspace::Rgba,bit_depth:8,frames:vec![SemioImageFrame {delay_ms:0,rgba8:pixels}],icc:None,metadata:Vec::new()};
    let bytes=png_bytes_from_semio_image(&image).unwrap();
    for case in fixture["compositing"].as_array().unwrap() {
        let mut document=document_with_solid_layer(255,0,0,255,3,1);
        let asset=crate::mint_raster_asset_child("mask-alpha",&RasterImageAsset {mime:"image/png".into(),data:bytes.clone()});
        document.assets.insert("mask-alpha".into(),asset).unwrap();
        let mask=crate::RasterLayerMask {enabled:case["enabled"].as_bool().unwrap(),linked:case["linked"].as_bool().unwrap(),invert:case["invert"].as_bool().unwrap(),width:Some(3),height:Some(1),image_key:Some("mask-alpha".into()),transform:RasterTransform {x:case["maskX"].as_f64().unwrap(),..RasterTransform::default()}};
        if case["group"].as_bool().unwrap() {
            let mut children=std::mem::take(&mut document.layers);let mut second=children[0].clone();if let RasterLayerNode::Pixel {id,..}=&mut second{id.push_str("-second");}children.push(second);
            let mut group=crate::standards::v1::subsets::any::schema::create_layer_of_kind("group");
            if let RasterLayerNode::Group {children:target,mask:target_mask,transform,opacity,..}=&mut group {*target=children;*target_mask=Some(mask);transform.x=case["layerX"].as_f64().unwrap();*opacity=case["opacity"].as_f64().unwrap() as f32;}
            document.layers.push(group);
        }else if let RasterLayerNode::Pixel {mask:target,transform,opacity,..}=&mut document.layers[0] {*target=Some(mask);transform.x=case["layerX"].as_f64().unwrap();*opacity=case["opacity"].as_f64().unwrap() as f32;}
        let mut job=raster_composite_job(&document).unwrap();let preparation=job.advance(1).unwrap();job.cancel();
        let result=raster_composite_image(&document);retire(document);
        assert_eq!(preparation.completed,1);
        assert_eq!(preparation.total,if case["group"].as_bool().unwrap(){15}else{6}+if case["enabled"].as_bool().unwrap(){3}else{0},"mask preparation belongs to the bounded export job");
        let alpha:Vec<u8>=result.unwrap().frames[0].rgba8.chunks_exact(4).map(|p|p[3]).collect();
        let expected:Vec<u8>=case["alpha"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect();
        assert_eq!(alpha,expected,"{}",case["name"]);
    }
}

#[semio_framework_async_macros::async_test]
async fn composite_flattens_a_pixel_layer_back_to_its_own_canvas() {
    let document = document_with_solid_layer(10, 20, 30, 255, 4, 2);
    let composite = raster_composite_image(&document).expect("composite");
    assert_eq!((composite.width, composite.height), (4, 2));
    let frame = composite.frames.first().expect("one frame");
    assert_eq!(frame.rgba8.len(), 4 * 2 * 4);
    assert_eq!(&frame.rgba8[..4], &[10, 20, 30, 255]);
    retire(document);
}

#[semio_framework_async_macros::async_test]
async fn composite_preserves_centered_pixels_at_negative_coordinates() {
    let mut document=document_with_solid_layer(12,34,56,255,3,2);
    if let RasterLayerNode::Pixel {transform,..}=&mut document.layers[0] {transform.x=-40.0;transform.y=-20.0;}
    let composite=raster_composite_image(&document).unwrap();
    assert_eq!((composite.width,composite.height),(3,2));
    assert_eq!(composite.frames[0].rgba8,[12,34,56,255].repeat(6));
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);
}

#[semio_framework_async_macros::async_test]
async fn composite_applies_a_visible_brightness_adjustment() {
    let mut document = document_with_solid_layer(64, 64, 64, 255, 2, 2);
    let mut adjustment=crate::standards::v1::subsets::any::schema::create_layer_of_kind("adjustment");
    if let RasterLayerNode::Adjustment {params,..}=&mut adjustment {params.insert("brightness".into(),dsl::DslValue::float(0.2)).unwrap();}
    document.layers.push(adjustment);
    let result=raster_composite_image(&document);
    retire(document);
    assert_eq!(result.expect("adjusted composite").frames[0].rgba8,[115,115,115,255].repeat(4));
}

#[semio_framework_async_macros::async_test]
async fn composite_applies_group_opacity_once_after_overlapping_children() {
    let mut document=document_with_solid_layer(255,0,0,255,2,2);
    let mut children=std::mem::take(&mut document.layers);
    let mut second=children[0].clone();if let RasterLayerNode::Pixel {id,..}=&mut second{id.push_str("-second");}children.push(second);
    let mut group=crate::standards::v1::subsets::any::schema::create_layer_of_kind("group");
    if let RasterLayerNode::Group {children:target,opacity,..}=&mut group {*target=children;*opacity=0.5;}
    document.layers.push(group);
    let result=raster_composite_image(&document);
    retire(document);
    assert_eq!(result.unwrap().frames[0].rgba8,[255,0,0,128].repeat(4));
}

#[semio_framework_async_macros::async_test]
async fn composite_svg_matches_adjusted_pixel_export() {
    let mut document=document_with_solid_layer(64,64,64,255,2,2);
    let mut adjustment=crate::standards::v1::subsets::any::schema::create_layer_of_kind("adjustment");
    if let RasterLayerNode::Adjustment {params,..}=&mut adjustment {params.insert("brightness".into(),dsl::DslValue::float(0.2)).unwrap();}
    document.layers.push(adjustment);
    let svg=raster_document_json_to_svg(&document);
    retire(document);
    let (svg,width,height)=svg.unwrap();
    let encoded=semio_framework_os::rasterize_svg_to_png_base64(&svg,width,height).unwrap();
    let png=base64_codec::base64_standard_decode(encoded.as_bytes()).unwrap();
    let image=semio_framework_pixels::decode_png(&png).unwrap();
    assert_eq!((image.width,image.height),(2,2));
    assert_eq!(image.pixels,[115,115,115,255].repeat(4));
}

#[semio_framework_async_macros::async_test]
async fn composite_refuses_an_unknown_blend_mode_with_a_reason() {
    let mut document = document_with_solid_layer(1, 2, 3, 255, 2, 2);
    if let Some(RasterLayerNode::Pixel { blend_mode, .. }) = document.layers.first_mut() {
        *blend_mode = "undefinedBlend".into();
    }
    let error = raster_composite_image(&document).expect_err("unknown blend modes must refuse");
    assert!(error.contains("unsupported blend mode"), "{error}");
    retire(document);
}

#[semio_framework_async_macros::async_test]
async fn composite_refuses_a_document_with_nothing_to_flatten() {
    let error = raster_composite_image(&crate::standards::v1::subsets::any::schema::empty_raster_snapshot()).expect_err("an empty document has no composite");
    assert!(error.contains("nothing to flatten"), "{error}");
}

#[semio_framework_async_macros::async_test]
async fn composite_exports_a_blank_pixel_layer_as_transparent() {
    let document=crate::standards::v1::subsets::any::schema::empty_raster_document();
    let result=raster_composite_image(&document);
    retire(document);
    let image=result.unwrap();
    assert_eq!((image.width,image.height),(512,512));
    assert!(image.frames[0].rgba8.iter().all(|v|*v==0));
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
    retire(reimported);
    retire(document);
}

/// 🧪️ A PNG export must carry the 8-byte PNG signature — the single sharpest proof that no leaf
/// is printing this artifact's own DSL text under a foreign extension any more.
#[semio_framework_async_macros::async_test]
async fn png_export_writes_a_real_png_signature() {
    let document = document_with_solid_layer(0, 128, 255, 255, 2, 2);
    let bytes = crate::io::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&document).expect("png export");
    assert_eq!(&bytes[..8], &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]);
    retire(document);
}

/// 🧫️ The one cross-language oracle both implementations of the bmp hop assert. The bun twin
/// (`🚪️io/🧪️tests/🟦️.ts`) writes and reads the SAME `bmpHex` from its own hand-written BMP v3
/// codec, so a drift between this plugin's Rust path (composite → stdio `SemioImageToBmp` →
/// stdio `encode_bmp`) and an independent second implementation fails in BOTH languages instead
/// of going unnoticed.
const BMP_PARITY_FIXTURES: &[&str] = &[include_str!("../../🧫️fixtures/🪟️solid-3x2.json"), include_str!("../../🧫️fixtures/🌈️gradient-5x3.json")];

fn parity_fixture(text: &str) -> (u32, u32, Vec<u8>, String) {
    use semio_s_artifact_stdio_json::schema::snapshot::{parse_json_text, JsonValue};
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
        retire(document);
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
        retire(document);
    }
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

#[semio_framework_async_macros::async_test]
async fn composite_source_preparation_copies_unique_images_in_bounded_grants() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧱️preparation/🔣️.json")).unwrap();
    let pixel:Vec<u8>=fixture["pixel"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect();
    let width=fixture["width"].as_u64().unwrap() as u32;let height=fixture["height"].as_u64().unwrap() as u32;
    let mut document=document_with_solid_layer(pixel[0],pixel[1],pixel[2],pixel[3],width,height);
    let mut duplicate=document.layers[0].clone();if let RasterLayerNode::Pixel {id,..}=&mut duplicate {id.push_str("-copy");}document.layers.push(duplicate);
    let mut preparation=RasterStackPreparation::new(&document).unwrap();
    assert_eq!(preparation.images.len(),fixture["uniqueImages"].as_u64().unwrap() as usize);
    assert!(preparation.images.values().all(|image|image.pixels.is_empty()));
    assert!(!preparation.advance(&document,0).unwrap());
    for expected in fixture["completedPixels"].as_array().unwrap() {
        let expected=expected.as_u64().unwrap() as usize;
        let done=preparation.advance(&document,usize::MAX).unwrap();
        assert_eq!(preparation.images.values().map(|image|image.pixels.len()/4).sum::<usize>(),expected);
        assert_eq!(done,expected==(width*height) as usize);
    }
    let mut job=preparation.into_job().unwrap();while !job.advance(32768).unwrap().done {}
    assert_eq!(job.into_result().unwrap().image.pixels,pixel.repeat((width*height) as usize));
    let mut cancelled=RasterStackPreparation::new(&document).unwrap();cancelled.advance(&document,1).unwrap();cancelled.cancel();
    assert!(cancelled.images.is_empty());assert!(cancelled.layers.is_empty());
    assert!(cancelled.advance(&document,1).is_err());assert!(cancelled.into_job().is_err());
    let incomplete=RasterStackPreparation::new(&document).unwrap();assert!(incomplete.into_job().is_err());
    retire(document);
}
