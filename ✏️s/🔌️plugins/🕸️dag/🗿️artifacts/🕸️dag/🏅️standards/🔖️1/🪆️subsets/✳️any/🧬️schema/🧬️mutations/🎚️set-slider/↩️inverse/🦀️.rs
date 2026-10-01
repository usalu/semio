//! ↩️ Inverse for `SetSlider` — ONE `set-slider` row with the field's OLD number read from BASE; a missing node or a node
//! that is no slider has no inverse.
use crate::mutations::{set_slider, DagMutation, DagSliderField};
use crate::{dag_working_scene, DagNodeKind, DagSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::SetSlider, base: &DagSnapshot) -> Vec<DagMutation> {
    match dag_working_scene(base).nodes.into_iter().find(|node| node.id == payload.id).map(|node| node.kind) {
        Some(DagNodeKind::Slider { value, min, max, .. }) => {
            let old = match payload.field {
                DagSliderField::Value => value,
                DagSliderField::Min => min,
                DagSliderField::Max => max,
            };
            vec![set_slider(payload.id.clone(), payload.field, old)]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
