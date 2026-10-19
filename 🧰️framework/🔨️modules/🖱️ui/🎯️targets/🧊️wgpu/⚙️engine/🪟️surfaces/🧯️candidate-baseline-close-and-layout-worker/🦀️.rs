//! 🧯️ Candidate baseline, close and layout worker: the candidate baseline, the surface close phases and steps and the layout worker admission with its retirement. See [`super`].

use super::*;

//#region 🔖️UiWindow
/// 🪙 A fallible, bounded initial candidate revision. The presented tree keeps the exact records
/// required by accessibility and intent dispatch while this owner acquires one credited record per
/// reconcile opportunity. Refusal drains the partial copy before surfacing a terminal fault.
pub struct UiCandidateBaseline {
    pub document: Option<UiDocumentTree>,
    pub next_record: usize,
    pub fault: Option<UiDocumentReconcileFault>,
}

pub enum UiCandidateBaselineStep {
    Pending,
    Complete(UiDocumentTree),
    Fault(UiDocumentReconcileFault),
}

impl UiCandidateBaseline {
    pub fn new(source: &UiDocumentTree) -> Self {
        Self { document: Some(source.credited_baseline()), next_record: 0, fault: None }
    }

    pub fn generation(&self) -> Option<u64> {
        self.document.as_ref().map(UiDocumentTree::generation)
    }

    fn refuse(&mut self, fault: UiDocumentReconcileFault) -> UiCandidateBaselineStep {
        self.fault = Some(fault);
        UiCandidateBaselineStep::Pending
    }

    pub fn step(&mut self, source: Option<&UiDocumentTree>) -> UiCandidateBaselineStep {
        if let Some(fault) = self.fault {
            if let Some(document) = self.document.as_mut() {
                if !document.close_step() {
                    return UiCandidateBaselineStep::Pending;
                }
            }
            self.document = None;
            return UiCandidateBaselineStep::Fault(fault);
        }
        let Some(source) = source else { return self.refuse(UiDocumentReconcileFault::MissingRecord) };
        let Some(candidate) = self.document.as_mut() else { return self.refuse(UiDocumentReconcileFault::MissingRecord) };
        if candidate.generation() != source.generation()
            || candidate.revision() != source.revision()
            || candidate.surface() != source.surface()
            || candidate.root_id() != source.root_id()
            || candidate.layout_epoch() != source.layout_epoch()
            || candidate.node_count() != source.node_count()
        {
            return self.refuse(UiDocumentReconcileFault::MissingRecord);
        }
        if self.next_record < source.node_count() {
            let record = match source.credited_record_at(self.next_record) {
                Ok(record) => record,
                Err(UiDocumentTreeFault::Credits) => return self.refuse(UiDocumentReconcileFault::DocumentCredits),
                Err(_) => return self.refuse(UiDocumentReconcileFault::MissingRecord),
            };
            if candidate.try_upsert_record(record).is_err() {
                return self.refuse(UiDocumentReconcileFault::DocumentCredits);
            }
            self.next_record += 1;
            return UiCandidateBaselineStep::Pending;
        }
        if candidate.validate_header().is_err() {
            return self.refuse(UiDocumentReconcileFault::MissingRecord);
        }
        match self.document.take() {
            Some(document) => UiCandidateBaselineStep::Complete(document),
            None => self.refuse(UiDocumentReconcileFault::MissingRecord),
        }
    }

    pub fn close_step(&mut self) -> bool {
        let Some(document) = self.document.as_mut() else { return true };
        if !document.close_step() {
            return false;
        }
        self.document = None;
        false
    }
}

pub fn retire_surface_scene(document_id: UiNodeId, window_id: &str, window_generation: u64, retirements: &mut VecDeque<UiRetiredComponentScene>, node_id: NodeId, node: &semio_framework_ui_retained_tree::tree::Node) -> bool {
    let UiNode::ComponentScene(scene) = &node.spec.0 else { return true };
    if retirements.len() == UI_RETIRED_COMPONENT_SCENE_CAPACITY {
        return false;
    }
    retirements.push_back(UiRetiredComponentScene {
        document_id,
        host_id: scene.host_id.clone(),
        window_id: window_id.to_string(),
        window_generation,
        component_generation: node.component_generation(),
        node: node_id,
        key: node.key.clone(),
        kind: scene.component_kind,
        surface_id: scene.surface_id.clone(),
    });
    true
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiSurfaceClosePhase {
    Router,
    PresentedRouter,
    LayoutAdmission,
    LayoutSession,
    LayoutJob,
    Previews,
    Paint,
    Draw,
    Hits,
    Document,
    PresentedDocument,
    Storage,
    PresentedStorage,
    Scheduler,
    Complete,
}

/// 🧹️ One retained-surface close opportunity and its externally owned scene handoff.
pub enum UiSurfaceCloseStep {
    Pending,
    RetiredScene(UiRetiredComponentScene),
    Refused(UiWorkerRetirementRefusal),
    Complete,
}

/// 🚨️ Retains the original close owner while exposing the exact structural refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiWorkerRetirementRefusal { pub stage: &'static str, pub kind: semio_framework_value::ValueRefusalKind, pub progress:semio_framework_value::RetainedCloneProgress }

pub fn close_layout_worker_session(session:&mut semio_framework_job_worker::MountedWorkerJobSession<MountedLayoutJob>,cx:&mut StepContext<'_>)->Result<Option<semio_framework_value::RetainedCloneProgress>,semio_framework_job::OriginalTurnFailure>{
 use semio_framework_job_worker::{WorkerJobDemandError};use semio_framework_job_worker_authority::{WorkerJobCloseStep};
 let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0{return Ok(Some(Default::default()))}
 if session.poll()!=semio_framework_job_worker::WorkerJobPoll::CheckedOut{session.begin_close();let demand=match session.retirement_demands(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(WorkerJobDemandError::Contention(_))=>return Ok(None),Err(WorkerJobDemandError::Refused(error))=>return Err(error.into())};if demand.copy_bytes>grant.maximum_copy_bytes||demand.capacity_bytes>grant.maximum_capacity_bytes||demand.release_bytes>grant.maximum_release_bytes||demand.depth>grant.maximum_depth{return Ok(Some(Default::default()))}}
 let step=session.close_step(cx)?;match step{WorkerJobCloseStep::Pending{progress}if progress.fits(grant)=>Ok(Some(progress)),WorkerJobCloseStep::Complete{progress}if progress.fits(grant)&&session.terminal_is_empty()=>Ok(Some(progress)),WorkerJobCloseStep::Blocked=>Ok(None),step=>Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"UI original worker retirement receipt invariant violated").with_retained_progress(step.progress()).into())}
}

/// 🚪️ Keeps original pending parameters until session admission or paid cancellation retirement.
#[derive(Default)]
pub struct LayoutWorkerAdmission {
    pub params: std::mem::ManuallyDrop<Option<semio_framework_job_worker_authority::BatchJobParams>>,
    pub cancel: std::mem::ManuallyDrop<Option<semio_framework_async::CancelTokenRetirement>>,
    pub failure: std::mem::ManuallyDrop<Option<semio_framework_value::ValueError>>,
    pub failure_close: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_value::ValueError>>>,
}

impl LayoutWorkerAdmission {
    pub fn terminal_is_empty(&self) -> bool { self.params.is_none() && self.cancel.is_none() && self.failure.is_none() && self.failure_close.is_none() }
    fn parameter_retirement_demands(&self) -> semio_framework_value::RetirementDemand {
        semio_framework_value::RetirementDemand { copy_bytes: 0, depth: 1, ..Default::default() }
    }
    pub fn close_step(&mut self, grant: semio_framework_job::RetainedCloneGrant, witness: Option<&semio_framework_job::CancelToken>) -> Result<Option<semio_framework_value::RetainedCloneProgress>, semio_framework_value::ValueError> {
        use semio_framework_value::RetainedCloneProgress;
        if self.terminal_is_empty() { return Ok(Some(Default::default())); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(Some(Default::default())); }
        if let Some(original) = self.failure_close.as_mut() {
            if original.terminal_is_empty() {
                drop(self.failure_close.take());
                return Ok(Some(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
            }
            if original.next_copy_byte_demand()? > grant.maximum_copy_bytes
                || original.next_capacity_byte_demand(grant.maximum_copy_bytes)? > grant.maximum_capacity_bytes
                || original.next_release_byte_demand()? > grant.maximum_release_bytes
                || original.next_depth_demand()? > grant.maximum_depth { return Ok(Some(Default::default())); }
            return original.step(grant).map(|step| Some(step.progress()));
        }
        if let Some(original) = self.failure.take() {
            *self.failure_close = Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(original).ok().expect("ValueError has original controlled retirement"));
            return Ok(Some(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if self.params.is_some() {
            let copied_bytes = self.parameter_retirement_demands().copy_bytes;
            if grant.maximum_copy_bytes < copied_bytes { return Ok(Some(Default::default())); }
            let params = self.params.take().unwrap();
            *self.cancel = Some(semio_framework_async::CancelTokenRetirement::from_token(params.cancel));
            return Ok(Some(RetainedCloneProgress { copied_items: 1, copied_bytes, ..Default::default() }));
        }
        if let Some(cancel) = self.cancel.as_mut() {
            if cancel.terminal_is_empty() {
                let copied_bytes = 0;
                if grant.maximum_copy_bytes < copied_bytes { return Ok(Some(Default::default())); }
                drop(self.cancel.take());
                return Ok(Some(RetainedCloneProgress { copied_items: 1, copied_bytes, ..Default::default() }));
            }
            let step = match witness.filter(|original| cancel.is_original_alias_witness(original)) {
                Some(original) => cancel.return_alias_step(original, grant),
                None => cancel.close_step(grant),
            };
            return match step {
                Ok(step) => Ok(Some(step.progress())),
                Err(semio_framework_async::CancelTokenRetirementError::Blocked(_)) => Ok(None),
                Err(semio_framework_async::CancelTokenRetirementError::Refused(error)) => Err(error),
            };
        }
        Ok(Some(Default::default()))
    }
}

impl Drop for LayoutWorkerAdmission {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "layout admission abandoned original parameters or cancellation"); }
}

/// 🧾️ Records one original worker close receipt before a child error crosses the layout boundary.
pub fn consume_layout_retirement(context:&mut StepContext<'_>,step:Result<Option<semio_framework_value::RetainedCloneProgress>,semio_framework_value::ValueError>)->Result<bool,semio_framework_value::ValueError>{
    match step{Ok(Some(progress))=>{context.consume_retained(progress)?;Ok(true)},Ok(None)=>Ok(false),Err(error)=>{context.consume_retained(error.retained_progress())?;Err(error)}}
}
//#endregion 🔖️UiWindow
