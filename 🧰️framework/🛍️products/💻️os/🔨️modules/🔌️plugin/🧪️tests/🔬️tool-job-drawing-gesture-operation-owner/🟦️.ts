import { toolJobDrawingGestureOperationOwnerExact } from "../../../../../../../📜️script.ts";

/** 🧪️ Executes tool job drawing gesture operation owner policy assertions. */
export function toolJobDrawingGestureOperationOwnerSelfTests(): number {
  const editor = `
struct DrawingGestureOperationOwner { base_revision: String }
struct DrawingInstanceOperationOwner { operations: FixedOperationRegistry<DrawingGestureOperationOwner, 64> }
struct DrawingGestureOperationJob;
impl ToolJobFactory for DrawingGestureOperationJobFactory { type Job = DrawingGestureOperationJob; }
impl ArtifactOwnedToolJobFactory for DrawingGestureOperationJobFactory { type Owner = semio_framework_plugin::EditorApp<DrawingPlayApp>; }
impl semio_framework_job::FixedOperationOwner for DrawingGestureOperationOwner {
  fn cancel(&mut self) {}
  fn terminal_is_empty(&self) -> bool { true }
}
impl DrawingInstanceOperationOwner {
  fn new() -> Self { todo!() }
  fn preview_projection(&mut self, canonical_base_revision: [u8; 32], active_utility: &str) { owner.preview_projection(operation.canonical_base_revision, &cfg.snapshot.active_utility_id); }
  fn dispatch(&mut self) {
    if observed_revision != base_revision {}
    let mut decoder = DrawingRetainedCommandDecoder; decoder.feed(*byte);
    let work: UiFixedList<TracePointerWork, TRACE_POINTER_WORK_CAPACITY>;
    struct DrawingDraftQuery; query.advance(snapshot); TracePointerJob::new_marquee();
    enum DrawingQueryPublication {} query.publication_step(); interaction_select_effect_from_targets();
    session.trace_pointer; session.point_query; session.draft_query;
    DrawingCommand::CanvasPointerDown(payload) => canvas_pointer_down::handle;
    if DRAWING_GESTURE_TOOL_IDS.contains(&command.command_id()) { "Drawing gesture commands are reachable only through their exact retained factory owner"; }
    if session.gesture.matches("idle") && session.trace_pointer.is_none() {}
  }
}
impl ArtifactEditor for DrawingPlayApp {
  fn build_instance_operation_owner() { DrawingInstanceOperationOwner::new(); }
  fn register_tool_job_factories(registry: &mut Registry) { registry.register(DrawingGestureOperationJobFactory::new(&controller)); }
  fn payload(request: Request) { instance_owner: request.instance_operation_owner; }
  async fn render_with_instance_operation_owner() { DrawingGesturePreview; DRAWING_GESTURE_PREVIEW_POINT_CAPACITY; }
}
`;
  const framework = `
trait ArtifactApp { fn build_instance_operation_owner() -> Box<dyn ArtifactInstanceOperationOwner>; }
struct VcsArtifactApp { instance_operation_owner: ArtifactInstanceOperationOwnerHandle }
struct Request { instance_operation_owner: request.instance_operation_owner }
fn render() { A::render_with_request_context(&self.instance_operation_owner); }
fn mount() { MountedWorkerJobSession::try_new(); }
`;
  const config = "struct DrawingConfig { locale: String } enum DrawingConfigMutation { SetLocale(String) }";
  const proto = "message DrawingConfig { string locale = 1; }";
  if (!toolJobDrawingGestureOperationOwnerExact(editor, config, proto, framework)) throw new Error("[verify interactivity tool-jobs] valid Drawing transient operation owner was rejected.");
  const hostile = [
    [editor.replace("struct DrawingGestureOperationOwner", "struct DrawingGestureOwner"), config, proto, framework],
    [editor.replace("type Owner = semio_framework_plugin::EditorApp<DrawingPlayApp>", "type Owner = WrongApp"), config, proto, framework],
    [editor.replace("if observed_revision != base_revision", "if active != key"), config, proto, framework],
    [editor.replace("owner.preview_projection(operation.canonical_base_revision, &cfg.snapshot.active_utility_id)", "DrawingGesturePreview::default()"), config, proto, framework],
    [editor, `${config} struct DrawingConfigCheckpoint { gesture_checkpoint_json: String }`, proto, framework],
    [editor, `${config} enum DrawingConfigMutation { SetGestureCheckpoint { json: String } }`, proto, framework],
    [editor, config, `${proto} message DrawingDraft { string gesture_session = 2; }`, framework],
    [`${editor}\nstatic DRAWING_SESSIONS: OnceLock<Mutex<Map>> = OnceLock::new();`, config, proto, framework],
    [`${editor}\nfn checkpoint_from_config() {}`, config, proto, framework],
    [`${editor}\nfn decode() { serde_json::from_slice(bytes); }`, config, proto, framework],
    [`${editor}\nfn dispatch() { command.dispatch(&doc); }`, config, proto, framework],
    [`${editor}\nfn decode() { let raw_bytes: Vec<u8> = Vec::new(); }`, config, proto, framework],
    [`${editor}\nfn pick() { flatten_drawing_layers(&doc.layers); }`, config, proto, framework],
    [`${editor}\nfn interaction_targets_json() {}`, config, proto, framework],
    [editor.replace("if DRAWING_GESTURE_TOOL_IDS.contains(&command.command_id())", "if false"), config, proto, framework],
    [`${editor}\nstatic TRACE_POINTER_JOBS: OnceLock<Map> = OnceLock::new();`, config, proto, framework],
    [editor, config, proto, framework.replace("A::render_with_request_context(&self.instance_operation_owner", "A::render")],
  ] as const;
  for (const [hostileEditor, hostileConfig, hostileProto, hostileFramework] of hostile) {
    if (toolJobDrawingGestureOperationOwnerExact(hostileEditor, hostileConfig, hostileProto, hostileFramework)) throw new Error("[verify interactivity tool-jobs] Drawing persisted-operation hostile fixture was falsely accepted.");
  }
  return hostile.length + 1;
}
