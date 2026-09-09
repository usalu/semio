//! 🔺️ Sparse diff builder for `ChangeOutdoorAirSystemEconomizerEnabled` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeOutdoorAirSystemEconomizerEnabled, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.outdoor_air_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Outdoor air system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };

    if existing.economizer_enabled == payload.new_economizer_enabled {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Outdoor air system {} already has that economizer setting.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.outdoor_air_systems.iter_mut().find(|item| item.id == payload.id) {
        item.economizer_enabled = payload.new_economizer_enabled;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
