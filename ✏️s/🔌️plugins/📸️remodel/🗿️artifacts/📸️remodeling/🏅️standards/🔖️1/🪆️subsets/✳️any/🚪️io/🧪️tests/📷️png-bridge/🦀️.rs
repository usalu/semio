use super::{png_bytes_from_semio_image, scene_from_png_bytes, semio_image_from_png_bytes};
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::remodeling_asset_chunk_source;

const PNG_FIXTURE: &[u8] = include_bytes!("../🧫️fixtures/📷️two-by-two.png");

#[test]
fn png_bridge_preserves_the_independent_fixture_pixels() {
    let decoded = semio_image_from_png_bytes(PNG_FIXTURE).unwrap();
    assert_eq!((decoded.width, decoded.height), (2, 2));
    assert_eq!(decoded.frames[0].rgba8, [255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255]);
    let encoded = png_bytes_from_semio_image(&decoded).unwrap();
    assert_eq!(semio_image_from_png_bytes(&encoded).unwrap(), decoded);
}

#[test]
fn png_import_publishes_owned_frame_bytes_for_reconstruction() {
    let scene = scene_from_png_bytes(PNG_FIXTURE).unwrap();
    let frame = &scene.streams[0].frames[0];
    let content = remodeling_asset_chunk_source(&scene, &frame.asset_id).unwrap();
    assert_eq!((content.width, content.height, content.mime.as_str()), (2, 2, "image/png"));
    let bytes: Vec<_> = content.leaves.iter().flat_map(|leaf| leaf.iter().copied()).collect();
    assert_eq!(bytes, PNG_FIXTURE);
}
