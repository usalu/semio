//! 📋️ Canonical simulation results and summary tables.

use crate::error::Diagnostics;
use crate::meters::MeterTable;
use crate::metrics::{EnvironmentalMetrics, ResilienceMetrics};
use crate::model::{EntityId, FixedTable, FixedTableError};
use crate::output::TimeSeriesTable;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use serde::{Deserialize, Serialize};

// #region 🔖️Summary
/// 📋️ Annual/monthly summary table row.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct SummaryRow {
    pub key: String,
    pub value: f64,
    pub unit: String,
}

/// 📋️ Summary tables (energy use, loads, comfort).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct SummaryTables {
    pub annual_energy: Vec<SummaryRow>,
    pub monthly_energy: Vec<(u8, Vec<SummaryRow>)>,
    pub peak_loads: Vec<SummaryRow>,
    pub comfort: Vec<SummaryRow>,
}

impl SummaryTables {
    pub fn add_annual(&mut self, key: impl Into<String>, value: f64, unit: impl Into<String>) {
        self.annual_energy.push(SummaryRow { key: key.into(), value, unit: unit.into() });
    }
}
// #endregion 🔖️Summary

// #region 🔖️Sizing
/// 📐️ Component sizing result.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct SizingResult {
    pub component: String,
    pub design_load_w: f64,
    pub design_flow_m3_s: f64,
    pub autosized: bool,
}

/// 📐️ Sizing tables from design-day calculations.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct SizingTables {
    pub zone_loads: Vec<SizingResult>,
    pub equipment: Vec<SizingResult>,
}
// #endregion 🔖️Sizing

// #region 🔖️PerSurface
/// 🧱️ One opaque surface's or one window's energy over the run period, in joules — the live
/// accumulator the kernel integrates flux × dt into, one row per envelope face.
///
/// `conduction_loss_j`/`conduction_gain_j` are the two signs of the SAME quantity, split so a colour
/// map never has to carry a signed field: the net conductive heat crossing the inside face of that
/// surface, taken at the zone-air ⇄ inside-face convective link (`area_m2 · h_inside ·
/// (T_air − T_inside_face)`). Positive means heat leaves the zone through the face (a loss); the
/// opposite sign accumulates into `conduction_gain_j`. That link — not the outside face — is what the
/// zone air heat balance actually sees, so summing `conduction_loss_j − conduction_gain_j` over every
/// face of a zone reproduces the envelope term of that zone's air balance exactly.
///
/// `solar_transmitted_j` is windows only: the beam + diffuse shortwave that passes THROUGH the
/// glazing into the zone. `solar_absorbed_j` is what the face itself keeps: outside-face absorbed
/// solar for a sun-exposed opaque surface, pane absorptance for a window, plus the inside-face share
/// of any solar another window transmitted onto it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct SurfaceEnergy {
    pub conduction_loss_j: f64,
    pub conduction_gain_j: f64,
    pub solar_transmitted_j: f64,
    pub solar_absorbed_j: f64,
}

impl SurfaceEnergy {
    /// 🔥️ Integrates one timestep of conductive flux. `flux_w` is positive when heat LEAVES the zone.
    pub fn accumulate_conduction(&mut self, flux_w: f64, dt_s: f64) {
        if !flux_w.is_finite() || !dt_s.is_finite() {
            return;
        }
        if flux_w >= 0.0 {
            self.conduction_loss_j += flux_w * dt_s;
        } else {
            self.conduction_gain_j += -flux_w * dt_s;
        }
    }

    /// ☀️ Integrates one timestep of transmitted shortwave (windows only).
    pub fn accumulate_solar_transmitted(&mut self, power_w: f64, dt_s: f64) {
        if power_w.is_finite() && dt_s.is_finite() && power_w > 0.0 {
            self.solar_transmitted_j += power_w * dt_s;
        }
    }

    /// ☀️ Integrates one timestep of shortwave absorbed by the face itself.
    pub fn accumulate_solar_absorbed(&mut self, power_w: f64, dt_s: f64) {
        if power_w.is_finite() && dt_s.is_finite() && power_w > 0.0 {
            self.solar_absorbed_j += power_w * dt_s;
        }
    }

    /// 📋️ The kWh projection published on [`Results::per_surface`].
    pub fn summary(&self, id: EntityId) -> SurfaceEnergySummary {
        const J_PER_KWH: f64 = 3_600_000.0;
        SurfaceEnergySummary {
            id,
            conduction_loss_kwh: self.conduction_loss_j / J_PER_KWH,
            conduction_gain_kwh: self.conduction_gain_j / J_PER_KWH,
            solar_transmitted_kwh: self.solar_transmitted_j / J_PER_KWH,
            solar_absorbed_kwh: self.solar_absorbed_j / J_PER_KWH,
        }
    }
}

/// 📋️ One published per-surface row, in kWh — what a 3d window colours a face by.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct SurfaceEnergySummary {
    pub id: EntityId,
    pub conduction_loss_kwh: f64,
    pub conduction_gain_kwh: f64,
    pub solar_transmitted_kwh: f64,
    pub solar_absorbed_kwh: f64,
}

/// 🧺️ `EntityId`-keyed per-surface energy table, admitted exactly once like
/// [`crate::meters::MeterTable`]'s own backing: one row per opaque surface and one per window, never
/// growing after admission.
///
/// Two `FixedTable`s rather than one, because `FixedTable::insert` only accepts keys in ascending
/// order and the model's opaque surfaces and its fenestrations are each ascending on their own but
/// interleave arbitrarily with one another. Every caller already knows which side of the envelope it
/// is on (`EnclosureFace::Opaque` vs `EnclosureFace::Window`), so the split costs nothing at the call
/// site and keeps admission a single pass over each model vector — exactly what
/// `SimulationModel::surfaces`/`::windows` already do.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct SurfaceEnergyTable {
    pub(crate) opaque: FixedTable<EntityId, SurfaceEnergy>,
    pub(crate) windows: FixedTable<EntityId, SurfaceEnergy>,
}

impl SurfaceEnergyTable {
    /// 🧺️ Reserves both backings once. Called from the numerical job's initialization stages.
    pub(crate) fn admit(&mut self, opaque: usize, windows: usize) -> Result<(), FixedTableError> {
        self.opaque.admit(opaque)?;
        self.windows.admit(windows)
    }

    pub(crate) fn insert_opaque(&mut self, id: EntityId) -> Result<(), FixedTableError> {
        self.opaque.insert(id, SurfaceEnergy::default()).map(|_| ())
    }

    pub(crate) fn insert_window(&mut self, id: EntityId) -> Result<(), FixedTableError> {
        self.windows.insert(id, SurfaceEnergy::default()).map(|_| ())
    }

    pub(crate) fn opaque_mut(&mut self, id: EntityId) -> Option<&mut SurfaceEnergy> {
        self.opaque.get_mut(&id)
    }

    pub(crate) fn window_mut(&mut self, id: EntityId) -> Option<&mut SurfaceEnergy> {
        self.windows.get_mut(&id)
    }

    /// 🧹️ One bounded retirement step, mirroring how the job drains `surfaces`/`windows`.
    pub(crate) fn pop(&mut self) -> bool {
        self.windows.pop().is_some() || self.opaque.pop().is_some()
    }

    pub fn len(&self) -> usize {
        self.opaque.len() + self.windows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 📋️ Every row as a kWh summary: opaque surfaces first, then windows.
    pub fn summaries(&self) -> impl Iterator<Item = SurfaceEnergySummary> + '_ {
        self.opaque.iter().chain(self.windows.iter()).map(|(id, energy)| energy.summary(*id))
    }

    /// 🔎️ One row by id, whichever side of the envelope it lives on.
    pub fn get(&self, id: EntityId) -> Option<&SurfaceEnergy> {
        self.opaque.get(&id).or_else(|| self.windows.get(&id))
    }
}
// #endregion 🔖️PerSurface

// #region 🔖️Results
/// 📋️ Complete simulation results (canonical structured format).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct Results {
    pub time_series: TimeSeriesTable,
    pub meters: MeterTable,
    pub summaries: SummaryTables,
    pub sizing: SizingTables,
    pub environmental: EnvironmentalMetrics,
    pub resilience: ResilienceMetrics,
    pub diagnostics: Diagnostics,
    pub run_metadata: RunMetadata,
    /// 🧱️ One row per opaque surface and per window, integrated over the run period only (warmup is
    /// excluded). Empty when the run never reached its run period.
    pub per_surface: Vec<SurfaceEnergySummary>,
}

/// 🏷️ Run metadata.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct RunMetadata {
    pub model_name: String,
    pub model_version: String,
    pub weather_location: String,
    pub timesteps: u32,
    pub warmup_days: u32,
    pub elapsed_ms: u64,
}
// #endregion 🔖️Results

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
