//! 📤️ Converts the first resolved Semio RGBA8 frame into an explicit bottom-up Direct RGB24 BMP
//! v3 profile. Alpha and later frames have no representation in that target profile.

use crate::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use {semio_framework_plugin::ArtifactSerializer,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_bmp::{standards::v_v3::subsets::any::io::bmp_direct_rgb24_from_rgba8, BmpSnapshot};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bmp", standard: StandardId("v3"), subset: SubsetId::ANY };

fn metadata_i32(from: &SemioImageSnapshot, key: &str) -> Result<i32, store::PackError> {
    from.metadata
        .iter()
        .find(|entry| entry.key == key)
        .map(|entry| entry.value.parse::<i32>().map_err(|_| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("semio/image→bmp: metadata {key} must be a signed 32-bit integer")))))
        .transpose()
        .map(|value| value.unwrap_or(0))
}

//#region 🔖️Serializer
pub struct SemioImageToBmp;

impl ArtifactSerializer for SemioImageToBmp {
    type From = SemioImageSnapshot;
    type Into = BmpSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let frame = from.frames.first().ok_or_else(|| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "semio/image→bmp: no frames to export")))?;
        let expected = usize::try_from(from.width)
            .ok()
            .and_then(|width| usize::try_from(from.height).ok().and_then(|height| width.checked_mul(height)))
            .and_then(|pixels| pixels.checked_mul(4))
            .ok_or_else(|| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, "semio/image→bmp: frame dimensions overflow address space")))?;
        if frame.rgba8.len() != expected {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "semio/image→bmp: frame pixel length does not match width*height*4")));
        }
        let x_pixels_per_meter = metadata_i32(from, "xPixelsPerMeter")?;
        let y_pixels_per_meter = metadata_i32(from, "yPixelsPerMeter")?;
        bmp_direct_rgb24_from_rgba8(from.width, from.height, &frame.rgba8, x_pixels_per_meter, y_pixels_per_meter)
            .map_err(|failure| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("semio/image→bmp: {failure}"))))
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
