//! puzzle3d → txt — `s.stdio.txt` is a carrier: its body is this artifact's own canonical DSL text,
//! verbatim, so the hop is `IoFidelity::Exact`.
use crate::Puzzle3dSnapshot;

pub fn register() {}

pub fn serialize_bytes(from: &Puzzle3dSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<Puzzle3dSnapshot as store::ArtifactDsl>::print_dsl(from).into_bytes())
}
