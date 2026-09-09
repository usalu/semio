//! 🔺️ Sparse diff builder for `ChangeFaultSeverity` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeFaultSeverity, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.faults.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fault {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !(0.0..=1.0).contains(&payload.new_severity) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Fault {}: severity must be a fraction in [0, 1], got {}.", payload.id.0, payload.new_severity), [payload.id.0.to_string()]);
    }
    if existing.severity == payload.new_severity {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Fault {} already carries this severity: {}.", payload.id.0, payload.new_severity));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.faults.iter_mut().find(|item| item.id == payload.id) {
        item.severity = payload.new_severity;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
