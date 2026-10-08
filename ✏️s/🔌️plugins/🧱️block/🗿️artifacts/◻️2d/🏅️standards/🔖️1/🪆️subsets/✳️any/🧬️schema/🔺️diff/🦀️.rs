//! 🧬️ Block2d diff schema — a field-sparse, id-keyed delta over the artifact: sub-documents carry field patches, id-keyed lists carry
//! removed/added/patched rows (never a whole row, list or sub-document copy). `absorb` coalesces per field and per id, `inverse` restores exact base values.

use crate::{Block2dHandleKind, Block2dHandleTemplate, Block2dPresentation, Block2dSnapshot};
use semio_s_plugin_block::{BlockAttributesDelta, BlockAuthorsDelta, BlockCamera2dPatch, BlockCompatibilityDelta, BlockKindIdentityPatch, BlockMetaPatch, BlockOptionalNumber, BlockOptionalText, BlockPatchError, block_patch, block_patch_absorb, block_patch_apply, block_patch_between, block_patch_inverse, block_patch_is_empty, block_rows, block_rows_absorb, block_rows_apply, block_rows_between, block_rows_inverse, block_rows_is_empty};
use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Field-sparse delta for the block2d artifact.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact_schema(id = "s.block.block2d")]
pub struct Block2dDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub node_kind: Option<BlockKindIdentityPatch>,
    #[state(artifact)]
    pub presentation: Option<Block2dPresentationPatch>,
    #[state(artifact)]
    pub handle_kinds: Option<Block2dHandleKindsDelta>,
    #[state(artifact)]
    pub handles: Option<Block2dHandlesDelta>,
    #[state(artifact)]
    pub compatibility: Option<BlockCompatibilityDelta>,
    #[state(artifact)]
    pub attributes: Option<BlockAttributesDelta>,
    #[state(artifact)]
    pub authors: Option<BlockAuthorsDelta>,
    #[state(artifact)]
    pub camera2d: Option<BlockCamera2dPatch>,
    #[state(artifact)]
    pub meta: Option<BlockMetaPatch>,
}
//#endregion 🔖️Diff

//#region 🔖️Patches
block_patch!(test; /// 🖌️ Field patch over the rim presentation.
    Block2dPresentationPatch for Block2dPresentation { plain {  } optional { shape: BlockOptionalText, radius: BlockOptionalNumber, width: BlockOptionalNumber, height: BlockOptionalNumber, color: BlockOptionalText, icon_kind: BlockOptionalText } });
block_patch!(test; /// 🔘️ Field patch over a handle kind (its id is the row identity).
    Block2dHandleKindPatch for Block2dHandleKind { plain { name: String, label: String, color: String, default_wire_kind: String } optional {  } });
block_patch!(test; /// 🌱️ Field patch over a handle template (its id is the row identity).
    Block2dHandleTemplatePatch for Block2dHandleTemplate { plain { handle_kind: String, angle: f64, radius: f64 } optional {  } });
block_rows!(test; /// 📂 Row delta over the handle kinds.
    Block2dHandleKindsDelta, Block2dHandleKindsPatchEntry, Block2dHandleKind, Block2dHandleKindPatch, id);
block_rows!(test; /// 📂 Row delta over the handle templates.
    Block2dHandlesDelta, Block2dHandlesPatchEntry, Block2dHandleTemplate, Block2dHandleTemplatePatch, id);
//#endregion 🔖️Patches

//#region 🔖️Apply
/// 🛡️ Lifts a patch rejection into this crate's typed apply error.
fn lift(error: BlockPatchError) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new(error.code, error.message).at(error.target)
}

impl protocol::MutationDiff<Block2dSnapshot> for Block2dDiff {
    fn apply(&self, base: &Block2dSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Block2dSnapshot> {
        let mut next = base.clone();
        if let Some(schema) = &self.schema {
            next.schema.clone_from(schema);
        }
        next.node_kind = block_patch_apply(&self.node_kind, &base.node_kind, "nodeKind").map_err(lift)?;
        next.presentation = block_patch_apply(&self.presentation, &base.presentation, "presentation").map_err(lift)?;
        next.camera2d = block_patch_apply(&self.camera2d, &base.camera2d, "camera2d").map_err(lift)?;
        next.meta = block_patch_apply(&self.meta, &base.meta, "meta").map_err(lift)?;
        next.handle_kinds = block_rows_apply(&self.handle_kinds, &base.handle_kinds, "handleKinds").map_err(lift)?;
        next.handles = block_rows_apply(&self.handles, &base.handles, "handles").map_err(lift)?;
        next.compatibility = block_rows_apply(&self.compatibility, &base.compatibility, "compatibility").map_err(lift)?;
        next.attributes = block_rows_apply(&self.attributes, &base.attributes, "attributes").map_err(lift)?;
        next.authors = block_rows_apply(&self.authors, &base.authors, "authors").map_err(lift)?;
        Ok(next)
    }
    fn absorb(&mut self, later: Self) {
        if later.schema.is_some() {
            self.schema = later.schema;
        }
        block_patch_absorb(&mut self.node_kind, later.node_kind);
        block_patch_absorb(&mut self.presentation, later.presentation);
        block_patch_absorb(&mut self.camera2d, later.camera2d);
        block_patch_absorb(&mut self.meta, later.meta);
        block_rows_absorb(&mut self.handle_kinds, later.handle_kinds);
        block_rows_absorb(&mut self.handles, later.handles);
        block_rows_absorb(&mut self.compatibility, later.compatibility);
        block_rows_absorb(&mut self.attributes, later.attributes);
        block_rows_absorb(&mut self.authors, later.authors);
    }
}

impl protocol::DiffAlgebra<Block2dSnapshot> for Block2dDiff {
    fn inverse(&self, base: &Block2dSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            node_kind: block_patch_inverse(&self.node_kind, &base.node_kind),
            presentation: block_patch_inverse(&self.presentation, &base.presentation),
            camera2d: block_patch_inverse(&self.camera2d, &base.camera2d),
            meta: block_patch_inverse(&self.meta, &base.meta),
            handle_kinds: block_rows_inverse(&self.handle_kinds, &base.handle_kinds),
            handles: block_rows_inverse(&self.handles, &base.handles),
            compatibility: block_rows_inverse(&self.compatibility, &base.compatibility),
            attributes: block_rows_inverse(&self.attributes, &base.attributes),
            authors: block_rows_inverse(&self.authors, &base.authors),
        }
    }
    fn between(base: &Block2dSnapshot, other: &Block2dSnapshot) -> Self {
        Self {
            schema: (base.schema != other.schema).then(|| other.schema.clone()),
            node_kind: block_patch_between(&base.node_kind, &other.node_kind),
            presentation: block_patch_between(&base.presentation, &other.presentation),
            camera2d: block_patch_between(&base.camera2d, &other.camera2d),
            meta: block_patch_between(&base.meta, &other.meta),
            handle_kinds: block_rows_between(&base.handle_kinds, &other.handle_kinds),
            handles: block_rows_between(&base.handles, &other.handles),
            compatibility: block_rows_between(&base.compatibility, &other.compatibility),
            attributes: block_rows_between(&base.attributes, &other.attributes),
            authors: block_rows_between(&base.authors, &other.authors),
        }
    }
    fn is_empty(&self) -> bool {
        self.schema.is_none()
            && block_patch_is_empty(&self.node_kind)
            && block_patch_is_empty(&self.presentation)
            && block_patch_is_empty(&self.camera2d)
            && block_patch_is_empty(&self.meta)
            && block_rows_is_empty(&self.handle_kinds)
            && block_rows_is_empty(&self.handles)
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
