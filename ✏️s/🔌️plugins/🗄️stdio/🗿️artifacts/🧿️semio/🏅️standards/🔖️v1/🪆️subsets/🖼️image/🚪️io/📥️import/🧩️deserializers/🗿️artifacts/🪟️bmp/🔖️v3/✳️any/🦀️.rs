//! 📥️ Projects checked BMP v3 source bytes into a resolved Semio RGBA8 image while recording
//! the source profile. Palette identity, packed sample precision, padding, gaps, and trailers remain
//! BMP-only authority and cannot be reconstructed from this neutral image projection.

use crate::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot, STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA};
use {semio_framework_plugin::ArtifactDeserializer,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_bmp::schema::snapshot::{BmpRowOrder, BmpProfile};
use semio_s_artifact_stdio_bmp::schema::operations::bmp_rgba8_preview;
use semio_s_artifact_stdio_bmp::BmpSnapshot;

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bmp", standard: StandardId("v3"), subset: SubsetId::ANY };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };

fn colorspace_from_profile(profile: BmpProfile) -> SemioColorspace {
    if profile.is_indexed() { SemioColorspace::Indexed } else { SemioColorspace::Rgb }
}

//#region 🔖️Deserializer
pub struct SemioImageFromBmp;

impl ArtifactDeserializer for SemioImageFromBmp {
    type From = BmpSnapshot;
    type Into = SemioImageSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        from.validate().map_err(|failure| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, failure)))?;
        let layout = &from.image;
        let rgba8 = bmp_rgba8_preview(from).map_err(|failure| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("bmp→semio/image: {failure}"))))?;
        let mut metadata = vec![
            SemioImageMetadataEntry { key: "bmp.profile".into(), value: layout.profile.id().into() },
            SemioImageMetadataEntry { key: "bmp.bitsPerPixel".into(), value: layout.profile.bits_per_pixel().to_string() },
            SemioImageMetadataEntry {
                key: "bmp.rowOrder".into(),
                value: match layout.row_order {
                    BmpRowOrder::TopDown => "topDown".into(),
                    BmpRowOrder::BottomUp => "bottomUp".into(),
                },
            },
        ];
        if layout.x_pixels_per_meter != 0 {
            metadata.push(SemioImageMetadataEntry { key: "xPixelsPerMeter".into(), value: layout.x_pixels_per_meter.to_string() });
        }
        if layout.y_pixels_per_meter != 0 {
            metadata.push(SemioImageMetadataEntry { key: "yPixelsPerMeter".into(), value: layout.y_pixels_per_meter.to_string() });
        }
        Ok(SemioImageSnapshot {
            schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(),
            width: layout.width,
            height: layout.height,
            colorspace: colorspace_from_profile(layout.profile),
            bit_depth: 8,
            frames: vec![SemioImageFrame { delay_ms: 0, rgba8 }],
            icc: None,
            metadata,
        })
    }
}
//#endregion 🔖️Deserializer

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
