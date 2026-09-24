//! puzzle2d → txt — `s.stdio.txt` is a carrier: its body is this artifact's own canonical DSL text,
//! verbatim, so the hop is `IoFidelity::Exact`.
use crate::Puzzle2dSnapshot;

pub fn register() {}

pub fn serialize_bytes(from: &Puzzle2dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<Puzzle2dSnapshot as store::ArtifactDsl>::print_dsl(from).into_bytes())
}
