//! 📐️ Exact, conflict-checked affine placement for pixel and group layers.
use semio_s_artifact_raster_raster_model::{RasterLayerNode, RasterLayerPatch};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_s_artifact_raster_raster_diff::{diff_patch_layer, RasterDiff};
use semio_s_artifact_raster_raster_model::{find_layer, layer_transform};
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract=::protocol)]
#[value(rename_all="camelCase")]
pub struct ChangeLayerTransform {pub layer_id:String,pub expected:RasterTransform,pub transform:RasterTransform}
pub fn validate(payload:&ChangeLayerTransform,base:&RasterSnapshot)->Result<(),protocol::OutcomeCode>{
    let Some(layer @ (RasterLayerNode::Pixel {..}|RasterLayerNode::Group {..}))=find_layer(&base.layers,&payload.layer_id) else {return Err(protocol::OutcomeCode::TargetMissing);};
    if layer_transform(layer)!=&payload.expected {return Err(protocol::OutcomeCode::TargetMismatch);}
    for value in [&payload.expected,&payload.transform] {semio_framework_pixels::compositing::inverse(value.as_affine()).map_err(|_|protocol::OutcomeCode::Invariant)?;}
    Ok(())
}
impl<Op> protocol::MutationKind<RasterSnapshot, Op> for ChangeLayerTransform where Op: protocol::Mutation<RasterSnapshot, Diff = semio_s_artifact_raster_raster_diff::RasterDiff> + From<crate::change_layer_transform::ChangeLayerTransform> {
    const SEMANTICS:protocol::SemanticDescriptor=protocol::SemanticDescriptor {verb:"change",entity:"layer-transform",kind:"change-layer-transform",record:"ChangedLayerTransform"};
    fn diff(&self,base:&RasterSnapshot)->protocol::MutationOutcome<RasterDiff>{
        if let Err(code)=validate(self,base){return protocol::MutationOutcome::refuse(code,"Transform cannot be applied to this layer revision.",[self.layer_id.clone()]);}
        protocol::MutationOutcome::new(diff_patch_layer(&self.layer_id,RasterLayerPatch {transform:Some(self.transform.clone()),..Default::default()}))
    }
    fn inverse(&self,base:&RasterSnapshot)-> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        if validate(self,base).is_err(){return Vec::new();}
        vec![Op::from(Self {layer_id:self.layer_id.clone(),expected:self.transform.clone(),transform:self.expected.clone()})]
    
    })())
}
    fn label(&self)->semio_framework_ui_locale::LocalizedLabel {semio_framework_ui_locale::LocalizedLabel::native("Transform layer","Ebene transformieren")}
    fn target(&self)->Vec<String>{vec![self.layer_id.clone()]}
}
