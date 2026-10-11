//! 🎭️ Undoable mask replacement guarded by its prior revision.
use crate::{RasterLayerMask, RasterLayerNode, RasterLayerPatch, RasterMaskContent, RasterMutation, RasterSnapshot};
use crate::diff::{diff_patch_layer, RasterDiff};
use crate::standards::v1::subsets::any::schema::find_layer;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeLayerMask {
    pub layer_id: String,
    pub expected: Option<RasterLayerMask>,
    pub mask: Option<RasterLayerMask>,
}

pub fn validate(payload: &ChangeLayerMask, base: &RasterSnapshot) -> Result<(), protocol::OutcomeCode> {
    let Some(RasterLayerNode::Pixel { mask, .. } | RasterLayerNode::Group { mask, .. }) = find_layer(&base.layers, &payload.layer_id) else { return Err(protocol::OutcomeCode::TargetMissing); };
    if mask != &payload.expected { return Err(protocol::OutcomeCode::TargetMismatch); }
    if let Some(mask) = &payload.mask {
        if let Some(key) = &mask.image_key {
            if key.is_empty() || !base.assets.contains_key(key) { return Err(protocol::OutcomeCode::TargetMissing); }
        }
        if [mask.width, mask.height].iter().flatten().any(|&n| n == 0 || n > 16384) || mask.width.zip(mask.height).is_some_and(|(w, h)| u64::from(w) * u64::from(h) > 16_777_216) { return Err(protocol::OutcomeCode::Invariant); }
        let t = &mask.transform;
        semio_framework_pixels::compositing::inverse(t.as_affine()).map_err(|_| protocol::OutcomeCode::Invariant)?;
    }
    Ok(())
}

impl protocol::MutationKind<RasterSnapshot, RasterMutation> for ChangeLayerMask {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "layer-mask", kind: "change-layer-mask", record: "ChangedLayerMask" };

    fn diff(&self, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
        if let Err(code) = validate(self, base) { return protocol::MutationOutcome::refuse(code, "Mask cannot be applied to this layer revision.", [self.layer_id.clone()]); }
        protocol::MutationOutcome::new(diff_patch_layer(&self.layer_id, RasterLayerPatch { mask_content: Some(RasterMaskContent { mask: self.mask.clone() }), ..Default::default() }))
    }

    fn inverse(&self, base: &RasterSnapshot) -> Result<Vec<RasterMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        let Some(RasterLayerNode::Pixel { mask, .. } | RasterLayerNode::Group { mask, .. }) = find_layer(&base.layers, &self.layer_id) else { return Vec::new(); };
        vec![RasterMutation::ChangeLayerMask(Self { layer_id: self.layer_id.clone(), expected: self.mask.clone(), mask: mask.clone() })]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Change layer mask", "Ebenenmaske ändern") }
    fn target(&self) -> Vec<String> { vec![self.layer_id.clone()] }
}
