use super::*;
use semio_s_artifact_stdio_bmp::standards::v_v3::subsets::any::io::{bmp_direct_rgb24_from_rgba8, decode_bmp};

fn sample_bmp() -> BmpSnapshot {
    bmp_direct_rgb24_from_rgba8(2, 1, &[255, 0, 0, 255, 0, 255, 0, 255], 2835, 2835).unwrap()
}

#[semio_framework_async_macros::async_test]
async fn maps_checked_direct_profile_pixels_and_resolution() {
    let semio = ::semio_framework_async::poll::resolve_ready(SemioImageFromBmp::deserialize(&sample_bmp())).expect("deserialize");
    assert_eq!((semio.width, semio.height), (2, 1));
    assert_eq!(semio.colorspace, SemioColorspace::Rgb);
    assert_eq!(semio.bit_depth, 8);
    assert_eq!(semio.frames[0].rgba8, vec![255, 0, 0, 255, 0, 255, 0, 255]);
    assert!(semio.metadata.iter().any(|entry| entry.key == "bmp.profile" && entry.value == "directRgb24"));
    assert!(semio.metadata.iter().any(|entry| entry.key == "bmp.bitsPerPixel" && entry.value == "24"));
    assert!(semio.metadata.iter().any(|entry| entry.key == "xPixelsPerMeter" && entry.value == "2835"));
}

#[semio_framework_async_macros::async_test]
async fn records_actual_indexed_profile_while_resolving_rgba8() {
    let bmp = decode_bmp(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️canonical-byte-authority/indexed-rgb8-duplicate-palette.bmp"))).unwrap();
    let semio = ::semio_framework_async::poll::resolve_ready(SemioImageFromBmp::deserialize(&bmp)).expect("deserialize");
    assert_eq!(semio.colorspace, SemioColorspace::Indexed);
    assert_eq!(semio.bit_depth, 8);
    assert_eq!(semio.frames[0].rgba8.len(), semio.width as usize * semio.height as usize * 4);
    assert!(semio.metadata.iter().any(|entry| entry.key == "bmp.profile" && entry.value == "indexedRgb8"));
    assert!(semio.metadata.iter().any(|entry| entry.key == "bmp.bitsPerPixel" && entry.value == "8"));
}
