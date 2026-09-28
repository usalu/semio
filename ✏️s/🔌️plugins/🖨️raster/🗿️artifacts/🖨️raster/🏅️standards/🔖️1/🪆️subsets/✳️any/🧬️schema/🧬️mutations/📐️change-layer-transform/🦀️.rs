//! 📐️ Exact, conflict-checked affine placement for pixel and group layers.
use crate::{RasterLayerNode,RasterLayerPatch,RasterMutation,RasterSnapshot,RasterTransform};
use crate::diff::{diff_patch_layer,RasterDiff};
use crate::standards::v1::subsets::any::schema::{find_layer,layer_transform};
#[derive(Clone,Debug,PartialEq,dsl::ToValue,dsl::FromValue,dsl::MutationLeaf)]
#[mutation_leaf(contract=::protocol)]
#[value(rename_all="camelCase")]
pub struct ChangeLayerTransform {pub layer_id:String,pub expected:RasterTransform,pub transform:RasterTransform}
pub fn validate(payload:&ChangeLayerTransform,base:&RasterSnapshot)->Result<(),&'static str>{
    let Some(layer @ (RasterLayerNode::Pixel {..}|RasterLayerNode::Group {..}))=find_layer(&base.layers,&payload.layer_id) else {return Err("mutation.target-missing");};
    if layer_transform(layer)!=&payload.expected {return Err("mutation.transform-conflict");}
    for value in [&payload.expected,&payload.transform] {semio_framework_pixels::compositing::inverse(value.as_affine()).map_err(|_|"mutation.transform-invalid")?;}
    Ok(())
}
impl protocol::MutationKind<RasterSnapshot,RasterMutation> for ChangeLayerTransform {
    const SEMANTICS:protocol::SemanticDescriptor=protocol::SemanticDescriptor {verb:"change",entity:"layer-transform",kind:"change-layer-transform",record:"ChangedLayerTransform"};
    fn diff(&self,base:&RasterSnapshot)->protocol::MutationOutcome<RasterDiff>{
        if let Err(code)=validate(self,base){return protocol::MutationOutcome::error(code,"Transform cannot be applied to this layer revision.",[self.layer_id.clone()]);}
        protocol::MutationOutcome::new(diff_patch_layer(&self.layer_id,RasterLayerPatch {transform:Some(self.transform.clone()),..Default::default()}))
    }
    fn inverse(&self,base:&RasterSnapshot)->Vec<RasterMutation>{
        if validate(self,base).is_err(){return Vec::new();}
        vec![RasterMutation::ChangeLayerTransform(Self {layer_id:self.layer_id.clone(),expected:self.transform.clone(),transform:self.expected.clone()})]
    }
    fn label(&self)->protocol::LocalizedLabel {protocol::LocalizedLabel::native("Transform layer","Ebene transformieren")}
    fn target(&self)->Vec<String>{vec![self.layer_id.clone()]}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
