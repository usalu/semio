//! Puzzle3dDecomposer
use semio_framework_plugin::{ArtifactDecomposer, Confidence, Decomposition, DecomposeSource};
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

#[derive(Clone, Debug, Default)]
pub struct Puzzle3dParts { pub snapshot: Option<Puzzle3dSnapshot> }

pub struct Puzzle3dDecomposer;

impl ArtifactDecomposer for Puzzle3dDecomposer {
    type Snapshot = Puzzle3dSnapshot;
    type Parts = Puzzle3dParts;
    fn decompose(sources: &[DecomposeSource<'_>]) -> Decomposition<Self::Parts> {
        let mut parts = Puzzle3dParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = Confidence::High;
        for source in sources {
            match source {
                DecomposeSource::Text(text) => match <Puzzle3dSnapshot as store::DocumentDsl>::parse_dsl(text) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = Confidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("puzzle3d.decompose.text", dsl::TextSpan::at(1, 1), err.to_string()));
                    }
                },
                DecomposeSource::Binary(bytes) => match <Puzzle3dSnapshot as store::DocumentPack>::decode_pack(bytes) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = Confidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("puzzle3d.decompose.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                    }
                },
            }
        }
        Decomposition { parts, confidence, diagnostics }
    }
}
