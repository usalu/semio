//! 🎨️ Replace only a layer's pixels, with optimistic image revision validation.
use crate::{RasterLayerNode, RasterLayerPatch, RasterPixelContent, RasterTransform, RasterSnapshot, RasterMutation};
use crate::diff::{diff_patch_layer, RasterDiff};
use crate::standards::v1::subsets::any::schema::find_layer;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeLayerPixels {
    pub layer_id: String,
    pub expected_image_key: Option<String>,
    pub content: RasterPixelContent,
    pub transform: Option<RasterTransform>,
}

pub fn validate(payload: &ChangeLayerPixels, base: &RasterSnapshot) -> Result<(), protocol::OutcomeCode> {
    let Some(RasterLayerNode::Pixel { image_key, .. }) = find_layer(&base.layers, &payload.layer_id) else { return Err(protocol::OutcomeCode::TargetMissing); };
    if image_key != &payload.expected_image_key { return Err(protocol::OutcomeCode::TargetMismatch); }
    if let Some(key) = &payload.content.image_key {
        if !base.assets.contains_key(key) { return Err(protocol::OutcomeCode::TargetMissing); }
    }
    if payload.content.width.is_some_and(|n| n == 0 || n > 16384) || payload.content.height.is_some_and(|n| n == 0 || n > 16384) { return Err(protocol::OutcomeCode::Invariant); }
    if let Some(value) = &payload.transform {
        semio_framework_pixels::compositing::inverse(value.as_affine()).map_err(|_| protocol::OutcomeCode::Invariant)?;
    }
    Ok(())
}

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for ChangeLayerPixels {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "layer-pixels", kind: "change-layer-pixels", record: "ChangedLayerPixels" };

    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        if let Err(code) = validate(self, base) { return protocol::MutationOutcome::refuse(code, "Pixel content cannot be applied to this image revision.", [self.layer_id.clone()]); }
        protocol::MutationOutcome::new(diff_patch_layer(&self.layer_id, RasterLayerPatch { pixel_content: Some(self.content.clone()), transform: self.transform.clone(), ..Default::default() }))
    }

    fn inverse(&self, base: &RasterSnapshot) -> Result<Vec<RasterMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        let Some(RasterLayerNode::Pixel { image_key, width, height, transform, .. }) = find_layer(&base.layers, &self.layer_id) else { return Vec::new(); };
        vec![RasterMutation::ChangeLayerPixels(Self { layer_id: self.layer_id.clone(), expected_image_key: self.content.image_key.clone(), content: RasterPixelContent { image_key: image_key.clone(), width: *width, height: *height }, transform: self.transform.as_ref().map(|_| transform.clone()) })]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Edit layer pixels", "Ebenenpixel bearbeiten") }
    fn target(&self) -> Vec<String> { vec![self.layer_id.clone()] }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
