//! 🎨️ Replace only a layer's pixels, with optimistic image revision validation.
use crate::{RasterLayerNode, RasterLayerPatch, RasterPixelContent, RasterTransform, RasterSnapshot, RasterMutation};
use crate::diff::{diff_patch_layer, RasterDiff};
use crate::standards::v1::subsets::any::schema::find_layer;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeLayerPixels {
    pub layer_id: String,
    pub expected_image_key: Option<String>,
    pub content: RasterPixelContent,
    pub transform: Option<RasterTransform>,
}

pub fn validate(payload: &ChangeLayerPixels, base: &RasterSnapshot) -> Result<(), &'static str> {
    let Some(RasterLayerNode::Pixel { image_key, .. }) = find_layer(&base.layers, &payload.layer_id) else { return Err("mutation.target-missing"); };
    if image_key != &payload.expected_image_key { return Err("mutation.image-conflict"); }
    if let Some(key) = &payload.content.image_key {
        if !base.assets.contains_key(key) { return Err("mutation.asset-missing"); }
    }
    if payload.content.width.is_some_and(|n| n == 0 || n > 16384) || payload.content.height.is_some_and(|n| n == 0 || n > 16384) { return Err("mutation.invariant"); }
    if let Some(value) = &payload.transform {
        if [value.x, value.y, value.scale_x, value.scale_y, value.rotation].iter().any(|n| !n.is_finite()) || value.scale_x == 0.0 || value.scale_y == 0.0 { return Err("mutation.invariant"); }
    }
    Ok(())
}

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for ChangeLayerPixels {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "layer-pixels", kind: "change-layer-pixels", record: "ChangedLayerPixels" };

    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        if let Err(code) = validate(self, base) { return protocol::MutationOutcome::error(code, "Pixel content cannot be applied to this image revision.", [self.layer_id.clone()]); }
        protocol::MutationOutcome::new(diff_patch_layer(&self.layer_id, RasterLayerPatch { pixel_content: Some(self.content.clone()), pixel_transform: self.transform.clone(), ..Default::default() }))
    }

    fn inverse(&self, base: &RasterSnapshot) -> Vec<RasterMutation> {
        let Some(RasterLayerNode::Pixel { image_key, width, height, transform, .. }) = find_layer(&base.layers, &self.layer_id) else { return Vec::new(); };
        vec![RasterMutation::ChangeLayerPixels(Self { layer_id: self.layer_id.clone(), expected_image_key: self.content.image_key.clone(), content: RasterPixelContent { image_key: image_key.clone(), width: *width, height: *height }, transform: self.transform.as_ref().map(|_| transform.clone()) })]
    }

    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native("Edit layer pixels", "Ebenenpixel bearbeiten") }
    fn target(&self) -> Vec<String> { vec![self.layer_id.clone()] }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
