//! 🖼️ Image asset admission and removal use sparse document deltas and exact inverses.
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use crate::remove_image_asset::mutation::remove_image_asset;
use semio_s_artifact_draw_drawing_diff::{DrawingDiff, DrawingAssetsDelta};
#[derive(Clone,Debug,PartialEq,semio_framework_value::RetainedClone,semio_framework_value::RetireOwned,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_dsl_record_derive::DslRecord,dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract=::protocol)]
#[cfg_attr(any(test, feature = "testkit"),derive(serde::Serialize,serde::Deserialize))]
#[value(rename_all="camelCase")]
#[cfg_attr(any(test, feature = "testkit"),serde(rename_all="camelCase"))]
#[dsl(keyword="import-image-asset")]
pub struct ImportImageAsset {pub asset_id:semio_framework_value::paged::PagedUtf8<{usize::MAX}>,#[dsl(block)] pub asset:DrawingImageAsset}
pub fn import_image_asset<Op: From<ImportImageAsset>>(asset_id:String,asset:DrawingImageAsset) -> Op {Op::from(ImportImageAsset {asset_id:asset_id.into(),asset})}
impl<Op> protocol::MutationKind<DrawingSnapshot, Op> for ImportImageAsset where Op: protocol::Mutation<DrawingSnapshot, Diff = semio_s_artifact_draw_drawing_diff::DrawingDiff> + From<crate::remove_image_asset::mutation::RemoveImageAsset> {
    const SEMANTICS:protocol::SemanticDescriptor=protocol::SemanticDescriptor {verb:"add",entity:"image-asset",kind:"import-image-asset",record:"AddedImageAsset"};
    fn diff(&self,base:&DrawingSnapshot)->protocol::MutationOutcome<DrawingDiff> {
        if self.asset_id.is_empty() || self.asset_id.len()>4096 || base.assets.get(&self.asset_id).is_some() || self.asset.width==0 || self.asset.height==0 || (self.asset.width as usize).checked_mul(self.asset.height as usize)!=Some(self.asset.samples.len()) {return protocol::MutationOutcome::fatal("mutation.invariant","Invalid or occupied image asset identity or sample dimensions",[self.asset_id.to_string_owner()]);}
        protocol::MutationOutcome::new(DrawingDiff {assets:Some(DrawingAssetsDelta {entries:[(self.asset_id.to_string_owner(),Some(self.asset.clone()))].into()}),..Default::default()})
    }
    fn inverse(&self,base:&DrawingSnapshot)->Result<Vec<Op>,semio_framework_value::ValueError> {Ok(if base.assets.get(&self.asset_id).is_none(){vec![remove_image_asset(self.asset_id.to_string_owner())]}else{Vec::new()})}
    fn label(&self)->semio_framework_ui_locale::LocalizedLabel {semio_framework_ui_locale::LocalizedLabel::native("Import image asset","Bildressource importieren")}
    fn target(&self)->Vec<String>{vec![self.asset_id.to_string_owner()]}
}
