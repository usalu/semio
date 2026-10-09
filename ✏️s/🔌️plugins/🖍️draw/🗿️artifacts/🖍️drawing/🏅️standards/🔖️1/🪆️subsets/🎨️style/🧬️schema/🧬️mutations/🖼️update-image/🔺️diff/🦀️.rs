//! 🖼️ Sparse image facet delta with finite positive dimensions.
pub fn diff(payload:&super::mutation::UpdateImage,base:&crate::DrawingSnapshot)->protocol::MutationOutcome<crate::DrawingDiff> {
    let image=match crate::schema::find_drawing_layer(base,&payload.layer_id) {
        Some(crate::DrawingLayerNode::Image(image))=>image,
        Some(_)=>return protocol::MutationOutcome::error("mutation.target-mismatch","The target layer is not an image",[payload.layer_id.to_string_owner()]),
        None=>return protocol::MutationOutcome::error("mutation.target-missing","The target image layer does not exist",[payload.layer_id.to_string_owner()]),
    };
    if ![payload.width,payload.height].iter().all(|value|value.is_finite()&&*value>0.0) {return protocol::MutationOutcome::fatal("mutation.invariant","Image dimensions must be finite and positive",[payload.layer_id.to_string_owner()]);}
    let patch=crate::diff::DrawingLayerPatch {
        image_key:(image.image_key!=payload.image_key).then(||payload.image_key.to_string_owner()),
        image_width:(image.width!=payload.width).then_some(payload.width),
        image_height:(image.height!=payload.height).then_some(payload.height),
        ..Default::default()
    };
    if patch.is_empty(){return protocol::MutationOutcome::empty();}
    protocol::MutationOutcome::new(crate::diff::layer_base_patch(&payload.layer_id,patch))
}
