//! 🪆️ Semio subset restrictions on the shared persisted child reference.
use store::os_io::ArtifactRef;

/// 🪪️ Validates exact identity and the declared Semio subset without minting ownership.
pub fn validate_semio_child_identity(child_id: &str, target: &ArtifactRef, subset: &str) -> Result<(), String> {
    if child_id != target.artifact_id || target.dialect.artifact_kind != "s.stdio.semio" || target.dialect.standard != "v1" || target.dialect.subset != subset {
        return Err("Semio child identity or dialect mismatch".into());
    }
    if ArtifactRef::parse_uri(&target.to_uri())? != *target {
        return Err("Semio child reference is not canonical".into());
    }
    Ok(())
}
