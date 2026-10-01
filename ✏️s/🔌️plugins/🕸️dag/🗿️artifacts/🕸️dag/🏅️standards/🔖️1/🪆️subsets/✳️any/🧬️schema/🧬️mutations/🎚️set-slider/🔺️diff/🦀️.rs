//! 🔺️ Sparse diff builder for `SetSlider` — the addressed slider's field takes the payload number; a value outside the
//! slider's range is clamped into it (`mutation.clamped`).
use crate::diff::DagDiff;
use crate::mutations::DagSliderField;
use crate::schema::diff::diff_replace_content;
use crate::{dag_working_scene, DagNodeKind, DagSnapshot};

//#region 🔖️Diff
/// 🏗️ A non-finite number is `mutation.invariant`; a missing node is `mutation.target-missing`; a node that is no slider,
/// or a bound that would cross the other bound, is `mutation.target-mismatch`; the number the field already holds is
/// `mutation.no-op`.
pub fn diff(payload: &super::mutation::SetSlider, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
    if !payload.value.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("slider \"{}\" {} must be finite", payload.id, payload.field.as_str()), [payload.id.clone()]);
    }
    let scene = dag_working_scene(base);
    let Some(existing) = scene.nodes.iter().find(|node| node.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let DagNodeKind::Slider { value, min, max, .. } = existing.kind else {
        return protocol::MutationOutcome::error("mutation.target-mismatch", format!("Node \"{}\" is not a slider.", payload.id), [payload.id.clone()]);
    };
    let (next, clamped) = match payload.field {
        DagSliderField::Value => {
            let next = if min.is_finite() && max.is_finite() { payload.value.clamp(min.min(max), max.max(min)) } else { payload.value };
            (next, next != payload.value)
        }
        DagSliderField::Min if payload.value > max => return protocol::MutationOutcome::error("mutation.target-mismatch", format!("slider \"{}\" minimum {} exceeds its maximum {max}", payload.id, payload.value), [payload.id.clone()]),
        DagSliderField::Max if payload.value < min => return protocol::MutationOutcome::error("mutation.target-mismatch", format!("slider \"{}\" maximum {} is below its minimum {min}", payload.id, payload.value), [payload.id.clone()]),
        DagSliderField::Min | DagSliderField::Max => (payload.value, false),
    };
    let current = match payload.field {
        DagSliderField::Value => value,
        DagSliderField::Min => min,
        DagSliderField::Max => max,
    };
    let clamp_message = clamped.then(|| protocol::MutationMessage::warn("mutation.clamped", format!("slider \"{}\" value {} clamped into [{min}, {max}]", payload.id, payload.value)).at([payload.id.clone()]));
    if next == current {
        return protocol::MutationOutcome::empty().absorb_messages(clamp_message.into_iter().chain([protocol::MutationMessage::warn("mutation.no-op", format!("slider \"{}\" {} is already {next}", payload.id, payload.field.as_str())).at([payload.id.clone()])]));
    }
    let mut nodes = scene.nodes;
    if let Some(DagNodeKind::Slider { value, min, max, .. }) = nodes.iter_mut().find(|node| node.id == payload.id).map(|node| &mut node.kind) {
        match payload.field {
            DagSliderField::Value => *value = next,
            DagSliderField::Min => *min = next,
            DagSliderField::Max => *max = next,
        }
    }
    protocol::MutationOutcome::new(diff_replace_content(nodes, scene.edges)).absorb_messages(clamp_message)
}
//#endregion 🔖️Diff
