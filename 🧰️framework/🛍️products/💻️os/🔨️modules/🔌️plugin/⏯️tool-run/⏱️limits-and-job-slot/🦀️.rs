//! ⏱️ Limits and job slot of the tool run ledger: the turn, refold, discard, panel and trace limits, the job site, the job handle and slot with their transitions and the trace lane helpers. See [`super`].

use super::*;

//#region 🔖️Limits
/// ⏱️ Wall budget of one driver turn; every job step inside it keeps its own interactive-lane slice.
pub const TOOL_RUN_TURN_WALL_US: u64 = 4_000;

/// 🧮️ Folded ops between two wall-clock checks of a bounded refold.
pub const TOOL_RUN_REFOLD_CHECK_OPS: usize = 64;

/// 🧹️ Discarded provisional ops cold-retired per driver or close turn.
pub const TOOL_RUN_DISCARD_OPS_PER_TURN: usize = 4_096;

/// 📜️ Newest step-log rows the panel shows.
pub const TOOL_RUN_PANEL_STEP_ROWS: usize = 16;

/// 🧾️ Newest trace records the panel's keyboard trace list shows.
pub const TOOL_RUN_PANEL_TRACE_ROWS: usize = 32;

/// 📦️ Encoded trace delta bytes one renderer refresh receives.
pub const TOOL_RUN_TRACE_DELTA_BYTES: usize = 262_144;

/// 🔁️ Refreshes one window may re-request a trace backlog while its renderer's echoed cursor stands still;
/// a renderer that never echoes therefore costs a bounded number of refreshes, never a refresh loop.
pub const TOOL_RUN_TRACE_STALL_REFRESHES: u8 = 4;

#[doc(hidden)]
pub const TOOL_RUN_JOB_SITE: &str = "tool-run.step";
//#endregion 🔖️Limits

//#region 🔖️Job
#[doc(hidden)]
pub enum ToolRunJobHandle<C> {
    Plain(ToolRunJob),
    Retargetable(Box<dyn ToolRunRetargetableJob<C>>),
}

#[doc(hidden)]
pub struct ToolRunJobSlot<C> {
    pub receiving: semio_framework_tool_run::ToolRunReceivingLimits,
    pub job: ToolRunJobHandle<C>,
    pub purpose: ToolRunJobPurpose,
    pub operation: semio_framework_job::OperationId,
    pub generation: semio_framework_job::Generation,
    pub cancel: Option<semio_framework_job::CancelToken>,
    pub cancel_retirement: Option<semio_framework_async::CancelTokenRetirement>,
    pub preview_sequence: u64,
    pub authority: Option<semio_framework_job::JobPayloadAuthority>,
    pub outcome: Option<semio_framework_job::JobOutcomeDescriptor>,
    pub outcome_received: bool,
    pub original_failure: std::mem::ManuallyDrop<Option<semio_framework_job::OriginalTurnFailure>>,
    failure_retirement: Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
    pub closing: bool,
}

impl<C> ToolRunJobSlot<C> {
    pub fn new(job: ToolRunJobHandle<C>, purpose: ToolRunJobPurpose, generation: u32, receiving: semio_framework_tool_run::ToolRunReceivingLimits) -> Self {
        Self { receiving, job, purpose, operation: semio_framework_job::allocate_operation_id(), generation: semio_framework_job::Generation(u64::from(generation)), cancel: None, cancel_retirement: None, preview_sequence: 0, authority: None, outcome:None, outcome_received:false, original_failure:std::mem::ManuallyDrop::new(None), failure_retirement:None, closing: false }
    }

    pub fn interactive(&mut self) -> &mut dyn semio_framework_job::InteractiveJob {
        match &mut self.job {
            ToolRunJobHandle::Plain(job) => job.as_mut(),
            ToolRunJobHandle::Retargetable(job) => job.as_mut(),
        }
    }

    fn job_is_empty(&self) -> bool {
        match &self.job {
            ToolRunJobHandle::Plain(job) => job.terminal_is_empty(),
            ToolRunJobHandle::Retargetable(job) => job.terminal_is_empty(),
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.job_is_empty() && self.authority.is_none() && self.outcome.is_none() && self.original_failure.is_none() && self.failure_retirement.is_none() && self.cancel.is_none() && self.cancel_retirement.is_none()
    }

    /// 📐️ Bytes of the boxed job frame once every owner inside it is returned: the last allocation a retiring slot still holds.
    pub(super) fn scaffold_release_bytes(&self) -> usize {
        match &self.job {
            ToolRunJobHandle::Plain(job) => size_of_val(job.as_ref()),
            ToolRunJobHandle::Retargetable(job) => size_of_val(job.as_ref()),
        }
    }

    pub fn retargetable(&mut self) -> Option<&mut dyn ToolRunRetargetableJob<C>> {
        match &mut self.job {
            ToolRunJobHandle::Retargetable(job) if !self.closing && self.outcome.is_none() => Some(job.as_mut()),
            _ => None,
        }
    }

    pub fn begin_close(&mut self) {
        if !self.closing {
            if let Some(cancel) = self.cancel.as_ref() {
                cancel.cancel_now();
            }
            self.interactive().begin_close();
            self.closing = true;
        }
    }

    pub fn borrow_outcome(&self)->Result<Option<semio_framework_job::JobOutcomeView<'_>>,ValueError>{
        let Some(descriptor)=self.outcome.as_ref()else{return Ok(None)};
        let job:&dyn semio_framework_job::InteractiveJob=match &self.job{ToolRunJobHandle::Plain(job)=>job.as_ref(),ToolRunJobHandle::Retargetable(job)=>job.as_ref()};
        job.borrow_outcome(descriptor).map(Some)
    }

    pub fn next_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.outcome.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        if let Some(owner)=self.failure_retirement.as_ref(){return Ok(if owner.terminal_is_empty(){RetirementDemand{release_bytes:size_of_val(owner.as_ref()),depth:1,..Default::default()}}else{RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original ToolRun failure depth overflow"))?}})}
        if self.original_failure.is_some(){return Ok(RetirementDemand{capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<semio_framework_job::OriginalTurnFailure>(),depth:2,..Default::default()})}
        let job:&dyn semio_framework_job::InteractiveJob=match &self.job{ToolRunJobHandle::Plain(job)=>job.as_ref(),ToolRunJobHandle::Retargetable(job)=>job.as_ref()};
        if self.closing&&job.terminal_is_empty(){
            if let Some(owner)=self.authority.as_ref(){return Ok(if owner.terminal_is_empty(){RetirementDemand{depth:1,..Default::default()}}else{owner.retirement_demands()})}
            if let Some(retirement)=self.cancel_retirement.as_ref(){return if retirement.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{retirement.retirement_demands()}}
            if self.cancel.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        }
        if !self.closing||job.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"a terminal tool job slot is removed by its ledger, never quoted"))}
        Ok(RetirementDemand{copy_bytes:job.next_close_copy_byte_demand()?,capacity_bytes:job.next_close_capacity_byte_demand(body)?,release_bytes:job.next_close_release_byte_demand()?,depth:job.next_close_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"tool job close depth overflow"))?})
    }

    pub fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<RetainedCloneStep,semio_framework_job::OriginalTurnFailure>{
        let grant=cx.retained_grant();
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
        let demand=self.next_demands(grant.maximum_copy_bytes)?;
        if !tool_grant_funds(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
        if self.outcome.is_none()&&self.failure_retirement.is_none()&&self.original_failure.is_none()&&!self.job_is_empty(){
            if cx.should_yield()||!tool_grant_funds(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
            let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};let step=cx.with_retained_loan(child,|cx|self.interactive().close_step(cx))?;
            return Ok(RetainedCloneStep::Progress(step.progress()))
        }
        let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{
        if let Some(descriptor)=self.outcome.as_mut(){if !descriptor.is_acknowledged(){return Ok(RetainedCloneStep::Progress(descriptor.acknowledge(grant).progress()))}drop(self.outcome.take());self.outcome_received=false;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}
        if let Some(owner)=self.failure_retirement.as_mut(){if owner.terminal_is_empty(){drop(self.failure_retirement.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}))}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=owner.close_step(child)?;admit_retained_clone_close(child,step,owner.terminal_is_empty(),"original ToolRun callback failure")?;return Ok(RetainedCloneStep::Progress(step.progress()))}
        if let Some(original)=self.original_failure.take(){let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};return match semio_framework_value::retirement::admit_owned_retirement(original,child){Ok((owner,progress))=>{self.failure_retirement=Some(owner);Ok(RetainedCloneStep::Progress(progress))},Err((error,original))=>{*self.original_failure=Some(original);Err(error)}}}
        if self.job_is_empty(){
            if let Some(owner)=self.authority.as_mut(){if owner.terminal_is_empty(){drop(self.authority.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}return owner.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}
            if let Some(retirement)=self.cancel_retirement.as_mut(){if retirement.terminal_is_empty(){drop(self.cancel_retirement.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}return match retirement.close_step(grant){Ok(step)=>Ok(RetainedCloneStep::Progress(step.progress())),Err(semio_framework_async::CancelTokenRetirementError::Blocked(_))=>Ok(RetainedCloneStep::Progress(Default::default())),Err(semio_framework_async::CancelTokenRetirementError::Refused(error))=>Err(error)}}
            if let Some(token)=self.cancel.take(){self.cancel_retirement=Some(semio_framework_async::CancelTokenRetirement::from_token(token));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}
        }
            Ok(RetainedCloneStep::Complete(Default::default()))
        })?;Ok(RetainedCloneStep::Progress(step.progress()))
    }
}

/// 📼️ Lane text of one delta: `None` for an empty, non-clearing delta, else its base64url (unpadded) pack body.
pub(super) fn tool_run_trace_lane_text(delta: &ToolRunTraceDelta) -> Option<String> {
    if !delta.clear && delta.pages.is_empty() {
        return None;
    }
    delta.encode().ok().map(base64_codec::base64_url_encode)
}

/// 🔎️ The first World3d, Canvas2d or Board2d scene surface of a rendered body, depth first.
#[doc(hidden)]
pub fn tool_run_scene_surface(node: &mut BuiltNode) -> Option<&mut BuiltNode> {
    let is_scene = matches!(&node.component, Component::Surface(props) if [World3dScene::SCHEMA, Canvas2dScene::SCHEMA, Board2dScene::SCHEMA].contains(&props.doc_schema.as_str()));
    if is_scene {
        return Some(node);
    }
    node.children.iter_mut().find_map(tool_run_scene_surface)
}

/// 📎️ Re-attaches `lane` to a scene surface: the spine's lane refs gain the `toolRunTrace` entry and the
/// surface gains its paged carrier, exactly as `SceneDoc::split_lanes` plus `scene_surface` would emit it.
/// A surface whose producer already carries the lane is left untouched.
#[doc(hidden)]
pub fn inject_tool_run_trace_lane_into(surface: &mut BuiltNode, lane: &str) -> UiAssemblyResult<()> {
    let Component::Surface(props) = &mut surface.component else { return Ok(()) };
    let (name, key) = match props.doc_schema.as_str() {
        World3dScene::SCHEMA => (World3dSceneLane::ToolRunTrace.name(), World3dSceneLane::ToolRunTrace.body_key()),
        Board2dScene::SCHEMA => (Board2dSceneLane::ToolRunTrace.name(), Board2dSceneLane::ToolRunTrace.body_key()),
        _ => (Canvas2dSceneLane::ToolRunTrace.name(), Canvas2dSceneLane::ToolRunTrace.body_key()),
    };
    if surface.children.iter().any(|child| child.key.as_str() == key) {
        return Ok(());
    }
    let reference = SceneLaneRef { lane: name.to_string(), bytes: lane.len() as u32, hash: scene_lane_hash(lane) };
    let encoded = match props.doc_schema.as_str() {
        World3dScene::SCHEMA => {
            let mut spine: World3dScene = semio_framework_ui_scene::decode(props).map_err(|error| ui_assembly_error_because("tool-run-trace.decode", error))?;
            spine.lanes.push(reference);
            semio_framework_ui_scene::encode(props.kind, &spine)
        }
        Board2dScene::SCHEMA => {
            let mut spine: Board2dScene = semio_framework_ui_scene::decode(props).map_err(|error| ui_assembly_error_because("tool-run-trace.decode", error))?;
            spine.lanes.push(reference);
            semio_framework_ui_scene::encode(props.kind, &spine)
        }
        _ => {
            let mut spine: Canvas2dScene = semio_framework_ui_scene::decode(props).map_err(|error| ui_assembly_error_because("tool-run-trace.decode", error))?;
            spine.lanes.push(reference);
            semio_framework_ui_scene::encode(props.kind, &spine)
        }
    };
    let mut encoded = encoded.map_err(|error| ui_assembly_error_because("tool-run-trace.encode", error))?;
    let carrier = paged_text_carrier(key, lane)?;
    if surface.children.len() == UI_BUILT_CHILDREN_MAX {
        return Err(ui_assembly_error("tool-run-trace.carrier"));
    }
    encoded.bindings = std::mem::take(&mut props.bindings);
    *props = encoded;
    surface.children.try_push(carrier).map_err(|_| ui_assembly_error("tool-run-trace.carrier"))
}
//#endregion 🔖️Job
