//! 🪆️ Semio subset restrictions on the shared persisted child reference.
use store::os_io::ArtifactRef;

/// 🪪️ Validates the declared Semio subset while retaining independent local and target identities.
pub fn validate_semio_child_identity(_child_id: &str, target: &ArtifactRef, subset: &str) -> Result<(), String> {
    if target.dialect.artifact_kind != "s.stdio.semio" || target.dialect.standard != "v1" || target.dialect.subset != subset {
        return Err("Semio child dialect mismatch".into());
    }
    Ok(())
}
