use super::*;
use semio_s_artifact_stdio_tiff::schema::snapshot::{TiffIfd, TiffSampleBlock, TiffTag, TiffWord64};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_tiff() -> TiffSnapshot {
    TiffSnapshot {
        ifds: vec![TiffIfd {
            entries: vec![
                TiffTag { tag: TAG_IMAGE_WIDTH, values: TiffValues::Long(vec![2]) },
                TiffTag { tag: TAG_IMAGE_LENGTH, values: TiffValues::Long(vec![1]) },
                TiffTag { tag: TAG_BITS_PER_SAMPLE, values: TiffValues::Short(vec![8, 8, 8]) },
                TiffTag { tag: TAG_PHOTOMETRIC, values: TiffValues::Short(vec![2]) },
                TiffTag { tag: 270, values: TiffValues::Ascii(vec!["semio fixture".into()]) },
                TiffTag { tag: TAG_SAMPLES_PER_PIXEL, values: TiffValues::Short(vec![3]) },
            ],
            blocks: vec![TiffSampleBlock { x: 0, y: 0, width: 2, height: 1, channels: 3, samples: [255u64, 0, 0, 0, 255, 0].into_iter().map(TiffWord64::from_word).collect() }],
        }],
        ..TiffSnapshot::default()
    }
}

#[semio_framework_async_macros::async_test]
async fn maps_pixels_and_description_tag() {
    let semio = ::semio_framework_async::poll::resolve_ready(SemioImageFromTiff::deserialize(&sample_tiff())).expect("deserialize");
    assert_eq!(semio.width, 2);
    assert_eq!(semio.height, 1);
    assert_eq!(semio.colorspace, SemioColorspace::Rgb);
    assert_eq!(semio.bit_depth, 8);
    assert_eq!(semio.frames[0].rgba8, vec![255, 0, 0, 255, 0, 255, 0, 255]);
    assert!(semio.metadata.iter().any(|m| m.key == "270" && m.value == "semio fixture"));
}
