//! puzzle3d → txt — `s.stdio.txt` is a carrier: its body is this artifact's own canonical DSL text,
//! verbatim, so the hop is `IoFidelity::Exact`.
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

pub fn register() {}

pub fn serialize_bytes(from: &Puzzle3dSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    Ok(<Puzzle3dSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(from).into_bytes())
}
