//! rewriting -> txt
use crate::artifacts::rewriting::RewritingSnapshot;
use semio_s_plugin_stdio::artifacts::txt::TxtSnapshot;

pub fn register() {}

pub fn serialize(snapshot: &RewritingSnapshot) -> Result<TxtSnapshot, store::TextError> {
    Ok(TxtSnapshot::from_body(&<RewritingSnapshot as store::ArtifactDsl>::print_dsl(snapshot)))
}

pub fn serialize_bytes(snapshot: &RewritingSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(serialize(snapshot)?.to_body().into_bytes())
}
