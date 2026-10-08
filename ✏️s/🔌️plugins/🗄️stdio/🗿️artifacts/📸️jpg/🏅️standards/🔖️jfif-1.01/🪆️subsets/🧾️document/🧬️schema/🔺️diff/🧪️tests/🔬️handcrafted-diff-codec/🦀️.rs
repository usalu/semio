use super::*;
use protocol::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn segment(marker: u8, data: Vec<u8>) -> JpgSegment {
    JpgSegment { marker, data }
}

/// 🌱 Builds distinct authored image dimensions, raster, JFIF metadata and ordered APP/COM content.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn snap_a() -> JpgSnapshot {
    JpgSnapshot { schema: "stdio.jpg".into(), image: crate::schema::snapshot::JpgImage { width: 4,height: 4,pixels: vec![0u8; 16],jfif_version: (1, 1),jfif_density_units: JfifDensityUnits::PixelsPerInch,jfif_x_density: 72,jfif_y_density: 72,jfif_thumbnail: Some(JfifThumbnail { width: 2, height: 1, rgb_data: vec![1, 2, 3, 4, 5, 6] }),other_segments: vec![segment(0xFE, vec![1, 2, 3]), segment(0xE1, vec![9, 9])] } }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn snap_b() -> JpgSnapshot {
    JpgSnapshot { schema: "stdio.jpg".into(), image: crate::schema::snapshot::JpgImage { width: 8,height: 6,pixels: vec![9u8; 12],jfif_version: (1, 2),jfif_density_units: JfifDensityUnits::Aspect,jfif_x_density: 1,jfif_y_density: 1,jfif_thumbnail: None,other_segments: vec![segment(0xFE, vec![4, 5, 6])] } }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn snap_c() -> JpgSnapshot {
    JpgSnapshot::default()
}

/// 🧪️ Checks text and binary laws for all nine semantic fields, both thumbnail transitions and segment collections.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let cases = demo_diff_cases();
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = JpgDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = JpgDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
