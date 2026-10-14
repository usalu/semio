//! 🔺️ Sparse diff builder for `ReorderAnnualScheduleRules` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, AnnualSchedulePatch, ListEdit, ModelPatch, Rows, ScheduleSetPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReorderAnnualScheduleRules, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.schedules.annual.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Annual schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.from as usize >= existing.rules.len() || payload.to as usize >= existing.rules.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Annual schedule {} has {} rules, so {} .. {} is not a move.", payload.id.0, existing.rules.len(), payload.from, payload.to), [payload.id.0.to_string()]);
    }
    if payload.from == payload.to {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Annual schedule {} rule {} is already at that position.", payload.id.0, payload.from));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { schedules: ScheduleSetPatch { annual: Rows::modifying(AnnualSchedulePatch { rules: ListEdit::moving(&existing.rules, payload.from as usize, payload.to as usize), ..AnnualSchedulePatch::of(payload.id) }), ..Default::default() }, ..Default::default() }))
}
//#endregion 🔖️Diff
