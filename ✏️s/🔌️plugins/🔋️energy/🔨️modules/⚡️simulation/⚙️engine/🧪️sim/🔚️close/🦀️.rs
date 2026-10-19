//! 🔚️ Job close ladder: close demands and steps, the interactive job trait implementation and the terminal-empty census.

use super::*;

impl EnergyJobAuthority {
    pub(super) fn begin_close(&mut self) {
        self.closing = true;
    }

    /// 🏷️ Resolves the outcome markers or the leased commit packet into the immutable descriptor's payload pair.
    fn commit_payloads(&self) -> (&RetainedJobPayload, &RetainedJobPayload) {
        (&self.markers.commit_state, self.commit_output.as_ref().map_or(&self.markers.commit_output, |packet| &packet.payload))
    }

    /// 📏️ Quotes the next close frontier: a payload owner, a staged page source, or one popped ladder rung.
    fn precompute_close_demands(&self, body: usize) -> Option<Result<RetirementDemand, ValueError>> {
        if !self.precompute_close.is_empty() || self.precompute.is_some() {
            return Some(self.precompute_close.next_owner_demands::<PrecomputeBuilder>(self.precompute.is_some(), &None, body));
        }
        self.pre.as_ref().map(|_| self.precompute_close.next_owner_demands::<PrecomputedModel>(true, &None, body))
    }

    pub fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
        if authority_inputs_are_retired(self){return self.retained_tail_demands(usize::MAX)}
        if authority_is_terminal_empty(self) {
            return Ok(RetirementDemand::default());
        }
        if !self.input_identity.terminal_is_empty() { return Ok(self.input_identity.close_demand()); }
        if let Some(build) = self.wire_build.as_ref() {
            return build.writer.retirement_demands();
        }
        if let Some(writer) = self.output_writer.as_ref() {
            return writer.retirement_demands();
        }
        if let Some(payload) = self.output_payload.as_ref() {
            return payload.retirement_demands();
        }
        for packet in [self.wire_ready.as_ref(), self.restore_input.as_ref(), self.publication.preview.as_ref(), self.publication.retiring_preview.as_ref(), self.commit_output.as_ref()].into_iter().flatten() {
            return packet.retirement_demands();
        }
        for queue in [&self.publication.checkpoints, &self.publication.commits, &self.publication.faults] {
            if let Some(demand) = queue.retirement_demands() {
                return demand;
            }
        }
        for reservation in [self.commit_census.as_ref().and_then(|work| work.reservation.as_ref()), self.commit_reservation.as_ref()].into_iter().flatten() {
            if let Some(demand) = reservation.close_source_demand() {
                return Ok(demand);
            }
        }
        if let Some(demand) = self.precompute_close_demands(0) { return demand; }
        Ok(ENERGY_RUNG_DEMAND)
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError> {
        self.closing = true;
        if authority_inputs_are_retired(self){return self.retire_retained_tail(grant)}
        if authority_is_terminal_empty(self) {
            return Ok(energy_done());
        }
        if !self.input_identity.terminal_is_empty() {
            return self.input_identity.close_step(grant).map(|progress|InteractiveJobCloseStep::Pending{progress});
        }
        if let Some(build) = self.wire_build.as_mut() {
            build.writer.begin_close();
            let (step, done) = energy_turn(energy_guarded(grant, build.writer.retirement_demands(), || build.writer.close_step(grant)))?;
            if done {
                self.wire_build = None;
            }
            return Ok(step);
        }
        if let Some(writer) = self.output_writer.as_mut() {
            writer.begin_close();
            let (step, done) = energy_turn(energy_guarded(grant, writer.retirement_demands(), || writer.close_step(grant)))?;
            if done {
                self.output_writer = None;
            }
            return Ok(step);
        }
        if let Some(payload) = self.output_payload.as_mut() {
            let (step, done) = energy_turn(energy_guarded(grant, payload.retirement_demands(), || payload.close_step(grant)))?;
            if done {
                self.output_payload = None;
            }
            return Ok(step);
        }
        macro_rules! close_packet {
            ($packet:expr) => {
                if let Some(packet) = $packet.as_mut() {
                    let (step, done) = energy_turn(packet.close_step(grant))?;
                    if done {
                        $packet = None;
                    }
                    return Ok(step);
                }
            };
        }
        close_packet!(self.wire_ready);
        close_packet!(self.restore_input);
        close_packet!(self.publication.preview);
        close_packet!(self.publication.retiring_preview);
        close_packet!(self.commit_output);
        for queue in [&mut self.publication.checkpoints, &mut self.publication.commits, &mut self.publication.faults] {
            if let Some(step) = queue.close_step(grant)? {
                return Ok(step);
            }
        }
        macro_rules! close_sources {
            ($reservation:expr) => {
                if let Some(demand) = $reservation.as_ref().and_then(EnergyCommitReservation::close_source_demand) {
                    if !energy_grant_covers(grant, demand) {
                        return Ok(energy_idle());
                    }
                    let released_bytes = $reservation.as_mut().and_then(EnergyCommitReservation::close_source_step).unwrap_or_default();
                    return Ok(energy_rung((1, released_bytes)));
                }
            };
        }
        if let Some(work) = self.commit_census.as_mut() {
            close_sources!(work.reservation);
            self.commit_census = None;
            return Ok(energy_one());
        }
        if self.commit_reservation.is_some() {
            close_sources!(self.commit_reservation);
            self.commit_reservation = None;
            return Ok(energy_one());
        }
        if !energy_grant_covers(grant, ENERGY_RUNG_DEMAND) {
            return Ok(energy_idle());
        }
        let maximum_bytes = grant.maximum_release_bytes;
        let maximum_items = grant.maximum_items;
        if std::mem::take(&mut self.commit_pages_mounted) != 0 || std::mem::take(&mut self.commit_items_encoded) != 0 {
            return Ok(energy_one());
        }
        if let Some((released_items, released_bytes)) = close_string_step(&mut self.close_string_owner, maximum_bytes) {
            return Ok(energy_rung((released_items, released_bytes)));
        }
        macro_rules! pop_owner {
            ($owners:expr) => {
                if $owners.pop().is_some() {
                    return Ok(energy_one());
                }
            };
        }
        pop_owner!(self.weather);
        if let Some((released_items, released_bytes)) = close_string_vector_step(&mut self.time_series_order, maximum_bytes) {
            return Ok(energy_rung((released_items, released_bytes)));
        }
        if let Some((released_items, released_bytes)) = close_string_vector_step(&mut self.meter_order, maximum_bytes) {
            return Ok(energy_rung((released_items, released_bytes)));
        }
        if let Some((released_items, released_bytes)) = close_time_series_step(&mut self.time_series, &mut self.close_string_owner, maximum_bytes) {
            return Ok(energy_rung((released_items, released_bytes)));
        }
        if let Some((released_items, released_bytes)) = close_meter_table_step(&mut self.meters, &mut self.close_string_owner, maximum_bytes) {
            return Ok(energy_rung((released_items, released_bytes)));
        }
        pop_owner!(self.zone_temperature_history);
        pop_owner!(self.previous_temperatures);
        pop_owner!(self.previous_loads);
        if let Some(work) = self.aggregate_zone_work.as_mut() {
            if let Some((released_items, released_bytes)) = close_aggregate_work_step(work, maximum_bytes) {
                return Ok(energy_rung((released_items, released_bytes)));
            }
            self.aggregate_zone_work = None;
            return Ok(energy_one());
        }
        if let Some(work) = self.aggregate_facility_work.as_mut() {
            if let Some((released_items, released_bytes)) = close_aggregate_work_step(work, maximum_bytes) {
                return Ok(energy_rung((released_items, released_bytes)));
            }
            self.aggregate_facility_work = None;
            return Ok(energy_one());
        }
        macro_rules! remove_validation_entry {
            ($owners:expr) => {{
                if $owners.pop().is_some() {
                    return Ok(energy_one());
                }
            }};
        }
        remove_validation_entry!(self.validation.zone_ids);
        remove_validation_entry!(self.validation.material_ids);
        remove_validation_entry!(self.validation.construction_ids);
        remove_validation_entry!(self.validation.surface_ids);
        if let Some((released_items, released_bytes)) = close_string_step(&mut self.finalization.row_unit, maximum_bytes) {
            return Ok(energy_rung((released_items, released_bytes)));
        }
        if let Some((released_items, released_bytes)) = close_string_step(&mut self.finalization.row_key, maximum_bytes) {
            return Ok(energy_rung((released_items, released_bytes)));
        }
        if !self.result_build.model_name.is_empty() || !self.result_build.model_version.is_empty() || !self.result_build.weather_location.is_empty() {
            if maximum_bytes == 0 {
                return Ok(energy_idle());
            }
            let released_bytes = self.result_build.weather_location.pop().or_else(|| self.result_build.model_version.pop()).or_else(|| self.result_build.model_name.pop()).map_or(0, |character| character.len_utf8());
            return Ok(energy_rung((0, released_bytes)));
        }
        if let Some(builder) = self.timestep_builder.as_mut() {
            if !builder.close_step(maximum_items) {
                return Ok(energy_one());
            }
            self.timestep_builder = None;
            return Ok(energy_one());
        }
        if let Some(work) = self.timestep_work.as_mut() {
            if !work.close_step(maximum_items) {
                return Ok(energy_one());
            }
            self.timestep_work = None;
            return Ok(energy_one());
        }

        if let Some(state) = self.state.as_mut() {
            if state.zones.pop().is_some() {
                return Ok(energy_one());
            }
            if state.surfaces.pop().is_some() {
                return Ok(energy_one());
            }
            if state.windows.pop().is_some() {
                return Ok(energy_one());
            }
            if state.per_surface.pop() {
                return Ok(energy_one());
            }
            self.state = None;
            return Ok(energy_one());
        }
        if let Some(results) = self.result.as_mut() {
            if let Some((released_items, released_bytes)) = close_results_step(results, &mut self.close_string_owner, maximum_bytes) {
                return Ok(energy_rung((released_items, released_bytes)));
            }
            self.result = None;
            return Ok(energy_one());
        }
        if let Some(sizing) = self.final_sizing.as_mut() {
            if let Some((released_items, released_bytes)) = close_sizing_step(sizing, maximum_bytes) {
                return Ok(energy_rung((released_items, released_bytes)));
            }
            self.final_sizing = None;
            return Ok(energy_one());
        }
        if let Some((released_items, released_bytes)) = close_summaries_step(&mut self.final_summaries, maximum_bytes) {
            return Ok(energy_rung((released_items, released_bytes)));
        }
        if let Some(builder) = self.sizing_builder.as_mut() {
            let (complete, released_items, released_bytes) = builder.close_step(maximum_bytes);
            if !complete {
                return Ok(energy_rung((released_items, released_bytes)));
            }
            self.sizing_builder = None;
            return Ok(energy_one());
        }
        if let Some((released_items, released_bytes)) = close_model_step(&mut self.model, maximum_bytes) {
            return Ok(energy_rung((released_items, released_bytes)));
        }
        if let Some((released_items, released_bytes)) = close_config_step(&mut self.config, maximum_bytes) {
            return Ok(energy_rung((released_items, released_bytes)));
        }
        if self.run_hours.take().is_some() || self.final_environmental.take().is_some() || self.final_resilience.take().is_some() || self.output_fault.take().is_some() || self.weather_fault.take().is_some() {
            return Ok(energy_one());
        }
        Ok(energy_done())
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        authority_is_terminal_empty(self)
    }
}


impl InteractiveJob for EnergyJob {
    fn step<'a>(&'a mut self, context: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, OriginalTurnFailure> {
        let authority = &mut **self;
        let run = match authority.pending_outcome.take() {
            Some(run) => run,
            None => authority.run(context),
        };
        let admitted = match run {
            EnergyRun::Yield => return Ok(None),
            EnergyRun::Cancelled => JobOutcomeBorrow::admit_cancelled(context)?,
            EnergyRun::Preview => JobOutcomeBorrow::admit_preview(context, &authority.markers.preview)?,
            EnergyRun::Checkpoint(applied_progress) => JobOutcomeBorrow::admit_checkpoint(context, &authority.markers.checkpoint, applied_progress)?,
            EnergyRun::Fault => JobOutcomeBorrow::admit_fault(context, &authority.markers.fault)?,
            EnergyRun::Complete => {
                let output = authority.commit_output.as_ref().map_or(&authority.markers.commit_output, |packet| &packet.payload);
                JobOutcomeBorrow::admit_complete(context, Some(&authority.markers.commit_state), Some(output))?
            }
        };
        if admitted.is_none() {
            authority.pending_outcome = Some(run);
        }
        Ok(admitted)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::PreviewReady => descriptor.preview(&self.markers.preview),
            JobOutcomeKind::CheckpointReady { .. } => descriptor.checkpoint(&self.markers.checkpoint),
            JobOutcomeKind::Complete => {
                let (state, output) = self.commit_payloads();
                descriptor.complete(Some(state), Some(output))
            }
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Fault => descriptor.fault(&self.markers.fault),
        }
    }

    fn begin_close(&mut self) {
        if let Some(authority)=self.authority.as_deref_mut(){authority.begin_close();}
    }

    fn close_step(&mut self, context: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        self.begin_close();
        if self.authority.is_none(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
        let demand=self.original_close_demands()?;
        let step=semio_framework_job::close_original_retained_step(context,demand,|grant|{
            if self.authority.as_deref().is_some_and(authority_is_terminal_empty){
                drop(self.authority.take());
                release_energy_abandonment_slot(self.abandonment_slot);
                return Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}));
            }
            let step=self.authority.as_deref_mut().expect("original Energy authority remains retained").close_step(grant)?;
            Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(step.progress()))
        })?;
        Ok(match step{InteractiveJobCloseStep::Pending{progress}if self.authority.is_none()=>InteractiveJobCloseStep::Complete{progress},step=>step})
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.original_close_demands()?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, ValueError> {
        Ok(self.original_close_demands()?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.original_close_demands()?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(self.original_close_demands()?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.authority.is_none()
    }
}
impl EnergyJob{
    fn original_close_demands(&self)->Result<RetirementDemand,ValueError>{
        let Some(authority)=self.authority.as_deref()else{return Ok(Default::default())};
        if authority.closing&&authority_is_terminal_empty(authority){return Ok(RetirementDemand{release_bytes:std::mem::size_of::<EnergyJobAuthority>(),depth:1,..Default::default()})}
        authority.close_demands()
    }
}

pub(super) fn advance_validation(work: &mut ValidationWork, stage: ValidationStage) {
    work.stage = stage;
    work.cursor = 0;
    work.child_cursor = 0;
}

pub(super) fn advance_finalization(work: &mut FinalizationWork, stage: FinalizationStage) {
    work.stage = stage;
    work.cursor = 0;
    work.row_cursor = 0;
    work.row_part = 0;
    work.row_reserved = false;
}

pub(super) fn build_summary_row_step(work: &mut FinalizationWork, summaries: &mut SummaryTables, maximum_slots: usize, key: &str, value: f64, unit: &str) -> bool {
    let (owner, source) = if work.row_part == 0 { (&mut work.row_key, key) } else { (&mut work.row_unit, unit) };
    if !work.row_reserved {
        if owner.try_reserve_exact(source.len()).is_err() {
            work.row_backing_rejected = true;
            return false;
        }
        work.row_reserved = true;
        return false;
    }
    if let Some(character) = source.get(work.row_cursor..).and_then(|remaining| remaining.chars().next()) {
        owner.push(character);
        work.row_cursor += character.len_utf8();
        return false;
    }
    if work.row_part == 0 {
        work.row_part = 1;
        work.row_cursor = 0;
        work.row_reserved = false;
        return false;
    }
    if summaries.annual_energy.len() >= maximum_slots {
        work.row_backing_rejected = true;
        return false;
    }
    summaries.annual_energy.push(SummaryRow { key: std::mem::take(&mut work.row_key), value, unit: std::mem::take(&mut work.row_unit) });
    true
}

pub(super) fn advance_result_build(work: &mut ResultBuildWork, stage: ResultBuildStage) {
    work.stage = stage;
    work.cursor = 0;
    work.reserved = false;
}

pub(super) fn copy_string_step(output: &mut String, cursor: &mut usize, reserved: &mut bool, source: &str) -> Result<bool, Error> {
    if !*reserved {
        output.try_reserve_exact(source.len()).map_err(|_| Error::severe("energy metadata backing rejected"))?;
        *reserved = true;
        return Ok(false);
    }
    let Some(character) = source.get(*cursor..).and_then(|remaining| remaining.chars().next()) else {
        return Ok(true);
    };
    output.push(character);
    *cursor += character.len_utf8();
    Ok(false)
}

pub(super) fn copy_aggregate_names_step(work: &mut AggregateZoneWork, parts: [&str; 3]) -> bool {
    if work.copy_index >= work.names.len() {
        return true;
    }
    let length = parts.iter().map(|part| part.len()).sum();
    if !work.reserved[work.copy_index] {
        if work.names[work.copy_index].try_reserve_exact(length).is_err() {
            work.backing_rejected = true;
            return false;
        }
        work.reserved[work.copy_index] = true;
        return false;
    }
    let mut offset = work.cursor;
    for part in parts {
        if offset < part.len() {
            if let Some(character) = part.get(offset..).and_then(|remaining| remaining.chars().next()) {
                work.names[work.copy_index].push(character);
                work.cursor += character.len_utf8();
                return false;
            }
        }
        offset = offset.saturating_sub(part.len());
    }
    work.copy_index += 1;
    work.cursor = 0;
    work.copy_index >= work.names.len()
}

pub(super) fn insert_admitted_meter(table: &mut MeterTable, order: &mut Vec<String>, work: &mut AggregateZoneWork, maximum_slots: usize, fuel: FuelType, end_use: EndUse) {
    if table.meters.len() >= maximum_slots {
        work.backing_rejected = true;
        return;
    }
    let [map_key, meter_name, order_key] = std::mem::replace(&mut work.names, std::array::from_fn(|_| String::new()));
    if table.meters.insert_stable(map_key, Meter { name: meter_name, fuel, end_use, energy_j: 0.0, peak_demand_w: 0.0, peak_demand_hour: 0.0 }).is_err() {
        work.backing_rejected = true;
        return;
    }
    order.push(order_key);
}

fn close_aggregate_work_step(work: &mut AggregateZoneWork, maximum_bytes: usize) -> Option<(usize, usize)> {
    if work.timestamps.pop().is_some() || work.values.pop().is_some() {
        return Some((1, 0));
    }
    for name in &mut work.names {
        if let Some(released) = close_string_step(name, maximum_bytes) {
            return Some(released);
        }
    }
    None
}

pub(super) fn authority_is_terminal_empty(authority:&EnergyJobAuthority)->bool{authority_inputs_are_retired(authority)&&authority.tail_moved&&authority.closing_tail.is_none()}

fn authority_inputs_are_retired(authority: &EnergyJobAuthority) -> bool {
    authority.closing
        && authority.input_identity.terminal_is_empty()
        && authority.output_writer.is_none()
        && authority.output_payload.is_none()
        && authority.commit_census.is_none()
        && authority.commit_reservation.is_none()
        && authority.commit_pages_mounted == 0
        && authority.commit_items_encoded == 0
        && authority.wire_build.is_none()
        && authority.wire_ready.is_none()
        && authority.restore_input.is_none()
        && authority.commit_output.is_none()
        && authority.publication.preview.is_none()
        && authority.publication.retiring_preview.is_none()
        && authority.publication.checkpoints.len == 0
        && !authority.publication.checkpoints.reserved_push
        && authority.publication.commits.len == 0
        && !authority.publication.commits.reserved_push
        && authority.publication.faults.len == 0
        && !authority.publication.faults.reserved_push
        && authority.close_string_owner.is_empty()
        && authority.weather.is_empty()
        && authority.time_series_order.is_empty()
        && authority.meter_order.is_empty()
        && authority.time_series.series.is_empty()
        && authority.meters.meters.is_empty()
        && authority.zone_temperature_history.is_empty()
        && authority.previous_temperatures.is_empty()
        && authority.previous_loads.is_empty()
        && authority.aggregate_zone_work.is_none()
        && authority.aggregate_facility_work.is_none()
        && authority.validation.zone_ids.is_empty()
        && authority.validation.material_ids.is_empty()
        && authority.validation.construction_ids.is_empty()
        && authority.validation.surface_ids.is_empty()
        && authority.finalization.row_key.is_empty()
        && authority.finalization.row_unit.is_empty()
        && authority.result_build.model_name.is_empty()
        && authority.result_build.model_version.is_empty()
        && authority.result_build.weather_location.is_empty()
        && authority.precompute.is_none()
        && authority.precompute_close.is_empty()
        && authority.pre.is_none()
        && authority.state.is_none()
        && authority.timestep_builder.is_none()
        && authority.run_hours.is_none()
        && authority.timestep_work.is_none()
        && authority.result.is_none()
        && authority.sizing_builder.is_none()
        && authority.final_sizing.is_none()
        && summaries_are_terminal_empty(&authority.final_summaries)
        && authority.final_environmental.is_none()
        && authority.final_resilience.is_none()
        && authority.output_fault.is_none()
        && authority.weather_fault.is_none()
        && model_is_terminal_empty(&authority.model)
        && config_is_terminal_empty(&authority.config)
}
