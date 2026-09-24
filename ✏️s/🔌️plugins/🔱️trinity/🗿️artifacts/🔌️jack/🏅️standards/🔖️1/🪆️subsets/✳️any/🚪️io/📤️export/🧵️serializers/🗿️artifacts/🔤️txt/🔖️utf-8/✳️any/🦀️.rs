//! jack → txt — `s.stdio.txt` is a carrier: its body is this artifact's own canonical DSL text,
//! verbatim, so the hop is `IoFidelity::Exact`.
use crate::JackSnapshot;

pub fn register() {}

pub fn serialize_bytes(from: &JackSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<JackSnapshot as store::ArtifactDsl>::print_dsl(from).into_bytes())
}
