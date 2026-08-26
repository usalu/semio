//! rewrite -> pdf
use crate::artifacts::rewrite::RewriteSnapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &RewriteSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<RewriteSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
