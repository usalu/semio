//! 📑️ GifDecomposer — local ArtifactDecomposer until SDK Wave 3.

use semio_framework_plugin::{ArtifactDecomposer, Confidence, Decomposition, DecomposeSource};
use crate::artifacts::gif::{GifSnapshot};

//#region 🔖️Parts
/// 🧩 Decomposed `stdio.gif` parts.
#[derive(Clone, Debug, Default)]
pub struct GifParts { pub snapshot: Option<GifSnapshot>, }
//#endregion 🔖️Parts

//#region 🔖️Decomposer
/// 📑️ Decomposes `stdio.gif` sources.
pub struct GifDecomposer;

impl ArtifactDecomposer for GifDecomposer {
    type Snapshot = GifSnapshot;
    type Parts = GifParts;
    fn decompose(sources: &[DecomposeSource<'_>]) -> Decomposition<Self::Parts> {
        let mut parts = GifParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = Confidence::High;
        for source in sources {
            match source {
                DecomposeSource::Text(text) => match <GifSnapshot as store::DocumentDsl>::parse_dsl(text) {
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
                DecomposeSource::Binary(bytes) => match <GifSnapshot as store::DocumentPack>::decode_pack(bytes) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = Confidence::Low;
                        diagnostics.push(dsl::Diagnostic::error(
                            "stdio.decompose.gif",
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
