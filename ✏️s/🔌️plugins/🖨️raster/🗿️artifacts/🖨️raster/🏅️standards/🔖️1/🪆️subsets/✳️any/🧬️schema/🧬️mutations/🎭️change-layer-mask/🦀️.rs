//! 🎭️ Undoable mask replacement guarded by its prior revision.
use crate::{RasterLayerMask, RasterLayerNode, RasterLayerPatch, RasterMaskContent, RasterMutation, RasterSnapshot};
use crate::diff::{diff_patch_layer, RasterDiff};
use crate::standards::v1::subsets::any::schema::find_layer;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeLayerMask {
    pub layer_id: String,
    pub expected: Option<RasterLayerMask>,
    pub mask: Option<RasterLayerMask>,
}

pub fn validate(payload: &ChangeLayerMask, base: &RasterSnapshot) -> Result<(), &'static str> {
    let Some(RasterLayerNode::Pixel { mask, .. } | RasterLayerNode::Group { mask, .. }) = find_layer(&base.layers, &payload.layer_id) else { return Err("mutation.target-missing"); };
    if mask != &payload.expected { return Err("mutation.target-mismatch"); }
    if let Some(mask) = &payload.mask {
        if let Some(key) = &mask.image_key {
            if key.is_empty() || !base.assets.contains_key(key) { return Err("mutation.target-missing"); }
        }
        if [mask.width, mask.height].iter().flatten().any(|&n| n == 0 || n > 16384) || mask.width.zip(mask.height).is_some_and(|(w, h)| u64::from(w) * u64::from(h) > 16_777_216) { return Err("mutation.invariant"); }
        let t = &mask.transform;
        semio_framework_pixels::compositing::inverse(t.as_affine()).map_err(|_| "mutation.invariant")?;
    }
    Ok(())
}

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for ChangeLayerMask {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "layer-mask", kind: "change-layer-mask", record: "ChangedLayerMask" };

    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        if let Err(code) = validate(self, base) { return protocol::MutationOutcome::refuse(code, "Mask cannot be applied to this layer revision.", [self.layer_id.clone()]); }
        protocol::MutationOutcome::new(diff_patch_layer(&self.layer_id, RasterLayerPatch { mask_content: Some(RasterMaskContent { mask: self.mask.clone() }), ..Default::default() }))
    }

    fn inverse(&self, base: &RasterSnapshot) -> Vec<RasterMutation> {
        let Some(RasterLayerNode::Pixel { mask, .. } | RasterLayerNode::Group { mask, .. }) = find_layer(&base.layers, &self.layer_id) else { return Vec::new(); };
        vec![RasterMutation::ChangeLayerMask(Self { layer_id: self.layer_id.clone(), expected: self.mask.clone(), mask: mask.clone() })]
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Change layer mask", "Ebenenmaske ändern") }
    fn target(&self) -> Vec<String> { vec![self.layer_id.clone()] }
}
