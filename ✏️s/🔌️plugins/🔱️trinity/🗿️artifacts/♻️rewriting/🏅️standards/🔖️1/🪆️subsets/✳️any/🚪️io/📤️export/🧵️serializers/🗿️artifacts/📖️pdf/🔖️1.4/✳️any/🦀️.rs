//! rewriting -> pdf
use crate::artifacts::rewriting::RewritingSnapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &RewritingSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<RewritingSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
