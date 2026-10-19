//! 🏁️ Job completion and interactive job: the measuring, metadata and pipeline steps, the job completion, its drop and the `InteractiveJob` implementation. See [`super`].

use super::*;

//#region ⚙️PreparationJob
impl PreparedRenderJob {
    fn measure_next(&mut self) -> Option<PreparedRenderUsage> {
        match self.section {
            PreparationSection::Draw => {
                let input = self.input.as_ref()?;
                self.has_animated_primitives |= Self::animated_primitive_at(&input.draw, self.draw_cursor);
                if let Some(usage) = Self::next_draw_usage(&input.draw, &mut self.draw_cursor) {
                    return Some(usage);
                }
                self.section = PreparationSection::Overlay;
                Some(PreparedRenderUsage::default())
            }
            PreparationSection::Overlay => {
                let input = self.input.as_ref()?;
                if let Some(overlay) = &input.overlay {
                    self.has_animated_primitives |= Self::animated_primitive_at(overlay, self.overlay_cursor);
                    if let Some(usage) = Self::next_draw_usage(overlay, &mut self.overlay_cursor) {
                        return Some(usage);
                    }
                }
                self.section = PreparationSection::Uploads;
                self.metadata_cursor = 0;
                Some(PreparedRenderUsage::default())
            }
            PreparationSection::Uploads => {
                let input = self.input.as_ref()?;
                if let Some(upload) = input.uploads.get(self.metadata_cursor) {
                    let source = self.metadata_cursor;
                    let Some(next) = self.metadata_cursor.checked_add(1) else {
                        self.fault = Some("prepared upload cursor exhausted");
                        return Some(PreparedRenderUsage::default());
                    };
                    let Some(bytes) = upload.byte_len() else {
                        self.fault = Some("prepared upload byte claim overflowed");
                        return Some(PreparedRenderUsage::default());
                    };
                    self.metadata_cursor = next;
                    let scene_raster_bytes = upload.scene_raster_byte_len();
                    let Some(digest) = u64::try_from(bytes).ok().zip(u64::try_from(scene_raster_bytes).ok()).map(|(bytes, scene)| bytes ^ scene.rotate_left(31)) else {
                        self.fault = Some("prepared upload digest exhausted");
                        return Some(PreparedRenderUsage::default());
                    };
                    let command = PreparedRenderCommand { kind: PreparedRenderCommandKind::Upload, source, digest, draw_cursor: None, packet_overlay: false };
                    let Some(commands) = self.commands.as_mut() else {
                        self.fault = Some("prepared render command owner was missing");
                        return Some(PreparedRenderUsage::default());
                    };
                    if commands.try_push(command).is_err() {
                        self.fault = Some("prepared render command page credits exhausted");
                        return Some(PreparedRenderUsage::default());
                    }
                    return Some(PreparedRenderUsage { upload_items: 1, upload_bytes: bytes, scene_raster_bytes, ..PreparedRenderUsage::default() });
                }
                self.section = PreparationSection::Evictions;
                self.metadata_cursor = 0;
                Some(PreparedRenderUsage::default())
            }
            PreparationSection::Evictions => {
                let input = self.input.as_ref()?;
                if let Some(eviction) = input.evictions.get(self.metadata_cursor) {
                    let Some(next) = self.metadata_cursor.checked_add(1) else {
                        self.fault = Some("prepared eviction cursor exhausted");
                        return Some(PreparedRenderUsage::default());
                    };
                    let Some(bytes) = eviction.byte_len() else {
                        self.fault = Some("prepared eviction byte claim overflowed");
                        return Some(PreparedRenderUsage::default());
                    };
                    self.metadata_cursor = next;
                    return Some(PreparedRenderUsage { upload_items: 1, upload_bytes: bytes, ..PreparedRenderUsage::default() });
                }
                self.section = PreparationSection::Damage;
                self.metadata_cursor = 0;
                Some(PreparedRenderUsage::default())
            }
            PreparationSection::Damage => self.measure_metadata(PreparationSection::Clips, |input| input.damage.len(), size_of::<ScissorRect>()),
            PreparationSection::Clips => self.measure_metadata(PreparationSection::Directives, |input| input.clips.len(), size_of::<ScissorRect>()),
            PreparationSection::Directives => self.measure_metadata(PreparationSection::Validate, |input| input.directives.len(), size_of::<RenderDirective>()),
            PreparationSection::Validate | PreparationSection::Snap | PreparationSection::Order | PreparationSection::Tessellate | PreparationSection::Batch | PreparationSection::Hash => self.advance_pipeline(),
            PreparationSection::Complete => None,
        }
    }

    fn measure_metadata(&mut self, next: PreparationSection, len: impl FnOnce(&PreparedRenderInput) -> usize, bytes: usize) -> Option<PreparedRenderUsage> {
        if self.metadata_cursor < len(self.input()?) {
            let Some(next) = self.metadata_cursor.checked_add(1) else {
                self.fault = Some("prepared metadata cursor exhausted");
                return Some(PreparedRenderUsage::default());
            };
            self.metadata_cursor = next;
            Some(PreparedRenderUsage { draw_items: 1, draw_bytes: bytes, ..PreparedRenderUsage::default() })
        } else {
            self.section = next;
            self.metadata_cursor = 0;
            Some(PreparedRenderUsage::default())
        }
    }

    fn animated_primitive_at(draw: &DrawList, cursor: DrawMeasureCursor) -> bool {
        let DrawMeasureCursor::LayerUi { layer, item, overlay } = cursor else { return false };
        let Some(layer) = draw.layers.get(layer) else { return false };
        let items = if overlay { &layer.overlay_ui_instances } else { &layer.ui_instances };
        items.get(item).is_some_and(|instance| {
            let kind = instance.params[2];
            kind == semio_framework_ui_draw_contract::KIND_LOADING_BORDER || kind == semio_framework_ui_draw_contract::KIND_WAITING_BORDER || kind == semio_framework_ui_draw_contract::KIND_INTRODUCING_BORDER
        })
    }

    fn advance_pipeline(&mut self) -> Option<PreparedRenderUsage> {
        let prepared_cursor = if self.pipeline_overlay { self.pipeline_overlay_cursor } else { self.pipeline_cursor };
        let usage = if self.pipeline_overlay {
            let input = self.input.as_ref()?;
            match input.overlay.as_ref() {
                Some(overlay) => Self::next_draw_usage(overlay, &mut self.pipeline_overlay_cursor),
                None => None,
            }
        } else {
            let input = self.input.as_ref()?;
            Self::next_draw_usage(&input.draw, &mut self.pipeline_cursor)
        };
        if let Some(usage) = usage {
            let source = self.metadata_cursor;
            let Some(next) = self.metadata_cursor.checked_add(1) else {
                self.fault = Some("prepared pipeline cursor exhausted");
                return Some(PreparedRenderUsage::default());
            };
            let Some(items) = u64::try_from(usage.draw_items).ok() else {
                self.fault = Some("prepared pipeline item digest exhausted");
                return Some(PreparedRenderUsage::default());
            };
            let Some(bytes) = u64::try_from(usage.draw_bytes).ok() else {
                self.fault = Some("prepared pipeline byte digest exhausted");
                return Some(PreparedRenderUsage::default());
            };
            self.metadata_cursor = next;
            self.digest = self.digest.rotate_left(7) ^ items ^ bytes.rotate_left(17);
            if matches!(self.section, PreparationSection::Tessellate) {
                let command = PreparedRenderCommand { kind: PreparedRenderCommandKind::Tessellate, source, digest: self.digest, draw_cursor: Some(prepared_cursor), packet_overlay: self.pipeline_overlay };
                let Some(commands) = self.commands.as_mut() else {
                    self.fault = Some("prepared render command owner was missing");
                    return Some(PreparedRenderUsage::default());
                };
                if commands.try_push(command).is_err() {
                    self.fault = Some("prepared render command page credits exhausted");
                    return Some(PreparedRenderUsage::default());
                }
            }
            return Some(PreparedRenderUsage::default());
        }
        if !self.pipeline_overlay {
            self.pipeline_overlay = true;
            self.metadata_cursor = 0;
            return Some(PreparedRenderUsage::default());
        }
        self.pipeline_overlay = false;
        self.pipeline_cursor = DrawMeasureCursor::default();
        self.pipeline_overlay_cursor = DrawMeasureCursor::default();
        self.metadata_cursor = 0;
        self.section = match self.section {
            PreparationSection::Validate => PreparationSection::Snap,
            PreparationSection::Snap => PreparationSection::Order,
            PreparationSection::Order => PreparationSection::Tessellate,
            PreparationSection::Tessellate => PreparationSection::Batch,
            PreparationSection::Batch => PreparationSection::Hash,
            PreparationSection::Hash => PreparationSection::Complete,
            _ => return None,
        };
        Some(PreparedRenderUsage::default())
    }

    fn include_usage(&mut self, usage: PreparedRenderUsage) -> bool {
        self.usage.include_draw(usage.draw_items, usage.draw_bytes) && (usage.upload_items == 0 || self.usage.include_upload(usage.upload_bytes, usage.scene_raster_bytes))
    }

    fn fault_outcome<'a>(&'a mut self, fault: &'static str, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        self.fault = Some(fault);
        self.closing = true;
        self.publish_fault(cx)
    }

    fn publish_fault<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        if !self.fault_payload.advance_initialization(cx)? || !self.fault_payload.seal(cx)? { return Ok(None); }
        JobOutcomeBorrow::admit_fault(cx, self.fault_payload.published().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "prepared fault owner was not sealed"))?).map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    pub(super) fn complete<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        if !self.packet_command_staged {
            let command = PreparedRenderCommand { kind: PreparedRenderCommandKind::Packet, source: 0, digest: self.digest, draw_cursor: None, packet_overlay: false };
            let Some(commands) = self.commands.as_mut() else { return self.fault_outcome("prepared render command owner was missing", cx) };
            if commands.try_push(command).is_err() {
                return self.fault_outcome("prepared render packet command credits exhausted", cx);
            }
            self.packet_command_staged = true;
            return JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if !self.published {
            let Some(input) = self.input.take() else { return self.fault_outcome("prepared render input was missing before publication", cx) };
            if !input.permit.as_ref().is_some_and(PreparedRenderProcessPermit::matches) {
                self.input = Some(input);
                return self.fault_outcome("prepared render process generation was stale before publication", cx);
            }
            let revision = input.scene_revision;
            let generation = input.preview_generation;
            let Some(commands) = self.commands.take() else {
                self.input = Some(input);
                return self.fault_outcome("prepared render command pages were missing before publication", cx);
            };
            let input = std::mem::ManuallyDrop::new(input);
            let packet = PreparedRenderPacket {
                has_animated_primitives: self.has_animated_primitives,
                scene_revision: revision,
                preview_generation: generation,
                damage: unsafe { std::ptr::read(&input.damage) },
                clips: unsafe { std::ptr::read(&input.clips) },
                directives: unsafe { std::ptr::read(&input.directives) },
                uploads: unsafe { std::ptr::read(&input.uploads) },
                evictions: unsafe { std::ptr::read(&input.evictions) },
                draw: unsafe { std::ptr::read(&input.draw) },
                overlay: unsafe { std::ptr::read(&input.overlay) },
                commands,
                time_seconds: input.time_seconds,
                usage: self.usage,
                limits: input.limits,
                permit: unsafe { std::ptr::read(&input.permit) },
                retirement_phase: 0,
                abandonment_slot: u8::MAX,
            };
            let packet = match packet.try_arm_abandonment() {
                Ok(packet) => packet,
                Err(packet) => {
                    self.rejected_packet = Some(packet);
                    return self.fault_outcome("prepared render packet abandonment admission was refused", cx);
                }
            };
            if let Err(packet) = self.receiver.publish(packet) {
                self.rejected_packet = Some(packet);
                return self.fault_outcome("prepared render packet mailbox publication refused", cx);
            }
            self.published = true;
            return JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        JobOutcomeBorrow::admit_complete(cx, None, None).map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

impl Drop for PreparedRenderJob {
    fn drop(&mut self) {
        if self.terminal_is_empty() || self.abandonment_slot == u8::MAX {
            return;
        }
        let slot = usize::from(self.abandonment_slot);
        let Some(state) = PREPARED_RENDER_JOB_ABANDONMENT_STATE.get(slot) else { return };
        if state.load(Ordering::Acquire) != 1 {
            return;
        }
        let job = Box::new(Self {
            has_animated_primitives: self.has_animated_primitives,
            input: self.input.take(),
            usage: std::mem::take(&mut self.usage),
            section: self.section,
            draw_cursor: self.draw_cursor,
            overlay_cursor: self.overlay_cursor,
            pipeline_cursor: self.pipeline_cursor,
            pipeline_overlay_cursor: self.pipeline_overlay_cursor,
            pipeline_overlay: self.pipeline_overlay,
            metadata_cursor: self.metadata_cursor,
            commands: self.commands.take(),
            digest: self.digest,
            fault: self.fault.take(),
            fault_payload: std::mem::replace(&mut self.fault_payload, RetainedPayloadBuilder::new(JobPayloadStream::Fault)),
            receiver: std::mem::replace(&mut self.receiver, PreparedRenderReceiver::unowned()),
            rejected_upload: self.rejected_upload.take(),
            rejected_packet: self.rejected_packet.take(),
            raster_backing_closed: self.raster_backing_closed,
            packet_command_staged: self.packet_command_staged,
            published: self.published,
            closing: true,
            abandonment_slot: self.abandonment_slot,
        });
        self.abandonment_slot = u8::MAX;
        self.closing = true;
        PREPARED_RENDER_JOB_ABANDONMENT_OWNER[slot].store(Box::into_raw(job), Ordering::Release);
        state.store(2, Ordering::Release);
    }
}

impl InteractiveJob for PreparedRenderJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        let grant = cx.retained_grant();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(None); }
        if cx.is_cancelled() {
            self.closing = true;
            return JobOutcomeBorrow::admit_cancelled(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if cx.should_yield() { return Ok(None); }
        if self.fault.is_some() { return self.publish_fault(cx); }
        if self.closing { return self.fault_outcome("prepared render job was stepped while closing", cx); }
        let Some(input) = self.input() else {
            return self.complete(cx);
        };
        if input.preview_generation != cx.generation().0 || !input.permit.as_ref().is_some_and(PreparedRenderProcessPermit::matches) {
            return self.fault_outcome("prepared render generation is stale", cx);
        }
        if let Some(producer) = self.input.as_mut().and_then(|input| input.raster_producers.front_mut()) {
            cx.consume_fuel(1);
            let step = producer.step(cx.generation().0);
            match step {
                PreparedRasterProducerStep::Pending => {},
                PreparedRasterProducerStep::Complete(upload) => {
                    let Some(input) = self.input.as_mut() else { return self.fault_outcome("prepared render input disappeared during raster publication", cx) };
                    input.raster_producers.pop_front();
                    if let Err(upload) = input.try_push_upload(upload) {
                        self.rejected_upload = Some(upload);
                        return self.fault_outcome("prepared render upload admission refused the exact raster owner", cx);
                    }
                }
                PreparedRasterProducerStep::Fault(fault) => return self.fault_outcome(fault, cx),
            }
            if cx.is_cancelled() {
                self.closing = true;
                return JobOutcomeBorrow::admit_cancelled(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
            }
            if self.input().is_some_and(|input| input.preview_generation != cx.generation().0) {
                return self.fault_outcome("prepared render generation became stale after raster work", cx);
            }
            return JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if !self.raster_backing_closed {
            cx.consume_fuel(1);
            let Some(input) = self.input.as_mut() else { return self.fault_outcome("prepared render input disappeared before producer backing close", cx) };
            if input.raster_producers.release_backing_step() {
                self.raster_backing_closed = true;
            }
            return JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        let Some(usage) = self.measure_next() else {
            return self.complete(cx);
        };
        cx.consume_fuel(1);
        if self.fault.is_some() || !self.include_usage(usage) {
            return self.fault_outcome("prepared render cumulative credits overflowed", cx);
        }
        let Some(input) = self.input() else { return self.fault_outcome("prepared render input disappeared after one worker unit", cx) };
        if !self.usage.fits(input.limits) {
            return self.fault_outcome("prepared render credits exceeded", cx);
        }
        if cx.is_cancelled() {
            self.closing = true;
            return JobOutcomeBorrow::admit_cancelled(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if input.preview_generation != cx.generation().0 || !input.permit.as_ref().is_some_and(PreparedRenderProcessPermit::matches) {
            return self.fault_outcome("prepared render generation became stale after one worker unit", cx);
        }
        JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Complete if self.published => descriptor.complete(None, None),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Fault if self.fault.is_some() => descriptor.fault(self.fault_payload.published().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "prepared original fault custody was not published"))?),
            _ => Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "prepared outcome descriptor does not match its original owner")),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<CloseStep,semio_framework_job::OriginalTurnFailure> {
        let grant=cx.retained_grant();
        if cx.should_yield(){return Ok(CloseStep::Pending{progress:Default::default()})}
        let step=self.close_granted(grant).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?;
        let step=step.admit(grant,self.terminal_is_empty()).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?;
        cx.consume_retained(step.progress()).map_err(semio_framework_job::OriginalTurnFailure::from)?;
        if step.progress().copied_items!=0{cx.consume_fuel(1)}
        Ok(step)
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { self.next_close_demands(0).map(|grant| grant.maximum_copy_bytes) }
    fn next_close_capacity_byte_demand(&self, _copy: usize) -> Result<usize, ValueError> { self.next_close_demands(_copy).map(|grant| grant.maximum_capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { self.next_close_demands(0).map(|grant| grant.maximum_release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { self.next_close_demands(0).map(|grant| grant.maximum_depth) }

    fn terminal_is_empty(&self) -> bool {
        self.closing && PreparedRenderJob::terminal_is_empty(self)
    }
}
//#endregion ⚙️PreparationJob
