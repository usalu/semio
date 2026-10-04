//! energy ← txt — the body of a `s.stdio.txt` carrier parsed as this artifact's own DSL, the inverse
//! of the sibling export leaf (`IoFidelity::Exact`).
use crate::EnergyModelSnapshot;

pub fn register() {}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<EnergyModelSnapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("energy←txt: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    <EnergyModelSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
