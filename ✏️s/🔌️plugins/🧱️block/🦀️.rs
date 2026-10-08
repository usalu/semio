//! 🧱️ Block plugin schema authority has no artifact dependencies.
extern crate semio_framework_os_kernel as protocol;
#[path = "🧬️schema/🧱️shared/🦀️.rs"]
mod schema;
pub use schema::{BlockKindIdentity, BlockAttribute, BlockAuthor, BlockCompatibilityRule, BlockRepresentation, BlockCamera2d, BlockCamera3d, BlockMeta};
pub use schema::{block_apply_error, block_insert_index, block_patch_absorb, block_patch_apply, block_patch_inverse, block_patch_is_empty};
pub use schema::{BlockAttributePatch, BlockAttributesDelta, BlockAttributesInsertion, BlockAttributesPatchEntry, BlockAttributesRelocation, BlockAttributesRemoval, BlockAuthorPatch, BlockAuthorsDelta, BlockAuthorsInsertion, BlockAuthorsPatchEntry, BlockAuthorsRelocation, BlockAuthorsRemoval, BlockCamera2dPatch, BlockCamera3dPatch, BlockCompatibilityDelta, BlockCompatibilityInsertion, BlockCompatibilityPatchEntry, BlockCompatibilityRelocation, BlockCompatibilityRemoval, BlockCompatibilityRulePatch};
pub use schema::{BlockKindIdentityPatch, BlockMetaPatch, BlockOptionalNumber, BlockOptionalOrientation, BlockOptionalScale, BlockOptionalText, BlockPatch, BlockPatchError, BlockRepresentationPatch, BlockRepresentationsDelta, BlockRepresentationsInsertion, BlockRepresentationsPatchEntry, BlockRepresentationsRelocation, BlockRepresentationsRemoval};
#[path = "🧬️schema/🧱️shared/♻️retirement/🦀️.rs"]
mod retirement;
