//! ♻️ Artifact identities retire through their defining owner.
use semio_framework_value::artifact_retire_struct;
artifact_retire_struct!(super::ArtifactDialect { artifact_kind, standard, subset });
artifact_retire_struct!(super::ArtifactRef { artifact_id, dialect });
