//! rewriting -> txt
use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;
use semio_s_artifact_stdio_txt::TxtSnapshot;

pub fn register() {}

pub fn serialize(snapshot: &RewritingSnapshot) -> Result<TxtSnapshot, semio_framework_diagnostic::TextError> {
    Ok(TxtSnapshot::from_body(&<RewritingSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(snapshot)))
}

pub fn serialize_bytes(snapshot: &RewritingSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    Ok(serialize(snapshot)?.to_body().into_bytes())
}
