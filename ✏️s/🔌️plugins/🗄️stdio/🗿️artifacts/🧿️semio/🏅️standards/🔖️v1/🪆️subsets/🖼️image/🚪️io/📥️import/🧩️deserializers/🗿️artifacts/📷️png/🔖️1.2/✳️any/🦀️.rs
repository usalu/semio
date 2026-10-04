//! 📥️ PNG 1.2 to Semio image through an explicit checked RGBA8 derivative.

use crate::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot, STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_png::{schema::snapshot::PngColorType, PngSnapshot};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };

fn colorspace_from_png(color_type: PngColorType) -> SemioColorspace {
    match color_type {
        PngColorType::Grayscale => SemioColorspace::Grayscale,
        PngColorType::Rgb => SemioColorspace::Rgb,
        PngColorType::Palette => SemioColorspace::Indexed,
        PngColorType::GrayscaleAlpha => SemioColorspace::GrayscaleAlpha,
        PngColorType::Rgba => SemioColorspace::Rgba,
    }
}

pub struct SemioImageFromPng;

impl ArtifactDeserializer for SemioImageFromPng {
    type From = PngSnapshot;
    type Into = SemioImageSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let projection = semio_s_artifact_stdio_png::io::project_png(&from.bytes).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))?;
        let expected = usize::try_from(projection.width).ok().and_then(|width| usize::try_from(projection.height).ok().and_then(|height| width.checked_mul(height))).and_then(|pixels| pixels.checked_mul(4)).ok_or_else(|| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "png→semio/image: raster extent overflow")))?;
        if projection.pixels.len() != expected { return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "png→semio/image: decoded RGBA8 length does not match IHDR"))); }
        let metadata = projection.text_chunks.iter().map(|text| SemioImageMetadataEntry { key: text.keyword.clone(), value: text.value.clone() }).collect();
        Ok(SemioImageSnapshot {
            schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(),
            width: projection.width,
            height: projection.height,
            colorspace: colorspace_from_png(projection.color_type),
            bit_depth: projection.bit_depth,
            frames: vec![SemioImageFrame { delay_ms: 0, rgba8: projection.pixels }],
            icc: None,
            metadata,
        })
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
