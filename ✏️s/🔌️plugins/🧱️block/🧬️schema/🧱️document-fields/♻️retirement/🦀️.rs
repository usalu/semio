//! ♻️ Shared Block records retire at their plugin owner.
use crate::{BlockKindIdentity, BlockAttribute, BlockAuthor, BlockCompatibilityRule, BlockRepresentation, BlockCamera2d, BlockCamera3d, BlockMeta, BlockOptionalText, BlockOptionalNumber, BlockOptionalOrientation, BlockOptionalScale, BlockAttributePatch, BlockAuthorPatch, BlockCompatibilityRulePatch, BlockRepresentationPatch};
//#region 🧱️SharedRows
semio_framework_value::artifact_retire_struct!(BlockKindIdentity { id, name, label, variant, description, icon, unit });
semio_framework_value::artifact_retire_struct!(BlockAttribute { key, value, definition });
semio_framework_value::artifact_retire_struct!(BlockAuthor { id, name, email });
semio_framework_value::artifact_retire_struct!(BlockCompatibilityRule { id, source, target, bidirectional });
semio_framework_value::artifact_retire_struct!(BlockCamera2d { x, y, zoom });
semio_framework_value::artifact_retire_struct!(BlockMeta { description });
semio_framework_value::artifact_retire_struct!(BlockRepresentation { id, name, mesh_url, tags, lod, description, attributes });
semio_framework_value::artifact_retire_struct!(BlockOptionalText { value });
semio_framework_value::artifact_retire_struct!(BlockOptionalNumber { value });
semio_framework_value::artifact_retire_struct!(BlockOptionalOrientation { value });
semio_framework_value::artifact_retire_struct!(BlockOptionalScale { value });
semio_framework_value::artifact_retire_struct!(BlockAttributePatch { value, definition });
semio_framework_value::artifact_retire_struct!(BlockAuthorPatch { name, email });
semio_framework_value::artifact_retire_struct!(BlockCompatibilityRulePatch { source, target, bidirectional });
semio_framework_value::artifact_retire_struct!(BlockRepresentationPatch { name, mesh_url, lod, description, tags_removed, tags_added, attributes_removed, attributes_added });

semio_framework_value::artifact_retire_struct!(BlockCamera3d { position, target, zoom });
//#endregion 🧱️SharedRows
