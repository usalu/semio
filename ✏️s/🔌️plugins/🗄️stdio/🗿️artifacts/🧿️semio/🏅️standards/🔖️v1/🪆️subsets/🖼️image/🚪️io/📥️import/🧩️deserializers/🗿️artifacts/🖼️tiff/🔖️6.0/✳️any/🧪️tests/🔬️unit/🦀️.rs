
use super::*;
use semio_s_artifact_stdio_tiff::schema::snapshot::{TiffFieldType, TiffIfd, TiffTag};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_tiff() -> TiffSnapshot {
    TiffSnapshot {
        ifds: vec![TiffIfd {
            pixels: Vec::new(),
            entries: vec![
                TiffTag { tag: TAG_IMAGE_WIDTH, kind: TiffFieldType::Long, values: TiffValues::Long(vec![2]) },
                TiffTag { tag: TAG_IMAGE_LENGTH, kind: TiffFieldType::Long, values: TiffValues::Long(vec![1]) },
                TiffTag { tag: TAG_BITS_PER_SAMPLE, kind: TiffFieldType::Short, values: TiffValues::Short(vec![8]) },
                TiffTag { tag: TAG_PHOTOMETRIC, kind: TiffFieldType::Short, values: TiffValues::Short(vec![2]) },
                TiffTag { tag: TAG_SAMPLES_PER_PIXEL, kind: TiffFieldType::Short, values: TiffValues::Short(vec![3]) },
                TiffTag { tag: 270, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("semio fixture".into()) },
            ],
        }],
        pixels: vec![255, 0, 0, 255, 0, 255, 0, 255],
        ..TiffSnapshot::default()
    }
}

#[semio_framework_async_macros::async_test]
async fn maps_pixels_and_description_tag() {
    let semio = semio_framework_plugin::resolve_ready(SemioImageFromTiff::deserialize(&sample_tiff())).expect("deserialize");
    assert_eq!(semio.width, 2);
    assert_eq!(semio.height, 1);
    assert_eq!(semio.colorspace, SemioColorspace::Rgb);
    assert_eq!(semio.bit_depth, 8);
    assert_eq!(semio.frames[0].rgba8, vec![255, 0, 0, 255, 0, 255, 0, 255]);
    assert!(semio.metadata.iter().any(|m| m.key == "270" && m.value == "semio fixture"));
}
