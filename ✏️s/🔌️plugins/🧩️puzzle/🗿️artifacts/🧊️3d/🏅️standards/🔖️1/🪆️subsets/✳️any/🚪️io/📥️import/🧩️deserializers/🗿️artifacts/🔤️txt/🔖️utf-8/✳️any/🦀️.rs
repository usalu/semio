//! of the sibling export leaf (`IoFidelity::Exact`).
//! puzzle3d ← txt — the body of a `s.stdio.txt` carrier parsed as this artifact's own DSL, the inverse
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

pub fn register() {}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Puzzle3dSnapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("puzzle3d←txt: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    <Puzzle3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)
}
