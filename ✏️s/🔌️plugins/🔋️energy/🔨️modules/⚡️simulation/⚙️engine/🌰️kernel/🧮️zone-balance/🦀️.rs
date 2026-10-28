//! 🧮️ The coupled surface and zone heat balance: window solar, beam patches, faces, the dense solve, settling and commit.

use super::*;

// #region 🔖️TimestepJob
impl TimestepWork {
    pub(super) fn step_balance(&mut self,model:&Model,pre:&PrecomputedModel,state:&mut SimulationModel,budget:DenseSolveBudget)->TimestepStep {
        let mut work=self.balance.take().unwrap_or_else(BalanceWork::new);let mut receipt=DenseSolveDemand::default();
        let result=self.balance_unit(&mut work,model,pre,state,budget,&mut receipt);
        let fault=match result {Ok(true)=>{self.zone_cursor=0;self.stage=TimestepStage::ZoneCommit;None},Ok(false)=>{self.balance=Some(work);None},Err(fault)=>{self.balance=Some(work);Some(fault)}};
        TimestepStep {progress:RetainedCloneProgress {copied_items:receipt.items as usize,copied_bytes:receipt.copy_bytes as usize,retained_capacity_bytes:0,released_bytes:0},fault}
    }

    fn balance_unit(&mut self, work: &mut BalanceWork, model: &Model, pre: &PrecomputedModel, state: &mut SimulationModel,budget:DenseSolveBudget,receipt:&mut DenseSolveDemand) -> Result<bool,DenseSolveFault> {
        match work.stage {
            BalanceStage::Weather => {
                let zone_step = self.substep_cursor * self.zone_steps / self.substeps.max(1) + 1;
                let weather = TimestepWeather::interpolate(&self.weather.previous, &self.weather.current, &self.weather.next, zone_step, self.zone_steps);
                let hour_of_day = self.weather.current.hour as f64 + zone_step as f64 / self.zone_steps as f64;
                let sun = sun_direction(model.site.latitude_deg, model.site.longitude_deg, model.site.time_zone_hours, self.date.day_of_year(), hour_of_day);
                self.sky = SkyState::new(sun, weather.beam_normal_w_m2, weather.diffuse_horizontal_w_m2, model.site.elevation_m, GROUND_REFLECTANCE);
                self.step_weather = weather;
                *work = BalanceWork { stage: BalanceStage::Clear, ..BalanceWork::new() };
            }
            BalanceStage::Clear => {
                let Some(zone_id) = pre.zone_order.get(work.zone_cursor).copied() else {
                    self.substep_cursor += 1;
                    if self.substep_cursor >= self.substeps {
                        return Ok(true);
                    }
                    *work = BalanceWork::new();
                    return Ok(false);
                };
                let faces = pre.enclosures.get(&zone_id).map_or(0, |enclosure| enclosure.faces.len());
                let solver = &mut state.solver;
                solver.inside_absorbed_w_m2[..faces].fill(0.0);
                solver.pane_absorbed_w_m2[..faces * MAX_PANES].fill(0.0);
                solver.inside_convection_w_m2k[..faces].fill(0.0);
                solver.reduction[..faces * 3].fill(0.0);
                if faces <= precompute::EXACT_ENCLOSURE_FACES {
                    let order = faces + 1;
                    solver.matrix[..order * order].fill(0.0);
                    solver.rhs[..order].fill(0.0);
                }
                work.stage = BalanceStage::WindowSolar;
            }
            BalanceStage::WindowSolar => self.unit_window_solar(work, pre, state),
            BalanceStage::BeamPatches => self.unit_beam_patch(work, pre, state),
            BalanceStage::DiffuseSolar => {
                let zone_id = pre.zone_order[work.zone_cursor];
                let Some(enclosure) = pre.enclosures.get(&zone_id) else {
                    work.stage = BalanceStage::Faces;
                    return Ok(false);
                };
                let flux = work.zone_diffuse_w * enclosure.diffuse_solar_multiplier;
                for (index, face) in enclosure.faces.iter().enumerate() {
                    match face {
                        EnclosureFace::Opaque(id) => state.solver.inside_absorbed_w_m2[index] += flux * pre.surfaces.get(id).map_or(0.0, |s| s.inside_solar_absorptance),
                        EnclosureFace::Window(id) => {
                            if let Some(window) = pre.windows.get(id) {
                                for pane in 0..window.glazing.panes {
                                    state.solver.pane_absorbed_w_m2[index * MAX_PANES + pane] += flux * window.glazing.diffuse_back_absorptance[pane];
                                }
                            }
                        }
                    }
                }
                work.stage = BalanceStage::Faces;
                work.face_cursor = 0;
            }
            BalanceStage::Faces => self.unit_face(work, model, pre, state),
            BalanceStage::Solve => {
                let loads = self.zone_loads[work.zone_cursor];
                let turn=self.unit_solve(work,model,loads,pre,state,budget);*receipt=turn.receipt;if let DenseSolveStatus::Fault(fault)=turn.status{return Err(fault);}
            }
            BalanceStage::IdealLoad => {
                let zone_id = pre.zone_order[work.zone_cursor];
                if let Some(ideal) = model.ideal_loads.get(work.ideal_cursor) {
                    if ideal.zone_id == zone_id {
                        work.selected_ideal = Some(work.ideal_cursor);
                        work.fault_cursor = 0;
                        work.fault_factor = 1.0;
                        work.stage = BalanceStage::Fault;
                    } else {
                        work.ideal_cursor += 1;
                    }
                } else {
                    work.stage = BalanceStage::ZoneEquipment;
                }
            }
            BalanceStage::Fault => {
                let ideal = &model.ideal_loads[work.selected_ideal.expect("selected ideal load")];
                if let Some(fault) = model.faults.get(work.fault_cursor) {
                    if fault.target_equipment_id == ideal.id {
                        work.fault_factor = 1.0 - pre.fault_severity.get(&ideal.id).copied().unwrap_or(fault.severity);
                        work.stage = BalanceStage::ApplyIdealLoad;
                    } else {
                        work.fault_cursor += 1;
                    }
                } else {
                    work.stage = BalanceStage::ApplyIdealLoad;
                }
            }
            BalanceStage::ApplyIdealLoad => {
                let ideal = &model.ideal_loads[work.selected_ideal.expect("selected ideal load")];
                let loads = self.zone_loads[work.zone_cursor];
                let zone_state = state.zones.get(&pre.zone_order[work.zone_cursor]);
                let output = ideal_loads_deliver(
                    &IdealLoadsInput {
                        zone_temp_c: zone_state.map_or(work.free_temp_c, |zone| zone.air.temp_c),
                        zone_humidity_ratio: zone_state.map_or(self.step_weather.humidity_ratio, |zone| zone.air.humidity_ratio),
                        outdoor_temp_c: self.step_weather.dry_bulb_c,
                        outdoor_humidity_ratio: self.step_weather.humidity_ratio,
                        heating_setpoint_c: loads.heating_setpoint_c,
                        cooling_setpoint_c: loads.cooling_setpoint_c,
                        zone_heating_demand_w: work.remaining_heating_w * work.fault_factor,
                        zone_cooling_demand_w: work.remaining_cooling_w * work.fault_factor,
                        occupancy: 1.0,
                        floor_area_m2: loads.floor_area_m2,
                    },
                    &IdealLoadsConfig {
                        max_heating_supply_air_temp_c: ideal.max_heating_supply_air_temp_c,
                        min_cooling_supply_air_temp_c: ideal.min_cooling_supply_air_temp_c,
                        max_heating_capacity_w: ideal.max_heating_capacity_w,
                        max_cooling_capacity_w: ideal.max_cooling_capacity_w,
                        outdoor_air_per_person_m3_s: ideal.outdoor_air_per_person_m3_s,
                        outdoor_air_per_area_m3_s_m2: ideal.outdoor_air_per_area_m3_s_m2,
                    },
                );
                work.delivered_w += output.sensible_heating_w - output.sensible_cooling_w;
                work.remaining_heating_w = (work.remaining_heating_w - output.sensible_heating_w).max(0.0);
                work.remaining_cooling_w = (work.remaining_cooling_w - output.sensible_cooling_w).max(0.0);
                work.delivered.heating_w += output.sensible_heating_w;
                work.delivered.cooling_w += output.sensible_cooling_w;
                work.ideal_cursor += 1;
                work.selected_ideal = None;
                work.stage = BalanceStage::IdealLoad;
            }
            BalanceStage::ZoneEquipment => {
                let zone_id = pre.zone_order[work.zone_cursor];
                if let Some(assignment) = model.zone_equipment.get(work.equipment_cursor) {
                    if assignment.zone_id == zone_id && (work.remaining_heating_w > 0.0 || work.remaining_cooling_w > 0.0) {
                        let equipment = match assignment.equipment_type {
                            model::ZoneEquipmentType::FanCoil => ZoneEquipment::FanCoil {
                                heating: None,
                                cooling: None,
                                fan: fans::Fan {
                                    fan_type: fans::FanType::VariableVolume,
                                    max_flow_m3_s: 0.5,
                                    max_pressure_rise_pa: 500.0,
                                    motor_efficiency: 0.85,
                                    pressure_curve: PerformanceCurve::Constant(1.0),
                                    efficiency_curve: PerformanceCurve::Constant(0.7),
                                    part_load_curve: PerformanceCurve::Constant(1.0),
                                },
                                max_flow_m3_s: 0.5,
                            },
                            _ => ZoneEquipment::Baseboard { heating: coils::HeatingCoil::Electric { capacity_w: assignment.heating_capacity_w, efficiency: 1.0 } },
                        };
                        let zone_state = state.zones.get(&zone_id);
                        let output = equipment.simulate(&ZoneEquipmentRequest {
                            zone_temperature_c: zone_state.map_or(work.free_temp_c, |zone| zone.air.temp_c),
                            zone_humidity_ratio: zone_state.map_or(self.step_weather.humidity_ratio, |zone| zone.air.humidity_ratio),
                            heating_load_w: work.remaining_heating_w,
                            cooling_load_w: work.remaining_cooling_w,
                            outdoor_temperature_c: self.step_weather.dry_bulb_c,
                            outdoor_humidity_ratio: self.step_weather.humidity_ratio,
                            outdoor_pressure_pa: self.step_weather.pressure_pa,
                            supply_air_temp_c: 16.0,
                            supply_air_humidity_ratio: self.step_weather.humidity_ratio,
                            supply_mass_flow_kg_s: 0.1,
                        });
                        let heating = output.delivered_heating_w.clamp(0.0, work.remaining_heating_w);
                        let cooling = output.delivered_cooling_w.clamp(0.0, work.remaining_cooling_w);
                        work.delivered_w += heating - cooling;
                        work.remaining_heating_w -= heating;
                        work.remaining_cooling_w -= cooling;
                        work.delivered.heating_w += heating;
                        work.delivered.cooling_w += cooling;
                        work.delivered.fan_w += output.fan_power_w;
                        work.delivered.compressor_w += output.compressor_power_w;
                        work.delivered.gas_w += output.gas_consumption_w;
                    }
                    work.equipment_cursor += 1;
                } else {
                    work.stage = BalanceStage::Settle;
                }
            }
            BalanceStage::Settle => {let turn=self.unit_settle(work,model,pre,state,budget);*receipt=turn.receipt;if let DenseSolveStatus::Fault(fault)=turn.status{return Err(fault);}},
            BalanceStage::Commit => self.unit_commit(work, model, pre, state),
        }
        Ok(false)
    }

    fn unit_window_solar(&mut self, work: &mut BalanceWork, pre: &PrecomputedModel, state: &mut SimulationModel) {
        let zone_id = pre.zone_order[work.zone_cursor];
        let Some(enclosure) = pre.enclosures.get(&zone_id) else {
            work.stage = BalanceStage::BeamPatches;
            return;
        };
        let Some(face) = enclosure.faces.get(work.face_cursor).copied() else {
            work.stage = BalanceStage::BeamPatches;
            work.face_cursor = 0;
            work.back_cursor = 0;
            return;
        };
        let index = work.face_cursor;
        work.face_cursor += 1;
        let EnclosureFace::Window(id) = face else { return };
        let Some(window) = pre.windows.get(&id) else { return };
        if !window.sun_exposed || self.sky.sun[2] <= 0.0 {
            return;
        }
        let cosine = incidence_cosine(window.normal, self.sky.sun);
        let lit = if cosine <= 0.0 { 0.0 } else if window.casters.is_empty() { 1.0 } else { sunlit_fraction(&window.polygon, window.normal, std::iter::empty(), window.casters.iter().map(|caster| pre.casters[*caster].as_slice()), self.sky.sun) };
        state.solver.reduction[index * 3] = lit;
        let incident = self.sky.incident(window.normal, lit, window.sky_isotropic_ratio, window.sky_horizon_ratio);
        let glazing = &window.glazing;
        let mut absorbed_w_m2 = 0.0;
        for pane in 0..glazing.panes {
            let pane_w_m2 = incident.beam_w_m2 * glazing.beam_front_absorptance(pane, cosine) + incident.diffuse_w_m2() * glazing.diffuse_front_absorptance[pane];
            state.solver.pane_absorbed_w_m2[index * MAX_PANES + pane] += pane_w_m2;
            absorbed_w_m2 += pane_w_m2;
        }
        let transmitted_w = incident.diffuse_w_m2() * glazing.diffuse_transmittance * window.area_m2;
        if state.warmup_complete {
            let beam_transmitted_w = incident.beam_w_m2 * glazing.beam_transmittance(cosine) * window.area_m2;
            let dt = self.dt_s;
            if let Some(row) = state.per_surface.window_mut(id) {
                row.accumulate_solar_transmitted(transmitted_w + beam_transmitted_w, dt);
                row.accumulate_solar_absorbed(absorbed_w_m2 * window.area_m2, dt);
            }
        }
        let n = enclosure.faces.len();
        match &enclosure.radiation {
            EnclosureRadiation::Exchange { view_factors, .. } => {
                for (back_index, back) in enclosure.faces.iter().enumerate() {
                    let part = transmitted_w * view_factors[index * n + back_index];
                    if part <= 0.0 {
                        continue;
                    }
                    match back {
                        EnclosureFace::Opaque(back_id) => {
                            let Some(surface) = pre.surfaces.get(back_id) else { continue };
                            if surface.area_m2 > 0.0 {
                                state.solver.inside_absorbed_w_m2[back_index] += part * surface.inside_solar_absorptance / surface.area_m2;
                            }
                            if state.warmup_complete {
                                let dt = self.dt_s;
                                if let Some(row) = state.per_surface.opaque_mut(*back_id) {
                                    row.accumulate_solar_absorbed(part * surface.inside_solar_absorptance, dt);
                                }
                            }
                            work.zone_diffuse_w += part * (1.0 - surface.inside_solar_absorptance);
                        }
                        EnclosureFace::Window(back_id) => {
                            let Some(other) = pre.windows.get(back_id) else { continue };
                            let mut absorbed = 0.0;
                            for pane in 0..other.glazing.panes {
                                state.solver.pane_absorbed_w_m2[back_index * MAX_PANES + pane] += part * other.glazing.diffuse_back_absorptance[pane] / other.area_m2;
                                absorbed += other.glazing.diffuse_back_absorptance[pane];
                            }
                            if state.warmup_complete {
                                let dt = self.dt_s;
                                if let Some(row) = state.per_surface.window_mut(*back_id) {
                                    row.accumulate_solar_absorbed(part * absorbed, dt);
                                }
                            }
                            work.zone_diffuse_w += part * other.glazing.diffuse_back_reflectance;
                        }
                    }
                }
            }
            EnclosureRadiation::MeanRadiant { .. } => work.zone_diffuse_w += transmitted_w,
        }
    }

    fn unit_beam_patch(&mut self, work: &mut BalanceWork, pre: &PrecomputedModel, state: &mut SimulationModel) {
        let zone_id = pre.zone_order[work.zone_cursor];
        let Some(enclosure) = pre.enclosures.get(&zone_id) else {
            work.stage = BalanceStage::DiffuseSolar;
            return;
        };
        let Some(face) = enclosure.faces.get(work.face_cursor).copied() else {
            work.stage = BalanceStage::DiffuseSolar;
            work.face_cursor = 0;
            return;
        };
        let lit = state.solver.reduction[work.face_cursor * 3];
        let window = match face {
            EnclosureFace::Window(id) => pre.windows.get(&id).filter(|window| window.sun_exposed && lit > 0.0 && self.sky.beam_normal_w_m2 > 0.0 && incidence_cosine(window.normal, self.sky.sun) > 0.0),
            EnclosureFace::Opaque(_) => None,
        };
        let Some(window) = window else {
            work.face_cursor += 1;
            work.back_cursor = 0;
            return;
        };
        let Some(back) = enclosure.faces.get(work.back_cursor).copied() else {
            work.face_cursor += 1;
            work.back_cursor = 0;
            return;
        };
        let back_index = work.back_cursor;
        work.back_cursor += 1;
        let sun = self.sky.sun;
        let cosine = incidence_cosine(window.normal, sun);
        let beam_w_m2 = self.sky.beam_normal_w_m2 * window.glazing.beam_transmittance(cosine) * cosine * lit;
        match back {
            EnclosureFace::Opaque(id) => {
                let Some(surface) = pre.surfaces.get(&id) else { return };
                let mut overlap = beam_overlap_m2(&window.polygon, window.normal, &surface.polygon, surface.normal, sun);
                for hosted in &surface.windows {
                    if let Some(opening) = pre.windows.get(hosted) {
                        overlap -= beam_overlap_m2(&window.polygon, window.normal, &opening.polygon, opening.normal, sun);
                    }
                }
                let absorbed_w = beam_w_m2 * overlap.max(0.0);
                if surface.area_m2 > 0.0 {
                    state.solver.inside_absorbed_w_m2[back_index] += absorbed_w * surface.inside_solar_absorptance / surface.area_m2;
                }
                if state.warmup_complete {
                    let dt = self.dt_s;
                    if let Some(row) = state.per_surface.opaque_mut(id) {
                        row.accumulate_solar_absorbed(absorbed_w * surface.inside_solar_absorptance, dt);
                    }
                }
                work.zone_diffuse_w += absorbed_w * (1.0 - surface.inside_solar_absorptance);
            }
            EnclosureFace::Window(id) => {
                let Some(other) = pre.windows.get(&id) else { return };
                let arriving_w = beam_w_m2 * beam_overlap_m2(&window.polygon, window.normal, &other.polygon, other.normal, sun);
                if arriving_w <= 0.0 {
                    return;
                }
                let back_cosine = incidence_cosine(other.normal, sun).abs();
                let mut absorbed = 0.0;
                for pane in 0..other.glazing.panes {
                    let fraction = other.glazing.beam_back_absorptance(pane, back_cosine);
                    absorbed += fraction;
                    state.solver.pane_absorbed_w_m2[back_index * MAX_PANES + pane] += arriving_w * fraction / other.area_m2;
                }
                if state.warmup_complete {
                    let dt = self.dt_s;
                    if let Some(row) = state.per_surface.window_mut(id) {
                        row.accumulate_solar_absorbed(arriving_w * absorbed, dt);
                    }
                }
                work.zone_diffuse_w += arriving_w * (1.0 - absorbed - other.glazing.beam_back_transmittance(back_cosine)).max(0.0);
            }
        }
    }

    fn unit_face(&mut self, work: &mut BalanceWork, model: &Model, pre: &PrecomputedModel, state: &mut SimulationModel) {
        let zone_id = pre.zone_order[work.zone_cursor];
        let Some(enclosure) = pre.enclosures.get(&zone_id) else {
            work.stage = BalanceStage::Solve;
            return;
        };
        let n = enclosure.faces.len();
        let Some(face) = enclosure.faces.get(work.face_cursor).copied() else {
            work.stage = BalanceStage::Solve;
            return;
        };
        let index = work.face_cursor;
        work.face_cursor += 1;
        let loads = self.zone_loads[work.zone_cursor];
        let air_c = state.zones.get(&zone_id).map_or(self.step_weather.dry_bulb_c, |zone| zone.air.temp_c);
        let humidity_ratio = state.zones.get(&zone_id).map_or(self.step_weather.humidity_ratio, |zone| zone.air.humidity_ratio);
        let weather = self.step_weather;
        let dt = self.dt_s;
        let radiant_w_m2 = if enclosure.radiant_weight_total_m2 > 0.0 { loads.radiant_w * enclosure.emissivities[index] / enclosure.radiant_weight_total_m2 } else { 0.0 };
        let nodes = state.solver.nodes;
        let (inside_c, diagonal, rhs, inside_h, area) = match face {
            EnclosureFace::Opaque(id) => {
                let (Some(surface), Some(surface_state)) = (pre.surfaces.get(&id), state.surfaces.get(&id)) else { return };
                let chain = &surface.chain;
                let count = chain.nodes();
                let outside_c = surface_state.temperatures_c[0];
                let (h_outside, q_outside) = match surface.boundary {
                    OutsideBoundary::OutdoorAir => {
                        let cosine = incidence_cosine(surface.normal, self.sky.sun);
                        let solar_w_m2 = if surface.sun_exposed && self.sky.sun[2] > 0.0 {
                            let openings = surface.windows.iter().filter_map(|window| pre.windows.get(window)).map(|window| window.polygon.as_slice());
                            let lit = if cosine <= 0.0 { 0.0 } else if surface.casters.is_empty() { 1.0 } else { sunlit_fraction(&surface.polygon, surface.normal, openings, surface.casters.iter().map(|caster| pre.casters[*caster].as_slice()), self.sky.sun) };
                            self.sky.incident(surface.normal, lit, surface.sky_isotropic_ratio, surface.sky_horizon_ratio).total_w_m2()
                        } else {
                            0.0
                        };
                        let wet = weather.raining && surface.wind_exposed;
                        let exterior_c = if wet { weather.wet_bulb_c } else { weather.dry_bulb_c };
                        let wind = if surface.wind_exposed { wind_speed_at_height(weather.wind_speed_m_s, surface.centroid_height_m) } else { 0.0 };
                        let h_convection = if wet { 1000.0 } else { exterior_convection_w_m2k(outside_c, weather.dry_bulb_c, surface.normal[2], wind, is_windward(surface.normal[2], surface.azimuth_deg, weather.wind_direction_deg), surface.roughness_multiplier) };
                        let (h_sky, h_air, h_ground) = exterior_radiation_w_m2k(outside_c, weather.dry_bulb_c, weather.sky_temperature_c, surface.outside_emissivity, surface.normal[2]);
                        if state.warmup_complete {
                            let absorbed_w = surface.outside_solar_absorptance * solar_w_m2 * surface.area_m2;
                            if let Some(row) = state.per_surface.opaque_mut(id) {
                                row.accumulate_solar_absorbed(absorbed_w, dt);
                            }
                        }
                        (h_convection + h_air + h_sky + h_ground, (h_convection + h_air) * exterior_c + h_sky * weather.sky_temperature_c + h_ground * weather.dry_bulb_c + surface.outside_solar_absorptance * solar_w_m2)
                    }
                    OutsideBoundary::Ground => (FIXED_TEMPERATURE_CONDUCTANCE_W_M2K, FIXED_TEMPERATURE_CONDUCTANCE_W_M2K * GroundTemperatureModel::Monthly { temperatures_c: model.ground_temperature.building_surface_c }.temperature_c(self.date.day_of_year())),
                    OutsideBoundary::OtherSideTemperature => (FIXED_TEMPERATURE_CONDUCTANCE_W_M2K, FIXED_TEMPERATURE_CONDUCTANCE_W_M2K * air_c),
                    OutsideBoundary::Adiabatic => (0.0, 0.0),
                    OutsideBoundary::Interzone(partner) => {
                        let partner_c = state.surfaces.get(&partner).map_or(air_c, SurfaceState::inside_temp_c);
                        (FIXED_TEMPERATURE_CONDUCTANCE_W_M2K, FIXED_TEMPERATURE_CONDUCTANCE_W_M2K * partner_c)
                    }
                };
                let offset = index * nodes;
                let (a, b) = (&mut state.solver.chain_a[offset..offset + count - 1], &mut state.solver.chain_b[offset..offset + count - 1]);
                eliminate_chain(&chain.conductance_w_m2k, &chain.capacitance_j_m2k, &surface_state.temperatures_c, |_| 0.0, dt, h_outside, q_outside, a, b);
                let inside_c = surface_state.inside_temp_c();
                let inside_h = interior_convection_w_m2k(inside_c, air_c, surface.normal[2]);
                let storage = chain.capacitance_j_m2k[count - 1] / dt;
                let last = chain.conductance_w_m2k[count - 2];
                let diagonal = storage + last * (1.0 - b[count - 2]) + inside_h;
                let rhs = storage * inside_c + last * a[count - 2] + state.solver.inside_absorbed_w_m2[index] + radiant_w_m2;
                (inside_c, diagonal, rhs, inside_h, surface.area_m2)
            }
            EnclosureFace::Window(id) => {
                let (Some(window), Some(window_state)) = (pre.windows.get(&id), state.windows.get(&id)) else { return };
                let glazing = &window.glazing;
                let count = glazing.faces();
                let faces_c = window_state.face_temperatures_c;
                let ratio = window.coefficient_adjustment;
                let wet = weather.raining && window.wind_exposed;
                let exterior_c = if wet { weather.wet_bulb_c } else { weather.dry_bulb_c };
                let wind = if window.wind_exposed { wind_speed_at_height(weather.wind_speed_m_s, window.centroid_height_m) } else { 0.0 };
                let h_convection = ratio * if wet { 1000.0 } else { exterior_convection_w_m2k(faces_c[0], weather.dry_bulb_c, window.normal[2], wind, is_windward(window.normal[2], window.azimuth_deg, weather.wind_direction_deg), 1.0) };
                let view_sky = 0.5 * (1.0 + window.normal[2]);
                let split = view_sky.sqrt();
                let ambient_emission = STEFAN_BOLTZMANN * (exterior_c + KELVIN).powi(4);
                let incoming = view_sky * (split * STEFAN_BOLTZMANN * (weather.sky_temperature_c + KELVIN).powi(4) + (1.0 - split) * ambient_emission) + (1.0 - view_sky) * ambient_emission;
                let outside_k = faces_c[0] + KELVIN;
                let emissivity = glazing.emissivity_front[0];
                let h_linear = 4.0 * emissivity * STEFAN_BOLTZMANN * outside_k.powi(3);
                let h_outside = h_convection + h_linear;
                let q_outside = h_convection * exterior_c + emissivity * incoming + 3.0 * emissivity * STEFAN_BOLTZMANN * outside_k.powi(4) - h_linear * KELVIN;
                let mut conductance = [0.0; 2 * MAX_PANES];
                for pane in 0..glazing.panes {
                    conductance[2 * pane] = glazing.pane_conductance_w_m2k[pane];
                    if pane + 1 < glazing.panes {
                        let (outer_k, inner_k) = (faces_c[2 * pane + 1] + KELVIN, faces_c[2 * pane + 2] + KELVIN);
                        let (e1, e2) = (glazing.emissivity_back[pane], glazing.emissivity_front[pane + 1]);
                        let effective = e1 * e2 / (1.0 - (1.0 - e1) * (1.0 - e2));
                        conductance[2 * pane + 1] = gap_conductance_w_m2k(outer_k, inner_k, glazing.gap_width_m[pane], window.height_m, window.tilt_deg, glazing.gap_gas[pane]) + STEFAN_BOLTZMANN * effective * (outer_k * outer_k + inner_k * inner_k) * (outer_k + inner_k);
                    }
                }
                let sources = |node: usize| state_pane_source(&state.solver.pane_absorbed_w_m2, index, node);
                let capacitance = [0.0; 2 * MAX_PANES];
                let offset = index * nodes;
                let mut a = [0.0; 2 * MAX_PANES];
                let mut b = [0.0; 2 * MAX_PANES];
                eliminate_chain(&conductance[..count - 1], &capacitance[..count], &faces_c[..count], sources, dt, h_outside, q_outside, &mut a[..count - 1], &mut b[..count - 1]);
                state.solver.chain_a[offset..offset + count - 1].copy_from_slice(&a[..count - 1]);
                state.solver.chain_b[offset..offset + count - 1].copy_from_slice(&b[..count - 1]);
                let inside_c = faces_c[count - 1];
                let inside_h = ratio * glazing_interior_convection_w_m2k(inside_c, air_c, humidity_ratio, weather.pressure_pa, window.height_m, window.tilt_deg);
                let last = conductance[count - 2];
                let diagonal = last * (1.0 - b[count - 2]) + inside_h;
                let rhs = last * a[count - 2] + state_pane_source(&state.solver.pane_absorbed_w_m2, index, count - 1) + radiant_w_m2;
                (inside_c, diagonal, rhs, inside_h, window.area_m2)
            }
        };
        state.solver.inside_convection_w_m2k[index] = inside_h;
        work.air_diagonal_w_k += area * inside_h;
        match &enclosure.radiation {
            EnclosureRadiation::Exchange { exchange_factors, .. } => {
                let order = n + 1;
                let inside_k = inside_c + KELVIN;
                let mut diagonal = diagonal;
                for (other_index, other) in enclosure.faces.iter().enumerate() {
                    if other_index == index {
                        continue;
                    }
                    let other_k = face_inside_c(state, pre, *other, air_c) + KELVIN;
                    let h_radiation = STEFAN_BOLTZMANN * exchange_factors[index * n + other_index] * (inside_k * inside_k + other_k * other_k) * (inside_k + other_k);
                    diagonal += h_radiation;
                    state.solver.matrix[index * order + other_index] = -h_radiation;
                }
                state.solver.matrix[index * order + index] = diagonal;
                state.solver.matrix[index * order + n] = -inside_h;
                state.solver.matrix[n * order + index] = -area * inside_h;
                state.solver.rhs[index] = rhs;
            }
            EnclosureRadiation::MeanRadiant { participation } => {
                let mean_k = state.zones.get(&zone_id).map_or(air_c, |zone| zone.mean_radiant_temp_c) + KELVIN;
                let inside_k = inside_c + KELVIN;
                let h_radiation = participation[index] * (mean_k * mean_k + inside_k * inside_k) * (mean_k + inside_k);
                let total = diagonal + h_radiation;
                let (alpha, beta, gamma) = (rhs / total, inside_h / total, h_radiation / total);
                state.solver.reduction[index * 3] = alpha;
                state.solver.reduction[index * 3 + 1] = beta;
                state.solver.reduction[index * 3 + 2] = gamma;
                let sums = &mut work.mean_radiant_sums;
                sums[0] += area * inside_h * (1.0 - beta);
                sums[1] += area * inside_h * gamma;
                sums[2] += area * inside_h * alpha;
                sums[3] += area * h_radiation * (1.0 - gamma);
                sums[4] += area * h_radiation * beta;
                sums[5] += area * h_radiation * alpha;
            }
        }
    }


    pub(super) fn thermal_demand(&self) -> DenseSolveDemand {
        let Some(work)=self.balance.as_ref() else {return DenseSolveDemand::default()};
        if !matches!(work.stage,BalanceStage::Solve|BalanceStage::Settle){return DenseSolveDemand::default();}
        match work.thermal_phase {
            2|5=>work.dense.as_ref().map_or(DenseSolveDemand {items:1,copy_bytes:256,depth:1,capacity_bytes:0},|cursor|{let mut demand=cursor.demand();demand.items=demand.items.max(1);demand.depth=demand.depth.max(1);demand}),
            1|4=>DenseSolveDemand {items:1,copy_bytes:24,depth:1,capacity_bytes:0},
            _=>DenseSolveDemand {items:1,copy_bytes:256,depth:1,capacity_bytes:0},
        }
    }

    fn unit_solve(&mut self,work:&mut BalanceWork,model:&Model,loads:ZoneLoads,pre:&PrecomputedModel,state:&mut SimulationModel,budget:DenseSolveBudget)->DenseSolveTurn {
        let zone_id=pre.zone_order[work.zone_cursor];
        let n=pre.enclosures.get(&zone_id).map_or(0,|enclosure|enclosure.faces.len());
        let exact=pre.enclosures.get(&zone_id).is_none_or(|enclosure|matches!(enclosure.radiation,EnclosureRadiation::Exchange {..}));
        let order=n+1;
        let priced=|bytes|DenseSolveTurn {status:DenseSolveStatus::Pending,receipt:DenseSolveDemand {items:1,copy_bytes:bytes,depth:1,capacity_bytes:0}};
        match work.thermal_phase {
            0=>{
                let Some(zone_state)=state.zones.get(&zone_id) else {return DenseSolveTurn {status:DenseSolveStatus::Fault(DenseSolveFault::Dimensions),receipt:Default::default()}};
                let zone=&model.zones[work.zone_cursor];let weather=self.step_weather;
                let capacity=zone_state.air.capacity_rate_w_k(zone.volume_m3*zone.multiplier.max(1) as f64,weather.pressure_pa,self.dt_s);
                let (storage,history)=zone_state.air.storage_terms(capacity);
                let outdoor=loads.outdoor_air_m3_s*moist_air_density(weather.dry_bulb_c,weather.humidity_ratio,weather.pressure_pa)*moist_air_cp_j_per_kg_k(weather.humidity_ratio);
                work.air_rhs_w=history+outdoor*weather.dry_bulb_c+loads.convective_w;
                if exact {
                    work.air_diagonal_w_k+=storage+outdoor;
                    state.solver.matrix[n*order+n]=work.air_diagonal_w_k;state.solver.rhs[n]=work.air_rhs_w;work.thermal_cursor=0;work.thermal_phase=1;
                } else {
                    let sums=work.mean_radiant_sums;work.air_diagonal_w_k=storage+outdoor+sums[0];
                    state.solver.scratch[..4].copy_from_slice(&[work.air_diagonal_w_k,-sums[1],-sums[4],sums[3]]);
                    state.solver.solution[..2].copy_from_slice(&[work.air_rhs_w+sums[2],sums[5]]);
                    work.dense=Some(DenseSolveCursor::new(2,4,2));work.thermal_phase=2;
                }
                priced(256)
            },
            1=>{
                let index=work.thermal_cursor;if index>=order*order+order{return DenseSolveTurn {status:DenseSolveStatus::Fault(DenseSolveFault::InvalidSnapshot),receipt:Default::default()}};
                if index<order*order {state.solver.scratch[index]=state.solver.matrix[index];}
                else {let row=index-order*order;state.solver.solution[row]=state.solver.rhs[row];}
                work.thermal_cursor+=1;
                if work.thermal_cursor==order*order+order {work.dense=Some(DenseSolveCursor::new(order,order*order,order));work.thermal_phase=2;}
                priced(16)
            },
            2|5=>{
                let Some(cursor)=work.dense.as_mut() else {return DenseSolveTurn {status:DenseSolveStatus::Fault(DenseSolveFault::InvalidSnapshot),receipt:Default::default()}};
                let size=cursor.order();let expected=if work.thermal_phase==2 {if exact {order}else{2}}else if exact {n}else{1};if size!=expected||state.solver.scratch.len()<size*size||state.solver.solution.len()<size{return DenseSolveTurn {status:DenseSolveStatus::Fault(DenseSolveFault::Dimensions),receipt:Default::default()}};let mut turn=cursor.step(&mut state.solver.scratch[..size*size],&mut state.solver.solution[..size],budget);
                if turn.status==DenseSolveStatus::Complete {turn.receipt.items=turn.receipt.items.max(1);turn.receipt.depth=turn.receipt.depth.max(1);work.dense=None;work.thermal_phase=if work.thermal_phase==2 {3}else{6};work.thermal_cursor=0;}
                turn
            },
            3=>{
                let free=state.solver.solution[if exact {n}else{0}];work.free_temp_c=free;
                work.target_temp_c=if !loads.controlled {None}else if free<loads.heating_setpoint_c {Some(loads.heating_setpoint_c)}else if free>loads.cooling_setpoint_c {Some(loads.cooling_setpoint_c)}else{None};
                if let Some(target)=work.target_temp_c {
                    work.thermal_cursor=0;
                    if exact {if n==0 {work.dense=Some(DenseSolveCursor::new(0,0,0));work.thermal_phase=5;}else{work.thermal_phase=4;}}
                    else {let sums=work.mean_radiant_sums;state.solver.scratch[0]=sums[3];state.solver.solution[0]=sums[5]+sums[4]*target;work.dense=Some(DenseSolveCursor::new(1,1,1));work.thermal_phase=5;}
                }else{work.required_w=0.0;work.thermal_phase=6;}
                priced(256)
            },
            4=>{
                let index=work.thermal_cursor;if n==0||index>=n*n+n{return DenseSolveTurn {status:DenseSolveStatus::Fault(DenseSolveFault::InvalidSnapshot),receipt:Default::default()}};
                if index<n*n {let (row,column)=(index/n,index%n);state.solver.scratch[index]=state.solver.matrix[row*order+column];}
                else {let row=index-n*n;state.solver.solution[row]=state.solver.rhs[row]-state.solver.matrix[row*order+n]*work.target_temp_c.expect("constrained target");}
                work.thermal_cursor+=1;
                if work.thermal_cursor==n*n+n {work.dense=Some(DenseSolveCursor::new(n,n*n,n));work.thermal_phase=5;}
                priced(if index<n*n {16}else{24})
            },
            6=>{
                if let Some(target)=work.target_temp_c {
                    if work.thermal_cursor==0 {work.required_w=work.air_diagonal_w_k*target-work.air_rhs_w;if !exact {let sums=work.mean_radiant_sums;work.required_w-=sums[1]*state.solver.solution[0]+sums[2];}work.thermal_cursor=1;return priced(256);}
                    if exact&&work.thermal_cursor<=n {let column=work.thermal_cursor-1;work.required_w+=state.solver.matrix[n*order+column]*state.solver.solution[column];work.thermal_cursor+=1;return priced(24);}
                }
                if !work.required_w.is_finite(){return DenseSolveTurn {status:DenseSolveStatus::Fault(DenseSolveFault::NonFiniteArithmetic),receipt:priced(256).receipt};}
                work.remaining_heating_w=work.required_w.max(0.0);work.remaining_cooling_w=(-work.required_w).max(0.0);work.delivered_w=0.0;work.ideal_cursor=0;work.equipment_cursor=0;work.thermal_phase=0;work.stage=BalanceStage::IdealLoad;priced(256)
            },
            _=>DenseSolveTurn {status:DenseSolveStatus::Fault(DenseSolveFault::InvalidSnapshot),receipt:Default::default()},
        }
    }

    fn unit_settle(&mut self,work:&mut BalanceWork,_model:&Model,pre:&PrecomputedModel,state:&mut SimulationModel,budget:DenseSolveBudget)->DenseSolveTurn {
        let zone_id=pre.zone_order[work.zone_cursor];let n=pre.enclosures.get(&zone_id).map_or(0,|enclosure|enclosure.faces.len());
        let exact=pre.enclosures.get(&zone_id).is_none_or(|enclosure|matches!(enclosure.radiation,EnclosureRadiation::Exchange {..}));let order=n+1;
        let priced=|bytes|DenseSolveTurn {status:DenseSolveStatus::Pending,receipt:DenseSolveDemand {items:1,copy_bytes:bytes,depth:1,capacity_bytes:0}};
        match work.thermal_phase {
            0=>{
                let met=work.target_temp_c.is_some()&&(work.delivered_w-work.required_w).abs()<=1e-9*work.required_w.abs().max(1.0);
                if met {let target=work.target_temp_c.expect("met target");if exact {state.solver.solution[n]=target;}else{state.solver.solution[1]=state.solver.solution[0];state.solver.solution[0]=target;}work.thermal_phase=3;}
                else if exact {work.thermal_cursor=0;work.thermal_phase=1;}
                else {let sums=work.mean_radiant_sums;state.solver.scratch[..4].copy_from_slice(&[work.air_diagonal_w_k,-sums[1],-sums[4],sums[3]]);state.solver.solution[..2].copy_from_slice(&[work.air_rhs_w+sums[2]+work.delivered_w,sums[5]]);work.dense=Some(DenseSolveCursor::new(2,4,2));work.thermal_phase=2;}
                priced(256)
            },
            1=>{
                let index=work.thermal_cursor;if index>=order*order+order{return DenseSolveTurn {status:DenseSolveStatus::Fault(DenseSolveFault::InvalidSnapshot),receipt:Default::default()}};if index<order*order {state.solver.scratch[index]=state.solver.matrix[index];}else{let row=index-order*order;state.solver.solution[row]=state.solver.rhs[row]+if row==n {work.delivered_w}else{0.0};}
                work.thermal_cursor+=1;if work.thermal_cursor==order*order+order {work.dense=Some(DenseSolveCursor::new(order,order*order,order));work.thermal_phase=2;}priced(16)
            },
            2=>{let Some(cursor)=work.dense.as_mut() else {return DenseSolveTurn {status:DenseSolveStatus::Fault(DenseSolveFault::InvalidSnapshot),receipt:Default::default()}};let size=cursor.order();let expected=if exact {order}else{2};if size!=expected||state.solver.scratch.len()<size*size||state.solver.solution.len()<size{return DenseSolveTurn {status:DenseSolveStatus::Fault(DenseSolveFault::Dimensions),receipt:Default::default()}};let mut turn=cursor.step(&mut state.solver.scratch[..size*size],&mut state.solver.solution[..size],budget);if turn.status==DenseSolveStatus::Complete {turn.receipt.items=turn.receipt.items.max(1);turn.receipt.depth=turn.receipt.depth.max(1);work.dense=None;work.thermal_phase=3;}turn},
            3=>{
                let air=state.solver.solution[if exact {n}else{0}];if !air.is_finite(){return DenseSolveTurn {status:DenseSolveStatus::Fault(DenseSolveFault::NonFiniteArithmetic),receipt:priced(256).receipt};}work.free_temp_c=air;
                if let Some(zone)=state.zones.get_mut(&zone_id){zone.heating_demand_w=work.required_w.max(0.0);zone.cooling_demand_w=(-work.required_w).max(0.0);zone.unmet_heating_w=work.remaining_heating_w;zone.unmet_cooling_w=work.remaining_cooling_w;}
                work.face_cursor=0;work.thermal_phase=0;work.stage=BalanceStage::Commit;priced(256)
            },
            _=>DenseSolveTurn {status:DenseSolveStatus::Fault(DenseSolveFault::InvalidSnapshot),receipt:Default::default()},
        }
    }

    fn unit_commit(&mut self, work: &mut BalanceWork, model: &Model, pre: &PrecomputedModel, state: &mut SimulationModel) {
        let zone_id = pre.zone_order[work.zone_cursor];
        let enclosure = pre.enclosures.get(&zone_id);
        let n = enclosure.map_or(0, |enclosure| enclosure.faces.len());
        let nodes = state.solver.nodes;
        if let Some(face) = enclosure.and_then(|enclosure| enclosure.faces.get(work.face_cursor)).copied() {
            let index = work.face_cursor;
            work.face_cursor += 1;
            let inside_c = match &enclosure.expect("enclosure").radiation {
                EnclosureRadiation::Exchange { .. } => state.solver.solution[index],
                EnclosureRadiation::MeanRadiant { .. } => {
                    let reduction = &state.solver.reduction[index * 3..index * 3 + 3];
                    reduction[0] + reduction[1] * state.solver.solution[0] + reduction[2] * state.solver.solution[1]
                }
            };
            let offset = index * nodes;
            let inside_h = state.solver.inside_convection_w_m2k[index];
            if state.warmup_complete {
                let (id, area_m2) = match face {
                    EnclosureFace::Opaque(id) => (id, pre.surfaces.get(&id).map_or(0.0, |surface| surface.area_m2)),
                    EnclosureFace::Window(id) => (id, pre.windows.get(&id).map_or(0.0, |window| window.area_m2)),
                };
                let flux_w = area_m2 * inside_h * (work.free_temp_c - inside_c);
                let dt = self.dt_s;
                let row = match face {
                    EnclosureFace::Opaque(_) => state.per_surface.opaque_mut(id),
                    EnclosureFace::Window(_) => state.per_surface.window_mut(id),
                };
                if let Some(row) = row {
                    row.accumulate_conduction(flux_w, dt);
                }
            }
            match face {
                EnclosureFace::Opaque(id) => {
                    let count = pre.surfaces.get(&id).map_or(0, |surface| surface.chain.nodes());
                    let solver = &state.solver;
                    if let Some(surface_state) = state.surfaces.get_mut(&id) {
                        substitute_chain(&solver.chain_a[offset..offset + count - 1], &solver.chain_b[offset..offset + count - 1], inside_c, &mut surface_state.temperatures_c[..count]);
                        surface_state.inside_convection_w_m2k = inside_h;
                    }
                }
                EnclosureFace::Window(id) => {
                    let count = pre.windows.get(&id).map_or(0, |window| window.glazing.faces());
                    let solver = &state.solver;
                    if let Some(window_state) = state.windows.get_mut(&id) {
                        substitute_chain(&solver.chain_a[offset..offset + count - 1], &solver.chain_b[offset..offset + count - 1], inside_c, &mut window_state.face_temperatures_c[..count]);
                        window_state.inside_convection_w_m2k = inside_h;
                    }
                }
            }
            return;
        }
        let exact = enclosure.is_none_or(|enclosure| matches!(enclosure.radiation, EnclosureRadiation::Exchange { .. }));
        let air_c = work.free_temp_c;
        let mean_radiant_c = if exact {
            let enclosure = enclosure.map(|enclosure| (enclosure.areas_m2.as_slice(), enclosure.emissivities.as_slice()));
            enclosure.map_or(air_c, |(areas, emissivities)| {
                let weight: f64 = (0..n).map(|i| areas[i] * emissivities[i]).sum();
                if weight > 0.0 { (0..n).map(|i| areas[i] * emissivities[i] * state.solver.solution[i]).sum::<f64>() / weight } else { air_c }
            })
        } else {
            state.solver.solution[1]
        };
        let zone = &model.zones[work.zone_cursor];
        let loads = self.zone_loads[work.zone_cursor];
        let weather = self.step_weather;
        let outdoor_mass_flow = loads.outdoor_air_m3_s * moist_air_density(weather.dry_bulb_c, weather.humidity_ratio, weather.pressure_pa);
        let delivered = work.delivered;
        if let Some(zone_state) = state.zones.get_mut(&zone_id) {
            let humidity = zone_state.air.next_humidity_ratio(zone.volume_m3 * zone.multiplier.max(1) as f64, weather.pressure_pa, self.dt_s, outdoor_mass_flow, weather.humidity_ratio, loads.latent_w);
            zone_state.air.commit(air_c, humidity);
            zone_state.mean_radiant_temp_c = mean_radiant_c;
            zone_state.hour_temperature_sum_c += air_c;
            zone_state.hour_steps += 1;
            zone_state.hour_delivered = accumulate_delivered(&zone_state.hour_delivered, &delivered);
        }
        work.next_zone();
    }
}
// #endregion 🔖️TimestepJob
