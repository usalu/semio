//! 🧩️ Pack load contract and value frames: the mounted token, the controlled demands, the pack load diagnostic, phase, progress and step, the erased pack load, the expected and built values, the value frame, the retained string and the retained typed state. See [`super`].

use super::*;

pub(super) type Mounted = store::mounted_pack_rt::RetainedValueToken;

#[doc(hidden)]
pub fn controlled_demands<T: RetireOwned>(owner: &ControlledRetirement<T>, body: usize) -> Result<RetirementDemand, ValueError> {
    Ok(owner.next_demand(body)?)
}

pub(super) fn pack_close_receipt(step: store::mounted_pack_rt::RetainedPackCloseStep, demand: RetirementDemand) -> RetainedCloneStep {
    let (items, bytes) = match step { store::mounted_pack_rt::RetainedPackCloseStep::Pending { released_items, released_bytes } => (released_items, released_bytes), store::mounted_pack_rt::RetainedPackCloseStep::Complete => (1, 0) };
    RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: usize::from(items != 0 || bytes != 0), copied_bytes: if items == 0 { 0 } else { demand.copy_bytes }, released_bytes: bytes, ..Default::default() })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowConfigPackLoadDiagnostic {
    EnvelopeIdentity,
    Pack,
    TypedState,
    History,
    InnerIdentity,
    Replay,
    Capacity,
    Stale,
    Cancelled,
    Retirement,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowConfigPackLoadPhase {
    EnvelopeIdentity,
    PackIngress,
    PackReplay,
    PackRetirement,
    HistoryReplay,
    InputRetirement,
    StoreHydration,
    StoreAddress,
    Ready,
    RetiringRejectedCandidate,
    RetiringDisplacedStore,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindowConfigPackLoadProgress {
    pub phase: WindowConfigPackLoadPhase,
    pub completed_bytes: u64,
    pub total_bytes: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowConfigPackLoadStep {
    Pending(WindowConfigPackLoadProgress),
    OwnershipProgress(WindowConfigPackLoadProgress,RetainedCloneProgress),
    Ready,
    Rejected(WindowConfigPackLoadDiagnostic),
    Complete,
}

pub trait ErasedWindowConfigPackLoad: Send {
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn window_kind_id(&self) -> &str;
    fn registry_lifetime(&self) -> u64;
    fn phase(&self) -> WindowConfigPackLoadPhase;
    fn progress(&self) -> WindowConfigPackLoadProgress;
    fn diagnostic(&self) -> Option<WindowConfigPackLoadDiagnostic>;
    fn advance(&mut self, grant: RetainedCloneGrant) -> WindowConfigPackLoadStep;
    fn request_cancel(&mut self);
    fn reject_stale(&mut self) -> WindowConfigPackLoadStep;
    fn demand_bytes(&self) -> usize;
    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError>;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, String>;
    fn terminal_is_empty(&self) -> bool;
}

pub struct WindowConfigPackLoad {
    pub(in super::super) inner:ManuallyDrop<Option<Box<dyn ErasedWindowConfigPackLoad>>>,
    pub(super) closed_progress:WindowConfigPackLoadProgress,
    pub(super) closed_diagnostic:Option<WindowConfigPackLoadDiagnostic>,
}

impl WindowConfigPackLoad {
    #[cfg(test)]
    #[doc(hidden)]
    pub fn candidate_actor_for_test<O:WindowConfigOwner>(&mut self)->Option<protocol::ActorId>{let typed=self.inner.as_mut()?.as_any_mut().downcast_mut::<TypedWindowConfigPackLoad<O>>()?;typed.candidate.as_ref().map(|candidate|candidate.store.local_actor_id().clone())}
    pub fn phase(&self)->WindowConfigPackLoadPhase{self.inner.as_ref().map_or(WindowConfigPackLoadPhase::Complete,|inner|inner.phase())}
    pub fn progress(&self)->WindowConfigPackLoadProgress{self.inner.as_ref().map_or(self.closed_progress,|inner|inner.progress())}
    pub fn diagnostic(&self)->Option<WindowConfigPackLoadDiagnostic>{self.inner.as_ref().map_or(self.closed_diagnostic,|inner|inner.diagnostic())}
    pub fn request_cancel(&mut self){if let Some(inner)=self.inner.as_mut(){inner.request_cancel();}}
    pub fn cold_work_grant(&mut self)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES.max(self.inner.as_mut().map_or(0,|inner|inner.demand_bytes())),maximum_capacity_bytes:store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES,maximum_release_bytes:store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES,maximum_depth:64}}
    pub fn terminal_is_empty(&self)->bool{self.inner.is_none()}
    pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        let Some(inner)=self.inner.as_ref()else{return Ok(Default::default());};
        if inner.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:size_of_val(inner.as_ref()),depth:1,..Default::default()});}
        let mut demand=inner.retirement_demands(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"window load frame depth overflow"))?;Ok(demand)
    }

    /// ♻️ Retires the load one granted unit at a time; the final unit releases the erased frame and records its terminal progress.
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,String>{
        let Some(inner)=self.inner.as_ref()else{return Ok(PluginLifecycleStep::Complete(Default::default()));};
        if !inner.terminal_is_empty(){
            return self.inner.as_mut().expect("checked window load frame remains").close_step(grant).map(|step|match step{PluginLifecycleStep::Complete(progress)=>PluginLifecycleStep::Progress(progress),other=>other});
        }
        let demand=self.retirement_demands(grant.maximum_copy_bytes).map_err(ValueError::into_message)?;
        if grant.maximum_items==0||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(PluginLifecycleStep::Progress(Default::default()));}
        let inner=self.inner.take().expect("checked window load frame remains");
        self.closed_progress=inner.progress();self.closed_diagnostic=inner.diagnostic();
        drop(inner);
        Ok(PluginLifecycleStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}))
    }

    /// 🎟️ Fixed caller policy; each currency retains its independent authority.
    pub const fn next_grant(&self) -> RetainedCloneGrant {
        RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 65_536, maximum_capacity_bytes: 65_536, maximum_release_bytes: 65_536, maximum_depth: 64 }
    }
}

impl Drop for WindowConfigPackLoad {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.inner.is_none(), "window config Pack load reached Drop before terminal-empty handoff or retirement");
    }
}

#[derive(Clone, Copy)]
pub(super) struct ExpectedValue {
    pub(super) shape: Option<semio_framework_dsl_record::BorrowedShape>,
    pub(super) dsl: bool,
}

impl ExpectedValue {
    pub(super) fn field(shape: Option<semio_framework_dsl_record::BorrowedShape>) -> Self {
        Self { shape, dsl: false }
    }

    pub(super) fn dsl() -> Self {
        Self { shape: None, dsl: true }
    }
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) enum BuiltValue {
    Field(semio_framework_dsl_record::FieldValue),
    Dsl(semio_framework_value::DslValue),
}

#[doc(hidden)]
pub enum ValueFrame {
    Record { kind: store::mounted_pack_rt::RetainedValueContainer, root: bool, spec: Option<semio_framework_dsl_record::BorrowedRecordSpec>, fields: semio_framework_dsl_record::RecordFields, field: Option<u16> },
    Sequence { kind: store::mounted_pack_rt::RetainedValueContainer, element: ExpectedValue, values: Vec<BuiltValue> },
    Map { dsl: bool, element: ExpectedValue, values: Vec<(String, BuiltValue)>, key: Option<String> },
    Statements { variants: &'static [(&'static str, fn() -> semio_framework_dsl_record::BorrowedRecordSpec)], values: Vec<(String, semio_framework_dsl_record::RecordValue)>, keyword: Option<String> },
    Bytes { values: Vec<u8>, remaining: usize },
}

#[derive(Clone, Copy)]
pub(super) enum ValueWrapper {
    Block,
    Dynamic,
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) enum StringTarget {
    Value(ExpectedValue),
    MapKey,
    StatementKeyword,
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct RetainedString {
    pub(super) target: StringTarget,
    pub(crate) value: String,
    pub(super) remaining: Option<u64>,
    pub(super) symbol: Option<(u64, usize, usize)>,
}

#[doc(hidden)]
pub struct RetainedWindowConfigTypedState<O: WindowConfigOwner> {
    #[doc(hidden)]
    pub spec: semio_framework_dsl_record::BorrowedRecordSpec,
    #[doc(hidden)]
    pub stack: Vec<ValueFrame>,
    pub(super) wrappers: Vec<(ValueWrapper, usize)>,
    pub(super) string: Option<RetainedString>,
    pub(super) tag: Option<u8>,
    #[doc(hidden)]
    pub root: Option<O::State>,
    pub(super) complete: bool,
    pub(super) handed_back: bool,
    pub(super) retirement_original: ManuallyDrop<Option<RetainedTypedOwners<O>>>,
    pub(super) retirement: Option<ControlledRetirement<RetainedTypedOwners<O>>>,
}

semio_framework_value::artifact_retire_struct!(ExpectedValue{shape,dsl});

semio_framework_value::artifact_retire_leaf!(ValueWrapper);

impl RetireOwned for ValueFrame {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::Record { spec, fields, .. } => semio_framework_value::artifact_retirement_sequence![spec, fields],
            Self::Sequence { element, values, .. } => semio_framework_value::artifact_retirement_sequence![element, values],
            Self::Map { element, values, key, .. } => semio_framework_value::artifact_retirement_sequence![element, values, key],
            Self::Statements { values, keyword, .. } => semio_framework_value::artifact_retirement_sequence![values, keyword],
            Self::Bytes { values, .. } => values.retirement(),
        }
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        use semio_framework_value::retirement::{sequence_birth_bytes, deferred_birth_bytes_for};
        match self {
            Self::Record { spec, fields, .. } => sequence_birth_bytes(&[deferred_birth_bytes_for(spec), deferred_birth_bytes_for(fields)]),
            Self::Sequence { element, values, .. } => sequence_birth_bytes(&[deferred_birth_bytes_for(element), deferred_birth_bytes_for(values)]),
            Self::Map { element, values, key, .. } => sequence_birth_bytes(&[deferred_birth_bytes_for(element), deferred_birth_bytes_for(values), deferred_birth_bytes_for(key)]),
            Self::Statements { values, keyword, .. } => sequence_birth_bytes(&[deferred_birth_bytes_for(values), deferred_birth_bytes_for(keyword)]),
            Self::Bytes { values, .. } => values.retirement_birth_bytes(),
        }
    }
    fn controlled_retirement_supported() -> bool { true }
    fn retirement_element_copy_bytes() -> usize { size_of::<Self>() }
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct RetainedTypedOwners<O: WindowConfigOwner> {
    pub(super) spec: semio_framework_dsl_record::BorrowedRecordSpec,
    pub(super) stack: Vec<ValueFrame>,
    pub(super) wrappers: Vec<(ValueWrapper, usize)>,
    pub(super) string: Option<RetainedString>,
    pub(super) root: Option<O::State>,
}
