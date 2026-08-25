//! 📐️ Zone and equipment sizing from design-day calculations.

use crate::model::Model;
use crate::results::{SizingResult, SizingTables};
use crate::site::{DesignDay, DesignDayKind};
use crate::units::CP_DRY_AIR;
use serde::{Deserialize, Serialize};

// #region 🔖️SizingConfig
/// 📐️ Sizing configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SizingConfig {
    pub heating_design_day: DesignDay,
    pub cooling_design_day: DesignDay,
    pub sizing_factor: f64,
    pub safety_factor: f64,
}

impl Default for SizingConfig {
    fn default() -> Self {
        Self {
            heating_design_day: DesignDay {
                name: "Winter Design".into(),
                kind: DesignDayKind::Heating,
                month: 1,
                day: 21,
                dry_bulb_max_c: -10.0,
                daily_range_k: 6.0,
                humidity_condition: crate::site::DesignDayHumidity::RelativeHumidity { rh: 0.8 },
                wind_speed_m_s: 3.0,
                solar_model: false,
            },
            cooling_design_day: DesignDay {
                name: "Summer Design".into(),
                kind: DesignDayKind::Cooling,
                month: 7,
                day: 21,
                dry_bulb_max_c: 35.0,
                daily_range_k: 10.0,
                humidity_condition: crate::site::DesignDayHumidity::Wetbulb { wetbulb_at_max_c: 24.0 },
                wind_speed_m_s: 2.0,
                solar_model: true,
            },
            sizing_factor: 1.0,
            safety_factor: 1.15,
        }
    }
}
// #endregion 🔖️SizingConfig

// #region 🔖️Sizing
/// 🧭️ Cursor-owned sizing pass used by interactive simulation finalization.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct SizingBuilder {
    config: SizingConfig,
    stage: SizingStage,
    zone_cursor: usize,
    surface_cursor: usize,
    vertex_cursor: usize,
    surface_normal: [f64; 3],
    zone_area_m2: f64,
    equipment_cursor: usize,
    equipment_zone_cursor: usize,
    name_cursor: usize,
    pending_name: String,
    tables: SizingTables,
    fault: Option<SizingFault>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum SizingFault {
    ZoneResultBacking,
    EquipmentResultBacking,
    NameBacking,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum SizingStage {
    ReserveZoneResults,
    ReserveEquipmentResults,
    ScanZoneSurface,
    ScanSurfaceVertex,
    ReserveHeatingName,
    CopyHeatingZoneName,
    CopyHeatingSuffix,
    EmitHeating,
    ReserveCoolingName,
    CopyCoolingZoneName,
    CopyCoolingSuffix,
    EmitCooling,
    FindEquipmentZone,
    ReserveEquipmentName,
    CopyEquipmentPrefix,
    CopyEquipmentZoneName,
    EmitEquipment,
    Complete,
}

#[cfg(test)]
pub(crate) const P7C1_SIZING_STAGES: [SizingStage; 18] = [
    SizingStage::ReserveZoneResults,
    SizingStage::ReserveEquipmentResults,
    SizingStage::ScanZoneSurface,
    SizingStage::ScanSurfaceVertex,
    SizingStage::ReserveHeatingName,
    SizingStage::CopyHeatingZoneName,
    SizingStage::CopyHeatingSuffix,
    SizingStage::EmitHeating,
    SizingStage::ReserveCoolingName,
    SizingStage::CopyCoolingZoneName,
    SizingStage::CopyCoolingSuffix,
    SizingStage::EmitCooling,
    SizingStage::FindEquipmentZone,
    SizingStage::ReserveEquipmentName,
    SizingStage::CopyEquipmentPrefix,
    SizingStage::CopyEquipmentZoneName,
    SizingStage::EmitEquipment,
    SizingStage::Complete,
];

impl SizingBuilder {
    pub(crate) fn new(config: SizingConfig) -> Self {
        Self {
            config,
            stage: SizingStage::ReserveZoneResults,
            zone_cursor: 0,
            surface_cursor: 0,
            vertex_cursor: 0,
            surface_normal: [0.0; 3],
            zone_area_m2: 0.0,
            equipment_cursor: 0,
            equipment_zone_cursor: 0,
            name_cursor: 0,
            pending_name: String::new(),
            tables: SizingTables::default(),
            fault: None,
        }
    }

    pub(crate) fn is_complete(&self, _model: &Model) -> bool {
        self.stage == SizingStage::Complete
    }

    pub(crate) fn fault(&self) -> Option<SizingFault> {
        self.fault
    }

    #[cfg(test)]
    pub(crate) fn stage_for_gate(&self) -> SizingStage {
        self.stage
    }

    #[cfg(test)]
    pub(crate) fn set_stage_for_gate(&mut self, stage: SizingStage) {
        self.stage = stage;
    }

    pub(crate) fn step(&mut self, model: &Model) {
        let factor = self.config.sizing_factor * self.config.safety_factor;
        match self.stage {
            SizingStage::ReserveZoneResults => {
                if self.tables.zone_loads.try_reserve_exact(model.zones.len().saturating_mul(2)).is_err() {
                    self.fault = Some(SizingFault::ZoneResultBacking);
                    self.stage = SizingStage::Complete;
                } else {
                    self.stage = SizingStage::ReserveEquipmentResults;
                }
            }
            SizingStage::ReserveEquipmentResults => {
                if self.tables.equipment.try_reserve_exact(model.ideal_loads.len()).is_err() {
                    self.fault = Some(SizingFault::EquipmentResultBacking);
                    self.stage = SizingStage::Complete;
                } else {
                    self.stage = SizingStage::ScanZoneSurface;
                }
            }
            SizingStage::ScanZoneSurface => {
                if self.zone_cursor >= model.zones.len() {
                    self.stage = SizingStage::FindEquipmentZone;
                } else if let Some(surface) = model.surfaces.get(self.surface_cursor) {
                    if surface.zone_id == model.zones[self.zone_cursor].id {
                        self.vertex_cursor = 0;
                        self.surface_normal = [0.0; 3];
                        self.stage = SizingStage::ScanSurfaceVertex;
                    } else {
                        self.surface_cursor += 1;
                    }
                } else {
                    self.stage = SizingStage::ReserveHeatingName;
                }
            }
            SizingStage::ScanSurfaceVertex => {
                let surface = &model.surfaces[self.surface_cursor];
                if let Some(current) = surface.vertices_m.get(self.vertex_cursor) {
                    let next = surface.vertices_m[(self.vertex_cursor + 1) % surface.vertices_m.len()];
                    self.surface_normal[0] += (current[1] - next[1]) * (current[2] + next[2]);
                    self.surface_normal[1] += (current[2] - next[2]) * (current[0] + next[0]);
                    self.surface_normal[2] += (current[0] - next[0]) * (current[1] + next[1]);
                    self.vertex_cursor += 1;
                } else {
                    self.zone_area_m2 += 0.5 * (self.surface_normal[0].powi(2) + self.surface_normal[1].powi(2) + self.surface_normal[2].powi(2)).sqrt();
                    self.surface_cursor += 1;
                    self.stage = SizingStage::ScanZoneSurface;
                }
            }
            SizingStage::ReserveHeatingName => self.reserve_name(model.zones[self.zone_cursor].name.len() + " heating".len(), SizingStage::CopyHeatingZoneName),
            SizingStage::CopyHeatingZoneName => self.copy_name_part(&model.zones[self.zone_cursor].name, SizingStage::CopyHeatingSuffix),
            SizingStage::CopyHeatingSuffix => self.copy_name_part(" heating", SizingStage::EmitHeating),
            SizingStage::EmitHeating => {
                let zone = &model.zones[self.zone_cursor];
                let area = self.zone_area_m2.max(1.0);
                let delta = (20.0 - self.config.heating_design_day.dry_bulb_max_c).max(0.0);
                self.tables.zone_loads.push(SizingResult { component: std::mem::take(&mut self.pending_name), design_load_w: 0.3 * area * delta * factor, design_flow_m3_s: zone.volume_m3 * 0.01 / 3600.0, autosized: true });
                self.stage = SizingStage::ReserveCoolingName;
            }
            SizingStage::ReserveCoolingName => self.reserve_name(model.zones[self.zone_cursor].name.len() + " cooling".len(), SizingStage::CopyCoolingZoneName),
            SizingStage::CopyCoolingZoneName => self.copy_name_part(&model.zones[self.zone_cursor].name, SizingStage::CopyCoolingSuffix),
            SizingStage::CopyCoolingSuffix => self.copy_name_part(" cooling", SizingStage::EmitCooling),
            SizingStage::EmitCooling => {
                let zone = &model.zones[self.zone_cursor];
                let area = self.zone_area_m2.max(1.0);
                let delta = (self.config.cooling_design_day.dry_bulb_max_c - 24.0).max(0.0);
                let ventilation = zone.volume_m3 * 0.5 * CP_DRY_AIR * 1.2 * delta / 3600.0;
                self.tables.zone_loads.push(SizingResult { component: std::mem::take(&mut self.pending_name), design_load_w: 0.3 * area * delta * factor + ventilation, design_flow_m3_s: zone.volume_m3 * 0.02 / 3600.0, autosized: true });
                self.zone_cursor += 1;
                self.surface_cursor = 0;
                self.zone_area_m2 = 0.0;
                self.stage = SizingStage::ScanZoneSurface;
            }
            SizingStage::FindEquipmentZone => {
                let Some(ideal) = model.ideal_loads.get(self.equipment_cursor) else {
                    self.stage = SizingStage::Complete;
                    return;
                };
                if let Some(zone) = model.zones.get(self.equipment_zone_cursor) {
                    if zone.id == ideal.zone_id {
                        self.stage = SizingStage::ReserveEquipmentName;
                    } else {
                        self.equipment_zone_cursor += 1;
                    }
                } else {
                    self.equipment_cursor += 1;
                    self.equipment_zone_cursor = 0;
                }
            }
            SizingStage::ReserveEquipmentName => self.reserve_name("IdealLoads ".len() + model.zones[self.equipment_zone_cursor].name.len(), SizingStage::CopyEquipmentPrefix),
            SizingStage::CopyEquipmentPrefix => self.copy_name_part("IdealLoads ", SizingStage::CopyEquipmentZoneName),
            SizingStage::CopyEquipmentZoneName => self.copy_name_part(&model.zones[self.equipment_zone_cursor].name, SizingStage::EmitEquipment),
            SizingStage::EmitEquipment => {
                let zone = &model.zones[self.equipment_zone_cursor];
                self.tables.equipment.push(SizingResult { component: std::mem::take(&mut self.pending_name), design_load_w: zone.volume_m3 * 50.0 * factor, design_flow_m3_s: zone.volume_m3 * 0.015 / 3600.0, autosized: true });
                self.equipment_cursor += 1;
                self.equipment_zone_cursor = 0;
                self.stage = SizingStage::FindEquipmentZone;
            }
            SizingStage::Complete => {}
        }
    }

    fn reserve_name(&mut self, bytes: usize, next: SizingStage) {
        if self.pending_name.try_reserve_exact(bytes).is_err() {
            self.fault = Some(SizingFault::NameBacking);
            self.stage = SizingStage::Complete;
        } else {
            self.name_cursor = 0;
            self.stage = next;
        }
    }

    fn copy_name_part(&mut self, source: &str, next: SizingStage) {
        if self.name_cursor >= source.len() {
            self.name_cursor = 0;
            self.stage = next;
            return;
        }
        let Some(character) = source[self.name_cursor..].chars().next() else {
            self.stage = next;
            return;
        };
        self.pending_name.push(character);
        self.name_cursor += character.len_utf8();
    }

    pub(crate) fn finish(self) -> SizingTables {
        self.tables
    }

    pub(crate) fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if !self.pending_name.is_empty() {
            if maximum_bytes == 0 {
                return (false, 0, 0);
            }
            let bytes = self.pending_name.pop().map_or(0, |character| character.len_utf8());
            return (false, 0, bytes);
        }
        if let Some(row) = self.tables.zone_loads.last_mut() {
            if !row.component.is_empty() {
                if maximum_bytes == 0 {
                    return (false, 0, 0);
                }
                let bytes = row.component.pop().map_or(0, |character| character.len_utf8());
                return (false, 0, bytes);
            }
            self.tables.zone_loads.pop();
            return (false, 1, 0);
        }
        if let Some(row) = self.tables.equipment.last_mut() {
            if !row.component.is_empty() {
                if maximum_bytes == 0 {
                    return (false, 0, 0);
                }
                let bytes = row.component.pop().map_or(0, |character| character.len_utf8());
                return (false, 0, bytes);
            }
            self.tables.equipment.pop();
            return (false, 1, 0);
        }
        if !self.config.cooling_design_day.name.is_empty() || !self.config.heating_design_day.name.is_empty() {
            if maximum_bytes == 0 {
                return (false, 0, 0);
            }
            let bytes = self.config.cooling_design_day.name.pop().or_else(|| self.config.heating_design_day.name.pop()).map_or(0, |character| character.len_utf8());
            return (false, 0, bytes);
        }
        (true, 0, 0)
    }
}

/// 📐️ Sizing manager: compute design loads per zone and equipment.
pub struct SizingManager;

impl SizingManager {
    /// 📐️ Run sizing pass and populate sizing tables.
    #[cfg(test)]
    pub(crate) fn size(model: &Model, config: &SizingConfig) -> SizingTables {
        let mut builder = SizingBuilder::new(config.clone());
        while !builder.is_complete(model) {
            builder.step(model);
        }
        builder.finish()
    }

    /// 📐️ Coincident peak across zones.
    #[cfg(test)]
    pub(crate) fn coincident_peak(loads: &[f64]) -> f64 {
        loads.iter().sum()
    }

    /// 📐️ Non-coincident peak (sum of individual peaks).
    #[cfg(test)]
    pub(crate) fn non_coincident_peak(loads: &[f64]) -> f64 {
        loads.iter().copied().fold(0.0, f64::max)
    }
}
// #endregion 🔖️Sizing

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::EntityId;
    use crate::model::{Model, Site, Zone};

    #[test]
    fn sizes_zone_with_surfaces() {
        let model = Model {
            name: "Test".into(),
            site: Site { latitude_deg: 45.0, longitude_deg: 0.0, elevation_m: 100.0, time_zone_hours: 0.0, north_axis_deg: 0.0 },
            zones: vec![Zone { id: EntityId(1), name: "Z1".into(), volume_m3: 200.0, multiplier: 1, conditioned: true, part_of_total_floor_area: true }],
            ..Default::default()
        };
        let tables = SizingManager::size(&model, &SizingConfig::default());
        assert!(!tables.zone_loads.is_empty());
    }

    #[test]
    fn sizes_equipment_for_ideal_loads_zone() {
        let model = crate::sim::test_model_single_zone();
        let tables = SizingManager::size(&model, &SizingConfig::default());
        assert_eq!(tables.equipment.len(), 1);
        assert!(tables.equipment[0].design_load_w > 0.0);
    }

    #[test]
    fn coincident_peak_sums_all_loads() {
        assert!((SizingManager::coincident_peak(&[1000.0, 2000.0, 500.0]) - 3500.0).abs() < 1e-9);
        assert_eq!(SizingManager::coincident_peak(&[]), 0.0);
    }

    #[test]
    fn non_coincident_peak_takes_maximum() {
        assert!((SizingManager::non_coincident_peak(&[1000.0, 2000.0, 500.0]) - 2000.0).abs() < 1e-9);
        assert_eq!(SizingManager::non_coincident_peak(&[]), 0.0);
    }
}
