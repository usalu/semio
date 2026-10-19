//! 🔍️ Job authority pipeline steps: validation, finalization, zone and facility aggregation, result build and encoded output.

use super::*;

impl EnergyJobAuthority {
    pub(super) fn fault() -> EnergyRun {
        EnergyRun::Fault
    }

    pub(super) fn begin_fault(&mut self) -> EnergyRun {
        if self.start_wire(EnergyWireKind::Fault, 0, JobPayloadStream::Fault).is_ok() {
            EnergyRun::Yield
        } else {
            Self::fault()
        }
    }

    pub(super) fn step_validation(&mut self) -> bool {
        let work = &mut self.validation;
        match work.stage {
            ValidationStage::ReserveZones => {
                if work.zone_ids.admit(self.model.zones.len()).is_err() {
                    work.fatal_code = 1;
                }
                advance_validation(work, ValidationStage::ReserveMaterials);
            }
            ValidationStage::ReserveMaterials => {
                if work.material_ids.admit(self.model.materials.len() + self.model.glazing_materials.len() + self.model.gas_materials.len()).is_err() {
                    work.fatal_code = 1;
                }
                advance_validation(work, ValidationStage::ReserveConstructions);
            }
            ValidationStage::ReserveConstructions => {
                if work.construction_ids.admit(self.model.constructions.len()).is_err() {
                    work.fatal_code = 1;
                }
                advance_validation(work, ValidationStage::ReserveSurfaces);
            }
            ValidationStage::ReserveSurfaces => {
                if work.surface_ids.admit(self.model.surfaces.len()).is_err() {
                    work.fatal_code = 1;
                }
                advance_validation(work, ValidationStage::IndexZones);
            }
            ValidationStage::IndexZones => {
                if let Some(zone) = self.model.zones.get(work.cursor) {
                    match work.zone_ids.insert(zone.id, ()) {
                        Ok(None) => {}
                        Ok(Some(_)) => work.fatal_code = 2,
                        Err(_) => work.fatal_code = 1,
                    }
                    work.cursor += 1;
                } else {
                    if work.zone_ids.is_empty() {
                        work.fatal_code = 3;
                    }
                    advance_validation(work, ValidationStage::IndexMaterials);
                }
            }
            ValidationStage::IndexMaterials => {
                let materials = self.model.materials.len();
                let glazing = self.model.glazing_materials.len();
                let layer = self.model.materials.get(work.cursor).map(|material| material.id).or_else(|| work.cursor.checked_sub(materials).and_then(|index| self.model.glazing_materials.get(index)).map(|material| material.id)).or_else(|| work.cursor.checked_sub(materials + glazing).and_then(|index| self.model.gas_materials.get(index)).map(|material| material.id));
                if let Some(layer_id) = layer {
                    match work.material_ids.insert(layer_id, ()) {
                        Ok(None) => {}
                        Ok(Some(_)) => work.fatal_code = 4,
                        Err(_) => work.fatal_code = 1,
                    }
                    work.cursor += 1;
                } else {
                    advance_validation(work, ValidationStage::IndexConstructions);
                }
            }
            ValidationStage::IndexConstructions => {
                if let Some(construction) = self.model.constructions.get(work.cursor) {
                    match work.construction_ids.insert(construction.id, ()) {
                        Ok(None) => {}
                        Ok(Some(_)) => work.fatal_code = 5,
                        Err(_) => work.fatal_code = 1,
                    }
                    work.cursor += 1;
                } else {
                    advance_validation(work, ValidationStage::IndexSurfaces);
                }
            }
            ValidationStage::IndexSurfaces => {
                if let Some(surface) = self.model.surfaces.get(work.cursor) {
                    match work.surface_ids.insert(surface.id, ()) {
                        Ok(None) => {}
                        Ok(Some(_)) => work.fatal_code = 6,
                        Err(_) => work.fatal_code = 1,
                    }
                    work.cursor += 1;
                } else {
                    advance_validation(work, ValidationStage::CheckZones);
                }
            }
            ValidationStage::CheckZones => {
                if let Some(zone) = self.model.zones.get(work.cursor) {
                    if zone.volume_m3 <= 0.0 {
                        work.fatal_code = 7;
                    }
                    work.cursor += 1;
                } else {
                    advance_validation(work, ValidationStage::CheckMaterials);
                }
            }
            ValidationStage::CheckMaterials => {
                if let Some(material) = self.model.materials.get(work.cursor) {
                    if material.thickness_m <= 0.0 || material.conductivity_w_m_k <= 0.0 {
                        work.fatal_code = 8;
                    }
                    work.cursor += 1;
                } else {
                    advance_validation(work, ValidationStage::CheckConstructions);
                }
            }
            ValidationStage::CheckConstructions => {
                if let Some(construction) = self.model.constructions.get(work.cursor) {
                    if construction.layer_material_ids.is_empty() {
                        work.fatal_code = 9;
                        work.cursor += 1;
                    } else {
                        work.stage = ValidationStage::CheckConstructionLayers;
                        work.child_cursor = 0;
                    }
                } else {
                    advance_validation(work, ValidationStage::CheckSurfaces);
                }
            }
            ValidationStage::CheckConstructionLayers => {
                let construction = &self.model.constructions[work.cursor];
                if let Some(material_id) = construction.layer_material_ids.get(work.child_cursor) {
                    if !work.material_ids.contains_key(material_id) {
                        work.fatal_code = 10;
                    }
                    work.child_cursor += 1;
                } else {
                    work.cursor += 1;
                    work.child_cursor = 0;
                    work.stage = ValidationStage::CheckConstructions;
                }
            }
            ValidationStage::CheckSurfaces => {
                if let Some(surface) = self.model.surfaces.get(work.cursor) {
                    if !work.zone_ids.contains_key(&surface.zone_id) || !work.construction_ids.contains_key(&surface.construction_id) || surface.vertices_m.len() < 3 {
                        work.fatal_code = 11;
                    }
                    if let model::OutsideBoundary::Interzone(other) = surface.outside_boundary_condition {
                        if !work.surface_ids.contains_key(&other) {
                            work.fatal_code = 12;
                        }
                    }
                    work.cursor += 1;
                } else {
                    advance_validation(work, ValidationStage::CheckFenestrations);
                }
            }
            ValidationStage::CheckFenestrations => {
                if let Some(fenestration) = self.model.fenestrations.get(work.cursor) {
                    if !work.surface_ids.contains_key(&fenestration.surface_id) {
                        work.fatal_code = 13;
                    }
                    work.cursor += 1;
                } else {
                    advance_validation(work, ValidationStage::CheckZoneReferences);
                }
            }
            ValidationStage::CheckZoneReferences => {
                let zone_id = match work.zone_reference_family {
                    0 => self.model.thermostats.get(work.cursor).map(|value| value.zone_id),
                    1 => self.model.humidistats.get(work.cursor).map(|value| value.zone_id),
                    2 => self.model.ideal_loads.get(work.cursor).map(|value| value.zone_id),
                    3 => self.model.zone_equipment.get(work.cursor).map(|value| value.zone_id),
                    4 => self.model.mechanical_ventilations.get(work.cursor).map(|value| value.zone_id),
                    5 => self.model.daylight_zones.get(work.cursor).map(|value| value.zone_id),
                    _ => {
                        advance_validation(work, ValidationStage::CheckPvThermal);
                        None
                    }
                };
                if let Some(zone_id) = zone_id {
                    if !work.zone_ids.contains_key(&zone_id) {
                        work.fatal_code = 14;
                    }
                    work.cursor += 1;
                } else if work.zone_reference_family <= 5 {
                    work.zone_reference_family += 1;
                    work.cursor = 0;
                }
            }
            ValidationStage::CheckPvThermal => {
                if let Some(pv) = self.model.pv_systems.get(work.cursor) {
                    if !pv.thermal_parameters_valid() { work.fatal_code = 15; }
                    work.cursor += 1;
                } else { advance_validation(work, ValidationStage::Complete); }
            }
            ValidationStage::Complete => return true,
        }
        false
    }

    pub(super) fn step_finalization(&mut self) -> bool {
        let work = &mut self.finalization;
        match work.stage {
            FinalizationStage::MeterTotals => {
                if let Some(name) = self.meter_order.get(work.cursor) {
                    let _ = name;
                    if let Some(meter) = self.meters.meters.get_index(work.cursor) {
                        match meter.fuel {
                            FuelType::Electricity => work.electricity_kwh += meter.energy_kwh(),
                            FuelType::NaturalGas => work.gas_kwh += meter.energy_kwh(),
                            _ => {}
                        }
                    }
                    work.cursor += 1;
                } else {
                    advance_finalization(work, FinalizationStage::FloorArea);
                }
            }
            FinalizationStage::FloorArea => {
                if let Some(zone) = self.model.zones.get(work.cursor) {
                    work.floor_area_m2 += zone.volume_m3 / 3.0;
                    work.cursor += 1;
                } else {
                    work.floor_area_m2 = work.floor_area_m2.max(1.0);
                    advance_finalization(work, FinalizationStage::HistoryResilience);
                }
            }
            FinalizationStage::HistoryResilience => {
                if let Some(temperature) = self.zone_temperature_history.get(work.cursor).copied() {
                    if temperature > 32.0 {
                        work.resilience.hours_above_heat_index_32c += 1;
                    }
                    if temperature < 10.0 {
                        work.resilience.hours_below_10c += 1;
                    }
                    if temperature < 19.0 {
                        work.resilience.unmet_heating_hours += 1;
                    }
                    if temperature > 27.0 {
                        work.resilience.unmet_cooling_hours += 1;
                    }
                    work.cursor += 1;
                } else {
                    advance_finalization(work, FinalizationStage::TariffPeriod);
                }
            }
            FinalizationStage::TariffPeriod => {
                work.annual_cost = work.electricity_kwh * 0.1 * 12.0;
                advance_finalization(work, FinalizationStage::LifeCycleYear);
            }
            FinalizationStage::LifeCycleYear => {
                if work.cursor < 25 {
                    let year = work.cursor + 1;
                    work.present_value_energy += work.annual_cost / 1.03_f64.powi(year as i32);
                    work.cursor += 1;
                } else {
                    advance_finalization(work, FinalizationStage::SummaryElectricity);
                }
            }
            FinalizationStage::SummaryElectricity => {
                let value = work.electricity_kwh;
                if build_summary_row_step(work, &mut self.final_summaries, self.result_backing.summary_slots, "Electricity", value, "kWh") {
                    advance_finalization(work, FinalizationStage::SummaryGas);
                }
            }
            FinalizationStage::SummaryGas => {
                let value = work.gas_kwh;
                if build_summary_row_step(work, &mut self.final_summaries, self.result_backing.summary_slots, "Natural Gas", value, "kWh") {
                    advance_finalization(work, FinalizationStage::SummaryEui);
                }
            }
            FinalizationStage::SummaryEui => {
                let value = work.electricity_kwh / work.floor_area_m2;
                if build_summary_row_step(work, &mut self.final_summaries, self.result_backing.summary_slots, "Energy Use Intensity", value, "kWh/m²") {
                    advance_finalization(work, FinalizationStage::SummaryAnnualCost);
                }
            }
            FinalizationStage::SummaryAnnualCost => {
                let value = work.annual_cost;
                if build_summary_row_step(work, &mut self.final_summaries, self.result_backing.summary_slots, "Annual Energy Cost", value, "USD") {
                    advance_finalization(work, FinalizationStage::SummaryLcca);
                }
            }
            FinalizationStage::SummaryLcca => {
                let value = work.present_value_energy;
                if build_summary_row_step(work, &mut self.final_summaries, self.result_backing.summary_slots, "LCCA Present Value", value, "USD") {
                    advance_finalization(work, FinalizationStage::Environmental);
                }
            }
            FinalizationStage::Environmental => {
                let factors = SourceEnergyFactors::default();
                let emissions = EmissionFactors::default();
                self.final_environmental = Some(EnvironmentalMetrics {
                    site_energy_kwh: work.electricity_kwh + work.gas_kwh,
                    source_energy_kwh: work.electricity_kwh * factors.electricity + work.gas_kwh * factors.natural_gas,
                    co2_kg: work.electricity_kwh * emissions.electricity_kg_per_kwh + work.gas_kwh * emissions.natural_gas_kg_per_kwh,
                });
                advance_finalization(work, FinalizationStage::Resilience);
            }
            FinalizationStage::Resilience => {
                self.final_resilience = Some(work.resilience.clone());
                advance_finalization(work, FinalizationStage::Complete);
            }
            FinalizationStage::Complete => return true,
        }
        false
    }

    pub(super) fn step_aggregate_zone(&mut self) -> bool {
        let Some(zone) = self.model.zones.get(self.aggregate_zone_cursor) else { return true };
        let Some(zone_state) = self.state.as_ref().and_then(|state| state.zones.get(&zone.id)) else { return true };
        let work = self.aggregate_zone_work.get_or_insert_with(AggregateZoneWork::new);
        let dt_s = 3600.0;
        match work.stage {
            AggregateZoneStage::Temperature => {
                if self.hour_index == 0 && work.phase == 0 {
                    if !copy_aggregate_names_step(work, ["Zone Air Temperature [", zone.name.as_str(), "]"]) {
                        return false;
                    }
                    work.phase = 1;
                } else if self.hour_index != 0 && work.phase == 0 {
                    work.phase = 4;
                }
                if work.phase == 1 {
                    if work.timestamps.try_reserve_exact(self.total_timesteps as usize).is_err() {
                        work.backing_rejected = true;
                        return false;
                    }
                    work.phase = 2;
                    return false;
                }
                if work.phase == 2 {
                    if work.values.try_reserve_exact(self.total_timesteps as usize).is_err() {
                        work.backing_rejected = true;
                        return false;
                    }
                    self.result_backing.samples_per_series = self.total_timesteps as usize;
                    work.phase = 3;
                    return false;
                }
                if work.phase == 3 {
                    if self.time_series.series.len() >= self.result_backing.series_slots {
                        work.backing_rejected = true;
                        return false;
                    }
                    let [map_key, series_key, order_key] = std::mem::replace(&mut work.names, std::array::from_fn(|_| String::new()));
                    if self
                        .time_series
                        .series
                        .insert_stable(
                            map_key,
                            TimeSeries { key: series_key, timestamps_hours: std::mem::take(&mut work.timestamps), values: std::mem::take(&mut work.values), unit: Unit::Celsius, admitted_samples: self.result_backing.samples_per_series },
                        )
                        .is_err()
                    {
                        work.backing_rejected = true;
                        return false;
                    }
                    self.time_series_order.push(order_key);
                    work.phase = 4;
                    return false;
                }
                let series = self.time_series.series.get_index_mut(self.aggregate_zone_cursor).expect("admitted zone series");
                if self.zone_temperature_history.len() >= self.result_backing.history_slots {
                    work.backing_rejected = true;
                    return false;
                }
                if let Err(fault) = series.append_admitted(self.hour_index as f64, zone_state.mean_air_temp_c) {
                    work.series_fault = Some(fault);
                    work.backing_rejected = true;
                    return false;
                }
                self.zone_temperature_history.push(zone_state.mean_air_temp_c);
                work.advance(AggregateZoneStage::Heating);
            }
            AggregateZoneStage::Heating => {
                if self.hour_index == 0 && work.phase == 0 {
                    if !copy_aggregate_names_step(work, ["", zone.name.as_str(), " Heating"]) {
                        return false;
                    }
                    work.phase = 1;
                } else if self.hour_index != 0 && work.phase == 0 {
                    work.phase = 2;
                }
                if work.phase == 1 {
                    insert_admitted_meter(&mut self.meters, &mut self.meter_order, work, self.result_backing.meter_slots, FuelType::Electricity, EndUse::Heating);
                    work.phase = 2;
                    return false;
                }
                self.meters.meters.get_index_mut(self.aggregate_zone_cursor * 3).expect("admitted heating meter").accumulate(zone_state.delivered.heating_w, dt_s, self.hour_index as f64);
                self.facility_electricity_j += zone_state.delivered.heating_w * dt_s;
                work.advance(AggregateZoneStage::Cooling);
            }
            AggregateZoneStage::Cooling => {
                if self.hour_index == 0 && work.phase == 0 {
                    if !copy_aggregate_names_step(work, ["", zone.name.as_str(), " Cooling"]) {
                        return false;
                    }
                    work.phase = 1;
                } else if self.hour_index != 0 && work.phase == 0 {
                    work.phase = 2;
                }
                if work.phase == 1 {
                    insert_admitted_meter(&mut self.meters, &mut self.meter_order, work, self.result_backing.meter_slots, FuelType::Electricity, EndUse::Cooling);
                    work.phase = 2;
                    return false;
                }
                self.meters.meters.get_index_mut(self.aggregate_zone_cursor * 3 + 1).expect("admitted cooling meter").accumulate(zone_state.delivered.cooling_w, dt_s, self.hour_index as f64);
                self.facility_electricity_j += zone_state.delivered.cooling_w * dt_s;
                work.advance(AggregateZoneStage::Fan);
            }
            AggregateZoneStage::Fan => {
                if self.hour_index == 0 && work.phase == 0 {
                    if !copy_aggregate_names_step(work, ["", zone.name.as_str(), " Fans"]) {
                        return false;
                    }
                    work.phase = 1;
                } else if self.hour_index != 0 && work.phase == 0 {
                    work.phase = 2;
                }
                if work.phase == 1 {
                    insert_admitted_meter(&mut self.meters, &mut self.meter_order, work, self.result_backing.meter_slots, FuelType::Electricity, EndUse::Fans);
                    work.phase = 2;
                    return false;
                }
                self.meters.meters.get_index_mut(self.aggregate_zone_cursor * 3 + 2).expect("admitted fan meter").accumulate(zone_state.delivered.fan_w, dt_s, self.hour_index as f64);
                self.facility_electricity_j += zone_state.delivered.fan_w * dt_s;
                work.advance(AggregateZoneStage::Complete);
            }
            AggregateZoneStage::Complete => {
                self.aggregate_zone_work = None;
                return true;
            }
        }
        false
    }

    pub(super) fn step_aggregate_facility(&mut self) -> bool {
        let Some(state) = &self.state else { return true };
        let dt_s = 3600.0;
        if self.aggregate_facility_cursor > 1 {
            self.aggregate_facility_cursor = 0;
            self.aggregate_facility_work = None;
            return true;
        }
        let work = self.aggregate_facility_work.get_or_insert_with(AggregateZoneWork::new);
        if self.hour_index == 0 && work.phase == 0 {
            let source = if self.aggregate_facility_cursor == 0 { "Facility Heating" } else { "Facility PV" };
            if !copy_aggregate_names_step(work, [source, "", ""]) {
                return false;
            }
            work.phase = 1;
        } else if self.hour_index != 0 && work.phase == 0 {
            work.phase = 2;
        }
        match self.aggregate_facility_cursor {
            0 => {
                if work.phase == 1 {
                    insert_admitted_meter(&mut self.meters, &mut self.meter_order, work, self.result_backing.meter_slots, FuelType::Electricity, EndUse::Heating);
                    work.phase = 2;
                    return false;
                }
                self.meters.meters.get_index_mut(self.model.zones.len() * 3).expect("admitted facility heating meter").accumulate(state.delivered_total.heating_w, dt_s, self.hour_index as f64);
                self.facility_electricity_j += state.delivered_total.heating_w * dt_s;
                self.aggregate_facility_cursor = 1;
                self.aggregate_facility_work = None;
            }
            1 => {
                if work.phase == 1 {
                    insert_admitted_meter(&mut self.meters, &mut self.meter_order, work, self.result_backing.meter_slots, FuelType::OnSiteGeneration, EndUse::Generators);
                    work.phase = 2;
                    return false;
                }
                self.meters.meters.get_index_mut(self.model.zones.len() * 3 + 1).expect("admitted facility PV meter").accumulate(-state.delivered_total.pv_generation_w, dt_s, self.hour_index as f64);
                self.aggregate_facility_cursor = 2;
                self.aggregate_facility_work = None;
            }
            _ => {
                self.aggregate_facility_cursor = 0;
                return true;
            }
        }
        false
    }

    pub(super) fn step_result_build(&mut self) -> Result<bool, Error> {
        match self.result_build.stage {
            ResultBuildStage::ModelName => {
                if copy_string_step(&mut self.result_build.model_name, &mut self.result_build.cursor, &mut self.result_build.reserved, &self.model.name)? {
                    advance_result_build(&mut self.result_build, ResultBuildStage::ModelVersion);
                }
            }
            ResultBuildStage::ModelVersion => {
                if copy_string_step(&mut self.result_build.model_version, &mut self.result_build.cursor, &mut self.result_build.reserved, &self.model.version)? {
                    advance_result_build(&mut self.result_build, ResultBuildStage::WeatherLocation);
                }
            }
            ResultBuildStage::WeatherLocation => {
                let source = self.config.weather.as_ref().map_or("synthetic", |weather| weather.location.as_str());
                if copy_string_step(&mut self.result_build.weather_location, &mut self.result_build.cursor, &mut self.result_build.reserved, source)? {
                    advance_result_build(&mut self.result_build, ResultBuildStage::Assemble);
                }
            }
            ResultBuildStage::Assemble => {
                self.result = Some(Results {
                    time_series: std::mem::take(&mut self.time_series),
                    meters: std::mem::take(&mut self.meters),
                    summaries: std::mem::take(&mut self.final_summaries),
                    sizing: self.final_sizing.take().unwrap_or_default(),
                    environmental: self.final_environmental.take().unwrap_or_default(),
                    resilience: self.final_resilience.take().unwrap_or_default(),
                    diagnostics: Default::default(),
                    // 🧱️ The per-surface accumulator, projected to kWh. `self.state` is still resident
                    // here — it is only released by `close_step`'s bounded retirement.
                    per_surface: self.state.as_ref().map(|state| state.per_surface.summaries().collect()).unwrap_or_default(),
                    run_metadata: RunMetadata {
                        model_name: std::mem::take(&mut self.result_build.model_name),
                        model_version: std::mem::take(&mut self.result_build.model_version),
                        weather_location: std::mem::take(&mut self.result_build.weather_location),
                        timesteps: self.hour_index,
                        warmup_days: self.config.warmup_days,
                        elapsed_ms: self.started.elapsed().as_millis() as u64,
                    },
                });
                advance_result_build(&mut self.result_build, ResultBuildStage::Complete);
            }
            ResultBuildStage::Complete => return Ok(true),
        }
        Ok(false)
    }

    fn write_output_fragment(&mut self, context: &mut StepContext<'_>, fragment: &[u8]) -> Result<bool, OutputFault> {
        let writer = self.output_writer.as_mut().ok_or(OutputFault::BackingRejected)?;
        if writer.staged_page_len().is_none() {
            let reservation = self.commit_reservation.as_mut().ok_or(OutputFault::BackingRejected)?;
            if self.commit_pages_mounted >= reservation.pages {
                return Err(OutputFault::BackingRejected);
            }
            let source = reservation.take_source(self.commit_pages_mounted).ok_or(OutputFault::BackingRejected)?;
            match context.admit_payload_page(writer, source) {
                Ok(page) => page.stage(),
                Err(rejected) => {
                    reservation.restore_source(self.commit_pages_mounted, rejected.into_source());
                    return Err(OutputFault::BackingRejected);
                }
            }
            self.commit_pages_mounted += 1;
            return Ok(false);
        }
        if writer.staged_page_remaining() < fragment.len() {
            writer.commit_staged_page().map_err(|_| OutputFault::BackingRejected)?;
            return Ok(false);
        }
        writer.write_staged(fragment).map_err(|_| OutputFault::BackingRejected)?;
        self.commit_items_encoded = self.commit_items_encoded.checked_add(1).ok_or(OutputFault::BackingRejected)?;
        Ok(true)
    }

    pub(super) fn step_commit_census(&mut self) -> Result<bool, OutputFault> {
        let result = self.result.as_ref().ok_or(OutputFault::MissingResult)?;
        let mut work = self.commit_census.take().unwrap_or_default();
        let add_output = |work: &mut CommitCensusWork, bytes: usize, items: usize| -> Result<(), OutputFault> {
            work.output_bytes = work.output_bytes.checked_add(bytes).ok_or(OutputFault::BackingRejected)?;
            work.output_items = work.output_items.checked_add(items).ok_or(OutputFault::BackingRejected)?;
            Ok(())
        };
        let add_resident = |work: &mut CommitCensusWork, totals: Option<(usize, usize, usize)>| -> Result<(), OutputFault> {
            if let Some((pages, bytes, items)) = totals {
                work.resident_pages = work.resident_pages.checked_add(pages).ok_or(OutputFault::BackingRejected)?;
                work.resident_bytes = work.resident_bytes.checked_add(bytes).ok_or(OutputFault::BackingRejected)?;
                work.resident_items = work.resident_items.checked_add(items).ok_or(OutputFault::BackingRejected)?;
            }
            Ok(())
        };
        match work.stage {
            CommitCensusStage::Base => {
                add_output(&mut work, ENERGY_WIRE_HEADER_BYTES + 20 + 4 + 4, 4)?;
                work.stage = CommitCensusStage::MeterName;
            }
            CommitCensusStage::MeterName => {
                let Some(meter) = result.meters.meters.get_index(work.record) else {
                    work.record = 0;
                    work.byte = 0;
                    work.sample = 0;
                    work.stage = CommitCensusStage::SeriesName;
                    self.commit_census = Some(work);
                    return Ok(false);
                };
                if work.sample == 0 {
                    add_output(&mut work, 4, 1)?;
                    work.sample = 1;
                } else if work.byte < meter.name.len() {
                    let character = meter.name[work.byte..].chars().next().ok_or(OutputFault::MissingMeter)?;
                    add_output(&mut work, character.len_utf8(), 1)?;
                    work.byte += character.len_utf8();
                } else {
                    work.stage = CommitCensusStage::MeterRecord;
                }
            }
            CommitCensusStage::MeterRecord => {
                add_output(&mut work, 29, 1)?;
                work.record += 1;
                work.byte = 0;
                work.sample = 0;
                work.stage = CommitCensusStage::MeterName;
            }
            CommitCensusStage::SeriesName => {
                let Some(series) = result.time_series.series.get_index(work.record) else {
                    work.record = 0;
                    work.byte = 0;
                    work.sample = 0;
                    work.stage = CommitCensusStage::Summary;
                    self.commit_census = Some(work);
                    return Ok(false);
                };
                if work.sample == 0 {
                    add_output(&mut work, 4, 1)?;
                    work.sample = 1;
                } else if work.byte < series.key.len() {
                    let character = series.key[work.byte..].chars().next().ok_or(OutputFault::MissingSeries)?;
                    add_output(&mut work, character.len_utf8(), 1)?;
                    work.byte += character.len_utf8();
                } else {
                    work.stage = CommitCensusStage::SeriesHeader;
                }
            }
            CommitCensusStage::SeriesHeader => {
                add_output(&mut work, 5, 1)?;
                work.sample = 0;
                work.stage = CommitCensusStage::SeriesSample;
            }
            CommitCensusStage::SeriesSample => {
                let series = result.time_series.series.get_index(work.record).ok_or(OutputFault::MissingSeries)?;
                if work.sample < series.values.len() {
                    add_output(&mut work, 16, 1)?;
                    work.sample += 1;
                } else {
                    work.record += 1;
                    work.byte = 0;
                    work.sample = 0;
                    work.stage = CommitCensusStage::SeriesName;
                }
            }
            CommitCensusStage::Summary => {
                if work.record < result.summaries.annual_energy.len() {
                    add_output(&mut work, 8, 1)?;
                    work.record += 1;
                } else {
                    work.record = 0;
                    work.stage = CommitCensusStage::PreviewChannel;
                }
            }
            CommitCensusStage::PreviewChannel => {
                let packet = if work.channel_slot == 0 { self.publication.preview.as_ref() } else { self.publication.retiring_preview.as_ref() };
                add_resident(&mut work, packet.map(|packet| (packet.payload.page_count(), packet.payload.len(), packet.payload.page_count().saturating_add(1))))?;
                work.channel_slot += 1;
                if work.channel_slot > 1 {
                    work.stage = CommitCensusStage::CheckpointChannel;
                    work.channel_slot = 0;
                }
            }
            CommitCensusStage::CheckpointChannel | CommitCensusStage::CommitChannel | CommitCensusStage::FaultChannel => {
                let queue = match work.stage {
                    CommitCensusStage::CheckpointChannel => &self.publication.checkpoints,
                    CommitCensusStage::CommitChannel => &self.publication.commits,
                    CommitCensusStage::FaultChannel => &self.publication.faults,
                    _ => unreachable!("channel stage matched"),
                };
                let totals = queue.retained_totals_at(work.channel_slot);
                add_resident(&mut work, totals)?;
                work.channel_slot += 1;
                if work.channel_slot > ENERGY_WIRE_QUEUE_SLOTS {
                    work.channel_slot = 0;
                    work.stage = match work.stage {
                        CommitCensusStage::CheckpointChannel => CommitCensusStage::CommitChannel,
                        CommitCensusStage::CommitChannel => CommitCensusStage::FaultChannel,
                        CommitCensusStage::FaultChannel => CommitCensusStage::ReadyPacket,
                        _ => unreachable!("channel stage matched"),
                    };
                }
            }
            CommitCensusStage::ReadyPacket => {
                add_resident(&mut work, self.wire_ready.as_ref().map(|packet| (packet.payload.page_count(), packet.payload.len(), packet.payload.page_count().saturating_add(1))))?;
                work.stage = CommitCensusStage::RestoreInput;
            }
            CommitCensusStage::RestoreInput => {
                add_resident(&mut work, self.restore_input.as_ref().map(|packet| (packet.payload.page_count(), packet.payload.len(), packet.payload.page_count().saturating_add(1))))?;
                work.stage = CommitCensusStage::Reserve;
            }
            CommitCensusStage::Reserve => {
                let pages = work.output_bytes.checked_add(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES - 1).ok_or(OutputFault::BackingRejected)? / semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
                let aggregate_pages = work.resident_pages.checked_add(pages).ok_or(OutputFault::BackingRejected)?;
                let aggregate_bytes = work.resident_bytes.checked_add(work.output_bytes).ok_or(OutputFault::BackingRejected)?;
                let aggregate_items = work.resident_items.checked_add(work.output_items).and_then(|items| items.checked_add(pages)).ok_or(OutputFault::BackingRejected)?;
                if self.publication.commits.len >= ENERGY_WIRE_QUEUE_SLOTS
                    || pages > self.numerical_census.pages
                    || aggregate_pages > semio_framework_job::JOB_PAYLOAD_OPERATION_PAGES
                    || aggregate_bytes > self.numerical_census.observed_bytes
                    || aggregate_bytes > semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES
                    || aggregate_items > self.numerical_census.observed_items
                {
                    return Err(OutputFault::BackingRejected);
                }
                self.publication.commits.reserve_push().map_err(|_| OutputFault::BackingRejected)?;
                work.reservation = Some(EnergyCommitReservation::new(pages, work.output_bytes, work.output_items));
                work.stage = CommitCensusStage::ReservePage;
            }
            CommitCensusStage::ReservePage => {
                let reservation = work.reservation.as_mut().ok_or(OutputFault::BackingRejected)?;
                if reservation.prepared < reservation.pages {
                    reservation.sources[reservation.prepared] = Some(semio_framework_job::JobPayloadPageSource::new());
                    reservation.prepared += 1;
                } else {
                    self.commit_reservation = work.reservation.take();
                    work.stage = CommitCensusStage::Complete;
                }
            }
            CommitCensusStage::Complete => {
                return Ok(true);
            }
        }
        self.commit_census = Some(work);
        Ok(false)
    }

    pub(super) fn encode_output_step(&mut self, context: &mut StepContext<'_>) -> Result<bool, OutputFault> {
        let Some(result) = self.result.as_ref() else { return Err(OutputFault::MissingResult) };
        match self.encode_section {
            0 => {
                let identity = self.wire_identity(0);
                let header = energy_wire_header(EnergyWireKind::Commit, identity, self.numerical_census, self.stage, self.tier);
                if self.write_output_fragment(context, &header)? {
                    self.encode_section = 1;
                }
            }
            1 => {
                let mut fragment = [0; 20];
                fragment[..8].copy_from_slice(b"RESULTS1");
                fragment[8..12].copy_from_slice(&result.run_metadata.timesteps.to_le_bytes());
                fragment[12..16].copy_from_slice(&result.run_metadata.warmup_days.to_le_bytes());
                fragment[16..20].copy_from_slice(&(self.meter_order.len() as u32).to_le_bytes());
                if self.write_output_fragment(context, &fragment)? {
                    self.encode_section = 2;
                }
            }
            2 => {
                if self.meter_order.get(self.encode_record_cursor).is_some() {
                    let Some(meter) = result.meters.meters.get_index(self.encode_record_cursor) else { return Err(OutputFault::MissingMeter) };
                    if self.encode_fragment_phase == 0 {
                        if self.write_output_fragment(context, &(meter.name.len() as u32).to_le_bytes())? {
                            self.encode_fragment_phase = 1;
                        }
                    } else if self.encode_fragment_phase == 1 && self.encode_name_cursor < meter.name.len() {
                        let character = meter.name[self.encode_name_cursor..].chars().next().ok_or(OutputFault::MissingMeter)?;
                        let mut storage = [0; 4];
                        let fragment = character.encode_utf8(&mut storage).as_bytes();
                        if self.write_output_fragment(context, fragment)? {
                            self.encode_name_cursor += fragment.len();
                        }
                    } else {
                        let mut fragment = [0; 29];
                        fragment[0] = fuel_code(meter.fuel);
                        fragment[1..5].copy_from_slice(&end_use_code(meter.end_use).to_le_bytes());
                        fragment[5..13].copy_from_slice(&meter.energy_j.to_le_bytes());
                        fragment[13..21].copy_from_slice(&meter.peak_demand_w.to_le_bytes());
                        fragment[21..29].copy_from_slice(&meter.peak_demand_hour.to_le_bytes());
                        if self.write_output_fragment(context, &fragment)? {
                            self.encode_record_cursor += 1;
                            self.encode_name_cursor = 0;
                            self.encode_fragment_phase = 0;
                        }
                    }
                } else {
                    let fragment = (self.time_series_order.len() as u32).to_le_bytes();
                    if self.write_output_fragment(context, &fragment)? {
                        self.encode_section = 3;
                        self.encode_record_cursor = 0;
                        self.encode_name_cursor = 0;
                        self.encode_fragment_phase = 0;
                    }
                }
            }
            3 => {
                let Some(_) = self.time_series_order.get(self.encode_record_cursor) else {
                    let fragment = (result.summaries.annual_energy.len() as u32).to_le_bytes();
                    if self.write_output_fragment(context, &fragment)? {
                        self.encode_section = 4;
                        self.encode_record_cursor = 0;
                        self.encode_sample_cursor = 0;
                        self.encode_fragment_phase = 0;
                    }
                    return Ok(false);
                };
                let Some(series) = result.time_series.series.get_index(self.encode_record_cursor) else { return Err(OutputFault::MissingSeries) };
                let series_len = series.values.len();
                if self.encode_fragment_phase == 0 {
                    if self.write_output_fragment(context, &(series.key.len() as u32).to_le_bytes())? {
                        self.encode_fragment_phase = 1;
                    }
                } else if self.encode_fragment_phase == 1 && self.encode_name_cursor < series.key.len() {
                    let character = series.key[self.encode_name_cursor..].chars().next().ok_or(OutputFault::MissingSeries)?;
                    let mut storage = [0; 4];
                    let fragment = character.encode_utf8(&mut storage).as_bytes();
                    if self.write_output_fragment(context, fragment)? {
                        self.encode_name_cursor += fragment.len();
                    }
                } else if self.encode_fragment_phase <= 2 {
                    let mut fragment = [0; 5];
                    fragment[0] = unit_code(series.unit);
                    fragment[1..5].copy_from_slice(&(series_len as u32).to_le_bytes());
                    if self.write_output_fragment(context, &fragment)? {
                        self.encode_fragment_phase = 3;
                    }
                } else if let Some((&timestamp, &value)) = series.timestamps_hours.get(self.encode_sample_cursor).zip(series.values.get(self.encode_sample_cursor)) {
                    let mut fragment = [0; 16];
                    fragment[..8].copy_from_slice(&timestamp.to_le_bytes());
                    fragment[8..].copy_from_slice(&value.to_le_bytes());
                    if self.write_output_fragment(context, &fragment)? {
                        self.encode_sample_cursor += 1;
                    }
                }
                if self.encode_sample_cursor >= series_len {
                    self.encode_record_cursor += 1;
                    self.encode_sample_cursor = 0;
                    self.encode_name_cursor = 0;
                    self.encode_fragment_phase = 0;
                }
            }
            4 => {
                if let Some(row) = result.summaries.annual_energy.get(self.encode_record_cursor) {
                    if self.write_output_fragment(context, &row.value.to_le_bytes())? {
                        self.encode_record_cursor += 1;
                    }
                } else {
                    self.encode_section = 5;
                }
            }
            _ => return Ok(true),
        }
        Ok(false)
    }

}
