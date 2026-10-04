//! 📤️ Semio image to a newly authored PNG 1.2 RGBA8 source.

use crate::standards::v1::subsets::image::schema::snapshot::{SemioImageSnapshot};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_png::{
    io::PngProjection,
    schema::snapshot::{PngChunkMarker, PngColorType, PngTextChunk, PngTextKind},
    PngSnapshot,
};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

pub struct SemioImageToPng;

impl ArtifactSerializer for SemioImageToPng {
    type From = SemioImageSnapshot;
    type Into = PngSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        if from.frames.len() != 1 { return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "semio/image→png: PNG 1.2 requires exactly one frame"))); }
        if from.icc.is_some() { return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "semio/image→png: iCCP authoring is not available"))); }
        let frame = &from.frames[0];
        let expected = usize::try_from(from.width).ok().and_then(|width| usize::try_from(from.height).ok().and_then(|height| width.checked_mul(height))).and_then(|pixels| pixels.checked_mul(4)).ok_or_else(|| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "semio/image→png: raster extent overflow")))?;
        if frame.rgba8.len() != expected { return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "semio/image→png: frame RGBA8 length does not match geometry"))); }
        let text_chunks: Vec<PngTextChunk> = from.metadata.iter().map(|entry| PngTextChunk { keyword: entry.key.clone(), value: entry.value.clone(), kind: PngTextKind::Text, ..Default::default() }).collect();
        let mut chunk_order = vec![PngChunkMarker::Ihdr];
        chunk_order.extend((0..text_chunks.len()).map(|index| PngChunkMarker::Text { index }));
        chunk_order.extend([PngChunkMarker::Idat, PngChunkMarker::Iend]);
        let projection = PngProjection {
            width: from.width, height: from.height, bit_depth: 8, color_type: PngColorType::Rgba, interlace: false,
            plte: None, trns: None, gama: None, chrm: None, srgb: None, phys: None, time: None, bkgd: None,
            text_chunks, pixels: frame.rgba8.clone(), chunk_order, unknown_chunks: Vec::new(),
        };
        let bytes = semio_s_artifact_stdio_png::io::author_png_projection(&projection).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))?;
        semio_s_artifact_stdio_png::io::decode_png(&bytes).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
