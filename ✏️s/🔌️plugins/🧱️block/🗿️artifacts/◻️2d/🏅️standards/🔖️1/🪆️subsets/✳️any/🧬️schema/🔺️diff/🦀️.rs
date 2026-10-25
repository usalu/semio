//! 🧬️ Block2d diff schema — a field-sparse, id-keyed delta over the artifact: sub-documents carry field patches, id-keyed lists carry
//! removed/added/patched rows (never a whole row, list or sub-document copy). `absorb` coalesces per field and per id, `inverse` restores exact base values.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_block_2d_model::{Block2dHandleKind, Block2dHandleTemplate, Block2dPresentation, Block2dSnapshot};
use semio_s_plugin_block::{BlockAttributesDelta, BlockAuthorsDelta, BlockCamera2dPatch, BlockCompatibilityDelta, BlockKindIdentityPatch, BlockMetaPatch, BlockOptionalNumber, BlockOptionalText, BlockPatchError, block_patch, block_patch_absorb, block_patch_apply, block_patch_inverse, block_patch_is_empty};
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
    pub handle_kinds: Block2dHandleKindsDelta,
    #[state(artifact)]
    pub handles: Block2dHandlesDelta,
    #[state(artifact)]
    pub compatibility: BlockCompatibilityDelta,
    #[state(artifact)]
    pub attributes: BlockAttributesDelta,
    #[state(artifact)]
    pub authors: BlockAuthorsDelta,
    #[state(artifact)]
    pub camera2d: Option<BlockCamera2dPatch>,
    #[state(artifact)]
    pub meta: Option<BlockMetaPatch>,
}
//#endregion 🔖️Diff

//#region 🔖️Patches
block_patch!(test; /// 🖌️ Field patch over the rim presentation.
    Block2dPresentationPatch for Block2dPresentation { plain {  } optional { shape: BlockOptionalText, radius: BlockOptionalNumber, width: BlockOptionalNumber, height: BlockOptionalNumber, color: BlockOptionalText, icon_kind: BlockOptionalText } });
semio_framework_value::artifact_retire_struct!(Block2dPresentationPatch { shape, radius, width, height, color, icon_kind });
block_patch!(test; /// 🔘️ Field patch over a handle kind (its id is the row identity).
    Block2dHandleKindPatch for Block2dHandleKind { plain { name: String, label: String, color: String, default_wire_kind: String } optional {  } });
semio_framework_value::artifact_retire_struct!(Block2dHandleKindPatch { name, label, color, default_wire_kind });
block_patch!(test; /// 🌱️ Field patch over a handle template (its id is the row identity).
    Block2dHandleTemplatePatch for Block2dHandleTemplate { plain { handle_kind: String, angle: f64, radius: f64 } optional {  } });
semio_framework_value::artifact_retire_struct!(Block2dHandleTemplatePatch { handle_kind, angle, radius });
protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📂 Row delta over the handle kinds.
    pub Block2dHandleKindsDelta { removal: Block2dHandleKindsRemoval, insertion: Block2dHandleKindsInsertion, relocation: Block2dHandleKindsRelocation, modification: Block2dHandleKindsPatchEntry, row: Block2dHandleKind, patch: Block2dHandleKindPatch, list: Vec<Block2dHandleKind>, key: String = by Block2dHandleKindKeys }
}

/// 🔑️ Keys the foreign `Block2dHandleKind` rows of their list delta by `id`.
pub(crate) struct Block2dHandleKindKeys;

impl protocol::list_delta::KeyOf<Block2dHandleKind> for Block2dHandleKindKeys {
    type Key = String;
    fn key_of(row: &Block2dHandleKind) -> String { row.id.clone() }
}
protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📂 Row delta over the handle templates.
    pub Block2dHandlesDelta { removal: Block2dHandlesRemoval, insertion: Block2dHandlesInsertion, relocation: Block2dHandlesRelocation, modification: Block2dHandlesPatchEntry, row: Block2dHandleTemplate, patch: Block2dHandleTemplatePatch, list: Vec<Block2dHandleTemplate>, key: String = by Block2dHandleTemplateKeys }
}

/// 🔑️ Keys the foreign `Block2dHandleTemplate` rows of their list delta by `id`.
pub(crate) struct Block2dHandleTemplateKeys;

impl protocol::list_delta::KeyOf<Block2dHandleTemplate> for Block2dHandleTemplateKeys {
    type Key = String;
    fn key_of(row: &Block2dHandleTemplate) -> String { row.id.clone() }
}
//#endregion 🔖️Patches

//#region 🔖️Apply
/// 🛡️ Lifts a patch rejection into this crate's typed apply error.
fn lift(error: BlockPatchError) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new(error.code, error.message).at(error.target)
}

impl protocol::MutationDiff<Block2dSnapshot> for Block2dDiff {}

impl protocol::ColdMutationDiff<Block2dSnapshot> for Block2dDiff {
    fn absorb(&mut self, later: Self) {
        if later.schema.is_some() {
            self.schema = later.schema;
        }
        block_patch_absorb(&mut self.node_kind, later.node_kind);
        block_patch_absorb(&mut self.presentation, later.presentation);
        block_patch_absorb(&mut self.camera2d, later.camera2d);
        block_patch_absorb(&mut self.meta, later.meta);
        self.handle_kinds.absorb(later.handle_kinds);
        self.handles.absorb(later.handles);
        self.compatibility.absorb(later.compatibility);
        self.attributes.absorb(later.attributes);
        self.authors.absorb(later.authors);
    }

    fn apply(&self, unpublished: &mut Block2dSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let base = &*unpublished;
        let mut next = base.clone();
        if let Some(schema) = &self.schema {
            next.schema.clone_from(schema);
        }
        next.node_kind = block_patch_apply(&self.node_kind, &base.node_kind, "nodeKind").map_err(lift)?;
        next.presentation = block_patch_apply(&self.presentation, &base.presentation, "presentation").map_err(lift)?;
        next.camera2d = block_patch_apply(&self.camera2d, &base.camera2d, "camera2d").map_err(lift)?;
        next.meta = block_patch_apply(&self.meta, &base.meta, "meta").map_err(lift)?;
        next.handle_kinds = self.handle_kinds.commit_onto(&base.handle_kinds, capability).map_err(|error| error.under(["handleKinds"]))?;
        next.handles = self.handles.commit_onto(&base.handles, capability).map_err(|error| error.under(["handles"]))?;
        next.compatibility = self.compatibility.commit_onto(&base.compatibility, capability).map_err(|error| error.under(["compatibility"]))?;
        next.attributes = self.attributes.commit_onto(&base.attributes, capability).map_err(|error| error.under(["attributes"]))?;
        next.authors = self.authors.commit_onto(&base.authors, capability).map_err(|error| error.under(["authors"]))?;
        *unpublished = next;
        Ok(())
    }
}

impl protocol::DiffAlgebra<Block2dSnapshot> for Block2dDiff {
    fn is_empty(&self) -> bool {
        self.schema.is_none()
            && block_patch_is_empty(&self.node_kind)
            && block_patch_is_empty(&self.presentation)
            && block_patch_is_empty(&self.camera2d)
            && block_patch_is_empty(&self.meta)
            && self.handle_kinds.is_empty()
            && self.handles.is_empty()
            && self.compatibility.is_empty()
            && self.attributes.is_empty()
            && self.authors.is_empty()
    }
}

impl protocol::ColdDiffAlgebra<Block2dSnapshot> for Block2dDiff {
    fn inverse(&self, base: &Block2dSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            node_kind: block_patch_inverse(&self.node_kind, &base.node_kind),
            presentation: block_patch_inverse(&self.presentation, &base.presentation),
            camera2d: block_patch_inverse(&self.camera2d, &base.camera2d),
            meta: block_patch_inverse(&self.meta, &base.meta),
            handle_kinds: self.handle_kinds.inverse(&base.handle_kinds),
            handles: self.handles.inverse(&base.handles),
            compatibility: self.compatibility.inverse(&base.compatibility),
            attributes: self.attributes.inverse(&base.attributes),
            authors: self.authors.inverse(&base.authors),
        }
    }
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `diff` schema facet.
pub const DIFF_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves
