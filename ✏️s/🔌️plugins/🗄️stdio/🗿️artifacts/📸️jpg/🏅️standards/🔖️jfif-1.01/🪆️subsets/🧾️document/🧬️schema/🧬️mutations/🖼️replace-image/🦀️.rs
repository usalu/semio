//! 🖼️ Replaces one owned JPEG image, preserving exact semantic metadata.
use crate::{JpgSnapshot,JpgImage,JpgDiff,JpgMutation};
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone,Debug,PartialEq,value_derive::ToValue,value_derive::FromValue,dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract=::protocol)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct ReplaceImage {pub image:JpgImage}
impl protocol::MutationKind<JpgSnapshot,JpgMutation> for ReplaceImage {
 const SEMANTICS:protocol::SemanticDescriptor=protocol::SemanticDescriptor{verb:"replace",entity:"image",kind:"replace-image",record:"ReplaceImage"};
 fn diff(&self,base:&JpgSnapshot)->protocol::MutationOutcome<JpgDiff>{
  protocol::MutationOutcome::new(crate::schema::diff::jpg_image_diff(&base.image,&self.image))
 }
 fn inverse(&self,base:&JpgSnapshot)->Result<Vec<JpgMutation>,semio_framework_value::ValueError>{Ok(if base.image==self.image{Vec::new()}else{vec![JpgMutation::ReplaceImage(Self{image:base.image.clone()})]})}
 fn label(&self)->semio_framework_ui_locale::LocalizedLabel{semio_framework_ui_locale::LocalizedLabel::native("Replace image","Bild ersetzen")}
 fn target(&self)->Vec<String>{vec!["image".into()]}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
