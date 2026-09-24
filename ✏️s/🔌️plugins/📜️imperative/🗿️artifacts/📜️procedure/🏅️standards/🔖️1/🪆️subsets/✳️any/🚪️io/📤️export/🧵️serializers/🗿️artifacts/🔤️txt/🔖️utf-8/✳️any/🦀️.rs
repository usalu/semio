//! procedure → txt — `s.stdio.txt` is a carrier: its body is this artifact's own canonical DSL text,
//! verbatim, so the hop is `IoFidelity::Exact`.
use crate::ProcedureSnapshot;

pub fn register() {}

pub fn serialize_bytes(from: &ProcedureSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<ProcedureSnapshot as store::ArtifactDsl>::print_dsl(from).into_bytes())
}
