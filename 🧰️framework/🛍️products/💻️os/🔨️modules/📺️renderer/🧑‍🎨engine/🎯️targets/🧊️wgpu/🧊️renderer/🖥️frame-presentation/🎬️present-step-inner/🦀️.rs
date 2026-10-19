//! 🎬️ Present step inner: the long `AppPresenter::present_step_inner` that runs one presentation opportunity. See [`super`].

use super::*;

//#region 📮️RuntimeMailbox
impl AppPresenter {
    /// 🩺️ The stored string is only ever surfaced by this fn's `Err` return,
    /// and that return is unreachable while `pending` is still held by the
    /// `Aborted` phase this line just entered — so an abort that never
    /// finishes closing its candidate reported NOTHING at all, leaving the
    /// host's own `frame gate blocked=true phase=Some(Aborted)
    /// retained-fault=true` as the only trace and the REASON invisible
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
    /// `📓️wgpu-wheel-zoom-a11y-live-2026-09-14.md` §3.4a).
    ///
    /// 🎟️ The pair the BUILD was admitted under, never `current()` — see
    /// `RuntimePresentationAuthority::admit_build` for the surface fault that re-reading a
    /// moving authority here produced.
    ///
    /// 🧷️ A prepared world draw whose mesh is not resident is SKIPPED, never fatal — the
    /// parity behaviour of React, which renders nothing until its loader resolves. It is
    /// still a residency-law breach on the build side, so it is reported on the transition
    /// rather than swallowed (`retain_ensured_world_draws`, `📓️w11a-…`).
    pub(super) fn present_step_inner(&mut self,_cx:&mut semio_framework_job::StepContext<'_>) -> Result<AppPresentStep, String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = RENDERER_IO_WAKE.swap(false, Ordering::AcqRel);
            let _ = pump_renderer_io_sessions(1);
        }
        let _ = semio_framework_job_worker::pump_worker_job_retirements(1, 1, semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES);
        if self.pending.is_none() {
            if let Some(retirement) = self.retirement.as_mut() {
                if retirement.step(&mut self.gpu, &self.gate)? {
                    self.retirement = None;
                }
                return Ok(AppPresentStep::Pending);
            }
            if let Some(fault) = self.retained_fault.take() {
                return Err(fault);
            }
            return Ok(AppPresentStep::Idle);
        }
        let upload_progress = self.gpu.prepared_upload_progress();
        let Some(cursor) = self.pending.as_mut() else {
            self.stall = AppPresentStallWatch::default();
            return Ok(AppPresentStep::Idle);
        };
        if let Some(shape) = Self::note_present_stall(&mut self.stall, cursor, upload_progress) {
            log_debug(&format!("[TRACE] os_host present stalled {shape} retained-fault={:?}", self.retained_fault));
            if !matches!(cursor.phase, AppPresentPhase::Aborted) {
                if self.retained_fault.is_none() {
                    self.retained_fault = Some(format!("presentation stalled: {shape}"));
                }
                cursor.phase = AppPresentPhase::Aborted;
                return Ok(AppPresentStep::Pending);
            }
        }
        match cursor.phase {
            AppPresentPhase::Aborted => {
                if !self.engine.close_active_candidate_step(&mut self.gpu)? {
                    return Ok(AppPresentStep::Pending);
                }
                if let Some(gpu_cursor) = cursor.gpu_cursor.as_mut() {
                    gpu_cursor.begin_close();
                    if !gpu_cursor.close_step() {
                        return Ok(AppPresentStep::Pending);
                    }
                    cursor.gpu_cursor = None;
                    return Ok(AppPresentStep::Pending);
                }
                if let Some(input_candidate) = cursor.frame.input_candidate {
                    let runtime = self.runtime.clone();
                    let Ok(mut runtime) = runtime.try_lock() else { return Ok(AppPresentStep::Pending) };
                    let Some(interaction) = runtime.interaction.as_mut() else { return Ok(AppPresentStep::Pending) };
                    if !interaction.shell.input.discard_presented_input_candidate(input_candidate) {
                        return Err("aborted presentation lost its staged input witness".to_string());
                    }
                    cursor.frame.input_candidate = None;
                }
                let Some(mut aborted) = self.pending.take() else { return Err("aborted presentation cursor was missing".to_string()) };
                let retirement = self.retirement.get_or_insert_with(|| AppPresentedRetirement::new(None));
                if retirement.completed_frame.is_some() {
                    self.pending = Some(aborted);
                    return Err("aborted presentation retirement capacity exhausted".to_string());
                }
                if let Some(witness) = aborted.raster_witness.take() {
                    if let Err(error) = self.raster_operation_authority.release(witness) {
                        aborted.raster_witness = Some(witness);
                        self.pending = Some(aborted);
                        return Err(error.to_string());
                    }
                    retirement.raster = RasterCandidateRetirement::Abort(witness);
                }
                retirement.completed_frame = Some(aborted.frame);
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::Fullscreen => {
                if let Some(active) = cursor.frame.fullscreen {
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        if let Some(window) = self.window.as_ref() {
                            window.set_fullscreen(if active { Some(Fullscreen::Borderless(None)) } else { None });
                        }
                    }
                    #[cfg(target_arch = "wasm32")]
                    {
                        use winit::platform::web::WindowExtWebSys;
                        if let Some(canvas) = self.window.as_ref().and_then(|window| window.canvas()) {
                            let document = canvas.owner_document();
                            if active {
                                if let Err(error) = canvas.request_fullscreen() {
                                    web_sys::console::error_2(&"Fullscreen request was rejected".into(), &error);
                                }
                            } else if let Some(document) = document {
                                document.exit_fullscreen();
                            }
                        }
                    }
                }
                cursor.phase = AppPresentPhase::Directives;
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::Engine => {
                if let Some(packet) = cursor.frame.engine_packets.get(cursor.engine) {
                    let candidate = cursor.raster_witness.ok_or_else(|| "engine raster operation witness was missing".to_string())?;
                    let expected = self.raster_operation_authority.current().ok_or_else(|| "engine raster operation authority was empty".to_string())?;
                    match self.engine.realize_step(&mut self.gpu, packet, candidate, expected) {
                        Ok(true) => cursor.engine += 1,
                        Ok(false) => return Ok(AppPresentStep::Pending),
                        Err(error) => {
                            cursor.phase = AppPresentPhase::Aborted;
                            if self.retained_fault.is_none() {
                                log_debug(&format!("[TRACE] os_host present aborted engine={} fault={error}", cursor.engine));
                                self.retained_fault = Some(format!("engine canvas present: {error}"));
                            }
                            return Ok(AppPresentStep::Pending);
                        }
                    }
                    return Ok(AppPresentStep::Pending);
                }
                cursor.phase = AppPresentPhase::Uploads;
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::BeginGpu => {
                let packet = cursor.frame.packet.as_ref().ok_or_else(|| "prepared frame packet was transferred before admission".to_string())?;
                let expected = self.presentation_authority.admitted();
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let token = semio_framework_ui_prepared::UiPresentToken::mint_for_current_thread();
                    if let Err(error) = self.gpu.begin_prepared(&token, &self.gate, packet, expected.scene_revision, expected.input_generation) {
                        cursor.phase = AppPresentPhase::Aborted;
                        return Err(format!("prepared frame admission: {error}"));
                    }
                }
                #[cfg(target_arch = "wasm32")]
                {
                    let Some(token) = self.offscreen_token.as_ref() else {
                        cursor.phase = AppPresentPhase::Aborted;
                        return Err("browser presentation requires dedicated Worker authority".to_string());
                    };
                    if let Err(error) = self.gpu.begin_prepared_offscreen(token, &self.gate, packet, expected.scene_revision, expected.input_generation) {
                        cursor.phase = AppPresentPhase::Aborted;
                        return Err(format!("offscreen prepared frame admission: {error}"));
                    }
                }
                let witness = match self.raster_operation_authority.begin(expected.scene_revision, expected.input_generation) {
                    Ok(witness) => witness,
                    Err(error) => {
                        cursor.phase = AppPresentPhase::Aborted;
                        return Err(format!("raster operation admission: {error}"));
                    }
                };
                cursor.raster_witness = Some(witness);
                if let Err(error) = self.gpu.begin_raster_ownership(witness) {
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err(format!("raster ownership admission: {error}"));
                }
                cursor.phase = AppPresentPhase::Ownership;
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::Ownership => {
                let packet = cursor.frame.packet.as_ref().ok_or_else(|| "prepared frame packet was transferred before raster ownership publication".to_string())?;
                let witness = cursor.raster_witness.ok_or_else(|| "raster operation witness was missing during ownership publication".to_string())?;
                cursor.raster_keep_steps = cursor.raster_keep_steps.saturating_add(1);
                match packet.raster_keep_step(&mut cursor.raster_keep) {
                    semio_framework_ui_prepared::PreparedRasterKeepStepV1::Pending => {}
                    semio_framework_ui_prepared::PreparedRasterKeepStepV1::Key(key) => {
                        if let Err(error) = self.gpu.publish_raster_ownership(witness, key) {
                            cursor.phase = AppPresentPhase::Aborted;
                            return Err(error);
                        }
                    }
                    semio_framework_ui_prepared::PreparedRasterKeepStepV1::Complete => {
                        if let Err(error) = self.gpu.seal_raster_ownership(witness) {
                            cursor.phase = AppPresentPhase::Aborted;
                            return Err(error);
                        }
                        cursor.phase = AppPresentPhase::Engine;
                    }
                }
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::Uploads => {
                let packet = cursor.frame.packet.as_ref().ok_or_else(|| "prepared frame packet was transferred before upload".to_string())?;
                if cursor.upload >= packet.uploads().len() {
                    cursor.phase = AppPresentPhase::Stage;
                    return Ok(AppPresentStep::Pending);
                }
                let candidate = cursor.raster_witness.ok_or_else(|| "prepared raster operation witness was missing".to_string())?;
                let expected = self.raster_operation_authority.current().ok_or_else(|| "prepared raster operation authority was empty".to_string())?;
                match self.gpu.apply_prepared_upload_step(packet, cursor.upload, candidate, expected) {
                    Ok(true) => cursor.upload += 1,
                    Ok(false) => {}
                    Err(error) => {
                        cursor.phase = AppPresentPhase::Aborted;
                        return Err(error);
                    }
                }
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::Stage => {
                let Some(packet) = cursor.frame.packet.take() else { return Err("validated prepared frame packet was missing".to_string()) };
                cursor.witness = match self.gate.stage_presented(packet) {
                    Ok(witness) => Some(witness),
                    Err(packet) => {
                        cursor.frame.packet = Some(packet);
                        cursor.phase = AppPresentPhase::Aborted;
                        return Err("prepared frame presenter witness was already occupied".to_string());
                    }
                };
                cursor.phase = AppPresentPhase::Render;
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::Render => {
                let Some(input_candidate) = cursor.frame.input_candidate else {
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err("prepared frame input candidate witness was missing before submit".to_string());
                };
                let runtime = self.runtime.clone();
                let Ok(mut runtime) = runtime.try_lock() else {
                    cursor.input_wait = AppPresentInputWait::RuntimeLock;
                    return Ok(AppPresentStep::RetryRuntime);
                };
                if runtime.interaction.is_none() {
                    drop(runtime);
                    match self.runtime.presenter_interaction_status() {
                        PresenterInteractionStep::Ready | PresenterInteractionStep::Restored => {
                            cursor.input_wait = AppPresentInputWait::None;
                            cursor.input_progress = cursor.input_progress.saturating_add(1);
                            return Ok(AppPresentStep::Pending);
                        }
                        PresenterInteractionStep::Waiting { site, request } => {
                            cursor.input_wait = AppPresentInputWait::InteractionCheckout { site, request };
                            return Ok(AppPresentStep::AwaitingRuntime);
                        }
                        PresenterInteractionStep::RuntimeLock => {
                            cursor.input_wait = AppPresentInputWait::RuntimeLock;
                            return Ok(AppPresentStep::RetryRuntime);
                        }
                        PresenterInteractionStep::Abandoned => {
                            cursor.frame.packet = self.gate.abort_pending();
                            cursor.witness = None;
                            cursor.phase = AppPresentPhase::Aborted;
                            return Err("presenter interaction checkout had no return owner before submit".to_string());
                        }
                    }
                }
                let Some(interaction) = runtime.interaction.as_mut() else { return Ok(AppPresentStep::AwaitingRuntime) };
                let AppInteractionState { shell, input, .. } = interaction;
                match shell.input.progress_presented_input_candidate(input, input_candidate) {
                    semio_framework_os_renderer_wgpu_ui_engine::scene_interaction::PresentedInputCandidateProgress::Ready => cursor.input_wait = AppPresentInputWait::None,
                    semio_framework_os_renderer_wgpu_ui_engine::scene_interaction::PresentedInputCandidateProgress::Pending { advanced } => {
                        cursor.input_wait = AppPresentInputWait::SceneIntent;
                        if advanced {
                            cursor.input_progress = cursor.input_progress.saturating_add(1);
                        }
                        return Ok(AppPresentStep::Pending);
                    }
                    semio_framework_os_renderer_wgpu_ui_engine::scene_interaction::PresentedInputCandidateProgress::Stale => {
                        drop(runtime);
                        cursor.frame.packet = self.gate.abort_pending();
                        cursor.witness = None;
                        cursor.phase = AppPresentPhase::Aborted;
                        return Ok(AppPresentStep::Pending);
                    }
                }
                if !shell.input.presented_input_candidate_matches(input_candidate) {
                    drop(runtime);
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.witness = None;
                    cursor.phase = AppPresentPhase::Aborted;
                    return Ok(AppPresentStep::Pending);
                }
                drop(runtime);
                let Some(witness) = cursor.witness.as_ref() else {
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err("prepared frame presenter witness was missing before submit".to_string());
                };
                let Some(packet) = self.gate.pending_presented(witness) else {
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.witness = None;
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err("prepared frame presenter witness was stale before submit".to_string());
                };
                let raster_witness = cursor.raster_witness.ok_or_else(|| "raster operation witness was missing before submit".to_string())?;
                if !self.raster_operation_authority.matches(raster_witness) {
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.witness = None;
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err("raster operation authority was stale before submit".to_string());
                }
                if cursor.gpu_cursor.is_none() {
                    match self.gpu.begin_prepared_present(packet, raster_witness) {
                        Ok(gpu_cursor) => cursor.gpu_cursor = Some(gpu_cursor),
                        Err(error) => {
                            cursor.frame.packet = self.gate.abort_pending();
                            cursor.witness = None;
                            cursor.phase = AppPresentPhase::Aborted;
                            return Err(format!("prepared frame submit admission: {error}"));
                        }
                    }
                    return Ok(AppPresentStep::Pending);
                }
                let Some(gpu_cursor) = cursor.gpu_cursor.as_mut() else { return Ok(AppPresentStep::Pending) };
                let outcome = self.gpu.prepared_present_step(packet, gpu_cursor);
                if let Some((key, version)) = self.gpu.take_missing_world_mesh() {
                    log_debug_once_per_transition("present-missing-world-mesh", true, &format!("[TRACE] os_host prepared world mesh was not resident, draw skipped: key={key} version={version}"));
                } else {
                    log_debug_once_per_transition("present-missing-world-mesh", false, "[TRACE] os_host prepared world mesh residency restored");
                }
                match outcome {
                    Ok(true) => cursor.phase = AppPresentPhase::CloseGpu,
                    Ok(false) => {}
                    Err(error) => {
                        gpu_cursor.begin_close();
                        cursor.frame.packet = self.gate.abort_pending();
                        cursor.witness = None;
                        cursor.phase = AppPresentPhase::Aborted;
                        return Err(format!("prepared frame submit step: {error}"));
                    }
                }
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::CloseGpu => {
                let Some(gpu_cursor) = cursor.gpu_cursor.as_mut() else {
                    cursor.phase = AppPresentPhase::Acknowledge;
                    return Ok(AppPresentStep::Pending);
                };
                gpu_cursor.begin_close();
                if gpu_cursor.close_step() {
                    cursor.gpu_cursor = None;
                    cursor.phase = AppPresentPhase::Acknowledge;
                }
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::Acknowledge => {
                let Some(witness) = cursor.witness.as_ref() else {
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err("prepared frame presenter witness was missing".to_string());
                };
                let Some(packet) = self.gate.pending_presented(&witness) else {
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err("prepared frame presenter witness was stale before acknowledgement".to_string());
                };
                let raster_witness = cursor.raster_witness.ok_or_else(|| "raster operation witness was missing before acknowledgement".to_string())?;
                if !self.raster_operation_authority.matches(raster_witness) || raster_witness.scene_revision != packet.scene_revision() || raster_witness.preview_generation != packet.preview_generation() {
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err("raster operation authority was stale before acknowledgement".to_string());
                }
                let runtime = self.runtime.clone();
                let Ok(mut runtime) = runtime.try_lock() else {
                    cursor.input_wait = AppPresentInputWait::RuntimeLock;
                    return Ok(AppPresentStep::RetryRuntime);
                };
                if runtime.interaction.is_none() {
                    drop(runtime);
                    match self.runtime.presenter_interaction_status() {
                        PresenterInteractionStep::Ready | PresenterInteractionStep::Restored => {
                            cursor.input_wait = AppPresentInputWait::None;
                            cursor.input_progress = cursor.input_progress.saturating_add(1);
                            return Ok(AppPresentStep::Pending);
                        }
                        PresenterInteractionStep::Waiting { site, request } => {
                            cursor.input_wait = AppPresentInputWait::InteractionCheckout { site, request };
                            return Ok(AppPresentStep::AwaitingRuntime);
                        }
                        PresenterInteractionStep::RuntimeLock => {
                            cursor.input_wait = AppPresentInputWait::RuntimeLock;
                            return Ok(AppPresentStep::RetryRuntime);
                        }
                        PresenterInteractionStep::Abandoned => {
                            cursor.frame.packet = self.gate.abort_pending();
                            cursor.phase = AppPresentPhase::Aborted;
                            return Err("presenter interaction checkout had no return owner before acknowledgement".to_string());
                        }
                    }
                }
                let Some(interaction) = runtime.interaction.as_mut() else { return Ok(AppPresentStep::AwaitingRuntime) };
                let Some(input_candidate) = cursor.frame.input_candidate else {
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err("prepared frame input candidate witness was missing before acknowledgement".to_string());
                };
                if !interaction.shell.input.presented_input_candidate_matches(input_candidate) {
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err("prepared frame input authority was stale before acknowledgement".to_string());
                }
                let AppInteractionState { shell, input, .. } = interaction;
                match shell.input.progress_presented_input_candidate(input, input_candidate) {
                    semio_framework_os_renderer_wgpu_ui_engine::scene_interaction::PresentedInputCandidateProgress::Ready => cursor.input_wait = AppPresentInputWait::None,
                    semio_framework_os_renderer_wgpu_ui_engine::scene_interaction::PresentedInputCandidateProgress::Pending { advanced } => {
                        cursor.input_wait = AppPresentInputWait::SceneIntent;
                        if advanced {
                            cursor.input_progress = cursor.input_progress.saturating_add(1);
                        }
                        return Ok(AppPresentStep::Pending);
                    }
                    semio_framework_os_renderer_wgpu_ui_engine::scene_interaction::PresentedInputCandidateProgress::Stale => {
                        cursor.frame.packet = self.gate.abort_pending();
                        cursor.phase = AppPresentPhase::Aborted;
                        return Err("prepared frame input authority was stale before acknowledgement".to_string());
                    }
                }
                if !semio_framework_os_renderer_wgpu_shell_view_panels::PanelsViewCtx::of(&mut shell).acknowledge_presented_input(input, input_candidate) {
                    return Ok(AppPresentStep::Pending);
                }
                cursor.has_animated_primitives = packet.has_animated_primitives();
                cursor.retained_control_deadline = shell.input.presented_retained_clock_deadline();
                if let Some(now_us) = semio_framework_job::default_now_us() {
                    cursor.shell_clock_deadline = shell.surfaces.next_chrome_deadline(now_us as f64 / 1_000_000.0, app_now_ms(), &shell.chrome, &shell.tutorial);
                }
                cursor.frame.input_candidate = None;
                let Some(witness) = cursor.witness.take() else { return Err("prepared frame presenter witness was lost during acknowledgement".to_string()) };
                let mut replacement = match self.gate.acknowledge_presented(witness) {
                    Ok(replacement) => replacement,
                    Err(_) => {
                        cursor.phase = AppPresentPhase::Aborted;
                        return Err("prepared frame presenter witness was stale or duplicated".to_string());
                    }
                };
                drop(runtime);
                self.raster_operation_authority.release(raster_witness).map_err(str::to_owned)?;
                cursor.raster_witness = None;
                self.retirement = Some(AppPresentedRetirement::commit(replacement.take_previous(), raster_witness));
                cursor.phase = AppPresentPhase::Media;
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::Media => Ok(AppPresentStep::Pending),
            AppPresentPhase::ProgressAcknowledge => {
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(progress) = cursor.frame.job_progress.as_mut() {
                    if !progress.acknowledge_presented() {
                        return Ok(AppPresentStep::Pending);
                    }
                }
                cursor.phase = AppPresentPhase::Fullscreen;
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::Directives => {
                if self.retirement.as_ref().is_some_and(|retirement|retirement.completed_frame.is_some()){return Ok(AppPresentStep::Pending)}
                if let Some(window) = self.window.as_ref() {
                    apply_window_cursor(window, cursor.frame.cursor, cursor.frame.theme_dark, &mut self.last_cursor);
                }
                let fullscreen = self.window.is_none().then_some(cursor.frame.fullscreen).flatten();
                let Some(mut completed) = self.pending.take() else { return Err("completed presentation cursor was missing".to_string()) };
                let accepted_cursor = completed.frame.cursor;
                let accepted_theme_dark = completed.frame.theme_dark;
                let accepted_generation = completed.frame.generation;
                let cursor_wake = completed.frame.cursor_wake.take();
                let retained_control_deadline = completed.retained_control_deadline;
                let shell_clock_deadline = completed.shell_clock_deadline;
                let has_animated_primitives = completed.has_animated_primitives;
                let retirement = self.retirement.get_or_insert_with(|| AppPresentedRetirement::new(None));
                if retirement.completed_frame.is_some() {
                    completed.frame.cursor_wake = cursor_wake;
                    self.pending = Some(completed);
                    return Err("completed presentation retirement capacity exhausted".to_string());
                }
                let media_slots = std::mem::take(&mut completed.media_slots);
                retirement.completed_frame = Some(completed.frame);
                Ok(AppPresentStep::Complete { generation: accepted_generation, cursor: accepted_cursor, theme_dark: accepted_theme_dark, fullscreen, cursor_wake, retained_control_deadline, shell_clock_deadline, has_animated_primitives, media_slots })
            }
        }
    }

}
//#endregion 📮️RuntimeMailbox
