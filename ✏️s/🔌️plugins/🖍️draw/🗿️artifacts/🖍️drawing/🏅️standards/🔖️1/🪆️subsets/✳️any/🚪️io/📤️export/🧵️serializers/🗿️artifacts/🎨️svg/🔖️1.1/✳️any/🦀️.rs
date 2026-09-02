//! 🚪️ drawing -> svg — foreign `Serializer<DrawingSnapshot>` (design.md §3). Real: builds a
//! `SemioDrawingSnapshot` bridge value and dispatches through stdio's real semio/drawing<->svg
//! `io_dispatch` bridge (`crate::artifacts::drawing::io::drawing_document_to_svg`). `IoFidelity::Lossy` —
//! gradients, `blendMode`/`fillRule`, and group/image opacity have no `SemioDrawingSnapshot`
//! equivalent and are honestly dropped (see that function's own module doc).

use crate::artifacts::drawing::DrawingSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId::ANY };

pub struct DrawingIntoSvg;

impl Serializer<DrawingSnapshot> for DrawingIntoSvg {
    const INTO: Dialect = SVG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn serialize(from: &DrawingSnapshot) -> IoResult<IoPayload> {
        let (svg_text, _width, _height) = crate::artifacts::drawing::io::drawing_document_to_svg(from).map_err(|message| IoError { message: format!("DrawingIntoSvg: {message}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Text(svg_text)))
    }
}
