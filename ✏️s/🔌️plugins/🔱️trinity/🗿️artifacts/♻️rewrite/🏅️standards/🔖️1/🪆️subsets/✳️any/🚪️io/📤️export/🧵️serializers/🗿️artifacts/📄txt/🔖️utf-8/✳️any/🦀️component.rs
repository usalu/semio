//! rewrite -> txt
use crate::artifacts::rewrite::RewriteSnapshot;
use semio_s_plugin_stdio::artifacts::txt::TxtSnapshot;

pub async fn register() {}

pub async fn serialize(snapshot: &RewriteSnapshot) -> Result<TxtSnapshot, store::TextError> {
    Ok(TxtSnapshot::from_body(&<RewriteSnapshot as store::ArtifactDsl>::print_dsl(snapshot)))
}

pub async fn serialize_bytes(snapshot: &RewriteSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(serialize(snapshot)?.to_body().into_bytes())
}
