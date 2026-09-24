//! puzzle5d → txt — `s.stdio.txt` is a carrier: its body is this artifact's own canonical DSL text,
//! verbatim, so the hop is `IoFidelity::Exact`.
use crate::Puzzle5dSnapshot;

pub fn register() {}

pub fn serialize_bytes(from: &Puzzle5dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<Puzzle5dSnapshot as store::ArtifactDsl>::print_dsl(from).into_bytes())
}
