//! 🧭️ The per-zone preparation step and the schedule lookup cursor it drives.

use super::*;

// #region 🔖️TimestepJob
impl TimestepWork {
    pub(super) fn step_zone_preparation(&mut self, model: &Model, config: &SimulationConfig, pre: &PrecomputedModel, state: &mut SimulationModel) {
        let Some(zone) = model.zones.get(self.zone_cursor) else {
            self.zone_cursor = 0;
            self.substep_cursor = 0;
            self.balance = Some(BalanceWork::new());
            self.stage = TimestepStage::Balance;
            return;
        };
        if self.zone_preparation.is_none() {
            if self.zone_cursor == 0 {
                state.delivered_total = DeliveredEnergy::default();
            }
            let geometry = pre.zone_geometry.get(&zone.id).cloned().unwrap_or_default();
            let zone_state = state.zones.get(&zone.id);
            let zone_temp_c = zone_state.map_or(self.weather.current.dry_bulb_c, |value| value.air.temp_c);
            let zone_humidity_ratio = zone_state.map_or(self.step_weather.humidity_ratio, |value| value.air.humidity_ratio);
            let setpoints = pre.default_setpoints.get(&zone.id).copied().unwrap_or_default();
            self.zone_preparation = Some(ZonePreparationWork {
                stage: ZonePreparationStage::Begin,
                cursor: 0,
                zone_index: self.zone_cursor,
                zone_id: zone.id,
                floor_area_m2: geometry.floor_area_m2,
                zone_temp_c,
                zone_humidity_ratio,
                daylight_target_lux: 0.0,
                daylight_transmittance: 0.0,
                daylight_area_m2: 0.0,
                lighting_dim: 1.0,
                internal_gain: GainDecomposition::default(),
                infiltration_flow_m3_s: 0.0,
                airflow_zone_node: None,
                airflow_flow_m3_s: 0.0,
                mechanical_flow_m3_s: 0.0,
                heating_setpoint_c: setpoints.heating_c,
                cooling_setpoint_c: setpoints.cooling_c,
                thermostat_schedule: 0,
                thermostat: ThermostatSpec {
                    heating_setpoint_c: setpoints.heating_c,
                    cooling_setpoint_c: setpoints.cooling_c,
                    heating_throttle_range_k: setpoints.heating_throttle_k,
                    cooling_throttle_range_k: setpoints.cooling_throttle_k,
                    min_heating_setpoint_c: 10.0,
                    max_cooling_setpoint_c: 35.0,
                },
                humidistat: None,
                controlled: false,
            });
            return;
        }
        let work = self.zone_preparation.as_mut().expect("zone preparation authority exists");
        match work.stage {
            ZonePreparationStage::Begin => advance_zone_preparation(work, ZonePreparationStage::DaylightConfig),
            ZonePreparationStage::DaylightConfig => {
                if let Some(daylight) = model.daylight_zones.get(work.cursor) {
                    if daylight.zone_id == work.zone_id && work.daylight_target_lux == 0.0 {
                        work.daylight_target_lux = daylight.illuminance_target_lux;
                        work.daylight_transmittance = daylight.window_transmittance;
                    }
                    work.cursor += 1;
                } else {
                    advance_zone_preparation(work, ZonePreparationStage::DaylightArea);
                }
            }
            ZonePreparationStage::DaylightArea => {
                if let Some(fenestration) = model.fenestrations.get(work.cursor) {
                    let belongs_to_zone = pre.surface_indices.get(&fenestration.surface_id).and_then(|index| model.surfaces.get(*index)).is_some_and(|surface| surface.zone_id == work.zone_id);
                    if belongs_to_zone {
                        work.daylight_area_m2 += fenestration.area_m2;
                    }
                    work.cursor += 1;
                } else {
                    advance_zone_preparation(work, ZonePreparationStage::DaylightEvaluate);
                }
            }
            ZonePreparationStage::DaylightEvaluate => {
                if work.daylight_target_lux > 0.0 {
                    let factor = simplified_daylight_factor(work.daylight_area_m2, work.floor_area_m2, work.daylight_transmittance);
                    let lux = reference_point_illuminance_lux(self.weather.current.diffuse_horizontal_irradiance_w_m2 * 120.0, self.weather.current.direct_normal_irradiance_w_m2 * 120.0, self.sun_alt.max(0.0) / 90.0, work.daylight_transmittance, factor, 1.0);
                    work.lighting_dim = lighting_dimming_fraction(lux, work.daylight_target_lux, 0.1);
                }
                advance_zone_preparation(work, ZonePreparationStage::People);
            }
            ZonePreparationStage::People => {
                if let Some(person) = model.people.get(work.cursor) {
                    if person.zone_id == work.zone_id {
                        let Some(occupancy) = schedule_lookup_step(&mut self.schedule_lookup, &config.schedules, person.schedule_id, &self.context) else {
                            return;
                        };
                        work.internal_gain = work.internal_gain.add(&compute_people_gain_w(person.people_per_area * work.floor_area_m2 * occupancy, ActivityLevel::OfficeWork, 1.0, person.radiant_fraction));
                    }
                    work.cursor += 1;
                } else {
                    advance_zone_preparation(work, ZonePreparationStage::Lighting);
                }
            }
            ZonePreparationStage::Lighting => {
                if let Some(light) = model.lighting.get(work.cursor) {
                    if light.zone_id == work.zone_id {
                        let Some(schedule) = schedule_lookup_step(&mut self.schedule_lookup, &config.schedules, light.schedule_id, &self.context) else {
                            return;
                        };
                        let fraction = schedule * work.lighting_dim;
                        let power = dimmed_lighting_power_w(light.watts_per_area * work.floor_area_m2, fraction);
                        work.internal_gain = work.internal_gain.add(&compute_lighting_gain_w(power / work.floor_area_m2.max(1.0), work.floor_area_m2, 1.0, light.radiant_fraction, light.return_air_fraction));
                    }
                    work.cursor += 1;
                } else {
                    advance_zone_preparation(work, ZonePreparationStage::Equipment);
                }
            }
            ZonePreparationStage::Equipment => {
                if let Some(equipment) = model.equipment.get(work.cursor) {
                    if equipment.zone_id == work.zone_id {
                        let Some(schedule) = schedule_lookup_step(&mut self.schedule_lookup, &config.schedules, equipment.schedule_id, &self.context) else {
                            return;
                        };
                        work.internal_gain = work.internal_gain.add(&compute_equipment_gain_w(equipment.watts_per_area, work.floor_area_m2, schedule, equipment.radiant_fraction, equipment.latent_fraction));
                    }
                    work.cursor += 1;
                } else {
                    advance_zone_preparation(work, ZonePreparationStage::Infiltration);
                }
            }
            ZonePreparationStage::Infiltration => {
                if let Some(infiltration) = model.infiltrations.get(work.cursor) {
                    if infiltration.zone_id == work.zone_id {
                        let Some(schedule_factor) = schedule_lookup_step(&mut self.schedule_lookup, &config.schedules, infiltration.schedule_id, &self.context) else {
                            return;
                        };
                        let geometry = pre.zone_geometry.get(&work.zone_id).cloned().unwrap_or_default();
                        let specification = InfiltrationSpec {
                            method: infiltration.method,
                            schedule_factor,
                            ach: infiltration.design_flow_ach,
                            flow_per_exterior_area_m3_s_m2: infiltration.flow_per_exterior_area_m3_s_m2,
                            effective_leakage_area_m2: infiltration.effective_leakage_area_m2,
                            discharge_coefficient: infiltration.discharge_coefficient,
                            constant_coefficient: infiltration.constant_term_coefficient,
                            temperature_coefficient: infiltration.temperature_term_coefficient,
                            velocity_coefficient: infiltration.velocity_term_coefficient,
                            velocity_squared_coefficient: infiltration.velocity_squared_term_coefficient,
                            stack_height_m: infiltration.stack_height_m,
                        };
                        work.infiltration_flow_m3_s += infiltration_flow_m3_s(&specification, zone.volume_m3, geometry.exterior_area_m2, self.weather.current.dry_bulb_c, work.zone_temp_c, self.weather.current.wind_speed_m_s, self.weather.current.atmospheric_pressure_pa);
                    }
                    work.cursor += 1;
                } else {
                    advance_zone_preparation(work, ZonePreparationStage::AirflowNode);
                }
            }
            ZonePreparationStage::AirflowNode => {
                let node = model.airflow_network.as_ref().and_then(|network| network.zone_node_ids.get(work.cursor));
                if let Some((zone_id, node_id)) = node {
                    if *zone_id == work.zone_id && work.airflow_zone_node.is_none() {
                        work.airflow_zone_node = Some(*node_id);
                    }
                    work.cursor += 1;
                } else {
                    advance_zone_preparation(work, ZonePreparationStage::AirflowLink);
                }
            }
            ZonePreparationStage::AirflowLink => {
                let link = model.airflow_network.as_ref().and_then(|network| network.link_ids.get(work.cursor));
                if link.is_some() {
                    work.cursor += 1;
                } else {
                    advance_zone_preparation(work, ZonePreparationStage::AirflowSolve);
                }
            }
            ZonePreparationStage::AirflowSolve => {
                if work.cursor < config.tolerances.max_iterations.max(1) as usize {
                    if work.cursor == 0 && work.airflow_zone_node.is_some() {
                        let stack = (work.zone_temp_c - self.weather.current.dry_bulb_c).abs().sqrt();
                        work.airflow_flow_m3_s = 0.01 * (self.weather.current.wind_speed_m_s + stack).powf(0.65);
                    }
                    work.cursor += 1;
                } else {
                    advance_zone_preparation(work, ZonePreparationStage::MechanicalVentilation);
                }
            }
            ZonePreparationStage::MechanicalVentilation => {
                if let Some(ventilation) = model.mechanical_ventilations.get(work.cursor) {
                    if ventilation.zone_id == work.zone_id {
                        let Some(schedule) = schedule_lookup_step(&mut self.schedule_lookup, &config.schedules, ventilation.schedule_id, &self.context) else {
                            return;
                        };
                        work.mechanical_flow_m3_s += ventilation.design_flow_m3_s * schedule;
                    }
                    work.cursor += 1;
                } else {
                    advance_zone_preparation(work, ZonePreparationStage::Thermostat);
                }
            }
            ZonePreparationStage::Thermostat => {
                if let Some(thermostat) = model.thermostats.get(work.cursor) {
                    if thermostat.zone_id == work.zone_id {
                        work.controlled = true;
                        if work.thermostat_schedule == 0 {
                            let Some(schedule) = schedule_lookup_step(&mut self.schedule_lookup, &config.schedules, thermostat.heating_setpoint_schedule_id, &self.context) else {
                                return;
                            };
                            work.heating_setpoint_c = schedule;
                            work.thermostat_schedule = 1;
                            return;
                        }
                        let Some(schedule) = schedule_lookup_step(&mut self.schedule_lookup, &config.schedules, thermostat.cooling_setpoint_schedule_id, &self.context) else {
                            return;
                        };
                        work.cooling_setpoint_c = schedule;
                        work.thermostat_schedule = 0;
                    }
                    work.cursor += 1;
                } else {
                    work.thermostat.heating_setpoint_c = work.heating_setpoint_c;
                    work.thermostat.cooling_setpoint_c = work.cooling_setpoint_c;
                    advance_zone_preparation(work, ZonePreparationStage::Humidistat);
                }
            }
            ZonePreparationStage::Humidistat => {
                if let Some(humidistat) = model.humidistats.get(work.cursor) {
                    if humidistat.zone_id == work.zone_id && work.humidistat.is_none() {
                        work.humidistat = Some(HumidistatSpec {
                            humidifying_setpoint_rh: 0.4,
                            dehumidifying_setpoint_rh: 0.6,
                            humidifying_throttle_range: humidistat.humidifying_throttle_range,
                            dehumidifying_throttle_range: humidistat.dehumidifying_throttle_range,
                        });
                    }
                    work.cursor += 1;
                } else {
                    advance_zone_preparation(work, ZonePreparationStage::Publish);
                }
            }
            ZonePreparationStage::Publish => {
                let zone_index = pre.zone_indices.get(&work.zone_id).copied().unwrap_or(work.zone_index);
                self.zone_loads[zone_index] = ZoneLoads {
                    convective_w: work.internal_gain.convective_w,
                    radiant_w: work.internal_gain.radiant_w,
                    latent_w: work.internal_gain.latent_w,
                    outdoor_air_m3_s: work.infiltration_flow_m3_s + work.airflow_flow_m3_s + work.mechanical_flow_m3_s,
                    heating_setpoint_c: work.heating_setpoint_c,
                    cooling_setpoint_c: work.cooling_setpoint_c,
                    floor_area_m2: work.floor_area_m2,
                    controlled: work.controlled,
                };
                self.zone_preparation = None;
                self.zone_cursor += 1;
            }
        }
    }
}
// #endregion 🔖️TimestepJob
fn advance_zone_preparation(work: &mut ZonePreparationWork, stage: ZonePreparationStage) {
    work.stage = stage;
    work.cursor = 0;
}

pub(super) fn schedule_lookup_step(work: &mut Option<ScheduleLookupWork>, schedules: &ScheduleSet, requested_id: schedule::ScheduleId, context: &ScheduleContext) -> Option<f64> {
    let cursor = work.get_or_insert(ScheduleLookupWork { requested_id, stage: ScheduleLookupStage::Constant, cursor: 0, annual_index: None, daily_id: None, daily_fallback: ScheduleLookupStage::Weekly });
    debug_assert_eq!(cursor.requested_id, requested_id);
    match cursor.stage {
        ScheduleLookupStage::Constant => {
            if let Some(schedule) = schedules.constants.get(cursor.cursor) {
                cursor.cursor += 1;
                if schedule.id == requested_id {
                    return Some(finish_schedule_lookup(work, schedule.value));
                }
            } else {
                cursor.stage = ScheduleLookupStage::Annual;
                cursor.cursor = 0;
            }
        }
        ScheduleLookupStage::Annual => {
            if let Some(schedule) = schedules.annual.get(cursor.cursor) {
                if schedule.id == requested_id {
                    cursor.annual_index = Some(cursor.cursor);
                    cursor.stage = ScheduleLookupStage::AnnualHoliday;
                    cursor.cursor = 0;
                } else {
                    cursor.cursor += 1;
                }
            } else {
                cursor.stage = ScheduleLookupStage::Weekly;
                cursor.cursor = 0;
            }
        }
        ScheduleLookupStage::AnnualHoliday => {
            let annual = &schedules.annual[cursor.annual_index.expect("annual schedule cursor")];
            if let Some(date) = annual.holiday_dates.get(cursor.cursor) {
                cursor.cursor += 1;
                if *date == (context.year, context.month, context.day) {
                    if let Some(daily_id) = annual.holiday_daily_schedule_id {
                        cursor.daily_id = Some(daily_id);
                        cursor.daily_fallback = ScheduleLookupStage::Weekly;
                        cursor.stage = ScheduleLookupStage::ResolveDaily;
                        cursor.cursor = 0;
                    }
                }
            } else {
                cursor.stage = ScheduleLookupStage::AnnualRule;
                cursor.cursor = 0;
            }
        }
        ScheduleLookupStage::AnnualRule => {
            let annual = &schedules.annual[cursor.annual_index.expect("annual schedule cursor")];
            if let Some(rule) = annual.rules.get(cursor.cursor) {
                cursor.cursor += 1;
                if schedule_date_in_range(context.month, context.day, rule.start_month, rule.start_day, rule.end_month, rule.end_day) {
                    cursor.daily_id = Some(rule.daily_schedule_id);
                    cursor.daily_fallback = ScheduleLookupStage::Weekly;
                    cursor.stage = ScheduleLookupStage::ResolveDaily;
                    cursor.cursor = 0;
                }
            } else {
                cursor.daily_id = Some(annual.default_daily_schedule_id);
                cursor.daily_fallback = ScheduleLookupStage::Weekly;
                cursor.stage = ScheduleLookupStage::ResolveDaily;
                cursor.cursor = 0;
            }
        }
        ScheduleLookupStage::ResolveDaily => {
            if let Some(schedule) = schedules.daily.get(cursor.cursor) {
                cursor.cursor += 1;
                if Some(schedule.id) == cursor.daily_id {
                    let mut value = schedule.hourly_values[(context.hour as usize).min(23)];
                    if let Some(limits) = schedule.limits {
                        value = value.clamp(limits.min, limits.max);
                    }
                    return Some(finish_schedule_lookup(work, value));
                }
            } else {
                cursor.stage = cursor.daily_fallback;
                cursor.cursor = 0;
                cursor.daily_id = None;
            }
        }
        ScheduleLookupStage::Weekly => {
            cursor.annual_index = None;
            if let Some(schedule) = schedules.weekly.get(cursor.cursor) {
                cursor.cursor += 1;
                if schedule.id == requested_id {
                    cursor.daily_id = Some(schedule.daily_schedule_ids[schedule::weekly_day_slot(context.day_of_week)]);
                    cursor.daily_fallback = ScheduleLookupStage::DirectDaily;
                    cursor.stage = ScheduleLookupStage::ResolveDaily;
                    cursor.cursor = 0;
                }
            } else {
                cursor.stage = ScheduleLookupStage::DirectDaily;
                cursor.cursor = 0;
            }
        }
        ScheduleLookupStage::DirectDaily => {
            if let Some(schedule) = schedules.daily.get(cursor.cursor) {
                cursor.cursor += 1;
                if schedule.id == requested_id {
                    let mut value = schedule.hourly_values[(context.hour as usize).min(23)];
                    if let Some(limits) = schedule.limits {
                        value = value.clamp(limits.min, limits.max);
                    }
                    return Some(finish_schedule_lookup(work, value));
                }
            } else {
                cursor.stage = ScheduleLookupStage::TimeSeries;
                cursor.cursor = 0;
            }
        }
        ScheduleLookupStage::TimeSeries => {
            if let Some(schedule) = schedules.time_series.get(cursor.cursor) {
                cursor.cursor += 1;
                if schedule.id == requested_id {
                    let index = (context.timestep_index as usize).min(schedule.values.len().saturating_sub(1));
                    return Some(finish_schedule_lookup(work, schedule.values.get(index).copied().unwrap_or(1.0)));
                }
            } else {
                return Some(finish_schedule_lookup(work, 1.0));
            }
        }
    }
    None
}

fn finish_schedule_lookup(work: &mut Option<ScheduleLookupWork>, value: f64) -> f64 {
    *work = None;
    value
}

fn schedule_date_in_range(month: u8, day: u8, start_month: u8, start_day: u8, end_month: u8, end_day: u8) -> bool {
    let current = month as u16 * 32 + day as u16;
    let start = start_month as u16 * 32 + start_day as u16;
    let end = end_month as u16 * 32 + end_day as u16;
    if start <= end {
        current >= start && current <= end
    } else {
        current >= start || current <= end
    }
}
