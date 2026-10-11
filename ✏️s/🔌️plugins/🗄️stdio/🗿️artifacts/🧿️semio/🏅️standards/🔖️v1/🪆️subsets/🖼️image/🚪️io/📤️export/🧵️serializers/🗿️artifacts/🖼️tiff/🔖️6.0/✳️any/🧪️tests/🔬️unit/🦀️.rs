use super::*;
use crate::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_semio() -> SemioImageSnapshot {
    SemioImageSnapshot {
        width: 2,
        height: 1,
        colorspace: SemioColorspace::Rgb,
        bit_depth: 8,
        frames: vec![SemioImageFrame { delay_ms: 0, rgba8: vec![255, 0, 0, 255, 0, 255, 0, 255] }],
        icc: None,
        metadata: vec![SemioImageMetadataEntry { key: "270".into(), value: "semio fixture".into() }],
        ..SemioImageSnapshot::default()
    }
}

/// 🧪️ Real round trip through tiff's own codec: the four authored samples per pixel and the description tag survive, and
/// the decoded page projects back to the frame's RGBA8.
#[semio_framework_async_macros::async_test]
async fn real_byte_round_trip_through_tiff_codec() {
    let semio = sample_semio();
    let tiff = ::semio_framework_async::poll::resolve_ready(SemioImageToTiff::serialize(&semio)).expect("serialize");
    let bytes = semio_s_artifact_stdio_tiff::standards::v6_0::subsets::document::io::encode_tiff(&tiff).expect("encode real tiff bytes");
    let decoded = semio_s_artifact_stdio_tiff::standards::v6_0::subsets::document::io::decode_tiff(&bytes).expect("decode real tiff bytes");
    assert_eq!(decoded.width(), Some(2));
    assert_eq!(decoded.height(), Some(1));
    let page = semio_s_artifact_stdio_tiff::standards::v6_0::subsets::document::io::decode_tiff_page_rgba(&decoded, 0).expect("project the decoded page");
    assert_eq!(page.pixels, semio.frames[0].rgba8, "RGBA must survive exactly");
    assert!(decoded.ifds[0].entries.iter().any(|t| t.tag == 270 && matches!(&t.values, TiffValues::Ascii(s) if s.as_slice() == ["semio fixture"])));
}
