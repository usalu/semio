//! 📑️ StlDecomposer — local ArtifactDecomposer until SDK Wave 3.

use semio_framework_plugin::{ArtifactDecomposer, Confidence, Decomposition, DecomposeSource};
use crate::artifacts::stl::{StlSnapshot};

//#region 🔖️Parts
/// 🧩 Decomposed `stdio.stl` parts.
#[derive(Clone, Debug, Default)]
pub struct StlParts { pub snapshot: Option<StlSnapshot>, }
//#endregion 🔖️Parts

//#region 🔖️Decomposer
/// 📑️ Decomposes `stdio.stl` sources.
pub struct StlDecomposer;

impl ArtifactDecomposer for StlDecomposer {
    type Snapshot = StlSnapshot;
    type Parts = StlParts;
    fn decompose(sources: &[DecomposeSource<'_>]) -> Decomposition<Self::Parts> {
        let mut parts = StlParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = Confidence::High;
        for source in sources {
            match source {
                DecomposeSource::Text(text) => match <StlSnapshot as store::DocumentDsl>::parse_dsl(text) {
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
                DecomposeSource::Binary(bytes) => match <StlSnapshot as store::DocumentPack>::decode_pack(bytes) {
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
