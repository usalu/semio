//! 📑️ DocxDecomposer — local ArtifactDecomposer until SDK Wave 3.

use semio_framework_plugin::{ArtifactDecomposer, Confidence, Decomposition, DecomposeSource};
use crate::artifacts::docx::{DocxSnapshot};

//#region 🔖️Parts
/// 🧩 Decomposed `stdio.docx` parts.
#[derive(Clone, Debug, Default)]
pub struct DocxParts { pub snapshot: Option<DocxSnapshot>, }
//#endregion 🔖️Parts

//#region 🔖️Decomposer
/// 📑️ Decomposes `stdio.docx` sources.
pub struct DocxDecomposer;

impl ArtifactDecomposer for DocxDecomposer {
    type Snapshot = DocxSnapshot;
    type Parts = DocxParts;
    fn decompose(sources: &[DecomposeSource<'_>]) -> Decomposition<Self::Parts> {
        let mut parts = DocxParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = Confidence::High;
        for source in sources {
            match source {
                DecomposeSource::Text(text) => match <DocxSnapshot as store::DocumentDsl>::parse_dsl(text) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = Confidence::Low;
                        diagnostics.push(dsl::Diagnostic::error(
                            "stdio.decompose.text",
                            dsl::TextSpan::at(1, 1),
                            err.to_string(),
                        ));
                    }
                },
                DecomposeSource::Binary(bytes) => match <DocxSnapshot as store::DocumentPack>::decode_pack(bytes) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = Confidence::Low;
                        diagnostics.push(dsl::Diagnostic::error(
                            "stdio.decompose.docx",
                            dsl::TextSpan::at(1, 1),
                            err.to_string(),
                        ));
                    }
                },
            }
        }
        Decomposition { parts, confidence, diagnostics }
    }
}
//#endregion 🔖️Decomposer
