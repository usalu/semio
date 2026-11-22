//! 🔬️ PDF page editor authoring behavior against a language-agnostic fixture and the independent lopdf reader.

#[cfg(test)]
pub(crate) use semio_s_artifact_stdio_pdf_pdf_snapshot_value_tests::standards;

#[cfg(test)]
#[path = "."]
mod editor {
    pub(crate) use semio_s_artifact_stdio_pdf_pdf_snapshot_value_tests::editor::*;

    #[path = "../../🦀️.rs"]
    mod authoring_tests;
}
