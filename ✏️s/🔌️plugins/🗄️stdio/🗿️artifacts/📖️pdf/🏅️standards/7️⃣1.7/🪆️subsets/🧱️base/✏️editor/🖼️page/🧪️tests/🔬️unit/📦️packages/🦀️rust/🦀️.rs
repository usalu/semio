//! 🔬️ PDF page editor behavior against native document fixtures.

#[cfg(test)]
pub(crate) use semio_s_artifact_stdio_pdf_pdf_snapshot_value_tests::standards;

#[cfg(test)]
#[path = "."]
mod editor {
    pub(crate) use semio_s_artifact_stdio_pdf_pdf_snapshot_value_tests::editor::*;

    #[path = "../../🦀️.rs"]
    mod page_tests;
}
