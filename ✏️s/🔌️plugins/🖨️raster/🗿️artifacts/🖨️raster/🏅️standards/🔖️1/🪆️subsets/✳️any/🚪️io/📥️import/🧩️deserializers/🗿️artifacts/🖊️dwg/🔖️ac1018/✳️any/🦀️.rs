//! 🖨️ raster ← dwg — the file is read by `s.stdio.semio/v1/drawing`'s own dwg import leaf
//! (`decode_drawing`, every version `decode_dwg` reads: AC1015 and the R2004 family), then fitted onto a
//! page and rendered into one pixel layer (`raster_document_from_dwg_drawing`).
//!
//! 🔖 `IoFidelity::Lossy`: vectors become pixels at one unit per pixel; layers, entity identity and
//! everything the drawing bridge itself drops (points, 3D faces, `z`) are gone.
use crate::RasterSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::io::{decode_drawing, SemioDrawingFormat};

pub fn register() {}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<RasterSnapshot, String> {
    crate::io::raster_document_from_dwg_drawing(&decode_drawing(bytes, SemioDrawingFormat::Dwg)?)
}
