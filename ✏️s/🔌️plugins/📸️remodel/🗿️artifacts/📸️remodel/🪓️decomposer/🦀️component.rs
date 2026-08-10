//! RemodelDecomposer
use semio_framework_plugin::{ArtifactDecomposer, Confidence, Decomposition, DecomposeSource};
use crate::artifacts::remodel::schema::snapshot::RemodelSnapshot;

#[derive(Clone, Debug, Default)]
pub struct RemodelParts { pub snapshot: Option<RemodelSnapshot> }

pub struct RemodelDecomposer;

impl ArtifactDecomposer for RemodelDecomposer {
    type Snapshot = RemodelSnapshot;
    type Parts = RemodelParts;
    fn decompose(sources: &[DecomposeSource<'_>]) -> Decomposition<Self::Parts> {
        let mut parts = RemodelParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = Confidence::High;
        for source in sources {
            match source {
                DecomposeSource::Text(text) => match <RemodelSnapshot as store::DocumentDsl>::parse_dsl(text) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = Confidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("remodel.decompose.text", dsl::TextSpan::at(1, 1), err.to_string()));
                    }
                },
                DecomposeSource::Binary(bytes) => match <RemodelSnapshot as store::DocumentPack>::decode_pack(bytes) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = Confidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("remodel.decompose.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                    }
                },
            }
        }
        Decomposition { parts, confidence, diagnostics }
    }
}
