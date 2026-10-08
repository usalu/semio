//! 🧱️ Block plugin schema authority has no artifact dependencies.
#[path = "🧬️schema/🧱️shared/🦀️.rs"]
mod schema;
pub use schema::{BlockKindIdentity, BlockAttribute, BlockAuthor, BlockCompatibilityRule, BlockRepresentation, BlockCamera2d, BlockCamera3d, BlockMeta};
pub use schema::{block_insert_order, block_patch_absorb, block_patch_apply, block_patch_between, block_patch_inverse, block_patch_is_empty, block_rows_absorb, block_rows_apply, block_rows_between, block_rows_inverse, block_rows_is_empty};
pub use schema::{BlockAttributePatch, BlockAttributesDelta, BlockAttributesPatchEntry, BlockAuthorPatch, BlockAuthorsDelta, BlockAuthorsPatchEntry, BlockCamera2dPatch, BlockCamera3dPatch, BlockCompatibilityDelta, BlockCompatibilityPatchEntry, BlockCompatibilityRulePatch};
pub use schema::{BlockKindIdentityPatch, BlockMetaPatch, BlockOptionalNumber, BlockOptionalOrientation, BlockOptionalScale, BlockOptionalText, BlockPatch, BlockPatchError, BlockRepresentationPatch, BlockRepresentationsDelta, BlockRepresentationsPatchEntry, BlockRows};
#[path = "🧬️schema/🧱️shared/♻️retirement/🦀️.rs"]
mod retirement;
