//! 📬️ Kernel client: the kernel future, the global client and its request methods.
use super::*;

//#region 🎠️KernelRuntime
//#region 🔖️KernelFuture — the leaf `Future` every `ProgramBridgeEntry` async method awaits
/// 🌉 The genuinely-yielding leaf every plugin call now awaits, replacing the old in-process
/// `WasmPluginRuntime::exchange` blocking call. The renderer worker-pool and app-task drivers
/// supply its `Waker`; this future only stores and wakes the most recent one.
pub struct KernelFuture {
    pub slot: Arc<ResponseSlot>,
    pub request: Option<KernelRequest>,
    pub queue: Arc<KernelRequestQueue>,
    pub finished: bool,
}

impl Drop for KernelFuture {
    fn drop(&mut self) {
        if !self.finished && self.request.is_none() {
            self.slot.abandoned.store(true, std::sync::atomic::Ordering::Release);
            self.queue.wake_consumer();
        }
    }
}

impl Future for KernelFuture {
    type Output = KernelOutcome;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if let Some(request) = this.request.take() {
            if let Err((request, slot)) = this.queue.try_push(request, this.slot.clone(), Some(cx.waker())) {
                drop(slot);
                this.request = Some(request);
                return Poll::Pending;
            }
        }
        let waker = cx.waker().clone();
        let mut state = this.slot.state.lock().expect("response slot lock");
        if let Some(outcome) = state.result.take() {
            let previous = state.waker.take();
            drop(state);
            drop(previous);
            this.finished = true;
            return Poll::Ready(outcome);
        }
        let previous = state.waker.replace(waker);
        drop(state);
        drop(previous);
        Poll::Pending
    }
}

fn persistent_command_completion_port_ready() -> bool {
    KERNEL_REQUEST_QUEUE_CAPACITY != 0 && semio_framework::kernel::COMMAND_MAXIMUM_PAGES != 0 && semio_framework::kernel::COMMAND_MAXIMUM_BYTES >= semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES
}
//#endregion

//#region 🔖️KernelClient
#[derive(Clone)]
pub struct KernelClient {
    pub queue: Arc<KernelRequestQueue>,
    _task: Arc<KernelPoolFuture>,
    close_submissions: Arc<KernelCloseSubmissionRegistry>,
}

fn global_client() -> &'static OnceLock<KernelClient> {
    static CLIENT: OnceLock<KernelClient> = OnceLock::new();
    &CLIENT
}

impl KernelClient {
    /// ▶️ Mounts the kernel request state machine on the process-wide worker pool exactly once.
    pub fn get() -> KernelClient {
        global_client()
            .get_or_init(|| {
                let queue = Arc::new(KernelRequestQueue::default());
                let task = KernelPoolFuture::spawn(crate::renderer_worker_pool(), semio_framework_async::Lane::Interactive, run_kernel_pool(queue.clone(),semio_framework_job::default_now_us));
                install_job_progress_acknowledge(|token| KernelClient::get().try_acknowledge_job_progress(token));
                KernelClient { queue, _task: task, close_submissions: Arc::new(KernelCloseSubmissionRegistry::new()) }
            })
            .clone()
    }

    fn submit(&self, request: KernelRequest) -> KernelFuture {
        KernelFuture { slot: Arc::new(ResponseSlot::default()), request: Some(request), queue: self.queue.clone(), finished: false }
    }

    pub fn try_acknowledge_job_progress(&self, token: JobProgressPresentationToken) -> bool {
        self.queue.try_push(KernelRequest::AcknowledgeJobProgress { token }, Arc::new(ResponseSlot::default()), None).is_ok()
    }

    /// 🐣️ Compiles `wasm_path` (once per content hash), activates `app_id` on it and opens its first
    /// instance. `artifact_schema` is the document schema the app edits, empty for an app that edits
    /// none: the mounted component becomes that kind's codec in this process
    /// ([`semio_framework_os_kernel::os_store::codec_registry::register_component_document_codec`]).
    pub async fn create_app(&self, wasm_path: PathBuf, plugin_id: String, app_id: String, artifact_schema: String) -> Result<u32, ProgramFault> {
        match self.submit(KernelRequest::CreateApp { owner: CreateAppRequestOwner::new(wasm_path, plugin_id, app_id, artifact_schema) }).await {
            KernelOutcome::Created(result) => result,
            KernelOutcome::Exchanged(_) => Err("kernel: unexpected Exchanged response for create_app".into()),
            KernelOutcome::ProductReplayMounted(_) => Err("kernel: unexpected product replay response for create_app".into()),
        }
    }

    pub fn begin_destroy_app(&self, instance: u32) -> Result<KernelCloseHandle, String> {
        let reservation = reserve_seq()?;
        let owner = Arc::new(KernelCloseSubmission {
            instance,
            realm: false,
            generation: reservation.generation(),
            queue: self.queue.clone(),
            pool: crate::renderer_worker_pool(),
            registry: Arc::downgrade(&self.close_submissions),
            phase: std::sync::atomic::AtomicU8::new(KERNEL_CLOSE_UNADMITTED),
        });
        reservation.commit()?;
        let handle = KernelCloseHandle { owner };
        let _ = handle.poll();
        Ok(handle)
    }

    pub fn begin_close_realm(&self) -> Result<KernelCloseHandle, String> {
        let reservation = reserve_seq()?;
        let owner = Arc::new(KernelCloseSubmission {
            instance: u32::MAX,
            realm: true,
            generation: reservation.generation(),
            queue: self.queue.clone(),
            pool: crate::renderer_worker_pool(),
            registry: Arc::downgrade(&self.close_submissions),
            phase: std::sync::atomic::AtomicU8::new(KERNEL_CLOSE_UNADMITTED),
        });
        reservation.commit()?;
        let handle = KernelCloseHandle { owner };
        let _ = handle.poll();
        Ok(handle)
    }

    pub fn destroy_app(&self, instance: u32) {
        let Ok(handle) = self.begin_destroy_app(instance) else { return };
        match handle.poll() {
            KernelCloseStatus::Pending | KernelCloseStatus::Complete => {}
            KernelCloseStatus::AdmissionBlocked | KernelCloseStatus::Fault => {
                panic!("kernel destroy close authority was not admitted; ProgramBridge must retain and poll begin_destroy_app")
            }
        }
    }

    pub async fn exchange_commands(&self, instance: u32, commands: Vec<protocol::AppCommand>) -> Result<ExchangeOutcome, ProgramFault> {
        if !persistent_command_completion_port_ready() {
            return Err(("persistent command completion submit/poll/cancel authority is not admitted".to_string()).into());
        }
        let mut envelopes = semio_framework::kernel::CommandEnvelopeSet::try_new().map_err(|fault| fault.describe())?;
        for command in commands {
            let seq = next_seq()?;
            let command = protocol::encode_app_command(&command).await.map_err(|fault| fault.describe())?;
            if let Err((fault, rejected)) = envelopes.try_push(semio_framework::kernel::CommandEnvelope { instance, seq, command }) {
                self.queue.enqueue_retained(KernelRequest::CloseRejectedCommandBuild { key: u64::from(instance), owner: semio_framework::kernel::RejectedCommandBuild::new(envelopes, rejected) }, Arc::new(ResponseSlot::default())).await;
                return Err((fault.describe()).into());
            }
        }
        let reservation = reserve_seq()?;
        let generation = reservation.generation();
        let batch = match semio_framework::kernel::CommandBatch::try_new(generation, envelopes) {
            Ok(batch) => {
                reservation.commit()?;
                batch
            }
            Err((fault, owners)) => {
                self.queue.enqueue_retained(KernelRequest::CloseRejectedCommandBuild { key: u64::from(instance), owner: semio_framework::kernel::RejectedCommandBuild::from_admitted(owners) }, Arc::new(ResponseSlot::default())).await;
                return Err((fault.describe()).into());
            }
        };
        let driver = semio_framework::kernel::CommandBatchDriver::new(generation, batch);
        match self.submit(KernelRequest::ExchangeCommands { instance, driver }).await {
            KernelOutcome::Exchanged(result) => result,
            KernelOutcome::Created(_) => Err("kernel: unexpected Created response for command exchange".into()),
            KernelOutcome::ProductReplayMounted(_) => Err("kernel: unexpected product replay response for command exchange".into()),
        }
    }

    pub async fn exchange_events(&self, instance: u32, events: Vec<Event>) -> Result<ExchangeOutcome, ProgramFault> {
        let event = match QueuedKernelEvent::try_from_events(events) {
            Ok(event) => event,
            Err(owner) => {
                self.queue.enqueue_retained(KernelRequest::CloseRejectedEvents { owner }, Arc::new(ResponseSlot::default())).await;
                return Err(("only one bounded SurfaceVisible or document-backbone Message event is admitted per kernel request turn".to_string()).into());
            }
        };
        match self.submit(KernelRequest::Exchange { instance, event }).await {
            KernelOutcome::Exchanged(result) => result,
            KernelOutcome::Created(_) => Err("kernel: unexpected Created response for exchange".into()),
            KernelOutcome::ProductReplayMounted(_) => Err("kernel: unexpected product replay response for exchange".into()),
        }
    }

    pub async fn advance_retained(&self, instance: u32, surface: SurfaceId) -> Result<ExchangeOutcome, ProgramFault> {
        match self.submit(KernelRequest::AdvanceRetained { instance, surface }).await {
            KernelOutcome::Exchanged(result) => result,
            KernelOutcome::Created(_) => Err("kernel: unexpected Created response for retained advance".into()),
            KernelOutcome::ProductReplayMounted(_) => Err("kernel: unexpected product replay response for retained advance".into()),
        }
    }

    pub async fn mount_product_replay(&self, owner: MountedProductReplayRequest) -> Result<(), MountedProductReplayRequest> {
        match self.submit(KernelRequest::MountProductReplay { owner }).await {
            KernelOutcome::ProductReplayMounted(result) => result,
            KernelOutcome::Created(_) | KernelOutcome::Exchanged(_) => unreachable!("mounted product replay request has one response class"),
        }
    }

    pub fn reserve_product_replay_admission(&self, instance: u32) -> Result<MountedProductReplayAdmissionPermit, String> {
        let token = mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").reserve(instance).ok_or_else(|| "mounted product replay refusal capacity is full before exchange".to_string())?;
        Ok(MountedProductReplayAdmissionPermit { token: Some(token) })
    }

    pub async fn retire_product_replay(&self, owner: MountedProductReplayRequest) {
        match self.submit(KernelRequest::RetireProductReplay { owner }).await {
            KernelOutcome::ProductReplayMounted(Ok(())) => {}
            KernelOutcome::ProductReplayMounted(Err(owner)) => owner.retire(),
            KernelOutcome::Created(_) | KernelOutcome::Exchanged(_) => unreachable!("product replay retirement has one response class"),
        }
    }

    pub async fn retire_product_replay_refusal(&self, owner: RefusedProductReplay) {
        match self.submit(KernelRequest::RetireProductReplayRefusal { owner }).await {
            KernelOutcome::ProductReplayMounted(Ok(())) => {}
            KernelOutcome::ProductReplayMounted(Err(_)) => unreachable!("typed product refusal has no raw request response"),
            KernelOutcome::Created(_) | KernelOutcome::Exchanged(_) => unreachable!("product replay refusal retirement has one response class"),
        }
    }

    pub async fn advance_product_replay(&self, instance: u32) -> Result<(), ProgramFault> {
        match self.submit(KernelRequest::AdvanceProductReplay { instance }).await {
            KernelOutcome::Exchanged(result) => result.map(|_| ()),
            KernelOutcome::Created(_) => Err("kernel: unexpected Created response for product replay advance".into()),
            KernelOutcome::ProductReplayMounted(_) => Err("kernel: unexpected mount response for product replay advance".into()),
        }
    }
}
//#endregion
//#endregion 🎠️KernelRuntime
