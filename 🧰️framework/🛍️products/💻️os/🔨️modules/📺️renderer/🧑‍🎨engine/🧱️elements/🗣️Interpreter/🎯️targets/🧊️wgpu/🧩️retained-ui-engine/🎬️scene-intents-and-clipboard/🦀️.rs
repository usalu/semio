//! 🎬️ Scene intents and clipboard io: the scene intent queue, the caret application and the mounted clipboard io of the retained UI engine; the ui-engine-focus crate of the renderer partition (plan section 5.2).
use super::*;

//#region 🧩️RetainedUiEngine
const SCENE_INTENT_CAPACITY: usize = 256;

pub const SCENE_INTENT_ID_BYTES: usize = 256;

/// 🖱️ One queued scene pointer/wheel event. `modifiers` rides all four variants because React's own
/// surface handlers read `shiftKey`/`ctrlKey`/`metaKey`/`altKey` off the DOM pointer event itself —
/// the merge mode a press selects is a property of THAT press, not of a separately sampled keyboard
/// state (`marqueeModeFromModifiers`, `🖱️ui/🎯️targets/⚛️react/🟦️.tsx:900`).
#[derive(Clone, Copy)]
pub enum SceneIntentEvent {
    PointerLeave,
    PointerDown { x: f32, y: f32, button: i16, modifiers: semio_ui_wgpu_events::EventModifiers },
    PointerUp { x: f32, y: f32, button: i16, modifiers: semio_ui_wgpu_events::EventModifiers },
    PointerMove { x: f32, y: f32, modifiers: semio_ui_wgpu_events::EventModifiers },
    Scroll { x: f32, y: f32, delta_y: f32, modifiers: semio_ui_wgpu_events::EventModifiers },
}

pub struct SceneInteractionIntent {
    pub generation: u64,
    pub tree_revision: u64,
    pub surface_generation: u64,
    pub pointer_id: Option<ui_render::PointerId>,
    pub window_id: String,
    pub host_id: String,
    pub node: NodeId,
    pub kind: semio_ui_declaration::ui::SurfaceKind,
    pub rect: Rect,
    pub event: SceneIntentEvent,
    pub driver_drag: semio_ui_wgpu_layout::chrome::UiDriverDrag,
    pub ink_job: Option<semio_renderer_wgpu_scene_ink::InkInteractionJob>,
    pub retiring: bool,
}

pub struct SceneIntentQueue {
    pub slots: Box<[Option<SceneInteractionIntent>; SCENE_INTENT_CAPACITY]>,
    head: usize,
    pub len: usize,
    next_generation: u64,
    pub retiring: Option<SceneInteractionIntent>,
}

impl Default for SceneIntentQueue {
    fn default() -> Self {
        Self { slots: semio_framework_async_slots::boxed_fixed_slots(|| None), head: 0, len: 0, next_generation: 1, retiring: None }
    }
}

impl SceneIntentQueue {
    pub fn push(
        &mut self,
        tree_revision: u64,
        surface_generation: u64,
        pointer_id: Option<ui_render::PointerId>,
        window_id: &str,
        host_id: &str,
        node: NodeId,
        kind: semio_ui_declaration::ui::SurfaceKind,
        rect: Rect,
        event: SceneIntentEvent,
        driver_drag: semio_ui_wgpu_layout::chrome::UiDriverDrag,
    ) -> Result<(), semio_ui_action::BoundedActionFault> {
        if window_id.len() > SCENE_INTENT_ID_BYTES || host_id.len() > SCENE_INTENT_ID_BYTES {
            return Err(semio_ui_action::BoundedActionFault::StringCredits);
        }
        if self.len == SCENE_INTENT_CAPACITY {
            return Err(semio_ui_action::BoundedActionFault::ItemCredits);
        }
        let generation = self.next_generation;
        self.next_generation = self.next_generation.wrapping_add(1).max(1);
        let index = (self.head + self.len) % SCENE_INTENT_CAPACITY;
        self.slots[index] =
            Some(SceneInteractionIntent { generation, tree_revision, surface_generation, pointer_id, window_id: window_id.to_owned(), host_id: host_id.to_owned(), node, kind, rect, event, driver_drag, ink_job: None, retiring: false });
        self.len += 1;
        Ok(())
    }

    pub fn pop_front(&mut self) -> Option<SceneInteractionIntent> {
        if self.len == 0 {
            return None;
        }
        let intent = self.slots[self.head].take();
        self.head = (self.head + 1) % SCENE_INTENT_CAPACITY;
        self.len -= 1;
        intent
    }

    pub fn push_front(&mut self, intent: SceneInteractionIntent) {
        debug_assert!(self.len < SCENE_INTENT_CAPACITY);
        self.head = (self.head + SCENE_INTENT_CAPACITY - 1) % SCENE_INTENT_CAPACITY;
        self.slots[self.head] = Some(intent);
        self.len += 1;
    }

    pub fn close_step(&mut self) -> bool {
        if let Some(intent) = self.retiring.as_mut() {
            if !intent.close_step() {
                return false;
            }
            self.retiring = None;
            return false;
        }
        self.retiring = self.pop_front();
        self.retiring.is_none()
    }

    pub fn close_window_step(&mut self, window_id: &str, generation: u64) -> bool {
        if let Some(intent) = self.retiring.as_mut().filter(|intent| intent.window_id == window_id && intent.surface_generation == generation) {
            if intent.close_step() {
                self.retiring = None;
            }
            return true;
        }
        let Some(offset) = (0..self.len).find(|offset| self.slots[(self.head + offset) % SCENE_INTENT_CAPACITY].as_ref().is_some_and(|intent| intent.window_id == window_id && intent.surface_generation == generation)) else { return false };
        let index = (self.head + offset) % SCENE_INTENT_CAPACITY;
        let intent = self.slots[index].as_mut().expect("occupied window intent");
        intent.retiring = true;
        if intent.close_step() {
            self.slots[index] = None;
            for cursor in offset..self.len - 1 {
                let next = (self.head + cursor + 1) % SCENE_INTENT_CAPACITY;
                self.slots[(self.head + cursor) % SCENE_INTENT_CAPACITY] = self.slots[next].take();
            }
            self.len -= 1;
        }
        true
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0 && self.retiring.is_none()
    }

    pub fn blocks_presented_candidate(&self, engine: &semio_ui_wgpu_engine::Ui, witness: u64) -> bool {
        let blocks = |intent: &SceneInteractionIntent| {
            engine.candidate_is_sealed_for(&intent.window_id, witness) && engine.tree_revision(&intent.window_id) == Some(intent.tree_revision) && engine.surface_generation(&intent.window_id) == Some(intent.surface_generation)
        };
        self.retiring.as_ref().is_some_and(blocks) || (0..self.len).any(|offset| self.slots[(self.head + offset) % SCENE_INTENT_CAPACITY].as_ref().is_some_and(blocks))
    }

    pub fn cancel_pointer(&mut self, pointer_id: ui_render::PointerId, owners: &[ScenePointerOwner]) {
        for slot in self.slots.iter_mut().filter_map(Option::as_mut) {
            if slot.pointer_id == Some(pointer_id)
                && owners.iter().any(|owner| owner.target.window_id == slot.window_id && owner.target.host_id == slot.host_id && owner.target.window_generation == slot.surface_generation)
                && slot.ink_job.as_ref().is_none_or(|job| job.matches_pointer_owner(slot.surface_generation, pointer_id))
            {
                slot.retiring = true;
            }
        }
    }
}

impl SceneInteractionIntent {
    pub fn close_step(&mut self) -> bool {
        let Some(job) = self.ink_job.as_mut() else {
            return true;
        };
        if job.close_step() {
            self.ink_job = None;
            true
        } else {
            false
        }
    }
}

pub static SCENE_INTENTS: WorkerCell<SceneIntentQueue> = WorkerCell::new(Default::default);

pub fn apply_presented_scene_caret(window_id: &str, caret: semio_ui_wgpu_engine_surface::UiPresentedCaret) -> bool {
    if caret.source != semio_ui_wgpu_engine_surface::UiCaretSource::Scene {
        return false;
    }
    let scene = UI_ENGINE.with(|cell| {
        let engine = cell.borrow();
        if engine.surface_token(window_id) != Some(caret.surface) {
            return None;
        }
        let node = engine.presented_node_at(window_id, caret.surface, caret.document_id)?;
        let tree = engine.tree(window_id)?;
        let UiNode::ComponentScene(scene) = &tree.node(node)?.spec.0 else { return None };
        Some((scene.host_id.clone(), scene.surface_id.clone(), scene.component_kind))
    });
    scene.is_some_and(|(host_id, surface_id, kind)| semio_renderer_wgpu_engine_graph::node_graph::component_scene_set_caret_visible(&host_id, &surface_id, kind, caret.visible))
}

/// 📋️ Writes to the OS clipboard via `ui_wgpu::wgpu::host` — the one indirection this module's own tests
/// swap for a mock (`MOCK_CLIPBOARD_WRITES`), so `cargo test` never touches a real display/clipboard.
#[cfg(all(not(any(test, feature = "testkit")),target_arch="wasm32"))]
pub fn write_os_clipboard(text: &str, _input: &mut semio_ui_wgpu_input::InputState<ActionDescriptor>) {
    #[cfg(target_arch = "wasm32")]
    {
        let text = text.to_string();
        semio_framework_async::browser::spawn_local(async move {
            semio_ui_wgpu_hosting::host::clipboard_write_text(&text).await;
        });
    }
}

#[cfg(any(test, feature = "testkit"))]
pub fn write_os_clipboard(text: &str, _input: &mut semio_ui_wgpu_input::InputState<ActionDescriptor>) {
    MOCK_CLIPBOARD_WRITES.with(|cell| cell.borrow_mut().push(text.to_string()));
}

#[cfg(all(not(target_arch="wasm32"),not(any(test, feature = "testkit"))))]
struct ClipboardInvocationInput { original:semio_framework_job::OriginalCallerSourceLoan,source:semio_ui_wgpu_input::ClipboardIntentSource,recipient:semio_ui_wgpu_input::ClipboardIntentRecipient,slot:u8,generation:u64 }

#[cfg(all(not(target_arch="wasm32"),not(any(test, feature = "testkit"))))]
type ClipboardJob = semio_ui_wgpu_hosting::host::ClipboardIoJob;
#[cfg(all(not(target_arch="wasm32"),not(any(test, feature = "testkit"))))]
type ClipboardSession = semio_framework_job_worker::WorkerJobSession<ClipboardJob>;

#[cfg(all(not(target_arch="wasm32"),not(any(test, feature = "testkit"))))]
struct MountedClipboardIo {
    invocation:ClipboardInvocationInput,
    job:Option<ClipboardJob>,
    pub params:Option<semio_framework_job_worker_authority::BatchJobParams>,
    parameter_cancel:Option<semio_framework_async::CancelTokenRetirement>,
    admission:semio_framework_job_worker::WorkerJobSessionAdmission<ClipboardJob, ClipboardSession>,
    session:Option<ClipboardSession>,
    pub outcome:Option<semio_framework_job_worker::WorkerJobOutcome<ClipboardJob>>,
    pub copied:Option<String>,
    delivered:bool,
    closing:bool,
}

#[cfg(all(not(target_arch="wasm32"),not(any(test, feature = "testkit"))))]
struct MountedClipboardSlot { generation:u64,mounted:Option<MountedClipboardIo>,pending_source:Option<semio_framework_job::OriginalCallerSourceLoan> }

#[cfg(all(not(target_arch="wasm32"),not(any(test, feature = "testkit"))))]
static MOUNTED_CLIPBOARD_IO:WorkerCell<[MountedClipboardSlot;32]>=WorkerCell::new(||std::array::from_fn(|_|MountedClipboardSlot{generation:0,mounted:None,pending_source:None}));

#[cfg(all(not(target_arch="wasm32"),not(any(test, feature = "testkit"))))]
fn clipboard_refusal(message:&'static str)->semio_framework_job::OriginalTurnFailure {semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,message).into()}

#[cfg(all(not(target_arch="wasm32"),not(any(test, feature = "testkit"))))]
pub fn validate_clipboard_source(source:semio_ui_wgpu_input::ClipboardIntentSource,cx:&semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{if source.operation!=cx.operation()||source.generation!=cx.generation()||!cx.original_source_epoch().is_some_and(|epoch|epoch>=source.epoch){return Err(clipboard_refusal("clipboard intent requires its actual initiating source and authenticated incoming epoch"))}Ok(())}

#[cfg(all(not(target_arch="wasm32"),not(any(test, feature = "testkit"))))]
pub fn submit_clipboard_io(source:&mut Option<semio_ui_wgpu_input::OriginalClipboardIntent>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
    let Some(intent)=source.as_ref()else{return Ok(true)};validate_clipboard_source(intent.source,cx)?;
    MOUNTED_CLIPBOARD_IO.with(|cell|{let mut slots=cell.borrow_mut();let Some((index,slot))=slots.iter_mut().enumerate().find(|(_,slot)|slot.mounted.is_none()&&slot.pending_source.is_none()&&slot.generation<u64::MAX)else{return Ok(false)};
        let grant=cx.retained_grant();let bytes=std::mem::size_of::<MountedClipboardIo>();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)};
        if !cx.original_source_parent()?.admit_loan(&mut slot.pending_source,cx)?{return Ok(false)}
        let original=slot.pending_source.take().expect("admitted original clipboard source");
        let intent=source.take().expect("admitted original clipboard intent");let invocation:ClipboardInvocationInput=ClipboardInvocationInput{original,source:intent.source,recipient:intent.recipient,slot:index as u8,generation:slot.generation+1};
        let job = match intent.text { Some(text) => ClipboardJob::write(text), None => ClipboardJob::read() };
        slot.generation=invocation.generation;slot.mounted=Some(MountedClipboardIo{invocation,job:Some(job),params:None,parameter_cancel:None,admission:Default::default(),session:None,outcome:None,copied:None,delivered:false,closing:false});
        cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_bytes:bytes,..Default::default()})?;Ok(true)
    })
}

#[cfg(all(not(target_arch="wasm32"),not(any(test, feature = "testkit"))))]
pub fn deliver_clipboard_recipient(recipient:&semio_ui_wgpu_input::ClipboardIntentRecipient,result:Result<Option<semio_ui_wgpu_hosting::host::ClipboardContent>,semio_ui_action::BoundedActionFault>,input:&mut semio_ui_wgpu_input::InputState<ActionDescriptor>){
    match recipient {
        semio_ui_wgpu_input::ClipboardIntentRecipient::Write(_)=>if let Err(fault)=result{input.record_action_fault(fault)},
        semio_ui_wgpu_input::ClipboardIntentRecipient::Window(window)=>match result{Ok(Some(semio_ui_wgpu_hosting::host::ClipboardContent::Text(text)))=>UI_ENGINE.with(|cell|{let mut engine=cell.borrow_mut();let commands=engine.dispatch_event(window,semio_ui_wgpu_events::UiEvent::Paste{text});engine.enqueue_async_commands(commands)}),Err(fault)=>input.record_action_fault(fault),_=>{}},
        semio_ui_wgpu_input::ClipboardIntentRecipient::Text{slot,generation}=>complete_pending_native_text_editor_clipboard(PendingNativeTextEditorClipboardToken{slot:*slot,generation:*generation},result),
        semio_ui_wgpu_input::ClipboardIntentRecipient::Ink{slot,generation}=>complete_pending_native_ink_clipboard(PendingNativeInkClipboardToken{slot:*slot,generation:*generation},result),
    }
}

#[cfg(all(not(target_arch="wasm32"),not(any(test, feature = "testkit"))))]
pub fn pump_clipboard_io_one(input:&mut semio_ui_wgpu_input::InputState<ActionDescriptor>,cx:&mut semio_framework_job::StepContext<'_>,cleanup:bool)->Result<bool,semio_framework_job::OriginalTurnFailure>{
    use semio_framework_job::{InteractiveJob, close_original_retained_step, receive_original_retained_work};
    use semio_framework_job_worker::{WorkerJobDemandError, WorkerJobPoll as Poll};
    use semio_framework_value::{RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome, RetirementDemand, ValueError};
    MOUNTED_CLIPBOARD_IO.with(|cell|{let mut slots=cell.borrow_mut();
        if let Some(slot)=slots.iter_mut().find(|slot|slot.pending_source.as_ref().is_some_and(|loan|loan.validate_retirement_context(cx).is_ok())){
            let loan=slot.pending_source.as_mut().expect("retained clipboard source");
            loan.close_step(cx.original_source_parent()?,cx)?;
            if loan.terminal_is_empty(){slot.pending_source=None;}
            return Ok(true)
        }
        let Some((index,slot))=slots.iter_mut().enumerate().find(|(_,slot)|slot.mounted.as_ref().is_some_and(|owner|owner.invocation.source.operation==cx.operation()&&owner.invocation.source.generation==cx.generation()))else{return Ok(false)};
        let owner=slot.mounted.as_mut().unwrap();validate_clipboard_source(owner.invocation.source,cx)?;if owner.invocation.slot as usize!=index||owner.invocation.generation!=slot.generation{return Err(clipboard_refusal("clipboard recipient generation changed before original acknowledgement"))}if !owner.invocation.original.terminal_is_empty(){owner.invocation.original.validate_context(cx)?;}
        if cx.should_yield(){return Ok(true)}if cleanup||cx.is_cancelled(){owner.closing=true;}
        if owner.closing {
            if let Some(outcome) = owner.outcome.as_mut() {
                if outcome.acknowledge_outcome(cx)? {
                    owner.outcome.take().unwrap().begin_close();
                }
                return Ok(true);
            }
            if let Some(session) = owner.session.as_mut() {
                session.begin_close();
                match session.has_original_cancel_alias_witness(cx.original_cancel_token()) {
                    Ok(true) => {
                        if let Some((turn, _)) = cx.with_original_effect(cx.retained_grant(), |cx,effect| session.return_original_cancel_alias_step(cx.original_cancel_token(),effect))? {
                            let result = match turn.result {
                                Ok(Some(step)) => Ok(step),
                                Ok(None) | Err(WorkerJobDemandError::Contention(_)) => Ok(RetainedCloneStep::Progress(Default::default())),
                                Err(WorkerJobDemandError::Refused(error)) => Err(error),
                            };
                            receive_original_retained_work(cx,RetainedWorkOutcome { result, work:turn.work })?;
                        }
                        return Ok(true);
                    }
                    Ok(false) => {}
                    Err(WorkerJobDemandError::Contention(_)) => return Ok(true),
                    Err(WorkerJobDemandError::Refused(error)) => return Err(error.into()),
                }
                session.close_step(cx)?;
                if session.terminal_is_empty() { owner.session=None; }
                return Ok(true);
            }
            if !owner.admission.terminal_is_empty() {
                owner.admission.close_step(cx)?;
                return Ok(true);
            }
            if let Some(job) = owner.job.as_mut() {
                job.begin_close();
                if job.terminal_is_empty() {
                    let demand = RetirementDemand { copy_bytes: std::mem::size_of::<Option<ClipboardJob>>(), depth:1, ..Default::default() };
                    close_original_retained_step(cx,demand,|_| {
                        owner.job = None;
                        RetainedWorkOutcome { result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items:1, copied_bytes:demand.copy_bytes, ..Default::default() })), work:1 }
                    })?;
                } else {
                    job.close_step(cx)?;
                }
                return Ok(true);
            }
            if owner.params.is_some() {
                let grant=cx.retained_grant();
                if grant.maximum_items==0||grant.maximum_depth==0 { return Ok(true); }
                let params=owner.params.take().unwrap();
                owner.parameter_cancel=Some(semio_framework_async::CancelTokenRetirement::from_token(params.cancel));
                receive_original_retained_work(cx,RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1})?;
                return Ok(true);
            }
            if let Some(cancel) = owner.parameter_cancel.as_mut() {
                if let Some((turn,_)) = cx.with_original_effect(cx.retained_grant(), |cx,effect| cancel.return_alias_step(cx.original_cancel_token(),effect))? {
                    let result = match turn.result {
                        Ok(step) => Ok(step),
                        Err(semio_framework_async::CancelTokenRetirementError::Blocked(_)) => Ok(RetainedCloneStep::Progress(Default::default())),
                        Err(semio_framework_async::CancelTokenRetirementError::Refused(error)) => Err(error),
                    };
                    receive_original_retained_work(cx,RetainedWorkOutcome { result, work:turn.work })?;
                }
                if cancel.terminal_is_empty() { owner.parameter_cancel=None; }
                return Ok(true);
            }
            let release=match &owner.invocation.recipient{semio_ui_wgpu_input::ClipboardIntentRecipient::Write(window)|semio_ui_wgpu_input::ClipboardIntentRecipient::Window(window)=>window.capacity(),_=>0}+owner.copied.as_ref().map_or(0,String::capacity);
            if release>0{let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth<2||grant.maximum_release_bytes<release{return Ok(true)}drop(std::mem::replace(&mut owner.invocation.recipient,semio_ui_wgpu_input::ClipboardIntentRecipient::Write(String::new())));drop(owner.copied.take());cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,released_bytes:release,..Default::default()})?;cx.consume_fuel(1);return Ok(true)}
            if !owner.invocation.original.terminal_is_empty(){let parent=cx.original_source_parent()?;let _=owner.invocation.original.close_step(parent,cx)?;return Ok(true)}
            if cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(true)}cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);slot.mounted=None;return Ok(true)
        }
        if owner.session.is_none(){
            if owner.params.is_none() && owner.admission.terminal_is_empty() {
                let mut cancel = None;
                let admitted = cx.admit_original_cancel_alias(&mut cancel);
                owner.params = cancel.map(|cancel| semio_framework_job_worker_authority::BatchJobParams {
                    operation:cx.operation(),
                    generation:cx.generation(),
                    cancel,
                    config:semio_framework_job_worker_authority::BatchDriveConfig {
                        retained:cx.retained_grant(),
                        site:"renderer_original_clipboard",
                        stage:semio_framework_job::InteractiveStage::InteractiveStep,
                        fuel_per_step:cx.fuel_remaining(),
                        step_budget_us:cx.deadline_us().saturating_sub(cx.now_us().unwrap_or(cx.deadline_us())),
                    },
                    now_us:cx.clock_source(),
                });
                admitted?;
                return Ok(true);
            }
            owner.admission.advance(&mut owner.job,&mut owner.params,&mut owner.session,cx)?;
            return Ok(true);
        }
        let session=owner.session.as_mut().unwrap();
        if owner.outcome.is_none(){match session.poll(){Poll::Idle=>{if let Some((ticket,kind))=session.try_step_on_original_caller(cx)?{let _=session.take_retained_step_receipt().map_err(|_|clipboard_refusal("clipboard original callback receipt remains checked out"))?;owner.outcome=Some(if kind==Poll::Terminal{session.take_terminal()}else{session.take_outcome(ticket)}.map_err(|_|clipboard_refusal("clipboard original outcome remains owned by its producer"))?);}return Ok(true)},Poll::Closing=>{owner.closing=true;return Ok(true)},Poll::TerminalEmpty=>{owner.session=None;owner.closing=true;return Ok(true)},_=>return Ok(true)}}
        let outcome=owner.outcome.as_mut().unwrap();
        if !owner.delivered&&outcome.outcome_descriptor().is_some_and(|descriptor|descriptor.is_acknowledged()){
            let terminal=outcome.outcome_descriptor().unwrap().is_terminal();let acknowledged=outcome.acknowledge_outcome(cx)?;if !acknowledged{return Ok(true)}let original=owner.outcome.take().unwrap();if terminal{original.begin_close();owner.closing=true;}else if let Err(original)=original.resume(){owner.outcome=Some(original);return Err(clipboard_refusal("clipboard acknowledged original yield remains checked out"))}return Ok(true)
        }
        if !owner.delivered{
            let Some(view)=outcome.outcome()?else{let original=owner.outcome.take().unwrap();if let Err(original)=original.resume(){owner.outcome=Some(original);return Err(clipboard_refusal("clipboard yielding original outcome refused resumption"))}return Ok(true)};
            let result=match &view{semio_framework_job::JobOutcomeView::Yield{..}=>{let acknowledged=outcome.acknowledge_outcome(cx)?;if !acknowledged{return Ok(true)}let original=owner.outcome.take().unwrap();if let Err(original)=original.resume(){owner.outcome=Some(original);return Err(clipboard_refusal("clipboard acknowledged yield refused original resumption"))}return Ok(true)},semio_framework_job::JobOutcomeView::Complete{..}=>{
                match semio_ui_wgpu_hosting::host::ClipboardIoJob::read_candidate(&view){Some(semio_ui_wgpu_hosting::host::ClipboardContentBorrow::Text(text))=>{
                    let grant=cx.retained_grant();if owner.copied.is_none(){if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<text.len(){return Ok(true)}let copied=String::with_capacity(text.len());let capacity=copied.capacity();cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,retained_capacity_bytes:capacity,..Default::default()})?;cx.consume_fuel(1);owner.copied=Some(copied);return Ok(true)}
                    let copied=owner.copied.as_mut().unwrap();if copied.len()<text.len(){let grant=cx.retained_grant();let mut end=(copied.len()+grant.maximum_copy_bytes).min(text.len());while !text.is_char_boundary(end){end-=1}let bytes=end-copied.len();if grant.maximum_items==0||grant.maximum_depth==0||bytes==0{return Ok(true)}copied.push_str(&text[copied.len()..end]);cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(true)}Ok(Some(semio_ui_wgpu_hosting::host::ClipboardContent::Text(owner.copied.take().unwrap())))},Some(semio_ui_wgpu_hosting::host::ClipboardContentBorrow::ImageRgba8{..})=>Err(semio_ui_action::BoundedActionFault::Structure),None=>Ok(None)}},_=>Err(semio_ui_action::BoundedActionFault::Structure)};
            if cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(true)}deliver_clipboard_recipient(&owner.invocation.recipient,result,input);owner.delivered=true;cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(true)
        }
        let acknowledged=outcome.acknowledge_outcome(cx)?;if acknowledged{owner.outcome.take().unwrap().begin_close();owner.closing=true;}Ok(true)
    })
}

pub fn write_owned_os_clipboard(text:String,input:&mut semio_ui_wgpu_input::InputState<ActionDescriptor>){
    #[cfg(any(any(test, feature = "testkit"),target_arch="wasm32"))]
    write_os_clipboard(&text,input);
    #[cfg(all(not(any(test, feature = "testkit")),not(target_arch="wasm32")))]
    input.retain_original_clipboard_intent(semio_ui_wgpu_input::ClipboardIntentRecipient::Write(String::new()),Some(text)).unwrap_or_else(|_|panic!("clipboard write retained source admission is required"));
}

#[cfg(any(test, feature = "testkit"))]
thread_local! {
    pub static MOCK_CLIPBOARD_WRITES: std::cell::RefCell<Vec<String>> = std::cell::RefCell::new(Vec::new());
    pub static MOCK_CLIPBOARD_READ: std::cell::RefCell<Option<String>> = std::cell::RefCell::new(None);
}

#[cfg(any(test, feature = "testkit"))]
pub fn read_os_clipboard() -> Option<String> {
    MOCK_CLIPBOARD_READ.with(|cell| cell.borrow().clone())
}
//#endregion 🧩️RetainedUiEngine
