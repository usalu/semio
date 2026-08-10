//! 📑️ PlyDecomposer — local ArtifactDecomposer until SDK Wave 3.

use semio_framework_plugin::{ArtifactDecomposer, Confidence, Decomposition, DecomposeSource};
use crate::artifacts::ply::{PlySnapshot};

//#region 🔖️Parts
/// 🧩 Decomposed `stdio.ply` parts.
#[derive(Clone, Debug, Default)]
pub struct PlyParts { pub snapshot: Option<PlySnapshot>, }
//#endregion 🔖️Parts

//#region 🔖️Decomposer
/// 📑️ Decomposes `stdio.ply` sources.
pub struct PlyDecomposer;

impl ArtifactDecomposer for PlyDecomposer {
    type Snapshot = PlySnapshot;
    type Parts = PlyParts;
    fn decompose(sources: &[DecomposeSource<'_>]) -> Decomposition<Self::Parts> {
        let mut parts = PlyParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = Confidence::High;
        for source in sources {
            match source {
                DecomposeSource::Text(text) => match <PlySnapshot as store::DocumentDsl>::parse_dsl(text) {
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
                DecomposeSource::Binary(bytes) => match <PlySnapshot as store::DocumentPack>::decode_pack(bytes) {
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
