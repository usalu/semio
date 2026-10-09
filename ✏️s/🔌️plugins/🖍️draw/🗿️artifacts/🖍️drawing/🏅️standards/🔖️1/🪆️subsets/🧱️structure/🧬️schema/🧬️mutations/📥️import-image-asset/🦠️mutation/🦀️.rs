//! 🖼️ Image asset admission and removal use sparse document deltas and exact inverses.
use crate::{DrawingSnapshot,DrawingImageAsset};
use crate::mutations::{DrawingMutation,remove_image_asset};
use crate::diff::{DrawingDiff,DrawingAssetsDelta};
#[derive(Clone,Debug,PartialEq,semio_framework_value::RetainedClone,semio_framework_value::RetireOwned,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_dsl_record_derive::DslRecord,dsl::MutationLeaf)]
#[mutation_leaf(contract=::protocol)]
#[cfg_attr(test,derive(serde::Serialize,serde::Deserialize))]
#[value(rename_all="camelCase")]
#[cfg_attr(test,serde(rename_all="camelCase"))]
#[dsl(keyword="import-image-asset")]
pub struct ImportImageAsset {pub asset_id:semio_framework_value::paged::PagedUtf8<{usize::MAX}>,#[dsl(block)] pub asset:DrawingImageAsset}
pub fn import_image_asset(asset_id:String,asset:DrawingImageAsset)->DrawingMutation {DrawingMutation::ImportImageAsset(ImportImageAsset {asset_id:asset_id.into(),asset})}
impl protocol::MutationKind<DrawingSnapshot,DrawingMutation> for ImportImageAsset {
    const SEMANTICS:protocol::SemanticDescriptor=protocol::SemanticDescriptor {verb:"import",entity:"image-asset",kind:"import-image-asset",record:"ImportedImageAsset"};
    fn diff(&self,base:&DrawingSnapshot)->protocol::MutationOutcome<DrawingDiff> {
        if self.asset_id.is_empty() || self.asset_id.len()>4096 || base.assets.get(&self.asset_id).is_some() || self.asset.width==0 || self.asset.height==0 || (self.asset.width as usize).checked_mul(self.asset.height as usize)!=Some(self.asset.samples.len()) {return protocol::MutationOutcome::fatal("mutation.invariant","Invalid or occupied image asset identity or sample dimensions",[self.asset_id.to_string_owner()]);}
        protocol::MutationOutcome::new(DrawingDiff {assets:Some(DrawingAssetsDelta {entries:[(self.asset_id.to_string_owner(),Some(self.asset.clone()))].into()}),..Default::default()})
    }
    fn inverse(&self,base:&DrawingSnapshot)->Result<Vec<DrawingMutation>,semio_framework_value::ValueError> {Ok(if base.assets.get(&self.asset_id).is_none(){vec![remove_image_asset(self.asset_id.to_string_owner())]}else{Vec::new()})}
    fn label(&self)->semio_framework_ui_locale::LocalizedLabel {semio_framework_ui_locale::LocalizedLabel::native("Import image asset","Bildressource importieren")}
    fn target(&self)->Vec<String>{vec![self.asset_id.to_string_owner()]}
}
