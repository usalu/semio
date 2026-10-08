//! 🧬️ Block3d diff schema — a field-sparse, id-keyed delta over the artifact: sub-documents carry field patches, id-keyed lists carry
//! removed/added/patched rows (never a whole row, list or sub-document copy). `absorb` coalesces per field and per id, `inverse` restores exact base values.

use crate::{Block3dSnapshot, Block3dVortexKind, Block3dVortexTemplate};
use semio_s_plugin_block::{BlockAttributesDelta, BlockAuthorsDelta, BlockCamera3dPatch, BlockCompatibilityDelta, BlockKindIdentityPatch, BlockMetaPatch, BlockOptionalText, BlockPatchError, BlockRepresentationsDelta, block_patch, block_patch_absorb, block_patch_apply, block_patch_inverse, block_patch_is_empty};
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
    pub representations: BlockRepresentationsDelta,
    #[state(artifact)]
    pub vortex_kinds: Block3dVortexKindsDelta,
    #[state(artifact)]
    pub vortices: Block3dVorticesDelta,
    #[state(artifact)]
    pub compatibility: BlockCompatibilityDelta,
    #[state(artifact)]
    pub attributes: BlockAttributesDelta,
    #[state(artifact)]
    pub authors: BlockAuthorsDelta,
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
protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📂 Row delta over the vortex kinds.
    pub Block3dVortexKindsDelta { removal: Block3dVortexKindsRemoval, insertion: Block3dVortexKindsInsertion, relocation: Block3dVortexKindsRelocation, modification: Block3dVortexKindsPatchEntry, row: Block3dVortexKind, patch: Block3dVortexKindPatch, key: id, values_only }
}
protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📂 Row delta over the vortex templates.
    pub Block3dVorticesDelta { removal: Block3dVorticesRemoval, insertion: Block3dVorticesInsertion, relocation: Block3dVorticesRelocation, modification: Block3dVorticesPatchEntry, row: Block3dVortexTemplate, patch: Block3dVortexTemplatePatch, key: id, values_only }
}
//#endregion 🔖️Patches

//#region 🔖️Apply
/// 🛡️ Lifts a patch rejection into this crate's typed apply error.
fn lift(error: BlockPatchError) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new(error.code, error.message).at(error.target)
}

impl protocol::MutationDiff<Block3dSnapshot> for Block3dDiff {
    fn apply(&self, base: &Block3dSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Block3dSnapshot> {
        let mut next = base.clone();
        if let Some(schema) = &self.schema {
            next.schema.clone_from(schema);
        }
        next.object_kind = block_patch_apply(&self.object_kind, &base.object_kind, "objectKind").map_err(lift)?;
        next.camera3d = block_patch_apply(&self.camera3d, &base.camera3d, "camera3d").map_err(lift)?;
        next.meta = block_patch_apply(&self.meta, &base.meta, "meta").map_err(lift)?;
        next.representations = self.representations.commit_onto(&base.representations, capability).map_err(|error| error.under(["representations"]))?;
        if !self.vortex_kinds.is_empty() {
            let kinds = self.vortex_kinds.commit_onto(&crate::vortex_kinds_of(base), capability).map_err(|error| error.under(["vortexKinds"]))?;
            crate::set_vortex_kinds(&mut next, &kinds);
        }
        next.vortices = self.vortices.commit_onto(&base.vortices, capability).map_err(|error| error.under(["vortices"]))?;
        next.compatibility = self.compatibility.commit_onto(&base.compatibility, capability).map_err(|error| error.under(["compatibility"]))?;
        next.attributes = self.attributes.commit_onto(&base.attributes, capability).map_err(|error| error.under(["attributes"]))?;
        next.authors = self.authors.commit_onto(&base.authors, capability).map_err(|error| error.under(["authors"]))?;
        Ok(next)
    }
    fn absorb(&mut self, later: Self) {
        if later.schema.is_some() {
            self.schema = later.schema;
        }
        block_patch_absorb(&mut self.object_kind, later.object_kind);
        block_patch_absorb(&mut self.camera3d, later.camera3d);
        block_patch_absorb(&mut self.meta, later.meta);
        self.representations.absorb(later.representations);
        self.vortex_kinds.absorb(later.vortex_kinds);
        self.vortices.absorb(later.vortices);
        self.compatibility.absorb(later.compatibility);
        self.attributes.absorb(later.attributes);
        self.authors.absorb(later.authors);
    }
}

impl protocol::DiffAlgebra<Block3dSnapshot> for Block3dDiff {
    fn inverse(&self, base: &Block3dSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            object_kind: block_patch_inverse(&self.object_kind, &base.object_kind),
            camera3d: block_patch_inverse(&self.camera3d, &base.camera3d),
            meta: block_patch_inverse(&self.meta, &base.meta),
            representations: self.representations.inverse(&base.representations),
            vortex_kinds: self.vortex_kinds.inverse(&crate::vortex_kinds_of(base)),
            vortices: self.vortices.inverse(&base.vortices),
            compatibility: self.compatibility.inverse(&base.compatibility),
            attributes: self.attributes.inverse(&base.attributes),
            authors: self.authors.inverse(&base.authors),
        }
    }
    fn is_empty(&self) -> bool {
        self.schema.is_none()
            && block_patch_is_empty(&self.object_kind)
            && block_patch_is_empty(&self.camera3d)
            && block_patch_is_empty(&self.meta)
            && self.representations.is_empty()
            && self.vortex_kinds.is_empty()
            && self.vortices.is_empty()
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
