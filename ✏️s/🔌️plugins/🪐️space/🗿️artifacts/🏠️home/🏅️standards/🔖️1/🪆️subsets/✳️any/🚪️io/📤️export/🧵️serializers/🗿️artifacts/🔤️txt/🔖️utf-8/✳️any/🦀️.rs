//! home → txt — `s.stdio.txt` is a carrier: its body is this artifact's own canonical DSL text,
//! verbatim, so the hop is `IoFidelity::Exact`.
use crate::SHomeSnapshot;

pub fn register() {}

pub fn serialize_bytes(from: &SHomeSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<SHomeSnapshot as store::ArtifactDsl>::print_dsl(from).into_bytes())
}
