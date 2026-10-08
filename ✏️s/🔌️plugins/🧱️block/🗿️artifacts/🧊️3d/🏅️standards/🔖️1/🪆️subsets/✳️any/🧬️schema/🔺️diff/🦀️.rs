//! 🧬️ Block3d diff schema — a field-sparse, id-keyed delta over the artifact: sub-documents carry field patches, id-keyed lists carry
//! removed/added/patched rows (never a whole row, list or sub-document copy). `absorb` coalesces per field and per id, `inverse` restores exact base values.

use crate::{Block3dSnapshot, Block3dVortexKind, Block3dVortexTemplate};
use semio_s_plugin_block::{BlockAttributesDelta, BlockAuthorsDelta, BlockCamera3dPatch, BlockCompatibilityDelta, BlockKindIdentityPatch, BlockMetaPatch, BlockOptionalText, BlockPatchError, BlockRepresentationsDelta, block_patch, block_patch_absorb, block_patch_apply, block_patch_between, block_patch_inverse, block_patch_is_empty, block_rows, block_rows_absorb, block_rows_apply, block_rows_between, block_rows_inverse, block_rows_is_empty};
use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Field-sparse delta for the block3d artifact.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact_schema(id = "s.block.block3d")]
pub struct Block3dDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub object_kind: Option<BlockKindIdentityPatch>,
    #[state(artifact)]
    pub representations: Option<BlockRepresentationsDelta>,
    #[state(artifact)]
    pub vortex_kinds: Option<Block3dVortexKindsDelta>,
    #[state(artifact)]
    pub vortices: Option<Block3dVorticesDelta>,
    #[state(artifact)]
    pub compatibility: Option<BlockCompatibilityDelta>,
    #[state(artifact)]
    pub attributes: Option<BlockAttributesDelta>,
    #[state(artifact)]
    pub authors: Option<BlockAuthorsDelta>,
    #[state(artifact)]
    pub camera3d: Option<BlockCamera3dPatch>,
    #[state(artifact)]
    pub meta: Option<BlockMetaPatch>,
}
//#endregion 🔖️Diff

//#region 🔖️Patches
block_patch!(test; /// 🔘️ Field patch over a vortex kind (its id is the row identity).
    Block3dVortexKindPatch for Block3dVortexKind { plain { name: String, label: String, color: String, default_cable_kind: String } optional {  } });
block_patch!(test; /// 🌱️ Field patch over a vortex template (its id is the row identity).
    Block3dVortexTemplatePatch for Block3dVortexTemplate { plain { vortex_kind: String, position: [f64; 3], direction: [f64; 3], radius: f64 } optional { label: BlockOptionalText } });
block_rows!(test; /// 📂 Row delta over the vortex kinds.
    Block3dVortexKindsDelta, Block3dVortexKindsPatchEntry, Block3dVortexKind, Block3dVortexKindPatch, id);
block_rows!(test; /// 📂 Row delta over the vortex templates.
    Block3dVorticesDelta, Block3dVorticesPatchEntry, Block3dVortexTemplate, Block3dVortexTemplatePatch, id);
//#endregion 🔖️Patches

//#region 🔖️Apply
/// 🛡️ Lifts a patch rejection into this crate's typed apply error.
fn lift(error: BlockPatchError) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new(error.code, error.message).at(error.target)
}

impl protocol::MutationDiff<Block3dSnapshot> for Block3dDiff {
    fn apply(&self, base: &Block3dSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Block3dSnapshot> {
        let mut next = base.clone();
        if let Some(schema) = &self.schema {
            next.schema.clone_from(schema);
        }
        next.object_kind = block_patch_apply(&self.object_kind, &base.object_kind, "objectKind").map_err(lift)?;
        next.camera3d = block_patch_apply(&self.camera3d, &base.camera3d, "camera3d").map_err(lift)?;
        next.meta = block_patch_apply(&self.meta, &base.meta, "meta").map_err(lift)?;
        next.representations = block_rows_apply(&self.representations, &base.representations, "representations").map_err(lift)?;
        if self.vortex_kinds.is_some() {
            let kinds = block_rows_apply(&self.vortex_kinds, &crate::vortex_kinds_of(base), "vortexKinds").map_err(lift)?;
            crate::set_vortex_kinds(&mut next, &kinds);
        }
        next.vortices = block_rows_apply(&self.vortices, &base.vortices, "vortices").map_err(lift)?;
        next.compatibility = block_rows_apply(&self.compatibility, &base.compatibility, "compatibility").map_err(lift)?;
        next.attributes = block_rows_apply(&self.attributes, &base.attributes, "attributes").map_err(lift)?;
        next.authors = block_rows_apply(&self.authors, &base.authors, "authors").map_err(lift)?;
        Ok(next)
    }
    fn absorb(&mut self, later: Self) {
        if later.schema.is_some() {
            self.schema = later.schema;
        }
        block_patch_absorb(&mut self.object_kind, later.object_kind);
        block_patch_absorb(&mut self.camera3d, later.camera3d);
        block_patch_absorb(&mut self.meta, later.meta);
        block_rows_absorb(&mut self.representations, later.representations);
        block_rows_absorb(&mut self.vortex_kinds, later.vortex_kinds);
        block_rows_absorb(&mut self.vortices, later.vortices);
        block_rows_absorb(&mut self.compatibility, later.compatibility);
        block_rows_absorb(&mut self.attributes, later.attributes);
        block_rows_absorb(&mut self.authors, later.authors);
    }
}

impl protocol::DiffAlgebra<Block3dSnapshot> for Block3dDiff {
    fn inverse(&self, base: &Block3dSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            object_kind: block_patch_inverse(&self.object_kind, &base.object_kind),
            camera3d: block_patch_inverse(&self.camera3d, &base.camera3d),
            meta: block_patch_inverse(&self.meta, &base.meta),
            representations: block_rows_inverse(&self.representations, &base.representations),
            vortex_kinds: block_rows_inverse(&self.vortex_kinds, &crate::vortex_kinds_of(base)),
            vortices: block_rows_inverse(&self.vortices, &base.vortices),
            compatibility: block_rows_inverse(&self.compatibility, &base.compatibility),
            attributes: block_rows_inverse(&self.attributes, &base.attributes),
            authors: block_rows_inverse(&self.authors, &base.authors),
        }
    }
    fn between(base: &Block3dSnapshot, other: &Block3dSnapshot) -> Self {
        Self {
            schema: (base.schema != other.schema).then(|| other.schema.clone()),
            object_kind: block_patch_between(&base.object_kind, &other.object_kind),
            camera3d: block_patch_between(&base.camera3d, &other.camera3d),
            meta: block_patch_between(&base.meta, &other.meta),
            representations: block_rows_between(&base.representations, &other.representations),
            vortex_kinds: block_rows_between(&crate::vortex_kinds_of(base), &crate::vortex_kinds_of(other)),
            vortices: block_rows_between(&base.vortices, &other.vortices),
            compatibility: block_rows_between(&base.compatibility, &other.compatibility),
            attributes: block_rows_between(&base.attributes, &other.attributes),
            authors: block_rows_between(&base.authors, &other.authors),
        }
    }
    fn is_empty(&self) -> bool {
        self.schema.is_none()
            && block_patch_is_empty(&self.object_kind)
            && block_patch_is_empty(&self.camera3d)
            && block_patch_is_empty(&self.meta)
            && block_rows_is_empty(&self.representations)
            && block_rows_is_empty(&self.vortex_kinds)
            && block_rows_is_empty(&self.vortices)
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
