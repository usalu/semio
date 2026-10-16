//! 📮️ Job authority wire surface: packet leases for preview, checkpoint, commit and fault, result hand-off, weather and wire building, and the numerical digest.

use super::*;

impl EnergyJobAuthority {
    pub fn stage(&self) -> EnergyJobStage {
        self.stage
    }

    pub fn cursor(&self) -> EnergyJobCursor {
        EnergyJobCursor {
            stage: self.stage,
            tier: self.tier,
            warmup_hour: self.warmup_hour,
            warmup_hours: self.config.warmup_days.saturating_mul(24),
            timestep: self.hour_index,
            total_timesteps: self.total_timesteps,
            facility_electricity_kwh: self.facility_electricity_j / 3_600_000.0,
        }
    }

    pub fn preview(&self) -> Option<&EnergyJobPreview> {
        self.publication.preview.as_ref().and_then(EnergyWirePacket::preview)
    }

    pub fn take_preview_packet(&mut self, generation: Generation) -> Result<Option<EnergyWirePacket>, EnergyWireRejection> {
        if generation != self.operation.generation {
            return Err(EnergyWireRejection::Identity);
        }
        Ok(self.publication.preview.take())
    }

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn retry_preview_packet(&mut self, packet: EnergyWirePacket) -> Result<(), EnergyWirePacket> {
        if !self.packet_matches(&packet, EnergyWireKind::Preview) || self.publication.preview.is_some() || self.publication.retiring_preview.is_some() {
            return Err(packet);
        }
        self.publication.preview = Some(packet);
        Ok(())
    }

    pub fn take_checkpoint_packet(&mut self, generation: Generation) -> Result<Option<EnergyWireLease>, EnergyWireRejection> {
        if generation != self.operation.generation {
            return Err(EnergyWireRejection::Identity);
        }
        Ok(self.publication.checkpoints.take())
    }

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn retry_checkpoint_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
        self.publication.checkpoints.retry(lease)
    }

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn ack_checkpoint_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
        self.publication.checkpoints.ack(lease)
    }

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn ack_checkpoint_for_restore(&mut self, lease: EnergyWireLease) -> Result<EnergyWirePacket, EnergyWireLease> {
        self.publication.checkpoints.ack_transfer(lease)
    }

    pub fn take_commit_packet(&mut self, generation: Generation) -> Result<Option<EnergyWireLease>, EnergyWireRejection> {
        if generation != self.operation.generation {
            return Err(EnergyWireRejection::Identity);
        }
        Ok(self.publication.commits.take())
    }

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn retry_commit_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
        self.publication.commits.retry(lease)
    }

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn ack_commit_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
        match self.publication.commits.ack(lease) {
            Ok(()) => {
                self.commit_acknowledged = true;
                Ok(())
            }
            Err(lease) => Err(lease),
        }
    }

    pub fn take_fault_packet(&mut self, generation: Generation) -> Result<Option<EnergyWireLease>, EnergyWireRejection> {
        if generation != self.operation.generation {
            return Err(EnergyWireRejection::Identity);
        }
        Ok(self.publication.faults.take())
    }

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn retry_fault_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
        self.publication.faults.retry(lease)
    }

    #[expect(clippy::result_large_err, reason = "Returns the exact rejected packet, lease, model, or job owner for retry and bounded retirement without allocating on refusal.")]
    pub fn ack_fault_packet(&mut self, lease: EnergyWireLease) -> Result<(), EnergyWireLease> {
        self.publication.faults.ack(lease)
    }

    fn packet_matches(&self, packet: &EnergyWirePacket, kind: EnergyWireKind) -> bool {
        packet.kind == kind
            && packet.identity.operation == self.operation.operation.0
            && packet.identity.base_revision == self.operation.base_revision.0
            && packet.identity.generation == self.operation.generation.0
            && packet.identity.seed == self.operation.seed
    }

    pub fn take_results(&mut self) -> Option<Results> {
        self.result.take()
    }

    /// 🧱️ The live per-surface energy table WHILE the run is running — the only path to per-surface
    /// physics before the job settles, since the framework tool-run driver discards the settled
    /// `Complete` candidate's payload unread. A run job reads this on a tier boundary and encodes it
    /// into its tick payload (`ToolRunView::payload`). `None` before the state is initialized.
    pub fn per_surface_energy(&self) -> Option<&results::SurfaceEnergyTable> {
        self.state.as_ref().map(|state| &state.per_surface)
    }

    pub(super) fn set_stage(&mut self, context: &mut StepContext<'_>, stage: EnergyJobStage) {
        self.stage = stage;
        context.set_stage(stage.label());
    }

    /// 🌦️ Weather-table index of the first hour of the run period.
    pub(super) fn run_start_record(&self) -> usize {
        if self.weather.len() < 8_760 {
            return 0;
        }
        let period = SimulationKernel::run_period(&self.config);
        (calendar::SimDate::new(period.year, period.start_month, period.start_day).day_of_year() as usize - 1) * 24
    }

    /// 🌦️ The hour at `index` with its neighbours the way an hourly weather file is interpolated:
    /// the previous hour (the day's last hour when the run or the warmup day starts), the current
    /// hour, and the next hour of the same day (the day's first hour after its last).
    pub(super) fn hour_weather(&self, index: usize, first_of_period: bool) -> HourWeather {
        let len = self.weather.len().max(1);
        let record = |i: usize| *self.weather.get_index(i % len).expect("admitted weather record");
        let hour = index % 24;
        let day_start = index - hour;
        let previous = if hour == 0 && (first_of_period || index == 0) { record(day_start + 23) } else { record(index + len - 1) };
        let next = if hour == 23 { record(day_start) } else { record(index + 1) };
        HourWeather { previous, current: record(index), next }
    }

    pub(super) fn weather_record(&self, index: usize) -> WeatherRecord {
        if let Some(epw) = &self.config.weather {
            if let Some(record) = epw.records.get(index) {
                return *record;
            }
        }
        match self.config.environment {
            SimulationEnvironment::HeatingDesignDay => site::design_day_hour(index as u32, -10.0),
            SimulationEnvironment::CoolingDesignDay => site::design_day_hour(index as u32, 35.0),
            SimulationEnvironment::WeatherRunPeriod | SimulationEnvironment::CustomDesignPeriod => site::synthetic_hour(index as u32),
        }
    }

    pub(super) fn wire_identity(&self, sequence: u64) -> EnergyWireIdentity {
        EnergyWireIdentity { operation: self.operation.operation.0, base_revision: self.operation.base_revision.0, generation: self.operation.generation.0, seed: self.operation.seed, sequence }
    }

    pub fn start_wire(&mut self, kind: EnergyWireKind, sequence: u64, stream: JobPayloadStream) -> Result<(), EnergyWireRejection> {
        if self.numerical_census.operations != 1 || self.numerical_census.process_jobs != 1 {
            return Err(EnergyWireRejection::Operations);
        }
        if self.numerical_census.pages == 0 || self.numerical_census.observed_items == 0 {
            return Err(EnergyWireRejection::Pages);
        }
        if self.wire_build.is_some() || self.wire_ready.is_some() || self.commit_reservation.is_some() {
            return Err(EnergyWireRejection::Saturated);
        }
        self.wire_build = Some(EnergyWireBuild { kind, identity: self.wire_identity(sequence), writer: semio_framework_job::RetainedJobPayloadWriter::new(stream), field: 0 });
        Ok(())
    }

    fn wire_fragment(&self, kind: EnergyWireKind, field: u8) -> Option<([u8; ENERGY_WIRE_HEADER_BYTES], usize)> {
        let mut bytes = [0; ENERGY_WIRE_HEADER_BYTES];
        let length = match field {
            0 => {
                bytes = energy_wire_header(kind, self.wire_identity(self.wire_build.as_ref()?.identity.sequence), self.numerical_census, self.stage, self.tier);
                ENERGY_WIRE_HEADER_BYTES
            }
            1 => {
                bytes[..4].copy_from_slice(&self.warmup_hour.to_le_bytes());
                bytes[4..8].copy_from_slice(&self.hour_index.to_le_bytes());
                bytes[8..12].copy_from_slice(&self.total_timesteps.to_le_bytes());
                if kind == EnergyWireKind::Preview {
                    bytes[12..20].copy_from_slice(&(self.facility_electricity_j / 3_600_000.0).to_le_bytes());
                    20
                } else {
                    12
                }
            }
            2 if kind == EnergyWireKind::Checkpoint => {
                bytes[..8].copy_from_slice(&self.rng_state.to_le_bytes());
                bytes[8..16].copy_from_slice(&(self.weather_cursor as u64).to_le_bytes());
                bytes[16..24].copy_from_slice(&(self.weather_target as u64).to_le_bytes());
                24
            }
            3 if kind == EnergyWireKind::Checkpoint => {
                bytes[..8].copy_from_slice(&(self.aggregate_zone_cursor as u64).to_le_bytes());
                bytes[8] = self.aggregate_facility_cursor;
                bytes[9] = self.initialize_backing_stage;
                bytes[10] = self.run_backing_stage;
                bytes[11] = u8::from(self.checkpoint_due);
                bytes[16..24].copy_from_slice(&(self.time_series.series.len() as u64).to_le_bytes());
                bytes[24..32].copy_from_slice(&(self.meters.meters.len() as u64).to_le_bytes());
                bytes[32..40].copy_from_slice(&(self.zone_temperature_history.len() as u64).to_le_bytes());
                bytes[40..48].copy_from_slice(&self.numerical_digest().to_le_bytes());
                48
            }
            _ => return None,
        };
        Some((bytes, length))
    }

    pub fn step_wire_build(&mut self, context: &mut StepContext<'_>) -> Result<bool, EnergyWireRejection> {
        let (kind, field) = self.wire_build.as_ref().map(|build| (build.kind, build.field)).ok_or(EnergyWireRejection::Backing)?;
        let Some((fragment, length)) = self.wire_fragment(kind, field) else {
            let build = self.wire_build.as_mut().expect("wire build remains mounted");
            if build.writer.staged_page_len().is_some() {
                build.writer.commit_staged_page().map_err(|_| EnergyWireRejection::Backing)?;
                return Ok(false);
            }
            let build = self.wire_build.take().expect("wire build remains mounted");
            let payload = build.writer.finish().map_err(|_| EnergyWireRejection::Backing)?;
            self.wire_ready = Some(EnergyWirePacket { kind: build.kind, identity: build.identity, payload, preview: None, reservation: None });
            return Ok(true);
        };
        let build = self.wire_build.as_mut().expect("wire build remains mounted");
        if build.writer.staged_page_len().is_none() {
            build.writer.begin_staged_page(context).map_err(|_| EnergyWireRejection::Backing)?;
            return Ok(false);
        }
        if build.writer.staged_page_remaining() < length {
            build.writer.commit_staged_page().map_err(|_| EnergyWireRejection::Backing)?;
            return Ok(false);
        }
        build.writer.write_staged(&fragment[..length]).map_err(|_| EnergyWireRejection::Backing)?;
        build.field = build.field.checked_add(1).ok_or(EnergyWireRejection::Items)?;
        Ok(false)
    }

    pub(super) fn begin_preview(&mut self, context: &mut StepContext<'_>) -> EnergyRun {
        let sequence = match context.next_preview_sequence() {
            Ok(sequence) => sequence,
            Err(_) => return EnergyRun::Fault,
        };
        if self.start_wire(EnergyWireKind::Preview, sequence, JobPayloadStream::Preview).is_err() {
            return EnergyRun::Yield;
        }
        EnergyRun::Yield
    }

    pub(super) fn drive_wire_publication(&mut self, context: &mut StepContext<'_>) -> Option<EnergyRun> {
        if let Some(retiring) = self.publication.retiring_preview.as_mut() {
            let step = retiring.close_step(context.retained_grant());
            match energy_spend(context, step) {
                Ok(true) => self.publication.retiring_preview = None,
                Ok(false) => return Some(EnergyRun::Yield),
                Err(_) => return Some(Self::fault()),
            }
        }
        if self.wire_build.is_some() {
            if self.step_wire_build(context).is_err() {
                return Some(Self::fault());
            }
            return Some(EnergyRun::Yield);
        }
        let mut packet = self.wire_ready.take()?;
        match packet.kind {
            EnergyWireKind::Preview => {
                if self.publication.preview.is_some() {
                    self.publication.retiring_preview = self.publication.preview.take();
                    self.wire_ready = Some(packet);
                    return Some(EnergyRun::Yield);
                }
                let Some(preview) = decode_preview_packet(&packet) else {
                    self.wire_ready = Some(packet);
                    return Some(Self::fault());
                };
                packet.preview = Some(preview);
                self.publication.preview = Some(packet);
                Some(EnergyRun::Preview)
            }
            EnergyWireKind::Checkpoint => match self.publication.checkpoints.push(packet) {
                Ok(()) => Some(EnergyRun::Checkpoint(self.hour_index as u64)),
                Err(packet) => {
                    self.wire_ready = Some(packet);
                    Some(EnergyRun::Yield)
                }
            },
            EnergyWireKind::Commit => match self.publication.commits.push(packet) {
                Ok(()) => {
                    self.stage = EnergyJobStage::Complete;
                    Some(EnergyRun::Yield)
                }
                Err(packet) => {
                    self.wire_ready = Some(packet);
                    Some(EnergyRun::Yield)
                }
            },
            EnergyWireKind::Fault => match self.publication.faults.push(packet) {
                Ok(()) => Some(Self::fault()),
                Err(packet) => {
                    self.wire_ready = Some(packet);
                    Some(EnergyRun::Yield)
                }
            },
        }
    }

    #[cfg(test)]
    pub(super) fn numerical_cursor_signature(&self) -> [u64; 13] {
        [
            self.stage.code() as u64,
            self.tier.code() as u64,
            self.weather_cursor as u64,
            self.weather_target as u64,
            self.initialize_cursor as u64,
            self.warmup_hour as u64,
            self.hour_index as u64,
            self.aggregate_zone_cursor as u64,
            self.aggregate_facility_cursor as u64,
            self.encode_section as u64,
            self.encode_record_cursor as u64,
            self.encode_sample_cursor as u64,
            self.encode_name_cursor as u64,
        ]
    }

    pub fn numerical_digest(&self) -> u64 {
        let mut digest = 0xcbf2_9ce4_8422_2325u64;
        for value in [
            self.stage.code() as u64,
            self.tier.code() as u64,
            self.rng_state,
            self.weather_cursor as u64,
            self.weather_target as u64,
            self.initialize_cursor as u64,
            self.initialize_backing_stage as u64,
            self.warmup_hour as u64,
            self.total_timesteps as u64,
            self.run_backing_stage as u64,
            self.hour_index as u64,
            self.aggregate_zone_cursor as u64,
            self.aggregate_facility_cursor as u64,
            self.facility_electricity_j.to_bits(),
            self.time_series.series.len() as u64,
            self.meters.meters.len() as u64,
            self.zone_temperature_history.len() as u64,
            self.previous_temperatures.len() as u64,
            self.previous_loads.len() as u64,
            self.state.as_ref().map_or(0, |state| state.zones.len()) as u64,
            self.state.as_ref().map_or(0, |state| state.surfaces.len()) as u64,
            self.validation.stage as u64,
            self.precompute.is_some() as u64,
            self.pre.is_some() as u64,
            self.timestep_builder.is_some() as u64,
            self.timestep_work.is_some() as u64,
            self.aggregate_zone_work.is_some() as u64,
            self.aggregate_facility_work.is_some() as u64,
            self.checkpoint_due as u64,
        ] {
            digest ^= value;
            digest = digest.wrapping_mul(0x100_0000_01b3);
        }
        if let Some(builder) = self.precompute.as_ref() {
            for value in builder.retained_wire_signature() {
                digest ^= value;
                digest = digest.wrapping_mul(0x100_0000_01b3);
            }
        }
        if let Some(builder) = self.timestep_builder.as_ref() {
            for value in builder.retained_wire_signature() {
                digest ^= value;
                digest = digest.wrapping_mul(0x100_0000_01b3);
            }
        }
        if let Some(work) = self.timestep_work.as_ref() {
            for value in work.retained_wire_signature() {
                digest ^= value;
                digest = digest.wrapping_mul(0x100_0000_01b3);
            }
        }
        for work in [self.aggregate_zone_work.as_ref(), self.aggregate_facility_work.as_ref()].into_iter().flatten() {
            for value in [work.stage as u64, work.phase as u64, work.copy_index as u64, work.cursor as u64, work.timestamps.len() as u64, work.values.len() as u64] {
                digest ^= value;
                digest = digest.wrapping_mul(0x100_0000_01b3);
            }
        }
        digest
    }

}
