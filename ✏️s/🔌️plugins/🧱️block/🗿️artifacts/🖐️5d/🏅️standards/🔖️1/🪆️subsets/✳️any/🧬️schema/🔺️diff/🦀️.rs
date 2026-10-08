//! 🧬️ Block5d diff schema — a field-sparse, id-keyed delta over the artifact: sub-documents carry field patches, id-keyed lists carry
//! removed/added/patched rows (never a whole row, list or sub-document copy). `absorb` coalesces per field and per id, `inverse` restores exact base values.

use crate::{Block5dGripKind, Block5dGripTemplate, Block5dPart2d, Block5dPart3d, Block5dSnapshot};
use semio_s_plugin_block::{BlockAttributesDelta, BlockAuthorsDelta, BlockCamera2dPatch, BlockCamera3dPatch, BlockCompatibilityDelta, BlockKindIdentityPatch, BlockMetaPatch, BlockOptionalNumber, BlockOptionalOrientation, BlockOptionalScale, BlockOptionalText, BlockPatchError, BlockRepresentationsDelta, block_patch, block_patch_absorb, block_patch_apply, block_patch_between, block_patch_inverse, block_patch_is_empty, block_rows, block_rows_absorb, block_rows_apply, block_rows_between, block_rows_inverse, block_rows_is_empty};
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
    pub representations: Option<BlockRepresentationsDelta>,
    #[state(artifact)]
    pub grip_kinds: Option<Block5dGripKindsDelta>,
    #[state(artifact)]
    pub grips: Option<Block5dGripsDelta>,
    #[state(artifact)]
    pub compatibility: Option<BlockCompatibilityDelta>,
    #[state(artifact)]
    pub attributes: Option<BlockAttributesDelta>,
    #[state(artifact)]
    pub authors: Option<BlockAuthorsDelta>,
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
block_rows!(test; /// 📂 Row delta over the grip kinds.
    Block5dGripKindsDelta, Block5dGripKindsPatchEntry, Block5dGripKind, Block5dGripKindPatch, id);
block_rows!(test; /// 📂 Row delta over the grip templates.
    Block5dGripsDelta, Block5dGripsPatchEntry, Block5dGripTemplate, Block5dGripTemplatePatch, id);
//#endregion 🔖️Patches

//#region 🔖️Apply
/// 🛡️ Lifts a patch rejection into this crate's typed apply error.
fn lift(error: BlockPatchError) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new(error.code, error.message).at(error.target)
}

impl protocol::MutationDiff<Block5dSnapshot> for Block5dDiff {
    fn apply(&self, base: &Block5dSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Block5dSnapshot> {
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
        next.representations = block_rows_apply(&self.representations, &base.representations, "representations").map_err(lift)?;
        next.grip_kinds = block_rows_apply(&self.grip_kinds, &base.grip_kinds, "gripKinds").map_err(lift)?;
        next.grips = block_rows_apply(&self.grips, &base.grips, "grips").map_err(lift)?;
        next.compatibility = block_rows_apply(&self.compatibility, &base.compatibility, "compatibility").map_err(lift)?;
        next.attributes = block_rows_apply(&self.attributes, &base.attributes, "attributes").map_err(lift)?;
        next.authors = block_rows_apply(&self.authors, &base.authors, "authors").map_err(lift)?;
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
        block_rows_absorb(&mut self.representations, later.representations);
        block_rows_absorb(&mut self.grip_kinds, later.grip_kinds);
        block_rows_absorb(&mut self.grips, later.grips);
        block_rows_absorb(&mut self.compatibility, later.compatibility);
        block_rows_absorb(&mut self.attributes, later.attributes);
        block_rows_absorb(&mut self.authors, later.authors);
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
            representations: block_rows_inverse(&self.representations, &base.representations),
            grip_kinds: block_rows_inverse(&self.grip_kinds, &base.grip_kinds),
            grips: block_rows_inverse(&self.grips, &base.grips),
            compatibility: block_rows_inverse(&self.compatibility, &base.compatibility),
            attributes: block_rows_inverse(&self.attributes, &base.attributes),
            authors: block_rows_inverse(&self.authors, &base.authors),
        }
    }
    fn between(base: &Block5dSnapshot, other: &Block5dSnapshot) -> Self {
        Self {
            schema: (base.schema != other.schema).then(|| other.schema.clone()),
            part_kind: block_patch_between(&base.part_kind, &other.part_kind),
            part_2d: block_patch_between(&base.part_2d, &other.part_2d),
            part_3d: block_patch_between(&base.part_3d, &other.part_3d),
            camera2d: block_patch_between(&base.camera2d, &other.camera2d),
            camera3d: block_patch_between(&base.camera3d, &other.camera3d),
            meta: block_patch_between(&base.meta, &other.meta),
            representations: block_rows_between(&base.representations, &other.representations),
            grip_kinds: block_rows_between(&base.grip_kinds, &other.grip_kinds),
            grips: block_rows_between(&base.grips, &other.grips),
            compatibility: block_rows_between(&base.compatibility, &other.compatibility),
            attributes: block_rows_between(&base.attributes, &other.attributes),
            authors: block_rows_between(&base.authors, &other.authors),
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
            && block_rows_is_empty(&self.representations)
            && block_rows_is_empty(&self.grip_kinds)
            && block_rows_is_empty(&self.grips)
            && block_rows_is_empty(&self.compatibility)
            && block_rows_is_empty(&self.attributes)
            && block_rows_is_empty(&self.authors)
    }
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
