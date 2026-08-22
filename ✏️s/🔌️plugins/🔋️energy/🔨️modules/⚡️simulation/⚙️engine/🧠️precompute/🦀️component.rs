//! 🧮️ Precompute geometry, CTF coefficients, solar factors, and zone topology.

use crate::envelope::ConductionState;
use crate::geometry::{polygon_normal, surface_area_m2, surface_tilt_azimuth};
use crate::material::{construction_thermal_mass, construction_u_value, R_FILM_EXTERIOR_M2K_W, R_FILM_INTERIOR_M2K_W};
use crate::model::{EntityId, Model, SurfaceClass};
use crate::site::solar_position;
use crate::solar::beam_incidence_cosine;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// #region 🔖️ZoneGeometry
/// 📐️ Precomputed zone geometry.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ZoneGeometry {
    pub floor_area_m2: f64,
    pub exterior_area_m2: f64,
    pub roof_area_m2: f64,
}

/// 📐️ Precomputed surface geometry and thermal properties.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SurfacePrecompute {
    pub area_m2: f64,
    pub u_value_w_m2k: f64,
    pub capacitance_j_m2k: f64,
    pub solar_absorptance: f64,
    pub emissivity: f64,
    pub tilt_deg: f64,
    pub azimuth_deg: f64,
    pub normal: [f64; 3],
    pub ctf: ConductionState,
    pub zone_id: EntityId,
    pub sun_exposed: bool,
}
// #endregion 🔖️ZoneGeometry

// #region 🔖️FenestrationPrecompute
/// 🪟️ Precomputed fenestration properties.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FenestrationPrecompute {
    pub surface_id: EntityId,
    pub area_m2: f64,
    pub u_value_w_m2k: f64,
    pub shgc: f64,
    pub vlt: f64,
    pub tilt_deg: f64,
    pub azimuth_deg: f64,
    pub normal: [f64; 3],
}
// #endregion 🔖️FenestrationPrecompute

// #region 🔖️ThermostatLookup
/// 🌡️ Resolved thermostat setpoints for a zone.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct ResolvedSetpoints {
    pub heating_c: f64,
    pub cooling_c: f64,
    pub heating_throttle_k: f64,
    pub cooling_throttle_k: f64,
}
// #endregion 🔖️ThermostatLookup

// #region 🔖️PrecomputedModel
/// 🧮️ All precomputed data for a simulation run.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PrecomputedModel {
    pub zone_geometry: HashMap<EntityId, ZoneGeometry>,
    pub surfaces: HashMap<EntityId, SurfacePrecompute>,
    pub fenestrations: HashMap<EntityId, FenestrationPrecompute>,
    pub default_setpoints: HashMap<EntityId, ResolvedSetpoints>,
    pub zone_timestep_s: f64,
    pub system_timestep_s: f64,
}

impl PrecomputedModel {
    /// 🧮️ Build precomputed data from model and timestep settings.
    pub fn build(model: &Model, zone_timestep_minutes: u32, system_timestep_minutes: u32) -> Self {
        let mut builder = PrecomputeBuilder::new(zone_timestep_minutes, system_timestep_minutes);
        while !builder.is_complete() {
            builder.step(model);
        }
        builder.finish()
    }

    /// ☀️ Solar incidence cosine for a surface at given solar position.
    pub fn surface_incidence(&self, surface_id: EntityId, sun_alt_deg: f64, sun_az_deg: f64) -> f64 {
        self.surfaces.get(&surface_id).map_or(0.0, |s| beam_incidence_cosine(s.normal, sun_alt_deg, sun_az_deg))
    }

    /// ☀️ Solar position for site at day/hour.
    pub fn solar_at(&self, model: &Model, day_of_year: u16, hour: f64) -> (f64, f64) {
        let pos = solar_position(model.site.latitude_deg, model.site.longitude_deg, day_of_year, hour);
        (pos.altitude_deg, pos.azimuth_deg)
    }
}
// #endregion 🔖️PrecomputedModel

// #region 🔖️PrecomputeBuilder
/// 🧮️ Persistent one-record-at-a-time precomputation used by interactive energy jobs and
/// by [`PrecomputedModel::build`]'s batch adapter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrecomputeStage {
    Zones,
    Surfaces,
    NormalizeZones,
    Thermostats,
    Fenestrations,
    Complete,
}

/// 🧮️ Cursor state for deterministic, resumable model precomputation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrecomputeBuilder {
    output: PrecomputedModel,
    stage: PrecomputeStage,
    cursor: usize,
}

impl PrecomputeBuilder {
    pub fn new(zone_timestep_minutes: u32, system_timestep_minutes: u32) -> Self {
        Self { output: PrecomputedModel { zone_timestep_s: zone_timestep_minutes as f64 * 60.0, system_timestep_s: system_timestep_minutes as f64 * 60.0, ..PrecomputedModel::default() }, stage: PrecomputeStage::Zones, cursor: 0 }
    }

    pub fn stage(&self) -> PrecomputeStage {
        self.stage
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn is_complete(&self) -> bool {
        self.stage == PrecomputeStage::Complete
    }

    pub fn step(&mut self, model: &Model) {
        match self.stage {
            PrecomputeStage::Zones => {
                if let Some(zone) = model.zones.get(self.cursor) {
                    self.output.zone_geometry.insert(zone.id, ZoneGeometry::default());
                    self.output.default_setpoints.insert(zone.id, ResolvedSetpoints { heating_c: 20.0, cooling_c: 26.0, heating_throttle_k: 2.0, cooling_throttle_k: 2.0 });
                    self.cursor += 1;
                } else {
                    self.advance(PrecomputeStage::Surfaces);
                }
            }
            PrecomputeStage::Surfaces => {
                if let Some(surface) = model.surfaces.get(self.cursor) {
                    let area_m2 = surface_area_m2(&surface.vertices_m);
                    let geometry = self.output.zone_geometry.entry(surface.zone_id).or_default();
                    geometry.floor_area_m2 += area_m2;
                    if matches!(surface.class, SurfaceClass::ExteriorWall | SurfaceClass::Roof) {
                        geometry.exterior_area_m2 += area_m2;
                    }
                    if matches!(surface.class, SurfaceClass::Roof | SurfaceClass::Ceiling) {
                        geometry.roof_area_m2 += area_m2;
                    }
                    let normal = polygon_normal(&surface.vertices_m);
                    let orient = surface_tilt_azimuth(normal, model.site.north_axis_deg);
                    let (u_value, capacitance, solar_abs, emissivity) = model.construction_by_id(surface.construction_id).map_or((0.3, 50_000.0, 0.7, 0.9), |construction| {
                        let layers: Vec<_> = construction.layer_material_ids.iter().filter_map(|id| model.material_by_id(*id)).cloned().collect();
                        let u_value = construction_u_value(&layers, R_FILM_INTERIOR_M2K_W, R_FILM_EXTERIOR_M2K_W);
                        let capacitance = construction_thermal_mass(&layers);
                        let outer = layers.last();
                        (u_value, capacitance, outer.map_or(0.7, |material| material.solar_absorptance), outer.map_or(0.9, |material| material.thermal_absorptance))
                    });
                    let ctf = ConductionState::from_u_and_capacitance(u_value, capacitance, self.output.zone_timestep_s);
                    self.output.surfaces.insert(
                        surface.id,
                        SurfacePrecompute {
                            area_m2,
                            u_value_w_m2k: u_value,
                            capacitance_j_m2k: capacitance,
                            solar_absorptance: solar_abs,
                            emissivity,
                            tilt_deg: orient.tilt_deg,
                            azimuth_deg: orient.azimuth_deg,
                            normal,
                            ctf,
                            zone_id: surface.zone_id,
                            sun_exposed: surface.sun_exposed,
                        },
                    );
                    self.cursor += 1;
                } else {
                    self.advance(PrecomputeStage::NormalizeZones);
                }
            }
            PrecomputeStage::NormalizeZones => {
                if let Some(zone) = model.zones.get(self.cursor) {
                    let geometry = self.output.zone_geometry.entry(zone.id).or_default();
                    geometry.floor_area_m2 = geometry.floor_area_m2.max(1.0);
                    self.cursor += 1;
                } else {
                    self.advance(PrecomputeStage::Thermostats);
                }
            }
            PrecomputeStage::Thermostats => {
                if let Some(thermostat) = model.thermostats.get(self.cursor) {
                    self.output.default_setpoints.insert(thermostat.zone_id, ResolvedSetpoints { heating_c: 20.0, cooling_c: 26.0, heating_throttle_k: thermostat.heating_throttle_range_k, cooling_throttle_k: thermostat.cooling_throttle_range_k });
                    self.cursor += 1;
                } else {
                    self.advance(PrecomputeStage::Fenestrations);
                }
            }
            PrecomputeStage::Fenestrations => {
                if let Some(fenestration) = model.fenestrations.get(self.cursor) {
                    if let Some(surface) = model.surfaces.iter().find(|surface| surface.id == fenestration.surface_id) {
                        let normal = polygon_normal(&surface.vertices_m);
                        let orient = surface_tilt_azimuth(normal, model.site.north_axis_deg);
                        self.output.fenestrations.insert(
                            fenestration.id,
                            FenestrationPrecompute {
                                surface_id: fenestration.surface_id,
                                area_m2: fenestration.area_m2,
                                u_value_w_m2k: fenestration.u_value_w_m2k,
                                shgc: fenestration.shgc,
                                vlt: fenestration.vlt,
                                tilt_deg: orient.tilt_deg,
                                azimuth_deg: orient.azimuth_deg,
                                normal,
                            },
                        );
                    }
                    self.cursor += 1;
                } else {
                    self.advance(PrecomputeStage::Complete);
                }
            }
            PrecomputeStage::Complete => {}
        }
    }

    pub fn finish(self) -> PrecomputedModel {
        debug_assert!(self.is_complete());
        self.output
    }

    fn advance(&mut self, stage: PrecomputeStage) {
        self.stage = stage;
        self.cursor = 0;
    }
}
// #endregion 🔖️PrecomputeBuilder

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::*;

    #[test]
    fn precompute_builds_surface_ctf() {
        let model = crate::sim::test_model_single_zone();
        let pre = PrecomputedModel::build(&model, 60, 60);
        assert!(!pre.surfaces.is_empty());
        assert!(pre.zone_geometry.contains_key(&EntityId(1)));
    }

    #[test]
    fn surface_incidence_is_zero_for_unknown_surface() {
        let model = crate::sim::test_model_single_zone();
        let pre = PrecomputedModel::build(&model, 60, 60);
        assert_eq!(pre.surface_incidence(EntityId(999), 45.0, 180.0), 0.0);
    }

    #[test]
    fn surface_incidence_matches_known_surface_normal() {
        let model = crate::sim::test_model_single_zone();
        let pre = PrecomputedModel::build(&model, 60, 60);
        let incidence = pre.surface_incidence(EntityId(30), 45.0, 180.0);
        assert!((-1.0..=1.0).contains(&incidence));
    }

    #[test]
    fn solar_at_returns_altitude_and_azimuth() {
        let model = crate::sim::test_model_single_zone();
        let pre = PrecomputedModel::build(&model, 60, 60);
        let (alt, az) = pre.solar_at(&model, 172, 12.0);
        assert!(alt > -90.0 && alt < 90.0);
        assert!((0.0..360.0).contains(&az));
    }

    #[test]
    fn thermostat_overrides_default_setpoints() {
        let mut model = crate::sim::test_model_single_zone();
        model.thermostats.push(Thermostat { id: EntityId(50), zone_id: EntityId(1), heating_setpoint_schedule_id: ScheduleId(1), cooling_setpoint_schedule_id: ScheduleId(1), heating_throttle_range_k: 3.0, cooling_throttle_range_k: 4.0 });
        let pre = PrecomputedModel::build(&model, 60, 60);
        let sp = pre.default_setpoints.get(&EntityId(1)).unwrap();
        assert!((sp.heating_throttle_k - 3.0).abs() < 1e-9);
        assert!((sp.cooling_throttle_k - 4.0).abs() < 1e-9);
    }

    #[test]
    fn fenestration_precompute_derives_from_host_surface() {
        let mut model = crate::sim::test_model_single_zone();
        model.fenestrations.push(Fenestration { id: EntityId(40), name: "Win".into(), surface_id: EntityId(30), u_value_w_m2k: 2.0, shgc: 0.4, vlt: 0.6, area_m2: 2.0, frame_conductance_w_k: 0.0, divider_conductance_w_k: 0.0 });
        let pre = PrecomputedModel::build(&model, 60, 60);
        let fen = pre.fenestrations.get(&EntityId(40)).unwrap();
        assert_eq!(fen.surface_id, EntityId(30));
        assert!((fen.shgc - 0.4).abs() < 1e-9);
    }
}
