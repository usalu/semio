use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_uses_the_frozen_window_kit_kind_id() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn render_projects_the_primary_ifd_to_browser_png() {
    let document = crate::standards::v6_0::subsets::document::schema::demo_tiff_snapshot();
    let view = image_view(&document).expect("preview");
    assert_eq!((view.width, view.height, view.mime.as_str()), (3, 2, "image/png"));
    let png = crate::standards::v6_0::subsets::document::io::encode_tiff_page_png(&document, 0).expect("PNG");
    assert_eq!(view.base64, semio_s_artifact_stdio_contract::base64_standard(&png.bytes));
    let decoded = semio_framework_pixels::decode_png(&png.bytes).expect("PNG");
    assert_eq!((decoded.width, decoded.height), (3, 2));
    assert_eq!(decoded.pixels, crate::standards::v6_0::subsets::document::io::decode_tiff_page_rgba(&document, 0).expect("RGBA").pixels);
    render(&document, Locale::En).expect("image window");
}

#[semio_framework_async_macros::async_test]
async fn render_projects_an_uncompressed_tiled_primary_ifd() {
    use crate::schema::snapshot::{TiffByteOrder, TiffFieldType, TiffIfd, TiffStorage, TiffStorageKind, TiffTag, TiffValues, TAG_BITS_PER_SAMPLE, TAG_COMPRESSION, TAG_IMAGE_LENGTH, TAG_IMAGE_WIDTH, TAG_PHOTOMETRIC, TAG_SAMPLES_PER_PIXEL, TAG_TILE_LENGTH, TAG_TILE_WIDTH};
    let short = |tag, values| TiffTag { tag, values: TiffValues::Short(values) };
    let long = |tag, value| TiffTag { tag, values: TiffValues::Long(vec![value]) };
    let document = TiffSnapshot {
        schema: crate::STDIO_TIFF_DOCUMENT_SCHEMA.into(),
        byte_order: TiffByteOrder::LittleEndian,
        ifds: vec![TiffIfd {
            entries: vec![long(TAG_IMAGE_WIDTH, 16), long(TAG_IMAGE_LENGTH, 16), short(TAG_BITS_PER_SAMPLE, vec![8, 8, 8]), short(TAG_COMPRESSION, vec![1]), short(TAG_PHOTOMETRIC, vec![2]), short(TAG_SAMPLES_PER_PIXEL, vec![3]), long(TAG_TILE_WIDTH, 16), long(TAG_TILE_LENGTH, 16)],
            storage: TiffStorage { kind: TiffStorageKind::Tiles, offsets_kind: TiffFieldType::Long, byte_counts_kind: TiffFieldType::Long, chunks: vec![[12, 34, 56].repeat(16 * 16)] },
        }],
    };
    let view = image_view(&document).expect("tiled preview");
    assert_eq!((view.width, view.height, view.mime.as_str()), (16, 16, "image/png"));
    let node = render(&document, Locale::De).expect("tiled image window");
    assert_eq!(node.key.as_str(), ImageWindowKit::KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn unsupported_projection_keeps_the_artifact_mounted_with_a_localized_state() {
    let mut document = crate::standards::v6_0::subsets::document::schema::demo_tiff_snapshot();
    document.ifds[0].storage.kind = crate::standards::v6_0::subsets::document::schema::snapshot::TiffStorageKind::Tiles;
    assert!(image_view(&document).is_err(), "the malformed tiled fixture has no tile geometry");

    let node = render(&document, Locale::De).expect("unsupported display capability does not reject the artifact window");
    assert_eq!(node.key.as_str(), ImageWindowKit::KIND_ID);
    assert_eq!(node.children.len(), 1);
    let semio_framework_plugin::Component::Text(text) = &node.children[0].component else { panic!("explicit unavailable state") };
    assert_eq!(text.value.0.as_str(), "Vorschau nicht verfügbar");
}
