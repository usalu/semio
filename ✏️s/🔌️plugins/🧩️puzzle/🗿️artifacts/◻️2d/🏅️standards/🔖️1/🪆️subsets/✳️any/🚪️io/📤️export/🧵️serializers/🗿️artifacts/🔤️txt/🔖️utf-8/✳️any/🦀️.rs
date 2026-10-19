//! 📜️ verbatim, so the hop is `IoFidelity::Exact`.
//! puzzle2d → txt — `s.stdio.txt` is a carrier: its body is this artifact's own canonical DSL text,
use crate::Puzzle2dSnapshot;

pub fn register() {}

pub fn serialize_bytes(from: &Puzzle2dSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    Ok(<Puzzle2dSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(from).into_bytes())
}
