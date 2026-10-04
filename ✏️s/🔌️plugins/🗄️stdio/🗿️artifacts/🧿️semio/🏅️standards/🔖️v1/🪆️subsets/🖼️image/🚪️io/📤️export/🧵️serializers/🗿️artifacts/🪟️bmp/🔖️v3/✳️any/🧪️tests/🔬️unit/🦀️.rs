use super::*;
use crate::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry};
use semio_s_artifact_stdio_bmp::standards::v_v3::subsets::any::io::{bmp_layout, bmp_rgba8_preview, decode_bmp, encode_bmp, BmpProfile};

fn sample_semio() -> SemioImageSnapshot {
    SemioImageSnapshot {
        width: 2,
        height: 1,
        colorspace: SemioColorspace::Rgb,
        bit_depth: 8,
        frames: vec![SemioImageFrame { delay_ms: 0, rgba8: vec![255, 0, 0, 255, 0, 255, 0, 255] }],
        icc: None,
        metadata: vec![SemioImageMetadataEntry { key: "xPixelsPerMeter".into(), value: "2835".into() }],
        ..SemioImageSnapshot::default()
    }
}

#[semio_framework_async_macros::async_test]
async fn writes_an_explicit_checked_direct_rgb24_profile() {
    let semio = sample_semio();
    let bmp = ::semio_framework_async::poll::resolve_ready(SemioImageToBmp::serialize(&semio)).expect("serialize");
    let layout = bmp_layout(&bmp).unwrap();
    assert_eq!(layout.profile, BmpProfile::DirectRgb24);
    assert_eq!((layout.width, layout.height), (semio.width, semio.height));
    assert_eq!(layout.x_pixels_per_meter, 2835);
    let bytes = encode_bmp(&bmp).expect("encode exact BMP bytes");
    let reopened = decode_bmp(&bytes).expect("reopen exact BMP bytes");
    assert_eq!(bmp_rgba8_preview(&reopened).unwrap(), vec![255, 0, 0, 255, 0, 255, 0, 255]);
}

#[semio_framework_async_macros::async_test]
async fn rejects_invalid_resolution_metadata_instead_of_silently_zeroing_it() {
    let mut semio = sample_semio();
    semio.metadata[0].value = "invalid".into();
    let failure = ::semio_framework_async::poll::resolve_ready(SemioImageToBmp::serialize(&semio)).unwrap_err();
    assert!(format!("{failure:?}").contains("signed 32-bit integer"));
}

#[semio_framework_async_macros::async_test]
async fn rejects_zero_and_partial_alpha_that_direct_rgb24_cannot_represent() {
    for alpha in [0, 128] {
        let mut semio = sample_semio();
        semio.frames[0].rgba8[3] = alpha;
        let failure = ::semio_framework_async::poll::resolve_ready(SemioImageToBmp::serialize(&semio)).unwrap_err();
        assert!(format!("{failure:?}").contains("cannot represent nonopaque"));
    }
}
