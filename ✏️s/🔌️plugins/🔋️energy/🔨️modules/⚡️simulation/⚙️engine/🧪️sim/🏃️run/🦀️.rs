//! 🏃️ Job authority run loop: one admitted step of the resumable simulation.

use super::*;

impl EnergyJobAuthority {
    pub fn run(&mut self, context: &mut StepContext<'_>) -> EnergyRun {
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return Self::fault();
        }
        if context.is_cancelled() {
            return EnergyRun::Cancelled;
        }
        if context.should_yield() {
            return EnergyRun::Yield;
        }
        if let Some(outcome) = self.drive_wire_publication(context) {
            return outcome;
        }
        context.consume_fuel(1);
        match self.stage {
            EnergyJobStage::Validate => {
                if self.input_identity.digest().is_none() { return match self.step_input_identity(context) { Ok(_) => EnergyRun::Yield, Err(_) => self.begin_fault() }; }
                let complete = self.step_validation();
                if self.validation.fatal_code != 0 {
                    { return self.begin_fault(); }
                }
                if !complete {
                    return EnergyRun::Yield;
                }
                self.weather_target = weather_copy_target(&self.config);
                self.set_stage(context, EnergyJobStage::ResolveWeather);
                EnergyRun::Yield
            }
            EnergyJobStage::ResolveWeather => self.run_resolve_weather(context),
            EnergyJobStage::Precompute => {
                let builder = self.precompute.as_mut().expect("precompute builder exists in Precompute stage");
                if builder.is_complete() {
                    self.pre = self.precompute.take().map(PrecomputeBuilder::finish);
                    self.state = Some(SimulationModel::default());
                    self.initialize_cursor = 0;
                    self.set_stage(context, EnergyJobStage::InitializeZones);
                } else {
                    builder.step(&self.model);
                    if builder.backing_rejected() {
                        { return self.begin_fault(); }
                    }
                }
                EnergyRun::Yield
            }
            EnergyJobStage::InitializeZones => self.run_initialize_zones(context),
            EnergyJobStage::InitializeSurfaces => self.run_initialize_surfaces(context),
            EnergyJobStage::InitializeWarmupHistory => self.run_initialize_warmup_history(context),
            EnergyJobStage::WarmupTimestep => self.run_warmup_timestep(context),
            EnergyJobStage::WarmupConvergence => self.run_warmup_convergence(context),
            EnergyJobStage::StartRun => self.run_start_run(context),
            EnergyJobStage::RunZoneTimestep => self.run_zone_timestep(context),
            EnergyJobStage::AggregateZone => {
                if self.aggregate_zone_cursor < self.model.zones.len() {
                    let complete = self.step_aggregate_zone();
                    if self.aggregate_zone_work.as_ref().is_some_and(|work| work.backing_rejected) {
                        { return self.begin_fault(); }
                    }
                    if complete {
                        self.aggregate_zone_cursor += 1;
                    }
                } else {
                    self.set_stage(context, EnergyJobStage::AggregateFacility);
                }
                EnergyRun::Yield
            }
            EnergyJobStage::AggregateFacility => {
                let complete = self.step_aggregate_facility();
                if self.aggregate_facility_work.as_ref().is_some_and(|work| work.backing_rejected) {
                    { return self.begin_fault(); }
                }
                if complete {
                    self.hour_index = self.hour_index.saturating_add(1);
                    self.checkpoint_due = self.hour_index.is_multiple_of(24);
                    self.set_stage(context, EnergyJobStage::PublishTimestep);
                }
                EnergyRun::Yield
            }
            EnergyJobStage::PublishTimestep => {
                if self.checkpoint_due {
                    self.checkpoint_due = false;
                    if self.start_wire(EnergyWireKind::Checkpoint, self.hour_index as u64, JobPayloadStream::CheckpointState).is_err() {
                        return EnergyRun::Yield;
                    }
                    return EnergyRun::Yield;
                }
                self.set_stage(context, EnergyJobStage::RunZoneTimestep);
                self.begin_preview(context)
            }
            EnergyJobStage::Finalize => {
                self.tier = EnergyQualityTier::Final;
                self.sizing_builder = Some(SizingBuilder::new(SizingConfig::default()));
                self.finalization = FinalizationWork::default();
                self.set_stage(context, EnergyJobStage::Size);
                EnergyRun::Yield
            }
            EnergyJobStage::Size => {
                let builder = self.sizing_builder.as_mut().expect("sizing builder exists");
                if builder.fault().is_some() {
                    { return self.begin_fault(); }
                }
                if builder.is_complete(&self.model) {
                    self.final_sizing = self.sizing_builder.take().map(SizingBuilder::finish);
                    self.set_stage(context, EnergyJobStage::FinalizeSummaries);
                } else {
                    builder.step(&self.model);
                }
                EnergyRun::Yield
            }
            EnergyJobStage::FinalizeSummaries => {
                let complete = self.step_finalization();
                if self.finalization.row_backing_rejected {
                    { return self.begin_fault(); }
                }
                if complete {
                    self.set_stage(context, EnergyJobStage::BuildResults);
                }
                EnergyRun::Yield
            }
            EnergyJobStage::FinalizeMetrics => {
                self.set_stage(context, EnergyJobStage::FinalizeSummaries);
                EnergyRun::Yield
            }
            EnergyJobStage::FinalizeEconomics => {
                self.set_stage(context, EnergyJobStage::FinalizeSummaries);
                EnergyRun::Yield
            }
            EnergyJobStage::BuildResults => match self.step_result_build() {
                Ok(true) => {
                    self.set_stage(context, EnergyJobStage::PublishFinal);
                    self.begin_preview(context)
                }
                Ok(false) => EnergyRun::Yield,
                Err(_) => self.begin_fault(),
            },
            EnergyJobStage::PublishFinal => {
                self.set_stage(context, EnergyJobStage::EncodeOutput);
                EnergyRun::Yield
            }
            EnergyJobStage::EncodeOutput => self.run_encode_output(context),
            EnergyJobStage::Complete => {
                if std::mem::take(&mut self.commit_acknowledged) {
                    return EnergyRun::Complete;
                }
                let Some(packet) = self.publication.commits.take_terminal() else { return EnergyRun::Yield };
                self.commit_output = Some(packet);
                EnergyRun::Complete
            }
        }
    }

    /// 🌦️ Validates one original weather row before admitting numerical precomputation.
    fn run_resolve_weather(&mut self, context: &mut StepContext<'_>) -> EnergyRun {
        if self.weather_cursor < self.weather_target {
            let receipt=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<WeatherRecord>()*4,..Default::default()};
            if !receipt.fits(context.retained_grant()){return EnergyRun::Yield;}
            let record = self.weather_record(self.weather_cursor);
            let first=self.weather_record(0);let previous=self.weather_cursor.checked_sub(1).map(|i|self.weather_record(i));
            if !crate::site::chronology::admit_row(self.weather_calendar(),first,previous,record){self.weather_fault=Some(WeatherFault::ChronologyRejected);return self.begin_fault();}
            if context.consume_retained(receipt).is_err(){return self.begin_fault();}
            self.weather_cursor += 1;
            if self.weather_cursor.is_multiple_of(256) {
                return self.begin_preview(context);
            }
            return EnergyRun::Yield;
        }
        let period=SimulationKernel::run_period(&self.config);let start=self.run_start_record();let end=crate::site::chronology::date_after_days(crate::calendar::SimDate::new(period.year,period.start_month,period.start_day),period.total_hours()/24-1).and_then(|date|self.weather_index(date,23));
        if start.zip(end).is_none_or(|(start,end)|end.checked_sub(start).and_then(|n|n.checked_add(1))!=Some(period.total_hours()as usize)){self.weather_fault=Some(WeatherFault::RunHourMissing);return self.begin_fault();}
        self.precompute = Some(PrecomputeBuilder::new(self.config.zone_timestep_minutes, self.config.system_timestep_minutes));
        self.set_stage(context, EnergyJobStage::Precompute);
        EnergyRun::Yield
    }

    /// 🪵️ Initializes the zone states of the simulation model.
    fn run_initialize_zones(&mut self, context: &mut StepContext<'_>) -> EnergyRun {
        if self.initialize_backing_stage == 0 {
            if self.state.as_mut().expect("state exists while reserving zones").zones.admit(self.model.zones.len()).is_err() {
                { return self.begin_fault(); }
            }
            self.initialize_backing_stage = 1;
            return EnergyRun::Yield;
        }
        if let Some(zone) = self.model.zones.get(self.initialize_cursor) {
            let weather = self.weather_record(self.run_start_record().expect("admitted run start"));
            if self.state.as_mut().expect("state exists while initializing").zones.insert(zone.id, ZoneState::new(INITIAL_TEMPERATURE_C, weather.humidity_ratio())).is_err() {
                { return self.begin_fault(); }
            }
            self.initialize_cursor += 1;
        } else {
            self.initialize_cursor = 0;
            self.initialize_backing_stage = 2;
            self.set_stage(context, EnergyJobStage::InitializeSurfaces);
        }
        EnergyRun::Yield
    }

    /// 🔩️ Initializes the surface states of the simulation model.
    fn run_initialize_surfaces(&mut self, context: &mut StepContext<'_>) -> EnergyRun {
        if self.initialize_backing_stage == 2 {
            if self.state.as_mut().expect("state exists while reserving surfaces").surfaces.admit(self.model.surfaces.len()).is_err() {
                { return self.begin_fault(); }
            }
            if self.state.as_mut().expect("state exists while reserving surfaces").per_surface.admit(self.model.surfaces.len(), self.model.fenestrations.len()).is_err() {
                return self.begin_fault();
            }
            self.initialize_backing_stage = 3;
            return EnergyRun::Yield;
        }
        if self.initialize_backing_stage == 3 {
            if let Some(surface) = self.model.surfaces.get(self.initialize_cursor) {
                if let Some(nodes) = self.pre.as_ref().and_then(|pre| pre.surfaces.get(&surface.id)).map(|precomputed| precomputed.chain.nodes()) {
                    let mut temperatures_c = Vec::new();
                    if temperatures_c.try_reserve_exact(nodes).is_err() {
                        { return self.begin_fault(); }
                    }
                    temperatures_c.resize(nodes, INITIAL_TEMPERATURE_C);
                    if self.state.as_mut().expect("state exists while initializing").surfaces.insert(surface.id, SurfaceState { temperatures_c, inside_convection_w_m2k: 0.0 }).is_err() {
                        { return self.begin_fault(); }
                    }
                    if self.state.as_mut().expect("state exists while initializing").per_surface.insert_opaque(surface.id).is_err() {
                        return self.begin_fault();
                    }
                }
                self.initialize_cursor += 1;
                return EnergyRun::Yield;
            }
            self.initialize_cursor = 0;
            self.initialize_backing_stage = 4;
            return EnergyRun::Yield;
        }
        if self.initialize_backing_stage == 4 {
            if self.state.as_mut().expect("state exists while reserving windows").windows.admit(self.model.fenestrations.len()).is_err() {
                { return self.begin_fault(); }
            }
            self.initialize_backing_stage = 5;
            return EnergyRun::Yield;
        }
        if self.initialize_backing_stage == 5 {
            if let Some(fenestration) = self.model.fenestrations.get(self.initialize_cursor) {
                if self.pre.as_ref().is_some_and(|pre| pre.windows.contains_key(&fenestration.id)) {
                    if self.state.as_mut().expect("state exists while initializing").windows.insert(fenestration.id, WindowState { face_temperatures_c: [INITIAL_TEMPERATURE_C; 2 * model::MAX_PANES], inside_convection_w_m2k: 0.0 }).is_err() {
                        { return self.begin_fault(); }
                    }
                    if self.state.as_mut().expect("state exists while initializing").per_surface.insert_window(fenestration.id).is_err() {
                        return self.begin_fault();
                    }
                }
                self.initialize_cursor += 1;
                return EnergyRun::Yield;
            }
            self.initialize_cursor = 0;
            self.initialize_backing_stage = 6;
            return EnergyRun::Yield;
        }
        let (faces, nodes) = self.pre.as_ref().map_or((0, 2), |pre| (pre.maximum_enclosure_faces, pre.maximum_nodes));
        if self.state.as_mut().expect("state exists while reserving the solver").solver.reserve(faces, nodes).is_err() {
            { return self.begin_fault(); }
        }
        self.set_stage(context, EnergyJobStage::InitializeWarmupHistory);
        self.begin_preview(context)
    }

    /// ⚙️ Initializes the warmup history of every zone.
    fn run_initialize_warmup_history(&mut self, context: &mut StepContext<'_>) -> EnergyRun {
        let zones = self.pre.as_ref().map_or(0, |pre| pre.zone_order.len());
        if self.previous_temperatures.capacity() < zones {
            if self.previous_temperatures.try_reserve_exact(zones).is_err() {
                { return self.begin_fault(); }
            }
            return EnergyRun::Yield;
        }
        if self.previous_temperatures.len() < zones {
            self.previous_temperatures.push(f64::NAN);
            return EnergyRun::Yield;
        }
        if self.previous_loads.capacity() < zones {
            if self.previous_loads.try_reserve_exact(zones).is_err() {
                { return self.begin_fault(); }
            }
            return EnergyRun::Yield;
        }
        if self.previous_loads.len() < zones {
            self.previous_loads.push(f64::NAN);
            return EnergyRun::Yield;
        }
        self.set_stage(context, EnergyJobStage::WarmupTimestep);
        EnergyRun::Yield
    }

    /// 🧲️ Runs one warmup timestep.
    fn run_warmup_timestep(&mut self, context: &mut StepContext<'_>) -> EnergyRun {
        let warmup_hours = self.config.warmup_days.saturating_mul(24);
        if self.warmup_hour >= warmup_hours {
            if let Some(state) = &mut self.state {
                state.warmup_complete = true;
            }
            self.set_stage(context, EnergyJobStage::StartRun);
            return EnergyRun::Yield;
        }
        let pre = self.pre.as_ref().expect("precompute complete before warmup");
        if self.timestep_work.is_none() && self.timestep_builder.is_none() {
            let Some(start)=self.run_start_record()else{return self.begin_fault();};
            let Some(weather)=self.hour_weather(start+self.warmup_hour as usize%24,true)else{return self.begin_fault();};
            let date = calendar::SimDate::new(weather.current.year, weather.current.month, weather.current.day);
            self.timestep_builder = Some(TimestepBuilder::new(weather, date, self.warmup_hour as f64));
            return EnergyRun::Yield;
        }
        if self.timestep_work.is_none() {
            match self.timestep_builder.as_mut().expect("warmup timestep builder").step(&self.model, pre) {
                Ok(Some(work)) => {
                    self.timestep_work = Some(work);
                    self.timestep_builder = None;
                }
                Ok(None) => return EnergyRun::Yield,
                Err(_) => return self.begin_fault(),
            }
        }
        let turn=self.timestep_work.as_mut().expect("warmup timestep work exists").step_funded(&self.model,&self.config,pre,self.state.as_mut().expect("state initialized before warmup"),context.retained_grant());
        if context.consume_retained(turn.progress).is_err()||turn.fault.is_some(){return self.begin_fault();}
        if !self.timestep_work.as_ref().is_some_and(TimestepWork::is_complete) {
            return EnergyRun::Yield;
        }
        self.rng_state = self.rng_state.rotate_left(17).wrapping_mul(0x94d0_49bb_1331_11eb);
        self.timestep_work = None;
        self.warmup_convergence =
            Some(WarmupConvergenceWork { stage: WarmupConvergenceStage::TemperatureCheck, cursor: 0, temperature_converged: true, load_converged: true, evaluate: self.warmup_hour > 24 && self.warmup_hour.is_multiple_of(24) });
        self.set_stage(context, EnergyJobStage::WarmupConvergence);
        EnergyRun::Yield
    }

    /// 🪛️ Checks the warmup convergence and starts the next warmup day or the run.
    fn run_warmup_convergence(&mut self, context: &mut StepContext<'_>) -> EnergyRun {
        let pre = self.pre.as_ref().expect("precompute complete during warmup convergence");
        let state = self.state.as_ref().expect("state exists during warmup convergence");
        let work = self.warmup_convergence.as_mut().expect("warmup convergence cursor exists");
        match work.stage {
            WarmupConvergenceStage::TemperatureCheck => {
                if !work.evaluate {
                    work.stage = WarmupConvergenceStage::LoadCheck;
                    work.cursor = 0;
                } else if let Some(zone_id) = pre.zone_order.get(work.cursor) {
                    let current = state.zones.get(zone_id).map_or(f64::NAN, |zone| zone.air.temp_c);
                    work.temperature_converged &= self.previous_temperatures.get(work.cursor).is_some_and(|previous| (current - previous).abs() <= self.config.tolerances.temperature_k);
                    work.cursor += 1;
                } else {
                    work.stage = WarmupConvergenceStage::LoadCheck;
                    work.cursor = 0;
                }
            }
            WarmupConvergenceStage::LoadCheck => {
                if !work.evaluate {
                    work.stage = WarmupConvergenceStage::TemperatureHistory;
                    work.cursor = 0;
                } else if let Some(zone_id) = pre.zone_order.get(work.cursor) {
                    let current = state.zones.get(zone_id).map_or(f64::NAN, |zone| zone.heating_demand_w + zone.cooling_demand_w);
                    work.load_converged &= self.previous_loads.get(work.cursor).is_some_and(|previous| (current - previous).abs() <= self.config.tolerances.energy_w);
                    work.cursor += 1;
                } else {
                    work.stage = WarmupConvergenceStage::TemperatureHistory;
                    work.cursor = 0;
                }
            }
            WarmupConvergenceStage::TemperatureHistory => {
                if let Some(zone_id) = pre.zone_order.get(work.cursor) {
                    self.previous_temperatures[work.cursor] = state.zones.get(zone_id).map_or(f64::NAN, |zone| zone.air.temp_c);
                    work.cursor += 1;
                } else {
                    work.stage = WarmupConvergenceStage::LoadHistory;
                    work.cursor = 0;
                }
            }
            WarmupConvergenceStage::LoadHistory => {
                if let Some(zone_id) = pre.zone_order.get(work.cursor) {
                    self.previous_loads[work.cursor] = state.zones.get(zone_id).map_or(f64::NAN, |zone| zone.heating_demand_w + zone.cooling_demand_w);
                    work.cursor += 1;
                } else {
                    work.stage = WarmupConvergenceStage::Complete;
                }
            }
            WarmupConvergenceStage::Complete => {
                let converged = work.evaluate && work.temperature_converged && work.load_converged;
                self.warmup_convergence = None;
                if converged {
                    self.warmup_hour = self.config.warmup_days.saturating_mul(24);
                } else {
                    self.warmup_hour = self.warmup_hour.saturating_add(1);
                }
                self.set_stage(context, EnergyJobStage::WarmupTimestep);
                if self.warmup_hour.is_multiple_of(24) {
                    return self.begin_preview(context);
                }
            }
        }
        EnergyRun::Yield
    }

    /// 🔧️ Starts the run period after the warmup.
    fn run_start_run(&mut self, context: &mut StepContext<'_>) -> EnergyRun {
        let series = self.numerical_census.series;
        let meters = self.numerical_census.meters;
        match self.run_backing_stage {
            0 => {
                let period = SimulationKernel::run_period(&self.config);
                self.total_timesteps = period.total_hours();
                self.run_hours = Some(period.hours());
                self.tier = match self.config.environment {
                    SimulationEnvironment::HeatingDesignDay | SimulationEnvironment::CoolingDesignDay | SimulationEnvironment::CustomDesignPeriod => EnergyQualityTier::DesignDay,
                    SimulationEnvironment::WeatherRunPeriod => EnergyQualityTier::CoarseTimestep,
                };
            }
            1 => {
                if self.time_series.series.admit(series).is_err() {
                    { return self.begin_fault(); }
                }
                self.result_backing.series_slots = self.time_series.series.capacity();
            }
            2 => {
                if self.meters.meters.admit(meters).is_err() {
                    { return self.begin_fault(); }
                }
                self.result_backing.meter_slots = self.meters.meters.capacity();
            }
            3 => {
                if self.time_series_order.try_reserve_exact(series).is_err() {
                    { return self.begin_fault(); }
                }
            }
            4 => {
                if self.meter_order.try_reserve_exact(meters).is_err() {
                    { return self.begin_fault(); }
                }
            }
            5 => {
                if self.zone_temperature_history.try_reserve_exact(self.numerical_census.history_values).is_err() {
                    { return self.begin_fault(); }
                }
                self.result_backing.history_slots = self.zone_temperature_history.capacity();
            }
            6 => {
                if self.final_summaries.annual_energy.try_reserve_exact(self.numerical_census.summary_rows).is_err() {
                    { return self.begin_fault(); }
                }
                self.result_backing.summary_slots = self.final_summaries.annual_energy.capacity();
            }
            _ => {
                self.set_stage(context, EnergyJobStage::RunZoneTimestep);
                return EnergyRun::Yield;
            }
        }
        self.run_backing_stage += 1;
        EnergyRun::Yield
    }

    /// 🔨️ Runs one zone timestep of the run period.
    fn run_zone_timestep(&mut self, context: &mut StepContext<'_>) -> EnergyRun {
        let pre = self.pre.as_ref().expect("precompute complete before run");
        if self.timestep_work.is_none() && self.timestep_builder.is_none() {
            let Some((date, hour, _)) = self.run_hours.as_mut().and_then(|hours| hours.next()) else {
                self.set_stage(context, EnergyJobStage::Finalize);
                return EnergyRun::Yield;
            };
            let Some(record)=self.weather_index(date,hour)else{self.weather_fault=Some(WeatherFault::RunHourMissing);return self.begin_fault();};
            let Some(weather)=self.hour_weather(record,false)else{return self.begin_fault();};
            self.timestep_builder = Some(TimestepBuilder::new(weather, date, self.hour_index as f64));
            return EnergyRun::Yield;
        }
        if self.timestep_work.is_none() {
            match self.timestep_builder.as_mut().expect("run timestep builder").step(&self.model, pre) {
                Ok(Some(work)) => {
                    self.timestep_work = Some(work);
                    self.timestep_builder = None;
                }
                Ok(None) => return EnergyRun::Yield,
                Err(_) => return self.begin_fault(),
            }
        }
        let turn=self.timestep_work.as_mut().expect("run timestep work exists").step_funded(&self.model,&self.config,pre,self.state.as_mut().expect("state initialized before run"),context.retained_grant());
        if context.consume_retained(turn.progress).is_err()||turn.fault.is_some(){return self.begin_fault();}
        if !self.timestep_work.as_ref().is_some_and(TimestepWork::is_complete) {
            return EnergyRun::Yield;
        }
        self.rng_state = self.rng_state.rotate_left(17).wrapping_mul(0x94d0_49bb_1331_11eb);
        self.timestep_work = None;
        self.aggregate_zone_cursor = 0;
        self.set_stage(context, EnergyJobStage::AggregateZone);
        EnergyRun::Yield
    }

    /// 🪚️ Encodes the retained results into the output pack.
    fn run_encode_output(&mut self, context: &mut StepContext<'_>) -> EnergyRun {
        if self.commit_reservation.is_none() {
            if self.commit_census.is_none() {
                self.commit_census = Some(CommitCensusWork::default());
                return EnergyRun::Yield;
            }
            match self.step_commit_census() {
                Ok(false) => return EnergyRun::Yield,
                Ok(true) => {}
                Err(fault) => {
                    self.output_fault = Some(fault);
                    return self.begin_fault();
                }
            }
        }
        if self.output_writer.is_none() && self.output_payload.is_none() {
            self.output_writer = Some(semio_framework_job_payload::RetainedJobPayloadWriter::new(JobPayloadStream::CommitOutput));
            return EnergyRun::Yield;
        }
        match self.encode_output_step(context) {
            Ok(false) => return EnergyRun::Yield,
            Ok(true) => {}
            Err(fault) => {
                self.output_fault = Some(fault);
                return self.begin_fault();
            }
        }
        if let Some(writer) = self.output_writer.as_mut() {
            if writer.staged_page_len().is_some() {
                if writer.commit_staged_page().is_err() {
                    self.output_fault = Some(OutputFault::BackingRejected);
                    return self.begin_fault();
                }
                return EnergyRun::Yield;
            }
            let writer = self.output_writer.take().expect("output writer exists");
            match writer.finish() {
                Ok(payload) => {
                    let reservation = self.commit_reservation.as_ref().expect("commit reservation remains mounted");
                    if payload.page_count() != reservation.pages || payload.len() != reservation.bytes || self.commit_pages_mounted != reservation.pages || self.commit_items_encoded != reservation.items {
                        self.output_payload = Some(payload);
                        self.output_fault = Some(OutputFault::BackingRejected);
                        return self.begin_fault();
                    }
                    self.output_payload = Some(payload);
                }
                Err(writer) => {
                    self.output_writer = Some(writer);
                    return EnergyRun::Yield;
                }
            }
            let payload = self.output_payload.take().expect("prepared commit payload exists");
            let packet = EnergyWirePacket { kind: EnergyWireKind::Commit, identity: self.wire_identity(0), payload, preview: None, reservation: self.commit_reservation.take() };
            match self.publication.commits.push_reserved(packet) {
                Ok(()) => {
                    self.commit_pages_mounted = 0;
                    self.commit_items_encoded = 0;
                    self.set_stage(context, EnergyJobStage::Complete);
                }
                Err(packet) => {
                    self.output_payload = Some(packet.payload);
                    self.commit_reservation = packet.reservation;
                    return EnergyRun::Yield;
                }
            }
        }
        EnergyRun::Yield
    }

}
