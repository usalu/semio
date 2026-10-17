//! 🏭️ The zone commit and the bounded plant, PV, battery, hot water, refrigeration and water stages.

use super::*;

// #region 🔖️TimestepJob
impl TimestepWork {
    pub(super) fn step_zone_commit(&mut self, pre: &PrecomputedModel, state: &mut SimulationModel) {
        let Some(zone_id) = pre.zone_order.get(self.zone_cursor).copied() else {
            self.secondary_cursor = 0;
            self.stage = TimestepStage::SecondaryPlant;
            return;
        };
        if let Some(zone_state) = state.zones.get_mut(&zone_id) {
            let steps = zone_state.hour_steps.max(1) as f64;
            zone_state.mean_air_temp_c = if zone_state.hour_steps == 0 { zone_state.air.temp_c } else { zone_state.hour_temperature_sum_c / steps };
            zone_state.delivered = zone_state.hour_delivered.scaled(1.0 / steps);
            zone_state.hour_temperature_sum_c = 0.0;
            zone_state.hour_delivered = DeliveredEnergy::default();
            zone_state.hour_steps = 0;
            let delivered = zone_state.delivered;
            state.delivered_total = accumulate_delivered(&state.delivered_total, &delivered);
        }
        self.zone_cursor += 1;
    }

    pub(super) fn step_plant_bounded(&mut self, model: &Model, pre: &PrecomputedModel, state: &mut SimulationModel) {
        let Some(plant) = model.plant_loops.get(self.secondary_cursor) else {
            self.secondary_cursor = 0;
            self.plant_work = None;
            self.stage = TimestepStage::SecondaryPv;
            return;
        };
        let work = self.plant_work.get_or_insert(PlantWork { plant_index: self.secondary_cursor, stage: PlantStage::ReduceZoneLoad, zone_cursor: 0, equipment_cursor: 0, total_load_w: 0.0, remaining_load_w: 0.0, first_load_w: 0.0 });
        match work.stage {
            PlantStage::ReduceZoneLoad => {
                if let Some(zone_id) = pre.zone_order.get(work.zone_cursor) {
                    if let Some(zone) = state.zones.get(zone_id) {
                        work.total_load_w += zone.heating_demand_w + zone.cooling_demand_w;
                    }
                    work.zone_cursor += 1;
                } else {
                    work.remaining_load_w = work.total_load_w;
                    work.stage = PlantStage::BuildPriority;
                }
            }
            PlantStage::BuildPriority => {
                if work.equipment_cursor < plant.equipment_ids.len() {
                    work.equipment_cursor += 1;
                } else {
                    work.equipment_cursor = 0;
                    work.stage = PlantStage::Dispatch;
                }
            }
            PlantStage::Dispatch => {
                if plant.equipment_ids.get(work.equipment_cursor).is_some() {
                    let load = if work.remaining_load_w.is_nan() { 100_000.0 } else { work.remaining_load_w.clamp(0.0, 100_000.0) };
                    if work.equipment_cursor == 0 {
                        work.first_load_w = load;
                    }
                    work.remaining_load_w -= load;
                    work.equipment_cursor += 1;
                } else {
                    work.stage = PlantStage::Simulate;
                }
            }
            PlantStage::Simulate => {
                let pump = Pump { design_head_pa: 200_000.0, design_flow_kg_s: plant.design_flow_kg_s, motor_efficiency: 0.85, part_load_curve: PerformanceCurve::Constant(1.0) };
                let loop_simulation =
                    PlantLoopSimulation { supply: PlantStream::new(plant.supply_temperature_c, plant.design_flow_kg_s), return_stream: PlantStream::new(plant.return_temperature_c, plant.design_flow_kg_s), pump, glycol_fraction: 0.0 };
                let output = loop_simulation.simulate(work.first_load_w);
                state.delivered_total.pump_w += output.electrical_power_w;
                state.plant_supply_c = output.outlet.temperature_c;
                self.secondary_cursor += 1;
                self.plant_work = None;
            }
        }
    }

    pub(super) fn step_pv(&mut self, model: &Model, state: &mut SimulationModel) {
        let Some(pv) = model.pv_systems.get(self.secondary_cursor) else {
            state.delivered_total.pv_generation_w += self.pv_generation_w;
            self.secondary_cursor = 0;
            self.stage = TimestepStage::SecondaryBattery;
            return;
        };
        self.secondary_cursor += 1;
        let system =
            PvSystem { dc_capacity_w: pv.dc_capacity_w, module_efficiency: pv.module_efficiency, area_m2: pv.area_m2, inverter_efficiency: pv.inverter_efficiency, temperature_coefficient: -0.004, tilt_deg: pv.tilt_deg, azimuth_deg: pv.azimuth_deg };
        let plane_irradiance = (self.weather.current.direct_normal_irradiance_w_m2 + self.weather.current.diffuse_horizontal_irradiance_w_m2) * system.orientation_factor(self.sun_alt, self.sun_az);
        self.pv_generation_w += system.simulate(plane_irradiance, self.weather.current.dry_bulb_c + 10.0);
    }

    pub(super) fn step_battery_bounded(&mut self, model: &Model, pre: &PrecomputedModel, state: &mut SimulationModel) {
        let Some(battery) = model.battery_storage.get(self.secondary_cursor) else {
            self.secondary_cursor = 0;
            self.battery_work = None;
            self.stage = TimestepStage::SecondaryServiceHotWater;
            return;
        };
        let work = self.battery_work.get_or_insert(BatteryWork { battery_index: self.secondary_cursor, zone_cursor: 0, net_load_w: 0.0 });
        if let Some(zone_id) = pre.zone_order.get(work.zone_cursor) {
            if let Some(zone) = state.zones.get(zone_id) {
                work.net_load_w += zone.delivered.total_electric_w();
            }
            work.zone_cursor += 1;
            return;
        }
        let charge_w = if self.pv_generation_w > work.net_load_w { (self.pv_generation_w - work.net_load_w).min(battery.max_charge_w) } else { -(work.net_load_w - self.pv_generation_w).min(battery.max_discharge_w) };
        state.battery_soc = (state.battery_soc + charge_w * 3600.0 / (battery.capacity_kwh * 3_600_000.0)).clamp(0.0, 1.0);
        state.delivered_total.battery_charge_w += charge_w.max(0.0);
        let transformer = Transformer { rated_kva: 100.0, no_load_loss_w: 50.0, load_loss_w: 200.0, impedance_fraction: 0.02 };
        let _ = grid_balance(work.net_load_w, self.pv_generation_w, 0.0, 0.0, charge_w, &transformer);
        self.secondary_cursor += 1;
        self.battery_work = None;
    }

    pub(super) fn step_service_hot_water(&mut self, model: &Model, config: &SimulationConfig, state: &mut SimulationModel) {
        let Some(system) = model.shw_systems.get(self.secondary_cursor) else {
            self.secondary_cursor = 0;
            self.stage = TimestepStage::SecondaryRefrigeration;
            return;
        };
        let Some(schedule) = schedule_lookup_step(&mut self.schedule_lookup, &config.schedules, system.schedule_id, &self.context) else {
            return;
        };
        self.secondary_cursor += 1;
        state.delivered_total.shw_electric_w += system.heater_capacity_w * schedule * 0.3;
    }

    pub(super) fn step_refrigeration(&mut self, model: &Model, config: &SimulationConfig, state: &mut SimulationModel) {
        let Some(system) = model.refrigeration_systems.get(self.secondary_cursor) else {
            self.secondary_cursor = 0;
            self.stage = TimestepStage::SecondaryWater;
            return;
        };
        let Some(schedule) = schedule_lookup_step(&mut self.schedule_lookup, &config.schedules, system.defrost_schedule_id, &self.context) else {
            return;
        };
        self.secondary_cursor += 1;
        state.delivered_total.refrigeration_w += system.design_load_w * schedule;
    }

    pub(super) fn step_water(&mut self, model: &Model, config: &SimulationConfig, state: &mut SimulationModel) {
        let Some(system) = model.water_systems.get(self.secondary_cursor) else {
            state.hour = self.hour as u32;
            self.stage = TimestepStage::Complete;
            return;
        };
        let Some(schedule) = schedule_lookup_step(&mut self.schedule_lookup, &config.schedules, system.schedule_id, &self.context) else {
            return;
        };
        self.secondary_cursor += 1;
        state.delivered_total.water_pump_w += system.peak_flow_l_s * 1000.0 * schedule * 50.0;
    }
}

// #endregion 🔖️TimestepJob
