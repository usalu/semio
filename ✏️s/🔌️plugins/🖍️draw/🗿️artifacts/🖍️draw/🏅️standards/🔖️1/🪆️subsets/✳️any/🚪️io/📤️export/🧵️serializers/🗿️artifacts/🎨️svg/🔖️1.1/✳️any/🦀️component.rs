//! 🚪️ draw -> svg — foreign `Serializer<DrawSnapshot>` (design.md §3). Real: builds a
//! `SemioDrawingSnapshot` bridge value and dispatches through stdio's real semio/drawing<->svg
//! `io_dispatch` bridge (`crate::artifacts::draw::io::draw_document_to_svg`). `IoFidelity::Lossy` —
//! gradients, `blendMode`/`fillRule`, and group/image opacity have no `SemioDrawingSnapshot`
//! equivalent and are honestly dropped (see that function's own module doc).

use crate::artifacts::draw::DrawSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId::ANY };

pub struct DrawIntoSvg;

impl Serializer<DrawSnapshot> for DrawIntoSvg {
    const INTO: Dialect = SVG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &DrawSnapshot) -> IoResult<IoPayload> {
        let (svg_text, _width, _height) = crate::artifacts::draw::io::draw_document_to_svg(from)
            .map_err(|message| IoError { message: format!("DrawIntoSvg: {message}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Text(svg_text)))
    }
}
