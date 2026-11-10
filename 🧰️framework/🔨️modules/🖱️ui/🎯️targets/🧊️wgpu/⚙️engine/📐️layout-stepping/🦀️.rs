//! 📐️ Layout stepping: the surface lane, the layout steps, the theme propagation, the layout queue and the shell settings. See [`super`].

use super::*;
use semio_framework_job_worker::AdmittedWorkerJobSession as _;

// #region engine
//#region 🔖️Ui
impl Ui {
    /// 🚦️Changes a surface lane without duplicating its pending queue entry.
    pub fn set_surface_lane(&mut self, window_id: &str, lane: SurfaceLane) {
        let Some(window) = self.window_mut(window_id) else { return };
        if window.lane == lane {
            return;
        }
        window.lane = lane;
    }

    /// 🧵️Advances one surface layout by one cursor unit under the caller's fuel/deadline and
    /// cancellation context. Completed geometry publishes only after the whole job is consistent.
    pub fn step_layouts(&mut self, pool: &semio_framework_async::WorkerPool, atlas: &mut FontAtlas, cx: &mut StepContext<'_>) -> UiLayoutStep {
        let _ = atlas;
        if cx.should_yield() {
            return UiLayoutStep::Idle;
        }
        if self.drive_theme_propagation_one() {
            return UiLayoutStep::Idle;
        }
        if let Some(entry) = self.layout_pressure.take() {
            let lane = self.windows.get_token(entry.token).map_or(SurfaceLane::Background, |window| window.lane);
            if let Err(entry) = self.layout_queues[lane.index()].try_push(entry) {
                self.layout_pressure = Some(entry);
            }
            return UiLayoutStep::Idle;
        }
        let Some((token, window_id, lane, _reason)) = self.next_layout() else { return UiLayoutStep::Idle };
        let theme = self.theme;
        let Some(window) = self.windows.get_token_mut(token) else { return UiLayoutStep::Idle };
        window.queued = false;
        if let Some(refusal) = window.layout_retirement_refusal {
            if !matches!(refusal.stage, "Layout.Admission" | "Layout.CloseAdmission") { return UiLayoutStep::Refused { window_id, lane, refusal }; }
            if !window.layout_admission.terminal_is_empty() {
                let grant = cx.retained_grant();
                match close_layout_admission(cx, &mut window.layout_admission, grant, true) {
                    Ok(_) => {
                        self.enqueue_layout(window_id.as_ref());
                        return UiLayoutStep::Yielded { window_id, lane, stage: "Layout.CloseAdmission", nodes: 1, glyphs: 0 };
                    }
                    Err(error) => {
                        let refusal = UiWorkerRetirementRefusal { stage: "Layout.CloseAdmission", kind: error.kind, progress: error.retained_progress() };
                        assert!(window.layout_admission.failure.is_none(), "original admission failure slot remains available after its prior error transfer");
                        *window.layout_admission.failure = Some(error);
                        window.layout_retirement_refusal = Some(refusal);
                        return UiLayoutStep::Refused { window_id, lane, refusal };
                    }
                }
            }
            window.layout_retirement_refusal = None;
        }
        let Some(root) = window.tree.root else { return UiLayoutStep::Idle };
        if cx.is_cancelled() {
            if let Some(job) = window.layout_job.as_mut() {
                job.begin_close();
            }
            window.layout_closing = window.layout_session.is_some() || window.layout_job.is_some() || !window.layout_admission.terminal_is_empty();
            if !window.layout_closing { return UiLayoutStep::Cancelled { window_id, lane }; }
        }
        if let Some(session) = window.layout_session.as_mut() {
            if window.layout_session_generation != Some(window.layout_generation) {
                window.layout_closing = true;
            }
            if session.poll() == semio_framework_job_worker::WorkerJobPoll::CheckedOut {
                if let Some((issued, original)) = session.checked_out_retained_step_receipt() {
                    if !original.fits(*issued) {
                        let refusal = UiWorkerRetirementRefusal { stage: "Layout.OriginalReceiptInvariant", kind: semio_framework_value::ValueRefusalKind::InvariantViolated, progress: *original };
                        window.layout_retirement_refusal = Some(refusal);
                        return UiLayoutStep::Refused { window_id, lane, refusal };
                    }
                    let remaining = cx.retained_grant();
                    if remaining.maximum_items == 0 || remaining.maximum_depth == 0 || !original.fits(remaining) {
                        self.enqueue_layout(window_id.as_ref());
                        return UiLayoutStep::Yielded { window_id, lane, stage: "Layout.OriginalReceiptBudget", nodes: 0, glyphs: 0 };
                    }
                    let progress = *original;
                    if let Err(error) = cx.consume_retained(progress) {
                        let refusal = UiWorkerRetirementRefusal { stage: "Layout.ReceiveOriginalReceipt", kind: error.kind, progress: error.retained_progress() };
                        window.layout_retirement_refusal = Some(refusal);
                        return UiLayoutStep::Refused { window_id, lane, refusal };
                    }
                    let (_, received) = session.take_checked_out_retained_step_receipt().expect("exclusive checked-out original receipt remains pending after admission");
                    assert_eq!(received, progress);
                    self.enqueue_layout(window_id.as_ref());
                    return UiLayoutStep::Yielded { window_id, lane, stage: "Layout.ReceiveOriginalReceipt", nodes: progress.copied_items, glyphs: 0 };
                }
            }
            if window.layout_closing {
                let witness=cx.original_cancel_token();
                let alias=match session.has_original_cancel_alias_witness(witness){Ok(true)=>match cx.with_original_effect(cx.retained_grant(),|cx,effect|session.return_original_cancel_alias_step(cx.original_cancel_token(),effect)){Ok(Some((turn,_)))=>turn,Ok(None)=>semio_framework_value::RetainedWorkOutcome{result:Ok(None),work:0},Err(error)=>semio_framework_value::RetainedWorkOutcome{result:Err(semio_framework_job_worker::WorkerJobDemandError::Refused(error)),work:0}},Ok(false)=>semio_framework_value::RetainedWorkOutcome{result:Ok(None),work:0},Err(error)=>semio_framework_value::RetainedWorkOutcome{result:Err(error),work:0}};
                cx.consume_work(alias.work);
                match alias.result {
                    Ok(Some(step)) => {
                        let progress = step.progress();
                        if let Err(error) = cx.consume_retained(progress) {
                            let refusal = UiWorkerRetirementRefusal { stage: "Layout.ReturnOriginalCancel", kind: error.kind, progress };
                            window.layout_retirement_refusal = Some(refusal);
                            return UiLayoutStep::Refused { window_id, lane, refusal };
                        }
                        self.enqueue_layout(window_id.as_ref());
                        return UiLayoutStep::Yielded { window_id, lane, stage: "Layout.ReturnOriginalCancel", nodes: progress.copied_items, glyphs: 0 };
                    }
                    Ok(None) => {}
                    Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_)) => {
                        self.enqueue_layout(window_id.as_ref());
                        return UiLayoutStep::Yielded { window_id, lane, stage: "Layout.ReturnOriginalCancel", nodes: 0, glyphs: 0 };
                    }
                    Err(semio_framework_job_worker::WorkerJobDemandError::Refused(error)) => {
                        let refusal = UiWorkerRetirementRefusal { stage: "Layout.ReturnOriginalCancel", kind: error.kind, progress: error.retained_progress() };
                        window.layout_retirement_refusal = Some(refusal);
                        return UiLayoutStep::Refused { window_id, lane, refusal };
                    }
                }
                let admitted = match close_layout_worker_session(session,cx).map(|progress|progress.is_some()) {
                        Ok(admitted) => admitted,
                        Err(error)=>{let refusal=window.retain_layout_close_failure("Layout.CloseSession",error);return UiLayoutStep::Refused{window_id,lane,refusal}}
                    };
                if admitted && session.terminal_is_empty() {
                    window.layout_session = None;
                    window.layout_session_generation = None;
                    window.layout_closing = false;
                }
                self.enqueue_layout(window_id.as_ref());
                return UiLayoutStep::Yielded { window_id, lane, stage: "Layout.CloseSession", nodes: 1, glyphs: 0 };
            }
            if session.poll() == semio_framework_job_worker::WorkerJobPoll::CheckedOut {
                if let Some(failure) = session.checked_out_failure() {
                    let refusal=UiWorkerRetirementRefusal{stage:"Layout.WorkerRefusal",kind:failure.primary_value().map_or(semio_framework_value::ValueRefusalKind::InvariantViolated,|error|error.kind),progress:failure.retained_progress()};
                    window.layout_retirement_refusal = Some(refusal);
                    session.begin_close();
                    window.layout_closing = true;
                    return UiLayoutStep::Refused { window_id, lane, refusal };
                }
                let descriptor = session.checked_out_outcome_descriptor();
                let terminal = descriptor.is_some_and(semio_framework_job::JobOutcomeDescriptor::is_terminal);
                if descriptor.is_some_and(|descriptor| !descriptor.is_acknowledged()) {
                    if let Err(error) = session.checked_out_outcome() {
                        let refusal = UiWorkerRetirementRefusal { stage: "Layout.BorrowOriginalOutcome", kind: error.kind, progress: error.retained_progress() };
                        window.layout_retirement_refusal = Some(refusal);
                        return UiLayoutStep::Refused { window_id, lane, refusal };
                    }
                }
                match session.acknowledge_checked_out_outcome(cx) {
                    Ok(true) => {}
                    Ok(false) => {
                        self.enqueue_layout(window_id.as_ref());
                        return UiLayoutStep::Yielded { window_id, lane, stage: "Layout.AcknowledgeOriginalOutcome", nodes: 0, glyphs: 0 };
                    }
                    Err(failure) => {
                        let refusal = window.retain_layout_close_failure("Layout.AcknowledgeOriginalOutcome", failure);
                        return UiLayoutStep::Refused { window_id, lane, refusal };
                    }
                }
                let identity = (token, window.layout_generation, window.revision, window.theme_revision, window.viewport_revision, window.viewport.0, window.viewport.1);
                let layout_preview = session.checked_out_job_mut().and_then(MountedLayoutJob::take_preview_one);
                let glyph_preview = session.checked_out_job_mut().and_then(|job| job.latest_glyph_preview());
                let root_intrinsic_height = session.checked_out_job_mut().and_then(|job| job.root_intrinsic_height());
                if let Some(preview) = layout_preview {
                    window.layout_preview = Some(preview);
                }
                if let Some(preview) = glyph_preview.filter(|preview| preview.generation == window.layout_generation && preview.revision == window.revision) {
                    window.glyph_preview = Some(preview);
                }
                let publish = session.checked_out_job_mut().filter(|job| job.stage() == LayoutJobStage::PublishResults).map(|job| job.publish_one(&mut window.tree, identity));
                if matches!(publish, Some(LayoutJobStep::Complete)) {
                    window.intrinsic_content_height = root_intrinsic_height.unwrap_or(f32::NAN);
                }
                if terminal || matches!(publish, Some(LayoutJobStep::Complete | LayoutJobStep::Fault(_))) || session.resume().is_err() {
                    session.begin_close();
                    window.layout_closing = true;
                }
                self.enqueue_layout(window_id.as_ref());
                return match publish {
                    Some(LayoutJobStep::Complete) => UiLayoutStep::Ready { window_id, lane },
                    Some(LayoutJobStep::Fault(_)) => UiLayoutStep::Cancelled { window_id, lane },
                    _ if terminal => UiLayoutStep::Cancelled { window_id, lane },
                    _ => UiLayoutStep::Yielded { window_id, lane, stage: "Layout.WorkerOutcome", nodes: usize::from(publish.is_some()), glyphs: 0 },
                };
            }
            let poll = match session.pump_one_for_interactive_turn(cx) {
                Err(failure) => Err(window.retain_layout_close_failure("Layout.WorkerPump", failure)),
                Ok(poll) => Ok(poll),
            };
            self.enqueue_layout(window_id.as_ref());
            return match poll {
                Err(refusal) => UiLayoutStep::Refused { window_id, lane, refusal },
                Ok(semio_framework_job_worker::WorkerJobPoll::Submitted) => UiLayoutStep::Awaiting { window_id, lane },
                Ok(semio_framework_job_worker::WorkerJobPoll::Outcome | semio_framework_job_worker::WorkerJobPoll::Terminal) => UiLayoutStep::Yielded { window_id, lane, stage: "Layout.WorkerTake", nodes: 0, glyphs: 0 },
                _ => UiLayoutStep::Yielded { window_id, lane, stage: "Layout.WorkerPool.UserVisible", nodes: 0, glyphs: 0 },
            };
        }
        if let Some(job) = window.layout_job.as_mut() {
            let identity = (token, window.layout_generation, window.revision, window.theme_revision, window.viewport_revision, window.viewport.0, window.viewport.1);
            if job.identity() != identity {
                job.begin_close();
                window.layout_closing = true;
            }
            if window.layout_closing {
                if !window.layout_admission.birth.terminal_is_empty() {
                    if let Err(failure) = window.layout_admission.birth.close_step(cx) {
                        let refusal = window.retain_layout_close_failure("Layout.CloseAdmission", failure);
                        return UiLayoutStep::Refused { window_id, lane, refusal };
                    }
                    self.enqueue_layout(window_id.as_ref());
                    return UiLayoutStep::Yielded { window_id, lane, stage: "Layout.CloseAdmission", nodes: 1, glyphs: 0 };
                }
                if !window.layout_admission.terminal_is_empty() {
                    let grant = cx.retained_grant();
                    if let Err(error) = close_layout_admission(cx, &mut window.layout_admission, grant, true) {
                        let refusal = UiWorkerRetirementRefusal { stage: "Layout.CloseAdmission", kind: error.kind, progress: error.retained_progress() };
                        window.layout_retirement_refusal = Some(refusal);
                        return UiLayoutStep::Refused { window_id, lane, refusal };
                    }
                    self.enqueue_layout(window_id.as_ref());
                    return UiLayoutStep::Yielded { window_id, lane, stage: "Layout.CloseAdmission", nodes: 1, glyphs: 0 };
                }
                let grant = cx.retained_grant();
                let step=match semio_framework_job::InteractiveJob::close_step(job,cx).and_then(|step|step.admit(grant,job.terminal_is_empty()).map_err(semio_framework_job::OriginalTurnFailure::from)){Ok(step)=>step,Err(failure)=>{let refusal=window.retain_layout_close_failure("Layout.CloseUnadmitted",failure);return UiLayoutStep::Refused{window_id,lane,refusal}}};
                if matches!(step, semio_framework_job::InteractiveJobCloseStep::Complete { .. }) {
                    window.layout_job = None;
                    window.layout_closing = false;
                }
                self.enqueue_layout(window_id.as_ref());
                return UiLayoutStep::Yielded { window_id, lane, stage: "Layout.CloseUnadmitted", nodes: 1, glyphs: 0 };
            }
            if job.is_admitted() {
                if window.layout_admission.params.is_none() {
                    let progress = semio_framework_value::RetainedCloneProgress { copied_items: 1, copied_bytes: 0, ..Default::default() };
                    if cx.retained_grant().maximum_depth == 0 || !progress.fits(cx.retained_grant()) {
                        self.enqueue_layout(window_id.as_ref());
                        return UiLayoutStep::Yielded { window_id, lane, stage: "Layout.AdmissionParameters", nodes: 0, glyphs: 0 };
                    }
                    *window.layout_admission.params = Some(semio_framework_job_worker_authority::BatchJobParams {
                        operation: cx.operation(),
                        generation: cx.generation(),
                        cancel: cx.cancel_token(),
                        config: semio_framework_job_worker_authority::BatchDriveConfig { retained: cx.retained_grant(), site: "ui.layout-text.worker", stage: semio_framework_job::InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_us: 1000 },
                        now_us: cx.clock_source(),
                    });
                    cx.consume_retained(progress).expect("original layout parameter metadata was admitted before transfer");
                    self.enqueue_layout(window_id.as_ref());
                    return UiLayoutStep::Yielded { window_id, lane, stage: "Layout.AdmissionParameters", nodes: progress.copied_items, glyphs: 0 };
                }
                match window.layout_admission.birth.advance(&mut window.layout_job, &mut window.layout_admission.params, &mut window.layout_session, cx) {
                    Ok(true) => {
                        window.layout_session_generation = Some(window.layout_generation);
                        self.enqueue_layout(window_id.as_ref());
                        return UiLayoutStep::Yielded { window_id, lane, stage: "Layout.Mount", nodes: 1, glyphs: 0 };
                    }
                    Ok(false) => {
                        self.enqueue_layout(window_id.as_ref());
                        return UiLayoutStep::Yielded { window_id, lane, stage: "Layout.Mount", nodes: 0, glyphs: 0 };
                    }
                    Err(failure) => {
                        let refusal = window.retain_layout_close_failure("Layout.Admission", failure);
                        if let Some(original) = window.layout_job.as_mut() { original.begin_close(); }
                        window.layout_closing = true;
                        self.enqueue_layout(window_id.as_ref());
                        return UiLayoutStep::Refused { window_id, lane, refusal };
                    }
                }
            }
            let outcome = job.admit_one(&window.tree, cx);
            match outcome {
                LayoutJobStep::Yield { stage, nodes, glyphs } => {
                    self.enqueue_layout(window_id.as_ref());
                    return UiLayoutStep::Yielded { window_id, lane, stage: stage_label(stage), nodes, glyphs };
                }
                LayoutJobStep::Cancelled | LayoutJobStep::Fault(_) => {
                    job.begin_close();
                    window.layout_closing = true;
                    self.enqueue_layout(window_id.as_ref());
                    return UiLayoutStep::Cancelled { window_id, lane };
                }
                LayoutJobStep::Complete => {}
            }
        }
        window.layout_job = MountedLayoutJob::try_new(
            &window.tree,
            root,
            MountedLayoutIdentity { surface: token, generation: window.layout_generation, revision: window.revision, theme_revision: window.theme_revision, viewport_revision: window.viewport_revision },
            theme,
            window.viewport.0,
            window.viewport.1,
            window.router.flow().block.is_reversed(),
            window.router.flow().inline,
        )
        .ok();
        if window.layout_job.is_some() {
            self.enqueue_layout(window_id.as_ref());
            UiLayoutStep::Yielded { window_id, lane, stage: "Layout.Preadmit", nodes: 0, glyphs: 0 }
        } else {
            UiLayoutStep::Ready { window_id, lane }
        }
    }

    pub fn drive_theme_propagation_one(&mut self) -> bool {
        let Some(mut cursor) = self.theme_propagation.take() else {
            let Some(theme) = self.pending_theme.take() else { return false };
            self.theme_propagation = Some(ThemePropagationCursor::new(theme));
            return true;
        };
        match cursor.phase {
            ThemePropagationPhase::Validate => {
                if cursor.slot == UI_LAYOUT_SURFACE_SLOTS {
                    cursor.slot = 0;
                    cursor.phase = ThemePropagationPhase::Apply;
                    self.theme_propagation = Some(cursor);
                    return true;
                }
                let token = self.windows.token_at(cursor.slot);
                if let Some(window) = token.and_then(|token| self.windows.get_token(token)).filter(|window| window.closing.is_none()) {
                    if window.layout_generation == u64::MAX || window.theme_revision == u64::MAX {
                        self.theme_fault = true;
                        self.theme_propagation = Some(cursor);
                        return true;
                    }
                }
                cursor.tokens[cursor.slot] = token;
                cursor.slot += 1;
                self.theme_propagation = Some(cursor);
                true
            }
            ThemePropagationPhase::Apply => {
                if cursor.slot == UI_LAYOUT_SURFACE_SLOTS {
                    cursor.phase = ThemePropagationPhase::Publish;
                    self.theme_propagation = Some(cursor);
                    return true;
                }
                let token = cursor.tokens[cursor.slot];
                cursor.slot += 1;
                if let Some(token) = token {
                    let Some(window) = self.windows.get_token_mut(token).filter(|window| window.closing.is_none()) else {
                        self.theme_propagation = Some(cursor);
                        return true;
                    };
                    let Some(layout_generation) = window.layout_generation.checked_add(1) else {
                        self.theme_fault = true;
                        self.theme_propagation = Some(cursor);
                        return true;
                    };
                    let Some(theme_revision) = window.theme_revision.checked_add(1) else {
                        self.theme_fault = true;
                        self.theme_propagation = Some(cursor);
                        return true;
                    };
                    window.layout_generation = layout_generation;
                    window.theme_revision = theme_revision;
                    if let Some(root) = window.tree.root {
                        window.tree.mark_dirty(root, NodeFlags::DIRTY_LAYOUT);
                    }
                    self.enqueue_layout_token(token, SurfaceLayoutReason::Theme);
                }
                self.theme_propagation = Some(cursor);
                true
            }
            ThemePropagationPhase::Publish => {
                self.theme = cursor.theme;
                self.theme_fault = false;
                if let Some(theme) = self.pending_theme.take() {
                    if theme_layout_identity(&self.theme) == theme_layout_identity(&theme) {
                        self.theme = theme;
                    } else {
                        self.theme_propagation = Some(ThemePropagationCursor::new(theme));
                    }
                }
                true
            }
        }
    }

    pub(super) fn enqueue_layout(&mut self, window_id: &str) {
        self.enqueue_layout_reason(window_id, SurfaceLayoutReason::Dirty);
    }

    pub(super) fn enqueue_layout_reason(&mut self, window_id: &str, reason: SurfaceLayoutReason) {
        let Some(token) = self.windows.token(window_id) else { return };
        self.enqueue_layout_token(token, reason);
    }

    fn enqueue_layout_token(&mut self, token: UiSurfaceToken, reason: SurfaceLayoutReason) {
        let Some(window) = self.windows.get_token_mut(token) else { return };
        if window.queued || window.closing.is_some() || window.layout_retirement_refusal.is_some() {
            return;
        }
        window.queued = true;
        let lane = window.lane;
        let entry = SurfaceLaneEntry { token, reason, epoch: window.layout_generation };
        if let Err(entry) = self.layout_queues[lane.index()].try_push(entry) {
            self.layout_pressure = Some(entry);
        }
    }

    fn next_layout(&mut self) -> Option<(UiSurfaceToken, SurfaceId, SurfaceLane, SurfaceLayoutReason)> {
        for _ in 0..LANE_WHEEL.len() {
            let lane = LANE_WHEEL[self.lane_cursor];
            self.lane_cursor = (self.lane_cursor + 1) % LANE_WHEEL.len();
            let Some(entry) = self.layout_queues[lane.index()].pop() else { continue };
            let Some(window) = self.windows.get_token(entry.token) else { continue };
            if window.closing.is_some() {
                continue;
            }
            if window.layout_generation != entry.epoch {
                let current = SurfaceLaneEntry { epoch: window.layout_generation, ..entry };
                if let Err(current) = self.layout_queues[window.lane.index()].try_push(current) {
                    self.layout_pressure = Some(current);
                }
                continue;
            }
            if window.lane != lane {
                if let Err(entry) = self.layout_queues[window.lane.index()].try_push(entry) {
                    self.layout_pressure = Some(entry);
                }
                continue;
            }
            let id = self.windows.id(entry.token)?.clone();
            return Some((entry.token, id, lane, entry.reason));
        }
        None
    }

    pub fn set_window_kind_icons(&mut self, icons: HashMap<String, IconName>) {
        self.shell.set_window_kind_icons(icons);
    }

    /// 🪟️ Rebuilds the shared `🐚️Shell`'s retained dock/split/tab chrome from a declarative
    /// `WindowLayout` (independent of any window's `apply_tree`d content — see `shell`'s doc comment).
    pub fn set_window_layout(&mut self, layout: WindowLayout) {
        self.shell.set_window_layout(layout);
    }

    /// 🧭️ Forwards to `Shell::set_navbar` (stub — see that method's doc comment).
    pub fn set_navbar(&mut self, items: Vec<String>) {
        self.shell.set_navbar(items);
    }

    pub fn shell(&self) -> &Shell {
        &self.shell
    }

    /// 🖼️ True when any window's retained tree still carries `DIRTY_LAYOUT`/`DIRTY_PAINT`/
    /// `SUBTREE_DIRTY` on its root. No animation-clock scaffolding exists anywhere in this crate yet
    /// (nothing under `arena`/`tree`/`reconcile`/`flex`/`paint`/`events`/`scene_slots`/`shell`
    /// schedules a future wake), so this is purely dirty-flag-driven; wiring a real animation deadline
    /// is separate follow-up work, not this façade's job to invent.
    pub fn needs_frame(&self) -> bool {
        self.theme_propagation.is_some() || self.pending_theme.is_some() || self.windows.values().any(UiWindow::is_dirty)
    }

}
//#endregion 🔖️Ui
// #endregion engine
