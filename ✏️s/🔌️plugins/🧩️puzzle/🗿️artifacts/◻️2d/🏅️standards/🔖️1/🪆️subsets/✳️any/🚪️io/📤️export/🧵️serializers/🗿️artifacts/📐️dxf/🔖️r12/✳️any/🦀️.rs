//! puzzle2d → dxf — the page drawing (`puzzle2d_board_drawing`) written as DXF R12 entities by `s.stdio.semio/v1/drawing`'s own
//! export leaf, the one dxf writer every owner shares.
//!
//! 🔖 `IoFidelity::Lossy`: a page drawing, not the document — there is no dxf import.
use crate::standards::v1::subsets::any::io::puzzle2d_board_drawing;
use crate::Puzzle2dSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::io::{encode_drawing, SemioDrawingFormat};

pub fn register() {}

pub fn serialize_bytes(snapshot: &Puzzle2dSnapshot) -> Result<Vec<u8>, store::TextError> {
    encode_drawing(&puzzle2d_board_drawing(snapshot), SemioDrawingFormat::Dxf).map_err(|error| store::TextError::new(format!("puzzle2d→dxf: {error}"), dsl::TextSpan::at(1, 1)))
}
