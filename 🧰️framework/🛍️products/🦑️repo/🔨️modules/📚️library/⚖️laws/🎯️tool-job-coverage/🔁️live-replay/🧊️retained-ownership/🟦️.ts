import { toolJobProductionProtocolSource, toolJobRustBlock } from "../../📋️inventory/🟦️.ts";

export function toolJobLayoutColdRelayRetainedExact(layoutExport: string, layoutWasm: string, pluginHost: string): boolean {
  const relayCompletionStart = pluginHost.indexOf("enum GuestRelayCompletion");
  const relayCompletion = relayCompletionStart < 0 ? undefined : toolJobRustBlock(pluginHost, pluginHost.indexOf("{", relayCompletionStart));
  const relaySenderStart = pluginHost.indexOf("struct GuestRelayCompletionSender");
  const relaySender = relaySenderStart < 0 ? undefined : toolJobRustBlock(pluginHost, pluginHost.indexOf("{", relaySenderStart));
  const relayRunStart = pluginHost.indexOf("async fn run_job_on_worker");
  const relayRun = relayRunStart < 0 ? undefined : toolJobRustBlock(pluginHost, pluginHost.indexOf("{", relayRunStart));
  return (
    layoutExport.includes("struct LayoutExportPublication") &&
    layoutExport.includes("bytes: [u8; MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES]") &&
    layoutExport.includes("writer: Option<RetainedJobPayloadWriter>") &&
    layoutExport.includes("fn encode_checkpoint(") &&
    layoutExport.includes("Result<[u8; MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES], String>") &&
    layoutExport.includes("fn layout_retained_publication_zero_grant_and_exact_writer_close_are_exact()") &&
    layoutExport.includes("fn begin_close(&mut self)") &&
    layoutExport.includes("self.close_stage = LayoutExportCloseStage::Publication") &&
    layoutExport.includes("snapshot_placeholder: Option<Arc<LayoutSnapshot>>") &&
    layoutWasm.includes("WorkerJobSession::try_new(job, params)") &&
    layoutWasm.includes("const LAYOUT_EXPORT_REJECTION_SLOTS: usize = 8") &&
    layoutWasm.includes("ClosingSession { session: WorkerJobSession<LayoutExportJob>, snapshot_owner: Arc<LayoutSnapshot> }") &&
    layoutWasm.includes("session.try_step_on_caller()") &&
    layoutWasm.includes("session.take_outcome(ticket)") &&
    layoutWasm.includes("session.take_terminal()") &&
    layoutWasm.includes("owner.resume()") &&
    layoutWasm.includes("owner.begin_close()") &&
    layoutWasm.includes("outcome.close_step(1, JOB_PAYLOAD_PAGE_BYTES)") &&
    layoutWasm.includes("retain_layout_export_session(") &&
    layoutWasm.includes("retain_layout_export_rejection(") &&
    !layoutWasm.includes("session.step(&self.pool") &&
    relayCompletion !== undefined &&
    relayCompletion.body.includes("Stepped(Result<GuestRelayStepCompletion, TurnFault>)") &&
    relayCompletion.body.includes("Rejected(GuestRelayOwnedBytes)") &&
    relayCompletion.body.includes("Fault(GuestRelayOwnedBytes)") &&
    !relayCompletion.body.includes("Result<JobStep") &&
    !relayCompletion.body.includes("Rejected(Vec<u8>)") &&
    !relayCompletion.body.includes("Fault(Vec<u8>)") &&
    pluginHost.includes("Running { progress: Option<GuestRelayOwnedBytes> }") &&
    pluginHost.includes("Done { output: GuestRelayOwnedBytes }") &&
    pluginHost.includes("Failed { error: GuestRelayOwnedBytes }") &&
    relaySender !== undefined &&
    relaySender.body.includes("slot: Arc<GuestRelayCompletionSlot>") &&
    !relaySender.body.includes("oneshot") &&
    pluginHost.includes("struct GuestRelayPublication") &&
    pluginHost.includes("writer: Option<semio_framework_job::RetainedJobPayloadWriter>") &&
    pluginHost.includes("const GUEST_RELAY_MOUNTED_SLOTS: usize = 16") &&
    pluginHost.includes("struct GuestRelayMountedRegistry") &&
    pluginHost.includes("struct GuestRelayMountedOutput") &&
    pluginHost.includes("Box<[std::mem::MaybeUninit<u8>; semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES]>") &&
    !pluginHost.includes("terminal_bytes: Vec<u8>") &&
    pluginHost.includes("next_generation: std::sync::atomic::AtomicU64") &&
    pluginHost.includes("u64::MAX => Some(0)") &&
    pluginHost.includes("fn guest_cold_relay_registry_max_plus_one_generation_and_zero_pump_are_exact()") &&
    pluginHost.includes("assert_eq!(output.storage.as_ptr(), output_identity)") &&
    pluginHost.includes("fn guest_cold_relay_publication_max_plus_one_selects_retained_fault_without_losing_source()") &&
    pluginHost.includes("impl std::future::Future for GuestRelayMountedFuture") &&
    pluginHost.includes("self.registry.pump(self.index, self.generation, context.waker())") &&
    pluginHost.includes("fn begin_close(&mut self)") &&
    pluginHost.includes("fn terminal_is_empty(&self) -> bool") &&
    relayRun !== undefined &&
    relayRun.body.includes("WorkerJobSession::try_new(relay, params)") &&
    relayRun.body.includes("self.relay_registry.mount(index, mounted_generation, owner)") &&
    !relayRun.body.includes("loop {") &&
    !relayRun.body.includes("session.step(")
  );
}

export function toolJobUniversalRetainedOwnershipExact(jobRuntime: string, nativeIo: string, wgpu: string, rustFiles: ReadonlyMap<string, string>): boolean {
  const mandatoryClose = [...rustFiles.entries()].every(([, raw]) => {
    const source = toolJobProductionProtocolSource(raw);
    for (const match of source.matchAll(/\bInteractiveJob\s+for\s+[^\{]+\{/g)) {
      const block = toolJobRustBlock(source, source.indexOf("{", match.index));
      if (!block || !block.body.includes("fn begin_close(") || !block.body.includes("fn close_step(") || !block.body.includes("fn terminal_is_empty(")) return false;
    }
    return true;
  });
  const production = [...rustFiles.values()].map(toolJobProductionProtocolSource).join("\n");
  const layoutExport = [...rustFiles.values()].find((source) => source.includes("pub struct LayoutExportJob"));
  const layoutWasm = [...rustFiles.values()].find((source) => source.includes("pub struct LayoutExportOperation"));
  const pluginHost = [...rustFiles.values()].find((source) => source.includes("struct GuestColdRelayJob"));
  const layoutColdRelay = layoutExport === undefined && layoutWasm === undefined && pluginHost === undefined ? true : layoutExport !== undefined && layoutWasm !== undefined && pluginHost !== undefined && toolJobLayoutColdRelayRetainedExact(layoutExport, layoutWasm, pluginHost);
  const animateRaw = [...rustFiles.values()].find((source) => source.includes("pub struct PresentationEnvelopeMaterializeHandle"));
  const animate = animateRaw === undefined ? undefined : toolJobProductionProtocolSource(animateRaw);
  const futureStart = wgpu.indexOf("impl std::future::Future for RendererIoHandle");
  const future = futureStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", futureStart));
  const presenterStart = wgpu.indexOf("pub(crate) fn present_step(&mut self)");
  const presenter = presenterStart < 0 ? undefined : toolJobRustBlock(wgpu, wgpu.indexOf("{", presenterStart));
  const closeCounterStart = jobRuntime.indexOf("fn checked_out_and_worker_begin_close_transitions_report_exact_zero_release()");
  const closeCounter = closeCounterStart < 0 ? undefined : toolJobRustBlock(jobRuntime, jobRuntime.indexOf("{", closeCounterStart));
  return (
    mandatoryClose &&
    layoutColdRelay &&
    !production.includes("WorkerJobSession::new") &&
    !/semio_framework_job::run_(?:to_completion|on_worker|on_worker_async)\s*\(/.test(production) &&
    !/semio_framework_job::drive_step\s*\(/.test(production) &&
    !/use\s+semio_framework_job::\{[^}]*\bdrive_step\b/s.test(production) &&
    !/\bjob::drive_step\s*\(/.test(production) &&
    !production.includes("Receiver<RuntimeFrameResult>") &&
    production.includes("session: Option<semio_framework_job::WorkerJobSession<ActiveFrameBuild>>") &&
    production.includes("rejected: Option<semio_framework_job::WorkerJobSessionAdmissionRejected<ActiveFrameBuild>>") &&
    jobRuntime.includes("pub trait InteractiveJob: Send") &&
    jobRuntime.includes("fn begin_close(&mut self);") &&
    jobRuntime.includes("fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize)") &&
    jobRuntime.includes("fn terminal_is_empty(&self) -> bool;") &&
    jobRuntime.includes("preadmitted_fault: Option<RetainedJobPayload>") &&
    jobRuntime.includes("preadmitted_static_payload(&payload_ledger") &&
    jobRuntime.includes('authority.preadmitted_fault.take().expect("worker panic retains its pre-admitted terminal fault page")') &&
    jobRuntime.includes("fn worker_panic_and_quiet_wake_publish_one_durable_terminal_intent()") &&
    jobRuntime.includes("assert_eq!(returned_fault_pointer, preadmitted_fault_pointer)") &&
    closeCounter !== undefined &&
    (closeCounter.body.match(/batch\.close_step\(1, JOB_PAYLOAD_PAGE_BYTES\)[\s\S]{0,160}?released_items:\s*0/g)?.length ?? 0) >= 2 &&
    jobRuntime.includes("child begin-close transfers control without claiming an owner release") &&
    !jobRuntime.includes("fn retained_static_payload(") &&
    jobRuntime.includes("pub fn try_step_on_caller(&self)") &&
    jobRuntime.includes("pub fn try_new(job: J, params: BatchJobParams) -> Result<Self, WorkerJobSessionAdmissionRejected<J>>") &&
    jobRuntime.includes("fault_source: ManuallyDrop<Option<JobPayloadPageSource>>") &&
    nativeIo.includes("pub struct NativePathSet") &&
    nativeIo.includes("pub struct NativeModifiedSet") &&
    nativeIo.includes("entries: std::mem::ManuallyDrop<[Option<PathBuf>; NATIVE_IO_PATH_CAPACITY]>") &&
    nativeIo.includes("entries: std::mem::ManuallyDrop<[Option<(PathBuf, std::time::SystemTime)>; NATIVE_IO_PATH_CAPACITY]>") &&
    nativeIo.includes("fn path_set_max_plus_one_identity_zero_grant_and_job_close_are_exact()") &&
    nativeIo.includes("assert_eq!(returned.as_os_str().as_encoded_bytes().as_ptr(), plus_one_pointer)") &&
    nativeIo.includes("ReadingModified { paths: NativePathSet, modified: NativeModifiedSet }") &&
    nativeIo.includes("ClosingScanFault") &&
    nativeIo.includes("ClosingModifiedFault") &&
    nativeIo.includes("pub fn retained_request_backing_identity(&self) -> Option<*const u8>") &&
    !nativeIo.includes("Paths(Vec<PathBuf>)") &&
    !nativeIo.includes("Modified(Vec<") &&
    !nativeIo.includes("WriteBytes") &&
    wgpu.includes("const RENDERER_IO_SESSION_SLOTS: usize = semio_framework_job::WORKER_JOB_SESSION_SLOTS + 1") &&
    wgpu.includes("generation: std::sync::atomic::AtomicU64") &&
    wgpu.includes("cancel_requested: std::sync::atomic::AtomicBool") &&
    wgpu.includes("fn pump_renderer_io_sessions(maximum_sessions: usize)") &&
    wgpu.includes("fn mounted_registry_max_plus_one_zero_pump_drop_and_generation_are_exact()") &&
    wgpu.includes("assert_eq!(returned_pointer, plus_one_request_pointer)") &&
    wgpu.includes("assert_eq!(pump_renderer_io_sessions(0), 0)") &&
    wgpu.includes('assert_eq!(pump_renderer_io_sessions(1), 1, "one host turn advances one mounted control opportunity")') &&
    wgpu.includes("session: Option<semio_framework_job::BatchJobSession<ui_wgpu::wgpu::PreparedRenderJob>>") &&
    wgpu.includes("rejected: Option<semio_framework_job::WorkerJobSessionAdmissionRejected<ui_wgpu::wgpu::PreparedRenderJob>>") &&
    wgpu.includes("checked_out_job_mut()?.take_packet()") &&
    wgpu.includes("slot.generation.load(std::sync::atomic::Ordering::Acquire) != generation") &&
    future !== undefined &&
    future.body.includes("renderer_io_with_node") &&
    future.body.includes("node.waker = Some(cx.waker().clone())") &&
    !future.body.includes("try_submit_step") &&
    !future.body.includes("close_step(") &&
    !future.body.includes("pump_one(") &&
    presenter !== undefined &&
    presenter.body.includes("pump_renderer_io_sessions(1)") &&
    presenter.body.includes("pump_worker_job_retirements(1, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)") &&
    animate !== undefined &&
    animate.includes("rejected: std::mem::ManuallyDrop<Option<semio_framework_job::WorkerJobSessionAdmissionRejected<PresentationEnvelopeMaterializeJob>>>") &&
    animate.includes("pending: Option<semio_framework_job::WorkerJobTicket>") &&
    animate.includes("retained_outcome: std::mem::ManuallyDrop<Option<semio_framework_job::StepOutcome>>") &&
    animate.includes("fault_writer: std::mem::ManuallyDrop<Option<semio_framework_job::RetainedJobPayloadWriter>>") &&
    animate.includes("retained_nested_outcome: std::mem::ManuallyDrop<Option<semio_framework_job::StepOutcome>>") &&
    animate.includes("WorkerJobSession::try_new(job, params)") &&
    animateRaw !== undefined &&
    animateRaw.includes("retained_presentation_envelope_caller_faults_and_zero_grant_closes_malformed_pack") &&
    animateRaw.includes("retained_presentation_envelope_caller_cancels_and_zero_grant_closes_without_output") &&
    animateRaw.includes("registry.close_step(operation, generation, &pool, 0, 0)") &&
    !animate.includes("oneshot::Receiver")
  );
}
