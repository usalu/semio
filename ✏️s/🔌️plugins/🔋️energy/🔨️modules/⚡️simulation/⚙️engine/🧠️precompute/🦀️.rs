//! 🧮️ Precompute geometry, CTF coefficients, solar factors, and zone topology.

use crate::envelope::ConductionState;
use crate::geometry::surface_tilt_azimuth;
use crate::material::{R_FILM_EXTERIOR_M2K_W, R_FILM_INTERIOR_M2K_W};
use crate::model::{EntityId, FixedTable, Model, SurfaceClass};
use crate::site::solar_position;
#[cfg(test)]
use crate::solar::beam_incidence_cosine;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use serde::{Deserialize, Serialize};

// #region 🔖️ZoneGeometry
/// 📐️ Precomputed zone geometry.
#[derive(Clone, Debug, Default, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct ZoneGeometry {
    pub floor_area_m2: f64,
    pub exterior_area_m2: f64,
    pub roof_area_m2: f64,
}

/// 📐️ Precomputed surface geometry and thermal properties.
#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
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
#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct FenestrationPrecompute {
    pub surface_id: EntityId,
    pub area_m2: f64,
    pub u_value_w_m2k: f64,
    pub shgc: f64,
    pub vlt: f64,
    pub tilt_deg: f64,
    pub azimuth_deg: f64,
    pub normal: [f64; 3],
    pub height_m: f64,
    pub width_m: f64,
    pub overhang_depth_m: f64,
    pub overhang_offset_m: f64,
    pub fin_depth_m: f64,
    pub fin_offset_m: f64,
}
// #endregion 🔖️FenestrationPrecompute

// #region 🔖️ThermostatLookup
/// 🌡️ Resolved thermostat setpoints for a zone.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct ResolvedSetpoints {
    pub heating_c: f64,
    pub cooling_c: f64,
    pub heating_throttle_k: f64,
    pub cooling_throttle_k: f64,
}
// #endregion 🔖️ThermostatLookup

// #region 🔖️PrecomputedModel
/// 🧮️ All precomputed data for a simulation run.
#[derive(Clone, Debug, Default, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct PrecomputedModel {
    pub(crate) zone_geometry: FixedTable<EntityId, ZoneGeometry>,
    pub(crate) surfaces: FixedTable<EntityId, SurfacePrecompute>,
    pub(crate) fenestrations: FixedTable<EntityId, FenestrationPrecompute>,
    pub(crate) default_setpoints: FixedTable<EntityId, ResolvedSetpoints>,
    pub(crate) zone_order: Vec<EntityId>,
    pub(crate) surface_order: Vec<EntityId>,
    pub(crate) fenestration_order: Vec<EntityId>,
    pub(crate) zone_indices: FixedTable<EntityId, usize>,
    pub(crate) surface_indices: FixedTable<EntityId, usize>,
    pub(crate) material_indices: FixedTable<EntityId, usize>,
    pub(crate) construction_indices: FixedTable<EntityId, usize>,
    pub(crate) fault_severity: FixedTable<EntityId, f64>,
    pub(crate) zone_timestep_s: f64,
    pub(crate) system_timestep_s: f64,
}

impl PrecomputedModel {
    /// 🧮️ Build precomputed data from model and timestep settings.
    #[cfg(test)]
    pub(crate) fn build(model: &Model, zone_timestep_minutes: u32, system_timestep_minutes: u32) -> Self {
        let mut builder = PrecomputeBuilder::new(zone_timestep_minutes, system_timestep_minutes);
        while !builder.is_complete() {
            builder.step(model);
        }
        builder.finish()
    }

    /// ☀️ Solar incidence cosine for a surface at given solar position.
    #[cfg(test)]
    pub(crate) fn surface_incidence(&self, surface_id: EntityId, sun_alt_deg: f64, sun_az_deg: f64) -> f64 {
        self.surfaces.get(&surface_id).map_or(0.0, |s| beam_incidence_cosine(s.normal, sun_alt_deg, sun_az_deg))
    }

    /// ☀️ Solar position for site at day-of-year and LOCAL STANDARD hour-of-day.
    pub(crate) fn solar_at(&self, model: &Model, day_of_year: u16, hour_local: f64) -> (f64, f64) {
        let pos = solar_position(model.site.latitude_deg, model.site.longitude_deg, model.site.time_zone_hours, day_of_year, hour_local);
        (pos.altitude_deg, pos.azimuth_deg)
    }

    pub(crate) fn close_step(&mut self, maximum_items: usize) -> bool {
        if maximum_items == 0 {
            return false;
        }
        macro_rules! remove_entry {
            ($owners:expr) => {{
                if $owners.pop().is_some() {
                    return false;
                }
            }};
        }
        remove_entry!(self.zone_geometry);
        remove_entry!(self.surfaces);
        remove_entry!(self.fenestrations);
        remove_entry!(self.default_setpoints);
        remove_entry!(self.zone_indices);
        remove_entry!(self.surface_indices);
        remove_entry!(self.material_indices);
        remove_entry!(self.construction_indices);
        remove_entry!(self.fault_severity);
        self.zone_order.pop().is_none() && self.surface_order.pop().is_none() && self.fenestration_order.pop().is_none()
    }
}
// #endregion 🔖️PrecomputedModel

// #region 🔖️PrecomputeBuilder
/// 🧮️ Persistent one-record-at-a-time precomputation used by interactive energy jobs and
/// by [`PrecomputedModel::build`]'s batch adapter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub(crate) enum PrecomputeStage {
    ReserveBacking,
    IndexMaterials,
    IndexConstructions,
    IndexSurfaces,
    Zones,
    Surfaces,
    NormalizeZones,
    Thermostats,
    Fenestrations,
    Faults,
    Complete,
}

#[cfg(test)]
pub(crate) const P7C1_PRECOMPUTE_STAGES: [PrecomputeStage; 11] = [
    PrecomputeStage::ReserveBacking,
    PrecomputeStage::IndexMaterials,
    PrecomputeStage::IndexConstructions,
    PrecomputeStage::IndexSurfaces,
    PrecomputeStage::Zones,
    PrecomputeStage::Surfaces,
    PrecomputeStage::NormalizeZones,
    PrecomputeStage::Thermostats,
    PrecomputeStage::Fenestrations,
    PrecomputeStage::Faults,
    PrecomputeStage::Complete,
];

#[cfg(test)]
pub(crate) const P7C1_SURFACE_PRECOMPUTE_STAGES: [SurfacePrecomputeStage; 4] = [SurfacePrecomputeStage::Area, SurfacePrecomputeStage::Normal, SurfacePrecomputeStage::Materials, SurfacePrecomputeStage::Publish];

/// 🧮️ Cursor state for deterministic, resumable model precomputation.
#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub(crate) struct PrecomputeBuilder {
    output: PrecomputedModel,
    stage: PrecomputeStage,
    cursor: usize,
    reserve_cursor: u8,
    backing_rejected: bool,
    surface_work: Option<SurfacePrecomputeWork>,
}

/// 🧱️ One-vertex or one-material cursor for a surface precompute record.
#[derive(Clone, Debug, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
struct SurfacePrecomputeWork {
    surface_index: usize,
    stage: SurfacePrecomputeStage,
    cursor: usize,
    area_m2: f64,
    normal_sum: [f64; 3],
    resistance_m2k_w: f64,
    capacitance_j_m2k: f64,
    solar_absorptance: f64,
    emissivity: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub(crate) enum SurfacePrecomputeStage {
    Area,
    Normal,
    Materials,
    Publish,
}

impl PrecomputeBuilder {
    pub(crate) fn retained_wire_signature(&self) -> [u64; 8] {
        [
            self.stage as u64,
            self.cursor as u64,
            self.reserve_cursor as u64,
            self.backing_rejected as u64,
            self.surface_work.as_ref().map_or(0, |work| work.surface_index as u64 + 1),
            self.surface_work.as_ref().map_or(0, |work| work.stage as u64),
            self.surface_work.as_ref().map_or(0, |work| work.cursor as u64),
            self.output.zone_order.len().wrapping_add(self.output.surface_order.len()).wrapping_add(self.output.fenestration_order.len()) as u64,
        ]
    }

    pub(crate) fn new(zone_timestep_minutes: u32, system_timestep_minutes: u32) -> Self {
        Self {
            output: PrecomputedModel { zone_timestep_s: zone_timestep_minutes as f64 * 60.0, system_timestep_s: system_timestep_minutes as f64 * 60.0, ..PrecomputedModel::default() },
            stage: PrecomputeStage::ReserveBacking,
            cursor: 0,
            reserve_cursor: 0,
            backing_rejected: false,
            surface_work: None,
        }
    }

    #[cfg(test)]
    pub(crate) fn stage(&self) -> PrecomputeStage {
        self.stage
    }

    #[cfg(test)]
    pub(crate) fn set_stage_for_gate(&mut self, stage: PrecomputeStage) {
        self.stage = stage;
    }

    #[cfg(test)]
    pub(crate) fn surface_stage_for_gate(&self) -> Option<SurfacePrecomputeStage> {
        self.surface_work.as_ref().map(|work| work.stage)
    }

    #[cfg(test)]
    pub(crate) fn set_surface_stage_for_gate(&mut self, stage: SurfacePrecomputeStage) -> bool {
        let Some(work) = self.surface_work.as_mut() else { return false };
        work.stage = stage;
        true
    }

    pub(crate) fn is_complete(&self) -> bool {
        self.stage == PrecomputeStage::Complete
    }

    pub(crate) fn backing_rejected(&self) -> bool {
        self.backing_rejected
            || self.output.zone_geometry.faulted()
            || self.output.surfaces.faulted()
            || self.output.fenestrations.faulted()
            || self.output.default_setpoints.faulted()
            || self.output.zone_indices.faulted()
            || self.output.surface_indices.faulted()
            || self.output.material_indices.faulted()
            || self.output.construction_indices.faulted()
            || self.output.fault_severity.faulted()
    }

    pub(crate) fn step(&mut self, model: &Model) {
        match self.stage {
            PrecomputeStage::ReserveBacking => {
                let rejected = match self.reserve_cursor {
                    0 => self.output.zone_geometry.admit(model.zones.len()).is_err(),
                    1 => self.output.surfaces.admit(model.surfaces.len()).is_err(),
                    2 => self.output.fenestrations.admit(model.fenestrations.len()).is_err(),
                    3 => self.output.default_setpoints.admit(model.zones.len()).is_err(),
                    4 => self.output.zone_indices.admit(model.zones.len()).is_err(),
                    5 => self.output.surface_indices.admit(model.surfaces.len()).is_err(),
                    6 => self.output.material_indices.admit(model.materials.len()).is_err(),
                    7 => self.output.construction_indices.admit(model.constructions.len()).is_err(),
                    8 => self.output.fault_severity.admit(model.faults.len()).is_err(),
                    9 => self.output.zone_order.try_reserve_exact(model.zones.len()).is_err(),
                    10 => self.output.surface_order.try_reserve_exact(model.surfaces.len()).is_err(),
                    11 => self.output.fenestration_order.try_reserve_exact(model.fenestrations.len()).is_err(),
                    _ => {
                        self.advance(PrecomputeStage::IndexMaterials);
                        return;
                    }
                };
                if rejected {
                    self.backing_rejected = true;
                    self.stage = PrecomputeStage::Complete;
                } else {
                    self.reserve_cursor += 1;
                }
            }
            PrecomputeStage::IndexMaterials => {
                if let Some(material) = model.materials.get(self.cursor) {
                    let _ = self.output.material_indices.insert(material.id, self.cursor);
                    self.cursor += 1;
                } else {
                    self.advance(PrecomputeStage::IndexConstructions);
                }
            }
            PrecomputeStage::IndexConstructions => {
                if let Some(construction) = model.constructions.get(self.cursor) {
                    let _ = self.output.construction_indices.insert(construction.id, self.cursor);
                    self.cursor += 1;
                } else {
                    self.advance(PrecomputeStage::IndexSurfaces);
                }
            }
            PrecomputeStage::IndexSurfaces => {
                if let Some(surface) = model.surfaces.get(self.cursor) {
                    let _ = self.output.surface_indices.insert(surface.id, self.cursor);
                    self.cursor += 1;
                } else {
                    self.advance(PrecomputeStage::Zones);
                }
            }
            PrecomputeStage::Zones => {
                if let Some(zone) = model.zones.get(self.cursor) {
                    let _ = self.output.zone_geometry.insert(zone.id, ZoneGeometry::default());
                    let _ = self.output.default_setpoints.insert(zone.id, ResolvedSetpoints { heating_c: 20.0, cooling_c: 26.0, heating_throttle_k: 2.0, cooling_throttle_k: 2.0 });
                    let _ = self.output.zone_indices.insert(zone.id, self.cursor);
                    self.output.zone_order.push(zone.id);
                    self.cursor += 1;
                } else {
                    self.advance(PrecomputeStage::Surfaces);
                }
            }
            PrecomputeStage::Surfaces => {
                if self.cursor >= model.surfaces.len() {
                    self.advance(PrecomputeStage::NormalizeZones);
                    return;
                }
                self.step_surface(model);
            }
            PrecomputeStage::NormalizeZones => {
                if let Some(zone) = model.zones.get(self.cursor) {
                    let Some(geometry) = self.output.zone_geometry.get_mut(&zone.id) else {
                        self.backing_rejected = true;
                        self.stage = PrecomputeStage::Complete;
                        return;
                    };
                    geometry.floor_area_m2 = geometry.floor_area_m2.max(1.0);
                    self.cursor += 1;
                } else {
                    self.advance(PrecomputeStage::Thermostats);
                }
            }
            PrecomputeStage::Thermostats => {
                if let Some(thermostat) = model.thermostats.get(self.cursor) {
                    let _ = self
                        .output
                        .default_setpoints
                        .insert(thermostat.zone_id, ResolvedSetpoints { heating_c: 20.0, cooling_c: 26.0, heating_throttle_k: thermostat.heating_throttle_range_k, cooling_throttle_k: thermostat.cooling_throttle_range_k });
                    self.cursor += 1;
                } else {
                    self.advance(PrecomputeStage::Fenestrations);
                }
            }
            PrecomputeStage::Fenestrations => {
                if let Some(fenestration) = model.fenestrations.get(self.cursor) {
                    if let Some(surface) = self.output.surface_indices.get(&fenestration.surface_id).and_then(|index| model.surfaces.get(*index)) {
                        let normal = self.output.surfaces.get(&surface.id).map_or([0.0, 0.0, 1.0], |value| value.normal);
                        let orient = surface_tilt_azimuth(normal, model.site.north_axis_deg);
                        let _ = self.output.fenestrations.insert(
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
                                height_m: fenestration.height_m.max(1e-6),
                                width_m: fenestration.area_m2 / fenestration.height_m.max(1e-6),
                                overhang_depth_m: fenestration.overhang_depth_m,
                                overhang_offset_m: fenestration.overhang_offset_m,
                                fin_depth_m: fenestration.fin_depth_m,
                                fin_offset_m: fenestration.fin_offset_m,
                            },
                        );
                        self.output.fenestration_order.push(fenestration.id);
                        // 🪟️ The host surface conducts through its NET opaque area: a window is not
                        // a hole in the wall's thermal area unless it is subtracted, and the window
                        // itself already contributes its own `u_value_w_m2k · area_m2` term.
                        if let Some(host) = self.output.surfaces.get_mut(&fenestration.surface_id) {
                            host.area_m2 = (host.area_m2 - fenestration.area_m2).max(0.0);
                        }
                    }
                    self.cursor += 1;
                } else {
                    self.advance(PrecomputeStage::Faults);
                }
            }
            PrecomputeStage::Faults => {
                if let Some(fault) = model.faults.get(self.cursor) {
                    let _ = self.output.fault_severity.insert(fault.target_equipment_id, fault.severity);
                    self.cursor += 1;
                } else {
                    self.advance(PrecomputeStage::Complete);
                }
            }
            PrecomputeStage::Complete => {}
        }
    }

    pub(crate) fn finish(self) -> PrecomputedModel {
        debug_assert!(self.is_complete());
        self.output
    }

    pub(crate) fn close_step(&mut self, maximum_items: usize) -> bool {
        if maximum_items == 0 {
            return false;
        }
        self.surface_work = None;
        self.output.close_step(maximum_items)
    }

    fn advance(&mut self, stage: PrecomputeStage) {
        self.stage = stage;
        self.cursor = 0;
        self.surface_work = None;
    }

    fn step_surface(&mut self, model: &Model) {
        let surface = &model.surfaces[self.cursor];
        let work = self.surface_work.get_or_insert(SurfacePrecomputeWork {
            surface_index: self.cursor,
            stage: SurfacePrecomputeStage::Area,
            cursor: 1,
            area_m2: 0.0,
            normal_sum: [0.0; 3],
            resistance_m2k_w: R_FILM_INTERIOR_M2K_W + R_FILM_EXTERIOR_M2K_W,
            capacitance_j_m2k: 0.0,
            solar_absorptance: 0.7,
            emissivity: 0.9,
        });
        match work.stage {
            SurfacePrecomputeStage::Area => {
                if work.cursor + 1 < surface.vertices_m.len() {
                    let origin = surface.vertices_m[0];
                    let a = subtract(surface.vertices_m[work.cursor], origin);
                    let b = subtract(surface.vertices_m[work.cursor + 1], origin);
                    work.area_m2 += length(cross(a, b)) * 0.5;
                    work.cursor += 1;
                } else {
                    work.stage = SurfacePrecomputeStage::Normal;
                    work.cursor = 0;
                }
            }
            SurfacePrecomputeStage::Normal => {
                if let Some(current) = surface.vertices_m.get(work.cursor).copied() {
                    let next = surface.vertices_m[(work.cursor + 1) % surface.vertices_m.len().max(1)];
                    work.normal_sum[0] += (current[1] - next[1]) * (current[2] + next[2]);
                    work.normal_sum[1] += (current[2] - next[2]) * (current[0] + next[0]);
                    work.normal_sum[2] += (current[0] - next[0]) * (current[1] + next[1]);
                    work.cursor += 1;
                } else {
                    work.stage = SurfacePrecomputeStage::Materials;
                    work.cursor = 0;
                }
            }
            SurfacePrecomputeStage::Materials => {
                let material_id = self.output.construction_indices.get(&surface.construction_id).and_then(|index| model.constructions.get(*index)).and_then(|construction| construction.layer_material_ids.get(work.cursor)).copied();
                if let Some(material) = material_id.and_then(|id| self.output.material_indices.get(&id)).and_then(|index| model.materials.get(*index)) {
                    work.resistance_m2k_w += material.thickness_m / material.conductivity_w_m_k;
                    work.capacitance_j_m2k += material.density_kg_m3 * material.specific_heat_j_kg_k * material.thickness_m;
                    // 🧱️ Layer order is OUTSIDE-first, the same convention EnergyPlus's
                    // `Construction` object uses, so the exterior optical properties come from
                    // layer 0 alone — every layer used to overwrite them, leaving the INNERMOST
                    // layer deciding how much sun an exterior wall absorbs.
                    if work.cursor == 0 {
                        work.solar_absorptance = material.solar_absorptance;
                        work.emissivity = material.thermal_absorptance;
                    }
                    work.cursor += 1;
                } else {
                    work.stage = SurfacePrecomputeStage::Publish;
                }
            }
            SurfacePrecomputeStage::Publish => {
                let normal = normalize(work.normal_sum);
                let orient = surface_tilt_azimuth(normal, model.site.north_axis_deg);
                let u_value = if work.resistance_m2k_w > 0.0 { 1.0 / work.resistance_m2k_w } else { f64::INFINITY };
                let capacitance = work.capacitance_j_m2k.max(1.0);
                let Some(geometry) = self.output.zone_geometry.get_mut(&surface.zone_id) else {
                    self.backing_rejected = true;
                    self.stage = PrecomputeStage::Complete;
                    return;
                };
                geometry.floor_area_m2 += work.area_m2;
                if matches!(surface.class, SurfaceClass::ExteriorWall | SurfaceClass::Roof) {
                    geometry.exterior_area_m2 += work.area_m2;
                }
                if matches!(surface.class, SurfaceClass::Roof | SurfaceClass::Ceiling) {
                    geometry.roof_area_m2 += work.area_m2;
                }
                let _ = self.output.surfaces.insert(
                    surface.id,
                    SurfacePrecompute {
                        area_m2: work.area_m2,
                        u_value_w_m2k: u_value,
                        capacitance_j_m2k: capacitance,
                        solar_absorptance: work.solar_absorptance,
                        emissivity: work.emissivity,
                        tilt_deg: orient.tilt_deg,
                        azimuth_deg: orient.azimuth_deg,
                        normal,
                        ctf: ConductionState::from_u_and_capacitance(u_value, capacitance, self.output.zone_timestep_s),
                        zone_id: surface.zone_id,
                        sun_exposed: surface.sun_exposed,
                    },
                );
                self.output.surface_order.push(surface.id);
                self.cursor += 1;
                self.surface_work = None;
            }
        }
    }
}
// #endregion 🔖️PrecomputeBuilder

fn subtract(left: [f64; 3], right: [f64; 3]) -> [f64; 3] {
    [left[0] - right[0], left[1] - right[1], left[2] - right[2]]
}

fn cross(left: [f64; 3], right: [f64; 3]) -> [f64; 3] {
    [left[1] * right[2] - left[2] * right[1], left[2] * right[0] - left[0] * right[2], left[0] * right[1] - left[1] * right[0]]
}

fn length(value: [f64; 3]) -> f64 {
    (value[0] * value[0] + value[1] * value[1] + value[2] * value[2]).sqrt()
}

fn normalize(value: [f64; 3]) -> [f64; 3] {
    let magnitude = length(value);
    if magnitude > 0.0 {
        [value[0] / magnitude, value[1] / magnitude, value[2] / magnitude]
    } else {
        [0.0, 0.0, 1.0]
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
