//! 🌐️ Native asset pumps: the native reference decode pump, the native asset handoff and transport and the native asset pump. See [`super`].

use super::*;

//#region 📮️RuntimeMailbox
impl<R: MailboxRuntime> RuntimeMailbox<R> {
    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn pump_native_reference_decode(&self, probe: &mut RendererAssetProbe, surface_token: AdmittedSurfaceToken, token: WorldAssetRequestToken, url: String) -> RendererAssetDecodeTurnKind {
        let job = {
            let mut slot = self.0.native_reference_decode.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            if let Some(job) = slot.as_ref() {
                if job.token != token || job.url != url {
                    self.record_frame_fault("native reference decode owner changed before terminal handback");
                    return RendererAssetDecodeTurnKind::Pending;
                }
                job.clone()
            } else {
                let bytes = match collect_world3d_asset_bytes(match probe.owner_mut() {
                    RendererAssetFetchOwner::World { owner, .. } | RendererAssetFetchOwner::Shared(owner) => owner,
                }) {
                    Ok(bytes) => bytes,
                    Err(_) => {
                        probe.begin_close();
                        return RendererAssetDecodeTurnKind::Pending;
                    }
                };
                let job = Arc::new(NativeReferenceDecodeJob::new(token, url.clone(), bytes));
                *slot = Some(job.clone());
                job
            }
        };
        if self.renderer_asset_cancelled(probe.owner()) {
            job.cancel();
        }
        match job.phase.load(Ordering::Acquire) {
            0 => {
                let scheduled = job.try_schedule(self.clone());
                if !scheduled {
                    if let Some(waker) = self.0.waker.lock().expect("runtime completion waker lock").as_ref() {
                        waker();
                    }
                }
                RendererAssetDecodeTurnKind::Pending
            }
            1 => RendererAssetDecodeTurnKind::Pending,
            2 => {
                let Some(output) = job.take_output() else { return RendererAssetDecodeTurnKind::Pending };
                let output = match output {
                    NativeReferenceDecodeOutput::Waiting(decoded) => {
                        let _ = self.0.scene_raster_pool.maintenance_step();
                        if job.rearm(decoded) {
                            if let Some(waker) = self.0.waker.lock().expect("runtime completion waker lock").as_ref() {
                                waker();
                            }
                            return RendererAssetDecodeTurnKind::Pending;
                        }
                        if self.close_native_reference_decode_step(Some(token)) {
                            probe.begin_close();
                            return RendererAssetDecodeTurnKind::Cancelled;
                        }
                        return RendererAssetDecodeTurnKind::Pending;
                    }
                    output => output,
                };
                let Ok(mut runtime) = self.try_lock() else {
                    job.restore_output(output);
                    return RendererAssetDecodeTurnKind::Pending;
                };
                let Some(interaction) = runtime.interaction_mut() else {
                    job.restore_output(output);
                    return RendererAssetDecodeTurnKind::Pending;
                };
                let Some(state) = interaction.shell.surfaces.world3d_states.get_token_mut(surface_token) else {
                    drop(runtime);
                    job.restore_output(output);
                    job.cancel();
                    if self.close_native_reference_decode_step(Some(token)) {
                        probe.begin_close();
                        return RendererAssetDecodeTurnKind::Cancelled;
                    }
                    return RendererAssetDecodeTurnKind::Pending;
                };
                let cancelled = job.cancelled.load(Ordering::Acquire);
                let stale = !world3d_reference_url_is_current(&state.scene, &url);
                if cancelled || stale {
                    if stale && !cancelled {
                        mark_world3d_asset_miss(&mut state.assets, &url);
                    }
                    drop(runtime);
                    job.restore_output(output);
                    job.cancel();
                    if self.close_native_reference_decode_step(Some(token)) {
                        probe.begin_close();
                        return RendererAssetDecodeTurnKind::Cancelled;
                    }
                    return RendererAssetDecodeTurnKind::Pending;
                }
                match output {
                    NativeReferenceDecodeOutput::Ready(lease) => {
                        let _ = apply_decoded_reference_raster(&mut state.session, &mut state.authority, &mut state.scene, &mut state.assets, &url, lease);
                    }
                    NativeReferenceDecodeOutput::Failed => mark_world3d_asset_miss(&mut state.assets, &url),
                    NativeReferenceDecodeOutput::Waiting(_) => unreachable!("waiting decode was rearmed above"),
                }
                drop(runtime);
                job.phase.store(4, Ordering::Release);
                let mut slot = self.0.native_reference_decode.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
                if !slot.as_ref().is_some_and(|candidate| Arc::ptr_eq(candidate, &job)) {
                    drop(slot);
                    self.record_frame_fault("native reference decode owner changed before publication handback");
                    return RendererAssetDecodeTurnKind::Fault;
                }
                *slot = None;
                probe.begin_close();
                RendererAssetDecodeTurnKind::Published
            }
            3 | 4 => {
                if self.close_native_reference_decode_step(Some(token)) {
                    probe.begin_close();
                    RendererAssetDecodeTurnKind::Cancelled
                } else {
                    RendererAssetDecodeTurnKind::Pending
                }
            }
            5 => RendererAssetDecodeTurnKind::Pending,
            6 => {
                if matches!(job.retire_payload_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), NativeReferenceDecodeCloseStep::Complete) {
                    let next = if job.cancelled.load(Ordering::Acquire) { 3 } else { 2 };
                    let _ = job.phase.compare_exchange(6, next, Ordering::AcqRel, Ordering::Acquire);
                }
                if let Some(waker) = self.0.waker.lock().expect("runtime completion waker lock").as_ref() {
                    waker();
                }
                RendererAssetDecodeTurnKind::Pending
            }
            _ => {
                self.record_frame_fault("native reference decode entered an invalid phase");
                RendererAssetDecodeTurnKind::Fault
            }
        }
    }

    pub fn pump_native_asset_handoff_step(&self, closing: bool) -> Option<bool> {
        let Ok(mut slot) = self.0.native_asset_handoff.try_lock() else { return Some(false) };
        let handoff = slot.as_mut()?;
        if closing {
            handoff.fetch.as_mut().expect("native handoff owns its request").begin_close();
            handoff.seal = false;
        }
        if let Some(payload) = handoff.payload.as_mut() {
            let _ = payload.close_step(semio_framework_job::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: semio_framework_job::JOB_PAYLOAD_PAGE_BYTES, maximum_capacity_bytes: 0, maximum_release_bytes: 2 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES, maximum_depth: 3 });
            if payload.terminal_is_empty() {
                handoff.payload = None;
            }
            return Some(true);
        }
        if handoff.reject {
            let fetch = handoff.fetch.as_mut().expect("native handoff owns its rejected request");
            if !self.reject_renderer_asset(fetch) {
                return Some(false);
            }
            fetch.begin_close();
            handoff.reject = false;
            return Some(true);
        }
        if handoff.seal {
            let fetch = handoff.fetch.as_mut().expect("native handoff owns its request");
            match self.renderer_asset_current(fetch) {
                None => return Some(false),
                Some(false) => {
                    fetch.begin_close();
                    handoff.seal = false;
                }
                Some(true) => match self.seal_renderer_asset_response(fetch) {
                    RendererAssetSealStep::Busy => return Some(false),
                    RendererAssetSealStep::Granted => handoff.seal = false,
                    RendererAssetSealStep::Refused(_) => {
                        fetch.begin_close();
                        handoff.seal = false;
                    }
                },
            }
            return Some(true);
        }
        let fetch = handoff.fetch.take().expect("native handoff owns its request");
        match self.return_renderer_asset_owner(fetch) {
            Ok(()) => {
                *slot = None;
                Some(true)
            }
            Err(fetch) => {
                handoff.fetch = Some(fetch);
                Some(false)
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn retain_native_asset_handoff(&self, fetch: RendererAssetFetchOwner, payload: Option<semio_framework_job::RetainedJobPayload>, seal: bool, reject: bool) {
        let mut slot = self.0.native_asset_handoff.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        assert!(slot.is_none(), "one native asset fetch is admitted at a time");
        *slot = Some(NativeAssetHandoff { fetch: Some(fetch), payload, seal, reject });
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn retain_native_asset_transport(&self, fetch: &RendererAssetFetchOwner, cancel: semio_framework_async::CancelToken, cancellation: semio_framework_os_services::HttpBodyCancellationHandle) -> NativeAssetTransportGuard<R> {
        let surface_token = match fetch {
            RendererAssetFetchOwner::World { surface: _, surface_token, owner: _ } => Some(*surface_token),
            RendererAssetFetchOwner::Shared(_) => None,
        };
        let request_token = fetch.owner().token();
        let mut slot = self.0.native_asset_transport.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        assert!(slot.is_none(), "one native asset transport is admitted at a time");
        *slot = Some(NativeAssetTransportLease { surface_token, request_token, cancel, cancellation, cancellation_step: None });
        NativeAssetTransportGuard { mailbox: self.clone(), surface_token, request_token, armed: true }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn cancel_native_asset_transport_step(&self, surface_token: Option<AdmittedSurfaceToken>, request_token: Option<WorldAssetRequestToken>) -> NativeAssetTransportCloseStep {
        let cancellation = {
            let Ok(mut slot) = self.0.native_asset_transport.try_lock() else { return NativeAssetTransportCloseStep::Busy };
            let Some(lease) = slot.as_mut() else { return NativeAssetTransportCloseStep::Terminal };
            if surface_token.is_some_and(|expected| lease.surface_token != Some(expected)) || request_token.is_some_and(|expected| lease.request_token != expected) {
                return NativeAssetTransportCloseStep::Terminal;
            }
            lease.cancel.cancel_now();
            if let Some(step) = lease.cancellation_step.as_ref() {
                return NativeAssetTransportCloseStep::Pending(step.clone());
            }
            lease.cancellation_step = Some(semio_framework_os_services::HttpBodyCancellationStep::Idle);
            lease.cancellation.clone()
        };
        let step = cancellation.cancel_in_flight();
        let mut slot = self.0.native_asset_transport.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(lease) = slot.as_mut().filter(|lease| !surface_token.is_some_and(|expected| lease.surface_token != Some(expected)) && !request_token.is_some_and(|expected| lease.request_token != expected)) {
            lease.cancellation_step = Some(step.clone());
        }
        NativeAssetTransportCloseStep::Pending(step)
    }

    #[cfg(all(any(test, feature = "testkit"), not(target_arch = "wasm32")))]
    pub fn native_asset_transport_live_count(&self) -> usize {
        usize::from(self.0.native_asset_transport.lock().expect("native asset transport slot").is_some())
    }

    #[cfg(all(any(test, feature = "testkit"), not(target_arch = "wasm32")))]
    pub fn wait_native_asset_page_test_barrier(&self, fetch: &RendererAssetFetchOwner) {
        let RendererAssetFetchOwner::World { surface_token, .. } = fetch else { return };
        let barrier = self.0.native_asset_page_barrier.lock().expect("native asset page test barrier").as_ref().filter(|barrier| barrier.surface_token == *surface_token).cloned();
        let Some(barrier) = barrier else { return };
        barrier.entered.store(true, Ordering::Release);
        while !barrier.release.load(Ordering::Acquire) {
            std::thread::yield_now();
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn pump_native_asset(&self) -> bool {
        if let Some(progressed) = self.pump_native_asset_handoff_step(false) {
            if progressed {
                if let Some(waker) = self.0.waker.lock().expect("runtime completion waker lock").as_ref() {
                    waker();
                }
            }
            return progressed;
        }
        if self.0.native_asset_fetching.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return false;
        }
        let Some(mut fetch) = self.take_renderer_asset_step() else {
            self.0.native_asset_fetching.store(false, Ordering::Release);
            return false;
        };
        match self.reserve_renderer_asset_response(&mut fetch, WORLD_ASSET_RESPONSE_BYTE_CAPACITY) {
            RendererAssetSealStep::Granted => {}
            admission => {
                if matches!(admission, RendererAssetSealStep::Refused(_)) {
                    fetch.begin_close();
                }
                self.retain_native_asset_handoff(fetch, None, false, false);
                self.0.native_asset_fetching.store(false, Ordering::Release);
                if let Some(waker) = self.0.waker.lock().expect("runtime completion waker lock").as_ref() {
                    waker();
                }
                return true;
            }
        }
        let mailbox = self.clone();
        spawn_app_task(async move {
            let (payload, seal, reject) = match stream_native_renderer_asset(&mailbox, &mut fetch).await {
                Ok(()) => (None, true, false),
                Err(NativeAssetStreamFailure::Cancelled) => {
                    fetch.begin_close();
                    (None, false, false)
                }
                Err(NativeAssetStreamFailure::CancelledPage { payload }) => {
                    fetch.begin_close();
                    (Some(payload), false, false)
                }
                Err(NativeAssetStreamFailure::Unavailable(detail)) => {
                    log_debug(&format!("native renderer asset unavailable: {detail}"));
                    (None, false, true)
                }
                Err(NativeAssetStreamFailure::Fault(detail)) => {
                    fetch.begin_close();
                    log_debug(&format!("native renderer asset fetch failed: {detail}"));
                    mailbox.record_frame_fault("native renderer asset fetch failed");
                    (None, false, false)
                }
                Err(NativeAssetStreamFailure::Page { detail, payload }) => {
                    fetch.begin_close();
                    mailbox.record_frame_fault(detail);
                    (Some(payload), false, false)
                }
            };
            mailbox.retain_native_asset_handoff(fetch, payload, seal, reject);
            mailbox.0.native_asset_fetching.store(false, Ordering::Release);
            if let Some(waker) = mailbox.0.waker.lock().expect("runtime completion waker lock").as_ref() {
                waker();
            }
        });
        true
    }

}
//#endregion 📮️RuntimeMailbox
