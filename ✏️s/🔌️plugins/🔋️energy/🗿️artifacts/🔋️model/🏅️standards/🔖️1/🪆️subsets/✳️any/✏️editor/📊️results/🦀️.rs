//! 📊️ Editor-side simulation results: decoding the energy simulation run's per-surface tick payload
//! and turning it into the colour map and legend a 3d model window paints its faces with.
//!
//! This module is the READ side of the channel `🧵️simulation-session/🦀️.rs` writes: the run job
//! encodes an `(id, 4 × f32)` row per envelope face into `ToolRunTick::payload` on every tier boundary
//! and at completion, and the framework surfaces exactly those bytes back as
//! `ToolRunView::payload`. Nothing else survives — the driver closes the settled `Complete`
//! candidate's payload pages unread (`📓️explore-results-display-and-tool-run-state.md` §2.2), so the
//! numerical `Results` never reaches a window.
//!
//! Every entry point is total: a missing run, a foreign run, a `None` payload, truncated bytes or a
//! wrong magic all collapse to `None`/an empty map, and an empty map colours nothing rather than
//! panicking. The ramp is fem's own 8-stop `VON_MISES_BANDS` walk (`✏️s/🔌️plugins/🏗️fem/⚙️engine/
//! 🖥️app-surface/🦀️.rs`), COPIED rather than imported: fem and energy have no cross-plugin
//! dependency and the framework exposes no Rust colour helper to a wasm plugin.

use crate::editor::model::config::EnergyModelConfig;
use crate::editor::model::modes::edit::tools;
use crate::energy_simulation_session::{ENERGY_SURFACE_PAYLOAD_HEADER_BYTES, ENERGY_SURFACE_PAYLOAD_MAGIC, ENERGY_SURFACE_PAYLOAD_ROW_BYTES};
use semio_framework_plugin::ToolRunView;
use std::collections::HashMap;

//#region 🎨️Ramp
/// 🎨️ The 8 stops a surface is coloured by, blue (least) → red (most). Same eight hexes as fem's
/// `VON_MISES_BANDS`; energy reads them low-to-high for losses, so a cold blue wall loses little and
/// a red one loses a lot.
pub const SURFACE_ENERGY_BANDS: [&str; 8] = ["#1d4ed8", "#2563eb", "#0ea5e9", "#22c55e", "#eab308", "#f97316", "#ef4444", "#b91c1c"];

/// 🎨️ Neutral grey for a face with no result at all — fem3d's own `nodal_stress: None` fallback.
pub const SURFACE_ENERGY_NEUTRAL: [f64; 3] = [0.78, 0.78, 0.8];

/// 🎨️ `#rrggbb` → three 0..1 channels.
pub fn hex_to_rgb01(hex: &str) -> [f64; 3] {
    let h = hex.trim_start_matches('#');
    if h.len() < 6 {
        return SURFACE_ENERGY_NEUTRAL;
    }
    let component = |slice: &str| u8::from_str_radix(slice, 16).unwrap_or(0) as f64 / 255.0;
    [component(&h[0..2]), component(&h[2..4]), component(&h[4..6])]
}

/// 🌡️ Maps `value` within `[min, max]` onto one of the 8 bands, low to high. Identical arithmetic to
/// fem's `von_mises_color`: clamp the normalized position, round to the nearest band.
pub fn band_color(value: f64, min: f64, max: f64) -> &'static str {
    let span = (max - min).max(1e-9);
    let t = ((value - min) / span).clamp(0.0, 1.0);
    let index = ((t * (SURFACE_ENERGY_BANDS.len() - 1) as f64).round() as usize).min(SURFACE_ENERGY_BANDS.len() - 1);
    SURFACE_ENERGY_BANDS[index]
}
//#endregion 🎨️Ramp

//#region 📊️Map
/// 📊️ One surface's published energy, in kWh.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SurfaceEnergy {
    pub conduction_loss_kwh: f64,
    pub conduction_gain_kwh: f64,
    pub solar_transmitted_kwh: f64,
    pub solar_absorbed_kwh: f64,
}

impl SurfaceEnergy {
    /// 📊️ The one scalar a given field colours by.
    pub fn field(&self, field: ResultField) -> f64 {
        match field {
            ResultField::ConductionLoss => self.conduction_loss_kwh,
            ResultField::ConductionGain => self.conduction_gain_kwh,
            ResultField::SolarTransmitted => self.solar_transmitted_kwh,
            ResultField::SolarAbsorbed => self.solar_absorbed_kwh,
        }
    }
}

/// 📊️ The whole decoded payload: entity id → that surface's energy.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SurfaceEnergyMap(pub HashMap<u32, SurfaceEnergy>);

impl SurfaceEnergyMap {
    pub fn get(&self, id: u32) -> Option<&SurfaceEnergy> {
        self.0.get(&id)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
//#endregion 📊️Map

//#region 🔖️Field
/// 🔖️ Which published quantity the 3d model window colours by. The wire ids are the values the
/// `set-result-field` action carries and the `resultField` config string stores.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ResultField {
    #[default]
    ConductionLoss,
    ConductionGain,
    SolarTransmitted,
    SolarAbsorbed,
}

impl ResultField {
    pub const ALL: [Self; 4] = [Self::ConductionLoss, Self::ConductionGain, Self::SolarTransmitted, Self::SolarAbsorbed];

    pub fn id(self) -> &'static str {
        match self {
            Self::ConductionLoss => "conductionLoss",
            Self::ConductionGain => "conductionGain",
            Self::SolarTransmitted => "solarTransmitted",
            Self::SolarAbsorbed => "solarAbsorbed",
        }
    }

    /// 🔖️ The field a wire id names; unknown ids fall back to the default rather than refusing.
    pub fn from_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|field| field.id() == id)
    }

    /// 🏷️ English caption stem, used by [`legend_caption`].
    pub fn label(self) -> &'static str {
        match self {
            Self::ConductionLoss => "Conduction loss",
            Self::ConductionGain => "Conduction gain",
            Self::SolarTransmitted => "Solar transmitted",
            Self::SolarAbsorbed => "Solar absorbed",
        }
    }
}

/// 🎚️ The field the editor config currently selects. An unrecognized stored string (an older
/// document, a hand-edited config) reads as the default rather than blanking the window.
pub fn result_field(cfg: &EnergyModelConfig) -> ResultField {
    ResultField::from_id(&cfg.result_field).unwrap_or_default()
}
//#endregion 🔖️Field

//#region 🔓️Decode
/// 🔓️ Decodes the run's tick payload. `None` for anything that is not a well-formed `ESF1` blob:
/// wrong magic, a header that does not fit, or a body whose length does not match its row count.
pub fn decode_run_payload(payload: &[u8]) -> Option<SurfaceEnergyMap> {
    if payload.len() < ENERGY_SURFACE_PAYLOAD_HEADER_BYTES || payload[0..4] != ENERGY_SURFACE_PAYLOAD_MAGIC {
        return None;
    }
    let count = u32::from_le_bytes(payload[4..8].try_into().ok()?) as usize;
    let body = &payload[ENERGY_SURFACE_PAYLOAD_HEADER_BYTES..];
    if body.len() < count.checked_mul(ENERGY_SURFACE_PAYLOAD_ROW_BYTES)? {
        return None;
    }
    let mut rows = HashMap::new();
    rows.try_reserve(count).ok()?;
    for index in 0..count {
        let row = &body[index * ENERGY_SURFACE_PAYLOAD_ROW_BYTES..(index + 1) * ENERGY_SURFACE_PAYLOAD_ROW_BYTES];
        let id = u32::from_le_bytes(row[0..4].try_into().ok()?);
        let scalar = |offset: usize| f64::from(f32::from_le_bytes(row[offset..offset + 4].try_into().unwrap_or([0; 4])));
        rows.insert(id, SurfaceEnergy { conduction_loss_kwh: scalar(4), conduction_gain_kwh: scalar(8), solar_transmitted_kwh: scalar(12), solar_absorbed_kwh: scalar(16) });
    }
    Some(SurfaceEnergyMap(rows))
}

/// 🔓️ The live energy simulation run's per-surface map, if there is one. Filters on
/// `tool_id == energySimulation` so another plugin's run in the same document never colours this one.
pub fn surface_energy_from_run(run: Option<&ToolRunView>) -> Option<SurfaceEnergyMap> {
    let run = run.filter(|run| run.tool_id == tools::simulation::TOOL_ID)?;
    decode_run_payload(run.payload.as_deref()?)
}
//#endregion 🔓️Decode

//#region 🎨️Colors
/// 🎨️ Per-surface RGB for one field, plus the `(min, max)` the ramp was normalized over — the same
/// global-extrema pattern fem3d uses across all solids. An empty map yields an empty colour map and
/// `(0.0, 0.0)`; a map where every value is identical still colours (the ramp's 1e-9 span floor puts
/// every face on the lowest band rather than dividing by zero).
pub fn surface_colors(map: &SurfaceEnergyMap, field: ResultField) -> (HashMap<u32, [f64; 3]>, f64, f64) {
    let mut minimum = f64::INFINITY;
    let mut maximum = f64::NEG_INFINITY;
    for energy in map.0.values() {
        let value = energy.field(field);
        if !value.is_finite() {
            continue;
        }
        minimum = minimum.min(value);
        maximum = maximum.max(value);
    }
    if !minimum.is_finite() || !maximum.is_finite() {
        return (HashMap::new(), 0.0, 0.0);
    }
    let colors = map.0.iter().map(|(id, energy)| (*id, hex_to_rgb01(band_color(energy.field(field), minimum, maximum)))).collect();
    (colors, minimum, maximum)
}

/// 🏷️ The legend a 3d window puts under its ramp, e.g. `"Conduction loss · 0.0 – 412.3 kWh"`.
pub fn legend_caption(field: ResultField, min: f64, max: f64) -> String {
    format!("{} · {min:.1} – {max:.1} kWh", field.label())
}
//#endregion 🎨️Colors

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
