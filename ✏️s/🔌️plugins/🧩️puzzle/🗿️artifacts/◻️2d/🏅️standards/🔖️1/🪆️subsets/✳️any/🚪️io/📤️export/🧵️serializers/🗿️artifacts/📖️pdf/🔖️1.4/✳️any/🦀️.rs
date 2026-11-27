//! 📤️ puzzle2d → pdf — the page drawing (`puzzle2d_board_drawing`) written as a one-page vector PDF 1.4 by
//! `s.stdio.semio/v1/drawing`'s own pdf leaf, the one pdf writer every owner shares.
//!
//! 🔖 `IoFidelity::Lossy`: a page drawing, not the document — there is no pdf import.
//!
//! 🧩️ The pdf route is not part of the puzzle2d library: only a composition root that offers pdf depends on this crate
//! and declares the artifact through [`artifact`].
use semio_framework::io_schema::{IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_os_kernel::io::io_mechanism::{serializer_entry, ArchiveChildren, IoEntry, IoRunControl, Serializer};
use semio_s_artifact_puzzle_2d::standards::v1::subsets::any::io::{puzzle2d_board_drawing, routes};
use semio_s_artifact_puzzle_2d::{ArtifactApps, Puzzle2dSnapshot};

const PDF: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId::ANY };

/// 📄️ The board's page drawing as PDF 1.4 bytes.
pub fn serialize_bytes(snapshot: &Puzzle2dSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    semio_s_artifact_stdio_semio_drawing_pdf_export::encode_drawing_pdf(&puzzle2d_board_drawing(snapshot), "1.4").map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("puzzle2d→pdf: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))
}

/// 📖️ The puzzle2d → pdf route.
pub struct PdfFormat;
impl Serializer<Puzzle2dSnapshot> for PdfFormat {
    const INTO: Dialect = PDF;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &Puzzle2dSnapshot, _: &ArchiveChildren, control: &mut IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        control.checkpoint().map_err(IoError::from_value_error)?;
        serialize_bytes(from).map(|bytes| IoOutcome::clean(IoPayload::Binary(bytes))).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string())))
    }
}

/// 🧾️ The subset's own routes and the pdf route, built once.
pub fn entries() -> &'static [IoEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<IoEntry>> = std::sync::OnceLock::new();
    ENTRIES
        .get_or_init(|| {
            let mut rows = routes::rows();
            rows.push(serializer_entry::<Puzzle2dSnapshot, PdfFormat>(routes::NATIVE));
            rows
        })
        .as_slice()
}

/// 🌳️ The puzzle2d declaration tree with its pdf route, for the composition root that offers pdf.
pub fn artifact<PA: ArtifactApps>() -> semio_plugin::app::declarations::ArtifactDeclaration<PA> {
    semio_s_artifact_puzzle_2d::artifact_with::<PA>(entries())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
