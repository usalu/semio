//! 📑️ CsvDecomposer — local ArtifactDecomposer until SDK Wave 3.

use semio_framework_plugin::{ArtifactDecomposer, Confidence, Decomposition, DecomposeSource};
use crate::artifacts::csv::{CsvSnapshot};

//#region 🔖️Parts
/// 🧩 Decomposed `stdio.csv` parts.
#[derive(Clone, Debug, Default)]
pub struct CsvParts { pub snapshot: Option<CsvSnapshot>, }
//#endregion 🔖️Parts

//#region 🔖️Decomposer
/// 📑️ Decomposes `stdio.csv` sources.
pub struct CsvDecomposer;

impl ArtifactDecomposer for CsvDecomposer {
    type Snapshot = CsvSnapshot;
    type Parts = CsvParts;
    fn decompose(sources: &[DecomposeSource<'_>]) -> Decomposition<Self::Parts> {
        let mut parts = CsvParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = Confidence::High;
        for source in sources {
            match source {
                DecomposeSource::Text(text) => match <CsvSnapshot as store::DocumentDsl>::parse_dsl(text) {
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
                DecomposeSource::Binary(bytes) => match <CsvSnapshot as store::DocumentPack>::decode_pack(bytes) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = Confidence::Low;
                        diagnostics.push(dsl::Diagnostic::error(
                            "stdio.decompose.binary",
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
