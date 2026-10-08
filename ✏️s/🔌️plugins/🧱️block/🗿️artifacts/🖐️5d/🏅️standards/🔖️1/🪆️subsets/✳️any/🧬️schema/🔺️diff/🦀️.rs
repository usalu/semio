//! 🧬️ Block5d diff schema — a field-sparse, id-keyed delta over the artifact: sub-documents carry field patches, id-keyed lists carry
//! removed/added/patched rows (never a whole row, list or sub-document copy). `absorb` coalesces per field and per id, `inverse` restores exact base values.

use crate::{Block5dGripKind, Block5dGripTemplate, Block5dPart2d, Block5dPart3d, Block5dSnapshot};
use semio_s_plugin_block::{BlockAttributesDelta, BlockAuthorsDelta, BlockCamera2dPatch, BlockCamera3dPatch, BlockCompatibilityDelta, BlockKindIdentityPatch, BlockMetaPatch, BlockOptionalNumber, BlockOptionalOrientation, BlockOptionalScale, BlockOptionalText, BlockPatchError, BlockRepresentationsDelta, block_patch, block_patch_absorb, block_patch_apply, block_patch_inverse, block_patch_is_empty};
use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Field-sparse delta for the block5d artifact.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact_schema(id = "s.block.block5d")]
pub struct Block5dDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub part_kind: Option<BlockKindIdentityPatch>,
    #[state(artifact)]
    pub part_2d: Option<Block5dPart2dPatch>,
    #[state(artifact)]
    pub part_3d: Option<Block5dPart3dPatch>,
    #[state(artifact)]
    pub representations: BlockRepresentationsDelta,
    #[state(artifact)]
    pub grip_kinds: Block5dGripKindsDelta,
    #[state(artifact)]
    pub grips: Block5dGripsDelta,
    #[state(artifact)]
    pub compatibility: BlockCompatibilityDelta,
    #[state(artifact)]
    pub attributes: BlockAttributesDelta,
    #[state(artifact)]
    pub authors: BlockAuthorsDelta,
    #[state(artifact)]
    pub camera2d: Option<BlockCamera2dPatch>,
    #[state(artifact)]
    pub camera3d: Option<BlockCamera3dPatch>,
    #[state(artifact)]
    pub meta: Option<BlockMetaPatch>,
}
//#endregion 🔖️Diff

//#region 🔖️Patches
block_patch!(test; /// 🖌️ Field patch over the 2D presentation.
    Block5dPart2dPatch for Block5dPart2d { plain {  } optional { shape: BlockOptionalText, radius: BlockOptionalNumber, width: BlockOptionalNumber, height: BlockOptionalNumber, color: BlockOptionalText, icon_kind: BlockOptionalText } });
block_patch!(test; /// 🧊️ Field patch over the 3D placement.
    Block5dPart3dPatch for Block5dPart3d { plain {  } optional { orientation: BlockOptionalOrientation, scale: BlockOptionalScale } });
block_patch!(test; /// 🔘️ Field patch over a grip kind (its id is the row identity).
    Block5dGripKindPatch for Block5dGripKind { plain { name: String, label: String, color: String, default_rope_kind: String } optional {  } });
block_patch!(test; /// 🌱️ Field patch over a grip template (its id is the row identity).
    Block5dGripTemplatePatch for Block5dGripTemplate { plain { grip_kind: String, angle: f64, radius_2d: f64, position: [f64; 3], direction: [f64; 3], radius_3d: f64 } optional {  } });
protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📂 Row delta over the grip kinds.
    pub Block5dGripKindsDelta { removal: Block5dGripKindsRemoval, insertion: Block5dGripKindsInsertion, relocation: Block5dGripKindsRelocation, modification: Block5dGripKindsPatchEntry, row: Block5dGripKind, patch: Block5dGripKindPatch, key: id, values_only }
}
protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📂 Row delta over the grip templates.
    pub Block5dGripsDelta { removal: Block5dGripsRemoval, insertion: Block5dGripsInsertion, relocation: Block5dGripsRelocation, modification: Block5dGripsPatchEntry, row: Block5dGripTemplate, patch: Block5dGripTemplatePatch, key: id, values_only }
}
//#endregion 🔖️Patches

//#region 🔖️Apply
/// 🛡️ Lifts a patch rejection into this crate's typed apply error.
fn lift(error: BlockPatchError) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new(error.code, error.message).at(error.target)
}

impl protocol::MutationDiff<Block5dSnapshot> for Block5dDiff {
    fn apply(&self, base: &Block5dSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Block5dSnapshot> {
        let mut next = base.clone();
        if let Some(schema) = &self.schema {
            next.schema.clone_from(schema);
        }
        next.part_kind = block_patch_apply(&self.part_kind, &base.part_kind, "partKind").map_err(lift)?;
        next.part_2d = block_patch_apply(&self.part_2d, &base.part_2d, "part2d").map_err(lift)?;
        next.part_3d = block_patch_apply(&self.part_3d, &base.part_3d, "part3d").map_err(lift)?;
        next.camera2d = block_patch_apply(&self.camera2d, &base.camera2d, "camera2d").map_err(lift)?;
        next.camera3d = block_patch_apply(&self.camera3d, &base.camera3d, "camera3d").map_err(lift)?;
        next.meta = block_patch_apply(&self.meta, &base.meta, "meta").map_err(lift)?;
        next.representations = self.representations.commit_onto(&base.representations, capability).map_err(|error| error.under(["representations"]))?;
        next.grip_kinds = self.grip_kinds.commit_onto(&base.grip_kinds, capability).map_err(|error| error.under(["gripKinds"]))?;
        next.grips = self.grips.commit_onto(&base.grips, capability).map_err(|error| error.under(["grips"]))?;
        next.compatibility = self.compatibility.commit_onto(&base.compatibility, capability).map_err(|error| error.under(["compatibility"]))?;
        next.attributes = self.attributes.commit_onto(&base.attributes, capability).map_err(|error| error.under(["attributes"]))?;
        next.authors = self.authors.commit_onto(&base.authors, capability).map_err(|error| error.under(["authors"]))?;
        Ok(next)
    }
    fn absorb(&mut self, later: Self) {
        if later.schema.is_some() {
            self.schema = later.schema;
        }
        block_patch_absorb(&mut self.part_kind, later.part_kind);
        block_patch_absorb(&mut self.part_2d, later.part_2d);
        block_patch_absorb(&mut self.part_3d, later.part_3d);
        block_patch_absorb(&mut self.camera2d, later.camera2d);
        block_patch_absorb(&mut self.camera3d, later.camera3d);
        block_patch_absorb(&mut self.meta, later.meta);
        self.representations.absorb(later.representations);
        self.grip_kinds.absorb(later.grip_kinds);
        self.grips.absorb(later.grips);
        self.compatibility.absorb(later.compatibility);
        self.attributes.absorb(later.attributes);
        self.authors.absorb(later.authors);
    }
}

impl protocol::DiffAlgebra<Block5dSnapshot> for Block5dDiff {
    fn inverse(&self, base: &Block5dSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            part_kind: block_patch_inverse(&self.part_kind, &base.part_kind),
            part_2d: block_patch_inverse(&self.part_2d, &base.part_2d),
            part_3d: block_patch_inverse(&self.part_3d, &base.part_3d),
            camera2d: block_patch_inverse(&self.camera2d, &base.camera2d),
            camera3d: block_patch_inverse(&self.camera3d, &base.camera3d),
            meta: block_patch_inverse(&self.meta, &base.meta),
            representations: self.representations.inverse(&base.representations),
            grip_kinds: self.grip_kinds.inverse(&base.grip_kinds),
            grips: self.grips.inverse(&base.grips),
            compatibility: self.compatibility.inverse(&base.compatibility),
            attributes: self.attributes.inverse(&base.attributes),
            authors: self.authors.inverse(&base.authors),
        }
    }
    fn is_empty(&self) -> bool {
        self.schema.is_none()
            && block_patch_is_empty(&self.part_kind)
            && block_patch_is_empty(&self.part_2d)
            && block_patch_is_empty(&self.part_3d)
            && block_patch_is_empty(&self.camera2d)
            && block_patch_is_empty(&self.camera3d)
            && block_patch_is_empty(&self.meta)
            && self.representations.is_empty()
            && self.grip_kinds.is_empty()
            && self.grips.is_empty()
            && self.compatibility.is_empty()
            && self.attributes.is_empty()
            && self.authors.is_empty()
    }
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
