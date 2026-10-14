//! 🔺️ Sparse diff builder for `ChangeOutdoorAirSystemEconomizerEnabled` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, OutdoorAirSystemPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeOutdoorAirSystemEconomizerEnabled, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.outdoor_air_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Outdoor air system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };

    if existing.economizer_enabled == payload.new_economizer_enabled {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Outdoor air system {} already has that economizer setting.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { outdoor_air_systems: Rows::modifying(OutdoorAirSystemPatch { economizer_enabled: Some(payload.new_economizer_enabled), ..OutdoorAirSystemPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
