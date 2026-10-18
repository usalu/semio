/** 🔭️ The universal retained session ownership and the layout export and cold relay retained ownership checks. */
import { toolJobUniversalRetainedOwnershipExact, toolJobLayoutColdRelayRetainedExact } from "../../../../../../🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🔁️live-replay/🧊️retained-ownership/🟦️.ts";

/** 🌐 Checks the job, native I/O, wgpu and animate retained session ownership and every single-edit mutation of them. */
export function universalRetainedOwnershipChecks() {
  const universalJob = `
pub trait InteractiveJob: Send { fn step(&mut self); fn begin_close(&mut self); fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize); fn terminal_is_empty(&self) -> bool; }
struct ExactJob;
impl InteractiveJob for ExactJob { fn step(&mut self) {} fn begin_close(&mut self) {} fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) {} fn terminal_is_empty(&self) -> bool { true } }
struct WorkerJobAuthority { preadmitted_fault: Option<RetainedJobPayload> }
fn admission() { preadmitted_static_payload(&payload_ledger, stream, bytes, source); authority.preadmitted_fault.take().expect("worker panic retains its pre-admitted terminal fault page"); }
fn worker_panic_and_quiet_wake_publish_one_durable_terminal_intent() { assert_eq!(returned_fault_pointer, preadmitted_fault_pointer); }
fn checked_out_and_worker_begin_close_transitions_report_exact_zero_release() { assert_eq!(batch.close_step(1, JOB_PAYLOAD_PAGE_BYTES), WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 }); assert_eq!(batch.close_step(1, JOB_PAYLOAD_PAGE_BYTES), WorkerJobCloseStep::Pending { released_items: 0, released_bytes: 0 }); }
fn child_counter() { "child begin-close transfers control without claiming an owner release"; }
pub struct WorkerJobSessionAdmissionRejected { fault_source: ManuallyDrop<Option<JobPayloadPageSource>> }
impl WorkerJobSession { pub fn try_step_on_caller(&self) {} }
impl BatchJobSession { pub fn try_new(job: J, params: BatchJobParams) -> Result<Self, WorkerJobSessionAdmissionRejected<J>> {} }
`;
  const universalNative = `
const NATIVE_IO_PATH_CAPACITY: usize = 256;
pub struct NativePathSet { entries: std::mem::ManuallyDrop<[Option<PathBuf>; NATIVE_IO_PATH_CAPACITY]> } pub struct NativeModifiedSet { entries: std::mem::ManuallyDrop<[Option<(PathBuf, std::time::SystemTime)>; NATIVE_IO_PATH_CAPACITY]> }
enum NativeIoState { ReadingModified { paths: NativePathSet, modified: NativeModifiedSet }, ClosingScanFault, ClosingModifiedFault }
impl NativeIoJob { pub fn retained_request_backing_identity(&self) -> Option<*const u8> {} }
fn path_set_max_plus_one_identity_zero_grant_and_job_close_are_exact() { assert_eq!(returned.as_os_str().as_encoded_bytes().as_ptr(), plus_one_pointer); }
`;
  const universalWgpu = `
const RENDERER_IO_SESSION_SLOTS: usize = semio_framework_job::WORKER_JOB_SESSION_SLOTS + 1;
struct Slot { generation: std::sync::atomic::AtomicU64, cancel_requested: std::sync::atomic::AtomicBool }
struct Prepared { session: Option<semio_framework_job::BatchJobSession<ui_wgpu::wgpu::PreparedRenderJob>>, rejected: Option<semio_framework_job::WorkerJobSessionAdmissionRejected<ui_wgpu::wgpu::PreparedRenderJob>> }
struct FrameBuildHandle { session: Option<semio_framework_job::WorkerJobSession<ActiveFrameBuild>>, rejected: Option<semio_framework_job::WorkerJobSessionAdmissionRejected<ActiveFrameBuild>> }
fn take_prepared() { checked_out_job_mut()?.take_packet(); }
fn renderer_io_with_node() { slot.generation.load(std::sync::atomic::Ordering::Acquire) != generation; }
fn pump_renderer_io_sessions(maximum_sessions: usize) {}
fn mounted_registry_max_plus_one_zero_pump_drop_and_generation_are_exact() { assert_eq!(returned_pointer, plus_one_request_pointer); assert_eq!(pump_renderer_io_sessions(0), 0); assert_eq!(pump_renderer_io_sessions(1), 1, "one host turn advances one mounted control opportunity"); }
impl std::future::Future for RendererIoHandle { fn poll(&mut self, cx: &mut Context) { renderer_io_with_node(); node.waker = Some(cx.waker().clone()); } }
impl Presenter { pub(crate) fn present_step(&mut self) { pump_renderer_io_sessions(1); pump_worker_job_retirements(1, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES); } }
`;
  const universalAnimate = `
pub struct PresentationEnvelopeMaterializeJob { fault_writer: std::mem::ManuallyDrop<Option<semio_framework_job::RetainedJobPayloadWriter>>, retained_nested_outcome: std::mem::ManuallyDrop<Option<semio_framework_job::StepOutcome>> }
impl InteractiveJob for PresentationEnvelopeMaterializeJob { fn step(&mut self) {} fn begin_close(&mut self) {} fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) {} fn terminal_is_empty(&self) -> bool { true } }
pub struct PresentationEnvelopeMaterializeHandle {
  rejected: std::mem::ManuallyDrop<Option<semio_framework_job::WorkerJobSessionAdmissionRejected<PresentationEnvelopeMaterializeJob>>>,
  pending: Option<semio_framework_job::WorkerJobTicket>,
  retained_outcome: std::mem::ManuallyDrop<Option<semio_framework_job::StepOutcome>>,
}
fn submit_materialize_presentation_projection() { WorkerJobSession::try_new(job, params); }
fn retained_presentation_envelope_caller_faults_and_zero_grant_closes_malformed_pack() { registry.close_step(operation, generation, &pool, 0, 0); }
fn retained_presentation_envelope_caller_cancels_and_zero_grant_closes_without_output() { registry.close_step(operation, generation, &pool, 0, 0); }
`;
  const universal = (job = universalJob, nativeIo = universalNative, wgpu = universalWgpu, animate = universalAnimate) => toolJobUniversalRetainedOwnershipExact(job, nativeIo, wgpu, new Map([["job.rs", job], ["native_io.rs", nativeIo], ["wgpu.rs", wgpu], ["animate.rs", animate]]));
  if (!universal()) throw new Error("[verify interactivity tool-jobs] self-test universal-retained-ownership-valid was falsely rejected.");
  if (universal(universalJob.replace("fn begin_close(&mut self) {}", "fn close_now(&mut self) {}"))) throw new Error("[verify interactivity tool-jobs] self-test universal-job-missing-mandatory-close was falsely accepted.");
  if (universal(`${universalJob}\nWorkerJobSession::new(job, params);`)) throw new Error("[verify interactivity tool-jobs] self-test universal-legacy-session-constructor was falsely accepted.");
  if (universal(`${universalJob}\nsemio_framework_job::run_to_completion(job);`)) throw new Error("[verify interactivity tool-jobs] self-test universal-legacy-terminal-drain was falsely accepted.");
  if (universal(`${universalJob}\nsemio_framework_job::drive_step(job, params);`)) throw new Error("[verify interactivity tool-jobs] self-test universal-direct-drive-bypasses-retained-session was falsely accepted.");
  if (universal(`${universalJob}\nuse semio_framework_job::{drive_step, StepBudget};\nfn bypass() { drive_step(job, params); }`)) throw new Error("[verify interactivity tool-jobs] self-test universal-imported-direct-drive-bypasses-retained-session was falsely accepted.");
  if (universal(`${universalJob}\nfn bypass() { job::drive_step(job, params); }`)) throw new Error("[verify interactivity tool-jobs] self-test universal-aliased-direct-drive-bypasses-retained-session was falsely accepted.");
  if (universal(`${universalJob}\nstruct FrameBuildHandle { runtime_in_flight: Option<Receiver<RuntimeFrameResult>> }`)) throw new Error("[verify interactivity tool-jobs] self-test universal-frame-async-receiver-bypasses-retained-session was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu.replace("rejected: Option<semio_framework_job::WorkerJobSessionAdmissionRejected<ActiveFrameBuild>>", "rejected: Option<String>"))) throw new Error("[verify interactivity tool-jobs] self-test universal-frame-admission-rejection-owner-missing was falsely accepted.");
  if (universal(universalJob.replace("preadmitted_static_payload(&payload_ledger", "retained_static_payload(&payload_ledger"))) throw new Error("[verify interactivity tool-jobs] self-test universal-post-work-fault-allocation was falsely accepted.");
  if (universal(universalJob.replace("assert_eq!(returned_fault_pointer, preadmitted_fault_pointer);", "assert_eq!(fault.detail.len(), 26);"))) throw new Error("[verify interactivity tool-jobs] self-test universal-preadmitted-fault-identity-proof-missing was falsely accepted.");
  if (universal(universalJob.replace("released_items: 0, released_bytes: 0 }); assert_eq!(batch.close_step", "released_items: 1, released_bytes: 0 }); assert_eq!(batch.close_step"))) throw new Error("[verify interactivity tool-jobs] self-test universal-begin-close-exact-counter-proof-missing was falsely accepted.");
  if (universal(universalJob, `${universalNative}\nenum Bad { Paths(Vec<PathBuf>) }`)) throw new Error("[verify interactivity tool-jobs] self-test native-I/O-resizable-path-output was falsely accepted.");
  if (universal(universalJob, `${universalNative}\nenum Bad { WriteBytes { bytes: Vec<u8> } }`)) throw new Error("[verify interactivity tool-jobs] self-test native-I/O-whole-write-buffer was falsely accepted.");
  if (universal(universalJob, universalNative.replace("std::mem::ManuallyDrop<[Option<PathBuf>; NATIVE_IO_PATH_CAPACITY]>", "[Option<PathBuf>; NATIVE_IO_PATH_CAPACITY]"))) throw new Error("[verify interactivity tool-jobs] self-test native-I/O-path-set-deep-drop-was falsely accepted.");
  if (universal(universalJob, universalNative.replace("assert_eq!(returned.as_os_str().as_encoded_bytes().as_ptr(), plus_one_pointer);", "assert_eq!(returned, expected_path);"))) throw new Error("[verify interactivity tool-jobs] self-test native-I/O-plus-one-path-identity-proof-missing was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu.replace("node.waker = Some(cx.waker().clone());", "node.waker = Some(cx.waker().clone()); session.try_submit_step();"))) throw new Error("[verify interactivity tool-jobs] self-test renderer-future-advances-worker-session was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu.replace("pump_renderer_io_sessions(1);", ""))) throw new Error("[verify interactivity tool-jobs] self-test renderer-host-pump-missing was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu.replace("slot.generation.load(std::sync::atomic::Ordering::Acquire) != generation;", "true;"))) throw new Error("[verify interactivity tool-jobs] self-test renderer-generation-recheck-missing was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu.replace("assert_eq!(returned_pointer, plus_one_request_pointer);", "assert!(returned_pointer.is_null());"))) throw new Error("[verify interactivity tool-jobs] self-test renderer-plus-one-request-identity-proof-missing was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu.replace("assert_eq!(pump_renderer_io_sessions(0), 0);", "pump_renderer_io_sessions(1);"))) throw new Error("[verify interactivity tool-jobs] self-test renderer-zero-grant-proof-missing was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu.replace("rejected: Option<semio_framework_job::WorkerJobSessionAdmissionRejected<ui_wgpu::wgpu::PreparedRenderJob>>", "rejected: Option<String>"))) throw new Error("[verify interactivity tool-jobs] self-test renderer-prepared-admission-rejection-owner-missing was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu.replace("checked_out_job_mut()?.take_packet()", "job.take_packet()"))) throw new Error("[verify interactivity tool-jobs] self-test renderer-prepared-packet-bypasses-checked-out-owner was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu, universalAnimate.replace("WorkerJobSession::try_new(job, params)", "WorkerJobSession::new(job, params)"))) throw new Error("[verify interactivity tool-jobs] self-test Animate-mounted-session-fallible-admission-missing was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu, universalAnimate.replace("pending: Option<semio_framework_job::WorkerJobTicket>", "pending: Option<semio_framework_async::oneshot::Receiver<semio_framework_job::StepOutcome>>"))) throw new Error("[verify interactivity tool-jobs] self-test Animate-mounted-session-ticket-owner-replaced-by-receiver was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu, universalAnimate.replace("retained_nested_outcome: std::mem::ManuallyDrop<Option<semio_framework_job::StepOutcome>>", "discard_nested_outcome: bool"))) throw new Error("[verify interactivity tool-jobs] self-test Animate-nested-output-close-owner-missing was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu, universalAnimate.replace("fault_writer: std::mem::ManuallyDrop<Option<semio_framework_job::RetainedJobPayloadWriter>>", "fault: Option<Vec<u8>>"))) throw new Error("[verify interactivity tool-jobs] self-test Animate-retained-fault-writer-replaced-by-Vec was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu, universalAnimate.replace("retained_presentation_envelope_caller_faults_and_zero_grant_closes_malformed_pack", "retained_presentation_envelope_caller_fault_drops_owner"))) throw new Error("[verify interactivity tool-jobs] self-test Animate-fault-zero-grant-fixture-missing was falsely accepted.");
  if (universal(universalJob, universalNative, universalWgpu, universalAnimate.replace("retained_presentation_envelope_caller_cancels_and_zero_grant_closes_without_output", "retained_presentation_envelope_caller_cancel_drops_owner"))) throw new Error("[verify interactivity tool-jobs] self-test Animate-cancel-zero-grant-fixture-missing was falsely accepted.");
}

/** 📡 Checks the layout export, layout wasm and guest cold relay retained ownership. */
export function layoutAndColdRelayRetainedChecks() {
  const retainedLayoutExport = `
pub struct LayoutExportJob; struct LayoutExportPublication { bytes: [u8; MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES], writer: Option<RetainedJobPayloadWriter> }
fn encode_checkpoint() -> Result<[u8; MAX_LAYOUT_EXPORT_CHECKPOINT_BYTES], String> {}
fn layout_retained_publication_zero_grant_and_exact_writer_close_are_exact() {}
fn begin_close(&mut self) { self.close_stage = LayoutExportCloseStage::Publication; }
fn close_step(&mut self) {} fn terminal_is_empty(&self) -> bool {}
snapshot_placeholder: Option<Arc<LayoutSnapshot>>
`;
  const retainedLayoutWasm = `
pub struct LayoutExportOperation; const LAYOUT_EXPORT_REJECTION_SLOTS: usize = 8;
enum Slot { ClosingSession { session: WorkerJobSession<LayoutExportJob>, snapshot_owner: Arc<LayoutSnapshot> } }
fn submit() { WorkerJobSession::try_new(job, params); session.try_step_on_caller(); session.take_outcome(ticket); session.take_terminal(); owner.resume(); owner.begin_close(); outcome.close_step(1, JOB_PAYLOAD_PAGE_BYTES); retain_layout_export_session(); retain_layout_export_rejection(); }
`;
  const retainedColdRelay = `
enum GuestRelayCompletion { Stepped(Result<GuestRelayStepCompletion, TurnFault>), Rejected(GuestRelayOwnedBytes), Fault(GuestRelayOwnedBytes) }
enum GuestRelayStepCompletion { Running { progress: Option<GuestRelayOwnedBytes> }, Done { output: GuestRelayOwnedBytes }, Failed { error: GuestRelayOwnedBytes } }
struct GuestRelayCompletionSender { slot: Arc<GuestRelayCompletionSlot> }
struct GuestColdRelayJob; struct GuestRelayPublication { writer: Option<semio_framework_job::RetainedJobPayloadWriter> }
const GUEST_RELAY_MOUNTED_SLOTS: usize = 16;
struct GuestRelayMountedOutput { storage: Box<[std::mem::MaybeUninit<u8>; semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES]> }
struct GuestRelayMountedRegistry { next_generation: std::sync::atomic::AtomicU64 }
fn reserve() { match generation { u64::MAX => Some(0), generation => Some(generation + 1) }; }
fn guest_cold_relay_registry_max_plus_one_generation_and_zero_pump_are_exact() { assert_eq!(output.storage.as_ptr(), output_identity); }
fn guest_cold_relay_publication_max_plus_one_selects_retained_fault_without_losing_source() {}
impl std::future::Future for GuestRelayMountedFuture { fn poll() { self.registry.pump(self.index, self.generation, context.waker()); } }
fn begin_close(&mut self) {} fn terminal_is_empty(&self) -> bool {}
async fn run_job_on_worker() { WorkerJobSession::try_new(relay, params); self.relay_registry.mount(index, mounted_generation, owner); }
`;
  if (!toolJobLayoutColdRelayRetainedExact(retainedLayoutExport, retainedLayoutWasm, retainedColdRelay)) throw new Error("[verify interactivity tool-jobs] self-test Layout-and-cold-relay-retained-valid was falsely rejected.");
  if (toolJobLayoutColdRelayRetainedExact(retainedLayoutExport.replace("writer: Option<RetainedJobPayloadWriter>", "bytes: Vec<u8>"), retainedLayoutWasm, retainedColdRelay)) throw new Error("[verify interactivity tool-jobs] self-test Layout-retained-publication-writer-removal was falsely accepted.");
  if (toolJobLayoutColdRelayRetainedExact(retainedLayoutExport, retainedLayoutWasm.replace("WorkerJobSession::try_new(job, params)", "WorkerJobSession::new(job, params)"), retainedColdRelay)) throw new Error("[verify interactivity tool-jobs] self-test Layout-Wasm-fallible-admission-removal was falsely accepted.");
  if (toolJobLayoutColdRelayRetainedExact(retainedLayoutExport, retainedLayoutWasm.replace("retain_layout_export_session();", "drop(session);"), retainedColdRelay)) throw new Error("[verify interactivity tool-jobs] self-test Layout-Wasm-live-drop-retirement-removal was falsely accepted.");
  if (toolJobLayoutColdRelayRetainedExact(retainedLayoutExport, retainedLayoutWasm, retainedColdRelay.replace("slot: Arc<GuestRelayCompletionSlot>", "receiver: oneshot::Receiver<GuestRelayCompletion>"))) throw new Error("[verify interactivity tool-jobs] self-test cold-relay-oneshot-restoration was falsely accepted.");
  if (toolJobLayoutColdRelayRetainedExact(retainedLayoutExport, retainedLayoutWasm, retainedColdRelay.replace("Rejected(GuestRelayOwnedBytes)", "Rejected(Vec<u8>)"))) throw new Error("[verify interactivity tool-jobs] self-test cold-relay-direct-Vec-completion-restoration was falsely accepted.");
  if (toolJobLayoutColdRelayRetainedExact(retainedLayoutExport, retainedLayoutWasm, retainedColdRelay.replace("Stepped(Result<GuestRelayStepCompletion, TurnFault>)", "Stepped(Result<JobStep, TurnFault>)"))) throw new Error("[verify interactivity tool-jobs] self-test cold-relay-indirect-Vec-step-completion-restoration was falsely accepted.");
  if (toolJobLayoutColdRelayRetainedExact(retainedLayoutExport, retainedLayoutWasm, retainedColdRelay.replace("Box<[std::mem::MaybeUninit<u8>; semio_framework_job::JOB_PAYLOAD_OPERATION_BYTES]>", "Vec<u8>"))) throw new Error("[verify interactivity tool-jobs] self-test cold-relay-mounted-output-backing-resized was falsely accepted.");
  if (toolJobLayoutColdRelayRetainedExact(retainedLayoutExport, retainedLayoutWasm, retainedColdRelay.replace("u64::MAX => Some(0)", "generation => Some(generation.wrapping_add(1))"))) throw new Error("[verify interactivity tool-jobs] self-test cold-relay-mounted-generation-wrap-restoration was falsely accepted.");
  if (toolJobLayoutColdRelayRetainedExact(retainedLayoutExport, retainedLayoutWasm, retainedColdRelay.replace("assert_eq!(output.storage.as_ptr(), output_identity);", "assert_eq!(generation, generation);"))) throw new Error("[verify interactivity tool-jobs] self-test cold-relay-mounted-output-identity-proof-removal was falsely accepted.");
  if (toolJobLayoutColdRelayRetainedExact(retainedLayoutExport, retainedLayoutWasm, retainedColdRelay.replace("const GUEST_RELAY_MOUNTED_SLOTS: usize = 16", "slots: Vec<GuestRelayMountedSession>"))) throw new Error("[verify interactivity tool-jobs] self-test cold-relay-fixed-mounted-registry-removal was falsely accepted.");
  if (toolJobLayoutColdRelayRetainedExact(retainedLayoutExport, retainedLayoutWasm, retainedColdRelay.replace("self.relay_registry.mount(index, mounted_generation, owner);", "loop { session.step().await; }"))) throw new Error("[verify interactivity tool-jobs] self-test cold-relay-run-loop-restoration was falsely accepted.");
  if (toolJobLayoutColdRelayRetainedExact(retainedLayoutExport, retainedLayoutWasm, retainedColdRelay.replace("guest_cold_relay_registry_max_plus_one_generation_and_zero_pump_are_exact", "guest_cold_relay_registry_smoke"))) throw new Error("[verify interactivity tool-jobs] self-test cold-relay-max-plus-one-generation-fixture-removal was falsely accepted.");
}
