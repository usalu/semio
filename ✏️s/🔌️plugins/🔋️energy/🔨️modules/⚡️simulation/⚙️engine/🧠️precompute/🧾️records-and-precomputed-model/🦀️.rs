//! 🧾️ The per-zone, surface, window and enclosure precompute records and the `PrecomputedModel` aggregate.

use super::*;

// #region 🔖️Records
/// 📐️ Precomputed zone geometry.
#[derive(Clone, Debug, Default, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub struct ZoneGeometry {
    pub floor_area_m2: f64,
    pub exterior_area_m2: f64,
    pub roof_area_m2: f64,
}

/// 🧱️ One opaque heat-transfer surface in world coordinates.
#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub struct SurfacePrecompute {
    pub zone_id: EntityId,
    pub class: SurfaceClass,
    pub boundary: OutsideBoundary,
    pub polygon: Vec<[f64; 3]>,
    pub normal: [f64; 3],
    pub gross_area_m2: f64,
    pub area_m2: f64,
    pub centroid_height_m: f64,
    pub tilt_deg: f64,
    pub azimuth_deg: f64,
    pub sun_exposed: bool,
    pub wind_exposed: bool,
    pub chain: NodeChain,
    pub outside_solar_absorptance: f64,
    pub outside_emissivity: f64,
    pub inside_solar_absorptance: f64,
    pub inside_emissivity: f64,
    pub roughness_multiplier: f64,
    pub sky_isotropic_ratio: f64,
    pub sky_horizon_ratio: f64,
    pub casters: Vec<usize>,
    pub windows: Vec<EntityId>,
}

/// 🪟️ One window placed on its host surface.
#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub struct WindowPrecompute {
    pub zone_id: EntityId,
    pub surface_id: EntityId,
    pub polygon: Vec<[f64; 3]>,
    pub normal: [f64; 3],
    pub area_m2: f64,
    pub height_m: f64,
    pub centroid_height_m: f64,
    pub tilt_deg: f64,
    pub azimuth_deg: f64,
    pub sun_exposed: bool,
    pub wind_exposed: bool,
    pub glazing: GlazingSystem,
    pub coefficient_adjustment: f64,
    pub sky_isotropic_ratio: f64,
    pub sky_horizon_ratio: f64,
    pub casters: Vec<usize>,
}

/// 🔲️ What one enclosure face is.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub enum EnclosureFace {
    Opaque(EntityId),
    Window(EntityId),
}

/// 🔲️ Most faces an enclosure exchanges long-wave radiation over exact gray-body factors; larger
/// enclosures exchange through a mean radiant node so every heat-balance step stays bounded.
pub const EXACT_ENCLOSURE_FACES: usize = 48;

/// 🔲️ Long-wave exchange model of one enclosure.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub enum EnclosureRadiation {
    /// Row-major approximate view factors `F[i][j]` from `i` to `j` and gray-body exchange factors
    /// `𝓕[i][j]`: the net flux into face `i` is `Σ_j σ 𝓕[i][j] (T_j⁴ − T_i⁴)`.
    Exchange { view_factors: Vec<f64>, exchange_factors: Vec<f64> },
    /// Carroll mean-radiant-temperature network: face `i` exchanges `participation[i]·(T_mrt⁴ − T_i⁴)`
    /// with a massless node at `T_mrt⁴ = Σ A·p·T⁴ / Σ A·p`; `participation` already includes σ.
    MeanRadiant { participation: Vec<f64> },
}

impl Default for EnclosureRadiation {
    fn default() -> Self {
        Self::MeanRadiant { participation: Vec::new() }
    }
}

/// 🔲️ One zone's radiant enclosure: faces, their areas and room-side emissivities, the long-wave
/// exchange model, and the diffuse-solar multiplier `1 / Σ A·α` that closes the shortwave balance.
#[derive(Clone, Debug, Default, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub struct EnclosurePrecompute {
    pub faces: Vec<EnclosureFace>,
    pub areas_m2: Vec<f64>,
    pub emissivities: Vec<f64>,
    pub radiation: EnclosureRadiation,
    pub diffuse_solar_multiplier: f64,
    pub radiant_weight_total_m2: f64,
}

/// 🌡️ Resolved thermostat setpoints for a zone.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub struct ResolvedSetpoints {
    pub heating_c: f64,
    pub cooling_c: f64,
    pub heating_throttle_k: f64,
    pub cooling_throttle_k: f64,
}
// #endregion 🔖️Records

// #region 🔖️PrecomputedModel
/// 🧮️ All precomputed data for a simulation run.
#[derive(Clone, Debug, Default, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned)]
pub struct PrecomputedModel {
    pub zone_geometry: FixedTable<EntityId, ZoneGeometry>,
    pub surfaces: FixedTable<EntityId, SurfacePrecompute>,
    pub windows: FixedTable<EntityId, WindowPrecompute>,
    pub enclosures: FixedTable<EntityId, EnclosurePrecompute>,
    pub default_setpoints: FixedTable<EntityId, ResolvedSetpoints>,
    pub zone_order: Vec<EntityId>,
    pub(crate) surface_order: Vec<EntityId>,
    pub(crate) window_order: Vec<EntityId>,
    pub casters: Vec<Vec<[f64; 3]>>,
    pub zone_indices: FixedTable<EntityId, usize>,
    pub surface_indices: FixedTable<EntityId, usize>,
    pub(crate) material_indices: FixedTable<EntityId, usize>,
    pub(crate) glazing_indices: FixedTable<EntityId, usize>,
    pub(crate) gas_indices: FixedTable<EntityId, usize>,
    pub(crate) construction_indices: FixedTable<EntityId, usize>,
    pub fault_severity: FixedTable<EntityId, f64>,
    pub zone_timestep_s: f64,
    pub(crate) system_timestep_s: f64,
    pub maximum_enclosure_faces: usize,
    pub maximum_nodes: usize,
}

impl PrecomputedModel {
    /// 🧮️ Builds precomputed data from model and timestep settings in one blocking call — the incremental
    /// [`PrecomputeBuilder`] driven to completion, for laws and offline tools that need the whole result at once.
    pub fn build(model: &Model, zone_timestep_minutes: u32, system_timestep_minutes: u32) -> Self {
        let mut builder = PrecomputeBuilder::new(zone_timestep_minutes, system_timestep_minutes);
        while !builder.is_complete() {
            builder.step(model);
        }
        builder.finish()
    }

    /// ⏱️ Length of one coupled heat-balance step [s].
    pub fn balance_step_s(&self) -> f64 {
        self.system_timestep_s.min(self.zone_timestep_s).max(1.0)
    }

}
// #endregion 🔖️PrecomputedModel
