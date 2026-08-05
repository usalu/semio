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
/// 📐️ Sizing manager: compute design loads per zone and equipment.
pub struct SizingManager;

impl SizingManager {
    /// 📐️ Run sizing pass and populate sizing tables.
    pub fn size(model: &Model, config: &SizingConfig) -> SizingTables {
        let mut tables = SizingTables::default();
        let sf = config.sizing_factor * config.safety_factor;

        for zone in &model.zones {
            let area = model.surfaces_for_zone(zone.id).iter().map(|s| crate::geometry::surface_area_m2(&s.vertices_m)).sum::<f64>().max(1.0);

            let u_avg = 0.3;
            let delta_t_heat = 20.0 - config.heating_design_day.dry_bulb_max_c;
            let delta_t_cool = config.cooling_design_day.dry_bulb_max_c - 24.0;
            let heating_load = u_avg * area * delta_t_heat.max(0.0) * sf;
            let cooling_load = u_avg * area * delta_t_cool.max(0.0) * sf;
            let ventilation = zone.volume_m3 * 0.5 * CP_DRY_AIR * 1.2 * delta_t_cool.max(0.0) / 3600.0;

            tables.zone_loads.push(SizingResult { component: format!("{} heating", zone.name), design_load_w: heating_load, design_flow_m3_s: zone.volume_m3 * 0.01 / 3600.0, autosized: true });
            tables.zone_loads.push(SizingResult { component: format!("{} cooling", zone.name), design_load_w: cooling_load + ventilation, design_flow_m3_s: zone.volume_m3 * 0.02 / 3600.0, autosized: true });
        }

        for ils in &model.ideal_loads {
            if let Some(zone) = model.zone_by_id(ils.zone_id) {
                tables.equipment.push(SizingResult { component: format!("IdealLoads {}", zone.name), design_load_w: zone.volume_m3 * 50.0 * sf, design_flow_m3_s: zone.volume_m3 * 0.015 / 3600.0, autosized: true });
            }
        }

        tables
    }

    /// 📐️ Coincident peak across zones.
    pub fn coincident_peak(loads: &[f64]) -> f64 {
        loads.iter().sum()
    }

    /// 📐️ Non-coincident peak (sum of individual peaks).
    pub fn non_coincident_peak(loads: &[f64]) -> f64 {
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
