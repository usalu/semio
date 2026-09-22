use super::*;
use crate::standards::v1::subsets::any::schema::mutations::binary::unit_tests::retirement::retire_raster_snapshot;

/// 📄️ The boot document is the committed Semio-logo carrier read through the artifact's own text
/// codec — not the empty scaffold, and not a Rust restatement of the `.dsl.semio` bytes.
#[semio_framework_async_macros::async_test]
async fn default_document_boots_on_the_semio_demo_carrier() {
    let document = default_raster_document();
    assert_ne!(document, empty_raster_document(), "the boot document must not fall back to the empty scaffold");
    assert_eq!(document.id, "semio-demo");
    assert_eq!(document.title.as_deref(), Some("Semio Raster Demo"));
    assert_eq!(document.layers.len(), 2, "the Semio logo carries a backdrop pixel layer and a brighten adjustment layer");
    assert_eq!(layer_node_id(&document.layers[0]), "backdrop");
    let RasterLayerNode::Adjustment { adjustment_kind, .. } = &document.layers[1] else {
        panic!("the second layer is the brighten adjustment");
    };
    assert_eq!(adjustment_kind, "brightnessContrast");
    retire_raster_snapshot(document);
}

/// 📚️ `raster_example_document` resolves exactly the ids this subset registers, and nothing else.
#[semio_framework_async_macros::async_test]
async fn only_a_registered_example_id_resolves_to_a_document() {
    let registered = raster_example_document(crate::examples::art_raster_demo::ID).expect("the demo example id resolves");
    let expected = default_raster_document();
    assert_eq!(registered, expected);
    assert!(raster_example_document("not-a-real-example").is_none());
    retire_raster_snapshot(registered);
    retire_raster_snapshot(expected);
}

/// 🖼️ LAW: the curated demo ships REAL media, and its backdrop layer declares exactly that media's own
/// pixel size.
///
/// The committed emblem used to be a 75-byte 2×2 RGBA swatch while the carrier declared a 1024×1024
/// backdrop, so even a perfectly working pipeline drew four pixels: the play pane looked identical to
/// the blank one the missing `paint-2d` lane route produced, which is what kept that defect hidden for
/// three sessions (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP). A curated example is content
/// the definition of done names — "VISIBLE correct content" — so both halves are pinned here: the
/// asset is a real image, and the declaration agrees with it.
///
/// The size is read straight out of the PNG's IHDR (bytes 16..24 of any PNG, big-endian) rather than
/// through a decoder, so this law needs no codec and cannot be satisfied by a re-encode.
#[semio_framework_async_macros::async_test]
async fn the_demo_carrier_ships_real_media_sized_exactly_as_its_backdrop_declares() {
    let asset = crate::examples::art_raster_demo::emblem_image_asset();
    assert_eq!(asset.mime, "image/png");
    assert!(asset.data.len() > 8 * 1024, "the curated demo ships real media, not a placeholder swatch: {} bytes", asset.data.len());
    assert!(asset.data.len() >= 24, "a PNG carries its IHDR in the first 24 bytes");
    let width = u32::from_be_bytes([asset.data[16], asset.data[17], asset.data[18], asset.data[19]]);
    let height = u32::from_be_bytes([asset.data[20], asset.data[21], asset.data[22], asset.data[23]]);
    assert!(width >= 256 && height >= 256, "the curated media is large enough to see: {width}x{height}");

    let document = default_raster_document();
    let RasterLayerNode::Pixel { width: layer_width, height: layer_height, image_key, .. } = &document.layers[0] else {
        panic!("the carrier's first layer is the backdrop pixel layer");
    };
    assert_eq!(image_key.as_deref(), Some("semio-emblem"));
    assert_eq!((*layer_width, *layer_height), (Some(width), Some(height)), "the carrier's backdrop declares the committed emblem's own size");
    retire_raster_snapshot(document);
}
