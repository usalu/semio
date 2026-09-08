//! 🔺️ Sparse diff builder for `ChangeWaterSystemFixtureCount` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeWaterSystemFixtureCount, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.water_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Water system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_fixture_count == 0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Water system {} needs at least one fixture.", payload.id.0), [payload.id.0.to_string()]);
    }
    if existing.fixture_count == payload.new_fixture_count {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Water system {} already carries this fixture_count: {}.", payload.id.0, payload.new_fixture_count));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.water_systems.iter_mut().find(|item| item.id == payload.id) {
        item.fixture_count = payload.new_fixture_count;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
