//! Puzzle2dDecomposer
use semio_framework_plugin::{ArtifactDecomposer, Confidence, Decomposition, DecomposeSource};
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

#[derive(Clone, Debug, Default)]
pub struct Puzzle2dParts { pub snapshot: Option<Puzzle2dSnapshot> }

pub struct Puzzle2dDecomposer;

impl ArtifactDecomposer for Puzzle2dDecomposer {
    type Snapshot = Puzzle2dSnapshot;
    type Parts = Puzzle2dParts;
    fn decompose(sources: &[DecomposeSource<'_>]) -> Decomposition<Self::Parts> {
        let mut parts = Puzzle2dParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = Confidence::High;
        for source in sources {
            match source {
                DecomposeSource::Text(text) => match <Puzzle2dSnapshot as store::DocumentDsl>::parse_dsl(text) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = Confidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("puzzle2d.decompose.text", dsl::TextSpan::at(1, 1), err.to_string()));
                    }
                },
                DecomposeSource::Binary(bytes) => match <Puzzle2dSnapshot as store::DocumentPack>::decode_pack(bytes) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = Confidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("puzzle2d.decompose.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                    }
                },
            }
        }
        Decomposition { parts, confidence, diagnostics }
    }
}
